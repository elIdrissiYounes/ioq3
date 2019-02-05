use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    use super::{libc};
}
#[header_src = "/usr/include/ctype.h"]
pub mod ctype_h {
    pub type unnamed = libc::c_uint;
    pub const _ISalnum: unnamed = 8;
    pub const _ISpunct: unnamed = 4;
    pub const _IScntrl: unnamed = 2;
    pub const _ISblank: unnamed = 1;
    pub const _ISgraph: unnamed = 32768;
    pub const _ISprint: unnamed = 16384;
    pub const _ISspace: unnamed = 8192;
    pub const _ISxdigit: unnamed = 4096;
    pub const _ISdigit: unnamed = 2048;
    pub const _ISalpha: unnamed = 1024;
    pub const _ISlower: unnamed = 512;
    pub const _ISupper: unnamed = 256;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint8_t};
}
#[header_src = "/usr/include/stdint.h"]
pub mod stdint_h {
    pub type intptr_t = libc::c_long;
    use super::{libc};
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
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    pub type qhandle_t = libc::c_int;
    pub type fileHandle_t = libc::c_int;
    // parameters to the main Error routine
    pub type unnamed_0 = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed_0 = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed_0 = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed_0 = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed_0 = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed_0 = 0;
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
    pub type connstate_t = libc::c_uint;
    // playing a cinematic or a static pic, not connected to a server
    pub const CA_CINEMATIC: connstate_t = 9;
    // game views should be displayed
    pub const CA_ACTIVE: connstate_t = 8;
    // got gamestate, waiting for first frame
    pub const CA_PRIMED: connstate_t = 7;
    // only during cgame initialization, never during main loop
    pub const CA_LOADING: connstate_t = 6;
    // netchan_t established, getting gamestate
    pub const CA_CONNECTED: connstate_t = 5;
    // sending challenge packets to the server
    pub const CA_CHALLENGING: connstate_t = 4;
    // sending request packets to the server
    pub const CA_CONNECTING: connstate_t = 3;
    // not used any more, was checking cd key 
    pub const CA_AUTHORIZING: connstate_t = 2;
    // not talking to a server
    pub const CA_DISCONNECTED: connstate_t = 1;
    pub const CA_UNINITIALIZED: connstate_t = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Com_HexStrToInt(str: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        #[no_mangle]
        pub fn Com_SkipTokens(s: *mut libc::c_char, numTokens: libc::c_int,
                              sep: *mut libc::c_char) -> *mut libc::c_char;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int,
                        src: *const libc::c_char);
        //=============================================
/*
short	BigShort(short l);
short	LittleShort(short l);
int		BigLong (int l);
int		LittleLong (int l);
qint64  BigLong64 (qint64 l);
qint64  LittleLong64 (qint64 l);
float	BigFloat (const float *l);
float	LittleFloat (const float *l);

void	Swap_Init (void);
*/
        #[no_mangle]
        pub fn va(format: *mut libc::c_char, ...) -> *mut libc::c_char;
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
    //============================================================================
    /*
==============================================================

NET

==============================================================
*/
    // if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
    // disables ipv6 multicast support if set.
    // number of old messages that must be kept on client and
    // server for delta comrpession and ping estimation
    // max number of usercmd_t in a packet
    // max string commands buffered for restransmit
    pub type netadrtype_t = libc::c_uint;
    pub const NA_UNSPEC: netadrtype_t = 7;
    pub const NA_MULTICAST6: netadrtype_t = 6;
    pub const NA_IP6: netadrtype_t = 5;
    pub const NA_IP: netadrtype_t = 4;
    pub const NA_BROADCAST: netadrtype_t = 3;
    pub const NA_LOOPBACK: netadrtype_t = 2;
    pub const NA_BOT: netadrtype_t = 1;
    // an address lookup failed
    pub const NA_BAD: netadrtype_t = 0;
    pub type netsrc_t = libc::c_uint;
    pub const NS_SERVER: netsrc_t = 1;
    pub const NS_CLIENT: netsrc_t = 0;
    // maximum length of an IPv6 address string including trailing '\0'
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netadr_t {
        pub type_0: netadrtype_t,
        pub ip: [byte; 4],
        pub ip6: [byte; 16],
        pub port: libc::c_ushort,
        pub scope_id: libc::c_ulong,
    }
    // max length of a message, which may
    // be fragmented into multiple packets
    // ACK window of 48 download chunks. Cannot set this higher, or clients
    // will overflow the reliable commands buffer
    // 896 byte block chunks
    /*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netchan_t {
        pub sock: netsrc_t,
        pub dropped: libc::c_int,
        pub remoteAddress: netadr_t,
        pub qport: libc::c_int,
        pub incomingSequence: libc::c_int,
        pub outgoingSequence: libc::c_int,
        pub fragmentSequence: libc::c_int,
        pub fragmentLength: libc::c_int,
        pub fragmentBuffer: [byte; 16384],
        pub unsentFragments: qboolean,
        pub unsentFragmentStart: libc::c_int,
        pub unsentLength: libc::c_int,
        pub unsentBuffer: [byte; 16384],
        pub challenge: libc::c_int,
        pub lastSentTime: libc::c_int,
        pub lastSentSize: libc::c_int,
        pub compat: qboolean,
    }
    pub type vm_t = vm_s;
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
    pub type completionFunc_t
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
                   -> ()>;
    /*
==============================================================

Edit fields and command line history/completion

==============================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct field_t {
        pub cursor: libc::c_int,
        pub scroll: libc::c_int,
        pub widthInChars: libc::c_int,
        pub buffer: [libc::c_char; 256],
    }
    use super::{libc};
    use super::q_shared_h::{byte, qboolean, fileHandle_t, cvar_t};
    use super::stdint_h::{intptr_t};
    extern "C" {
        /*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
        pub type vm_s;
        #[no_mangle]
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
        // allocates an initial text buffer that will grow as needed
        #[no_mangle]
        pub fn Cbuf_AddText(text: *const libc::c_char);
        #[no_mangle]
        pub fn Cmd_AddCommand(cmd_name: *const libc::c_char,
                              function: xcommand_t);
        // callback with each valid string
        #[no_mangle]
        pub fn Cmd_SetCommandCompletionFunc(command: *const libc::c_char,
                                            complete: completionFunc_t);
        #[no_mangle]
        pub fn Cmd_Argc() -> libc::c_int;
        #[no_mangle]
        pub fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
        // updates an interpreted modules' version of a cvar
        #[no_mangle]
        pub fn Cvar_Set(var_name: *const libc::c_char,
                        value: *const libc::c_char);
        // don't set the cvar immediately
        #[no_mangle]
        pub fn Cvar_SetValue(var_name: *const libc::c_char,
                             value: libc::c_float);
        // expands value to a string and calls Cvar_Set/Cvar_SetSafe
        #[no_mangle]
        pub fn Cvar_VariableValue(var_name: *const libc::c_char)
         -> libc::c_float;
        #[no_mangle]
        pub fn Cvar_VariableIntegerValue(var_name: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub static mut cvar_modifiedFlags: libc::c_int;
        #[no_mangle]
        pub fn FS_FOpenFileWrite(qpath: *const libc::c_char) -> fileHandle_t;
        #[no_mangle]
        pub fn FS_FOpenFileRead(qpath: *const libc::c_char,
                                file: *mut fileHandle_t, uniqueFILE: qboolean)
         -> libc::c_long;
        // returns 1 if a file is in the PAK file, otherwise -1
        #[no_mangle]
        pub fn FS_Write(buffer: *const libc::c_void, len: libc::c_int,
                        f: fileHandle_t) -> libc::c_int;
        #[no_mangle]
        pub fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int,
                       f: fileHandle_t) -> libc::c_int;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        #[no_mangle]
        pub fn FS_Printf(f: fileHandle_t, fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn Field_Clear(edit: *mut field_t);
        #[no_mangle]
        pub fn Field_AutoComplete(edit: *mut field_t);
        #[no_mangle]
        pub fn Field_CompleteKeyname();
        #[no_mangle]
        pub fn Field_CompleteCommand(cmd: *mut libc::c_char,
                                     doCommands: qboolean, doCvars: qboolean);
        #[no_mangle]
        pub fn CopyString(in_0: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub static mut con_autochat: *mut cvar_t;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        // note that this isn't journaled...
        #[no_mangle]
        pub fn Sys_GetClipboardData() -> *mut libc::c_char;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/keys.h"]
pub mod keys_h {
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct qkey_t {
        pub down: qboolean,
        pub repeats: libc::c_int,
        pub binding: *mut libc::c_char,
    }
    use super::q_shared_h::{qboolean};
    use super::{libc};
    use super::qcommon_h::{field_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/cl_keys.c"]
pub mod cl_keys_c {
    // will be <= nextHistoryLine
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct keyname_t {
        pub name: *mut libc::c_char,
        pub keynum: libc::c_int,
    }
    use super::{libc};
    use super::q_shared_h::{qboolean};
    use super::qcommon_h::{field_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/keycodes.h"]
pub mod keycodes_h {
    pub const K_PAD0_RIGHTTRIGGER: unnamed_3 = 364;
    pub const K_PAD0_LEFTTRIGGER: unnamed_3 = 363;
    pub const K_PAD0_RIGHTSTICK_DOWN: unnamed_3 = 362;
    pub const K_PAD0_RIGHTSTICK_UP: unnamed_3 = 361;
    pub const K_PAD0_RIGHTSTICK_RIGHT: unnamed_3 = 360;
    pub const K_PAD0_RIGHTSTICK_LEFT: unnamed_3 = 359;
    pub const K_PAD0_LEFTSTICK_DOWN: unnamed_3 = 358;
    pub const K_PAD0_LEFTSTICK_UP: unnamed_3 = 357;
    pub const K_PAD0_LEFTSTICK_RIGHT: unnamed_3 = 356;
    pub const K_PAD0_LEFTSTICK_LEFT: unnamed_3 = 355;
    pub const K_PAD0_DPAD_RIGHT: unnamed_3 = 354;
    pub const K_PAD0_DPAD_LEFT: unnamed_3 = 353;
    pub const K_PAD0_DPAD_DOWN: unnamed_3 = 352;
    pub const K_PAD0_DPAD_UP: unnamed_3 = 351;
    pub const K_PAD0_RIGHTSHOULDER: unnamed_3 = 350;
    pub const K_PAD0_LEFTSHOULDER: unnamed_3 = 349;
    pub const K_PAD0_RIGHTSTICK_CLICK: unnamed_3 = 348;
    pub const K_PAD0_LEFTSTICK_CLICK: unnamed_3 = 347;
    pub const K_PAD0_START: unnamed_3 = 346;
    pub const K_PAD0_GUIDE: unnamed_3 = 345;
    pub const K_PAD0_BACK: unnamed_3 = 344;
    pub const K_PAD0_Y: unnamed_3 = 343;
    pub const K_PAD0_X: unnamed_3 = 342;
    pub const K_PAD0_B: unnamed_3 = 341;
    // Gamepad controls
	// Ordered to match SDL2 game controller buttons and axes
	// Do not change this order without also changing IN_GamepadMove() in SDL_input.c
    pub const K_PAD0_A: unnamed_3 = 340;
    pub const K_UNDO: unnamed_3 = 339;
    pub const K_EURO: unnamed_3 = 338;
    pub const K_POWER: unnamed_3 = 130;
    pub const K_MENU: unnamed_3 = 337;
    pub const K_BREAK: unnamed_3 = 336;
    pub const K_SCROLLOCK: unnamed_3 = 335;
    pub const K_SYSREQ: unnamed_3 = 334;
    pub const K_PRINT: unnamed_3 = 333;
    pub const K_HELP: unnamed_3 = 332;
    pub const K_MODE: unnamed_3 = 331;
    pub const K_COMPOSE: unnamed_3 = 330;
    pub const K_SUPER: unnamed_3 = 329;
    pub const K_WORLD_95: unnamed_3 = 328;
    pub const K_WORLD_94: unnamed_3 = 327;
    pub const K_WORLD_93: unnamed_3 = 326;
    pub const K_WORLD_92: unnamed_3 = 325;
    pub const K_WORLD_91: unnamed_3 = 324;
    pub const K_WORLD_90: unnamed_3 = 323;
    pub const K_WORLD_89: unnamed_3 = 322;
    pub const K_WORLD_88: unnamed_3 = 321;
    pub const K_WORLD_87: unnamed_3 = 320;
    pub const K_WORLD_86: unnamed_3 = 319;
    pub const K_WORLD_85: unnamed_3 = 318;
    pub const K_WORLD_84: unnamed_3 = 317;
    pub const K_WORLD_83: unnamed_3 = 316;
    pub const K_WORLD_82: unnamed_3 = 315;
    pub const K_WORLD_81: unnamed_3 = 314;
    pub const K_WORLD_80: unnamed_3 = 313;
    pub const K_WORLD_79: unnamed_3 = 312;
    pub const K_WORLD_78: unnamed_3 = 311;
    pub const K_WORLD_77: unnamed_3 = 310;
    pub const K_WORLD_76: unnamed_3 = 309;
    pub const K_WORLD_75: unnamed_3 = 308;
    pub const K_WORLD_74: unnamed_3 = 307;
    pub const K_WORLD_73: unnamed_3 = 306;
    pub const K_WORLD_72: unnamed_3 = 305;
    pub const K_WORLD_71: unnamed_3 = 304;
    pub const K_WORLD_70: unnamed_3 = 303;
    pub const K_WORLD_69: unnamed_3 = 302;
    pub const K_WORLD_68: unnamed_3 = 301;
    pub const K_WORLD_67: unnamed_3 = 300;
    pub const K_WORLD_66: unnamed_3 = 299;
    pub const K_WORLD_65: unnamed_3 = 298;
    pub const K_WORLD_64: unnamed_3 = 297;
    pub const K_WORLD_63: unnamed_3 = 296;
    pub const K_WORLD_62: unnamed_3 = 295;
    pub const K_WORLD_61: unnamed_3 = 294;
    pub const K_WORLD_60: unnamed_3 = 293;
    pub const K_WORLD_59: unnamed_3 = 292;
    pub const K_WORLD_58: unnamed_3 = 291;
    pub const K_WORLD_57: unnamed_3 = 290;
    pub const K_WORLD_56: unnamed_3 = 289;
    pub const K_WORLD_55: unnamed_3 = 288;
    pub const K_WORLD_54: unnamed_3 = 287;
    pub const K_WORLD_53: unnamed_3 = 286;
    pub const K_WORLD_52: unnamed_3 = 285;
    pub const K_WORLD_51: unnamed_3 = 284;
    pub const K_WORLD_50: unnamed_3 = 283;
    pub const K_WORLD_49: unnamed_3 = 282;
    pub const K_WORLD_48: unnamed_3 = 281;
    pub const K_WORLD_47: unnamed_3 = 280;
    pub const K_WORLD_46: unnamed_3 = 279;
    pub const K_WORLD_45: unnamed_3 = 278;
    pub const K_WORLD_44: unnamed_3 = 277;
    pub const K_WORLD_43: unnamed_3 = 276;
    pub const K_WORLD_42: unnamed_3 = 275;
    pub const K_WORLD_41: unnamed_3 = 274;
    pub const K_WORLD_40: unnamed_3 = 273;
    pub const K_WORLD_39: unnamed_3 = 272;
    pub const K_WORLD_38: unnamed_3 = 271;
    pub const K_WORLD_37: unnamed_3 = 270;
    pub const K_WORLD_36: unnamed_3 = 269;
    pub const K_WORLD_35: unnamed_3 = 268;
    pub const K_WORLD_34: unnamed_3 = 267;
    pub const K_WORLD_33: unnamed_3 = 266;
    pub const K_WORLD_32: unnamed_3 = 265;
    pub const K_WORLD_31: unnamed_3 = 264;
    pub const K_WORLD_30: unnamed_3 = 263;
    pub const K_WORLD_29: unnamed_3 = 262;
    pub const K_WORLD_28: unnamed_3 = 261;
    pub const K_WORLD_27: unnamed_3 = 260;
    pub const K_WORLD_26: unnamed_3 = 259;
    pub const K_WORLD_25: unnamed_3 = 258;
    pub const K_WORLD_24: unnamed_3 = 257;
    pub const K_WORLD_23: unnamed_3 = 256;
    pub const K_WORLD_22: unnamed_3 = 255;
    pub const K_WORLD_21: unnamed_3 = 254;
    pub const K_WORLD_20: unnamed_3 = 253;
    pub const K_WORLD_19: unnamed_3 = 252;
    pub const K_WORLD_18: unnamed_3 = 251;
    pub const K_WORLD_17: unnamed_3 = 250;
    pub const K_WORLD_16: unnamed_3 = 249;
    pub const K_WORLD_15: unnamed_3 = 248;
    pub const K_WORLD_14: unnamed_3 = 247;
    pub const K_WORLD_13: unnamed_3 = 246;
    pub const K_WORLD_12: unnamed_3 = 245;
    pub const K_WORLD_11: unnamed_3 = 244;
    pub const K_WORLD_10: unnamed_3 = 243;
    pub const K_WORLD_9: unnamed_3 = 242;
    pub const K_WORLD_8: unnamed_3 = 241;
    pub const K_WORLD_7: unnamed_3 = 240;
    pub const K_WORLD_6: unnamed_3 = 239;
    pub const K_WORLD_5: unnamed_3 = 238;
    pub const K_WORLD_4: unnamed_3 = 237;
    pub const K_WORLD_3: unnamed_3 = 236;
    pub const K_WORLD_2: unnamed_3 = 235;
    pub const K_WORLD_1: unnamed_3 = 234;
    pub const K_WORLD_0: unnamed_3 = 233;
    pub const K_PAUSE: unnamed_3 = 131;
    pub const K_KP_EQUALS: unnamed_3 = 177;
    pub const K_KP_STAR: unnamed_3 = 176;
    pub const K_KP_NUMLOCK: unnamed_3 = 175;
    pub const K_KP_PLUS: unnamed_3 = 174;
    pub const K_KP_MINUS: unnamed_3 = 173;
    pub const K_KP_SLASH: unnamed_3 = 172;
    pub const K_KP_DEL: unnamed_3 = 171;
    pub const K_KP_INS: unnamed_3 = 170;
    pub const K_KP_ENTER: unnamed_3 = 169;
    pub const K_KP_PGDN: unnamed_3 = 168;
    pub const K_KP_DOWNARROW: unnamed_3 = 167;
    pub const K_KP_END: unnamed_3 = 166;
    pub const K_KP_RIGHTARROW: unnamed_3 = 165;
    pub const K_KP_5: unnamed_3 = 164;
    pub const K_KP_LEFTARROW: unnamed_3 = 163;
    pub const K_KP_PGUP: unnamed_3 = 162;
    pub const K_KP_UPARROW: unnamed_3 = 161;
    pub const K_KP_HOME: unnamed_3 = 160;
    pub const K_AUX16: unnamed_3 = 232;
    pub const K_AUX15: unnamed_3 = 231;
    pub const K_AUX14: unnamed_3 = 230;
    pub const K_AUX13: unnamed_3 = 229;
    pub const K_AUX12: unnamed_3 = 228;
    pub const K_AUX11: unnamed_3 = 227;
    pub const K_AUX10: unnamed_3 = 226;
    pub const K_AUX9: unnamed_3 = 225;
    pub const K_AUX8: unnamed_3 = 224;
    pub const K_AUX7: unnamed_3 = 223;
    pub const K_AUX6: unnamed_3 = 222;
    pub const K_AUX5: unnamed_3 = 221;
    pub const K_AUX4: unnamed_3 = 220;
    pub const K_AUX3: unnamed_3 = 219;
    pub const K_AUX2: unnamed_3 = 218;
    pub const K_AUX1: unnamed_3 = 217;
    pub const K_JOY32: unnamed_3 = 216;
    pub const K_JOY31: unnamed_3 = 215;
    pub const K_JOY30: unnamed_3 = 214;
    pub const K_JOY29: unnamed_3 = 213;
    pub const K_JOY28: unnamed_3 = 212;
    pub const K_JOY27: unnamed_3 = 211;
    pub const K_JOY26: unnamed_3 = 210;
    pub const K_JOY25: unnamed_3 = 209;
    pub const K_JOY24: unnamed_3 = 208;
    pub const K_JOY23: unnamed_3 = 207;
    pub const K_JOY22: unnamed_3 = 206;
    pub const K_JOY21: unnamed_3 = 205;
    pub const K_JOY20: unnamed_3 = 204;
    pub const K_JOY19: unnamed_3 = 203;
    pub const K_JOY18: unnamed_3 = 202;
    pub const K_JOY17: unnamed_3 = 201;
    pub const K_JOY16: unnamed_3 = 200;
    pub const K_JOY15: unnamed_3 = 199;
    pub const K_JOY14: unnamed_3 = 198;
    pub const K_JOY13: unnamed_3 = 197;
    pub const K_JOY12: unnamed_3 = 196;
    pub const K_JOY11: unnamed_3 = 195;
    pub const K_JOY10: unnamed_3 = 194;
    pub const K_JOY9: unnamed_3 = 193;
    pub const K_JOY8: unnamed_3 = 192;
    pub const K_JOY7: unnamed_3 = 191;
    pub const K_JOY6: unnamed_3 = 190;
    pub const K_JOY5: unnamed_3 = 189;
    pub const K_JOY4: unnamed_3 = 188;
    pub const K_JOY3: unnamed_3 = 187;
    pub const K_JOY2: unnamed_3 = 186;
    pub const K_JOY1: unnamed_3 = 185;
    pub const K_MWHEELDOWN: unnamed_3 = 183;
    pub const K_MWHEELUP: unnamed_3 = 184;
    pub const K_MOUSE5: unnamed_3 = 182;
    pub const K_MOUSE4: unnamed_3 = 181;
    pub const K_MOUSE3: unnamed_3 = 180;
    pub const K_MOUSE2: unnamed_3 = 179;
    pub const K_MOUSE1: unnamed_3 = 178;
    pub const K_END: unnamed_3 = 144;
    pub const K_HOME: unnamed_3 = 143;
    pub const K_PGUP: unnamed_3 = 142;
    pub const K_PGDN: unnamed_3 = 141;
    pub const K_DEL: unnamed_3 = 140;
    pub const K_INS: unnamed_3 = 139;
    pub const K_F15: unnamed_3 = 159;
    pub const K_F14: unnamed_3 = 158;
    pub const K_F13: unnamed_3 = 157;
    pub const K_F12: unnamed_3 = 156;
    pub const K_F11: unnamed_3 = 155;
    pub const K_F10: unnamed_3 = 154;
    pub const K_F9: unnamed_3 = 153;
    pub const K_F8: unnamed_3 = 152;
    pub const K_F7: unnamed_3 = 151;
    pub const K_F6: unnamed_3 = 150;
    pub const K_F5: unnamed_3 = 149;
    pub const K_F4: unnamed_3 = 148;
    pub const K_F3: unnamed_3 = 147;
    pub const K_F2: unnamed_3 = 146;
    pub const K_F1: unnamed_3 = 145;
    pub const K_CAPSLOCK: unnamed_3 = 129;
    pub const K_COMMAND: unnamed_3 = 128;
    pub const K_SHIFT: unnamed_3 = 138;
    pub const K_CTRL: unnamed_3 = 137;
    pub const K_ALT: unnamed_3 = 136;
    pub const K_RIGHTARROW: unnamed_3 = 135;
    pub const K_LEFTARROW: unnamed_3 = 134;
    pub const K_DOWNARROW: unnamed_3 = 133;
    pub const K_UPARROW: unnamed_3 = 132;
    pub const K_BACKSPACE: unnamed_3 = 127;
    pub const K_SPACE: unnamed_3 = 32;
    pub const K_ESCAPE: unnamed_3 = 27;
    pub const K_ENTER: unnamed_3 = 13;
    pub const K_TAB: unnamed_3 = 9;
    pub const MAX_KEYS: unnamed_3 = 366;
    // Pseudo-key that brings the console down
    pub const K_CONSOLE: unnamed_3 = 365;
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
    //
// these are the key numbers that should be passed to KeyEvent
//
    // normal keys should be passed as lowercased ascii
    pub type unnamed_3 = libc::c_uint;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/cgame/cg_public.h"]
pub mod cg_public_h {
    //	int (*CG_LastAttacker)( void );
    pub const CG_KEY_EVENT: unnamed_5 = 6;
    pub const CGAME_EVENT_NONE: unnamed_4 = 0;
    //	void	(*CG_MouseEvent)( int dx, int dy );
    pub const CG_EVENT_HANDLING: unnamed_5 = 8;
    pub type unnamed_4 = libc::c_uint;
    pub const CGAME_EVENT_EDITHUD: unnamed_4 = 3;
    pub const CGAME_EVENT_SCOREBOARD: unnamed_4 = 2;
    pub const CGAME_EVENT_TEAMMENU: unnamed_4 = 1;
    /*
==================================================================

functions exported to the main executable

==================================================================
*/
    pub type unnamed_5 = libc::c_uint;
    //	void	(*CG_KeyEvent)( int key, qboolean down );
    pub const CG_MOUSE_EVENT: unnamed_5 = 7;
    //	int (*CG_CrosshairPlayer)( void );
    pub const CG_LAST_ATTACKER: unnamed_5 = 5;
    //	void (*CG_DrawActiveFrame)( int serverTime, stereoFrame_t stereoView, qboolean demoPlayback );
	// Generates and draws a game scene and status information at the given time.
	// If demoPlayback is set, local movement prediction will not be enabled
    pub const CG_CROSSHAIR_PLAYER: unnamed_5 = 4;
    //	qboolean (*CG_ConsoleCommand)( void );
	// a console command has been issued locally that is not recognized by the
	// main game system.
	// use Cmd_Argc() / Cmd_Argv() to read the command, return qfalse if the
	// command is not known to the game
    pub const CG_DRAW_ACTIVE_FRAME: unnamed_5 = 3;
    //	void (*CG_Shutdown)( void );
	// opportunity to flush and close any open files
    pub const CG_CONSOLE_COMMAND: unnamed_5 = 2;
    //	void CG_Init( int serverMessageNum, int serverCommandSequence, int clientNum )
	// called when the level loads or when the renderer is restarted
	// all media should be registered at this time
	// cgame will display loading status by calling SCR_Update, which
	// will call CG_DrawInformation during the loading process
	// reliableCommandSequence will be 0 on fresh loads, but higher for
	// demos, tourney restarts, or vid_restarts
    pub const CG_SHUTDOWN: unnamed_5 = 1;
    pub const CG_INIT: unnamed_5 = 0;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/ui/ui_public.h"]
pub mod ui_public_h {
    //	void	UI_Shutdown( void );
    pub const UI_KEY_EVENT: unnamed_2 = 3;
    pub const UIMENU_MAIN: unnamed_1 = 1;
    //	qboolean UI_IsFullscreen( void );
    pub const UI_SET_ACTIVE_MENU: unnamed_2 = 7;
    pub const UIMENU_INGAME: unnamed_1 = 2;
    pub type unnamed_1 = libc::c_uint;
    pub const UIMENU_POSTGAME: unnamed_1 = 6;
    pub const UIMENU_TEAM: unnamed_1 = 5;
    pub const UIMENU_BAD_CD_KEY: unnamed_1 = 4;
    pub const UIMENU_NEED_CD: unnamed_1 = 3;
    pub const UIMENU_NONE: unnamed_1 = 0;
    pub type unnamed_2 = libc::c_uint;
    //	void	UI_DrawConnectScreen( qboolean overlay );
    pub const UI_HASUNIQUECDKEY: unnamed_2 = 10;
    //	qboolean UI_ConsoleCommand( int realTime );
    pub const UI_DRAW_CONNECT_SCREEN: unnamed_2 = 9;
    //	void	UI_SetActiveMenu( uiMenuCommand_t menu );
    pub const UI_CONSOLE_COMMAND: unnamed_2 = 8;
    //	void	UI_Refresh( int time );
    pub const UI_IS_FULLSCREEN: unnamed_2 = 6;
    //	void	UI_MouseEvent( int dx, int dy );
    pub const UI_REFRESH: unnamed_2 = 5;
    //	void	UI_KeyEvent( int key );
    pub const UI_MOUSE_EVENT: unnamed_2 = 4;
    //	void	UI_Init( void );
    pub const UI_SHUTDOWN: unnamed_2 = 2;
    pub const UI_INIT: unnamed_2 = 1;
    // system reserved
    pub const UI_GETAPIVERSION: unnamed_2 = 0;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/client.h"]
pub mod client_h {
    /*
=============================================================================

the clientConnection_t structure is wiped when disconnecting from a server,
either to go to a full screen console, play a demo, or connect to a different server

A connection can be to either a server through the network layer or a
demo through a file.

=============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientConnection_t {
        pub state: connstate_t,
        pub clientNum: libc::c_int,
        pub lastPacketSentTime: libc::c_int,
        pub lastPacketTime: libc::c_int,
        pub servername: [libc::c_char; 4096],
        pub serverAddress: netadr_t,
        pub connectTime: libc::c_int,
        pub connectPacketCount: libc::c_int,
        pub serverMessage: [libc::c_char; 1024],
        pub challenge: libc::c_int,
        pub checksumFeed: libc::c_int,
        pub reliableSequence: libc::c_int,
        pub reliableAcknowledge: libc::c_int,
        pub reliableCommands: [[libc::c_char; 1024]; 64],
        pub serverMessageSequence: libc::c_int,
        pub serverCommandSequence: libc::c_int,
        pub lastExecutedServerCommand: libc::c_int,
        pub serverCommands: [[libc::c_char; 1024]; 64],
        pub download: fileHandle_t,
        pub downloadTempName: [libc::c_char; 4096],
        pub downloadName: [libc::c_char; 4096],
        pub cURLEnabled: qboolean,
        pub cURLUsed: qboolean,
        pub cURLDisconnected: qboolean,
        pub downloadURL: [libc::c_char; 4096],
        pub downloadCURL: *mut libc::c_void,
        pub downloadCURLM: *mut libc::c_void,
        pub sv_allowDownload: libc::c_int,
        pub sv_dlURL: [libc::c_char; 256],
        pub downloadNumber: libc::c_int,
        pub downloadBlock: libc::c_int,
        pub downloadCount: libc::c_int,
        pub downloadSize: libc::c_int,
        pub downloadList: [libc::c_char; 1024],
        pub downloadRestart: qboolean,
        pub demoName: [libc::c_char; 64],
        pub spDemoRecording: qboolean,
        pub demorecording: qboolean,
        pub demoplaying: qboolean,
        pub demowaiting: qboolean,
        pub firstDemoFrameSkipped: qboolean,
        pub demofile: fileHandle_t,
        pub timeDemoFrames: libc::c_int,
        pub timeDemoStart: libc::c_int,
        pub timeDemoBaseTime: libc::c_int,
        pub timeDemoLastFrame: libc::c_int,
        pub timeDemoMinDuration: libc::c_int,
        pub timeDemoMaxDuration: libc::c_int,
        pub timeDemoDurations: [libc::c_uchar; 4096],
        pub aviVideoFrameRemainder: libc::c_float,
        pub aviSoundFrameRemainder: libc::c_float,
        pub voipEnabled: qboolean,
        pub voipCodecInitialized: qboolean,
        pub opusDecoder: [*mut OpusDecoder; 64],
        pub voipIncomingGeneration: [byte; 64],
        pub voipIncomingSequence: [libc::c_int; 64],
        pub voipGain: [libc::c_float; 64],
        pub voipIgnore: [qboolean; 64],
        pub voipMuteAll: qboolean,
        pub voipTargets: [uint8_t; 8],
        pub voipFlags: uint8_t,
        pub opusEncoder: *mut OpusEncoder,
        pub voipOutgoingDataSize: libc::c_int,
        pub voipOutgoingDataFrames: libc::c_int,
        pub voipOutgoingSequence: libc::c_int,
        pub voipOutgoingGeneration: byte,
        pub voipOutgoingData: [byte; 1024],
        pub voipPower: libc::c_float,
        pub compat: qboolean,
        pub netchan: netchan_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientStatic_t {
        pub cddialog: qboolean,
        pub rendererStarted: qboolean,
        pub soundStarted: qboolean,
        pub soundRegistered: qboolean,
        pub uiStarted: qboolean,
        pub cgameStarted: qboolean,
        pub framecount: libc::c_int,
        pub frametime: libc::c_int,
        pub realtime: libc::c_int,
        pub realFrametime: libc::c_int,
        pub numlocalservers: libc::c_int,
        pub localServers: [serverInfo_t; 128],
        pub numglobalservers: libc::c_int,
        pub globalServers: [serverInfo_t; 4096],
        pub numGlobalServerAddresses: libc::c_int,
        pub globalServerAddresses: [netadr_t; 4096],
        pub numfavoriteservers: libc::c_int,
        pub favoriteServers: [serverInfo_t; 128],
        pub pingUpdateSource: libc::c_int,
        pub updateServer: netadr_t,
        pub updateChallenge: [libc::c_char; 1024],
        pub updateInfoString: [libc::c_char; 1024],
        pub authorizeServer: netadr_t,
        pub rconAddress: netadr_t,
        pub glconfig: glconfig_t,
        pub charSetShader: qhandle_t,
        pub whiteShader: qhandle_t,
        pub consoleShader: qhandle_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct serverInfo_t {
        pub adr: netadr_t,
        pub hostName: [libc::c_char; 32],
        pub mapName: [libc::c_char; 32],
        pub game: [libc::c_char; 32],
        pub netType: libc::c_int,
        pub gameType: libc::c_int,
        pub clients: libc::c_int,
        pub maxClients: libc::c_int,
        pub minPing: libc::c_int,
        pub maxPing: libc::c_int,
        pub ping: libc::c_int,
        pub visible: qboolean,
        pub punkbuster: libc::c_int,
        pub g_humanplayers: libc::c_int,
        pub g_needpass: libc::c_int,
    }
    use super::q_shared_h::{connstate_t, fileHandle_t, qboolean, byte,
                            qhandle_t};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t, vm_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    use super::tr_types_h::{glconfig_t};
    extern "C" {
        //=============================================================================
        // interface to cgame dll or vm
        #[no_mangle]
        pub static mut cgvm: *mut vm_t;
        // interface to ui dll or vm
        #[no_mangle]
        pub static mut uivm: *mut vm_t;
        #[no_mangle]
        pub static mut clc: clientConnection_t;
        #[no_mangle]
        pub fn Con_Bottom();
        #[no_mangle]
        pub fn Con_Top();
        #[no_mangle]
        pub fn Con_PageDown();
        #[no_mangle]
        pub fn Con_PageUp();
        // the parseEntities array must be large enough to hold PACKET_BACKUP frames of
// entities, so that when a delta compressed message arives from the server
// it can be un-deltad from the original 
        #[no_mangle]
        pub static mut g_console_field_width: libc::c_int;
        #[no_mangle]
        pub fn SCR_UpdateScreen();
        // 20ms at 48k
        // 3 frame is 60ms of audio, the max opus will encode at once
        //=================================================
        //
// cl_main
//
        #[no_mangle]
        pub fn CL_AddReliableCommand(cmd: *const libc::c_char,
                                     isDisconnectCmd: qboolean);
        #[no_mangle]
        pub fn CL_Disconnect_f();
        #[no_mangle]
        pub fn Con_ToggleConsole_f();
        #[no_mangle]
        pub fn SCR_DrawBigString(x: libc::c_int, y: libc::c_int,
                                 s: *const libc::c_char, alpha: libc::c_float,
                                 noColorEscape: qboolean);
        #[no_mangle]
        pub fn SCR_DrawSmallChar(x: libc::c_int, y: libc::c_int,
                                 ch: libc::c_int);
        #[no_mangle]
        pub static mut cls: clientStatic_t;
        #[no_mangle]
        pub fn SCR_DrawSmallStringExt(x: libc::c_int, y: libc::c_int,
                                      string: *const libc::c_char,
                                      setColor: *mut libc::c_float,
                                      forceColor: qboolean,
                                      noColorEscape: qboolean);
    }
}
#[header_src = "/usr/include/opus/opus.h"]
pub mod opus_h {
    extern "C" {
        /* Copyright (c) 2010-2011 Xiph.Org Foundation, Skype Limited
   Written by Jean-Marc Valin and Koen Vos */
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
        /* *
 * @file opus.h
 * @brief Opus reference implementation API
 */
        /* *
 * @mainpage Opus
 *
 * The Opus codec is designed for interactive speech and audio transmission over the Internet.
 * It is designed by the IETF Codec Working Group and incorporates technology from
 * Skype's SILK codec and Xiph.Org's CELT codec.
 *
 * The Opus codec is designed to handle a wide range of interactive audio applications,
 * including Voice over IP, videoconferencing, in-game chat, and even remote live music
 * performances. It can scale from low bit-rate narrowband speech to very high quality
 * stereo music. Its main features are:

 * @li Sampling rates from 8 to 48 kHz
 * @li Bit-rates from 6 kb/s to 510 kb/s
 * @li Support for both constant bit-rate (CBR) and variable bit-rate (VBR)
 * @li Audio bandwidth from narrowband to full-band
 * @li Support for speech and music
 * @li Support for mono and stereo
 * @li Support for multichannel (up to 255 channels)
 * @li Frame sizes from 2.5 ms to 60 ms
 * @li Good loss robustness and packet loss concealment (PLC)
 * @li Floating point and fixed-point implementation
 *
 * Documentation sections:
 * @li @ref opus_encoder
 * @li @ref opus_decoder
 * @li @ref opus_repacketizer
 * @li @ref opus_multistream
 * @li @ref opus_libinfo
 * @li @ref opus_custom
 */
        /* * @defgroup opus_encoder Opus Encoder
  * @{
  *
  * @brief This page describes the process and functions used to encode Opus.
  *
  * Since Opus is a stateful codec, the encoding process starts with creating an encoder
  * state. This can be done with:
  *
  * @code
  * int          error;
  * OpusEncoder *enc;
  * enc = opus_encoder_create(Fs, channels, application, &error);
  * @endcode
  *
  * From this point, @c enc can be used for encoding an audio stream. An encoder state
  * @b must @b not be used for more than one stream at the same time. Similarly, the encoder
  * state @b must @b not be re-initialized for each frame.
  *
  * While opus_encoder_create() allocates memory for the state, it's also possible
  * to initialize pre-allocated memory:
  *
  * @code
  * int          size;
  * int          error;
  * OpusEncoder *enc;
  * size = opus_encoder_get_size(channels);
  * enc = malloc(size);
  * error = opus_encoder_init(enc, Fs, channels, application);
  * @endcode
  *
  * where opus_encoder_get_size() returns the required size for the encoder state. Note that
  * future versions of this code may change the size, so no assuptions should be made about it.
  *
  * The encoder state is always continuous in memory and only a shallow copy is sufficient
  * to copy it (e.g. memcpy())
  *
  * It is possible to change some of the encoder's settings using the opus_encoder_ctl()
  * interface. All these settings already default to the recommended value, so they should
  * only be changed when necessary. The most common settings one may want to change are:
  *
  * @code
  * opus_encoder_ctl(enc, OPUS_SET_BITRATE(bitrate));
  * opus_encoder_ctl(enc, OPUS_SET_COMPLEXITY(complexity));
  * opus_encoder_ctl(enc, OPUS_SET_SIGNAL(signal_type));
  * @endcode
  *
  * where
  *
  * @arg bitrate is in bits per second (b/s)
  * @arg complexity is a value from 1 to 10, where 1 is the lowest complexity and 10 is the highest
  * @arg signal_type is either OPUS_AUTO (default), OPUS_SIGNAL_VOICE, or OPUS_SIGNAL_MUSIC
  *
  * See @ref opus_encoderctls and @ref opus_genericctls for a complete list of parameters that can be set or queried. Most parameters can be set or changed at any time during a stream.
  *
  * To encode a frame, opus_encode() or opus_encode_float() must be called with exactly one frame (2.5, 5, 10, 20, 40 or 60 ms) of audio data:
  * @code
  * len = opus_encode(enc, audio_frame, frame_size, packet, max_packet);
  * @endcode
  *
  * where
  * <ul>
  * <li>audio_frame is the audio data in opus_int16 (or float for opus_encode_float())</li>
  * <li>frame_size is the duration of the frame in samples (per channel)</li>
  * <li>packet is the byte array to which the compressed data is written</li>
  * <li>max_packet is the maximum number of bytes that can be written in the packet (4000 bytes is recommended).
  *     Do not use max_packet to control VBR target bitrate, instead use the #OPUS_SET_BITRATE CTL.</li>
  * </ul>
  *
  * opus_encode() and opus_encode_float() return the number of bytes actually written to the packet.
  * The return value <b>can be negative</b>, which indicates that an error has occurred. If the return value
  * is 1 byte, then the packet does not need to be transmitted (DTX).
  *
  * Once the encoder state if no longer needed, it can be destroyed with
  *
  * @code
  * opus_encoder_destroy(enc);
  * @endcode
  *
  * If the encoder was created with opus_encoder_init() rather than opus_encoder_create(),
  * then no action is required aside from potentially freeing the memory that was manually
  * allocated for it (calling free(enc) for the example above)
  *
  */
        /* * Opus encoder state.
  * This contains the complete state of an Opus encoder.
  * It is position independent and can be freely copied.
  * @see opus_encoder_create,opus_encoder_init
  */
        pub type OpusEncoder;
        /* *@}*/
        /* * @defgroup opus_decoder Opus Decoder
  * @{
  *
  * @brief This page describes the process and functions used to decode Opus.
  *
  * The decoding process also starts with creating a decoder
  * state. This can be done with:
  * @code
  * int          error;
  * OpusDecoder *dec;
  * dec = opus_decoder_create(Fs, channels, &error);
  * @endcode
  * where
  * @li Fs is the sampling rate and must be 8000, 12000, 16000, 24000, or 48000
  * @li channels is the number of channels (1 or 2)
  * @li error will hold the error code in case of failure (or #OPUS_OK on success)
  * @li the return value is a newly created decoder state to be used for decoding
  *
  * While opus_decoder_create() allocates memory for the state, it's also possible
  * to initialize pre-allocated memory:
  * @code
  * int          size;
  * int          error;
  * OpusDecoder *dec;
  * size = opus_decoder_get_size(channels);
  * dec = malloc(size);
  * error = opus_decoder_init(dec, Fs, channels);
  * @endcode
  * where opus_decoder_get_size() returns the required size for the decoder state. Note that
  * future versions of this code may change the size, so no assuptions should be made about it.
  *
  * The decoder state is always continuous in memory and only a shallow copy is sufficient
  * to copy it (e.g. memcpy())
  *
  * To decode a frame, opus_decode() or opus_decode_float() must be called with a packet of compressed audio data:
  * @code
  * frame_size = opus_decode(dec, packet, len, decoded, max_size, 0);
  * @endcode
  * where
  *
  * @li packet is the byte array containing the compressed data
  * @li len is the exact number of bytes contained in the packet
  * @li decoded is the decoded audio data in opus_int16 (or float for opus_decode_float())
  * @li max_size is the max duration of the frame in samples (per channel) that can fit into the decoded_frame array
  *
  * opus_decode() and opus_decode_float() return the number of samples (per channel) decoded from the packet.
  * If that value is negative, then an error has occurred. This can occur if the packet is corrupted or if the audio
  * buffer is too small to hold the decoded audio.
  *
  * Opus is a stateful codec with overlapping blocks and as a result Opus
  * packets are not coded independently of each other. Packets must be
  * passed into the decoder serially and in the correct order for a correct
  * decode. Lost packets can be replaced with loss concealment by calling
  * the decoder with a null pointer and zero length for the missing packet.
  *
  * A single codec state may only be accessed from a single thread at
  * a time and any required locking must be performed by the caller. Separate
  * streams must be decoded with separate decoder states and can be decoded
  * in parallel unless the library was compiled with NONTHREADSAFE_PSEUDOSTACK
  * defined.
  *
  */
        /* * Opus decoder state.
  * This contains the complete state of an Opus decoder.
  * It is position independent and can be freely copied.
  * @see opus_decoder_create,opus_decoder_init
  */
        pub type OpusDecoder;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/curl/multi.h"]
pub mod multi_h {
    pub type CURLM = ();
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/curl/curl.h"]
pub mod curl_h {
    pub type CURL = ();
    use super::{libc};
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/renderercommon/tr_types.h"]
pub mod tr_types_h {
    /*
** glconfig_t
**
** Contains variables specific to the OpenGL configuration
** being run right now.  These are constant once the OpenGL
** subsystem is initialized.
*/
    pub type textureCompression_t = libc::c_uint;
    // this is for the GL_EXT_texture_compression_s3tc extension.
    pub const TC_S3TC_ARB: textureCompression_t = 2;
    // this is for the GL_S3_s3tc extension.
    pub const TC_S3TC: textureCompression_t = 1;
    pub const TC_NONE: textureCompression_t = 0;
    pub type glDriverType_t = libc::c_uint;
    // driver is a 3Dfx standalone driver
    pub const GLDRV_VOODOO: glDriverType_t = 2;
    // WARNING: there are tests that check for
								// > GLDRV_ICD for minidriverness, so this
								// should always be the lowest value in this
								// enum set
    // driver is a non-3Dfx standalone driver
    pub const GLDRV_STANDALONE: glDriverType_t = 1;
    // driver is integrated with window system
    pub const GLDRV_ICD: glDriverType_t = 0;
    pub type glHardwareType_t = libc::c_uint;
    // where you don't have src*dst
    pub const GLHW_PERMEDIA2: glHardwareType_t = 4;
    // where you can't modulate alpha on alpha textures
    pub const GLHW_RAGEPRO: glHardwareType_t = 3;
    // the hardware type then there can NOT exist a secondary
							// display adapter
    // where you can't interpolate alpha
    pub const GLHW_RIVA128: glHardwareType_t = 2;
    // Voodoo Banshee or Voodoo3, relevant since if this is
    pub const GLHW_3DFX_2D3D: glHardwareType_t = 1;
    // where everything works the way it should
    pub const GLHW_GENERIC: glHardwareType_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct glconfig_t {
        pub renderer_string: [libc::c_char; 1024],
        pub vendor_string: [libc::c_char; 1024],
        pub version_string: [libc::c_char; 1024],
        pub extensions_string: [libc::c_char; 8192],
        pub maxTextureSize: libc::c_int,
        pub numTextureUnits: libc::c_int,
        pub colorBits: libc::c_int,
        pub depthBits: libc::c_int,
        pub stencilBits: libc::c_int,
        pub driverType: glDriverType_t,
        pub hardwareType: glHardwareType_t,
        pub deviceSupportsGamma: qboolean,
        pub textureCompression: textureCompression_t,
        pub textureEnvAddAvailable: qboolean,
        pub vidWidth: libc::c_int,
        pub vidHeight: libc::c_int,
        pub windowAspect: libc::c_float,
        pub displayFrequency: libc::c_int,
        pub isFullscreen: qboolean,
        pub stereoEnabled: qboolean,
        pub smpActive: qboolean,
    }
    use super::{libc};
    use super::q_shared_h::{qboolean};
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/client/snd_public.h"]
pub mod snd_public_h {
    extern "C" {
        // stop all sounds and the background track
        #[no_mangle]
        pub fn S_StopAllSounds();
    }
}
use self::types_h::{__uint8_t};
use self::ctype_h::{unnamed, _ISalnum, _ISpunct, _IScntrl, _ISblank, _ISgraph,
                    _ISprint, _ISspace, _ISxdigit, _ISdigit, _ISalpha,
                    _ISlower, _ISupper, __ctype_b_loc, tolower};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, qhandle_t, fileHandle_t,
                       unnamed_0, ERR_NEED_CD, ERR_DISCONNECT,
                       ERR_SERVERDISCONNECT, ERR_DROP, ERR_FATAL, cvar_s,
                       cvar_t, connstate_t, CA_CINEMATIC, CA_ACTIVE,
                       CA_PRIMED, CA_LOADING, CA_CONNECTED, CA_CHALLENGING,
                       CA_CONNECTING, CA_AUTHORIZING, CA_DISCONNECTED,
                       CA_UNINITIALIZED, COM_Parse, Com_HexStrToInt,
                       Com_sprintf, Com_SkipTokens, Q_stricmp, Q_strncpyz,
                       Q_strcat, va, Com_Error, Com_Printf};
use self::qcommon_h::{netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6, NA_IP,
                      NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD, netsrc_t,
                      NS_SERVER, NS_CLIENT, netadr_t, netchan_t, vm_t,
                      xcommand_t, completionFunc_t, field_t, vm_s, VM_Call,
                      Cbuf_AddText, Cmd_AddCommand,
                      Cmd_SetCommandCompletionFunc, Cmd_Argc, Cmd_Argv,
                      Cvar_Set, Cvar_SetValue, Cvar_VariableValue,
                      Cvar_VariableIntegerValue, cvar_modifiedFlags,
                      FS_FOpenFileWrite, FS_FOpenFileRead, FS_Write, FS_Read,
                      FS_FCloseFile, FS_Printf, Field_Clear,
                      Field_AutoComplete, Field_CompleteKeyname,
                      Field_CompleteCommand, CopyString, Com_DPrintf,
                      con_autochat, Z_Free, Sys_GetClipboardData};
use self::keys_h::{qkey_t};
use self::cl_keys_c::{keyname_t};
use self::keycodes_h::{K_PAD0_RIGHTTRIGGER, K_PAD0_LEFTTRIGGER,
                       K_PAD0_RIGHTSTICK_DOWN, K_PAD0_RIGHTSTICK_UP,
                       K_PAD0_RIGHTSTICK_RIGHT, K_PAD0_RIGHTSTICK_LEFT,
                       K_PAD0_LEFTSTICK_DOWN, K_PAD0_LEFTSTICK_UP,
                       K_PAD0_LEFTSTICK_RIGHT, K_PAD0_LEFTSTICK_LEFT,
                       K_PAD0_DPAD_RIGHT, K_PAD0_DPAD_LEFT, K_PAD0_DPAD_DOWN,
                       K_PAD0_DPAD_UP, K_PAD0_RIGHTSHOULDER,
                       K_PAD0_LEFTSHOULDER, K_PAD0_RIGHTSTICK_CLICK,
                       K_PAD0_LEFTSTICK_CLICK, K_PAD0_START, K_PAD0_GUIDE,
                       K_PAD0_BACK, K_PAD0_Y, K_PAD0_X, K_PAD0_B, K_PAD0_A,
                       K_UNDO, K_EURO, K_POWER, K_MENU, K_BREAK, K_SCROLLOCK,
                       K_SYSREQ, K_PRINT, K_HELP, K_MODE, K_COMPOSE, K_SUPER,
                       K_WORLD_95, K_WORLD_94, K_WORLD_93, K_WORLD_92,
                       K_WORLD_91, K_WORLD_90, K_WORLD_89, K_WORLD_88,
                       K_WORLD_87, K_WORLD_86, K_WORLD_85, K_WORLD_84,
                       K_WORLD_83, K_WORLD_82, K_WORLD_81, K_WORLD_80,
                       K_WORLD_79, K_WORLD_78, K_WORLD_77, K_WORLD_76,
                       K_WORLD_75, K_WORLD_74, K_WORLD_73, K_WORLD_72,
                       K_WORLD_71, K_WORLD_70, K_WORLD_69, K_WORLD_68,
                       K_WORLD_67, K_WORLD_66, K_WORLD_65, K_WORLD_64,
                       K_WORLD_63, K_WORLD_62, K_WORLD_61, K_WORLD_60,
                       K_WORLD_59, K_WORLD_58, K_WORLD_57, K_WORLD_56,
                       K_WORLD_55, K_WORLD_54, K_WORLD_53, K_WORLD_52,
                       K_WORLD_51, K_WORLD_50, K_WORLD_49, K_WORLD_48,
                       K_WORLD_47, K_WORLD_46, K_WORLD_45, K_WORLD_44,
                       K_WORLD_43, K_WORLD_42, K_WORLD_41, K_WORLD_40,
                       K_WORLD_39, K_WORLD_38, K_WORLD_37, K_WORLD_36,
                       K_WORLD_35, K_WORLD_34, K_WORLD_33, K_WORLD_32,
                       K_WORLD_31, K_WORLD_30, K_WORLD_29, K_WORLD_28,
                       K_WORLD_27, K_WORLD_26, K_WORLD_25, K_WORLD_24,
                       K_WORLD_23, K_WORLD_22, K_WORLD_21, K_WORLD_20,
                       K_WORLD_19, K_WORLD_18, K_WORLD_17, K_WORLD_16,
                       K_WORLD_15, K_WORLD_14, K_WORLD_13, K_WORLD_12,
                       K_WORLD_11, K_WORLD_10, K_WORLD_9, K_WORLD_8,
                       K_WORLD_7, K_WORLD_6, K_WORLD_5, K_WORLD_4, K_WORLD_3,
                       K_WORLD_2, K_WORLD_1, K_WORLD_0, K_PAUSE, K_KP_EQUALS,
                       K_KP_STAR, K_KP_NUMLOCK, K_KP_PLUS, K_KP_MINUS,
                       K_KP_SLASH, K_KP_DEL, K_KP_INS, K_KP_ENTER, K_KP_PGDN,
                       K_KP_DOWNARROW, K_KP_END, K_KP_RIGHTARROW, K_KP_5,
                       K_KP_LEFTARROW, K_KP_PGUP, K_KP_UPARROW, K_KP_HOME,
                       K_AUX16, K_AUX15, K_AUX14, K_AUX13, K_AUX12, K_AUX11,
                       K_AUX10, K_AUX9, K_AUX8, K_AUX7, K_AUX6, K_AUX5,
                       K_AUX4, K_AUX3, K_AUX2, K_AUX1, K_JOY32, K_JOY31,
                       K_JOY30, K_JOY29, K_JOY28, K_JOY27, K_JOY26, K_JOY25,
                       K_JOY24, K_JOY23, K_JOY22, K_JOY21, K_JOY20, K_JOY19,
                       K_JOY18, K_JOY17, K_JOY16, K_JOY15, K_JOY14, K_JOY13,
                       K_JOY12, K_JOY11, K_JOY10, K_JOY9, K_JOY8, K_JOY7,
                       K_JOY6, K_JOY5, K_JOY4, K_JOY3, K_JOY2, K_JOY1,
                       K_MWHEELDOWN, K_MWHEELUP, K_MOUSE5, K_MOUSE4, K_MOUSE3,
                       K_MOUSE2, K_MOUSE1, K_END, K_HOME, K_PGUP, K_PGDN,
                       K_DEL, K_INS, K_F15, K_F14, K_F13, K_F12, K_F11, K_F10,
                       K_F9, K_F8, K_F7, K_F6, K_F5, K_F4, K_F3, K_F2, K_F1,
                       K_CAPSLOCK, K_COMMAND, K_SHIFT, K_CTRL, K_ALT,
                       K_RIGHTARROW, K_LEFTARROW, K_DOWNARROW, K_UPARROW,
                       K_BACKSPACE, K_SPACE, K_ESCAPE, K_ENTER, K_TAB,
                       MAX_KEYS, K_CONSOLE, unnamed_3};
use self::cg_public_h::{CG_KEY_EVENT, CGAME_EVENT_NONE, CG_EVENT_HANDLING,
                        unnamed_4, CGAME_EVENT_EDITHUD,
                        CGAME_EVENT_SCOREBOARD, CGAME_EVENT_TEAMMENU,
                        unnamed_5, CG_MOUSE_EVENT, CG_LAST_ATTACKER,
                        CG_CROSSHAIR_PLAYER, CG_DRAW_ACTIVE_FRAME,
                        CG_CONSOLE_COMMAND, CG_SHUTDOWN, CG_INIT};
use self::ui_public_h::{UI_KEY_EVENT, UIMENU_MAIN, UI_SET_ACTIVE_MENU,
                        UIMENU_INGAME, unnamed_1, UIMENU_POSTGAME,
                        UIMENU_TEAM, UIMENU_BAD_CD_KEY, UIMENU_NEED_CD,
                        UIMENU_NONE, unnamed_2, UI_HASUNIQUECDKEY,
                        UI_DRAW_CONNECT_SCREEN, UI_CONSOLE_COMMAND,
                        UI_IS_FULLSCREEN, UI_REFRESH, UI_MOUSE_EVENT,
                        UI_SHUTDOWN, UI_INIT, UI_GETAPIVERSION};
use self::client_h::{clientConnection_t, clientStatic_t, serverInfo_t, cgvm,
                     uivm, clc, Con_Bottom, Con_Top, Con_PageDown, Con_PageUp,
                     g_console_field_width, SCR_UpdateScreen,
                     CL_AddReliableCommand, CL_Disconnect_f,
                     Con_ToggleConsole_f, SCR_DrawBigString,
                     SCR_DrawSmallChar, cls, SCR_DrawSmallStringExt};
use self::opus_h::{OpusEncoder, OpusDecoder};
use self::multi_h::{CURLM};
use self::curl_h::{CURL};
use self::tr_types_h::{textureCompression_t, TC_S3TC_ARB, TC_S3TC, TC_NONE,
                       glDriverType_t, GLDRV_VOODOO, GLDRV_STANDALONE,
                       GLDRV_ICD, glHardwareType_t, GLHW_PERMEDIA2,
                       GLHW_RAGEPRO, GLHW_RIVA128, GLHW_3DFX_2D3D,
                       GLHW_GENERIC, glconfig_t};
use self::string_h::{memcpy, memmove, strcat, strchr, strlen};
use self::stdlib_h::{atoi};
use self::snd_public_h::{S_StopAllSounds};
/*
==============================================================

CLIENT / SERVER SYSTEMS

==============================================================
*/
//
// client interface
//
#[no_mangle]
pub unsafe extern "C" fn CL_InitKeyCommands() {
    Cmd_AddCommand(b"bind\x00" as *const u8 as *const libc::c_char,
                   Some(Key_Bind_f));
    Cmd_SetCommandCompletionFunc(b"bind\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Key_CompleteBind));
    Cmd_AddCommand(b"unbind\x00" as *const u8 as *const libc::c_char,
                   Some(Key_Unbind_f));
    Cmd_SetCommandCompletionFunc(b"unbind\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Key_CompleteUnbind));
    Cmd_AddCommand(b"unbindall\x00" as *const u8 as *const libc::c_char,
                   Some(Key_Unbindall_f));
    Cmd_AddCommand(b"bindlist\x00" as *const u8 as *const libc::c_char,
                   Some(Key_Bindlist_f));
}
/*
============
Key_Bindlist_f

============
*/
#[no_mangle]
pub unsafe extern "C" fn Key_Bindlist_f() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < MAX_KEYS as libc::c_int {
        if !keys[i as usize].binding.is_null() &&
               0 != *keys[i as usize].binding.offset(0isize) as libc::c_int {
            Com_Printf(b"%s \"%s\"\n\x00" as *const u8 as *const libc::c_char,
                       Key_KeynumToString(i), keys[i as usize].binding);
        }
        i += 1
    };
}
#[no_mangle]
pub static mut keys: [qkey_t; 366] =
    [qkey_t{down: qfalse,
            repeats: 0,
            binding: 0 as *const libc::c_char as *mut libc::c_char,}; 366];
#[no_mangle]
pub unsafe extern "C" fn Key_KeynumToString(mut keynum: libc::c_int)
 -> *mut libc::c_char {
    let mut kn: *mut keyname_t = 0 as *mut keyname_t;
    static mut tinystr: [libc::c_char; 5] = [0; 5];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if keynum == -1i32 {
        return b"<KEY NOT FOUND>\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if keynum < 0i32 || keynum >= MAX_KEYS as libc::c_int {
        return b"<OUT OF RANGE>\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    if keynum > 32i32 && keynum < 127i32 && keynum != '\"' as i32 &&
           keynum != ';' as i32 {
        tinystr[0usize] = keynum as libc::c_char;
        tinystr[1usize] = 0i32 as libc::c_char;
        return tinystr.as_mut_ptr()
    }
    kn = keynames.as_mut_ptr();
    while !(*kn).name.is_null() {
        if keynum == (*kn).keynum { return (*kn).name }
        kn = kn.offset(1isize)
    }
    i = keynum >> 4i32;
    j = keynum & 15i32;
    tinystr[0usize] = '0' as i32 as libc::c_char;
    tinystr[1usize] = 'x' as i32 as libc::c_char;
    tinystr[2usize] =
        (if i > 9i32 { i - 10i32 + 'a' as i32 } else { i + '0' as i32 }) as
            libc::c_char;
    tinystr[3usize] =
        (if j > 9i32 { j - 10i32 + 'a' as i32 } else { j + '0' as i32 }) as
            libc::c_char;
    tinystr[4usize] = 0i32 as libc::c_char;
    return tinystr.as_mut_ptr();
}
// names not in this list can either be lowercase ascii, or '0xnn' hex sequences
#[no_mangle]
pub static mut keynames: [keyname_t; 244] =
    [keyname_t{name:
                   b"TAB\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_TAB as libc::c_int,},
     keyname_t{name:
                   b"ENTER\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_ENTER as libc::c_int,},
     keyname_t{name:
                   b"ESCAPE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_ESCAPE as libc::c_int,},
     keyname_t{name:
                   b"SPACE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_SPACE as libc::c_int,},
     keyname_t{name:
                   b"BACKSPACE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_BACKSPACE as libc::c_int,},
     keyname_t{name:
                   b"UPARROW\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_UPARROW as libc::c_int,},
     keyname_t{name:
                   b"DOWNARROW\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_DOWNARROW as libc::c_int,},
     keyname_t{name:
                   b"LEFTARROW\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_LEFTARROW as libc::c_int,},
     keyname_t{name:
                   b"RIGHTARROW\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_RIGHTARROW as libc::c_int,},
     keyname_t{name:
                   b"ALT\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_ALT as libc::c_int,},
     keyname_t{name:
                   b"CTRL\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_CTRL as libc::c_int,},
     keyname_t{name:
                   b"SHIFT\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_SHIFT as libc::c_int,},
     keyname_t{name:
                   b"COMMAND\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_COMMAND as libc::c_int,},
     keyname_t{name:
                   b"CAPSLOCK\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_CAPSLOCK as libc::c_int,},
     keyname_t{name:
                   b"F1\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F1 as libc::c_int,},
     keyname_t{name:
                   b"F2\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F2 as libc::c_int,},
     keyname_t{name:
                   b"F3\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F3 as libc::c_int,},
     keyname_t{name:
                   b"F4\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F4 as libc::c_int,},
     keyname_t{name:
                   b"F5\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F5 as libc::c_int,},
     keyname_t{name:
                   b"F6\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F6 as libc::c_int,},
     keyname_t{name:
                   b"F7\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F7 as libc::c_int,},
     keyname_t{name:
                   b"F8\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F8 as libc::c_int,},
     keyname_t{name:
                   b"F9\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F9 as libc::c_int,},
     keyname_t{name:
                   b"F10\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F10 as libc::c_int,},
     keyname_t{name:
                   b"F11\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F11 as libc::c_int,},
     keyname_t{name:
                   b"F12\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F12 as libc::c_int,},
     keyname_t{name:
                   b"F13\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F13 as libc::c_int,},
     keyname_t{name:
                   b"F14\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F14 as libc::c_int,},
     keyname_t{name:
                   b"F15\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_F15 as libc::c_int,},
     keyname_t{name:
                   b"INS\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_INS as libc::c_int,},
     keyname_t{name:
                   b"DEL\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_DEL as libc::c_int,},
     keyname_t{name:
                   b"PGDN\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PGDN as libc::c_int,},
     keyname_t{name:
                   b"PGUP\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PGUP as libc::c_int,},
     keyname_t{name:
                   b"HOME\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_HOME as libc::c_int,},
     keyname_t{name:
                   b"END\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_END as libc::c_int,},
     keyname_t{name:
                   b"MOUSE1\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_MOUSE1 as libc::c_int,},
     keyname_t{name:
                   b"MOUSE2\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_MOUSE2 as libc::c_int,},
     keyname_t{name:
                   b"MOUSE3\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_MOUSE3 as libc::c_int,},
     keyname_t{name:
                   b"MOUSE4\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_MOUSE4 as libc::c_int,},
     keyname_t{name:
                   b"MOUSE5\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_MOUSE5 as libc::c_int,},
     keyname_t{name:
                   b"MWHEELUP\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_MWHEELUP as libc::c_int,},
     keyname_t{name:
                   b"MWHEELDOWN\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_MWHEELDOWN as libc::c_int,},
     keyname_t{name:
                   b"JOY1\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY1 as libc::c_int,},
     keyname_t{name:
                   b"JOY2\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY2 as libc::c_int,},
     keyname_t{name:
                   b"JOY3\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY3 as libc::c_int,},
     keyname_t{name:
                   b"JOY4\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY4 as libc::c_int,},
     keyname_t{name:
                   b"JOY5\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY5 as libc::c_int,},
     keyname_t{name:
                   b"JOY6\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY6 as libc::c_int,},
     keyname_t{name:
                   b"JOY7\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY7 as libc::c_int,},
     keyname_t{name:
                   b"JOY8\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY8 as libc::c_int,},
     keyname_t{name:
                   b"JOY9\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY9 as libc::c_int,},
     keyname_t{name:
                   b"JOY10\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY10 as libc::c_int,},
     keyname_t{name:
                   b"JOY11\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY11 as libc::c_int,},
     keyname_t{name:
                   b"JOY12\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY12 as libc::c_int,},
     keyname_t{name:
                   b"JOY13\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY13 as libc::c_int,},
     keyname_t{name:
                   b"JOY14\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY14 as libc::c_int,},
     keyname_t{name:
                   b"JOY15\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY15 as libc::c_int,},
     keyname_t{name:
                   b"JOY16\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY16 as libc::c_int,},
     keyname_t{name:
                   b"JOY17\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY17 as libc::c_int,},
     keyname_t{name:
                   b"JOY18\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY18 as libc::c_int,},
     keyname_t{name:
                   b"JOY19\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY19 as libc::c_int,},
     keyname_t{name:
                   b"JOY20\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY20 as libc::c_int,},
     keyname_t{name:
                   b"JOY21\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY21 as libc::c_int,},
     keyname_t{name:
                   b"JOY22\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY22 as libc::c_int,},
     keyname_t{name:
                   b"JOY23\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY23 as libc::c_int,},
     keyname_t{name:
                   b"JOY24\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY24 as libc::c_int,},
     keyname_t{name:
                   b"JOY25\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY25 as libc::c_int,},
     keyname_t{name:
                   b"JOY26\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY26 as libc::c_int,},
     keyname_t{name:
                   b"JOY27\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY27 as libc::c_int,},
     keyname_t{name:
                   b"JOY28\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY28 as libc::c_int,},
     keyname_t{name:
                   b"JOY29\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY29 as libc::c_int,},
     keyname_t{name:
                   b"JOY30\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY30 as libc::c_int,},
     keyname_t{name:
                   b"JOY31\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY31 as libc::c_int,},
     keyname_t{name:
                   b"JOY32\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_JOY32 as libc::c_int,},
     keyname_t{name:
                   b"AUX1\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX1 as libc::c_int,},
     keyname_t{name:
                   b"AUX2\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX2 as libc::c_int,},
     keyname_t{name:
                   b"AUX3\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX3 as libc::c_int,},
     keyname_t{name:
                   b"AUX4\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX4 as libc::c_int,},
     keyname_t{name:
                   b"AUX5\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX5 as libc::c_int,},
     keyname_t{name:
                   b"AUX6\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX6 as libc::c_int,},
     keyname_t{name:
                   b"AUX7\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX7 as libc::c_int,},
     keyname_t{name:
                   b"AUX8\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX8 as libc::c_int,},
     keyname_t{name:
                   b"AUX9\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX9 as libc::c_int,},
     keyname_t{name:
                   b"AUX10\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX10 as libc::c_int,},
     keyname_t{name:
                   b"AUX11\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX11 as libc::c_int,},
     keyname_t{name:
                   b"AUX12\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX12 as libc::c_int,},
     keyname_t{name:
                   b"AUX13\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX13 as libc::c_int,},
     keyname_t{name:
                   b"AUX14\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX14 as libc::c_int,},
     keyname_t{name:
                   b"AUX15\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX15 as libc::c_int,},
     keyname_t{name:
                   b"AUX16\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_AUX16 as libc::c_int,},
     keyname_t{name:
                   b"KP_HOME\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_HOME as libc::c_int,},
     keyname_t{name:
                   b"KP_UPARROW\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_UPARROW as libc::c_int,},
     keyname_t{name:
                   b"KP_PGUP\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_PGUP as libc::c_int,},
     keyname_t{name:
                   b"KP_LEFTARROW\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_LEFTARROW as libc::c_int,},
     keyname_t{name:
                   b"KP_5\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_5 as libc::c_int,},
     keyname_t{name:
                   b"KP_RIGHTARROW\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_RIGHTARROW as libc::c_int,},
     keyname_t{name:
                   b"KP_END\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_END as libc::c_int,},
     keyname_t{name:
                   b"KP_DOWNARROW\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_DOWNARROW as libc::c_int,},
     keyname_t{name:
                   b"KP_PGDN\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_PGDN as libc::c_int,},
     keyname_t{name:
                   b"KP_ENTER\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_ENTER as libc::c_int,},
     keyname_t{name:
                   b"KP_INS\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_INS as libc::c_int,},
     keyname_t{name:
                   b"KP_DEL\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_DEL as libc::c_int,},
     keyname_t{name:
                   b"KP_SLASH\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_SLASH as libc::c_int,},
     keyname_t{name:
                   b"KP_MINUS\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_MINUS as libc::c_int,},
     keyname_t{name:
                   b"KP_PLUS\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_PLUS as libc::c_int,},
     keyname_t{name:
                   b"KP_NUMLOCK\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_NUMLOCK as libc::c_int,},
     keyname_t{name:
                   b"KP_STAR\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_STAR as libc::c_int,},
     keyname_t{name:
                   b"KP_EQUALS\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_KP_EQUALS as libc::c_int,},
     keyname_t{name:
                   b"PAUSE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PAUSE as libc::c_int,},
     keyname_t{name:
                   b"SEMICOLON\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: ';' as i32,},
     keyname_t{name:
                   b"WORLD_0\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_0 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_1\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_1 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_2\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_2 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_3\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_3 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_4\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_4 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_5\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_5 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_6\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_6 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_7\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_7 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_8\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_8 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_9\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_9 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_10\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_10 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_11\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_11 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_12\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_12 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_13\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_13 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_14\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_14 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_15\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_15 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_16\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_16 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_17\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_17 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_18\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_18 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_19\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_19 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_20\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_20 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_21\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_21 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_22\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_22 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_23\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_23 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_24\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_24 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_25\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_25 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_26\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_26 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_27\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_27 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_28\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_28 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_29\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_29 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_30\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_30 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_31\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_31 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_32\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_32 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_33\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_33 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_34\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_34 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_35\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_35 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_36\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_36 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_37\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_37 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_38\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_38 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_39\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_39 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_40\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_40 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_41\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_41 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_42\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_42 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_43\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_43 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_44\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_44 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_45\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_45 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_46\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_46 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_47\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_47 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_48\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_48 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_49\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_49 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_50\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_50 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_51\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_51 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_52\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_52 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_53\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_53 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_54\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_54 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_55\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_55 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_56\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_56 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_57\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_57 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_58\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_58 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_59\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_59 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_60\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_60 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_61\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_61 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_62\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_62 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_63\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_63 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_64\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_64 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_65\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_65 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_66\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_66 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_67\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_67 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_68\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_68 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_69\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_69 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_70\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_70 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_71\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_71 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_72\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_72 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_73\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_73 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_74\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_74 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_75\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_75 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_76\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_76 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_77\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_77 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_78\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_78 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_79\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_79 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_80\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_80 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_81\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_81 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_82\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_82 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_83\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_83 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_84\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_84 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_85\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_85 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_86\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_86 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_87\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_87 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_88\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_88 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_89\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_89 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_90\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_90 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_91\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_91 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_92\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_92 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_93\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_93 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_94\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_94 as libc::c_int,},
     keyname_t{name:
                   b"WORLD_95\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_WORLD_95 as libc::c_int,},
     keyname_t{name:
                   b"WINDOWS\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_SUPER as libc::c_int,},
     keyname_t{name:
                   b"COMPOSE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_COMPOSE as libc::c_int,},
     keyname_t{name:
                   b"MODE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_MODE as libc::c_int,},
     keyname_t{name:
                   b"HELP\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_HELP as libc::c_int,},
     keyname_t{name:
                   b"PRINT\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PRINT as libc::c_int,},
     keyname_t{name:
                   b"SYSREQ\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_SYSREQ as libc::c_int,},
     keyname_t{name:
                   b"SCROLLOCK\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_SCROLLOCK as libc::c_int,},
     keyname_t{name:
                   b"BREAK\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_BREAK as libc::c_int,},
     keyname_t{name:
                   b"MENU\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_MENU as libc::c_int,},
     keyname_t{name:
                   b"POWER\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_POWER as libc::c_int,},
     keyname_t{name:
                   b"EURO\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_EURO as libc::c_int,},
     keyname_t{name:
                   b"UNDO\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_UNDO as libc::c_int,},
     keyname_t{name:
                   b"PAD0_A\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PAD0_A as libc::c_int,},
     keyname_t{name:
                   b"PAD0_B\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PAD0_B as libc::c_int,},
     keyname_t{name:
                   b"PAD0_X\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PAD0_X as libc::c_int,},
     keyname_t{name:
                   b"PAD0_Y\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PAD0_Y as libc::c_int,},
     keyname_t{name:
                   b"PAD0_BACK\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PAD0_BACK as libc::c_int,},
     keyname_t{name:
                   b"PAD0_GUIDE\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PAD0_GUIDE as libc::c_int,},
     keyname_t{name:
                   b"PAD0_START\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PAD0_START as libc::c_int,},
     keyname_t{name:
                   b"PAD0_LEFTSTICK_CLICK\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_LEFTSTICK_CLICK as libc::c_int,},
     keyname_t{name:
                   b"PAD0_RIGHTSTICK_CLICK\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_RIGHTSTICK_CLICK as libc::c_int,},
     keyname_t{name:
                   b"PAD0_LEFTSHOULDER\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_LEFTSHOULDER as libc::c_int,},
     keyname_t{name:
                   b"PAD0_RIGHTSHOULDER\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_RIGHTSHOULDER as libc::c_int,},
     keyname_t{name:
                   b"PAD0_DPAD_UP\x00" as *const u8 as *const libc::c_char as
                       *mut libc::c_char,
               keynum: K_PAD0_DPAD_UP as libc::c_int,},
     keyname_t{name:
                   b"PAD0_DPAD_DOWN\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char,
               keynum: K_PAD0_DPAD_DOWN as libc::c_int,},
     keyname_t{name:
                   b"PAD0_DPAD_LEFT\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char,
               keynum: K_PAD0_DPAD_LEFT as libc::c_int,},
     keyname_t{name:
                   b"PAD0_DPAD_RIGHT\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char,
               keynum: K_PAD0_DPAD_RIGHT as libc::c_int,},
     keyname_t{name:
                   b"PAD0_LEFTSTICK_LEFT\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_LEFTSTICK_LEFT as libc::c_int,},
     keyname_t{name:
                   b"PAD0_LEFTSTICK_RIGHT\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_LEFTSTICK_RIGHT as libc::c_int,},
     keyname_t{name:
                   b"PAD0_LEFTSTICK_UP\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_LEFTSTICK_UP as libc::c_int,},
     keyname_t{name:
                   b"PAD0_LEFTSTICK_DOWN\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_LEFTSTICK_DOWN as libc::c_int,},
     keyname_t{name:
                   b"PAD0_RIGHTSTICK_LEFT\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_RIGHTSTICK_LEFT as libc::c_int,},
     keyname_t{name:
                   b"PAD0_RIGHTSTICK_RIGHT\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_RIGHTSTICK_RIGHT as libc::c_int,},
     keyname_t{name:
                   b"PAD0_RIGHTSTICK_UP\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_RIGHTSTICK_UP as libc::c_int,},
     keyname_t{name:
                   b"PAD0_RIGHTSTICK_DOWN\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_RIGHTSTICK_DOWN as libc::c_int,},
     keyname_t{name:
                   b"PAD0_LEFTTRIGGER\x00" as *const u8 as *const libc::c_char
                       as *mut libc::c_char,
               keynum: K_PAD0_LEFTTRIGGER as libc::c_int,},
     keyname_t{name:
                   b"PAD0_RIGHTTRIGGER\x00" as *const u8 as
                       *const libc::c_char as *mut libc::c_char,
               keynum: K_PAD0_RIGHTTRIGGER as libc::c_int,},
     keyname_t{name: 0 as *const libc::c_char as *mut libc::c_char,
               keynum: 0i32,}];
/*
===================
Key_Unbindall_f
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Key_Unbindall_f() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < MAX_KEYS as libc::c_int {
        if !keys[i as usize].binding.is_null() {
            Key_SetBinding(i, b"\x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Key_SetBinding(mut keynum: libc::c_int,
                                        mut binding: *const libc::c_char) {
    if keynum < 0i32 || keynum >= MAX_KEYS as libc::c_int { return }
    if !keys[keynum as usize].binding.is_null() {
        Z_Free(keys[keynum as usize].binding as *mut libc::c_void);
    }
    keys[keynum as usize].binding = CopyString(binding);
    cvar_modifiedFlags |= 0x1i32;
}
/*
====================
Key_CompleteUnbind
====================
*/
unsafe extern "C" fn Key_CompleteUnbind(mut args: *mut libc::c_char,
                                        mut argNum: libc::c_int) {
    if argNum == 2i32 {
        let mut p: *mut libc::c_char =
            Com_SkipTokens(args, 1i32,
                           b" \x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char);
        if p > args { Field_CompleteKeyname(); }
    };
}
/*
===================
Key_Unbind_f
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Key_Unbind_f() {
    let mut b: libc::c_int = 0;
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"unbind <key> : remove commands from a key\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    b = Key_StringToKeynum(Cmd_Argv(1i32));
    if b == -1i32 {
        Com_Printf(b"\"%s\" isn\'t a valid key\n\x00" as *const u8 as
                       *const libc::c_char, Cmd_Argv(1i32));
        return
    }
    Key_SetBinding(b, b"\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Key_StringToKeynum(mut str: *mut libc::c_char)
 -> libc::c_int {
    let mut kn: *mut keyname_t = 0 as *mut keyname_t;
    let mut n: libc::c_int = 0;
    if str.is_null() || 0 == *str.offset(0isize) { return -1i32 }
    if 0 == *str.offset(1isize) {
        return tolower(*str.offset(0isize) as libc::c_int)
    }
    n = Com_HexStrToInt(str);
    if n >= 0i32 && n < MAX_KEYS as libc::c_int { return n }
    kn = keynames.as_mut_ptr();
    while !(*kn).name.is_null() {
        if 0 == Q_stricmp(str, (*kn).name) { return (*kn).keynum }
        kn = kn.offset(1isize)
    }
    return -1i32;
}
/*
====================
Key_CompleteBind
====================
*/
unsafe extern "C" fn Key_CompleteBind(mut args: *mut libc::c_char,
                                      mut argNum: libc::c_int) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if argNum == 2i32 {
        p =
            Com_SkipTokens(args, 1i32,
                           b" \x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char);
        if p > args { Field_CompleteKeyname(); }
    } else if argNum >= 3i32 {
        p =
            Com_SkipTokens(args, 2i32,
                           b" \x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char);
        if p > args { Field_CompleteCommand(p, qtrue, qtrue); }
    };
}
/*
===================
Key_Bind_f
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Key_Bind_f() {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    c = Cmd_Argc();
    if c < 2i32 {
        Com_Printf(b"bind <key> [command] : attach a command to a key\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    b = Key_StringToKeynum(Cmd_Argv(1i32));
    if b == -1i32 {
        Com_Printf(b"\"%s\" isn\'t a valid key\n\x00" as *const u8 as
                       *const libc::c_char, Cmd_Argv(1i32));
        return
    }
    if c == 2i32 {
        if !keys[b as usize].binding.is_null() &&
               0 != *keys[b as usize].binding.offset(0isize) as libc::c_int {
            Com_Printf(b"\"%s\" = \"%s\"\n\x00" as *const u8 as
                           *const libc::c_char, Key_KeynumToString(b),
                       keys[b as usize].binding);
        } else {
            Com_Printf(b"\"%s\" is not bound\n\x00" as *const u8 as
                           *const libc::c_char, Key_KeynumToString(b));
        }
        return
    }
    cmd[0usize] = 0i32 as libc::c_char;
    i = 2i32;
    while i < c {
        strcat(cmd.as_mut_ptr(), Cmd_Argv(i));
        if i != c - 1i32 {
            strcat(cmd.as_mut_ptr(),
                   b" \x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    }
    Key_SetBinding(b, cmd.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CL_KeyEvent(mut key: libc::c_int, mut down: qboolean,
                                     mut time: libc::c_uint) {
    if 0 != down as u64 {
        CL_KeyDownEvent(key, time);
    } else { CL_KeyUpEvent(key, time); };
}
/*
===================
CL_KeyUpEvent

Called by CL_KeyEvent to handle a keyrelease
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_KeyUpEvent(mut key: libc::c_int,
                                       mut time: libc::c_uint) {
    keys[key as usize].repeats = 0i32;
    keys[key as usize].down = qfalse;
    anykeydown -= 1;
    if anykeydown < 0i32 { anykeydown = 0i32 }
    if key == K_CONSOLE as libc::c_int ||
           key == K_ESCAPE as libc::c_int &&
               0 != keys[K_SHIFT as libc::c_int as usize].down as libc::c_uint
       {
        return
    }
    CL_ParseBinding(key, qfalse, time);
    if 0 != Key_GetCatcher() & 0x2i32 && !uivm.is_null() {
        VM_Call(uivm, UI_KEY_EVENT as libc::c_int, key,
                qfalse as libc::c_int);
    } else if 0 != Key_GetCatcher() & 0x8i32 && !cgvm.is_null() {
        VM_Call(cgvm, CG_KEY_EVENT as libc::c_int, key,
                qfalse as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Key_GetCatcher() -> libc::c_int {
    return keyCatchers;
}
static mut keyCatchers: libc::c_int = 0i32;
/*
===================
CL_ParseBinding

Execute the commands in the bind string
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_ParseBinding(mut key: libc::c_int,
                                         mut down: qboolean,
                                         mut time: libc::c_uint) {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut p: *mut libc::c_char = buf.as_mut_ptr();
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut allCommands: qboolean = qfalse;
    let mut allowUpCmds: qboolean = qfalse;
    if clc.state as libc::c_uint ==
           CA_DISCONNECTED as libc::c_int as libc::c_uint &&
           Key_GetCatcher() == 0i32 {
        return
    }
    if keys[key as usize].binding.is_null() ||
           0 == *keys[key as usize].binding.offset(0isize) {
        return
    }
    Q_strncpyz(buf.as_mut_ptr(), keys[key as usize].binding,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    allCommands = (Key_GetCatcher() == 0i32) as libc::c_int as qboolean;
    allowUpCmds =
        (clc.state as libc::c_uint !=
             CA_DISCONNECTED as libc::c_int as libc::c_uint) as libc::c_int as
            qboolean;
    loop  {
        while 0 !=
                  *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as
                      libc::c_int &
                      _ISspace as libc::c_int as libc::c_ushort as libc::c_int
              {
            p = p.offset(1isize)
        }
        end = strchr(p, ';' as i32);
        if !end.is_null() { *end = '\u{0}' as i32 as libc::c_char }
        if *p as libc::c_int == '+' as i32 {
            if 0 != allCommands as libc::c_uint ||
                   0 != allowUpCmds as libc::c_uint && 0 == down as u64 {
                let mut cmd: [libc::c_char; 1024] = [0; 1024];
                Com_sprintf(cmd.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong as libc::c_int,
                            b"%c%s %d %d\n\x00" as *const u8 as
                                *const libc::c_char,
                            if 0 != down as libc::c_uint {
                                '+' as i32
                            } else { '-' as i32 }, p.offset(1isize), key,
                            time);
                Cbuf_AddText(cmd.as_mut_ptr());
            }
        } else if 0 != down as u64 {
            if 0 != allCommands as libc::c_uint ||
                   0 != CL_BindUICommand(p) as libc::c_uint {
                Cbuf_AddText(p);
                Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
            }
        }
        if end.is_null() { break ; }
        p = end.offset(1isize)
    };
}
/*
===================
CL_BindUICommand

Returns qtrue if bind command should be executed while user interface is shown
===================
*/
unsafe extern "C" fn CL_BindUICommand(mut cmd: *const libc::c_char)
 -> qboolean {
    if 0 != Key_GetCatcher() & 0x1i32 { return qfalse }
    if 0 ==
           Q_stricmp(cmd,
                     b"toggleconsole\x00" as *const u8 as *const libc::c_char)
       {
        return qtrue
    }
    if 0 ==
           Q_stricmp(cmd,
                     b"togglemenu\x00" as *const u8 as *const libc::c_char) {
        return qtrue
    }
    return qfalse;
}
#[no_mangle]
pub static mut anykeydown: libc::c_int = 0;
/*
===================
CL_KeyDownEvent

Called by CL_KeyEvent to handle a keypress
===================
*/
#[no_mangle]
pub unsafe extern "C" fn CL_KeyDownEvent(mut key: libc::c_int,
                                         mut time: libc::c_uint) {
    keys[key as usize].down = qtrue;
    keys[key as usize].repeats += 1;
    if keys[key as usize].repeats == 1i32 { anykeydown += 1 }
    if 0 != keys[K_ALT as libc::c_int as usize].down as libc::c_uint &&
           key == K_ENTER as libc::c_int {
        if keys[K_ENTER as libc::c_int as usize].repeats > 1i32 { return }
        Cvar_SetValue(b"r_fullscreen\x00" as *const u8 as *const libc::c_char,
                      (0 ==
                           Cvar_VariableIntegerValue(b"r_fullscreen\x00" as
                                                         *const u8 as
                                                         *const libc::c_char))
                          as libc::c_int as libc::c_float);
        return
    }
    if key == K_CONSOLE as libc::c_int ||
           0 != keys[K_SHIFT as libc::c_int as usize].down as libc::c_uint &&
               key == K_ESCAPE as libc::c_int {
        Con_ToggleConsole_f();
        Key_ClearStates();
        return
    }
    if (key < 128i32 || key == K_MOUSE1 as libc::c_int) &&
           (0 != clc.demoplaying as libc::c_uint ||
                clc.state as libc::c_uint ==
                    CA_CINEMATIC as libc::c_int as libc::c_uint) &&
           Key_GetCatcher() == 0i32 {
        if Cvar_VariableValue(b"com_cameraMode\x00" as *const u8 as
                                  *const libc::c_char) ==
               0i32 as libc::c_float {
            Cvar_Set(b"nextdemo\x00" as *const u8 as *const libc::c_char,
                     b"\x00" as *const u8 as *const libc::c_char);
            key = K_ESCAPE as libc::c_int
        }
    }
    if key == K_ESCAPE as libc::c_int {
        if 0 != Key_GetCatcher() & 0x4i32 { Message_Key(key); return }
        if 0 != Key_GetCatcher() & 0x8i32 {
            Key_SetCatcher(Key_GetCatcher() & !0x8i32);
            VM_Call(cgvm, CG_EVENT_HANDLING as libc::c_int,
                    CGAME_EVENT_NONE as libc::c_int);
            return
        }
        if 0 == Key_GetCatcher() & 0x2i32 {
            if clc.state as libc::c_uint ==
                   CA_ACTIVE as libc::c_int as libc::c_uint &&
                   0 == clc.demoplaying as u64 {
                VM_Call(uivm, UI_SET_ACTIVE_MENU as libc::c_int,
                        UIMENU_INGAME as libc::c_int);
            } else if clc.state as libc::c_uint !=
                          CA_DISCONNECTED as libc::c_int as libc::c_uint {
                CL_Disconnect_f();
                S_StopAllSounds();
                VM_Call(uivm, UI_SET_ACTIVE_MENU as libc::c_int,
                        UIMENU_MAIN as libc::c_int);
            }
            return
        }
        VM_Call(uivm, UI_KEY_EVENT as libc::c_int, key, qtrue as libc::c_int);
        return
    }
    CL_ParseBinding(key, qtrue, time);
    if 0 != Key_GetCatcher() & 0x1i32 {
        Console_Key(key);
    } else if 0 != Key_GetCatcher() & 0x2i32 {
        if !uivm.is_null() {
            VM_Call(uivm, UI_KEY_EVENT as libc::c_int, key,
                    qtrue as libc::c_int);
        }
    } else if 0 != Key_GetCatcher() & 0x8i32 {
        if !cgvm.is_null() {
            VM_Call(cgvm, CG_KEY_EVENT as libc::c_int, key,
                    qtrue as libc::c_int);
        }
    } else if 0 != Key_GetCatcher() & 0x4i32 {
        Message_Key(key);
    } else if clc.state as libc::c_uint ==
                  CA_DISCONNECTED as libc::c_int as libc::c_uint {
        Console_Key(key);
    };
}
/*
=============================================================================

CONSOLE LINE EDITING

==============================================================================
*/
/*
====================
Console_Key

Handles history and console scrollback
====================
*/
#[no_mangle]
pub unsafe extern "C" fn Console_Key(mut key: libc::c_int) {
    if key == 'l' as i32 &&
           0 != keys[K_CTRL as libc::c_int as usize].down as libc::c_uint {
        Cbuf_AddText(b"clear\n\x00" as *const u8 as *const libc::c_char);
        return
    }
    if key == K_ENTER as libc::c_int || key == K_KP_ENTER as libc::c_int {
        if clc.state as libc::c_uint !=
               CA_ACTIVE as libc::c_int as libc::c_uint &&
               0 != (*con_autochat).integer &&
               0 != g_consoleField.buffer[0usize] as libc::c_int &&
               g_consoleField.buffer[0usize] as libc::c_int != '\\' as i32 &&
               g_consoleField.buffer[0usize] as libc::c_int != '/' as i32 {
            let mut temp: [libc::c_char; 255] = [0; 255];
            Q_strncpyz(temp.as_mut_ptr(), g_consoleField.buffer.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 255]>() as
                           libc::c_ulong as libc::c_int);
            Com_sprintf(g_consoleField.buffer.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as
                            libc::c_ulong as libc::c_int,
                        b"\\%s\x00" as *const u8 as *const libc::c_char,
                        temp.as_mut_ptr());
            g_consoleField.cursor += 1
        }
        Com_Printf(b"]%s\n\x00" as *const u8 as *const libc::c_char,
                   g_consoleField.buffer.as_mut_ptr());
        if g_consoleField.buffer[0usize] as libc::c_int == '\\' as i32 ||
               g_consoleField.buffer[0usize] as libc::c_int == '/' as i32 {
            Cbuf_AddText(g_consoleField.buffer.as_mut_ptr().offset(1isize));
            Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
        } else if 0 == g_consoleField.buffer[0usize] {
            return
        } else {
            if 0 != (*con_autochat).integer {
                Cbuf_AddText(b"cmd say \x00" as *const u8 as
                                 *const libc::c_char);
            }
            Cbuf_AddText(g_consoleField.buffer.as_mut_ptr());
            Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
        }
        historyEditLines[(nextHistoryLine % 32i32) as usize] = g_consoleField;
        nextHistoryLine += 1;
        historyLine = nextHistoryLine;
        Field_Clear(&mut g_consoleField);
        g_consoleField.widthInChars = g_console_field_width;
        CL_SaveConsoleHistory();
        if clc.state as libc::c_uint ==
               CA_DISCONNECTED as libc::c_int as libc::c_uint {
            SCR_UpdateScreen();
        }
        return
    }
    if key == K_TAB as libc::c_int {
        Field_AutoComplete(&mut g_consoleField);
        return
    }
    if key == K_MWHEELUP as libc::c_int &&
           0 != keys[K_SHIFT as libc::c_int as usize].down as libc::c_uint ||
           key == K_UPARROW as libc::c_int ||
           key == K_KP_UPARROW as libc::c_int ||
           tolower(key) == 'p' as i32 &&
               0 != keys[K_CTRL as libc::c_int as usize].down as libc::c_uint
       {
        if nextHistoryLine - historyLine < 32i32 && historyLine > 0i32 {
            historyLine -= 1
        }
        g_consoleField = historyEditLines[(historyLine % 32i32) as usize];
        return
    }
    if key == K_MWHEELDOWN as libc::c_int &&
           0 != keys[K_SHIFT as libc::c_int as usize].down as libc::c_uint ||
           key == K_DOWNARROW as libc::c_int ||
           key == K_KP_DOWNARROW as libc::c_int ||
           tolower(key) == 'n' as i32 &&
               0 != keys[K_CTRL as libc::c_int as usize].down as libc::c_uint
       {
        historyLine += 1;
        if historyLine >= nextHistoryLine {
            historyLine = nextHistoryLine;
            Field_Clear(&mut g_consoleField);
            g_consoleField.widthInChars = g_console_field_width;
            return
        }
        g_consoleField = historyEditLines[(historyLine % 32i32) as usize];
        return
    }
    if key == K_PGUP as libc::c_int { Con_PageUp(); return }
    if key == K_PGDN as libc::c_int { Con_PageDown(); return }
    if key == K_MWHEELUP as libc::c_int {
        Con_PageUp();
        if 0 != keys[K_CTRL as libc::c_int as usize].down as u64 {
            Con_PageUp();
            Con_PageUp();
        }
        return
    }
    if key == K_MWHEELDOWN as libc::c_int {
        Con_PageDown();
        if 0 != keys[K_CTRL as libc::c_int as usize].down as u64 {
            Con_PageDown();
            Con_PageDown();
        }
        return
    }
    if key == K_HOME as libc::c_int &&
           0 != keys[K_CTRL as libc::c_int as usize].down as libc::c_uint {
        Con_Top();
        return
    }
    if key == K_END as libc::c_int &&
           0 != keys[K_CTRL as libc::c_int as usize].down as libc::c_uint {
        Con_Bottom();
        return
    }
    Field_KeyDownEvent(&mut g_consoleField, key);
}
#[no_mangle]
pub static mut g_consoleField: field_t =
    field_t{cursor: 0, scroll: 0, widthInChars: 0, buffer: [0; 256],};
// NOTE TTimo the declaration of field_t and Field_Clear is now in qcommon/qcommon.h
#[no_mangle]
pub unsafe extern "C" fn Field_KeyDownEvent(mut edit: *mut field_t,
                                            mut key: libc::c_int) {
    let mut len: libc::c_int = 0;
    if (key == K_INS as libc::c_int || key == K_KP_INS as libc::c_int) &&
           0 != keys[K_SHIFT as libc::c_int as usize].down as libc::c_uint {
        Field_Paste(edit);
        return
    }
    key = tolower(key);
    len = strlen((*edit).buffer.as_mut_ptr()) as libc::c_int;
    match key {
        140 => {
            if (*edit).cursor < len {
                memmove((*edit).buffer.as_mut_ptr().offset((*edit).cursor as
                                                               isize) as
                            *mut libc::c_void,
                        (*edit).buffer.as_mut_ptr().offset((*edit).cursor as
                                                               isize).offset(1isize)
                            as *const libc::c_void,
                        (len - (*edit).cursor) as libc::c_ulong);
            }
        }
        135 => { if (*edit).cursor < len { (*edit).cursor += 1 } }
        134 => { if (*edit).cursor > 0i32 { (*edit).cursor -= 1 } }
        143 => { (*edit).cursor = 0i32 }
        144 => { (*edit).cursor = len }
        139 => {
            key_overstrikeMode =
                (0 == key_overstrikeMode as u64) as libc::c_int as qboolean
        }
        _ => { }
    }
    if (*edit).cursor < (*edit).scroll {
        (*edit).scroll = (*edit).cursor
    } else if (*edit).cursor >= (*edit).scroll + (*edit).widthInChars &&
                  (*edit).cursor <= len {
        (*edit).scroll = (*edit).cursor - (*edit).widthInChars + 1i32
    };
}
#[no_mangle]
pub static mut key_overstrikeMode: qboolean = qfalse;
/*
================
Field_Paste
================
*/
#[no_mangle]
pub unsafe extern "C" fn Field_Paste(mut edit: *mut field_t) {
    let mut cbd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pasteLen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    cbd = Sys_GetClipboardData();
    if cbd.is_null() { return }
    pasteLen = strlen(cbd) as libc::c_int;
    i = 0i32;
    while i < pasteLen {
        Field_CharEvent(edit, *cbd.offset(i as isize) as libc::c_int);
        i += 1
    }
    Z_Free(cbd as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Field_CharEvent(mut edit: *mut field_t,
                                         mut ch: libc::c_int) {
    let mut len: libc::c_int = 0;
    if ch == 'v' as i32 - 'a' as i32 + 1i32 { Field_Paste(edit); return }
    if ch == 'c' as i32 - 'a' as i32 + 1i32 { Field_Clear(edit); return }
    len = strlen((*edit).buffer.as_mut_ptr()) as libc::c_int;
    if ch == 'h' as i32 - 'a' as i32 + 1i32 {
        if (*edit).cursor > 0i32 {
            memmove((*edit).buffer.as_mut_ptr().offset((*edit).cursor as
                                                           isize).offset(-1isize)
                        as *mut libc::c_void,
                    (*edit).buffer.as_mut_ptr().offset((*edit).cursor as
                                                           isize) as
                        *const libc::c_void,
                    (len + 1i32 - (*edit).cursor) as libc::c_ulong);
            (*edit).cursor -= 1;
            if (*edit).cursor < (*edit).scroll { (*edit).scroll -= 1 }
        }
        return
    }
    if ch == 'a' as i32 - 'a' as i32 + 1i32 {
        (*edit).cursor = 0i32;
        (*edit).scroll = 0i32;
        return
    }
    if ch == 'e' as i32 - 'a' as i32 + 1i32 {
        (*edit).cursor = len;
        (*edit).scroll = (*edit).cursor - (*edit).widthInChars;
        return
    }
    if ch < 32i32 { return }
    if 0 != key_overstrikeMode as u64 {
        if (*edit).cursor == 256i32 - 2i32 { return }
        (*edit).buffer[(*edit).cursor as usize] = ch as libc::c_char;
        (*edit).cursor += 1
    } else {
        if len == 256i32 - 2i32 { return }
        memmove((*edit).buffer.as_mut_ptr().offset((*edit).cursor as
                                                       isize).offset(1isize)
                    as *mut libc::c_void,
                (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize) as
                    *const libc::c_void,
                (len + 1i32 - (*edit).cursor) as libc::c_ulong);
        (*edit).buffer[(*edit).cursor as usize] = ch as libc::c_char;
        (*edit).cursor += 1
    }
    if (*edit).cursor >= (*edit).widthInChars { (*edit).scroll += 1 }
    if (*edit).cursor == len + 1i32 {
        (*edit).buffer[(*edit).cursor as usize] = 0i32 as libc::c_char
    };
}
// the line being displayed from history buffer
#[no_mangle]
pub static mut historyLine: libc::c_int = 0;
#[no_mangle]
pub static mut historyEditLines: [field_t; 32] =
    [field_t{cursor: 0, scroll: 0, widthInChars: 0, buffer: [0; 256],}; 32];
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
/*

key up events are sent even if in console mode

*/
// the last line in the history buffer, not masked
#[no_mangle]
pub static mut nextHistoryLine: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn CL_SaveConsoleHistory() {
    let mut i: libc::c_int = 0;
    let mut lineLength: libc::c_int = 0;
    let mut saveBufferLength: libc::c_int = 0;
    let mut additionalLength: libc::c_int = 0;
    let mut f: fileHandle_t = 0;
    consoleSaveBuffer[0usize] = '\u{0}' as i32 as libc::c_char;
    i = (nextHistoryLine - 1i32) % 32i32;
    loop  {
        if 0 != historyEditLines[i as usize].buffer[0usize] {
            lineLength =
                strlen(historyEditLines[i as usize].buffer.as_mut_ptr()) as
                    libc::c_int;
            saveBufferLength =
                strlen(consoleSaveBuffer.as_mut_ptr()) as libc::c_int;
            additionalLength =
                (lineLength as
                     libc::c_ulong).wrapping_add(strlen(b"999 999 999  \x00"
                                                            as *const u8 as
                                                            *const libc::c_char))
                    as libc::c_int;
            if !(saveBufferLength + additionalLength < 1024i32) { break ; }
            Q_strcat(consoleSaveBuffer.as_mut_ptr(), 1024i32,
                     va(b"%d %d %d %s \x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        historyEditLines[i as usize].cursor,
                        historyEditLines[i as usize].scroll, lineLength,
                        historyEditLines[i as usize].buffer.as_mut_ptr()));
        }
        i = (i - 1i32 + 32i32) % 32i32;
        if !(i != (nextHistoryLine - 1i32) % 32i32) { break ; }
    }
    consoleSaveBufferSize =
        strlen(consoleSaveBuffer.as_mut_ptr()) as libc::c_int;
    f =
        FS_FOpenFileWrite(b"q3history\x00" as *const u8 as
                              *const libc::c_char);
    if 0 == f {
        Com_Printf(b"Couldn\'t write %s.\n\x00" as *const u8 as
                       *const libc::c_char,
                   b"q3history\x00" as *const u8 as *const libc::c_char);
        return
    }
    if FS_Write(consoleSaveBuffer.as_mut_ptr() as *const libc::c_void,
                consoleSaveBufferSize, f) < consoleSaveBufferSize {
        Com_Printf(b"Couldn\'t write %s.\n\x00" as *const u8 as
                       *const libc::c_char,
                   b"q3history\x00" as *const u8 as *const libc::c_char);
    }
    FS_FCloseFile(f);
}
static mut consoleSaveBufferSize: libc::c_int = 0i32;
// This must not exceed MAX_CMD_LINE
static mut consoleSaveBuffer: [libc::c_char; 1024] = [0; 1024];
//============================================================================
/*
================
Message_Key

In game talk message
================
*/
#[no_mangle]
pub unsafe extern "C" fn Message_Key(mut key: libc::c_int) {
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    if key == K_ESCAPE as libc::c_int {
        Key_SetCatcher(Key_GetCatcher() & !0x4i32);
        Field_Clear(&mut chatField);
        return
    }
    if key == K_ENTER as libc::c_int || key == K_KP_ENTER as libc::c_int {
        if 0 != chatField.buffer[0usize] as libc::c_int &&
               clc.state as libc::c_uint ==
                   CA_ACTIVE as libc::c_int as libc::c_uint {
            if chat_playerNum != -1i32 {
                Com_sprintf(buffer.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong as libc::c_int,
                            b"tell %i \"%s\"\n\x00" as *const u8 as
                                *const libc::c_char, chat_playerNum,
                            chatField.buffer.as_mut_ptr());
            } else if 0 != chat_team as u64 {
                Com_sprintf(buffer.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong as libc::c_int,
                            b"say_team \"%s\"\n\x00" as *const u8 as
                                *const libc::c_char,
                            chatField.buffer.as_mut_ptr());
            } else {
                Com_sprintf(buffer.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                libc::c_ulong as libc::c_int,
                            b"say \"%s\"\n\x00" as *const u8 as
                                *const libc::c_char,
                            chatField.buffer.as_mut_ptr());
            }
            CL_AddReliableCommand(buffer.as_mut_ptr(), qfalse);
        }
        Key_SetCatcher(Key_GetCatcher() & !0x4i32);
        Field_Clear(&mut chatField);
        return
    }
    Field_KeyDownEvent(&mut chatField, key);
}
#[no_mangle]
pub static mut chatField: field_t =
    field_t{cursor: 0, scroll: 0, widthInChars: 0, buffer: [0; 256],};
#[no_mangle]
pub unsafe extern "C" fn Key_SetCatcher(mut catcher: libc::c_int) {
    if catcher != keyCatchers { Key_ClearStates(); }
    keyCatchers = catcher;
}
#[no_mangle]
pub unsafe extern "C" fn Key_ClearStates() {
    let mut i: libc::c_int = 0;
    anykeydown = 0i32;
    i = 0i32;
    while i < MAX_KEYS as libc::c_int {
        if 0 != keys[i as usize].down as u64 {
            CL_KeyEvent(i, qfalse, 0i32 as libc::c_uint);
        }
        keys[i as usize].down = qfalse;
        keys[i as usize].repeats = 0i32;
        i += 1
    };
}
#[no_mangle]
pub static mut chat_team: qboolean = qfalse;
#[no_mangle]
pub static mut chat_playerNum: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn CL_CharEvent(mut key: libc::c_int) {
    if key == 127i32 { return }
    if 0 != Key_GetCatcher() & 0x1i32 {
        Field_CharEvent(&mut g_consoleField, key);
    } else if 0 != Key_GetCatcher() & 0x2i32 {
        VM_Call(uivm, UI_KEY_EVENT as libc::c_int, key | 1024i32,
                qtrue as libc::c_int);
    } else if 0 != Key_GetCatcher() & 0x4i32 {
        Field_CharEvent(&mut chatField, key);
    } else if clc.state as libc::c_uint ==
                  CA_DISCONNECTED as libc::c_int as libc::c_uint {
        Field_CharEvent(&mut g_consoleField, key);
    };
}
// Restart sound subsystem
#[no_mangle]
pub unsafe extern "C" fn Key_KeynameCompletion(mut callback:
                                                   Option<unsafe extern "C" fn(_:
                                                                                   *const libc::c_char)
                                                              -> ()>) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while !keynames[i as usize].name.is_null() {
        callback.expect("non-null function pointer")(keynames[i as
                                                                  usize].name);
        i += 1
    };
}
// for keyname autocompletion
#[no_mangle]
pub unsafe extern "C" fn Key_WriteBindings(mut f: fileHandle_t) {
    let mut i: libc::c_int = 0;
    FS_Printf(f, b"unbindall\n\x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < MAX_KEYS as libc::c_int {
        if !keys[i as usize].binding.is_null() &&
               0 != *keys[i as usize].binding.offset(0isize) as libc::c_int {
            FS_Printf(f,
                      b"bind %s \"%s\"\n\x00" as *const u8 as
                          *const libc::c_char, Key_KeynumToString(i),
                      keys[i as usize].binding);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Field_Draw(mut edit: *mut field_t,
                                    mut x: libc::c_int, mut y: libc::c_int,
                                    mut width: libc::c_int,
                                    mut showCursor: qboolean,
                                    mut noColorEscape: qboolean) {
    Field_VariableSizeDraw(edit, x, y, width, 8i32, showCursor,
                           noColorEscape);
}
// because a raw semicolon separates commands
/*
=============================================================================

EDIT FIELDS

=============================================================================
*/
/*
===================
Field_Draw

Handles horizontal scrolling and cursor blinking
x, y, and width are in pixels
===================
*/
#[no_mangle]
pub unsafe extern "C" fn Field_VariableSizeDraw(mut edit: *mut field_t,
                                                mut x: libc::c_int,
                                                mut y: libc::c_int,
                                                mut width: libc::c_int,
                                                mut size: libc::c_int,
                                                mut showCursor: qboolean,
                                                mut noColorEscape: qboolean) {
    let mut len: libc::c_int = 0;
    let mut drawLen: libc::c_int = 0;
    let mut prestep: libc::c_int = 0;
    let mut cursorChar: libc::c_int = 0;
    let mut str: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    drawLen = (*edit).widthInChars - 1i32;
    len = strlen((*edit).buffer.as_mut_ptr()) as libc::c_int;
    if len <= drawLen {
        prestep = 0i32
    } else {
        if (*edit).scroll + drawLen > len {
            (*edit).scroll = len - drawLen;
            if (*edit).scroll < 0i32 { (*edit).scroll = 0i32 }
        }
        prestep = (*edit).scroll
    }
    if prestep + drawLen > len { drawLen = len - prestep }
    if drawLen >= 1024i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"drawLen >= MAX_STRING_CHARS\x00" as *const u8 as
                      *const libc::c_char);
    }
    memcpy(str.as_mut_ptr() as *mut libc::c_void,
           (*edit).buffer.as_mut_ptr().offset(prestep as isize) as
               *const libc::c_void, drawLen as libc::c_ulong);
    str[drawLen as usize] = 0i32 as libc::c_char;
    if size == 8i32 {
        let mut color: [libc::c_float; 4] = [0.; 4];
        color[3usize] = 1.0f64 as libc::c_float;
        color[2usize] = color[3usize];
        color[1usize] = color[2usize];
        color[0usize] = color[1usize];
        SCR_DrawSmallStringExt(x, y, str.as_mut_ptr(), color.as_mut_ptr(),
                               qfalse, noColorEscape);
    } else {
        SCR_DrawBigString(x, y, str.as_mut_ptr(), 1.0f64 as libc::c_float,
                          noColorEscape);
    }
    if 0 != showCursor as u64 {
        if 0 != cls.realtime >> 8i32 & 1i32 { return }
        if 0 != key_overstrikeMode as u64 {
            cursorChar = 11i32
        } else { cursorChar = 10i32 }
        i =
            (drawLen as libc::c_ulong).wrapping_sub(strlen(str.as_mut_ptr()))
                as libc::c_int;
        if size == 8i32 {
            SCR_DrawSmallChar(x + ((*edit).cursor - prestep - i) * size, y,
                              cursorChar);
        } else {
            str[0usize] = cursorChar as libc::c_char;
            str[1usize] = 0i32 as libc::c_char;
            SCR_DrawBigString(x + ((*edit).cursor - prestep - i) * size, y,
                              str.as_mut_ptr(), 1.0f64 as libc::c_float,
                              qfalse);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Field_BigDraw(mut edit: *mut field_t,
                                       mut x: libc::c_int, mut y: libc::c_int,
                                       mut width: libc::c_int,
                                       mut showCursor: qboolean,
                                       mut noColorEscape: qboolean) {
    Field_VariableSizeDraw(edit, x, y, width, 16i32, showCursor,
                           noColorEscape);
}
#[no_mangle]
pub unsafe extern "C" fn Key_GetBinding(mut keynum: libc::c_int)
 -> *mut libc::c_char {
    if keynum < 0i32 || keynum >= MAX_KEYS as libc::c_int {
        return b"\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    return keys[keynum as usize].binding;
}
#[no_mangle]
pub unsafe extern "C" fn Key_IsDown(mut keynum: libc::c_int) -> qboolean {
    if keynum < 0i32 || keynum >= MAX_KEYS as libc::c_int { return qfalse }
    return keys[keynum as usize].down;
}
#[no_mangle]
pub unsafe extern "C" fn Key_GetOverstrikeMode() -> qboolean {
    return key_overstrikeMode;
}
#[no_mangle]
pub unsafe extern "C" fn Key_SetOverstrikeMode(mut state: qboolean) {
    key_overstrikeMode = state;
}
#[no_mangle]
pub unsafe extern "C" fn Key_GetKey(mut binding: *const libc::c_char)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if !binding.is_null() {
        i = 0i32;
        while i < MAX_KEYS as libc::c_int {
            if !keys[i as usize].binding.is_null() &&
                   Q_stricmp(binding, keys[i as usize].binding) == 0i32 {
                return i
            }
            i += 1
        }
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn CL_LoadConsoleHistory() {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut text_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut numChars: libc::c_int = 0;
    let mut numLines: libc::c_int = 0i32;
    let mut f: fileHandle_t = 0;
    consoleSaveBufferSize =
        FS_FOpenFileRead(b"q3history\x00" as *const u8 as *const libc::c_char,
                         &mut f, qfalse) as libc::c_int;
    if 0 == f {
        Com_Printf(b"Couldn\'t read %s.\n\x00" as *const u8 as
                       *const libc::c_char,
                   b"q3history\x00" as *const u8 as *const libc::c_char);
        return
    }
    if consoleSaveBufferSize < 1024i32 &&
           FS_Read(consoleSaveBuffer.as_mut_ptr() as *mut libc::c_void,
                   consoleSaveBufferSize, f) == consoleSaveBufferSize {
        consoleSaveBuffer[consoleSaveBufferSize as usize] =
            '\u{0}' as i32 as libc::c_char;
        text_p = consoleSaveBuffer.as_mut_ptr();
        i = 32i32 - 1i32;
        while i >= 0i32 {
            token = COM_Parse(&mut text_p);
            if 0 == *token { break ; }
            historyEditLines[i as usize].cursor = atoi(token);
            token = COM_Parse(&mut text_p);
            if 0 == *token { break ; }
            historyEditLines[i as usize].scroll = atoi(token);
            token = COM_Parse(&mut text_p);
            if 0 == *token { break ; }
            numChars = atoi(token);
            text_p = text_p.offset(1isize);
            if numChars as libc::c_ulong >
                   strlen(consoleSaveBuffer.as_mut_ptr()).wrapping_sub(text_p.wrapping_offset_from(consoleSaveBuffer.as_mut_ptr())
                                                                           as
                                                                           libc::c_long
                                                                           as
                                                                           libc::c_ulong)
               {
                Com_DPrintf(b"^3WARNING: probable corrupt history\n\x00" as
                                *const u8 as *const libc::c_char);
                break ;
            } else {
                memcpy(historyEditLines[i as usize].buffer.as_mut_ptr() as
                           *mut libc::c_void, text_p as *const libc::c_void,
                       numChars as libc::c_ulong);
                historyEditLines[i as usize].buffer[numChars as usize] =
                    '\u{0}' as i32 as libc::c_char;
                text_p = text_p.offset(numChars as isize);
                numLines += 1;
                i -= 1
            }
        }
        memmove(&mut historyEditLines[0usize] as *mut field_t as
                    *mut libc::c_void,
                &mut historyEditLines[(i + 1i32) as usize] as *mut field_t as
                    *const libc::c_void,
                (numLines as
                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<field_t>()
                                                     as libc::c_ulong));
        i = numLines;
        while i < 32i32 {
            Field_Clear(&mut historyEditLines[i as usize]);
            i += 1
        }
        nextHistoryLine = numLines;
        historyLine = nextHistoryLine
    } else {
        Com_Printf(b"Couldn\'t read %s.\n\x00" as *const u8 as
                       *const libc::c_char,
                   b"q3history\x00" as *const u8 as *const libc::c_char);
    }
    FS_FCloseFile(f);
}