use ::libc;

pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu;
pub use crate::src::q3_ui::ui_video::UI_GraphicsOptionsMenu;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;

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
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menuslider_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayOptionsInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub graphics: crate::ui_local_h::menutext_s,
    pub display: crate::ui_local_h::menutext_s,
    pub sound: crate::ui_local_h::menutext_s,
    pub network: crate::ui_local_h::menutext_s,
    pub brightness: crate::ui_local_h::menuslider_s,
    pub screensize: crate::ui_local_h::menuslider_s,
    pub back: crate::ui_local_h::menubitmap_s,
}

static mut displayOptionsInfo: displayOptionsInfo_t = displayOptionsInfo_t {
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
    framel: crate::ui_local_h::menubitmap_s {
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
    framer: crate::ui_local_h::menubitmap_s {
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
    graphics: crate::ui_local_h::menutext_s {
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
    display: crate::ui_local_h::menutext_s {
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
    sound: crate::ui_local_h::menutext_s {
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
    network: crate::ui_local_h::menutext_s {
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
    brightness: crate::ui_local_h::menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    screensize: crate::ui_local_h::menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
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
=================
UI_DisplayOptionsMenu_Event
=================
*/

unsafe extern "C" fn UI_DisplayOptionsMenu_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        10 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_video::UI_GraphicsOptionsMenu();
        }
        12 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu();
        }
        13 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu();
        }
        14 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_gamma\x00" as *const u8 as *const i8,
                displayOptionsInfo.brightness.curvalue / 10.0f32,
            );
        }
        15 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"cg_viewsize\x00" as *const u8 as *const i8,
                displayOptionsInfo.screensize.curvalue * 10f32,
            );
        }
        16 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        11 | _ => {}
    };
}
/*
===============
UI_DisplayOptionsMenu_Init
===============
*/

unsafe extern "C" fn UI_DisplayOptionsMenu_Init() {
    let mut y: i32 = 0;
    crate::stdlib::memset(
        &mut displayOptionsInfo as *mut displayOptionsInfo_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<displayOptionsInfo_t>(),
    );
    UI_DisplayOptionsMenu_Cache();
    displayOptionsInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    displayOptionsInfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    displayOptionsInfo.banner.generic.type_0 = 10;
    displayOptionsInfo.banner.generic.flags = 0x8;
    displayOptionsInfo.banner.generic.x = 320;
    displayOptionsInfo.banner.generic.y = 16;
    displayOptionsInfo.banner.string = b"SYSTEM SETUP\x00" as *const u8 as *mut i8;
    displayOptionsInfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    displayOptionsInfo.banner.style = 0x1;
    displayOptionsInfo.framel.generic.type_0 = 6;
    displayOptionsInfo.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const i8;
    displayOptionsInfo.framel.generic.flags = 0x4000;
    displayOptionsInfo.framel.generic.x = 0;
    displayOptionsInfo.framel.generic.y = 78;
    displayOptionsInfo.framel.width = 256;
    displayOptionsInfo.framel.height = 329;
    displayOptionsInfo.framer.generic.type_0 = 6;
    displayOptionsInfo.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const i8;
    displayOptionsInfo.framer.generic.flags = 0x4000;
    displayOptionsInfo.framer.generic.x = 376;
    displayOptionsInfo.framer.generic.y = 76;
    displayOptionsInfo.framer.width = 256;
    displayOptionsInfo.framer.height = 334;
    displayOptionsInfo.graphics.generic.type_0 = 9;
    displayOptionsInfo.graphics.generic.flags = 0x10 | 0x100;
    displayOptionsInfo.graphics.generic.id = 10;
    displayOptionsInfo.graphics.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.graphics.generic.x = 216;
    displayOptionsInfo.graphics.generic.y = 240 - 2 * 27;
    displayOptionsInfo.graphics.string = b"GRAPHICS\x00" as *const u8 as *mut i8;
    displayOptionsInfo.graphics.style = 0x2;
    displayOptionsInfo.graphics.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    displayOptionsInfo.display.generic.type_0 = 9;
    displayOptionsInfo.display.generic.flags = 0x10;
    displayOptionsInfo.display.generic.id = 11;
    displayOptionsInfo.display.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.display.generic.x = 216;
    displayOptionsInfo.display.generic.y = 240 - 27;
    displayOptionsInfo.display.string = b"DISPLAY\x00" as *const u8 as *mut i8;
    displayOptionsInfo.display.style = 0x2;
    displayOptionsInfo.display.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    displayOptionsInfo.sound.generic.type_0 = 9;
    displayOptionsInfo.sound.generic.flags = 0x10 | 0x100;
    displayOptionsInfo.sound.generic.id = 12;
    displayOptionsInfo.sound.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.sound.generic.x = 216;
    displayOptionsInfo.sound.generic.y = 240;
    displayOptionsInfo.sound.string = b"SOUND\x00" as *const u8 as *mut i8;
    displayOptionsInfo.sound.style = 0x2;
    displayOptionsInfo.sound.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    displayOptionsInfo.network.generic.type_0 = 9;
    displayOptionsInfo.network.generic.flags = 0x10 | 0x100;
    displayOptionsInfo.network.generic.id = 13;
    displayOptionsInfo.network.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.network.generic.x = 216;
    displayOptionsInfo.network.generic.y = 240 + 27;
    displayOptionsInfo.network.string = b"NETWORK\x00" as *const u8 as *mut i8;
    displayOptionsInfo.network.style = 0x2;
    displayOptionsInfo.network.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    y = 240 - 1 * (16 + 2);
    displayOptionsInfo.brightness.generic.type_0 = 1;
    displayOptionsInfo.brightness.generic.name = b"Brightness:\x00" as *const u8 as *const i8;
    displayOptionsInfo.brightness.generic.flags = 0x100 | 0x2;
    displayOptionsInfo.brightness.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.brightness.generic.id = 14;
    displayOptionsInfo.brightness.generic.x = 400;
    displayOptionsInfo.brightness.generic.y = y;
    displayOptionsInfo.brightness.minvalue = 5f32;
    displayOptionsInfo.brightness.maxvalue = 20f32;
    if crate::src::q3_ui::ui_atoms::uis
        .glconfig
        .deviceSupportsGamma as u64
        == 0
    {
        displayOptionsInfo.brightness.generic.flags |= 0x2000
    }
    y += 16 + 2;
    displayOptionsInfo.screensize.generic.type_0 = 1;
    displayOptionsInfo.screensize.generic.name = b"Screen Size:\x00" as *const u8 as *const i8;
    displayOptionsInfo.screensize.generic.flags = 0x100 | 0x2;
    displayOptionsInfo.screensize.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.screensize.generic.id = 15;
    displayOptionsInfo.screensize.generic.x = 400;
    displayOptionsInfo.screensize.generic.y = y;
    displayOptionsInfo.screensize.minvalue = 3f32;
    displayOptionsInfo.screensize.maxvalue = 10f32;
    displayOptionsInfo.back.generic.type_0 = 6;
    displayOptionsInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    displayOptionsInfo.back.generic.flags = 0x4 | 0x100;
    displayOptionsInfo.back.generic.callback = Some(
        UI_DisplayOptionsMenu_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    displayOptionsInfo.back.generic.id = 16;
    displayOptionsInfo.back.generic.x = 0;
    displayOptionsInfo.back.generic.y = 480 - 64;
    displayOptionsInfo.back.width = 128;
    displayOptionsInfo.back.height = 64;
    displayOptionsInfo.back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.graphics as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.display as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.sound as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.network as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.brightness as *mut crate::ui_local_h::menuslider_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.screensize as *mut crate::ui_local_h::menuslider_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    displayOptionsInfo.brightness.curvalue = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_gamma\x00" as *const u8 as *const i8,
    ) * 10f32;
    displayOptionsInfo.screensize.curvalue = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cg_viewsize\x00" as *const u8 as *const i8,
    ) / 10f32;
}
/*
===============
UI_DisplayOptionsMenu_Cache
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DisplayOptionsMenu_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame2_l\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame1_r\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const i8,
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
//
// ui_playermodel.c
//
//
// ui_playersettings.c
//
//
// ui_preferences.c
//
//
// ui_specifyleague.c
//
//
// ui_specifyserver.c
//
//
// ui_servers2.c
//
//
// ui_startserver.c
//
//
// ui_serverinfo.c
//
//
// ui_video.c
//
//
// ui_players.c
//
//FIXME ripped from cg_local.h
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// model info
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// currently in use drawing parms
// animation vars
//
// ui_atoms.c
//
//
// ui_spLevel.c
//
//
// ui_spArena.c
//
//
// ui_spPostgame.c
//
//
// ui_spSkill.c
//
//
// ui_syscalls.c
//
// don't use EXEC_NOW!
// fsOrigin_t
//
// ui_addbots.c
//
//
// ui_removebots.c
//
//
// ui_teamorders.c
//
//
// ui_loadconfig.c
//
//
// ui_saveconfig.c
//
//
// ui_display.c
//
/*
===============
UI_DisplayOptionsMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DisplayOptionsMenu() {
    UI_DisplayOptionsMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut displayOptionsInfo.menu);
    crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.display as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
}
