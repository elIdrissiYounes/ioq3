use bg_misc::bg_itemlist;
use libc;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
    AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
    AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t, Com_sprintf,
    Q_stricmp, Q_strncpyz, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::{memset, strchr, strlen, strtol};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, textureCompression_t, GLDRV_ICD,
    GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC, GLHW_PERMEDIA2, GLHW_RAGEPRO,
    GLHW_RIVA128, TC_NONE, TC_S3TC, TC_S3TC_ARB,
};
use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
use ui_atoms::{
    uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
    UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic, UI_DrawNamedPic,
    UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped, UI_DrawString, UI_FillRect,
    UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent, UI_MouseEvent, UI_PopMenu,
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menulist_s, menuslider_s,
    menutext_s, trap_Cmd_ExecuteText, trap_Cvar_Reset, trap_Cvar_Set, trap_Cvar_SetValue,
    trap_Cvar_VariableStringBuffer, trap_Cvar_VariableValue, trap_R_RegisterShaderNoMip,
    uiStatic_t,
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

unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10i32,
    ) as libc::c_int;
}
//
// ui_video.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_GraphicsOptionsMenu() {
    GraphicsOptions_MenuInit();
    UI_PushMenu(&mut s_graphicsoptions.menu);
    Menu_SetCursorToItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.graphics as *mut menutext_s as *mut libc::c_void,
    );
}
static mut s_graphicsoptions: graphicsoptions_t = graphicsoptions_t {
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
    list: menulist_s {
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
    ratio: menulist_s {
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
    mode: menulist_s {
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
    driver: menulist_s {
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
    tq: menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    fs: menulist_s {
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
    lighting: menulist_s {
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
    allow_extensions: menulist_s {
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
    texturebits: menulist_s {
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
    colordepth: menulist_s {
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
    geometry: menulist_s {
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
    filter: menulist_s {
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
    driverinfo: menutext_s {
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
    apply: menubitmap_s {
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
#[no_mangle]
pub unsafe extern "C" fn GraphicsOptions_MenuInit() {
    static mut s_driver_names: [*const libc::c_char; 3] = [
        b"Default\x00" as *const u8 as *const libc::c_char,
        b"Voodoo\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut tq_names: [*const libc::c_char; 4] = [
        b"Default\x00" as *const u8 as *const libc::c_char,
        b"16 bit\x00" as *const u8 as *const libc::c_char,
        b"32 bit\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut s_graphics_options_names: [*const libc::c_char; 7] = [
        b"Very High Quality\x00" as *const u8 as *const libc::c_char,
        b"High Quality\x00" as *const u8 as *const libc::c_char,
        b"Normal\x00" as *const u8 as *const libc::c_char,
        b"Fast\x00" as *const u8 as *const libc::c_char,
        b"Fastest\x00" as *const u8 as *const libc::c_char,
        b"Custom\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut lighting_names: [*const libc::c_char; 3] = [
        b"Lightmap\x00" as *const u8 as *const libc::c_char,
        b"Vertex\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut colordepth_names: [*const libc::c_char; 4] = [
        b"Default\x00" as *const u8 as *const libc::c_char,
        b"16 bit\x00" as *const u8 as *const libc::c_char,
        b"32 bit\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut filter_names: [*const libc::c_char; 3] = [
        b"Bilinear\x00" as *const u8 as *const libc::c_char,
        b"Trilinear\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut quality_names: [*const libc::c_char; 4] = [
        b"Low\x00" as *const u8 as *const libc::c_char,
        b"Medium\x00" as *const u8 as *const libc::c_char,
        b"High\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut enabled_names: [*const libc::c_char; 3] = [
        b"Off\x00" as *const u8 as *const libc::c_char,
        b"On\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut y: libc::c_int = 0;
    memset(
        &mut s_graphicsoptions as *mut graphicsoptions_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<graphicsoptions_t>() as libc::c_ulong,
    );
    GraphicsOptions_GetResolutions();
    GraphicsOptions_GetAspectRatios();
    GraphicsOptions_Cache();
    s_graphicsoptions.menu.wrapAround = qtrue;
    s_graphicsoptions.menu.fullscreen = qtrue;
    s_graphicsoptions.menu.draw = Some(GraphicsOptions_MenuDraw);
    s_graphicsoptions.banner.generic.type_0 = 10i32;
    s_graphicsoptions.banner.generic.x = 320i32;
    s_graphicsoptions.banner.generic.y = 16i32;
    s_graphicsoptions.banner.string =
        b"SYSTEM SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.banner.color = color_white.as_mut_ptr();
    s_graphicsoptions.banner.style = 0x1i32;
    s_graphicsoptions.framel.generic.type_0 = 6i32;
    s_graphicsoptions.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.framel.generic.flags = 0x4000i32 as libc::c_uint;
    s_graphicsoptions.framel.generic.x = 0i32;
    s_graphicsoptions.framel.generic.y = 78i32;
    s_graphicsoptions.framel.width = 256i32;
    s_graphicsoptions.framel.height = 329i32;
    s_graphicsoptions.framer.generic.type_0 = 6i32;
    s_graphicsoptions.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.framer.generic.flags = 0x4000i32 as libc::c_uint;
    s_graphicsoptions.framer.generic.x = 376i32;
    s_graphicsoptions.framer.generic.y = 76i32;
    s_graphicsoptions.framer.width = 256i32;
    s_graphicsoptions.framer.height = 334i32;
    s_graphicsoptions.graphics.generic.type_0 = 9i32;
    s_graphicsoptions.graphics.generic.flags = 0x10i32 as libc::c_uint;
    s_graphicsoptions.graphics.generic.id = 106i32;
    s_graphicsoptions.graphics.generic.callback = Some(GraphicsOptions_Event);
    s_graphicsoptions.graphics.generic.x = 216i32;
    s_graphicsoptions.graphics.generic.y = 240i32 - 2i32 * 27i32;
    s_graphicsoptions.graphics.string =
        b"GRAPHICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.graphics.style = 0x2i32;
    s_graphicsoptions.graphics.color = color_red.as_mut_ptr();
    s_graphicsoptions.display.generic.type_0 = 9i32;
    s_graphicsoptions.display.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_graphicsoptions.display.generic.id = 107i32;
    s_graphicsoptions.display.generic.callback = Some(GraphicsOptions_Event);
    s_graphicsoptions.display.generic.x = 216i32;
    s_graphicsoptions.display.generic.y = 240i32 - 27i32;
    s_graphicsoptions.display.string =
        b"DISPLAY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.display.style = 0x2i32;
    s_graphicsoptions.display.color = color_red.as_mut_ptr();
    s_graphicsoptions.sound.generic.type_0 = 9i32;
    s_graphicsoptions.sound.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_graphicsoptions.sound.generic.id = 108i32;
    s_graphicsoptions.sound.generic.callback = Some(GraphicsOptions_Event);
    s_graphicsoptions.sound.generic.x = 216i32;
    s_graphicsoptions.sound.generic.y = 240i32;
    s_graphicsoptions.sound.string =
        b"SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.sound.style = 0x2i32;
    s_graphicsoptions.sound.color = color_red.as_mut_ptr();
    s_graphicsoptions.network.generic.type_0 = 9i32;
    s_graphicsoptions.network.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_graphicsoptions.network.generic.id = 109i32;
    s_graphicsoptions.network.generic.callback = Some(GraphicsOptions_Event);
    s_graphicsoptions.network.generic.x = 216i32;
    s_graphicsoptions.network.generic.y = 240i32 + 27i32;
    s_graphicsoptions.network.string =
        b"NETWORK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.network.style = 0x2i32;
    s_graphicsoptions.network.color = color_red.as_mut_ptr();
    y = 240i32 - 7i32 * (16i32 + 2i32);
    s_graphicsoptions.list.generic.type_0 = 3i32;
    s_graphicsoptions.list.generic.name =
        b"Graphics Settings:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.list.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.list.generic.x = 400i32;
    s_graphicsoptions.list.generic.y = y;
    s_graphicsoptions.list.generic.callback = Some(GraphicsOptions_Event);
    s_graphicsoptions.list.generic.id = 103i32;
    s_graphicsoptions.list.itemnames = s_graphics_options_names.as_mut_ptr();
    y += 2i32 * (16i32 + 2i32);
    s_graphicsoptions.driver.generic.type_0 = 3i32;
    s_graphicsoptions.driver.generic.name = b"GL Driver:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.driver.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.driver.generic.x = 400i32;
    s_graphicsoptions.driver.generic.y = y;
    s_graphicsoptions.driver.itemnames = s_driver_names.as_mut_ptr();
    s_graphicsoptions.driver.curvalue = (uis.glconfig.driverType as libc::c_uint
        == GLDRV_VOODOO as libc::c_int as libc::c_uint)
        as libc::c_int;
    y += 16i32 + 2i32;
    s_graphicsoptions.allow_extensions.generic.type_0 = 3i32;
    s_graphicsoptions.allow_extensions.generic.name =
        b"GL Extensions:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.allow_extensions.generic.flags =
        0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.allow_extensions.generic.x = 400i32;
    s_graphicsoptions.allow_extensions.generic.y = y;
    s_graphicsoptions.allow_extensions.itemnames = enabled_names.as_mut_ptr();
    y += 16i32 + 2i32;
    s_graphicsoptions.ratio.generic.type_0 = 3i32;
    s_graphicsoptions.ratio.generic.name = b"Aspect Ratio:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.ratio.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.ratio.generic.x = 400i32;
    s_graphicsoptions.ratio.generic.y = y;
    s_graphicsoptions.ratio.itemnames = ratios.as_mut_ptr();
    s_graphicsoptions.ratio.generic.callback = Some(GraphicsOptions_Event);
    s_graphicsoptions.ratio.generic.id = 110i32;
    y += 16i32 + 2i32;
    s_graphicsoptions.mode.generic.type_0 = 3i32;
    s_graphicsoptions.mode.generic.name = b"Resolution:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.mode.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.mode.generic.x = 400i32;
    s_graphicsoptions.mode.generic.y = y;
    s_graphicsoptions.mode.itemnames = resolutions;
    s_graphicsoptions.mode.generic.callback = Some(GraphicsOptions_Event);
    s_graphicsoptions.mode.generic.id = 104i32;
    y += 16i32 + 2i32;
    s_graphicsoptions.colordepth.generic.type_0 = 3i32;
    s_graphicsoptions.colordepth.generic.name =
        b"Color Depth:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.colordepth.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.colordepth.generic.x = 400i32;
    s_graphicsoptions.colordepth.generic.y = y;
    s_graphicsoptions.colordepth.itemnames = colordepth_names.as_mut_ptr();
    y += 16i32 + 2i32;
    s_graphicsoptions.fs.generic.type_0 = 3i32;
    s_graphicsoptions.fs.generic.name = b"Fullscreen:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.fs.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.fs.generic.x = 400i32;
    s_graphicsoptions.fs.generic.y = y;
    s_graphicsoptions.fs.itemnames = enabled_names.as_mut_ptr();
    y += 16i32 + 2i32;
    s_graphicsoptions.lighting.generic.type_0 = 3i32;
    s_graphicsoptions.lighting.generic.name = b"Lighting:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.lighting.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.lighting.generic.x = 400i32;
    s_graphicsoptions.lighting.generic.y = y;
    s_graphicsoptions.lighting.itemnames = lighting_names.as_mut_ptr();
    y += 16i32 + 2i32;
    s_graphicsoptions.geometry.generic.type_0 = 3i32;
    s_graphicsoptions.geometry.generic.name =
        b"Geometric Detail:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.geometry.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.geometry.generic.x = 400i32;
    s_graphicsoptions.geometry.generic.y = y;
    s_graphicsoptions.geometry.itemnames = quality_names.as_mut_ptr();
    y += 16i32 + 2i32;
    s_graphicsoptions.tq.generic.type_0 = 1i32;
    s_graphicsoptions.tq.generic.name = b"Texture Detail:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.tq.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.tq.generic.x = 400i32;
    s_graphicsoptions.tq.generic.y = y;
    s_graphicsoptions.tq.minvalue = 0i32 as libc::c_float;
    s_graphicsoptions.tq.maxvalue = 3i32 as libc::c_float;
    s_graphicsoptions.tq.generic.callback = Some(GraphicsOptions_TQEvent);
    y += 16i32 + 2i32;
    s_graphicsoptions.texturebits.generic.type_0 = 3i32;
    s_graphicsoptions.texturebits.generic.name =
        b"Texture Quality:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.texturebits.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.texturebits.generic.x = 400i32;
    s_graphicsoptions.texturebits.generic.y = y;
    s_graphicsoptions.texturebits.itemnames = tq_names.as_mut_ptr();
    y += 16i32 + 2i32;
    s_graphicsoptions.filter.generic.type_0 = 3i32;
    s_graphicsoptions.filter.generic.name =
        b"Texture Filter:\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.filter.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_graphicsoptions.filter.generic.x = 400i32;
    s_graphicsoptions.filter.generic.y = y;
    s_graphicsoptions.filter.itemnames = filter_names.as_mut_ptr();
    y += 2i32 * 16i32;
    s_graphicsoptions.driverinfo.generic.type_0 = 9i32;
    s_graphicsoptions.driverinfo.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_graphicsoptions.driverinfo.generic.callback = Some(GraphicsOptions_Event);
    s_graphicsoptions.driverinfo.generic.id = 105i32;
    s_graphicsoptions.driverinfo.generic.x = 320i32;
    s_graphicsoptions.driverinfo.generic.y = y;
    s_graphicsoptions.driverinfo.string =
        b"Driver Info\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.driverinfo.style = 0x1i32 | 0x10i32;
    s_graphicsoptions.driverinfo.color = color_red.as_mut_ptr();
    s_graphicsoptions.back.generic.type_0 = 6i32;
    s_graphicsoptions.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_graphicsoptions.back.generic.callback = Some(GraphicsOptions_Event);
    s_graphicsoptions.back.generic.id = 101i32;
    s_graphicsoptions.back.generic.x = 0i32;
    s_graphicsoptions.back.generic.y = 480i32 - 64i32;
    s_graphicsoptions.back.width = 128i32;
    s_graphicsoptions.back.height = 64i32;
    s_graphicsoptions.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_graphicsoptions.apply.generic.type_0 = 6i32;
    s_graphicsoptions.apply.generic.name =
        b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char;
    s_graphicsoptions.apply.generic.flags = 0x10i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint
        | 0x4000i32 as libc::c_uint;
    s_graphicsoptions.apply.generic.callback = Some(GraphicsOptions_ApplyChanges);
    s_graphicsoptions.apply.generic.x = 640i32;
    s_graphicsoptions.apply.generic.y = 480i32 - 64i32;
    s_graphicsoptions.apply.width = 128i32;
    s_graphicsoptions.apply.height = 64i32;
    s_graphicsoptions.apply.focuspic =
        b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.graphics as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.display as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.sound as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.network as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.list as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.driver as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.allow_extensions as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.ratio as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.mode as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.colordepth as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.fs as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.lighting as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.geometry as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.tq as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.texturebits as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.filter as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.driverinfo as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.apply as *mut menubitmap_s as *mut libc::c_void,
    );
    GraphicsOptions_SetMenuItems();
    GraphicsOptions_GetInitialVideo();
    if uis.glconfig.driverType as libc::c_uint == GLDRV_ICD as libc::c_int as libc::c_uint
        && uis.glconfig.hardwareType as libc::c_uint
            == GLHW_3DFX_2D3D as libc::c_int as libc::c_uint
    {
        s_graphicsoptions.driver.generic.flags |=
            0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint
    };
}
/*
=================
GraphicsOptions_GetInitialVideo
=================
*/
unsafe extern "C" fn GraphicsOptions_GetInitialVideo() {
    s_ivo.colordepth = s_graphicsoptions.colordepth.curvalue;
    s_ivo.driver = s_graphicsoptions.driver.curvalue;
    s_ivo.mode = s_graphicsoptions.mode.curvalue;
    s_ivo.fullscreen = s_graphicsoptions.fs.curvalue as qboolean;
    s_ivo.extensions = s_graphicsoptions.allow_extensions.curvalue as qboolean;
    s_ivo.tq = s_graphicsoptions.tq.curvalue as libc::c_int;
    s_ivo.lighting = s_graphicsoptions.lighting.curvalue;
    s_ivo.geometry = s_graphicsoptions.geometry.curvalue;
    s_ivo.filter = s_graphicsoptions.filter.curvalue;
    s_ivo.texturebits = s_graphicsoptions.texturebits.curvalue;
}
static mut s_ivo: InitialVideoOptions_s = InitialVideoOptions_s {
    mode: 0,
    fullscreen: qfalse,
    tq: 0,
    lighting: 0,
    colordepth: 0,
    texturebits: 0,
    geometry: 0,
    filter: 0,
    driver: 0,
    extensions: qfalse,
};
/*
=================
GraphicsOptions_SetMenuItems
=================
*/
unsafe extern "C" fn GraphicsOptions_SetMenuItems() {
    s_graphicsoptions.mode.curvalue = GraphicsOptions_FindDetectedResolution(
        trap_Cvar_VariableValue(b"r_mode\x00" as *const u8 as *const libc::c_char) as libc::c_int,
    );
    if s_graphicsoptions.mode.curvalue < 0i32 {
        if 0 != resolutionsDetected as u64 {
            let mut i: libc::c_int = 0;
            let mut buf: [libc::c_char; 1024] = [0; 1024];
            trap_Cvar_VariableStringBuffer(
                b"r_customwidth\x00" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(2i32 as libc::c_ulong) as libc::c_int,
            );
            buf[strlen(buf.as_mut_ptr()).wrapping_add(1i32 as libc::c_ulong) as usize] =
                0i32 as libc::c_char;
            buf[strlen(buf.as_mut_ptr()) as usize] = 'x' as i32 as libc::c_char;
            trap_Cvar_VariableStringBuffer(
                b"r_customheight\x00" as *const u8 as *const libc::c_char,
                buf.as_mut_ptr().offset(strlen(buf.as_mut_ptr()) as isize),
                (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                    .wrapping_sub(strlen(buf.as_mut_ptr())) as libc::c_int,
            );
            i = 0i32;
            while !detectedResolutions[i as usize].is_null() {
                if 0 == Q_stricmp(buf.as_mut_ptr(), detectedResolutions[i as usize]) {
                    s_graphicsoptions.mode.curvalue = i;
                    break;
                } else {
                    i += 1
                }
            }
            if s_graphicsoptions.mode.curvalue < 0i32 {
                s_graphicsoptions.mode.curvalue = 0i32
            }
        } else {
            s_graphicsoptions.mode.curvalue = 3i32
        }
    }
    s_graphicsoptions.ratio.curvalue = resToRatio[s_graphicsoptions.mode.curvalue as usize];
    s_graphicsoptions.fs.curvalue =
        trap_Cvar_VariableValue(b"r_fullscreen\x00" as *const u8 as *const libc::c_char)
            as libc::c_int;
    s_graphicsoptions.allow_extensions.curvalue =
        trap_Cvar_VariableValue(b"r_allowExtensions\x00" as *const u8 as *const libc::c_char)
            as libc::c_int;
    s_graphicsoptions.tq.curvalue = 3i32 as libc::c_float
        - trap_Cvar_VariableValue(b"r_picmip\x00" as *const u8 as *const libc::c_char);
    if s_graphicsoptions.tq.curvalue < 0i32 as libc::c_float {
        s_graphicsoptions.tq.curvalue = 0i32 as libc::c_float
    } else if s_graphicsoptions.tq.curvalue > 3i32 as libc::c_float {
        s_graphicsoptions.tq.curvalue = 3i32 as libc::c_float
    }
    s_graphicsoptions.lighting.curvalue =
        (trap_Cvar_VariableValue(b"r_vertexLight\x00" as *const u8 as *const libc::c_char)
            != 0i32 as libc::c_float) as libc::c_int;
    match trap_Cvar_VariableValue(b"r_texturebits\x00" as *const u8 as *const libc::c_char)
        as libc::c_int
    {
        16 => s_graphicsoptions.texturebits.curvalue = 1i32,
        32 => s_graphicsoptions.texturebits.curvalue = 2i32,
        0 | _ => s_graphicsoptions.texturebits.curvalue = 0i32,
    }
    if 0 == Q_stricmp(
        UI_Cvar_VariableString(b"r_textureMode\x00" as *const u8 as *const libc::c_char),
        b"GL_LINEAR_MIPMAP_NEAREST\x00" as *const u8 as *const libc::c_char,
    ) {
        s_graphicsoptions.filter.curvalue = 0i32
    } else {
        s_graphicsoptions.filter.curvalue = 1i32
    }
    if trap_Cvar_VariableValue(b"r_lodBias\x00" as *const u8 as *const libc::c_char)
        > 0i32 as libc::c_float
    {
        if trap_Cvar_VariableValue(b"r_subdivisions\x00" as *const u8 as *const libc::c_char)
            >= 20i32 as libc::c_float
        {
            s_graphicsoptions.geometry.curvalue = 0i32
        } else {
            s_graphicsoptions.geometry.curvalue = 1i32
        }
    } else {
        s_graphicsoptions.geometry.curvalue = 2i32
    }
    match trap_Cvar_VariableValue(b"r_colorbits\x00" as *const u8 as *const libc::c_char)
        as libc::c_int
    {
        16 => s_graphicsoptions.colordepth.curvalue = 1i32,
        32 => s_graphicsoptions.colordepth.curvalue = 2i32,
        0 | _ => s_graphicsoptions.colordepth.curvalue = 0i32,
    }
    if s_graphicsoptions.fs.curvalue == 0i32 {
        s_graphicsoptions.colordepth.curvalue = 0i32
    }
    if s_graphicsoptions.driver.curvalue == 1i32 {
        s_graphicsoptions.colordepth.curvalue = 1i32
    };
}
static mut resToRatio: [libc::c_int; 32] = [0; 32];
static mut detectedResolutions: [*const libc::c_char; 32] = [0 as *const libc::c_char; 32];
static mut resolutionsDetected: qboolean = qfalse;
/*
=================
GraphicsOptions_FindDetectedResolution
=================
*/
unsafe extern "C" fn GraphicsOptions_FindDetectedResolution(mut mode: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if 0 == resolutionsDetected as u64 {
        return mode;
    }
    if mode < 0i32 {
        return -1i32;
    }
    i = 0i32;
    while !detectedResolutions[i as usize].is_null() {
        if 0 == Q_stricmp(
            builtinResolutions[mode as usize],
            detectedResolutions[i as usize],
        ) {
            return i;
        }
        i += 1
    }
    return -1i32;
}
// JDC: this was tq 3
static mut builtinResolutions: [*const libc::c_char; 13] = [
    b"320x240\x00" as *const u8 as *const libc::c_char,
    b"400x300\x00" as *const u8 as *const libc::c_char,
    b"512x384\x00" as *const u8 as *const libc::c_char,
    b"640x480\x00" as *const u8 as *const libc::c_char,
    b"800x600\x00" as *const u8 as *const libc::c_char,
    b"960x720\x00" as *const u8 as *const libc::c_char,
    b"1024x768\x00" as *const u8 as *const libc::c_char,
    b"1152x864\x00" as *const u8 as *const libc::c_char,
    b"1280x1024\x00" as *const u8 as *const libc::c_char,
    b"1600x1200\x00" as *const u8 as *const libc::c_char,
    b"2048x1536\x00" as *const u8 as *const libc::c_char,
    b"856x480\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
GraphicsOptions_ApplyChanges
=================
*/
unsafe extern "C" fn GraphicsOptions_ApplyChanges(
    mut unused: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3i32 {
        return;
    }
    match s_graphicsoptions.texturebits.curvalue {
        0 => {
            trap_Cvar_SetValue(
                b"r_texturebits\x00" as *const u8 as *const libc::c_char,
                0i32 as libc::c_float,
            );
        }
        1 => {
            trap_Cvar_SetValue(
                b"r_texturebits\x00" as *const u8 as *const libc::c_char,
                16i32 as libc::c_float,
            );
        }
        2 => {
            trap_Cvar_SetValue(
                b"r_texturebits\x00" as *const u8 as *const libc::c_char,
                32i32 as libc::c_float,
            );
        }
        _ => {}
    }
    trap_Cvar_SetValue(
        b"r_picmip\x00" as *const u8 as *const libc::c_char,
        3i32 as libc::c_float - s_graphicsoptions.tq.curvalue,
    );
    trap_Cvar_SetValue(
        b"r_allowExtensions\x00" as *const u8 as *const libc::c_char,
        s_graphicsoptions.allow_extensions.curvalue as libc::c_float,
    );
    if 0 != resolutionsDetected as u64 {
        let mut mode: libc::c_int = 0;
        if s_graphicsoptions.mode.curvalue == -1i32
            || s_graphicsoptions.mode.curvalue as libc::c_ulong
                >= (::std::mem::size_of::<[*const libc::c_char; 32]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
        {
            s_graphicsoptions.mode.curvalue = 0i32
        }
        mode = GraphicsOptions_FindBuiltinResolution(s_graphicsoptions.mode.curvalue);
        if mode == -1i32 {
            let mut w: [libc::c_char; 16] = [0; 16];
            let mut h: [libc::c_char; 16] = [0; 16];
            Q_strncpyz(
                w.as_mut_ptr(),
                detectedResolutions[s_graphicsoptions.mode.curvalue as usize],
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
            );
            *strchr(w.as_mut_ptr(), 'x' as i32) = 0i32 as libc::c_char;
            Q_strncpyz(
                h.as_mut_ptr(),
                strchr(
                    detectedResolutions[s_graphicsoptions.mode.curvalue as usize],
                    'x' as i32,
                )
                .offset(1isize),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
            );
            trap_Cvar_Set(
                b"r_customwidth\x00" as *const u8 as *const libc::c_char,
                w.as_mut_ptr(),
            );
            trap_Cvar_Set(
                b"r_customheight\x00" as *const u8 as *const libc::c_char,
                h.as_mut_ptr(),
            );
        }
        trap_Cvar_SetValue(
            b"r_mode\x00" as *const u8 as *const libc::c_char,
            mode as libc::c_float,
        );
    } else {
        trap_Cvar_SetValue(
            b"r_mode\x00" as *const u8 as *const libc::c_char,
            s_graphicsoptions.mode.curvalue as libc::c_float,
        );
    }
    trap_Cvar_SetValue(
        b"r_fullscreen\x00" as *const u8 as *const libc::c_char,
        s_graphicsoptions.fs.curvalue as libc::c_float,
    );
    match s_graphicsoptions.colordepth.curvalue {
        0 => {
            trap_Cvar_SetValue(
                b"r_colorbits\x00" as *const u8 as *const libc::c_char,
                0i32 as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"r_depthbits\x00" as *const u8 as *const libc::c_char,
                0i32 as libc::c_float,
            );
            trap_Cvar_Reset(b"r_stencilbits\x00" as *const u8 as *const libc::c_char);
        }
        1 => {
            trap_Cvar_SetValue(
                b"r_colorbits\x00" as *const u8 as *const libc::c_char,
                16i32 as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"r_depthbits\x00" as *const u8 as *const libc::c_char,
                16i32 as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"r_stencilbits\x00" as *const u8 as *const libc::c_char,
                0i32 as libc::c_float,
            );
        }
        2 => {
            trap_Cvar_SetValue(
                b"r_colorbits\x00" as *const u8 as *const libc::c_char,
                32i32 as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"r_depthbits\x00" as *const u8 as *const libc::c_char,
                24i32 as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"r_stencilbits\x00" as *const u8 as *const libc::c_char,
                8i32 as libc::c_float,
            );
        }
        _ => {}
    }
    trap_Cvar_SetValue(
        b"r_vertexLight\x00" as *const u8 as *const libc::c_char,
        s_graphicsoptions.lighting.curvalue as libc::c_float,
    );
    if s_graphicsoptions.geometry.curvalue == 2i32 {
        trap_Cvar_SetValue(
            b"r_lodBias\x00" as *const u8 as *const libc::c_char,
            0i32 as libc::c_float,
        );
        trap_Cvar_SetValue(
            b"r_subdivisions\x00" as *const u8 as *const libc::c_char,
            4i32 as libc::c_float,
        );
    } else if s_graphicsoptions.geometry.curvalue == 1i32 {
        trap_Cvar_SetValue(
            b"r_lodBias\x00" as *const u8 as *const libc::c_char,
            1i32 as libc::c_float,
        );
        trap_Cvar_SetValue(
            b"r_subdivisions\x00" as *const u8 as *const libc::c_char,
            12i32 as libc::c_float,
        );
    } else {
        trap_Cvar_SetValue(
            b"r_lodBias\x00" as *const u8 as *const libc::c_char,
            1i32 as libc::c_float,
        );
        trap_Cvar_SetValue(
            b"r_subdivisions\x00" as *const u8 as *const libc::c_char,
            20i32 as libc::c_float,
        );
    }
    if 0 != s_graphicsoptions.filter.curvalue {
        trap_Cvar_Set(
            b"r_textureMode\x00" as *const u8 as *const libc::c_char,
            b"GL_LINEAR_MIPMAP_LINEAR\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        trap_Cvar_Set(
            b"r_textureMode\x00" as *const u8 as *const libc::c_char,
            b"GL_LINEAR_MIPMAP_NEAREST\x00" as *const u8 as *const libc::c_char,
        );
    }
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        b"vid_restart\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
GraphicsOptions_FindBuiltinResolution
=================
*/
unsafe extern "C" fn GraphicsOptions_FindBuiltinResolution(mut mode: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if 0 == resolutionsDetected as u64 {
        return mode;
    }
    if mode < 0i32 {
        return -1i32;
    }
    i = 0i32;
    while !builtinResolutions[i as usize].is_null() {
        if 0 == Q_stricmp(
            builtinResolutions[i as usize],
            detectedResolutions[mode as usize],
        ) {
            return i;
        }
        i += 1
    }
    return -1i32;
}
/*
=================
GraphicsOptions_Event
=================
*/
unsafe extern "C" fn GraphicsOptions_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    let mut ivo: *mut InitialVideoOptions_s = 0 as *mut InitialVideoOptions_s;
    if event != 3i32 {
        return;
    }
    let mut current_block_28: u64;
    match (*(ptr as *mut menucommon_s)).id {
        110 => {
            s_graphicsoptions.mode.curvalue = ratioToRes[s_graphicsoptions.ratio.curvalue as usize];
            // fall through to apply mode constraints
            current_block_28 = 16640963930099419567;
        }
        104 => {
            current_block_28 = 16640963930099419567;
        }
        103 => {
            ivo = &mut s_ivo_templates[s_graphicsoptions.list.curvalue as usize]
                as *mut InitialVideoOptions_s;
            s_graphicsoptions.mode.curvalue = GraphicsOptions_FindDetectedResolution((*ivo).mode);
            s_graphicsoptions.ratio.curvalue = resToRatio[s_graphicsoptions.mode.curvalue as usize];
            s_graphicsoptions.tq.curvalue = (*ivo).tq as libc::c_float;
            s_graphicsoptions.lighting.curvalue = (*ivo).lighting;
            s_graphicsoptions.colordepth.curvalue = (*ivo).colordepth;
            s_graphicsoptions.texturebits.curvalue = (*ivo).texturebits;
            s_graphicsoptions.geometry.curvalue = (*ivo).geometry;
            s_graphicsoptions.filter.curvalue = (*ivo).filter;
            s_graphicsoptions.fs.curvalue = (*ivo).fullscreen as libc::c_int;
            current_block_28 = 652864300344834934;
        }
        105 => {
            UI_DriverInfo_Menu();
            current_block_28 = 652864300344834934;
        }
        101 => {
            UI_PopMenu();
            current_block_28 = 652864300344834934;
        }
        107 => {
            UI_PopMenu();
            UI_DisplayOptionsMenu();
            current_block_28 = 652864300344834934;
        }
        108 => {
            UI_PopMenu();
            UI_SoundOptionsMenu();
            current_block_28 = 652864300344834934;
        }
        109 => {
            UI_PopMenu();
            UI_NetworkOptionsMenu();
            current_block_28 = 652864300344834934;
        }
        106 | _ => {
            current_block_28 = 652864300344834934;
        }
    }
    match current_block_28 {
        16640963930099419567 => {
            if s_graphicsoptions.driver.curvalue == 1i32 {
                if s_graphicsoptions.mode.curvalue < 2i32 {
                    s_graphicsoptions.mode.curvalue = 2i32
                } else if s_graphicsoptions.mode.curvalue > 6i32 {
                    s_graphicsoptions.mode.curvalue = 6i32
                }
            }
            s_graphicsoptions.ratio.curvalue = resToRatio[s_graphicsoptions.mode.curvalue as usize]
        }
        _ => {}
    };
}
/*
=================
UI_DriverInfo_Menu
=================
*/
unsafe extern "C" fn UI_DriverInfo_Menu() {
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    memset(
        &mut s_driverinfo as *mut driverinfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<driverinfo_t>() as libc::c_ulong,
    );
    DriverInfo_Cache();
    s_driverinfo.menu.fullscreen = qtrue;
    s_driverinfo.menu.draw = Some(DriverInfo_MenuDraw);
    s_driverinfo.banner.generic.type_0 = 10i32;
    s_driverinfo.banner.generic.x = 320i32;
    s_driverinfo.banner.generic.y = 16i32;
    s_driverinfo.banner.string =
        b"DRIVER INFO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_driverinfo.banner.color = color_white.as_mut_ptr();
    s_driverinfo.banner.style = 0x1i32;
    s_driverinfo.framel.generic.type_0 = 6i32;
    s_driverinfo.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_driverinfo.framel.generic.flags = 0x4000i32 as libc::c_uint;
    s_driverinfo.framel.generic.x = 0i32;
    s_driverinfo.framel.generic.y = 78i32;
    s_driverinfo.framel.width = 256i32;
    s_driverinfo.framel.height = 329i32;
    s_driverinfo.framer.generic.type_0 = 6i32;
    s_driverinfo.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_driverinfo.framer.generic.flags = 0x4000i32 as libc::c_uint;
    s_driverinfo.framer.generic.x = 376i32;
    s_driverinfo.framer.generic.y = 76i32;
    s_driverinfo.framer.width = 256i32;
    s_driverinfo.framer.height = 334i32;
    s_driverinfo.back.generic.type_0 = 6i32;
    s_driverinfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_driverinfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_driverinfo.back.generic.callback = Some(DriverInfo_Event);
    s_driverinfo.back.generic.id = 100i32;
    s_driverinfo.back.generic.x = 0i32;
    s_driverinfo.back.generic.y = 480i32 - 64i32;
    s_driverinfo.back.width = 128i32;
    s_driverinfo.back.height = 64i32;
    s_driverinfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Q_strncpyz(
        s_driverinfo.stringbuff.as_mut_ptr(),
        uis.glconfig.extensions_string.as_mut_ptr(),
        1024i32,
    );
    eptr = s_driverinfo.stringbuff.as_mut_ptr();
    while s_driverinfo.numstrings < 40i32 && 0 != *eptr as libc::c_int {
        while 0 != *eptr as libc::c_int && *eptr as libc::c_int == ' ' as i32 {
            let fresh0 = eptr;
            eptr = eptr.offset(1);
            *fresh0 = '\u{0}' as i32 as libc::c_char
        }
        if 0 != *eptr as libc::c_int && *eptr as libc::c_int != ' ' as i32 {
            let fresh1 = s_driverinfo.numstrings;
            s_driverinfo.numstrings = s_driverinfo.numstrings + 1;
            s_driverinfo.strings[fresh1 as usize] = eptr
        }
        while 0 != *eptr as libc::c_int && *eptr as libc::c_int != ' ' as i32 {
            eptr = eptr.offset(1isize)
        }
    }
    i = 0i32;
    while i < s_driverinfo.numstrings {
        len = strlen(s_driverinfo.strings[i as usize]) as libc::c_int;
        if len > 32i32 {
            *s_driverinfo.strings[i as usize].offset((len - 1i32) as isize) =
                '>' as i32 as libc::c_char;
            *s_driverinfo.strings[i as usize].offset(len as isize) = '\u{0}' as i32 as libc::c_char
        }
        i += 1
    }
    Menu_AddItem(
        &mut s_driverinfo.menu,
        &mut s_driverinfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_driverinfo.menu,
        &mut s_driverinfo.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_driverinfo.menu,
        &mut s_driverinfo.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_driverinfo.menu,
        &mut s_driverinfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
    UI_PushMenu(&mut s_driverinfo.menu);
}
static mut s_driverinfo: driverinfo_t = driverinfo_t {
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
    stringbuff: [0; 1024],
    strings: [0 as *const libc::c_char as *mut libc::c_char; 64],
    numstrings: 0,
};
/*
=================
DriverInfo_Event
=================
*/
unsafe extern "C" fn DriverInfo_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        100 => {
            UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
DriverInfo_MenuDraw
=================
*/
unsafe extern "C" fn DriverInfo_MenuDraw() {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    Menu_Draw(&mut s_driverinfo.menu);
    UI_DrawString(
        320i32,
        80i32,
        b"VENDOR\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_red.as_mut_ptr(),
    );
    UI_DrawString(
        320i32,
        152i32,
        b"PIXELFORMAT\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_red.as_mut_ptr(),
    );
    UI_DrawString(
        320i32,
        192i32,
        b"EXTENSIONS\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_red.as_mut_ptr(),
    );
    UI_DrawString(
        320i32,
        80i32 + 16i32,
        uis.glconfig.vendor_string.as_mut_ptr(),
        0x1i32 | 0x10i32,
        text_color_normal.as_mut_ptr(),
    );
    UI_DrawString(
        320i32,
        96i32 + 16i32,
        uis.glconfig.version_string.as_mut_ptr(),
        0x1i32 | 0x10i32,
        text_color_normal.as_mut_ptr(),
    );
    UI_DrawString(
        320i32,
        112i32 + 16i32,
        uis.glconfig.renderer_string.as_mut_ptr(),
        0x1i32 | 0x10i32,
        text_color_normal.as_mut_ptr(),
    );
    UI_DrawString(
        320i32,
        152i32 + 16i32,
        va(
            b"color(%d-bits) Z(%d-bits) stencil(%d-bits)\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            uis.glconfig.colorBits,
            uis.glconfig.depthBits,
            uis.glconfig.stencilBits,
        ),
        0x1i32 | 0x10i32,
        text_color_normal.as_mut_ptr(),
    );
    y = 192i32 + 16i32;
    i = 0i32;
    while i < s_driverinfo.numstrings / 2i32 {
        UI_DrawString(
            320i32 - 4i32,
            y,
            s_driverinfo.strings[(i * 2i32) as usize],
            0x2i32 | 0x10i32,
            text_color_normal.as_mut_ptr(),
        );
        UI_DrawString(
            320i32 + 4i32,
            y,
            s_driverinfo.strings[(i * 2i32 + 1i32) as usize],
            0i32 | 0x10i32,
            text_color_normal.as_mut_ptr(),
        );
        y += 16i32;
        i += 1
    }
    if 0 != s_driverinfo.numstrings & 1i32 {
        UI_DrawString(
            320i32,
            y,
            s_driverinfo.strings[(s_driverinfo.numstrings - 1i32) as usize],
            0x1i32 | 0x10i32,
            text_color_normal.as_mut_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn DriverInfo_Cache() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while !driverinfo_artlist[i as usize].is_null() {
        trap_R_RegisterShaderNoMip(driverinfo_artlist[i as usize]);
        i += 1
    }
}
/*
=======================================================================

DRIVER INFORMATION MENU

=======================================================================
*/
static mut driverinfo_artlist: [*mut libc::c_char; 5] = [
    b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut s_ivo_templates: [InitialVideoOptions_s; 6] = [
    InitialVideoOptions_s {
        mode: 6i32,
        fullscreen: qtrue,
        tq: 3i32,
        lighting: 0i32,
        colordepth: 2i32,
        texturebits: 2i32,
        geometry: 2i32,
        filter: 1i32,
        driver: 0i32,
        extensions: qtrue,
    },
    InitialVideoOptions_s {
        mode: 4i32,
        fullscreen: qtrue,
        tq: 2i32,
        lighting: 0i32,
        colordepth: 2i32,
        texturebits: 2i32,
        geometry: 1i32,
        filter: 1i32,
        driver: 0i32,
        extensions: qtrue,
    },
    InitialVideoOptions_s {
        mode: 3i32,
        fullscreen: qtrue,
        tq: 2i32,
        lighting: 0i32,
        colordepth: 0i32,
        texturebits: 0i32,
        geometry: 1i32,
        filter: 0i32,
        driver: 0i32,
        extensions: qtrue,
    },
    InitialVideoOptions_s {
        mode: 2i32,
        fullscreen: qtrue,
        tq: 1i32,
        lighting: 0i32,
        colordepth: 1i32,
        texturebits: 0i32,
        geometry: 0i32,
        filter: 0i32,
        driver: 0i32,
        extensions: qtrue,
    },
    InitialVideoOptions_s {
        mode: 2i32,
        fullscreen: qtrue,
        tq: 1i32,
        lighting: 1i32,
        colordepth: 1i32,
        texturebits: 0i32,
        geometry: 0i32,
        filter: 0i32,
        driver: 0i32,
        extensions: qtrue,
    },
    InitialVideoOptions_s {
        mode: 3i32,
        fullscreen: qtrue,
        tq: 1i32,
        lighting: 0i32,
        colordepth: 0i32,
        texturebits: 0i32,
        geometry: 1i32,
        filter: 0i32,
        driver: 0i32,
        extensions: qtrue,
    },
];
static mut ratioToRes: [libc::c_int; 32] = [0; 32];
/*
================
GraphicsOptions_TQEvent
================
*/
unsafe extern "C" fn GraphicsOptions_TQEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    s_graphicsoptions.tq.curvalue =
        (s_graphicsoptions.tq.curvalue as libc::c_double + 0.5f64) as libc::c_int as libc::c_float;
}
static mut resolutions: *mut *const libc::c_char = unsafe { builtinResolutions.as_ptr() as *mut _ };
static mut ratios: [*const libc::c_char; 32] = [0 as *const libc::c_char; 32];
/*
================
GraphicsOptions_MenuDraw
================
*/
#[no_mangle]
pub unsafe extern "C" fn GraphicsOptions_MenuDraw() {
    GraphicsOptions_UpdateMenuItems();
    Menu_Draw(&mut s_graphicsoptions.menu);
}
/*
=================
GraphicsOptions_UpdateMenuItems
=================
*/
unsafe extern "C" fn GraphicsOptions_UpdateMenuItems() {
    if s_graphicsoptions.driver.curvalue == 1i32 {
        s_graphicsoptions.fs.curvalue = 1i32;
        s_graphicsoptions.fs.generic.flags |= 0x2000i32 as libc::c_uint;
        s_graphicsoptions.colordepth.curvalue = 1i32
    } else {
        s_graphicsoptions.fs.generic.flags &= !(0x2000i32 as libc::c_uint)
    }
    if s_graphicsoptions.fs.curvalue == 0i32 || s_graphicsoptions.driver.curvalue == 1i32 {
        s_graphicsoptions.colordepth.curvalue = 0i32;
        s_graphicsoptions.colordepth.generic.flags |= 0x2000i32 as libc::c_uint
    } else {
        s_graphicsoptions.colordepth.generic.flags &= !(0x2000i32 as libc::c_uint)
    }
    if s_graphicsoptions.allow_extensions.curvalue == 0i32 {
        if s_graphicsoptions.texturebits.curvalue == 0i32 {
            s_graphicsoptions.texturebits.curvalue = 1i32
        }
    }
    s_graphicsoptions.apply.generic.flags |= 0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    if s_ivo.mode != s_graphicsoptions.mode.curvalue {
        s_graphicsoptions.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if s_ivo.fullscreen as libc::c_uint != s_graphicsoptions.fs.curvalue as libc::c_uint {
        s_graphicsoptions.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if s_ivo.extensions as libc::c_uint
        != s_graphicsoptions.allow_extensions.curvalue as libc::c_uint
    {
        s_graphicsoptions.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if s_ivo.tq as libc::c_float != s_graphicsoptions.tq.curvalue {
        s_graphicsoptions.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if s_ivo.lighting != s_graphicsoptions.lighting.curvalue {
        s_graphicsoptions.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if s_ivo.colordepth != s_graphicsoptions.colordepth.curvalue {
        s_graphicsoptions.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if s_ivo.driver != s_graphicsoptions.driver.curvalue {
        s_graphicsoptions.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if s_ivo.texturebits != s_graphicsoptions.texturebits.curvalue {
        s_graphicsoptions.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if s_ivo.geometry != s_graphicsoptions.geometry.curvalue {
        s_graphicsoptions.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if s_ivo.filter != s_graphicsoptions.filter.curvalue {
        s_graphicsoptions.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    GraphicsOptions_CheckConfig();
}
/*
=================
GraphicsOptions_CheckConfig
=================
*/
unsafe extern "C" fn GraphicsOptions_CheckConfig() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[InitialVideoOptions_s; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<InitialVideoOptions_s>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong)
    {
        if !(s_ivo_templates[i as usize].colordepth != s_graphicsoptions.colordepth.curvalue) {
            if !(s_ivo_templates[i as usize].driver != s_graphicsoptions.driver.curvalue) {
                if !(GraphicsOptions_FindDetectedResolution(s_ivo_templates[i as usize].mode)
                    != s_graphicsoptions.mode.curvalue)
                {
                    if !(s_ivo_templates[i as usize].fullscreen as libc::c_uint
                        != s_graphicsoptions.fs.curvalue as libc::c_uint)
                    {
                        if !(s_ivo_templates[i as usize].tq as libc::c_float
                            != s_graphicsoptions.tq.curvalue)
                        {
                            if !(s_ivo_templates[i as usize].lighting
                                != s_graphicsoptions.lighting.curvalue)
                            {
                                if !(s_ivo_templates[i as usize].geometry
                                    != s_graphicsoptions.geometry.curvalue)
                                {
                                    if !(s_ivo_templates[i as usize].filter
                                        != s_graphicsoptions.filter.curvalue)
                                    {
                                        s_graphicsoptions.list.curvalue = i;
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    s_graphicsoptions.list.curvalue = (::std::mem::size_of::<[InitialVideoOptions_s; 6]>()
        as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<InitialVideoOptions_s>() as libc::c_ulong)
        .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GraphicsOptions_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char);
}
/*
=================
GraphicsOptions_GetAspectRatios
=================
*/
unsafe extern "C" fn GraphicsOptions_GetAspectRatios() {
    let mut i: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    r = 0i32;
    while !(*resolutions.offset(r as isize)).is_null() {
        let mut w: libc::c_int = 0;
        let mut h: libc::c_int = 0;
        let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut str: [libc::c_char; 8] = [0; 8];
        x = strchr(*resolutions.offset(r as isize), 'x' as i32).offset(1isize);
        Q_strncpyz(
            str.as_mut_ptr(),
            *resolutions.offset(r as isize),
            x.wrapping_offset_from(*resolutions.offset(r as isize)) as libc::c_long as libc::c_int,
        );
        w = atoi(str.as_mut_ptr());
        h = atoi(x);
        Com_sprintf(
            str.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
            b"%.2f:1\x00" as *const u8 as *const libc::c_char,
            (w as libc::c_float / h as libc::c_float) as libc::c_double,
        );
        i = 0i32;
        while !knownRatios[i as usize][0usize].is_null() {
            if 0 == Q_stricmp(str.as_mut_ptr(), knownRatios[i as usize][0usize]) {
                Q_strncpyz(
                    str.as_mut_ptr(),
                    knownRatios[i as usize][1usize],
                    ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
                );
                break;
            } else {
                i += 1
            }
        }
        i = 0i32;
        while 0 != ratioBuf[i as usize][0usize] {
            if 0 == Q_stricmp(str.as_mut_ptr(), ratioBuf[i as usize].as_mut_ptr()) {
                break;
            }
            i += 1
        }
        if 0 == ratioBuf[i as usize][0usize] {
            Q_strncpyz(
                ratioBuf[i as usize].as_mut_ptr(),
                str.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong as libc::c_int,
            );
            ratioToRes[i as usize] = r
        }
        ratios[r as usize] = ratioBuf[r as usize].as_mut_ptr();
        resToRatio[r as usize] = i;
        r += 1
    }
    ratios[r as usize] = 0 as *const libc::c_char;
}
static mut ratioBuf: [[libc::c_char; 8]; 32] = [[0; 8]; 32];
static mut knownRatios: [[*const libc::c_char; 2]; 8] = [
    [
        b"1.25:1\x00" as *const u8 as *const libc::c_char,
        b"5:4\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.33:1\x00" as *const u8 as *const libc::c_char,
        b"4:3\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.50:1\x00" as *const u8 as *const libc::c_char,
        b"3:2\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.56:1\x00" as *const u8 as *const libc::c_char,
        b"14:9\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.60:1\x00" as *const u8 as *const libc::c_char,
        b"16:10\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.67:1\x00" as *const u8 as *const libc::c_char,
        b"5:3\x00" as *const u8 as *const libc::c_char,
    ],
    [
        b"1.78:1\x00" as *const u8 as *const libc::c_char,
        b"16:9\x00" as *const u8 as *const libc::c_char,
    ],
    [0 as *const libc::c_char, 0 as *const libc::c_char],
];
/*
=================
GraphicsOptions_GetResolutions
=================
*/
unsafe extern "C" fn GraphicsOptions_GetResolutions() {
    Q_strncpyz(
        resbuf.as_mut_ptr(),
        UI_Cvar_VariableString(b"r_availableModes\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != *resbuf.as_mut_ptr() {
        let mut s: *mut libc::c_char = resbuf.as_mut_ptr();
        let mut i: libc::c_uint = 0i32 as libc::c_uint;
        while !s.is_null()
            && (i as libc::c_ulong)
                < (::std::mem::size_of::<[*const libc::c_char; 32]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong)
        {
            let fresh2 = i;
            i = i.wrapping_add(1);
            detectedResolutions[fresh2 as usize] = s;
            s = strchr(s, ' ' as i32);
            if !s.is_null() {
                let fresh3 = s;
                s = s.offset(1);
                *fresh3 = '\u{0}' as i32 as libc::c_char
            }
        }
        detectedResolutions[i as usize] = 0 as *const libc::c_char;
        if i > 0i32 as libc::c_uint {
            resolutions = detectedResolutions.as_mut_ptr();
            resolutionsDetected = qtrue
        }
    };
}
static mut resbuf: [libc::c_char; 1024] = [0; 1024];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct driverinfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub back: menubitmap_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub stringbuff: [libc::c_char; 1024],
    pub strings: [*mut libc::c_char; 64],
    pub numstrings: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InitialVideoOptions_s {
    pub mode: libc::c_int,
    pub fullscreen: qboolean,
    pub tq: libc::c_int,
    pub lighting: libc::c_int,
    pub colordepth: libc::c_int,
    pub texturebits: libc::c_int,
    pub geometry: libc::c_int,
    pub filter: libc::c_int,
    pub driver: libc::c_int,
    pub extensions: qboolean,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphicsoptions_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub graphics: menutext_s,
    pub display: menutext_s,
    pub sound: menutext_s,
    pub network: menutext_s,
    pub list: menulist_s,
    pub ratio: menulist_s,
    pub mode: menulist_s,
    pub driver: menulist_s,
    pub tq: menuslider_s,
    pub fs: menulist_s,
    pub lighting: menulist_s,
    pub allow_extensions: menulist_s,
    pub texturebits: menulist_s,
    pub colordepth: menulist_s,
    pub geometry: menulist_s,
    pub filter: menulist_s,
    pub driverinfo: menutext_s,
    pub apply: menubitmap_s,
    pub back: menubitmap_s,
}
