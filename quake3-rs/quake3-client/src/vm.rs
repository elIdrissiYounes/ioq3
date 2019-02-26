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
        pub fn qsort(__base: *mut libc::c_void, __nmemb: size_t,
                     __size: size_t, __compar: __compar_fn_t);
    }
}
#[header_src = "/usr/include/stdint.h"]
pub mod stdint_h {
    pub type intptr_t = libc::c_long;
    use super::{libc};
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
    // font rendering values used by ui and cgame
    // default
    // default
    pub type ha_pref = libc::c_uint;
    pub const h_dontcare: ha_pref = 2;
    pub const h_low: ha_pref = 1;
    pub const h_high: ha_pref = 0;
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
        pub fn Hunk_AllocDebug(size: libc::c_int, preference: ha_pref,
                               label: *mut libc::c_char,
                               file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn COM_StripExtension(in_0: *const libc::c_char,
                                  out: *mut libc::c_char,
                                  destsize: libc::c_int);
        #[no_mangle]
        pub fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/qcommon/qfiles.h"]
pub mod qfiles_h {
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
// qfiles.h: quake file formats
// This file must be identical in the quake and utils directories
//
    //Ignore __attribute__ on non-gcc platforms
    // surface geometry should not exceed these limits
    // the maximum size of game relative pathnames
    /*
========================================================================

QVM files

========================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vmHeader_t {
        pub vmMagic: libc::c_int,
        pub instructionCount: libc::c_int,
        pub codeOffset: libc::c_int,
        pub codeLength: libc::c_int,
        pub dataOffset: libc::c_int,
        pub dataLength: libc::c_int,
        pub litLength: libc::c_int,
        pub bssLength: libc::c_int,
        pub jtrgLength: libc::c_int,
    }
    use super::{libc};
}
#[header_src =
      "ioq3/code/qcommon/vm_local.h"]
pub mod vm_local_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vm_s {
        pub programStack: libc::c_int,
        pub systemCall: Option<unsafe extern "C" fn(_: *mut intptr_t)
                                   -> intptr_t>,
        pub name: [libc::c_char; 64],
        pub searchPath: *mut libc::c_void,
        pub dllHandle: *mut libc::c_void,
        pub entryPoint: Option<unsafe extern "C" fn(_: libc::c_int, ...)
                                   -> intptr_t>,
        pub destroy: Option<unsafe extern "C" fn(_: *mut vm_t) -> ()>,
        pub currentlyInterpreting: qboolean,
        pub compiled: qboolean,
        pub codeBase: *mut byte,
        pub entryOfs: libc::c_int,
        pub codeLength: libc::c_int,
        pub instructionPointers: *mut intptr_t,
        pub instructionCount: libc::c_int,
        pub dataBase: *mut byte,
        pub dataMask: libc::c_int,
        pub dataAlloc: libc::c_int,
        pub stackBottom: libc::c_int,
        pub numSymbols: libc::c_int,
        pub symbols: *mut vmSymbol_s,
        pub callLevel: libc::c_int,
        pub breakFunction: libc::c_int,
        pub breakCount: libc::c_int,
        pub jumpTableTargets: *mut byte,
        pub numJumpTableTargets: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vmSymbol_s {
        pub next: *mut vmSymbol_s,
        pub symValue: libc::c_int,
        pub profileCount: libc::c_int,
        pub symName: [libc::c_char; 0],
    }
    pub type vmSymbol_t = vmSymbol_s;
    use super::{libc};
    use super::stdint_h::{intptr_t};
    use super::qcommon_h::{vm_t};
    use super::q_shared_h::{qboolean, byte};
    use super::qfiles_h::{vmHeader_t};
    use super::stddef_h::{size_t};
    extern "C" {
        #[no_mangle]
        pub fn VM_PrepareInterpreter(vm: *mut vm_t, header: *mut vmHeader_t);
        #[no_mangle]
        pub fn VM_Compile(vm: *mut vm_t, header: *mut vmHeader_t);
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    /*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
    pub type vm_t = vm_s;
    pub type vmInterpret_t = libc::c_uint;
    pub const VMI_COMPILED: vmInterpret_t = 2;
    pub const VMI_BYTECODE: vmInterpret_t = 1;
    pub const VMI_NATIVE: vmInterpret_t = 0;
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
    use super::vm_local_h::{vm_s};
    use super::{libc};
    use super::q_shared_h::{cvar_t, qboolean};
    use super::stdint_h::{intptr_t};
    extern "C" {
        #[no_mangle]
        pub fn Cmd_AddCommand(cmd_name: *const libc::c_char,
                              function: xcommand_t);
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
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
        #[no_mangle]
        pub fn Hunk_MemoryRemaining() -> libc::c_int;
        // forces flush on files we're writing to.
        #[no_mangle]
        pub fn FS_FreeFile(buffer: *mut libc::c_void);
        #[no_mangle]
        pub fn FS_ReadFile(qpath: *const libc::c_char,
                           buffer: *mut *mut libc::c_void) -> libc::c_long;
        #[no_mangle]
        pub static mut com_developer: *mut cvar_t;
        #[no_mangle]
        pub fn Sys_UnloadDll(dllHandle: *mut libc::c_void);
        #[no_mangle]
        pub fn FS_Which(filename: *const libc::c_char,
                        searchPath: *mut libc::c_void) -> qboolean;
        // note: you can't just fclose from another DLL, due to MS libc issues
        #[no_mangle]
        pub fn FS_ReadFileDir(qpath: *const libc::c_char,
                              searchPath: *mut libc::c_void, unpure: qboolean,
                              buffer: *mut *mut libc::c_void) -> libc::c_long;
        // general development dll loading for virtual machine testing
        #[no_mangle]
        pub fn Sys_LoadGameDll(name: *const libc::c_char,
                               entryPoint:
                                   *mut Option<unsafe extern "C" fn(_:
                                                                        libc::c_int, ...)
                                                   -> intptr_t>,
                               systemcalls:
                                   Option<unsafe extern "C" fn(_:
                                                                   intptr_t, ...)
                                              -> intptr_t>)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn FS_FindVM(startSearch: *mut *mut libc::c_void,
                         found: *mut libc::c_char, foundlen: libc::c_int,
                         name: *const libc::c_char, enableDll: libc::c_int)
         -> libc::c_int;
    }
}
#[header_src =
      "ioq3/code/qcommon/vm.c"]
pub mod vm_c {
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed_0 {
        pub c: *mut libc::c_char,
        pub v: *mut libc::c_void,
    }
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed_1 {
        pub h: *mut vmHeader_t,
        pub v: *mut libc::c_void,
    }
    use super::{libc};
    use super::qfiles_h::{vmHeader_t};
    use super::qcommon_h::{vm_t};
    use super::q_shared_h::{qboolean};
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::FILE_h::{FILE};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn fopen(_: *const libc::c_char, _: *const libc::c_char)
         -> *mut FILE;
        #[no_mangle]
        pub fn fprintf(_: *mut FILE, _: *const libc::c_char, ...)
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
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src =
      "ioq3/code/qcommon/vm_variadic.h"]
pub mod vm_variadic_h {
    use super::stdint_h::{intptr_t};
    extern "C" {
        #[no_mangle]
        pub fn VM_DllSyscall(arg: intptr_t, ...) -> intptr_t;
    }
}
use self::types_h::{__off_t, __off64_t};
use self::stddef_h::{size_t};
use self::struct_FILE_h::{_IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt,
                          _IO_marker};
use self::FILE_h::{FILE};
use self::stdlib_h::{__compar_fn_t, qsort};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, unnamed, ERR_NEED_CD,
                       ERR_DISCONNECT, ERR_SERVERDISCONNECT, ERR_DROP,
                       ERR_FATAL, ha_pref, h_dontcare, h_low, h_high, cvar_s,
                       cvar_t, Hunk_AllocDebug, COM_StripExtension, COM_Parse,
                       Com_sprintf, Q_stricmp, Q_strncpyz, Com_Error,
                       Com_Printf};
use self::qfiles_h::{vmHeader_t};
use self::vm_local_h::{vm_s, vmSymbol_s, vmSymbol_t, VM_PrepareInterpreter,
                       VM_Compile};
use self::qcommon_h::{vm_t, vmInterpret_t, VMI_COMPILED, VMI_BYTECODE,
                      VMI_NATIVE, xcommand_t, Cmd_AddCommand, Z_Free,
                      Z_MallocDebug, Cvar_Get, Hunk_MemoryRemaining,
                      FS_FreeFile, FS_ReadFile, com_developer, Sys_UnloadDll,
                      FS_Which, FS_ReadFileDir, Sys_LoadGameDll, FS_FindVM};
use self::vm_c::{unnamed_0, unnamed_1};
use self::stdio_h::{fopen, fprintf};
use self::string_h::{memcpy, memset, strcmp, strlen};
use self::vm_variadic_h::{VM_DllSyscall};
#[no_mangle]
pub unsafe extern "C" fn VM_Init() {
    Cvar_Get(b"vm_cgame\x00" as *const u8 as *const libc::c_char,
             b"2\x00" as *const u8 as *const libc::c_char, 0x1i32);
    Cvar_Get(b"vm_game\x00" as *const u8 as *const libc::c_char,
             b"2\x00" as *const u8 as *const libc::c_char, 0x1i32);
    Cvar_Get(b"vm_ui\x00" as *const u8 as *const libc::c_char,
             b"2\x00" as *const u8 as *const libc::c_char, 0x1i32);
    Cmd_AddCommand(b"vmprofile\x00" as *const u8 as *const libc::c_char,
                   Some(VM_VmProfile_f));
    Cmd_AddCommand(b"vminfo\x00" as *const u8 as *const libc::c_char,
                   Some(VM_VmInfo_f));
    memset(vmTable.as_mut_ptr() as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<[vm_t; 3]>() as libc::c_ulong);
}
#[no_mangle]
pub static mut vmTable: [vm_t; 3] =
    [vm_s{programStack: 0,
          systemCall: None,
          name: [0; 64],
          searchPath: 0 as *const libc::c_void as *mut libc::c_void,
          dllHandle: 0 as *const libc::c_void as *mut libc::c_void,
          entryPoint: None,
          destroy: None,
          currentlyInterpreting: qfalse,
          compiled: qfalse,
          codeBase: 0 as *const byte as *mut byte,
          entryOfs: 0,
          codeLength: 0,
          instructionPointers: 0 as *const intptr_t as *mut intptr_t,
          instructionCount: 0,
          dataBase: 0 as *const byte as *mut byte,
          dataMask: 0,
          dataAlloc: 0,
          stackBottom: 0,
          numSymbols: 0,
          symbols: 0 as *const vmSymbol_s as *mut vmSymbol_s,
          callLevel: 0,
          breakFunction: 0,
          breakCount: 0,
          jumpTableTargets: 0 as *const byte as *mut byte,
          numJumpTableTargets: 0,}; 3];
#[no_mangle]
pub unsafe extern "C" fn VM_VmInfo_f() {
    let mut vm: *mut vm_t = 0 as *mut vm_t;
    let mut i: libc::c_int = 0;
    Com_Printf(b"Registered virtual machines:\n\x00" as *const u8 as
                   *const libc::c_char);
    i = 0i32;
    while i < 3i32 {
        vm = &mut *vmTable.as_mut_ptr().offset(i as isize) as *mut vm_t;
        if 0 == (*vm).name[0usize] { break ; }
        Com_Printf(b"%s : \x00" as *const u8 as *const libc::c_char,
                   (*vm).name.as_mut_ptr());
        if !(*vm).dllHandle.is_null() {
            Com_Printf(b"native\n\x00" as *const u8 as *const libc::c_char);
        } else {
            if 0 != (*vm).compiled as u64 {
                Com_Printf(b"compiled on load\n\x00" as *const u8 as
                               *const libc::c_char);
            } else {
                Com_Printf(b"interpreted\n\x00" as *const u8 as
                               *const libc::c_char);
            }
            Com_Printf(b"    code length : %7i\n\x00" as *const u8 as
                           *const libc::c_char, (*vm).codeLength);
            Com_Printf(b"    table length: %7i\n\x00" as *const u8 as
                           *const libc::c_char,
                       (*vm).instructionCount * 4i32);
            Com_Printf(b"    data length : %7i\n\x00" as *const u8 as
                           *const libc::c_char, (*vm).dataMask + 1i32);
        }
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn VM_VmProfile_f() {
    let mut vm: *mut vm_t = 0 as *mut vm_t;
    let mut sorted: *mut *mut vmSymbol_t = 0 as *mut *mut vmSymbol_t;
    let mut sym: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    let mut i: libc::c_int = 0;
    let mut total: libc::c_double = 0.;
    if lastVM.is_null() { return }
    vm = lastVM;
    if 0 == (*vm).numSymbols { return }
    sorted =
        Z_MallocDebug(((*vm).numSymbols as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut vmSymbol_t>()
                                                           as libc::c_ulong)
                          as libc::c_int,
                      b"vm->numSymbols * sizeof( *sorted )\x00" as *const u8
                          as *const libc::c_char as *mut libc::c_char,
                      b"code/qcommon/vm.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 766i32) as
            *mut *mut vmSymbol_t;
    let ref mut fresh0 = *sorted.offset(0isize);
    *fresh0 = (*vm).symbols;
    total = (**sorted.offset(0isize)).profileCount as libc::c_double;
    i = 1i32;
    while i < (*vm).numSymbols {
        let ref mut fresh1 = *sorted.offset(i as isize);
        *fresh1 = (**sorted.offset((i - 1i32) as isize)).next;
        total += (**sorted.offset(i as isize)).profileCount as libc::c_double;
        i += 1
    }
    qsort(sorted as *mut libc::c_void, (*vm).numSymbols as size_t,
          ::std::mem::size_of::<*mut vmSymbol_t>() as libc::c_ulong,
          Some(VM_ProfileSort));
    i = 0i32;
    while i < (*vm).numSymbols {
        let mut perc: libc::c_int = 0;
        sym = *sorted.offset(i as isize);
        perc =
            ((100i32 as libc::c_float * (*sym).profileCount as libc::c_float)
                 as libc::c_double / total) as libc::c_int;
        Com_Printf(b"%2i%% %9i %s\n\x00" as *const u8 as *const libc::c_char,
                   perc, (*sym).profileCount, (*sym).symName.as_mut_ptr());
        (*sym).profileCount = 0i32;
        i += 1
    }
    Com_Printf(b"    %9.0f total\n\x00" as *const u8 as *const libc::c_char,
               total);
    Z_Free(sorted as *mut libc::c_void);
}
//=================================================================
unsafe extern "C" fn VM_ProfileSort(mut a: *const libc::c_void,
                                    mut b: *const libc::c_void)
 -> libc::c_int {
    let mut sa: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    let mut sb: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    sa = *(a as *mut *mut vmSymbol_t);
    sb = *(b as *mut *mut vmSymbol_t);
    if (*sa).profileCount < (*sb).profileCount { return -1i32 }
    if (*sa).profileCount > (*sb).profileCount { return 1i32 }
    return 0i32;
}
#[no_mangle]
pub static mut lastVM: *mut vm_t = 0 as *const vm_t as *mut vm_t;
#[no_mangle]
pub unsafe extern "C" fn VM_Create(mut module: *const libc::c_char,
                                   mut systemCalls:
                                       Option<unsafe extern "C" fn(_:
                                                                       *mut intptr_t)
                                                  -> intptr_t>,
                                   mut interpret: vmInterpret_t)
 -> *mut vm_t {
    let mut vm: *mut vm_t = 0 as *mut vm_t;
    let mut header: *mut vmHeader_t = 0 as *mut vmHeader_t;
    let mut i: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut retval: libc::c_int = 0;
    let mut filename: [libc::c_char; 4096] = [0; 4096];
    let mut startSearch: *mut libc::c_void = 0 as *mut libc::c_void;
    if module.is_null() || 0 == *module.offset(0isize) ||
           systemCalls.is_none() {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"VM_Create: bad parms\x00" as *const u8 as
                      *const libc::c_char);
    }
    remaining = Hunk_MemoryRemaining();
    i = 0i32;
    while i < 3i32 {
        if 0 == Q_stricmp(vmTable[i as usize].name.as_mut_ptr(), module) {
            vm = &mut *vmTable.as_mut_ptr().offset(i as isize) as *mut vm_t;
            return vm
        }
        i += 1
    }
    i = 0i32;
    while i < 3i32 {
        if 0 == vmTable[i as usize].name[0usize] { break ; }
        i += 1
    }
    if i == 3i32 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"VM_Create: no free vm_t\x00" as *const u8 as
                      *const libc::c_char);
    }
    vm = &mut *vmTable.as_mut_ptr().offset(i as isize) as *mut vm_t;
    Q_strncpyz((*vm).name.as_mut_ptr(), module,
               ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as
                   libc::c_int);
    loop  {
        retval =
            FS_FindVM(&mut startSearch, filename.as_mut_ptr(),
                      ::std::mem::size_of::<[libc::c_char; 4096]>() as
                          libc::c_ulong as libc::c_int, module,
                      (interpret as libc::c_uint ==
                           VMI_NATIVE as libc::c_int as libc::c_uint) as
                          libc::c_int);
        if retval == VMI_NATIVE as libc::c_int {
            Com_Printf(b"Try loading dll file %s\n\x00" as *const u8 as
                           *const libc::c_char, filename.as_mut_ptr());
            (*vm).dllHandle =
                Sys_LoadGameDll(filename.as_mut_ptr(), &mut (*vm).entryPoint,
                                Some(VM_DllSyscall));
            if !(*vm).dllHandle.is_null() {
                (*vm).systemCall = systemCalls;
                return vm
            }
            Com_Printf(b"Failed loading dll, trying next\n\x00" as *const u8
                           as *const libc::c_char);
        } else if retval == VMI_COMPILED as libc::c_int {
            (*vm).searchPath = startSearch;
            header = VM_LoadQVM(vm, qtrue, qfalse);
            if !header.is_null() { break ; }
            Q_strncpyz((*vm).name.as_mut_ptr(), module,
                       ::std::mem::size_of::<[libc::c_char; 64]>() as
                           libc::c_ulong as libc::c_int);
        }
        if !(retval >= 0i32) { break ; }
    }
    if retval < 0i32 { return 0 as *mut vm_t }
    (*vm).systemCall = systemCalls;
    (*vm).instructionCount = (*header).instructionCount;
    (*vm).instructionPointers =
        Hunk_AllocDebug(((*vm).instructionCount as
                             libc::c_ulong).wrapping_mul(::std::mem::size_of::<intptr_t>()
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"vm->instructionCount * sizeof(*vm->instructionPointers)\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        b"code/qcommon/vm.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 592i32)
            as *mut intptr_t;
    (*vm).codeLength = (*header).codeLength;
    (*vm).compiled = qfalse;
    if interpret as libc::c_uint !=
           VMI_BYTECODE as libc::c_int as libc::c_uint {
        (*vm).compiled = qtrue;
        VM_Compile(vm, header);
    }
    if 0 == (*vm).compiled as u64 { VM_PrepareInterpreter(vm, header); }
    FS_FreeFile(header as *mut libc::c_void);
    VM_LoadSymbols(vm);
    (*vm).programStack = (*vm).dataMask + 1i32;
    (*vm).stackBottom = (*vm).programStack - 0x10000i32;
    Com_Printf(b"%s loaded in %d bytes on the hunk\n\x00" as *const u8 as
                   *const libc::c_char, module,
               remaining - Hunk_MemoryRemaining());
    return vm;
}
/*
===============
VM_LoadSymbols
===============
*/
#[no_mangle]
pub unsafe extern "C" fn VM_LoadSymbols(mut vm: *mut vm_t) {
    let mut mapfile: unnamed_0 = unnamed_0{c: 0 as *mut libc::c_char,};
    let mut text_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut symbols: [libc::c_char; 64] = [0; 64];
    let mut prev: *mut *mut vmSymbol_t = 0 as *mut *mut vmSymbol_t;
    let mut sym: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    let mut count: libc::c_int = 0;
    let mut value: libc::c_int = 0;
    let mut chars: libc::c_int = 0;
    let mut segment: libc::c_int = 0;
    let mut numInstructions: libc::c_int = 0;
    if 0 == (*com_developer).integer { return }
    COM_StripExtension((*vm).name.as_mut_ptr(), name.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 64]>() as
                           libc::c_ulong as libc::c_int);
    Com_sprintf(symbols.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"vm/%s.map\x00" as *const u8 as *const libc::c_char,
                name.as_mut_ptr());
    FS_ReadFile(symbols.as_mut_ptr(), &mut mapfile.v);
    if mapfile.c.is_null() {
        Com_Printf(b"Couldn\'t load symbol file: %s\n\x00" as *const u8 as
                       *const libc::c_char, symbols.as_mut_ptr());
        return
    }
    numInstructions = (*vm).instructionCount;
    text_p = mapfile.c;
    prev = &mut (*vm).symbols;
    count = 0i32;
    loop  {
        token = COM_Parse(&mut text_p);
        if 0 == *token.offset(0isize) { break ; }
        segment = ParseHex(token);
        if 0 != segment {
            COM_Parse(&mut text_p);
            COM_Parse(&mut text_p);
        } else {
            // only load code segment values
            token = COM_Parse(&mut text_p);
            if 0 == *token.offset(0isize) {
                Com_Printf(b"WARNING: incomplete line at end of file\n\x00" as
                               *const u8 as *const libc::c_char);
                break ;
            } else {
                value = ParseHex(token);
                token = COM_Parse(&mut text_p);
                if 0 == *token.offset(0isize) {
                    Com_Printf(b"WARNING: incomplete line at end of file\n\x00"
                                   as *const u8 as *const libc::c_char);
                    break ;
                } else {
                    chars = strlen(token) as libc::c_int;
                    sym =
                        Hunk_AllocDebug((::std::mem::size_of::<vmSymbol_t>()
                                             as
                                             libc::c_ulong).wrapping_add(chars
                                                                             as
                                                                             libc::c_ulong)
                                            as libc::c_int, h_high,
                                        b"sizeof( *sym ) + chars\x00" as
                                            *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        b"code/qcommon/vm.c\x00" as *const u8
                                            as *const libc::c_char as
                                            *mut libc::c_char, 280i32) as
                            *mut vmSymbol_t;
                    *prev = sym;
                    prev = &mut (*sym).next;
                    (*sym).next = 0 as *mut vmSymbol_s;
                    if value >= 0i32 && value < numInstructions {
                        value =
                            *(*vm).instructionPointers.offset(value as isize)
                                as libc::c_int
                    }
                    (*sym).symValue = value;
                    Q_strncpyz((*sym).symName.as_mut_ptr(), token,
                               chars + 1i32);
                    count += 1
                }
            }
        }
    }
    (*vm).numSymbols = count;
    Com_Printf(b"%i symbols parsed from %s\n\x00" as *const u8 as
                   *const libc::c_char, count, symbols.as_mut_ptr());
    FS_FreeFile(mapfile.v);
}
/*
=====================
VM_SymbolForCompiledPointer
=====================
*/
// 64bit!
/*
===============
ParseHex
===============
*/
#[no_mangle]
pub unsafe extern "C" fn ParseHex(mut text: *const libc::c_char)
 -> libc::c_int {
    let mut value: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    value = 0i32;
    loop  {
        let fresh2 = text;
        text = text.offset(1);
        c = *fresh2 as libc::c_int;
        if !(c != 0i32) { break ; }
        if c >= '0' as i32 && c <= '9' as i32 {
            value = value * 16i32 + c - '0' as i32
        } else if c >= 'a' as i32 && c <= 'f' as i32 {
            value = value * 16i32 + 10i32 + c - 'a' as i32
        } else {
            if !(c >= 'A' as i32 && c <= 'F' as i32) { continue ; }
            value = value * 16i32 + 10i32 + c - 'A' as i32
        }
    }
    return value;
}
/*
=================
VM_LoadQVM

Load a .qvm file
=================
*/
#[no_mangle]
pub unsafe extern "C" fn VM_LoadQVM(mut vm: *mut vm_t, mut alloc: qboolean,
                                    mut unpure: qboolean) -> *mut vmHeader_t {
    let mut dataLength: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut filename: [libc::c_char; 64] = [0; 64];
    let mut header: unnamed_1 = unnamed_1{h: 0 as *mut vmHeader_t,};
    Com_sprintf(filename.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                    as libc::c_int,
                b"vm/%s.qvm\x00" as *const u8 as *const libc::c_char,
                (*vm).name.as_mut_ptr());
    Com_Printf(b"Loading vm file %s...\n\x00" as *const u8 as
                   *const libc::c_char, filename.as_mut_ptr());
    FS_ReadFileDir(filename.as_mut_ptr(), (*vm).searchPath, unpure,
                   &mut header.v);
    if header.h.is_null() {
        Com_Printf(b"Failed.\n\x00" as *const u8 as *const libc::c_char);
        VM_Free(vm);
        Com_Printf(b"^3Warning: Couldn\'t open VM file %s\n\x00" as *const u8
                       as *const libc::c_char, filename.as_mut_ptr());
        return 0 as *mut vmHeader_t
    }
    FS_Which(filename.as_mut_ptr(), (*vm).searchPath);
    if (*header.h).vmMagic == 0x12721445i32 {
        Com_Printf(b"...which has vmMagic VM_MAGIC_VER2\n\x00" as *const u8 as
                       *const libc::c_char);
        i = 0i32;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<vmHeader_t>() as
                       libc::c_ulong).wrapping_div(4i32 as libc::c_ulong) {
            *(header.h as *mut libc::c_int).offset(i as isize) =
                *(header.h as *mut libc::c_int).offset(i as isize);
            i += 1
        }
        if (*header.h).jtrgLength < 0i32 || (*header.h).bssLength < 0i32 ||
               (*header.h).dataLength < 0i32 || (*header.h).litLength < 0i32
               || (*header.h).codeLength <= 0i32 {
            VM_Free(vm);
            FS_FreeFile(header.v);
            Com_Printf(b"^3Warning: %s has bad header\n\x00" as *const u8 as
                           *const libc::c_char, filename.as_mut_ptr());
            return 0 as *mut vmHeader_t
        }
    } else if (*header.h).vmMagic == 0x12721444i32 {
        i = 0i32;
        while (i as libc::c_ulong) <
                  (::std::mem::size_of::<vmHeader_t>() as
                       libc::c_ulong).wrapping_sub(::std::mem::size_of::<libc::c_int>()
                                                       as
                                                       libc::c_ulong).wrapping_div(4i32
                                                                                       as
                                                                                       libc::c_ulong)
              {
            *(header.h as *mut libc::c_int).offset(i as isize) =
                *(header.h as *mut libc::c_int).offset(i as isize);
            i += 1
        }
        if (*header.h).bssLength < 0i32 || (*header.h).dataLength < 0i32 ||
               (*header.h).litLength < 0i32 || (*header.h).codeLength <= 0i32
           {
            VM_Free(vm);
            FS_FreeFile(header.v);
            Com_Printf(b"^3Warning: %s has bad header\n\x00" as *const u8 as
                           *const libc::c_char, filename.as_mut_ptr());
            return 0 as *mut vmHeader_t
        }
    } else {
        VM_Free(vm);
        FS_FreeFile(header.v);
        Com_Printf(b"^3Warning: %s does not have a recognisable magic number in its header\n\x00"
                       as *const u8 as *const libc::c_char,
                   filename.as_mut_ptr());
        return 0 as *mut vmHeader_t
    }
    dataLength =
        (*header.h).dataLength + (*header.h).litLength +
            (*header.h).bssLength;
    i = 0i32;
    while dataLength > 1i32 << i { i += 1 }
    dataLength = 1i32 << i;
    if 0 != alloc as u64 {
        (*vm).dataAlloc = dataLength + 4i32;
        (*vm).dataBase =
            Hunk_AllocDebug((*vm).dataAlloc, h_high,
                            b"vm->dataAlloc\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            b"code/qcommon/vm.c\x00" as *const u8 as
                                *const libc::c_char as *mut libc::c_char,
                            399i32) as *mut byte;
        (*vm).dataMask = dataLength - 1i32
    } else {
        if (*vm).dataAlloc != dataLength + 4i32 {
            VM_Free(vm);
            FS_FreeFile(header.v);
            Com_Printf(b"^3Warning: Data region size of %s not matching after VM_Restart()\n\x00"
                           as *const u8 as *const libc::c_char,
                       filename.as_mut_ptr());
            return 0 as *mut vmHeader_t
        }
        memset((*vm).dataBase as *mut libc::c_void, 0i32,
               (*vm).dataAlloc as libc::c_ulong);
    }
    memcpy((*vm).dataBase as *mut libc::c_void,
           (header.h as *mut byte).offset((*header.h).dataOffset as isize) as
               *const libc::c_void,
           ((*header.h).dataLength + (*header.h).litLength) as libc::c_ulong);
    i = 0i32;
    while i < (*header.h).dataLength {
        *((*vm).dataBase.offset(i as isize) as *mut libc::c_int) =
            *((*vm).dataBase.offset(i as isize) as *mut libc::c_int);
        i += 4i32
    }
    if (*header.h).vmMagic == 0x12721445i32 {
        let mut previousNumJumpTableTargets: libc::c_int =
            (*vm).numJumpTableTargets;
        (*header.h).jtrgLength &= !0x3i32;
        (*vm).numJumpTableTargets = (*header.h).jtrgLength >> 2i32;
        Com_Printf(b"Loading %d jump table targets\n\x00" as *const u8 as
                       *const libc::c_char, (*vm).numJumpTableTargets);
        if 0 != alloc as u64 {
            (*vm).jumpTableTargets =
                Hunk_AllocDebug((*header.h).jtrgLength, h_high,
                                b"header.h->jtrgLength\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                b"code/qcommon/vm.c\x00" as *const u8 as
                                    *const libc::c_char as *mut libc::c_char,
                                438i32) as *mut byte
        } else {
            if (*vm).numJumpTableTargets != previousNumJumpTableTargets {
                VM_Free(vm);
                FS_FreeFile(header.v);
                Com_Printf(b"^3Warning: Jump table size of %s not matching after VM_Restart()\n\x00"
                               as *const u8 as *const libc::c_char,
                           filename.as_mut_ptr());
                return 0 as *mut vmHeader_t
            }
            memset((*vm).jumpTableTargets as *mut libc::c_void, 0i32,
                   (*header.h).jtrgLength as libc::c_ulong);
        }
        memcpy((*vm).jumpTableTargets as *mut libc::c_void,
               (header.h as
                    *mut byte).offset((*header.h).dataOffset as
                                          isize).offset((*header.h).dataLength
                                                            as
                                                            isize).offset((*header.h).litLength
                                                                              as
                                                                              isize)
                   as *const libc::c_void,
               (*header.h).jtrgLength as libc::c_ulong);
        i = 0i32;
        while i < (*header.h).jtrgLength {
            *((*vm).jumpTableTargets.offset(i as isize) as *mut libc::c_int) =
                *((*vm).jumpTableTargets.offset(i as isize) as
                      *mut libc::c_int);
            i += 4i32
        }
    }
    return header.h;
}
// module should be bare: "cgame", not "cgame.dll" or "vm/cgame.qvm"
#[no_mangle]
pub unsafe extern "C" fn VM_Free(mut vm: *mut vm_t) {
    if vm.is_null() { return }
    if 0 != (*vm).callLevel {
        if 0 == forced_unload {
            Com_Error(ERR_FATAL as libc::c_int,
                      b"VM_Free(%s) on running vm\x00" as *const u8 as
                          *const libc::c_char, (*vm).name.as_mut_ptr());
        } else {
            Com_Printf(b"forcefully unloading %s vm\n\x00" as *const u8 as
                           *const libc::c_char, (*vm).name.as_mut_ptr());
        }
    }
    if (*vm).destroy.is_some() {
        (*vm).destroy.expect("non-null function pointer")(vm);
    }
    if !(*vm).dllHandle.is_null() {
        Sys_UnloadDll((*vm).dllHandle);
        memset(vm as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<vm_t>() as libc::c_ulong);
    }
    memset(vm as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<vm_t>() as libc::c_ulong);
    currentVM = 0 as *mut vm_t;
    lastVM = 0 as *mut vm_t;
}
#[no_mangle]
pub static mut currentVM: *mut vm_t = 0 as *const vm_t as *mut vm_t;
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
// vm.c -- virtual machine
/*


intermix code and data
symbol table

a dll has one imported function: VM_SystemCall
and one exported function: Perform


*/
// used by Com_Error to get rid of running vm's before longjmp
static mut forced_unload: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn VM_Clear() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        VM_Free(&mut *vmTable.as_mut_ptr().offset(i as isize));
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn VM_Forced_Unload_Start() { forced_unload = 1i32; }
#[no_mangle]
pub unsafe extern "C" fn VM_Forced_Unload_Done() { forced_unload = 0i32; }
#[no_mangle]
pub unsafe extern "C" fn VM_Restart(mut vm: *mut vm_t, mut unpure: qboolean)
 -> *mut vm_t {
    let mut header: *mut vmHeader_t = 0 as *mut vmHeader_t;
    if !(*vm).dllHandle.is_null() {
        let mut name: [libc::c_char; 64] = [0; 64];
        let mut systemCall:
                Option<unsafe extern "C" fn(_: *mut intptr_t) -> intptr_t> =
            None;
        systemCall = (*vm).systemCall;
        Q_strncpyz(name.as_mut_ptr(), (*vm).name.as_mut_ptr(),
                   ::std::mem::size_of::<[libc::c_char; 64]>() as
                       libc::c_ulong as libc::c_int);
        VM_Free(vm);
        vm = VM_Create(name.as_mut_ptr(), systemCall, VMI_NATIVE);
        return vm
    }
    Com_Printf(b"VM_Restart()\n\x00" as *const u8 as *const libc::c_char);
    header = VM_LoadQVM(vm, qfalse, unpure);
    if header.is_null() {
        Com_Error(ERR_DROP as libc::c_int,
                  b"VM_Restart failed\x00" as *const u8 as
                      *const libc::c_char);
    }
    FS_FreeFile(header as *mut libc::c_void);
    return vm;
}
#[no_mangle]
pub unsafe extern "C" fn VM_Debug(mut level: libc::c_int) {
    vm_debugLevel = level;
}
#[no_mangle]
pub static mut vm_debugLevel: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn VM_ArgPtr(mut intValue: intptr_t)
 -> *mut libc::c_void {
    if 0 == intValue { return 0 as *mut libc::c_void }
    if currentVM.is_null() { return 0 as *mut libc::c_void }
    if (*currentVM).entryPoint.is_some() {
        return (*currentVM).dataBase.offset(intValue as isize) as
                   *mut libc::c_void
    } else {
        return (*currentVM).dataBase.offset((intValue &
                                                 (*currentVM).dataMask as
                                                     libc::c_long) as isize)
                   as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn VM_ExplicitArgPtr(mut vm: *mut vm_t,
                                           mut intValue: intptr_t)
 -> *mut libc::c_void {
    if 0 == intValue { return 0 as *mut libc::c_void }
    if currentVM.is_null() { return 0 as *mut libc::c_void }
    if (*vm).entryPoint.is_some() {
        return (*vm).dataBase.offset(intValue as isize) as *mut libc::c_void
    } else {
        return (*vm).dataBase.offset((intValue &
                                          (*vm).dataMask as libc::c_long) as
                                         isize) as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn VM_ValueToFunctionSymbol(mut vm: *mut vm_t,
                                                  mut value: libc::c_int)
 -> *mut vmSymbol_t {
    let mut sym: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    static mut nullSym: vmSymbol_t =
        vmSymbol_s{next: 0 as *const vmSymbol_s as *mut vmSymbol_s,
                   symValue: 0,
                   profileCount: 0,
                   symName: [],};
    sym = (*vm).symbols;
    if sym.is_null() { return &mut nullSym }
    while !(*sym).next.is_null() && (*(*sym).next).symValue <= value {
        sym = (*sym).next
    }
    return sym;
}
#[no_mangle]
pub unsafe extern "C" fn VM_SymbolToValue(mut vm: *mut vm_t,
                                          mut symbol: *const libc::c_char)
 -> libc::c_int {
    let mut sym: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    sym = (*vm).symbols;
    while !sym.is_null() {
        if 0 == strcmp(symbol, (*sym).symName.as_mut_ptr()) {
            return (*sym).symValue
        }
        sym = (*sym).next
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn VM_ValueToSymbol(mut vm: *mut vm_t,
                                          mut value: libc::c_int)
 -> *const libc::c_char {
    let mut sym: *mut vmSymbol_t = 0 as *mut vmSymbol_t;
    static mut text: [libc::c_char; 1024] = [0; 1024];
    sym = (*vm).symbols;
    if sym.is_null() {
        return b"NO SYMBOLS\x00" as *const u8 as *const libc::c_char
    }
    while !(*sym).next.is_null() && (*(*sym).next).symValue <= value {
        sym = (*sym).next
    }
    if value == (*sym).symValue { return (*sym).symName.as_mut_ptr() }
    Com_sprintf(text.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"%s+%i\x00" as *const u8 as *const libc::c_char,
                (*sym).symName.as_mut_ptr(), value - (*sym).symValue);
    return text.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn VM_LogSyscalls(mut args: *mut libc::c_int) {
    static mut callnum: libc::c_int = 0;
    static mut f: *mut FILE = 0 as *const FILE as *mut FILE;
    if f.is_null() {
        f =
            fopen(b"syscalls.log\x00" as *const u8 as *const libc::c_char,
                  b"w\x00" as *const u8 as *const libc::c_char)
    }
    callnum += 1;
    fprintf(f,
            b"%i: %p (%i) = %i %i %i %i\n\x00" as *const u8 as
                *const libc::c_char, callnum,
            args.wrapping_offset_from((*currentVM).dataBase as
                                          *mut libc::c_int) as libc::c_long as
                *mut libc::c_void, *args.offset(0isize), *args.offset(1isize),
            *args.offset(2isize), *args.offset(3isize), *args.offset(4isize));
}
#[no_mangle]
pub unsafe extern "C" fn VM_BlockCopy(mut dest: libc::c_uint,
                                      mut src: libc::c_uint, mut n: size_t) {
    let mut dataMask: libc::c_uint = (*currentVM).dataMask as libc::c_uint;
    if dest & dataMask != dest || src & dataMask != src ||
           (dest as libc::c_ulong).wrapping_add(n) & dataMask as libc::c_ulong
               != (dest as libc::c_ulong).wrapping_add(n) ||
           (src as libc::c_ulong).wrapping_add(n) & dataMask as libc::c_ulong
               != (src as libc::c_ulong).wrapping_add(n) {
        Com_Error(ERR_DROP as libc::c_int,
                  b"OP_BLOCK_COPY out of range!\x00" as *const u8 as
                      *const libc::c_char);
    }
    memcpy((*currentVM).dataBase.offset(dest as isize) as *mut libc::c_void,
           (*currentVM).dataBase.offset(src as isize) as *const libc::c_void,
           n);
}