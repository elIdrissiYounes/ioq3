use libc;
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
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    pub type sfxHandle_t = libc::c_int;
    // parameters to the main Error routine
    pub type unnamed = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed = 0;
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
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
    //=====================================================================
    // in order from highest priority to lowest
// if none of the catchers are active, bound key strings will be executed
    // sound channels
// channel 0 never willingly overrides
// other channels will allways override a playing sound on that channel
    pub type unnamed_0 = libc::c_uint;
    // announcer voices, etc
    pub const CHAN_ANNOUNCER: unnamed_0 = 7;
    // chat messages, etc
    pub const CHAN_LOCAL_SOUND: unnamed_0 = 6;
    pub const CHAN_BODY: unnamed_0 = 5;
    pub const CHAN_ITEM: unnamed_0 = 4;
    pub const CHAN_VOICE: unnamed_0 = 3;
    pub const CHAN_WEAPON: unnamed_0 = 2;
    // menu sounds, etc
    pub const CHAN_LOCAL: unnamed_0 = 1;
    pub const CHAN_AUTO: unnamed_0 = 0;
    use super::{libc};
    extern "C" {
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    // Pulls off \n terminated lines of text from the command buffer and sends
// them through Cmd_ExecuteString.  Stops when the buffer is empty.
// Normally called once per frame, but may be explicitly invoked.
// Do not call inside a command function, or current args will be destroyed.
    //===========================================================================
    /*

Command execution takes a null terminated string, breaks it into tokens,
then searches for a command or variable that matches the first token.

*/
    pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
    use super::{libc};
    use super::q_shared_h::{cvar_t};
    extern "C" {
        #[no_mangle]
        pub fn Cmd_AddCommand(cmd_name: *const libc::c_char,
                              function: xcommand_t);
        // called by the init functions of other parts of the program to
// register commands and functions to call for them.
// The cmd_name is referenced later, so it should not be in temp memory
// if function is NULL, the command will be forwarded to the server
// as a clc_clientCommand instead of executed locally
        #[no_mangle]
        pub fn Cmd_RemoveCommand(cmd_name: *const libc::c_char);
        #[no_mangle]
        pub fn Cmd_Argc() -> libc::c_int;
        #[no_mangle]
        pub fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
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
        // updates an interpreted modules' version of a cvar
        #[no_mangle]
        pub fn Cvar_Set(var_name: *const libc::c_char,
                        value: *const libc::c_char);
        #[no_mangle]
        pub static mut com_unfocused: *mut cvar_t;
        #[no_mangle]
        pub static mut com_minimized: *mut cvar_t;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_local.h"]
pub mod snd_local_h {
    // Interface between Q3 sound "api" and the sound backend
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct soundInterface_t {
        pub Shutdown: Option<unsafe extern "C" fn() -> ()>,
        pub StartSound: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: sfxHandle_t) -> ()>,
        pub StartLocalSound: Option<unsafe extern "C" fn(_: sfxHandle_t,
                                                         _: libc::c_int)
                                        -> ()>,
        pub StartBackgroundTrack: Option<unsafe extern "C" fn(_:
                                                                  *const libc::c_char,
                                                              _:
                                                                  *const libc::c_char)
                                             -> ()>,
        pub StopBackgroundTrack: Option<unsafe extern "C" fn() -> ()>,
        pub RawSamples: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: libc::c_int,
                                                    _: *const byte,
                                                    _: libc::c_float,
                                                    _: libc::c_int) -> ()>,
        pub StopAllSounds: Option<unsafe extern "C" fn() -> ()>,
        pub ClearLoopingSounds: Option<unsafe extern "C" fn(_: qboolean)
                                           -> ()>,
        pub AddLoopingSound: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: *const vec_t,
                                                         _: *const vec_t,
                                                         _: sfxHandle_t)
                                        -> ()>,
        pub AddRealLoopingSound: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: *const vec_t,
                                                             _: *const vec_t,
                                                             _: sfxHandle_t)
                                            -> ()>,
        pub StopLoopingSound: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub Respatialize: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *const vec_t,
                                                      _: *mut vec3_t,
                                                      _: libc::c_int) -> ()>,
        pub UpdateEntityPosition: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _: *const vec_t)
                                             -> ()>,
        pub Update: Option<unsafe extern "C" fn() -> ()>,
        pub DisableSounds: Option<unsafe extern "C" fn() -> ()>,
        pub BeginRegistration: Option<unsafe extern "C" fn() -> ()>,
        pub RegisterSound: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                       _: qboolean)
                                      -> sfxHandle_t>,
        pub ClearSoundBuffer: Option<unsafe extern "C" fn() -> ()>,
        pub SoundInfo: Option<unsafe extern "C" fn() -> ()>,
        pub SoundList: Option<unsafe extern "C" fn() -> ()>,
        pub StartCapture: Option<unsafe extern "C" fn() -> ()>,
        pub AvailableCaptureSamples: Option<unsafe extern "C" fn()
                                                -> libc::c_int>,
        pub Capture: Option<unsafe extern "C" fn(_: libc::c_int, _: *mut byte)
                                -> ()>,
        pub StopCapture: Option<unsafe extern "C" fn() -> ()>,
        pub MasterGain: Option<unsafe extern "C" fn(_: libc::c_float) -> ()>,
    }
    use super::q_shared_h::{vec_t, sfxHandle_t, byte, qboolean, vec3_t,
                            cvar_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn S_Base_Init(si_0: *mut soundInterface_t) -> qboolean;
        #[no_mangle]
        pub fn S_AL_Init(si_0: *mut soundInterface_t) -> qboolean;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_main.c"]
pub mod snd_main_c {
    use super::q_shared_h::{cvar_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_public.h"]
pub mod snd_public_h {
    use super::q_shared_h::{vec_t, sfxHandle_t, byte, qboolean, vec3_t};
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_codec.h"]
pub mod snd_codec_h {
    extern "C" {
        // Codec management
        #[no_mangle]
        pub fn S_CodecInit();
        #[no_mangle]
        pub fn S_CodecShutdown();
    }
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, sfxHandle_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, vec_t, vec3_t, cvar_s, cvar_t,
                       unnamed_0, CHAN_ANNOUNCER, CHAN_LOCAL_SOUND, CHAN_BODY,
                       CHAN_ITEM, CHAN_VOICE, CHAN_WEAPON, CHAN_LOCAL,
                       CHAN_AUTO, Com_Error, Com_Printf};
use self::qcommon_h::{xcommand_t, Cmd_AddCommand, Cmd_RemoveCommand, Cmd_Argc,
                      Cmd_Argv, Cvar_Get, Cvar_Set, com_unfocused,
                      com_minimized};
use self::snd_local_h::{soundInterface_t, S_Base_Init, S_AL_Init};
use self::string_h::{memset};
use self::snd_codec_h::{S_CodecInit, S_CodecShutdown};
// for writing the config files
#[no_mangle]
pub unsafe extern "C" fn S_ClearSoundBuffer() {
    if si.ClearSoundBuffer.is_some() {
        si.ClearSoundBuffer.expect("non-null function pointer")();
    };
}
static mut si: soundInterface_t =
    soundInterface_t{Shutdown: None,
                     StartSound: None,
                     StartLocalSound: None,
                     StartBackgroundTrack: None,
                     StopBackgroundTrack: None,
                     RawSamples: None,
                     StopAllSounds: None,
                     ClearLoopingSounds: None,
                     AddLoopingSound: None,
                     AddRealLoopingSound: None,
                     StopLoopingSound: None,
                     Respatialize: None,
                     UpdateEntityPosition: None,
                     Update: None,
                     DisableSounds: None,
                     BeginRegistration: None,
                     RegisterSound: None,
                     ClearSoundBuffer: None,
                     SoundInfo: None,
                     SoundList: None,
                     StartCapture: None,
                     AvailableCaptureSamples: None,
                     Capture: None,
                     StopCapture: None,
                     MasterGain: None,};
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
pub unsafe extern "C" fn S_Init() {
    let mut cv: *mut cvar_t = 0 as *mut cvar_t;
    let mut started: qboolean = qfalse;
    Com_Printf(b"------ Initializing Sound ------\n\x00" as *const u8 as
                   *const libc::c_char);
    s_volume =
        Cvar_Get(b"s_volume\x00" as *const u8 as *const libc::c_char,
                 b"0.8\x00" as *const u8 as *const libc::c_char, 0x1i32);
    s_musicVolume =
        Cvar_Get(b"s_musicvolume\x00" as *const u8 as *const libc::c_char,
                 b"0.25\x00" as *const u8 as *const libc::c_char, 0x1i32);
    s_muted =
        Cvar_Get(b"s_muted\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x40i32);
    s_doppler =
        Cvar_Get(b"s_doppler\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    s_backend =
        Cvar_Get(b"s_backend\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0x40i32);
    s_muteWhenMinimized =
        Cvar_Get(b"s_muteWhenMinimized\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    s_muteWhenUnfocused =
        Cvar_Get(b"s_muteWhenUnfocused\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    cv =
        Cvar_Get(b"s_initsound\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0i32);
    if 0 == (*cv).integer {
        Com_Printf(b"Sound disabled.\n\x00" as *const u8 as
                       *const libc::c_char);
    } else {
        S_CodecInit();
        Cmd_AddCommand(b"play\x00" as *const u8 as *const libc::c_char,
                       Some(S_Play_f));
        Cmd_AddCommand(b"music\x00" as *const u8 as *const libc::c_char,
                       Some(S_Music_f));
        Cmd_AddCommand(b"stopmusic\x00" as *const u8 as *const libc::c_char,
                       Some(S_StopMusic_f));
        Cmd_AddCommand(b"s_list\x00" as *const u8 as *const libc::c_char,
                       Some(S_SoundList));
        Cmd_AddCommand(b"s_stop\x00" as *const u8 as *const libc::c_char,
                       Some(S_StopAllSounds));
        Cmd_AddCommand(b"s_info\x00" as *const u8 as *const libc::c_char,
                       Some(S_SoundInfo));
        cv =
            Cvar_Get(b"s_useOpenAL\x00" as *const u8 as *const libc::c_char,
                     b"1\x00" as *const u8 as *const libc::c_char,
                     0x1i32 | 0x20i32);
        if 0 != (*cv).integer {
            started = S_AL_Init(&mut si);
            Cvar_Set(b"s_backend\x00" as *const u8 as *const libc::c_char,
                     b"OpenAL\x00" as *const u8 as *const libc::c_char);
        }
        if 0 == started as u64 {
            started = S_Base_Init(&mut si);
            Cvar_Set(b"s_backend\x00" as *const u8 as *const libc::c_char,
                     b"base\x00" as *const u8 as *const libc::c_char);
        }
        if 0 != started as u64 {
            if 0 == S_ValidSoundInterface(&mut si) as u64 {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"Sound interface invalid\x00" as *const u8 as
                              *const libc::c_char);
            }
            S_SoundInfo();
            Com_Printf(b"Sound initialization successful.\n\x00" as *const u8
                           as *const libc::c_char);
        } else {
            Com_Printf(b"Sound initialization failed.\n\x00" as *const u8 as
                           *const libc::c_char);
        }
    }
    Com_Printf(b"--------------------------------\n\x00" as *const u8 as
                   *const libc::c_char);
}
/*
=================
S_SoundInfo
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_SoundInfo() {
    if si.SoundInfo.is_some() {
        si.SoundInfo.expect("non-null function pointer")();
    };
}
/*
=================
S_ValidateInterface
=================
*/
unsafe extern "C" fn S_ValidSoundInterface(mut si_0: *mut soundInterface_t)
 -> qboolean {
    if (*si_0).Shutdown.is_none() { return qfalse }
    if (*si_0).StartSound.is_none() { return qfalse }
    if (*si_0).StartLocalSound.is_none() { return qfalse }
    if (*si_0).StartBackgroundTrack.is_none() { return qfalse }
    if (*si_0).StopBackgroundTrack.is_none() { return qfalse }
    if (*si_0).RawSamples.is_none() { return qfalse }
    if (*si_0).StopAllSounds.is_none() { return qfalse }
    if (*si_0).ClearLoopingSounds.is_none() { return qfalse }
    if (*si_0).AddLoopingSound.is_none() { return qfalse }
    if (*si_0).AddRealLoopingSound.is_none() { return qfalse }
    if (*si_0).StopLoopingSound.is_none() { return qfalse }
    if (*si_0).Respatialize.is_none() { return qfalse }
    if (*si_0).UpdateEntityPosition.is_none() { return qfalse }
    if (*si_0).Update.is_none() { return qfalse }
    if (*si_0).DisableSounds.is_none() { return qfalse }
    if (*si_0).BeginRegistration.is_none() { return qfalse }
    if (*si_0).RegisterSound.is_none() { return qfalse }
    if (*si_0).ClearSoundBuffer.is_none() { return qfalse }
    if (*si_0).SoundInfo.is_none() { return qfalse }
    if (*si_0).SoundList.is_none() { return qfalse }
    if (*si_0).StartCapture.is_none() { return qfalse }
    if (*si_0).AvailableCaptureSamples.is_none() { return qfalse }
    if (*si_0).Capture.is_none() { return qfalse }
    if (*si_0).StopCapture.is_none() { return qfalse }
    if (*si_0).MasterGain.is_none() { return qfalse }
    return qtrue;
}
// stop all sounds and the background track
#[no_mangle]
pub unsafe extern "C" fn S_StopAllSounds() {
    if si.StopAllSounds.is_some() {
        si.StopAllSounds.expect("non-null function pointer")();
    };
}
/*
=================
S_SoundList
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_SoundList() {
    if si.SoundList.is_some() {
        si.SoundList.expect("non-null function pointer")();
    };
}
/*
=================
S_Music_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_StopMusic_f() {
    if si.StopBackgroundTrack.is_none() { return }
    si.StopBackgroundTrack.expect("non-null function pointer")();
}
/*
=================
S_Music_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Music_f() {
    let mut c: libc::c_int = 0;
    if si.StartBackgroundTrack.is_none() { return }
    c = Cmd_Argc();
    if c == 2i32 {
        si.StartBackgroundTrack.expect("non-null function pointer")(Cmd_Argv(1i32),
                                                                    0 as
                                                                        *const libc::c_char);
    } else if c == 3i32 {
        si.StartBackgroundTrack.expect("non-null function pointer")(Cmd_Argv(1i32),
                                                                    Cmd_Argv(2i32));
    } else {
        Com_Printf(b"Usage: music <musicfile> [loopfile]\n\x00" as *const u8
                       as *const libc::c_char);
        return
    };
}
//=============================================================================
/*
=================
S_Play_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn S_Play_f() {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut h: sfxHandle_t = 0;
    if si.RegisterSound.is_none() || si.StartLocalSound.is_none() { return }
    c = Cmd_Argc();
    if c < 2i32 {
        Com_Printf(b"Usage: play <sound filename> [sound filename] [sound filename] ...\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    i = 1i32;
    while i < c {
        h =
            si.RegisterSound.expect("non-null function pointer")(Cmd_Argv(i),
                                                                 qfalse);
        if 0 != h {
            si.StartLocalSound.expect("non-null function pointer")(h,
                                                                   CHAN_LOCAL_SOUND
                                                                       as
                                                                       libc::c_int);
        }
        i += 1
    };
}
#[no_mangle]
pub static mut s_muteWhenUnfocused: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_muteWhenMinimized: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
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
#[no_mangle]
pub static mut s_backend: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_doppler: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_muted: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_musicVolume: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut s_volume: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn S_Shutdown() {
    if si.Shutdown.is_some() {
        si.Shutdown.expect("non-null function pointer")();
    }
    memset(&mut si as *mut soundInterface_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<soundInterface_t>() as libc::c_ulong);
    Cmd_RemoveCommand(b"play\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"music\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"stopmusic\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"s_list\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"s_stop\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"s_info\x00" as *const u8 as *const libc::c_char);
    S_CodecShutdown();
}
// if origin is NULL, the sound will be dynamically sourced from the entity
#[no_mangle]
pub unsafe extern "C" fn S_StartSound(mut origin: *mut vec_t,
                                      mut entnum: libc::c_int,
                                      mut entchannel: libc::c_int,
                                      mut sfx: sfxHandle_t) {
    if si.StartSound.is_some() {
        si.StartSound.expect("non-null function pointer")(origin, entnum,
                                                          entchannel, sfx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_StartLocalSound(mut sfx: sfxHandle_t,
                                           mut channelNum: libc::c_int) {
    if si.StartLocalSound.is_some() {
        si.StartLocalSound.expect("non-null function pointer")(sfx,
                                                               channelNum);
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_StartBackgroundTrack(mut intro:
                                                    *const libc::c_char,
                                                mut loop_0:
                                                    *const libc::c_char) {
    if si.StartBackgroundTrack.is_some() {
        si.StartBackgroundTrack.expect("non-null function pointer")(intro,
                                                                    loop_0);
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_StopBackgroundTrack() {
    if si.StopBackgroundTrack.is_some() {
        si.StopBackgroundTrack.expect("non-null function pointer")();
    };
}
// cinematics and voice-over-network will send raw samples
// 1.0 volume will be direct output of source samples
#[no_mangle]
pub unsafe extern "C" fn S_RawSamples(mut stream: libc::c_int,
                                      mut samples: libc::c_int,
                                      mut rate: libc::c_int,
                                      mut width: libc::c_int,
                                      mut channels: libc::c_int,
                                      mut data: *const byte,
                                      mut volume: libc::c_float,
                                      mut entityNum: libc::c_int) {
    if si.RawSamples.is_some() {
        si.RawSamples.expect("non-null function pointer")(stream, samples,
                                                          rate, width,
                                                          channels, data,
                                                          volume, entityNum);
    };
}
// all continuous looping sounds must be added before calling S_Update
#[no_mangle]
pub unsafe extern "C" fn S_ClearLoopingSounds(mut killall: qboolean) {
    if si.ClearLoopingSounds.is_some() {
        si.ClearLoopingSounds.expect("non-null function pointer")(killall);
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_AddLoopingSound(mut entityNum: libc::c_int,
                                           mut origin: *const vec_t,
                                           mut velocity: *const vec_t,
                                           mut sfx: sfxHandle_t) {
    if si.AddLoopingSound.is_some() {
        si.AddLoopingSound.expect("non-null function pointer")(entityNum,
                                                               origin,
                                                               velocity, sfx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_AddRealLoopingSound(mut entityNum: libc::c_int,
                                               mut origin: *const vec_t,
                                               mut velocity: *const vec_t,
                                               mut sfx: sfxHandle_t) {
    if si.AddRealLoopingSound.is_some() {
        si.AddRealLoopingSound.expect("non-null function pointer")(entityNum,
                                                                   origin,
                                                                   velocity,
                                                                   sfx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_StopLoopingSound(mut entityNum: libc::c_int) {
    if si.StopLoopingSound.is_some() {
        si.StopLoopingSound.expect("non-null function pointer")(entityNum);
    };
}
// recompute the relative volumes for all running sounds
// relative to the given entityNum / orientation
#[no_mangle]
pub unsafe extern "C" fn S_Respatialize(mut entityNum: libc::c_int,
                                        mut origin: *const vec_t,
                                        mut axis: *mut vec3_t,
                                        mut inwater: libc::c_int) {
    if si.Respatialize.is_some() {
        si.Respatialize.expect("non-null function pointer")(entityNum, origin,
                                                            axis, inwater);
    };
}
// let the sound system know where an entity currently is
#[no_mangle]
pub unsafe extern "C" fn S_UpdateEntityPosition(mut entityNum: libc::c_int,
                                                mut origin: *const vec_t) {
    if si.UpdateEntityPosition.is_some() {
        si.UpdateEntityPosition.expect("non-null function pointer")(entityNum,
                                                                    origin);
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_Update() {
    if 0 != (*s_muted).integer {
        if !(0 != (*s_muteWhenMinimized).integer &&
                 0 != (*com_minimized).integer) &&
               !(0 != (*s_muteWhenUnfocused).integer &&
                     0 != (*com_unfocused).integer) {
            (*s_muted).integer = qfalse as libc::c_int;
            (*s_muted).modified = qtrue
        }
    } else if 0 != (*s_muteWhenMinimized).integer &&
                  0 != (*com_minimized).integer ||
                  0 != (*s_muteWhenUnfocused).integer &&
                      0 != (*com_unfocused).integer {
        (*s_muted).integer = qtrue as libc::c_int;
        (*s_muted).modified = qtrue
    }
    if si.Update.is_some() {
        si.Update.expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_DisableSounds() {
    if si.DisableSounds.is_some() {
        si.DisableSounds.expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_BeginRegistration() {
    if si.BeginRegistration.is_some() {
        si.BeginRegistration.expect("non-null function pointer")();
    };
}
// RegisterSound will allways return a valid sample, even if it
// has to create a placeholder.  This prevents continuous filesystem
// checks for missing files
#[no_mangle]
pub unsafe extern "C" fn S_RegisterSound(mut sample: *const libc::c_char,
                                         mut compressed: qboolean)
 -> sfxHandle_t {
    if si.RegisterSound.is_some() {
        return si.RegisterSound.expect("non-null function pointer")(sample,
                                                                    compressed)
    } else { return 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn S_StartCapture() {
    if si.StartCapture.is_some() {
        si.StartCapture.expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_AvailableCaptureSamples() -> libc::c_int {
    if si.AvailableCaptureSamples.is_some() {
        return si.AvailableCaptureSamples.expect("non-null function pointer")()
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn S_Capture(mut samples: libc::c_int,
                                   mut data: *mut byte) {
    if si.Capture.is_some() {
        si.Capture.expect("non-null function pointer")(samples, data);
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_StopCapture() {
    if si.StopCapture.is_some() {
        si.StopCapture.expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn S_MasterGain(mut gain: libc::c_float) {
    if si.MasterGain.is_some() {
        si.MasterGain.expect("non-null function pointer")(gain);
    };
}