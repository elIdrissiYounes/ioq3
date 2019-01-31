use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __off_t = libc::c_long;
    pub type __off64_t = libc::c_long;
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
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
    /*
==============================================================

VoIP

==============================================================
*/
    // if you change the count of flags be sure to also change VOIP_FLAGNUM
    // spatialized voip message
    // non-spatialized voip message
    // number of flags voip knows. You will have to bump protocol version number if you
// change this.
    /*
==============================================================

COLLISION DETECTION

==============================================================
*/
    // plane types are used to speed some tests
// 0-2 are axial planes
    /*
=================
PlaneTypeForNormal
=================
*/
    // plane_t structure
// !!! if this is changed, it must be changed in asm code too !!!
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cplane_s {
        pub normal: vec3_t,
        pub dist: libc::c_float,
        pub type_0: byte,
        pub signbits: byte,
        pub pad: [byte; 2],
    }
    // mode parm for FS_FOpenFile
    pub type fsMode_t = libc::c_uint;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const FS_APPEND: fsMode_t = 2;
    pub const FS_WRITE: fsMode_t = 1;
    pub const FS_READ: fsMode_t = 0;
    pub type cplane_t = cplane_s;
    use super::{libc};
    extern "C" {
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
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_script.h"]
pub mod l_script_h {
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
    /* ****************************************************************************
 * name:		l_script.h
 *
 * desc:		lexicographical parser
 *
 * $Archive: /source/code/botlib/l_script.h $
 *
 *****************************************************************************/
    //undef if binary numbers of the form 0b... or 0B... are not allowed
    //undef if not using the token.intvalue and token.floatvalue
    //use dollar sign also as punctuation
    //maximum token length
    //script flags
    //token types
    // string
    // literal
    // number
    // name
    // punctuation
    //string sub type
//---------------
//		the length of the string
//literal sub type
//----------------
//		the ASCII code of the literal
//number sub type
//---------------
    // decimal number
    // hexadecimal number
    // octal number
    // binary number
    //BINARYNUMBERS
    // floating point number
    // integer number
    // long number
    // unsigned number
    //punctuation sub type
//--------------------
    //name sub type
//-------------
//		the length of the name
    //punctuation
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct punctuation_s {
        pub p: *mut libc::c_char,
        pub n: libc::c_int,
        pub next: *mut punctuation_s,
    }
    pub type punctuation_t = punctuation_s;
    //token
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct token_s {
        pub string: [libc::c_char; 1024],
        pub type_0: libc::c_int,
        pub subtype: libc::c_int,
        pub intvalue: libc::c_ulong,
        pub floatvalue: libc::c_float,
        pub whitespace_p: *mut libc::c_char,
        pub endwhitespace_p: *mut libc::c_char,
        pub line: libc::c_int,
        pub linescrossed: libc::c_int,
        pub next: *mut token_s,
    }
    pub type token_t = token_s;
    //script file
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct script_s {
        pub filename: [libc::c_char; 1024],
        pub buffer: *mut libc::c_char,
        pub script_p: *mut libc::c_char,
        pub end_p: *mut libc::c_char,
        pub lastscript_p: *mut libc::c_char,
        pub whitespace_p: *mut libc::c_char,
        pub endwhitespace_p: *mut libc::c_char,
        pub length: libc::c_int,
        pub line: libc::c_int,
        pub lastline: libc::c_int,
        pub tokenavailable: libc::c_int,
        pub flags: libc::c_int,
        pub punctuations: *mut punctuation_t,
        pub punctuationtable: *mut *mut punctuation_t,
        pub token: token_t,
        pub next: *mut script_s,
    }
    pub type script_t = script_s;
    use super::{libc};
    extern "C" {
        //remove any leading and trailing double quotes from the token
        #[no_mangle]
        pub fn StripDoubleQuotes(string: *mut libc::c_char);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_precomp.h"]
pub mod l_precomp_h {
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
    /* ****************************************************************************
 * name:		l_precomp.h
 *
 * desc:		pre compiler
 *
 * $Archive: /source/code/botlib/l_precomp.h $
 *
 *****************************************************************************/
    //macro definitions
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct define_s {
        pub name: *mut libc::c_char,
        pub flags: libc::c_int,
        pub builtin: libc::c_int,
        pub numparms: libc::c_int,
        pub parms: *mut token_t,
        pub tokens: *mut token_t,
        pub next: *mut define_s,
        pub hashnext: *mut define_s,
    }
    pub type define_t = define_s;
    //indents
//used for conditional compilation directives:
//#if, #else, #elif, #ifdef, #ifndef
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct indent_s {
        pub type_0: libc::c_int,
        pub skip: libc::c_int,
        pub script: *mut script_t,
        pub next: *mut indent_s,
    }
    pub type indent_t = indent_s;
    //source file
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct source_s {
        pub filename: [libc::c_char; 1024],
        pub includepath: [libc::c_char; 1024],
        pub punctuations: *mut punctuation_t,
        pub scriptstack: *mut script_t,
        pub tokens: *mut token_t,
        pub defines: *mut define_t,
        pub definehash: *mut *mut define_t,
        pub indentstack: *mut indent_t,
        pub skip: libc::c_int,
        pub token: token_t,
    }
    pub type source_t = source_s;
    use super::{libc};
    use super::l_script_h::{token_t, script_t, punctuation_t};
    extern "C" {
        //read a token from the source
        #[no_mangle]
        pub fn PC_ReadToken(source: *mut source_t, token: *mut token_t)
         -> libc::c_int;
        //expect a certain token
        #[no_mangle]
        pub fn PC_ExpectTokenString(source: *mut source_t,
                                    string: *mut libc::c_char) -> libc::c_int;
        //expect a certain token type
        #[no_mangle]
        pub fn PC_ExpectTokenType(source: *mut source_t, type_0: libc::c_int,
                                  subtype: libc::c_int, token: *mut token_t)
         -> libc::c_int;
        //expect a token
        #[no_mangle]
        pub fn PC_ExpectAnyToken(source: *mut source_t, token: *mut token_t)
         -> libc::c_int;
        //returns true when the token is available
        #[no_mangle]
        pub fn PC_CheckTokenString(source: *mut source_t,
                                   string: *mut libc::c_char) -> libc::c_int;
        //unread the last token read from the script
        #[no_mangle]
        pub fn PC_UnreadLastToken(source: *mut source_t);
        //set the base folder to load files from
        #[no_mangle]
        pub fn PC_SetBaseFolder(path: *mut libc::c_char);
        //load a source file
        #[no_mangle]
        pub fn LoadSourceFile(filename: *const libc::c_char) -> *mut source_t;
        //free the given source
        #[no_mangle]
        pub fn FreeSource(source: *mut source_t);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_chat.h"]
pub mod be_ai_chat_h {
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
/*****************************************************************************
 * name:		be_ai_chat.h
 *
 * desc:		char AI
 *
 * $Archive: /source/code/botlib/be_ai_chat.h $
 *
 *****************************************************************************/
    //a console message
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_consolemessage_s {
        pub handle: libc::c_int,
        pub time: libc::c_float,
        pub type_0: libc::c_int,
        pub message: [libc::c_char; 256],
        pub prev: *mut bot_consolemessage_s,
        pub next: *mut bot_consolemessage_s,
    }
    //returned to AI when a match is found
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_match_s {
        pub string: [libc::c_char; 256],
        pub type_0: libc::c_int,
        pub subtype: libc::c_int,
        pub variables: [bot_matchvariable_t; 8],
    }
    pub type bot_matchvariable_t = bot_matchvariable_s;
    //match variable
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_matchvariable_s {
        pub offset: libc::c_char,
        pub length: libc::c_int,
    }
    pub type bot_consolemessage_t = bot_consolemessage_s;
    pub type bot_match_t = bot_match_s;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/botlib.h"]
pub mod botlib_h {
    //bsp_trace_t hit surface
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_surface_s {
        pub name: [libc::c_char; 16],
        pub flags: libc::c_int,
        pub value: libc::c_int,
    }
    pub type bsp_surface_t = bsp_surface_s;
    //remove the bsp_trace_s structure definition l8r on
//a trace is returned when a box is swept through the world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_trace_s {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub exp_dist: libc::c_float,
        pub sidenum: libc::c_int,
        pub surface: bsp_surface_t,
        pub contents: libc::c_int,
        pub ent: libc::c_int,
    }
    pub type bsp_trace_t = bsp_trace_s;
    //bot AI library exported functions
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct botlib_import_s {
        pub Print: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut libc::c_char, ...)
                              -> ()>,
        pub Trace: Option<unsafe extern "C" fn(_: *mut bsp_trace_t,
                                               _: *mut vec_t, _: *mut vec_t,
                                               _: *mut vec_t, _: *mut vec_t,
                                               _: libc::c_int, _: libc::c_int)
                              -> ()>,
        pub EntityTrace: Option<unsafe extern "C" fn(_: *mut bsp_trace_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
        pub PointContents: Option<unsafe extern "C" fn(_: *mut vec_t)
                                      -> libc::c_int>,
        pub inPVS: Option<unsafe extern "C" fn(_: *mut vec_t, _: *mut vec_t)
                              -> libc::c_int>,
        pub BSPEntityData: Option<unsafe extern "C" fn()
                                      -> *mut libc::c_char>,
        pub BSPModelMinsMaxsOrigin: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t)
                                               -> ()>,
        pub BotClientCommand: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _:
                                                              *mut libc::c_char)
                                         -> ()>,
        pub GetMemory: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut libc::c_void>,
        pub FreeMemory: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> ()>,
        pub AvailableMemory: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub HunkAlloc: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut libc::c_void>,
        pub FS_FOpenFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *mut fileHandle_t,
                                                      _: fsMode_t)
                                     -> libc::c_int>,
        pub FS_Read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: libc::c_int,
                                                 _: fileHandle_t)
                                -> libc::c_int>,
        pub FS_Write: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                                  _: libc::c_int,
                                                  _: fileHandle_t)
                                 -> libc::c_int>,
        pub FS_FCloseFile: Option<unsafe extern "C" fn(_: fileHandle_t)
                                      -> ()>,
        pub FS_Seek: Option<unsafe extern "C" fn(_: fileHandle_t,
                                                 _: libc::c_long,
                                                 _: libc::c_int)
                                -> libc::c_int>,
        pub DebugLineCreate: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub DebugLineDelete: Option<unsafe extern "C" fn(_: libc::c_int)
                                        -> ()>,
        pub DebugLineShow: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: libc::c_int) -> ()>,
        pub DebugPolygonCreate: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: *mut vec3_t)
                                           -> libc::c_int>,
        pub DebugPolygonDelete: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
    }
    pub type botlib_import_t = botlib_import_s;
    use super::{libc};
    use super::q_shared_h::{qboolean, vec3_t, cplane_t, vec_t, fileHandle_t,
                            fsMode_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_chat.c"]
pub mod be_ai_chat_c {
    pub type bot_replychat_t = bot_replychat_s;
    //reply chat
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_replychat_s {
        pub keys: *mut bot_replychatkey_t,
        pub priority: libc::c_float,
        pub numchatmessages: libc::c_int,
        pub firstchatmessage: *mut bot_chatmessage_t,
        pub next: *mut bot_replychat_s,
    }
    pub type bot_chatmessage_t = bot_chatmessage_s;
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
    /* ****************************************************************************
 * name:		be_ai_chat.c
 *
 * desc:		bot chat AI
 *
 * $Archive: /MissionPack/code/botlib/be_ai_chat.c $
 *
 *****************************************************************************/
    //escape character
    //'_'
    //
// "hi ", people, " ", 0, " entered the game"
//becomes:
// "hi _rpeople_ _v0_ entered the game"
//
    //match piece types
    //variable match piece
    //string match piece
    //reply chat key flags
    //key must be present
    //key must be absent
    //name of bot must be present
    //key is a string
    //key is a match template
    //key is a series of botnames
    //bot must be female
    //bot must be male
    //bot must be genderless
    //time to ignore a chat message after using it
    //the actuall chat messages
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_chatmessage_s {
        pub chatmessage: *mut libc::c_char,
        pub time: libc::c_float,
        pub next: *mut bot_chatmessage_s,
    }
    pub type bot_replychatkey_t = bot_replychatkey_s;
    //reply chat key
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_replychatkey_s {
        pub flags: libc::c_int,
        pub string: *mut libc::c_char,
        pub match_0: *mut bot_matchpiece_t,
        pub next: *mut bot_replychatkey_s,
    }
    pub type bot_matchpiece_t = bot_matchpiece_s;
    //piece of a match template
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_matchpiece_s {
        pub type_0: libc::c_int,
        pub firststring: *mut bot_matchstring_t,
        pub variable: libc::c_int,
        pub next: *mut bot_matchpiece_s,
    }
    pub type bot_matchstring_t = bot_matchstring_s;
    //fixed match string
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_matchstring_s {
        pub string: *mut libc::c_char,
        pub next: *mut bot_matchstring_s,
    }
    pub type bot_stringlist_t = bot_stringlist_s;
    //string list
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_stringlist_s {
        pub string: *mut libc::c_char,
        pub next: *mut bot_stringlist_s,
    }
    pub type bot_randomstring_t = bot_randomstring_s;
    //random string
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_randomstring_s {
        pub string: *mut libc::c_char,
        pub next: *mut bot_randomstring_s,
    }
    pub type bot_randomlist_t = bot_randomlist_s;
    //list with random strings
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_randomlist_s {
        pub string: *mut libc::c_char,
        pub numstrings: libc::c_int,
        pub firstrandomstring: *mut bot_randomstring_t,
        pub next: *mut bot_randomlist_s,
    }
    pub type bot_matchtemplate_t = bot_matchtemplate_s;
    //match template
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_matchtemplate_s {
        pub context: libc::c_ulong,
        pub type_0: libc::c_int,
        pub subtype: libc::c_int,
        pub first: *mut bot_matchpiece_t,
        pub next: *mut bot_matchtemplate_s,
    }
    pub type bot_synonymlist_t = bot_synonymlist_s;
    //list with synonyms
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_synonymlist_s {
        pub context: libc::c_ulong,
        pub totalweight: libc::c_float,
        pub firstsynonym: *mut bot_synonym_t,
        pub next: *mut bot_synonymlist_s,
    }
    pub type bot_synonym_t = bot_synonym_s;
    //synonym
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_synonym_s {
        pub string: *mut libc::c_char,
        pub weight: libc::c_float,
        pub next: *mut bot_synonym_s,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_ichatdata_t {
        pub chat: *mut bot_chat_t,
        pub filename: [libc::c_char; 64],
        pub chatname: [libc::c_char; 64],
    }
    pub type bot_chat_t = bot_chat_s;
    //bot chat lines
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_chat_s {
        pub types: *mut bot_chattype_t,
    }
    pub type bot_chattype_t = bot_chattype_s;
    //bot chat type with chat lines
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_chattype_s {
        pub name: [libc::c_char; 32],
        pub numchatmessages: libc::c_int,
        pub firstchatmessage: *mut bot_chatmessage_t,
        pub next: *mut bot_chattype_s,
    }
    pub type bot_chatstate_t = bot_chatstate_s;
    //chat state of a bot
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_chatstate_s {
        pub gender: libc::c_int,
        pub client: libc::c_int,
        pub name: [libc::c_char; 32],
        pub chatmessage: [libc::c_char; 256],
        pub handle: libc::c_int,
        pub firstmessage: *mut bot_consolemessage_t,
        pub lastmessage: *mut bot_consolemessage_t,
        pub numconsolemessages: libc::c_int,
        pub chat: *mut bot_chat_t,
    }
    use super::{libc};
    use super::be_ai_chat_h::{bot_consolemessage_t, bot_match_t};
    use super::l_precomp_h::{source_t};
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
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    use super::FILE_h::{FILE};
    extern "C" {
        #[no_mangle]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, ...)
         -> libc::c_int;
        #[no_mangle]
        pub fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, ...)
         -> libc::c_int;
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
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
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
        pub fn rand() -> libc::c_int;
    }
}
#[header_src = "/usr/include/ctype.h"]
pub mod ctype_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn toupper(_: libc::c_int) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
    extern "C" {
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedHunkMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
    use super::{libc};
    extern "C" {
        //gets the value of the library variable with the given name
        #[no_mangle]
        pub fn LibVarGetValue(var_name: *const libc::c_char) -> libc::c_float;
        //creates the library variable if not existing already and returns the value
        #[no_mangle]
        pub fn LibVarValue(var_name: *const libc::c_char,
                           value: *const libc::c_char) -> libc::c_float;
        //creates the library variable if not existing already and returns the value string
        #[no_mangle]
        pub fn LibVarString(var_name: *const libc::c_char,
                            value: *const libc::c_char) -> *mut libc::c_char;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_variadic.h"]
pub mod l_variadic_h {
    use super::l_precomp_h::{source_t};
    use super::{libc};
    extern "C" {
        //print a source error
        #[no_mangle]
        pub fn SourceError(source: *mut source_t,
                           str: *mut libc::c_char, ...);
        //print a source warning
        #[no_mangle]
        pub fn SourceWarning(source: *mut source_t,
                             str: *mut libc::c_char, ...);
        //write to the current opened log file
        #[no_mangle]
        pub fn Log_Write(fmt: *mut libc::c_char, ...);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_log.h"]
pub mod l_log_h {
    use super::FILE_h::{FILE};
    extern "C" {
        //write to the current opened log file
        //write to the current opened log file with a time stamp
        //returns a pointer to the log file
        #[no_mangle]
        pub fn Log_FilePointer() -> *mut FILE;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_main.h"]
pub mod be_aas_main_h {
    use super::{libc};
    extern "C" {
        //returns the current time
        #[no_mangle]
        pub fn AAS_Time() -> libc::c_float;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    use super::botlib_h::{botlib_import_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
        //true if developer is on
        #[no_mangle]
        pub static mut botDeveloper: libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ea.h"]
pub mod be_ea_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn EA_Command(client: libc::c_int, command: *mut libc::c_char);
    }
}
use self::types_h::{__off_t, __off64_t};
use self::stddef_h::{size_t};
use self::libio_h::{_IO_FILE, _IO_lock_t, _IO_marker};
use self::FILE_h::{FILE};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, Q_stricmp, Q_strncpyz,
                       va};
use self::l_script_h::{punctuation_s, punctuation_t, token_s, token_t,
                       script_s, script_t, StripDoubleQuotes};
use self::l_precomp_h::{define_s, define_t, indent_s, indent_t, source_s,
                        source_t, PC_ReadToken, PC_ExpectTokenString,
                        PC_ExpectTokenType, PC_ExpectAnyToken,
                        PC_CheckTokenString, PC_UnreadLastToken,
                        PC_SetBaseFolder, LoadSourceFile, FreeSource};
use self::be_ai_chat_h::{bot_consolemessage_s, bot_match_s,
                         bot_matchvariable_t, bot_matchvariable_s,
                         bot_consolemessage_t, bot_match_t};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_ai_chat_c::{bot_replychat_t, bot_replychat_s, bot_chatmessage_t,
                         bot_chatmessage_s, bot_replychatkey_t,
                         bot_replychatkey_s, bot_matchpiece_t,
                         bot_matchpiece_s, bot_matchstring_t,
                         bot_matchstring_s, bot_stringlist_t,
                         bot_stringlist_s, bot_randomstring_t,
                         bot_randomstring_s, bot_randomlist_t,
                         bot_randomlist_s, bot_matchtemplate_t,
                         bot_matchtemplate_s, bot_synonymlist_t,
                         bot_synonymlist_s, bot_synonym_t, bot_synonym_s,
                         bot_ichatdata_t, bot_chat_t, bot_chat_s,
                         bot_chattype_t, bot_chattype_s, bot_chatstate_t,
                         bot_chatstate_s};
use self::assert_h::{__assert_fail};
use self::stdio_h::{fprintf, sprintf};
use self::string_h::{memcpy, memmove, memset, strcpy, strncpy, strcat, strcmp,
                     strlen};
use self::stdlib_h::{rand};
use self::ctype_h::{toupper};
use self::l_memory_h::{GetClearedMemory, GetClearedHunkMemory, FreeMemory};
use self::l_libvar_h::{LibVarGetValue, LibVarValue, LibVarString};
use self::l_variadic_h::{SourceError, SourceWarning, Log_Write};
use self::l_log_h::{Log_FilePointer};
use self::be_aas_main_h::{AAS_Time};
use self::be_interface_h::{botimport, botDeveloper};
use self::be_ea_h::{EA_Command};
//setup the chat AI
#[no_mangle]
pub unsafe extern "C" fn BotSetupChatAI() -> libc::c_int {
    let mut file: *mut libc::c_char = 0 as *mut libc::c_char;
    file =
        LibVarString(b"synfile\x00" as *const u8 as *const libc::c_char,
                     b"syn.c\x00" as *const u8 as *const libc::c_char);
    synonyms = BotLoadSynonyms(file);
    file =
        LibVarString(b"rndfile\x00" as *const u8 as *const libc::c_char,
                     b"rnd.c\x00" as *const u8 as *const libc::c_char);
    randomstrings = BotLoadRandomStrings(file);
    file =
        LibVarString(b"matchfile\x00" as *const u8 as *const libc::c_char,
                     b"match.c\x00" as *const u8 as *const libc::c_char);
    matchtemplates = BotLoadMatchTemplates(file);
    if 0. ==
           LibVarValue(b"nochat\x00" as *const u8 as *const libc::c_char,
                       b"0\x00" as *const u8 as *const libc::c_char) {
        file =
            LibVarString(b"rchatfile\x00" as *const u8 as *const libc::c_char,
                         b"rchat.c\x00" as *const u8 as *const libc::c_char);
        replychats = BotLoadReplyChat(file)
    }
    InitConsoleMessageHeap();
    return 0i32;
}
//end of the function BotChatStateFromHandle
//===========================================================================
// initialize the heap with unused console messages
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn InitConsoleMessageHeap() {
    let mut i: libc::c_int = 0;
    let mut max_messages: libc::c_int = 0;
    if !consolemessageheap.is_null() {
        FreeMemory(consolemessageheap as *mut libc::c_void);
    }
    max_messages =
        LibVarValue(b"max_messages\x00" as *const u8 as *const libc::c_char,
                    b"1024\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    consolemessageheap =
        GetClearedHunkMemory((max_messages as
                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<bot_consolemessage_t>()
                                                                  as
                                                                  libc::c_ulong))
            as *mut bot_consolemessage_t;
    let ref mut fresh0 = (*consolemessageheap.offset(0isize)).prev;
    *fresh0 = 0 as *mut bot_consolemessage_s;
    let ref mut fresh1 = (*consolemessageheap.offset(0isize)).next;
    *fresh1 =
        &mut *consolemessageheap.offset(1isize) as *mut bot_consolemessage_t;
    i = 1i32;
    while i < max_messages - 1i32 {
        let ref mut fresh2 = (*consolemessageheap.offset(i as isize)).prev;
        *fresh2 =
            &mut *consolemessageheap.offset((i - 1i32) as isize) as
                *mut bot_consolemessage_t;
        let ref mut fresh3 = (*consolemessageheap.offset(i as isize)).next;
        *fresh3 =
            &mut *consolemessageheap.offset((i + 1i32) as isize) as
                *mut bot_consolemessage_t;
        i += 1
    }
    let ref mut fresh4 =
        (*consolemessageheap.offset((max_messages - 1i32) as isize)).prev;
    *fresh4 =
        &mut *consolemessageheap.offset((max_messages - 2i32) as isize) as
            *mut bot_consolemessage_t;
    let ref mut fresh5 =
        (*consolemessageheap.offset((max_messages - 1i32) as isize)).next;
    *fresh5 = 0 as *mut bot_consolemessage_s;
    freeconsolemessages = consolemessageheap;
}
//console message heap
#[no_mangle]
pub static mut consolemessageheap: *mut bot_consolemessage_t =
    0 as *const bot_consolemessage_t as *mut bot_consolemessage_t;
#[no_mangle]
pub static mut freeconsolemessages: *mut bot_consolemessage_t =
    0 as *const bot_consolemessage_t as *mut bot_consolemessage_t;
//end if
//end of the function BotCheckValidReplyChatKeySet
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLoadReplyChat(mut filename: *mut libc::c_char)
 -> *mut bot_replychat_t {
    let mut chatmessagestring: [libc::c_char; 256] = [0; 256];
    let mut namebuffer: [libc::c_char; 256] = [0; 256];
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    let mut chatmessage: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut replychat: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut replychatlist: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char);
    source = LoadSourceFile(filename);
    if source.is_null() {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"counldn\'t load %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            filename);
        return 0 as *mut bot_replychat_t
    }
    replychatlist = 0 as *mut bot_replychat_t;
    while 0 != PC_ReadToken(source, &mut token) {
        if 0 !=
               strcmp(token.string.as_mut_ptr(),
                      b"[\x00" as *const u8 as *const libc::c_char) {
            SourceError(source,
                        b"expected [, found %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            BotFreeReplyChat(replychatlist);
            FreeSource(source);
            return 0 as *mut bot_replychat_t
        }
        replychat =
            GetClearedHunkMemory(::std::mem::size_of::<bot_replychat_t>() as
                                     libc::c_ulong) as *mut bot_replychat_t;
        (*replychat).keys = 0 as *mut bot_replychatkey_t;
        (*replychat).next = replychatlist;
        replychatlist = replychat;
        loop  {
            key =
                GetClearedHunkMemory(::std::mem::size_of::<bot_replychatkey_t>()
                                         as libc::c_ulong) as
                    *mut bot_replychatkey_t;
            (*key).flags = 0i32;
            (*key).string = 0 as *mut libc::c_char;
            (*key).match_0 = 0 as *mut bot_matchpiece_t;
            (*key).next = (*replychat).keys;
            (*replychat).keys = key;
            if 0 !=
                   PC_CheckTokenString(source,
                                       b"&\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char) {
                (*key).flags |= 1i32
            } else if 0 !=
                          PC_CheckTokenString(source,
                                              b"!\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char) {
                (*key).flags |= 2i32
            }
            if 0 !=
                   PC_CheckTokenString(source,
                                       b"name\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char) {
                (*key).flags |= 4i32
            } else if 0 !=
                          PC_CheckTokenString(source,
                                              b"female\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char) {
                (*key).flags |= 64i32
            } else if 0 !=
                          PC_CheckTokenString(source,
                                              b"male\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char) {
                (*key).flags |= 128i32
            } else if 0 !=
                          PC_CheckTokenString(source,
                                              b"it\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char) {
                (*key).flags |= 256i32
            } else if 0 !=
                          PC_CheckTokenString(source,
                                              b"(\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char) {
                (*key).flags |= 16i32;
                (*key).match_0 =
                    BotLoadMatchPieces(source,
                                       b")\x00" as *const u8 as
                                           *const libc::c_char as
                                           *mut libc::c_char);
                if (*key).match_0.is_null() {
                    BotFreeReplyChat(replychatlist);
                    return 0 as *mut bot_replychat_t
                }
            } else if 0 !=
                          PC_CheckTokenString(source,
                                              b"<\x00" as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char) {
                (*key).flags |= 32i32;
                strcpy(namebuffer.as_mut_ptr(),
                       b"\x00" as *const u8 as *const libc::c_char);
                loop  {
                    if 0 == PC_ExpectTokenType(source, 1i32, 0i32, &mut token)
                       {
                        BotFreeReplyChat(replychatlist);
                        FreeSource(source);
                        return 0 as *mut bot_replychat_t
                    }
                    StripDoubleQuotes(token.string.as_mut_ptr());
                    if 0 != strlen(namebuffer.as_mut_ptr()) {
                        strcat(namebuffer.as_mut_ptr(),
                               b"\\\x00" as *const u8 as *const libc::c_char);
                    }
                    strcat(namebuffer.as_mut_ptr(),
                           token.string.as_mut_ptr());
                    if !(0 !=
                             PC_CheckTokenString(source,
                                                 b",\x00" as *const u8 as
                                                     *const libc::c_char as
                                                     *mut libc::c_char)) {
                        break ;
                    }
                }
                if 0 ==
                       PC_ExpectTokenString(source,
                                            b">\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) {
                    BotFreeReplyChat(replychatlist);
                    FreeSource(source);
                    return 0 as *mut bot_replychat_t
                }
                (*key).string =
                    GetClearedHunkMemory(strlen(namebuffer.as_mut_ptr()).wrapping_add(1i32
                                                                                          as
                                                                                          libc::c_ulong))
                        as *mut libc::c_char;
                strcpy((*key).string, namebuffer.as_mut_ptr());
            } else {
                (*key).flags |= 8i32;
                if 0 == PC_ExpectTokenType(source, 1i32, 0i32, &mut token) {
                    BotFreeReplyChat(replychatlist);
                    FreeSource(source);
                    return 0 as *mut bot_replychat_t
                }
                StripDoubleQuotes(token.string.as_mut_ptr());
                (*key).string =
                    GetClearedHunkMemory(strlen(token.string.as_mut_ptr()).wrapping_add(1i32
                                                                                            as
                                                                                            libc::c_ulong))
                        as *mut libc::c_char;
                strcpy((*key).string, token.string.as_mut_ptr());
            }
            PC_CheckTokenString(source,
                                b",\x00" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char);
            if !(0 ==
                     PC_CheckTokenString(source,
                                         b"]\x00" as *const u8 as
                                             *const libc::c_char as
                                             *mut libc::c_char)) {
                break ;
            }
        }
        BotCheckValidReplyChatKeySet(source, (*replychat).keys);
        if 0 ==
               PC_ExpectTokenString(source,
                                    b"=\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) ||
               0 == PC_ExpectTokenType(source, 3i32, 0i32, &mut token) {
            BotFreeReplyChat(replychatlist);
            FreeSource(source);
            return 0 as *mut bot_replychat_t
        }
        (*replychat).priority = token.floatvalue;
        if 0 ==
               PC_ExpectTokenString(source,
                                    b"{\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) {
            BotFreeReplyChat(replychatlist);
            FreeSource(source);
            return 0 as *mut bot_replychat_t
        }
        (*replychat).numchatmessages = 0i32;
        while 0 ==
                  PC_CheckTokenString(source,
                                      b"}\x00" as *const u8 as
                                          *const libc::c_char as
                                          *mut libc::c_char) {
            if 0 == BotLoadChatMessage(source, chatmessagestring.as_mut_ptr())
               {
                BotFreeReplyChat(replychatlist);
                FreeSource(source);
                return 0 as *mut bot_replychat_t
            }
            chatmessage =
                GetClearedHunkMemory((::std::mem::size_of::<bot_chatmessage_t>()
                                          as
                                          libc::c_ulong).wrapping_add(strlen(chatmessagestring.as_mut_ptr())).wrapping_add(1i32
                                                                                                                               as
                                                                                                                               libc::c_ulong))
                    as *mut bot_chatmessage_t;
            (*chatmessage).chatmessage =
                (chatmessage as
                     *mut libc::c_char).offset(::std::mem::size_of::<bot_chatmessage_t>()
                                                   as libc::c_ulong as isize);
            strcpy((*chatmessage).chatmessage,
                   chatmessagestring.as_mut_ptr());
            (*chatmessage).time = (-2i32 * 20i32) as libc::c_float;
            (*chatmessage).next = (*replychat).firstchatmessage;
            (*replychat).firstchatmessage = chatmessage;
            (*replychat).numchatmessages += 1
        }
    }
    FreeSource(source);
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"loaded %s\n\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        filename);
    if 0 != botDeveloper { BotCheckReplyChatIntegrety(replychatlist); }
    if replychatlist.is_null() {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"no rchats\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
    }
    return replychatlist;
}
//end for
//end of the function BotCheckInitialChatIntegrety
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotCheckReplyChatIntegrety(mut replychat:
                                                        *mut bot_replychat_t) {
    let mut rp: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut cm: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut stringlist: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    let mut s: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    let mut nexts: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    stringlist = 0 as *mut bot_stringlist_t;
    rp = replychat;
    while !rp.is_null() {
        cm = (*rp).firstchatmessage;
        while !cm.is_null() {
            stringlist =
                BotCheckChatMessageIntegrety((*cm).chatmessage, stringlist);
            cm = (*cm).next
        }
        rp = (*rp).next
    }
    s = stringlist;
    while !s.is_null() {
        nexts = (*s).next;
        FreeMemory(s as *mut libc::c_void);
        s = nexts
    };
}
//end of the function BotFindStringInList
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotCheckChatMessageIntegrety(mut message:
                                                          *mut libc::c_char,
                                                      mut stringlist:
                                                          *mut bot_stringlist_t)
 -> *mut bot_stringlist_t {
    let mut i: libc::c_int = 0;
    let mut msgptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: [libc::c_char; 256] = [0; 256];
    let mut s: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    msgptr = message;
    while 0 != *msgptr {
        if *msgptr as libc::c_int == 0x1i32 {
            msgptr = msgptr.offset(1isize);
            match *msgptr as libc::c_int {
                118 => {
                    msgptr = msgptr.offset(1isize);
                    while 0 != *msgptr as libc::c_int &&
                              *msgptr as libc::c_int != 0x1i32 {
                        msgptr = msgptr.offset(1isize)
                    }
                    if 0 != *msgptr { msgptr = msgptr.offset(1isize) }
                }
                114 => {
                    msgptr = msgptr.offset(1isize);
                    i = 0i32;
                    while 0 != *msgptr as libc::c_int &&
                              *msgptr as libc::c_int != 0x1i32 {
                        let fresh6 = msgptr;
                        msgptr = msgptr.offset(1);
                        temp[i as usize] = *fresh6;
                        i += 1
                    }
                    temp[i as usize] = '\u{0}' as i32 as libc::c_char;
                    if 0 != *msgptr { msgptr = msgptr.offset(1isize) }
                    if RandomString(temp.as_mut_ptr()).is_null() {
                        if BotFindStringInList(stringlist,
                                               temp.as_mut_ptr()).is_null() {
                            Log_Write(b"%s = {\"%s\"} //MISSING RANDOM\r\n\x00"
                                          as *const u8 as *const libc::c_char
                                          as *mut libc::c_char,
                                      temp.as_mut_ptr(), temp.as_mut_ptr());
                            s =
                                GetClearedMemory((::std::mem::size_of::<bot_stringlist_t>()
                                                      as
                                                      libc::c_ulong).wrapping_add(strlen(temp.as_mut_ptr())).wrapping_add(1i32
                                                                                                                              as
                                                                                                                              libc::c_ulong))
                                    as *mut bot_stringlist_t;
                            (*s).string =
                                (s as
                                     *mut libc::c_char).offset(::std::mem::size_of::<bot_stringlist_t>()
                                                                   as
                                                                   libc::c_ulong
                                                                   as isize);
                            strcpy((*s).string, temp.as_mut_ptr());
                            (*s).next = stringlist;
                            stringlist = s
                        }
                    }
                }
                _ => {
                    //end if
                    //end if
                    botimport.Print.expect("non-null function pointer")(4i32,
                                                                        b"BotCheckChatMessageIntegrety: message \"%s\" invalid escape char\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        message);
                }
            }
        } else { msgptr = msgptr.offset(1isize) }
    }
    return stringlist;
}
//end else
//end of the function BotMatchVariable
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFindStringInList(mut list: *mut bot_stringlist_t,
                                             mut string: *mut libc::c_char)
 -> *mut bot_stringlist_t {
    let mut s: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    s = list;
    while !s.is_null() {
        if 0 == strcmp((*s).string, string) { return s }
        s = (*s).next
    }
    return 0 as *mut bot_stringlist_t;
}
//end of the function BotLoadRandomStrings
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn RandomString(mut name: *mut libc::c_char)
 -> *mut libc::c_char {
    let mut random: *mut bot_randomlist_t = 0 as *mut bot_randomlist_t;
    let mut rs: *mut bot_randomstring_t = 0 as *mut bot_randomstring_t;
    let mut i: libc::c_int = 0;
    random = randomstrings;
    while !random.is_null() {
        if 0 == strcmp((*random).string, name) {
            i =
                ((rand() & 0x7fffi32) as libc::c_float /
                     0x7fffi32 as libc::c_float *
                     (*random).numstrings as libc::c_float) as libc::c_int;
            rs = (*random).firstrandomstring;
            while !rs.is_null() {
                i -= 1;
                if i < 0i32 { break ; }
                rs = (*rs).next
            }
            if !rs.is_null() { return (*rs).string }
        }
        random = (*random).next
    }
    return 0 as *mut libc::c_char;
}
//list with random strings
#[no_mangle]
pub static mut randomstrings: *mut bot_randomlist_t =
    0 as *const bot_randomlist_t as *mut bot_randomlist_t;
//end for
//end of the function BotDumpReplyChat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFreeReplyChat(mut replychat:
                                              *mut bot_replychat_t) {
    let mut rp: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut nextrp: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut nextkey: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut cm: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut nextcm: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    rp = replychat;
    while !rp.is_null() {
        nextrp = (*rp).next;
        key = (*rp).keys;
        while !key.is_null() {
            nextkey = (*key).next;
            if !(*key).match_0.is_null() {
                BotFreeMatchPieces((*key).match_0);
            }
            if !(*key).string.is_null() {
                FreeMemory((*key).string as *mut libc::c_void);
            }
            FreeMemory(key as *mut libc::c_void);
            key = nextkey
        }
        cm = (*rp).firstchatmessage;
        while !cm.is_null() {
            nextcm = (*cm).next;
            FreeMemory(cm as *mut libc::c_void);
            cm = nextcm
        }
        FreeMemory(rp as *mut libc::c_void);
        rp = nextrp
    };
}
//end for
//end of the function BotDumpMatchTemplates
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFreeMatchPieces(mut matchpieces:
                                                *mut bot_matchpiece_t) {
    let mut mp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut nextmp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut ms: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    let mut nextms: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    mp = matchpieces;
    while !mp.is_null() {
        nextmp = (*mp).next;
        if (*mp).type_0 == 2i32 {
            ms = (*mp).firststring;
            while !ms.is_null() {
                nextms = (*ms).next;
                FreeMemory(ms as *mut libc::c_void);
                ms = nextms
            }
        }
        FreeMemory(mp as *mut libc::c_void);
        mp = nextmp
    };
}
//end while
//end of the function BotReplaceReplySynonyms
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLoadChatMessage(mut source: *mut source_t,
                                            mut chatmessagestring:
                                                *mut libc::c_char)
 -> libc::c_int {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    ptr = chatmessagestring;
    *ptr = 0i32 as libc::c_char;
    loop  {
        if 0 == PC_ExpectAnyToken(source, &mut token) {
            return qfalse as libc::c_int
        }
        if token.type_0 == 1i32 {
            StripDoubleQuotes(token.string.as_mut_ptr());
            if strlen(ptr).wrapping_add(strlen(token.string.as_mut_ptr())).wrapping_add(1i32
                                                                                            as
                                                                                            libc::c_ulong)
                   > 256i32 as libc::c_ulong {
                SourceError(source,
                            b"chat message too long\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
                return qfalse as libc::c_int
            }
            strcat(ptr, token.string.as_mut_ptr());
        } else if token.type_0 == 3i32 && 0 != token.subtype & 0x1000i32 {
            if strlen(ptr).wrapping_add(7i32 as libc::c_ulong) >
                   256i32 as libc::c_ulong {
                SourceError(source,
                            b"chat message too long\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
                return qfalse as libc::c_int
            }
            sprintf(&mut *ptr.offset(strlen(ptr) as isize) as
                        *mut libc::c_char,
                    b"%cv%ld%c\x00" as *const u8 as *const libc::c_char,
                    0x1i32, token.intvalue, 0x1i32);
        } else if token.type_0 == 4i32 {
            if strlen(ptr).wrapping_add(7i32 as libc::c_ulong) >
                   256i32 as libc::c_ulong {
                SourceError(source,
                            b"chat message too long\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char);
                return qfalse as libc::c_int
            }
            sprintf(&mut *ptr.offset(strlen(ptr) as isize) as
                        *mut libc::c_char,
                    b"%cr%s%c\x00" as *const u8 as *const libc::c_char,
                    0x1i32, token.string.as_mut_ptr(), 0x1i32);
        } else {
            SourceError(source,
                        b"unknown message component %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            return qfalse as libc::c_int
        }
        //end else
        if 0 !=
               PC_CheckTokenString(source,
                                   b";\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char) {
            break ;
        }
        if 0 ==
               PC_ExpectTokenString(source,
                                    b",\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) {
            return qfalse as libc::c_int
        }
    }
    return qtrue as libc::c_int;
}
//end for
//end of the function BotFreeReplyChat
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotCheckValidReplyChatKeySet(mut source:
                                                          *mut source_t,
                                                      mut keys:
                                                          *mut bot_replychatkey_t) {
    let mut allprefixed: libc::c_int = 0;
    let mut hasvariableskey: libc::c_int = 0;
    let mut hasstringkey: libc::c_int = 0;
    let mut m: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut ms: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut key2: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    allprefixed = qtrue as libc::c_int;
    hasstringkey = qfalse as libc::c_int;
    hasvariableskey = hasstringkey;
    key = keys;
    while !key.is_null() {
        if 0 == (*key).flags & (1i32 | 2i32) {
            allprefixed = qfalse as libc::c_int;
            if 0 != (*key).flags & 16i32 {
                m = (*key).match_0;
                while !m.is_null() {
                    if (*m).type_0 == 1i32 {
                        hasvariableskey = qtrue as libc::c_int
                    }
                    m = (*m).next
                }
            } else if 0 != (*key).flags & 8i32 {
                hasstringkey = qtrue as libc::c_int
            }
        } else if 0 != (*key).flags & 1i32 && 0 != (*key).flags & 8i32 {
            key2 = keys;
            while !key2.is_null() {
                if !(key2 == key) {
                    if !(0 != (*key2).flags & 2i32) {
                        if 0 != (*key2).flags & 16i32 {
                            m = (*key2).match_0;
                            while !m.is_null() {
                                if (*m).type_0 == 2i32 {
                                    ms = (*m).firststring;
                                    while !ms.is_null() {
                                        if StringContains((*ms).string,
                                                          (*key).string,
                                                          qfalse as
                                                              libc::c_int) !=
                                               -1i32 {
                                            break ;
                                        }
                                        ms = (*ms).next
                                    }
                                    //end if
                                    //end for
                                    if !ms.is_null() { break ; }
                                } else if (*m).type_0 == 1i32 { break ; }
                                m = (*m).next
                            }
                            if m.is_null() {
                                SourceWarning(source,
                                              b"one of the match templates does not leave space for the key %s with the & prefix\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char,
                                              (*key).string);
                            }
                        }
                    }
                }
                key2 = (*key2).next
            }
        }
        if 0 != (*key).flags & 2i32 && 0 != (*key).flags & 8i32 {
            key2 = keys;
            while !key2.is_null() {
                if !(key2 == key) {
                    if !(0 != (*key2).flags & 2i32) {
                        if 0 != (*key2).flags & 8i32 {
                            if StringContains((*key2).string, (*key).string,
                                              qfalse as libc::c_int) != -1i32
                               {
                                SourceWarning(source,
                                              b"the key %s with prefix ! is inside the key %s\x00"
                                                  as *const u8 as
                                                  *const libc::c_char as
                                                  *mut libc::c_char,
                                              (*key).string, (*key2).string);
                            }
                        } else if 0 != (*key2).flags & 16i32 {
                            m = (*key2).match_0;
                            while !m.is_null() {
                                if (*m).type_0 == 2i32 {
                                    ms = (*m).firststring;
                                    while !ms.is_null() {
                                        if StringContains((*ms).string,
                                                          (*key).string,
                                                          qfalse as
                                                              libc::c_int) !=
                                               -1i32 {
                                            SourceWarning(source,
                                                          b"the key %s with prefix ! is inside the match template string %s\x00"
                                                              as *const u8 as
                                                              *const libc::c_char
                                                              as
                                                              *mut libc::c_char,
                                                          (*key).string,
                                                          (*ms).string);
                                        }
                                        ms = (*ms).next
                                    }
                                }
                                m = (*m).next
                            }
                        }
                    }
                }
                key2 = (*key2).next
            }
        }
        key = (*key).next
    }
    if 0 != allprefixed {
        SourceWarning(source,
                      b"all keys have a & or ! prefix\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char);
    }
    if 0 != hasvariableskey && 0 != hasstringkey {
        SourceWarning(source,
                      b"variables from the match template(s) could be invalid when outputting one of the chat messages\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char);
    };
}
//checks if the first string contains the second one, returns index into first string or -1 if not found
#[no_mangle]
pub unsafe extern "C" fn StringContains(mut str1: *mut libc::c_char,
                                        mut str2: *mut libc::c_char,
                                        mut casesensitive: libc::c_int)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    if str1.is_null() || str2.is_null() { return -1i32 }
    len = strlen(str1).wrapping_sub(strlen(str2)) as libc::c_int;
    index = 0i32;
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
        if 0 == *str2.offset(j as isize) { return index }
        i += 1;
        str1 = str1.offset(1isize);
        index += 1
    }
    return -1i32;
}
//end for
//end of the function BotFreeMatchPieces
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLoadMatchPieces(mut source: *mut source_t,
                                            mut endtoken: *mut libc::c_char)
 -> *mut bot_matchpiece_t {
    let mut lastwasvariable: libc::c_int = 0;
    let mut emptystring: libc::c_int = 0;
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    let mut matchpiece: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut firstpiece: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut lastpiece: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut matchstring: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    let mut lastmatchstring: *mut bot_matchstring_t =
        0 as *mut bot_matchstring_t;
    firstpiece = 0 as *mut bot_matchpiece_t;
    lastpiece = 0 as *mut bot_matchpiece_t;
    lastwasvariable = qfalse as libc::c_int;
    while 0 != PC_ReadToken(source, &mut token) {
        if token.type_0 == 3i32 && 0 != token.subtype & 0x1000i32 {
            if token.intvalue >= 8i32 as libc::c_ulong {
                SourceError(source,
                            b"can\'t have more than %d match variables\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char, 8i32);
                FreeSource(source);
                BotFreeMatchPieces(firstpiece);
                return 0 as *mut bot_matchpiece_t
            }
            if 0 != lastwasvariable {
                SourceError(source,
                            b"not allowed to have adjacent variables\x00" as
                                *const u8 as *const libc::c_char as
                                *mut libc::c_char);
                FreeSource(source);
                BotFreeMatchPieces(firstpiece);
                return 0 as *mut bot_matchpiece_t
            }
            lastwasvariable = qtrue as libc::c_int;
            matchpiece =
                GetClearedHunkMemory(::std::mem::size_of::<bot_matchpiece_t>()
                                         as libc::c_ulong) as
                    *mut bot_matchpiece_t;
            (*matchpiece).type_0 = 1i32;
            (*matchpiece).variable = token.intvalue as libc::c_int;
            (*matchpiece).next = 0 as *mut bot_matchpiece_s;
            if !lastpiece.is_null() {
                (*lastpiece).next = matchpiece
            } else { firstpiece = matchpiece }
            lastpiece = matchpiece
        } else if token.type_0 == 1i32 {
            matchpiece =
                GetClearedHunkMemory(::std::mem::size_of::<bot_matchpiece_t>()
                                         as libc::c_ulong) as
                    *mut bot_matchpiece_t;
            (*matchpiece).firststring = 0 as *mut bot_matchstring_t;
            (*matchpiece).type_0 = 2i32;
            (*matchpiece).variable = 0i32;
            (*matchpiece).next = 0 as *mut bot_matchpiece_s;
            if !lastpiece.is_null() {
                (*lastpiece).next = matchpiece
            } else { firstpiece = matchpiece }
            lastpiece = matchpiece;
            lastmatchstring = 0 as *mut bot_matchstring_t;
            emptystring = qfalse as libc::c_int;
            loop  {
                if !(*matchpiece).firststring.is_null() {
                    if 0 == PC_ExpectTokenType(source, 1i32, 0i32, &mut token)
                       {
                        FreeSource(source);
                        BotFreeMatchPieces(firstpiece);
                        return 0 as *mut bot_matchpiece_t
                    }
                }
                StripDoubleQuotes(token.string.as_mut_ptr());
                matchstring =
                    GetClearedHunkMemory((::std::mem::size_of::<bot_matchstring_t>()
                                              as
                                              libc::c_ulong).wrapping_add(strlen(token.string.as_mut_ptr())).wrapping_add(1i32
                                                                                                                              as
                                                                                                                              libc::c_ulong))
                        as *mut bot_matchstring_t;
                (*matchstring).string =
                    (matchstring as
                         *mut libc::c_char).offset(::std::mem::size_of::<bot_matchstring_t>()
                                                       as libc::c_ulong as
                                                       isize);
                strcpy((*matchstring).string, token.string.as_mut_ptr());
                if 0 == strlen(token.string.as_mut_ptr()) {
                    emptystring = qtrue as libc::c_int
                }
                (*matchstring).next = 0 as *mut bot_matchstring_s;
                if !lastmatchstring.is_null() {
                    (*lastmatchstring).next = matchstring
                } else { (*matchpiece).firststring = matchstring }
                lastmatchstring = matchstring;
                if !(0 !=
                         PC_CheckTokenString(source,
                                             b"|\x00" as *const u8 as
                                                 *const libc::c_char as
                                                 *mut libc::c_char)) {
                    break ;
                }
            }
            if 0 == emptystring { lastwasvariable = qfalse as libc::c_int }
        } else {
            SourceError(source,
                        b"invalid token %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            FreeSource(source);
            BotFreeMatchPieces(firstpiece);
            return 0 as *mut bot_matchpiece_t
        }
        //end else
        if 0 != PC_CheckTokenString(source, endtoken) { break ; }
        if 0 ==
               PC_ExpectTokenString(source,
                                    b",\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) {
            FreeSource(source);
            BotFreeMatchPieces(firstpiece);
            return 0 as *mut bot_matchpiece_t
        }
    }
    return firstpiece;
}
//reply chats
#[no_mangle]
pub static mut replychats: *mut bot_replychat_t =
    0 as *const bot_replychat_t as *mut bot_replychat_t;
//end for
//end of the function BotFreeMatchTemplates
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLoadMatchTemplates(mut matchfile:
                                                   *mut libc::c_char)
 -> *mut bot_matchtemplate_t {
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    let mut matchtemplate: *mut bot_matchtemplate_t =
        0 as *mut bot_matchtemplate_t;
    let mut matches: *mut bot_matchtemplate_t = 0 as *mut bot_matchtemplate_t;
    let mut lastmatch: *mut bot_matchtemplate_t =
        0 as *mut bot_matchtemplate_t;
    let mut context: libc::c_ulong = 0;
    PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *const libc::c_char as
                         *mut libc::c_char);
    source = LoadSourceFile(matchfile);
    if source.is_null() {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"counldn\'t load %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            matchfile);
        return 0 as *mut bot_matchtemplate_t
    }
    matches = 0 as *mut bot_matchtemplate_t;
    lastmatch = 0 as *mut bot_matchtemplate_t;
    while 0 != PC_ReadToken(source, &mut token) {
        if token.type_0 != 3i32 || 0 == token.subtype & 0x1000i32 {
            SourceError(source,
                        b"expected integer, found %s\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        token.string.as_mut_ptr());
            BotFreeMatchTemplates(matches);
            FreeSource(source);
            return 0 as *mut bot_matchtemplate_t
        }
        context = token.intvalue;
        if 0 ==
               PC_ExpectTokenString(source,
                                    b"{\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char) {
            BotFreeMatchTemplates(matches);
            FreeSource(source);
            return 0 as *mut bot_matchtemplate_t
        }
        while 0 != PC_ReadToken(source, &mut token) {
            if 0 ==
                   strcmp(token.string.as_mut_ptr(),
                          b"}\x00" as *const u8 as *const libc::c_char) {
                break ;
            }
            PC_UnreadLastToken(source);
            matchtemplate =
                GetClearedHunkMemory(::std::mem::size_of::<bot_matchtemplate_t>()
                                         as libc::c_ulong) as
                    *mut bot_matchtemplate_t;
            (*matchtemplate).context = context;
            (*matchtemplate).next = 0 as *mut bot_matchtemplate_s;
            if !lastmatch.is_null() {
                (*lastmatch).next = matchtemplate
            } else { matches = matchtemplate }
            lastmatch = matchtemplate;
            (*matchtemplate).first =
                BotLoadMatchPieces(source,
                                   b"=\x00" as *const u8 as
                                       *const libc::c_char as
                                       *mut libc::c_char);
            if (*matchtemplate).first.is_null() {
                BotFreeMatchTemplates(matches);
                return 0 as *mut bot_matchtemplate_t
            }
            if 0 ==
                   PC_ExpectTokenString(source,
                                        b"(\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char) ||
                   0 ==
                       PC_ExpectTokenType(source, 3i32, 0x1000i32, &mut token)
               {
                BotFreeMatchTemplates(matches);
                FreeSource(source);
                return 0 as *mut bot_matchtemplate_t
            }
            (*matchtemplate).type_0 = token.intvalue as libc::c_int;
            if 0 ==
                   PC_ExpectTokenString(source,
                                        b",\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char) ||
                   0 ==
                       PC_ExpectTokenType(source, 3i32, 0x1000i32, &mut token)
               {
                BotFreeMatchTemplates(matches);
                FreeSource(source);
                return 0 as *mut bot_matchtemplate_t
            }
            (*matchtemplate).subtype = token.intvalue as libc::c_int;
            if 0 ==
                   PC_ExpectTokenString(source,
                                        b")\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char) ||
                   0 ==
                       PC_ExpectTokenString(source,
                                            b";\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) {
                BotFreeMatchTemplates(matches);
                FreeSource(source);
                return 0 as *mut bot_matchtemplate_t
            }
        }
    }
    FreeSource(source);
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"loaded %s\n\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        matchfile);
    return matches;
}
//end of the function BotLoadMatchPieces
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFreeMatchTemplates(mut mt:
                                                   *mut bot_matchtemplate_t) {
    let mut nextmt: *mut bot_matchtemplate_t = 0 as *mut bot_matchtemplate_t;
    while !mt.is_null() {
        nextmt = (*mt).next;
        BotFreeMatchPieces((*mt).first);
        FreeMemory(mt as *mut libc::c_void);
        mt = nextmt
    };
}
//list with match strings
#[no_mangle]
pub static mut matchtemplates: *mut bot_matchtemplate_t =
    0 as *const bot_matchtemplate_t as *mut bot_matchtemplate_t;
//end for
//end for
//end of the function BotDumpRandomStringList
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLoadRandomStrings(mut filename: *mut libc::c_char)
 -> *mut bot_randomlist_t {
    let mut pass: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut chatmessagestring: [libc::c_char; 256] = [0; 256];
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    let mut randomlist: *mut bot_randomlist_t = 0 as *mut bot_randomlist_t;
    let mut lastrandom: *mut bot_randomlist_t = 0 as *mut bot_randomlist_t;
    let mut random: *mut bot_randomlist_t = 0 as *mut bot_randomlist_t;
    let mut randomstring: *mut bot_randomstring_t =
        0 as *mut bot_randomstring_t;
    size = 0i32;
    randomlist = 0 as *mut bot_randomlist_t;
    random = 0 as *mut bot_randomlist_t;
    pass = 0i32;
    while pass < 2i32 {
        if 0 != pass && 0 != size {
            ptr =
                GetClearedHunkMemory(size as libc::c_ulong) as
                    *mut libc::c_char
        }
        PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char);
        source = LoadSourceFile(filename);
        if source.is_null() {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"counldn\'t load %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                filename);
            return 0 as *mut bot_randomlist_t
        }
        randomlist = 0 as *mut bot_randomlist_t;
        lastrandom = 0 as *mut bot_randomlist_t;
        while 0 != PC_ReadToken(source, &mut token) {
            let mut len: size_t = 0;
            if token.type_0 != 4i32 {
                SourceError(source,
                            b"unknown random %s\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            token.string.as_mut_ptr());
                FreeSource(source);
                return 0 as *mut bot_randomlist_t
            }
            len =
                strlen(token.string.as_mut_ptr()).wrapping_add(1i32 as
                                                                   libc::c_ulong);
            len =
                len.wrapping_add(::std::mem::size_of::<libc::c_long>() as
                                     libc::c_ulong).wrapping_sub(1i32 as
                                                                     libc::c_ulong)
                    &
                    !(::std::mem::size_of::<libc::c_long>() as
                          libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong);
            size =
                (size as
                     libc::c_ulong).wrapping_add((::std::mem::size_of::<bot_randomlist_t>()
                                                      as
                                                      libc::c_ulong).wrapping_add(len))
                    as libc::c_int as libc::c_int;
            if 0 != pass && !ptr.is_null() {
                random = ptr as *mut bot_randomlist_t;
                ptr =
                    ptr.offset(::std::mem::size_of::<bot_randomlist_t>() as
                                   libc::c_ulong as isize);
                (*random).string = ptr;
                ptr = ptr.offset(len as isize);
                strcpy((*random).string, token.string.as_mut_ptr());
                (*random).firstrandomstring = 0 as *mut bot_randomstring_t;
                (*random).numstrings = 0i32;
                if !lastrandom.is_null() {
                    (*lastrandom).next = random
                } else { randomlist = random }
                lastrandom = random
            }
            if 0 ==
                   PC_ExpectTokenString(source,
                                        b"=\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char) ||
                   0 ==
                       PC_ExpectTokenString(source,
                                            b"{\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) {
                FreeSource(source);
                return 0 as *mut bot_randomlist_t
            }
            while 0 ==
                      PC_CheckTokenString(source,
                                          b"}\x00" as *const u8 as
                                              *const libc::c_char as
                                              *mut libc::c_char) {
                if 0 ==
                       BotLoadChatMessage(source,
                                          chatmessagestring.as_mut_ptr()) {
                    FreeSource(source);
                    return 0 as *mut bot_randomlist_t
                }
                len =
                    strlen(chatmessagestring.as_mut_ptr()).wrapping_add(1i32
                                                                            as
                                                                            libc::c_ulong);
                len =
                    len.wrapping_add(::std::mem::size_of::<libc::c_long>() as
                                         libc::c_ulong).wrapping_sub(1i32 as
                                                                         libc::c_ulong)
                        &
                        !(::std::mem::size_of::<libc::c_long>() as
                              libc::c_ulong).wrapping_sub(1i32 as
                                                              libc::c_ulong);
                size =
                    (size as
                         libc::c_ulong).wrapping_add((::std::mem::size_of::<bot_randomstring_t>()
                                                          as
                                                          libc::c_ulong).wrapping_add(len))
                        as libc::c_int as libc::c_int;
                if 0 != pass && !ptr.is_null() {
                    randomstring = ptr as *mut bot_randomstring_t;
                    ptr =
                        ptr.offset(::std::mem::size_of::<bot_randomstring_t>()
                                       as libc::c_ulong as isize);
                    (*randomstring).string = ptr;
                    ptr = ptr.offset(len as isize);
                    strcpy((*randomstring).string,
                           chatmessagestring.as_mut_ptr());
                    (*random).numstrings += 1;
                    (*randomstring).next = (*random).firstrandomstring;
                    (*random).firstrandomstring = randomstring
                }
            }
        }
        FreeSource(source);
        pass += 1
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"loaded %s\n\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        filename);
    return randomlist;
}
//end for
//end of the function BotDumpSynonymList
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLoadSynonyms(mut filename: *mut libc::c_char)
 -> *mut bot_synonymlist_t {
    let mut pass: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut contextlevel: libc::c_int = 0;
    let mut numsynonyms: libc::c_int = 0;
    let mut context: libc::c_ulong = 0;
    let mut contextstack: [libc::c_ulong; 32] = [0; 32];
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    let mut synlist: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut lastsyn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut syn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut synonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    let mut lastsynonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    size = 0i32;
    synlist = 0 as *mut bot_synonymlist_t;
    syn = 0 as *mut bot_synonymlist_t;
    synonym = 0 as *mut bot_synonym_t;
    pass = 0i32;
    while pass < 2i32 {
        if 0 != pass && 0 != size {
            ptr =
                GetClearedHunkMemory(size as libc::c_ulong) as
                    *mut libc::c_char
        }
        PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char);
        source = LoadSourceFile(filename);
        if source.is_null() {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"counldn\'t load %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                filename);
            return 0 as *mut bot_synonymlist_t
        }
        context = 0i32 as libc::c_ulong;
        contextlevel = 0i32;
        synlist = 0 as *mut bot_synonymlist_t;
        lastsyn = 0 as *mut bot_synonymlist_t;
        while 0 != PC_ReadToken(source, &mut token) {
            if token.type_0 == 3i32 {
                context |= token.intvalue;
                contextstack[contextlevel as usize] = token.intvalue;
                contextlevel += 1;
                if contextlevel >= 32i32 {
                    SourceError(source,
                                b"more than 32 context levels\x00" as
                                    *const u8 as *const libc::c_char as
                                    *mut libc::c_char);
                    FreeSource(source);
                    return 0 as *mut bot_synonymlist_t
                }
                if 0 ==
                       PC_ExpectTokenString(source,
                                            b"{\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) {
                    FreeSource(source);
                    return 0 as *mut bot_synonymlist_t
                }
            } else if token.type_0 == 5i32 {
                if 0 ==
                       strcmp(token.string.as_mut_ptr(),
                              b"}\x00" as *const u8 as *const libc::c_char) {
                    contextlevel -= 1;
                    if contextlevel < 0i32 {
                        SourceError(source,
                                    b"too many }\x00" as *const u8 as
                                        *const libc::c_char as
                                        *mut libc::c_char);
                        FreeSource(source);
                        return 0 as *mut bot_synonymlist_t
                    }
                    context &= !contextstack[contextlevel as usize]
                } else if 0 ==
                              strcmp(token.string.as_mut_ptr(),
                                     b"[\x00" as *const u8 as
                                         *const libc::c_char) {
                    size =
                        (size as
                             libc::c_ulong).wrapping_add(::std::mem::size_of::<bot_synonymlist_t>()
                                                             as libc::c_ulong)
                            as libc::c_int as libc::c_int;
                    if 0 != pass && !ptr.is_null() {
                        syn = ptr as *mut bot_synonymlist_t;
                        ptr =
                            ptr.offset(::std::mem::size_of::<bot_synonymlist_t>()
                                           as libc::c_ulong as isize);
                        (*syn).context = context;
                        (*syn).firstsynonym = 0 as *mut bot_synonym_t;
                        (*syn).next = 0 as *mut bot_synonymlist_s;
                        if !lastsyn.is_null() {
                            (*lastsyn).next = syn
                        } else { synlist = syn }
                        lastsyn = syn
                    }
                    numsynonyms = 0i32;
                    lastsynonym = 0 as *mut bot_synonym_t;
                    loop  {
                        let mut len: size_t = 0;
                        if 0 ==
                               PC_ExpectTokenString(source,
                                                    b"(\x00" as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char) ||
                               0 ==
                                   PC_ExpectTokenType(source, 1i32, 0i32,
                                                      &mut token) {
                            FreeSource(source);
                            return 0 as *mut bot_synonymlist_t
                        }
                        StripDoubleQuotes(token.string.as_mut_ptr());
                        if strlen(token.string.as_mut_ptr()) <=
                               0i32 as libc::c_ulong {
                            SourceError(source,
                                        b"empty string\x00" as *const u8 as
                                            *const libc::c_char as
                                            *mut libc::c_char);
                            FreeSource(source);
                            return 0 as *mut bot_synonymlist_t
                        }
                        len =
                            strlen(token.string.as_mut_ptr()).wrapping_add(1i32
                                                                               as
                                                                               libc::c_ulong);
                        len =
                            len.wrapping_add(::std::mem::size_of::<libc::c_long>()
                                                 as
                                                 libc::c_ulong).wrapping_sub(1i32
                                                                                 as
                                                                                 libc::c_ulong)
                                &
                                !(::std::mem::size_of::<libc::c_long>() as
                                      libc::c_ulong).wrapping_sub(1i32 as
                                                                      libc::c_ulong);
                        size =
                            (size as
                                 libc::c_ulong).wrapping_add((::std::mem::size_of::<bot_synonym_t>()
                                                                  as
                                                                  libc::c_ulong).wrapping_add(len))
                                as libc::c_int as libc::c_int;
                        if 0 != pass && !ptr.is_null() {
                            synonym = ptr as *mut bot_synonym_t;
                            ptr =
                                ptr.offset(::std::mem::size_of::<bot_synonym_t>()
                                               as libc::c_ulong as isize);
                            (*synonym).string = ptr;
                            ptr = ptr.offset(len as isize);
                            strcpy((*synonym).string,
                                   token.string.as_mut_ptr());
                            if !lastsynonym.is_null() {
                                (*lastsynonym).next = synonym
                            } else { (*syn).firstsynonym = synonym }
                            lastsynonym = synonym
                        }
                        numsynonyms += 1;
                        if 0 ==
                               PC_ExpectTokenString(source,
                                                    b",\x00" as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char) ||
                               0 ==
                                   PC_ExpectTokenType(source, 3i32, 0i32,
                                                      &mut token) ||
                               0 ==
                                   PC_ExpectTokenString(source,
                                                        b")\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char)
                           {
                            FreeSource(source);
                            return 0 as *mut bot_synonymlist_t
                        }
                        if 0 != pass && !ptr.is_null() {
                            (*synonym).weight = token.floatvalue;
                            (*syn).totalweight += (*synonym).weight
                        }
                        //end if
                        if 0 !=
                               PC_CheckTokenString(source,
                                                   b"]\x00" as *const u8 as
                                                       *const libc::c_char as
                                                       *mut libc::c_char) {
                            break ;
                        }
                        if 0 ==
                               PC_ExpectTokenString(source,
                                                    b",\x00" as *const u8 as
                                                        *const libc::c_char as
                                                        *mut libc::c_char) {
                            FreeSource(source);
                            return 0 as *mut bot_synonymlist_t
                        }
                    }
                    if numsynonyms < 2i32 {
                        SourceError(source,
                                    b"synonym must have at least two entries\x00"
                                        as *const u8 as *const libc::c_char as
                                        *mut libc::c_char);
                        FreeSource(source);
                        return 0 as *mut bot_synonymlist_t
                    }
                } else {
                    SourceError(source,
                                b"unexpected %s\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                token.string.as_mut_ptr());
                    FreeSource(source);
                    return 0 as *mut bot_synonymlist_t
                }
            }
        }
        FreeSource(source);
        if contextlevel > 0i32 {
            SourceError(source,
                        b"missing }\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char);
            return 0 as *mut bot_synonymlist_t
        }
        pass += 1
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"loaded %s\n\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        filename);
    return synlist;
}
//list with synonyms
#[no_mangle]
pub static mut synonyms: *mut bot_synonymlist_t =
    0 as *const bot_synonymlist_t as *mut bot_synonymlist_t;
//shutdown the chat AI
#[no_mangle]
pub unsafe extern "C" fn BotShutdownChatAI() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 64i32 {
        if !botchatstates[i as usize].is_null() { BotFreeChatState(i); }
        i += 1
    }
    i = 0i32;
    while i < 64i32 {
        if !ichatdata[i as usize].is_null() {
            FreeMemory((*ichatdata[i as usize]).chat as *mut libc::c_void);
            FreeMemory(ichatdata[i as usize] as *mut libc::c_void);
            ichatdata[i as usize] = 0 as *mut bot_ichatdata_t
        }
        i += 1
    }
    if !consolemessageheap.is_null() {
        FreeMemory(consolemessageheap as *mut libc::c_void);
    }
    consolemessageheap = 0 as *mut bot_consolemessage_t;
    if !matchtemplates.is_null() { BotFreeMatchTemplates(matchtemplates); }
    matchtemplates = 0 as *mut bot_matchtemplate_t;
    if !randomstrings.is_null() {
        FreeMemory(randomstrings as *mut libc::c_void);
    }
    randomstrings = 0 as *mut bot_randomlist_t;
    if !synonyms.is_null() { FreeMemory(synonyms as *mut libc::c_void); }
    synonyms = 0 as *mut bot_synonymlist_t;
    if !replychats.is_null() { BotFreeReplyChat(replychats); }
    replychats = 0 as *mut bot_replychat_t;
}
#[no_mangle]
pub static mut ichatdata: [*mut bot_ichatdata_t; 64] =
    [0 as *const bot_ichatdata_t as *mut bot_ichatdata_t; 64];
//frees the chatstate
#[no_mangle]
pub unsafe extern "C" fn BotFreeChatState(mut handle: libc::c_int) {
    let mut m: bot_consolemessage_t =
        bot_consolemessage_s{handle: 0,
                             time: 0.,
                             type_0: 0,
                             message: [0; 256],
                             prev: 0 as *mut bot_consolemessage_s,
                             next: 0 as *mut bot_consolemessage_s,};
    let mut h: libc::c_int = 0;
    if handle <= 0i32 || handle > 64i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"chat state handle %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return
    }
    if botchatstates[handle as usize].is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"invalid chat state %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return
    }
    if 0. !=
           LibVarGetValue(b"bot_reloadcharacters\x00" as *const u8 as
                              *const libc::c_char) {
        BotFreeChatFile(handle);
    }
    h = BotNextConsoleMessage(handle, &mut m);
    while 0 != h {
        BotRemoveConsoleMessage(handle, h);
        h = BotNextConsoleMessage(handle, &mut m)
    }
    FreeMemory(botchatstates[handle as usize] as *mut libc::c_void);
    botchatstates[handle as usize] = 0 as *mut bot_chatstate_t;
}
#[no_mangle]
pub static mut botchatstates: [*mut bot_chatstate_t; 65] =
    [0 as *const bot_chatstate_t as *mut bot_chatstate_t; 65];
//removes the console message from the chat state
#[no_mangle]
pub unsafe extern "C" fn BotRemoveConsoleMessage(mut chatstate: libc::c_int,
                                                 mut handle: libc::c_int) {
    let mut m: *mut bot_consolemessage_t = 0 as *mut bot_consolemessage_t;
    let mut nextm: *mut bot_consolemessage_t = 0 as *mut bot_consolemessage_t;
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return }
    m = (*cs).firstmessage;
    while !m.is_null() {
        nextm = (*m).next;
        if (*m).handle == handle {
            if !(*m).next.is_null() {
                (*(*m).next).prev = (*m).prev
            } else { (*cs).lastmessage = (*m).prev }
            if !(*m).prev.is_null() {
                (*(*m).prev).next = (*m).next
            } else { (*cs).firstmessage = (*m).next }
            FreeConsoleMessage(m);
            (*cs).numconsolemessages -= 1;
            break ;
        } else { m = nextm }
    };
}
//end of the function AllocConsoleMessage
//===========================================================================
// deallocate one console message from the heap
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn FreeConsoleMessage(mut message:
                                                *mut bot_consolemessage_t) {
    if !freeconsolemessages.is_null() {
        (*freeconsolemessages).prev = message
    }
    (*message).prev = 0 as *mut bot_consolemessage_s;
    (*message).next = freeconsolemessages;
    freeconsolemessages = message;
}
//========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotChatStateFromHandle(mut handle: libc::c_int)
 -> *mut bot_chatstate_t {
    if handle <= 0i32 || handle > 64i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"chat state handle %d out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return 0 as *mut bot_chatstate_t
    }
    if botchatstates[handle as usize].is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"invalid chat state %d\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            handle);
        return 0 as *mut bot_chatstate_t
    }
    return botchatstates[handle as usize];
}
//returns the next console message from the state
#[no_mangle]
pub unsafe extern "C" fn BotNextConsoleMessage(mut chatstate: libc::c_int,
                                               mut cm:
                                                   *mut bot_consolemessage_t)
 -> libc::c_int {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    let mut firstmsg: *mut bot_consolemessage_t =
        0 as *mut bot_consolemessage_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return 0i32 }
    firstmsg = (*cs).firstmessage;
    if !firstmsg.is_null() {
        (*cm).handle = (*firstmsg).handle;
        (*cm).time = (*firstmsg).time;
        (*cm).type_0 = (*firstmsg).type_0;
        Q_strncpyz((*cm).message.as_mut_ptr(),
                   (*firstmsg).message.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 256]>() as
                       libc::c_ulong as libc::c_int);
        return (*cm).handle
    }
    return 0i32;
}
//end of the function BotLoadInitialChat
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotFreeChatFile(mut chatstate: libc::c_int) {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return }
    if !(*cs).chat.is_null() { FreeMemory((*cs).chat as *mut libc::c_void); }
    (*cs).chat = 0 as *mut bot_chat_t;
}
//returns the handle to a newly allocated chat state
#[no_mangle]
pub unsafe extern "C" fn BotAllocChatState() -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i <= 64i32 {
        if botchatstates[i as usize].is_null() {
            botchatstates[i as usize] =
                GetClearedMemory(::std::mem::size_of::<bot_chatstate_t>() as
                                     libc::c_ulong) as *mut bot_chatstate_t;
            return i
        }
        i += 1
    }
    return 0i32;
}
//adds a console message to the chat state
#[no_mangle]
pub unsafe extern "C" fn BotQueueConsoleMessage(mut chatstate: libc::c_int,
                                                mut type_0: libc::c_int,
                                                mut message:
                                                    *mut libc::c_char) {
    let mut m: *mut bot_consolemessage_t = 0 as *mut bot_consolemessage_t;
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return }
    m = AllocConsoleMessage();
    if m.is_null() {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"empty console message heap\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return
    }
    (*cs).handle += 1;
    if (*cs).handle <= 0i32 || (*cs).handle > 8192i32 { (*cs).handle = 1i32 }
    (*m).handle = (*cs).handle;
    (*m).time = AAS_Time();
    (*m).type_0 = type_0;
    Q_strncpyz((*m).message.as_mut_ptr(), message, 256i32);
    (*m).next = 0 as *mut bot_consolemessage_s;
    if !(*cs).lastmessage.is_null() {
        (*(*cs).lastmessage).next = m;
        (*m).prev = (*cs).lastmessage;
        (*cs).lastmessage = m
    } else {
        (*cs).lastmessage = m;
        (*cs).firstmessage = m;
        (*m).prev = 0 as *mut bot_consolemessage_s
    }
    (*cs).numconsolemessages += 1;
}
//end of the function InitConsoleMessageHeap
//===========================================================================
// allocate one console message from the heap
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn AllocConsoleMessage() -> *mut bot_consolemessage_t {
    let mut message: *mut bot_consolemessage_t =
        0 as *mut bot_consolemessage_t;
    message = freeconsolemessages;
    if !freeconsolemessages.is_null() {
        freeconsolemessages = (*freeconsolemessages).next
    }
    if !freeconsolemessages.is_null() {
        (*freeconsolemessages).prev = 0 as *mut bot_consolemessage_s
    }
    return message;
}
//returns the number of console messages currently stored in the state
#[no_mangle]
pub unsafe extern "C" fn BotNumConsoleMessages(mut chatstate: libc::c_int)
 -> libc::c_int {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return 0i32 }
    return (*cs).numconsolemessages;
}
//selects a chat message of the given type
#[no_mangle]
pub unsafe extern "C" fn BotInitialChat(mut chatstate: libc::c_int,
                                        mut type_0: *mut libc::c_char,
                                        mut mcontext: libc::c_int,
                                        mut var0: *mut libc::c_char,
                                        mut var1: *mut libc::c_char,
                                        mut var2: *mut libc::c_char,
                                        mut var3: *mut libc::c_char,
                                        mut var4: *mut libc::c_char,
                                        mut var5: *mut libc::c_char,
                                        mut var6: *mut libc::c_char,
                                        mut var7: *mut libc::c_char) {
    let mut message: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: libc::c_int = 0;
    let mut match_0: bot_match_t =
        bot_match_s{string: [0; 256],
                    type_0: 0,
                    subtype: 0,
                    variables:
                        [bot_matchvariable_s{offset: 0, length: 0,}; 8],};
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return }
    if (*cs).chat.is_null() { return }
    message = BotChooseInitialChatMessage(cs, type_0);
    if message.is_null() { return }
    memset(&mut match_0 as *mut bot_match_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<bot_match_t>() as libc::c_ulong);
    index = 0i32;
    if !var0.is_null() {
        strcat(match_0.string.as_mut_ptr(), var0);
        match_0.variables[0usize].offset = index as libc::c_char;
        match_0.variables[0usize].length = strlen(var0) as libc::c_int;
        index =
            (index as libc::c_ulong).wrapping_add(strlen(var0)) as libc::c_int
                as libc::c_int
    }
    if !var1.is_null() {
        strcat(match_0.string.as_mut_ptr(), var1);
        match_0.variables[1usize].offset = index as libc::c_char;
        match_0.variables[1usize].length = strlen(var1) as libc::c_int;
        index =
            (index as libc::c_ulong).wrapping_add(strlen(var1)) as libc::c_int
                as libc::c_int
    }
    if !var2.is_null() {
        strcat(match_0.string.as_mut_ptr(), var2);
        match_0.variables[2usize].offset = index as libc::c_char;
        match_0.variables[2usize].length = strlen(var2) as libc::c_int;
        index =
            (index as libc::c_ulong).wrapping_add(strlen(var2)) as libc::c_int
                as libc::c_int
    }
    if !var3.is_null() {
        strcat(match_0.string.as_mut_ptr(), var3);
        match_0.variables[3usize].offset = index as libc::c_char;
        match_0.variables[3usize].length = strlen(var3) as libc::c_int;
        index =
            (index as libc::c_ulong).wrapping_add(strlen(var3)) as libc::c_int
                as libc::c_int
    }
    if !var4.is_null() {
        strcat(match_0.string.as_mut_ptr(), var4);
        match_0.variables[4usize].offset = index as libc::c_char;
        match_0.variables[4usize].length = strlen(var4) as libc::c_int;
        index =
            (index as libc::c_ulong).wrapping_add(strlen(var4)) as libc::c_int
                as libc::c_int
    }
    if !var5.is_null() {
        strcat(match_0.string.as_mut_ptr(), var5);
        match_0.variables[5usize].offset = index as libc::c_char;
        match_0.variables[5usize].length = strlen(var5) as libc::c_int;
        index =
            (index as libc::c_ulong).wrapping_add(strlen(var5)) as libc::c_int
                as libc::c_int
    }
    if !var6.is_null() {
        strcat(match_0.string.as_mut_ptr(), var6);
        match_0.variables[6usize].offset = index as libc::c_char;
        match_0.variables[6usize].length = strlen(var6) as libc::c_int;
        index =
            (index as libc::c_ulong).wrapping_add(strlen(var6)) as libc::c_int
                as libc::c_int
    }
    if !var7.is_null() {
        strcat(match_0.string.as_mut_ptr(), var7);
        match_0.variables[7usize].offset = index as libc::c_char;
        match_0.variables[7usize].length = strlen(var7) as libc::c_int
    }
    BotConstructChatMessage(cs, message, mcontext as libc::c_ulong,
                            &mut match_0, 0i32 as libc::c_ulong,
                            qfalse as libc::c_int);
}
//end of the function BotExpandChatMessage
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotConstructChatMessage(mut chatstate:
                                                     *mut bot_chatstate_t,
                                                 mut message:
                                                     *mut libc::c_char,
                                                 mut mcontext: libc::c_ulong,
                                                 mut match_0:
                                                     *mut bot_match_t,
                                                 mut vcontext: libc::c_ulong,
                                                 mut reply: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut srcmessage: [libc::c_char; 256] = [0; 256];
    strcpy(srcmessage.as_mut_ptr(), message);
    i = 0i32;
    while i < 10i32 {
        if 0 ==
               BotExpandChatMessage((*chatstate).chatmessage.as_mut_ptr(),
                                    srcmessage.as_mut_ptr(), mcontext,
                                    match_0, vcontext, reply) {
            break ;
        }
        strcpy(srcmessage.as_mut_ptr(),
               (*chatstate).chatmessage.as_mut_ptr());
        i += 1
    }
    if i >= 10i32 {
        botimport.Print.expect("non-null function pointer")(2i32,
                                                            b"too many expansions in chat message\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        botimport.Print.expect("non-null function pointer")(2i32,
                                                            b"%s\n\x00" as
                                                                *const u8 as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            (*chatstate).chatmessage.as_mut_ptr());
    };
}
//end of the function BotLoadChatFile
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotExpandChatMessage(mut outmessage:
                                                  *mut libc::c_char,
                                              mut message: *mut libc::c_char,
                                              mut mcontext: libc::c_ulong,
                                              mut match_0: *mut bot_match_t,
                                              mut vcontext: libc::c_ulong,
                                              mut reply: libc::c_int)
 -> libc::c_int {
    let mut num: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut expansion: libc::c_int = 0;
    let mut outputbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut msgptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: [libc::c_char; 256] = [0; 256];
    expansion = qfalse as libc::c_int;
    msgptr = message;
    outputbuf = outmessage;
    len = 0i32;
    while 0 != *msgptr {
        if *msgptr as libc::c_int == 0x1i32 {
            msgptr = msgptr.offset(1isize);
            match *msgptr as libc::c_int {
                118 => {
                    msgptr = msgptr.offset(1isize);
                    num = 0i32;
                    while 0 != *msgptr as libc::c_int &&
                              *msgptr as libc::c_int != 0x1i32 {
                        let fresh7 = msgptr;
                        msgptr = msgptr.offset(1);
                        num =
                            num * 10i32 + *fresh7 as libc::c_int - '0' as i32
                    }
                    if 0 != *msgptr { msgptr = msgptr.offset(1isize) }
                    if num > 8i32 {
                        botimport.Print.expect("non-null function pointer")(3i32,
                                                                            b"BotConstructChat: message %s variable %d out of range\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char,
                                                                            message,
                                                                            num);
                        return qfalse as libc::c_int
                    }
                    if (*match_0).variables[num as usize].offset as
                           libc::c_int >= 0i32 {
                        if (*match_0).variables[num as usize].offset as
                               libc::c_int >= 0i32 {
                        } else {
                            __assert_fail(b"match->variables[num].offset >= 0\x00"
                                              as *const u8 as
                                              *const libc::c_char,
                                          b"code/botlib/be_ai_chat.c\x00" as
                                              *const u8 as
                                              *const libc::c_char,
                                          2317i32 as libc::c_uint,
                                          (*::std::mem::transmute::<&[u8; 91],
                                                                    &[libc::c_char; 91]>(b"int BotExpandChatMessage(char *, char *, unsigned long, bot_match_t *, unsigned long, int)\x00")).as_ptr());
                        }
                        ptr =
                            &mut (*match_0).string[(*match_0).variables[num as
                                                                            usize].offset
                                                       as libc::c_int as
                                                       usize] as
                                *mut libc::c_char;
                        i = 0i32;
                        while i < (*match_0).variables[num as usize].length {
                            temp[i as usize] = *ptr.offset(i as isize);
                            i += 1
                        }
                        temp[i as usize] = 0i32 as libc::c_char;
                        if 0 != reply {
                            BotReplaceReplySynonyms(temp.as_mut_ptr(),
                                                    vcontext);
                        } else {
                            BotReplaceSynonyms(temp.as_mut_ptr(), vcontext);
                        }
                        if (len as
                                libc::c_ulong).wrapping_add(strlen(temp.as_mut_ptr()))
                               >= 256i32 as libc::c_ulong {
                            botimport.Print.expect("non-null function pointer")(3i32,
                                                                                b"BotConstructChat: message %s too long\n\x00"
                                                                                    as
                                                                                    *const u8
                                                                                    as
                                                                                    *const libc::c_char
                                                                                    as
                                                                                    *mut libc::c_char,
                                                                                message);
                            return qfalse as libc::c_int
                        }
                        strcpy(&mut *outputbuf.offset(len as isize),
                               temp.as_mut_ptr());
                        len =
                            (len as
                                 libc::c_ulong).wrapping_add(strlen(temp.as_mut_ptr()))
                                as libc::c_int as libc::c_int
                    }
                }
                114 => {
                    //end if
                    msgptr = msgptr.offset(1isize);
                    i = 0i32;
                    while 0 != *msgptr as libc::c_int &&
                              *msgptr as libc::c_int != 0x1i32 {
                        let fresh8 = msgptr;
                        msgptr = msgptr.offset(1);
                        temp[i as usize] = *fresh8;
                        i += 1
                    }
                    temp[i as usize] = '\u{0}' as i32 as libc::c_char;
                    if 0 != *msgptr { msgptr = msgptr.offset(1isize) }
                    ptr = RandomString(temp.as_mut_ptr());
                    if ptr.is_null() {
                        botimport.Print.expect("non-null function pointer")(3i32,
                                                                            b"BotConstructChat: unknown random string %s\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char,
                                                                            temp.as_mut_ptr());
                        return qfalse as libc::c_int
                    }
                    if (len as libc::c_ulong).wrapping_add(strlen(ptr)) >=
                           256i32 as libc::c_ulong {
                        botimport.Print.expect("non-null function pointer")(3i32,
                                                                            b"BotConstructChat: message \"%s\" too long\n\x00"
                                                                                as
                                                                                *const u8
                                                                                as
                                                                                *const libc::c_char
                                                                                as
                                                                                *mut libc::c_char,
                                                                            message);
                        return qfalse as libc::c_int
                    }
                    strcpy(&mut *outputbuf.offset(len as isize), ptr);
                    len =
                        (len as libc::c_ulong).wrapping_add(strlen(ptr)) as
                            libc::c_int as libc::c_int;
                    expansion = qtrue as libc::c_int
                }
                _ => {
                    botimport.Print.expect("non-null function pointer")(4i32,
                                                                        b"BotConstructChat: message \"%s\" invalid escape char\n\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        message);
                }
            }
        } else {
            //end default
            //end switch
            //end if
            let fresh10 = len;
            len = len + 1;
            let fresh9 = msgptr;
            msgptr = msgptr.offset(1);
            *outputbuf.offset(fresh10 as isize) = *fresh9;
            if !(len >= 256i32) { continue ; }
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"BotConstructChat: message \"%s\" too long\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                message);
            break ;
        }
    }
    *outputbuf.offset(len as isize) = '\u{0}' as i32 as libc::c_char;
    BotReplaceWeightedSynonyms(outputbuf, mcontext);
    return expansion;
}
//end for
//end for
//end of the function BotReplaceSynonyms
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotReplaceWeightedSynonyms(mut string:
                                                        *mut libc::c_char,
                                                    mut context:
                                                        libc::c_ulong) {
    let mut syn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut synonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    let mut replacement: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    let mut weight: libc::c_float = 0.;
    let mut curweight: libc::c_float = 0.;
    syn = synonyms;
    while !syn.is_null() {
        if !(0 == (*syn).context & context) {
            weight =
                (rand() & 0x7fffi32) as libc::c_float /
                    0x7fffi32 as libc::c_float * (*syn).totalweight;
            if !(0. == weight) {
                curweight = 0i32 as libc::c_float;
                replacement = (*syn).firstsynonym;
                while !replacement.is_null() {
                    curweight += (*replacement).weight;
                    if weight < curweight { break ; }
                    replacement = (*replacement).next
                }
                //end for
                if !replacement.is_null() {
                    synonym = (*syn).firstsynonym;
                    while !synonym.is_null() {
                        if !(synonym == replacement) {
                            StringReplaceWords(string, (*synonym).string,
                                               (*replacement).string);
                        }
                        synonym = (*synonym).next
                    }
                }
            }
        }
        syn = (*syn).next
    };
}
//end of the function StringContainsWord
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn StringReplaceWords(mut string: *mut libc::c_char,
                                            mut synonym: *mut libc::c_char,
                                            mut replacement:
                                                *mut libc::c_char) {
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str2: *mut libc::c_char = 0 as *mut libc::c_char;
    str = StringContainsWord(string, synonym, qfalse as libc::c_int);
    while !str.is_null() {
        str2 = StringContainsWord(string, replacement, qfalse as libc::c_int);
        while !str2.is_null() {
            if str2 <= str && str < str2.offset(strlen(replacement) as isize)
               {
                break ;
            }
            str2 =
                StringContainsWord(str2.offset(1isize), replacement,
                                   qfalse as libc::c_int)
        }
        if str2.is_null() {
            memmove(str.offset(strlen(replacement) as isize) as
                        *mut libc::c_void,
                    str.offset(strlen(synonym) as isize) as
                        *const libc::c_void,
                    strlen(str.offset(strlen(synonym) as
                                          isize)).wrapping_add(1i32 as
                                                                   libc::c_ulong));
            memcpy(str as *mut libc::c_void,
                   replacement as *const libc::c_void, strlen(replacement));
        }
        str =
            StringContainsWord(str.offset(strlen(replacement) as isize),
                               synonym, qfalse as libc::c_int)
    };
}
//end of the function StringContains
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn StringContainsWord(mut str1: *mut libc::c_char,
                                            mut str2: *mut libc::c_char,
                                            mut casesensitive: libc::c_int)
 -> *mut libc::c_char {
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    len = strlen(str1).wrapping_sub(strlen(str2)) as libc::c_int;
    i = 0i32;
    while i <= len {
        //if not at the start of the string
        if 0 != i {
            while 0 != *str1 as libc::c_int &&
                      *str1 as libc::c_int != ' ' as i32 &&
                      *str1 as libc::c_int != '.' as i32 &&
                      *str1 as libc::c_int != ',' as i32 &&
                      *str1 as libc::c_int != '!' as i32 {
                str1 = str1.offset(1isize)
            }
            if 0 == *str1 { break ; }
            str1 = str1.offset(1isize)
        }
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
        if 0 == *str2.offset(j as isize) {
            if 0 == *str1.offset(j as isize) ||
                   *str1.offset(j as isize) as libc::c_int == ' ' as i32 ||
                   *str1.offset(j as isize) as libc::c_int == '.' as i32 ||
                   *str1.offset(j as isize) as libc::c_int == ',' as i32 ||
                   *str1.offset(j as isize) as libc::c_int == '!' as i32 {
                return str1
            }
        }
        i += 1;
        str1 = str1.offset(1isize)
    }
    return 0 as *mut libc::c_char;
}
//replace all the context related synonyms in the string
#[no_mangle]
pub unsafe extern "C" fn BotReplaceSynonyms(mut string: *mut libc::c_char,
                                            mut context: libc::c_ulong) {
    let mut syn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut synonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    syn = synonyms;
    while !syn.is_null() {
        if !(0 == (*syn).context & context) {
            synonym = (*(*syn).firstsynonym).next;
            while !synonym.is_null() {
                StringReplaceWords(string, (*synonym).string,
                                   (*(*syn).firstsynonym).string);
                synonym = (*synonym).next
            }
        }
        syn = (*syn).next
    };
}
//end for
//end for
//end of the function BotReplaceWeightedSynonyms
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotReplaceReplySynonyms(mut string:
                                                     *mut libc::c_char,
                                                 mut context: libc::c_ulong) {
    let mut str1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut str2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut replacement: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut syn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut synonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    str1 = string;
    while 0 != *str1 {
        while 0 != *str1 as libc::c_int && *str1 as libc::c_int <= ' ' as i32
              {
            str1 = str1.offset(1isize)
        }
        if 0 == *str1 { break ; }
        syn = synonyms;
        while !syn.is_null() {
            if !(0 == (*syn).context & context) {
                synonym = (*(*syn).firstsynonym).next;
                while !synonym.is_null() {
                    str2 =
                        StringContainsWord(str1, (*synonym).string,
                                           qfalse as libc::c_int);
                    if !(str2.is_null() || str2 != str1) {
                        replacement = (*(*syn).firstsynonym).string;
                        str2 =
                            StringContainsWord(str1, replacement,
                                               qfalse as libc::c_int);
                        if !(!str2.is_null() && str2 == str1) {
                            memmove(str1.offset(strlen(replacement) as isize)
                                        as *mut libc::c_void,
                                    str1.offset(strlen((*synonym).string) as
                                                    isize) as
                                        *const libc::c_void,
                                    strlen(str1.offset(strlen((*synonym).string)
                                                           as
                                                           isize)).wrapping_add(1i32
                                                                                    as
                                                                                    libc::c_ulong));
                            memcpy(str1 as *mut libc::c_void,
                                   replacement as *const libc::c_void,
                                   strlen(replacement));
                            //
                            break ;
                        }
                    }
                    synonym = (*synonym).next
                }
                //end for
                //if a synonym has been replaced
                if !synonym.is_null() { break ; }
            }
            syn = (*syn).next
        }
        while 0 != *str1 as libc::c_int && *str1 as libc::c_int > ' ' as i32 {
            str1 = str1.offset(1isize)
        }
        if 0 == *str1 { break ; }
    };
}
//end if
//end of the function BotConstructChatMessage
//===========================================================================
// randomly chooses one of the chat message of the given type
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotChooseInitialChatMessage(mut cs:
                                                         *mut bot_chatstate_t,
                                                     mut type_0:
                                                         *mut libc::c_char)
 -> *mut libc::c_char {
    let mut n: libc::c_int = 0;
    let mut numchatmessages: libc::c_int = 0;
    let mut besttime: libc::c_float = 0.;
    let mut t: *mut bot_chattype_t = 0 as *mut bot_chattype_t;
    let mut m: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut bestchatmessage: *mut bot_chatmessage_t =
        0 as *mut bot_chatmessage_t;
    let mut chat: *mut bot_chat_t = 0 as *mut bot_chat_t;
    chat = (*cs).chat;
    t = (*chat).types;
    while !t.is_null() {
        if 0 == Q_stricmp((*t).name.as_mut_ptr(), type_0) {
            numchatmessages = 0i32;
            m = (*t).firstchatmessage;
            while !m.is_null() {
                if !((*m).time > AAS_Time()) { numchatmessages += 1 }
                m = (*m).next
            }
            if numchatmessages <= 0i32 {
                besttime = 0i32 as libc::c_float;
                bestchatmessage = 0 as *mut bot_chatmessage_t;
                m = (*t).firstchatmessage;
                while !m.is_null() {
                    if 0. == besttime || (*m).time < besttime {
                        bestchatmessage = m;
                        besttime = (*m).time
                    }
                    m = (*m).next
                }
                if !bestchatmessage.is_null() {
                    return (*bestchatmessage).chatmessage
                }
            } else {
                n =
                    ((rand() & 0x7fffi32) as libc::c_float /
                         0x7fffi32 as libc::c_float *
                         numchatmessages as libc::c_float) as libc::c_int;
                m = (*t).firstchatmessage;
                while !m.is_null() {
                    if !((*m).time > AAS_Time()) {
                        n -= 1;
                        if n < 0i32 {
                            (*m).time = AAS_Time() + 20i32 as libc::c_float;
                            return (*m).chatmessage
                        }
                    }
                    m = (*m).next
                }
            }
            return 0 as *mut libc::c_char
        }
        t = (*t).next
    }
    return 0 as *mut libc::c_char;
}
//returns the number of initial chat messages of the given type
#[no_mangle]
pub unsafe extern "C" fn BotNumInitialChats(mut chatstate: libc::c_int,
                                            mut type_0: *mut libc::c_char)
 -> libc::c_int {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    let mut t: *mut bot_chattype_t = 0 as *mut bot_chattype_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return 0i32 }
    t = (*(*cs).chat).types;
    while !t.is_null() {
        if 0 == Q_stricmp((*t).name.as_mut_ptr(), type_0) {
            if 0. !=
                   LibVarGetValue(b"bot_testichat\x00" as *const u8 as
                                      *const libc::c_char) {
                botimport.Print.expect("non-null function pointer")(1i32,
                                                                    b"%s has %d chat lines\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    type_0,
                                                                    (*t).numchatmessages);
                botimport.Print.expect("non-null function pointer")(1i32,
                                                                    b"-------------------\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char);
            }
            return (*t).numchatmessages
        }
        t = (*t).next
    }
    return 0i32;
}
//find and select a reply for the given message
#[no_mangle]
pub unsafe extern "C" fn BotReplyChat(mut chatstate: libc::c_int,
                                      mut message: *mut libc::c_char,
                                      mut mcontext: libc::c_int,
                                      mut vcontext: libc::c_int,
                                      mut var0: *mut libc::c_char,
                                      mut var1: *mut libc::c_char,
                                      mut var2: *mut libc::c_char,
                                      mut var3: *mut libc::c_char,
                                      mut var4: *mut libc::c_char,
                                      mut var5: *mut libc::c_char,
                                      mut var6: *mut libc::c_char,
                                      mut var7: *mut libc::c_char)
 -> libc::c_int {
    let mut rchat: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut bestrchat: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut m: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut bestchatmessage: *mut bot_chatmessage_t =
        0 as *mut bot_chatmessage_t;
    let mut match_0: bot_match_t =
        bot_match_s{string: [0; 256],
                    type_0: 0,
                    subtype: 0,
                    variables:
                        [bot_matchvariable_s{offset: 0, length: 0,}; 8],};
    let mut bestmatch: bot_match_t =
        bot_match_s{string: [0; 256],
                    type_0: 0,
                    subtype: 0,
                    variables:
                        [bot_matchvariable_s{offset: 0, length: 0,}; 8],};
    let mut bestpriority: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut numchatmessages: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return qfalse as libc::c_int }
    memset(&mut match_0 as *mut bot_match_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<bot_match_t>() as libc::c_ulong);
    strcpy(match_0.string.as_mut_ptr(), message);
    bestpriority = -1i32;
    bestchatmessage = 0 as *mut bot_chatmessage_t;
    bestrchat = 0 as *mut bot_replychat_t;
    rchat = replychats;
    while !rchat.is_null() {
        found = qfalse as libc::c_int;
        key = (*rchat).keys;
        while !key.is_null() {
            res = qfalse as libc::c_int;
            if 0 != (*key).flags & 4i32 {
                res =
                    (StringContains(message, (*cs).name.as_mut_ptr(),
                                    qfalse as libc::c_int) != -1i32) as
                        libc::c_int
            } else if 0 != (*key).flags & 32i32 {
                res =
                    (StringContains((*key).string, (*cs).name.as_mut_ptr(),
                                    qfalse as libc::c_int) != -1i32) as
                        libc::c_int
            } else if 0 != (*key).flags & 64i32 {
                res = ((*cs).gender == 1i32) as libc::c_int
            } else if 0 != (*key).flags & 128i32 {
                res = ((*cs).gender == 2i32) as libc::c_int
            } else if 0 != (*key).flags & 256i32 {
                res = ((*cs).gender == 0i32) as libc::c_int
            } else if 0 != (*key).flags & 16i32 {
                res = StringsMatch((*key).match_0, &mut match_0)
            } else if 0 != (*key).flags & 8i32 {
                res =
                    (StringContainsWord(message, (*key).string,
                                        qfalse as libc::c_int) !=
                         0 as *mut libc::c_void as *mut libc::c_char) as
                        libc::c_int
            }
            //if the key must be present
            if 0 != (*key).flags & 1i32 {
                if 0 == res { found = qfalse as libc::c_int; break ; }
            } else if 0 != (*key).flags & 2i32 {
                if 0 != res { found = qfalse as libc::c_int; break ; }
            } else if 0 != res { found = qtrue as libc::c_int }
            key = (*key).next
        }
        if 0 != found {
            if (*rchat).priority > bestpriority as libc::c_float {
                numchatmessages = 0i32;
                m = (*rchat).firstchatmessage;
                while !m.is_null() {
                    if !((*m).time > AAS_Time()) { numchatmessages += 1 }
                    m = (*m).next
                }
                num =
                    ((rand() & 0x7fffi32) as libc::c_float /
                         0x7fffi32 as libc::c_float *
                         numchatmessages as libc::c_float) as libc::c_int;
                m = (*rchat).firstchatmessage;
                while !m.is_null() {
                    num -= 1;
                    if num < 0i32 { break ; }
                    (*m).time > AAS_Time();
                    m = (*m).next
                }
                if !m.is_null() {
                    memcpy(&mut bestmatch as *mut bot_match_t as
                               *mut libc::c_void,
                           &mut match_0 as *mut bot_match_t as
                               *const libc::c_void,
                           ::std::mem::size_of::<bot_match_t>() as
                               libc::c_ulong);
                    bestchatmessage = m;
                    bestrchat = rchat;
                    bestpriority = (*rchat).priority as libc::c_int
                }
            }
        }
        rchat = (*rchat).next
    }
    if !bestchatmessage.is_null() {
        index = strlen(bestmatch.string.as_mut_ptr()) as libc::c_int;
        if !var0.is_null() {
            strcat(bestmatch.string.as_mut_ptr(), var0);
            bestmatch.variables[0usize].offset = index as libc::c_char;
            bestmatch.variables[0usize].length = strlen(var0) as libc::c_int;
            index =
                (index as libc::c_ulong).wrapping_add(strlen(var0)) as
                    libc::c_int as libc::c_int
        }
        if !var1.is_null() {
            strcat(bestmatch.string.as_mut_ptr(), var1);
            bestmatch.variables[1usize].offset = index as libc::c_char;
            bestmatch.variables[1usize].length = strlen(var1) as libc::c_int;
            index =
                (index as libc::c_ulong).wrapping_add(strlen(var1)) as
                    libc::c_int as libc::c_int
        }
        if !var2.is_null() {
            strcat(bestmatch.string.as_mut_ptr(), var2);
            bestmatch.variables[2usize].offset = index as libc::c_char;
            bestmatch.variables[2usize].length = strlen(var2) as libc::c_int;
            index =
                (index as libc::c_ulong).wrapping_add(strlen(var2)) as
                    libc::c_int as libc::c_int
        }
        if !var3.is_null() {
            strcat(bestmatch.string.as_mut_ptr(), var3);
            bestmatch.variables[3usize].offset = index as libc::c_char;
            bestmatch.variables[3usize].length = strlen(var3) as libc::c_int;
            index =
                (index as libc::c_ulong).wrapping_add(strlen(var3)) as
                    libc::c_int as libc::c_int
        }
        if !var4.is_null() {
            strcat(bestmatch.string.as_mut_ptr(), var4);
            bestmatch.variables[4usize].offset = index as libc::c_char;
            bestmatch.variables[4usize].length = strlen(var4) as libc::c_int;
            index =
                (index as libc::c_ulong).wrapping_add(strlen(var4)) as
                    libc::c_int as libc::c_int
        }
        if !var5.is_null() {
            strcat(bestmatch.string.as_mut_ptr(), var5);
            bestmatch.variables[5usize].offset = index as libc::c_char;
            bestmatch.variables[5usize].length = strlen(var5) as libc::c_int;
            index =
                (index as libc::c_ulong).wrapping_add(strlen(var5)) as
                    libc::c_int as libc::c_int
        }
        if !var6.is_null() {
            strcat(bestmatch.string.as_mut_ptr(), var6);
            bestmatch.variables[6usize].offset = index as libc::c_char;
            bestmatch.variables[6usize].length = strlen(var6) as libc::c_int;
            index =
                (index as libc::c_ulong).wrapping_add(strlen(var6)) as
                    libc::c_int as libc::c_int
        }
        if !var7.is_null() {
            strcat(bestmatch.string.as_mut_ptr(), var7);
            bestmatch.variables[7usize].offset = index as libc::c_char;
            bestmatch.variables[7usize].length = strlen(var7) as libc::c_int
        }
        if 0. !=
               LibVarGetValue(b"bot_testrchat\x00" as *const u8 as
                                  *const libc::c_char) {
            m = (*bestrchat).firstchatmessage;
            while !m.is_null() {
                BotConstructChatMessage(cs, (*m).chatmessage,
                                        mcontext as libc::c_ulong,
                                        &mut bestmatch,
                                        vcontext as libc::c_ulong,
                                        qtrue as libc::c_int);
                BotRemoveTildes((*cs).chatmessage.as_mut_ptr());
                botimport.Print.expect("non-null function pointer")(1i32,
                                                                    b"%s\n\x00"
                                                                        as
                                                                        *const u8
                                                                        as
                                                                        *const libc::c_char
                                                                        as
                                                                        *mut libc::c_char,
                                                                    (*cs).chatmessage.as_mut_ptr());
                m = (*m).next
            }
        } else {
            (*bestchatmessage).time = AAS_Time() + 20i32 as libc::c_float;
            BotConstructChatMessage(cs, (*bestchatmessage).chatmessage,
                                    mcontext as libc::c_ulong, &mut bestmatch,
                                    vcontext as libc::c_ulong,
                                    qtrue as libc::c_int);
        }
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
//end of the function IsWhiteSpace
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotRemoveTildes(mut message: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while 0 != *message.offset(i as isize) {
        if *message.offset(i as isize) as libc::c_int == '~' as i32 {
            memmove(&mut *message.offset(i as isize) as *mut libc::c_char as
                        *mut libc::c_void,
                    &mut *message.offset((i + 1i32) as isize) as
                        *mut libc::c_char as *const libc::c_void,
                    strlen(&mut *message.offset((i + 1i32) as
                                                    isize)).wrapping_add(1i32
                                                                             as
                                                                             libc::c_ulong));
        }
        i += 1
    };
}
//end of the function BotLoadMatchTemplates
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn StringsMatch(mut pieces: *mut bot_matchpiece_t,
                                      mut match_0: *mut bot_match_t)
 -> libc::c_int {
    let mut lastvariable: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut strptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newstrptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut ms: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    lastvariable = -1i32;
    strptr = (*match_0).string.as_mut_ptr();
    mp = pieces;
    while !mp.is_null() {
        if (*mp).type_0 == 2i32 {
            newstrptr = 0 as *mut libc::c_char;
            ms = (*mp).firststring;
            while !ms.is_null() {
                if 0 == strlen((*ms).string) {
                    newstrptr = strptr;
                    break ;
                } else {
                    index =
                        StringContains(strptr, (*ms).string,
                                       qfalse as libc::c_int);
                    if index >= 0i32 {
                        newstrptr = strptr.offset(index as isize);
                        if lastvariable >= 0i32 {
                            (*match_0).variables[lastvariable as usize].length
                                =
                                (newstrptr.wrapping_offset_from((*match_0).string.as_mut_ptr())
                                     as libc::c_long -
                                     (*match_0).variables[lastvariable as
                                                              usize].offset as
                                         libc::c_long) as libc::c_int;
                            lastvariable = -1i32;
                            break ;
                        } else {
                            //end if
                            if index == 0i32 { break ; }
                            newstrptr = 0 as *mut libc::c_char
                        }
                    }
                    ms = (*ms).next
                }
            }
            if newstrptr.is_null() { return qfalse as libc::c_int }
            strptr = newstrptr.offset(strlen((*ms).string) as isize)
        } else if (*mp).type_0 == 1i32 {
            (*match_0).variables[(*mp).variable as usize].offset =
                strptr.wrapping_offset_from((*match_0).string.as_mut_ptr()) as
                    libc::c_long as libc::c_char;
            lastvariable = (*mp).variable
        }
        mp = (*mp).next
    }
    if mp.is_null() && (lastvariable >= 0i32 || 0 == strlen(strptr)) {
        if lastvariable >= 0i32 {
            if (*match_0).variables[lastvariable as usize].offset as
                   libc::c_int >= 0i32 {
            } else {
                __assert_fail(b"match->variables[lastvariable].offset >= 0\x00"
                                  as *const u8 as *const libc::c_char,
                              b"code/botlib/be_ai_chat.c\x00" as *const u8 as
                                  *const libc::c_char,
                              1441i32 as libc::c_uint,
                              (*::std::mem::transmute::<&[u8; 52],
                                                        &[libc::c_char; 52]>(b"int StringsMatch(bot_matchpiece_t *, bot_match_t *)\x00")).as_ptr());
            }
            (*match_0).variables[lastvariable as usize].length =
                strlen(&mut (*match_0).string[(*match_0).variables[lastvariable
                                                                       as
                                                                       usize].offset
                                                  as libc::c_int as usize]) as
                    libc::c_int
        }
        return qtrue as libc::c_int
    }
    return qfalse as libc::c_int;
}
//returns the length of the currently selected chat message
#[no_mangle]
pub unsafe extern "C" fn BotChatLength(mut chatstate: libc::c_int)
 -> libc::c_int {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return 0i32 }
    return strlen((*cs).chatmessage.as_mut_ptr()) as libc::c_int;
}
//enters the selected chat message
#[no_mangle]
pub unsafe extern "C" fn BotEnterChat(mut chatstate: libc::c_int,
                                      mut clientto: libc::c_int,
                                      mut sendto: libc::c_int) {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return }
    if 0 != strlen((*cs).chatmessage.as_mut_ptr()) {
        BotRemoveTildes((*cs).chatmessage.as_mut_ptr());
        if 0. !=
               LibVarGetValue(b"bot_testichat\x00" as *const u8 as
                                  *const libc::c_char) {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"%s\n\x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                (*cs).chatmessage.as_mut_ptr());
        } else {
            match sendto {
                1 => {
                    EA_Command((*cs).client,
                               va(b"say_team %s\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                                  (*cs).chatmessage.as_mut_ptr()));
                }
                2 => {
                    EA_Command((*cs).client,
                               va(b"tell %d %s\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, clientto,
                                  (*cs).chatmessage.as_mut_ptr()));
                }
                _ => {
                    EA_Command((*cs).client,
                               va(b"say %s\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                                  (*cs).chatmessage.as_mut_ptr()));
                }
            }
        }
        strcpy((*cs).chatmessage.as_mut_ptr(),
               b"\x00" as *const u8 as *const libc::c_char);
    };
}
//get the chat message ready to be output
#[no_mangle]
pub unsafe extern "C" fn BotGetChatMessage(mut chatstate: libc::c_int,
                                           mut buf: *mut libc::c_char,
                                           mut size: libc::c_int) {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return }
    BotRemoveTildes((*cs).chatmessage.as_mut_ptr());
    strncpy(buf, (*cs).chatmessage.as_mut_ptr(),
            (size - 1i32) as libc::c_ulong);
    *buf.offset((size - 1i32) as isize) = '\u{0}' as i32 as libc::c_char;
    strcpy((*cs).chatmessage.as_mut_ptr(),
           b"\x00" as *const u8 as *const libc::c_char);
}
//finds a match for the given string using the match templates
#[no_mangle]
pub unsafe extern "C" fn BotFindMatch(mut str: *mut libc::c_char,
                                      mut match_0: *mut bot_match_t,
                                      mut context: libc::c_ulong)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ms: *mut bot_matchtemplate_t = 0 as *mut bot_matchtemplate_t;
    Q_strncpyz((*match_0).string.as_mut_ptr(), str, 256i32);
    while 0 != strlen((*match_0).string.as_mut_ptr()) &&
              (*match_0).string[strlen((*match_0).string.as_mut_ptr()).wrapping_sub(1i32
                                                                                        as
                                                                                        libc::c_ulong)
                                    as usize] as libc::c_int == '\n' as i32 {
        (*match_0).string[strlen((*match_0).string.as_mut_ptr()).wrapping_sub(1i32
                                                                                  as
                                                                                  libc::c_ulong)
                              as usize] = '\u{0}' as i32 as libc::c_char
    }
    ms = matchtemplates;
    while !ms.is_null() {
        if !(0 == (*ms).context & context) {
            i = 0i32;
            while i < 8i32 {
                (*match_0).variables[i as usize].offset =
                    -1i32 as libc::c_char;
                i += 1
            }
            if 0 != StringsMatch((*ms).first, match_0) {
                (*match_0).type_0 = (*ms).type_0;
                (*match_0).subtype = (*ms).subtype;
                return qtrue as libc::c_int
            }
        }
        ms = (*ms).next
    }
    return qfalse as libc::c_int;
}
//returns a variable from a match
#[no_mangle]
pub unsafe extern "C" fn BotMatchVariable(mut match_0: *mut bot_match_t,
                                          mut variable: libc::c_int,
                                          mut buf: *mut libc::c_char,
                                          mut size: libc::c_int) {
    if variable < 0i32 || variable >= 8i32 {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"BotMatchVariable: variable out of range\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        strcpy(buf, b"\x00" as *const u8 as *const libc::c_char);
        return
    }
    if (*match_0).variables[variable as usize].offset as libc::c_int >= 0i32 {
        if (*match_0).variables[variable as usize].length < size {
            size = (*match_0).variables[variable as usize].length + 1i32
        }
        if (*match_0).variables[variable as usize].offset as libc::c_int >=
               0i32 {
        } else {
            __assert_fail(b"match->variables[variable].offset >= 0\x00" as
                              *const u8 as *const libc::c_char,
                          b"code/botlib/be_ai_chat.c\x00" as *const u8 as
                              *const libc::c_char, 1502i32 as libc::c_uint,
                          (*::std::mem::transmute::<&[u8; 55],
                                                    &[libc::c_char; 55]>(b"void BotMatchVariable(bot_match_t *, int, char *, int)\x00")).as_ptr());
        }
        strncpy(buf,
                &mut (*match_0).string[(*match_0).variables[variable as
                                                                usize].offset
                                           as libc::c_int as usize],
                (size - 1i32) as libc::c_ulong);
        *buf.offset((size - 1i32) as isize) = '\u{0}' as i32 as libc::c_char
    } else { strcpy(buf, b"\x00" as *const u8 as *const libc::c_char); };
}
//unify all the white spaces in the string
#[no_mangle]
pub unsafe extern "C" fn UnifyWhiteSpaces(mut string: *mut libc::c_char) {
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldptr: *mut libc::c_char = 0 as *mut libc::c_char;
    oldptr = string;
    ptr = oldptr;
    while 0 != *ptr {
        while 0 != *ptr as libc::c_int && 0 != IsWhiteSpace(*ptr) {
            ptr = ptr.offset(1isize)
        }
        if ptr > oldptr {
            if oldptr > string && 0 != *ptr as libc::c_int {
                let fresh11 = oldptr;
                oldptr = oldptr.offset(1);
                *fresh11 = ' ' as i32 as libc::c_char
            }
            if ptr > oldptr {
                memmove(oldptr as *mut libc::c_void,
                        ptr as *const libc::c_void,
                        strlen(ptr).wrapping_add(1i32 as libc::c_ulong));
            }
        }
        while 0 != *ptr as libc::c_int && 0 == IsWhiteSpace(*ptr) {
            ptr = ptr.offset(1isize)
        }
        oldptr = ptr
    };
}
//end of the function BotNumConsoleMessages
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn IsWhiteSpace(mut c: libc::c_char) -> libc::c_int {
    if c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32 ||
           c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32 ||
           c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32 ||
           c as libc::c_int == '(' as i32 || c as libc::c_int == ')' as i32 ||
           c as libc::c_int == '?' as i32 || c as libc::c_int == ':' as i32 ||
           c as libc::c_int == '\'' as i32 || c as libc::c_int == '/' as i32
           || c as libc::c_int == ',' as i32 || c as libc::c_int == '.' as i32
           || c as libc::c_int == '[' as i32 || c as libc::c_int == ']' as i32
           || c as libc::c_int == '-' as i32 || c as libc::c_int == '_' as i32
           || c as libc::c_int == '+' as i32 || c as libc::c_int == '=' as i32
       {
        return qfalse as libc::c_int
    }
    return qtrue as libc::c_int;
}
//loads a chat file for the chat state
#[no_mangle]
pub unsafe extern "C" fn BotLoadChatFile(mut chatstate: libc::c_int,
                                         mut chatfile: *mut libc::c_char,
                                         mut chatname: *mut libc::c_char)
 -> libc::c_int {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    let mut n: libc::c_int = 0;
    let mut avail: libc::c_int = 0i32;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return 8i32 }
    BotFreeChatFile(chatstate);
    if 0. ==
           LibVarGetValue(b"bot_reloadcharacters\x00" as *const u8 as
                              *const libc::c_char) {
        avail = -1i32;
        n = 0i32;
        while n < 64i32 {
            if ichatdata[n as usize].is_null() {
                if avail == -1i32 { avail = n }
            } else if !(strcmp(chatfile,
                               (*ichatdata[n as usize]).filename.as_mut_ptr())
                            != 0i32) {
                if !(strcmp(chatname,
                            (*ichatdata[n as usize]).chatname.as_mut_ptr()) !=
                         0i32) {
                    (*cs).chat = (*ichatdata[n as usize]).chat;
                    return 0i32
                }
            }
            n += 1
        }
        if avail == -1i32 {
            botimport.Print.expect("non-null function pointer")(4i32,
                                                                b"ichatdata table full; couldn\'t load chat %s from %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                chatname,
                                                                chatfile);
            return 8i32
        }
    }
    (*cs).chat = BotLoadInitialChat(chatfile, chatname);
    if (*cs).chat.is_null() {
        botimport.Print.expect("non-null function pointer")(4i32,
                                                            b"couldn\'t load chat %s from %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            chatname,
                                                            chatfile);
        return 8i32
    }
    if 0. ==
           LibVarGetValue(b"bot_reloadcharacters\x00" as *const u8 as
                              *const libc::c_char) {
        ichatdata[avail as usize] =
            GetClearedMemory(::std::mem::size_of::<bot_ichatdata_t>() as
                                 libc::c_ulong) as *mut bot_ichatdata_t;
        (*ichatdata[avail as usize]).chat = (*cs).chat;
        Q_strncpyz((*ichatdata[avail as usize]).chatname.as_mut_ptr(),
                   chatname,
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong as libc::c_int);
        Q_strncpyz((*ichatdata[avail as usize]).filename.as_mut_ptr(),
                   chatfile,
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong as libc::c_int);
    }
    return 0i32;
}
//end of the function BotDumpInitialChat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLoadInitialChat(mut chatfile: *mut libc::c_char,
                                            mut chatname: *mut libc::c_char)
 -> *mut bot_chat_t {
    let mut pass: libc::c_int = 0;
    let mut foundchat: libc::c_int = 0;
    let mut indent: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut chatmessagestring: [libc::c_char; 256] = [0; 256];
    let mut source: *mut source_t = 0 as *mut source_t;
    let mut token: token_t =
        token_s{string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut libc::c_char,
                endwhitespace_p: 0 as *mut libc::c_char,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut token_s,};
    let mut chat: *mut bot_chat_t = 0 as *mut bot_chat_t;
    let mut chattype: *mut bot_chattype_t = 0 as *mut bot_chattype_t;
    let mut chatmessage: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    size = 0i32;
    foundchat = qfalse as libc::c_int;
    pass = 0i32;
    while pass < 2i32 {
        if 0 != pass && 0 != size {
            ptr = GetClearedMemory(size as libc::c_ulong) as *mut libc::c_char
        }
        PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *const libc::c_char
                             as *mut libc::c_char);
        source = LoadSourceFile(chatfile);
        if source.is_null() {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"counldn\'t load %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                chatfile);
            return 0 as *mut bot_chat_t
        }
        if 0 != pass {
            chat = ptr as *mut bot_chat_t;
            ptr =
                ptr.offset(::std::mem::size_of::<bot_chat_t>() as
                               libc::c_ulong as isize)
        }
        size =
            ::std::mem::size_of::<bot_chat_t>() as libc::c_ulong as
                libc::c_int;
        while 0 != PC_ReadToken(source, &mut token) {
            if 0 ==
                   strcmp(token.string.as_mut_ptr(),
                          b"chat\x00" as *const u8 as *const libc::c_char) {
                if 0 == PC_ExpectTokenType(source, 1i32, 0i32, &mut token) {
                    FreeSource(source);
                    return 0 as *mut bot_chat_t
                }
                StripDoubleQuotes(token.string.as_mut_ptr());
                if 0 ==
                       PC_ExpectTokenString(source,
                                            b"{\x00" as *const u8 as
                                                *const libc::c_char as
                                                *mut libc::c_char) {
                    FreeSource(source);
                    return 0 as *mut bot_chat_t
                }
                if 0 == Q_stricmp(token.string.as_mut_ptr(), chatname) {
                    foundchat = qtrue as libc::c_int;
                    loop  {
                        if 0 == PC_ExpectAnyToken(source, &mut token) {
                            FreeSource(source);
                            return 0 as *mut bot_chat_t
                        }
                        //end if
                        if 0 ==
                               strcmp(token.string.as_mut_ptr(),
                                      b"}\x00" as *const u8 as
                                          *const libc::c_char) {
                            break ;
                        }
                        if 0 !=
                               strcmp(token.string.as_mut_ptr(),
                                      b"type\x00" as *const u8 as
                                          *const libc::c_char) {
                            SourceError(source,
                                        b"expected type found %s\x00" as
                                            *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        token.string.as_mut_ptr());
                            FreeSource(source);
                            return 0 as *mut bot_chat_t
                        }
                        if 0 ==
                               PC_ExpectTokenType(source, 1i32, 0i32,
                                                  &mut token) ||
                               0 ==
                                   PC_ExpectTokenString(source,
                                                        b"{\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char)
                           {
                            FreeSource(source);
                            return 0 as *mut bot_chat_t
                        }
                        StripDoubleQuotes(token.string.as_mut_ptr());
                        if 0 != pass && !ptr.is_null() {
                            chattype = ptr as *mut bot_chattype_t;
                            Q_strncpyz((*chattype).name.as_mut_ptr(),
                                       token.string.as_mut_ptr(), 32i32);
                            (*chattype).firstchatmessage =
                                0 as *mut bot_chatmessage_t;
                            (*chattype).next = (*chat).types;
                            (*chat).types = chattype;
                            ptr =
                                ptr.offset(::std::mem::size_of::<bot_chattype_t>()
                                               as libc::c_ulong as isize)
                        }
                        size =
                            (size as
                                 libc::c_ulong).wrapping_add(::std::mem::size_of::<bot_chattype_t>()
                                                                 as
                                                                 libc::c_ulong)
                                as libc::c_int as libc::c_int;
                        while 0 ==
                                  PC_CheckTokenString(source,
                                                      b"}\x00" as *const u8 as
                                                          *const libc::c_char
                                                          as
                                                          *mut libc::c_char) {
                            let mut len: size_t = 0;
                            if 0 ==
                                   BotLoadChatMessage(source,
                                                      chatmessagestring.as_mut_ptr())
                               {
                                FreeSource(source);
                                return 0 as *mut bot_chat_t
                            }
                            len =
                                strlen(chatmessagestring.as_mut_ptr()).wrapping_add(1i32
                                                                                        as
                                                                                        libc::c_ulong);
                            len =
                                len.wrapping_add(::std::mem::size_of::<libc::c_long>()
                                                     as
                                                     libc::c_ulong).wrapping_sub(1i32
                                                                                     as
                                                                                     libc::c_ulong)
                                    &
                                    !(::std::mem::size_of::<libc::c_long>() as
                                          libc::c_ulong).wrapping_sub(1i32 as
                                                                          libc::c_ulong);
                            if 0 != pass && !ptr.is_null() {
                                chatmessage = ptr as *mut bot_chatmessage_t;
                                (*chatmessage).time =
                                    (-2i32 * 20i32) as libc::c_float;
                                (*chatmessage).next =
                                    (*chattype).firstchatmessage;
                                (*chattype).firstchatmessage = chatmessage;
                                ptr =
                                    ptr.offset(::std::mem::size_of::<bot_chatmessage_t>()
                                                   as libc::c_ulong as isize);
                                (*chatmessage).chatmessage = ptr;
                                strcpy((*chatmessage).chatmessage,
                                       chatmessagestring.as_mut_ptr());
                                ptr = ptr.offset(len as isize);
                                (*chattype).numchatmessages += 1
                            }
                            size =
                                (size as
                                     libc::c_ulong).wrapping_add((::std::mem::size_of::<bot_chatmessage_t>()
                                                                      as
                                                                      libc::c_ulong).wrapping_add(len))
                                    as libc::c_int as libc::c_int
                        }
                    }
                } else {
                    indent = 1i32;
                    while 0 != indent {
                        if 0 == PC_ExpectAnyToken(source, &mut token) {
                            FreeSource(source);
                            return 0 as *mut bot_chat_t
                        }
                        if 0 ==
                               strcmp(token.string.as_mut_ptr(),
                                      b"{\x00" as *const u8 as
                                          *const libc::c_char) {
                            indent += 1
                        } else if 0 ==
                                      strcmp(token.string.as_mut_ptr(),
                                             b"}\x00" as *const u8 as
                                                 *const libc::c_char) {
                            indent -= 1
                        }
                    }
                }
            } else {
                SourceError(source,
                            b"unknown definition %s\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            token.string.as_mut_ptr());
                FreeSource(source);
                return 0 as *mut bot_chat_t
            }
        }
        FreeSource(source);
        if 0 == foundchat {
            botimport.Print.expect("non-null function pointer")(3i32,
                                                                b"couldn\'t find chat %s in %s\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                chatname,
                                                                chatfile);
            return 0 as *mut bot_chat_t
        }
        pass += 1
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"loaded %s from %s\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        chatname, chatfile);
    if 0 != botDeveloper { BotCheckInitialChatIntegrety(chat); }
    return chat;
}
//end of the function BotCheckChatMessageIntegrety
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotCheckInitialChatIntegrety(mut chat:
                                                          *mut bot_chat_t) {
    let mut t: *mut bot_chattype_t = 0 as *mut bot_chattype_t;
    let mut cm: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut stringlist: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    let mut s: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    let mut nexts: *mut bot_stringlist_t = 0 as *mut bot_stringlist_t;
    stringlist = 0 as *mut bot_stringlist_t;
    t = (*chat).types;
    while !t.is_null() {
        cm = (*t).firstchatmessage;
        while !cm.is_null() {
            stringlist =
                BotCheckChatMessageIntegrety((*cm).chatmessage, stringlist);
            cm = (*cm).next
        }
        t = (*t).next
    }
    s = stringlist;
    while !s.is_null() {
        nexts = (*s).next;
        FreeMemory(s as *mut libc::c_void);
        s = nexts
    };
}
//store the gender of the bot in the chat state
#[no_mangle]
pub unsafe extern "C" fn BotSetChatGender(mut chatstate: libc::c_int,
                                          mut gender: libc::c_int) {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return }
    match gender {
        1 => { (*cs).gender = 1i32 }
        2 => { (*cs).gender = 2i32 }
        _ => { (*cs).gender = 0i32 }
    };
}
//store the bot name in the chat state
#[no_mangle]
pub unsafe extern "C" fn BotSetChatName(mut chatstate: libc::c_int,
                                        mut name: *mut libc::c_char,
                                        mut client: libc::c_int) {
    let mut cs: *mut bot_chatstate_t = 0 as *mut bot_chatstate_t;
    cs = BotChatStateFromHandle(chatstate);
    if cs.is_null() { return }
    (*cs).client = client;
    memset((*cs).name.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong);
    strncpy((*cs).name.as_mut_ptr(), name,
            (::std::mem::size_of::<[libc::c_char; 32]>() as
                 libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong));
    (*cs).name[(::std::mem::size_of::<[libc::c_char; 32]>() as
                    libc::c_ulong).wrapping_sub(1i32 as libc::c_ulong) as
                   usize] = '\u{0}' as i32 as libc::c_char;
}
//end if
//end of the function StringReplaceWords
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotDumpSynonymList(mut synlist:
                                                *mut bot_synonymlist_t) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut syn: *mut bot_synonymlist_t = 0 as *mut bot_synonymlist_t;
    let mut synonym: *mut bot_synonym_t = 0 as *mut bot_synonym_t;
    fp = Log_FilePointer();
    if fp.is_null() { return }
    syn = synlist;
    while !syn.is_null() {
        fprintf(fp, b"%ld : [\x00" as *const u8 as *const libc::c_char,
                (*syn).context);
        synonym = (*syn).firstsynonym;
        while !synonym.is_null() {
            fprintf(fp,
                    b"(\"%s\", %1.2f)\x00" as *const u8 as
                        *const libc::c_char, (*synonym).string,
                    (*synonym).weight as libc::c_double);
            if !(*synonym).next.is_null() {
                fprintf(fp, b", \x00" as *const u8 as *const libc::c_char);
            }
            synonym = (*synonym).next
        }
        fprintf(fp, b"]\n\x00" as *const u8 as *const libc::c_char);
        syn = (*syn).next
    };
}
//end of the function BotLoadChatMessage
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotDumpRandomStringList(mut randomlist:
                                                     *mut bot_randomlist_t) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut random: *mut bot_randomlist_t = 0 as *mut bot_randomlist_t;
    let mut rs: *mut bot_randomstring_t = 0 as *mut bot_randomstring_t;
    fp = Log_FilePointer();
    if fp.is_null() { return }
    random = randomlist;
    while !random.is_null() {
        fprintf(fp, b"%s = {\x00" as *const u8 as *const libc::c_char,
                (*random).string);
        rs = (*random).firstrandomstring;
        while !rs.is_null() {
            fprintf(fp, b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                    (*rs).string);
            if !(*rs).next.is_null() {
                fprintf(fp, b", \x00" as *const u8 as *const libc::c_char);
            } else {
                fprintf(fp, b"}\n\x00" as *const u8 as *const libc::c_char);
            }
            rs = (*rs).next
        }
        random = (*random).next
    };
}
//end of the function RandomString
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotDumpMatchTemplates(mut matches:
                                                   *mut bot_matchtemplate_t) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut mt: *mut bot_matchtemplate_t = 0 as *mut bot_matchtemplate_t;
    let mut mp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    let mut ms: *mut bot_matchstring_t = 0 as *mut bot_matchstring_t;
    fp = Log_FilePointer();
    if fp.is_null() { return }
    mt = matches;
    while !mt.is_null() {
        fprintf(fp, b"{ \x00" as *const u8 as *const libc::c_char);
        mp = (*mt).first;
        while !mp.is_null() {
            if (*mp).type_0 == 2i32 {
                ms = (*mp).firststring;
                while !ms.is_null() {
                    fprintf(fp,
                            b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                            (*ms).string);
                    if !(*ms).next.is_null() {
                        fprintf(fp,
                                b"|\x00" as *const u8 as *const libc::c_char);
                    }
                    ms = (*ms).next
                }
            } else if (*mp).type_0 == 1i32 {
                fprintf(fp, b"%d\x00" as *const u8 as *const libc::c_char,
                        (*mp).variable);
            }
            if !(*mp).next.is_null() {
                fprintf(fp, b", \x00" as *const u8 as *const libc::c_char);
            }
            mp = (*mp).next
        }
        fprintf(fp,
                b" = (%d, %d);}\n\x00" as *const u8 as *const libc::c_char,
                (*mt).type_0, (*mt).subtype);
        mt = (*mt).next
    };
}
//end for
//end of the function BotCheckReplyChatIntegrety
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotDumpReplyChat(mut replychat:
                                              *mut bot_replychat_t) {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut rp: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut cm: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    let mut mp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    fp = Log_FilePointer();
    if fp.is_null() { return }
    fprintf(fp,
            b"BotDumpReplyChat:\n\x00" as *const u8 as *const libc::c_char);
    rp = replychat;
    while !rp.is_null() {
        fprintf(fp, b"[\x00" as *const u8 as *const libc::c_char);
        key = (*rp).keys;
        while !key.is_null() {
            if 0 != (*key).flags & 1i32 {
                fprintf(fp, b"&\x00" as *const u8 as *const libc::c_char);
            } else if 0 != (*key).flags & 2i32 {
                fprintf(fp, b"!\x00" as *const u8 as *const libc::c_char);
            }
            if 0 != (*key).flags & 4i32 {
                fprintf(fp, b"name\x00" as *const u8 as *const libc::c_char);
            } else if 0 != (*key).flags & 64i32 {
                fprintf(fp,
                        b"female\x00" as *const u8 as *const libc::c_char);
            } else if 0 != (*key).flags & 128i32 {
                fprintf(fp, b"male\x00" as *const u8 as *const libc::c_char);
            } else if 0 != (*key).flags & 256i32 {
                fprintf(fp, b"it\x00" as *const u8 as *const libc::c_char);
            } else if 0 != (*key).flags & 16i32 {
                fprintf(fp, b"(\x00" as *const u8 as *const libc::c_char);
                mp = (*key).match_0;
                while !mp.is_null() {
                    if (*mp).type_0 == 2i32 {
                        fprintf(fp,
                                b"\"%s\"\x00" as *const u8 as
                                    *const libc::c_char,
                                (*(*mp).firststring).string);
                    } else {
                        fprintf(fp,
                                b"%d\x00" as *const u8 as *const libc::c_char,
                                (*mp).variable);
                    }
                    if !(*mp).next.is_null() {
                        fprintf(fp,
                                b", \x00" as *const u8 as
                                    *const libc::c_char);
                    }
                    mp = (*mp).next
                }
                fprintf(fp, b")\x00" as *const u8 as *const libc::c_char);
            } else if 0 != (*key).flags & 8i32 {
                fprintf(fp, b"\"%s\"\x00" as *const u8 as *const libc::c_char,
                        (*key).string);
            }
            if !(*key).next.is_null() {
                fprintf(fp, b", \x00" as *const u8 as *const libc::c_char);
            } else {
                fprintf(fp,
                        b"] = %1.0f\n\x00" as *const u8 as
                            *const libc::c_char,
                        (*rp).priority as libc::c_double);
            }
            key = (*key).next
        }
        fprintf(fp, b"{\n\x00" as *const u8 as *const libc::c_char);
        cm = (*rp).firstchatmessage;
        while !cm.is_null() {
            fprintf(fp,
                    b"\t\"%s\";\n\x00" as *const u8 as *const libc::c_char,
                    (*cm).chatmessage);
            cm = (*cm).next
        }
        fprintf(fp, b"}\n\x00" as *const u8 as *const libc::c_char);
        rp = (*rp).next
    };
}
//end of the function BotLoadReplyChat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotDumpInitialChat(mut chat: *mut bot_chat_t) {
    let mut t: *mut bot_chattype_t = 0 as *mut bot_chattype_t;
    let mut m: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    Log_Write(b"{\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char);
    t = (*chat).types;
    while !t.is_null() {
        Log_Write(b" type \"%s\"\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, (*t).name.as_mut_ptr());
        Log_Write(b" {\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
        Log_Write(b"  numchatmessages = %d\x00" as *const u8 as
                      *const libc::c_char as *mut libc::c_char,
                  (*t).numchatmessages);
        m = (*t).firstchatmessage;
        while !m.is_null() {
            Log_Write(b"  \"%s\"\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char, (*m).chatmessage);
            m = (*m).next
        }
        Log_Write(b" }\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char);
        t = (*t).next
    }
    Log_Write(b"}\x00" as *const u8 as *const libc::c_char as
                  *mut libc::c_char);
}
//end of the function BotInitialChat
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotPrintReplyChatKeys(mut replychat:
                                                   *mut bot_replychat_t) {
    let mut key: *mut bot_replychatkey_t = 0 as *mut bot_replychatkey_t;
    let mut mp: *mut bot_matchpiece_t = 0 as *mut bot_matchpiece_t;
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"[\x00" as *const u8
                                                            as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
    key = (*replychat).keys;
    while !key.is_null() {
        if 0 != (*key).flags & 1i32 {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"&\x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
        } else if 0 != (*key).flags & 2i32 {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"!\x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
        }
        if 0 != (*key).flags & 4i32 {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"name\x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
        } else if 0 != (*key).flags & 64i32 {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"female\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
        } else if 0 != (*key).flags & 128i32 {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"male\x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
        } else if 0 != (*key).flags & 256i32 {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"it\x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
        } else if 0 != (*key).flags & 16i32 {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"(\x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
            mp = (*key).match_0;
            while !mp.is_null() {
                if (*mp).type_0 == 2i32 {
                    botimport.Print.expect("non-null function pointer")(1i32,
                                                                        b"\"%s\"\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        (*(*mp).firststring).string);
                } else {
                    botimport.Print.expect("non-null function pointer")(1i32,
                                                                        b"%d\x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char,
                                                                        (*mp).variable);
                }
                if !(*mp).next.is_null() {
                    botimport.Print.expect("non-null function pointer")(1i32,
                                                                        b", \x00"
                                                                            as
                                                                            *const u8
                                                                            as
                                                                            *const libc::c_char
                                                                            as
                                                                            *mut libc::c_char);
                }
                mp = (*mp).next
            }
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b")\x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
        } else if 0 != (*key).flags & 8i32 {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"\"%s\"\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                (*key).string);
        }
        if !(*key).next.is_null() {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b", \x00" as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char);
        } else {
            botimport.Print.expect("non-null function pointer")(1i32,
                                                                b"] = %1.0f\n\x00"
                                                                    as
                                                                    *const u8
                                                                    as
                                                                    *const libc::c_char
                                                                    as
                                                                    *mut libc::c_char,
                                                                (*replychat).priority
                                                                    as
                                                                    libc::c_double);
        }
        key = (*key).next
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"{\n\x00" as
                                                            *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
}
//end of the function BotSetChatName
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotResetChatAI() {
    let mut rchat: *mut bot_replychat_t = 0 as *mut bot_replychat_t;
    let mut m: *mut bot_chatmessage_t = 0 as *mut bot_chatmessage_t;
    rchat = replychats;
    while !rchat.is_null() {
        m = (*rchat).firstchatmessage;
        while !m.is_null() {
            (*m).time = 0i32 as libc::c_float;
            m = (*m).next
        }
        rchat = (*rchat).next
    };
}