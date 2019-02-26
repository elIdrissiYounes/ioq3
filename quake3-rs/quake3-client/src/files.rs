#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           extern_types,
           libc,
           ptr_wrapping_offset_from)]
extern crate libc;
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __off_t = libc::c_long;
    pub type __off64_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/lib/clang/7.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/bits/types/struct_FILE.h"]
pub mod struct_FILE_h {
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
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut libc::c_void,
        pub __pad5: size_t,
        pub _mode: libc::c_int,
        pub _unused2: [libc::c_char; 20],
    }
    pub type _IO_lock_t = ();
    use super::{libc};
    use super::types_h::{__off_t, __off64_t};
    use super::stddef_h::{size_t};
    extern "C" {
        pub type _IO_wide_data;
        pub type _IO_codecvt;
        pub type _IO_marker;
    }
}
#[header_src = "/usr/include/bits/types/FILE.h"]
pub mod FILE_h {
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::{_IO_FILE};
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    pub type off_t = __off_t;
    use super::types_h::{__off_t};
    use super::{libc};
    use super::FILE_h::{FILE};
    use super::stddef_h::{size_t};
    extern "C" {
        #[no_mangle]
        pub fn remove(__filename: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn rename(__old: *const libc::c_char, __new: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn fclose(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        pub fn fflush(__stream: *mut FILE) -> libc::c_int;
        #[no_mangle]
        pub fn setvbuf(__stream: *mut FILE, __buf: *mut libc::c_char,
                       __modes: libc::c_int, __n: size_t) -> libc::c_int;
        #[no_mangle]
        pub fn fread(_: *mut libc::c_void, _: libc::c_ulong, _: libc::c_ulong,
                     _: *mut FILE) -> libc::c_ulong;
        #[no_mangle]
        pub fn fwrite(_: *const libc::c_void, _: libc::c_ulong,
                      _: libc::c_ulong, _: *mut FILE) -> libc::c_ulong;
        #[no_mangle]
        pub fn fseek(__stream: *mut FILE, __off: libc::c_long,
                     __whence: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn ftell(__stream: *mut FILE) -> libc::c_long;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    pub type __compar_fn_t
        =
        Option<unsafe extern "C" fn(_: *const libc::c_void,
                                    _: *const libc::c_void) -> libc::c_int>;
    use super::{libc};
    use super::stddef_h::{size_t};
    extern "C" {
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn qsort(__base: *mut libc::c_void, __nmemb: size_t,
                     __size: size_t, __compar: __compar_fn_t);
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
    // mode parm for FS_FOpenFile
    pub type fsMode_t = libc::c_uint;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const FS_APPEND: fsMode_t = 2;
    pub const FS_WRITE: fsMode_t = 1;
    pub const FS_READ: fsMode_t = 0;
    pub type unnamed_0 = libc::c_uint;
    pub const FS_SEEK_SET: unnamed_0 = 2;
    pub const FS_SEEK_END: unnamed_0 = 1;
    pub const FS_SEEK_CUR: unnamed_0 = 0;
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
        pub fn COM_GetExtension(name: *const libc::c_char)
         -> *const libc::c_char;
        #[no_mangle]
        pub fn COM_StripExtension(in_0: *const libc::c_char,
                                  out: *mut libc::c_char,
                                  destsize: libc::c_int);
        #[no_mangle]
        pub fn COM_CompareExtension(in_0: *const libc::c_char,
                                    ext: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn Q_stricmpn(s1: *const libc::c_char, s2: *const libc::c_char,
                          n: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Q_strlwr(s1: *mut libc::c_char) -> *mut libc::c_char;
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
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    pub type unnamed_1 = libc::c_uint;
    pub const VMI_COMPILED: unnamed_1 = 2;
    pub const VMI_BYTECODE: unnamed_1 = 1;
    pub const VMI_NATIVE: unnamed_1 = 0;
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
    use super::q_shared_h::{cvar_t, qboolean, fileHandle_t, fsMode_t};
    use super::FILE_h::{FILE};
    extern "C" {
        /*
==============================================================

PROTOCOL

==============================================================
*/
        // 1.31 - 67
        // maintain a list of compatible protocols for demo playing
// NOTE: that stuff only works with two digits protocols
        #[no_mangle]
        pub static mut demo_protocols: [libc::c_int; 0];
        // allocates an initial text buffer that will grow as needed
        #[no_mangle]
        pub fn Cbuf_AddText(text: *const libc::c_char);
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
        // The functions that execute commands get their parameters with these
// functions. Cmd_Argv () will return an empty string, not a NULL
// if arg > argc, so string operations are allways safe.
        #[no_mangle]
        pub fn Cmd_TokenizeString(text: *const libc::c_char);
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
        // returns 0 if not defined or non numeric
        #[no_mangle]
        pub fn Cvar_VariableString(var_name: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cvar_ForceReset(var_name: *const libc::c_char);
        #[no_mangle]
        pub static mut com_basegame: *mut cvar_t;
        #[no_mangle]
        pub static mut com_journalDataFile: fileHandle_t;
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub static mut com_journal: *mut cvar_t;
        #[no_mangle]
        pub fn Hunk_AllocateTempMemory(size: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Sys_FOpen(ospath: *const libc::c_char,
                         mode: *const libc::c_char) -> *mut FILE;
        #[no_mangle]
        pub static mut com_legacyprotocol: *mut cvar_t;
        #[no_mangle]
        pub static mut com_protocol: *mut cvar_t;
        #[no_mangle]
        pub static mut com_fullyInitialized: qboolean;
        #[no_mangle]
        pub static mut com_standalone: *mut cvar_t;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Sys_FreeFileList(list: *mut *mut libc::c_char);
        #[no_mangle]
        pub fn CopyString(in_0: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Sys_ListFiles(directory: *const libc::c_char,
                             extension: *const libc::c_char,
                             filter: *mut libc::c_char,
                             numfiles: *mut libc::c_int, wantsubs: qboolean)
         -> *mut *mut libc::c_char;
        #[no_mangle]
        pub fn Com_FilterPath(filter: *mut libc::c_char,
                              name: *mut libc::c_char,
                              casesensitive: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Com_BlockChecksum(buffer: *const libc::c_void,
                                 length: libc::c_int) -> libc::c_uint;
        #[no_mangle]
        pub fn Sys_Mkdir(path: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn Sys_SteamPath() -> *mut libc::c_char;
        #[no_mangle]
        pub fn Sys_GogPath() -> *mut libc::c_char;
        #[no_mangle]
        pub fn Sys_DefaultHomePath() -> *mut libc::c_char;
        #[no_mangle]
        pub fn Sys_DefaultInstallPath() -> *mut libc::c_char;
        #[no_mangle]
        pub fn Com_StartupVariable(match_0: *const libc::c_char);
        #[no_mangle]
        pub fn Com_SafeMode() -> qboolean;
        #[no_mangle]
        pub fn Sys_InitPIDFile(gamedir: *const libc::c_char);
        #[no_mangle]
        pub fn Sys_RemovePIDFile(gamedir: *const libc::c_char);
        #[no_mangle]
        pub fn Com_GameRestart(checksumFeed: libc::c_int,
                               disconnect: qboolean);
        // for writing the config files
        #[no_mangle]
        pub fn S_ClearSoundBuffer();
        #[no_mangle]
        pub fn Sys_DllExtension(name: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn Sys_Mkfifo(ospath: *const libc::c_char) -> *mut FILE;
        #[no_mangle]
        pub fn Hunk_ClearTempMemory();
        #[no_mangle]
        pub fn Hunk_FreeTempMemory(buf: *mut libc::c_void);
    }
}
#[header_src =
      "ioq3/code/qcommon/files.c"]
pub mod files_c {
    pub type searchpath_t = searchpath_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct searchpath_s {
        pub next: *mut searchpath_s,
        pub pack: *mut pack_t,
        pub dir: *mut directory_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct directory_t {
        pub path: [libc::c_char; 4096],
        pub fullpath: [libc::c_char; 4096],
        pub gamedir: [libc::c_char; 4096],
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct pack_t {
        pub pakPathname: [libc::c_char; 4096],
        pub pakFilename: [libc::c_char; 4096],
        pub pakBasename: [libc::c_char; 4096],
        pub pakGamename: [libc::c_char; 4096],
        pub handle: unzFile,
        pub checksum: libc::c_int,
        pub pure_checksum: libc::c_int,
        pub numfiles: libc::c_int,
        pub referenced: libc::c_int,
        pub hashSize: libc::c_int,
        pub hashTable: *mut *mut fileInPack_t,
        pub buildBuffer: *mut fileInPack_t,
    }
    pub type fileInPack_t = fileInPack_s;
    // if this is defined, the executable positively won't work with any paks other
// than the demo pak, even if productid is present.  This is only used for our
// last demo release to prevent the mac and linux users from using the demo
// executable with the production windows pak before the mac/linux products
// hit the shelves a little later
// NOW defined in build files
//#define PRE_RELEASE_TADEMO
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct fileInPack_s {
        pub name: *mut libc::c_char,
        pub pos: libc::c_ulong,
        pub len: libc::c_ulong,
        pub next: *mut fileInPack_s,
    }
    pub type qfile_gut = qfile_gus;
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union qfile_gus {
        pub o: *mut FILE,
        pub z: unzFile,
    }
    pub type qfile_ut = qfile_us;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct qfile_us {
        pub file: qfile_gut,
        pub unique: qboolean,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct fileHandleData_t {
        pub handleFiles: qfile_ut,
        pub handleSync: qboolean,
        pub fileSize: libc::c_int,
        pub zipFilePos: libc::c_int,
        pub zipFileLen: libc::c_int,
        pub zipFile: qboolean,
        pub name: [libc::c_char; 256],
    }
    use super::{libc};
    use super::unzip_h::{unzFile};
    use super::FILE_h::{FILE};
    use super::q_shared_h::{qboolean, fileHandle_t};
    extern "C" {
        #[no_mangle]
        pub fn Com_AppendCDKey(filename: *const libc::c_char);
        #[no_mangle]
        pub fn Com_ReadCDKey(filename: *const libc::c_char);
    }
}
#[header_src =
      "ioq3/code/qcommon/unzip.h"]
pub mod unzip_h {
    /* unzip.h -- IO for uncompress .zip files using zlib
   Version 1.01e, February 12th, 2005

   Copyright (C) 1998-2005 Gilles Vollant

   This unzip package allow extract file from .ZIP file, compatible with PKZip 2.04g
     WinZip, InfoZip tools and compatible.

   Multi volume ZipFile (span) are not supported.
   Encryption compatible with pkzip 2.04g only supported
   Old compressions used by old PKZip 1.x are not supported


   I WAIT FEEDBACK at mail info@winimage.com
   Visit also http://www.winimage.com/zLibDll/unzip.htm for evolution

   Condition of use and distribution are the same than zlib :

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
    /* for more info about .ZIP format, see
      http://www.info-zip.org/pub/infozip/doc/appnote-981119-iz.zip
      http://www.info-zip.org/pub/infozip/doc/
   PkWare has also a specification at :
      ftp://ftp.pkware.com/probdesc.zip
*/
    pub type unzFile = voidp;
    pub type unz_file_info = unz_file_info_s;
    /* unz_file_info contain information about a file in the zipfile */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct unz_file_info_s {
        pub version: uLong,
        pub version_needed: uLong,
        pub flag: uLong,
        pub compression_method: uLong,
        pub dosDate: uLong,
        pub crc: uLong,
        pub compressed_size: uLong,
        pub uncompressed_size: uLong,
        pub size_filename: uLong,
        pub size_file_extra: uLong,
        pub size_file_comment: uLong,
        pub disk_num_start: uLong,
        pub internal_fa: uLong,
        pub external_fa: uLong,
        pub tmu_date: tm_unz,
    }
    pub type tm_unz = tm_unz_s;
    /* tm_unz contain date/time info */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct tm_unz_s {
        pub tm_sec: uInt,
        pub tm_min: uInt,
        pub tm_hour: uInt,
        pub tm_mday: uInt,
        pub tm_mon: uInt,
        pub tm_year: uInt,
    }
    pub type unz_global_info = unz_global_info_s;
    /* unz_global_info structure contain global data about the ZIPfile
   These data comes from the end of central dir */
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct unz_global_info_s {
        pub number_entry: uLong,
        pub size_comment: uLong,
    }
    use super::zconf_h::{voidp, uLong, uInt};
    use super::{libc};
    use super::stdio_h::{off_t};
    extern "C" {
        /*
   Open a Zip file, like unzOpen, but provide a set of file low level API
      for read/write the zip file (see ioapi.h)
*/
        #[no_mangle]
        pub fn unzClose(file: unzFile) -> libc::c_int;
        /*
  Same than unzOpenCurrentFile, but open for read raw the file (not uncompress)
    if raw==1
  *method will receive method of compression, *level will receive level of
     compression
  note : you can set level parameter as NULL (if you did not want known level,
         but you CANNOT set method parameter as NULL
*/
        #[no_mangle]
        pub fn unzCloseCurrentFile(file: unzFile) -> libc::c_int;
        /*
  Close the file in zip opened with unzOpenCurrentFile
  Return UNZ_CRCERROR if all the file was read but the CRC is not good
*/
        #[no_mangle]
        pub fn unzReadCurrentFile(file: unzFile, buf: voidp,
                                  len: libc::c_uint) -> libc::c_int;
        /*
  Get Info about the current file
  if pfile_info!=NULL, the *pfile_info structure will contain somes info about
        the current file
  if szFileName!=NULL, the filemane string will be copied in szFileName
            (fileNameBufferSize is the size of the buffer)
  if extraField!=NULL, the extra field information will be copied in extraField
            (extraFieldBufferSize is the size of the buffer).
            This is the Central-header version of the extra field
  if szComment!=NULL, the comment string of the file will be copied in szComment
            (commentBufferSize is the size of the buffer)
*/
        /* **************************************************************************/
/* for reading the content of the current zipfile, you can open it, read data
   from it, and close it (you can close it before reading all the file)
   */
        #[no_mangle]
        pub fn unzOpenCurrentFile(file: unzFile) -> libc::c_int;
        /* Set the current file offset */
        #[no_mangle]
        pub fn unzSetOffset(file: unzFile, pos: uLong) -> libc::c_int;
        /*
   Compare two filename (fileName1,fileName2).
   If iCaseSenisivity = 1, comparison is case sensitivity (like strcmp)
   If iCaseSenisivity = 2, comparison is not case sensitivity (like strcmpi
                                or strcasecmp)
   If iCaseSenisivity = 0, case sensitivity is defaut of your operating system
    (like 1 on Unix, 2 on Windows)
*/
        #[no_mangle]
        pub fn unzOpen(path: *const libc::c_char) -> unzFile;
        /*
  Set the current file of the zipfile to the first file.
  return UNZ_OK if there is no problem
*/
        #[no_mangle]
        pub fn unzGoToNextFile(file: unzFile) -> libc::c_int;
        /*
  Read extra field from the current file (opened by unzOpenCurrentFile)
  This is the local-header version of the extra field (sometimes, there is
    more info in the local-header version than in the central-header)

  if buf==NULL, it return the size of the local extra field

  if buf!=NULL, len is the size of the buffer, the extra header is copied in
    buf.
  the return value is the number of bytes copied in buf, or (if <0)
    the error code
*/
        /* **************************************************************************/
        /* Get the current file offset */
        #[no_mangle]
        pub fn unzGetOffset(file: unzFile) -> uLong;
        /* ****************************************** */
        #[no_mangle]
        pub fn unzGetCurrentFileInfo(file: unzFile,
                                     pfile_info: *mut unz_file_info,
                                     szFileName: *mut libc::c_char,
                                     fileNameBufferSize: uLong,
                                     extraField: *mut libc::c_void,
                                     extraFieldBufferSize: uLong,
                                     szComment: *mut libc::c_char,
                                     commentBufferSize: uLong) -> libc::c_int;
        /*
  Get the global comment string of the ZipFile, in the szComment buffer.
  uSizeBuf is the size of the szComment buffer.
  return the number of byte copied or an error code <0
*/
        /* **************************************************************************/
/* Unzip package allow you browse the directory of the zipfile */
        #[no_mangle]
        pub fn unzGoToFirstFile(file: unzFile) -> libc::c_int;
        /*
  Close a ZipFile opened with unzipOpen.
  If there is files inside the .Zip opened with unzOpenCurrentFile (see later),
    these files MUST be closed with unzipCloseCurrentFile before call unzipClose.
  return UNZ_OK if there is no problem. */
        #[no_mangle]
        pub fn unzGetGlobalInfo(file: unzFile,
                                pglobal_info: *mut unz_global_info)
         -> libc::c_int;
        /*
  Read bytes from the current file (opened by unzOpenCurrentFile)
  buf contain buffer where data must be copied
  len the size of buf.

  return the number of byte copied if somes bytes are copied
  return 0 if the end of file was reached
  return <0 with error code if there is an error
    (UNZ_ERRNO for IO error, or zLib error for uncompress error)
*/
        #[no_mangle]
        pub fn unztell(file: unzFile) -> off_t;
    }
}
#[header_src = "/usr/include/zconf.h"]
pub mod zconf_h {
    pub type voidp = *mut libc::c_void;
    pub type uLong = libc::c_ulong;
    pub type uInt = libc::c_uint;
    use super::{libc};
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
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
use self::types_h::{__off_t, __off64_t};
use self::stddef_h::{size_t};
use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt,
                          _IO_marker};
use self::FILE_h::{FILE};
use self::stdio_h::{off_t, remove, rename, fclose, fflush, setvbuf, fread,
                    fwrite, fseek, ftell};
use self::stdlib_h::{__compar_fn_t, atoi, qsort};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, fsMode_t, FS_APPEND_SYNC,
                       FS_APPEND, FS_WRITE, FS_READ, unnamed_0, FS_SEEK_SET,
                       FS_SEEK_END, FS_SEEK_CUR, cvar_s, cvar_t,
                       COM_GetExtension, COM_StripExtension,
                       COM_CompareExtension, Com_sprintf, Q_stricmp,
                       Q_stricmpn, Q_strlwr, Q_strncpyz, Q_strcat, va,
                       Com_Error, Com_Printf};
use self::qcommon_h::{unnamed_1, VMI_COMPILED, VMI_BYTECODE, VMI_NATIVE,
                      xcommand_t, demo_protocols, Cbuf_AddText,
                      Cmd_AddCommand, Cmd_RemoveCommand, Cmd_Argc, Cmd_Argv,
                      Cmd_TokenizeString, Cvar_Get, Cvar_Set,
                      Cvar_VariableString, Cvar_ForceReset, com_basegame,
                      com_journalDataFile, Com_DPrintf, com_journal,
                      Hunk_AllocateTempMemory, Sys_FOpen, com_legacyprotocol,
                      com_protocol, com_fullyInitialized, com_standalone,
                      Z_Free, Z_MallocDebug, Sys_FreeFileList, CopyString,
                      Sys_ListFiles, Com_FilterPath, Com_BlockChecksum,
                      Sys_Mkdir, Sys_SteamPath, Sys_GogPath,
                      Sys_DefaultHomePath, Sys_DefaultInstallPath,
                      Com_StartupVariable, Com_SafeMode, Sys_InitPIDFile,
                      Sys_RemovePIDFile, Com_GameRestart, S_ClearSoundBuffer,
                      Sys_DllExtension, Sys_Mkfifo, Hunk_ClearTempMemory,
                      Hunk_FreeTempMemory};
use self::files_c::{searchpath_t, searchpath_s, directory_t, pack_t,
                    fileInPack_t, fileInPack_s, qfile_gut, qfile_gus,
                    qfile_ut, qfile_us, fileHandleData_t, Com_AppendCDKey,
                    Com_ReadCDKey};
use self::unzip_h::{unzFile, unz_file_info, unz_file_info_s, tm_unz, tm_unz_s,
                    unz_global_info, unz_global_info_s, unzClose,
                    unzCloseCurrentFile, unzReadCurrentFile,
                    unzOpenCurrentFile, unzSetOffset, unzOpen,
                    unzGoToNextFile, unzGetOffset, unzGetCurrentFileInfo,
                    unzGoToFirstFile, unzGetGlobalInfo, unztell};
use self::zconf_h::{voidp, uLong, uInt};
use self::string_h::{memcpy, memmove, memset, strcpy, strcmp, strchr, strrchr,
                     strstr, strlen};
use self::ctype_h::{tolower};
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
pub unsafe extern "C" fn FS_Initialized() -> qboolean {
    return (fs_searchpaths != 0 as *mut libc::c_void as *mut searchpath_t) as
               libc::c_int as qboolean;
}
static mut fs_searchpaths: *mut searchpath_t =
    0 as *const searchpath_t as *mut searchpath_t;
#[no_mangle]
pub unsafe extern "C" fn FS_InitFilesystem() {
    Com_StartupVariable(b"fs_basepath\x00" as *const u8 as
                            *const libc::c_char);
    Com_StartupVariable(b"fs_homepath\x00" as *const u8 as
                            *const libc::c_char);
    Com_StartupVariable(b"fs_game\x00" as *const u8 as *const libc::c_char);
    if 0 ==
           FS_FilenameCompare(Cvar_VariableString(b"fs_game\x00" as *const u8
                                                      as *const libc::c_char),
                              (*com_basegame).string) as u64 {
        Cvar_Set(b"fs_game\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char);
    }
    FS_Startup((*com_basegame).string);
    FS_CheckPak0();
    if FS_ReadFile(b"default.cfg\x00" as *const u8 as *const libc::c_char,
                   0 as *mut *mut libc::c_void) <= 0i32 as libc::c_long {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Couldn\'t load default.cfg\x00" as *const u8 as
                      *const libc::c_char);
    }
    Q_strncpyz(lastValidBase.as_mut_ptr(), (*fs_basepath).string,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz(lastValidComBaseGame.as_mut_ptr(), (*com_basegame).string,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz(lastValidFsBaseGame.as_mut_ptr(), (*fs_basegame).string,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz(lastValidGame.as_mut_ptr(), (*fs_gamedirvar).string,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
}
#[no_mangle]
pub static mut lastValidGame: [libc::c_char; 4096] = [0; 4096];
static mut fs_gamedirvar: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut lastValidFsBaseGame: [libc::c_char; 4096] = [0; 4096];
static mut fs_basegame: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut lastValidComBaseGame: [libc::c_char; 4096] = [0; 4096];
// last valid game folder used
#[no_mangle]
pub static mut lastValidBase: [libc::c_char; 4096] = [0; 4096];
static mut fs_basepath: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn FS_ReadFile(mut qpath: *const libc::c_char,
                                     mut buffer: *mut *mut libc::c_void)
 -> libc::c_long {
    return FS_ReadFileDir(qpath, 0 as *mut libc::c_void, qfalse, buffer);
}
// note: you can't just fclose from another DLL, due to MS libc issues
#[no_mangle]
pub unsafe extern "C" fn FS_ReadFileDir(mut qpath: *const libc::c_char,
                                        mut searchPath: *mut libc::c_void,
                                        mut unpure: qboolean,
                                        mut buffer: *mut *mut libc::c_void)
 -> libc::c_long {
    let mut h: fileHandle_t = 0;
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut buf: *mut byte = 0 as *mut byte;
    let mut isConfig: qboolean = qfalse;
    let mut len: libc::c_long = 0;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if qpath.is_null() || 0 == *qpath.offset(0isize) {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"FS_ReadFile with empty name\x00" as *const u8 as
                      *const libc::c_char);
    }
    buf = 0 as *mut byte;
    if !strstr(qpath,
               b".cfg\x00" as *const u8 as *const libc::c_char).is_null() {
        isConfig = qtrue;
        if !com_journal.is_null() && (*com_journal).integer == 2i32 {
            let mut r: libc::c_int = 0;
            Com_DPrintf(b"Loading %s from journal file.\n\x00" as *const u8 as
                            *const libc::c_char, qpath);
            r =
                FS_Read(&mut len as *mut libc::c_long as *mut libc::c_void,
                        ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                            as libc::c_int, com_journalDataFile);
            if r as libc::c_ulong !=
                   ::std::mem::size_of::<libc::c_long>() as libc::c_ulong {
                if !buffer.is_null() { *buffer = 0 as *mut libc::c_void }
                return -1i32 as libc::c_long
            }
            if 0 == len {
                if buffer.is_null() { return 1i32 as libc::c_long }
                *buffer = 0 as *mut libc::c_void;
                return -1i32 as libc::c_long
            }
            if buffer.is_null() { return len }
            buf =
                Hunk_AllocateTempMemory((len + 1i32 as libc::c_long) as
                                            libc::c_int) as *mut byte;
            *buffer = buf as *mut libc::c_void;
            r =
                FS_Read(buf as *mut libc::c_void, len as libc::c_int,
                        com_journalDataFile);
            if r as libc::c_long != len {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"Read from journalDataFile failed\x00" as *const u8
                              as *const libc::c_char);
            }
            fs_loadCount += 1;
            fs_loadStack += 1;
            *buf.offset(len as isize) = 0i32 as byte;
            return len
        }
    } else { isConfig = qfalse }
    search = searchPath as *mut searchpath_t;
    if search.is_null() {
        len = FS_FOpenFileRead(qpath, &mut h, qfalse)
    } else {
        len = FS_FOpenFileReadDir(qpath, search, &mut h, qfalse, unpure)
    }
    if h == 0i32 {
        if !buffer.is_null() { *buffer = 0 as *mut libc::c_void }
        if 0 != isConfig as libc::c_uint && !com_journal.is_null() &&
               (*com_journal).integer == 1i32 {
            Com_DPrintf(b"Writing zero for %s to journal file.\n\x00" as
                            *const u8 as *const libc::c_char, qpath);
            len = 0i32 as libc::c_long;
            FS_Write(&mut len as *mut libc::c_long as *const libc::c_void,
                     ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as
                         libc::c_int, com_journalDataFile);
            FS_Flush(com_journalDataFile);
        }
        return -1i32 as libc::c_long
    }
    if buffer.is_null() {
        if 0 != isConfig as libc::c_uint && !com_journal.is_null() &&
               (*com_journal).integer == 1i32 {
            Com_DPrintf(b"Writing len for %s to journal file.\n\x00" as
                            *const u8 as *const libc::c_char, qpath);
            FS_Write(&mut len as *mut libc::c_long as *const libc::c_void,
                     ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as
                         libc::c_int, com_journalDataFile);
            FS_Flush(com_journalDataFile);
        }
        FS_FCloseFile(h);
        return len
    }
    fs_loadCount += 1;
    fs_loadStack += 1;
    buf =
        Hunk_AllocateTempMemory((len + 1i32 as libc::c_long) as libc::c_int)
            as *mut byte;
    *buffer = buf as *mut libc::c_void;
    FS_Read(buf as *mut libc::c_void, len as libc::c_int, h);
    *buf.offset(len as isize) = 0i32 as byte;
    FS_FCloseFile(h);
    if 0 != isConfig as libc::c_uint && !com_journal.is_null() &&
           (*com_journal).integer == 1i32 {
        Com_DPrintf(b"Writing %s to journal file.\n\x00" as *const u8 as
                        *const libc::c_char, qpath);
        FS_Write(&mut len as *mut libc::c_long as *const libc::c_void,
                 ::std::mem::size_of::<libc::c_long>() as libc::c_ulong as
                     libc::c_int, com_journalDataFile);
        FS_Write(buf as *const libc::c_void, len as libc::c_int,
                 com_journalDataFile);
        FS_Flush(com_journalDataFile);
    }
    return len;
}
// where are we?
#[no_mangle]
pub unsafe extern "C" fn FS_Flush(mut f: fileHandle_t) {
    fflush(fsh[f as usize].handleFiles.file.o);
}
static mut fsh: [fileHandleData_t; 64] =
    [fileHandleData_t{handleFiles:
                          qfile_us{file:
                                       qfile_gus{o:
                                                     0 as *const FILE as
                                                         *mut FILE,},
                                   unique: qfalse,},
                      handleSync: qfalse,
                      fileSize: 0,
                      zipFilePos: 0,
                      zipFileLen: 0,
                      zipFile: qfalse,
                      name: [0; 256],}; 64];
// returns 1 if a file is in the PAK file, otherwise -1
#[no_mangle]
pub unsafe extern "C" fn FS_Write(mut buffer: *const libc::c_void,
                                  mut len: libc::c_int, mut h: fileHandle_t)
 -> libc::c_int {
    let mut block: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut written: libc::c_int = 0;
    let mut buf: *mut byte = 0 as *mut byte;
    let mut tries: libc::c_int = 0;
    let mut f: *mut FILE = 0 as *mut FILE;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if 0 == h { return 0i32 }
    f = FS_FileForHandle(h);
    buf = buffer as *mut byte;
    remaining = len;
    tries = 0i32;
    while 0 != remaining {
        block = remaining;
        written =
            fwrite(buf as *const libc::c_void, 1i32 as libc::c_ulong,
                   block as libc::c_ulong, f) as libc::c_int;
        if written == 0i32 {
            if 0 == tries {
                tries = 1i32
            } else {
                Com_Printf(b"FS_Write: 0 bytes written\n\x00" as *const u8 as
                               *const libc::c_char);
                return 0i32
            }
        }
        if written == -1i32 {
            Com_Printf(b"FS_Write: -1 bytes written\n\x00" as *const u8 as
                           *const libc::c_char);
            return 0i32
        }
        remaining -= written;
        buf = buf.offset(written as isize)
    }
    if 0 != fsh[h as usize].handleSync as u64 { fflush(f); }
    return len;
}
unsafe extern "C" fn FS_FileForHandle(mut f: fileHandle_t) -> *mut FILE {
    if f < 1i32 || f >= 64i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"FS_FileForHandle: out of range\x00" as *const u8 as
                      *const libc::c_char);
    }
    if fsh[f as usize].zipFile as libc::c_uint ==
           qtrue as libc::c_int as libc::c_uint {
        Com_Error(ERR_DROP as libc::c_int,
                  b"FS_FileForHandle: can\'t get FILE on zip file\x00" as
                      *const u8 as *const libc::c_char);
    }
    if fsh[f as usize].handleFiles.file.o.is_null() {
        Com_Error(ERR_DROP as libc::c_int,
                  b"FS_FileForHandle: NULL\x00" as *const u8 as
                      *const libc::c_char);
    }
    return fsh[f as usize].handleFiles.file.o;
}
// properly handles partial reads and reads from other dlls
#[no_mangle]
pub unsafe extern "C" fn FS_FCloseFile(mut f: fileHandle_t) {
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if fsh[f as usize].zipFile as libc::c_uint ==
           qtrue as libc::c_int as libc::c_uint {
        unzCloseCurrentFile(fsh[f as usize].handleFiles.file.z);
        if 0 != fsh[f as usize].handleFiles.unique as u64 {
            unzClose(fsh[f as usize].handleFiles.file.z);
        }
        memset(&mut *fsh.as_mut_ptr().offset(f as isize) as
                   *mut fileHandleData_t as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<fileHandleData_t>() as libc::c_ulong);
        return
    }
    if !fsh[f as usize].handleFiles.file.o.is_null() {
        fclose(fsh[f as usize].handleFiles.file.o);
    }
    memset(&mut *fsh.as_mut_ptr().offset(f as isize) as *mut fileHandleData_t
               as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<fileHandleData_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn FS_Read(mut buffer: *mut libc::c_void,
                                 mut len: libc::c_int, mut f: fileHandle_t)
 -> libc::c_int {
    let mut block: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut read: libc::c_int = 0;
    let mut buf: *mut byte = 0 as *mut byte;
    let mut tries: libc::c_int = 0;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if 0 == f { return 0i32 }
    buf = buffer as *mut byte;
    fs_readCount += len;
    if fsh[f as usize].zipFile as libc::c_uint ==
           qfalse as libc::c_int as libc::c_uint {
        remaining = len;
        tries = 0i32;
        while 0 != remaining {
            block = remaining;
            read =
                fread(buf as *mut libc::c_void, 1i32 as libc::c_ulong,
                      block as libc::c_ulong,
                      fsh[f as usize].handleFiles.file.o) as libc::c_int;
            if read == 0i32 {
                if 0 == tries { tries = 1i32 } else { return len - remaining }
            }
            if read == -1i32 {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"FS_Read: -1 bytes read\x00" as *const u8 as
                              *const libc::c_char);
            }
            remaining -= read;
            buf = buf.offset(read as isize)
        }
        return len
    } else {
        return unzReadCurrentFile(fsh[f as usize].handleFiles.file.z, buffer,
                                  len as libc::c_uint)
    };
}
// total bytes read
static mut fs_readCount: libc::c_int = 0;
// total files in memory
static mut fs_loadStack: libc::c_int = 0;
// total files read
static mut fs_loadCount: libc::c_int = 0;
/*
===========
FS_FOpenFileReadDir

Tries opening file "filename" in searchpath "search"
Returns filesize and an open FILE pointer.
===========
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FOpenFileReadDir(mut filename:
                                                 *const libc::c_char,
                                             mut search: *mut searchpath_t,
                                             mut file: *mut fileHandle_t,
                                             mut uniqueFILE: qboolean,
                                             mut unpure: qboolean)
 -> libc::c_long {
    let mut hash: libc::c_long = 0;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut pakFile: *mut fileInPack_t = 0 as *mut fileInPack_t;
    let mut dir: *mut directory_t = 0 as *mut directory_t;
    let mut netpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filep: *mut FILE = 0 as *mut FILE;
    let mut len: libc::c_int = 0;
    if filename.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"FS_FOpenFileRead: NULL \'filename\' parameter passed\x00"
                      as *const u8 as *const libc::c_char);
    }
    if *filename.offset(0isize) as libc::c_int == '/' as i32 ||
           *filename.offset(0isize) as libc::c_int == '\\' as i32 {
        filename = filename.offset(1isize)
    }
    if !strstr(filename,
               b"..\x00" as *const u8 as *const libc::c_char).is_null() ||
           !strstr(filename,
                   b"::\x00" as *const u8 as *const libc::c_char).is_null() {
        if file.is_null() { return qfalse as libc::c_int as libc::c_long }
        *file = 0i32;
        return -1i32 as libc::c_long
    }
    if 0 != com_fullyInitialized as libc::c_uint &&
           !strstr(filename,
                   b"q3key\x00" as *const u8 as *const libc::c_char).is_null()
       {
        if file.is_null() { return qfalse as libc::c_int as libc::c_long }
        *file = 0i32;
        return -1i32 as libc::c_long
    }
    if file.is_null() {
        if !(*search).pack.is_null() {
            hash = FS_HashFileName(filename, (*(*search).pack).hashSize);
            if !(*(*(*search).pack).hashTable.offset(hash as isize)).is_null()
               {
                pak = (*search).pack;
                pakFile = *(*pak).hashTable.offset(hash as isize);
                loop  {
                    if 0 ==
                           FS_FilenameCompare((*pakFile).name, filename) as
                               u64 {
                        if 0 != (*pakFile).len {
                            return (*pakFile).len as libc::c_long
                        } else { return 1i32 as libc::c_long }
                    }
                    pakFile = (*pakFile).next;
                    if pakFile.is_null() { break ; }
                }
            }
        } else if !(*search).dir.is_null() {
            dir = (*search).dir;
            netpath =
                FS_BuildOSPath((*dir).path.as_mut_ptr(),
                               (*dir).gamedir.as_mut_ptr(), filename);
            filep =
                Sys_FOpen(netpath,
                          b"rb\x00" as *const u8 as *const libc::c_char);
            if !filep.is_null() {
                len = FS_fplength(filep) as libc::c_int;
                fclose(filep);
                if 0 != len {
                    return len as libc::c_long
                } else { return 1i32 as libc::c_long }
            }
        }
        return 0i32 as libc::c_long
    }
    *file = FS_HandleForFile();
    fsh[*file as usize].handleFiles.unique = uniqueFILE;
    if !(*search).pack.is_null() {
        hash = FS_HashFileName(filename, (*(*search).pack).hashSize);
        if !(*(*(*search).pack).hashTable.offset(hash as isize)).is_null() {
            if 0 == unpure as u64 && 0 == FS_PakIsPure((*search).pack) as u64
               {
                *file = 0i32;
                return -1i32 as libc::c_long
            }
            pak = (*search).pack;
            pakFile = *(*pak).hashTable.offset(hash as isize);
            loop  {
                if 0 == FS_FilenameCompare((*pakFile).name, filename) as u64 {
                    len = strlen(filename) as libc::c_int;
                    if 0 == (*pak).referenced & 0x1i32 {
                        if 0 ==
                               FS_IsExt(filename,
                                        b".shader\x00" as *const u8 as
                                            *const libc::c_char, len) as u64
                               &&
                               0 ==
                                   FS_IsExt(filename,
                                            b".txt\x00" as *const u8 as
                                                *const libc::c_char, len) as
                                       u64 &&
                               0 ==
                                   FS_IsExt(filename,
                                            b".cfg\x00" as *const u8 as
                                                *const libc::c_char, len) as
                                       u64 &&
                               0 ==
                                   FS_IsExt(filename,
                                            b".config\x00" as *const u8 as
                                                *const libc::c_char, len) as
                                       u64 &&
                               0 ==
                                   FS_IsExt(filename,
                                            b".bot\x00" as *const u8 as
                                                *const libc::c_char, len) as
                                       u64 &&
                               0 ==
                                   FS_IsExt(filename,
                                            b".arena\x00" as *const u8 as
                                                *const libc::c_char, len) as
                                       u64 &&
                               0 ==
                                   FS_IsExt(filename,
                                            b".menu\x00" as *const u8 as
                                                *const libc::c_char, len) as
                                       u64 &&
                               Q_stricmp(filename,
                                         b"vm/qagame.qvm\x00" as *const u8 as
                                             *const libc::c_char) != 0i32 &&
                               strstr(filename,
                                      b"levelshots\x00" as *const u8 as
                                          *const libc::c_char).is_null() {
                            (*pak).referenced |= 0x1i32
                        }
                    }
                    if !strstr(filename,
                               b"cgame.qvm\x00" as *const u8 as
                                   *const libc::c_char).is_null() {
                        (*pak).referenced |= 0x4i32
                    }
                    if !strstr(filename,
                               b"ui.qvm\x00" as *const u8 as
                                   *const libc::c_char).is_null() {
                        (*pak).referenced |= 0x2i32
                    }
                    if 0 != uniqueFILE as u64 {
                        fsh[*file as usize].handleFiles.file.z =
                            unzOpen((*pak).pakFilename.as_mut_ptr());
                        if fsh[*file as usize].handleFiles.file.z.is_null() {
                            Com_Error(ERR_FATAL as libc::c_int,
                                      b"Couldn\'t open %s\x00" as *const u8 as
                                          *const libc::c_char,
                                      (*pak).pakFilename.as_mut_ptr());
                        }
                    } else {
                        fsh[*file as usize].handleFiles.file.z = (*pak).handle
                    }
                    Q_strncpyz(fsh[*file as usize].name.as_mut_ptr(),
                               filename,
                               ::std::mem::size_of::<[libc::c_char; 256]>() as
                                   libc::c_ulong as libc::c_int);
                    fsh[*file as usize].zipFile = qtrue;
                    unzSetOffset(fsh[*file as usize].handleFiles.file.z,
                                 (*pakFile).pos);
                    unzOpenCurrentFile(fsh[*file as
                                               usize].handleFiles.file.z);
                    fsh[*file as usize].zipFilePos =
                        (*pakFile).pos as libc::c_int;
                    fsh[*file as usize].zipFileLen =
                        (*pakFile).len as libc::c_int;
                    if 0 != (*fs_debug).integer {
                        Com_Printf(b"FS_FOpenFileRead: %s (found in \'%s\')\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   filename, (*pak).pakFilename.as_mut_ptr());
                    }
                    return (*pakFile).len as libc::c_long
                }
                pakFile = (*pakFile).next;
                if pakFile.is_null() { break ; }
            }
        }
    } else if !(*search).dir.is_null() {
        len = strlen(filename) as libc::c_int;
        if 0 == unpure as u64 && 0 != fs_numServerPaks {
            if 0 ==
                   FS_IsExt(filename,
                            b".cfg\x00" as *const u8 as *const libc::c_char,
                            len) as u64 &&
                   0 ==
                       FS_IsExt(filename,
                                b".menu\x00" as *const u8 as
                                    *const libc::c_char, len) as u64 &&
                   0 ==
                       FS_IsExt(filename,
                                b".game\x00" as *const u8 as
                                    *const libc::c_char, len) as u64 &&
                   0 ==
                       FS_IsExt(filename,
                                b".dat\x00" as *const u8 as
                                    *const libc::c_char, len) as u64 &&
                   0 == FS_IsDemoExt(filename, len) as u64 {
                *file = 0i32;
                return -1i32 as libc::c_long
            }
        }
        dir = (*search).dir;
        netpath =
            FS_BuildOSPath((*dir).path.as_mut_ptr(),
                           (*dir).gamedir.as_mut_ptr(), filename);
        filep =
            Sys_FOpen(netpath, b"rb\x00" as *const u8 as *const libc::c_char);
        if filep.is_null() { *file = 0i32; return -1i32 as libc::c_long }
        Q_strncpyz(fsh[*file as usize].name.as_mut_ptr(), filename,
                   ::std::mem::size_of::<[libc::c_char; 256]>() as
                       libc::c_ulong as libc::c_int);
        fsh[*file as usize].zipFile = qfalse;
        if 0 != (*fs_debug).integer {
            Com_Printf(b"FS_FOpenFileRead: %s (found in \'%s%c%s\')\n\x00" as
                           *const u8 as *const libc::c_char, filename,
                       (*dir).path.as_mut_ptr(), '/' as i32,
                       (*dir).gamedir.as_mut_ptr());
        }
        fsh[*file as usize].handleFiles.file.o = filep;
        return FS_fplength(filep)
    }
    *file = 0i32;
    return -1i32 as libc::c_long;
}
/*
================
FS_fplength
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_fplength(mut h: *mut FILE) -> libc::c_long {
    let mut pos: libc::c_long = 0;
    let mut end: libc::c_long = 0;
    pos = ftell(h);
    fseek(h, 0i32 as libc::c_long, 2i32);
    end = ftell(h);
    fseek(h, pos, 0i32);
    return end;
}
static mut fs_debug: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn FS_BuildOSPath(mut base: *const libc::c_char,
                                        mut game: *const libc::c_char,
                                        mut qpath: *const libc::c_char)
 -> *mut libc::c_char {
    let mut temp: [libc::c_char; 4096] = [0; 4096];
    static mut ospath: [[libc::c_char; 4096]; 2] = [[0; 4096]; 2];
    static mut toggle: libc::c_int = 0;
    toggle ^= 1i32;
    if game.is_null() || 0 == *game.offset(0isize) {
        game = fs_gamedir.as_mut_ptr()
    }
    Com_sprintf(temp.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"/%s/%s\x00" as *const u8 as *const libc::c_char, game,
                qpath);
    FS_ReplaceSeparators(temp.as_mut_ptr());
    Com_sprintf(ospath[toggle as usize].as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%s%s\x00" as *const u8 as *const libc::c_char, base,
                temp.as_mut_ptr());
    return ospath[toggle as usize].as_mut_ptr();
}
/*
====================
FS_ReplaceSeparators

Fix things up differently for win/unix/mac
====================
*/
unsafe extern "C" fn FS_ReplaceSeparators(mut path: *mut libc::c_char) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lastCharWasSep: qboolean = qfalse;
    s = path;
    while 0 != *s {
        if *s as libc::c_int == '/' as i32 || *s as libc::c_int == '\\' as i32
           {
            if 0 == lastCharWasSep as u64 {
                *s = '/' as i32 as libc::c_char;
                lastCharWasSep = qtrue
            } else {
                memmove(s as *mut libc::c_void,
                        s.offset(1isize) as *const libc::c_void, strlen(s));
            }
        } else { lastCharWasSep = qfalse }
        s = s.offset(1isize)
    };
}
// this will be a single file name with no separators
static mut fs_gamedir: [libc::c_char; 4096] = [0; 4096];
/*
===========
FS_IsDemoExt

Return qtrue if filename has a demo extension
===========
*/
#[no_mangle]
pub unsafe extern "C" fn FS_IsDemoExt(mut filename: *const libc::c_char,
                                      mut namelen: libc::c_int) -> qboolean {
    let mut ext_test: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut index: libc::c_int = 0;
    let mut protocol: libc::c_int = 0;
    ext_test = strrchr(filename, '.' as i32);
    if !ext_test.is_null() &&
           0 ==
               Q_stricmpn(ext_test.offset(1isize),
                          b"dm_\x00" as *const u8 as *const libc::c_char,
                          (::std::mem::size_of::<[libc::c_char; 4]>() as
                               libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                               as
                                                               libc::c_ulong).wrapping_sub(1i32
                                                                                               as
                                                                                               libc::c_ulong)
                              as libc::c_int) {
        protocol =
            atoi(ext_test.offset((::std::mem::size_of::<[libc::c_char; 4]>()
                                      as
                                      libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_char>()
                                                                      as
                                                                      libc::c_ulong)
                                     as isize));
        if protocol == (*com_protocol).integer { return qtrue }
        if protocol == (*com_legacyprotocol).integer { return qtrue }
        index = 0i32;
        while 0 != *demo_protocols.as_mut_ptr().offset(index as isize) {
            if *demo_protocols.as_mut_ptr().offset(index as isize) == protocol
               {
                return qtrue
            }
            index += 1
        }
    }
    return qfalse;
}
/*
===========
FS_IsExt

Return qtrue if ext matches file extension filename
===========
*/
#[no_mangle]
pub unsafe extern "C" fn FS_IsExt(mut filename: *const libc::c_char,
                                  mut ext: *const libc::c_char,
                                  mut namelen: libc::c_int) -> qboolean {
    let mut extlen: libc::c_int = 0;
    extlen = strlen(ext) as libc::c_int;
    if extlen > namelen { return qfalse }
    filename = filename.offset((namelen - extlen) as isize);
    return (0 == Q_stricmp(filename, ext)) as libc::c_int as qboolean;
}
// never load anything from pk3 files that are not present at the server when pure
static mut fs_numServerPaks: libc::c_int = 0i32;
// seek on a file
#[no_mangle]
pub unsafe extern "C" fn FS_FilenameCompare(mut s1: *const libc::c_char,
                                            mut s2: *const libc::c_char)
 -> qboolean {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    loop  {
        let fresh0 = s1;
        s1 = s1.offset(1);
        c1 = *fresh0 as libc::c_int;
        let fresh1 = s2;
        s2 = s2.offset(1);
        c2 = *fresh1 as libc::c_int;
        if c1 >= 'a' as i32 && c1 <= 'z' as i32 {
            c1 -= 'a' as i32 - 'A' as i32
        }
        if c2 >= 'a' as i32 && c2 <= 'z' as i32 {
            c2 -= 'a' as i32 - 'A' as i32
        }
        if c1 == '\\' as i32 || c1 == ':' as i32 { c1 = '/' as i32 }
        if c2 == '\\' as i32 || c2 == ':' as i32 { c2 = '/' as i32 }
        if c1 != c2 { return qtrue }
        if !(0 != c1) { break ; }
    }
    return qfalse;
}
/*
=================
FS_PakIsPure
=================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_PakIsPure(mut pack: *mut pack_t) -> qboolean {
    let mut i: libc::c_int = 0;
    if 0 != fs_numServerPaks {
        i = 0i32;
        while i < fs_numServerPaks {
            if (*pack).checksum == fs_serverPaks[i as usize] { return qtrue }
            i += 1
        }
        return qfalse
    }
    return qtrue;
}
// checksums
static mut fs_serverPaks: [libc::c_int; 4096] = [0; 4096];
/*
================
return a hash value for the filename
================
*/
unsafe extern "C" fn FS_HashFileName(mut fname: *const libc::c_char,
                                     mut hashSize: libc::c_int)
 -> libc::c_long {
    let mut i: libc::c_int = 0;
    let mut hash: libc::c_long = 0;
    let mut letter: libc::c_char = 0;
    hash = 0i32 as libc::c_long;
    i = 0i32;
    while *fname.offset(i as isize) as libc::c_int != '\u{0}' as i32 {
        letter =
            tolower(*fname.offset(i as isize) as libc::c_int) as libc::c_char;
        if letter as libc::c_int == '.' as i32 {
            // don't include extension
            break ;
        } else {
            if letter as libc::c_int == '\\' as i32 {
                letter = '/' as i32 as libc::c_char
            }
            if letter as libc::c_int == '/' as i32 {
                letter = '/' as i32 as libc::c_char
            }
            hash += letter as libc::c_long * (i + 119i32) as libc::c_long;
            i += 1
        }
    }
    hash = hash ^ hash >> 10i32 ^ hash >> 20i32;
    hash &= (hashSize - 1i32) as libc::c_long;
    return hash;
}
unsafe extern "C" fn FS_HandleForFile() -> fileHandle_t {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < 64i32 {
        if fsh[i as usize].handleFiles.file.o.is_null() { return i }
        i += 1
    }
    Com_Error(ERR_DROP as libc::c_int,
              b"FS_HandleForFile: none free\x00" as *const u8 as
                  *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn FS_FOpenFileRead(mut filename: *const libc::c_char,
                                          mut file: *mut fileHandle_t,
                                          mut uniqueFILE: qboolean)
 -> libc::c_long {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut len: libc::c_long = 0;
    let mut isLocalConfig: qboolean = qfalse;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    isLocalConfig =
        (0 ==
             strcmp(filename,
                    b"autoexec.cfg\x00" as *const u8 as *const libc::c_char)
             ||
             0 ==
                 strcmp(filename,
                        b"q3config.cfg\x00" as *const u8 as
                            *const libc::c_char)) as libc::c_int as qboolean;
    search = fs_searchpaths;
    while !search.is_null() {
        // autoexec.cfg and q3config.cfg can only be loaded outside of pk3 files.
        if !(0 != isLocalConfig as libc::c_uint && !(*search).pack.is_null())
           {
            len =
                FS_FOpenFileReadDir(filename, search, file, uniqueFILE,
                                    qfalse);
            if file.is_null() {
                if len > 0i32 as libc::c_long { return len }
            } else if len >= 0i32 as libc::c_long && 0 != *file { return len }
        }
        search = (*search).next
    }
    if !file.is_null() {
        *file = 0i32;
        return -1i32 as libc::c_long
    } else { return 0i32 as libc::c_long };
}
/*
===================
FS_CheckPak0

Check whether any of the original id pak files is present,
and start up in standalone mode, if there are none and a
different com_basegame was set.
Note: If you're building a game that doesn't depend on the
Q3 media pak0.pk3, you'll want to remove this by defining
STANDALONE in q_shared.h
===================
*/
unsafe extern "C" fn FS_CheckPak0() {
    let mut path: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut curpack: *mut pack_t = 0 as *mut pack_t;
    let mut pakBasename: *const libc::c_char = 0 as *const libc::c_char;
    let mut founddemo: qboolean = qfalse;
    let mut foundPak: libc::c_uint = 0i32 as libc::c_uint;
    let mut foundTA: libc::c_uint = 0i32 as libc::c_uint;
    path = fs_searchpaths;
    while !path.is_null() {
        if !(*path).pack.is_null() {
            curpack = (*path).pack;
            pakBasename = (*curpack).pakBasename.as_mut_ptr();
            if 0 ==
                   Q_stricmpn((*curpack).pakGamename.as_mut_ptr(),
                              b"demoq3\x00" as *const u8 as
                                  *const libc::c_char, 4096i32) &&
                   0 ==
                       Q_stricmpn(pakBasename,
                                  b"pak0\x00" as *const u8 as
                                      *const libc::c_char, 4096i32) {
                if (*curpack).checksum as libc::c_uint == 2985612116u32 {
                    founddemo = qtrue
                }
            } else if 0 ==
                          Q_stricmpn((*curpack).pakGamename.as_mut_ptr(),
                                     b"baseq3\x00" as *const u8 as
                                         *const libc::c_char, 4096i32) &&
                          strlen(pakBasename) == 4i32 as libc::c_ulong &&
                          0 ==
                              Q_stricmpn(pakBasename,
                                         b"pak\x00" as *const u8 as
                                             *const libc::c_char, 3i32) &&
                          *pakBasename.offset(3isize) as libc::c_int >=
                              '0' as i32 &&
                          *pakBasename.offset(3isize) as libc::c_int <=
                              '0' as i32 + 9i32 - 1i32 {
                if (*curpack).checksum as libc::c_uint !=
                       pak_checksums[(*pakBasename.offset(3isize) as
                                          libc::c_int - '0' as i32) as usize]
                   {
                    if *pakBasename.offset(3isize) as libc::c_int ==
                           '0' as i32 {
                        Com_Printf(b"\n\n**************************************************\nWARNING: baseq3/pak0.pk3 is present but its checksum (%u)\nis not correct. Please re-copy pak0.pk3 from your\nlegitimate Q3 CDROM.\n**************************************************\n\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   (*curpack).checksum);
                    } else {
                        Com_Printf(b"\n\n**************************************************\nWARNING: baseq3/pak%d.pk3 is present but its checksum (%u)\nis not correct. Please re-install the point release\n**************************************************\n\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   *pakBasename.offset(3isize) as libc::c_int
                                       - '0' as i32, (*curpack).checksum);
                    }
                }
                foundPak |=
                    (1i32 <<
                         *pakBasename.offset(3isize) as libc::c_int -
                             '0' as i32) as libc::c_uint
            } else if 0 ==
                          Q_stricmpn((*curpack).pakGamename.as_mut_ptr(),
                                     b"missionpack\x00" as *const u8 as
                                         *const libc::c_char, 4096i32) &&
                          strlen(pakBasename) == 4i32 as libc::c_ulong &&
                          0 ==
                              Q_stricmpn(pakBasename,
                                         b"pak\x00" as *const u8 as
                                             *const libc::c_char, 3i32) &&
                          *pakBasename.offset(3isize) as libc::c_int >=
                              '0' as i32 &&
                          *pakBasename.offset(3isize) as libc::c_int <=
                              '0' as i32 + 4i32 - 1i32 {
                if (*curpack).checksum as libc::c_uint !=
                       missionpak_checksums[(*pakBasename.offset(3isize) as
                                                 libc::c_int - '0' as i32) as
                                                usize] {
                    Com_Printf(b"\n\n**************************************************\nWARNING: missionpack/pak%d.pk3 is present but its checksum (%u)\nis not correct. Please re-install Team Arena\n**************************************************\n\n\n\x00"
                                   as *const u8 as *const libc::c_char,
                               *pakBasename.offset(3isize) as libc::c_int -
                                   '0' as i32, (*curpack).checksum);
                }
                foundTA |=
                    (1i32 <<
                         *pakBasename.offset(3isize) as libc::c_int -
                             '0' as i32) as libc::c_uint
            } else {
                let mut index: libc::c_int = 0;
                index = 0i32;
                while (index as libc::c_ulong) <
                          (::std::mem::size_of::<[libc::c_uint; 9]>() as
                               libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_uint>()
                                                               as
                                                               libc::c_ulong)
                      {
                    if (*curpack).checksum as libc::c_uint ==
                           pak_checksums[index as usize] {
                        Com_Printf(b"\n\n**************************************************\nWARNING: %s is renamed pak file %s%cpak%d.pk3\nRunning in standalone mode won\'t work\nPlease rename, or remove this file\n**************************************************\n\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   (*curpack).pakFilename.as_mut_ptr(),
                                   b"baseq3\x00" as *const u8 as
                                       *const libc::c_char, '/' as i32,
                                   index);
                        foundPak |= 0x80000000u32
                    }
                    index += 1
                }
                index = 0i32;
                while (index as libc::c_ulong) <
                          (::std::mem::size_of::<[libc::c_uint; 4]>() as
                               libc::c_ulong).wrapping_div(::std::mem::size_of::<libc::c_uint>()
                                                               as
                                                               libc::c_ulong)
                      {
                    if (*curpack).checksum as libc::c_uint ==
                           missionpak_checksums[index as usize] {
                        Com_Printf(b"\n\n**************************************************\nWARNING: %s is renamed pak file %s%cpak%d.pk3\nRunning in standalone mode won\'t work\nPlease rename, or remove this file\n**************************************************\n\n\n\x00"
                                       as *const u8 as *const libc::c_char,
                                   (*curpack).pakFilename.as_mut_ptr(),
                                   b"missionpack\x00" as *const u8 as
                                       *const libc::c_char, '/' as i32,
                                   index);
                        foundTA |= 0x80000000u32
                    }
                    index += 1
                }
            }
        }
        path = (*path).next
    }
    if 0 == foundPak && 0 == foundTA &&
           0 !=
               Q_stricmp((*com_basegame).string,
                         b"baseq3\x00" as *const u8 as *const libc::c_char) {
        Cvar_Set(b"com_standalone\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char);
    } else {
        Cvar_Set(b"com_standalone\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char);
    }
    if 0 == (*com_standalone).integer {
        if 0 == foundPak & 0x1i32 as libc::c_uint {
            if 0 != founddemo as u64 {
                Com_Printf(b"\n\n**************************************************\nWARNING: It looks like you\'re using pak0.pk3\nfrom the demo. This may work fine, but it is not\nguaranteed or supported.\n**************************************************\n\n\n\x00"
                               as *const u8 as *const libc::c_char);
                foundPak |= 0x1i32 as libc::c_uint
            }
        }
    }
    if 0 == (*com_standalone).integer &&
           foundPak & 0x1ffi32 as libc::c_uint != 0x1ffi32 as libc::c_uint {
        let mut errorText: [libc::c_char; 1024] =
            *::std::mem::transmute::<&[u8; 1024],
                                     &mut [libc::c_char; 1024]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        if foundPak & 0x1i32 as libc::c_uint != 0x1i32 as libc::c_uint {
            Q_strcat(errorText.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong as libc::c_int,
                     b"\"pak0.pk3\" is missing. Please copy it from your legitimate Q3 CDROM. \x00"
                         as *const u8 as *const libc::c_char);
        }
        if foundPak & 0x1fei32 as libc::c_uint != 0x1fei32 as libc::c_uint {
            Q_strcat(errorText.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong as libc::c_int,
                     b"Point Release files are missing. Please re-install the 1.32 point release. \x00"
                         as *const u8 as *const libc::c_char);
        }
        Q_strcat(errorText.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong as libc::c_int,
                 va(b"Also check that your ioq3 executable is in the correct place and that every file in the \"%s\" directory is present and readable\x00"
                        as *const u8 as *const libc::c_char as
                        *mut libc::c_char,
                    b"baseq3\x00" as *const u8 as *const libc::c_char));
        Com_Error(ERR_FATAL as libc::c_int,
                  b"%s\x00" as *const u8 as *const libc::c_char,
                  errorText.as_mut_ptr());
    }
    if 0 == (*com_standalone).integer && 0 != foundTA &&
           foundTA & 0xfi32 as libc::c_uint != 0xfi32 as libc::c_uint {
        let mut errorText_0: [libc::c_char; 1024] =
            *::std::mem::transmute::<&[u8; 1024],
                                     &mut [libc::c_char; 1024]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00");
        if foundTA & 0x1i32 as libc::c_uint != 0x1i32 as libc::c_uint {
            Com_sprintf(errorText_0.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as
                            libc::c_ulong as libc::c_int,
                        b"\"missionpack%cpak0.pk3\" is missing. Please copy it from your legitimate Quake 3 Team Arena CDROM. \x00"
                            as *const u8 as *const libc::c_char, '/' as i32);
        }
        if foundTA & 0xei32 as libc::c_uint != 0xei32 as libc::c_uint {
            Q_strcat(errorText_0.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 1024]>() as
                         libc::c_ulong as libc::c_int,
                     b"Team Arena Point Release files are missing. Please re-install the latest Team Arena point release.\x00"
                         as *const u8 as *const libc::c_char);
        }
        Com_Error(ERR_FATAL as libc::c_int,
                  b"%s\x00" as *const u8 as *const libc::c_char,
                  errorText_0.as_mut_ptr());
    };
}
static mut missionpak_checksums: [libc::c_uint; 4] =
    [2430342401u32, 511014160u32, 2662638993u32, 1438664554u32];
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
/*****************************************************************************
 * name:		files.c
 *
 * desc:		handle based filesystem for Quake III Arena 
 *
 * $Archive: /MissionPack/code/qcommon/files.c $
 *
 *****************************************************************************/
/*
=============================================================================

QUAKE3 FILESYSTEM

All of Quake's data access is through a hierarchical file system, but the contents of 
the file system can be transparently merged from several sources.

A "qpath" is a reference to game file data.  MAX_ZPATH is 256 characters, which must include
a terminating zero. "..", "\\", and ":" are explicitly illegal in qpaths to prevent any
references outside the quake directory system.

The "base path" is the path to the directory holding all the game directories and usually
the executable.  It defaults to ".", but can be overridden with a "+set fs_basepath c:\quake3"
command line to allow code debugging in a different directory.  Basepath cannot
be modified at all after startup.  Any files that are created (demos, screenshots,
etc) will be created relative to the base path, so base path should usually be writable.

The "home path" is the path used for all write access. On win32 systems we have "base path"
== "home path", but on *nix systems the base installation is usually readonly, and
"home path" points to ~/.q3a or similar

The user can also install custom mods and content in "home path", so it should be searched
along with "home path" and "cd path" for game content.


The "base game" is the directory under the paths where data comes from by default, and
can be either "baseq3" or "demoq3".

The "current game" may be the same as the base game, or it may be the name of another
directory under the paths that should be searched for files before looking in the base game.
This is the basis for addons.

Clients automatically set the game directory after receiving a gamestate from a server,
so only servers need to worry about +set fs_game.

No other directories outside of the base game and current game will ever be referenced by
filesystem functions.

To save disk space and speed loading, directory trees can be collapsed into zip files.
The files use a ".pk3" extension to prevent users from unzipping them accidentally, but
otherwise the are simply normal uncompressed zip files.  A game directory can have multiple
zip files of the form "pak0.pk3", "pak1.pk3", etc.  Zip files are searched in decending order
from the highest number to the lowest, and will always take precedence over the filesystem.
This allows a pk3 distributed as a patch to override all existing data.

Because we will have updated executables freely available online, there is no point to
trying to restrict demo / oem versions of the game with code changes.  Demo / oem versions
should be exactly the same executables as release versions, but with different data that
automatically restricts where game media can come from to prevent add-ons from working.

File search order: when FS_FOpenFileRead gets called it will go through the fs_searchpaths
structure and stop on the first successful hit. fs_searchpaths is built with successive
calls to FS_AddGameDirectory

Additionally, we search in several subdirectories:
current game is the current mode
base game is a variable to allow mods based on other mods
(such as baseq3 + missionpack content combination in a mod for instance)
BASEGAME is the hardcoded base game ("baseq3")

e.g. the qpath "sound/newstuff/test.wav" would be searched for in the following places:

home path + current game's zip files
home path + current game's directory
base path + current game's zip files
base path + current game's directory
cd path + current game's zip files
cd path + current game's directory

home path + base game's zip file
home path + base game's directory
base path + base game's zip file
base path + base game's directory
cd path + base game's zip file
cd path + base game's directory

home path + BASEGAME's zip file
home path + BASEGAME's directory
base path + BASEGAME's zip file
base path + BASEGAME's directory
cd path + BASEGAME's zip file
cd path + BASEGAME's directory

server download, to be written to home path + current game's directory


The filesystem can be safely shutdown and reinitialized with different
basedir / cddir / game combinations, but all other subsystems that rely on it
(sound, video) must also be forced to restart.

Because the same files are loaded by both the clip model (CM_) and renderer (TR_)
subsystems, a simple single-file caching scheme is used.  The CM_ subsystems will
load the file with a request to cache.  Only one file will be kept cached at a time,
so any models that are going to be referenced by both subsystems should alternate
between the CM_ load function and the ref load function.

TODO: A qpath that starts with a leading slash will always refer to the base game, even if another
game is currently active.  This allows character models, skins, and sounds to be downloaded
to a common directory no matter which game is active.

How to prevent downloading zip files?
Pass pk3 file names in systeminfo, and download before FS_Restart()?

Aborting a download disconnects the client from the server.

How to mark files as downloadable?  Commercial add-ons won't be downloadable.

Non-commercial downloads will want to download the entire zip file.
the game would have to be reset to actually read the zip in

Auto-update information

Path separators

Casing

  separate server gamedir and client gamedir, so if the user starts
  a local game after having connected to a network game, it won't stick
  with the network game.

  allow menu options for game selection?

Read / write config to floppy option.

Different version coexistance?

When building a pak file, make sure a q3config.cfg isn't present in it,
or configs will never get loaded from disk!

  todo:

  downloading (outside fs?)
  game directory passing and restarting

=============================================================================

*/
// every time a new demo pk3 file is built, this checksum must be updated.
// the easiest way to get it is to just run the game and see what it spits out
static mut pak_checksums: [libc::c_uint; 9] =
    [1566731103u32, 298122907u32, 412165236u32, 2991495316u32, 1197932710u32,
     4087071573u32, 3709064859u32, 908855077u32, 977125798u32];
/*
================
FS_Startup
================
*/
unsafe extern "C" fn FS_Startup(mut gameName: *const libc::c_char) {
    let mut homePath: *const libc::c_char = 0 as *const libc::c_char;
    Com_Printf(b"----- FS_Startup -----\n\x00" as *const u8 as
                   *const libc::c_char);
    fs_packFiles = 0i32;
    fs_debug =
        Cvar_Get(b"fs_debug\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    fs_basepath =
        Cvar_Get(b"fs_basepath\x00" as *const u8 as *const libc::c_char,
                 Sys_DefaultInstallPath(), 0x10i32 | 0x2000i32);
    fs_basegame =
        Cvar_Get(b"fs_basegame\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0x10i32);
    homePath = Sys_DefaultHomePath();
    if homePath.is_null() || 0 == *homePath.offset(0isize) {
        homePath = (*fs_basepath).string
    }
    fs_homepath =
        Cvar_Get(b"fs_homepath\x00" as *const u8 as *const libc::c_char,
                 homePath, 0x10i32 | 0x2000i32);
    fs_gamedirvar =
        Cvar_Get(b"fs_game\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 0x10i32 | 0x8i32);
    if 0 == *gameName.offset(0isize) {
        Cvar_ForceReset(b"com_basegame\x00" as *const u8 as
                            *const libc::c_char);
    }
    if 0 == FS_FilenameCompare((*fs_gamedirvar).string, gameName) as u64 {
        Cvar_ForceReset(b"fs_game\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != FS_InvalidGameDir(gameName) as u64 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Invalid com_basegame \'%s\'\x00" as *const u8 as
                      *const libc::c_char, gameName);
    }
    if 0 != FS_InvalidGameDir((*fs_basegame).string) as u64 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Invalid fs_basegame \'%s\'\x00" as *const u8 as
                      *const libc::c_char, (*fs_basegame).string);
    }
    if 0 != FS_InvalidGameDir((*fs_gamedirvar).string) as u64 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Invalid fs_game \'%s\'\x00" as *const u8 as
                      *const libc::c_char, (*fs_gamedirvar).string);
    }
    fs_gogpath =
        Cvar_Get(b"fs_gogpath\x00" as *const u8 as *const libc::c_char,
                 Sys_GogPath(), 0x10i32 | 0x2000i32);
    if 0 != *(*fs_gogpath).string.offset(0isize) {
        FS_AddGameDirectory((*fs_gogpath).string, gameName);
    }
    fs_steampath =
        Cvar_Get(b"fs_steampath\x00" as *const u8 as *const libc::c_char,
                 Sys_SteamPath(), 0x10i32 | 0x2000i32);
    if 0 != *(*fs_steampath).string.offset(0isize) {
        FS_AddGameDirectory((*fs_steampath).string, gameName);
    }
    if 0 != *(*fs_basepath).string.offset(0isize) {
        FS_AddGameDirectory((*fs_basepath).string, gameName);
    }
    if 0 != *(*fs_homepath).string.offset(0isize) as libc::c_int &&
           0 != Q_stricmp((*fs_homepath).string, (*fs_basepath).string) {
        FS_CreatePath((*fs_homepath).string);
        FS_AddGameDirectory((*fs_homepath).string, gameName);
    }
    if 0 != *(*fs_basegame).string.offset(0isize) as libc::c_int &&
           0 != Q_stricmp((*fs_basegame).string, gameName) {
        if 0 != *(*fs_gogpath).string.offset(0isize) {
            FS_AddGameDirectory((*fs_gogpath).string, (*fs_basegame).string);
        }
        if 0 != *(*fs_steampath).string.offset(0isize) {
            FS_AddGameDirectory((*fs_steampath).string,
                                (*fs_basegame).string);
        }
        if 0 != *(*fs_basepath).string.offset(0isize) {
            FS_AddGameDirectory((*fs_basepath).string, (*fs_basegame).string);
        }
        if 0 != *(*fs_homepath).string.offset(0isize) as libc::c_int &&
               0 != Q_stricmp((*fs_homepath).string, (*fs_basepath).string) {
            FS_AddGameDirectory((*fs_homepath).string, (*fs_basegame).string);
        }
    }
    if 0 != *(*fs_gamedirvar).string.offset(0isize) as libc::c_int &&
           0 != Q_stricmp((*fs_gamedirvar).string, gameName) {
        if 0 != *(*fs_gogpath).string.offset(0isize) {
            FS_AddGameDirectory((*fs_gogpath).string,
                                (*fs_gamedirvar).string);
        }
        if 0 != *(*fs_steampath).string.offset(0isize) {
            FS_AddGameDirectory((*fs_steampath).string,
                                (*fs_gamedirvar).string);
        }
        if 0 != *(*fs_basepath).string.offset(0isize) {
            FS_AddGameDirectory((*fs_basepath).string,
                                (*fs_gamedirvar).string);
        }
        if 0 != *(*fs_homepath).string.offset(0isize) as libc::c_int &&
               0 != Q_stricmp((*fs_homepath).string, (*fs_basepath).string) {
            FS_AddGameDirectory((*fs_homepath).string,
                                (*fs_gamedirvar).string);
        }
    }
    if 0 == (*com_standalone).integer {
        Com_ReadCDKey(b"baseq3\x00" as *const u8 as *const libc::c_char);
        if 0 != *(*fs_gamedirvar).string.offset(0isize) {
            Com_AppendCDKey((*fs_gamedirvar).string);
        }
    }
    Cmd_AddCommand(b"path\x00" as *const u8 as *const libc::c_char,
                   Some(FS_Path_f));
    Cmd_AddCommand(b"dir\x00" as *const u8 as *const libc::c_char,
                   Some(FS_Dir_f));
    Cmd_AddCommand(b"fdir\x00" as *const u8 as *const libc::c_char,
                   Some(FS_NewDir_f));
    Cmd_AddCommand(b"touchFile\x00" as *const u8 as *const libc::c_char,
                   Some(FS_TouchFile_f));
    Cmd_AddCommand(b"which\x00" as *const u8 as *const libc::c_char,
                   Some(FS_Which_f));
    FS_ReorderPurePaks();
    FS_Path_f();
    (*fs_gamedirvar).modified = qfalse;
    Com_Printf(b"----------------------\n\x00" as *const u8 as
                   *const libc::c_char);
    Com_Printf(b"%d files in pk3 files\n\x00" as *const u8 as
                   *const libc::c_char, fs_packFiles);
}
// total number of files in packs
static mut fs_packFiles: libc::c_int = 0i32;
/*
============
FS_Path_f

============
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Path_f() {
    let mut s: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut i: libc::c_int = 0;
    Com_Printf(b"We are looking in the current search path:\n\x00" as
                   *const u8 as *const libc::c_char);
    s = fs_searchpaths;
    while !s.is_null() {
        if !(*s).pack.is_null() {
            Com_Printf(b"%s (%i files)\n\x00" as *const u8 as
                           *const libc::c_char,
                       (*(*s).pack).pakFilename.as_mut_ptr(),
                       (*(*s).pack).numfiles);
            if 0 != fs_numServerPaks {
                if 0 == FS_PakIsPure((*s).pack) as u64 {
                    Com_Printf(b"    not on the pure list\n\x00" as *const u8
                                   as *const libc::c_char);
                } else {
                    Com_Printf(b"    on the pure list\n\x00" as *const u8 as
                                   *const libc::c_char);
                }
            }
        } else {
            Com_Printf(b"%s%c%s\n\x00" as *const u8 as *const libc::c_char,
                       (*(*s).dir).path.as_mut_ptr(), '/' as i32,
                       (*(*s).dir).gamedir.as_mut_ptr());
        }
        s = (*s).next
    }
    Com_Printf(b"\n\x00" as *const u8 as *const libc::c_char);
    i = 1i32;
    while i < 64i32 {
        if !fsh[i as usize].handleFiles.file.o.is_null() {
            Com_Printf(b"handle %i: %s\n\x00" as *const u8 as
                           *const libc::c_char, i,
                       fsh[i as usize].name.as_mut_ptr());
        }
        i += 1
    };
}
/*
================
FS_ReorderPurePaks
NOTE TTimo: the reordering that happens here is not reflected in the cvars (\cvarlist *pak*)
  this can lead to misleading situations, see https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=540
================
*/
unsafe extern "C" fn FS_ReorderPurePaks() {
    let mut s: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut i: libc::c_int = 0;
    // for linked list reordering
    let mut p_insert_index: *mut *mut searchpath_t =
        0 as *mut *mut searchpath_t;
    // when doing the scan
    let mut p_previous: *mut *mut searchpath_t = 0 as *mut *mut searchpath_t;
    fs_reordered = qfalse;
    if 0 == fs_numServerPaks { return }
    p_insert_index = &mut fs_searchpaths;
    i = 0i32;
    while i < fs_numServerPaks {
        p_previous = p_insert_index;
        s = *p_insert_index;
        while !s.is_null() {
            // the part of the list before p_insert_index has been sorted already
            if !(*s).pack.is_null() &&
                   fs_serverPaks[i as usize] == (*(*s).pack).checksum {
                fs_reordered = qtrue;
                *p_previous = (*s).next;
                (*s).next = *p_insert_index;
                *p_insert_index = s;
                p_insert_index = &mut (*s).next;
                // iterate to next server pack
                break ;
            } else { p_previous = &mut (*s).next; s = (*s).next }
        }
        i += 1
    };
}
// TTimo - https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=540
// wether we did a reorder on the current search path when joining the server
static mut fs_reordered: qboolean = qfalse;
/*
============
FS_Which_f
============
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Which_f() {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    filename = Cmd_Argv(1i32);
    if 0 == *filename.offset(0isize) {
        Com_Printf(b"Usage: which <file>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if *filename.offset(0isize) as libc::c_int == '/' as i32 ||
           *filename.offset(0isize) as libc::c_int == '\\' as i32 {
        filename = filename.offset(1isize)
    }
    search = fs_searchpaths;
    while !search.is_null() {
        if 0 != FS_Which(filename, search as *mut libc::c_void) as u64 {
            return
        }
        search = (*search).next
    }
    Com_Printf(b"File not found: \"%s\"\n\x00" as *const u8 as
                   *const libc::c_char, filename);
}
#[no_mangle]
pub unsafe extern "C" fn FS_Which(mut filename: *const libc::c_char,
                                  mut searchPath: *mut libc::c_void)
 -> qboolean {
    let mut search: *mut searchpath_t = searchPath as *mut searchpath_t;
    if FS_FOpenFileReadDir(filename, search, 0 as *mut fileHandle_t, qfalse,
                           qfalse) > 0i32 as libc::c_long {
        if !(*search).pack.is_null() {
            Com_Printf(b"File \"%s\" found in \"%s\"\n\x00" as *const u8 as
                           *const libc::c_char, filename,
                       (*(*search).pack).pakFilename.as_mut_ptr());
            return qtrue
        } else {
            if !(*search).dir.is_null() {
                Com_Printf(b"File \"%s\" found at \"%s\"\n\x00" as *const u8
                               as *const libc::c_char, filename,
                           (*(*search).dir).fullpath.as_mut_ptr());
                return qtrue
            }
        }
    }
    return qfalse;
}
/*
============
FS_TouchFile_f
============
*/
#[no_mangle]
pub unsafe extern "C" fn FS_TouchFile_f() {
    let mut f: fileHandle_t = 0;
    if Cmd_Argc() != 2i32 {
        Com_Printf(b"Usage: touchFile <file>\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    FS_FOpenFileRead(Cmd_Argv(1i32), &mut f, qfalse);
    if 0 != f { FS_FCloseFile(f); };
}
/*
================
FS_NewDir_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_NewDir_f() {
    let mut filter: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ndirs: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if Cmd_Argc() < 2i32 {
        Com_Printf(b"usage: fdir <filter>\n\x00" as *const u8 as
                       *const libc::c_char);
        Com_Printf(b"example: fdir *q3dm*.bsp\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    filter = Cmd_Argv(1i32);
    Com_Printf(b"---------------\n\x00" as *const u8 as *const libc::c_char);
    dirnames =
        FS_ListFilteredFiles(b"\x00" as *const u8 as *const libc::c_char,
                             b"\x00" as *const u8 as *const libc::c_char,
                             filter, &mut ndirs, qfalse);
    FS_SortFileList(dirnames, ndirs);
    i = 0i32;
    while i < ndirs {
        FS_ConvertPath(*dirnames.offset(i as isize));
        Com_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                   *dirnames.offset(i as isize));
        i += 1
    }
    Com_Printf(b"%d files listed\n\x00" as *const u8 as *const libc::c_char,
               ndirs);
    FS_FreeFileList(dirnames);
}
// directory should not have either a leading or trailing /
// if extension is "/", only subdirectories will be returned
// the returned files will not include any directories or /
#[no_mangle]
pub unsafe extern "C" fn FS_FreeFileList(mut list: *mut *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if list.is_null() { return }
    i = 0i32;
    while !(*list.offset(i as isize)).is_null() {
        Z_Free(*list.offset(i as isize) as *mut libc::c_void);
        i += 1
    }
    Z_Free(list as *mut libc::c_void);
}
/*
===========
FS_ConvertPath
===========
*/
#[no_mangle]
pub unsafe extern "C" fn FS_ConvertPath(mut s: *mut libc::c_char) {
    while 0 != *s {
        if *s as libc::c_int == '\\' as i32 || *s as libc::c_int == ':' as i32
           {
            *s = '/' as i32 as libc::c_char
        }
        s = s.offset(1isize)
    };
}
/*
================
FS_SortFileList
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_SortFileList(mut filelist: *mut *mut libc::c_char,
                                         mut numfiles: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut numsortedfiles: libc::c_int = 0;
    let mut sortedlist: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    sortedlist =
        Z_MallocDebug(((numfiles + 1i32) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                           as libc::c_ulong)
                          as libc::c_int,
                      b"( numfiles + 1 ) * sizeof( *sortedlist )\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/files.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 2706i32)
            as *mut *mut libc::c_char;
    let ref mut fresh2 = *sortedlist.offset(0isize);
    *fresh2 = 0 as *mut libc::c_char;
    numsortedfiles = 0i32;
    i = 0i32;
    while i < numfiles {
        j = 0i32;
        while j < numsortedfiles {
            if FS_PathCmp(*filelist.offset(i as isize),
                          *sortedlist.offset(j as isize)) < 0i32 {
                break ;
            }
            j += 1
        }
        k = numsortedfiles;
        while k > j {
            let ref mut fresh3 = *sortedlist.offset(k as isize);
            *fresh3 = *sortedlist.offset((k - 1i32) as isize);
            k -= 1
        }
        let ref mut fresh4 = *sortedlist.offset(j as isize);
        *fresh4 = *filelist.offset(i as isize);
        numsortedfiles += 1;
        i += 1
    }
    memcpy(filelist as *mut libc::c_void, sortedlist as *const libc::c_void,
           (numfiles as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                as libc::c_ulong));
    Z_Free(sortedlist as *mut libc::c_void);
}
/*
===========
FS_PathCmp

Ignore case and seprator char distinctions
===========
*/
#[no_mangle]
pub unsafe extern "C" fn FS_PathCmp(mut s1: *const libc::c_char,
                                    mut s2: *const libc::c_char)
 -> libc::c_int {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    loop  {
        let fresh5 = s1;
        s1 = s1.offset(1);
        c1 = *fresh5 as libc::c_int;
        let fresh6 = s2;
        s2 = s2.offset(1);
        c2 = *fresh6 as libc::c_int;
        if c1 >= 'a' as i32 && c1 <= 'z' as i32 {
            c1 -= 'a' as i32 - 'A' as i32
        }
        if c2 >= 'a' as i32 && c2 <= 'z' as i32 {
            c2 -= 'a' as i32 - 'A' as i32
        }
        if c1 == '\\' as i32 || c1 == ':' as i32 { c1 = '/' as i32 }
        if c2 == '\\' as i32 || c2 == ':' as i32 { c2 = '/' as i32 }
        if c1 < c2 { return -1i32 }
        if c1 > c2 { return 1i32 }
        if !(0 != c1) { break ; }
    }
    return 0i32;
}
/*
===============
FS_ListFilteredFiles

Returns a uniqued list of files that match the given criteria
from all search paths
===============
*/
#[no_mangle]
pub unsafe extern "C" fn FS_ListFilteredFiles(mut path: *const libc::c_char,
                                              mut extension:
                                                  *const libc::c_char,
                                              mut filter: *mut libc::c_char,
                                              mut numfiles: *mut libc::c_int,
                                              mut allowNonPureFilesOnDisk:
                                                  qboolean)
 -> *mut *mut libc::c_char {
    let mut nfiles: libc::c_int = 0;
    let mut listCopy: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut list: [*mut libc::c_char; 4096] = [0 as *mut libc::c_char; 4096];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut i: libc::c_int = 0;
    let mut pathLength: libc::c_int = 0;
    let mut extensionLength: libc::c_int = 0;
    let mut length: libc::c_int = 0;
    let mut pathDepth: libc::c_int = 0;
    let mut temp: libc::c_int = 0;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut buildBuffer: *mut fileInPack_t = 0 as *mut fileInPack_t;
    let mut zpath: [libc::c_char; 256] = [0; 256];
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if path.is_null() { *numfiles = 0i32; return 0 as *mut *mut libc::c_char }
    if extension.is_null() {
        extension = b"\x00" as *const u8 as *const libc::c_char
    }
    pathLength = strlen(path) as libc::c_int;
    if *path.offset((pathLength - 1i32) as isize) as libc::c_int ==
           '\\' as i32 ||
           *path.offset((pathLength - 1i32) as isize) as libc::c_int ==
               '/' as i32 {
        pathLength -= 1
    }
    extensionLength = strlen(extension) as libc::c_int;
    nfiles = 0i32;
    FS_ReturnPath(path, zpath.as_mut_ptr(), &mut pathDepth);
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file?
        if !(*search).pack.is_null() {
            //ZOID:  If we are pure, don't search for files on paks that
			// aren't on the pure list
            if !(0 == FS_PakIsPure((*search).pack) as u64) {
                pak = (*search).pack;
                buildBuffer = (*pak).buildBuffer;
                i = 0i32;
                while i < (*pak).numfiles {
                    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                    let mut zpathLen: libc::c_int = 0;
                    let mut depth: libc::c_int = 0;
                    name = (*buildBuffer.offset(i as isize)).name;
                    //
                    if !filter.is_null() {
                        // case insensitive
                        if !(0 ==
                                 Com_FilterPath(filter, name,
                                                qfalse as libc::c_int)) {
                            nfiles =
                                FS_AddFileToList(name, list.as_mut_ptr(),
                                                 nfiles)
                        }
                    } else {
                        zpathLen =
                            FS_ReturnPath(name, zpath.as_mut_ptr(),
                                          &mut depth);
                        if !(depth - pathDepth > 2i32 || pathLength > zpathLen
                                 || 0 != Q_stricmpn(name, path, pathLength)) {
                            length = strlen(name) as libc::c_int;
                            if !(length < extensionLength) {
                                if !(0 !=
                                         Q_stricmp(name.offset(length as
                                                                   isize).offset(-(extensionLength
                                                                                       as
                                                                                       isize)),
                                                   extension)) {
                                    temp = pathLength;
                                    if 0 != pathLength { temp += 1 }
                                    nfiles =
                                        FS_AddFileToList(name.offset(temp as
                                                                         isize),
                                                         list.as_mut_ptr(),
                                                         nfiles)
                                }
                            }
                        }
                    }
                    i += 1
                }
            }
        } else if !(*search).dir.is_null() {
            // scan for files in the filesystem
            let mut netpath: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut numSysFiles: libc::c_int = 0;
            let mut sysFiles: *mut *mut libc::c_char =
                0 as *mut *mut libc::c_char;
            let mut name_0: *mut libc::c_char = 0 as *mut libc::c_char;
            // don't scan directories for files if we are pure or restricted
            if !(0 != fs_numServerPaks && 0 == allowNonPureFilesOnDisk as u64)
               {
                netpath =
                    FS_BuildOSPath((*(*search).dir).path.as_mut_ptr(),
                                   (*(*search).dir).gamedir.as_mut_ptr(),
                                   path);
                sysFiles =
                    Sys_ListFiles(netpath, extension, filter,
                                  &mut numSysFiles, qfalse);
                i = 0i32;
                while i < numSysFiles {
                    name_0 = *sysFiles.offset(i as isize);
                    nfiles =
                        FS_AddFileToList(name_0, list.as_mut_ptr(), nfiles);
                    i += 1
                }
                Sys_FreeFileList(sysFiles);
            }
        }
        search = (*search).next
    }
    *numfiles = nfiles;
    if 0 == nfiles { return 0 as *mut *mut libc::c_char }
    listCopy =
        Z_MallocDebug(((nfiles + 1i32) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                           as libc::c_ulong)
                          as libc::c_int,
                      b"( nfiles + 1 ) * sizeof( *listCopy )\x00" as *const u8
                          as *const libc::c_char as *mut libc::c_char,
                      b"code/qcommon/files.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 2333i32)
            as *mut *mut libc::c_char;
    i = 0i32;
    while i < nfiles {
        let ref mut fresh7 = *listCopy.offset(i as isize);
        *fresh7 = list[i as usize];
        i += 1
    }
    let ref mut fresh8 = *listCopy.offset(i as isize);
    *fresh8 = 0 as *mut libc::c_char;
    return listCopy;
}
/*
==================
FS_AddFileToList
==================
*/
unsafe extern "C" fn FS_AddFileToList(mut name: *mut libc::c_char,
                                      mut list: *mut *mut libc::c_char,
                                      mut nfiles: libc::c_int)
 -> libc::c_int {
    let mut i: libc::c_int = 0;
    if nfiles == 0x1000i32 - 1i32 { return nfiles }
    i = 0i32;
    while i < nfiles {
        if 0 == Q_stricmp(name, *list.offset(i as isize)) { return nfiles }
        i += 1
    }
    let ref mut fresh9 = *list.offset(nfiles as isize);
    *fresh9 = CopyString(name);
    nfiles += 1;
    return nfiles;
}
/*
=================================================================================

DIRECTORY SCANNING FUNCTIONS

=================================================================================
*/
unsafe extern "C" fn FS_ReturnPath(mut zname: *const libc::c_char,
                                   mut zpath: *mut libc::c_char,
                                   mut depth: *mut libc::c_int)
 -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut at: libc::c_int = 0;
    let mut newdep: libc::c_int = 0;
    newdep = 0i32;
    *zpath.offset(0isize) = 0i32 as libc::c_char;
    len = 0i32;
    at = 0i32;
    while *zname.offset(at as isize) as libc::c_int != 0i32 {
        if *zname.offset(at as isize) as libc::c_int == '/' as i32 ||
               *zname.offset(at as isize) as libc::c_int == '\\' as i32 {
            len = at;
            newdep += 1
        }
        at += 1
    }
    strcpy(zpath, zname);
    *zpath.offset(len as isize) = 0i32 as libc::c_char;
    *depth = newdep;
    return len;
}
//============================================================================
/*
================
FS_Dir_f
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_Dir_f() {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extension: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ndirs: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if Cmd_Argc() < 2i32 || Cmd_Argc() > 3i32 {
        Com_Printf(b"usage: dir <directory> [extension]\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if Cmd_Argc() == 2i32 {
        path = Cmd_Argv(1i32);
        extension =
            b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else { path = Cmd_Argv(1i32); extension = Cmd_Argv(2i32) }
    Com_Printf(b"Directory of %s %s\n\x00" as *const u8 as
                   *const libc::c_char, path, extension);
    Com_Printf(b"---------------\n\x00" as *const u8 as *const libc::c_char);
    dirnames = FS_ListFiles(path, extension, &mut ndirs);
    i = 0i32;
    while i < ndirs {
        Com_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                   *dirnames.offset(i as isize));
        i += 1
    }
    FS_FreeFileList(dirnames);
}
#[no_mangle]
pub unsafe extern "C" fn FS_ListFiles(mut path: *const libc::c_char,
                                      mut extension: *const libc::c_char,
                                      mut numfiles: *mut libc::c_int)
 -> *mut *mut libc::c_char {
    return FS_ListFilteredFiles(path, extension, 0 as *mut libc::c_char,
                                numfiles, qfalse);
}
static mut fs_homepath: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
// shutdown and restart the filesystem so changes to fs_gamedir can take effect
#[no_mangle]
pub unsafe extern "C" fn FS_AddGameDirectory(mut path: *const libc::c_char,
                                             mut dir: *const libc::c_char) {
    let mut sp: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut curpath: [libc::c_char; 4097] = [0; 4097];
    let mut pakfile: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut numfiles: libc::c_int = 0;
    let mut pakfiles: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pakfilesi: libc::c_int = 0;
    let mut pakfilestmp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut numdirs: libc::c_int = 0;
    let mut pakdirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pakdirsi: libc::c_int = 0;
    let mut pakdirstmp: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pakwhich: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    sp = fs_searchpaths;
    while !sp.is_null() {
        if !(*sp).dir.is_null() &&
               0 == Q_stricmp((*(*sp).dir).path.as_mut_ptr(), path) &&
               0 == Q_stricmp((*(*sp).dir).gamedir.as_mut_ptr(), dir) {
            return
        }
        sp = (*sp).next
    }
    Q_strncpyz(fs_gamedir.as_mut_ptr(), dir,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz(curpath.as_mut_ptr(),
               FS_BuildOSPath(path, dir,
                              b"\x00" as *const u8 as *const libc::c_char),
               ::std::mem::size_of::<[libc::c_char; 4097]>() as libc::c_ulong
                   as libc::c_int);
    curpath[strlen(curpath.as_mut_ptr()).wrapping_sub(1i32 as libc::c_ulong)
                as usize] = '\u{0}' as i32 as libc::c_char;
    pakfiles =
        Sys_ListFiles(curpath.as_mut_ptr(),
                      b".pk3\x00" as *const u8 as *const libc::c_char,
                      0 as *mut libc::c_char, &mut numfiles, qfalse);
    qsort(pakfiles as *mut libc::c_void, numfiles as size_t,
          ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
          Some(paksort));
    if 0 != fs_numServerPaks {
        numdirs = 0i32;
        pakdirs = 0 as *mut *mut libc::c_char
    } else {
        pakdirs =
            Sys_ListFiles(curpath.as_mut_ptr(),
                          b"/\x00" as *const u8 as *const libc::c_char,
                          0 as *mut libc::c_char, &mut numdirs, qfalse);
        qsort(pakdirs as *mut libc::c_void, numdirs as size_t,
              ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
              Some(paksort));
    }
    pakfilesi = 0i32;
    pakdirsi = 0i32;
    while pakfilesi < numfiles || pakdirsi < numdirs {
        if pakfilesi >= numfiles {
            pakwhich = 0i32
        } else if pakdirsi >= numdirs {
            pakwhich = 1i32
        } else {
            pakfilestmp =
                &mut *pakfiles.offset(pakfilesi as isize) as
                    *mut *mut libc::c_char;
            pakdirstmp =
                &mut *pakdirs.offset(pakdirsi as isize) as
                    *mut *mut libc::c_char;
            pakwhich =
                (paksort(pakfilestmp as *const libc::c_void,
                         pakdirstmp as *const libc::c_void) < 0i32) as
                    libc::c_int
        }
        if 0 != pakwhich {
            pakfile =
                FS_BuildOSPath(path, dir,
                               *pakfiles.offset(pakfilesi as isize));
            pak =
                FS_LoadZipFile(pakfile, *pakfiles.offset(pakfilesi as isize));
            if pak.is_null() {
                pakfilesi += 1
            } else {
                Q_strncpyz((*pak).pakPathname.as_mut_ptr(),
                           curpath.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 4096]>() as
                               libc::c_ulong as libc::c_int);
                Q_strncpyz((*pak).pakGamename.as_mut_ptr(), dir,
                           ::std::mem::size_of::<[libc::c_char; 4096]>() as
                               libc::c_ulong as libc::c_int);
                fs_packFiles += (*pak).numfiles;
                search =
                    Z_MallocDebug(::std::mem::size_of::<searchpath_t>() as
                                      libc::c_ulong as libc::c_int,
                                  b"sizeof(searchpath_t)\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                                  b"code/qcommon/files.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 2973i32) as
                        *mut searchpath_t;
                (*search).pack = pak;
                (*search).next = fs_searchpaths;
                fs_searchpaths = search;
                pakfilesi += 1
            }
        } else {
            len = strlen(*pakdirs.offset(pakdirsi as isize)) as libc::c_int;
            if 0 ==
                   FS_IsExt(*pakdirs.offset(pakdirsi as isize),
                            b".pk3dir\x00" as *const u8 as
                                *const libc::c_char, len) as u64 {
                pakdirsi += 1
            } else {
                pakfile =
                    FS_BuildOSPath(path, dir,
                                   *pakdirs.offset(pakdirsi as isize));
                search =
                    Z_MallocDebug(::std::mem::size_of::<searchpath_t>() as
                                      libc::c_ulong as libc::c_int,
                                  b"sizeof(searchpath_t)\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                                  b"code/qcommon/files.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 2993i32) as
                        *mut searchpath_t;
                (*search).dir =
                    Z_MallocDebug(::std::mem::size_of::<directory_t>() as
                                      libc::c_ulong as libc::c_int,
                                  b"sizeof(*search->dir)\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char,
                                  b"code/qcommon/files.c\x00" as *const u8 as
                                      *const libc::c_char as
                                      *mut libc::c_char, 2994i32) as
                        *mut directory_t;
                Q_strncpyz((*(*search).dir).path.as_mut_ptr(),
                           curpath.as_mut_ptr(),
                           ::std::mem::size_of::<[libc::c_char; 4096]>() as
                               libc::c_ulong as libc::c_int);
                Q_strncpyz((*(*search).dir).fullpath.as_mut_ptr(), pakfile,
                           ::std::mem::size_of::<[libc::c_char; 4096]>() as
                               libc::c_ulong as libc::c_int);
                Q_strncpyz((*(*search).dir).gamedir.as_mut_ptr(),
                           *pakdirs.offset(pakdirsi as isize),
                           ::std::mem::size_of::<[libc::c_char; 4096]>() as
                               libc::c_ulong as libc::c_int);
                (*search).next = fs_searchpaths;
                fs_searchpaths = search;
                pakdirsi += 1
            }
        }
    }
    Sys_FreeFileList(pakfiles);
    Sys_FreeFileList(pakdirs);
    search =
        Z_MallocDebug(::std::mem::size_of::<searchpath_t>() as libc::c_ulong
                          as libc::c_int,
                      b"sizeof(searchpath_t)\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      b"code/qcommon/files.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 3014i32)
            as *mut searchpath_t;
    (*search).dir =
        Z_MallocDebug(::std::mem::size_of::<directory_t>() as libc::c_ulong as
                          libc::c_int,
                      b"sizeof( *search->dir )\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      b"code/qcommon/files.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 3015i32)
            as *mut directory_t;
    Q_strncpyz((*(*search).dir).path.as_mut_ptr(), path,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz((*(*search).dir).fullpath.as_mut_ptr(), curpath.as_mut_ptr(),
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz((*(*search).dir).gamedir.as_mut_ptr(), dir,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    (*search).next = fs_searchpaths;
    fs_searchpaths = search;
}
/*
==========================================================================

ZIP FILE LOADING

==========================================================================
*/
/*
=================
FS_LoadZipFile

Creates a new pak_t in the search chain for the contents
of a zip file.
=================
*/
unsafe extern "C" fn FS_LoadZipFile(mut zipfile: *const libc::c_char,
                                    mut basename: *const libc::c_char)
 -> *mut pack_t {
    let mut buildBuffer: *mut fileInPack_t = 0 as *mut fileInPack_t;
    let mut pack: *mut pack_t = 0 as *mut pack_t;
    let mut uf: unzFile = 0 as *mut libc::c_void;
    let mut err: libc::c_int = 0;
    let mut gi: unz_global_info =
        unz_global_info_s{number_entry: 0, size_comment: 0,};
    let mut filename_inzip: [libc::c_char; 256] = [0; 256];
    let mut file_info: unz_file_info =
        unz_file_info_s{version: 0,
                        version_needed: 0,
                        flag: 0,
                        compression_method: 0,
                        dosDate: 0,
                        crc: 0,
                        compressed_size: 0,
                        uncompressed_size: 0,
                        size_filename: 0,
                        size_file_extra: 0,
                        size_file_comment: 0,
                        disk_num_start: 0,
                        internal_fa: 0,
                        external_fa: 0,
                        tmu_date:
                            tm_unz_s{tm_sec: 0,
                                     tm_min: 0,
                                     tm_hour: 0,
                                     tm_mday: 0,
                                     tm_mon: 0,
                                     tm_year: 0,},};
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut hash: libc::c_long = 0;
    let mut fs_numHeaderLongs: libc::c_int = 0;
    let mut fs_headerLongs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut namePtr: *mut libc::c_char = 0 as *mut libc::c_char;
    fs_numHeaderLongs = 0i32;
    uf = unzOpen(zipfile);
    err = unzGetGlobalInfo(uf, &mut gi);
    if err != 0i32 { return 0 as *mut pack_t }
    len = 0i32;
    unzGoToFirstFile(uf);
    i = 0i32;
    while (i as libc::c_ulong) < gi.number_entry {
        err =
            unzGetCurrentFileInfo(uf, &mut file_info,
                                  filename_inzip.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 256]>()
                                      as libc::c_ulong,
                                  0 as *mut libc::c_void, 0i32 as uLong,
                                  0 as *mut libc::c_char, 0i32 as uLong);
        if err != 0i32 { break ; }
        len =
            (len as
                 libc::c_ulong).wrapping_add(strlen(filename_inzip.as_mut_ptr()).wrapping_add(1i32
                                                                                                  as
                                                                                                  libc::c_ulong))
                as libc::c_int as libc::c_int;
        unzGoToNextFile(uf);
        i += 1
    }
    buildBuffer =
        Z_MallocDebug(gi.number_entry.wrapping_mul(::std::mem::size_of::<fileInPack_t>()
                                                       as
                                                       libc::c_ulong).wrapping_add(len
                                                                                       as
                                                                                       libc::c_ulong)
                          as libc::c_int,
                      b"(gi.number_entry * sizeof( fileInPack_t )) + len\x00"
                          as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/files.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 2043i32)
            as *mut fileInPack_t;
    namePtr =
        (buildBuffer as
             *mut libc::c_char).offset(gi.number_entry.wrapping_mul(::std::mem::size_of::<fileInPack_t>()
                                                                        as
                                                                        libc::c_ulong)
                                           as isize);
    fs_headerLongs =
        Z_MallocDebug(gi.number_entry.wrapping_add(1i32 as
                                                       libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                                       as
                                                                                       libc::c_ulong)
                          as libc::c_int,
                      b"( gi.number_entry + 1 ) * sizeof(int)\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/files.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 2045i32)
            as *mut libc::c_int;
    let fresh10 = fs_numHeaderLongs;
    fs_numHeaderLongs = fs_numHeaderLongs + 1;
    *fs_headerLongs.offset(fresh10 as isize) = fs_checksumFeed;
    i = 1i32;
    while i <= 1024i32 {
        if i as libc::c_ulong > gi.number_entry { break ; }
        i <<= 1i32
    }
    pack =
        Z_MallocDebug((::std::mem::size_of::<pack_t>() as
                           libc::c_ulong).wrapping_add((i as
                                                            libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut fileInPack_t>()
                                                                                            as
                                                                                            libc::c_ulong))
                          as libc::c_int,
                      b"sizeof( pack_t ) + i * sizeof(fileInPack_t *)\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/files.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 2056i32)
            as *mut pack_t;
    (*pack).hashSize = i;
    (*pack).hashTable =
        (pack as
             *mut libc::c_char).offset(::std::mem::size_of::<pack_t>() as
                                           libc::c_ulong as isize) as
            *mut *mut fileInPack_t;
    i = 0i32;
    while i < (*pack).hashSize {
        let ref mut fresh11 = *(*pack).hashTable.offset(i as isize);
        *fresh11 = 0 as *mut fileInPack_t;
        i += 1
    }
    Q_strncpyz((*pack).pakFilename.as_mut_ptr(), zipfile,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz((*pack).pakBasename.as_mut_ptr(), basename,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    if strlen((*pack).pakBasename.as_mut_ptr()) > 4i32 as libc::c_ulong &&
           0 ==
               Q_stricmp((*pack).pakBasename.as_mut_ptr().offset(strlen((*pack).pakBasename.as_mut_ptr())
                                                                     as
                                                                     isize).offset(-4isize),
                         b".pk3\x00" as *const u8 as *const libc::c_char) {
        (*pack).pakBasename[strlen((*pack).pakBasename.as_mut_ptr()).wrapping_sub(4i32
                                                                                      as
                                                                                      libc::c_ulong)
                                as usize] = 0i32 as libc::c_char
    }
    (*pack).handle = uf;
    (*pack).numfiles = gi.number_entry as libc::c_int;
    unzGoToFirstFile(uf);
    i = 0i32;
    while (i as libc::c_ulong) < gi.number_entry {
        err =
            unzGetCurrentFileInfo(uf, &mut file_info,
                                  filename_inzip.as_mut_ptr(),
                                  ::std::mem::size_of::<[libc::c_char; 256]>()
                                      as libc::c_ulong,
                                  0 as *mut libc::c_void, 0i32 as uLong,
                                  0 as *mut libc::c_char, 0i32 as uLong);
        if err != 0i32 { break ; }
        if file_info.uncompressed_size > 0i32 as libc::c_ulong {
            let fresh12 = fs_numHeaderLongs;
            fs_numHeaderLongs = fs_numHeaderLongs + 1;
            *fs_headerLongs.offset(fresh12 as isize) =
                file_info.crc as libc::c_int
        }
        Q_strlwr(filename_inzip.as_mut_ptr());
        hash = FS_HashFileName(filename_inzip.as_mut_ptr(), (*pack).hashSize);
        let ref mut fresh13 = (*buildBuffer.offset(i as isize)).name;
        *fresh13 = namePtr;
        strcpy((*buildBuffer.offset(i as isize)).name,
               filename_inzip.as_mut_ptr());
        namePtr =
            namePtr.offset(strlen(filename_inzip.as_mut_ptr()).wrapping_add(1i32
                                                                                as
                                                                                libc::c_ulong)
                               as isize);
        (*buildBuffer.offset(i as isize)).pos = unzGetOffset(uf);
        (*buildBuffer.offset(i as isize)).len = file_info.uncompressed_size;
        let ref mut fresh14 = (*buildBuffer.offset(i as isize)).next;
        *fresh14 = *(*pack).hashTable.offset(hash as isize);
        let ref mut fresh15 = *(*pack).hashTable.offset(hash as isize);
        *fresh15 = &mut *buildBuffer.offset(i as isize) as *mut fileInPack_t;
        unzGoToNextFile(uf);
        i += 1
    }
    (*pack).checksum =
        Com_BlockChecksum(&mut *fs_headerLongs.offset(1isize) as
                              *mut libc::c_int as *const libc::c_void,
                          (::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong).wrapping_mul((fs_numHeaderLongs
                                                                - 1i32) as
                                                               libc::c_ulong)
                              as libc::c_int) as libc::c_int;
    (*pack).pure_checksum =
        Com_BlockChecksum(fs_headerLongs as *const libc::c_void,
                          (::std::mem::size_of::<libc::c_int>() as
                               libc::c_ulong).wrapping_mul(fs_numHeaderLongs
                                                               as
                                                               libc::c_ulong)
                              as libc::c_int) as libc::c_int;
    (*pack).checksum = (*pack).checksum;
    (*pack).pure_checksum = (*pack).pure_checksum;
    Z_Free(fs_headerLongs as *mut libc::c_void);
    (*pack).buildBuffer = buildBuffer;
    return pack;
}
static mut fs_checksumFeed: libc::c_int = 0;
//===========================================================================
unsafe extern "C" fn paksort(mut a: *const libc::c_void,
                             mut b: *const libc::c_void) -> libc::c_int {
    let mut aa: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bb: *mut libc::c_char = 0 as *mut libc::c_char;
    aa = *(a as *mut *mut libc::c_char);
    bb = *(b as *mut *mut libc::c_char);
    return FS_PathCmp(aa, bb);
}
static mut fs_steampath: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut fs_gogpath: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn FS_CreatePath(mut OSPath: *mut libc::c_char)
 -> qboolean {
    let mut ofs: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: [libc::c_char; 4096] = [0; 4096];
    if !strstr(OSPath,
               b"..\x00" as *const u8 as *const libc::c_char).is_null() ||
           !strstr(OSPath,
                   b"::\x00" as *const u8 as *const libc::c_char).is_null() {
        Com_Printf(b"WARNING: refusing to create relative path \"%s\"\n\x00"
                       as *const u8 as *const libc::c_char, OSPath);
        return qtrue
    }
    Q_strncpyz(path.as_mut_ptr(), OSPath,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    FS_ReplaceSeparators(path.as_mut_ptr());
    ofs = strchr(path.as_mut_ptr(), '/' as i32);
    if !ofs.is_null() { ofs = ofs.offset(1isize) }
    while !ofs.is_null() && 0 != *ofs as libc::c_int {
        if *ofs as libc::c_int == '/' as i32 {
            *ofs = 0i32 as libc::c_char;
            if 0 == Sys_Mkdir(path.as_mut_ptr()) as u64 {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"FS_CreatePath: failed to create path \"%s\"\x00"
                              as *const u8 as *const libc::c_char,
                          path.as_mut_ptr());
            }
            *ofs = '/' as i32 as libc::c_char
        }
        ofs = ofs.offset(1isize)
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn FS_InvalidGameDir(mut gamedir: *const libc::c_char)
 -> qboolean {
    if 0 == strcmp(gamedir, b".\x00" as *const u8 as *const libc::c_char) ||
           0 == strcmp(gamedir, b"..\x00" as *const u8 as *const libc::c_char)
           || !strchr(gamedir, '/' as i32).is_null() ||
           !strchr(gamedir, '\\' as i32).is_null() {
        return qtrue
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn FS_Shutdown(mut closemfp: qboolean) {
    let mut p: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut next: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 64i32 {
        if 0 != fsh[i as usize].fileSize { FS_FCloseFile(i); }
        i += 1
    }
    p = fs_searchpaths;
    while !p.is_null() {
        next = (*p).next;
        if !(*p).pack.is_null() { FS_FreePak((*p).pack); }
        if !(*p).dir.is_null() { Z_Free((*p).dir as *mut libc::c_void); }
        Z_Free(p as *mut libc::c_void);
        p = next
    }
    fs_searchpaths = 0 as *mut searchpath_t;
    Cmd_RemoveCommand(b"path\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"dir\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"fdir\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"touchFile\x00" as *const u8 as *const libc::c_char);
    Cmd_RemoveCommand(b"which\x00" as *const u8 as *const libc::c_char);
}
/*
=================
FS_FreePak

Frees a pak structure and releases all associated resources
=================
*/
unsafe extern "C" fn FS_FreePak(mut thepak: *mut pack_t) {
    unzClose((*thepak).handle);
    Z_Free((*thepak).buildBuffer as *mut libc::c_void);
    Z_Free(thepak as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn FS_ConditionalRestart(mut checksumFeed: libc::c_int,
                                               mut disconnect: qboolean)
 -> qboolean {
    if 0 != (*fs_gamedirvar).modified as u64 {
        if 0 !=
               FS_FilenameCompare(lastValidGame.as_mut_ptr(),
                                  (*fs_gamedirvar).string) as libc::c_uint &&
               (0 != *lastValidGame.as_mut_ptr() as libc::c_int ||
                    0 !=
                        FS_FilenameCompare((*fs_gamedirvar).string,
                                           (*com_basegame).string) as
                            libc::c_uint) &&
               (0 != *(*fs_gamedirvar).string as libc::c_int ||
                    0 !=
                        FS_FilenameCompare(lastValidGame.as_mut_ptr(),
                                           (*com_basegame).string) as
                            libc::c_uint) {
            Com_GameRestart(checksumFeed, disconnect);
            return qtrue
        } else { (*fs_gamedirvar).modified = qfalse }
    }
    if checksumFeed != fs_checksumFeed {
        FS_Restart(checksumFeed);
    } else if 0 != fs_numServerPaks && 0 == fs_reordered as u64 {
        FS_ReorderPurePaks();
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn FS_Restart(mut checksumFeed: libc::c_int) {
    let mut lastGameDir: *const libc::c_char = 0 as *const libc::c_char;
    FS_Shutdown(qfalse);
    fs_checksumFeed = checksumFeed;
    FS_ClearPakReferences(0i32);
    FS_Startup((*com_basegame).string);
    FS_CheckPak0();
    if FS_ReadFile(b"default.cfg\x00" as *const u8 as *const libc::c_char,
                   0 as *mut *mut libc::c_void) <= 0i32 as libc::c_long {
        if 0 != lastValidBase[0usize] {
            FS_PureServerSetLoadedPaks(b"\x00" as *const u8 as
                                           *const libc::c_char,
                                       b"\x00" as *const u8 as
                                           *const libc::c_char);
            Cvar_Set(b"fs_basepath\x00" as *const u8 as *const libc::c_char,
                     lastValidBase.as_mut_ptr());
            Cvar_Set(b"com_basegame\x00" as *const u8 as *const libc::c_char,
                     lastValidComBaseGame.as_mut_ptr());
            Cvar_Set(b"fs_basegame\x00" as *const u8 as *const libc::c_char,
                     lastValidFsBaseGame.as_mut_ptr());
            Cvar_Set(b"fs_game\x00" as *const u8 as *const libc::c_char,
                     lastValidGame.as_mut_ptr());
            lastValidBase[0usize] = '\u{0}' as i32 as libc::c_char;
            lastValidComBaseGame[0usize] = '\u{0}' as i32 as libc::c_char;
            lastValidFsBaseGame[0usize] = '\u{0}' as i32 as libc::c_char;
            lastValidGame[0usize] = '\u{0}' as i32 as libc::c_char;
            FS_Restart(checksumFeed);
            Com_Error(ERR_DROP as libc::c_int,
                      b"Invalid game folder\x00" as *const u8 as
                          *const libc::c_char);
        }
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Couldn\'t load default.cfg\x00" as *const u8 as
                      *const libc::c_char);
    }
    lastGameDir =
        if 0 != lastValidGame[0usize] as libc::c_int {
            lastValidGame.as_mut_ptr()
        } else { lastValidComBaseGame.as_mut_ptr() };
    if 0 != Q_stricmp(FS_GetCurrentGameDir(), lastGameDir) {
        Sys_RemovePIDFile(lastGameDir);
        Sys_InitPIDFile(FS_GetCurrentGameDir());
        if 0 == Com_SafeMode() as u64 {
            Cbuf_AddText(b"exec q3config.cfg\n\x00" as *const u8 as
                             *const libc::c_char);
        }
    }
    Q_strncpyz(lastValidBase.as_mut_ptr(), (*fs_basepath).string,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz(lastValidComBaseGame.as_mut_ptr(), (*com_basegame).string,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz(lastValidFsBaseGame.as_mut_ptr(), (*fs_basegame).string,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz(lastValidGame.as_mut_ptr(), (*fs_gamedirvar).string,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn FS_GetCurrentGameDir() -> *const libc::c_char {
    if 0 != *(*fs_gamedirvar).string.offset(0isize) {
        return (*fs_gamedirvar).string
    }
    return (*com_basegame).string;
}
#[no_mangle]
pub unsafe extern "C" fn FS_PureServerSetLoadedPaks(mut pakSums:
                                                        *const libc::c_char,
                                                    mut pakNames:
                                                        *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    Cmd_TokenizeString(pakSums);
    c = Cmd_Argc();
    if c > 4096i32 { c = 4096i32 }
    fs_numServerPaks = c;
    i = 0i32;
    while i < c { fs_serverPaks[i as usize] = atoi(Cmd_Argv(i)); i += 1 }
    if 0 != fs_numServerPaks {
        Com_DPrintf(b"Connected to a pure server.\n\x00" as *const u8 as
                        *const libc::c_char);
    } else if 0 != fs_reordered as u64 {
        Com_DPrintf(b"FS search reorder is required\n\x00" as *const u8 as
                        *const libc::c_char);
        FS_Restart(fs_checksumFeed);
        return
    }
    i = 0i32;
    while i < c {
        if !fs_serverPakNames[i as usize].is_null() {
            Z_Free(fs_serverPakNames[i as usize] as *mut libc::c_void);
        }
        fs_serverPakNames[i as usize] = 0 as *mut libc::c_char;
        i += 1
    }
    if !pakNames.is_null() && 0 != *pakNames as libc::c_int {
        Cmd_TokenizeString(pakNames);
        d = Cmd_Argc();
        if d > 4096i32 { d = 4096i32 }
        i = 0i32;
        while i < d {
            fs_serverPakNames[i as usize] = CopyString(Cmd_Argv(i));
            i += 1
        }
    };
}
// pk3 names
static mut fs_serverPakNames: [*mut libc::c_char; 4096] =
    [0 as *const libc::c_char as *mut libc::c_char; 4096];
// Returns a space separated string containing the checksums of all loaded 
// AND referenced pk3 files. Servers with sv_pure set will get this string 
// back from clients for pure validation 
#[no_mangle]
pub unsafe extern "C" fn FS_ClearPakReferences(mut flags: libc::c_int) {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    if 0 == flags { flags = -1i32 }
    search = fs_searchpaths;
    while !search.is_null() {
        if !(*search).pack.is_null() {
            (*(*search).pack).referenced &= !flags
        }
        search = (*search).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn FS_FileExists(mut file: *const libc::c_char)
 -> qboolean {
    return FS_FileInPathExists(FS_BuildOSPath((*fs_homepath).string,
                                              fs_gamedir.as_mut_ptr(), file));
}
/*
================
FS_FileInPathExists

Tests if path and file exists
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_FileInPathExists(mut testpath:
                                                 *const libc::c_char)
 -> qboolean {
    let mut filep: *mut FILE = 0 as *mut FILE;
    filep =
        Sys_FOpen(testpath, b"rb\x00" as *const u8 as *const libc::c_char);
    if !filep.is_null() { fclose(filep); return qtrue }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn FS_FindVM(mut startSearch: *mut *mut libc::c_void,
                                   mut found: *mut libc::c_char,
                                   mut foundlen: libc::c_int,
                                   mut name: *const libc::c_char,
                                   mut enableDll: libc::c_int)
 -> libc::c_int {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut lastSearch: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut dir: *mut directory_t = 0 as *mut directory_t;
    let mut pack: *mut pack_t = 0 as *mut pack_t;
    let mut dllName: [libc::c_char; 4096] = [0; 4096];
    let mut qvmName: [libc::c_char; 4096] = [0; 4096];
    let mut netpath: *mut libc::c_char = 0 as *mut libc::c_char;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if 0 != enableDll {
        Com_sprintf(dllName.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 4096]>() as
                        libc::c_ulong as libc::c_int,
                    b"%sx86_64.so\x00" as *const u8 as *const libc::c_char,
                    name);
    }
    Com_sprintf(qvmName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"vm/%s.qvm\x00" as *const u8 as *const libc::c_char, name);
    lastSearch = *startSearch as *mut searchpath_t;
    if (*startSearch).is_null() {
        search = fs_searchpaths
    } else { search = (*lastSearch).next }
    while !search.is_null() {
        if !(*search).dir.is_null() && 0 == fs_numServerPaks {
            dir = (*search).dir;
            if 0 != enableDll {
                netpath =
                    FS_BuildOSPath((*dir).path.as_mut_ptr(),
                                   (*dir).gamedir.as_mut_ptr(),
                                   dllName.as_mut_ptr());
                if 0 != FS_FileInPathExists(netpath) as u64 {
                    Q_strncpyz(found, netpath, foundlen);
                    *startSearch = search as *mut libc::c_void;
                    return VMI_NATIVE as libc::c_int
                }
            }
            if FS_FOpenFileReadDir(qvmName.as_mut_ptr(), search,
                                   0 as *mut fileHandle_t, qfalse, qfalse) >
                   0i32 as libc::c_long {
                *startSearch = search as *mut libc::c_void;
                return VMI_COMPILED as libc::c_int
            }
        } else if !(*search).pack.is_null() {
            pack = (*search).pack;
            if !lastSearch.is_null() && !(*lastSearch).pack.is_null() {
                // make sure we only try loading one VM file per game dir
				// i.e. if VM from pak7.pk3 fails we won't try one from pak6.pk3
                if 0 ==
                       FS_FilenameCompare((*(*lastSearch).pack).pakPathname.as_mut_ptr(),
                                          (*pack).pakPathname.as_mut_ptr()) as
                           u64 {
                    search = (*search).next;
                    continue ;
                }
            }
            if FS_FOpenFileReadDir(qvmName.as_mut_ptr(), search,
                                   0 as *mut fileHandle_t, qfalse, qfalse) >
                   0i32 as libc::c_long {
                *startSearch = search as *mut libc::c_void;
                return VMI_COMPILED as libc::c_int
            }
        }
        search = (*search).next
    }
    return -1i32;
}
#[no_mangle]
pub unsafe extern "C" fn FS_CompareZipChecksum(mut zipfile:
                                                   *const libc::c_char)
 -> qboolean {
    let mut thepak: *mut pack_t = 0 as *mut pack_t;
    let mut index: libc::c_int = 0;
    let mut checksum: libc::c_int = 0;
    thepak =
        FS_LoadZipFile(zipfile, b"\x00" as *const u8 as *const libc::c_char);
    if thepak.is_null() { return qfalse }
    checksum = (*thepak).checksum;
    FS_FreePak(thepak);
    index = 0i32;
    while index < fs_numServerReferencedPaks {
        if checksum == fs_serverReferencedPaks[index as usize] {
            return qtrue
        }
        index += 1
    }
    return qfalse;
}
// checksums
static mut fs_serverReferencedPaks: [libc::c_int; 4096] = [0; 4096];
// only used for autodownload, to make sure the client has at least
// all the pk3 files that are referenced at the server side
static mut fs_numServerReferencedPaks: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn FS_LoadStack() -> libc::c_int {
    return fs_loadStack;
}
#[no_mangle]
pub unsafe extern "C" fn FS_GetFileList(mut path: *const libc::c_char,
                                        mut extension: *const libc::c_char,
                                        mut listbuf: *mut libc::c_char,
                                        mut bufsize: libc::c_int)
 -> libc::c_int {
    let mut nFiles: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut nTotal: libc::c_int = 0;
    let mut nLen: libc::c_int = 0;
    let mut pFiles: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    *listbuf = 0i32 as libc::c_char;
    nFiles = 0i32;
    nTotal = 0i32;
    if Q_stricmp(path, b"$modlist\x00" as *const u8 as *const libc::c_char) ==
           0i32 {
        return FS_GetModList(listbuf, bufsize)
    }
    pFiles = FS_ListFiles(path, extension, &mut nFiles);
    i = 0i32;
    while i < nFiles {
        nLen =
            strlen(*pFiles.offset(i as
                                      isize)).wrapping_add(1i32 as
                                                               libc::c_ulong)
                as libc::c_int;
        if nTotal + nLen + 1i32 < bufsize {
            strcpy(listbuf, *pFiles.offset(i as isize));
            listbuf = listbuf.offset(nLen as isize);
            nTotal += nLen;
            i += 1
        } else { nFiles = i; break ; }
    }
    FS_FreeFileList(pFiles);
    return nFiles;
}
#[no_mangle]
pub unsafe extern "C" fn FS_GetModList(mut listbuf: *mut libc::c_char,
                                       mut bufsize: libc::c_int)
 -> libc::c_int {
    let mut nMods: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut nTotal: libc::c_int = 0;
    let mut nLen: libc::c_int = 0;
    let mut nPaks: libc::c_int = 0;
    let mut nDirs: libc::c_int = 0;
    let mut nPakDirs: libc::c_int = 0;
    let mut nPotential: libc::c_int = 0;
    let mut nDescLen: libc::c_int = 0;
    let mut pFiles: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pPaks: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut pDirs: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut description: [libc::c_char; 4096] = [0; 4096];
    let mut dummy: libc::c_int = 0;
    let mut pFiles0: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut bDrop: qboolean = qfalse;
    // paths to search for mods
    let paths: [*const libc::c_char; 4] =
        [(*fs_basepath).string as *const libc::c_char,
         (*fs_homepath).string as *const libc::c_char,
         (*fs_steampath).string as *const libc::c_char,
         (*fs_gogpath).string as *const libc::c_char];
    *listbuf = 0i32 as libc::c_char;
    nTotal = 0i32;
    nMods = nTotal;
    i = 0i32;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*const libc::c_char; 4]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                   as libc::c_ulong) {
        pFiles0 =
            Sys_ListFiles(paths[i as usize], 0 as *const libc::c_char,
                          0 as *mut libc::c_char, &mut dummy, qtrue);
        pFiles = Sys_ConcatenateFileLists(pFiles, pFiles0);
        i += 1
    }
    nPotential = Sys_CountFileList(pFiles) as libc::c_int;
    i = 0i32;
    while i < nPotential {
        name = *pFiles.offset(i as isize);
        if i != 0i32 {
            bDrop = qfalse;
            j = 0i32;
            while j < i {
                if Q_stricmp(*pFiles.offset(j as isize), name) == 0i32 {
                    bDrop = qtrue;
                    break ;
                } else { j += 1 }
            }
        }
        // we also drop "baseq3" "." and ".."
        if !(0 != bDrop as libc::c_uint ||
                 Q_stricmp(name, (*com_basegame).string) == 0i32 ||
                 Q_stricmpn(name,
                            b".\x00" as *const u8 as *const libc::c_char,
                            1i32) == 0i32) {
            j = 0i32;
            while (j as libc::c_ulong) <
                      (::std::mem::size_of::<[*const libc::c_char; 4]>() as
                           libc::c_ulong).wrapping_div(::std::mem::size_of::<*const libc::c_char>()
                                                           as libc::c_ulong) {
                path =
                    FS_BuildOSPath(paths[j as usize], name,
                                   b"\x00" as *const u8 as
                                       *const libc::c_char);
                nPakDirs = 0i32;
                nDirs = nPakDirs;
                nPaks = nDirs;
                pPaks =
                    Sys_ListFiles(path,
                                  b".pk3\x00" as *const u8 as
                                      *const libc::c_char,
                                  0 as *mut libc::c_char, &mut nPaks, qfalse);
                pDirs =
                    Sys_ListFiles(path,
                                  b"/\x00" as *const u8 as
                                      *const libc::c_char,
                                  0 as *mut libc::c_char, &mut nDirs, qfalse);
                k = 0i32;
                while k < nDirs {
                    if 0 !=
                           FS_IsExt(*pDirs.offset(k as isize),
                                    b".pk3dir\x00" as *const u8 as
                                        *const libc::c_char,
                                    strlen(*pDirs.offset(k as isize)) as
                                        libc::c_int) as u64 {
                        nPakDirs += 1
                    }
                    k += 1
                }
                Sys_FreeFileList(pPaks);
                Sys_FreeFileList(pDirs);
                if nPaks > 0i32 || nPakDirs > 0i32 { break ; }
                j += 1
            }
            if nPaks > 0i32 || nPakDirs > 0i32 {
                nLen =
                    strlen(name).wrapping_add(1i32 as libc::c_ulong) as
                        libc::c_int;
                FS_GetModDescription(name, description.as_mut_ptr(),
                                     ::std::mem::size_of::<[libc::c_char; 4096]>()
                                         as libc::c_ulong as libc::c_int);
                nDescLen =
                    strlen(description.as_mut_ptr()).wrapping_add(1i32 as
                                                                      libc::c_ulong)
                        as libc::c_int;
                if !(nTotal + nLen + 1i32 + nDescLen + 1i32 < bufsize) {
                    break ;
                }
                strcpy(listbuf, name);
                listbuf = listbuf.offset(nLen as isize);
                strcpy(listbuf, description.as_mut_ptr());
                listbuf = listbuf.offset(nDescLen as isize);
                nTotal += nLen + nDescLen;
                nMods += 1
            }
        }
        i += 1
    }
    Sys_FreeFileList(pFiles);
    return nMods;
}
#[no_mangle]
pub unsafe extern "C" fn FS_GetModDescription(mut modDir: *const libc::c_char,
                                              mut description:
                                                  *mut libc::c_char,
                                              mut descriptionLen:
                                                  libc::c_int) {
    let mut descHandle: fileHandle_t = 0;
    let mut descPath: [libc::c_char; 64] = [0; 64];
    let mut nDescLen: libc::c_int = 0;
    let mut file: *mut FILE = 0 as *mut FILE;
    Com_sprintf(descPath.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"%s%cdescription.txt\x00" as *const u8 as
                    *const libc::c_char, modDir, '/' as i32);
    nDescLen =
        FS_SV_FOpenFileRead(descPath.as_mut_ptr(), &mut descHandle) as
            libc::c_int;
    if nDescLen > 0i32 {
        file = FS_FileForHandle(descHandle);
        memset(description as *mut libc::c_void, 0i32,
               descriptionLen as libc::c_ulong);
        nDescLen =
            fread(description as *mut libc::c_void, 1i32 as libc::c_ulong,
                  descriptionLen as libc::c_ulong, file) as libc::c_int;
        if nDescLen >= 0i32 {
            *description.offset(nDescLen as isize) =
                '\u{0}' as i32 as libc::c_char
        }
    } else { Q_strncpyz(description, modDir, descriptionLen); }
    if 0 != descHandle { FS_FCloseFile(descHandle); };
}
#[no_mangle]
pub unsafe extern "C" fn FS_SV_FOpenFileRead(mut filename:
                                                 *const libc::c_char,
                                             mut fp: *mut fileHandle_t)
 -> libc::c_long {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: fileHandle_t = 0i32;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    f = FS_HandleForFile();
    fsh[f as usize].zipFile = qfalse;
    Q_strncpyz(fsh[f as usize].name.as_mut_ptr(), filename,
               ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                   as libc::c_int);
    S_ClearSoundBuffer();
    ospath =
        FS_BuildOSPath((*fs_homepath).string, filename,
                       b"\x00" as *const u8 as *const libc::c_char);
    *ospath.offset(strlen(ospath).wrapping_sub(1i32 as libc::c_ulong) as
                       isize) = '\u{0}' as i32 as libc::c_char;
    if 0 != (*fs_debug).integer {
        Com_Printf(b"FS_SV_FOpenFileRead (fs_homepath): %s\n\x00" as *const u8
                       as *const libc::c_char, ospath);
    }
    fsh[f as usize].handleFiles.file.o =
        Sys_FOpen(ospath, b"rb\x00" as *const u8 as *const libc::c_char);
    fsh[f as usize].handleSync = qfalse;
    if fsh[f as usize].handleFiles.file.o.is_null() {
        if 0 != Q_stricmp((*fs_homepath).string, (*fs_basepath).string) {
            ospath =
                FS_BuildOSPath((*fs_basepath).string, filename,
                               b"\x00" as *const u8 as *const libc::c_char);
            *ospath.offset(strlen(ospath).wrapping_sub(1i32 as libc::c_ulong)
                               as isize) = '\u{0}' as i32 as libc::c_char;
            if 0 != (*fs_debug).integer {
                Com_Printf(b"FS_SV_FOpenFileRead (fs_basepath): %s\n\x00" as
                               *const u8 as *const libc::c_char, ospath);
            }
            fsh[f as usize].handleFiles.file.o =
                Sys_FOpen(ospath,
                          b"rb\x00" as *const u8 as *const libc::c_char);
            fsh[f as usize].handleSync = qfalse
        }
        if fsh[f as usize].handleFiles.file.o.is_null() &&
               0 != *(*fs_steampath).string.offset(0isize) as libc::c_int {
            ospath =
                FS_BuildOSPath((*fs_steampath).string, filename,
                               b"\x00" as *const u8 as *const libc::c_char);
            *ospath.offset(strlen(ospath).wrapping_sub(1i32 as libc::c_ulong)
                               as isize) = '\u{0}' as i32 as libc::c_char;
            if 0 != (*fs_debug).integer {
                Com_Printf(b"FS_SV_FOpenFileRead (fs_steampath): %s\n\x00" as
                               *const u8 as *const libc::c_char, ospath);
            }
            fsh[f as usize].handleFiles.file.o =
                Sys_FOpen(ospath,
                          b"rb\x00" as *const u8 as *const libc::c_char);
            fsh[f as usize].handleSync = qfalse
        }
        if fsh[f as usize].handleFiles.file.o.is_null() &&
               0 != *(*fs_gogpath).string.offset(0isize) as libc::c_int {
            ospath =
                FS_BuildOSPath((*fs_gogpath).string, filename,
                               b"\x00" as *const u8 as *const libc::c_char);
            *ospath.offset(strlen(ospath).wrapping_sub(1i32 as libc::c_ulong)
                               as isize) = '\u{0}' as i32 as libc::c_char;
            if 0 != (*fs_debug).integer {
                Com_Printf(b"FS_SV_FOpenFileRead (fs_gogpath): %s\n\x00" as
                               *const u8 as *const libc::c_char, ospath);
            }
            fsh[f as usize].handleFiles.file.o =
                Sys_FOpen(ospath,
                          b"rb\x00" as *const u8 as *const libc::c_char);
            fsh[f as usize].handleSync = qfalse
        }
        if fsh[f as usize].handleFiles.file.o.is_null() { f = 0i32 }
    }
    *fp = f;
    if 0 != f { return FS_filelength(f) }
    return -1i32 as libc::c_long;
}
// writes a complete file, creating any subdirectories needed
#[no_mangle]
pub unsafe extern "C" fn FS_filelength(mut f: fileHandle_t) -> libc::c_long {
    let mut h: *mut FILE = 0 as *mut FILE;
    h = FS_FileForHandle(f);
    if h.is_null() {
        return -1i32 as libc::c_long
    } else { return FS_fplength(h) };
}
/*
=======================
Sys_ConcatenateFileLists

mkv: Naive implementation. Concatenates three lists into a
     new list, and frees the old lists from the heap.
bk001129 - from cvs1.17 (mkv)

FIXME TTimo those two should move to common.c next to Sys_ListFiles
=======================
 */
unsafe extern "C" fn Sys_CountFileList(mut list: *mut *mut libc::c_char)
 -> libc::c_uint {
    let mut i: libc::c_int = 0i32;
    if !list.is_null() {
        while !(*list).is_null() { list = list.offset(1isize); i += 1 }
    }
    return i as libc::c_uint;
}
unsafe extern "C" fn Sys_ConcatenateFileLists(mut list0:
                                                  *mut *mut libc::c_char,
                                              mut list1:
                                                  *mut *mut libc::c_char)
 -> *mut *mut libc::c_char {
    let mut totalLength: libc::c_int = 0i32;
    let mut cat: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut dst: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut src: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    totalLength =
        (totalLength as libc::c_uint).wrapping_add(Sys_CountFileList(list0))
            as libc::c_int as libc::c_int;
    totalLength =
        (totalLength as libc::c_uint).wrapping_add(Sys_CountFileList(list1))
            as libc::c_int as libc::c_int;
    cat =
        Z_MallocDebug(((totalLength + 1i32) as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut libc::c_char>()
                                                           as libc::c_ulong)
                          as libc::c_int,
                      b"( totalLength + 1 ) * sizeof( char* )\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/files.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 2447i32)
            as *mut *mut libc::c_char;
    dst = cat;
    if !list0.is_null() {
        src = list0;
        while !(*src).is_null() {
            *dst = *src;
            src = src.offset(1isize);
            dst = dst.offset(1isize)
        }
    }
    if !list1.is_null() {
        src = list1;
        while !(*src).is_null() {
            *dst = *src;
            src = src.offset(1isize);
            dst = dst.offset(1isize)
        }
    }
    *dst = 0 as *mut libc::c_char;
    if !list0.is_null() { Z_Free(list0 as *mut libc::c_void); }
    if !list1.is_null() { Z_Free(list1 as *mut libc::c_void); }
    return cat;
}
#[no_mangle]
pub unsafe extern "C" fn FS_FOpenFileWrite(mut filename: *const libc::c_char)
 -> fileHandle_t {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: fileHandle_t = 0;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    f = FS_HandleForFile();
    fsh[f as usize].zipFile = qfalse;
    ospath =
        FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(),
                       filename);
    if 0 != (*fs_debug).integer {
        Com_Printf(b"FS_FOpenFileWrite: %s\n\x00" as *const u8 as
                       *const libc::c_char, ospath);
    }
    FS_CheckFilenameIsMutable(ospath,
                              (*::std::mem::transmute::<&[u8; 18],
                                                        &[libc::c_char; 18]>(b"FS_FOpenFileWrite\x00")).as_ptr());
    if 0 != FS_CreatePath(ospath) as u64 { return 0i32 }
    fsh[f as usize].handleFiles.file.o =
        Sys_FOpen(ospath, b"wb\x00" as *const u8 as *const libc::c_char);
    Q_strncpyz(fsh[f as usize].name.as_mut_ptr(), filename,
               ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                   as libc::c_int);
    fsh[f as usize].handleSync = qfalse;
    if fsh[f as usize].handleFiles.file.o.is_null() { f = 0i32 }
    return f;
}
/*
=================
FS_CheckFilenameIsMutable

ERR_FATAL if trying to maniuplate a file with the platform library, QVM, or pk3 extension
=================
 */
unsafe extern "C" fn FS_CheckFilenameIsMutable(mut filename:
                                                   *const libc::c_char,
                                               mut function:
                                                   *const libc::c_char) {
    if 0 != Sys_DllExtension(filename) as libc::c_uint ||
           0 !=
               COM_CompareExtension(filename,
                                    b".qvm\x00" as *const u8 as
                                        *const libc::c_char) as libc::c_uint
           ||
           0 !=
               COM_CompareExtension(filename,
                                    b".pk3\x00" as *const u8 as
                                        *const libc::c_char) as libc::c_uint {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"%s: Not allowed to manipulate \'%s\' due to %s extension\x00"
                      as *const u8 as *const libc::c_char, function, filename,
                  COM_GetExtension(filename));
    };
}
#[no_mangle]
pub unsafe extern "C" fn FS_FOpenFileAppend(mut filename: *const libc::c_char)
 -> fileHandle_t {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: fileHandle_t = 0;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    f = FS_HandleForFile();
    fsh[f as usize].zipFile = qfalse;
    Q_strncpyz(fsh[f as usize].name.as_mut_ptr(), filename,
               ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                   as libc::c_int);
    S_ClearSoundBuffer();
    ospath =
        FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(),
                       filename);
    if 0 != (*fs_debug).integer {
        Com_Printf(b"FS_FOpenFileAppend: %s\n\x00" as *const u8 as
                       *const libc::c_char, ospath);
    }
    FS_CheckFilenameIsMutable(ospath,
                              (*::std::mem::transmute::<&[u8; 19],
                                                        &[libc::c_char; 19]>(b"FS_FOpenFileAppend\x00")).as_ptr());
    if 0 != FS_CreatePath(ospath) as u64 { return 0i32 }
    fsh[f as usize].handleFiles.file.o =
        Sys_FOpen(ospath, b"ab\x00" as *const u8 as *const libc::c_char);
    fsh[f as usize].handleSync = qfalse;
    if fsh[f as usize].handleFiles.file.o.is_null() { f = 0i32 }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn FS_FCreateOpenPipeFile(mut filename:
                                                    *const libc::c_char)
 -> fileHandle_t {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fifo: *mut FILE = 0 as *mut FILE;
    let mut f: fileHandle_t = 0;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    f = FS_HandleForFile();
    fsh[f as usize].zipFile = qfalse;
    Q_strncpyz(fsh[f as usize].name.as_mut_ptr(), filename,
               ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                   as libc::c_int);
    S_ClearSoundBuffer();
    ospath =
        FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(),
                       filename);
    if 0 != (*fs_debug).integer {
        Com_Printf(b"FS_FCreateOpenPipeFile: %s\n\x00" as *const u8 as
                       *const libc::c_char, ospath);
    }
    FS_CheckFilenameIsMutable(ospath,
                              (*::std::mem::transmute::<&[u8; 23],
                                                        &[libc::c_char; 23]>(b"FS_FCreateOpenPipeFile\x00")).as_ptr());
    fifo = Sys_Mkfifo(ospath);
    if !fifo.is_null() {
        fsh[f as usize].handleFiles.file.o = fifo;
        fsh[f as usize].handleSync = qfalse
    } else {
        Com_Printf(b"^3WARNING: Could not create new com_pipefile at %s. com_pipefile will not be used.\n\x00"
                       as *const u8 as *const libc::c_char, ospath);
        f = 0i32
    }
    return f;
}
// will properly create any needed paths and deal with seperater character issues
#[no_mangle]
pub unsafe extern "C" fn FS_SV_FOpenFileWrite(mut filename:
                                                  *const libc::c_char)
 -> fileHandle_t {
    let mut ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: fileHandle_t = 0;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    ospath =
        FS_BuildOSPath((*fs_homepath).string, filename,
                       b"\x00" as *const u8 as *const libc::c_char);
    *ospath.offset(strlen(ospath).wrapping_sub(1i32 as libc::c_ulong) as
                       isize) = '\u{0}' as i32 as libc::c_char;
    f = FS_HandleForFile();
    fsh[f as usize].zipFile = qfalse;
    if 0 != (*fs_debug).integer {
        Com_Printf(b"FS_SV_FOpenFileWrite: %s\n\x00" as *const u8 as
                       *const libc::c_char, ospath);
    }
    FS_CheckFilenameIsMutable(ospath,
                              (*::std::mem::transmute::<&[u8; 21],
                                                        &[libc::c_char; 21]>(b"FS_SV_FOpenFileWrite\x00")).as_ptr());
    if 0 != FS_CreatePath(ospath) as u64 { return 0i32 }
    Com_DPrintf(b"writing to: %s\n\x00" as *const u8 as *const libc::c_char,
                ospath);
    fsh[f as usize].handleFiles.file.o =
        Sys_FOpen(ospath, b"wb\x00" as *const u8 as *const libc::c_char);
    Q_strncpyz(fsh[f as usize].name.as_mut_ptr(), filename,
               ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong
                   as libc::c_int);
    fsh[f as usize].handleSync = qfalse;
    if fsh[f as usize].handleFiles.file.o.is_null() { f = 0i32 }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn FS_SV_Rename(mut from: *const libc::c_char,
                                      mut to: *const libc::c_char,
                                      mut safe: qboolean) {
    let mut from_ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut to_ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    S_ClearSoundBuffer();
    from_ospath =
        FS_BuildOSPath((*fs_homepath).string, from,
                       b"\x00" as *const u8 as *const libc::c_char);
    to_ospath =
        FS_BuildOSPath((*fs_homepath).string, to,
                       b"\x00" as *const u8 as *const libc::c_char);
    *from_ospath.offset(strlen(from_ospath).wrapping_sub(1i32 as
                                                             libc::c_ulong) as
                            isize) = '\u{0}' as i32 as libc::c_char;
    *to_ospath.offset(strlen(to_ospath).wrapping_sub(1i32 as libc::c_ulong) as
                          isize) = '\u{0}' as i32 as libc::c_char;
    if 0 != (*fs_debug).integer {
        Com_Printf(b"FS_SV_Rename: %s --> %s\n\x00" as *const u8 as
                       *const libc::c_char, from_ospath, to_ospath);
    }
    if 0 != safe as u64 {
        FS_CheckFilenameIsMutable(to_ospath,
                                  (*::std::mem::transmute::<&[u8; 13],
                                                            &[libc::c_char; 13]>(b"FS_SV_Rename\x00")).as_ptr());
    }
    rename(from_ospath, to_ospath);
}
// if uniqueFILE is true, then a new FILE will be fopened even if the file
// is found in an already open pak file.  If uniqueFILE is false, you must call
// FS_FCloseFile instead of fclose, otherwise the pak FILE would be improperly closed
// It is generally safe to always set uniqueFILE to true, because the majority of
// file IO goes through FS_ReadFile, which Does The Right Thing already.
#[no_mangle]
pub unsafe extern "C" fn FS_FileIsInPAK(mut filename: *const libc::c_char,
                                        mut pChecksum: *mut libc::c_int)
 -> libc::c_int {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut pakFile: *mut fileInPack_t = 0 as *mut fileInPack_t;
    let mut hash: libc::c_long = 0i32 as libc::c_long;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if filename.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"FS_FOpenFileRead: NULL \'filename\' parameter passed\x00"
                      as *const u8 as *const libc::c_char);
    }
    if *filename.offset(0isize) as libc::c_int == '/' as i32 ||
           *filename.offset(0isize) as libc::c_int == '\\' as i32 {
        filename = filename.offset(1isize)
    }
    if !strstr(filename,
               b"..\x00" as *const u8 as *const libc::c_char).is_null() ||
           !strstr(filename,
                   b"::\x00" as *const u8 as *const libc::c_char).is_null() {
        return -1i32
    }
    search = fs_searchpaths;
    while !search.is_null() {
        if !(*search).pack.is_null() {
            hash = FS_HashFileName(filename, (*(*search).pack).hashSize)
        }
        // is the element a pak file?
        if !(*search).pack.is_null() &&
               !(*(*(*search).pack).hashTable.offset(hash as isize)).is_null()
           {
            // disregard if it doesn't match one of the allowed pure pak files
            if !(0 == FS_PakIsPure((*search).pack) as u64) {
                pak = (*search).pack;
                pakFile = *(*pak).hashTable.offset(hash as isize);
                loop  {
                    if 0 ==
                           FS_FilenameCompare((*pakFile).name, filename) as
                               u64 {
                        if !pChecksum.is_null() {
                            *pChecksum = (*pak).pure_checksum
                        }
                        return 1i32
                    }
                    pakFile = (*pakFile).next;
                    if pakFile.is_null() { break ; }
                }
            }
        }
        search = (*search).next
    }
    return -1i32;
}
// returns the length of the file
// a null buffer will just return the file length without loading
// as a quick check for existence. -1 length == not present
// A 0 byte will always be appended at the end, so string ops are safe.
// the buffer should be considered read-only, because it may be cached
// for other uses.
#[no_mangle]
pub unsafe extern "C" fn FS_ForceFlush(mut f: fileHandle_t) {
    let mut file: *mut FILE = 0 as *mut FILE;
    file = FS_FileForHandle(f);
    setvbuf(file, 0 as *mut libc::c_char, 2i32, 0i32 as size_t);
}
// forces flush on files we're writing to.
#[no_mangle]
pub unsafe extern "C" fn FS_FreeFile(mut buffer: *mut libc::c_void) {
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if buffer.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"FS_FreeFile( NULL )\x00" as *const u8 as
                      *const libc::c_char);
    }
    fs_loadStack -= 1;
    Hunk_FreeTempMemory(buffer);
    if fs_loadStack == 0i32 { Hunk_ClearTempMemory(); };
}
// frees the memory returned by FS_ReadFile
#[no_mangle]
pub unsafe extern "C" fn FS_WriteFile(mut qpath: *const libc::c_char,
                                      mut buffer: *const libc::c_void,
                                      mut size: libc::c_int) {
    let mut f: fileHandle_t = 0;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if qpath.is_null() || buffer.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"FS_WriteFile: NULL parameter\x00" as *const u8 as
                      *const libc::c_char);
    }
    f = FS_FOpenFileWrite(qpath);
    if 0 == f {
        Com_Printf(b"Failed to open %s\n\x00" as *const u8 as
                       *const libc::c_char, qpath);
        return
    }
    FS_Write(buffer, size, f);
    FS_FCloseFile(f);
}
// doesn't work for files that are opened from a pack file
#[no_mangle]
pub unsafe extern "C" fn FS_FTell(mut f: fileHandle_t) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    if fsh[f as usize].zipFile as libc::c_uint ==
           qtrue as libc::c_int as libc::c_uint {
        pos = unztell(fsh[f as usize].handleFiles.file.z) as libc::c_int
    } else { pos = ftell(fsh[f as usize].handleFiles.file.o) as libc::c_int }
    return pos;
}
// like fprintf
#[no_mangle]
pub unsafe extern "C" fn FS_FOpenFileByMode(mut qpath: *const libc::c_char,
                                            mut f: *mut fileHandle_t,
                                            mut mode: fsMode_t)
 -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut sync: qboolean = qfalse;
    sync = qfalse;
    let mut current_block_14: u64;
    match mode as libc::c_uint {
        0 => {
            r = FS_FOpenFileRead(qpath, f, qtrue) as libc::c_int;
            current_block_14 = 17833034027772472439;
        }
        1 => {
            *f = FS_FOpenFileWrite(qpath);
            r = 0i32;
            if *f == 0i32 { r = -1i32 }
            current_block_14 = 17833034027772472439;
        }
        3 => { sync = qtrue; current_block_14 = 17569574545379342456; }
        2 => { current_block_14 = 17569574545379342456; }
        _ => {
            Com_Error(ERR_FATAL as libc::c_int,
                      b"FS_FOpenFileByMode: bad mode\x00" as *const u8 as
                          *const libc::c_char);
        }
    }
    match current_block_14 {
        17569574545379342456 => {
            *f = FS_FOpenFileAppend(qpath);
            r = 0i32;
            if *f == 0i32 { r = -1i32 }
        }
        _ => { }
    }
    if f.is_null() { return r }
    if 0 != *f { fsh[*f as usize].fileSize = r }
    fsh[*f as usize].handleSync = sync;
    return r;
}
// opens a file for reading, writing, or appending depending on the value of mode
#[no_mangle]
pub unsafe extern "C" fn FS_Seek(mut f: fileHandle_t,
                                 mut offset: libc::c_long,
                                 mut origin: libc::c_int) -> libc::c_int {
    let mut _origin: libc::c_int = 0;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    if fsh[f as usize].zipFile as libc::c_uint ==
           qtrue as libc::c_int as libc::c_uint {
        let mut buffer: [byte; 65536] = [0; 65536];
        let mut remainder: libc::c_int = 0;
        let mut currentPosition: libc::c_int = FS_FTell(f);
        if offset < 0i32 as libc::c_long {
            match origin {
                1 => {
                    remainder =
                        (fsh[f as usize].zipFileLen as libc::c_long + offset)
                            as libc::c_int
                }
                0 => {
                    remainder =
                        (currentPosition as libc::c_long + offset) as
                            libc::c_int
                }
                2 | _ => { remainder = 0i32 }
            }
            if remainder < 0i32 { remainder = 0i32 }
            origin = FS_SEEK_SET as libc::c_int
        } else if origin == FS_SEEK_END as libc::c_int {
            remainder =
                ((fsh[f as usize].zipFileLen - currentPosition) as
                     libc::c_long + offset) as libc::c_int
        } else { remainder = offset as libc::c_int }
        match origin {
            2 => {
                if remainder == currentPosition {
                    return offset as libc::c_int
                }
                unzSetOffset(fsh[f as usize].handleFiles.file.z,
                             fsh[f as usize].zipFilePos as uLong);
                unzOpenCurrentFile(fsh[f as usize].handleFiles.file.z);
            }
            1 | 0 => { }
            _ => {
                //fallthrough
                Com_Error(ERR_FATAL as libc::c_int,
                          b"Bad origin in FS_Seek\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        while remainder > 65536i32 {
            FS_Read(buffer.as_mut_ptr() as *mut libc::c_void, 65536i32, f);
            remainder -= 65536i32
        }
        FS_Read(buffer.as_mut_ptr() as *mut libc::c_void, remainder, f);
        return offset as libc::c_int
    } else {
        let mut file: *mut FILE = 0 as *mut FILE;
        file = FS_FileForHandle(f);
        match origin {
            0 => { _origin = 1i32 }
            1 => { _origin = 2i32 }
            2 => { _origin = 0i32 }
            _ => {
                Com_Error(ERR_FATAL as libc::c_int,
                          b"Bad origin in FS_Seek\x00" as *const u8 as
                              *const libc::c_char);
            }
        }
        return fseek(file, offset, _origin)
    };
}
#[no_mangle]
pub unsafe extern "C" fn FS_LoadedPakNames() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    info[0usize] = 0i32 as libc::c_char;
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file?
        if !(*search).pack.is_null() {
            if 0 != *info.as_mut_ptr() {
                Q_strcat(info.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 8192]>() as
                             libc::c_ulong as libc::c_int,
                         b" \x00" as *const u8 as *const libc::c_char);
            }
            Q_strcat(info.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 8192]>() as
                         libc::c_ulong as libc::c_int,
                     (*(*search).pack).pakBasename.as_mut_ptr());
        }
        search = (*search).next
    }
    return info.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn FS_LoadedPakChecksums() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    info[0usize] = 0i32 as libc::c_char;
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file? 
        if !(*search).pack.is_null() {
            Q_strcat(info.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 8192]>() as
                         libc::c_ulong as libc::c_int,
                     va(b"%i \x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char, (*(*search).pack).checksum));
        }
        search = (*search).next
    }
    return info.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn FS_LoadedPakPureChecksums() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    info[0usize] = 0i32 as libc::c_char;
    search = fs_searchpaths;
    while !search.is_null() {
        // is the element a pak file? 
        if !(*search).pack.is_null() {
            Q_strcat(info.as_mut_ptr(),
                     ::std::mem::size_of::<[libc::c_char; 8192]>() as
                         libc::c_ulong as libc::c_int,
                     va(b"%i \x00" as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        (*(*search).pack).pure_checksum));
        }
        search = (*search).next
    }
    return info.as_mut_ptr();
}
// Returns a space separated string containing the checksums of all loaded pk3 files.
// Servers with sv_pure set will get this string and pass it to clients.
#[no_mangle]
pub unsafe extern "C" fn FS_ReferencedPakNames() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    info[0usize] = 0i32 as libc::c_char;
    search = fs_searchpaths;
    while !search.is_null() {
        if !(*search).pack.is_null() {
            if 0 != (*(*search).pack).referenced ||
                   0 !=
                       Q_stricmpn((*(*search).pack).pakGamename.as_mut_ptr(),
                                  (*com_basegame).string,
                                  strlen((*com_basegame).string) as
                                      libc::c_int) {
                if 0 != *info.as_mut_ptr() {
                    Q_strcat(info.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 8192]>() as
                                 libc::c_ulong as libc::c_int,
                             b" \x00" as *const u8 as *const libc::c_char);
                }
                Q_strcat(info.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 8192]>() as
                             libc::c_ulong as libc::c_int,
                         (*(*search).pack).pakGamename.as_mut_ptr());
                Q_strcat(info.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 8192]>() as
                             libc::c_ulong as libc::c_int,
                         b"/\x00" as *const u8 as *const libc::c_char);
                Q_strcat(info.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 8192]>() as
                             libc::c_ulong as libc::c_int,
                         (*(*search).pack).pakBasename.as_mut_ptr());
            }
        }
        search = (*search).next
    }
    return info.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn FS_ReferencedPakChecksums() -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    info[0usize] = 0i32 as libc::c_char;
    search = fs_searchpaths;
    while !search.is_null() {
        if !(*search).pack.is_null() {
            if 0 != (*(*search).pack).referenced ||
                   0 !=
                       Q_stricmpn((*(*search).pack).pakGamename.as_mut_ptr(),
                                  (*com_basegame).string,
                                  strlen((*com_basegame).string) as
                                      libc::c_int) {
                Q_strcat(info.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 8192]>() as
                             libc::c_ulong as libc::c_int,
                         va(b"%i \x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                            (*(*search).pack).checksum));
            }
        }
        search = (*search).next
    }
    return info.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn FS_ReferencedPakPureChecksums()
 -> *const libc::c_char {
    static mut info: [libc::c_char; 8192] = [0; 8192];
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut nFlags: libc::c_int = 0;
    let mut numPaks: libc::c_int = 0;
    let mut checksum: libc::c_int = 0;
    info[0usize] = 0i32 as libc::c_char;
    checksum = fs_checksumFeed;
    numPaks = 0i32;
    nFlags = 0x4i32;
    while 0 != nFlags {
        if 0 != nFlags & 0x1i32 {
            info[strlen(info.as_mut_ptr()).wrapping_add(1i32 as libc::c_ulong)
                     as usize] = '\u{0}' as i32 as libc::c_char;
            info[strlen(info.as_mut_ptr()).wrapping_add(2i32 as libc::c_ulong)
                     as usize] = '\u{0}' as i32 as libc::c_char;
            info[strlen(info.as_mut_ptr()) as usize] =
                '@' as i32 as libc::c_char;
            info[strlen(info.as_mut_ptr()) as usize] =
                ' ' as i32 as libc::c_char
        }
        search = fs_searchpaths;
        while !search.is_null() {
            // is the element a pak file and has it been referenced based on flag?
            if !(*search).pack.is_null() &&
                   0 != (*(*search).pack).referenced & nFlags {
                Q_strcat(info.as_mut_ptr(),
                         ::std::mem::size_of::<[libc::c_char; 8192]>() as
                             libc::c_ulong as libc::c_int,
                         va(b"%i \x00" as *const u8 as *const libc::c_char as
                                *mut libc::c_char,
                            (*(*search).pack).pure_checksum));
                if 0 != nFlags & (0x4i32 | 0x2i32) { break ; }
                checksum ^= (*(*search).pack).pure_checksum;
                numPaks += 1
            }
            search = (*search).next
        }
        nFlags = nFlags >> 1i32
    }
    checksum ^= numPaks;
    Q_strcat(info.as_mut_ptr(),
             ::std::mem::size_of::<[libc::c_char; 8192]>() as libc::c_ulong as
                 libc::c_int,
             va(b"%i \x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char, checksum));
    return info.as_mut_ptr();
}
// clears referenced booleans on loaded pk3s
#[no_mangle]
pub unsafe extern "C" fn FS_PureServerSetReferencedPaks(mut pakSums:
                                                            *const libc::c_char,
                                                        mut pakNames:
                                                            *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut d: libc::c_int = 0i32;
    Cmd_TokenizeString(pakSums);
    c = Cmd_Argc();
    if c > 4096i32 { c = 4096i32 }
    i = 0i32;
    while i < c {
        fs_serverReferencedPaks[i as usize] = atoi(Cmd_Argv(i));
        i += 1
    }
    i = 0i32;
    while (i as libc::c_ulong) <
              (::std::mem::size_of::<[*mut libc::c_char; 4096]>() as
                   libc::c_ulong).wrapping_div(::std::mem::size_of::<*mut libc::c_char>()
                                                   as libc::c_ulong) {
        if !fs_serverReferencedPakNames[i as usize].is_null() {
            Z_Free(fs_serverReferencedPakNames[i as usize] as
                       *mut libc::c_void);
        }
        fs_serverReferencedPakNames[i as usize] = 0 as *mut libc::c_char;
        i += 1
    }
    if !pakNames.is_null() && 0 != *pakNames as libc::c_int {
        Cmd_TokenizeString(pakNames);
        d = Cmd_Argc();
        if d > c { d = c }
        i = 0i32;
        while i < d {
            fs_serverReferencedPakNames[i as usize] = CopyString(Cmd_Argv(i));
            i += 1
        }
    }
    if d < c { c = d }
    fs_numServerReferencedPaks = c;
}
// pk3 names
static mut fs_serverReferencedPakNames: [*mut libc::c_char; 4096] =
    [0 as *const libc::c_char as *mut libc::c_char; 4096];
// If the string is empty, all data sources will be allowed.
// If not empty, only pk3 files that match one of the space
// separated checksums will be checked for files, with the
// sole exception of .cfg files.
#[no_mangle]
pub unsafe extern "C" fn FS_CheckDirTraversal(mut checkdir:
                                                  *const libc::c_char)
 -> qboolean {
    if !strstr(checkdir,
               b"../\x00" as *const u8 as *const libc::c_char).is_null() ||
           !strstr(checkdir,
                   b"..\\\x00" as *const u8 as *const libc::c_char).is_null()
       {
        return qtrue
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn FS_idPak(mut pak: *mut libc::c_char,
                                  mut base: *mut libc::c_char,
                                  mut numPaks: libc::c_int) -> qboolean {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 9i32 {
        if 0 ==
               FS_FilenameCompare(pak,
                                  va(b"%s/pak%d\x00" as *const u8 as
                                         *const libc::c_char as
                                         *mut libc::c_char, base, i)) as u64 {
            break ;
        }
        i += 1
    }
    if i < numPaks { return qtrue }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn FS_ComparePaks(mut neededpaks: *mut libc::c_char,
                                        mut len: libc::c_int,
                                        mut dlstring: qboolean) -> qboolean {
    let mut sp: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut havepak: qboolean = qfalse;
    let mut origpos: *mut libc::c_char = neededpaks;
    let mut i: libc::c_int = 0;
    if 0 == fs_numServerReferencedPaks { return qfalse }
    *neededpaks = 0i32 as libc::c_char;
    i = 0i32;
    while i < fs_numServerReferencedPaks {
        havepak = qfalse;
        // never autodownload any of the id paks
        if !(0 !=
                 FS_idPak(fs_serverReferencedPakNames[i as usize],
                          b"baseq3\x00" as *const u8 as *const libc::c_char as
                              *mut libc::c_char, 9i32) as libc::c_uint ||
                 0 !=
                     FS_idPak(fs_serverReferencedPakNames[i as usize],
                              b"missionpack\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              4i32) as libc::c_uint) {
            // Make sure the server cannot make us write to non-quake3 directories.
            if 0 !=
                   FS_CheckDirTraversal(fs_serverReferencedPakNames[i as
                                                                        usize])
                       as u64 {
                Com_Printf(b"WARNING: Invalid download name %s\n\x00" as
                               *const u8 as *const libc::c_char,
                           fs_serverReferencedPakNames[i as usize]);
            } else {
                sp = fs_searchpaths;
                while !sp.is_null() {
                    if !(*sp).pack.is_null() &&
                           (*(*sp).pack).checksum ==
                               fs_serverReferencedPaks[i as usize] {
                        havepak = qtrue;
                        break ;
                    } else { sp = (*sp).next }
                }
                if 0 == havepak as u64 &&
                       !fs_serverReferencedPakNames[i as usize].is_null() &&
                       0 !=
                           *fs_serverReferencedPakNames[i as usize] as
                               libc::c_int {
                    // Don't got it
                    if 0 != dlstring as u64 {
                        origpos = origpos.offset(strlen(origpos) as isize);
                        Q_strcat(neededpaks, len,
                                 b"@\x00" as *const u8 as
                                     *const libc::c_char);
                        Q_strcat(neededpaks, len,
                                 fs_serverReferencedPakNames[i as usize]);
                        Q_strcat(neededpaks, len,
                                 b".pk3\x00" as *const u8 as
                                     *const libc::c_char);
                        Q_strcat(neededpaks, len,
                                 b"@\x00" as *const u8 as
                                     *const libc::c_char);
                        if 0 !=
                               FS_SV_FileExists(va(b"%s.pk3\x00" as *const u8
                                                       as *const libc::c_char
                                                       as *mut libc::c_char,
                                                   fs_serverReferencedPakNames[i
                                                                                   as
                                                                                   usize]))
                                   as u64 {
                            let mut st: [libc::c_char; 256] = [0; 256];
                            Com_sprintf(st.as_mut_ptr(),
                                        ::std::mem::size_of::<[libc::c_char; 256]>()
                                            as libc::c_ulong as libc::c_int,
                                        b"%s.%08x.pk3\x00" as *const u8 as
                                            *const libc::c_char,
                                        fs_serverReferencedPakNames[i as
                                                                        usize],
                                        fs_serverReferencedPaks[i as usize]);
                            Q_strcat(neededpaks, len, st.as_mut_ptr());
                        } else {
                            Q_strcat(neededpaks, len,
                                     fs_serverReferencedPakNames[i as usize]);
                            Q_strcat(neededpaks, len,
                                     b".pk3\x00" as *const u8 as
                                         *const libc::c_char);
                        }
                        // Find out whether it might have overflowed the buffer and don't add this file to the
				// list if that is the case.
                        if strlen(origpos).wrapping_add(origpos.wrapping_offset_from(neededpaks)
                                                            as libc::c_long as
                                                            libc::c_ulong) >=
                               (len - 1i32) as libc::c_ulong {
                            *origpos = '\u{0}' as i32 as libc::c_char;
                            break ;
                        }
                    } else {
                        Q_strcat(neededpaks, len,
                                 fs_serverReferencedPakNames[i as usize]);
                        Q_strcat(neededpaks, len,
                                 b".pk3\x00" as *const u8 as
                                     *const libc::c_char);
                        if 0 !=
                               FS_SV_FileExists(va(b"%s.pk3\x00" as *const u8
                                                       as *const libc::c_char
                                                       as *mut libc::c_char,
                                                   fs_serverReferencedPakNames[i
                                                                                   as
                                                                                   usize]))
                                   as u64 {
                            Q_strcat(neededpaks, len,
                                     b" (local file exists with wrong checksum)\x00"
                                         as *const u8 as *const libc::c_char);
                        }
                        Q_strcat(neededpaks, len,
                                 b"\n\x00" as *const u8 as
                                     *const libc::c_char);
                    }
                }
            }
        }
        i += 1
    }
    if 0 != *neededpaks { return qtrue }
    return qfalse;
}
/*
================
FS_SV_FileExists

Tests if the file exists 
================
*/
#[no_mangle]
pub unsafe extern "C" fn FS_SV_FileExists(mut file: *const libc::c_char)
 -> qboolean {
    let mut testpath: *mut libc::c_char = 0 as *mut libc::c_char;
    testpath =
        FS_BuildOSPath((*fs_homepath).string, file,
                       b"\x00" as *const u8 as *const libc::c_char);
    *testpath.offset(strlen(testpath).wrapping_sub(1i32 as libc::c_ulong) as
                         isize) = '\u{0}' as i32 as libc::c_char;
    return FS_FileInPathExists(testpath);
}
#[no_mangle]
pub unsafe extern "C" fn FS_Rename(mut from: *const libc::c_char,
                                   mut to: *const libc::c_char) {
    let mut from_ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut to_ospath: *mut libc::c_char = 0 as *mut libc::c_char;
    if fs_searchpaths.is_null() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Filesystem call made without initialization\x00" as
                      *const u8 as *const libc::c_char);
    }
    S_ClearSoundBuffer();
    from_ospath =
        FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(), from);
    to_ospath =
        FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(), to);
    if 0 != (*fs_debug).integer {
        Com_Printf(b"FS_Rename: %s --> %s\n\x00" as *const u8 as
                       *const libc::c_char, from_ospath, to_ospath);
    }
    FS_CheckFilenameIsMutable(to_ospath,
                              (*::std::mem::transmute::<&[u8; 10],
                                                        &[libc::c_char; 10]>(b"FS_Rename\x00")).as_ptr());
    rename(from_ospath, to_ospath);
}
#[no_mangle]
pub unsafe extern "C" fn FS_Remove(mut osPath: *const libc::c_char) {
    FS_CheckFilenameIsMutable(osPath,
                              (*::std::mem::transmute::<&[u8; 10],
                                                        &[libc::c_char; 10]>(b"FS_Remove\x00")).as_ptr());
    remove(osPath);
}
#[no_mangle]
pub unsafe extern "C" fn FS_HomeRemove(mut homePath: *const libc::c_char) {
    FS_CheckFilenameIsMutable(homePath,
                              (*::std::mem::transmute::<&[u8; 14],
                                                        &[libc::c_char; 14]>(b"FS_HomeRemove\x00")).as_ptr());
    remove(FS_BuildOSPath((*fs_homepath).string, fs_gamedir.as_mut_ptr(),
                          homePath));
}
#[no_mangle]
pub unsafe extern "C" fn FS_FilenameCompletion(mut dir: *const libc::c_char,
                                               mut ext: *const libc::c_char,
                                               mut stripExt: qboolean,
                                               mut callback:
                                                   Option<unsafe extern "C" fn(_:
                                                                                   *const libc::c_char)
                                                              -> ()>,
                                               mut allowNonPureFilesOnDisk:
                                                   qboolean) {
    let mut filenames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nfiles: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut filename: [libc::c_char; 1024] = [0; 1024];
    filenames =
        FS_ListFilteredFiles(dir, ext, 0 as *mut libc::c_char, &mut nfiles,
                             allowNonPureFilesOnDisk);
    FS_SortFileList(filenames, nfiles);
    i = 0i32;
    while i < nfiles {
        FS_ConvertPath(*filenames.offset(i as isize));
        Q_strncpyz(filename.as_mut_ptr(), *filenames.offset(i as isize),
                   1024i32);
        if 0 != stripExt as u64 {
            COM_StripExtension(filename.as_mut_ptr(), filename.as_mut_ptr(),
                               ::std::mem::size_of::<[libc::c_char; 1024]>()
                                   as libc::c_ulong as libc::c_int);
        }
        callback.expect("non-null function pointer")(filename.as_mut_ptr());
        i += 1
    }
    FS_FreeFileList(filenames);
}