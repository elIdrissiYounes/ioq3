use libc;
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    use super::{libc};
    extern "C" {
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char)
         -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
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
 * name:		l_libvar.h
 *
 * desc:		botlib vars
 *
 * $Archive: /source/code/botlib/l_libvar.h $
 *
 *****************************************************************************/
    //library variable
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct libvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub value: libc::c_float,
        pub next: *mut libvar_s,
    }
    pub type libvar_t = libvar_s;
    use super::{libc};
    use super::q_shared_h::{qboolean};
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
    extern "C" {
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
        pub fn GetMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_libvar.c"]
pub mod l_libvar_c {
    use super::l_libvar_h::{libvar_t};
    use super::{libc};
}
use self::q_shared_h::{qboolean, qtrue, qfalse, Q_stricmp};
use self::l_libvar_h::{libvar_s, libvar_t};
use self::string_h::{memset, strcpy, strlen};
use self::l_memory_h::{GetMemory, FreeMemory};
//removes all library variables
#[no_mangle]
pub unsafe extern "C" fn LibVarDeAllocAll() {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v = libvarlist;
    while !v.is_null() {
        libvarlist = (*libvarlist).next;
        LibVarDeAlloc(v);
        v = libvarlist
    }
    libvarlist = 0 as *mut libvar_t;
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
/* ****************************************************************************
 * name:		l_libvar.c
 *
 * desc:		bot library variables
 *
 * $Archive: /MissionPack/code/botlib/l_libvar.c $
 *
 *****************************************************************************/
//list with library variables
#[no_mangle]
pub static mut libvarlist: *mut libvar_t =
    0 as *const libvar_t as *mut libvar_t;
//end of the function LibVarAlloc
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn LibVarDeAlloc(mut v: *mut libvar_t) {
    if !(*v).string.is_null() {
        FreeMemory((*v).string as *mut libc::c_void);
    }
    FreeMemory((*v).name as *mut libc::c_void);
    FreeMemory(v as *mut libc::c_void);
}
//gets the library variable with the given name
#[no_mangle]
pub unsafe extern "C" fn LibVarGet(mut var_name: *const libc::c_char)
 -> *mut libvar_t {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v = libvarlist;
    while !v.is_null() {
        if 0 == Q_stricmp((*v).name, var_name) { return v }
        v = (*v).next
    }
    return 0 as *mut libvar_t;
}
//gets the string of the library variable with the given name
#[no_mangle]
pub unsafe extern "C" fn LibVarGetString(mut var_name: *const libc::c_char)
 -> *mut libc::c_char {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v = LibVarGet(var_name);
    if !v.is_null() {
        return (*v).string
    } else {
        return b"\x00" as *const u8 as *const libc::c_char as
                   *mut libc::c_char
    };
}
//gets the value of the library variable with the given name
#[no_mangle]
pub unsafe extern "C" fn LibVarGetValue(mut var_name: *const libc::c_char)
 -> libc::c_float {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v = LibVarGet(var_name);
    if !v.is_null() {
        return (*v).value
    } else { return 0i32 as libc::c_float };
}
//creates the library variable if not existing already and returns it
#[no_mangle]
pub unsafe extern "C" fn LibVar(mut var_name: *const libc::c_char,
                                mut value: *const libc::c_char)
 -> *mut libvar_t {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v = LibVarGet(var_name);
    if !v.is_null() { return v }
    v = LibVarAlloc(var_name);
    (*v).string =
        GetMemory(strlen(value).wrapping_add(1i32 as libc::c_ulong)) as
            *mut libc::c_char;
    strcpy((*v).string, value);
    (*v).value = LibVarStringValue((*v).string);
    (*v).modified = qtrue;
    return v;
}
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn LibVarStringValue(mut string: *const libc::c_char)
 -> libc::c_float {
    let mut dotfound: libc::c_int = 0i32;
    let mut value: libc::c_float = 0i32 as libc::c_float;
    while 0 != *string {
        if (*string as libc::c_int) < '0' as i32 ||
               *string as libc::c_int > '9' as i32 {
            if 0 != dotfound || *string as libc::c_int != '.' as i32 {
                return 0i32 as libc::c_float
            } else { dotfound = 10i32; string = string.offset(1isize) }
        }
        if 0 != dotfound {
            value =
                value +
                    (*string as libc::c_int - '0' as i32) as libc::c_float /
                        dotfound as libc::c_float;
            dotfound *= 10i32
        } else {
            value =
                (value as libc::c_double * 10.0f64 +
                     (*string as libc::c_int - '0' as i32) as libc::c_float as
                         libc::c_double) as libc::c_float
        }
        string = string.offset(1isize)
    }
    return value;
}
//end of the function LibVarStringValue
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn LibVarAlloc(mut var_name: *const libc::c_char)
 -> *mut libvar_t {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v =
        GetMemory(::std::mem::size_of::<libvar_t>() as libc::c_ulong) as
            *mut libvar_t;
    memset(v as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<libvar_t>() as libc::c_ulong);
    (*v).name =
        GetMemory(strlen(var_name).wrapping_add(1i32 as libc::c_ulong)) as
            *mut libc::c_char;
    strcpy((*v).name, var_name);
    (*v).next = libvarlist;
    libvarlist = v;
    return v;
}
//creates the library variable if not existing already and returns the value
#[no_mangle]
pub unsafe extern "C" fn LibVarValue(mut var_name: *const libc::c_char,
                                     mut value: *const libc::c_char)
 -> libc::c_float {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v = LibVar(var_name, value);
    return (*v).value;
}
//creates the library variable if not existing already and returns the value string
#[no_mangle]
pub unsafe extern "C" fn LibVarString(mut var_name: *const libc::c_char,
                                      mut value: *const libc::c_char)
 -> *mut libc::c_char {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v = LibVar(var_name, value);
    return (*v).string;
}
//sets the library variable
#[no_mangle]
pub unsafe extern "C" fn LibVarSet(mut var_name: *const libc::c_char,
                                   mut value: *const libc::c_char) {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v = LibVarGet(var_name);
    if !v.is_null() {
        FreeMemory((*v).string as *mut libc::c_void);
    } else { v = LibVarAlloc(var_name) }
    (*v).string =
        GetMemory(strlen(value).wrapping_add(1i32 as libc::c_ulong)) as
            *mut libc::c_char;
    strcpy((*v).string, value);
    (*v).value = LibVarStringValue((*v).string);
    (*v).modified = qtrue;
}
//returns true if the library variable has been modified
#[no_mangle]
pub unsafe extern "C" fn LibVarChanged(mut var_name: *const libc::c_char)
 -> qboolean {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v = LibVarGet(var_name);
    if !v.is_null() { return (*v).modified } else { return qfalse };
}
//sets the library variable to unmodified
#[no_mangle]
pub unsafe extern "C" fn LibVarSetNotModified(mut var_name:
                                                  *const libc::c_char) {
    let mut v: *mut libvar_t = 0 as *mut libvar_t;
    v = LibVarGet(var_name);
    if !v.is_null() { (*v).modified = qfalse };
}