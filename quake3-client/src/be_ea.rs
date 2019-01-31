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
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/botlib.h"]
pub mod botlib_h {
    //debug line colors
    //0xf2f2f0f0L
    //0xd0d1d2d3L
    //0xf3f3f1f1L
    //0xdcdddedfL
    //0xe0e1e2e3L
    //Print types
    //console message types
    //botlib error codes
    //no error
    //library not setup
    //invalid entity number
    //no AAS file available
    //cannot open AAS file
    //incorrect AAS file id
    //incorrect AAS file version
    //cannot read AAS file lump
    //cannot load initial chats
    //cannot load item weights
    //cannot load item config
    //cannot load weapon weights
    //cannot load weapon config
    //action flags
    //the bot input, will be converted to a usercmd_t
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_input_s {
        pub thinktime: libc::c_float,
        pub dir: vec3_t,
        pub speed: libc::c_float,
        pub viewangles: vec3_t,
        pub actionflags: libc::c_int,
        pub weapon: libc::c_int,
    }
    pub type bot_input_t = bot_input_s;
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
    use super::q_shared_h::{vec3_t, qboolean, cplane_t, vec_t, fileHandle_t,
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
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_memory.h"]
pub mod l_memory_h {
    use super::{libc};
    extern "C" {
        //allocate a memory block of the given size and clear it
        #[no_mangle]
        pub fn GetClearedHunkMemory(size: libc::c_ulong) -> *mut libc::c_void;
        //free the given memory block
        #[no_mangle]
        pub fn FreeMemory(ptr: *mut libc::c_void);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ea.h"]
pub mod be_ea_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    use super::botlib_h::{bot_input_t};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ea.c"]
pub mod be_ea_c {
    use super::botlib_h::{bot_input_t};
    use super::{libc};
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cplane_t, va};
use self::botlib_h::{bot_input_s, bot_input_t, bsp_surface_s, bsp_surface_t,
                     bsp_trace_s, bsp_trace_t, botlib_import_s,
                     botlib_import_t};
use self::be_interface_h::{botlib_globals_s, botlib_globals_t, botlibglobals,
                           botimport};
use self::string_h::{memcpy};
use self::l_memory_h::{GetClearedHunkMemory, FreeMemory};
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
/* ****************************************************************************
 * name:		be_ea.h
 *
 * desc:		elementary actions
 *
 * $Archive: /source/code/botlib/be_ea.h $
 *
 *****************************************************************************/
//ClientCommand elementary actions
#[no_mangle]
pub unsafe extern "C" fn EA_Say(mut client: libc::c_int,
                                mut str: *mut libc::c_char) {
    botimport.BotClientCommand.expect("non-null function pointer")(client,
                                                                   va(b"say %s\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                      str));
}
#[no_mangle]
pub unsafe extern "C" fn EA_SayTeam(mut client: libc::c_int,
                                    mut str: *mut libc::c_char) {
    botimport.BotClientCommand.expect("non-null function pointer")(client,
                                                                   va(b"say_team %s\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                      str));
}
#[no_mangle]
pub unsafe extern "C" fn EA_Command(mut client: libc::c_int,
                                    mut command: *mut libc::c_char) {
    botimport.BotClientCommand.expect("non-null function pointer")(client,
                                                                   command);
}
#[no_mangle]
pub unsafe extern "C" fn EA_Action(mut client: libc::c_int,
                                   mut action: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= action;
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
 * name:		be_ea.c
 *
 * desc:		elementary actions
 *
 * $Archive: /MissionPack/code/botlib/be_ea.c $
 *
 *****************************************************************************/
#[no_mangle]
pub static mut botinputs: *mut bot_input_t =
    0 as *const bot_input_t as *mut bot_input_t;
#[no_mangle]
pub unsafe extern "C" fn EA_Crouch(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x80i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_Walk(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x80000i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_MoveUp(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x20i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_MoveDown(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x100i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_MoveForward(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x200i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_MoveBack(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x800i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_MoveLeft(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x1000i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_MoveRight(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x2000i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_Attack(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x1i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_Respawn(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x8i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_Talk(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x10000i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_Gesture(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x20000i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_Use(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).actionflags |= 0x2i32;
}
//regular elementary actions
#[no_mangle]
pub unsafe extern "C" fn EA_SelectWeapon(mut client: libc::c_int,
                                         mut weapon: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).weapon = weapon;
}
#[no_mangle]
pub unsafe extern "C" fn EA_Jump(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    if 0 != (*bi).actionflags & 0x10000000i32 {
        (*bi).actionflags &= !0x10i32
    } else { (*bi).actionflags |= 0x10i32 };
}
#[no_mangle]
pub unsafe extern "C" fn EA_DelayedJump(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    if 0 != (*bi).actionflags & 0x10000000i32 {
        (*bi).actionflags &= !0x8000i32
    } else { (*bi).actionflags |= 0x8000i32 };
}
#[no_mangle]
pub unsafe extern "C" fn EA_Move(mut client: libc::c_int, mut dir: *mut vec_t,
                                 mut speed: libc::c_float) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).dir[0usize] = *dir.offset(0isize);
    (*bi).dir[1usize] = *dir.offset(1isize);
    (*bi).dir[2usize] = *dir.offset(2isize);
    if speed > 400i32 as libc::c_float {
        speed = 400i32 as libc::c_float
    } else if speed < -400i32 as libc::c_float {
        speed = -400i32 as libc::c_float
    }
    (*bi).speed = speed;
}
#[no_mangle]
pub unsafe extern "C" fn EA_View(mut client: libc::c_int,
                                 mut viewangles: *mut vec_t) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).viewangles[0usize] = *viewangles.offset(0isize);
    (*bi).viewangles[1usize] = *viewangles.offset(1isize);
    (*bi).viewangles[2usize] = *viewangles.offset(2isize);
}
//send regular input to the server
#[no_mangle]
pub unsafe extern "C" fn EA_EndRegular(mut client: libc::c_int,
                                       mut thinktime: libc::c_float) {
}
#[no_mangle]
pub unsafe extern "C" fn EA_GetInput(mut client: libc::c_int,
                                     mut thinktime: libc::c_float,
                                     mut input: *mut bot_input_t) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).thinktime = thinktime;
    memcpy(input as *mut libc::c_void, bi as *const libc::c_void,
           ::std::mem::size_of::<bot_input_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn EA_ResetInput(mut client: libc::c_int) {
    let mut bi: *mut bot_input_t = 0 as *mut bot_input_t;
    let mut jumped: libc::c_int = qfalse as libc::c_int;
    bi = &mut *botinputs.offset(client as isize) as *mut bot_input_t;
    (*bi).thinktime = 0i32 as libc::c_float;
    (*bi).dir[2usize] = 0i32 as vec_t;
    (*bi).dir[1usize] = (*bi).dir[2usize];
    (*bi).dir[0usize] = (*bi).dir[1usize];
    (*bi).speed = 0i32 as libc::c_float;
    jumped = (*bi).actionflags & 0x10i32;
    (*bi).actionflags = 0i32;
    if 0 != jumped { (*bi).actionflags |= 0x10000000i32 };
}
//setup and shutdown routines
#[no_mangle]
pub unsafe extern "C" fn EA_Setup() -> libc::c_int {
    botinputs =
        GetClearedHunkMemory((botlibglobals.maxclients as
                                  libc::c_ulong).wrapping_mul(::std::mem::size_of::<bot_input_t>()
                                                                  as
                                                                  libc::c_ulong))
            as *mut bot_input_t;
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn EA_Shutdown() {
    FreeMemory(botinputs as *mut libc::c_void);
    botinputs = 0 as *mut bot_input_t;
}
//end of the function EA_SayTeam
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn EA_Tell(mut client: libc::c_int,
                                 mut clientto: libc::c_int,
                                 mut str: *mut libc::c_char) {
    botimport.BotClientCommand.expect("non-null function pointer")(client,
                                                                   va(b"tell %d, %s\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                      clientto,
                                                                      str));
}
//end of the function EA_SayTeam
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn EA_UseItem(mut client: libc::c_int,
                                    mut it: *mut libc::c_char) {
    botimport.BotClientCommand.expect("non-null function pointer")(client,
                                                                   va(b"use %s\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                      it));
}
//end of the function EA_UseItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn EA_DropItem(mut client: libc::c_int,
                                     mut it: *mut libc::c_char) {
    botimport.BotClientCommand.expect("non-null function pointer")(client,
                                                                   va(b"drop %s\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                      it));
}
//end of the function EA_DropItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn EA_UseInv(mut client: libc::c_int,
                                   mut inv: *mut libc::c_char) {
    botimport.BotClientCommand.expect("non-null function pointer")(client,
                                                                   va(b"invuse %s\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                      inv));
}
//end of the function EA_UseInv
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn EA_DropInv(mut client: libc::c_int,
                                    mut inv: *mut libc::c_char) {
    botimport.BotClientCommand.expect("non-null function pointer")(client,
                                                                   va(b"invdrop %s\x00"
                                                                          as
                                                                          *const u8
                                                                          as
                                                                          *const libc::c_char
                                                                          as
                                                                          *mut libc::c_char,
                                                                      inv));
}