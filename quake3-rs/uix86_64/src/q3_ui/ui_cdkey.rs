use ::libc;

pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_DrawChar;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_FillRect;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_orange;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::color_yellow;
pub use crate::src::q3_ui::ui_qmenu::listbar_color;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_GetCDKey;
pub use crate::src::ui::ui_syscalls::trap_Key_GetOverstrikeMode;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_SetCDKey;
pub use crate::src::ui::ui_syscalls::trap_VerifyCDKey;

pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menufield_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::mfield_t;
pub use crate::ui_local_h::uiStatic_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdkeyMenuInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub frame: crate::ui_local_h::menubitmap_s,
    pub cdkey: crate::ui_local_h::menufield_s,
    pub accept: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
}

static mut cdkeyMenuInfo: cdkeyMenuInfo_t = cdkeyMenuInfo_t {
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
    banner: crate::ui_local_h::menutext_s {
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
    cdkey: crate::ui_local_h::menufield_s {
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
        field: crate::ui_local_h::mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    accept: crate::ui_local_h::menubitmap_s {
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
    back: crate::ui_local_h::menubitmap_s {
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
};
/*
===============
UI_CDKeyMenu_Event
===============
*/

unsafe extern "C" fn UI_CDKeyMenu_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        11 => {
            if cdkeyMenuInfo.cdkey.field.buffer[0] != 0 {
                crate::src::ui::ui_syscalls::trap_SetCDKey(
                    cdkeyMenuInfo.cdkey.field.buffer.as_mut_ptr(),
                );
            }
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        12 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
UI_CDKeyMenu_PreValidateKey
=================
*/

unsafe extern "C" fn UI_CDKeyMenu_PreValidateKey(mut key: *const i8) -> i32 {
    let mut ch: i8 = 0;
    if crate::stdlib::strlen(key) != 16 {
        return 1i32;
    }
    loop {
        let fresh0 = key;
        key = key.offset(1);
        ch = *fresh0;
        if !(ch != 0) {
            break;
        }
        match ch as i32 {
            50 | 51 | 55 | 97 | 98 | 99 | 100 | 103 | 104 | 106 | 108 | 112 | 114 | 115 | 116
            | 119 => {
                continue;
            }
            _ => {}
        }
        return -(1);
    }
    return 0;
}
/*
=================
UI_CDKeyMenu_DrawKey
=================
*/

unsafe extern "C" fn UI_CDKeyMenu_DrawKey(mut self_0: *mut libc::c_void) {
    let mut f: *mut crate::ui_local_h::menufield_s = 0 as *mut crate::ui_local_h::menufield_s;
    let mut focus: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    let mut style: i32 = 0;
    let mut c: i8 = 0;
    let mut color: *mut f32 = 0 as *mut f32;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut val: i32 = 0;
    f = self_0 as *mut crate::ui_local_h::menufield_s;
    focus = ((*(*f).generic.parent).cursor == (*f).generic.menuPosition)
        as crate::src::qcommon::q_shared::qboolean;
    style = 0;
    if focus as u64 != 0 {
        color = crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr()
    } else {
        color = crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr()
    }
    x = 320 - 8 * 16;
    y = 240 - 16 / 2;
    crate::src::q3_ui::ui_atoms::UI_FillRect(
        x as f32,
        y as f32,
        (16i32 * 16) as f32,
        16f32,
        crate::src::q3_ui::ui_qmenu::listbar_color.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(x, y, (*f).field.buffer.as_mut_ptr(), style, color);
    // draw cursor if we have focus
    if focus as u64 != 0 {
        if crate::src::ui::ui_syscalls::trap_Key_GetOverstrikeMode() as u64 != 0 {
            c = 11
        } else {
            c = 10
        }
        style &= !(0x4000);
        style |= 0x1000;
        crate::src::q3_ui::ui_atoms::UI_DrawChar(
            x + (*f).field.cursor * 16i32,
            y,
            c as i32,
            style,
            crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
        );
    }
    val = UI_CDKeyMenu_PreValidateKey((*f).field.buffer.as_mut_ptr());
    if val == 1 {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            320i32,
            376i32,
            b"Please enter your CD Key\x00" as *const u8 as *const i8,
            0x1i32 | 0x10i32,
            crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
        );
    } else if val == 0 {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            320i32,
            376i32,
            b"The CD Key appears to be valid, thank you\x00" as *const u8 as *const i8,
            0x1i32 | 0x10i32,
            crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
        );
    } else {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            320i32,
            376i32,
            b"The CD Key is not valid\x00" as *const u8 as *const i8,
            0x1i32 | 0x10i32,
            crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
        );
    };
}
/*
===============
UI_CDKeyMenu_Init
===============
*/

unsafe extern "C" fn UI_CDKeyMenu_Init() {
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"ui_cdkeychecked\x00" as *const u8 as *const i8,
        b"1\x00" as *const u8 as *const i8,
    );
    UI_CDKeyMenu_Cache();
    crate::stdlib::memset(
        &mut cdkeyMenuInfo as *mut cdkeyMenuInfo_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<cdkeyMenuInfo_t>(),
    );
    cdkeyMenuInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    cdkeyMenuInfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    cdkeyMenuInfo.banner.generic.type_0 = 10;
    cdkeyMenuInfo.banner.generic.x = 320;
    cdkeyMenuInfo.banner.generic.y = 16;
    cdkeyMenuInfo.banner.string = b"CD KEY\x00" as *const u8 as *mut i8;
    cdkeyMenuInfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    cdkeyMenuInfo.banner.style = 0x1;
    cdkeyMenuInfo.frame.generic.type_0 = 6;
    cdkeyMenuInfo.frame.generic.name = b"menu/art/cut_frame\x00" as *const u8 as *const i8;
    cdkeyMenuInfo.frame.generic.flags = 0x4000;
    cdkeyMenuInfo.frame.generic.x = 142;
    cdkeyMenuInfo.frame.generic.y = 118;
    cdkeyMenuInfo.frame.width = 359;
    cdkeyMenuInfo.frame.height = 256;
    cdkeyMenuInfo.cdkey.generic.type_0 = 4;
    cdkeyMenuInfo.cdkey.generic.name = b"CD Key:\x00" as *const u8 as *const i8;
    cdkeyMenuInfo.cdkey.generic.flags = 0x40000;
    cdkeyMenuInfo.cdkey.generic.x = (320f64 - 16f64 * 2.5) as i32;
    cdkeyMenuInfo.cdkey.generic.y = 240 - 16 / 2;
    cdkeyMenuInfo.cdkey.field.widthInChars = 16;
    cdkeyMenuInfo.cdkey.field.maxchars = 16;
    cdkeyMenuInfo.cdkey.generic.ownerdraw =
        Some(UI_CDKeyMenu_DrawKey as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    cdkeyMenuInfo.accept.generic.type_0 = 6;
    cdkeyMenuInfo.accept.generic.name = b"menu/art/accept_0\x00" as *const u8 as *const i8;
    cdkeyMenuInfo.accept.generic.flags = 0x10 | 0x100;
    cdkeyMenuInfo.accept.generic.id = 11;
    cdkeyMenuInfo.accept.generic.callback =
        Some(UI_CDKeyMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cdkeyMenuInfo.accept.generic.x = 640;
    cdkeyMenuInfo.accept.generic.y = 480 - 64;
    cdkeyMenuInfo.accept.width = 128;
    cdkeyMenuInfo.accept.height = 64;
    cdkeyMenuInfo.accept.focuspic = b"menu/art/accept_1\x00" as *const u8 as *mut i8;
    cdkeyMenuInfo.back.generic.type_0 = 6;
    cdkeyMenuInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    cdkeyMenuInfo.back.generic.flags = 0x4 | 0x100;
    cdkeyMenuInfo.back.generic.id = 12;
    cdkeyMenuInfo.back.generic.callback =
        Some(UI_CDKeyMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    cdkeyMenuInfo.back.generic.x = 0;
    cdkeyMenuInfo.back.generic.y = 480 - 64;
    cdkeyMenuInfo.back.width = 128;
    cdkeyMenuInfo.back.height = 64;
    cdkeyMenuInfo.back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cdkeyMenuInfo.menu,
        &mut cdkeyMenuInfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cdkeyMenuInfo.menu,
        &mut cdkeyMenuInfo.frame as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cdkeyMenuInfo.menu,
        &mut cdkeyMenuInfo.cdkey as *mut crate::ui_local_h::menufield_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut cdkeyMenuInfo.menu,
        &mut cdkeyMenuInfo.accept as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    if crate::src::q3_ui::ui_atoms::uis.menusp != 0 {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut cdkeyMenuInfo.menu,
            &mut cdkeyMenuInfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
        );
    }
    crate::src::ui::ui_syscalls::trap_GetCDKey(
        cdkeyMenuInfo.cdkey.field.buffer.as_mut_ptr(),
        cdkeyMenuInfo.cdkey.field.maxchars + 1,
    );
    if crate::src::ui::ui_syscalls::trap_VerifyCDKey(
        cdkeyMenuInfo.cdkey.field.buffer.as_mut_ptr(),
        0 as *const i8,
    ) == crate::src::qcommon::q_shared::qfalse
    {
        cdkeyMenuInfo.cdkey.field.buffer[0] = 0
    };
}
/*
=================
UI_CDKeyMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_CDKeyMenu_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/accept_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/accept_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/cut_frame\x00" as *const u8 as *const i8,
    );
}
/*
===============
UI_CDKeyMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_CDKeyMenu() {
    UI_CDKeyMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut cdkeyMenuInfo.menu);
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
//
// ui_connect.c
//
//
// ui_controls2.c
//
//
// ui_demo2.c
//
//
// ui_cinematics.c
//
//
// ui_mods.c
//
//
// ui_cdkey.c
//
/*
===============
UI_CDKeyMenu_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_CDKeyMenu_f() {
    UI_CDKeyMenu();
}
