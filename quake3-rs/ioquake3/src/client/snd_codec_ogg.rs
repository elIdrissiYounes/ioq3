use ::libc;

pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::int64_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::config_types_h::ogg_int64_t;
pub use crate::src::client::snd_codec::snd_codec_s;
pub use crate::src::client::snd_codec::snd_codec_t;
pub use crate::src::client::snd_codec::snd_info_s;
pub use crate::src::client::snd_codec::snd_info_t;
pub use crate::src::client::snd_codec::snd_stream_s;
pub use crate::src::client::snd_codec::snd_stream_t;
pub use crate::src::client::snd_codec::S_CodecUtilClose;
pub use crate::src::client::snd_codec::S_CodecUtilOpen;
pub use crate::src::client::snd_codec::CODEC_CLOSE;
pub use crate::src::client::snd_codec::CODEC_LOAD;
pub use crate::src::client::snd_codec::CODEC_OPEN;
pub use crate::src::client::snd_codec::CODEC_READ;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_clear;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_info;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_open_callbacks;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_pcm_total;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_read;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_seekable;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_streams;
pub use crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::FS_SEEK_CUR;
pub use crate::src::qcommon::q_shared::FS_SEEK_END;
pub use crate::src::qcommon::q_shared::FS_SEEK_SET;

pub use crate::codec_h::alloc_chain;
pub use crate::codec_h::vorbis_block;
pub use crate::codec_h::vorbis_comment;
pub use crate::codec_h::vorbis_dsp_state;
pub use crate::codec_h::vorbis_info;
pub use crate::ogg_h::ogg_stream_state;
pub use crate::ogg_h::ogg_sync_state;
pub use crate::ogg_h::oggpack_buffer;

// Q3 OGG codec
#[no_mangle]

pub static mut ogg_codec: crate::src::client::snd_codec::snd_codec_t = unsafe {
    {
        let mut init = crate::src::client::snd_codec::snd_codec_s {
            ext: b"ogg\x00" as *const u8 as *mut i8,
            load: Some(
                S_OGG_CodecLoad
                    as unsafe extern "C" fn(
                        _: *const i8,
                        _: *mut crate::src::client::snd_codec::snd_info_t,
                    ) -> *mut libc::c_void,
            ),
            open: Some(
                S_OGG_CodecOpenStream
                    as unsafe extern "C" fn(
                        _: *const i8,
                    )
                        -> *mut crate::src::client::snd_codec::snd_stream_t,
            ),
            read: Some(
                S_OGG_CodecReadStream
                    as unsafe extern "C" fn(
                        _: *mut crate::src::client::snd_codec::snd_stream_t,
                        _: i32,
                        _: *mut libc::c_void,
                    ) -> i32,
            ),
            close: Some(
                S_OGG_CodecCloseStream
                    as unsafe extern "C" fn(
                        _: *mut crate::src::client::snd_codec::snd_stream_t,
                    ) -> (),
            ),
            next: 0 as *mut crate::src::client::snd_codec::snd_codec_t,
        };
        init
    }
};
// callbacks for vobisfile
// fread() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OGG_Callback_read(
    mut ptr: *mut libc::c_void,
    mut size: crate::stddef_h::size_t,
    mut nmemb: crate::stddef_h::size_t,
    mut datasource: *mut libc::c_void,
) -> crate::stddef_h::size_t {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    let mut byteSize: i32 = 0;
    let mut bytesRead: i32 = 0;
    let mut nMembRead: crate::stddef_h::size_t = 0;
    // check if input is valid
    if ptr.is_null() {
        *crate::stdlib::__errno_location() = 14;
        return 0usize;
    }
    if !(size != 0 && nmemb != 0) {
        // It's not an error, caller just wants zero bytes!
        *crate::stdlib::__errno_location() = 0;
        return 0usize;
    }
    if datasource.is_null() {
        *crate::stdlib::__errno_location() = 9;
        return 0usize;
    }
    // we use a snd_stream_t in the generic pointer to pass around
    stream = datasource as *mut crate::src::client::snd_codec::snd_stream_t;
    // FS_Read does not support multi-byte elements
    byteSize = nmemb.wrapping_mul(size) as i32;
    // read it with the Q3 function FS_Read()
    bytesRead = crate::src::qcommon::files::FS_Read(ptr, byteSize, (*stream).file);
    // update the file position
    (*stream).pos += bytesRead;
    // this function returns the number of elements read not the number of bytes
    nMembRead = (bytesRead as usize).wrapping_div(size);
    // even if the last member is only read partially
    // it is counted as a whole in the return value
    if (bytesRead as usize).wrapping_rem(size) != 0 {
        nMembRead = nMembRead.wrapping_add(1)
    }
    return nMembRead;
}
// fseek() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OGG_Callback_seek(
    mut datasource: *mut libc::c_void,
    mut offset: crate::config_types_h::ogg_int64_t,
    mut whence: i32,
) -> i32 {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    let mut retVal: i32 = 0;
    // check if input is valid
    if datasource.is_null() {
        *crate::stdlib::__errno_location() = 9;
        return -(1i32);
    }
    // snd_stream_t in the generic pointer
    stream = datasource as *mut crate::src::client::snd_codec::snd_stream_t;
    // we must map the whence to its Q3 counterpart
    match whence {
        0 => {
            // set the file position in the actual file with the Q3 function
            retVal = crate::src::qcommon::files::FS_Seek(
                (*stream).file,
                offset,
                crate::src::qcommon::q_shared::FS_SEEK_SET as i32,
            );
            // something has gone wrong, so we return here
            if retVal < 0 {
                return retVal;
            }
            // keep track of file position
            (*stream).pos = offset as i32
        }
        1 => {
            // set the file position in the actual file with the Q3 function
            retVal = crate::src::qcommon::files::FS_Seek(
                (*stream).file,
                offset,
                crate::src::qcommon::q_shared::FS_SEEK_CUR as i32,
            );
            // something has gone wrong, so we return here
            if retVal < 0 {
                return retVal;
            }
            // keep track of file position
            (*stream).pos += offset as i32
        }
        2 => {
            // set the file position in the actual file with the Q3 function
            retVal = crate::src::qcommon::files::FS_Seek(
                (*stream).file,
                offset,
                crate::src::qcommon::q_shared::FS_SEEK_END as i32,
            );
            // something has gone wrong, so we return here
            if retVal < 0 {
                return retVal;
            }
            // keep track of file position
            (*stream).pos = (*stream).length + offset as i32
        }
        _ => {
            // unknown whence, so we return an error
            *crate::stdlib::__errno_location() = 22;
            return -(1i32);
        }
    }
    // stream->pos shouldn't be smaller than zero or bigger than the filesize
    (*stream).pos = if (*stream).pos < 0 { 0 } else { (*stream).pos };
    (*stream).pos = if (*stream).pos > (*stream).length {
        (*stream).length
    } else {
        (*stream).pos
    };
    return 0;
}
// fclose() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OGG_Callback_close(mut _datasource: *mut libc::c_void) -> i32 {
    // we do nothing here and close all things manually in S_OGG_CodecCloseStream()
    return 0;
}
// ftell() replacement
#[no_mangle]

pub unsafe extern "C" fn S_OGG_Callback_tell(mut datasource: *mut libc::c_void) -> isize {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    // check if input is valid
    if datasource.is_null() {
        *crate::stdlib::__errno_location() = 9;
        return -1isize;
    }
    // snd_stream_t in the generic pointer
    stream = datasource as *mut crate::src::client::snd_codec::snd_stream_t;
    return crate::src::qcommon::files::FS_FTell((*stream).file) as isize;
}
// the callback structure
#[no_mangle]

pub static mut S_OGG_Callbacks: crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks = {
    let mut init = crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_callbacks {
        read_func: Some(
            S_OGG_Callback_read
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: crate::stddef_h::size_t,
                    _: crate::stddef_h::size_t,
                    _: *mut libc::c_void,
                ) -> crate::stddef_h::size_t,
        ),
        seek_func: Some(
            S_OGG_Callback_seek
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: crate::config_types_h::ogg_int64_t,
                    _: i32,
                ) -> i32,
        ),
        close_func: Some(S_OGG_Callback_close as unsafe extern "C" fn(_: *mut libc::c_void) -> i32),
        tell_func: Some(S_OGG_Callback_tell as unsafe extern "C" fn(_: *mut libc::c_void) -> isize),
    };
    init
};
/*
=================
S_OGG_CodecOpenStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OGG_CodecOpenStream(
    mut filename: *const i8,
) -> *mut crate::src::client::snd_codec::snd_stream_t {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    // OGG codec control structure
    let mut vf: *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File =
        0 as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File;
    // some variables used to get informations about the OGG
    let mut OGGInfo: *mut crate::codec_h::vorbis_info = 0 as *mut crate::codec_h::vorbis_info;
    let mut numSamples: crate::config_types_h::ogg_int64_t = 0;
    // check if input is valid
    if filename.is_null() {
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // Open the stream
    stream = crate::src::client::snd_codec::S_CodecUtilOpen(filename, &mut ogg_codec);
    if stream.is_null() {
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // alloctate the OggVorbis_File
    vf = crate::src::qcommon::common::Z_Malloc(::std::mem::size_of::<
        crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    >() as i32) as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File;
    if vf.is_null() {
        crate::src::client::snd_codec::S_CodecUtilClose(&mut stream);
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // open the codec with our callbacks and stream as the generic pointer
    if crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_open_callbacks(
        stream as *mut libc::c_void,
        vf,
        0 as *const i8,
        0,
        S_OGG_Callbacks,
    ) != 0
    {
        crate::src::qcommon::common::Z_Free(vf as *mut libc::c_void);
        crate::src::client::snd_codec::S_CodecUtilClose(&mut stream);
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // the stream must be seekable
    if crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_seekable(vf) == 0 {
        crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_clear(vf);
        crate::src::qcommon::common::Z_Free(vf as *mut libc::c_void);
        crate::src::client::snd_codec::S_CodecUtilClose(&mut stream);
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // we only support OGGs with one substream
    if crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_streams(vf) != 1 {
        crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_clear(vf);
        crate::src::qcommon::common::Z_Free(vf as *mut libc::c_void);
        crate::src::client::snd_codec::S_CodecUtilClose(&mut stream);
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // get the info about channels and rate
    OGGInfo = crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_info(vf, 0);
    if OGGInfo.is_null() {
        crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_clear(vf);
        crate::src::qcommon::common::Z_Free(vf as *mut libc::c_void);
        crate::src::client::snd_codec::S_CodecUtilClose(&mut stream);
        return 0 as *mut crate::src::client::snd_codec::snd_stream_t;
    }
    // get the number of sample-frames in the OGG
    numSamples = crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_pcm_total(vf, 0);
    // fill in the info-structure in the stream
    (*stream).info.rate = (*OGGInfo).rate as i32;
    (*stream).info.width = 2;
    (*stream).info.channels = (*OGGInfo).channels;
    (*stream).info.samples = numSamples as i32;
    (*stream).info.size = (*stream).info.samples * (*stream).info.channels * (*stream).info.width;
    (*stream).info.dataofs = 0;
    // We use stream->pos for the file pointer in the compressed ogg file
    (*stream).pos = 0;
    // We use the generic pointer in stream for the OGG codec control structure
    (*stream).ptr = vf as *mut libc::c_void;
    return stream;
}
/*
=================
S_OGG_CodecCloseStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OGG_CodecCloseStream(
    mut stream: *mut crate::src::client::snd_codec::snd_stream_t,
) {
    // check if input is valid
    if stream.is_null() {
        return;
    }
    // let the OGG codec cleanup its stuff
    crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_clear(
        (*stream).ptr as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
    );
    // free the OGG codec control struct
    crate::src::qcommon::common::Z_Free((*stream).ptr);
    // close the stream
    crate::src::client::snd_codec::S_CodecUtilClose(&mut stream);
}
/*
=================
S_OGG_CodecReadStream
=================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OGG_CodecReadStream(
    mut stream: *mut crate::src::client::snd_codec::snd_stream_t,
    mut bytes: i32,
    mut buffer: *mut libc::c_void,
) -> i32 {
    // buffer handling
    let mut bytesRead: i32 = 0;
    let mut bytesLeft: i32 = 0;
    let mut c: i32 = 0;
    let mut bufPtr: *mut i8 = 0 as *mut i8;
    // Bitstream for the decoder
    let mut BS: i32 = 0;
    // big endian machines want their samples in big endian order
    let mut IsBigEndian: i32 = 0;
    // Q3_BIG_ENDIAN
    // check if input is valid
    if !(!stream.is_null() && !buffer.is_null()) {
        return 0i32;
    }
    if bytes <= 0 {
        return 0i32;
    }
    bytesRead = 0;
    bytesLeft = bytes;
    bufPtr = buffer as *mut i8;
    // cycle until we have the requested or all available bytes read
    while -(1) != 0 {
        // read some bytes from the OGG codec
        c = crate::src::libvorbis_1_3_6::lib::vorbisfile::ov_read(
            (*stream).ptr as *mut crate::src::libvorbis_1_3_6::lib::vorbisfile::OggVorbis_File,
            bufPtr,
            bytesLeft,
            IsBigEndian,
            2,
            1,
            &mut BS,
        ) as i32;
        // no more bytes are left
        if c <= 0 {
            break;
        }
        bytesRead += c;
        bytesLeft -= c;
        bufPtr = bufPtr.offset(c as isize);
        // we have enough bytes
        if bytesLeft <= 0 {
            break;
        }
    }
    return bytesRead;
}
// Ogg Vorbis codec
/*
=====================================================================
S_OGG_CodecLoad

We handle S_OGG_CodecLoad as a special case of the streaming functions
where we read the whole stream at once.
======================================================================
*/
#[no_mangle]

pub unsafe extern "C" fn S_OGG_CodecLoad(
    mut filename: *const i8,
    mut info: *mut crate::src::client::snd_codec::snd_info_t,
) -> *mut libc::c_void {
    let mut stream: *mut crate::src::client::snd_codec::snd_stream_t =
        0 as *mut crate::src::client::snd_codec::snd_stream_t;
    let mut buffer: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut bytesRead: i32 = 0;
    // check if input is valid
    if !(!filename.is_null() && !info.is_null()) {
        return 0 as *mut libc::c_void;
    }
    // open the file as a stream
    stream = S_OGG_CodecOpenStream(filename);
    if stream.is_null() {
        return 0 as *mut libc::c_void;
    }
    // copy over the info
    (*info).rate = (*stream).info.rate;
    (*info).width = (*stream).info.width;
    (*info).channels = (*stream).info.channels;
    (*info).samples = (*stream).info.samples;
    (*info).size = (*stream).info.size;
    (*info).dataofs = (*stream).info.dataofs;
    // allocate a buffer
    // this buffer must be free-ed by the caller of this function
    buffer = crate::src::qcommon::common::Hunk_AllocateTempMemory((*info).size)
        as *mut crate::src::qcommon::q_shared::byte;
    if buffer.is_null() {
        S_OGG_CodecCloseStream(stream);
        return 0 as *mut libc::c_void;
    }
    // fill the buffer
    bytesRead = S_OGG_CodecReadStream(stream, (*info).size, buffer as *mut libc::c_void);
    // we don't even have read a single byte
    if bytesRead <= 0 {
        crate::src::qcommon::common::Hunk_FreeTempMemory(buffer as *mut libc::c_void);
        S_OGG_CodecCloseStream(stream);
        return 0 as *mut libc::c_void;
    }
    S_OGG_CodecCloseStream(stream);
    return buffer as *mut libc::c_void;
}
// USE_CODEC_VORBIS
