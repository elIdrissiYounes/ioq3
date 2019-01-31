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
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    use super::botlib_h::{botlib_import_t};
    extern "C" {
        #[no_mangle]
        pub static mut botimport: botlib_import_t;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_memory.c"]
pub mod l_memory_c {
    use super::{libc};
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t};
use self::botlib_h::{bsp_surface_s, bsp_surface_t, bsp_trace_s, bsp_trace_t,
                     botlib_import_s, botlib_import_t};
use self::string_h::{memset};
use self::be_interface_h::{botimport};
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
 * name:		l_memory.h
 *
 * desc:		memory management
 *
 * $Archive: /source/code/botlib/l_memory.h $
 *
 *****************************************************************************/
//#define MEMDEBUG
//allocate a memory block of the given size
#[no_mangle]
pub unsafe extern "C" fn GetMemory(mut size: libc::c_ulong)
 -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut memid: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    ptr =
        botimport.GetMemory.expect("non-null function pointer")(size.wrapping_add(::std::mem::size_of::<libc::c_ulong>()
                                                                                      as
                                                                                      libc::c_ulong)
                                                                    as
                                                                    libc::c_int);
    if ptr.is_null() { return 0 as *mut libc::c_void }
    memid = ptr as *mut libc::c_ulong;
    *memid = 0x12345678i64 as libc::c_ulong;
    return (ptr as
                *mut libc::c_char).offset(::std::mem::size_of::<libc::c_ulong>()
                                              as libc::c_ulong as isize) as
               *mut libc::c_ulong as *mut libc::c_void;
}
//allocate a memory block of the given size and clear it
#[no_mangle]
pub unsafe extern "C" fn GetClearedMemory(mut size: libc::c_ulong)
 -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = GetMemory(size);
    memset(ptr, 0i32, size);
    return ptr;
}
//
//allocate a memory block of the given size
#[no_mangle]
pub unsafe extern "C" fn GetHunkMemory(mut size: libc::c_ulong)
 -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut memid: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    ptr =
        botimport.HunkAlloc.expect("non-null function pointer")(size.wrapping_add(::std::mem::size_of::<libc::c_ulong>()
                                                                                      as
                                                                                      libc::c_ulong)
                                                                    as
                                                                    libc::c_int);
    if ptr.is_null() { return 0 as *mut libc::c_void }
    memid = ptr as *mut libc::c_ulong;
    *memid = 0x87654321i64 as libc::c_ulong;
    return (ptr as
                *mut libc::c_char).offset(::std::mem::size_of::<libc::c_ulong>()
                                              as libc::c_ulong as isize) as
               *mut libc::c_ulong as *mut libc::c_void;
}
//allocate a memory block of the given size and clear it
#[no_mangle]
pub unsafe extern "C" fn GetClearedHunkMemory(mut size: libc::c_ulong)
 -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = GetHunkMemory(size);
    memset(ptr, 0i32, size);
    return ptr;
}
//free the given memory block
#[no_mangle]
pub unsafe extern "C" fn FreeMemory(mut ptr: *mut libc::c_void) {
    let mut memid: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    memid =
        (ptr as
             *mut libc::c_char).offset(-(::std::mem::size_of::<libc::c_ulong>()
                                             as libc::c_ulong as isize)) as
            *mut libc::c_ulong;
    if *memid == 0x12345678i64 as libc::c_ulong {
        botimport.FreeMemory.expect("non-null function pointer")(memid as
                                                                     *mut libc::c_void);
    };
}
//returns the amount available memory
#[no_mangle]
pub unsafe extern "C" fn AvailableMemory() -> libc::c_int {
    return botimport.AvailableMemory.expect("non-null function pointer")();
}
//prints the total used memory size
#[no_mangle]
pub unsafe extern "C" fn PrintUsedMemorySize() { }
//print all memory blocks with label
#[no_mangle]
pub unsafe extern "C" fn PrintMemoryLabels() { }
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
 * name:		l_memory.c
 *
 * desc:		memory allocation
 *
 * $Archive: /MissionPack/code/botlib/l_memory.c $
 *
 *****************************************************************************/
//#define MEMDEBUG
//#define MEMORYMANEGER
#[no_mangle]
pub static mut allocatedmemory: libc::c_int = 0;
#[no_mangle]
pub static mut totalmemorysize: libc::c_int = 0;
#[no_mangle]
pub static mut numblocks: libc::c_int = 0;