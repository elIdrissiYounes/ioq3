#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, extern_types, libc)]
extern crate libc;
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/usr/include/AL/al.h"]
pub mod al_h {
    pub type ALboolean = libc::c_char;
    pub type ALchar = libc::c_char;
    pub type ALint = libc::c_int;
    pub type ALuint = libc::c_uint;
    pub type ALsizei = libc::c_int;
    pub type ALenum = libc::c_int;
    pub type ALfloat = libc::c_float;
    pub type ALdouble = libc::c_double;
    pub type ALvoid = ();
    pub type LPALENABLE = Option<unsafe extern "C" fn(_: ALenum) -> ()>;
    pub type LPALDISABLE = Option<unsafe extern "C" fn(_: ALenum) -> ()>;
    pub type LPALISENABLED
        =
        Option<unsafe extern "C" fn(_: ALenum) -> ALboolean>;
    pub type LPALGETSTRING
        =
        Option<unsafe extern "C" fn(_: ALenum) -> *const ALchar>;
    pub type LPALGETBOOLEANV
        =
        Option<unsafe extern "C" fn(_: ALenum, _: *mut ALboolean) -> ()>;
    pub type LPALGETINTEGERV
        =
        Option<unsafe extern "C" fn(_: ALenum, _: *mut ALint) -> ()>;
    pub type LPALGETFLOATV
        =
        Option<unsafe extern "C" fn(_: ALenum, _: *mut ALfloat) -> ()>;
    pub type LPALGETDOUBLEV
        =
        Option<unsafe extern "C" fn(_: ALenum, _: *mut ALdouble) -> ()>;
    pub type LPALGETBOOLEAN
        =
        Option<unsafe extern "C" fn(_: ALenum) -> ALboolean>;
    pub type LPALGETINTEGER
        =
        Option<unsafe extern "C" fn(_: ALenum) -> ALint>;
    pub type LPALGETFLOAT
        =
        Option<unsafe extern "C" fn(_: ALenum) -> ALfloat>;
    pub type LPALGETDOUBLE
        =
        Option<unsafe extern "C" fn(_: ALenum) -> ALdouble>;
    pub type LPALGETERROR = Option<unsafe extern "C" fn() -> ALenum>;
    pub type LPALISEXTENSIONPRESENT
        =
        Option<unsafe extern "C" fn(_: *const ALchar) -> ALboolean>;
    pub type LPALGETPROCADDRESS
        =
        Option<unsafe extern "C" fn(_: *const ALchar) -> *mut libc::c_void>;
    pub type LPALGETENUMVALUE
        =
        Option<unsafe extern "C" fn(_: *const ALchar) -> ALenum>;
    pub type LPALLISTENERF
        =
        Option<unsafe extern "C" fn(_: ALenum, _: ALfloat) -> ()>;
    pub type LPALLISTENER3F
        =
        Option<unsafe extern "C" fn(_: ALenum, _: ALfloat, _: ALfloat,
                                    _: ALfloat) -> ()>;
    pub type LPALLISTENERFV
        =
        Option<unsafe extern "C" fn(_: ALenum, _: *const ALfloat) -> ()>;
    pub type LPALLISTENERI
        =
        Option<unsafe extern "C" fn(_: ALenum, _: ALint) -> ()>;
    pub type LPALGETLISTENERF
        =
        Option<unsafe extern "C" fn(_: ALenum, _: *mut ALfloat) -> ()>;
    pub type LPALGETLISTENER3F
        =
        Option<unsafe extern "C" fn(_: ALenum, _: *mut ALfloat,
                                    _: *mut ALfloat, _: *mut ALfloat) -> ()>;
    pub type LPALGETLISTENERFV
        =
        Option<unsafe extern "C" fn(_: ALenum, _: *mut ALfloat) -> ()>;
    pub type LPALGETLISTENERI
        =
        Option<unsafe extern "C" fn(_: ALenum, _: *mut ALint) -> ()>;
    pub type LPALGENSOURCES
        =
        Option<unsafe extern "C" fn(_: ALsizei, _: *mut ALuint) -> ()>;
    pub type LPALDELETESOURCES
        =
        Option<unsafe extern "C" fn(_: ALsizei, _: *const ALuint) -> ()>;
    pub type LPALISSOURCE
        =
        Option<unsafe extern "C" fn(_: ALuint) -> ALboolean>;
    pub type LPALSOURCEF
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum, _: ALfloat) -> ()>;
    pub type LPALSOURCE3F
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum, _: ALfloat,
                                    _: ALfloat, _: ALfloat) -> ()>;
    pub type LPALSOURCEFV
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum, _: *const ALfloat)
                   -> ()>;
    pub type LPALSOURCEI
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum, _: ALint) -> ()>;
    pub type LPALGETSOURCEF
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum, _: *mut ALfloat)
                   -> ()>;
    pub type LPALGETSOURCE3F
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum, _: *mut ALfloat,
                                    _: *mut ALfloat, _: *mut ALfloat) -> ()>;
    pub type LPALGETSOURCEFV
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum, _: *mut ALfloat)
                   -> ()>;
    pub type LPALGETSOURCEI
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum, _: *mut ALint)
                   -> ()>;
    pub type LPALSOURCEPLAYV
        =
        Option<unsafe extern "C" fn(_: ALsizei, _: *const ALuint) -> ()>;
    pub type LPALSOURCESTOPV
        =
        Option<unsafe extern "C" fn(_: ALsizei, _: *const ALuint) -> ()>;
    pub type LPALSOURCEREWINDV
        =
        Option<unsafe extern "C" fn(_: ALsizei, _: *const ALuint) -> ()>;
    pub type LPALSOURCEPAUSEV
        =
        Option<unsafe extern "C" fn(_: ALsizei, _: *const ALuint) -> ()>;
    pub type LPALSOURCEPLAY = Option<unsafe extern "C" fn(_: ALuint) -> ()>;
    pub type LPALSOURCESTOP = Option<unsafe extern "C" fn(_: ALuint) -> ()>;
    pub type LPALSOURCEREWIND = Option<unsafe extern "C" fn(_: ALuint) -> ()>;
    pub type LPALSOURCEPAUSE = Option<unsafe extern "C" fn(_: ALuint) -> ()>;
    pub type LPALSOURCEQUEUEBUFFERS
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALsizei, _: *const ALuint)
                   -> ()>;
    pub type LPALSOURCEUNQUEUEBUFFERS
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALsizei, _: *mut ALuint)
                   -> ()>;
    pub type LPALGENBUFFERS
        =
        Option<unsafe extern "C" fn(_: ALsizei, _: *mut ALuint) -> ()>;
    pub type LPALDELETEBUFFERS
        =
        Option<unsafe extern "C" fn(_: ALsizei, _: *const ALuint) -> ()>;
    pub type LPALISBUFFER
        =
        Option<unsafe extern "C" fn(_: ALuint) -> ALboolean>;
    pub type LPALBUFFERDATA
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum,
                                    _: *const libc::c_void, _: ALsizei,
                                    _: ALsizei) -> ()>;
    pub type LPALGETBUFFERF
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum, _: *mut ALfloat)
                   -> ()>;
    pub type LPALGETBUFFERI
        =
        Option<unsafe extern "C" fn(_: ALuint, _: ALenum, _: *mut ALint)
                   -> ()>;
    pub type LPALDOPPLERFACTOR
        =
        Option<unsafe extern "C" fn(_: ALfloat) -> ()>;
    pub type LPALSPEEDOFSOUND
        =
        Option<unsafe extern "C" fn(_: ALfloat) -> ()>;
    pub type LPALDISTANCEMODEL
        =
        Option<unsafe extern "C" fn(_: ALenum) -> ()>;
    use super::{libc};
}
#[header_src = "/usr/include/AL/alc.h"]
pub mod alc_h {
    pub type ALCdevice = ALCdevice_struct;
    pub type ALCcontext = ALCcontext_struct;
    pub type ALCboolean = libc::c_char;
    pub type ALCchar = libc::c_char;
    pub type ALCint = libc::c_int;
    pub type ALCuint = libc::c_uint;
    pub type ALCsizei = libc::c_int;
    pub type ALCenum = libc::c_int;
    pub type ALCvoid = ();
    pub type LPALCCREATECONTEXT
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice, _: *const ALCint)
                   -> *mut ALCcontext>;
    pub type LPALCMAKECONTEXTCURRENT
        =
        Option<unsafe extern "C" fn(_: *mut ALCcontext) -> ALCboolean>;
    pub type LPALCPROCESSCONTEXT
        =
        Option<unsafe extern "C" fn(_: *mut ALCcontext) -> ()>;
    pub type LPALCSUSPENDCONTEXT
        =
        Option<unsafe extern "C" fn(_: *mut ALCcontext) -> ()>;
    pub type LPALCDESTROYCONTEXT
        =
        Option<unsafe extern "C" fn(_: *mut ALCcontext) -> ()>;
    pub type LPALCGETCURRENTCONTEXT
        =
        Option<unsafe extern "C" fn() -> *mut ALCcontext>;
    pub type LPALCGETCONTEXTSDEVICE
        =
        Option<unsafe extern "C" fn(_: *mut ALCcontext) -> *mut ALCdevice>;
    pub type LPALCOPENDEVICE
        =
        Option<unsafe extern "C" fn(_: *const ALCchar) -> *mut ALCdevice>;
    pub type LPALCCLOSEDEVICE
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice) -> ALCboolean>;
    pub type LPALCGETERROR
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice) -> ALCenum>;
    pub type LPALCISEXTENSIONPRESENT
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice, _: *const ALCchar)
                   -> ALCboolean>;
    pub type LPALCGETPROCADDRESS
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice, _: *const ALCchar)
                   -> *mut libc::c_void>;
    pub type LPALCGETENUMVALUE
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice, _: *const ALCchar)
                   -> ALCenum>;
    pub type LPALCGETSTRING
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice, _: ALCenum)
                   -> *const ALCchar>;
    pub type LPALCGETINTEGERV
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice, _: ALCenum,
                                    _: ALCsizei, _: *mut ALCint) -> ()>;
    pub type LPALCCAPTUREOPENDEVICE
        =
        Option<unsafe extern "C" fn(_: *const ALCchar, _: ALCuint, _: ALCenum,
                                    _: ALCsizei) -> *mut ALCdevice>;
    pub type LPALCCAPTURECLOSEDEVICE
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice) -> ALCboolean>;
    pub type LPALCCAPTURESTART
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice) -> ()>;
    pub type LPALCCAPTURESTOP
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice) -> ()>;
    pub type LPALCCAPTURESAMPLES
        =
        Option<unsafe extern "C" fn(_: *mut ALCdevice, _: *mut libc::c_void,
                                    _: ALCsizei) -> ()>;
    use super::{libc};
    extern "C" {
        pub type ALCdevice_struct;
        pub type ALCcontext_struct;
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/client/qal.h"]
pub mod qal_h {
    use super::al_h::{LPALENABLE, LPALDISABLE, LPALISENABLED, LPALGETSTRING,
                      LPALGETBOOLEANV, LPALGETINTEGERV, LPALGETFLOATV,
                      LPALGETDOUBLEV, LPALGETBOOLEAN, LPALGETINTEGER,
                      LPALGETFLOAT, LPALGETDOUBLE, LPALGETERROR,
                      LPALISEXTENSIONPRESENT, LPALGETPROCADDRESS,
                      LPALGETENUMVALUE, LPALLISTENERF, LPALLISTENER3F,
                      LPALLISTENERFV, LPALLISTENERI, LPALGETLISTENERF,
                      LPALGETLISTENER3F, LPALGETLISTENERFV, LPALGETLISTENERI,
                      LPALGENSOURCES, LPALDELETESOURCES, LPALISSOURCE,
                      LPALSOURCEF, LPALSOURCE3F, LPALSOURCEFV, LPALSOURCEI,
                      LPALGETSOURCEF, LPALGETSOURCE3F, LPALGETSOURCEFV,
                      LPALGETSOURCEI, LPALSOURCEPLAYV, LPALSOURCESTOPV,
                      LPALSOURCEREWINDV, LPALSOURCEPAUSEV, LPALSOURCEPLAY,
                      LPALSOURCESTOP, LPALSOURCEREWIND, LPALSOURCEPAUSE,
                      LPALSOURCEQUEUEBUFFERS, LPALSOURCEUNQUEUEBUFFERS,
                      LPALGENBUFFERS, LPALDELETEBUFFERS, LPALISBUFFER,
                      LPALBUFFERDATA, LPALGETBUFFERF, LPALGETBUFFERI,
                      LPALDOPPLERFACTOR, LPALSPEEDOFSOUND, LPALDISTANCEMODEL};
    use super::alc_h::{LPALCCREATECONTEXT, LPALCMAKECONTEXTCURRENT,
                       LPALCPROCESSCONTEXT, LPALCSUSPENDCONTEXT,
                       LPALCDESTROYCONTEXT, LPALCGETCURRENTCONTEXT,
                       LPALCGETCONTEXTSDEVICE, LPALCOPENDEVICE,
                       LPALCCLOSEDEVICE, LPALCGETERROR,
                       LPALCISEXTENSIONPRESENT, LPALCGETPROCADDRESS,
                       LPALCGETENUMVALUE, LPALCGETSTRING, LPALCGETINTEGERV,
                       LPALCCAPTUREOPENDEVICE, LPALCCAPTURECLOSEDEVICE,
                       LPALCCAPTURESTART, LPALCCAPTURESTOP,
                       LPALCCAPTURESAMPLES};
    use super::q_shared_h::{qboolean};
    use super::{libc};
}
#[header_src =
      "ioq3/code/client/qal.c"]
pub mod qal_c { }
#[header_src = "/usr/include/SDL2/SDL_loadso.h"]
pub mod SDL_loadso_h {
    use super::{libc};
    extern "C" {
        /* *
 *  Unload a shared object from memory.
 */
        #[no_mangle]
        pub fn SDL_UnloadObject(handle: *mut libc::c_void);
        /* *
 *  Given an object handle, this function looks up the address of the
 *  named function in the shared object and returns it.  This address
 *  is no longer valid after calling SDL_UnloadObject().
 */
        #[no_mangle]
        pub fn SDL_LoadFunction(handle: *mut libc::c_void,
                                name: *const libc::c_char)
         -> *mut libc::c_void;
    }
}
#[header_src =
      "ioq3/code/sys/sys_loadlib.h"]
pub mod sys_loadlib_h {
    use super::{libc};
    use super::q_shared_h::{qboolean};
    extern "C" {
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
        pub fn Sys_LoadDll(name: *const libc::c_char, useSystemLib: qboolean)
         -> *mut libc::c_void;
    }
}
use self::q_shared_h::{qboolean, qtrue, qfalse, Com_Printf};
use self::al_h::{ALboolean, ALchar, ALint, ALuint, ALsizei, ALenum, ALfloat,
                 ALdouble, ALvoid, LPALENABLE, LPALDISABLE, LPALISENABLED,
                 LPALGETSTRING, LPALGETBOOLEANV, LPALGETINTEGERV,
                 LPALGETFLOATV, LPALGETDOUBLEV, LPALGETBOOLEAN,
                 LPALGETINTEGER, LPALGETFLOAT, LPALGETDOUBLE, LPALGETERROR,
                 LPALISEXTENSIONPRESENT, LPALGETPROCADDRESS, LPALGETENUMVALUE,
                 LPALLISTENERF, LPALLISTENER3F, LPALLISTENERFV, LPALLISTENERI,
                 LPALGETLISTENERF, LPALGETLISTENER3F, LPALGETLISTENERFV,
                 LPALGETLISTENERI, LPALGENSOURCES, LPALDELETESOURCES,
                 LPALISSOURCE, LPALSOURCEF, LPALSOURCE3F, LPALSOURCEFV,
                 LPALSOURCEI, LPALGETSOURCEF, LPALGETSOURCE3F,
                 LPALGETSOURCEFV, LPALGETSOURCEI, LPALSOURCEPLAYV,
                 LPALSOURCESTOPV, LPALSOURCEREWINDV, LPALSOURCEPAUSEV,
                 LPALSOURCEPLAY, LPALSOURCESTOP, LPALSOURCEREWIND,
                 LPALSOURCEPAUSE, LPALSOURCEQUEUEBUFFERS,
                 LPALSOURCEUNQUEUEBUFFERS, LPALGENBUFFERS, LPALDELETEBUFFERS,
                 LPALISBUFFER, LPALBUFFERDATA, LPALGETBUFFERF, LPALGETBUFFERI,
                 LPALDOPPLERFACTOR, LPALSPEEDOFSOUND, LPALDISTANCEMODEL};
use self::alc_h::{ALCdevice, ALCcontext, ALCboolean, ALCchar, ALCint, ALCuint,
                  ALCsizei, ALCenum, ALCvoid, LPALCCREATECONTEXT,
                  LPALCMAKECONTEXTCURRENT, LPALCPROCESSCONTEXT,
                  LPALCSUSPENDCONTEXT, LPALCDESTROYCONTEXT,
                  LPALCGETCURRENTCONTEXT, LPALCGETCONTEXTSDEVICE,
                  LPALCOPENDEVICE, LPALCCLOSEDEVICE, LPALCGETERROR,
                  LPALCISEXTENSIONPRESENT, LPALCGETPROCADDRESS,
                  LPALCGETENUMVALUE, LPALCGETSTRING, LPALCGETINTEGERV,
                  LPALCCAPTUREOPENDEVICE, LPALCCAPTURECLOSEDEVICE,
                  LPALCCAPTURESTART, LPALCCAPTURESTOP, LPALCCAPTURESAMPLES,
                  ALCdevice_struct, ALCcontext_struct};
use self::qcommon_h::{Com_DPrintf};
use self::SDL_loadso_h::{SDL_UnloadObject, SDL_LoadFunction};
use self::sys_loadlib_h::{Sys_LoadDll};
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
/* Hack to enable compiling both on OpenAL SDK and OpenAL-soft. */
#[no_mangle]
pub static mut qalEnable: LPALENABLE = None;
#[no_mangle]
pub static mut qalDisable: LPALDISABLE = None;
#[no_mangle]
pub static mut qalIsEnabled: LPALISENABLED = None;
#[no_mangle]
pub static mut qalGetString: LPALGETSTRING = None;
#[no_mangle]
pub static mut qalGetBooleanv: LPALGETBOOLEANV = None;
#[no_mangle]
pub static mut qalGetIntegerv: LPALGETINTEGERV = None;
#[no_mangle]
pub static mut qalGetFloatv: LPALGETFLOATV = None;
#[no_mangle]
pub static mut qalGetDoublev: LPALGETDOUBLEV = None;
#[no_mangle]
pub static mut qalGetBoolean: LPALGETBOOLEAN = None;
#[no_mangle]
pub static mut qalGetInteger: LPALGETINTEGER = None;
#[no_mangle]
pub static mut qalGetFloat: LPALGETFLOAT = None;
#[no_mangle]
pub static mut qalGetDouble: LPALGETDOUBLE = None;
#[no_mangle]
pub static mut qalGetError: LPALGETERROR = None;
#[no_mangle]
pub static mut qalIsExtensionPresent: LPALISEXTENSIONPRESENT = None;
#[no_mangle]
pub static mut qalGetProcAddress: LPALGETPROCADDRESS = None;
#[no_mangle]
pub static mut qalGetEnumValue: LPALGETENUMVALUE = None;
#[no_mangle]
pub static mut qalListenerf: LPALLISTENERF = None;
#[no_mangle]
pub static mut qalListener3f: LPALLISTENER3F = None;
#[no_mangle]
pub static mut qalListenerfv: LPALLISTENERFV = None;
#[no_mangle]
pub static mut qalListeneri: LPALLISTENERI = None;
#[no_mangle]
pub static mut qalGetListenerf: LPALGETLISTENERF = None;
#[no_mangle]
pub static mut qalGetListener3f: LPALGETLISTENER3F = None;
#[no_mangle]
pub static mut qalGetListenerfv: LPALGETLISTENERFV = None;
#[no_mangle]
pub static mut qalGetListeneri: LPALGETLISTENERI = None;
#[no_mangle]
pub static mut qalGenSources: LPALGENSOURCES = None;
#[no_mangle]
pub static mut qalDeleteSources: LPALDELETESOURCES = None;
#[no_mangle]
pub static mut qalIsSource: LPALISSOURCE = None;
#[no_mangle]
pub static mut qalSourcef: LPALSOURCEF = None;
#[no_mangle]
pub static mut qalSource3f: LPALSOURCE3F = None;
#[no_mangle]
pub static mut qalSourcefv: LPALSOURCEFV = None;
#[no_mangle]
pub static mut qalSourcei: LPALSOURCEI = None;
#[no_mangle]
pub static mut qalGetSourcef: LPALGETSOURCEF = None;
#[no_mangle]
pub static mut qalGetSource3f: LPALGETSOURCE3F = None;
#[no_mangle]
pub static mut qalGetSourcefv: LPALGETSOURCEFV = None;
#[no_mangle]
pub static mut qalGetSourcei: LPALGETSOURCEI = None;
#[no_mangle]
pub static mut qalSourcePlayv: LPALSOURCEPLAYV = None;
#[no_mangle]
pub static mut qalSourceStopv: LPALSOURCESTOPV = None;
#[no_mangle]
pub static mut qalSourceRewindv: LPALSOURCEREWINDV = None;
#[no_mangle]
pub static mut qalSourcePausev: LPALSOURCEPAUSEV = None;
#[no_mangle]
pub static mut qalSourcePlay: LPALSOURCEPLAY = None;
#[no_mangle]
pub static mut qalSourceStop: LPALSOURCESTOP = None;
#[no_mangle]
pub static mut qalSourceRewind: LPALSOURCEREWIND = None;
#[no_mangle]
pub static mut qalSourcePause: LPALSOURCEPAUSE = None;
#[no_mangle]
pub static mut qalSourceQueueBuffers: LPALSOURCEQUEUEBUFFERS = None;
#[no_mangle]
pub static mut qalSourceUnqueueBuffers: LPALSOURCEUNQUEUEBUFFERS = None;
#[no_mangle]
pub static mut qalGenBuffers: LPALGENBUFFERS = None;
#[no_mangle]
pub static mut qalDeleteBuffers: LPALDELETEBUFFERS = None;
#[no_mangle]
pub static mut qalIsBuffer: LPALISBUFFER = None;
#[no_mangle]
pub static mut qalBufferData: LPALBUFFERDATA = None;
#[no_mangle]
pub static mut qalGetBufferf: LPALGETBUFFERF = None;
#[no_mangle]
pub static mut qalGetBufferi: LPALGETBUFFERI = None;
#[no_mangle]
pub static mut qalDopplerFactor: LPALDOPPLERFACTOR = None;
#[no_mangle]
pub static mut qalSpeedOfSound: LPALSPEEDOFSOUND = None;
#[no_mangle]
pub static mut qalDistanceModel: LPALDISTANCEMODEL = None;
#[no_mangle]
pub static mut qalcCreateContext: LPALCCREATECONTEXT = None;
#[no_mangle]
pub static mut qalcMakeContextCurrent: LPALCMAKECONTEXTCURRENT = None;
#[no_mangle]
pub static mut qalcProcessContext: LPALCPROCESSCONTEXT = None;
#[no_mangle]
pub static mut qalcSuspendContext: LPALCSUSPENDCONTEXT = None;
#[no_mangle]
pub static mut qalcDestroyContext: LPALCDESTROYCONTEXT = None;
#[no_mangle]
pub static mut qalcGetCurrentContext: LPALCGETCURRENTCONTEXT = None;
#[no_mangle]
pub static mut qalcGetContextsDevice: LPALCGETCONTEXTSDEVICE = None;
#[no_mangle]
pub static mut qalcOpenDevice: LPALCOPENDEVICE = None;
#[no_mangle]
pub static mut qalcCloseDevice: LPALCCLOSEDEVICE = None;
#[no_mangle]
pub static mut qalcGetError: LPALCGETERROR = None;
#[no_mangle]
pub static mut qalcIsExtensionPresent: LPALCISEXTENSIONPRESENT = None;
#[no_mangle]
pub static mut qalcGetProcAddress: LPALCGETPROCADDRESS = None;
#[no_mangle]
pub static mut qalcGetEnumValue: LPALCGETENUMVALUE = None;
#[no_mangle]
pub static mut qalcGetString: LPALCGETSTRING = None;
#[no_mangle]
pub static mut qalcGetIntegerv: LPALCGETINTEGERV = None;
#[no_mangle]
pub static mut qalcCaptureOpenDevice: LPALCCAPTUREOPENDEVICE = None;
#[no_mangle]
pub static mut qalcCaptureCloseDevice: LPALCCAPTURECLOSEDEVICE = None;
#[no_mangle]
pub static mut qalcCaptureStart: LPALCCAPTURESTART = None;
#[no_mangle]
pub static mut qalcCaptureStop: LPALCCAPTURESTOP = None;
#[no_mangle]
pub static mut qalcCaptureSamples: LPALCCAPTURESAMPLES = None;
#[no_mangle]
pub unsafe extern "C" fn QAL_Init(mut libname: *const libc::c_char)
 -> qboolean {
    if !OpenALLib.is_null() { return qtrue }
    OpenALLib = Sys_LoadDll(libname, qtrue);
    if OpenALLib.is_null() { return qfalse }
    alinit_fail = qfalse;
    qalEnable =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALENABLE>(GPA(b"alEnable\x00" as *const u8
                                                    as *const libc::c_char as
                                                    *mut libc::c_char));
    qalDisable =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALDISABLE>(GPA(b"alDisable\x00" as *const u8
                                                     as *const libc::c_char as
                                                     *mut libc::c_char));
    qalIsEnabled =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALISENABLED>(GPA(b"alIsEnabled\x00" as
                                                       *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char));
    qalGetString =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETSTRING>(GPA(b"alGetString\x00" as
                                                       *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char));
    qalGetBooleanv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETBOOLEANV>(GPA(b"alGetBooleanv\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
    qalGetIntegerv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETINTEGERV>(GPA(b"alGetIntegerv\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
    qalGetFloatv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETFLOATV>(GPA(b"alGetFloatv\x00" as
                                                       *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char));
    qalGetDoublev =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETDOUBLEV>(GPA(b"alGetDoublev\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalGetBoolean =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETBOOLEAN>(GPA(b"alGetBoolean\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalGetInteger =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETINTEGER>(GPA(b"alGetInteger\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalGetFloat =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETFLOAT>(GPA(b"alGetFloat\x00" as
                                                      *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char));
    qalGetDouble =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETDOUBLE>(GPA(b"alGetDouble\x00" as
                                                       *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char));
    qalGetError =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETERROR>(GPA(b"alGetError\x00" as
                                                      *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char));
    qalIsExtensionPresent =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALISEXTENSIONPRESENT>(GPA(b"alIsExtensionPresent\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char));
    qalGetProcAddress =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETPROCADDRESS>(GPA(b"alGetProcAddress\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char));
    qalGetEnumValue =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETENUMVALUE>(GPA(b"alGetEnumValue\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char));
    qalListenerf =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALLISTENERF>(GPA(b"alListenerf\x00" as
                                                       *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char));
    qalListener3f =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALLISTENER3F>(GPA(b"alListener3f\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalListenerfv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALLISTENERFV>(GPA(b"alListenerfv\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalListeneri =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALLISTENERI>(GPA(b"alListeneri\x00" as
                                                       *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char));
    qalGetListenerf =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETLISTENERF>(GPA(b"alGetListenerf\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char));
    qalGetListener3f =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETLISTENER3F>(GPA(b"alGetListener3f\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char));
    qalGetListenerfv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETLISTENERFV>(GPA(b"alGetListenerfv\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char));
    qalGetListeneri =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETLISTENERI>(GPA(b"alGetListeneri\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char));
    qalGenSources =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGENSOURCES>(GPA(b"alGenSources\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalDeleteSources =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALDELETESOURCES>(GPA(b"alDeleteSources\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char));
    qalIsSource =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALISSOURCE>(GPA(b"alIsSource\x00" as
                                                      *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char));
    qalSourcef =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEF>(GPA(b"alSourcef\x00" as *const u8
                                                     as *const libc::c_char as
                                                     *mut libc::c_char));
    qalSource3f =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCE3F>(GPA(b"alSource3f\x00" as
                                                      *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char));
    qalSourcefv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEFV>(GPA(b"alSourcefv\x00" as
                                                      *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char));
    qalSourcei =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEI>(GPA(b"alSourcei\x00" as *const u8
                                                     as *const libc::c_char as
                                                     *mut libc::c_char));
    qalGetSourcef =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETSOURCEF>(GPA(b"alGetSourcef\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalGetSource3f =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETSOURCE3F>(GPA(b"alGetSource3f\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
    qalGetSourcefv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETSOURCEFV>(GPA(b"alGetSourcefv\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
    qalGetSourcei =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETSOURCEI>(GPA(b"alGetSourcei\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalSourcePlayv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEPLAYV>(GPA(b"alSourcePlayv\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
    qalSourceStopv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCESTOPV>(GPA(b"alSourceStopv\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
    qalSourceRewindv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEREWINDV>(GPA(b"alSourceRewindv\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char));
    qalSourcePausev =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEPAUSEV>(GPA(b"alSourcePausev\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char));
    qalSourcePlay =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEPLAY>(GPA(b"alSourcePlay\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalSourceStop =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCESTOP>(GPA(b"alSourceStop\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalSourceRewind =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEREWIND>(GPA(b"alSourceRewind\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char));
    qalSourcePause =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEPAUSE>(GPA(b"alSourcePause\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
    qalSourceQueueBuffers =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEQUEUEBUFFERS>(GPA(b"alSourceQueueBuffers\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char));
    qalSourceUnqueueBuffers =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSOURCEUNQUEUEBUFFERS>(GPA(b"alSourceUnqueueBuffers\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char));
    qalGenBuffers =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGENBUFFERS>(GPA(b"alGenBuffers\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalDeleteBuffers =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALDELETEBUFFERS>(GPA(b"alDeleteBuffers\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char));
    qalIsBuffer =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALISBUFFER>(GPA(b"alIsBuffer\x00" as
                                                      *const u8 as
                                                      *const libc::c_char as
                                                      *mut libc::c_char));
    qalBufferData =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALBUFFERDATA>(GPA(b"alBufferData\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalGetBufferf =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETBUFFERF>(GPA(b"alGetBufferf\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalGetBufferi =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALGETBUFFERI>(GPA(b"alGetBufferi\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalDopplerFactor =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALDOPPLERFACTOR>(GPA(b"alDopplerFactor\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char));
    qalSpeedOfSound =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALSPEEDOFSOUND>(GPA(b"alSpeedOfSound\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char));
    qalDistanceModel =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALDISTANCEMODEL>(GPA(b"alDistanceModel\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char));
    qalcCreateContext =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCCREATECONTEXT>(GPA(b"alcCreateContext\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char));
    qalcMakeContextCurrent =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCMAKECONTEXTCURRENT>(GPA(b"alcMakeContextCurrent\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char
                                                                 as
                                                                 *mut libc::c_char));
    qalcProcessContext =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCPROCESSCONTEXT>(GPA(b"alcProcessContext\x00"
                                                             as *const u8 as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char));
    qalcSuspendContext =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCSUSPENDCONTEXT>(GPA(b"alcSuspendContext\x00"
                                                             as *const u8 as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char));
    qalcDestroyContext =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCDESTROYCONTEXT>(GPA(b"alcDestroyContext\x00"
                                                             as *const u8 as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char));
    qalcGetCurrentContext =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCGETCURRENTCONTEXT>(GPA(b"alcGetCurrentContext\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char));
    qalcGetContextsDevice =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCGETCONTEXTSDEVICE>(GPA(b"alcGetContextsDevice\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char));
    qalcOpenDevice =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCOPENDEVICE>(GPA(b"alcOpenDevice\x00" as
                                                         *const u8 as
                                                         *const libc::c_char
                                                         as
                                                         *mut libc::c_char));
    qalcCloseDevice =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCCLOSEDEVICE>(GPA(b"alcCloseDevice\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char));
    qalcGetError =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCGETERROR>(GPA(b"alcGetError\x00" as
                                                       *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char));
    qalcIsExtensionPresent =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCISEXTENSIONPRESENT>(GPA(b"alcIsExtensionPresent\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char
                                                                 as
                                                                 *mut libc::c_char));
    qalcGetProcAddress =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCGETPROCADDRESS>(GPA(b"alcGetProcAddress\x00"
                                                             as *const u8 as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char));
    qalcGetEnumValue =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCGETENUMVALUE>(GPA(b"alcGetEnumValue\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char));
    qalcGetString =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCGETSTRING>(GPA(b"alcGetString\x00" as
                                                        *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char));
    qalcGetIntegerv =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCGETINTEGERV>(GPA(b"alcGetIntegerv\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char));
    qalcCaptureOpenDevice =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCCAPTUREOPENDEVICE>(GPA(b"alcCaptureOpenDevice\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char));
    qalcCaptureCloseDevice =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCCAPTURECLOSEDEVICE>(GPA(b"alcCaptureCloseDevice\x00"
                                                                 as *const u8
                                                                 as
                                                                 *const libc::c_char
                                                                 as
                                                                 *mut libc::c_char));
    qalcCaptureStart =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCCAPTURESTART>(GPA(b"alcCaptureStart\x00"
                                                           as *const u8 as
                                                           *const libc::c_char
                                                           as
                                                           *mut libc::c_char));
    qalcCaptureStop =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCCAPTURESTOP>(GPA(b"alcCaptureStop\x00" as
                                                          *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char));
    qalcCaptureSamples =
        ::std::mem::transmute::<*mut libc::c_void,
                                LPALCCAPTURESAMPLES>(GPA(b"alcCaptureSamples\x00"
                                                             as *const u8 as
                                                             *const libc::c_char
                                                             as
                                                             *mut libc::c_char));
    if 0 != alinit_fail as u64 {
        QAL_Shutdown();
        Com_Printf(b" One or more symbols not found\n\x00" as *const u8 as
                       *const libc::c_char);
        return qfalse
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn QAL_Shutdown() {
    if !OpenALLib.is_null() {
        SDL_UnloadObject(OpenALLib);
        OpenALLib = 0 as *mut libc::c_void
    }
    qalEnable = None;
    qalDisable = None;
    qalIsEnabled = None;
    qalGetString = None;
    qalGetBooleanv = None;
    qalGetIntegerv = None;
    qalGetFloatv = None;
    qalGetDoublev = None;
    qalGetBoolean = None;
    qalGetInteger = None;
    qalGetFloat = None;
    qalGetDouble = None;
    qalGetError = None;
    qalIsExtensionPresent = None;
    qalGetProcAddress = None;
    qalGetEnumValue = None;
    qalListenerf = None;
    qalListener3f = None;
    qalListenerfv = None;
    qalListeneri = None;
    qalGetListenerf = None;
    qalGetListener3f = None;
    qalGetListenerfv = None;
    qalGetListeneri = None;
    qalGenSources = None;
    qalDeleteSources = None;
    qalIsSource = None;
    qalSourcef = None;
    qalSource3f = None;
    qalSourcefv = None;
    qalSourcei = None;
    qalGetSourcef = None;
    qalGetSource3f = None;
    qalGetSourcefv = None;
    qalGetSourcei = None;
    qalSourcePlayv = None;
    qalSourceStopv = None;
    qalSourceRewindv = None;
    qalSourcePausev = None;
    qalSourcePlay = None;
    qalSourceStop = None;
    qalSourceRewind = None;
    qalSourcePause = None;
    qalSourceQueueBuffers = None;
    qalSourceUnqueueBuffers = None;
    qalGenBuffers = None;
    qalDeleteBuffers = None;
    qalIsBuffer = None;
    qalBufferData = None;
    qalGetBufferf = None;
    qalGetBufferi = None;
    qalDopplerFactor = None;
    qalSpeedOfSound = None;
    qalDistanceModel = None;
    qalcCreateContext = None;
    qalcMakeContextCurrent = None;
    qalcProcessContext = None;
    qalcSuspendContext = None;
    qalcDestroyContext = None;
    qalcGetCurrentContext = None;
    qalcGetContextsDevice = None;
    qalcOpenDevice = None;
    qalcCloseDevice = None;
    qalcGetError = None;
    qalcIsExtensionPresent = None;
    qalcGetProcAddress = None;
    qalcGetEnumValue = None;
    qalcGetString = None;
    qalcGetIntegerv = None;
    qalcCaptureOpenDevice = None;
    qalcCaptureCloseDevice = None;
    qalcCaptureStart = None;
    qalcCaptureStop = None;
    qalcCaptureSamples = None;
}
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
// Dynamically loads OpenAL
static mut OpenALLib: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
static mut alinit_fail: qboolean = qfalse;
/*
=================
GPA
=================
*/
unsafe extern "C" fn GPA(mut str: *mut libc::c_char) -> *mut libc::c_void {
    let mut rv: *mut libc::c_void = 0 as *mut libc::c_void;
    rv = SDL_LoadFunction(OpenALLib, str);
    if rv.is_null() {
        Com_Printf(b" Can\'t load symbol %s\n\x00" as *const u8 as
                       *const libc::c_char, str);
        alinit_fail = qtrue;
        return 0 as *mut libc::c_void
    } else {
        Com_DPrintf(b" Loaded symbol %s (%p)\n\x00" as *const u8 as
                        *const libc::c_char, str, rv);
        return rv
    };
}