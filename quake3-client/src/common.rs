use libc;
#[header_src = "vararg"]
pub mod vararg {
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __off_t = libc::c_long;
    pub type __off64_t = libc::c_long;
    pub type __time_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/libio.h"]
pub mod libio_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: libc::c_int,
        pub _IO_read_ptr: *mut libc::c_char,
        pub _IO_read_end: *mut libc::c_char,
        pub _IO_read_base: *mut libc::c_char,
        pub _IO_write_base: *mut libc::c_char,
        pub _IO_write_ptr: *mut libc::c_char,
        pub _IO_write_end: *mut libc::c_char,
        pub _IO_buf_base: *mut libc::c_char,
        pub _IO_buf_end: *mut libc::c_char,
        pub _IO_save_base: *mut libc::c_char,
        pub _IO_backup_base: *mut libc::c_char,
        pub _IO_save_end: *mut libc::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: libc::c_int,
        pub _flags2: libc::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: libc::c_ushort,
        pub _vtable_offset: libc::c_schar,
        pub _shortbuf: [libc::c_char; 1],
        pub _lock: *mut libc::c_void,
        pub _offset: __off64_t,
        pub __pad1: *mut libc::c_void,
        pub __pad2: *mut libc::c_void,
        pub __pad3: *mut libc::c_void,
        pub __pad4: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    pub type _IO_lock_t = ();
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct _IO_marker {
        pub _next: *mut _IO_marker,
        pub _sbuf: *mut _IO_FILE,
        pub _pos: libc::c_int,
    }
    use super::{libc};
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::{size_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/FILE.h"]
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::libio_h::{_IO_FILE};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stdarg.h"]
pub mod stdarg_h {
    pub type va_list = __builtin_va_list;
    use super::vararg::{__builtin_va_list};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/time_t.h"]
pub mod time_t_h {
    pub type time_t = __time_t;
    use super::types_h::{__time_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/__sigset_t.h"]
pub mod __sigset_t_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct __sigset_t {
        pub __val: [libc::c_ulong; 16],
    }
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_tm.h"]
pub mod struct_tm_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct tm {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
        pub tm_gmtoff: libc::c_long,
        pub tm_zone: *const libc::c_char,
    }
    use super::{libc};
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
    pub type fileHandle_t = libc::c_int;
    // expand constants before stringifying them
    // angle indexes
    // up / down
    // left / right
    // fall over
    // the game guarantees that no string from the network will ever
// exceed MAX_STRING_CHARS
    // max length of a string passed to Cmd_TokenizeString
    // max tokens resulting from Cmd_TokenizeString
    // max length of an individual token
    // used for system info key only
    // max length of a quake game pathname
    // max length of a client name
    // parameters for command buffer stuffing
    pub type unnamed = libc::c_uint;
    // add to end of the command buffer (normal case)
    pub const EXEC_APPEND: unnamed = 2;
    // because some commands might cause the VM to be unloaded...
    // insert at current position, but don't run yet
    pub const EXEC_INSERT: unnamed = 1;
    // don't return until completed, a VM should NEVER use this,
    pub const EXEC_NOW: unnamed = 0;
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
    // font rendering values used by ui and cgame
    // default
    // default
    pub type ha_pref = libc::c_uint;
    pub const h_dontcare: ha_pref = 2;
    pub const h_low: ha_pref = 1;
    pub const h_high: ha_pref = 0;
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
    // real time
//=============================================
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct qtime_s {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
    }
    pub type qtime_t = qtime_s;
    use super::{libc};
    extern "C" {
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
        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int,
                        src: *const libc::c_char);
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn qvmftolsse() -> libc::c_int;
        #[no_mangle]
        pub fn COM_CompareExtension(in_0: *const libc::c_char,
                                    ext: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn COM_DefaultExtension(path: *mut libc::c_char,
                                    maxSize: libc::c_int,
                                    extension: *const libc::c_char);
        #[no_mangle]
        pub fn Com_HexStrToInt(str: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn Com_SkipCharset(s: *mut libc::c_char, sep: *mut libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn Q_stricmpn(s1: *const libc::c_char, s2: *const libc::c_char,
                          n: libc::c_int) -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        #[no_mangle]
        pub fn Com_TruncateLongString(buffer: *mut libc::c_char,
                                      s: *const libc::c_char);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/common.c"]
pub mod common_c {
    pub type hunkblock_t = hunkblock_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct hunkblock_s {
        pub size: libc::c_int,
        pub printed: byte,
        pub next: *mut hunkblock_s,
        pub label: *mut libc::c_char,
        pub file: *mut libc::c_char,
        pub line: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct hunkUsed_t {
        pub mark: libc::c_int,
        pub permanent: libc::c_int,
        pub temp: libc::c_int,
        pub tempHighwater: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct memblock_s {
        pub size: libc::c_int,
        pub tag: libc::c_int,
        pub next: *mut memblock_s,
        pub prev: *mut memblock_s,
        pub id: libc::c_int,
        pub d: zonedebug_t,
    }
    pub type zonedebug_t = zonedebug_s;
    /*
==============================================================================

						ZONE MEMORY ALLOCATION

There is never any space between memblocks, and there will never be two
contiguous free memblocks.

The rover can be left pointing at a non-empty block

The zone calls are pretty much only used for small strings and structures,
all big things are allocated on the hunk.
==============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct zonedebug_s {
        // static mem blocks to reduce a lot of small zone overhead
        /*
==============================================================================

Goals:
	reproducible without history effects -- no out of memory errors on weird map to map changes
	allow restarting of the client without fragmentation
	minimize total pages in use at run time
	minimize total pages needed during load time

  Single block of memory with stack allocators coming from both ends towards the middle.

  One side is designated the temporary memory allocator.

  Temporary memory can be allocated and freed in any order.

  A highwater mark is kept of the most in use at any time.

  When there is no temporary memory allocated, the permanent and temp sides
  can be switched, allowing the already touched temp memory to be used for
  permanent storage.

  Temp memory must never be allocated on two ends at once, or fragmentation
  could occur.

  If we have any in-use temp memory, additional temp allocations must come from
  that side.

  If not, we can choose to make either side the new temp side and push future
  permanent allocations to the other side.  Permanent allocations should be
  kept on the side that has the current greatest wasted highwater mark.

==============================================================================
*/
        // TTimo: centralizing the cl_cdkey stuff after I discovered a buffer overflow problem with the dedicated server version
//   not sure it's necessary to have different defaults for regular and dedicated, but I don't want to risk it
//   https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=470
        /*
=================
Com_ReadCDKey
=================
*/
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
// qcommon.h -- definitions common between client and server, but not game.or ref modules
        //Ignore __attribute__ on non-gcc platforms
        //#define	PRE_RELEASE_DEMO
        //============================================================================
        //
// msg.c
//
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
        // an address lookup failed
        // maximum length of an IPv6 address string including trailing '\0'
        // Pulls off \n terminated lines of text from the command buffer and sends
// them through Cmd_ExecuteString.  Stops when the buffer is empty.
// Normally called once per frame, but may be explicitly invoked.
// Do not call inside a command function, or current args will be destroyed.
        //===========================================================================
        /*

Command execution takes a null terminated string, breaks it into tokens,
then searches for a command or variable that matches the first token.

*/
        /*
==============================================================

Edit fields and command line history/completion

==============================================================
*/
        // returned by Sys_GetProcessorFeatures
        // centralized and cleaned, that's the max string you can send to a Com_Printf / Com_DPrintf (above gets truncated)
        pub label: *mut libc::c_char,
        pub file: *mut libc::c_char,
        pub line: libc::c_int,
        pub allocSize: libc::c_int,
    }
    pub type memblock_t = memblock_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct memzone_t {
        pub size: libc::c_int,
        pub used: libc::c_int,
        pub blocklist: memblock_t,
        pub rover: *mut memblock_t,
    }
    pub type memstatic_t = memstatic_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct memstatic_s {
        // evPtr is a char*
        pub b: memblock_t,
        pub mem: [byte; 2],
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct hunkHeader_t {
        // evValue is an axis number and evValue2 is the current state (-127 to 127)
        pub magic: libc::c_int,
        pub size: libc::c_int,
    }
    use super::{libc};
    use super::q_shared_h::{byte, cvar_t, qboolean};
    use super::include_setjmp_h::{jmp_buf};
    use super::qcommon_h::{sysEvent_t};
    use super::FILE_h::{FILE};
    extern "C" {
        #[no_mangle]
        pub fn SV_ShutdownGameProgs();
        #[no_mangle]
        pub fn CL_CDKeyValidate(key: *const libc::c_char,
                                checksum: *const libc::c_char) -> qboolean;
    }
}
#[header_src = "/usr/include/setjmp.h"]
pub mod include_setjmp_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct __jmp_buf_tag {
        pub __jmpbuf: __jmp_buf,
        pub __mask_was_saved: libc::c_int,
        pub __saved_mask: __sigset_t,
    }
    pub type jmp_buf = [__jmp_buf_tag; 1];
    use super::setjmp_h::{__jmp_buf};
    use super::{libc};
    use super::__sigset_t_h::{__sigset_t};
    extern "C" {
        #[no_mangle]
        pub fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
        #[no_mangle]
        pub fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/setjmp.h"]
pub mod setjmp_h {
    pub type __jmp_buf = [libc::c_long; 8];
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct msg_t {
        pub allowoverflow: qboolean,
        pub overflowed: qboolean,
        pub oob: qboolean,
        pub data: *mut byte,
        pub maxsize: libc::c_int,
        pub cursize: libc::c_int,
        pub readcount: libc::c_int,
        pub bit: libc::c_int,
    }
    pub type netadrtype_t = libc::c_uint;
    pub const NA_UNSPEC: netadrtype_t = 7;
    pub const NA_MULTICAST6: netadrtype_t = 6;
    pub const NA_IP6: netadrtype_t = 5;
    pub const NA_IP: netadrtype_t = 4;
    pub const NA_BROADCAST: netadrtype_t = 3;
    pub const NA_LOOPBACK: netadrtype_t = 2;
    pub const NA_BOT: netadrtype_t = 1;
    pub const NA_BAD: netadrtype_t = 0;
    pub type netsrc_t = libc::c_uint;
    pub const NS_SERVER: netsrc_t = 1;
    pub const NS_CLIENT: netsrc_t = 0;
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
    pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
    pub type completionFunc_t
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
                   -> ()>;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct field_t {
        pub cursor: libc::c_int,
        pub scroll: libc::c_int,
        pub widthInChars: libc::c_int,
        pub buffer: [libc::c_char; 256],
    }
    pub type cpuFeatures_t = libc::c_uint;
    pub const CF_ALTIVEC: cpuFeatures_t = 128;
    pub const CF_SSE2: cpuFeatures_t = 64;
    pub const CF_SSE: cpuFeatures_t = 32;
    pub const CF_3DNOW_EXT: cpuFeatures_t = 16;
    pub const CF_3DNOW: cpuFeatures_t = 8;
    pub const CF_MMX_EXT: cpuFeatures_t = 4;
    pub const CF_MMX: cpuFeatures_t = 2;
    pub const CF_RDTSC: cpuFeatures_t = 1;
    pub type sysEventType_t = libc::c_uint;
    pub const SE_CONSOLE: sysEventType_t = 5;
    pub const SE_JOYSTICK_AXIS: sysEventType_t = 4;
    // evValue and evValue2 are relative signed x / y moves
    pub const SE_MOUSE: sysEventType_t = 3;
    // evValue is an ascii char
    pub const SE_CHAR: sysEventType_t = 2;
    // evValue is a key code, evValue2 is the down flag
    pub const SE_KEY: sysEventType_t = 1;
    // SE_NONE must be zero
    // evTime is still valid
    pub const SE_NONE: sysEventType_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sysEvent_t {
        pub evTime: libc::c_int,
        pub evType: sysEventType_t,
        pub evValue: libc::c_int,
        pub evValue2: libc::c_int,
        pub evPtrLength: libc::c_int,
        pub evPtr: *mut libc::c_void,
    }
    pub const TAG_SMALL: unnamed_1 = 4;
    pub const TAG_STATIC: unnamed_1 = 5;
    pub const TAG_GENERAL: unnamed_1 = 1;
    pub type unnamed_1 = libc::c_uint;
    pub const TAG_RENDERER: unnamed_1 = 3;
    pub const TAG_BOTLIB: unnamed_1 = 2;
    pub const TAG_FREE: unnamed_1 = 0;
    use super::q_shared_h::{qboolean, byte, fileHandle_t, cvar_t, qtime_t};
    use super::{libc};
    use super::stdint_uintn_h::{uint8_t};
    extern "C" {
        #[no_mangle]
        pub fn Sys_Error(error: *const libc::c_char, ...) -> !;
        #[no_mangle]
        pub fn FS_HomeRemove(homePath: *const libc::c_char);
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        #[no_mangle]
        pub fn VM_Forced_Unload_Done();
        #[no_mangle]
        pub fn SV_Shutdown(finalmsg: *mut libc::c_char);
        #[no_mangle]
        pub fn CL_Shutdown(finalmsg: *mut libc::c_char, disconnect: qboolean,
                           quit: qboolean);
        #[no_mangle]
        pub fn VM_Forced_Unload_Start();
        #[no_mangle]
        pub fn FS_PureServerSetLoadedPaks(pakSums: *const libc::c_char,
                                          pakNames: *const libc::c_char);
        // returns 1 if a file is in the PAK file, otherwise -1
        #[no_mangle]
        pub fn FS_Write(buffer: *const libc::c_void, len: libc::c_int,
                        f: fileHandle_t) -> libc::c_int;
        // whenever a cvar is modifed, its flags will be OR'd into this, so
// a single check can determine if any CVAR_USERINFO, CVAR_SERVERINFO,
// etc, variables have been modified since the last check.  The bit
// can then be cleared to allow another change detection.
        /*
==============================================================

FILESYSTEM

No stdio calls should be used by any part of the game, because
we need to deal with all sorts of directory and seperator char
issues.
==============================================================
*/
        // referenced flags
// these are in loop specific order so don't change the order
        // number of id paks that will never be autodownloaded from baseq3/missionpack
        #[no_mangle]
        pub fn FS_Initialized() -> qboolean;
        // don't set the cvar immediately
        #[no_mangle]
        pub fn Cvar_SetValue(var_name: *const libc::c_char,
                             value: libc::c_float);
        // returns the length of the file
// a null buffer will just return the file length without loading
// as a quick check for existence. -1 length == not present
// A 0 byte will always be appended at the end, so string ops are safe.
// the buffer should be considered read-only, because it may be cached
// for other uses.
        #[no_mangle]
        pub fn FS_ForceFlush(f: fileHandle_t);
        #[no_mangle]
        pub fn FS_FOpenFileWrite(qpath: *const libc::c_char) -> fileHandle_t;
        #[no_mangle]
        pub fn Sys_Print(msg: *const libc::c_char);
        // adds the current command line as a clc_clientCommand to the client message.
// things like godmode, noclip, etc, are commands directed to the server,
// so when they are typed in at the console, they will need to be forwarded.
        #[no_mangle]
        pub fn CL_CDDialog();
        // bring up the "need a cd to play" dialog
        #[no_mangle]
        pub fn CL_FlushMemory();
        #[no_mangle]
        pub fn CL_Disconnect(showMainMenu: qboolean);
        // the keyboard binding interface must be setup before execing
// config files, but the rest of client startup will happen later
        #[no_mangle]
        pub fn CL_Init();
        // updates an interpreted modules' version of a cvar
        #[no_mangle]
        pub fn Cvar_Set(var_name: *const libc::c_char,
                        value: *const libc::c_char);
        // Sys_Milliseconds should only be used for profiling purposes,
// any game related timing information should come from event timestamps
        #[no_mangle]
        pub fn Sys_Milliseconds() -> libc::c_int;
        #[no_mangle]
        pub fn Sys_RandomBytes(string: *mut byte, len: libc::c_int)
         -> qboolean;
        #[no_mangle]
        pub fn MSG_Init(buf: *mut msg_t, data: *mut byte,
                        length: libc::c_int);
        #[no_mangle]
        pub fn MSG_ReportChangeVectors_f();
        #[no_mangle]
        pub fn NET_Restart_f();
        #[no_mangle]
        pub fn NET_FlushPacketQueue();
        #[no_mangle]
        pub fn NET_GetLoopPacket(sock: netsrc_t, net_from: *mut netadr_t,
                                 net_message: *mut msg_t) -> qboolean;
        #[no_mangle]
        pub fn NET_Sleep(msec: libc::c_int);
        #[no_mangle]
        pub fn Netchan_Init(qport: libc::c_int);
        #[no_mangle]
        pub fn VM_Init();
        #[no_mangle]
        pub fn VM_Clear();
        /*
==============================================================

CMD

Command text buffering and command execution

==============================================================
*/
        /*

Any number of commands can be added in a frame, from several different sources.
Most commands come from either keybindings or console line input, but entire text
files can be execed.

*/
        #[no_mangle]
        pub fn Cbuf_Init();
        // allocates an initial text buffer that will grow as needed
        #[no_mangle]
        pub fn Cbuf_AddText(text: *const libc::c_char);
        // Adds command text at the end of the buffer, does NOT add a final \n
        #[no_mangle]
        pub fn Cbuf_ExecuteText(exec_when: libc::c_int,
                                text: *const libc::c_char);
        // this can be used in place of either Cbuf_AddText or Cbuf_InsertText
        #[no_mangle]
        pub fn Cbuf_Execute();
        #[no_mangle]
        pub fn Cmd_Init();
        #[no_mangle]
        pub fn Cmd_AddCommand(cmd_name: *const libc::c_char,
                              function: xcommand_t);
        #[no_mangle]
        pub fn Cmd_CommandCompletion(callback:
                                         Option<unsafe extern "C" fn(_:
                                                                         *const libc::c_char)
                                                    -> ()>);
        // callback with each valid string
        #[no_mangle]
        pub fn Cmd_SetCommandCompletionFunc(command: *const libc::c_char,
                                            complete: completionFunc_t);
        #[no_mangle]
        pub fn Cmd_CompleteArgument(command: *const libc::c_char,
                                    args: *mut libc::c_char,
                                    argNum: libc::c_int);
        #[no_mangle]
        pub fn Cmd_CompleteCfgName(args: *mut libc::c_char,
                                   argNum: libc::c_int);
        #[no_mangle]
        pub fn Cmd_Argc() -> libc::c_int;
        #[no_mangle]
        pub fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_Args() -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_ArgsFrom(arg: libc::c_int) -> *mut libc::c_char;
        // The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
        #[no_mangle]
        pub fn Cmd_TokenizeString(text: *const libc::c_char);
        #[no_mangle]
        pub fn Cmd_TokenizeStringIgnoreQuotes(text_in: *const libc::c_char);
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
        // will create the variable with no flags if it doesn't exist
        #[no_mangle]
        pub fn Cvar_Set2(var_name: *const libc::c_char,
                         value: *const libc::c_char, force: qboolean)
         -> *mut cvar_t;
        // returns 0 if not defined or non numeric
        #[no_mangle]
        pub fn Cvar_VariableString(var_name: *const libc::c_char)
         -> *mut libc::c_char;
        // returns an empty string if not defined
        #[no_mangle]
        pub fn Cvar_Flags(var_name: *const libc::c_char) -> libc::c_int;
        // returns CVAR_NONEXISTENT if cvar doesn't exist or the flags of that particular CVAR.
        #[no_mangle]
        pub fn Cvar_CommandCompletion(callback:
                                          Option<unsafe extern "C" fn(_:
                                                                          *const libc::c_char)
                                                     -> ()>);
        // called by Cmd_ExecuteString when Cmd_Argv(0) doesn't match a known
// command.  Returns true if the command was a variable reference that
// was handled. (print or change)
        #[no_mangle]
        pub fn Cvar_WriteVariables(f: fileHandle_t);
        // writes lines containing "set variable value" for all variables
// with the archive flag set to true.
        #[no_mangle]
        pub fn Cvar_Init();
        #[no_mangle]
        pub fn Cvar_CheckRange(cv: *mut cvar_t, minVal: libc::c_float,
                               maxVal: libc::c_float,
                               shouldBeIntegral: qboolean);
        #[no_mangle]
        pub fn Cvar_SetDescription(var: *mut cvar_t,
                                   var_description: *const libc::c_char);
        #[no_mangle]
        pub fn Cvar_Restart(unsetVM: qboolean);
        #[no_mangle]
        pub static mut cvar_modifiedFlags: libc::c_int;
        #[no_mangle]
        pub fn FS_InitFilesystem();
        #[no_mangle]
        pub fn FS_Shutdown(closemfp: qboolean);
        #[no_mangle]
        pub fn FS_Restart(checksumFeed: libc::c_int);
        #[no_mangle]
        pub fn FS_LoadStack() -> libc::c_int;
        #[no_mangle]
        pub fn FS_FCreateOpenPipeFile(filename: *const libc::c_char)
         -> fileHandle_t;
        #[no_mangle]
        pub fn FS_SV_FOpenFileRead(filename: *const libc::c_char,
                                   fp: *mut fileHandle_t) -> libc::c_long;
        #[no_mangle]
        pub fn FS_FOpenFileRead(qpath: *const libc::c_char,
                                file: *mut fileHandle_t, uniqueFILE: qboolean)
         -> libc::c_long;
        #[no_mangle]
        pub fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int,
                       f: fileHandle_t) -> libc::c_int;
        #[no_mangle]
        pub fn FS_Printf(f: fileHandle_t, fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn FS_FilenameCompletion(dir: *const libc::c_char,
                                     ext: *const libc::c_char,
                                     stripExt: qboolean,
                                     callback:
                                         Option<unsafe extern "C" fn(_:
                                                                         *const libc::c_char)
                                                    -> ()>,
                                     allowNonPureFilesOnDisk: qboolean);
        #[no_mangle]
        pub fn FS_GetCurrentGameDir() -> *const libc::c_char;
        #[no_mangle]
        pub fn CL_JoystickEvent(axis: libc::c_int, value: libc::c_int,
                                time_0: libc::c_int);
        // char events are for field typing, not game control
        #[no_mangle]
        pub fn CL_MouseEvent(dx: libc::c_int, dy: libc::c_int,
                             time_0: libc::c_int);
        #[no_mangle]
        pub fn CL_CharEvent(key: libc::c_int);
        #[no_mangle]
        pub fn CL_KeyEvent(key: libc::c_int, down: qboolean,
                           time_0: libc::c_uint);
        #[no_mangle]
        pub fn SV_PacketEvent(from: netadr_t, msg: *mut msg_t);
        #[no_mangle]
        pub fn CL_PacketEvent(from: netadr_t, msg: *mut msg_t);
        #[no_mangle]
        pub fn Sys_ConsoleInput() -> *mut libc::c_char;
        #[no_mangle]
        pub fn Sys_Quit() -> !;
        // initialize renderer interface
        #[no_mangle]
        pub fn CL_StartHunkUsers(rendererOnly: qboolean);
        #[no_mangle]
        pub fn Sys_GetProcessorFeatures() -> cpuFeatures_t;
        // AVI files have the start of pixel lines 4 byte-aligned
        //
// server interface
//
        #[no_mangle]
        pub fn SV_Init();
        #[no_mangle]
        pub fn Sys_InitPIDFile(gamedir: *const libc::c_char);
        /*
==============================================================

NON-PORTABLE SYSTEM SERVICES

==============================================================
*/
        #[no_mangle]
        pub fn Sys_Init();
        // for keyname autocompletion
        #[no_mangle]
        pub fn Key_WriteBindings(f: fileHandle_t);
        #[no_mangle]
        pub fn Sys_SetEnv(name: *const libc::c_char,
                          value: *const libc::c_char);
        /*
==============================================================

CLIENT / SERVER SYSTEMS

==============================================================
*/
        //
// client interface
//
        #[no_mangle]
        pub fn CL_InitKeyCommands();
        #[no_mangle]
        pub fn SV_Frame(msec: libc::c_int);
        #[no_mangle]
        pub fn IN_Frame();
        #[no_mangle]
        pub fn SV_SendQueuedPackets() -> libc::c_int;
        #[no_mangle]
        pub fn SV_FrameMsec() -> libc::c_int;
    }
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    use super::vararg::{__va_list_tag};
    extern "C" {
        #[no_mangle]
        pub fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                         _: *const libc::c_char, _: *mut __va_list_tag)
         -> libc::c_int;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn atof(__nptr: *const libc::c_char) -> libc::c_double;
        #[no_mangle]
        pub fn rand() -> libc::c_int;
        #[no_mangle]
        pub fn srand(__seed: libc::c_uint);
        #[no_mangle]
        pub fn calloc(_: libc::c_ulong, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[header_src = "/usr/include/time.h"]
pub mod time_h {
    use super::time_t_h::{time_t};
    use super::struct_tm_h::{tm};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn time(__timer: *mut time_t) -> time_t;
        #[no_mangle]
        pub fn localtime(__timer: *const time_t) -> *mut tm;
        #[no_mangle]
        pub fn asctime(__tp: *const tm) -> *mut libc::c_char;
    }
}
#[header_src = "/usr/include/ctype.h"]
pub mod ctype_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn toupper(_: libc::c_int) -> libc::c_int;
    }
}
use self::vararg::{__builtin_va_list, __va_list_tag};
use self::types_h::{__uint8_t, __off_t, __off64_t, __time_t};
use self::stddef_h::{size_t};
use self::libio_h::{_IO_FILE, _IO_lock_t, _IO_marker};
use self::FILE_h::{FILE};
use self::stdarg_h::{va_list};
use self::time_t_h::{time_t};
use self::__sigset_t_h::{__sigset_t};
use self::struct_tm_h::{tm};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, unnamed,
                       EXEC_APPEND, EXEC_INSERT, EXEC_NOW, unnamed_0,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, ha_pref, h_dontcare, h_low,
                       h_high, cvar_t, cvar_s, qtime_s, qtime_t, va, Q_strcat,
                       Com_sprintf, Q_stricmp, qvmftolsse,
                       COM_CompareExtension, COM_DefaultExtension,
                       Com_HexStrToInt, Com_SkipCharset, Q_stricmpn,
                       Q_strncpyz, Com_TruncateLongString};
use self::common_c::{hunkblock_t, hunkblock_s, hunkUsed_t, memblock_s,
                     zonedebug_t, zonedebug_s, memblock_t, memzone_t,
                     memstatic_t, memstatic_s, hunkHeader_t,
                     SV_ShutdownGameProgs, CL_CDKeyValidate};
use self::include_setjmp_h::{__jmp_buf_tag, jmp_buf, longjmp, _setjmp};
use self::setjmp_h::{__jmp_buf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, xcommand_t,
                      completionFunc_t, field_t, cpuFeatures_t, CF_ALTIVEC,
                      CF_SSE2, CF_SSE, CF_3DNOW_EXT, CF_3DNOW, CF_MMX_EXT,
                      CF_MMX, CF_RDTSC, sysEventType_t, SE_CONSOLE,
                      SE_JOYSTICK_AXIS, SE_MOUSE, SE_CHAR, SE_KEY, SE_NONE,
                      sysEvent_t, TAG_SMALL, TAG_STATIC, TAG_GENERAL,
                      unnamed_1, TAG_RENDERER, TAG_BOTLIB, TAG_FREE,
                      Sys_Error, FS_HomeRemove, FS_FCloseFile,
                      VM_Forced_Unload_Done, SV_Shutdown, CL_Shutdown,
                      VM_Forced_Unload_Start, FS_PureServerSetLoadedPaks,
                      FS_Write, FS_Initialized, Cvar_SetValue, FS_ForceFlush,
                      FS_FOpenFileWrite, Sys_Print, CL_CDDialog,
                      CL_FlushMemory, CL_Disconnect, CL_Init, Cvar_Set,
                      Sys_Milliseconds, Sys_RandomBytes, MSG_Init,
                      MSG_ReportChangeVectors_f, NET_Restart_f,
                      NET_FlushPacketQueue, NET_GetLoopPacket, NET_Sleep,
                      Netchan_Init, VM_Init, VM_Clear, Cbuf_Init,
                      Cbuf_AddText, Cbuf_ExecuteText, Cbuf_Execute, Cmd_Init,
                      Cmd_AddCommand, Cmd_CommandCompletion,
                      Cmd_SetCommandCompletionFunc, Cmd_CompleteArgument,
                      Cmd_CompleteCfgName, Cmd_Argc, Cmd_Argv, Cmd_Args,
                      Cmd_ArgsFrom, Cmd_TokenizeString,
                      Cmd_TokenizeStringIgnoreQuotes, Cvar_Get, Cvar_Set2,
                      Cvar_VariableString, Cvar_Flags, Cvar_CommandCompletion,
                      Cvar_WriteVariables, Cvar_Init, Cvar_CheckRange,
                      Cvar_SetDescription, Cvar_Restart, cvar_modifiedFlags,
                      FS_InitFilesystem, FS_Shutdown, FS_Restart,
                      FS_LoadStack, FS_FCreateOpenPipeFile,
                      FS_SV_FOpenFileRead, FS_FOpenFileRead, FS_Read,
                      FS_Printf, FS_FilenameCompletion, FS_GetCurrentGameDir,
                      CL_JoystickEvent, CL_MouseEvent, CL_CharEvent,
                      CL_KeyEvent, SV_PacketEvent, CL_PacketEvent,
                      Sys_ConsoleInput, Sys_Quit, CL_StartHunkUsers,
                      Sys_GetProcessorFeatures, SV_Init, Sys_InitPIDFile,
                      Sys_Init, Key_WriteBindings, Sys_SetEnv,
                      CL_InitKeyCommands, SV_Frame, IN_Frame,
                      SV_SendQueuedPackets, SV_FrameMsec};
use self::stdio_h::{vsnprintf};
use self::string_h::{memmove, memset, strcpy, strcat, strcmp, strlen};
use self::stdlib_h::{atof, rand, srand, calloc, getenv};
use self::time_h::{time, localtime, asctime};
use self::ctype_h::{tolower, toupper};
#[no_mangle]
pub unsafe extern "C" fn Hunk_AllocDebug(mut size: libc::c_int,
                                         mut preference: ha_pref,
                                         mut label: *mut libc::c_char,
                                         mut file: *mut libc::c_char,
                                         mut line: libc::c_int)
 -> *mut libc::c_void {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    if s_hunkData.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Hunk_Alloc: Hunk memory system not initialized\x00" as
                      *const u8 as *const libc::c_char);
    }
    if preference as libc::c_uint == h_dontcare as libc::c_int as libc::c_uint
           || (*hunk_temp).temp != (*hunk_temp).permanent {
        Hunk_SwapBanks();
    } else if preference as libc::c_uint ==
                  h_low as libc::c_int as libc::c_uint &&
                  hunk_permanent != &mut hunk_low as *mut hunkUsed_t {
        Hunk_SwapBanks();
    } else if preference as libc::c_uint ==
                  h_high as libc::c_int as libc::c_uint &&
                  hunk_permanent != &mut hunk_high as *mut hunkUsed_t {
        Hunk_SwapBanks();
    }
    size =
        (size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<hunkblock_t>()
                                             as libc::c_ulong) as libc::c_int
            as libc::c_int;
    size = size + 31i32 & !31i32;
    if hunk_low.temp + hunk_high.temp + size > s_hunkTotal {
        Hunk_Log();
        Hunk_SmallLog();
        Com_Error(ERR_DROP as libc::c_int,
                  b"Hunk_Alloc failed on %i: %s, line: %d (%s)\x00" as
                      *const u8 as *const libc::c_char, size, file, line,
                  label);
    }
    if hunk_permanent == &mut hunk_low as *mut hunkUsed_t {
        buf =
            s_hunkData.offset((*hunk_permanent).permanent as isize) as
                *mut libc::c_void;
        (*hunk_permanent).permanent += size
    } else {
        (*hunk_permanent).permanent += size;
        buf =
            s_hunkData.offset(s_hunkTotal as
                                  isize).offset(-((*hunk_permanent).permanent
                                                      as isize)) as
                *mut libc::c_void
    }
    (*hunk_permanent).temp = (*hunk_permanent).permanent;
    memset(buf, 0i32, size as libc::c_ulong);
    let mut block: *mut hunkblock_t = 0 as *mut hunkblock_t;
    block = buf as *mut hunkblock_t;
    (*block).size =
        (size as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<hunkblock_t>()
                                             as libc::c_ulong) as libc::c_int;
    (*block).file = file;
    (*block).label = label;
    (*block).line = line;
    (*block).next = hunkblocks;
    hunkblocks = block;
    buf =
        (buf as
             *mut byte).offset(::std::mem::size_of::<hunkblock_t>() as
                                   libc::c_ulong as isize) as
            *mut libc::c_void;
    return buf;
}
static mut hunkblocks: *mut hunkblock_t =
    0 as *const hunkblock_t as *mut hunkblock_t;
static mut hunk_permanent: *mut hunkUsed_t =
    0 as *const hunkUsed_t as *mut hunkUsed_t;
static mut s_hunkTotal: libc::c_int = 0;
static mut s_hunkData: *mut byte = 0 as *const byte as *mut byte;
static mut hunk_low: hunkUsed_t =
    hunkUsed_t{mark: 0, permanent: 0, temp: 0, tempHighwater: 0,};
// this is only here so the functions in q_shared.c and bg_*.c can link
#[no_mangle]
pub unsafe extern "C" fn Com_Error(mut code: libc::c_int,
                                   mut fmt: *const libc::c_char, ...) -> ! {
    static mut lastErrorTime: libc::c_int = 0;
    static mut errorCount: libc::c_int = 0;
    let mut currentTime: libc::c_int = 0;
    let mut restartClient: qboolean = qfalse;
    if 0 != com_errorEntered as u64 {
        Sys_Error(b"recursive error after: %s\x00" as *const u8 as
                      *const libc::c_char, com_errorMessage.as_mut_ptr());
    }
    com_errorEntered = qtrue;
    Cvar_Set(b"com_errorCode\x00" as *const u8 as *const libc::c_char,
             va(b"%i\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char, code));
    if !com_buildScript.is_null() && 0 != (*com_buildScript).integer {
        code = ERR_FATAL as libc::c_int
    }
    currentTime = Sys_Milliseconds();
    if currentTime - lastErrorTime < 100i32 {
        errorCount += 1;
        if errorCount > 3i32 { code = ERR_FATAL as libc::c_int }
    } else { errorCount = 0i32 }
    lastErrorTime = currentTime;
    vsnprintf(com_errorMessage.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
              fmt, argptr);
    if code != ERR_DISCONNECT as libc::c_int &&
           code != ERR_NEED_CD as libc::c_int {
        Cvar_Set(b"com_errorMessage\x00" as *const u8 as *const libc::c_char,
                 com_errorMessage.as_mut_ptr());
    }
    restartClient =
        (0 != com_gameClientRestarting as libc::c_uint &&
             !(!com_cl_running.is_null() && 0 != (*com_cl_running).integer))
            as libc::c_int as qboolean;
    com_gameRestarting = qfalse;
    com_gameClientRestarting = qfalse;
    if code == ERR_DISCONNECT as libc::c_int ||
           code == ERR_SERVERDISCONNECT as libc::c_int {
        VM_Forced_Unload_Start();
        SV_Shutdown(b"Server disconnected\x00" as *const u8 as
                        *const libc::c_char as *mut libc::c_char);
        if 0 != restartClient as u64 { CL_Init(); }
        CL_Disconnect(qtrue);
        CL_FlushMemory();
        VM_Forced_Unload_Done();
        FS_PureServerSetLoadedPaks(b"\x00" as *const u8 as
                                       *const libc::c_char,
                                   b"\x00" as *const u8 as
                                       *const libc::c_char);
        com_errorEntered = qfalse;
        longjmp(abortframe.as_mut_ptr(), -1i32);
    } else {
        if code == ERR_DROP as libc::c_int {
            Com_Printf(b"********************\nERROR: %s\n********************\n\x00"
                           as *const u8 as *const libc::c_char,
                       com_errorMessage.as_mut_ptr());
            VM_Forced_Unload_Start();
            SV_Shutdown(va(b"Server crashed: %s\x00" as *const u8 as
                               *const libc::c_char as *mut libc::c_char,
                           com_errorMessage.as_mut_ptr()));
            if 0 != restartClient as u64 { CL_Init(); }
            CL_Disconnect(qtrue);
            CL_FlushMemory();
            VM_Forced_Unload_Done();
            FS_PureServerSetLoadedPaks(b"\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"\x00" as *const u8 as
                                           *const libc::c_char);
            com_errorEntered = qfalse;
            longjmp(abortframe.as_mut_ptr(), -1i32);
        } else {
            if code == ERR_NEED_CD as libc::c_int {
                VM_Forced_Unload_Start();
                SV_Shutdown(b"Server didn\'t have CD\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
                if 0 != restartClient as u64 { CL_Init(); }
                if !com_cl_running.is_null() && 0 != (*com_cl_running).integer
                   {
                    CL_Disconnect(qtrue);
                    CL_FlushMemory();
                    VM_Forced_Unload_Done();
                    CL_CDDialog();
                } else {
                    Com_Printf(b"Server didn\'t have CD\n\x00" as *const u8 as
                                   *const libc::c_char);
                    VM_Forced_Unload_Done();
                }
                FS_PureServerSetLoadedPaks(b"\x00" as *const u8 as
                                               *const libc::c_char,
                                           b"\x00" as *const u8 as
                                               *const libc::c_char);
                com_errorEntered = qfalse;
                longjmp(abortframe.as_mut_ptr(), -1i32);
            } else {
                VM_Forced_Unload_Start();
                CL_Shutdown(va(b"Client fatal crashed: %s\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               com_errorMessage.as_mut_ptr()), qtrue, qtrue);
                SV_Shutdown(va(b"Server fatal crashed: %s\x00" as *const u8 as
                                   *const libc::c_char as *mut libc::c_char,
                               com_errorMessage.as_mut_ptr()));
                VM_Forced_Unload_Done();
            }
        }
    }
    Com_Shutdown();
    Sys_Error(b"%s\x00" as *const u8 as *const libc::c_char,
              com_errorMessage.as_mut_ptr());
}
#[no_mangle]
pub static mut com_errorMessage: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
pub unsafe extern "C" fn Com_Shutdown() {
    if 0 != logfile { FS_FCloseFile(logfile); logfile = 0i32 }
    if 0 != com_journalFile {
        FS_FCloseFile(com_journalFile);
        com_journalFile = 0i32
    }
    if 0 != pipefile {
        FS_FCloseFile(pipefile);
        FS_HomeRemove((*com_pipefile).string);
    };
}
#[no_mangle]
pub static mut com_pipefile: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut pipefile: fileHandle_t = 0;
#[no_mangle]
pub static mut com_journalFile: fileHandle_t = 0;
static mut logfile: fileHandle_t = 0;
// an ERR_DROP occurred, exit the entire frame
#[no_mangle]
pub static mut abortframe: jmp_buf =
    [__jmp_buf_tag{__jmpbuf: [0; 8],
                   __mask_was_saved: 0,
                   __saved_mask: __sigset_t{__val: [0; 16],},}; 1];
#[no_mangle]
pub static mut com_errorEntered: qboolean = qfalse;
#[no_mangle]
pub unsafe extern "C" fn Com_Printf(mut fmt: *const libc::c_char, ...) {
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    static mut opening_qconsole: qboolean = qfalse;
    vsnprintf(msg.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
              fmt, argptr_0);
    if !rd_buffer.is_null() {
        if strlen(msg.as_mut_ptr()).wrapping_add(strlen(rd_buffer)) >
               (rd_buffersize - 1i32) as libc::c_ulong {
            rd_flush.expect("non-null function pointer")(rd_buffer);
            *rd_buffer = 0i32 as libc::c_char
        }
        Q_strcat(rd_buffer, rd_buffersize, msg.as_mut_ptr());
        return
    }
    Sys_Print(msg.as_mut_ptr());
    if !com_logfile.is_null() && 0 != (*com_logfile).integer {
        if 0 == logfile && 0 != FS_Initialized() as libc::c_uint &&
               0 == opening_qconsole as u64 {
            let mut newtime: *mut tm = 0 as *mut tm;
            let mut aclock: time_t = 0;
            opening_qconsole = qtrue;
            time(&mut aclock);
            newtime = localtime(&mut aclock);
            logfile =
                FS_FOpenFileWrite(b"qconsole.log\x00" as *const u8 as
                                      *const libc::c_char);
            if 0 != logfile {
                Com_Printf(b"logfile opened on %s\n\x00" as *const u8 as
                               *const libc::c_char, asctime(newtime));
                if (*com_logfile).integer > 1i32 { FS_ForceFlush(logfile); }
            } else {
                Com_Printf(b"Opening qconsole.log failed!\n\x00" as *const u8
                               as *const libc::c_char);
                Cvar_SetValue(b"logfile\x00" as *const u8 as
                                  *const libc::c_char, 0i32 as libc::c_float);
            }
            opening_qconsole = qfalse
        }
        if 0 != logfile && 0 != FS_Initialized() as libc::c_uint {
            FS_Write(msg.as_mut_ptr() as *const libc::c_void,
                     strlen(msg.as_mut_ptr()) as libc::c_int, logfile);
        }
    };
}
// 1 = buffer log, 2 = flush after each print
#[no_mangle]
pub static mut com_logfile: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut rd_buffersize: libc::c_int = 0;
//============================================================================
static mut rd_buffer: *mut libc::c_char =
    0 as *const libc::c_char as *mut libc::c_char;
static mut rd_flush: Option<unsafe extern "C" fn(_: *mut libc::c_char) -> ()>
       =
    None;
#[no_mangle]
pub static mut com_cl_running: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_gameClientRestarting: qboolean = qfalse;
#[no_mangle]
pub static mut com_gameRestarting: qboolean = qfalse;
// for building release pak files
#[no_mangle]
pub static mut com_buildScript: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
/*
=================
Hunk_SmallLog
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Hunk_SmallLog() {
    let mut block: *mut hunkblock_t = 0 as *mut hunkblock_t;
    let mut block2: *mut hunkblock_t = 0 as *mut hunkblock_t;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut size: libc::c_int = 0;
    let mut locsize: libc::c_int = 0;
    let mut numBlocks: libc::c_int = 0;
    if 0 == logfile || 0 == FS_Initialized() as u64 { return }
    block = hunkblocks;
    while !block.is_null() {
        (*block).printed = qfalse as libc::c_int as byte;
        block = (*block).next
    }
    size = 0i32;
    numBlocks = 0i32;
    Com_sprintf(buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"\r\n================\r\nHunk Small log\r\n================\r\n\x00"
                    as *const u8 as *const libc::c_char);
    FS_Write(buf.as_mut_ptr() as *const libc::c_void,
             strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
    block = hunkblocks;
    while !block.is_null() {
        if !(0 != (*block).printed) {
            locsize = (*block).size;
            block2 = (*block).next;
            while !block2.is_null() {
                if !((*block).line != (*block2).line) {
                    if !(0 != Q_stricmp((*block).file, (*block2).file)) {
                        size += (*block2).size;
                        locsize += (*block2).size;
                        (*block2).printed = qtrue as libc::c_int as byte
                    }
                }
                block2 = (*block2).next
            }
            Com_sprintf(buf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 4096]>() as
                            libc::c_ulong as libc::c_int,
                        b"size = %8d: %s, line: %d (%s)\r\n\x00" as *const u8
                            as *const libc::c_char, locsize, (*block).file,
                        (*block).line, (*block).label);
            FS_Write(buf.as_mut_ptr() as *const libc::c_void,
                     strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
            size += (*block).size;
            numBlocks += 1
        }
        block = (*block).next
    }
    Com_sprintf(buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%d Hunk memory\r\n\x00" as *const u8 as *const libc::c_char,
                size);
    FS_Write(buf.as_mut_ptr() as *const libc::c_void,
             strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
    Com_sprintf(buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%d hunk blocks\r\n\x00" as *const u8 as *const libc::c_char,
                numBlocks);
    FS_Write(buf.as_mut_ptr() as *const libc::c_void,
             strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_Log() {
    let mut block: *mut hunkblock_t = 0 as *mut hunkblock_t;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut size: libc::c_int = 0;
    let mut numBlocks: libc::c_int = 0;
    if 0 == logfile || 0 == FS_Initialized() as u64 { return }
    size = 0i32;
    numBlocks = 0i32;
    Com_sprintf(buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"\r\n================\r\nHunk log\r\n================\r\n\x00"
                    as *const u8 as *const libc::c_char);
    FS_Write(buf.as_mut_ptr() as *const libc::c_void,
             strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
    block = hunkblocks;
    while !block.is_null() {
        Com_sprintf(buf.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as
                        libc::c_ulong as libc::c_int,
                    b"size = %8d: %s, line: %d (%s)\r\n\x00" as *const u8 as
                        *const libc::c_char, (*block).size, (*block).file,
                    (*block).line, (*block).label);
        FS_Write(buf.as_mut_ptr() as *const libc::c_void,
                 strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
        size += (*block).size;
        numBlocks += 1;
        block = (*block).next
    }
    Com_sprintf(buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%d Hunk memory\r\n\x00" as *const u8 as *const libc::c_char,
                size);
    FS_Write(buf.as_mut_ptr() as *const libc::c_void,
             strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
    Com_sprintf(buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%d hunk blocks\r\n\x00" as *const u8 as *const libc::c_char,
                numBlocks);
    FS_Write(buf.as_mut_ptr() as *const libc::c_void,
             strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
}
static mut hunk_high: hunkUsed_t =
    hunkUsed_t{mark: 0, permanent: 0, temp: 0, tempHighwater: 0,};
unsafe extern "C" fn Hunk_SwapBanks() {
    let mut swap: *mut hunkUsed_t = 0 as *mut hunkUsed_t;
    if (*hunk_temp).temp != (*hunk_temp).permanent { return }
    if (*hunk_temp).tempHighwater - (*hunk_temp).permanent >
           (*hunk_permanent).tempHighwater - (*hunk_permanent).permanent {
        swap = hunk_temp;
        hunk_temp = hunk_permanent;
        hunk_permanent = swap
    };
}
static mut hunk_temp: *mut hunkUsed_t =
    0 as *const hunkUsed_t as *mut hunkUsed_t;
#[no_mangle]
pub static mut Q_VMftol: Option<unsafe extern "C" fn() -> libc::c_int> = None;
#[no_mangle]
pub unsafe extern "C" fn Com_RandomBytes(mut string: *mut byte,
                                         mut len: libc::c_int) {
    let mut i: libc::c_int = 0;
    if 0 != Sys_RandomBytes(string, len) as u64 { return }
    Com_Printf(b"Com_RandomBytes: using weak randomization\n\x00" as *const u8
                   as *const libc::c_char);
    i = 0i32;
    while i < len {
        *string.offset(i as isize) = (rand() % 256i32) as libc::c_uchar;
        i += 1
    };
}
/*
==============================================================

PROTOCOL

==============================================================
*/
// 1.31 - 67
// maintain a list of compatible protocols for demo playing
// NOTE: that stuff only works with two digits protocols
#[no_mangle]
pub static mut demo_protocols: [libc::c_int; 3] = [67i32, 66i32, 0i32];
#[no_mangle]
pub unsafe extern "C" fn Field_Clear(mut edit: *mut field_t) {
    memset((*edit).buffer.as_mut_ptr() as *mut libc::c_void, 0i32,
           256i32 as libc::c_ulong);
    (*edit).cursor = 0i32;
    (*edit).scroll = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn Field_AutoComplete(mut field: *mut field_t) {
    completionField = field;
    Field_CompleteCommand((*completionField).buffer.as_mut_ptr(), qtrue,
                          qtrue);
}
// field we are working on, passed to Field_AutoComplete(&g_consoleCommand for instance)
static mut completionField: *mut field_t =
    0 as *const field_t as *mut field_t;
#[no_mangle]
pub unsafe extern "C" fn Field_CompleteCommand(mut cmd: *mut libc::c_char,
                                               mut doCommands: qboolean,
                                               mut doCvars: qboolean) {
    let mut completionArgument: libc::c_int = 0i32;
    cmd =
        Com_SkipCharset(cmd,
                        b" \"\x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char);
    Cmd_TokenizeStringIgnoreQuotes(cmd);
    completionArgument = Cmd_Argc();
    if *cmd.offset(strlen(cmd) as isize).offset(-1isize) as libc::c_int ==
           ' ' as i32 {
        completionString = b"\x00" as *const u8 as *const libc::c_char;
        completionArgument += 1
    } else { completionString = Cmd_Argv(completionArgument - 1i32) }
    if completionArgument > 1i32 {
        let mut baseCmd: *const libc::c_char = Cmd_Argv(0i32);
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        p = Field_FindFirstSeparator(cmd);
        if !p.is_null() {
            Field_CompleteCommand(p.offset(1isize), qtrue, qtrue);
        } else { Cmd_CompleteArgument(baseCmd, cmd, completionArgument); }
    } else {
        if *completionString.offset(0isize) as libc::c_int == '\\' as i32 ||
               *completionString.offset(0isize) as libc::c_int == '/' as i32 {
            completionString = completionString.offset(1isize)
        }
        matchCount = 0i32;
        shortestMatch[0usize] = 0i32 as libc::c_char;
        if strlen(completionString) == 0i32 as libc::c_ulong { return }
        if 0 != doCommands as u64 {
            Cmd_CommandCompletion(Some(FindMatches));
        }
        if 0 != doCvars as u64 { Cvar_CommandCompletion(Some(FindMatches)); }
        if 0 == Field_Complete() as u64 {
            if 0 != doCommands as u64 {
                Cmd_CommandCompletion(Some(PrintMatches));
            }
            if 0 != doCvars as u64 {
                Cvar_CommandCompletion(Some(PrintCvarMatches));
            }
        }
    };
}
/*
===============
PrintCvarMatches

===============
*/
unsafe extern "C" fn PrintCvarMatches(mut s: *const libc::c_char) {
    let mut value: [libc::c_char; 64] = [0; 64];
    if 0 ==
           Q_stricmpn(s, shortestMatch.as_mut_ptr(),
                      strlen(shortestMatch.as_mut_ptr()) as libc::c_int) {
        Com_TruncateLongString(value.as_mut_ptr(), Cvar_VariableString(s));
        Com_Printf(b"    %s = \"%s\"\n\x00" as *const u8 as
                       *const libc::c_char, s, value.as_mut_ptr());
    };
}
static mut shortestMatch: [libc::c_char; 1024] = [0; 1024];
/*
===============
PrintMatches

===============
*/
unsafe extern "C" fn PrintMatches(mut s: *const libc::c_char) {
    if 0 ==
           Q_stricmpn(s, shortestMatch.as_mut_ptr(),
                      strlen(shortestMatch.as_mut_ptr()) as libc::c_int) {
        Com_Printf(b"    %s\n\x00" as *const u8 as *const libc::c_char, s);
    };
}
/*
===============
Field_Complete
===============
*/
unsafe extern "C" fn Field_Complete() -> qboolean {
    let mut completionOffset: libc::c_int = 0;
    if matchCount == 0i32 { return qtrue }
    completionOffset =
        strlen((*completionField).buffer.as_mut_ptr()).wrapping_sub(strlen(completionString))
            as libc::c_int;
    Q_strncpyz(&mut (*completionField).buffer[completionOffset as usize],
               shortestMatch.as_mut_ptr(),
               (::std::mem::size_of::<[libc::c_char; 256]>() as
                    libc::c_ulong).wrapping_sub(completionOffset as
                                                    libc::c_ulong) as
                   libc::c_int);
    (*completionField).cursor =
        strlen((*completionField).buffer.as_mut_ptr()) as libc::c_int;
    if matchCount == 1i32 {
        Q_strcat((*completionField).buffer.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                     as libc::c_int,
                 b" \x00" as *const u8 as *const libc::c_char);
        (*completionField).cursor += 1;
        return qtrue
    }
    Com_Printf(b"]%s\n\x00" as *const u8 as *const libc::c_char,
               (*completionField).buffer.as_mut_ptr());
    return qfalse;
}
static mut matchCount: libc::c_int = 0;
static mut completionString: *const libc::c_char = 0 as *const libc::c_char;
/*
===============
FindMatches

===============
*/
unsafe extern "C" fn FindMatches(mut s: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    if 0 !=
           Q_stricmpn(s, completionString,
                      strlen(completionString) as libc::c_int) {
        return
    }
    matchCount += 1;
    if matchCount == 1i32 {
        Q_strncpyz(shortestMatch.as_mut_ptr(), s,
                   ::std::mem::size_of::<[libc::c_char; 1024]>() as
                       libc::c_ulong as libc::c_int);
        return
    }
    i = 0i32;
    while 0 != shortestMatch[i as usize] {
        if i as libc::c_ulong >= strlen(s) {
            shortestMatch[i as usize] = 0i32 as libc::c_char;
            break ;
        } else {
            if tolower(shortestMatch[i as usize] as libc::c_int) !=
                   tolower(*s.offset(i as isize) as libc::c_int) {
                shortestMatch[i as usize] = 0i32 as libc::c_char
            }
            i += 1
        }
    };
}
/*
===============
Field_FindFirstSeparator
===============
*/
unsafe extern "C" fn Field_FindFirstSeparator(mut s: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while (i as libc::c_ulong) < strlen(s) {
        if *s.offset(i as isize) as libc::c_int == ';' as i32 {
            return &mut *s.offset(i as isize) as *mut libc::c_char
        }
        i += 1
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Field_CompleteFilename(mut dir: *const libc::c_char,
                                                mut ext: *const libc::c_char,
                                                mut stripExt: qboolean,
                                                mut allowNonPureFilesOnDisk:
                                                    qboolean) {
    matchCount = 0i32;
    shortestMatch[0usize] = 0i32 as libc::c_char;
    FS_FilenameCompletion(dir, ext, stripExt, Some(FindMatches),
                          allowNonPureFilesOnDisk);
    if 0 == Field_Complete() as u64 {
        FS_FilenameCompletion(dir, ext, stripExt, Some(PrintMatches),
                              allowNonPureFilesOnDisk);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Field_CompletePlayerName(mut names:
                                                      *mut *const libc::c_char,
                                                  mut nameCount:
                                                      libc::c_int) {
    let mut whitespace: qboolean = qfalse;
    matchCount = 0i32;
    shortestMatch[0usize] = 0i32 as libc::c_char;
    if nameCount <= 0i32 { return }
    Name_PlayerNameCompletion(names, nameCount, Some(FindMatches));
    if *completionString.offset(0isize) as libc::c_int == '\u{0}' as i32 {
        Com_PlayerNameToFieldString(shortestMatch.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 1024]>()
                                        as libc::c_ulong as libc::c_int,
                                    *names.offset(0isize));
    }
    if *completionString.offset(0isize) as libc::c_int != '\u{0}' as i32 &&
           Q_stricmp(shortestMatch.as_mut_ptr(), completionString) == 0i32 &&
           nameCount > 1i32 {
        let mut i: libc::c_int = 0;
        i = 0i32;
        while i < nameCount {
            if Q_stricmp(*names.offset(i as isize), completionString) == 0i32
               {
                i += 1;
                if i >= nameCount { i = 0i32 }
                Com_PlayerNameToFieldString(shortestMatch.as_mut_ptr(),
                                            ::std::mem::size_of::<[libc::c_char; 1024]>()
                                                as libc::c_ulong as
                                                libc::c_int,
                                            *names.offset(i as isize));
                break ;
            } else { i += 1 }
        }
    }
    if matchCount > 1i32 {
        Com_Printf(b"]%s\n\x00" as *const u8 as *const libc::c_char,
                   (*completionField).buffer.as_mut_ptr());
        Name_PlayerNameCompletion(names, nameCount, Some(PrintMatches));
    }
    whitespace =
        (if nameCount == 1i32 {
             qtrue as libc::c_int
         } else { qfalse as libc::c_int }) as qboolean;
    0 == Field_CompletePlayerNameFinal(whitespace) as u64;
}
/*
===============
Field_CompletePlayerName
===============
*/
unsafe extern "C" fn Field_CompletePlayerNameFinal(mut whitespace: qboolean)
 -> qboolean {
    let mut completionOffset: libc::c_int = 0;
    if matchCount == 0i32 { return qtrue }
    completionOffset =
        strlen((*completionField).buffer.as_mut_ptr()).wrapping_sub(strlen(completionString))
            as libc::c_int;
    Q_strncpyz(&mut (*completionField).buffer[completionOffset as usize],
               shortestMatch.as_mut_ptr(),
               (::std::mem::size_of::<[libc::c_char; 256]>() as
                    libc::c_ulong).wrapping_sub(completionOffset as
                                                    libc::c_ulong) as
                   libc::c_int);
    (*completionField).cursor =
        strlen((*completionField).buffer.as_mut_ptr()) as libc::c_int;
    if matchCount == 1i32 && 0 != whitespace as libc::c_uint {
        Q_strcat((*completionField).buffer.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                     as libc::c_int,
                 b" \x00" as *const u8 as *const libc::c_char);
        (*completionField).cursor += 1;
        return qtrue
    }
    return qfalse;
}
unsafe extern "C" fn Name_PlayerNameCompletion(mut names:
                                                   *mut *const libc::c_char,
                                               mut nameCount: libc::c_int,
                                               mut callback:
                                                   Option<unsafe extern "C" fn(_:
                                                                                   *const libc::c_char)
                                                              -> ()>) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < nameCount {
        callback.expect("non-null function pointer")(*names.offset(i as
                                                                       isize));
        i += 1
    };
}
// checks for and removes command line "+set var arg" constructs
// if match is NULL, all set commands will be executed, otherwise
// only a set with the exact name.  Only used during startup.
#[no_mangle]
pub unsafe extern "C" fn Com_PlayerNameToFieldString(mut str:
                                                         *mut libc::c_char,
                                                     mut length: libc::c_int,
                                                     mut name:
                                                         *const libc::c_char)
 -> qboolean {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut x1: libc::c_int = 0;
    let mut x2: libc::c_int = 0;
    if str.is_null() || name.is_null() { return qfalse }
    if length <= 0i32 { return qtrue }
    *str = '\u{0}' as i32 as libc::c_char;
    p = name;
    i = 0i32;
    while *p as libc::c_int != '\u{0}' as i32 {
        if i + 1i32 >= length { break ; }
        if *p as libc::c_int <= ' ' as i32 {
            if i + 5i32 + 1i32 >= length { break ; }
            x1 = *p as libc::c_int >> 4i32;
            x2 = *p as libc::c_int & 15i32;
            *str.offset((i + 0i32) as isize) = '\\' as i32 as libc::c_char;
            *str.offset((i + 1i32) as isize) = '0' as i32 as libc::c_char;
            *str.offset((i + 2i32) as isize) = 'x' as i32 as libc::c_char;
            *str.offset((i + 3i32) as isize) =
                (if x1 > 9i32 {
                     x1 - 10i32 + 'a' as i32
                 } else { x1 + '0' as i32 }) as libc::c_char;
            *str.offset((i + 4i32) as isize) =
                (if x2 > 9i32 {
                     x2 - 10i32 + 'a' as i32
                 } else { x2 + '0' as i32 }) as libc::c_char;
            i += 4i32
        } else { *str.offset(i as isize) = *p }
        i += 1;
        p = p.offset(1isize)
    }
    *str.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    return qtrue;
}
/*
==============================================================

MISC

==============================================================
*/
// centralizing the declarations for cl_cdkey
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=470
#[no_mangle]
pub static mut cl_cdkey: [libc::c_char; 34] =
    [49, 50, 51, 52, 53, 54, 55, 56, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
#[no_mangle]
pub unsafe extern "C" fn Com_QueueEvent(mut time_0: libc::c_int,
                                        mut type_0: sysEventType_t,
                                        mut value: libc::c_int,
                                        mut value2: libc::c_int,
                                        mut ptrLength: libc::c_int,
                                        mut ptr: *mut libc::c_void) {
    let mut ev: *mut sysEvent_t = 0 as *mut sysEvent_t;
    if type_0 as libc::c_uint == SE_MOUSE as libc::c_int as libc::c_uint &&
           eventHead != eventTail {
        ev =
            &mut eventQueue[(eventHead + 256i32 - 1i32 & 256i32 - 1i32) as
                                usize] as *mut sysEvent_t;
        if (*ev).evType as libc::c_uint ==
               SE_MOUSE as libc::c_int as libc::c_uint {
            (*ev).evValue += value;
            (*ev).evValue2 += value2;
            return
        }
    }
    ev =
        &mut eventQueue[(eventHead & 256i32 - 1i32) as usize] as
            *mut sysEvent_t;
    if eventHead - eventTail >= 256i32 {
        Com_Printf(b"Com_QueueEvent: overflow\n\x00" as *const u8 as
                       *const libc::c_char);
        if !(*ev).evPtr.is_null() { Z_Free((*ev).evPtr); }
        eventTail += 1
    }
    eventHead += 1;
    if time_0 == 0i32 { time_0 = Sys_Milliseconds() }
    (*ev).evTime = time_0;
    (*ev).evType = type_0;
    (*ev).evValue = value;
    (*ev).evValue2 = value2;
    (*ev).evPtrLength = ptrLength;
    (*ev).evPtr = ptr;
}
static mut eventHead: libc::c_int = 0i32;
static mut eventTail: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn Z_Free(mut ptr: *mut libc::c_void) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    let mut other: *mut memblock_t = 0 as *mut memblock_t;
    let mut zone: *mut memzone_t = 0 as *mut memzone_t;
    if ptr.is_null() {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Z_Free: NULL pointer\x00" as *const u8 as
                      *const libc::c_char);
    }
    block =
        (ptr as
             *mut byte).offset(-(::std::mem::size_of::<memblock_t>() as
                                     libc::c_ulong as isize)) as
            *mut memblock_t;
    if (*block).id != 0x1d4a11i32 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Z_Free: freed a pointer without ZONEID\x00" as *const u8
                      as *const libc::c_char);
    }
    if (*block).tag == 0i32 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Z_Free: freed a freed pointer\x00" as *const u8 as
                      *const libc::c_char);
    }
    if (*block).tag == TAG_STATIC as libc::c_int { return }
    if *((block as *mut byte).offset((*block).size as isize).offset(-4isize)
             as *mut libc::c_int) != 0x1d4a11i32 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Z_Free: memory block wrote past end\x00" as *const u8 as
                      *const libc::c_char);
    }
    if (*block).tag == TAG_SMALL as libc::c_int {
        zone = smallzone
    } else { zone = mainzone }
    (*zone).used -= (*block).size;
    memset(ptr, 0xaai32,
           ((*block).size as
                libc::c_ulong).wrapping_sub(::std::mem::size_of::<memblock_t>()
                                                as libc::c_ulong));
    (*block).tag = 0i32;
    other = (*block).prev;
    if 0 == (*other).tag {
        (*other).size += (*block).size;
        (*other).next = (*block).next;
        (*(*other).next).prev = other;
        if block == (*zone).rover { (*zone).rover = other }
        block = other
    }
    (*zone).rover = block;
    other = (*block).next;
    if 0 == (*other).tag {
        (*block).size += (*other).size;
        (*block).next = (*other).next;
        (*(*block).next).prev = block
    };
}
// main zone for all "dynamic" memory allocation
static mut mainzone: *mut memzone_t = 0 as *const memzone_t as *mut memzone_t;
// we also have a small zone for small allocations that would only
// fragment the main zone (think of cvar and cmd strings)
static mut smallzone: *mut memzone_t =
    0 as *const memzone_t as *mut memzone_t;
/*
========================================================================

EVENT LOOP

========================================================================
*/
static mut eventQueue: [sysEvent_t; 256] =
    [sysEvent_t{evTime: 0,
                evType: SE_NONE,
                evValue: 0,
                evValue2: 0,
                evPtrLength: 0,
                evPtr: 0 as *const libc::c_void as *mut libc::c_void,}; 256];
#[no_mangle]
pub unsafe extern "C" fn Com_EventLoop() -> libc::c_int {
    let mut ev: sysEvent_t =
        sysEvent_t{evTime: 0,
                   evType: SE_NONE,
                   evValue: 0,
                   evValue2: 0,
                   evPtrLength: 0,
                   evPtr: 0 as *const libc::c_void as *mut libc::c_void,};
    let mut evFrom: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    let mut bufData: [byte; 16384] = [0; 16384];
    let mut buf: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    MSG_Init(&mut buf, bufData.as_mut_ptr(),
             ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                 libc::c_int);
    loop  {
        ev = Com_GetEvent();
        if ev.evType as libc::c_uint == SE_NONE as libc::c_int as libc::c_uint
           {
            while 0 !=
                      NET_GetLoopPacket(NS_CLIENT, &mut evFrom, &mut buf) as
                          u64 {
                CL_PacketEvent(evFrom, &mut buf);
            }
            while 0 !=
                      NET_GetLoopPacket(NS_SERVER, &mut evFrom, &mut buf) as
                          u64 {
                if 0 != (*com_sv_running).integer {
                    Com_RunAndTimeServerPacket(&mut evFrom, &mut buf);
                }
            }
            return ev.evTime
        }
        match ev.evType as libc::c_uint {
            1 => {
                CL_KeyEvent(ev.evValue, ev.evValue2 as qboolean,
                            ev.evTime as libc::c_uint);
            }
            2 => { CL_CharEvent(ev.evValue); }
            3 => { CL_MouseEvent(ev.evValue, ev.evValue2, ev.evTime); }
            4 => { CL_JoystickEvent(ev.evValue, ev.evValue2, ev.evTime); }
            5 => {
                Cbuf_AddText(ev.evPtr as *mut libc::c_char);
                Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
            }
            _ => {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"Com_EventLoop: bad event type %i\x00" as *const u8
                              as *const libc::c_char,
                          ev.evType as libc::c_uint);
            }
        }
        if !ev.evPtr.is_null() { Z_Free(ev.evPtr); }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Com_RunAndTimeServerPacket(mut evFrom: *mut netadr_t,
                                                    mut buf: *mut msg_t) {
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    let mut msec: libc::c_int = 0;
    t1 = 0i32;
    if 0 != (*com_speeds).integer { t1 = Sys_Milliseconds() }
    SV_PacketEvent(*evFrom, buf);
    if 0 != (*com_speeds).integer {
        t2 = Sys_Milliseconds();
        msec = t2 - t1;
        if (*com_speeds).integer == 3i32 {
            Com_Printf(b"SV_PacketEvent time: %i\n\x00" as *const u8 as
                           *const libc::c_char, msec);
        }
    };
}
#[no_mangle]
pub static mut com_speeds: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_sv_running: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
/*
=================
Com_GetEvent
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_GetEvent() -> sysEvent_t {
    if com_pushedEventsHead > com_pushedEventsTail {
        com_pushedEventsTail += 1;
        return com_pushedEvents[(com_pushedEventsTail - 1i32 & 1024i32 - 1i32)
                                    as usize]
    }
    return Com_GetRealEvent();
}
/*
=================
Com_GetRealEvent
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_GetRealEvent() -> sysEvent_t {
    let mut r: libc::c_int = 0;
    let mut ev: sysEvent_t =
        sysEvent_t{evTime: 0,
                   evType: SE_NONE,
                   evValue: 0,
                   evValue2: 0,
                   evPtrLength: 0,
                   evPtr: 0 as *const libc::c_void as *mut libc::c_void,};
    if (*com_journal).integer == 2i32 {
        r =
            FS_Read(&mut ev as *mut sysEvent_t as *mut libc::c_void,
                    ::std::mem::size_of::<sysEvent_t>() as libc::c_ulong as
                        libc::c_int, com_journalFile);
        if r as libc::c_ulong !=
               ::std::mem::size_of::<sysEvent_t>() as libc::c_ulong {
            Com_Error(ERR_FATAL as libc::c_int,
                      b"Error reading from journal file\x00" as *const u8 as
                          *const libc::c_char);
        }
        if 0 != ev.evPtrLength {
            ev.evPtr =
                Z_MallocDebug(ev.evPtrLength,
                              b"ev.evPtrLength\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              b"code/qcommon/common.c\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              2066i32);
            r = FS_Read(ev.evPtr, ev.evPtrLength, com_journalFile);
            if r != ev.evPtrLength {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"Error reading from journal file\x00" as *const u8
                              as *const libc::c_char);
            }
        }
    } else {
        ev = Com_GetSystemEvent();
        if (*com_journal).integer == 1i32 {
            r =
                FS_Write(&mut ev as *mut sysEvent_t as *const libc::c_void,
                         ::std::mem::size_of::<sysEvent_t>() as libc::c_ulong
                             as libc::c_int, com_journalFile);
            if r as libc::c_ulong !=
                   ::std::mem::size_of::<sysEvent_t>() as libc::c_ulong {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"Error writing to journal file\x00" as *const u8 as
                              *const libc::c_char);
            }
            if 0 != ev.evPtrLength {
                r = FS_Write(ev.evPtr, ev.evPtrLength, com_journalFile);
                if r != ev.evPtrLength {
                    Com_Error(ERR_FATAL as libc::c_int,
                              b"Error writing to journal file\x00" as
                                  *const u8 as *const libc::c_char);
                }
            }
        }
    }
    return ev;
}
#[no_mangle]
pub static mut com_journal: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn Com_GetSystemEvent() -> sysEvent_t {
    let mut ev: sysEvent_t =
        sysEvent_t{evTime: 0,
                   evType: SE_NONE,
                   evValue: 0,
                   evValue2: 0,
                   evPtrLength: 0,
                   evPtr: 0 as *const libc::c_void as *mut libc::c_void,};
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if eventHead > eventTail {
        eventTail += 1;
        return eventQueue[(eventTail - 1i32 & 256i32 - 1i32) as usize]
    }
    s = Sys_ConsoleInput();
    if !s.is_null() {
        let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_int = 0;
        len = strlen(s).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
        b =
            Z_MallocDebug(len,
                          b"len\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char,
                          b"code/qcommon/common.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          2031i32) as *mut libc::c_char;
        strcpy(b, s);
        Com_QueueEvent(0i32, SE_CONSOLE, 0i32, 0i32, len,
                       b as *mut libc::c_void);
    }
    if eventHead > eventTail {
        eventTail += 1;
        return eventQueue[(eventTail - 1i32 & 256i32 - 1i32) as usize]
    }
    memset(&mut ev as *mut sysEvent_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<sysEvent_t>() as libc::c_ulong);
    ev.evTime = Sys_Milliseconds();
    return ev;
}
// returns 0 filled memory
#[no_mangle]
pub unsafe extern "C" fn Z_MallocDebug(mut size: libc::c_int,
                                       mut label: *mut libc::c_char,
                                       mut file: *mut libc::c_char,
                                       mut line: libc::c_int)
 -> *mut libc::c_void {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    buf =
        Z_TagMallocDebug(size, TAG_GENERAL as libc::c_int, label, file, line);
    memset(buf, 0i32, size as libc::c_ulong);
    return buf;
}
/*

--- low memory ----
server vm
server clipmap
---mark---
renderer initialization (shaders, etc)
UI vm
cgame vm
renderer map
renderer models

---free---

temp file loading
--- high memory ---

*/
// NOT 0 filled memory
#[no_mangle]
pub unsafe extern "C" fn Z_TagMallocDebug(mut size: libc::c_int,
                                          mut tag: libc::c_int,
                                          mut label: *mut libc::c_char,
                                          mut file: *mut libc::c_char,
                                          mut line: libc::c_int)
 -> *mut libc::c_void {
    let mut allocSize: libc::c_int = 0;
    let mut extra: libc::c_int = 0;
    let mut start: *mut memblock_t = 0 as *mut memblock_t;
    let mut rover: *mut memblock_t = 0 as *mut memblock_t;
    let mut new: *mut memblock_t = 0 as *mut memblock_t;
    let mut base: *mut memblock_t = 0 as *mut memblock_t;
    let mut zone: *mut memzone_t = 0 as *mut memzone_t;
    if 0 == tag {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Z_TagMalloc: tried to use a 0 tag\x00" as *const u8 as
                      *const libc::c_char);
    }
    if tag == TAG_SMALL as libc::c_int {
        zone = smallzone
    } else { zone = mainzone }
    allocSize = size;
    size =
        (size as
             libc::c_ulong).wrapping_add(::std::mem::size_of::<memblock_t>()
                                             as libc::c_ulong) as libc::c_int
            as libc::c_int;
    size += 4i32;
    size =
        ((size as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<intptr_t>() as
                                              libc::c_ulong).wrapping_sub(1i32
                                                                              as
                                                                              libc::c_ulong)
             &
             !(::std::mem::size_of::<intptr_t>() as
                   libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong)) as
            libc::c_int;
    rover = (*zone).rover;
    base = rover;
    start = (*base).prev;
    loop  {
        if rover == start {
            Z_LogHeap();
            Com_Error(ERR_FATAL as libc::c_int,
                      b"Z_Malloc: failed on allocation of %i bytes from the %s zone: %s, line: %d (%s)\x00"
                          as *const u8 as *const libc::c_char, size,
                      if zone == smallzone {
                          b"small\x00" as *const u8 as *const libc::c_char
                      } else {
                          b"main\x00" as *const u8 as *const libc::c_char
                      }, file, line, label);
        }
        if 0 != (*rover).tag {
            rover = (*rover).next;
            base = rover
        } else { rover = (*rover).next }
        if !(0 != (*base).tag || (*base).size < size) { break ; }
    }
    extra = (*base).size - size;
    if extra > 64i32 {
        new = (base as *mut byte).offset(size as isize) as *mut memblock_t;
        (*new).size = extra;
        (*new).tag = 0i32;
        (*new).prev = base;
        (*new).id = 0x1d4a11i32;
        (*new).next = (*base).next;
        (*(*new).next).prev = new;
        (*base).next = new;
        (*base).size = size
    }
    (*base).tag = tag;
    (*zone).rover = (*base).next;
    (*zone).used += (*base).size;
    (*base).id = 0x1d4a11i32;
    (*base).d.label = label;
    (*base).d.file = file;
    (*base).d.line = line;
    (*base).d.allocSize = allocSize;
    *((base as *mut byte).offset((*base).size as isize).offset(-4isize) as
          *mut libc::c_int) = 0x1d4a11i32;
    return (base as
                *mut byte).offset(::std::mem::size_of::<memblock_t>() as
                                      libc::c_ulong as isize) as
               *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn Z_LogHeap() {
    Z_LogZoneHeap(mainzone,
                  b"MAIN\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
    Z_LogZoneHeap(smallzone,
                  b"SMALL\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
}
/*
========================
Z_LogZoneHeap
========================
*/
#[no_mangle]
pub unsafe extern "C" fn Z_LogZoneHeap(mut zone: *mut memzone_t,
                                       mut name: *mut libc::c_char) {
    let mut dump: [libc::c_char; 32] = [0; 32];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut size: libc::c_int = 0;
    let mut allocSize: libc::c_int = 0;
    let mut numBlocks: libc::c_int = 0;
    if 0 == logfile || 0 == FS_Initialized() as u64 { return }
    numBlocks = 0i32;
    size = numBlocks;
    allocSize = 0i32;
    Com_sprintf(buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"\r\n================\r\n%s log\r\n================\r\n\x00"
                    as *const u8 as *const libc::c_char, name);
    FS_Write(buf.as_mut_ptr() as *const libc::c_void,
             strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
    block = (*zone).blocklist.next;
    while (*block).next != &mut (*zone).blocklist as *mut memblock_t {
        if 0 != (*block).tag {
            ptr =
                (block as
                     *mut libc::c_char).offset(::std::mem::size_of::<memblock_t>()
                                                   as libc::c_ulong as isize);
            j = 0i32;
            i = 0i32;
            while i < 20i32 && i < (*block).d.allocSize {
                if *ptr.offset(i as isize) as libc::c_int >= 32i32 &&
                       (*ptr.offset(i as isize) as libc::c_int) < 127i32 {
                    let fresh0 = j;
                    j = j + 1;
                    dump[fresh0 as usize] = *ptr.offset(i as isize)
                } else {
                    let fresh1 = j;
                    j = j + 1;
                    dump[fresh1 as usize] = '_' as i32 as libc::c_char
                }
                i += 1
            }
            dump[j as usize] = '\u{0}' as i32 as libc::c_char;
            Com_sprintf(buf.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 4096]>() as
                            libc::c_ulong as libc::c_int,
                        b"size = %8d: %s, line: %d (%s) [%s]\r\n\x00" as
                            *const u8 as *const libc::c_char,
                        (*block).d.allocSize, (*block).d.file,
                        (*block).d.line, (*block).d.label, dump.as_mut_ptr());
            FS_Write(buf.as_mut_ptr() as *const libc::c_void,
                     strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
            allocSize += (*block).d.allocSize;
            size += (*block).size;
            numBlocks += 1
        }
        block = (*block).next
    }
    size =
        (size as
             libc::c_ulong).wrapping_sub((numBlocks as
                                              libc::c_ulong).wrapping_mul(::std::mem::size_of::<zonedebug_t>()
                                                                              as
                                                                              libc::c_ulong))
            as libc::c_int as libc::c_int;
    Com_sprintf(buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%d %s memory in %d blocks\r\n\x00" as *const u8 as
                    *const libc::c_char, size, name, numBlocks);
    FS_Write(buf.as_mut_ptr() as *const libc::c_void,
             strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
    Com_sprintf(buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%d %s memory overhead\r\n\x00" as *const u8 as
                    *const libc::c_char, size - allocSize, name);
    FS_Write(buf.as_mut_ptr() as *const libc::c_void,
             strlen(buf.as_mut_ptr()) as libc::c_int, logfile);
}
static mut com_pushedEventsTail: libc::c_int = 0i32;
static mut com_pushedEvents: [sysEvent_t; 1024] =
    [sysEvent_t{evTime: 0,
                evType: SE_NONE,
                evValue: 0,
                evValue2: 0,
                evPtrLength: 0,
                evPtr: 0 as *const libc::c_void as *mut libc::c_void,}; 1024];
/*
===================================================================

EVENTS AND JOURNALING

In addition to these events, .cfg files are also copied to the
journaled file
===================================================================
*/
static mut com_pushedEventsHead: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn CopyString(mut in_0: *const libc::c_char)
 -> *mut libc::c_char {
    let mut out: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == *in_0.offset(0isize) {
        return (&mut emptystring as *mut memstatic_t as
                    *mut libc::c_char).offset(::std::mem::size_of::<memblock_t>()
                                                  as libc::c_ulong as isize)
    } else {
        if 0 == *in_0.offset(1isize) {
            if *in_0.offset(0isize) as libc::c_int >= '0' as i32 &&
                   *in_0.offset(0isize) as libc::c_int <= '9' as i32 {
                return (&mut numberstring[(*in_0.offset(0isize) as libc::c_int
                                               - '0' as i32) as usize] as
                            *mut memstatic_t as
                            *mut libc::c_char).offset(::std::mem::size_of::<memblock_t>()
                                                          as libc::c_ulong as
                                                          isize)
            }
        }
    }
    out =
        S_MallocDebug(strlen(in_0).wrapping_add(1i32 as libc::c_ulong) as
                          libc::c_int,
                      b"strlen(in)+1\x00" as *const u8 as *const libc::c_char
                          as *mut libc::c_char,
                      b"code/qcommon/common.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 1203i32)
            as *mut libc::c_char;
    strcpy(out, in_0);
    return out;
}
// returns 0 filled memory
#[no_mangle]
pub unsafe extern "C" fn S_MallocDebug(mut size: libc::c_int,
                                       mut label: *mut libc::c_char,
                                       mut file: *mut libc::c_char,
                                       mut line: libc::c_int)
 -> *mut libc::c_void {
    return Z_TagMallocDebug(size, TAG_SMALL as libc::c_int, label, file,
                            line);
}
// Initialized in run_static_initializers
#[no_mangle]
pub static mut numberstring: [memstatic_t; 10] =
    [memstatic_s{b:
                     memblock_s{size: 0,
                                tag: 0,
                                next:
                                    0 as *const memblock_s as *mut memblock_s,
                                prev:
                                    0 as *const memblock_s as *mut memblock_s,
                                id: 0,
                                d:
                                    zonedebug_s{label: 0 as *mut libc::c_char,
                                                file: 0 as *mut libc::c_char,
                                                line: 0,
                                                allocSize: 0,},},
                 mem: [0; 2],}; 10];
// Initialized in run_static_initializers
#[no_mangle]
pub static mut emptystring: memstatic_t =
    memstatic_s{b:
                    memblock_s{size: 0,
                               tag: 0,
                               next:
                                   0 as *const memblock_s as *mut memblock_s,
                               prev:
                                   0 as *const memblock_s as *mut memblock_s,
                               id: 0,
                               d:
                                   zonedebug_s{label: 0 as *mut libc::c_char,
                                               file: 0 as *mut libc::c_char,
                                               line: 0,
                                               allocSize: 0,},},
                mem: [0; 2],};
#[no_mangle]
pub unsafe extern "C" fn Info_Print(mut s: *const libc::c_char) {
    let mut key: [libc::c_char; 8192] = [0; 8192];
    let mut value: [libc::c_char; 8192] = [0; 8192];
    let mut o: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut l: libc::c_int = 0;
    if *s as libc::c_int == '\\' as i32 { s = s.offset(1isize) }
    while 0 != *s {
        o = key.as_mut_ptr();
        while 0 != *s as libc::c_int && *s as libc::c_int != '\\' as i32 {
            let fresh3 = o;
            o = o.offset(1);
            let fresh2 = s;
            s = s.offset(1);
            *fresh3 = *fresh2
        }
        l =
            o.wrapping_offset_from(key.as_mut_ptr()) as libc::c_long as
                libc::c_int;
        if l < 20i32 {
            memset(o as *mut libc::c_void, ' ' as i32,
                   (20i32 - l) as libc::c_ulong);
            key[20usize] = 0i32 as libc::c_char
        } else { *o = 0i32 as libc::c_char }
        Com_Printf(b"%s \x00" as *const u8 as *const libc::c_char,
                   key.as_mut_ptr());
        if 0 == *s {
            Com_Printf(b"MISSING VALUE\n\x00" as *const u8 as
                           *const libc::c_char);
            return
        }
        o = value.as_mut_ptr();
        s = s.offset(1isize);
        while 0 != *s as libc::c_int && *s as libc::c_int != '\\' as i32 {
            let fresh5 = o;
            o = o.offset(1);
            let fresh4 = s;
            s = s.offset(1);
            *fresh5 = *fresh4
        }
        *o = 0i32 as libc::c_char;
        if 0 != *s { s = s.offset(1isize) }
        Com_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                   value.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn Com_BeginRedirect(mut buffer: *mut libc::c_char,
                                           mut buffersize: libc::c_int,
                                           mut flush:
                                               Option<unsafe extern "C" fn(_:
                                                                               *mut libc::c_char)
                                                          -> ()>) {
    if buffer.is_null() || 0 == buffersize || flush.is_none() { return }
    rd_buffer = buffer;
    rd_buffersize = buffersize;
    rd_flush = flush;
    *rd_buffer = 0i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Com_EndRedirect() {
    if rd_flush.is_some() {
        rd_flush.expect("non-null function pointer")(rd_buffer);
    }
    rd_buffer = 0 as *mut libc::c_char;
    rd_buffersize = 0i32;
    rd_flush = None;
}
#[no_mangle]
pub unsafe extern "C" fn Com_DPrintf(mut fmt: *const libc::c_char, ...) {
    let mut msg: [libc::c_char; 4096] = [0; 4096];
    if com_developer.is_null() || 0 == (*com_developer).integer { return }
    vsnprintf(msg.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
              fmt, argptr_1);
    Com_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
               msg.as_mut_ptr());
}
#[no_mangle]
pub static mut com_developer: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn Com_Quit_f() -> ! {
    // don't try to shutdown if we are in a recursive error
    let mut p: *mut libc::c_char = Cmd_Args();
    if 0 == com_errorEntered as u64 {
        VM_Forced_Unload_Start();
        SV_Shutdown((if 0 != *p.offset(0isize) as libc::c_int {
                         p
                     } else {
                         b"Server quit\x00" as *const u8 as
                             *const libc::c_char
                     }) as *mut libc::c_char);
        CL_Shutdown((if 0 != *p.offset(0isize) as libc::c_int {
                         p
                     } else {
                         b"Client quit\x00" as *const u8 as
                             *const libc::c_char
                     }) as *mut libc::c_char, qtrue, qtrue);
        VM_Forced_Unload_Done();
        Com_Shutdown();
        FS_Shutdown(qtrue);
    }
    Sys_Quit();
}
#[no_mangle]
pub unsafe extern "C" fn Com_GameRestart(mut checksumFeed: libc::c_int,
                                         mut disconnect: qboolean) {
    if 0 == com_gameRestarting as u64 &&
           0 != com_fullyInitialized as libc::c_uint {
        com_gameRestarting = qtrue;
        com_gameClientRestarting = (*com_cl_running).integer as qboolean;
        if 0 != (*com_sv_running).integer {
            SV_Shutdown(b"Game directory changed\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
        }
        if 0 != com_gameClientRestarting as u64 {
            if 0 != disconnect as u64 { CL_Disconnect(qfalse); }
            CL_Shutdown(b"Game directory changed\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        disconnect, qfalse);
        }
        FS_Restart(checksumFeed);
        Cvar_Restart(qtrue);
        Com_ExecuteCfg();
        if 0 != disconnect as u64 { NET_Restart_f(); }
        if 0 != com_gameClientRestarting as u64 {
            CL_Init();
            CL_StartHunkUsers(qfalse);
        }
        com_gameRestarting = qfalse;
        com_gameClientRestarting = qfalse
    };
}
/*
==================
Com_ExecuteCfg

For controlling environment variables
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_ExecuteCfg() {
    Cbuf_ExecuteText(EXEC_NOW as libc::c_int,
                     b"exec default.cfg\n\x00" as *const u8 as
                         *const libc::c_char);
    Cbuf_Execute();
    if 0 == Com_SafeMode() as u64 {
        Cbuf_ExecuteText(EXEC_NOW as libc::c_int,
                         b"exec q3config_server.cfg\n\x00" as *const u8 as
                             *const libc::c_char);
        Cbuf_Execute();
        Cbuf_ExecuteText(EXEC_NOW as libc::c_int,
                         b"exec autoexec.cfg\n\x00" as *const u8 as
                             *const libc::c_char);
        Cbuf_Execute();
    };
}
#[no_mangle]
pub unsafe extern "C" fn Com_SafeMode() -> qboolean {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < com_numConsoleLines {
        Cmd_TokenizeString(com_consoleLines[i as usize]);
        if 0 ==
               Q_stricmp(Cmd_Argv(0i32),
                         b"safe\x00" as *const u8 as *const libc::c_char) ||
               0 ==
                   Q_stricmp(Cmd_Argv(0i32),
                             b"cvar_restart\x00" as *const u8 as
                                 *const libc::c_char) {
            *com_consoleLines[i as usize].offset(0isize) =
                0i32 as libc::c_char;
            return qtrue
        }
        i += 1
    }
    return qfalse;
}
#[no_mangle]
pub static mut com_consoleLines: [*mut libc::c_char; 32] =
    [0 as *const libc::c_char as *mut libc::c_char; 32];
/*
============================================================================

COMMAND LINE FUNCTIONS

+ characters separate the commandLine string into multiple console
command lines.

All of these are valid:

quake3 +set test blah +map test
quake3 set test blah+map test
quake3 set test blah + map test

============================================================================
*/
#[no_mangle]
pub static mut com_numConsoleLines: libc::c_int = 0;
#[no_mangle]
pub static mut com_fullyInitialized: qboolean = qfalse;
// will be journaled properly
#[no_mangle]
pub unsafe extern "C" fn Com_Milliseconds() -> libc::c_int {
    let mut ev: sysEvent_t =
        sysEvent_t{evTime: 0,
                   evType: SE_NONE,
                   evValue: 0,
                   evValue2: 0,
                   evPtrLength: 0,
                   evPtr: 0 as *const libc::c_void as *mut libc::c_void,};
    loop  {
        ev = Com_GetRealEvent();
        if ev.evType as libc::c_uint != SE_NONE as libc::c_int as libc::c_uint
           {
            Com_PushEvent(&mut ev);
        }
        if !(ev.evType as libc::c_uint !=
                 SE_NONE as libc::c_int as libc::c_uint) {
            break ;
        }
    }
    return ev.evTime;
}
/*
=================
Com_PushEvent
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_PushEvent(mut event: *mut sysEvent_t) {
    let mut ev: *mut sysEvent_t = 0 as *mut sysEvent_t;
    static mut printedWarning: libc::c_int = 0i32;
    ev =
        &mut com_pushedEvents[(com_pushedEventsHead & 1024i32 - 1i32) as
                                  usize] as *mut sysEvent_t;
    if com_pushedEventsHead - com_pushedEventsTail >= 1024i32 {
        if 0 == printedWarning {
            printedWarning = qtrue as libc::c_int;
            Com_Printf(b"WARNING: Com_PushEvent overflow\n\x00" as *const u8
                           as *const libc::c_char);
        }
        if !(*ev).evPtr.is_null() { Z_Free((*ev).evPtr); }
        com_pushedEventsTail += 1
    } else { printedWarning = qfalse as libc::c_int }
    *ev = *event;
    com_pushedEventsHead += 1;
}
#[no_mangle]
pub unsafe extern "C" fn Com_Filter(mut filter: *mut libc::c_char,
                                    mut name: *mut libc::c_char,
                                    mut casesensitive: libc::c_int)
 -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    while 0 != *filter {
        if *filter as libc::c_int == '*' as i32 {
            filter = filter.offset(1isize);
            i = 0i32;
            while 0 != *filter {
                if *filter as libc::c_int == '*' as i32 ||
                       *filter as libc::c_int == '?' as i32 {
                    break ;
                }
                buf[i as usize] = *filter;
                filter = filter.offset(1isize);
                i += 1
            }
            buf[i as usize] = '\u{0}' as i32 as libc::c_char;
            if 0 != strlen(buf.as_mut_ptr()) {
                ptr =
                    Com_StringContains(name, buf.as_mut_ptr(), casesensitive);
                if ptr.is_null() { return qfalse as libc::c_int }
                name = ptr.offset(strlen(buf.as_mut_ptr()) as isize)
            }
        } else if *filter as libc::c_int == '?' as i32 {
            filter = filter.offset(1isize);
            name = name.offset(1isize)
        } else if *filter as libc::c_int == '[' as i32 &&
                      *filter.offset(1isize) as libc::c_int == '[' as i32 {
            filter = filter.offset(1isize)
        } else if *filter as libc::c_int == '[' as i32 {
            filter = filter.offset(1isize);
            found = qfalse as libc::c_int;
            while 0 != *filter as libc::c_int && 0 == found {
                if *filter as libc::c_int == ']' as i32 &&
                       *filter.offset(1isize) as libc::c_int != ']' as i32 {
                    break ;
                }
                if *filter.offset(1isize) as libc::c_int == '-' as i32 &&
                       0 != *filter.offset(2isize) as libc::c_int &&
                       (*filter.offset(2isize) as libc::c_int != ']' as i32 ||
                            *filter.offset(3isize) as libc::c_int ==
                                ']' as i32) {
                    if 0 != casesensitive {
                        if *name as libc::c_int >= *filter as libc::c_int &&
                               *name as libc::c_int <=
                                   *filter.offset(2isize) as libc::c_int {
                            found = qtrue as libc::c_int
                        }
                    } else if toupper(*name as libc::c_int) >=
                                  toupper(*filter as libc::c_int) &&
                                  toupper(*name as libc::c_int) <=
                                      toupper(*filter.offset(2isize) as
                                                  libc::c_int) {
                        found = qtrue as libc::c_int
                    }
                    filter = filter.offset(3isize)
                } else {
                    if 0 != casesensitive {
                        if *filter as libc::c_int == *name as libc::c_int {
                            found = qtrue as libc::c_int
                        }
                    } else if toupper(*filter as libc::c_int) ==
                                  toupper(*name as libc::c_int) {
                        found = qtrue as libc::c_int
                    }
                    filter = filter.offset(1isize)
                }
            }
            if 0 == found { return qfalse as libc::c_int }
            while 0 != *filter {
                if *filter as libc::c_int == ']' as i32 &&
                       *filter.offset(1isize) as libc::c_int != ']' as i32 {
                    break ;
                }
                filter = filter.offset(1isize)
            }
            filter = filter.offset(1isize);
            name = name.offset(1isize)
        } else {
            if 0 != casesensitive {
                if *filter as libc::c_int != *name as libc::c_int {
                    return qfalse as libc::c_int
                }
            } else if toupper(*filter as libc::c_int) !=
                          toupper(*name as libc::c_int) {
                return qfalse as libc::c_int
            }
            filter = filter.offset(1isize);
            name = name.offset(1isize)
        }
    }
    return qtrue as libc::c_int;
}
/*
============
Com_StringContains
============
*/
#[no_mangle]
pub unsafe extern "C" fn Com_StringContains(mut str1: *mut libc::c_char,
                                            mut str2: *mut libc::c_char,
                                            mut casesensitive: libc::c_int)
 -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    len = strlen(str1).wrapping_sub(strlen(str2)) as libc::c_int;
    i = 0i32;
    while i <= len {
        j = 0i32;
        while 0 != *str2.offset(j as isize) {
            if 0 != casesensitive {
                if *str1.offset(j as isize) as libc::c_int !=
                       *str2.offset(j as isize) as libc::c_int {
                    break ;
                }
            } else if toupper(*str1.offset(j as isize) as libc::c_int) !=
                          toupper(*str2.offset(j as isize) as libc::c_int) {
                break ;
            }
            j += 1
        }
        if 0 == *str2.offset(j as isize) { return str1 }
        i += 1;
        str1 = str1.offset(1isize)
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Com_FilterPath(mut filter: *mut libc::c_char,
                                        mut name: *mut libc::c_char,
                                        mut casesensitive: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut new_filter: [libc::c_char; 64] = [0; 64];
    let mut new_name: [libc::c_char; 64] = [0; 64];
    i = 0i32;
    while i < 64i32 - 1i32 && 0 != *filter.offset(i as isize) as libc::c_int {
        if *filter.offset(i as isize) as libc::c_int == '\\' as i32 ||
               *filter.offset(i as isize) as libc::c_int == ':' as i32 {
            new_filter[i as usize] = '/' as i32 as libc::c_char
        } else { new_filter[i as usize] = *filter.offset(i as isize) }
        i += 1
    }
    new_filter[i as usize] = '\u{0}' as i32 as libc::c_char;
    i = 0i32;
    while i < 64i32 - 1i32 && 0 != *name.offset(i as isize) as libc::c_int {
        if *name.offset(i as isize) as libc::c_int == '\\' as i32 ||
               *name.offset(i as isize) as libc::c_int == ':' as i32 {
            new_name[i as usize] = '/' as i32 as libc::c_char
        } else { new_name[i as usize] = *name.offset(i as isize) }
        i += 1
    }
    new_name[i as usize] = '\u{0}' as i32 as libc::c_char;
    return Com_Filter(new_filter.as_mut_ptr(), new_name.as_mut_ptr(),
                      casesensitive);
}
#[no_mangle]
pub unsafe extern "C" fn Com_RealTime(mut qtime: *mut qtime_t)
 -> libc::c_int {
    let mut t: time_t = 0;
    let mut tms: *mut tm = 0 as *mut tm;
    t = time(0 as *mut time_t);
    if qtime.is_null() { return t as libc::c_int }
    tms = localtime(&mut t);
    if !tms.is_null() {
        (*qtime).tm_sec = (*tms).tm_sec;
        (*qtime).tm_min = (*tms).tm_min;
        (*qtime).tm_hour = (*tms).tm_hour;
        (*qtime).tm_mday = (*tms).tm_mday;
        (*qtime).tm_mon = (*tms).tm_mon;
        (*qtime).tm_year = (*tms).tm_year;
        (*qtime).tm_wday = (*tms).tm_wday;
        (*qtime).tm_yday = (*tms).tm_yday;
        (*qtime).tm_isdst = (*tms).tm_isdst
    }
    return t as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Com_IsVoipTarget(mut voipTargets: *mut uint8_t,
                                          mut voipTargetsSize: libc::c_int,
                                          mut clientNum: libc::c_int)
 -> qboolean {
    let mut index: libc::c_int = 0;
    if clientNum < 0i32 {
        index = 0i32;
        while index < voipTargetsSize {
            if 0 != *voipTargets.offset(index as isize) { return qtrue }
            index += 1
        }
        return qfalse
    }
    index = clientNum >> 3i32;
    if index < voipTargetsSize {
        return (*voipTargets.offset(index as isize) as libc::c_int &
                    1i32 << (clientNum & 0x7i32)) as qboolean
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn Com_StartupVariable(mut match_0:
                                                 *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0i32;
    while i < com_numConsoleLines {
        Cmd_TokenizeString(com_consoleLines[i as usize]);
        if !(0 !=
                 strcmp(Cmd_Argv(0i32),
                        b"set\x00" as *const u8 as *const libc::c_char)) {
            s = Cmd_Argv(1i32);
            if match_0.is_null() || 0 == strcmp(s, match_0) {
                if Cvar_Flags(s) as libc::c_uint == 0x80000000u32 {
                    Cvar_Get(s, Cmd_ArgsFrom(2i32), 0x80i32);
                } else { Cvar_Set2(s, Cmd_ArgsFrom(2i32), qfalse); }
            }
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Com_FieldStringToPlayerName(mut name:
                                                         *mut libc::c_char,
                                                     mut length: libc::c_int,
                                                     mut rawname:
                                                         *const libc::c_char)
 -> qboolean {
    let mut hex: [libc::c_char; 5] = [0; 5];
    let mut i: libc::c_int = 0;
    let mut ch: libc::c_int = 0;
    if name.is_null() || rawname.is_null() { return qfalse }
    if length <= 0i32 { return qtrue }
    i = 0i32;
    while 0 != *rawname as libc::c_int && i + 1i32 <= length {
        if *rawname as libc::c_int == '\\' as i32 {
            Q_strncpyz(hex.as_mut_ptr(), rawname.offset(1isize),
                       ::std::mem::size_of::<[libc::c_char; 5]>() as
                           libc::c_ulong as libc::c_int);
            ch = Com_HexStrToInt(hex.as_mut_ptr());
            if ch > -1i32 {
                *name.offset(i as isize) = ch as libc::c_char;
                rawname = rawname.offset(4isize)
            } else { *name.offset(i as isize) = *rawname }
        } else { *name.offset(i as isize) = *rawname }
        rawname = rawname.offset(1isize);
        i += 1
    }
    *name.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn Com_strCompare(mut a: *const libc::c_void,
                                        mut b: *const libc::c_void)
 -> libc::c_int {
    let mut pa: *mut *const libc::c_char = a as *mut *const libc::c_char;
    let mut pb: *mut *const libc::c_char = b as *mut *const libc::c_char;
    return strcmp(*pa, *pb);
}
#[no_mangle]
pub static mut com_dedicated: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_timescale: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_version: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_blood: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_cameraMode: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_ansiColor: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_unfocused: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_maxfpsUnfocused: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_minimized: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_maxfpsMinimized: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_altivec: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_standalone: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_basegame: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_homepath: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// both client and server must agree to pause
#[no_mangle]
pub static mut cl_paused: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_paused: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cl_packetdelay: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_packetdelay: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_gamename: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_protocol: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_legacyprotocol: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
// com_speeds times
#[no_mangle]
pub static mut time_game: libc::c_int = 0;
#[no_mangle]
pub static mut time_frontend: libc::c_int = 0;
// renderer backend time
#[no_mangle]
pub static mut time_backend: libc::c_int = 0;
#[no_mangle]
pub static mut com_frameTime: libc::c_int = 0;
#[no_mangle]
pub static mut com_journalDataFile: fileHandle_t = 0;
#[no_mangle]
pub unsafe extern "C" fn Z_FreeTags(mut tag: libc::c_int) {
    let mut zone: *mut memzone_t = 0 as *mut memzone_t;
    if tag == TAG_SMALL as libc::c_int {
        zone = smallzone
    } else { zone = mainzone }
    (*zone).rover = (*zone).blocklist.next;
    loop  {
        if (*(*zone).rover).tag == tag {
            Z_Free((*zone).rover.offset(1isize) as *mut libc::c_void);
        } else { (*zone).rover = (*(*zone).rover).next }
        if !((*zone).rover != &mut (*zone).blocklist as *mut memblock_t) {
            break ;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Z_AvailableMemory() -> libc::c_int {
    return Z_AvailableZoneMemory(mainzone);
}
/*
========================
Z_AvailableZoneMemory
========================
*/
unsafe extern "C" fn Z_AvailableZoneMemory(mut zone: *mut memzone_t)
 -> libc::c_int {
    return (*zone).size - (*zone).used;
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_Clear() {
    SV_ShutdownGameProgs();
    hunk_low.mark = 0i32;
    hunk_low.permanent = 0i32;
    hunk_low.temp = 0i32;
    hunk_low.tempHighwater = 0i32;
    hunk_high.mark = 0i32;
    hunk_high.permanent = 0i32;
    hunk_high.temp = 0i32;
    hunk_high.tempHighwater = 0i32;
    hunk_permanent = &mut hunk_low;
    hunk_temp = &mut hunk_high;
    Com_Printf(b"Hunk_Clear: reset the hunk ok\n\x00" as *const u8 as
                   *const libc::c_char);
    VM_Clear();
    hunkblocks = 0 as *mut hunkblock_t;
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_ClearToMark() {
    hunk_low.temp = hunk_low.mark;
    hunk_low.permanent = hunk_low.temp;
    hunk_high.temp = hunk_high.mark;
    hunk_high.permanent = hunk_high.temp;
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_SetMark() {
    hunk_low.mark = hunk_low.permanent;
    hunk_high.mark = hunk_high.permanent;
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_CheckMark() -> qboolean {
    if 0 != hunk_low.mark || 0 != hunk_high.mark { return qtrue }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_ClearTempMemory() {
    if !s_hunkData.is_null() { (*hunk_temp).temp = (*hunk_temp).permanent };
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_AllocateTempMemory(mut size: libc::c_int)
 -> *mut libc::c_void {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut hdr: *mut hunkHeader_t = 0 as *mut hunkHeader_t;
    if s_hunkData.is_null() {
        return Z_MallocDebug(size,
                             b"size\x00" as *const u8 as *const libc::c_char
                                 as *mut libc::c_char,
                             b"code/qcommon/common.c\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char,
                             1798i32)
    }
    Hunk_SwapBanks();
    size =
        ((size as
              libc::c_ulong).wrapping_add(::std::mem::size_of::<intptr_t>() as
                                              libc::c_ulong).wrapping_sub(1i32
                                                                              as
                                                                              libc::c_ulong)
             &
             !(::std::mem::size_of::<intptr_t>() as
                   libc::c_ulong).wrapping_sub(1i32 as
                                                   libc::c_ulong)).wrapping_add(::std::mem::size_of::<hunkHeader_t>()
                                                                                    as
                                                                                    libc::c_ulong)
            as libc::c_int;
    if (*hunk_temp).temp + (*hunk_permanent).permanent + size > s_hunkTotal {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Hunk_AllocateTempMemory: failed on %i\x00" as *const u8 as
                      *const libc::c_char, size);
    }
    if hunk_temp == &mut hunk_low as *mut hunkUsed_t {
        buf =
            s_hunkData.offset((*hunk_temp).temp as isize) as
                *mut libc::c_void;
        (*hunk_temp).temp += size
    } else {
        (*hunk_temp).temp += size;
        buf =
            s_hunkData.offset(s_hunkTotal as
                                  isize).offset(-((*hunk_temp).temp as isize))
                as *mut libc::c_void
    }
    if (*hunk_temp).temp > (*hunk_temp).tempHighwater {
        (*hunk_temp).tempHighwater = (*hunk_temp).temp
    }
    hdr = buf as *mut hunkHeader_t;
    buf = hdr.offset(1isize) as *mut libc::c_void;
    (*hdr).magic = 0x89537892u32 as libc::c_int;
    (*hdr).size = size;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_FreeTempMemory(mut buf: *mut libc::c_void) {
    let mut hdr: *mut hunkHeader_t = 0 as *mut hunkHeader_t;
    if s_hunkData.is_null() { Z_Free(buf); return }
    hdr = (buf as *mut hunkHeader_t).offset(-1isize);
    if (*hdr).magic as libc::c_uint != 0x89537892u32 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Hunk_FreeTempMemory: bad magic\x00" as *const u8 as
                      *const libc::c_char);
    }
    (*hdr).magic = 0x89537893u32 as libc::c_int;
    if hunk_temp == &mut hunk_low as *mut hunkUsed_t {
        if hdr ==
               s_hunkData.offset((*hunk_temp).temp as
                                     isize).offset(-((*hdr).size as isize)) as
                   *mut libc::c_void as *mut hunkHeader_t {
            (*hunk_temp).temp -= (*hdr).size
        } else {
            Com_Printf(b"Hunk_FreeTempMemory: not the final block\n\x00" as
                           *const u8 as *const libc::c_char);
        }
    } else if hdr ==
                  s_hunkData.offset(s_hunkTotal as
                                        isize).offset(-((*hunk_temp).temp as
                                                            isize)) as
                      *mut libc::c_void as *mut hunkHeader_t {
        (*hunk_temp).temp -= (*hdr).size
    } else {
        Com_Printf(b"Hunk_FreeTempMemory: not the final block\n\x00" as
                       *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Hunk_MemoryRemaining() -> libc::c_int {
    let mut low: libc::c_int = 0;
    let mut high: libc::c_int = 0;
    low =
        if hunk_low.permanent > hunk_low.temp {
            hunk_low.permanent
        } else { hunk_low.temp };
    high =
        if hunk_high.permanent > hunk_high.temp {
            hunk_high.permanent
        } else { hunk_high.temp };
    return s_hunkTotal - (low + high);
}
#[no_mangle]
pub unsafe extern "C" fn Com_TouchMemory() {
    let mut start: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_uint = 0;
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    Z_CheckHeap();
    start = Sys_Milliseconds();
    sum = 0i32 as libc::c_uint;
    j = hunk_low.permanent >> 2i32;
    i = 0i32;
    while i < j {
        sum =
            sum.wrapping_add(*(s_hunkData as
                                   *mut libc::c_int).offset(i as isize) as
                                 libc::c_uint);
        i += 64i32
    }
    i = s_hunkTotal - hunk_high.permanent >> 2i32;
    j = hunk_high.permanent >> 2i32;
    while i < j {
        sum =
            sum.wrapping_add(*(s_hunkData as
                                   *mut libc::c_int).offset(i as isize) as
                                 libc::c_uint);
        i += 64i32
    }
    block = (*mainzone).blocklist.next;
    loop  {
        if 0 != (*block).tag {
            j = (*block).size >> 2i32;
            i = 0i32;
            while i < j {
                sum =
                    sum.wrapping_add(*(block as
                                           *mut libc::c_int).offset(i as
                                                                        isize)
                                         as libc::c_uint);
                i += 64i32
            }
        }
        if (*block).next == &mut (*mainzone).blocklist as *mut memblock_t {
            // all blocks have been hit	
            break ;
        } else { block = (*block).next }
    }
    end = Sys_Milliseconds();
    Com_Printf(b"Com_TouchMemory: %i msec\n\x00" as *const u8 as
                   *const libc::c_char, end - start);
}
unsafe extern "C" fn Z_CheckHeap() {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    block = (*mainzone).blocklist.next;
    loop  {
        if (*block).next == &mut (*mainzone).blocklist as *mut memblock_t {
            // all blocks have been hit
            break ;
        } else {
            if (block as *mut byte).offset((*block).size as isize) !=
                   (*block).next as *mut byte {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"Z_CheckHeap: block size does not touch the next block\x00"
                              as *const u8 as *const libc::c_char);
            }
            if (*(*block).next).prev != block {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"Z_CheckHeap: next block doesn\'t have proper back link\x00"
                              as *const u8 as *const libc::c_char);
            }
            if 0 == (*block).tag && 0 == (*(*block).next).tag {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"Z_CheckHeap: two consecutive free blocks\x00" as
                              *const u8 as *const libc::c_char);
            }
            block = (*block).next
        }
    };
}
// commandLine should not include the executable name (argv[0])
#[no_mangle]
pub unsafe extern "C" fn Com_Init(mut commandLine: *mut libc::c_char) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut qport: libc::c_int = 0;
    Com_Printf(b"%s %s %s\n\x00" as *const u8 as *const libc::c_char,
               b"ioq3 1.36_GIT_3db749c8-2019-01-29\x00" as *const u8 as
                   *const libc::c_char,
               b"linux-x86_64-debug\x00" as *const u8 as *const libc::c_char,
               b"Jan 30 2019\x00" as *const u8 as *const libc::c_char);
    if 0 != _setjmp(abortframe.as_mut_ptr()) {
        Sys_Error(b"Error during initialization\x00" as *const u8 as
                      *const libc::c_char);
    }
    memset(&mut eventQueue[0usize] as *mut sysEvent_t as *mut libc::c_void,
           0i32,
           (256i32 as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<sysEvent_t>()
                                                as libc::c_ulong));
    Com_InitRand();
    Com_InitPushEvent();
    Com_InitSmallZoneMemory();
    Cvar_Init();
    Com_ParseCommandLine(commandLine);
    Cbuf_Init();
    Com_DetectSSE();
    Com_StartupVariable(0 as *const libc::c_char);
    Com_InitZoneMemory();
    Cmd_Init();
    com_developer =
        Cvar_Get(b"developer\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x100i32);
    CL_InitKeyCommands();
    com_standalone =
        Cvar_Get(b"com_standalone\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x40i32);
    com_basegame =
        Cvar_Get(b"com_basegame\x00" as *const u8 as *const libc::c_char,
                 b"baseq3\x00" as *const u8 as *const libc::c_char, 0x10i32);
    com_homepath =
        Cvar_Get(b"com_homepath\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 0x10i32 | 0x2000i32);
    FS_InitFilesystem();
    Com_InitJournaling();
    Cmd_AddCommand(b"setenv\x00" as *const u8 as *const libc::c_char,
                   Some(Com_Setenv_f));
    if !com_developer.is_null() && 0 != (*com_developer).integer {
        Cmd_AddCommand(b"error\x00" as *const u8 as *const libc::c_char,
                       ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                          -> !>,
                                               xcommand_t>(Some(Com_Error_f)));
        Cmd_AddCommand(b"crash\x00" as *const u8 as *const libc::c_char,
                       Some(Com_Crash_f));
        Cmd_AddCommand(b"freeze\x00" as *const u8 as *const libc::c_char,
                       Some(Com_Freeze_f));
    }
    Cmd_AddCommand(b"quit\x00" as *const u8 as *const libc::c_char,
                   ::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                      -> !>,
                                           xcommand_t>(Some(Com_Quit_f)));
    Cmd_AddCommand(b"changeVectors\x00" as *const u8 as *const libc::c_char,
                   Some(MSG_ReportChangeVectors_f));
    Cmd_AddCommand(b"writeconfig\x00" as *const u8 as *const libc::c_char,
                   Some(Com_WriteConfig_f));
    Cmd_SetCommandCompletionFunc(b"writeconfig\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cmd_CompleteCfgName));
    Cmd_AddCommand(b"game_restart\x00" as *const u8 as *const libc::c_char,
                   Some(Com_GameRestart_f));
    Com_ExecuteCfg();
    Com_StartupVariable(0 as *const libc::c_char);
    com_dedicated =
        Cvar_Get(b"dedicated\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x10i32);
    Cvar_CheckRange(com_dedicated, 1i32 as libc::c_float,
                    2i32 as libc::c_float, qtrue);
    Com_InitHunkMemory();
    cvar_modifiedFlags &= !0x1i32;
    com_altivec =
        Cvar_Get(b"com_altivec\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    com_maxfps =
        Cvar_Get(b"com_maxfps\x00" as *const u8 as *const libc::c_char,
                 b"85\x00" as *const u8 as *const libc::c_char, 0x1i32);
    com_blood =
        Cvar_Get(b"com_blood\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    com_logfile =
        Cvar_Get(b"logfile\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x100i32);
    com_timescale =
        Cvar_Get(b"timescale\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 0x200i32 | 0x8i32);
    com_fixedtime =
        Cvar_Get(b"fixedtime\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    com_showtrace =
        Cvar_Get(b"com_showtrace\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    com_speeds =
        Cvar_Get(b"com_speeds\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    com_timedemo =
        Cvar_Get(b"timedemo\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    com_cameraMode =
        Cvar_Get(b"com_cameraMode\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    cl_paused =
        Cvar_Get(b"cl_paused\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x40i32);
    sv_paused =
        Cvar_Get(b"sv_paused\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x40i32);
    cl_packetdelay =
        Cvar_Get(b"cl_packetdelay\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    sv_packetdelay =
        Cvar_Get(b"sv_packetdelay\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x200i32);
    com_sv_running =
        Cvar_Get(b"sv_running\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x40i32);
    com_cl_running =
        Cvar_Get(b"cl_running\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x40i32);
    com_buildScript =
        Cvar_Get(b"com_buildScript\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    com_ansiColor =
        Cvar_Get(b"com_ansiColor\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    com_unfocused =
        Cvar_Get(b"com_unfocused\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x40i32);
    com_maxfpsUnfocused =
        Cvar_Get(b"com_maxfpsUnfocused\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    com_minimized =
        Cvar_Get(b"com_minimized\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x40i32);
    com_maxfpsMinimized =
        Cvar_Get(b"com_maxfpsMinimized\x00" as *const u8 as
                     *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    com_abnormalExit =
        Cvar_Get(b"com_abnormalExit\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x40i32);
    com_busyWait =
        Cvar_Get(b"com_busyWait\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    Cvar_Get(b"com_errorMessage\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x40i32 | 0x400i32);
    com_introPlayed =
        Cvar_Get(b"com_introplayed\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x1i32);
    s =
        va(b"%s %s %s\x00" as *const u8 as *const libc::c_char as
               *mut libc::c_char,
           b"ioq3 1.36_GIT_3db749c8-2019-01-29\x00" as *const u8 as
               *const libc::c_char,
           b"linux-x86_64-debug\x00" as *const u8 as *const libc::c_char,
           b"Jan 30 2019\x00" as *const u8 as *const libc::c_char);
    com_version =
        Cvar_Get(b"version\x00" as *const u8 as *const libc::c_char, s,
                 0x40i32 | 0x4i32);
    com_gamename =
        Cvar_Get(b"com_gamename\x00" as *const u8 as *const libc::c_char,
                 b"Quake3Arena\x00" as *const u8 as *const libc::c_char,
                 0x4i32 | 0x10i32);
    com_protocol =
        Cvar_Get(b"com_protocol\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, 71i32), 0x4i32 | 0x10i32);
    com_legacyprotocol =
        Cvar_Get(b"com_legacyprotocol\x00" as *const u8 as
                     *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, 68i32), 0x10i32);
    if (*com_legacyprotocol).integer > 0i32 {
        Cvar_Get(b"protocol\x00" as *const u8 as *const libc::c_char,
                 (*com_legacyprotocol).string, 0x40i32);
    } else {
        Cvar_Get(b"protocol\x00" as *const u8 as *const libc::c_char,
                 (*com_protocol).string, 0x40i32);
    }
    Sys_Init();
    Sys_InitPIDFile(FS_GetCurrentGameDir());
    Com_RandomBytes(&mut qport as *mut libc::c_int as *mut byte,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                        libc::c_int);
    Netchan_Init(qport & 0xffffi32);
    VM_Init();
    SV_Init();
    (*com_dedicated).modified = qfalse;
    com_frameTime = Com_Milliseconds();
    if 0 == Com_AddStartupCommands() as u64 {
        if 0 == (*com_dedicated).integer {
            Cbuf_AddText(b"cinematic idlogo.RoQ\n\x00" as *const u8 as
                             *const libc::c_char);
            if 0 == (*com_introPlayed).integer {
                Cvar_Set((*com_introPlayed).name,
                         b"1\x00" as *const u8 as *const libc::c_char);
                Cvar_Set(b"nextmap\x00" as *const u8 as *const libc::c_char,
                         b"cinematic intro.RoQ\x00" as *const u8 as
                             *const libc::c_char);
            }
        }
    }
    Cvar_Set(b"r_uiFullScreen\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char);
    CL_StartHunkUsers(qfalse);
    Cvar_Set(b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    com_fullyInitialized = qtrue;
    Com_DetectAltivec();
    com_pipefile =
        Cvar_Get(b"com_pipefile\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x20i32);
    if 0 != *(*com_pipefile).string.offset(0isize) {
        pipefile = FS_FCreateOpenPipeFile((*com_pipefile).string)
    }
    Com_Printf(b"--- Common Initialization Complete ---\n\x00" as *const u8 as
                   *const libc::c_char);
}
// STANDALONE
unsafe extern "C" fn Com_DetectAltivec() {
    if 0 != (*com_altivec).integer {
        static mut altivec: qboolean = qfalse;
        static mut detected: qboolean = qfalse;
        if 0 == detected as u64 {
            altivec =
                (Sys_GetProcessorFeatures() as libc::c_uint &
                     CF_ALTIVEC as libc::c_int as libc::c_uint) as qboolean;
            detected = qtrue
        }
        if 0 == altivec as u64 {
            Cvar_Set(b"com_altivec\x00" as *const u8 as *const libc::c_char,
                     b"0\x00" as *const u8 as *const libc::c_char);
        }
    };
}
// for automated data building scripts
#[no_mangle]
pub static mut com_introPlayed: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
/*
=================
Com_AddStartupCommands

Adds command line parameters as script statements
Commands are separated by + signs

Returns qtrue if any late commands were added, which
will keep the demoloop from immediately starting
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_AddStartupCommands() -> qboolean {
    let mut i: libc::c_int = 0;
    let mut added: qboolean = qfalse;
    added = qfalse;
    i = 0i32;
    while i < com_numConsoleLines {
        if !(com_consoleLines[i as usize].is_null() ||
                 0 == *com_consoleLines[i as usize].offset(0isize)) {
            // set commands already added with Com_StartupVariable
            if !(0 ==
                     Q_stricmpn(com_consoleLines[i as usize],
                                b"set \x00" as *const u8 as
                                    *const libc::c_char, 4i32)) {
                added = qtrue;
                Cbuf_AddText(com_consoleLines[i as usize]);
                Cbuf_AddText(b"\n\x00" as *const u8 as *const libc::c_char);
            }
        }
        i += 1
    }
    return added;
}
#[no_mangle]
pub static mut com_busyWait: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_abnormalExit: *mut cvar_t =
    0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_timedemo: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_showtrace: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// events are written here
// config files are written here
#[no_mangle]
pub static mut com_fixedtime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut com_maxfps: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
=================
Com_InitHunkZoneMemory
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_InitHunkMemory() {
    let mut cv: *mut cvar_t = 0 as *mut cvar_t;
    let mut nMinAlloc: libc::c_int = 0;
    let mut pMsg: *mut libc::c_char = 0 as *mut libc::c_char;
    if FS_LoadStack() != 0i32 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Hunk initialization failed. File system load stack not zero\x00"
                      as *const u8 as *const libc::c_char);
    }
    cv =
        Cvar_Get(b"com_hunkMegs\x00" as *const u8 as *const libc::c_char,
                 b"128\x00" as *const u8 as *const libc::c_char,
                 0x20i32 | 0x1i32);
    Cvar_SetDescription(cv,
                        b"The size of the hunk memory segment\x00" as
                            *const u8 as *const libc::c_char);
    if !com_dedicated.is_null() && 0 != (*com_dedicated).integer {
        nMinAlloc = 1i32;
        pMsg =
            b"Minimum com_hunkMegs for a dedicated server is %i, allocating %i megs.\n\x00"
                as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        nMinAlloc = 56i32;
        pMsg =
            b"Minimum com_hunkMegs is %i, allocating %i megs.\n\x00" as
                *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if (*cv).integer < nMinAlloc {
        s_hunkTotal = 1024i32 * 1024i32 * nMinAlloc;
        Com_Printf(pMsg, nMinAlloc, s_hunkTotal / (1024i32 * 1024i32));
    } else { s_hunkTotal = (*cv).integer * 1024i32 * 1024i32 }
    s_hunkData =
        calloc((s_hunkTotal + 31i32) as libc::c_ulong, 1i32 as libc::c_ulong)
            as *mut byte;
    if s_hunkData.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Hunk data failed to allocate %i megs\x00" as *const u8 as
                      *const libc::c_char, s_hunkTotal / (1024i32 * 1024i32));
    }
    s_hunkData =
        (s_hunkData as intptr_t + 31i32 as libc::c_long &
             !31i32 as libc::c_long) as *mut byte;
    Hunk_Clear();
    Cmd_AddCommand(b"meminfo\x00" as *const u8 as *const libc::c_char,
                   Some(Com_Meminfo_f));
    Cmd_AddCommand(b"zonelog\x00" as *const u8 as *const libc::c_char,
                   Some(Z_LogHeap));
    Cmd_AddCommand(b"hunklog\x00" as *const u8 as *const libc::c_char,
                   Some(Hunk_Log));
    Cmd_AddCommand(b"hunksmalllog\x00" as *const u8 as *const libc::c_char,
                   Some(Hunk_SmallLog));
}
/*
=================
Com_Meminfo_f
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_Meminfo_f() {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    let mut zoneBytes: libc::c_int = 0;
    let mut zoneBlocks: libc::c_int = 0;
    let mut smallZoneBytes: libc::c_int = 0;
    let mut botlibBytes: libc::c_int = 0;
    let mut rendererBytes: libc::c_int = 0;
    let mut unused: libc::c_int = 0;
    zoneBytes = 0i32;
    botlibBytes = 0i32;
    rendererBytes = 0i32;
    zoneBlocks = 0i32;
    block = (*mainzone).blocklist.next;
    loop  {
        if Cmd_Argc() != 1i32 {
            Com_Printf(b"block:%p    size:%7i    tag:%3i\n\x00" as *const u8
                           as *const libc::c_char, block as *mut libc::c_void,
                       (*block).size, (*block).tag);
        }
        if 0 != (*block).tag {
            zoneBytes += (*block).size;
            zoneBlocks += 1;
            if (*block).tag == TAG_BOTLIB as libc::c_int {
                botlibBytes += (*block).size
            } else if (*block).tag == TAG_RENDERER as libc::c_int {
                rendererBytes += (*block).size
            }
        }
        if (*block).next == &mut (*mainzone).blocklist as *mut memblock_t {
            // all blocks have been hit	
            break ;
        } else {
            if (block as *mut byte).offset((*block).size as isize) !=
                   (*block).next as *mut byte {
                Com_Printf(b"ERROR: block size does not touch the next block\n\x00"
                               as *const u8 as *const libc::c_char);
            }
            if (*(*block).next).prev != block {
                Com_Printf(b"ERROR: next block doesn\'t have proper back link\n\x00"
                               as *const u8 as *const libc::c_char);
            }
            if 0 == (*block).tag && 0 == (*(*block).next).tag {
                Com_Printf(b"ERROR: two consecutive free blocks\n\x00" as
                               *const u8 as *const libc::c_char);
            }
            block = (*block).next
        }
    }
    smallZoneBytes = 0i32;
    block = (*smallzone).blocklist.next;
    loop  {
        if 0 != (*block).tag { smallZoneBytes += (*block).size }
        if (*block).next == &mut (*smallzone).blocklist as *mut memblock_t {
            // all blocks have been hit	
            break ;
        } else { block = (*block).next }
    }
    Com_Printf(b"%8i bytes total hunk\n\x00" as *const u8 as
                   *const libc::c_char, s_hunkTotal);
    Com_Printf(b"%8i bytes total zone\n\x00" as *const u8 as
                   *const libc::c_char, s_zoneTotal);
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(b"%8i low mark\n\x00" as *const u8 as *const libc::c_char,
               hunk_low.mark);
    Com_Printf(b"%8i low permanent\n\x00" as *const u8 as *const libc::c_char,
               hunk_low.permanent);
    if hunk_low.temp != hunk_low.permanent {
        Com_Printf(b"%8i low temp\n\x00" as *const u8 as *const libc::c_char,
                   hunk_low.temp);
    }
    Com_Printf(b"%8i low tempHighwater\n\x00" as *const u8 as
                   *const libc::c_char, hunk_low.tempHighwater);
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(b"%8i high mark\n\x00" as *const u8 as *const libc::c_char,
               hunk_high.mark);
    Com_Printf(b"%8i high permanent\n\x00" as *const u8 as
                   *const libc::c_char, hunk_high.permanent);
    if hunk_high.temp != hunk_high.permanent {
        Com_Printf(b"%8i high temp\n\x00" as *const u8 as *const libc::c_char,
                   hunk_high.temp);
    }
    Com_Printf(b"%8i high tempHighwater\n\x00" as *const u8 as
                   *const libc::c_char, hunk_high.tempHighwater);
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(b"%8i total hunk in use\n\x00" as *const u8 as
                   *const libc::c_char,
               hunk_low.permanent + hunk_high.permanent);
    unused = 0i32;
    if hunk_low.tempHighwater > hunk_low.permanent {
        unused += hunk_low.tempHighwater - hunk_low.permanent
    }
    if hunk_high.tempHighwater > hunk_high.permanent {
        unused += hunk_high.tempHighwater - hunk_high.permanent
    }
    Com_Printf(b"%8i unused highwater\n\x00" as *const u8 as
                   *const libc::c_char, unused);
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    Com_Printf(b"%8i bytes in %i zone blocks\n\x00" as *const u8 as
                   *const libc::c_char, zoneBytes, zoneBlocks);
    Com_Printf(b"        %8i bytes in dynamic botlib\n\x00" as *const u8 as
                   *const libc::c_char, botlibBytes);
    Com_Printf(b"        %8i bytes in dynamic renderer\n\x00" as *const u8 as
                   *const libc::c_char, rendererBytes);
    Com_Printf(b"        %8i bytes in dynamic other\n\x00" as *const u8 as
                   *const libc::c_char,
               zoneBytes - (botlibBytes + rendererBytes));
    Com_Printf(b"        %8i bytes in small Zone memory\n\x00" as *const u8 as
                   *const libc::c_char, smallZoneBytes);
}
static mut s_zoneTotal: libc::c_int = 0;
/*
==================
Com_GameRestart_f

Expose possibility to change current running mod to the user
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_GameRestart_f() {
    Cvar_Set(b"fs_game\x00" as *const u8 as *const libc::c_char,
             Cmd_Argv(1i32));
    Com_GameRestart(0i32, qtrue);
}
#[no_mangle]
pub unsafe extern "C" fn Com_WriteConfig_f() {
    let mut filename: [libc::c_char; 64] = [0; 64];
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"Usage: writeconfig <filename>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Q_strncpyz(filename.as_mut_ptr(), Cmd_Argv(1i32),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    COM_DefaultExtension(filename.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 64]>() as
                             libc::c_ulong as libc::c_int,
                         b".cfg\x00" as *const u8 as *const libc::c_char);
    if 0 ==
           COM_CompareExtension(filename.as_mut_ptr(),
                                b".cfg\x00" as *const u8 as
                                    *const libc::c_char) as u64 {
        Com_Printf(b"Com_WriteConfig_f: Only the \".cfg\" extension is supported by this command!\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    Com_Printf(b"Writing %s.\n\x00" as *const u8 as *const libc::c_char,
               filename.as_mut_ptr());
    Com_WriteConfigToFile(filename.as_mut_ptr());
}
//==================================================================
#[no_mangle]
pub unsafe extern "C" fn Com_WriteConfigToFile(mut filename:
                                                   *const libc::c_char) {
    let mut f: fileHandle_t = 0;
    f = FS_FOpenFileWrite(filename);
    if 0 == f {
        Com_Printf(b"Couldn\'t write %s.\n\x00" as *const u8 as
                       *const libc::c_char, filename);
        return
    }
    FS_Printf(f,
              b"// generated by quake, do not modify\n\x00" as *const u8 as
                  *const libc::c_char);
    Key_WriteBindings(f);
    Cvar_WriteVariables(f);
    FS_FCloseFile(f);
}
/*
=============
Com_Freeze_f

Just freeze in place for a given number of seconds to test
error recovery
=============
*/
unsafe extern "C" fn Com_Freeze_f() {
    let mut s: libc::c_float = 0.;
    let mut start: libc::c_int = 0;
    let mut now: libc::c_int = 0;
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"freeze <seconds>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    s = atof(Cmd_Argv(1i32)) as libc::c_float;
    start = Com_Milliseconds();
    loop  {
        now = Com_Milliseconds();
        if (now - start) as libc::c_double * 0.001f64 > s as libc::c_double {
            break ;
        }
    };
}
/*
=================
Com_Crash_f

A way to force a bus error for development reasons
=================
*/
unsafe extern "C" fn Com_Crash_f() {
    ::std::ptr::write_volatile(0 as *mut libc::c_int, 0x12345678i32);
}
//============================================================================
/*
=============
Com_Error_f

Just throw a fatal error to
test error shutdown procedures
=============
*/
unsafe extern "C" fn Com_Error_f() -> ! {
    if Cmd_Argc() > 1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Testing drop error\x00" as *const u8 as
                      *const libc::c_char);
    } else {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Testing fatal error\x00" as *const u8 as
                      *const libc::c_char);
    };
}
/*
==================
Com_Setenv_f

For controlling environment variables
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_Setenv_f() {
    let mut argc: libc::c_int = Cmd_Argc();
    let mut arg1: *mut libc::c_char = Cmd_Argv(1i32);
    if argc > 2i32 {
        let mut arg2: *mut libc::c_char = Cmd_ArgsFrom(2i32);
        Sys_SetEnv(arg1, arg2);
    } else if argc == 2i32 {
        let mut env: *mut libc::c_char = getenv(arg1);
        if !env.is_null() {
            Com_Printf(b"%s=%s\n\x00" as *const u8 as *const libc::c_char,
                       arg1, env);
        } else {
            Com_Printf(b"%s undefined\n\x00" as *const u8 as
                           *const libc::c_char, arg1);
        }
    };
}
/*
=================
Com_InitJournaling
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_InitJournaling() {
    Com_StartupVariable(b"journal\x00" as *const u8 as *const libc::c_char);
    com_journal =
        Cvar_Get(b"journal\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x10i32);
    if 0 == (*com_journal).integer { return }
    if (*com_journal).integer == 1i32 {
        Com_Printf(b"Journaling events\n\x00" as *const u8 as
                       *const libc::c_char);
        com_journalFile =
            FS_FOpenFileWrite(b"journal.dat\x00" as *const u8 as
                                  *const libc::c_char);
        com_journalDataFile =
            FS_FOpenFileWrite(b"journaldata.dat\x00" as *const u8 as
                                  *const libc::c_char)
    } else if (*com_journal).integer == 2i32 {
        Com_Printf(b"Replaying journaled events\n\x00" as *const u8 as
                       *const libc::c_char);
        FS_FOpenFileRead(b"journal.dat\x00" as *const u8 as
                             *const libc::c_char, &mut com_journalFile,
                         qtrue);
        FS_FOpenFileRead(b"journaldata.dat\x00" as *const u8 as
                             *const libc::c_char, &mut com_journalDataFile,
                         qtrue);
    }
    if 0 == com_journalFile || 0 == com_journalDataFile {
        Cvar_Set(b"com_journal\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char);
        com_journalFile = 0i32;
        com_journalDataFile = 0i32;
        Com_Printf(b"Couldn\'t open journal files\n\x00" as *const u8 as
                       *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Com_InitZoneMemory() {
    let mut cv: *mut cvar_t = 0 as *mut cvar_t;
    cv =
        Cvar_Get(b"com_zoneMegs\x00" as *const u8 as *const libc::c_char,
                 b"24\x00" as *const u8 as *const libc::c_char,
                 0x20i32 | 0x1i32);
    if (*cv).integer < 24i32 {
        s_zoneTotal = 1024i32 * 1024i32 * 24i32
    } else { s_zoneTotal = (*cv).integer * 1024i32 * 1024i32 }
    mainzone =
        calloc(s_zoneTotal as libc::c_ulong, 1i32 as libc::c_ulong) as
            *mut memzone_t;
    if mainzone.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Zone data failed to allocate %i megs\x00" as *const u8 as
                      *const libc::c_char, s_zoneTotal / (1024i32 * 1024i32));
    }
    Z_ClearZone(mainzone, s_zoneTotal);
}
/*
========================
Z_ClearZone
========================
*/
unsafe extern "C" fn Z_ClearZone(mut zone: *mut memzone_t,
                                 mut size: libc::c_int) {
    let mut block: *mut memblock_t = 0 as *mut memblock_t;
    block =
        (zone as
             *mut byte).offset(::std::mem::size_of::<memzone_t>() as
                                   libc::c_ulong as isize) as *mut memblock_t;
    (*zone).blocklist.prev = block;
    (*zone).blocklist.next = (*zone).blocklist.prev;
    (*zone).blocklist.tag = 1i32;
    (*zone).blocklist.id = 0i32;
    (*zone).blocklist.size = 0i32;
    (*zone).rover = block;
    (*zone).size = size;
    (*zone).used = 0i32;
    (*block).next = &mut (*zone).blocklist;
    (*block).prev = (*block).next;
    (*block).tag = 0i32;
    (*block).id = 0x1d4a11i32;
    (*block).size =
        (size as
             libc::c_ulong).wrapping_sub(::std::mem::size_of::<memzone_t>() as
                                             libc::c_ulong) as libc::c_int;
}
/*
=================
Com_DetectSSE
Find out whether we have SSE support for Q_ftol function
=================
*/
unsafe extern "C" fn Com_DetectSSE() {
    Q_VMftol = Some(qvmftolsse);
    Com_Printf(b"SSE instruction set enabled\n\x00" as *const u8 as
                   *const libc::c_char);
}
/*
==================
Com_ParseCommandLine

Break it up into multiple console lines
==================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_ParseCommandLine(mut commandLine:
                                                  *mut libc::c_char) {
    let mut inq: libc::c_int = 0i32;
    com_consoleLines[0usize] = commandLine;
    com_numConsoleLines = 1i32;
    while 0 != *commandLine {
        if *commandLine as libc::c_int == '\"' as i32 {
            inq = (0 == inq) as libc::c_int
        }
        if *commandLine as libc::c_int == '+' as i32 && 0 == inq ||
               *commandLine as libc::c_int == '\n' as i32 ||
               *commandLine as libc::c_int == '\r' as i32 {
            if com_numConsoleLines == 32i32 { return }
            com_consoleLines[com_numConsoleLines as usize] =
                commandLine.offset(1isize);
            com_numConsoleLines += 1;
            *commandLine = 0i32 as libc::c_char
        }
        commandLine = commandLine.offset(1isize)
    };
}
/*
=================
Com_InitZoneMemory
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_InitSmallZoneMemory() {
    s_smallZoneTotal = 512i32 * 1024i32;
    smallzone =
        calloc(s_smallZoneTotal as libc::c_ulong, 1i32 as libc::c_ulong) as
            *mut memzone_t;
    if smallzone.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Small zone data failed to allocate %1.1f megs\x00" as
                      *const u8 as *const libc::c_char,
                  (s_smallZoneTotal as libc::c_float /
                       (1024i32 * 1024i32) as libc::c_float) as
                      libc::c_double);
    }
    Z_ClearZone(smallzone, s_smallZoneTotal);
}
static mut s_smallZoneTotal: libc::c_int = 0;
/*
=================
Com_InitPushEvent
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_InitPushEvent() {
    memset(com_pushedEvents.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[sysEvent_t; 1024]>() as libc::c_ulong);
    com_pushedEventsHead = 0i32;
    com_pushedEventsTail = 0i32;
}
/*
=================
Com_InitRand
Seed the random number generator, if possible with an OS supplied random seed.
=================
*/
unsafe extern "C" fn Com_InitRand() {
    let mut seed: libc::c_uint = 0;
    if 0 !=
           Sys_RandomBytes(&mut seed as *mut libc::c_uint as *mut byte,
                           ::std::mem::size_of::<libc::c_uint>() as
                               libc::c_ulong as libc::c_int) as u64 {
        srand(seed);
    } else { srand(time(0 as *mut time_t) as libc::c_uint); };
}
#[no_mangle]
pub unsafe extern "C" fn Com_Frame() {
    let mut msec: libc::c_int = 0;
    let mut minMsec: libc::c_int = 0;
    let mut timeVal: libc::c_int = 0;
    let mut timeValSV: libc::c_int = 0;
    static mut lastTime: libc::c_int = 0i32;
    static mut bias: libc::c_int = 0i32;
    let mut timeBeforeFirstEvents: libc::c_int = 0;
    let mut timeBeforeServer: libc::c_int = 0;
    let mut timeBeforeEvents: libc::c_int = 0;
    let mut timeBeforeClient: libc::c_int = 0;
    let mut timeAfter: libc::c_int = 0;
    if 0 != _setjmp(abortframe.as_mut_ptr()) { return }
    timeBeforeFirstEvents = 0i32;
    timeBeforeServer = 0i32;
    timeBeforeEvents = 0i32;
    timeBeforeClient = 0i32;
    timeAfter = 0i32;
    Com_WriteConfiguration();
    if 0 != (*com_speeds).integer {
        timeBeforeFirstEvents = Sys_Milliseconds()
    }
    if 0 == (*com_timedemo).integer {
        if 0 != (*com_dedicated).integer {
            minMsec = SV_FrameMsec()
        } else {
            if 0 != (*com_minimized).integer &&
                   (*com_maxfpsMinimized).integer > 0i32 {
                minMsec = 1000i32 / (*com_maxfpsMinimized).integer
            } else if 0 != (*com_unfocused).integer &&
                          (*com_maxfpsUnfocused).integer > 0i32 {
                minMsec = 1000i32 / (*com_maxfpsUnfocused).integer
            } else if (*com_maxfps).integer > 0i32 {
                minMsec = 1000i32 / (*com_maxfps).integer
            } else { minMsec = 1i32 }
            timeVal = com_frameTime - lastTime;
            bias += timeVal - minMsec;
            if bias > minMsec { bias = minMsec }
            minMsec -= bias
        }
    } else { minMsec = 1i32 }
    loop  {
        if 0 != (*com_sv_running).integer {
            timeValSV = SV_SendQueuedPackets();
            timeVal = Com_TimeVal(minMsec);
            if timeValSV < timeVal { timeVal = timeValSV }
        } else { timeVal = Com_TimeVal(minMsec) }
        if 0 != (*com_busyWait).integer || timeVal < 1i32 {
            NET_Sleep(0i32);
        } else { NET_Sleep(timeVal - 1i32); }
        if !(0 != Com_TimeVal(minMsec)) { break ; }
    }
    IN_Frame();
    lastTime = com_frameTime;
    com_frameTime = Com_EventLoop();
    msec = com_frameTime - lastTime;
    Cbuf_Execute();
    if 0 != (*com_altivec).modified as u64 {
        Com_DetectAltivec();
        (*com_altivec).modified = qfalse
    }
    msec = Com_ModifyMsec(msec);
    if 0 != (*com_speeds).integer { timeBeforeServer = Sys_Milliseconds() }
    SV_Frame(msec);
    if 0 != (*com_dedicated).modified as u64 {
        Cvar_Get(b"dedicated\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
        (*com_dedicated).modified = qfalse;
        if 0 == (*com_dedicated).integer {
            SV_Shutdown(b"dedicated set to 0\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char);
            CL_FlushMemory();
        }
    }
    if 0 != (*com_speeds).integer {
        timeAfter = Sys_Milliseconds();
        timeBeforeEvents = timeAfter;
        timeBeforeClient = timeAfter
    }
    NET_FlushPacketQueue();
    if 0 != (*com_speeds).integer {
        let mut all: libc::c_int = 0;
        let mut sv: libc::c_int = 0;
        let mut ev: libc::c_int = 0;
        let mut cl: libc::c_int = 0;
        all = timeAfter - timeBeforeServer;
        sv = timeBeforeEvents - timeBeforeServer;
        ev =
            timeBeforeServer - timeBeforeFirstEvents + timeBeforeClient -
                timeBeforeEvents;
        cl = timeAfter - timeBeforeClient;
        sv -= time_game;
        cl -= time_frontend + time_backend;
        Com_Printf(b"frame:%i all:%3i sv:%3i ev:%3i cl:%3i gm:%3i rf:%3i bk:%3i\n\x00"
                       as *const u8 as *const libc::c_char, com_frameNumber,
                   all, sv, ev, cl, time_game, time_frontend, time_backend);
    }
    if 0 != (*com_showtrace).integer {
        extern "C" {
            #[no_mangle]
            pub static mut c_traces: libc::c_int;
        }
        extern "C" {
            #[no_mangle]
            pub static mut c_brush_traces: libc::c_int;
        }
        extern "C" {
            #[no_mangle]
            pub static mut c_patch_traces: libc::c_int;
        }
        extern "C" {
            #[no_mangle]
            pub static mut c_pointcontents: libc::c_int;
        }
        Com_Printf(b"%4i traces  (%ib %ip) %4i points\n\x00" as *const u8 as
                       *const libc::c_char, c_traces, c_brush_traces,
                   c_patch_traces, c_pointcontents);
        c_traces = 0i32;
        c_brush_traces = 0i32;
        c_patch_traces = 0i32;
        c_pointcontents = 0i32
    }
    Com_ReadFromPipe();
    com_frameNumber += 1;
}
// com_speeds times
// renderer frontend time
// renderer backend time
#[no_mangle]
pub static mut com_frameNumber: libc::c_int = 0;
/*
===============
Com_ReadFromPipe

Read whatever is in com_pipefile, if anything, and execute it
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Com_ReadFromPipe() {
    static mut buf: [libc::c_char; 1024] = [0; 1024];
    static mut accu: libc::c_int = 0i32;
    let mut read: libc::c_int = 0;
    if 0 == pipefile { return }
    loop  {
        read =
            FS_Read(buf.as_mut_ptr().offset(accu as isize) as
                        *mut libc::c_void,
                    (::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong).wrapping_sub(accu as
                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                         as
                                                                                         libc::c_ulong)
                        as libc::c_int, pipefile);
        if !(read > 0i32) { break ; }
        let mut brk: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut i: libc::c_int = 0;
        i = accu;
        while i < accu + read {
            if buf[i as usize] as libc::c_int == '\u{0}' as i32 {
                buf[i as usize] = '\n' as i32 as libc::c_char
            }
            if buf[i as usize] as libc::c_int == '\n' as i32 ||
                   buf[i as usize] as libc::c_int == '\r' as i32 {
                brk = &mut buf[(i + 1i32) as usize] as *mut libc::c_char
            }
            i += 1
        }
        buf[(accu + read) as usize] = '\u{0}' as i32 as libc::c_char;
        accu += read;
        if !brk.is_null() {
            let mut tmp: libc::c_char = *brk;
            *brk = '\u{0}' as i32 as libc::c_char;
            Cbuf_ExecuteText(EXEC_APPEND as libc::c_int, buf.as_mut_ptr());
            *brk = tmp;
            accu =
                (accu as libc::c_long -
                     brk.wrapping_offset_from(buf.as_mut_ptr()) as
                         libc::c_long) as libc::c_int;
            memmove(buf.as_mut_ptr() as *mut libc::c_void,
                    brk as *const libc::c_void,
                    (accu + 1i32) as libc::c_ulong);
        } else if accu as libc::c_ulong >=
                      (::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong)
         {
            Cbuf_ExecuteText(EXEC_APPEND as libc::c_int, buf.as_mut_ptr());
            accu = 0i32
        }
    };
}
/*
================
Com_ModifyMsec
================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_ModifyMsec(mut msec: libc::c_int)
 -> libc::c_int {
    let mut clampTime: libc::c_int = 0;
    if 0 != (*com_fixedtime).integer {
        msec = (*com_fixedtime).integer
    } else if 0. != (*com_timescale).value {
        msec = (msec as libc::c_float * (*com_timescale).value) as libc::c_int
    } else if 0 != (*com_cameraMode).integer {
        msec = (msec as libc::c_float * (*com_timescale).value) as libc::c_int
    }
    if msec < 1i32 && 0. != (*com_timescale).value { msec = 1i32 }
    if 0 != (*com_dedicated).integer {
        if 0 != (*com_sv_running).integer && msec > 500i32 {
            Com_Printf(b"Hitch warning: %i msec frame time\n\x00" as *const u8
                           as *const libc::c_char, msec);
        }
        clampTime = 5000i32
    } else if 0 == (*com_sv_running).integer {
        clampTime = 5000i32
    } else { clampTime = 200i32 }
    if msec > clampTime { msec = clampTime }
    return msec;
}
/*
=================
Com_TimeVal
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_TimeVal(mut minMsec: libc::c_int)
 -> libc::c_int {
    let mut timeVal: libc::c_int = 0;
    timeVal = Sys_Milliseconds() - com_frameTime;
    if timeVal >= minMsec {
        timeVal = 0i32
    } else { timeVal = minMsec - timeVal }
    return timeVal;
}
/*
===============
Com_WriteConfiguration

Writes key bindings and archived cvars to config file if modified
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Com_WriteConfiguration() {
    if 0 == com_fullyInitialized as u64 { return }
    if 0 == cvar_modifiedFlags & 0x1i32 { return }
    cvar_modifiedFlags &= !0x1i32;
    Com_WriteConfigToFile(b"q3config_server.cfg\x00" as *const u8 as
                              *const libc::c_char);
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
// common.c -- misc functions used in client and server
// umask
#[no_mangle]
pub static mut com_argc: libc::c_int = 0;
#[no_mangle]
pub static mut com_argv: [*mut libc::c_char; 51] =
    [0 as *const libc::c_char as *mut libc::c_char; 51];
#[no_mangle]
pub static mut debuglogfile: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub unsafe extern "C" fn Com_ReadCDKey(mut filename: *const libc::c_char) {
    let mut f: fileHandle_t = 0;
    let mut buffer: [libc::c_char; 33] = [0; 33];
    let mut fbuffer: [libc::c_char; 4096] = [0; 4096];
    Com_sprintf(fbuffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%s/q3key\x00" as *const u8 as *const libc::c_char,
                filename);
    FS_SV_FOpenFileRead(fbuffer.as_mut_ptr(), &mut f);
    if 0 == f {
        Q_strncpyz(cl_cdkey.as_mut_ptr(),
                   b"                \x00" as *const u8 as
                       *const libc::c_char, 17i32);
        return
    }
    memset(buffer.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong);
    FS_Read(buffer.as_mut_ptr() as *mut libc::c_void, 16i32, f);
    FS_FCloseFile(f);
    if 0 !=
           CL_CDKeyValidate(buffer.as_mut_ptr(), 0 as *const libc::c_char) as
               u64 {
        Q_strncpyz(cl_cdkey.as_mut_ptr(), buffer.as_mut_ptr(), 17i32);
    } else {
        Q_strncpyz(cl_cdkey.as_mut_ptr(),
                   b"                \x00" as *const u8 as
                       *const libc::c_char, 17i32);
    };
}
/*
=================
Com_AppendCDKey
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Com_AppendCDKey(mut filename: *const libc::c_char) {
    let mut f: fileHandle_t = 0;
    let mut buffer: [libc::c_char; 33] = [0; 33];
    let mut fbuffer: [libc::c_char; 4096] = [0; 4096];
    Com_sprintf(fbuffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%s/q3key\x00" as *const u8 as *const libc::c_char,
                filename);
    FS_SV_FOpenFileRead(fbuffer.as_mut_ptr(), &mut f);
    if 0 == f {
        Q_strncpyz(&mut cl_cdkey[16usize],
                   b"                \x00" as *const u8 as
                       *const libc::c_char, 17i32);
        return
    }
    memset(buffer.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong);
    FS_Read(buffer.as_mut_ptr() as *mut libc::c_void, 16i32, f);
    FS_FCloseFile(f);
    if 0 !=
           CL_CDKeyValidate(buffer.as_mut_ptr(), 0 as *const libc::c_char) as
               u64 {
        strcat(&mut cl_cdkey[16usize], buffer.as_mut_ptr());
    } else {
        Q_strncpyz(&mut cl_cdkey[16usize],
                   b"                \x00" as *const u8 as
                       *const libc::c_char, 17i32);
    };
}
unsafe extern "C" fn run_static_initializers() {
    numberstring =
        [memstatic_s{b:
                         memblock_s{size:
                                        ((::std::mem::size_of::<memstatic_t>()
                                              as
                                              libc::c_ulong).wrapping_add(3i32
                                                                              as
                                                                              libc::c_ulong)
                                             & !3i32 as libc::c_ulong) as
                                            libc::c_int,
                                    tag: TAG_STATIC as libc::c_int,
                                    next: 0 as *mut memblock_s,
                                    prev: 0 as *mut memblock_s,
                                    id: 0x1d4a11i32,
                                    d:
                                        zonedebug_s{label:
                                                        0 as
                                                            *mut libc::c_char,
                                                    file:
                                                        0 as
                                                            *mut libc::c_char,
                                                    line: 0,
                                                    allocSize: 0,},},
                     mem: ['0' as i32 as byte, '\u{0}' as i32 as byte],},
         memstatic_s{b:
                         memblock_s{size:
                                        ((::std::mem::size_of::<memstatic_t>()
                                              as
                                              libc::c_ulong).wrapping_add(3i32
                                                                              as
                                                                              libc::c_ulong)
                                             & !3i32 as libc::c_ulong) as
                                            libc::c_int,
                                    tag: TAG_STATIC as libc::c_int,
                                    next: 0 as *mut memblock_s,
                                    prev: 0 as *mut memblock_s,
                                    id: 0x1d4a11i32,
                                    d:
                                        zonedebug_s{label:
                                                        0 as
                                                            *mut libc::c_char,
                                                    file:
                                                        0 as
                                                            *mut libc::c_char,
                                                    line: 0,
                                                    allocSize: 0,},},
                     mem: ['1' as i32 as byte, '\u{0}' as i32 as byte],},
         memstatic_s{b:
                         memblock_s{size:
                                        ((::std::mem::size_of::<memstatic_t>()
                                              as
                                              libc::c_ulong).wrapping_add(3i32
                                                                              as
                                                                              libc::c_ulong)
                                             & !3i32 as libc::c_ulong) as
                                            libc::c_int,
                                    tag: TAG_STATIC as libc::c_int,
                                    next: 0 as *mut memblock_s,
                                    prev: 0 as *mut memblock_s,
                                    id: 0x1d4a11i32,
                                    d:
                                        zonedebug_s{label:
                                                        0 as
                                                            *mut libc::c_char,
                                                    file:
                                                        0 as
                                                            *mut libc::c_char,
                                                    line: 0,
                                                    allocSize: 0,},},
                     mem: ['2' as i32 as byte, '\u{0}' as i32 as byte],},
         memstatic_s{b:
                         memblock_s{size:
                                        ((::std::mem::size_of::<memstatic_t>()
                                              as
                                              libc::c_ulong).wrapping_add(3i32
                                                                              as
                                                                              libc::c_ulong)
                                             & !3i32 as libc::c_ulong) as
                                            libc::c_int,
                                    tag: TAG_STATIC as libc::c_int,
                                    next: 0 as *mut memblock_s,
                                    prev: 0 as *mut memblock_s,
                                    id: 0x1d4a11i32,
                                    d:
                                        zonedebug_s{label:
                                                        0 as
                                                            *mut libc::c_char,
                                                    file:
                                                        0 as
                                                            *mut libc::c_char,
                                                    line: 0,
                                                    allocSize: 0,},},
                     mem: ['3' as i32 as byte, '\u{0}' as i32 as byte],},
         memstatic_s{b:
                         memblock_s{size:
                                        ((::std::mem::size_of::<memstatic_t>()
                                              as
                                              libc::c_ulong).wrapping_add(3i32
                                                                              as
                                                                              libc::c_ulong)
                                             & !3i32 as libc::c_ulong) as
                                            libc::c_int,
                                    tag: TAG_STATIC as libc::c_int,
                                    next: 0 as *mut memblock_s,
                                    prev: 0 as *mut memblock_s,
                                    id: 0x1d4a11i32,
                                    d:
                                        zonedebug_s{label:
                                                        0 as
                                                            *mut libc::c_char,
                                                    file:
                                                        0 as
                                                            *mut libc::c_char,
                                                    line: 0,
                                                    allocSize: 0,},},
                     mem: ['4' as i32 as byte, '\u{0}' as i32 as byte],},
         memstatic_s{b:
                         memblock_s{size:
                                        ((::std::mem::size_of::<memstatic_t>()
                                              as
                                              libc::c_ulong).wrapping_add(3i32
                                                                              as
                                                                              libc::c_ulong)
                                             & !3i32 as libc::c_ulong) as
                                            libc::c_int,
                                    tag: TAG_STATIC as libc::c_int,
                                    next: 0 as *mut memblock_s,
                                    prev: 0 as *mut memblock_s,
                                    id: 0x1d4a11i32,
                                    d:
                                        zonedebug_s{label:
                                                        0 as
                                                            *mut libc::c_char,
                                                    file:
                                                        0 as
                                                            *mut libc::c_char,
                                                    line: 0,
                                                    allocSize: 0,},},
                     mem: ['5' as i32 as byte, '\u{0}' as i32 as byte],},
         memstatic_s{b:
                         memblock_s{size:
                                        ((::std::mem::size_of::<memstatic_t>()
                                              as
                                              libc::c_ulong).wrapping_add(3i32
                                                                              as
                                                                              libc::c_ulong)
                                             & !3i32 as libc::c_ulong) as
                                            libc::c_int,
                                    tag: TAG_STATIC as libc::c_int,
                                    next: 0 as *mut memblock_s,
                                    prev: 0 as *mut memblock_s,
                                    id: 0x1d4a11i32,
                                    d:
                                        zonedebug_s{label:
                                                        0 as
                                                            *mut libc::c_char,
                                                    file:
                                                        0 as
                                                            *mut libc::c_char,
                                                    line: 0,
                                                    allocSize: 0,},},
                     mem: ['6' as i32 as byte, '\u{0}' as i32 as byte],},
         memstatic_s{b:
                         memblock_s{size:
                                        ((::std::mem::size_of::<memstatic_t>()
                                              as
                                              libc::c_ulong).wrapping_add(3i32
                                                                              as
                                                                              libc::c_ulong)
                                             & !3i32 as libc::c_ulong) as
                                            libc::c_int,
                                    tag: TAG_STATIC as libc::c_int,
                                    next: 0 as *mut memblock_s,
                                    prev: 0 as *mut memblock_s,
                                    id: 0x1d4a11i32,
                                    d:
                                        zonedebug_s{label:
                                                        0 as
                                                            *mut libc::c_char,
                                                    file:
                                                        0 as
                                                            *mut libc::c_char,
                                                    line: 0,
                                                    allocSize: 0,},},
                     mem: ['7' as i32 as byte, '\u{0}' as i32 as byte],},
         memstatic_s{b:
                         memblock_s{size:
                                        ((::std::mem::size_of::<memstatic_t>()
                                              as
                                              libc::c_ulong).wrapping_add(3i32
                                                                              as
                                                                              libc::c_ulong)
                                             & !3i32 as libc::c_ulong) as
                                            libc::c_int,
                                    tag: TAG_STATIC as libc::c_int,
                                    next: 0 as *mut memblock_s,
                                    prev: 0 as *mut memblock_s,
                                    id: 0x1d4a11i32,
                                    d:
                                        zonedebug_s{label:
                                                        0 as
                                                            *mut libc::c_char,
                                                    file:
                                                        0 as
                                                            *mut libc::c_char,
                                                    line: 0,
                                                    allocSize: 0,},},
                     mem: ['8' as i32 as byte, '\u{0}' as i32 as byte],},
         memstatic_s{b:
                         memblock_s{size:
                                        ((::std::mem::size_of::<memstatic_t>()
                                              as
                                              libc::c_ulong).wrapping_add(3i32
                                                                              as
                                                                              libc::c_ulong)
                                             & !3i32 as libc::c_ulong) as
                                            libc::c_int,
                                    tag: TAG_STATIC as libc::c_int,
                                    next: 0 as *mut memblock_s,
                                    prev: 0 as *mut memblock_s,
                                    id: 0x1d4a11i32,
                                    d:
                                        zonedebug_s{label:
                                                        0 as
                                                            *mut libc::c_char,
                                                    file:
                                                        0 as
                                                            *mut libc::c_char,
                                                    line: 0,
                                                    allocSize: 0,},},
                     mem: ['9' as i32 as byte, '\u{0}' as i32 as byte],}];
    emptystring =
        memstatic_s{b:
                        memblock_s{size:
                                       ((::std::mem::size_of::<memblock_t>()
                                             as
                                             libc::c_ulong).wrapping_add(2i32
                                                                             as
                                                                             libc::c_ulong).wrapping_add(3i32
                                                                                                             as
                                                                                                             libc::c_ulong)
                                            & !3i32 as libc::c_ulong) as
                                           libc::c_int,
                                   tag: TAG_STATIC as libc::c_int,
                                   next: 0 as *mut memblock_s,
                                   prev: 0 as *mut memblock_s,
                                   id: 0x1d4a11i32,
                                   d:
                                       zonedebug_s{label:
                                                       0 as *mut libc::c_char,
                                                   file:
                                                       0 as *mut libc::c_char,
                                                   line: 0,
                                                   allocSize: 0,},},
                    mem: ['\u{0}' as i32 as byte, '\u{0}' as i32 as byte],}
}
#[used]
#[cfg_attr ( target_os = "linux" , link_section = ".init_array" )]
#[cfg_attr ( target_os = "windows" , link_section = ".CRT$XIB" )]
#[cfg_attr ( target_os = "macos" , link_section = "__DATA,__mod_init_func" )]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];