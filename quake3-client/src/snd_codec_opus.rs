use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __int16_t = libc::c_short;
    pub type __uint32_t = libc::c_uint;
    pub type __int64_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h"]
pub mod stdint_intn_h {
    pub type int16_t = __int16_t;
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int64_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
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
//
    // q_shared.h -- included first by ALL program modules.
// A user mod should never modify this file
    // Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
    // When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
// You shouldn't change this unless you know what you're doing
    // number of supported master servers
    // standard demo extension
    //Ignore __attribute__ on non-gcc platforms
    /* *********************************************************************
  VM Considerations

  The VM can not use the standard system headers because we aren't really
  using the compiler they were meant for.  We use bg_lib.h which contains
  prototypes for the functions we define for our own use in bg_lib.c.

  When writing mods, please add needed headers HERE, do not start including
  stuff like <stdio.h> in the various .c files that make up each of the VMs
  since you will be including system headers files can will have issues.

  Remember, if you use a C library function that is not defined in bg_lib.c,
  you will have to add your own version for support in the VM.

 **********************************************************************/
    //=============================================================
    pub type byte = libc::c_uchar;
    pub type fileHandle_t = libc::c_int;
    pub type unnamed = libc::c_uint;
    pub const FS_SEEK_SET: unnamed = 2;
    pub const FS_SEEK_END: unnamed = 1;
    pub const FS_SEEK_CUR: unnamed = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/usr/include/opus/opus_types.h"]
pub mod opus_types_h {
    /* (C) COPYRIGHT 1994-2002 Xiph.Org Foundation */
/* Modified by Jean-Marc Valin */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
/* opus_types.h based on ogg_types.h from libogg */
    /* *
   @file opus_types.h
   @brief Opus reference implementation types
*/
    /* Use the real stdint.h if it's there (taken from Paul Hsieh's pstdint.h) */
    pub type opus_int16 = int16_t;
    pub type opus_uint32 = uint32_t;
    use super::stdint_intn_h::{int16_t};
    use super::stdint_uintn_h::{uint32_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_codec.h"]
pub mod snd_codec_h {
    /*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.
Copyright (C) 2005 Stuart Dalton (badcdev@gmail.com)

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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snd_info_s {
        pub rate: libc::c_int,
        pub width: libc::c_int,
        pub channels: libc::c_int,
        pub samples: libc::c_int,
        pub size: libc::c_int,
        pub dataofs: libc::c_int,
    }
    pub type snd_info_t = snd_info_s;
    // Codec data structure
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snd_codec_s {
        pub ext: *mut libc::c_char,
        pub load: CODEC_LOAD,
        pub open: CODEC_OPEN,
        pub read: CODEC_READ,
        pub close: CODEC_CLOSE,
        pub next: *mut snd_codec_t,
    }
    pub type snd_codec_t = snd_codec_s;
    pub type CODEC_CLOSE
        =
        Option<unsafe extern "C" fn(_: *mut snd_stream_t) -> ()>;
    pub type snd_stream_t = snd_stream_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct snd_stream_s {
        pub codec: *mut snd_codec_t,
        pub file: fileHandle_t,
        pub info: snd_info_t,
        pub length: libc::c_int,
        pub pos: libc::c_int,
        pub ptr: *mut libc::c_void,
    }
    pub type CODEC_READ
        =
        Option<unsafe extern "C" fn(_: *mut snd_stream_t, _: libc::c_int,
                                    _: *mut libc::c_void) -> libc::c_int>;
    pub type CODEC_OPEN
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char)
                   -> *mut snd_stream_t>;
    // Codec functions
    pub type CODEC_LOAD
        =
        Option<unsafe extern "C" fn(_: *const libc::c_char,
                                    _: *mut snd_info_t) -> *mut libc::c_void>;
    use super::{libc};
    use super::q_shared_h::{fileHandle_t};
    extern "C" {
        // Util functions (used by codecs)
        #[no_mangle]
        pub fn S_CodecUtilOpen(filename: *const libc::c_char,
                               codec: *mut snd_codec_t) -> *mut snd_stream_t;
        #[no_mangle]
        pub fn S_CodecUtilClose(stream: *mut *mut snd_stream_t);
    }
}
#[header_src = "/usr/include/opus/opusfile.h"]
pub mod opusfile_h {
    /*Warning attributes for libopusfile functions.*/
    /* *@endcond*/
    /* *\defgroup error_codes Error Codes*/
/*@{*/
/**\name List of possible error codes
   Many of the functions in this library return a negative error code when a
    function fails.
   This list provides a brief explanation of the common errors.
   See each individual function for more details on what a specific error code
    means in that context.*/
/*@{*/
    /* *A request did not succeed.*/
    /*Currently not used externally.*/
    /* *There was a hole in the page sequence numbers (e.g., a page was corrupt or
    missing).*/
    /* *An underlying read, seek, or tell operation failed when it should have
    succeeded.*/
    /* *A <code>NULL</code> pointer was passed where one was unexpected, or an
    internal memory allocation failed, or an internal library error was
    encountered.*/
    /* *The stream used a feature that is not implemented, such as an unsupported
    channel family.*/
    /* *One or more parameters to a function were invalid.*/
    /* *A purported Ogg Opus stream did not begin with an Ogg page, a purported
    header packet did not start with one of the required strings, "OpusHead" or
    "OpusTags", or a link in a chained file was encountered that did not
    contain any logical Opus streams.*/
    /* *A required header packet was not properly formatted, contained illegal
    values, or was missing altogether.*/
    /* *The ID header contained an unrecognized version number.*/
    /*Currently not used at all.*/
    /* *An audio packet failed to decode properly.
   This is usually caused by a multistream Ogg packet where the durations of
    the individual Opus packets contained in it are not all the same.*/
    /* *We failed to find data we had seen before, or the bitstream structure was
    sufficiently malformed that seeking to the target destination was
    impossible.*/
    /* *An operation that requires seeking was requested on an unseekable stream.*/
    /* *The first or last granule position of a link failed basic validity checks.*/
    /*@}*/
/*@}*/
    /* *\defgroup header_info Header Information*/
/*@{*/
    /* *The maximum number of channels in an Ogg Opus stream.*/
    /* *Ogg Opus bitstream information.
   This contains the basic playback parameters for a stream, and corresponds to
    the initial ID header packet of an Ogg Opus stream.*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct OpusHead {
        pub version: libc::c_int,
        pub channel_count: libc::c_int,
        pub pre_skip: libc::c_uint,
        pub input_sample_rate: opus_uint32,
        pub output_gain: libc::c_int,
        pub mapping_family: libc::c_int,
        pub stream_count: libc::c_int,
        pub coupled_count: libc::c_int,
        pub mapping: [libc::c_uchar; 255],
    }
    /* *The callbacks used to access non-<code>FILE</code> stream resources.
   The function prototypes are basically the same as for the stdio functions
    <code>fread()</code>, <code>fseek()</code>, <code>ftell()</code>, and
    <code>fclose()</code>.
   The differences are that the <code>FILE *</code> arguments have been
    replaced with a <code>void *</code>, which is to be used as a pointer to
    whatever internal data these functions might need, that #seek and #tell
    take and return 64-bit offsets, and that #seek <em>must</em> return -1 if
    the stream is unseekable.*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct OpusFileCallbacks {
        pub read: op_read_func,
        pub seek: op_seek_func,
        pub tell: op_tell_func,
        pub close: op_close_func,
    }
    /* *Closes the underlying stream.
   \retval 0   Success.
   \retval EOF An error occurred.
               <code>errno</code> need not be set.*/
    pub type op_close_func
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>;
    /* *Obtains the current value of the position indicator for \a _stream.
   \return The current position indicator.*/
    pub type op_tell_func
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void)
                   -> libc::c_longlong>;
    /* *Sets the position indicator for \a _stream.
   The new position, measured in bytes, is obtained by adding \a _offset
    bytes to the position specified by \a _whence.
   If \a _whence is set to <code>SEEK_SET</code>, <code>SEEK_CUR</code>, or
    <code>SEEK_END</code>, the offset is relative to the start of the stream,
    the current position indicator, or end-of-file, respectively.
   \retval 0  Success.
   \retval -1 Seeking is not supported or an error occurred.
              <code>errno</code> need not be set.*/
    pub type op_seek_func
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_longlong,
                                    _: libc::c_int) -> libc::c_int>;
    /* *Skip the certificate check when connecting via TLS/SSL (https).
   \param _b <code>opus_int32</code>: Whether or not to skip the certificate
              check.
             The check will be skipped if \a _b is non-zero, and will not be
              skipped if \a _b is zero.
   \hideinitializer*/
    /* *Proxy connections through the given host.
   If no port is specified via #OP_HTTP_PROXY_PORT, the port number defaults
    to 8080 (http-alt).
   All proxy parameters are ignored for non-http and non-https URLs.
   \param _host <code>const char *</code>: The proxy server hostname.
                This may be <code>NULL</code> to disable the use of a proxy
                 server.
   \hideinitializer*/
    /* *Use the given port when proxying connections.
   This option only has an effect if #OP_HTTP_PROXY_HOST is specified with a
    non-<code>NULL</code> \a _host.
   If this option is not provided, the proxy port number defaults to 8080
    (http-alt).
   All proxy parameters are ignored for non-http and non-https URLs.
   \param _port <code>opus_int32</code>: The proxy server port.
                This must be in the range 0...65535 (inclusive), or the
                 URL function this is passed to will fail.
   \hideinitializer*/
    /* *Use the given user name for authentication when proxying connections.
   All proxy parameters are ignored for non-http and non-https URLs.
   \param _user const char *: The proxy server user name.
                              This may be <code>NULL</code> to disable proxy
                               authentication.
                              A non-<code>NULL</code> value only has an effect
                               if #OP_HTTP_PROXY_HOST and #OP_HTTP_PROXY_PASS
                               are also specified with non-<code>NULL</code>
                               arguments.
   \hideinitializer*/
    /* *Use the given password for authentication when proxying connections.
   All proxy parameters are ignored for non-http and non-https URLs.
   \param _pass const char *: The proxy server password.
                              This may be <code>NULL</code> to disable proxy
                               authentication.
                              A non-<code>NULL</code> value only has an effect
                               if #OP_HTTP_PROXY_HOST and #OP_HTTP_PROXY_USER
                               are also specified with non-<code>NULL</code>
                               arguments.
   \hideinitializer*/
    /* *Parse information about the streaming server (if any) and return it.
   Very little validation is done.
   In particular, OpusServerInfo::url may not be a valid URL,
    OpusServerInfo::bitrate_kbps may not really be in kbps, and
    OpusServerInfo::content_type may not be a valid MIME type.
   The character set of the string fields is not specified anywhere, and should
    not be assumed to be valid UTF-8.
   \param _info OpusServerInfo *: Returns information about the server.
                                  If there is any error opening the stream, the
                                   contents of this structure remain
                                   unmodified.
                                  On success, fills in the structure with the
                                   server information that was available, if
                                   any.
                                  After a successful return, the contents of
                                   this structure should be freed by calling
                                   opus_server_info_clear().
   \hideinitializer*/
    /*@}*/
/*@}*/
    /* *\defgroup stream_callbacks Abstract Stream Reading Interface*/
/*@{*/
/**\name Functions for reading from streams
   These functions define the interface used to read from and seek in a stream
    of data.
   A stream does not need to implement seeking, but the decoder will not be
    able to seek if it does not do so.
   These functions also include some convenience routines for working with
    standard <code>FILE</code> pointers, complete streams stored in a single
    block of memory, or URLs.*/
/*@{*/
    /* *Reads up to \a _nbytes bytes of data from \a _stream.
   \param      _stream The stream to read from.
   \param[out] _ptr    The buffer to store the data in.
   \param      _nbytes The maximum number of bytes to read.
                       This function may return fewer, though it will not
                        return zero unless it reaches end-of-file.
   \return The number of bytes successfully read, or a negative value on
            error.*/
    pub type op_read_func
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                    _: *mut libc::c_uchar, _: libc::c_int)
                   -> libc::c_int>;
    use super::{libc};
    use super::opus_types_h::{opus_uint32, opus_int16};
    use super::config_types_h::{ogg_int64_t};
    use super::stddef_h::{size_t};
    extern "C" {
        pub type OggOpusFile;
        /* *Release all memory used by an \c OggOpusFile.
   \param _of The \c OggOpusFile to free.*/
        #[no_mangle]
        pub fn op_free(_of: *mut OggOpusFile);
        /* *Reads more samples from the stream.
   \note Although \a _buf_size must indicate the total number of values that
    can be stored in \a _pcm, the return value is the number of samples
    <em>per channel</em>.
   This is done because
   <ol>
   <li>The channel count cannot be known a priori (reading more samples might
        advance us into the next link, with a different channel count), so
        \a _buf_size cannot also be in units of samples per channel,</li>
   <li>Returning the samples per channel matches the <code>libopus</code> API
        as closely as we're able,</li>
   <li>Returning the total number of values instead of samples per channel
        would mean the caller would need a division to compute the samples per
        channel, and might worry about the possibility of getting back samples
        for some channels and not others, and</li>
   <li>This approach is relatively fool-proof: if an application passes too
        small a value to \a _buf_size, they will simply get fewer samples back,
        and if they assume the return value is the total number of values, then
        they will simply read too few (rather than reading too many and going
        off the end of the buffer).</li>
   </ol>
   \param      _of       The \c OggOpusFile from which to read.
   \param[out] _pcm      A buffer in which to store the output PCM samples, as
                          signed native-endian 16-bit values at 48&nbsp;kHz
                          with a nominal range of <code>[-32768,32767)</code>.
                         Multiple channels are interleaved using the
                          <a href="http://www.xiph.org/vorbis/doc/Vorbis_I_spec.html#x1-800004.3.9">Vorbis
                          channel ordering</a>.
                         This must have room for at least \a _buf_size values.
   \param      _buf_size The number of values that can be stored in \a _pcm.
                         It is recommended that this be large enough for at
                          least 120 ms of data at 48 kHz per channel (5760
                          values per channel).
                         Smaller buffers will simply return less data, possibly
                          consuming more memory to buffer the data internally.
                         <tt>libopusfile</tt> may return less data than
                          requested.
                         If so, there is no guarantee that the remaining data
                          in \a _pcm will be unmodified.
   \param[out] _li       The index of the link this data was decoded from.
                         You may pass <code>NULL</code> if you do not need this
                          information.
                         If this function fails (returning a negative value),
                          this parameter is left unset.
   \return The number of samples read per channel on success, or a negative
            value on failure.
           The channel count can be retrieved on success by calling
            <code>op_head(_of,*_li)</code>.
           The number of samples returned may be 0 if the buffer was too small
            to store even a single sample for all channels, or if end-of-file
            was reached.
           The list of possible failure codes follows.
           Most of them can only be returned by unseekable, chained streams
            that encounter a new link.
   \retval #OP_HOLE          There was a hole in the data, and some samples
                              may have been skipped.
                             Call this function again to continue decoding
                              past the hole.
   \retval #OP_EREAD         An underlying read operation failed.
                             This may signal a truncation attack from an
                              <https:> source.
   \retval #OP_EFAULT        An internal memory allocation failed.
   \retval #OP_EIMPL         An unseekable stream encountered a new link that
                              used a feature that is not implemented, such as
                              an unsupported channel family.
   \retval #OP_EINVAL        The stream was only partially open.
   \retval #OP_ENOTFORMAT    An unseekable stream encountered a new link that
                              did not have any logical Opus streams in it.
   \retval #OP_EBADHEADER    An unseekable stream encountered a new link with a
                              required header packet that was not properly
                              formatted, contained illegal values, or was
                              missing altogether.
   \retval #OP_EVERSION      An unseekable stream encountered a new link with
                              an ID header that contained an unrecognized
                              version number.
   \retval #OP_EBADPACKET    Failed to properly decode the next packet.
   \retval #OP_EBADLINK      We failed to find data we had seen before.
   \retval #OP_EBADTIMESTAMP An unseekable stream encountered a new link with
                              a starting timestamp that failed basic validity
                              checks.*/
        #[no_mangle]
        pub fn op_read(_of: *mut OggOpusFile, _pcm: *mut opus_int16,
                       _buf_size: libc::c_int, _li: *mut libc::c_int)
         -> libc::c_int;
        /* *Get the total PCM length (number of samples at 48 kHz) of the stream, or of
    an individual link in a (possibly-chained) Ogg Opus stream.
   Users looking for <code>op_time_total()</code> should use op_pcm_total()
    instead.
   Because timestamps in Opus are fixed at 48 kHz, there is no need for a
    separate function to convert this to seconds (and leaving it out avoids
    introducing floating point to the API, for those that wish to avoid it).
   \param _of The \c OggOpusFile from which to retrieve the PCM offset.
   \param _li The index of the link whose PCM length should be computed.
              Use a negative number to get the PCM length of the entire stream.
   \return The PCM length of the entire stream if \a _li is negative, the PCM
            length of link \a _li if it is non-negative, or a negative value on
            error.
   \retval #OP_EINVAL The stream is not seekable (so we can't know the length),
                       \a _li wasn't less than the total number of links in
                       the stream, or the stream was only partially open.*/
        #[no_mangle]
        pub fn op_pcm_total(_of: *const OggOpusFile, _li: libc::c_int)
         -> ogg_int64_t;
        /* *Get the ID header information for the given link in a (possibly chained) Ogg
    Opus stream.
   This function may be called on partially-opened streams, but it will always
    return the ID header information of the Opus stream in the first link.
   \param _of The \c OggOpusFile from which to retrieve the ID header
               information.
   \param _li The index of the link whose ID header information should be
               retrieved.
              Use a negative number to get the ID header information of the
               current link.
              For an unseekable stream, \a _li is ignored, and the ID header
               information for the current link is always returned, if
               available.
   \return The contents of the ID header for the given link.*/
        #[no_mangle]
        pub fn op_head(_of: *const OggOpusFile, _li: libc::c_int)
         -> *const OpusHead;
        /*@}*/
/*@}*/
        /* *\defgroup stream_info Stream Information*/
/*@{*/
/**\name Functions for obtaining information about streams

   These functions allow you to get basic information about a stream, including
    seekability, the number of links (for chained streams), plus the size,
    duration, bitrate, header parameters, and meta information for each link
    (or, where available, the stream as a whole).
   Some of these (size, duration) are only available for seekable streams.
   You can also query the current stream position, link, and playback time,
    and instantaneous bitrate during playback.

   Some of these functions may be used successfully on the partially open
    streams returned by op_test_callbacks() or one of the associated
    convenience functions.
   Their documention will indicate so explicitly.*/
/*@{*/
        /* *Returns whether or not the stream being read is seekable.
   This is true if
   <ol>
   <li>The <code><a href="#op_seek_func">seek()</a></code> and
    <code><a href="#op_tell_func">tell()</a></code> callbacks are both
    non-<code>NULL</code>,</li>
   <li>The <code><a href="#op_seek_func">seek()</a></code> callback was
    successfully executed at least once, and</li>
   <li>The <code><a href="#op_tell_func">tell()</a></code> callback was
    successfully able to report the position indicator afterwards.</li>
   </ol>
   This function may be called on partially-opened streams.
   \param _of The \c OggOpusFile whose seekable status is to be returned.
   \return A non-zero value if seekable, and 0 if unseekable.*/
        #[no_mangle]
        pub fn op_seekable(_of: *const OggOpusFile) -> libc::c_int;
        /* *Open a stream using the given set of callbacks to access it.
   \param _stream        The stream to read from (e.g., a <code>FILE *</code>).
                         This value will be passed verbatim as the first
                          argument to all of the callbacks.
   \param _cb            The callbacks with which to access the stream.
                         <code><a href="#op_read_func">read()</a></code> must
                          be implemented.
                         <code><a href="#op_seek_func">seek()</a></code> and
                          <code><a href="#op_tell_func">tell()</a></code> may
                          be <code>NULL</code>, or may always return -1 to
                          indicate a stream is unseekable, but if
                          <code><a href="#op_seek_func">seek()</a></code> is
                          implemented and succeeds on a particular stream, then
                          <code><a href="#op_tell_func">tell()</a></code> must
                          also.
                         <code><a href="#op_close_func">close()</a></code> may
                          be <code>NULL</code>, but if it is not, it will be
                          called when the \c OggOpusFile is destroyed by
                          op_free().
                         It will not be called if op_open_callbacks() fails
                          with an error.
   \param _initial_data  An initial buffer of data from the start of the
                          stream.
                         Applications can read some number of bytes from the
                          start of the stream to help identify this as an Opus
                          stream, and then provide them here to allow the
                          stream to be opened, even if it is unseekable.
   \param _initial_bytes The number of bytes in \a _initial_data.
                         If the stream is seekable, its current position (as
                          reported by
                          <code><a href="#opus_tell_func">tell()</a></code>
                          at the start of this function) must be equal to
                          \a _initial_bytes.
                         Otherwise, seeking to absolute positions will
                          generate inconsistent results.
   \param[out] _error    Returns 0 on success, or a failure code on error.
                         You may pass in <code>NULL</code> if you don't want
                          the failure code.
                         The failure code will be one of
                         <dl>
                           <dt>#OP_EREAD</dt>
                           <dd>An underlying read, seek, or tell operation
                            failed when it should have succeeded, or we failed
                            to find data in the stream we had seen before.</dd>
                           <dt>#OP_EFAULT</dt>
                           <dd>There was a memory allocation failure, or an
                            internal library error.</dd>
                           <dt>#OP_EIMPL</dt>
                           <dd>The stream used a feature that is not
                            implemented, such as an unsupported channel
                            family.</dd>
                           <dt>#OP_EINVAL</dt>
                           <dd><code><a href="#op_seek_func">seek()</a></code>
                            was implemented and succeeded on this source, but
                            <code><a href="#op_tell_func">tell()</a></code>
                            did not, or the starting position indicator was
                            not equal to \a _initial_bytes.</dd>
                           <dt>#OP_ENOTFORMAT</dt>
                           <dd>The stream contained a link that did not have
                            any logical Opus streams in it.</dd>
                           <dt>#OP_EBADHEADER</dt>
                           <dd>A required header packet was not properly
                            formatted, contained illegal values, or was missing
                            altogether.</dd>
                           <dt>#OP_EVERSION</dt>
                           <dd>An ID header contained an unrecognized version
                            number.</dd>
                           <dt>#OP_EBADLINK</dt>
                           <dd>We failed to find data we had seen before after
                            seeking.</dd>
                           <dt>#OP_EBADTIMESTAMP</dt>
                           <dd>The first or last timestamp in a link failed
                            basic validity checks.</dd>
                         </dl>
   \return A freshly opened \c OggOpusFile, or <code>NULL</code> on error.
           <tt>libopusfile</tt> does <em>not</em> take ownership of the stream
            if the call fails.
           The calling application is responsible for closing the stream if
            this call returns an error.*/
        #[no_mangle]
        pub fn op_open_callbacks(_stream: *mut libc::c_void,
                                 _cb: *const OpusFileCallbacks,
                                 _initial_data: *const libc::c_uchar,
                                 _initial_bytes: size_t,
                                 _error: *mut libc::c_int)
         -> *mut OggOpusFile;
    }
}
#[header_src = "/usr/include/ogg/config_types.h"]
pub mod config_types_h {
    pub type ogg_int64_t = int64_t;
    use super::stdint_intn_h::{int64_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    use super::q_shared_h::{fileHandle_t};
    extern "C" {
        #[no_mangle]
        pub fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int,
                       f: fileHandle_t) -> libc::c_int;
        // doesn't work for files that are opened from a pack file
        #[no_mangle]
        pub fn FS_FTell(f: fileHandle_t) -> libc::c_int;
        // opens a file for reading, writing, or appending depending on the value of mode
        #[no_mangle]
        pub fn FS_Seek(f: fileHandle_t, offset: libc::c_long,
                       origin: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Hunk_AllocateTempMemory(size: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Hunk_FreeTempMemory(buf: *mut libc::c_void);
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/client/snd_codec_opus.c"]
pub mod snd_codec_opus_c {
    use super::opusfile_h::{OpusFileCallbacks};
    use super::{libc};
}
#[header_src = "/usr/include/errno.h"]
pub mod errno_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
use self::types_h::{__int16_t, __uint32_t, __int64_t};
use self::stddef_h::{size_t};
use self::stdint_intn_h::{int16_t, int64_t};
use self::stdint_uintn_h::{uint32_t};
use self::q_shared_h::{byte, fileHandle_t, unnamed, FS_SEEK_SET, FS_SEEK_END,
                       FS_SEEK_CUR, Com_Printf};
use self::opus_types_h::{opus_int16, opus_uint32};
use self::snd_codec_h::{snd_info_s, snd_info_t, snd_codec_s, snd_codec_t,
                        CODEC_CLOSE, snd_stream_t, snd_stream_s, CODEC_READ,
                        CODEC_OPEN, CODEC_LOAD, S_CodecUtilOpen,
                        S_CodecUtilClose};
use self::opusfile_h::{OpusHead, OpusFileCallbacks, op_close_func,
                       op_tell_func, op_seek_func, op_read_func, OggOpusFile,
                       op_free, op_read, op_pcm_total, op_head, op_seekable,
                       op_open_callbacks};
use self::config_types_h::{ogg_int64_t};
use self::qcommon_h::{FS_Read, FS_FTell, FS_Seek, Hunk_AllocateTempMemory,
                      Hunk_FreeTempMemory};
use self::errno_h::{__errno_location};
// USE_CODEC_VORBIS
// Ogg Opus codec
#[no_mangle]
pub static mut opus_codec: snd_codec_t =
    snd_codec_s{ext:
                    b"opus\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char,
                load: Some(S_OggOpus_CodecLoad),
                open: Some(S_OggOpus_CodecOpenStream),
                read: Some(S_OggOpus_CodecReadStream),
                close: Some(S_OggOpus_CodecCloseStream),
                next: 0 as *const snd_codec_t as *mut snd_codec_t,};
#[no_mangle]
pub unsafe extern "C" fn S_OggOpus_CodecCloseStream(mut stream:
                                                        *mut snd_stream_t) {
    if stream.is_null() { return }
    op_free((*stream).ptr as *mut OggOpusFile);
    S_CodecUtilClose(&mut stream);
}
#[no_mangle]
pub unsafe extern "C" fn S_OggOpus_CodecReadStream(mut stream:
                                                       *mut snd_stream_t,
                                                   mut bytes: libc::c_int,
                                                   mut buffer:
                                                       *mut libc::c_void)
 -> libc::c_int {
    // buffer handling
    let mut samplesRead: libc::c_int = 0;
    let mut samplesLeft: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut bufPtr: *mut opus_int16 = 0 as *mut opus_int16;
    if !(!stream.is_null() && !buffer.is_null()) { return 0i32 }
    if bytes <= 0i32 { return 0i32 }
    samplesRead = 0i32;
    samplesLeft = bytes / (*stream).info.channels / (*stream).info.width;
    bufPtr = buffer as *mut opus_int16;
    if samplesLeft <= 0i32 { return 0i32 }
    while 0 != -1i32 {
        c =
            op_read((*stream).ptr as *mut OggOpusFile,
                    bufPtr.offset((samplesRead * (*stream).info.channels) as
                                      isize),
                    samplesLeft * (*stream).info.channels,
                    0 as *mut libc::c_int);
        // no more samples are left
        if c <= 0i32 { break ; }
        samplesRead += c;
        samplesLeft -= c;
        // we have enough samples
        if samplesLeft <= 0i32 { break ; }
    }
    return samplesRead * (*stream).info.channels * (*stream).info.width;
}
#[no_mangle]
pub unsafe extern "C" fn S_OggOpus_CodecOpenStream(mut filename:
                                                       *const libc::c_char)
 -> *mut snd_stream_t {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    // Opus codec control structure
    let mut of: *mut OggOpusFile = 0 as *mut OggOpusFile;
    // some variables used to get informations about the file
    let mut opusInfo: *const OpusHead = 0 as *const OpusHead;
    let mut numSamples: ogg_int64_t = 0;
    if filename.is_null() { return 0 as *mut snd_stream_t }
    stream = S_CodecUtilOpen(filename, &mut opus_codec);
    if stream.is_null() { return 0 as *mut snd_stream_t }
    of =
        op_open_callbacks(stream as *mut libc::c_void, &S_OggOpus_Callbacks,
                          0 as *const libc::c_uchar, 0i32 as size_t,
                          0 as *mut libc::c_int);
    if of.is_null() {
        S_CodecUtilClose(&mut stream);
        return 0 as *mut snd_stream_t
    }
    if 0 == op_seekable(of) {
        op_free(of);
        S_CodecUtilClose(&mut stream);
        return 0 as *mut snd_stream_t
    }
    opusInfo = op_head(of, -1i32);
    if opusInfo.is_null() {
        op_free(of);
        S_CodecUtilClose(&mut stream);
        return 0 as *mut snd_stream_t
    }
    if (*opusInfo).stream_count != 1i32 {
        op_free(of);
        S_CodecUtilClose(&mut stream);
        Com_Printf(b"Only Ogg Opus files with one stream are support\n\x00" as
                       *const u8 as *const libc::c_char);
        return 0 as *mut snd_stream_t
    }
    if (*opusInfo).channel_count != 1i32 && (*opusInfo).channel_count != 2i32
       {
        op_free(of);
        S_CodecUtilClose(&mut stream);
        Com_Printf(b"Only mono and stereo Ogg Opus files are supported\n\x00"
                       as *const u8 as *const libc::c_char);
        return 0 as *mut snd_stream_t
    }
    numSamples = op_pcm_total(of, -1i32);
    (*stream).info.rate = 48000i32;
    (*stream).info.width = 2i32;
    (*stream).info.channels = (*opusInfo).channel_count;
    (*stream).info.samples = numSamples as libc::c_int;
    (*stream).info.size =
        (*stream).info.samples * (*stream).info.channels *
            (*stream).info.width;
    (*stream).info.dataofs = 0i32;
    (*stream).pos = 0i32;
    (*stream).ptr = of as *mut libc::c_void;
    return stream;
}
// the callback structure
#[no_mangle]
pub static mut S_OggOpus_Callbacks: OpusFileCallbacks =
    OpusFileCallbacks{read: Some(S_OggOpus_Callback_read),
                      seek: Some(S_OggOpus_Callback_seek),
                      tell: Some(S_OggOpus_Callback_tell),
                      close: Some(S_OggOpus_Callback_close),};
// fclose() replacement
#[no_mangle]
pub unsafe extern "C" fn S_OggOpus_Callback_close(mut datasource:
                                                      *mut libc::c_void)
 -> libc::c_int {
    return 0i32;
}
// ftell() replacement
#[no_mangle]
pub unsafe extern "C" fn S_OggOpus_Callback_tell(mut datasource:
                                                     *mut libc::c_void)
 -> libc::c_longlong {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    if datasource.is_null() {
        *__errno_location() = 9i32;
        return -1i32 as libc::c_longlong
    }
    stream = datasource as *mut snd_stream_t;
    return FS_FTell((*stream).file) as libc::c_longlong;
}
// fseek() replacement
#[no_mangle]
pub unsafe extern "C" fn S_OggOpus_Callback_seek(mut datasource:
                                                     *mut libc::c_void,
                                                 mut offset: libc::c_longlong,
                                                 mut whence: libc::c_int)
 -> libc::c_int {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    let mut retVal: libc::c_int = 0i32;
    if datasource.is_null() { *__errno_location() = 9i32; return -1i32 }
    stream = datasource as *mut snd_stream_t;
    match whence {
        0 => {
            retVal =
                FS_Seek((*stream).file, offset as libc::c_long,
                        FS_SEEK_SET as libc::c_int);
            if retVal < 0i32 { return retVal }
            (*stream).pos = offset as libc::c_int
        }
        1 => {
            retVal =
                FS_Seek((*stream).file, offset as libc::c_long,
                        FS_SEEK_CUR as libc::c_int);
            if retVal < 0i32 { return retVal }
            (*stream).pos += offset as libc::c_int
        }
        2 => {
            retVal =
                FS_Seek((*stream).file, offset as libc::c_long,
                        FS_SEEK_END as libc::c_int);
            if retVal < 0i32 { return retVal }
            (*stream).pos = (*stream).length + offset as libc::c_int
        }
        _ => { *__errno_location() = 22i32; return -1i32 }
    }
    (*stream).pos = if (*stream).pos < 0i32 { 0i32 } else { (*stream).pos };
    (*stream).pos =
        if (*stream).pos > (*stream).length {
            (*stream).length
        } else { (*stream).pos };
    return 0i32;
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.
Copyright (C) 2005 Stuart Dalton (badcdev@gmail.com)
Copyright (C) 2005-2006 Joerg Dietrich <dietrich_joerg@gmx.de>

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
// Ogg Opus support is enabled by this define
// includes for the Q3 sound system
// includes for the Ogg Opus codec
// samples are 16 bit
// Q3 Ogg Opus codec
// callbacks for opusfile
// fread() replacement
#[no_mangle]
pub unsafe extern "C" fn S_OggOpus_Callback_read(mut datasource:
                                                     *mut libc::c_void,
                                                 mut ptr: *mut libc::c_uchar,
                                                 mut size: libc::c_int)
 -> libc::c_int {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    let mut bytesRead: libc::c_int = 0i32;
    if ptr.is_null() { *__errno_location() = 14i32; return -1i32 }
    if 0 == size { *__errno_location() = 0i32; return 0i32 }
    if size < 0i32 { *__errno_location() = 22i32; return -1i32 }
    if datasource.is_null() { *__errno_location() = 9i32; return -1i32 }
    stream = datasource as *mut snd_stream_t;
    bytesRead = FS_Read(ptr as *mut libc::c_void, size, (*stream).file);
    (*stream).pos += bytesRead;
    return bytesRead;
}
#[no_mangle]
pub unsafe extern "C" fn S_OggOpus_CodecLoad(mut filename:
                                                 *const libc::c_char,
                                             mut info: *mut snd_info_t)
 -> *mut libc::c_void {
    let mut stream: *mut snd_stream_t = 0 as *mut snd_stream_t;
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut bytesRead: libc::c_int = 0;
    if !(!filename.is_null() && !info.is_null()) {
        return 0 as *mut libc::c_void
    }
    stream = S_OggOpus_CodecOpenStream(filename);
    if stream.is_null() { return 0 as *mut libc::c_void }
    (*info).rate = (*stream).info.rate;
    (*info).width = (*stream).info.width;
    (*info).channels = (*stream).info.channels;
    (*info).samples = (*stream).info.samples;
    (*info).size = (*stream).info.size;
    (*info).dataofs = (*stream).info.dataofs;
    buffer = Hunk_AllocateTempMemory((*info).size) as *mut byte;
    if buffer.is_null() {
        S_OggOpus_CodecCloseStream(stream);
        return 0 as *mut libc::c_void
    }
    bytesRead =
        S_OggOpus_CodecReadStream(stream, (*info).size,
                                  buffer as *mut libc::c_void);
    if bytesRead <= 0i32 {
        Hunk_FreeTempMemory(buffer as *mut libc::c_void);
        S_OggOpus_CodecCloseStream(stream);
        return 0 as *mut libc::c_void
    }
    S_OggOpus_CodecCloseStream(stream);
    return buffer as *mut libc::c_void;
}