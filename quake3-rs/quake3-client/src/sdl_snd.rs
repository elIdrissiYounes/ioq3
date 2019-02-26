#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
extern crate libc;
#[header_src = "/usr/lib/clang/7.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __int16_t = libc::c_short;
    pub type __uint16_t = libc::c_ushort;
    pub type __uint32_t = libc::c_uint;
    use super::{libc};
}
#[header_src = "/usr/include/bits/stdint-intn.h"]
pub mod stdint_intn_h {
    pub type int16_t = __int16_t;
    use super::types_h::{__int16_t};
}
#[header_src = "/usr/include/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
}
#[header_src = "/usr/include/SDL2/SDL_stdinc.h"]
pub mod SDL_stdinc_h {
    /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
    /* *
 *  \file SDL_stdinc.h
 *
 *  This is a general header that includes C language support.
 */
    /* *
 *  The number of elements in an array.
 */
    /* *
 *  Macro useful for building other macros with strings in them
 *
 *  e.g. #define LOG_ERROR(X) OutputDebugString(SDL_STRINGIFY_ARG(__FUNCTION__) ": " X "\n")
 */
    /* *
 *  \name Cast operators
 *
 *  Use proper C++ casts when compiled as C++ to be compatible with the option
 *  -Wold-style-cast of GCC (and -Werror=old-style-cast in GCC 4.2 and above).
 */
/* @{ */
    /* @} */
    /* Cast operators */
    /* Define a four character code as a Uint32 */
    /* *
 *  \name Basic data types
 */
/* @{ */
    pub type unnamed = libc::c_uint;
    pub const SDL_TRUE: unnamed = 1;
    pub const SDL_FALSE: unnamed = 0;
    /* *
 * \brief An unsigned 8-bit integer type.
 */
    /* 255 */
    /* 0 */
    pub type Uint8 = uint8_t;
    /* *
 * \brief A signed 16-bit integer type.
 */
    /* 32767 */
    /* -32768 */
    pub type Sint16 = int16_t;
    /* *
 * \brief An unsigned 16-bit integer type.
 */
    /* 65535 */
    /* 0 */
    pub type Uint16 = uint16_t;
    /* *
 * \brief An unsigned 32-bit integer type.
 */
    /* 4294967295 */
    /* 0 */
    pub type Uint32 = uint32_t;
    use super::{libc};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::stdint_intn_h::{int16_t};
    use super::stddef_h::{size_t};
    extern "C" {
        #[no_mangle]
        pub fn SDL_memset(dst: *mut libc::c_void, c: libc::c_int, len: size_t)
         -> *mut libc::c_void;
    }
}
#[header_src = "/usr/include/SDL2/SDL_audio.h"]
pub mod SDL_audio_h {
    /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
    /* *
 *  \file SDL_audio.h
 *
 *  Access to the raw audio mixing buffer for the SDL library.
 */
    /* Set up for C function definitions, even when using C++ */
    /* *
 *  \brief Audio format flags.
 *
 *  These are what the 16 bits in SDL_AudioFormat currently mean...
 *  (Unspecified bits are always zero).
 *
 *  \verbatim
    ++-----------------------sample is signed if set
    ||
    ||       ++-----------sample is bigendian if set
    ||       ||
    ||       ||          ++---sample is float if set
    ||       ||          ||
    ||       ||          || +---sample bit size---+
    ||       ||          || |                     |
    15 14 13 12 11 10 09 08 07 06 05 04 03 02 01 00
    \endverbatim
 *
 *  There are macros in SDL 2.0 and later to query these bits.
 */
    pub type SDL_AudioFormat = Uint16;
    /* *
 *  \name Audio flags
 */
/* @{ */
    /* *
 *  \name Audio format flags
 *
 *  Defaults to LSB byte order.
 */
/* @{ */
    /* *< Unsigned 8-bit samples */
    /* *< Signed 8-bit samples */
    /* *< Unsigned 16-bit samples */
    /* *< Signed 16-bit samples */
    /* *< As above, but big-endian byte order */
    /* *< As above, but big-endian byte order */
    /* @} */
    /* *
 *  \name int32 support
 */
/* @{ */
    /* *< 32-bit integer samples */
    /* *< As above, but big-endian byte order */
    /* @} */
    /* *
 *  \name float32 support
 */
/* @{ */
    /* *< 32-bit floating point samples */
    /* *< As above, but big-endian byte order */
    /* @} */
    /* *
 *  \name Native audio byte ordering
 */
/* @{ */
    /* @} */
    /* *
 *  \name Allow change flags
 *
 *  Which audio format changes are allowed when opening a device.
 */
/* @{ */
    /* @} */
    /* @} */
    /* Audio flags */
    /* *
 *  This function is called when the audio device needs more data.
 *
 *  \param userdata An application-specific parameter saved in
 *                  the SDL_AudioSpec structure
 *  \param stream A pointer to the audio data buffer.
 *  \param len    The length of that buffer in bytes.
 *
 *  Once the callback returns, the buffer will no longer be valid.
 *  Stereo samples are stored in a LRLRLR ordering.
 *
 *  You can choose to avoid callbacks and use SDL_QueueAudio() instead, if
 *  you like. Just open your audio device with a NULL callback.
 */
    pub type SDL_AudioCallback
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut Uint8,
                                    _: libc::c_int) -> ()>;
    /* *
 *  The calculated values in this structure are calculated by SDL_OpenAudio().
 *
 *  For multi-channel audio, the default SDL channel mapping is:
 *  2:  FL FR                       (stereo)
 *  3:  FL FR LFE                   (2.1 surround)
 *  4:  FL FR BL BR                 (quad)
 *  5:  FL FR FC BL BR              (quad + center)
 *  6:  FL FR FC LFE SL SR          (5.1 surround - last two can also be BL BR)
 *  7:  FL FR FC LFE BC SL SR       (6.1 surround)
 *  8:  FL FR FC LFE BL BR SL SR    (7.1 surround)
 */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct SDL_AudioSpec {
        pub freq: libc::c_int,
        pub format: SDL_AudioFormat,
        pub channels: Uint8,
        pub silence: Uint8,
        pub samples: Uint16,
        pub padding: Uint16,
        pub size: Uint32,
        pub callback: SDL_AudioCallback,
        pub userdata: *mut libc::c_void,
    }
    /* *
 *  SDL Audio Device IDs.
 *
 *  A successful call to SDL_OpenAudio() is always device id 1, and legacy
 *  SDL audio APIs assume you want this device ID. SDL_OpenAudioDevice() calls
 *  always returns devices >= 2 on success. The legacy calls are good both
 *  for backwards compatibility and when you don't care about multiple,
 *  specific, or capture devices.
 */
    pub type SDL_AudioDeviceID = Uint32;
    use super::SDL_stdinc_h::{Uint16, Uint8, Uint32};
    use super::{libc};
    extern "C" {
        /* @} */
        /* *
 *  This function returns the name of the current audio driver, or NULL
 *  if no driver has been initialized.
 */
        #[no_mangle]
        pub fn SDL_GetCurrentAudioDriver() -> *const libc::c_char;
        /* *
 *  Open a specific audio device. Passing in a device name of NULL requests
 *  the most reasonable default (and is equivalent to calling SDL_OpenAudio()).
 *
 *  The device name is a UTF-8 string reported by SDL_GetAudioDeviceName(), but
 *  some drivers allow arbitrary and driver-specific strings, such as a
 *  hostname/IP address for a remote audio server, or a filename in the
 *  diskaudio driver.
 *
 *  \return 0 on error, a valid device ID that is >= 2 on success.
 *
 *  SDL_OpenAudio(), unlike this function, always acts on device ID 1.
 */
        #[no_mangle]
        pub fn SDL_OpenAudioDevice(device: *const libc::c_char,
                                   iscapture: libc::c_int,
                                   desired: *const SDL_AudioSpec,
                                   obtained: *mut SDL_AudioSpec,
                                   allowed_changes: libc::c_int)
         -> SDL_AudioDeviceID;
        #[no_mangle]
        pub fn SDL_PauseAudioDevice(dev: SDL_AudioDeviceID,
                                    pause_on: libc::c_int);
        /* *
 *  Dequeue more audio on non-callback devices.
 *
 *  (If you are looking to queue audio for output on a non-callback playback
 *  device, you want SDL_QueueAudio() instead. This will always return 0
 *  if you use it with playback devices.)
 *
 *  SDL offers two ways to retrieve audio from a capture device: you can
 *  either supply a callback that SDL triggers with some frequency as the
 *  device records more audio data, (push method), or you can supply no
 *  callback, and then SDL will expect you to retrieve data at regular
 *  intervals (pull method) with this function.
 *
 *  There are no limits on the amount of data you can queue, short of
 *  exhaustion of address space. Data from the device will keep queuing as
 *  necessary without further intervention from you. This means you will
 *  eventually run out of memory if you aren't routinely dequeueing data.
 *
 *  Capture devices will not queue data when paused; if you are expecting
 *  to not need captured audio for some length of time, use
 *  SDL_PauseAudioDevice() to stop the capture device from queueing more
 *  data. This can be useful during, say, level loading times. When
 *  unpaused, capture devices will start queueing data from that point,
 *  having flushed any capturable data available while paused.
 *
 *  This function is thread-safe, but dequeueing from the same device from
 *  two threads at once does not promise which thread will dequeued data
 *  first.
 *
 *  You may not dequeue audio from a device that is using an
 *  application-supplied callback; doing so returns an error. You have to use
 *  the audio callback, or dequeue audio with this function, but not both.
 *
 *  You should not call SDL_LockAudio() on the device before queueing; SDL
 *  handles locking internally for this function.
 *
 *  \param dev The device ID from which we will dequeue audio.
 *  \param data A pointer into where audio data should be copied.
 *  \param len The number of bytes (not samples!) to which (data) points.
 *  \return number of bytes dequeued, which could be less than requested.
 *
 *  \sa SDL_GetQueuedAudioSize
 *  \sa SDL_ClearQueuedAudio
 */
        #[no_mangle]
        pub fn SDL_DequeueAudio(dev: SDL_AudioDeviceID,
                                data: *mut libc::c_void, len: Uint32)
         -> Uint32;
        /* *
 *  Get the number of bytes of still-queued audio.
 *
 *  For playback device:
 *
 *    This is the number of bytes that have been queued for playback with
 *    SDL_QueueAudio(), but have not yet been sent to the hardware. This
 *    number may shrink at any time, so this only informs of pending data.
 *
 *    Once we've sent it to the hardware, this function can not decide the
 *    exact byte boundary of what has been played. It's possible that we just
 *    gave the hardware several kilobytes right before you called this
 *    function, but it hasn't played any of it yet, or maybe half of it, etc.
 *
 *  For capture devices:
 *
 *    This is the number of bytes that have been captured by the device and
 *    are waiting for you to dequeue. This number may grow at any time, so
 *    this only informs of the lower-bound of available data.
 *
 *  You may not queue audio on a device that is using an application-supplied
 *  callback; calling this function on such a device always returns 0.
 *  You have to queue audio with SDL_QueueAudio()/SDL_DequeueAudio(), or use
 *  the audio callback, but not both.
 *
 *  You should not call SDL_LockAudio() on the device before querying; SDL
 *  handles locking internally for this function.
 *
 *  \param dev The device ID of which we will query queued audio size.
 *  \return Number of bytes (not samples!) of queued audio.
 *
 *  \sa SDL_QueueAudio
 *  \sa SDL_ClearQueuedAudio
 */
        #[no_mangle]
        pub fn SDL_GetQueuedAudioSize(dev: SDL_AudioDeviceID) -> Uint32;
        /* *
 *  Drop any queued audio data. For playback devices, this is any queued data
 *  still waiting to be submitted to the hardware. For capture devices, this
 *  is any data that was queued by the device that hasn't yet been dequeued by
 *  the application.
 *
 *  Immediately after this call, SDL_GetQueuedAudioSize() will return 0. For
 *  playback devices, the hardware will start playing silence if more audio
 *  isn't queued. Unpaused capture devices will start filling the queue again
 *  as soon as they have more data available (which, depending on the state
 *  of the hardware and the thread, could be before this function call
 *  returns!).
 *
 *  This will not prevent playback of queued audio that's already been sent
 *  to the hardware, as we can not undo that, so expect there to be some
 *  fraction of a second of audio that might still be heard. This can be
 *  useful if you want to, say, drop any pending music during a level change
 *  in your game.
 *
 *  You may not queue audio on a device that is using an application-supplied
 *  callback; calling this function on such a device is always a no-op.
 *  You have to queue audio with SDL_QueueAudio()/SDL_DequeueAudio(), or use
 *  the audio callback, but not both.
 *
 *  You should not call SDL_LockAudio() on the device before clearing the
 *  queue; SDL handles locking internally for this function.
 *
 *  This function always succeeds and thus returns void.
 *
 *  \param dev The device ID of which to clear the audio queue.
 *
 *  \sa SDL_QueueAudio
 *  \sa SDL_GetQueuedAudioSize
 */
        #[no_mangle]
        pub fn SDL_ClearQueuedAudio(dev: SDL_AudioDeviceID);
        #[no_mangle]
        pub fn SDL_LockAudioDevice(dev: SDL_AudioDeviceID);
        #[no_mangle]
        pub fn SDL_UnlockAudioDevice(dev: SDL_AudioDeviceID);
        #[no_mangle]
        pub fn SDL_CloseAudioDevice(dev: SDL_AudioDeviceID);
    }
}
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
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
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub resetString: *mut libc::c_char,
        pub latchedString: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub validate: qboolean,
        pub integral: qboolean,
        pub min: libc::c_float,
        pub max: libc::c_float,
        pub description: *mut libc::c_char,
        pub next: *mut cvar_t,
        pub prev: *mut cvar_t,
        pub hashNext: *mut cvar_t,
        pub hashPrev: *mut cvar_t,
        pub hashIndex: libc::c_int,
    }
    /*
==========================================================

CVARS (console variables)

Many variables can be used for cheating purposes, so when
cheats is zero, force all unspecified variables to their
default values.
==========================================================
*/
    // set to cause it to be saved to vars.rc
    // used for system variables, not for player
					// specific configurations
    // sent to server on connect or change
    // sent in response to front end requests
    // these cvars will be duplicated on all clients
    // don't allow change from console at all,
    // but can be set from the command line
    // will only change when C code next does
    // a Cvar_Get(), so it can't be changed
					// without proper initialization.  modified
					// will be set, even though the value hasn't
					// changed yet
    // display only, cannot be set by user at all
    // created by a set command
    // can be set even when cheats are disabled, but is not archived
    // can not be changed if cheats are disabled
    // do not clear when a cvar_restart is issued
    // cvar was created by a server the client connected to.
    // cvar was created exclusively in one of the VMs.
    // prevent modifying this var from VMs or the server
    // These flags are only returned by the Cvar_Flags() function
    // Cvar was modified
    // Cvar doesn't exist.
    // nothing outside the Cvar_*() functions should modify these fields!
    pub type cvar_t = cvar_s;
    use super::{libc};
    extern "C" {
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/client/snd_local.h"]
pub mod snd_local_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct dma_t {
        pub channels: libc::c_int,
        pub samples: libc::c_int,
        pub fullsamples: libc::c_int,
        pub submission_chunk: libc::c_int,
        pub samplebits: libc::c_int,
        pub isfloat: libc::c_int,
        pub speed: libc::c_int,
        pub buffer: *mut byte,
    }
    use super::{libc};
    use super::q_shared_h::{byte, qboolean};
    extern "C" {
        #[no_mangle]
        pub static mut dma: dma_t;
    }
}
#[header_src =
      "ioq3/code/sdl/sdl_snd.c"]
pub mod sdl_snd_c {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct unnamed_0 {
        pub enumFormat: Uint16,
        pub stringFormat: *mut libc::c_char,
    }
    use super::SDL_stdinc_h::{Uint16};
    use super::{libc};
    use super::q_shared_h::{qboolean, cvar_t};
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn free(__ptr: *mut libc::c_void);
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src = "/usr/include/SDL2/SDL_error.h"]
pub mod SDL_error_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn SDL_GetError() -> *const libc::c_char;
    }
}
#[header_src = "/usr/include/SDL2/SDL.h"]
pub mod SDL_h {
    use super::{libc};
    use super::SDL_stdinc_h::{Uint32};
    extern "C" {
        /*
  Simple DirectMedia Layer
  Copyright (C) 1997-2018 Sam Lantinga <slouken@libsdl.org>

  This software is provided 'as-is', without any express or implied
  warranty.  In no event will the authors be held liable for any damages
  arising from the use of this software.

  Permission is granted to anyone to use this software for any purpose,
  including commercial applications, and to alter it and redistribute it
  freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not
     claim that you wrote the original software. If you use this software
     in a product, an acknowledgment in the product documentation would be
     appreciated but is not required.
  2. Altered source versions must be plainly marked as such, and must not be
     misrepresented as being the original software.
  3. This notice may not be removed or altered from any source distribution.
*/
        /* *
 *  \file SDL.h
 *
 *  Main include header for the SDL library
 */
        /* Set up for C function definitions, even when using C++ */
        /* As of version 0.5, SDL is loaded dynamically into the application */
        /* *
 *  \name SDL_INIT_*
 *
 *  These are the flags which may be passed to SDL_Init().  You should
 *  specify the subsystems which you will be using in your application.
 */
/* @{ */
        /* *< SDL_INIT_VIDEO implies SDL_INIT_EVENTS */
        /* *< SDL_INIT_JOYSTICK implies SDL_INIT_EVENTS */
        /* *< SDL_INIT_GAMECONTROLLER implies SDL_INIT_JOYSTICK */
        /* *< compatibility; this flag is ignored. */
        /* @} */
        /* *
 *  This function initializes  the subsystems specified by \c flags
 */
        #[no_mangle]
        pub fn SDL_Init(flags: Uint32) -> libc::c_int;
        /* *
 *  This function cleans up specific SDL subsystems
 */
        #[no_mangle]
        pub fn SDL_QuitSubSystem(flags: Uint32);
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::q_shared_h::{cvar_t};
    use super::{libc};
    extern "C" {
        // Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
        /*
==============================================================

CVAR

==============================================================
*/
        /*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
        #[no_mangle]
        pub fn Cvar_Get(var_name: *const libc::c_char,
                        value: *const libc::c_char, flags: libc::c_int)
         -> *mut cvar_t;
    }
}
#[header_src =
      "ioq3/code/client/client.h"]
pub mod client_h {
    use super::q_shared_h::{cvar_t};
    extern "C" {
        #[no_mangle]
        pub static mut cl_useMumble: *mut cvar_t;
    }
}
use self::stddef_h::{size_t};
use self::types_h::{__uint8_t, __int16_t, __uint16_t, __uint32_t};
use self::stdint_intn_h::{int16_t};
use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
use self::SDL_stdinc_h::{unnamed, SDL_TRUE, SDL_FALSE, Uint8, Sint16, Uint16,
                         Uint32, SDL_memset};
use self::SDL_audio_h::{SDL_AudioFormat, SDL_AudioCallback, SDL_AudioSpec,
                        SDL_AudioDeviceID, SDL_GetCurrentAudioDriver,
                        SDL_OpenAudioDevice, SDL_PauseAudioDevice,
                        SDL_DequeueAudio, SDL_GetQueuedAudioSize,
                        SDL_ClearQueuedAudio, SDL_LockAudioDevice,
                        SDL_UnlockAudioDevice, SDL_CloseAudioDevice};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, cvar_s, cvar_t,
                       Q_stricmp, Com_Printf};
use self::snd_local_h::{dma_t, dma};
use self::sdl_snd_c::{unnamed_0};
use self::stdlib_h::{calloc, free};
use self::string_h::{memcpy, memset};
use self::SDL_error_h::{SDL_GetError};
use self::SDL_h::{SDL_Init, SDL_QuitSubSystem};
use self::qcommon_h::{Cvar_Get};
use self::client_h::{cl_useMumble};
/*
====================================================================

  SYSTEM SPECIFIC FUNCTIONS

====================================================================
*/
// initializes cycling through a DMA buffer and returns information on it
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_Init() -> qboolean {
    let mut desired: SDL_AudioSpec =
        SDL_AudioSpec{freq: 0,
                      format: 0,
                      channels: 0,
                      silence: 0,
                      samples: 0,
                      padding: 0,
                      size: 0,
                      callback: None,
                      userdata: 0 as *mut libc::c_void,};
    let mut obtained: SDL_AudioSpec =
        SDL_AudioSpec{freq: 0,
                      format: 0,
                      channels: 0,
                      silence: 0,
                      samples: 0,
                      padding: 0,
                      size: 0,
                      callback: None,
                      userdata: 0 as *mut libc::c_void,};
    let mut tmp: libc::c_int = 0;
    if 0 != snd_inited as u64 { return qtrue }
    if s_sdlBits.is_null() {
        s_sdlBits =
            Cvar_Get(b"s_sdlBits\x00" as *const u8 as *const libc::c_char,
                     b"16\x00" as *const u8 as *const libc::c_char, 0x1i32);
        s_sdlSpeed =
            Cvar_Get(b"s_sdlSpeed\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
        s_sdlChannels =
            Cvar_Get(b"s_sdlChannels\x00" as *const u8 as *const libc::c_char,
                     b"2\x00" as *const u8 as *const libc::c_char, 0x1i32);
        s_sdlDevSamps =
            Cvar_Get(b"s_sdlDevSamps\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
        s_sdlMixSamps =
            Cvar_Get(b"s_sdlMixSamps\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char, 0x1i32)
    }
    Com_Printf(b"SDL_Init( SDL_INIT_AUDIO )... \x00" as *const u8 as
                   *const libc::c_char);
    if SDL_Init(0x10u32) != 0i32 {
        Com_Printf(b"FAILED (%s)\n\x00" as *const u8 as *const libc::c_char,
                   SDL_GetError());
        return qfalse
    }
    Com_Printf(b"OK\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(b"SDL audio driver is \"%s\".\n\x00" as *const u8 as
                   *const libc::c_char, SDL_GetCurrentAudioDriver());
    memset(&mut desired as *mut SDL_AudioSpec as *mut libc::c_void,
           '\u{0}' as i32,
           ::std::mem::size_of::<SDL_AudioSpec>() as libc::c_ulong);
    memset(&mut obtained as *mut SDL_AudioSpec as *mut libc::c_void,
           '\u{0}' as i32,
           ::std::mem::size_of::<SDL_AudioSpec>() as libc::c_ulong);
    tmp = (*s_sdlBits).value as libc::c_int;
    if tmp != 16i32 && tmp != 8i32 { tmp = 16i32 }
    desired.freq = (*s_sdlSpeed).value as libc::c_int;
    if 0 == desired.freq { desired.freq = 22050i32 }
    desired.format =
        (if tmp == 16i32 { 0x8010i32 } else { 0x8i32 }) as SDL_AudioFormat;
    if 0. != (*s_sdlDevSamps).value {
        desired.samples = (*s_sdlDevSamps).value as Uint16
    } else if desired.freq <= 11025i32 {
        desired.samples = 256i32 as Uint16
    } else if desired.freq <= 22050i32 {
        desired.samples = 512i32 as Uint16
    } else if desired.freq <= 44100i32 {
        desired.samples = 1024i32 as Uint16
    } else { desired.samples = 2048i32 as Uint16 }
    desired.channels = (*s_sdlChannels).value as libc::c_int as Uint8;
    desired.callback = Some(SNDDMA_AudioCallback);
    sdlPlaybackDevice =
        SDL_OpenAudioDevice(0 as *const libc::c_char,
                            SDL_FALSE as libc::c_int, &mut desired,
                            &mut obtained, 0x1i32 | 0x2i32 | 0x4i32 | 0x8i32);
    if sdlPlaybackDevice == 0i32 as libc::c_uint {
        Com_Printf(b"SDL_OpenAudioDevice() failed: %s\n\x00" as *const u8 as
                       *const libc::c_char, SDL_GetError());
        SDL_QuitSubSystem(0x10u32);
        return qfalse
    }
    SNDDMA_PrintAudiospec(b"SDL_AudioSpec\x00" as *const u8 as
                              *const libc::c_char, &mut obtained);
    tmp = (*s_sdlMixSamps).value as libc::c_int;
    if 0 == tmp {
        tmp =
            obtained.samples as libc::c_int * obtained.channels as libc::c_int
                * 10i32
    }
    tmp -= tmp % obtained.channels as libc::c_int;
    dmapos = 0i32;
    dma.samplebits = obtained.format as libc::c_int & 0xffi32;
    dma.isfloat = obtained.format as libc::c_int & 1i32 << 8i32;
    dma.channels = obtained.channels as libc::c_int;
    dma.samples = tmp;
    dma.fullsamples = dma.samples / dma.channels;
    dma.submission_chunk = 1i32;
    dma.speed = obtained.freq;
    dmasize = dma.samples * (dma.samplebits / 8i32);
    dma.buffer =
        calloc(1i32 as libc::c_ulong, dmasize as libc::c_ulong) as *mut byte;
    s_sdlCapture =
        Cvar_Get(b"s_sdlCapture\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x20i32);
    if Q_stricmp(SDL_GetCurrentAudioDriver(),
                 b"pulseaudio\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        Com_Printf(b"SDL audio capture support disabled for pulseaudio (https://bugzilla.libsdl.org/show_bug.cgi?id=4087)\n\x00"
                       as *const u8 as *const libc::c_char);
    } else if 0 == (*s_sdlCapture).integer {
        Com_Printf(b"SDL audio capture support disabled by user (\'+set s_sdlCapture 1\' to enable)\n\x00"
                       as *const u8 as *const libc::c_char);
    } else if 0 != (*cl_useMumble).integer {
        Com_Printf(b"SDL audio capture support disabled for Mumble support\n\x00"
                       as *const u8 as *const libc::c_char);
    } else {
        let mut spec: SDL_AudioSpec =
            SDL_AudioSpec{freq: 0,
                          format: 0,
                          channels: 0,
                          silence: 0,
                          samples: 0,
                          padding: 0,
                          size: 0,
                          callback: None,
                          userdata: 0 as *mut libc::c_void,};
        SDL_memset(&mut spec as *mut SDL_AudioSpec as *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<SDL_AudioSpec>() as libc::c_ulong);
        spec.freq = 48000i32;
        spec.format = 0x8010i32 as SDL_AudioFormat;
        spec.channels = 1i32 as Uint8;
        spec.samples = (20i32 * 48i32 * 3i32 * 4i32) as Uint16;
        sdlCaptureDevice =
            SDL_OpenAudioDevice(0 as *const libc::c_char,
                                SDL_TRUE as libc::c_int, &mut spec,
                                0 as *mut SDL_AudioSpec, 0i32);
        Com_Printf(b"SDL capture device %s.\n\x00" as *const u8 as
                       *const libc::c_char,
                   if sdlCaptureDevice == 0i32 as libc::c_uint {
                       b"failed to open\x00" as *const u8 as
                           *const libc::c_char
                   } else {
                       b"opened\x00" as *const u8 as *const libc::c_char
                   });
    }
    sdlMasterGain = 1.0f32;
    Com_Printf(b"Starting SDL audio callback...\n\x00" as *const u8 as
                   *const libc::c_char);
    SDL_PauseAudioDevice(sdlPlaybackDevice, 0i32);
    Com_Printf(b"SDL audio initialized.\n\x00" as *const u8 as
                   *const libc::c_char);
    snd_inited = qtrue;
    return qtrue;
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
#[no_mangle]
pub static mut snd_inited: qboolean = qfalse;
static mut sdlPlaybackDevice: SDL_AudioDeviceID = 0;
static mut sdlMasterGain: libc::c_float = 1.0f32;
static mut sdlCaptureDevice: SDL_AudioDeviceID = 0;
static mut s_sdlCapture: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut dmasize: libc::c_int = 0i32;
/* The audio callback. All the magic happens here. */
static mut dmapos: libc::c_int = 0i32;
#[no_mangle]
pub static mut s_sdlMixSamps: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
===============
SNDDMA_PrintAudiospec
===============
*/
unsafe extern "C" fn SNDDMA_PrintAudiospec(mut str: *const libc::c_char,
                                           mut spec: *const SDL_AudioSpec) {
    let mut i: libc::c_int = 0;
    let mut fmt: *mut libc::c_char = 0 as *mut libc::c_char;
    Com_Printf(b"%s:\n\x00" as *const u8 as *const libc::c_char, str);
    i = 0i32;
    while i < formatToStringTableSize {
        if (*spec).format as libc::c_int ==
               formatToStringTable[i as usize].enumFormat as libc::c_int {
            fmt = formatToStringTable[i as usize].stringFormat
        }
        i += 1
    }
    if !fmt.is_null() {
        Com_Printf(b"  Format:   %s\n\x00" as *const u8 as
                       *const libc::c_char, fmt);
    } else {
        Com_Printf(b"  Format:   ^1UNKNOWN\n\x00" as *const u8 as
                       *const libc::c_char);
    }
    Com_Printf(b"  Freq:     %d\n\x00" as *const u8 as *const libc::c_char,
               (*spec).freq);
    Com_Printf(b"  Samples:  %d\n\x00" as *const u8 as *const libc::c_char,
               (*spec).samples as libc::c_int);
    Com_Printf(b"  Channels: %d\n\x00" as *const u8 as *const libc::c_char,
               (*spec).channels as libc::c_int);
}
static mut formatToStringTable: [unnamed_0; 8] =
    [unnamed_0{enumFormat: 0x8i32 as Uint16,
               stringFormat:
                   b"AUDIO_U8\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,},
     unnamed_0{enumFormat: 0x8008i32 as Uint16,
               stringFormat:
                   b"AUDIO_S8\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,},
     unnamed_0{enumFormat: 0x10i32 as Uint16,
               stringFormat:
                   b"AUDIO_U16LSB\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,},
     unnamed_0{enumFormat: 0x8010i32 as Uint16,
               stringFormat:
                   b"AUDIO_S16LSB\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,},
     unnamed_0{enumFormat: 0x1010i32 as Uint16,
               stringFormat:
                   b"AUDIO_U16MSB\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,},
     unnamed_0{enumFormat: 0x9010i32 as Uint16,
               stringFormat:
                   b"AUDIO_S16MSB\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,},
     unnamed_0{enumFormat: 0x8120i32 as Uint16,
               stringFormat:
                   b"AUDIO_F32LSB\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,},
     unnamed_0{enumFormat: 0x9120i32 as Uint16,
               stringFormat:
                   b"AUDIO_F32MSB\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,}];
// Initialized in run_static_initializers
static mut formatToStringTableSize: libc::c_int = 0;
/*
===============
SNDDMA_AudioCallback
===============
*/
unsafe extern "C" fn SNDDMA_AudioCallback(mut userdata: *mut libc::c_void,
                                          mut stream: *mut Uint8,
                                          mut len: libc::c_int) {
    let mut pos: libc::c_int = dmapos * (dma.samplebits / 8i32);
    if pos >= dmasize { pos = 0i32; dmapos = pos }
    if 0 == snd_inited as u64 {
        memset(stream as *mut libc::c_void, '\u{0}' as i32,
               len as libc::c_ulong);
        return
    } else {
        let mut tobufend: libc::c_int = dmasize - pos;
        let mut len1: libc::c_int = len;
        let mut len2: libc::c_int = 0i32;
        if len1 > tobufend { len1 = tobufend; len2 = len - len1 }
        memcpy(stream as *mut libc::c_void,
               dma.buffer.offset(pos as isize) as *const libc::c_void,
               len1 as libc::c_ulong);
        if len2 <= 0i32 {
            dmapos += len1 / (dma.samplebits / 8i32)
        } else {
            memcpy(stream.offset(len1 as isize) as *mut libc::c_void,
                   dma.buffer as *const libc::c_void, len2 as libc::c_ulong);
            dmapos = len2 / (dma.samplebits / 8i32)
        }
    }
    if dmapos >= dmasize { dmapos = 0i32 }
    if sdlMasterGain != 1.0f32 {
        let mut i: libc::c_int = 0;
        if 0 != dma.isfloat && dma.samplebits == 32i32 {
            let mut ptr: *mut libc::c_float = stream as *mut libc::c_float;
            len =
                (len as
                     libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_float>()
                                                     as libc::c_ulong) as
                    libc::c_int as libc::c_int;
            i = 0i32;
            while i < len {
                *ptr *= sdlMasterGain;
                i += 1;
                ptr = ptr.offset(1isize)
            }
        } else if dma.samplebits == 16i32 {
            let mut ptr_0: *mut Sint16 = stream as *mut Sint16;
            len =
                (len as
                     libc::c_ulong).wrapping_div(::std::mem::size_of::<Sint16>()
                                                     as libc::c_ulong) as
                    libc::c_int as libc::c_int;
            i = 0i32;
            while i < len {
                *ptr_0 = (*ptr_0 as libc::c_float * sdlMasterGain) as Sint16;
                i += 1;
                ptr_0 = ptr_0.offset(1isize)
            }
        } else if dma.samplebits == 8i32 {
            let mut ptr_1: *mut Uint8 = stream;
            len =
                (len as
                     libc::c_ulong).wrapping_div(::std::mem::size_of::<Uint8>()
                                                     as libc::c_ulong) as
                    libc::c_int as libc::c_int;
            i = 0i32;
            while i < len {
                *ptr_1 = (*ptr_1 as libc::c_float * sdlMasterGain) as Uint8;
                i += 1;
                ptr_1 = ptr_1.offset(1isize)
            }
        }
    };
}
#[no_mangle]
pub static mut s_sdlChannels: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_sdlDevSamps: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_sdlSpeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_sdlBits: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// gets the current DMA position
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_GetDMAPos() -> libc::c_int { return dmapos; }
// shutdown the DMA xfer.
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_Shutdown() {
    if sdlPlaybackDevice != 0i32 as libc::c_uint {
        Com_Printf(b"Closing SDL audio playback device...\n\x00" as *const u8
                       as *const libc::c_char);
        SDL_CloseAudioDevice(sdlPlaybackDevice);
        Com_Printf(b"SDL audio playback device closed.\n\x00" as *const u8 as
                       *const libc::c_char);
        sdlPlaybackDevice = 0i32 as SDL_AudioDeviceID
    }
    if 0 != sdlCaptureDevice {
        Com_Printf(b"Closing SDL audio capture device...\n\x00" as *const u8
                       as *const libc::c_char);
        SDL_CloseAudioDevice(sdlCaptureDevice);
        Com_Printf(b"SDL audio capture device closed.\n\x00" as *const u8 as
                       *const libc::c_char);
        sdlCaptureDevice = 0i32 as SDL_AudioDeviceID
    }
    SDL_QuitSubSystem(0x10u32);
    free(dma.buffer as *mut libc::c_void);
    dma.buffer = 0 as *mut byte;
    dmasize = 0i32;
    dmapos = dmasize;
    snd_inited = qfalse;
    Com_Printf(b"SDL audio shut down.\n\x00" as *const u8 as
                   *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_BeginPainting() {
    SDL_LockAudioDevice(sdlPlaybackDevice);
}
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_Submit() {
    SDL_UnlockAudioDevice(sdlPlaybackDevice);
}
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_StartCapture() {
    if 0 != sdlCaptureDevice {
        SDL_ClearQueuedAudio(sdlCaptureDevice);
        SDL_PauseAudioDevice(sdlCaptureDevice, 0i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_AvailableCaptureSamples() -> libc::c_int {
    return (if 0 != sdlCaptureDevice {
                SDL_GetQueuedAudioSize(sdlCaptureDevice).wrapping_div(2i32 as
                                                                          libc::c_uint)
            } else { 0i32 as libc::c_uint }) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_Capture(mut samples: libc::c_int,
                                        mut data: *mut byte) {
    if 0 != sdlCaptureDevice {
        SDL_DequeueAudio(sdlCaptureDevice, data as *mut libc::c_void,
                         (samples * 2i32) as Uint32);
    } else {
        SDL_memset(data as *mut libc::c_void, '\u{0}' as i32,
                   (samples * 2i32) as size_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_StopCapture() {
    if 0 != sdlCaptureDevice {
        SDL_PauseAudioDevice(sdlCaptureDevice, 1i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_MasterGain(mut val: libc::c_float) {
    sdlMasterGain = val;
}
unsafe extern "C" fn run_static_initializers() {
    formatToStringTableSize =
        (::std::mem::size_of::<[unnamed_0; 8]>() as
             libc::c_ulong).wrapping_div(::std::mem::size_of::<unnamed_0>() as
                                             libc::c_ulong) as libc::c_int
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];