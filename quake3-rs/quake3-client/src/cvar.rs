#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           label_break_value,
           libc,
           ptr_wrapping_offset_from)]
extern crate libc;
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    pub type fileHandle_t = libc::c_int;
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
    pub type cvarHandle_t = libc::c_int;
    // the modules that run in the virtual machine can't access the cvar_t directly,
// so they must ask for structured updates
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vmCvar_t {
        pub handle: cvarHandle_t,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub string: [libc::c_char; 256],
    }
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        #[no_mangle]
        pub fn Com_SkipTokens(s: *mut libc::c_char, numTokens: libc::c_int,
                              sep: *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Q_isanumber(s: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn Q_isintegral(f: libc::c_float) -> qboolean;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
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
        pub fn Info_SetValueForKey(s: *mut libc::c_char,
                                   key: *const libc::c_char,
                                   value: *const libc::c_char);
        #[no_mangle]
        pub fn Info_SetValueForKey_Big(s: *mut libc::c_char,
                                       key: *const libc::c_char,
                                       value: *const libc::c_char);
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
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
    pub type completionFunc_t
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
                   -> ()>;
    use super::{libc};
    use super::q_shared_h::{cvar_t, qboolean, vmCvar_t, fileHandle_t};
    extern "C" {
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
        #[no_mangle]
        pub fn Cmd_Args() -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cmd_ArgsFrom(arg: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn CopyString(in_0: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub static mut com_errorEntered: qboolean;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        // returns 1 if a file is in the PAK file, otherwise -1
        #[no_mangle]
        pub fn FS_Write(buffer: *const libc::c_void, len: libc::c_int,
                        f: fileHandle_t) -> libc::c_int;
        #[no_mangle]
        pub fn Com_Filter(filter: *mut libc::c_char, name: *mut libc::c_char,
                          casesensitive: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Field_CompleteCommand(cmd: *mut libc::c_char,
                                     doCommands: qboolean, doCvars: qboolean);
    }
}
#[header_src = "/usr/include/assert.h"]
pub mod assert_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
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
        pub fn atof(__nptr: *const libc::c_char) -> libc::c_double;
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
}
#[header_src = "/usr/include/ctype.h"]
pub mod ctype_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/qcommon/cvar.c"]
pub mod cvar_c {
    use super::q_shared_h::{cvar_t};
    use super::{libc};
}
use self::q_shared_h::{qboolean, qtrue, qfalse, fileHandle_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, cvar_s, cvar_t, cvarHandle_t,
                       vmCvar_t, Com_sprintf, Com_SkipTokens, Q_isanumber,
                       Q_isintegral, Q_stricmp, Q_strncpyz, va,
                       Info_SetValueForKey, Info_SetValueForKey_Big,
                       Com_Error, Com_Printf};
use self::qcommon_h::{xcommand_t, completionFunc_t, Cmd_AddCommand,
                      Cmd_SetCommandCompletionFunc, Cmd_Argc, Cmd_Argv,
                      Cmd_Args, Cmd_ArgsFrom, CopyString, com_errorEntered,
                      Z_Free, Com_DPrintf, FS_Write, Com_Filter,
                      Field_CompleteCommand};
use self::assert_h::{__assert_fail};
use self::string_h::{memset, strcmp, strchr, strlen};
use self::stdlib_h::{atof, atoi};
use self::ctype_h::{tolower};
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
pub unsafe extern "C" fn Cvar_Get(mut var_name: *const libc::c_char,
                                  mut var_value: *const libc::c_char,
                                  mut flags: libc::c_int) -> *mut cvar_t {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    let mut hash: libc::c_long = 0;
    let mut index: libc::c_int = 0;
    if var_name.is_null() || var_value.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Cvar_Get: NULL parameter\x00" as *const u8 as
                      *const libc::c_char);
    }
    if 0 == Cvar_ValidateString(var_name) as u64 {
        Com_Printf(b"invalid cvar name string: %s\n\x00" as *const u8 as
                       *const libc::c_char, var_name);
        var_name = b"BADNAME\x00" as *const u8 as *const libc::c_char
    }
    var = Cvar_FindVar(var_name);
    if !var.is_null() {
        var_value = Cvar_Validate(var, var_value, qfalse);
        if 0 != (*var).flags & 0x1000i32 {
            if 0 == flags & 0x1000i32 { (*var).flags &= !0x1000i32 }
        } else if 0 == (*var).flags & 0x80i32 {
            if 0 != flags & 0x1000i32 { flags &= !0x1000i32 }
        }
        if 0 != (*var).flags & 0x80i32 {
            (*var).flags &= !0x80i32;
            Z_Free((*var).resetString as *mut libc::c_void);
            (*var).resetString = CopyString(var_value);
            if 0 != flags & 0x40i32 {
                if !(*var).latchedString.is_null() {
                    Z_Free((*var).latchedString as *mut libc::c_void);
                }
                (*var).latchedString = CopyString(var_value)
            }
        }
        if 0 != (*var).flags & 0x800i32 {
            if 0 == flags & 0x800i32 { (*var).flags &= !0x800i32 }
        } else if 0 != flags & 0x800i32 { flags &= !0x800i32 }
        (*var).flags |= flags;
        if 0 == *(*var).resetString.offset(0isize) {
            Z_Free((*var).resetString as *mut libc::c_void);
            (*var).resetString = CopyString(var_value)
        } else if 0 != *var_value.offset(0isize) as libc::c_int &&
                      0 != strcmp((*var).resetString, var_value) {
            Com_DPrintf(b"Warning: cvar \"%s\" given initial values: \"%s\" and \"%s\"\n\x00"
                            as *const u8 as *const libc::c_char, var_name,
                        (*var).resetString, var_value);
        }
        if !(*var).latchedString.is_null() {
            let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
            s = (*var).latchedString;
            (*var).latchedString = 0 as *mut libc::c_char;
            Cvar_Set2(var_name, s, qtrue);
            Z_Free(s as *mut libc::c_void);
        }
        cvar_modifiedFlags |= flags;
        return var
    }
    index = 0i32;
    while index < 2048i32 {
        if cvar_indexes[index as usize].name.is_null() { break ; }
        index += 1
    }
    if index >= 2048i32 {
        if 0 == com_errorEntered as u64 {
            Com_Error(ERR_FATAL as libc::c_int,
                      b"Error: Too many cvars, cannot create a new one!\x00"
                          as *const u8 as *const libc::c_char);
        }
        return 0 as *mut cvar_t
    }
    var =
        &mut *cvar_indexes.as_mut_ptr().offset(index as isize) as *mut cvar_t;
    if index >= cvar_numIndexes { cvar_numIndexes = index + 1i32 }
    (*var).name = CopyString(var_name);
    (*var).string = CopyString(var_value);
    (*var).modified = qtrue;
    (*var).modificationCount = 1i32;
    (*var).value = atof((*var).string) as libc::c_float;
    (*var).integer = atoi((*var).string);
    (*var).resetString = CopyString(var_value);
    (*var).validate = qfalse;
    (*var).description = 0 as *mut libc::c_char;
    (*var).next = cvar_vars;
    if !cvar_vars.is_null() { (*cvar_vars).prev = var }
    (*var).prev = 0 as *mut cvar_t;
    cvar_vars = var;
    (*var).flags = flags;
    cvar_modifiedFlags |= (*var).flags;
    hash = generateHashValue(var_name);
    (*var).hashIndex = hash as libc::c_int;
    (*var).hashNext = hashTable[hash as usize];
    if !hashTable[hash as usize].is_null() {
        (*hashTable[hash as usize]).hashPrev = var
    }
    (*var).hashPrev = 0 as *mut cvar_t;
    hashTable[hash as usize] = var;
    return var;
}
static mut hashTable: [*mut cvar_t; 256] =
    [0 as *const cvar_t as *mut cvar_t; 256];
/*
================
return a hash value for the filename
================
*/
unsafe extern "C" fn generateHashValue(mut fname: *const libc::c_char)
 -> libc::c_long {
    let mut i: libc::c_int = 0;
    let mut hash: libc::c_long = 0;
    let mut letter: libc::c_char = 0;
    hash = 0i32 as libc::c_long;
    i = 0i32;
    while *fname.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        letter =
            tolower(*fname.offset(i as isize) as libc::c_int) as libc::c_char;
        hash += letter as libc::c_long * (i + 119i32) as libc::c_long;
        i += 1
    }
    hash &= (256i32 - 1i32) as libc::c_long;
    return hash;
}
#[no_mangle]
pub static mut cvar_modifiedFlags: libc::c_int = 0;
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
// cvar.c -- dynamic variable tracking
#[no_mangle]
pub static mut cvar_vars: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cvar_numIndexes: libc::c_int = 0;
#[no_mangle]
pub static mut cvar_indexes: [cvar_t; 2048] =
    [cvar_s{name: 0 as *const libc::c_char as *mut libc::c_char,
            string: 0 as *const libc::c_char as *mut libc::c_char,
            resetString: 0 as *const libc::c_char as *mut libc::c_char,
            latchedString: 0 as *const libc::c_char as *mut libc::c_char,
            flags: 0,
            modified: qfalse,
            modificationCount: 0,
            value: 0.,
            integer: 0,
            validate: qfalse,
            integral: qfalse,
            min: 0.,
            max: 0.,
            description: 0 as *const libc::c_char as *mut libc::c_char,
            next: 0 as *const cvar_t as *mut cvar_t,
            prev: 0 as *const cvar_t as *mut cvar_t,
            hashNext: 0 as *const cvar_t as *mut cvar_t,
            hashPrev: 0 as *const cvar_t as *mut cvar_t,
            hashIndex: 0,}; 2048];
// will create the variable with no flags if it doesn't exist
#[no_mangle]
pub unsafe extern "C" fn Cvar_Set2(mut var_name: *const libc::c_char,
                                   mut value: *const libc::c_char,
                                   mut force: qboolean) -> *mut cvar_t {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    if 0 == Cvar_ValidateString(var_name) as u64 {
        Com_Printf(b"invalid cvar name string: %s\n\x00" as *const u8 as
                       *const libc::c_char, var_name);
        var_name = b"BADNAME\x00" as *const u8 as *const libc::c_char
    }
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        if value.is_null() { return 0 as *mut cvar_t }
        if 0 == force as u64 {
            return Cvar_Get(var_name, value, 0x80i32)
        } else { return Cvar_Get(var_name, value, 0i32) }
    }
    if value.is_null() { value = (*var).resetString }
    value = Cvar_Validate(var, value, qtrue);
    if 0 != (*var).flags & 0x20i32 && !(*var).latchedString.is_null() {
        if 0 == strcmp(value, (*var).string) {
            Z_Free((*var).latchedString as *mut libc::c_void);
            (*var).latchedString = 0 as *mut libc::c_char;
            return var
        }
        if 0 == strcmp(value, (*var).latchedString) { return var }
    } else if 0 == strcmp(value, (*var).string) { return var }
    cvar_modifiedFlags |= (*var).flags;
    if 0 == force as u64 {
        if 0 != (*var).flags & 0x40i32 {
            Com_Printf(b"%s is read only.\n\x00" as *const u8 as
                           *const libc::c_char, var_name);
            return var
        }
        if 0 != (*var).flags & 0x10i32 {
            Com_Printf(b"%s is write protected.\n\x00" as *const u8 as
                           *const libc::c_char, var_name);
            return var
        }
        if 0 != (*var).flags & 0x200i32 && 0 == (*cvar_cheats).integer {
            Com_Printf(b"%s is cheat protected.\n\x00" as *const u8 as
                           *const libc::c_char, var_name);
            return var
        }
        if 0 != (*var).flags & 0x20i32 {
            if !(*var).latchedString.is_null() {
                if strcmp(value, (*var).latchedString) == 0i32 { return var }
                Z_Free((*var).latchedString as *mut libc::c_void);
            } else if strcmp(value, (*var).string) == 0i32 { return var }
            Com_Printf(b"%s will be changed upon restarting.\n\x00" as
                           *const u8 as *const libc::c_char, var_name);
            (*var).latchedString = CopyString(value);
            (*var).modified = qtrue;
            (*var).modificationCount += 1;
            return var
        }
    } else if !(*var).latchedString.is_null() {
        Z_Free((*var).latchedString as *mut libc::c_void);
        (*var).latchedString = 0 as *mut libc::c_char
    }
    if 0 == strcmp(value, (*var).string) { return var }
    (*var).modified = qtrue;
    (*var).modificationCount += 1;
    Z_Free((*var).string as *mut libc::c_void);
    (*var).string = CopyString(value);
    (*var).value = atof((*var).string) as libc::c_float;
    (*var).integer = atoi((*var).string);
    return var;
}
#[no_mangle]
pub static mut cvar_cheats: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
============
Cvar_Validate
============
*/
unsafe extern "C" fn Cvar_Validate(mut var: *mut cvar_t,
                                   mut value: *const libc::c_char,
                                   mut warn: qboolean)
 -> *const libc::c_char {
    static mut s: [libc::c_char; 256] = [0; 256];
    let mut valuef: libc::c_float = 0.;
    let mut changed: qboolean = qfalse;
    if 0 == (*var).validate as u64 { return value }
    if value.is_null() { return value }
    if 0 != Q_isanumber(value) as u64 {
        valuef = atof(value) as libc::c_float;
        if 0 != (*var).integral as u64 {
            if 0 == Q_isintegral(valuef) as u64 {
                if 0 != warn as u64 {
                    Com_Printf(b"WARNING: cvar \'%s\' must be integral\x00" as
                                   *const u8 as *const libc::c_char,
                               (*var).name);
                }
                valuef = valuef as libc::c_int as libc::c_float;
                changed = qtrue
            }
        }
    } else {
        if 0 != warn as u64 {
            Com_Printf(b"WARNING: cvar \'%s\' must be numeric\x00" as
                           *const u8 as *const libc::c_char, (*var).name);
        }
        valuef = atof((*var).resetString) as libc::c_float;
        changed = qtrue
    }
    if valuef < (*var).min {
        if 0 != warn as u64 {
            if 0 != changed as u64 {
                Com_Printf(b" and is\x00" as *const u8 as
                               *const libc::c_char);
            } else {
                Com_Printf(b"WARNING: cvar \'%s\'\x00" as *const u8 as
                               *const libc::c_char, (*var).name);
            }
            if 0 != Q_isintegral((*var).min) as u64 {
                Com_Printf(b" out of range (min %d)\x00" as *const u8 as
                               *const libc::c_char,
                           (*var).min as libc::c_int);
            } else {
                Com_Printf(b" out of range (min %f)\x00" as *const u8 as
                               *const libc::c_char,
                           (*var).min as libc::c_double);
            }
        }
        valuef = (*var).min;
        changed = qtrue
    } else if valuef > (*var).max {
        if 0 != warn as u64 {
            if 0 != changed as u64 {
                Com_Printf(b" and is\x00" as *const u8 as
                               *const libc::c_char);
            } else {
                Com_Printf(b"WARNING: cvar \'%s\'\x00" as *const u8 as
                               *const libc::c_char, (*var).name);
            }
            if 0 != Q_isintegral((*var).max) as u64 {
                Com_Printf(b" out of range (max %d)\x00" as *const u8 as
                               *const libc::c_char,
                           (*var).max as libc::c_int);
            } else {
                Com_Printf(b" out of range (max %f)\x00" as *const u8 as
                               *const libc::c_char,
                           (*var).max as libc::c_double);
            }
        }
        valuef = (*var).max;
        changed = qtrue
    }
    if 0 != changed as u64 {
        if 0 != Q_isintegral(valuef) as u64 {
            Com_sprintf(s.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as
                            libc::c_ulong as libc::c_int,
                        b"%d\x00" as *const u8 as *const libc::c_char,
                        valuef as libc::c_int);
            if 0 != warn as u64 {
                Com_Printf(b", setting to %d\n\x00" as *const u8 as
                               *const libc::c_char, valuef as libc::c_int);
            }
        } else {
            Com_sprintf(s.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 256]>() as
                            libc::c_ulong as libc::c_int,
                        b"%f\x00" as *const u8 as *const libc::c_char,
                        valuef as libc::c_double);
            if 0 != warn as u64 {
                Com_Printf(b", setting to %f\n\x00" as *const u8 as
                               *const libc::c_char, valuef as libc::c_double);
            }
        }
        return s.as_mut_ptr()
    } else { return value };
}
/*
============
Cvar_FindVar
============
*/
unsafe extern "C" fn Cvar_FindVar(mut var_name: *const libc::c_char)
 -> *mut cvar_t {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    let mut hash: libc::c_long = 0;
    hash = generateHashValue(var_name);
    var = hashTable[hash as usize];
    while !var.is_null() {
        if 0 == Q_stricmp(var_name, (*var).name) { return var }
        var = (*var).hashNext
    }
    return 0 as *mut cvar_t;
}
/*
============
Cvar_ValidateString
============
*/
unsafe extern "C" fn Cvar_ValidateString(mut s: *const libc::c_char)
 -> qboolean {
    if s.is_null() { return qfalse }
    if !strchr(s, '\\' as i32).is_null() { return qfalse }
    if !strchr(s, '\"' as i32).is_null() { return qfalse }
    if !strchr(s, ';' as i32).is_null() { return qfalse }
    return qtrue;
}
// creates the variable if it doesn't exist, or returns the existing one
// if it exists, the value will not be changed, but flags will be ORed in
// that allows variables to be unarchived without needing bitflags
// if value is "", the value will not override a previously set value.
#[no_mangle]
pub unsafe extern "C" fn Cvar_Register(mut vmCvar: *mut vmCvar_t,
                                       mut varName: *const libc::c_char,
                                       mut defaultValue: *const libc::c_char,
                                       mut flags: libc::c_int) {
    let mut cv: *mut cvar_t = 0 as *mut cvar_t;
    if flags & (0x1i32 | 0x40i32) == 0x1i32 | 0x40i32 {
        Com_DPrintf(b"^3WARNING: Unsetting CVAR_ROM from cvar \'%s\', since it is also CVAR_ARCHIVE\n\x00"
                        as *const u8 as *const libc::c_char, varName);
        flags &= !0x40i32
    }
    if 0 != flags & 0x80i32 {
        Com_DPrintf(b"^3WARNING: VM tried to set CVAR_USER_CREATED on cvar \'%s\'\n\x00"
                        as *const u8 as *const libc::c_char, varName);
        flags &= !0x80i32
    }
    if 0 != flags & 0x800i32 {
        Com_DPrintf(b"^3WARNING: VM tried to set CVAR_SERVER_CREATED on cvar \'%s\'\n\x00"
                        as *const u8 as *const libc::c_char, varName);
        flags &= !0x800i32
    }
    if 0 != flags & 0x2000i32 {
        Com_DPrintf(b"^3WARNING: VM tried to set CVAR_PROTECTED on cvar \'%s\'\n\x00"
                        as *const u8 as *const libc::c_char, varName);
        flags &= !0x2000i32
    }
    if 0 != flags & 0x40000000i32 {
        Com_DPrintf(b"^3WARNING: VM tried to set CVAR_MODIFIED on cvar \'%s\'\n\x00"
                        as *const u8 as *const libc::c_char, varName);
        flags &= !0x40000000i32
    }
    if 0 != flags as libc::c_uint & 0x80000000u32 {
        Com_DPrintf(b"^3WARNING: VM tried to set CVAR_NONEXISTENT on cvar \'%s\'\n\x00"
                        as *const u8 as *const libc::c_char, varName);
        flags = (flags as libc::c_uint & !0x80000000u32) as libc::c_int
    }
    cv = Cvar_FindVar(varName);
    if !cv.is_null() && 0 != (*cv).flags & 0x2000i32 {
        Com_DPrintf(b"^3WARNING: VM tried to register protected cvar \'%s\' with value \'%s\'%s\n\x00"
                        as *const u8 as *const libc::c_char, varName,
                    defaultValue,
                    if flags & !(*cv).flags != 0i32 {
                        b" and new flags\x00" as *const u8 as
                            *const libc::c_char
                    } else { b"\x00" as *const u8 as *const libc::c_char });
    } else { cv = Cvar_Get(varName, defaultValue, flags | 0x1000i32) }
    if vmCvar.is_null() { return }
    (*vmCvar).handle =
        cv.wrapping_offset_from(cvar_indexes.as_mut_ptr()) as libc::c_long as
            cvarHandle_t;
    (*vmCvar).modificationCount = -1i32;
    Cvar_Update(vmCvar);
}
// basically a slightly modified Cvar_Get for the interpreted modules
#[no_mangle]
pub unsafe extern "C" fn Cvar_Update(mut vmCvar: *mut vmCvar_t) {
    let mut cv: *mut cvar_t = 0 as *mut cvar_t;
    if !vmCvar.is_null() {
    } else {
        __assert_fail(b"vmCvar\x00" as *const u8 as *const libc::c_char,
                      b"code/qcommon/cvar.c\x00" as *const u8 as
                          *const libc::c_char, 1389i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 29],
                                                &[libc::c_char; 29]>(b"void Cvar_Update(vmCvar_t *)\x00")).as_ptr());
    }
    if (*vmCvar).handle as libc::c_uint >= cvar_numIndexes as libc::c_uint {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Cvar_Update: handle out of range\x00" as *const u8 as
                      *const libc::c_char);
    }
    cv = cvar_indexes.as_mut_ptr().offset((*vmCvar).handle as isize);
    if (*cv).modificationCount == (*vmCvar).modificationCount { return }
    if (*cv).string.is_null() { return }
    (*vmCvar).modificationCount = (*cv).modificationCount;
    if strlen((*cv).string).wrapping_add(1i32 as libc::c_ulong) >
           256i32 as libc::c_ulong {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Cvar_Update: src %s length %u exceeds MAX_CVAR_VALUE_STRING\x00"
                      as *const u8 as *const libc::c_char, (*cv).string,
                  strlen((*cv).string) as libc::c_uint);
    }
    Q_strncpyz((*vmCvar).string.as_mut_ptr(), (*cv).string, 256i32);
    (*vmCvar).value = (*cv).value;
    (*vmCvar).integer = (*cv).integer;
}
// updates an interpreted modules' version of a cvar
#[no_mangle]
pub unsafe extern "C" fn Cvar_Set(mut var_name: *const libc::c_char,
                                  mut value: *const libc::c_char) {
    Cvar_Set2(var_name, value, qtrue);
}
// same as Cvar_Set, but allows more control over setting of cvar
#[no_mangle]
pub unsafe extern "C" fn Cvar_SetSafe(mut var_name: *const libc::c_char,
                                      mut value: *const libc::c_char) {
    let mut flags: libc::c_int = Cvar_Flags(var_name);
    if flags as libc::c_uint != 0x80000000u32 && 0 != flags & 0x2000i32 {
        if !value.is_null() {
            Com_Error(ERR_DROP as libc::c_int,
                      b"Restricted source tried to set \"%s\" to \"%s\"\x00"
                          as *const u8 as *const libc::c_char, var_name,
                      value);
        } else {
            Com_Error(ERR_DROP as libc::c_int,
                      b"Restricted source tried to modify \"%s\"\x00" as
                          *const u8 as *const libc::c_char, var_name);
        }
    }
    Cvar_Set(var_name, value);
}
// returns an empty string if not defined
#[no_mangle]
pub unsafe extern "C" fn Cvar_Flags(mut var_name: *const libc::c_char)
 -> libc::c_int {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        return 0x80000000u32 as libc::c_int
    } else if 0 != (*var).modified as u64 {
        return (*var).flags | 0x40000000i32
    } else { return (*var).flags };
}
// sometimes we set variables from an untrusted source: fail if flags & CVAR_PROTECTED
#[no_mangle]
pub unsafe extern "C" fn Cvar_SetLatched(mut var_name: *const libc::c_char,
                                         mut value: *const libc::c_char) {
    Cvar_Set2(var_name, value, qfalse);
}
// don't set the cvar immediately
#[no_mangle]
pub unsafe extern "C" fn Cvar_SetValue(mut var_name: *const libc::c_char,
                                       mut value: libc::c_float) {
    let mut val: [libc::c_char; 32] = [0; 32];
    if value == value as libc::c_int as libc::c_float {
        Com_sprintf(val.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 32]>() as
                        libc::c_ulong as libc::c_int,
                    b"%i\x00" as *const u8 as *const libc::c_char,
                    value as libc::c_int);
    } else {
        Com_sprintf(val.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 32]>() as
                        libc::c_ulong as libc::c_int,
                    b"%f\x00" as *const u8 as *const libc::c_char,
                    value as libc::c_double);
    }
    Cvar_Set(var_name, val.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_SetValueSafe(mut var_name: *const libc::c_char,
                                           mut value: libc::c_float) {
    let mut val: [libc::c_char; 32] = [0; 32];
    if 0 != Q_isintegral(value) as u64 {
        Com_sprintf(val.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 32]>() as
                        libc::c_ulong as libc::c_int,
                    b"%i\x00" as *const u8 as *const libc::c_char,
                    value as libc::c_int);
    } else {
        Com_sprintf(val.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 32]>() as
                        libc::c_ulong as libc::c_int,
                    b"%f\x00" as *const u8 as *const libc::c_char,
                    value as libc::c_double);
    }
    Cvar_SetSafe(var_name, val.as_mut_ptr());
}
// expands value to a string and calls Cvar_Set/Cvar_SetSafe
#[no_mangle]
pub unsafe extern "C" fn Cvar_VariableValue(mut var_name: *const libc::c_char)
 -> libc::c_float {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() { return 0i32 as libc::c_float }
    return (*var).value;
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_VariableIntegerValue(mut var_name:
                                                       *const libc::c_char)
 -> libc::c_int {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() { return 0i32 }
    return (*var).integer;
}
// returns 0 if not defined or non numeric
#[no_mangle]
pub unsafe extern "C" fn Cvar_VariableString(mut var_name:
                                                 *const libc::c_char)
 -> *mut libc::c_char {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        return b"\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    return (*var).string;
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_VariableStringBuffer(mut var_name:
                                                       *const libc::c_char,
                                                   mut buffer:
                                                       *mut libc::c_char,
                                                   mut bufsize: libc::c_int) {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        *buffer = 0i32 as libc::c_char
    } else { Q_strncpyz(buffer, (*var).string, bufsize); };
}
// returns CVAR_NONEXISTENT if cvar doesn't exist or the flags of that particular CVAR.
#[no_mangle]
pub unsafe extern "C" fn Cvar_CommandCompletion(mut callback:
                                                    Option<unsafe extern "C" fn(_:
                                                                                    *const libc::c_char)
                                                               -> ()>) {
    let mut cvar: *mut cvar_t = 0 as *mut cvar_t;
    cvar = cvar_vars;
    while !cvar.is_null() {
        if !(*cvar).name.is_null() {
            callback.expect("non-null function pointer")((*cvar).name);
        }
        cvar = (*cvar).next
    };
}
// callback with each valid string
#[no_mangle]
pub unsafe extern "C" fn Cvar_Reset(mut var_name: *const libc::c_char) {
    Cvar_Set2(var_name, 0 as *const libc::c_char, qfalse);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_ForceReset(mut var_name: *const libc::c_char) {
    Cvar_Set2(var_name, 0 as *const libc::c_char, qtrue);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_SetCheatState() {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = cvar_vars;
    while !var.is_null() {
        if 0 != (*var).flags & 0x200i32 {
            if !(*var).latchedString.is_null() {
                Z_Free((*var).latchedString as *mut libc::c_void);
                (*var).latchedString = 0 as *mut libc::c_char
            }
            if 0 != strcmp((*var).resetString, (*var).string) {
                Cvar_Set((*var).name, (*var).resetString);
            }
        }
        var = (*var).next
    };
}
// reset all testing vars to a safe value
#[no_mangle]
pub unsafe extern "C" fn Cvar_Command() -> qboolean {
    let mut v: *mut cvar_t = 0 as *mut cvar_t;
    v = Cvar_FindVar(Cmd_Argv(0i32));
    if v.is_null() { return qfalse }
    if Cmd_Argc() == 1i32 { Cvar_Print(v); return qtrue }
    Cvar_Set2((*v).name, Cmd_Args(), qfalse);
    return qtrue;
}
/*
============
Cvar_Print

Prints the value, default, and latched string of the given variable
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Print(mut v: *mut cvar_t) {
    Com_Printf(b"\"%s\" is:\"%s^7\"\x00" as *const u8 as *const libc::c_char,
               (*v).name, (*v).string);
    if 0 == (*v).flags & 0x40i32 {
        if 0 == Q_stricmp((*v).string, (*v).resetString) {
            Com_Printf(b", the default\x00" as *const u8 as
                           *const libc::c_char);
        } else {
            Com_Printf(b" default:\"%s^7\"\x00" as *const u8 as
                           *const libc::c_char, (*v).resetString);
        }
    }
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    if !(*v).latchedString.is_null() {
        Com_Printf(b"latched: \"%s\"\n\x00" as *const u8 as
                       *const libc::c_char, (*v).latchedString);
    }
    if !(*v).description.is_null() {
        Com_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                   (*v).description);
    };
}
// called by Cmd_ExecuteString when Cmd_Argv(0) doesn't match a known
// command.  Returns true if the command was a variable reference that
// was handled. (print or change)
#[no_mangle]
pub unsafe extern "C" fn Cvar_WriteVariables(mut f: fileHandle_t) {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut current_block_5: u64;
    var = cvar_vars;
    while !var.is_null() {
        if !((*var).name.is_null() ||
                 Q_stricmp((*var).name,
                           b"cl_cdkey\x00" as *const u8 as
                               *const libc::c_char) == 0i32) {
            if 0 != (*var).flags & 0x1i32 {
                // write the latched value, even if it hasn't taken effect yet
                if !(*var).latchedString.is_null() {
                    if strlen((*var).name).wrapping_add(strlen((*var).latchedString)).wrapping_add(10i32
                                                                                                       as
                                                                                                       libc::c_ulong)
                           >
                           ::std::mem::size_of::<[libc::c_char; 1024]>() as
                               libc::c_ulong {
                        Com_Printf(b"^3WARNING: value of variable \"%s\" too long to write to file\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   (*var).name);
                        current_block_5 = 16559507199688588974;
                    } else {
                        Com_sprintf(buffer.as_mut_ptr(),
                                    ::std::mem::size_of::<[libc::c_char; 1024]>()
                                        as libc::c_ulong as libc::c_int,
                                    b"seta %s \"%s\"\n\x00" as *const u8 as
                                        *const libc::c_char, (*var).name,
                                    (*var).latchedString);
                        current_block_5 = 12599329904712511516;
                    }
                } else if strlen((*var).name).wrapping_add(strlen((*var).string)).wrapping_add(10i32
                                                                                                   as
                                                                                                   libc::c_ulong)
                              >
                              ::std::mem::size_of::<[libc::c_char; 1024]>() as
                                  libc::c_ulong {
                    Com_Printf(b"^3WARNING: value of variable \"%s\" too long to write to file\n\x00"
                                   as *const u8 as *const libc::c_char,
                               (*var).name);
                    current_block_5 = 16559507199688588974;
                } else {
                    Com_sprintf(buffer.as_mut_ptr(),
                                ::std::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong as libc::c_int,
                                b"seta %s \"%s\"\n\x00" as *const u8 as
                                    *const libc::c_char, (*var).name,
                                (*var).string);
                    current_block_5 = 12599329904712511516;
                }
                match current_block_5 {
                    16559507199688588974 => { }
                    _ => {
                        FS_Write(buffer.as_mut_ptr() as *const libc::c_void,
                                 strlen(buffer.as_mut_ptr()) as libc::c_int,
                                 f);
                    }
                }
            }
        }
        var = (*var).next
    };
}
// writes lines containing "set variable value" for all variables
// with the archive flag set to true.
#[no_mangle]
pub unsafe extern "C" fn Cvar_Init() {
    memset(cvar_indexes.as_mut_ptr() as *mut libc::c_void, '\u{0}' as i32,
           ::std::mem::size_of::<[cvar_t; 2048]>() as libc::c_ulong);
    memset(hashTable.as_mut_ptr() as *mut libc::c_void, '\u{0}' as i32,
           ::std::mem::size_of::<[*mut cvar_t; 256]>() as libc::c_ulong);
    cvar_cheats =
        Cvar_Get(b"sv_cheats\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 0x40i32 | 0x8i32);
    Cmd_AddCommand(b"print\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_Print_f));
    Cmd_AddCommand(b"toggle\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_Toggle_f));
    Cmd_SetCommandCompletionFunc(b"toggle\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cvar_CompleteCvarName));
    Cmd_AddCommand(b"set\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_Set_f));
    Cmd_SetCommandCompletionFunc(b"set\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cvar_CompleteCvarName));
    Cmd_AddCommand(b"sets\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_Set_f));
    Cmd_SetCommandCompletionFunc(b"sets\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cvar_CompleteCvarName));
    Cmd_AddCommand(b"setu\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_Set_f));
    Cmd_SetCommandCompletionFunc(b"setu\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cvar_CompleteCvarName));
    Cmd_AddCommand(b"seta\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_Set_f));
    Cmd_SetCommandCompletionFunc(b"seta\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cvar_CompleteCvarName));
    Cmd_AddCommand(b"reset\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_Reset_f));
    Cmd_SetCommandCompletionFunc(b"reset\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cvar_CompleteCvarName));
    Cmd_AddCommand(b"unset\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_Unset_f));
    Cmd_SetCommandCompletionFunc(b"unset\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cvar_CompleteCvarName));
    Cmd_AddCommand(b"cvarlist\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_List_f));
    Cmd_AddCommand(b"cvar_modified\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_ListModified_f));
    Cmd_AddCommand(b"cvar_restart\x00" as *const u8 as *const libc::c_char,
                   Some(Cvar_Restart_f));
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_Restart_f() { Cvar_Restart(qfalse); }
#[no_mangle]
pub unsafe extern "C" fn Cvar_Restart(mut unsetVM: qboolean) {
    let mut curvar: *mut cvar_t = 0 as *mut cvar_t;
    curvar = cvar_vars;
    while !curvar.is_null() {
        if 0 != (*curvar).flags & 0x80i32 ||
               0 != unsetVM as libc::c_uint &&
                   0 != (*curvar).flags & 0x1000i32 {
            curvar = Cvar_Unset(curvar)
        } else {
            if 0 == (*curvar).flags & (0x40i32 | 0x10i32 | 0x400i32) {
                Cvar_Set2((*curvar).name, (*curvar).resetString, qfalse);
            }
            curvar = (*curvar).next
        }
    };
}
/*
============
Cvar_Unset

Unsets a cvar
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Unset(mut cv: *mut cvar_t) -> *mut cvar_t {
    let mut next: *mut cvar_t = (*cv).next;
    cvar_modifiedFlags |= (*cv).flags;
    if !(*cv).name.is_null() { Z_Free((*cv).name as *mut libc::c_void); }
    if !(*cv).string.is_null() { Z_Free((*cv).string as *mut libc::c_void); }
    if !(*cv).latchedString.is_null() {
        Z_Free((*cv).latchedString as *mut libc::c_void);
    }
    if !(*cv).resetString.is_null() {
        Z_Free((*cv).resetString as *mut libc::c_void);
    }
    if !(*cv).description.is_null() {
        Z_Free((*cv).description as *mut libc::c_void);
    }
    if !(*cv).prev.is_null() {
        (*(*cv).prev).next = (*cv).next
    } else { cvar_vars = (*cv).next }
    if !(*cv).next.is_null() { (*(*cv).next).prev = (*cv).prev }
    if !(*cv).hashPrev.is_null() {
        (*(*cv).hashPrev).hashNext = (*cv).hashNext
    } else { hashTable[(*cv).hashIndex as usize] = (*cv).hashNext }
    if !(*cv).hashNext.is_null() {
        (*(*cv).hashNext).hashPrev = (*cv).hashPrev
    }
    memset(cv as *mut libc::c_void, '\u{0}' as i32,
           ::std::mem::size_of::<cvar_t>() as libc::c_ulong);
    return next;
}
/*
============
Cvar_ListModified_f
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_ListModified_f() {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    let mut totalModified: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() > 1i32 {
        match_0 = Cmd_Argv(1i32)
    } else { match_0 = 0 as *mut libc::c_char }
    totalModified = 0i32;
    var = cvar_vars;
    while !var.is_null() {
        if !((*var).name.is_null() || 0 == (*var).modificationCount) {
            value =
                if !(*var).latchedString.is_null() {
                    (*var).latchedString
                } else { (*var).string };
            if !(0 == strcmp(value, (*var).resetString)) {
                totalModified += 1;
                if !(!match_0.is_null() &&
                         0 ==
                             Com_Filter(match_0, (*var).name,
                                        qfalse as libc::c_int)) {
                    if 0 != (*var).flags & 0x4i32 {
                        Com_Printf(b"S\x00" as *const u8 as
                                       *const libc::c_char);
                    } else {
                        Com_Printf(b" \x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    if 0 != (*var).flags & 0x8i32 {
                        Com_Printf(b"s\x00" as *const u8 as
                                       *const libc::c_char);
                    } else {
                        Com_Printf(b" \x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    if 0 != (*var).flags & 0x2i32 {
                        Com_Printf(b"U\x00" as *const u8 as
                                       *const libc::c_char);
                    } else {
                        Com_Printf(b" \x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    if 0 != (*var).flags & 0x40i32 {
                        Com_Printf(b"R\x00" as *const u8 as
                                       *const libc::c_char);
                    } else {
                        Com_Printf(b" \x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    if 0 != (*var).flags & 0x10i32 {
                        Com_Printf(b"I\x00" as *const u8 as
                                       *const libc::c_char);
                    } else {
                        Com_Printf(b" \x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    if 0 != (*var).flags & 0x1i32 {
                        Com_Printf(b"A\x00" as *const u8 as
                                       *const libc::c_char);
                    } else {
                        Com_Printf(b" \x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    if 0 != (*var).flags & 0x20i32 {
                        Com_Printf(b"L\x00" as *const u8 as
                                       *const libc::c_char);
                    } else {
                        Com_Printf(b" \x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    if 0 != (*var).flags & 0x200i32 {
                        Com_Printf(b"C\x00" as *const u8 as
                                       *const libc::c_char);
                    } else {
                        Com_Printf(b" \x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    if 0 != (*var).flags & 0x80i32 {
                        Com_Printf(b"?\x00" as *const u8 as
                                       *const libc::c_char);
                    } else {
                        Com_Printf(b" \x00" as *const u8 as
                                       *const libc::c_char);
                    }
                    Com_Printf(b" %s \"%s\", default \"%s\"\n\x00" as
                                   *const u8 as *const libc::c_char,
                               (*var).name, value, (*var).resetString);
                }
            }
        }
        var = (*var).next
    }
    Com_Printf(b"\n%i total modified cvars\n\x00" as *const u8 as
                   *const libc::c_char, totalModified);
}
/*
============
Cvar_List_f
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_List_f() {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    let mut i: libc::c_int = 0;
    let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() > 1i32 {
        match_0 = Cmd_Argv(1i32)
    } else { match_0 = 0 as *mut libc::c_char }
    i = 0i32;
    var = cvar_vars;
    while !var.is_null() {
        if !((*var).name.is_null() ||
                 !match_0.is_null() &&
                     0 ==
                         Com_Filter(match_0, (*var).name,
                                    qfalse as libc::c_int)) {
            if 0 != (*var).flags & 0x4i32 {
                Com_Printf(b"S\x00" as *const u8 as *const libc::c_char);
            } else {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            if 0 != (*var).flags & 0x8i32 {
                Com_Printf(b"s\x00" as *const u8 as *const libc::c_char);
            } else {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            if 0 != (*var).flags & 0x2i32 {
                Com_Printf(b"U\x00" as *const u8 as *const libc::c_char);
            } else {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            if 0 != (*var).flags & 0x40i32 {
                Com_Printf(b"R\x00" as *const u8 as *const libc::c_char);
            } else {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            if 0 != (*var).flags & 0x10i32 {
                Com_Printf(b"I\x00" as *const u8 as *const libc::c_char);
            } else {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            if 0 != (*var).flags & 0x1i32 {
                Com_Printf(b"A\x00" as *const u8 as *const libc::c_char);
            } else {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            if 0 != (*var).flags & 0x20i32 {
                Com_Printf(b"L\x00" as *const u8 as *const libc::c_char);
            } else {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            if 0 != (*var).flags & 0x200i32 {
                Com_Printf(b"C\x00" as *const u8 as *const libc::c_char);
            } else {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            if 0 != (*var).flags & 0x80i32 {
                Com_Printf(b"?\x00" as *const u8 as *const libc::c_char);
            } else {
                Com_Printf(b" \x00" as *const u8 as *const libc::c_char);
            }
            Com_Printf(b" %s \"%s\"\n\x00" as *const u8 as
                           *const libc::c_char, (*var).name, (*var).string);
        }
        var = (*var).next;
        i += 1
    }
    Com_Printf(b"\n%i total cvars\n\x00" as *const u8 as *const libc::c_char,
               i);
    Com_Printf(b"%i cvar indexes\n\x00" as *const u8 as *const libc::c_char,
               cvar_numIndexes);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_CompleteCvarName(mut args: *mut libc::c_char,
                                               mut argNum: libc::c_int) {
    if argNum == 2i32 {
        let mut p: *mut libc::c_char =
            Com_SkipTokens(args, 1i32,
                           b" \x00" as *const u8 as *const libc::c_char as
                               *mut libc::c_char);
        if p > args { Field_CompleteCommand(p, qfalse, qtrue); }
    };
}
/*
============
Cvar_Unset_f

Unsets a userdefined cvar
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Unset_f() {
    let mut cv: *mut cvar_t = 0 as *mut cvar_t;
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"Usage: %s <varname>\n\x00" as *const u8 as
                       *const libc::c_char, Cmd_Argv(0i32));
        return
    }
    cv = Cvar_FindVar(Cmd_Argv(1i32));
    if cv.is_null() { return }
    if 0 != (*cv).flags & 0x80i32 {
        Cvar_Unset(cv);
    } else {
        Com_Printf(b"Error: %s: Variable %s is not user created.\n\x00" as
                       *const u8 as *const libc::c_char, Cmd_Argv(0i32),
                   (*cv).name);
    };
}
/*
============
Cvar_Reset_f
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Reset_f() {
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"usage: reset <variable>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    Cvar_Reset(Cmd_Argv(1i32));
}
/*
============
Cvar_Set_f

Allows setting and defining of arbitrary cvars from console, even if they
weren't declared in C code.
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Set_f() {
    let mut c: libc::c_int = 0;
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut cvar_t = 0 as *mut cvar_t;
    c = Cmd_Argc();
    cmd = Cmd_Argv(0i32);
    if c < 2i32 {
        Com_Printf(b"usage: %s <variable> <value>\n\x00" as *const u8 as
                       *const libc::c_char, cmd);
        return
    }
    if c == 2i32 { Cvar_Print_f(); return }
    v = Cvar_Set2(Cmd_Argv(1i32), Cmd_ArgsFrom(2i32), qfalse);
    if v.is_null() { return }
    match *cmd.offset(3isize) as libc::c_int {
        97 => {
            if 0 == (*v).flags & 0x1i32 {
                (*v).flags |= 0x1i32;
                cvar_modifiedFlags |= 0x1i32
            }
        }
        117 => {
            if 0 == (*v).flags & 0x2i32 {
                (*v).flags |= 0x2i32;
                cvar_modifiedFlags |= 0x2i32
            }
        }
        115 => {
            if 0 == (*v).flags & 0x4i32 {
                (*v).flags |= 0x4i32;
                cvar_modifiedFlags |= 0x4i32
            }
        }
        _ => { }
    };
}
/*
============
Cvar_Print_f

Prints the contents of a cvar 
(preferred over Cvar_Command where cvar names and commands conflict)
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Print_f() {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cv: *mut cvar_t = 0 as *mut cvar_t;
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"usage: print <variable>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    name = Cmd_Argv(1i32);
    cv = Cvar_FindVar(name);
    if !cv.is_null() {
        Cvar_Print(cv);
    } else {
        Com_Printf(b"Cvar %s does not exist.\n\x00" as *const u8 as
                       *const libc::c_char, name);
    };
}
/*
============
Cvar_Toggle_f

Toggles a cvar for easy single key binding, optionally through a list of
given values
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cvar_Toggle_f() {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = Cmd_Argc();
    let mut curval: *mut libc::c_char = 0 as *mut libc::c_char;
    if c < 2i32 {
        Com_Printf(b"usage: toggle <variable> [value1, value2, ...]\n\x00" as
                       *const u8 as *const libc::c_char);
        return
    }
    if c == 2i32 {
        Cvar_Set2(Cmd_Argv(1i32),
                  va(b"%d\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char,
                     (0. == Cvar_VariableValue(Cmd_Argv(1i32))) as
                         libc::c_int), qfalse);
        return
    }
    if c == 3i32 {
        Com_Printf(b"toggle: nothing to toggle to\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    curval = Cvar_VariableString(Cmd_Argv(1i32));
    i = 2i32;
    while i + 1i32 < c {
        if strcmp(curval, Cmd_Argv(i)) == 0i32 {
            Cvar_Set2(Cmd_Argv(1i32), Cmd_Argv(i + 1i32), qfalse);
            return
        }
        i += 1
    }
    Cvar_Set2(Cmd_Argv(1i32), Cmd_Argv(2i32), qfalse);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_InfoString(mut bit: libc::c_int)
 -> *mut libc::c_char {
    static mut info: [libc::c_char; 1024] = [0; 1024];
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    info[0usize] = 0i32 as libc::c_char;
    var = cvar_vars;
    while !var.is_null() {
        if !(*var).name.is_null() && 0 != (*var).flags & bit {
            Info_SetValueForKey(info.as_mut_ptr(), (*var).name,
                                (*var).string);
        }
        var = (*var).next
    }
    return info.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_InfoString_Big(mut bit: libc::c_int)
 -> *mut libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    info[0usize] = 0i32 as libc::c_char;
    var = cvar_vars;
    while !var.is_null() {
        if !(*var).name.is_null() && 0 != (*var).flags & bit {
            Info_SetValueForKey_Big(info.as_mut_ptr(), (*var).name,
                                    (*var).string);
        }
        var = (*var).next
    }
    return info.as_mut_ptr();
}
// returns an info string containing all the cvars that have the given bit set
// in their flags ( CVAR_USERINFO, CVAR_SERVERINFO, CVAR_SYSTEMINFO, etc )
#[no_mangle]
pub unsafe extern "C" fn Cvar_InfoStringBuffer(mut bit: libc::c_int,
                                               mut buff: *mut libc::c_char,
                                               mut buffsize: libc::c_int) {
    Q_strncpyz(buff, Cvar_InfoString(bit), buffsize);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_CheckRange(mut var: *mut cvar_t,
                                         mut min: libc::c_float,
                                         mut max: libc::c_float,
                                         mut integral: qboolean) {
    (*var).validate = qtrue;
    (*var).min = min;
    (*var).max = max;
    (*var).integral = integral;
    Cvar_Set((*var).name, (*var).string);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_SetDescription(mut var: *mut cvar_t,
                                             mut var_description:
                                                 *const libc::c_char) {
    if !var_description.is_null() &&
           *var_description.offset(0isize) as libc::c_int != '\u{0}' as i32 {
        if !(*var).description.is_null() {
            Z_Free((*var).description as *mut libc::c_void);
        }
        (*var).description = CopyString(var_description)
    };
}