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
use q_shared_h::{qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, vec4_t, vec_t};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menulist_s, menutext_s,
    trap_Cvar_SetValue, trap_Cvar_VariableValue, trap_R_RegisterShaderNoMip,
};
use ui_main::{
    ui_browserGameType, ui_browserMaster, ui_browserShowEmpty, ui_browserShowFull,
    ui_browserSortKey, ui_cdkeychecked, UI_RegisterCvars, UI_UpdateCvars,
};
use ui_menu::{MainMenu_Cache, UI_MainMenu};
use ui_mfield::{MField_Draw, MenuField_Draw, MenuField_Init, MenuField_Key};
use ui_mods::{UI_ModsMenu, UI_ModsMenu_Cache};
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
extern crate libc;

//
// ui_network.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_NetworkOptionsMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_NetworkOptionsMenu() {
    UI_NetworkOptionsMenu_Init();
    UI_PushMenu(&mut networkOptionsInfo.menu);
    Menu_SetCursorToItem(
        &mut networkOptionsInfo.menu,
        &mut networkOptionsInfo.network as *mut menutext_s as *mut libc::c_void,
    );
}
static mut networkOptionsInfo: networkOptionsInfo_t = networkOptionsInfo_t {
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
    graphics: menutext_s {
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
    display: menutext_s {
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
    sound: menutext_s {
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
    network: menutext_s {
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
    rate: menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
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
===============
UI_NetworkOptionsMenu_Init
===============
*/
unsafe extern "C" fn UI_NetworkOptionsMenu_Init() {
    let mut y: libc::c_int = 0;
    let mut rate: libc::c_int = 0;
    memset(
        &mut networkOptionsInfo as *mut networkOptionsInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<networkOptionsInfo_t>() as libc::c_ulong,
    );
    UI_NetworkOptionsMenu_Cache();
    networkOptionsInfo.menu.wrapAround = qtrue;
    networkOptionsInfo.menu.fullscreen = qtrue;
    networkOptionsInfo.banner.generic.type_0 = 10i32;
    networkOptionsInfo.banner.generic.flags = 0x8i32 as libc::c_uint;
    networkOptionsInfo.banner.generic.x = 320i32;
    networkOptionsInfo.banner.generic.y = 16i32;
    networkOptionsInfo.banner.string =
        b"SYSTEM SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    networkOptionsInfo.banner.color = color_white.as_mut_ptr();
    networkOptionsInfo.banner.style = 0x1i32;
    networkOptionsInfo.framel.generic.type_0 = 6i32;
    networkOptionsInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    networkOptionsInfo.framel.generic.flags = 0x4000i32 as libc::c_uint;
    networkOptionsInfo.framel.generic.x = 0i32;
    networkOptionsInfo.framel.generic.y = 78i32;
    networkOptionsInfo.framel.width = 256i32;
    networkOptionsInfo.framel.height = 329i32;
    networkOptionsInfo.framer.generic.type_0 = 6i32;
    networkOptionsInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    networkOptionsInfo.framer.generic.flags = 0x4000i32 as libc::c_uint;
    networkOptionsInfo.framer.generic.x = 376i32;
    networkOptionsInfo.framer.generic.y = 76i32;
    networkOptionsInfo.framer.width = 256i32;
    networkOptionsInfo.framer.height = 334i32;
    networkOptionsInfo.graphics.generic.type_0 = 9i32;
    networkOptionsInfo.graphics.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    networkOptionsInfo.graphics.generic.id = 10i32;
    networkOptionsInfo.graphics.generic.callback = Some(UI_NetworkOptionsMenu_Event);
    networkOptionsInfo.graphics.generic.x = 216i32;
    networkOptionsInfo.graphics.generic.y = 240i32 - 2i32 * 27i32;
    networkOptionsInfo.graphics.string =
        b"GRAPHICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    networkOptionsInfo.graphics.style = 0x2i32;
    networkOptionsInfo.graphics.color = color_red.as_mut_ptr();
    networkOptionsInfo.display.generic.type_0 = 9i32;
    networkOptionsInfo.display.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    networkOptionsInfo.display.generic.id = 11i32;
    networkOptionsInfo.display.generic.callback = Some(UI_NetworkOptionsMenu_Event);
    networkOptionsInfo.display.generic.x = 216i32;
    networkOptionsInfo.display.generic.y = 240i32 - 27i32;
    networkOptionsInfo.display.string =
        b"DISPLAY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    networkOptionsInfo.display.style = 0x2i32;
    networkOptionsInfo.display.color = color_red.as_mut_ptr();
    networkOptionsInfo.sound.generic.type_0 = 9i32;
    networkOptionsInfo.sound.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    networkOptionsInfo.sound.generic.id = 12i32;
    networkOptionsInfo.sound.generic.callback = Some(UI_NetworkOptionsMenu_Event);
    networkOptionsInfo.sound.generic.x = 216i32;
    networkOptionsInfo.sound.generic.y = 240i32;
    networkOptionsInfo.sound.string =
        b"SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    networkOptionsInfo.sound.style = 0x2i32;
    networkOptionsInfo.sound.color = color_red.as_mut_ptr();
    networkOptionsInfo.network.generic.type_0 = 9i32;
    networkOptionsInfo.network.generic.flags = 0x10i32 as libc::c_uint;
    networkOptionsInfo.network.generic.id = 13i32;
    networkOptionsInfo.network.generic.callback = Some(UI_NetworkOptionsMenu_Event);
    networkOptionsInfo.network.generic.x = 216i32;
    networkOptionsInfo.network.generic.y = 240i32 + 27i32;
    networkOptionsInfo.network.string =
        b"NETWORK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    networkOptionsInfo.network.style = 0x2i32;
    networkOptionsInfo.network.color = color_red.as_mut_ptr();
    y = 240i32 - 1i32 * (16i32 + 2i32);
    networkOptionsInfo.rate.generic.type_0 = 3i32;
    networkOptionsInfo.rate.generic.name = b"Data Rate:\x00" as *const u8 as *const libc::c_char;
    networkOptionsInfo.rate.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    networkOptionsInfo.rate.generic.callback = Some(UI_NetworkOptionsMenu_Event);
    networkOptionsInfo.rate.generic.id = 14i32;
    networkOptionsInfo.rate.generic.x = 400i32;
    networkOptionsInfo.rate.generic.y = y;
    networkOptionsInfo.rate.itemnames = rate_items.as_mut_ptr();
    networkOptionsInfo.back.generic.type_0 = 6i32;
    networkOptionsInfo.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    networkOptionsInfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    networkOptionsInfo.back.generic.callback = Some(UI_NetworkOptionsMenu_Event);
    networkOptionsInfo.back.generic.id = 15i32;
    networkOptionsInfo.back.generic.x = 0i32;
    networkOptionsInfo.back.generic.y = 480i32 - 64i32;
    networkOptionsInfo.back.width = 128i32;
    networkOptionsInfo.back.height = 64i32;
    networkOptionsInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut networkOptionsInfo.menu,
        &mut networkOptionsInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut networkOptionsInfo.menu,
        &mut networkOptionsInfo.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut networkOptionsInfo.menu,
        &mut networkOptionsInfo.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut networkOptionsInfo.menu,
        &mut networkOptionsInfo.graphics as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut networkOptionsInfo.menu,
        &mut networkOptionsInfo.display as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut networkOptionsInfo.menu,
        &mut networkOptionsInfo.sound as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut networkOptionsInfo.menu,
        &mut networkOptionsInfo.network as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut networkOptionsInfo.menu,
        &mut networkOptionsInfo.rate as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut networkOptionsInfo.menu,
        &mut networkOptionsInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
    rate = trap_Cvar_VariableValue(b"rate\x00" as *const u8 as *const libc::c_char) as libc::c_int;
    if rate <= 2500i32 {
        networkOptionsInfo.rate.curvalue = 0i32
    } else if rate <= 3000i32 {
        networkOptionsInfo.rate.curvalue = 1i32
    } else if rate <= 4000i32 {
        networkOptionsInfo.rate.curvalue = 2i32
    } else if rate <= 5000i32 {
        networkOptionsInfo.rate.curvalue = 3i32
    } else {
        networkOptionsInfo.rate.curvalue = 4i32
    };
}
/*
=================
UI_NetworkOptionsMenu_Event
=================
*/
unsafe extern "C" fn UI_NetworkOptionsMenu_Event(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        10 => {
            UI_PopMenu();
            UI_GraphicsOptionsMenu();
        }
        11 => {
            UI_PopMenu();
            UI_DisplayOptionsMenu();
        }
        12 => {
            UI_PopMenu();
            UI_SoundOptionsMenu();
        }
        14 => {
            if networkOptionsInfo.rate.curvalue == 0i32 {
                trap_Cvar_SetValue(
                    b"rate\x00" as *const u8 as *const libc::c_char,
                    2500i32 as libc::c_float,
                );
            } else if networkOptionsInfo.rate.curvalue == 1i32 {
                trap_Cvar_SetValue(
                    b"rate\x00" as *const u8 as *const libc::c_char,
                    3000i32 as libc::c_float,
                );
            } else if networkOptionsInfo.rate.curvalue == 2i32 {
                trap_Cvar_SetValue(
                    b"rate\x00" as *const u8 as *const libc::c_char,
                    4000i32 as libc::c_float,
                );
            } else if networkOptionsInfo.rate.curvalue == 3i32 {
                trap_Cvar_SetValue(
                    b"rate\x00" as *const u8 as *const libc::c_char,
                    5000i32 as libc::c_float,
                );
            } else if networkOptionsInfo.rate.curvalue == 4i32 {
                trap_Cvar_SetValue(
                    b"rate\x00" as *const u8 as *const libc::c_char,
                    25000i32 as libc::c_float,
                );
            }
        }
        15 => {
            UI_PopMenu();
        }
        13 | _ => {}
    };
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
/*
=======================================================================

NETWORK OPTIONS MENU

=======================================================================
*/
static mut rate_items: [*const libc::c_char; 6] = [
    b"<= 28.8K\x00" as *const u8 as *const libc::c_char,
    b"33.6K\x00" as *const u8 as *const libc::c_char,
    b"56K\x00" as *const u8 as *const libc::c_char,
    b"ISDN\x00" as *const u8 as *const libc::c_char,
    b"LAN/Cable/xDSL\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct networkOptionsInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub graphics: menutext_s,
    pub display: menutext_s,
    pub sound: menutext_s,
    pub network: menutext_s,
    pub rate: menulist_s,
    pub back: menubitmap_s,
}
