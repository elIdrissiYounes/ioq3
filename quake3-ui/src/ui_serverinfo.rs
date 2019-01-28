use libc;
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, va, vec4_t, vec_t, Info_NextPair, Q_strcat,
    Q_stricmp,
};
use stdlib::memset;
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menutext_s, trap_Cvar_Set,
    trap_Cvar_VariableStringBuffer, trap_Cvar_VariableValue, trap_GetConfigString,
    trap_R_RegisterShaderNoMip,
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
use ui_servers2::{punkbuster_items, ArenaServers_Cache, UI_ArenaServersMenu};
use ui_setup::{UI_SetupMenu, UI_SetupMenu_Cache};
use ui_sound::{UI_SoundOptionsMenu, UI_SoundOptionsMenu_Cache};
use ui_sparena::UI_SPArena_Start;
use ui_specifyserver::{SpecifyServer_Cache, UI_SpecifyServerMenu};
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

//
// ui_serverinfo.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_ServerInfoMenu() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut value: [libc::c_char; 1024] = [0; 1024];
    memset(
        &mut s_serverinfo as *mut serverinfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<serverinfo_t>() as libc::c_ulong,
    );
    ServerInfo_Cache();
    s_serverinfo.menu.draw = Some(ServerInfo_MenuDraw);
    s_serverinfo.menu.key = Some(ServerInfo_MenuKey);
    s_serverinfo.menu.wrapAround = qtrue;
    s_serverinfo.menu.fullscreen = qtrue;
    s_serverinfo.banner.generic.type_0 = 10i32;
    s_serverinfo.banner.generic.x = 320i32;
    s_serverinfo.banner.generic.y = 16i32;
    s_serverinfo.banner.string =
        b"SERVER INFO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serverinfo.banner.color = color_white.as_mut_ptr();
    s_serverinfo.banner.style = 0x1i32;
    s_serverinfo.framel.generic.type_0 = 6i32;
    s_serverinfo.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_serverinfo.framel.generic.flags = 0x4000i32 as libc::c_uint;
    s_serverinfo.framel.generic.x = 0i32;
    s_serverinfo.framel.generic.y = 78i32;
    s_serverinfo.framel.width = 256i32;
    s_serverinfo.framel.height = 329i32;
    s_serverinfo.framer.generic.type_0 = 6i32;
    s_serverinfo.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_serverinfo.framer.generic.flags = 0x4000i32 as libc::c_uint;
    s_serverinfo.framer.generic.x = 376i32;
    s_serverinfo.framer.generic.y = 76i32;
    s_serverinfo.framer.width = 256i32;
    s_serverinfo.framer.height = 334i32;
    s_serverinfo.add.generic.type_0 = 9i32;
    s_serverinfo.add.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_serverinfo.add.generic.callback = Some(ServerInfo_Event);
    s_serverinfo.add.generic.id = 100i32;
    s_serverinfo.add.generic.x = 320i32;
    s_serverinfo.add.generic.y = 371i32;
    s_serverinfo.add.string =
        b"ADD TO FAVORITES\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serverinfo.add.style = 0x1i32 | 0x10i32;
    s_serverinfo.add.color = color_red.as_mut_ptr();
    if 0. != trap_Cvar_VariableValue(b"sv_running\x00" as *const u8 as *const libc::c_char) {
        s_serverinfo.add.generic.flags |= 0x2000i32 as libc::c_uint
    }
    s_serverinfo.back.generic.type_0 = 6i32;
    s_serverinfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_serverinfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_serverinfo.back.generic.callback = Some(ServerInfo_Event);
    s_serverinfo.back.generic.id = 101i32;
    s_serverinfo.back.generic.x = 0i32;
    s_serverinfo.back.generic.y = 480i32 - 64i32;
    s_serverinfo.back.width = 128i32;
    s_serverinfo.back.height = 64i32;
    s_serverinfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    trap_GetConfigString(0i32, s_serverinfo.info.as_mut_ptr(), 1024i32);
    s_serverinfo.numlines = 0i32;
    s = s_serverinfo.info.as_mut_ptr();
    while !s.is_null() {
        Info_NextPair(&mut s, key.as_mut_ptr(), value.as_mut_ptr());
        if 0 == key[0usize] {
            break;
        }
        s_serverinfo.numlines += 1
    }
    if s_serverinfo.numlines > 16i32 {
        s_serverinfo.numlines = 16i32
    }
    Menu_AddItem(
        &mut s_serverinfo.menu,
        &mut s_serverinfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serverinfo.menu,
        &mut s_serverinfo.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serverinfo.menu,
        &mut s_serverinfo.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serverinfo.menu,
        &mut s_serverinfo.add as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serverinfo.menu,
        &mut s_serverinfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
    UI_PushMenu(&mut s_serverinfo.menu);
}
static mut s_serverinfo: serverinfo_t = serverinfo_t {
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
    add: menutext_s {
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
    info: [0; 1024],
    numlines: 0,
};
/*
=================
ServerInfo_Event
=================
*/
unsafe extern "C" fn ServerInfo_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    match (*(ptr as *mut menucommon_s)).id {
        100 => {
            if !(event != 3i32) {
                Favorites_Add();
                UI_PopMenu();
            }
        }
        101 => {
            if !(event != 3i32) {
                UI_PopMenu();
            }
        }
        _ => {}
    };
}
/*
=================
Favorites_Add

Add current server to favorites
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Favorites_Add() {
    let mut adrstr: [libc::c_char; 128] = [0; 128];
    let mut serverbuff: [libc::c_char; 128] = [0; 128];
    let mut i: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    trap_Cvar_VariableStringBuffer(
        b"cl_currentServerAddress\x00" as *const u8 as *const libc::c_char,
        serverbuff.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
    );
    if 0 == serverbuff[0usize] {
        return;
    }
    best = 0i32;
    i = 0i32;
    while i < 16i32 {
        trap_Cvar_VariableStringBuffer(
            va(
                b"server%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i + 1i32,
            ),
            adrstr.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        );
        if 0 == Q_stricmp(serverbuff.as_mut_ptr(), adrstr.as_mut_ptr()) {
            return;
        }
        if 0 == adrstr[0usize] && 0 == best {
            best = i + 1i32
        }
        i += 1
    }
    if 0 != best {
        trap_Cvar_Set(
            va(
                b"server%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                best,
            ),
            serverbuff.as_mut_ptr(),
        );
    };
}
/*
=================
ServerInfo_MenuKey
=================
*/
unsafe extern "C" fn ServerInfo_MenuKey(mut key: libc::c_int) -> sfxHandle_t {
    return Menu_DefaultKey(&mut s_serverinfo.menu, key);
}
/*
=================
ServerInfo_MenuDraw
=================
*/
unsafe extern "C" fn ServerInfo_MenuDraw() {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut value: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0i32;
    let mut y: libc::c_int = 0;
    y = 480i32 / 2i32 - s_serverinfo.numlines * 16i32 / 2i32 - 20i32;
    s = s_serverinfo.info.as_mut_ptr();
    while !s.is_null() && i < s_serverinfo.numlines {
        Info_NextPair(&mut s, key.as_mut_ptr(), value.as_mut_ptr());
        if 0 == key[0usize] {
            break;
        }
        Q_strcat(
            key.as_mut_ptr(),
            1024i32,
            b":\x00" as *const u8 as *const libc::c_char,
        );
        UI_DrawString(
            (640i32 as libc::c_double * 0.50f64 - 8i32 as libc::c_double) as libc::c_int,
            y,
            key.as_mut_ptr(),
            0x2i32 | 0x10i32,
            color_red.as_mut_ptr(),
        );
        UI_DrawString(
            (640i32 as libc::c_double * 0.50f64 + 8i32 as libc::c_double) as libc::c_int,
            y,
            value.as_mut_ptr(),
            0i32 | 0x10i32,
            text_color_normal.as_mut_ptr(),
        );
        y += 16i32;
        i += 1
    }
    Menu_Draw(&mut s_serverinfo.menu);
}
#[no_mangle]
pub unsafe extern "C" fn ServerInfo_Cache() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while !serverinfo_artlist[i as usize].is_null() {
        trap_R_RegisterShaderNoMip(serverinfo_artlist[i as usize]);
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
static mut serverinfo_artlist: [*mut libc::c_char; 5] = [
    b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serverinfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub back: menubitmap_s,
    pub add: menutext_s,
    pub info: [libc::c_char; 1024],
    pub numlines: libc::c_int,
}
