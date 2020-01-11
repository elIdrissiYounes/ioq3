use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::src::q3_ui::ui_atoms::UI_ForceMenuOff;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_team::stdlib_h::atoi;
pub use crate::src::qcommon::q_math::colorRed;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_GetConfigString;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

pub use crate::stdlib::strtol;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct teammain_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub frame: crate::ui_local_h::menubitmap_s,
    pub joinred: crate::ui_local_h::menutext_s,
    pub joinblue: crate::ui_local_h::menutext_s,
    pub joingame: crate::ui_local_h::menutext_s,
    pub spectate: crate::ui_local_h::menutext_s,
}

static mut s_teammain: teammain_t = teammain_t {
    menu: crate::ui_local_h::menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [0 as *mut libc::c_void; 64],
        draw: None,
        key: None,
        wrapAround: crate::src::qcommon::q_shared::qfalse,
        fullscreen: crate::src::qcommon::q_shared::qfalse,
        showlogo: crate::src::qcommon::q_shared::qfalse,
    },
    frame: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *mut i8,
        errorpic: 0 as *mut i8,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *mut f32,
    },
    joinred: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *mut i8,
        style: 0,
        color: 0 as *mut f32,
    },
    joinblue: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *mut i8,
        style: 0,
        color: 0 as *mut f32,
    },
    joingame: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *mut i8,
        style: 0,
        color: 0 as *mut f32,
    },
    spectate: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *mut i8,
        style: 0,
        color: 0 as *mut f32,
    },
};
/*
===============
TeamMain_MenuEvent
===============
*/

unsafe extern "C" fn TeamMain_MenuEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        100 => {
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                b"cmd team red\n\x00" as *const u8 as *const i8,
            );
            crate::src::q3_ui::ui_atoms::UI_ForceMenuOff();
        }
        101 => {
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                b"cmd team blue\n\x00" as *const u8 as *const i8,
            );
            crate::src::q3_ui::ui_atoms::UI_ForceMenuOff();
        }
        102 => {
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                b"cmd team free\n\x00" as *const u8 as *const i8,
            );
            crate::src::q3_ui::ui_atoms::UI_ForceMenuOff();
        }
        103 => {
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                b"cmd team spectator\n\x00" as *const u8 as *const i8,
            );
            crate::src::q3_ui::ui_atoms::UI_ForceMenuOff();
        }
        _ => {}
    };
}
/*
===============
TeamMain_MenuInit
===============
*/
#[no_mangle]

pub unsafe extern "C" fn TeamMain_MenuInit() {
    let mut y: i32 = 0;
    let mut gametype: i32 = 0;
    let mut info: [i8; 1024] = [0; 1024];
    crate::stdlib::memset(
        &mut s_teammain as *mut teammain_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<teammain_t>(),
    );
    TeamMain_Cache();
    s_teammain.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_teammain.menu.fullscreen = crate::src::qcommon::q_shared::qfalse;
    s_teammain.frame.generic.type_0 = 6;
    s_teammain.frame.generic.flags = 0x4000;
    s_teammain.frame.generic.name = b"menu/art/cut_frame\x00" as *const u8 as *const i8;
    s_teammain.frame.generic.x = 142;
    s_teammain.frame.generic.y = 118;
    s_teammain.frame.width = 359;
    s_teammain.frame.height = 256;
    y = 194;
    s_teammain.joinred.generic.type_0 = 9;
    s_teammain.joinred.generic.flags = 0x8 | 0x100;
    s_teammain.joinred.generic.id = 100;
    s_teammain.joinred.generic.callback =
        Some(TeamMain_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_teammain.joinred.generic.x = 320;
    s_teammain.joinred.generic.y = y;
    s_teammain.joinred.string = b"JOIN RED\x00" as *const u8 as *mut i8;
    s_teammain.joinred.style = 0x1 | 0x10;
    s_teammain.joinred.color = crate::src::qcommon::q_math::colorRed.as_mut_ptr();
    y += 20;
    s_teammain.joinblue.generic.type_0 = 9;
    s_teammain.joinblue.generic.flags = 0x8 | 0x100;
    s_teammain.joinblue.generic.id = 101;
    s_teammain.joinblue.generic.callback =
        Some(TeamMain_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_teammain.joinblue.generic.x = 320;
    s_teammain.joinblue.generic.y = y;
    s_teammain.joinblue.string = b"JOIN BLUE\x00" as *const u8 as *mut i8;
    s_teammain.joinblue.style = 0x1 | 0x10;
    s_teammain.joinblue.color = crate::src::qcommon::q_math::colorRed.as_mut_ptr();
    y += 20;
    s_teammain.joingame.generic.type_0 = 9;
    s_teammain.joingame.generic.flags = 0x8 | 0x100;
    s_teammain.joingame.generic.id = 102;
    s_teammain.joingame.generic.callback =
        Some(TeamMain_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_teammain.joingame.generic.x = 320;
    s_teammain.joingame.generic.y = y;
    s_teammain.joingame.string = b"JOIN GAME\x00" as *const u8 as *mut i8;
    s_teammain.joingame.style = 0x1 | 0x10;
    s_teammain.joingame.color = crate::src::qcommon::q_math::colorRed.as_mut_ptr();
    y += 20;
    s_teammain.spectate.generic.type_0 = 9;
    s_teammain.spectate.generic.flags = 0x8 | 0x100;
    s_teammain.spectate.generic.id = 103;
    s_teammain.spectate.generic.callback =
        Some(TeamMain_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_teammain.spectate.generic.x = 320;
    s_teammain.spectate.generic.y = y;
    s_teammain.spectate.string = b"SPECTATE\x00" as *const u8 as *mut i8;
    s_teammain.spectate.style = 0x1 | 0x10;
    s_teammain.spectate.color = crate::src::qcommon::q_math::colorRed.as_mut_ptr();
    crate::src::ui::ui_syscalls::trap_GetConfigString(0, info.as_mut_ptr(), 1024);
    gametype = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info.as_mut_ptr(),
        b"g_gametype\x00" as *const u8 as *const i8,
    ));
    // set initial states
    match gametype {
        2 | 0 | 1 => {
            s_teammain.joinred.generic.flags |= 0x2000;
            s_teammain.joinblue.generic.flags |= 0x2000
        }
        3 | 4 | _ => s_teammain.joingame.generic.flags |= 0x2000,
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_teammain.menu,
        &mut s_teammain.frame as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_teammain.menu,
        &mut s_teammain.joinred as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_teammain.menu,
        &mut s_teammain.joinblue as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_teammain.menu,
        &mut s_teammain.joingame as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_teammain.menu,
        &mut s_teammain.spectate as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
}
/*
===============
TeamMain_Cache
===============
*/
#[no_mangle]

pub unsafe extern "C" fn TeamMain_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/cut_frame\x00" as *const u8 as *const i8,
    );
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
//
//NOTE: include the ui_public.h from the new UI
//redefine to old API version
//
// ui_qmenu.c
//
// edit field is only numbers
// steady focus
// pulse if focus
// only mouse input allowed
// skips drawing
// grays and disables
// disables any input
// skip default initialization
// edit field is all lower case
// edit field is all upper case
// callback notifications
//
// ui_mfield.c
//
//
// ui_menu.c
//
//
// ui_credits.c
//
//
// ui_ingame.c
//
//
// ui_confirm.c
//
//
// ui_setup.c
//
//
// ui_team.c
//
/*
===============
UI_TeamMainMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_TeamMainMenu() {
    TeamMain_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut s_teammain.menu);
}
