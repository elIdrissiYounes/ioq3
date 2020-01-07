use ::libc;

pub use crate::botlib_h::bot_input_s;
pub use crate::botlib_h::bot_input_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::be_interface::botimport;
pub use crate::src::botlib::be_interface::botlib_globals_s;
pub use crate::src::botlib::be_interface::botlib_globals_t;
pub use crate::src::botlib::be_interface::botlibglobals;
use crate::src::botlib::l_memory::FreeMemory;
use crate::src::botlib::l_memory::GetClearedHunkMemory;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
use crate::stdlib::memcpy;
#[no_mangle]

pub static mut botinputs: *mut crate::botlib_h::bot_input_t =
    0 as *mut crate::botlib_h::bot_input_t;
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
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Say(mut client: i32, mut str: *mut i8) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(b"say %s\x00" as *const u8 as *mut i8, str),
    );
}
//end of the function EA_Say
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_SayTeam(mut client: i32, mut str: *mut i8) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(b"say_team %s\x00" as *const u8 as *mut i8, str),
    );
}
//end of the function EA_SayTeam
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Tell(mut client: i32, mut clientto: i32, mut str: *mut i8) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(
            b"tell %d, %s\x00" as *const u8 as *mut i8,
            clientto,
            str,
        ),
    );
}
//end of the function EA_SayTeam
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_UseItem(mut client: i32, mut it: *mut i8) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(b"use %s\x00" as *const u8 as *mut i8, it),
    );
}
//end of the function EA_UseItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_DropItem(mut client: i32, mut it: *mut i8) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(b"drop %s\x00" as *const u8 as *mut i8, it),
    );
}
//end of the function EA_DropItem
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_UseInv(mut client: i32, mut inv: *mut i8) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(b"invuse %s\x00" as *const u8 as *mut i8, inv),
    );
}
//end of the function EA_UseInv
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_DropInv(mut client: i32, mut inv: *mut i8) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(
        client,
        crate::src::qcommon::q_shared::va(b"invdrop %s\x00" as *const u8 as *mut i8, inv),
    );
}
//end of the function EA_DropInv
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Gesture(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x20000;
}
//end of the function EA_Gesture
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Command(mut client: i32, mut command: *mut i8) {
    crate::src::botlib::be_interface::botimport
        .BotClientCommand
        .expect("non-null function pointer")(client, command);
}
//regular elementary actions
//end of the function EA_Command
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_SelectWeapon(mut client: i32, mut weapon: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).weapon = weapon;
}
//end of the function EA_SelectWeapon
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Attack(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x1;
}
//end of the function EA_Attack
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Talk(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x10000;
}
//end of the function EA_Talk
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Use(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x2;
}
//end of the function EA_Use
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Respawn(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x8;
}
//end of the function EA_Respawn
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Jump(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t; //end if
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    if (*bi).actionflags & 0x10000000 != 0 {
        (*bi).actionflags &= !(0x10)
    } else {
        (*bi).actionflags |= 0x10
    };
    //end if
}
//end of the function EA_Jump
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_DelayedJump(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t; //end if
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    if (*bi).actionflags & 0x10000000 != 0 {
        (*bi).actionflags &= !(0x8000)
    } else {
        (*bi).actionflags |= 0x8000
    };
    //end if
}
//end of the function EA_DelayedJump
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Crouch(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x80;
}
//end of the function EA_Crouch
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Walk(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x80000;
}
//end of the function EA_Walk
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Action(mut client: i32, mut action: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= action;
}
//end of function EA_Action
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveUp(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x20;
}
//end of the function EA_MoveUp
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveDown(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x100;
}
//end of the function EA_MoveDown
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveForward(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x200;
}
//end of the function EA_MoveForward
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveBack(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x800;
}
//end of the function EA_MoveBack
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveLeft(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x1000;
}
//end of the function EA_MoveLeft
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_MoveRight(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).actionflags |= 0x2000;
}
//end of the function EA_MoveRight
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Move(
    mut client: i32,
    mut dir: *mut crate::src::qcommon::q_shared::vec_t,
    mut speed: f32,
) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).dir[0] = *dir.offset(0);
    (*bi).dir[1] = *dir.offset(1);
    (*bi).dir[2] = *dir.offset(2);
    //cap speed
    if speed > 400f32 {
        speed = 400f32
    } else if speed < -400f32 {
        speed = -400f32
    }
    (*bi).speed = speed;
}
//end of the function EA_Move
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_View(
    mut client: i32,
    mut viewangles: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).viewangles[0] = *viewangles.offset(0);
    (*bi).viewangles[1] = *viewangles.offset(1);
    (*bi).viewangles[2] = *viewangles.offset(2);
}
//send regular input to the server
//end of the function EA_View
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_EndRegular(mut client: i32, mut thinktime: f32) {}
//end of the function EA_EndRegular
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_GetInput(
    mut client: i32,
    mut thinktime: f32,
    mut input: *mut crate::botlib_h::bot_input_t,
) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).thinktime = thinktime;
    crate::stdlib::memcpy(
        input as *mut libc::c_void,
        bi as *const libc::c_void,
        ::std::mem::size_of::<crate::botlib_h::bot_input_t>(),
    );
}
//end of the function EA_GetInput
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_ResetInput(mut client: i32) {
    let mut bi: *mut crate::botlib_h::bot_input_t = 0 as *mut crate::botlib_h::bot_input_t;
    let mut jumped: i32 = crate::src::qcommon::q_shared::qfalse as i32;
    bi = &mut *botinputs.offset(client as isize) as *mut crate::botlib_h::bot_input_t;
    (*bi).thinktime = 0f32;
    (*bi).dir[2] = 0f32;
    (*bi).dir[1] = (*bi).dir[2];
    (*bi).dir[0] = (*bi).dir[1];
    (*bi).speed = 0f32;
    jumped = (*bi).actionflags & 0x10;
    (*bi).actionflags = 0;
    if jumped != 0 {
        (*bi).actionflags |= 0x10000000
    };
}
//setup and shutdown routines
//end of the function EA_ResetInput
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Setup() -> i32 {
    //initialize the bot inputs
    botinputs = crate::src::botlib::l_memory::GetClearedHunkMemory(
        (crate::src::botlib::be_interface::botlibglobals.maxclients as usize)
            .wrapping_mul(::std::mem::size_of::<crate::botlib_h::bot_input_t>()),
    ) as *mut crate::botlib_h::bot_input_t;
    return 0;
}
//end of the function EA_Setup
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EA_Shutdown() {
    crate::src::botlib::l_memory::FreeMemory(botinputs as *mut libc::c_void);
    botinputs = 0 as *mut crate::botlib_h::bot_input_t;
}
//end of the function EA_Shutdown
