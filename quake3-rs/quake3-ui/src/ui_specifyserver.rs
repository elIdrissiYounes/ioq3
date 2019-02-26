#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use bg_misc::bg_itemlist;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, colorYellow, g_color_table, vec3_origin,
    vectoangles, AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract,
    AnglesToAxis, AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t, Com_sprintf,
    EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::{memset, strcpy, strlen};
use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
use ui_atoms::{
    uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
    UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic, UI_DrawNamedPic,
    UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped, UI_DrawRect, UI_DrawString,
    UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent, UI_MouseEvent, UI_PopMenu,
    UI_ProportionalSizeScale, UI_ProportionalStringWidth, UI_PushMenu, UI_Refresh,
    UI_SetActiveMenu, UI_SetColor, UI_Shutdown,
};
use ui_cdkey::{UI_CDKeyMenu, UI_CDKeyMenu_Cache, UI_CDKeyMenu_f};
use ui_cinematics::{UI_CinematicsMenu, UI_CinematicsMenu_Cache, UI_CinematicsMenu_f};
use ui_confirm::{ConfirmMenu_Cache, UI_ConfirmMenu, UI_ConfirmMenu_Style, UI_Message};
use ui_connect::UI_DrawConnectScreen;
use ui_controls2::{Controls_Cache, UI_ControlsMenu};
use ui_credits::UI_CreditMenu;
use ui_demo2::{Demos_Cache, UI_DemosMenu};
use ui_display::{UI_DisplayOptionsMenu, UI_DisplayOptionsMenu_Cache};
use ui_gameinfo::{
    UI_CanShowTierVideo, UI_GetArenaInfoByMap, UI_GetArenaInfoByNumber, UI_GetAwardLevel,
    UI_GetBestScore, UI_GetBotInfoByName, UI_GetBotInfoByNumber, UI_GetCurrentGame,
    UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers, UI_GetSpecialArenaInfo,
    UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f, UI_SPUnlock_f,
    UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
};
use ui_ingame::{InGame_Cache, UI_InGameMenu};
use ui_local_h::{
    _tag_menuframework, menubitmap_s, menucommon_s, menufield_s, menuframework_s, menutext_s,
    mfield_t, trap_Cmd_ExecuteText, trap_R_RegisterShaderNoMip,
};
use ui_main::{
    ui_browserGameType, ui_browserMaster, ui_browserShowEmpty, ui_browserShowFull,
    ui_browserSortKey, ui_cdkeychecked, UI_RegisterCvars, UI_UpdateCvars,
};
use ui_menu::{MainMenu_Cache, UI_MainMenu};
use ui_mfield::{MField_Draw, MenuField_Draw, MenuField_Init, MenuField_Key};
use ui_mods::{UI_ModsMenu, UI_ModsMenu_Cache};
use ui_network::{UI_NetworkOptionsMenu, UI_NetworkOptionsMenu_Cache};
use ui_playermodel::{PlayerModel_Cache, UI_PlayerModelMenu};
use ui_players::{UI_DrawPlayer, UI_PlayerInfo_SetInfo, UI_PlayerInfo_SetModel};
use ui_playersettings::{PlayerSettings_Cache, UI_PlayerSettingsMenu};
use ui_preferences::{Preferences_Cache, UI_PreferencesMenu};
use ui_qmenu::{
    color_black, color_orange, color_red, color_white, color_yellow, listbar_color,
    menu_buzz_sound, menu_in_sound, menu_move_sound, menu_null_sound, menu_out_sound,
    menu_text_color, text_color_disabled, text_color_highlight, text_color_normal,
    weaponChangeSound, Bitmap_Draw, Bitmap_Init, Menu_AddItem, Menu_Cache, Menu_DefaultKey,
    Menu_Draw, Menu_ItemAtCursor, Menu_SetCursor, Menu_SetCursorToItem, ScrollList_Key,
};
use ui_removebots::{UI_RemoveBotsMenu, UI_RemoveBots_Cache};
use ui_serverinfo::{ServerInfo_Cache, UI_ServerInfoMenu};
use ui_servers2::{punkbuster_items, ArenaServers_Cache, UI_ArenaServersMenu};
use ui_setup::{UI_SetupMenu, UI_SetupMenu_Cache};
use ui_sound::{UI_SoundOptionsMenu, UI_SoundOptionsMenu_Cache};
use ui_sparena::UI_SPArena_Start;
use ui_splevel::{UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f};
use ui_sppostgame::{
    ui_medalPicNames, ui_medalSounds, UI_SPPostgameMenu_Cache, UI_SPPostgameMenu_f,
};
use ui_spskill::{UI_SPSkillMenu, UI_SPSkillMenu_Cache};
use ui_startserver::{
    ServerOptions_Cache, StartServer_Cache, UI_BotSelectMenu_Cache, UI_StartServerMenu,
};
use ui_team::{TeamMain_Cache, UI_TeamMainMenu};
use ui_teamorders::{UI_TeamOrdersMenu, UI_TeamOrdersMenu_f};
use ui_video::{DriverInfo_Cache, GraphicsOptions_Cache, UI_GraphicsOptionsMenu};
extern crate libc;

//
// ui_specifyserver.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_SpecifyServerMenu() {
    SpecifyServer_MenuInit();
    UI_PushMenu(&mut s_specifyserver.menu);
}
static mut s_specifyserver: specifyserver_t = specifyserver_t {
    menu: _tag_menuframework {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [0 as *const libc::c_void as *mut libc::c_void; 64],
        draw: None,
        key: None,
        wrapAround: qfalse,
        fullscreen: qfalse,
        showlogo: qfalse,
    },
    banner: menutext_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        string: 0 as *const libc::c_char as *mut libc::c_char,
        style: 0,
        color: 0 as *const libc::c_float as *mut libc::c_float,
    },
    framel: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    framer: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    domain: menufield_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        field: mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    port: menufield_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        field: mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    go: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
    back: menubitmap_s {
        generic: menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const menuframework_s as *mut menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *const libc::c_char as *mut libc::c_char,
        errorpic: 0 as *const libc::c_char as *mut libc::c_char,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *const libc::c_float as *mut libc::c_float,
    },
};
/*
=================
SpecifyServer_MenuInit
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SpecifyServer_MenuInit() {
    memset(
        &mut s_specifyserver as *mut specifyserver_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<specifyserver_t>() as libc::c_ulong,
    );
    SpecifyServer_Cache();
    s_specifyserver.menu.wrapAround = qtrue;
    s_specifyserver.menu.fullscreen = qtrue;
    s_specifyserver.banner.generic.type_0 = 10i32;
    s_specifyserver.banner.generic.x = 320i32;
    s_specifyserver.banner.generic.y = 16i32;
    s_specifyserver.banner.string =
        b"SPECIFY SERVER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_specifyserver.banner.color = color_white.as_mut_ptr();
    s_specifyserver.banner.style = 0x1i32;
    s_specifyserver.framel.generic.type_0 = 6i32;
    s_specifyserver.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.framel.generic.flags = 0x4000i32 as libc::c_uint;
    s_specifyserver.framel.generic.x = 0i32;
    s_specifyserver.framel.generic.y = 78i32;
    s_specifyserver.framel.width = 256i32;
    s_specifyserver.framel.height = 329i32;
    s_specifyserver.framer.generic.type_0 = 6i32;
    s_specifyserver.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.framer.generic.flags = 0x4000i32 as libc::c_uint;
    s_specifyserver.framer.generic.x = 376i32;
    s_specifyserver.framer.generic.y = 76i32;
    s_specifyserver.framer.width = 256i32;
    s_specifyserver.framer.height = 334i32;
    s_specifyserver.domain.generic.type_0 = 4i32;
    s_specifyserver.domain.generic.name = b"Address:\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.domain.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_specifyserver.domain.generic.x = 206i32;
    s_specifyserver.domain.generic.y = 220i32;
    s_specifyserver.domain.field.widthInChars = 38i32;
    s_specifyserver.domain.field.maxchars = 80i32;
    s_specifyserver.port.generic.type_0 = 4i32;
    s_specifyserver.port.generic.name = b"Port:\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.port.generic.flags =
        0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint | 0x20i32 as libc::c_uint;
    s_specifyserver.port.generic.x = 206i32;
    s_specifyserver.port.generic.y = 250i32;
    s_specifyserver.port.field.widthInChars = 6i32;
    s_specifyserver.port.field.maxchars = 5i32;
    s_specifyserver.go.generic.type_0 = 6i32;
    s_specifyserver.go.generic.name = b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.go.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_specifyserver.go.generic.callback = Some(SpecifyServer_Event);
    s_specifyserver.go.generic.id = 103i32;
    s_specifyserver.go.generic.x = 640i32;
    s_specifyserver.go.generic.y = 480i32 - 64i32;
    s_specifyserver.go.width = 128i32;
    s_specifyserver.go.height = 64i32;
    s_specifyserver.go.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_specifyserver.back.generic.type_0 = 6i32;
    s_specifyserver.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_specifyserver.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_specifyserver.back.generic.callback = Some(SpecifyServer_Event);
    s_specifyserver.back.generic.id = 102i32;
    s_specifyserver.back.generic.x = 0i32;
    s_specifyserver.back.generic.y = 480i32 - 64i32;
    s_specifyserver.back.width = 128i32;
    s_specifyserver.back.height = 64i32;
    s_specifyserver.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut s_specifyserver.menu,
        &mut s_specifyserver.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu,
        &mut s_specifyserver.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu,
        &mut s_specifyserver.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu,
        &mut s_specifyserver.domain as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu,
        &mut s_specifyserver.port as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu,
        &mut s_specifyserver.go as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_specifyserver.menu,
        &mut s_specifyserver.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Com_sprintf(
        s_specifyserver.port.field.buffer.as_mut_ptr(),
        6i32,
        b"%i\x00" as *const u8 as *const libc::c_char,
        27960i32,
    );
}
/*
=================
SpecifyServer_Event
=================
*/
unsafe extern "C" fn SpecifyServer_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    let mut buff: [libc::c_char; 256] = [0; 256];
    match (*(ptr as *mut menucommon_s)).id {
        103 => {
            if !(event != 3i32) {
                if 0 != s_specifyserver.domain.field.buffer[0usize] {
                    strcpy(
                        buff.as_mut_ptr(),
                        s_specifyserver.domain.field.buffer.as_mut_ptr(),
                    );
                    if 0 != s_specifyserver.port.field.buffer[0usize] {
                        Com_sprintf(
                            buff.as_mut_ptr().offset(strlen(buff.as_mut_ptr()) as isize),
                            128i32,
                            b":%s\x00" as *const u8 as *const libc::c_char,
                            s_specifyserver.port.field.buffer.as_mut_ptr(),
                        );
                    }
                    trap_Cmd_ExecuteText(
                        EXEC_APPEND as libc::c_int,
                        va(
                            b"connect %s\n\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            buff.as_mut_ptr(),
                        ),
                    );
                }
            }
        }
        102 => {
            if !(event != 3i32) {
                UI_PopMenu();
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn SpecifyServer_Cache() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while !specifyserver_artlist[i as usize].is_null() {
        trap_R_RegisterShaderNoMip(specifyserver_artlist[i as usize]);
        i += 1
    }
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
/* ********************************************************************************
    SPECIFY SERVER
*********************************************************************************/
static mut specifyserver_artlist: [*mut libc::c_char; 7] = [
    b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct specifyserver_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub domain: menufield_s,
    pub port: menufield_s,
    pub go: menubitmap_s,
    pub back: menubitmap_s,
}
