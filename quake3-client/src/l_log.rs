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
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __off_t = libc::c_long;
    pub type __off64_t = libc::c_long;
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
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
    }
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
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
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
 * name:		be_interface.h
 *
 * desc:		botlib interface
 *
 * $Archive: /source/code/botlib/be_interface.h $
 *
 *****************************************************************************/
    //#define DEBUG			//debug code
    //randomize bot behaviour
    //FIXME: get rid of this global structure
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct botlib_globals_s {
        pub botlibsetup: libc::c_int,
        pub maxentities: libc::c_int,
        pub maxclients: libc::c_int,
        pub time: libc::c_float,
    }
    pub type botlib_globals_t = botlib_globals_s;
    use super::{libc};
    use super::botlib_h::{botlib_import_t};
    extern "C" {
        #[no_mangle]
        pub static mut botlibglobals: botlib_globals_t;
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_log.c"]
pub mod l_log_c {
    pub type logfile_t = logfile_s;
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
 * name:		l_log.c
 *
 * desc:		log file
 *
 * $Archive: /MissionPack/CODE/botlib/l_log.c $
 *
 *****************************************************************************/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct logfile_s {
        pub filename: [libc::c_char; 1024],
        pub fp: *mut FILE,
        pub numwrites: libc::c_int,
    }
    use super::{libc};
    use super::FILE_h::{FILE};
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    use super::FILE_h::{FILE};
    use super::vararg::{__va_list_tag};
    extern "C" {
        #[no_mangle]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        pub fn fopen(__filename: *const libc::c_char,
                     __modes: *const libc::c_char) -> *mut FILE;
        #[no_mangle]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, ...)
         -> libc::c_int;
        #[no_mangle]
        pub fn vfprintf(_: *mut FILE, _: *const libc::c_char,
                        _: *mut __va_list_tag) -> libc::c_int;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    use super::{libc};
    extern "C" {
        // returns 0 if not defined or non numeric
        #[no_mangle]
        pub fn Cvar_VariableString(var_name: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn FS_BuildOSPath(base: *const libc::c_char,
                              game: *const libc::c_char,
                              qpath: *const libc::c_char)
         -> *mut libc::c_char;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
    use super::{libc};
    extern "C" {
        //creates the library variable if not existing already and returns the value
        #[no_mangle]
        pub fn LibVarValue(var_name: *const libc::c_char,
                           value: *const libc::c_char) -> libc::c_float;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_log.h"]
pub mod l_log_h {
    use super::{libc};
    use super::FILE_h::{FILE};
}
use self::vararg::{__builtin_va_list, __va_list_tag};
use self::stddef_h::{size_t};
use self::types_h::{__off_t, __off64_t};
use self::libio_h::{_IO_FILE, _IO_lock_t, _IO_marker};
use self::FILE_h::{FILE};
use self::stdarg_h::{va_list};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, Q_strncpyz};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::be_interface_h::{botlib_globals_s, botlib_globals_t, botlibglobals,
                           botimport};
use self::l_log_c::{logfile_t, logfile_s};
use self::stdio_h::{fclose, fflush, fopen, fprintf, vfprintf};
use self::string_h::{strlen};
use self::qcommon_h::{Cvar_VariableString, FS_BuildOSPath};
use self::l_libvar_h::{LibVarValue};
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
 * name:		l_log.h
 *
 * desc:		log file
 *
 * $Archive: /source/code/botlib/l_log.h $
 *
 *****************************************************************************/
//open a log file
#[no_mangle]
pub unsafe extern "C" fn Log_Open(mut filename: *mut libc::c_char) {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0. ==
           LibVarValue(b"log\x00" as *const u8 as *const libc::c_char,
                       b"0\x00" as *const u8 as *const libc::c_char) {
        return
    }
    if filename.is_null() || 0 == strlen(filename) {
        botimport.Print.expect("non-null function pointer")(1i32,
                                                            b"openlog <filename>\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char);
        return
    }
    if !logfile.fp.is_null() {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"log file %s is already opened\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            logfile.filename.as_mut_ptr());
        return
    }
    ospath =
        FS_BuildOSPath(Cvar_VariableString(b"fs_homepath\x00" as *const u8 as
                                               *const libc::c_char),
                       Cvar_VariableString(b"fs_game\x00" as *const u8 as
                                               *const libc::c_char),
                       filename);
    logfile.fp = fopen(ospath, b"wb\x00" as *const u8 as *const libc::c_char);
    if logfile.fp.is_null() {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"can\'t open the log file %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            filename);
        return
    }
    Q_strncpyz(logfile.filename.as_mut_ptr(), filename, 1024i32);
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"Opened log %s\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        logfile.filename.as_mut_ptr());
}
static mut logfile: logfile_t =
    logfile_s{filename: [0; 1024],
              fp: 0 as *const FILE as *mut FILE,
              numwrites: 0,};
//close the current log file
#[no_mangle]
pub unsafe extern "C" fn Log_Close() {
    if logfile.fp.is_null() { return }
    if 0 != fclose(logfile.fp) {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"can\'t close log file %s\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            logfile.filename.as_mut_ptr());
        return
    }
    logfile.fp = 0 as *mut FILE;
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"Closed log %s\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char,
                                                        logfile.filename.as_mut_ptr());
}
//close log file if present
#[no_mangle]
pub unsafe extern "C" fn Log_Shutdown() {
    if !logfile.fp.is_null() { Log_Close(); };
}
//write to the current opened log file
#[no_mangle]
pub unsafe extern "C" fn Log_Write(mut fmt: *mut libc::c_char, ...) {
    if logfile.fp.is_null() { return }
    vfprintf(logfile.fp, fmt, ap);
    fflush(logfile.fp);
}
//write to the current opened log file with a time stamp
#[no_mangle]
pub unsafe extern "C" fn Log_WriteTimeStamped(mut fmt:
                                                  *mut libc::c_char, ...) {
    if logfile.fp.is_null() { return }
    fprintf(logfile.fp,
            b"%d   %02d:%02d:%02d:%02d   \x00" as *const u8 as
                *const libc::c_char, logfile.numwrites,
            (botlibglobals.time / 60i32 as libc::c_float /
                 60i32 as libc::c_float) as libc::c_int,
            (botlibglobals.time / 60i32 as libc::c_float) as libc::c_int,
            botlibglobals.time as libc::c_int,
            (botlibglobals.time * 100i32 as libc::c_float) as libc::c_int -
                botlibglobals.time as libc::c_int * 100i32);
    vfprintf(logfile.fp, fmt, ap_0);
    fprintf(logfile.fp, b"\r\n\x00" as *const u8 as *const libc::c_char);
    logfile.numwrites += 1;
    fflush(logfile.fp);
}
//returns a pointer to the log file
#[no_mangle]
pub unsafe extern "C" fn Log_FilePointer() -> *mut FILE { return logfile.fp; }
//flush log file
#[no_mangle]
pub unsafe extern "C" fn Log_Flush() {
    if !logfile.fp.is_null() { fflush(logfile.fp); };
}