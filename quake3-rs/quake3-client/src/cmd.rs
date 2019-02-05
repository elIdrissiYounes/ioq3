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
        #[no_mangle]
        pub fn COM_DefaultExtension(path: *mut libc::c_char,
                                    maxSize: libc::c_int,
                                    extension: *const libc::c_char);
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
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cmd.c"]
pub mod cmd_c {
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
// cmd.c -- Quake script command processing module
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cmd_t {
        pub data: *mut byte,
        pub maxsize: libc::c_int,
        pub cursize: libc::c_int,
    }
    pub type cmd_function_t = cmd_function_s;
    /*
=============================================================================

					COMMAND EXECUTION

=============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cmd_function_s {
        pub next: *mut cmd_function_s,
        pub name: *mut libc::c_char,
        pub function: xcommand_t,
        pub complete: completionFunc_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed_1 {
        pub c: *mut libc::c_char,
        pub v: *mut libc::c_void,
    }
    use super::q_shared_h::{byte};
    use super::{libc};
    use super::qcommon_h::{xcommand_t, completionFunc_t};
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
    pub type completionFunc_t
        =
        Option<unsafe extern "C" fn(_: *mut libc::c_char, _: libc::c_int)
                   -> ()>;
    use super::{libc};
    use super::q_shared_h::{qboolean, cvar_t};
    extern "C" {
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        // do a screen update before starting to load a map
// when the server is going to load a new map, the entire hunk
// will be cleared, so the client must shutdown cgame, ui, and
// the renderer
        #[no_mangle]
        pub fn CL_ForwardCommandToServer(string: *const libc::c_char);
        //
// UI interface
//
        #[no_mangle]
        pub fn UI_GameCommand() -> qboolean;
        #[no_mangle]
        pub static mut com_cl_running: *mut cvar_t;
        #[no_mangle]
        pub fn SV_GameCommand() -> qboolean;
        #[no_mangle]
        pub static mut com_sv_running: *mut cvar_t;
        #[no_mangle]
        pub fn CL_GameCommand() -> qboolean;
        // reset all testing vars to a safe value
        #[no_mangle]
        pub fn Cvar_Command() -> qboolean;
        #[no_mangle]
        pub fn CopyString(in_0: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn S_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Cvar_CompleteCvarName(args: *mut libc::c_char,
                                     argNum: libc::c_int);
        // returns 0 if not defined or non numeric
        #[no_mangle]
        pub fn Cvar_VariableString(var_name: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn Field_CompleteFilename(dir: *const libc::c_char,
                                      ext: *const libc::c_char,
                                      stripExt: qboolean,
                                      allowNonPureFilesOnDisk: qboolean);
        // forces flush on files we're writing to.
        #[no_mangle]
        pub fn FS_FreeFile(buffer: *mut libc::c_void);
        #[no_mangle]
        pub fn FS_ReadFile(qpath: *const libc::c_char,
                           buffer: *mut *mut libc::c_void) -> libc::c_long;
        #[no_mangle]
        pub fn Com_Filter(filter: *mut libc::c_char, name: *mut libc::c_char,
                          casesensitive: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
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
        pub fn memmove(_: *mut libc::c_void, _: *const libc::c_void,
                       _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strpbrk(_: *const libc::c_char, _: *const libc::c_char)
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
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, unnamed, EXEC_APPEND,
                       EXEC_INSERT, EXEC_NOW, unnamed_0, ERR_NEED_CD,
                       ERR_DISCONNECT, ERR_SERVERDISCONNECT, ERR_DROP,
                       ERR_FATAL, cvar_s, cvar_t, COM_DefaultExtension,
                       Q_stricmp, Q_strncpyz, va, Com_Error, Com_Printf};
use self::cmd_c::{cmd_t, cmd_function_t, cmd_function_s, unnamed_1};
use self::qcommon_h::{xcommand_t, completionFunc_t, Com_DPrintf,
                      CL_ForwardCommandToServer, UI_GameCommand,
                      com_cl_running, SV_GameCommand, com_sv_running,
                      CL_GameCommand, Cvar_Command, CopyString, S_MallocDebug,
                      Cvar_CompleteCvarName, Cvar_VariableString,
                      Field_CompleteFilename, FS_FreeFile, FS_ReadFile,
                      Com_Filter, Z_Free};
use self::string_h::{memcpy, memmove, strcat, strcmp, strpbrk, strlen};
use self::stdlib_h::{atoi};
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
pub unsafe extern "C" fn Cbuf_Init() {
    cmd_text.data = cmd_text_buf.as_mut_ptr();
    cmd_text.maxsize = 128i32 * 1024i32;
    cmd_text.cursize = 0i32;
}
#[no_mangle]
pub static mut cmd_text: cmd_t =
    cmd_t{data: 0 as *const byte as *mut byte, maxsize: 0, cursize: 0,};
#[no_mangle]
pub static mut cmd_text_buf: [byte; 131072] = [0; 131072];
// allocates an initial text buffer that will grow as needed
#[no_mangle]
pub unsafe extern "C" fn Cbuf_AddText(mut text: *const libc::c_char) {
    let mut l: libc::c_int = 0;
    l = strlen(text) as libc::c_int;
    if cmd_text.cursize + l >= cmd_text.maxsize {
        Com_Printf(b"Cbuf_AddText: overflow\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    memcpy(&mut *cmd_text.data.offset(cmd_text.cursize as isize) as *mut byte
               as *mut libc::c_void, text as *const libc::c_void,
           l as libc::c_ulong);
    cmd_text.cursize += l;
}
// Adds command text at the end of the buffer, does NOT add a final \n
#[no_mangle]
pub unsafe extern "C" fn Cbuf_ExecuteText(mut exec_when: libc::c_int,
                                          mut text: *const libc::c_char) {
    match exec_when {
        0 => {
            if !text.is_null() && strlen(text) > 0i32 as libc::c_ulong {
                Com_DPrintf(b"^3EXEC_NOW %s\n\x00" as *const u8 as
                                *const libc::c_char, text);
                Cmd_ExecuteString(text);
            } else {
                Cbuf_Execute();
                Com_DPrintf(b"^3EXEC_NOW %s\n\x00" as *const u8 as
                                *const libc::c_char, cmd_text.data);
            }
        }
        1 => { Cbuf_InsertText(text); }
        2 => { Cbuf_AddText(text); }
        _ => {
            Com_Error(ERR_FATAL as libc::c_int,
                      b"Cbuf_ExecuteText: bad exec_when\x00" as *const u8 as
                          *const libc::c_char);
        }
    };
}
/*
============
Cbuf_InsertText

Adds command text immediately after the current command
Adds a \n to the text
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cbuf_InsertText(mut text: *const libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    len = strlen(text).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
    if len + cmd_text.cursize > cmd_text.maxsize {
        Com_Printf(b"Cbuf_InsertText overflowed\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    i = cmd_text.cursize - 1i32;
    while i >= 0i32 {
        *cmd_text.data.offset((i + len) as isize) =
            *cmd_text.data.offset(i as isize);
        i -= 1
    }
    memcpy(cmd_text.data as *mut libc::c_void, text as *const libc::c_void,
           (len - 1i32) as libc::c_ulong);
    *cmd_text.data.offset((len - 1i32) as isize) = '\n' as i32 as byte;
    cmd_text.cursize += len;
}
// this can be used in place of either Cbuf_AddText or Cbuf_InsertText
#[no_mangle]
pub unsafe extern "C" fn Cbuf_Execute() {
    let mut i: libc::c_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut quotes: libc::c_int = 0;
    // This will keep // style comments all on one line by not breaking on
	// a semicolon.  It will keep /* ... */ style comments all on one line by not
	// breaking it for semicolon or newline.
    let mut in_star_comment: qboolean = qfalse;
    let mut in_slash_comment: qboolean = qfalse;
    while 0 != cmd_text.cursize {
        if cmd_wait > 0i32 {
            cmd_wait -= 1;
            break ;
        } else {
            text = cmd_text.data as *mut libc::c_char;
            quotes = 0i32;
            i = 0i32;
            while i < cmd_text.cursize {
                if *text.offset(i as isize) as libc::c_int == '\"' as i32 {
                    quotes += 1
                }
                if 0 == quotes & 1i32 {
                    if i < cmd_text.cursize - 1i32 {
                        if 0 == in_star_comment as u64 &&
                               *text.offset(i as isize) as libc::c_int ==
                                   '/' as i32 &&
                               *text.offset((i + 1i32) as isize) as
                                   libc::c_int == '/' as i32 {
                            in_slash_comment = qtrue
                        } else if 0 == in_slash_comment as u64 &&
                                      *text.offset(i as isize) as libc::c_int
                                          == '/' as i32 &&
                                      *text.offset((i + 1i32) as isize) as
                                          libc::c_int == '*' as i32 {
                            in_star_comment = qtrue
                        } else if 0 != in_star_comment as libc::c_uint &&
                                      *text.offset(i as isize) as libc::c_int
                                          == '*' as i32 &&
                                      *text.offset((i + 1i32) as isize) as
                                          libc::c_int == '/' as i32 {
                            in_star_comment = qfalse;
                            i += 1;
                            break ;
                        }
                    }
                    if 0 == in_slash_comment as u64 &&
                           0 == in_star_comment as u64 &&
                           *text.offset(i as isize) as libc::c_int ==
                               ';' as i32 {
                        break ;
                    }
                }
                if 0 == in_star_comment as u64 &&
                       (*text.offset(i as isize) as libc::c_int == '\n' as i32
                            ||
                            *text.offset(i as isize) as libc::c_int ==
                                '\r' as i32) {
                    in_slash_comment = qfalse;
                    break ;
                } else { i += 1 }
            }
            if i >= 1024i32 - 1i32 { i = 1024i32 - 1i32 }
            memcpy(line.as_mut_ptr() as *mut libc::c_void,
                   text as *const libc::c_void, i as libc::c_ulong);
            line[i as usize] = 0i32 as libc::c_char;
            if i == cmd_text.cursize {
                cmd_text.cursize = 0i32
            } else {
                i += 1;
                cmd_text.cursize -= i;
                memmove(text as *mut libc::c_void,
                        text.offset(i as isize) as *const libc::c_void,
                        cmd_text.cursize as libc::c_ulong);
            }
            Cmd_ExecuteString(line.as_mut_ptr());
        }
    };
}
// Takes a null terminated string.  Does not need to be /n terminated.
// breaks the string up into arg tokens.
#[no_mangle]
pub unsafe extern "C" fn Cmd_ExecuteString(mut text: *const libc::c_char) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    let mut prev: *mut *mut cmd_function_t = 0 as *mut *mut cmd_function_t;
    Cmd_TokenizeString(text);
    if 0 == Cmd_Argc() { return }
    prev = &mut cmd_functions;
    while !(*prev).is_null() {
        cmd = *prev;
        if 0 == Q_stricmp(cmd_argv[0usize], (*cmd).name) {
            *prev = (*cmd).next;
            (*cmd).next = cmd_functions;
            cmd_functions = cmd;
            // perform the action
            if (*cmd).function.is_none() {
                // let the cgame or game handle it
                break ;
            } else {
                (*cmd).function.expect("non-null function pointer")();
                return
            }
        } else { prev = &mut (*cmd).next }
    }
    if 0 != Cvar_Command() as u64 { return }
    if !com_cl_running.is_null() && 0 != (*com_cl_running).integer &&
           0 != CL_GameCommand() as libc::c_uint {
        return
    }
    if !com_sv_running.is_null() && 0 != (*com_sv_running).integer &&
           0 != SV_GameCommand() as libc::c_uint {
        return
    }
    if !com_cl_running.is_null() && 0 != (*com_cl_running).integer &&
           0 != UI_GameCommand() as libc::c_uint {
        return
    }
    CL_ForwardCommandToServer(text);
}
// possible commands to execute
static mut cmd_functions: *mut cmd_function_t =
    0 as *const cmd_function_t as *mut cmd_function_t;
// points into cmd_tokenized
static mut cmd_argv: [*mut libc::c_char; 1024] =
    [0 as *const libc::c_char as *mut libc::c_char; 1024];
#[no_mangle]
pub unsafe extern "C" fn Cmd_Argc() -> libc::c_int { return cmd_argc; }
static mut cmd_argc: libc::c_int = 0;
// The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
#[no_mangle]
pub unsafe extern "C" fn Cmd_TokenizeString(mut text_in:
                                                *const libc::c_char) {
    Cmd_TokenizeString2(text_in, qfalse);
}
/*
============
Cmd_TokenizeString

Parses the given string into command line tokens.
The text is copied to a separate buffer and 0 characters
are inserted in the appropriate place, The argv array
will point into this temporary buffer.
============
*/
// NOTE TTimo define that to track tokenization issues
//#define TKN_DBG
unsafe extern "C" fn Cmd_TokenizeString2(mut text_in: *const libc::c_char,
                                         mut ignoreQuotes: qboolean) {
    let mut text: *const libc::c_char = 0 as *const libc::c_char;
    let mut textOut: *mut libc::c_char = 0 as *mut libc::c_char;
    cmd_argc = 0i32;
    if text_in.is_null() { return }
    Q_strncpyz(cmd_cmd.as_mut_ptr(), text_in,
               ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong
                   as libc::c_int);
    text = text_in;
    textOut = cmd_tokenized.as_mut_ptr();
    loop  {
        if cmd_argc == 1024i32 { return }
        loop  {
            while 0 != *text as libc::c_int &&
                      *text as libc::c_int <= ' ' as i32 {
                text = text.offset(1isize)
            }
            if 0 == *text { return }
            if *text.offset(0isize) as libc::c_int == '/' as i32 &&
                   *text.offset(1isize) as libc::c_int == '/' as i32 {
                return
            }
            // skip /* */ comments
            if *text.offset(0isize) as libc::c_int == '/' as i32 &&
                   *text.offset(1isize) as libc::c_int == '*' as i32 {
                while 0 != *text as libc::c_int &&
                          (*text.offset(0isize) as libc::c_int != '*' as i32
                               ||
                               *text.offset(1isize) as libc::c_int !=
                                   '/' as i32) {
                    text = text.offset(1isize)
                }
                if 0 == *text { return }
                text = text.offset(2isize)
            } else {
                // we are ready to parse a token
                break ;
            }
        }
        // handle quoted strings
    // NOTE TTimo this doesn't handle \" escaping
        if 0 == ignoreQuotes as u64 && *text as libc::c_int == '\"' as i32 {
            cmd_argv[cmd_argc as usize] = textOut;
            cmd_argc += 1;
            text = text.offset(1isize);
            while 0 != *text as libc::c_int &&
                      *text as libc::c_int != '\"' as i32 {
                let fresh1 = textOut;
                textOut = textOut.offset(1);
                let fresh0 = text;
                text = text.offset(1);
                *fresh1 = *fresh0
            }
            let fresh2 = textOut;
            textOut = textOut.offset(1);
            *fresh2 = 0i32 as libc::c_char;
            if 0 == *text { return }
            text = text.offset(1isize)
        } else {
            cmd_argv[cmd_argc as usize] = textOut;
            cmd_argc += 1;
            while *text as libc::c_int > ' ' as i32 {
                if 0 == ignoreQuotes as u64 &&
                       *text.offset(0isize) as libc::c_int == '\"' as i32 {
                    break ;
                }
                if *text.offset(0isize) as libc::c_int == '/' as i32 &&
                       *text.offset(1isize) as libc::c_int == '/' as i32 {
                    break ;
                }
                // skip /* */ comments
                if *text.offset(0isize) as libc::c_int == '/' as i32 &&
                       *text.offset(1isize) as libc::c_int == '*' as i32 {
                    break ;
                }
                let fresh4 = textOut;
                textOut = textOut.offset(1);
                let fresh3 = text;
                text = text.offset(1);
                *fresh4 = *fresh3
            }
            let fresh5 = textOut;
            textOut = textOut.offset(1);
            *fresh5 = 0i32 as libc::c_char;
            if 0 == *text { return }
        }
    };
}
// will have 0 bytes inserted
static mut cmd_tokenized: [libc::c_char; 9216] = [0; 9216];
// the original command we received (no token processing)
static mut cmd_cmd: [libc::c_char; 8192] = [0; 8192];
#[no_mangle]
pub static mut cmd_wait: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Cmd_Init() {
    Cmd_AddCommand(b"cmdlist\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_List_f));
    Cmd_AddCommand(b"exec\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Exec_f));
    Cmd_AddCommand(b"execq\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Exec_f));
    Cmd_SetCommandCompletionFunc(b"exec\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cmd_CompleteCfgName));
    Cmd_SetCommandCompletionFunc(b"execq\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cmd_CompleteCfgName));
    Cmd_AddCommand(b"vstr\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Vstr_f));
    Cmd_SetCommandCompletionFunc(b"vstr\x00" as *const u8 as
                                     *const libc::c_char,
                                 Some(Cvar_CompleteCvarName));
    Cmd_AddCommand(b"echo\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Echo_f));
    Cmd_AddCommand(b"wait\x00" as *const u8 as *const libc::c_char,
                   Some(Cmd_Wait_f));
}
//=============================================================================
/*
============
Cmd_Wait_f

Causes execution of the remainder of the command buffer to be delayed until
next frame.  This allows commands like:
bind g "cmd use rocket ; +attack ; wait ; -attack ; cmd use blaster"
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Wait_f() {
    if Cmd_Argc() == 2i32 {
        cmd_wait = atoi(Cmd_Argv(1i32));
        if cmd_wait < 0i32 { cmd_wait = 1i32 }
    } else { cmd_wait = 1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Argv(mut arg: libc::c_int) -> *mut libc::c_char {
    if arg as libc::c_uint >= cmd_argc as libc::c_uint {
        return b"\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    }
    return cmd_argv[arg as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_AddCommand(mut cmd_name: *const libc::c_char,
                                        mut function: xcommand_t) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    if !Cmd_FindCommand(cmd_name).is_null() {
        if function.is_some() {
            Com_Printf(b"Cmd_AddCommand: %s already defined\n\x00" as
                           *const u8 as *const libc::c_char, cmd_name);
        }
        return
    }
    cmd =
        S_MallocDebug(::std::mem::size_of::<cmd_function_t>() as libc::c_ulong
                          as libc::c_int,
                      b"sizeof(cmd_function_t)\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      b"code/qcommon/cmd.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 651i32) as
            *mut cmd_function_t;
    (*cmd).name = CopyString(cmd_name);
    (*cmd).function = function;
    (*cmd).complete = None;
    (*cmd).next = cmd_functions;
    cmd_functions = cmd;
}
/*
============
Cmd_FindCommand
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_FindCommand(mut cmd_name: *const libc::c_char)
 -> *mut cmd_function_t {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    cmd = cmd_functions;
    while !cmd.is_null() {
        if 0 == Q_stricmp(cmd_name, (*cmd).name) { return cmd }
        cmd = (*cmd).next
    }
    return 0 as *mut cmd_function_t;
}
/*
===============
Cmd_Echo_f

Just prints the rest of the line to the console
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Echo_f() {
    Com_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char, Cmd_Args());
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Args() -> *mut libc::c_char {
    static mut cmd_args: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    cmd_args[0usize] = 0i32 as libc::c_char;
    i = 1i32;
    while i < cmd_argc {
        strcat(cmd_args.as_mut_ptr(), cmd_argv[i as usize]);
        if i != cmd_argc - 1i32 {
            strcat(cmd_args.as_mut_ptr(),
                   b" \x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    }
    return cmd_args.as_mut_ptr();
}
// callback with each valid string
#[no_mangle]
pub unsafe extern "C" fn Cmd_SetCommandCompletionFunc(mut command:
                                                          *const libc::c_char,
                                                      mut complete:
                                                          completionFunc_t) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    cmd = cmd_functions;
    while !cmd.is_null() {
        if 0 == Q_stricmp(command, (*cmd).name) {
            (*cmd).complete = complete;
            return
        }
        cmd = (*cmd).next
    };
}
/*
===============
Cmd_Vstr_f

Inserts the current value of a variable as command text
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Vstr_f() {
    let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"vstr <variablename> : execute a variable command\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    v = Cvar_VariableString(Cmd_Argv(1i32));
    Cbuf_InsertText(va(b"%s\n\x00" as *const u8 as *const libc::c_char as
                           *mut libc::c_char, v));
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_CompleteCfgName(mut args: *mut libc::c_char,
                                             mut argNum: libc::c_int) {
    if argNum == 2i32 {
        Field_CompleteFilename(b"\x00" as *const u8 as *const libc::c_char,
                               b"cfg\x00" as *const u8 as *const libc::c_char,
                               qfalse, qtrue);
    };
}
/*
==============================================================================

						SCRIPT COMMANDS

==============================================================================
*/
/*
===============
Cmd_Exec_f
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_Exec_f() {
    let mut quiet: qboolean = qfalse;
    let mut f: unnamed_1 = unnamed_1{c: 0 as *mut libc::c_char,};
    let mut filename: [libc::c_char; 64] = [0; 64];
    quiet =
        (0 ==
             Q_stricmp(Cmd_Argv(0i32),
                       b"execq\x00" as *const u8 as *const libc::c_char)) as
            libc::c_int as qboolean;
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"exec%s <filename> : execute a script file%s\n\x00" as
                       *const u8 as *const libc::c_char,
                   if 0 != quiet as libc::c_uint {
                       b"q\x00" as *const u8 as *const libc::c_char
                   } else { b"\x00" as *const u8 as *const libc::c_char },
                   if 0 != quiet as libc::c_uint {
                       b" without notification\x00" as *const u8 as
                           *const libc::c_char
                   } else { b"\x00" as *const u8 as *const libc::c_char });
        return
    }
    Q_strncpyz(filename.as_mut_ptr(), Cmd_Argv(1i32),
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    COM_DefaultExtension(filename.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 64]>() as
                             libc::c_ulong as libc::c_int,
                         b".cfg\x00" as *const u8 as *const libc::c_char);
    FS_ReadFile(filename.as_mut_ptr(), &mut f.v);
    if f.c.is_null() {
        Com_Printf(b"couldn\'t exec %s\n\x00" as *const u8 as
                       *const libc::c_char, filename.as_mut_ptr());
        return
    }
    if 0 == quiet as u64 {
        Com_Printf(b"execing %s\n\x00" as *const u8 as *const libc::c_char,
                   filename.as_mut_ptr());
    }
    Cbuf_InsertText(f.c);
    FS_FreeFile(f.v);
}
/*
============
Cmd_List_f
============
*/
#[no_mangle]
pub unsafe extern "C" fn Cmd_List_f() {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    let mut i: libc::c_int = 0;
    let mut match_0: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() > 1i32 {
        match_0 = Cmd_Argv(1i32)
    } else { match_0 = 0 as *mut libc::c_char }
    i = 0i32;
    cmd = cmd_functions;
    while !cmd.is_null() {
        if !(!match_0.is_null() &&
                 0 == Com_Filter(match_0, (*cmd).name, qfalse as libc::c_int))
           {
            Com_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                       (*cmd).name);
            i += 1
        }
        cmd = (*cmd).next
    }
    Com_Printf(b"%i commands\n\x00" as *const u8 as *const libc::c_char, i);
}
// called by the init functions of other parts of the program to
// register commands and functions to call for them.
// The cmd_name is referenced later, so it should not be in temp memory
// if function is NULL, the command will be forwarded to the server
// as a clc_clientCommand instead of executed locally
#[no_mangle]
pub unsafe extern "C" fn Cmd_RemoveCommand(mut cmd_name:
                                               *const libc::c_char) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    let mut back: *mut *mut cmd_function_t = 0 as *mut *mut cmd_function_t;
    back = &mut cmd_functions;
    loop  {
        cmd = *back;
        if cmd.is_null() { return }
        if 0 == strcmp(cmd_name, (*cmd).name) {
            *back = (*cmd).next;
            Z_Free((*cmd).name as *mut libc::c_void);
            Z_Free(cmd as *mut libc::c_void);
            return
        }
        back = &mut (*cmd).next
    };
}
// don't allow VMs to remove system commands
#[no_mangle]
pub unsafe extern "C" fn Cmd_RemoveCommandSafe(mut cmd_name:
                                                   *const libc::c_char) {
    let mut cmd: *mut cmd_function_t = Cmd_FindCommand(cmd_name);
    if cmd.is_null() { return }
    if (*cmd).function.is_some() {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Restricted source tried to remove system command \"%s\"\x00"
                      as *const u8 as *const libc::c_char, cmd_name);
    }
    Cmd_RemoveCommand(cmd_name);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_CommandCompletion(mut callback:
                                                   Option<unsafe extern "C" fn(_:
                                                                                   *const libc::c_char)
                                                              -> ()>) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    cmd = cmd_functions;
    while !cmd.is_null() {
        callback.expect("non-null function pointer")((*cmd).name);
        cmd = (*cmd).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_CompleteArgument(mut command:
                                                  *const libc::c_char,
                                              mut args: *mut libc::c_char,
                                              mut argNum: libc::c_int) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    cmd = cmd_functions;
    while !cmd.is_null() {
        if 0 == Q_stricmp(command, (*cmd).name) {
            if (*cmd).complete.is_some() {
                (*cmd).complete.expect("non-null function pointer")(args,
                                                                    argNum);
            }
            return
        }
        cmd = (*cmd).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_ArgvBuffer(mut arg: libc::c_int,
                                        mut buffer: *mut libc::c_char,
                                        mut bufferLength: libc::c_int) {
    Q_strncpyz(buffer, Cmd_Argv(arg), bufferLength);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_ArgsFrom(mut arg: libc::c_int)
 -> *mut libc::c_char {
    static mut cmd_args: [libc::c_char; 8192] = [0; 8192];
    let mut i: libc::c_int = 0;
    cmd_args[0usize] = 0i32 as libc::c_char;
    if arg < 0i32 { arg = 0i32 }
    i = arg;
    while i < cmd_argc {
        strcat(cmd_args.as_mut_ptr(), cmd_argv[i as usize]);
        if i != cmd_argc - 1i32 {
            strcat(cmd_args.as_mut_ptr(),
                   b" \x00" as *const u8 as *const libc::c_char);
        }
        i += 1
    }
    return cmd_args.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_ArgsBuffer(mut buffer: *mut libc::c_char,
                                        mut bufferLength: libc::c_int) {
    Q_strncpyz(buffer, Cmd_Args(), bufferLength);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Cmd() -> *mut libc::c_char {
    return cmd_cmd.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Args_Sanitize() {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < cmd_argc {
        let mut c: *mut libc::c_char = cmd_argv[i as usize];
        if strlen(c) > (256i32 - 1i32) as libc::c_ulong {
            *c.offset((256i32 - 1i32) as isize) =
                '\u{0}' as i32 as libc::c_char
        }
        loop  {
            c = strpbrk(c, b"\n\r;\x00" as *const u8 as *const libc::c_char);
            if c.is_null() { break ; }
            *c = ' ' as i32 as libc::c_char;
            c = c.offset(1isize)
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_TokenizeStringIgnoreQuotes(mut text_in:
                                                            *const libc::c_char) {
    Cmd_TokenizeString2(text_in, qtrue);
}