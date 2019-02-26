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
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, vec4_t, vec_t, EXEC_APPEND,
    EXEC_INSERT, EXEC_NOW,
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menutext_s,
    trap_Cmd_ExecuteText, trap_Cvar_VariableValue, trap_R_RegisterShaderNoMip,
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
// ui_setup.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_SetupMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_SetupMenu() {
    UI_SetupMenu_Init();
    UI_PushMenu(&mut setupMenuInfo.menu);
}
static mut setupMenuInfo: setupMenuInfo_t = setupMenuInfo_t {
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
    setupplayer: menutext_s {
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
    setupcontrols: menutext_s {
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
    setupsystem: menutext_s {
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
    game: menutext_s {
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
    cdkey: menutext_s {
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
    defaults: menutext_s {
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
};
/*
===============
UI_SetupMenu_Init
===============
*/
unsafe extern "C" fn UI_SetupMenu_Init() {
    let mut y: libc::c_int = 0;
    UI_SetupMenu_Cache();
    memset(
        &mut setupMenuInfo as *mut setupMenuInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<setupMenuInfo_t>() as libc::c_ulong,
    );
    setupMenuInfo.menu.wrapAround = qtrue;
    setupMenuInfo.menu.fullscreen = qtrue;
    setupMenuInfo.banner.generic.type_0 = 10i32;
    setupMenuInfo.banner.generic.x = 320i32;
    setupMenuInfo.banner.generic.y = 16i32;
    setupMenuInfo.banner.string =
        b"SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.banner.color = color_white.as_mut_ptr();
    setupMenuInfo.banner.style = 0x1i32;
    setupMenuInfo.framel.generic.type_0 = 6i32;
    setupMenuInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    setupMenuInfo.framel.generic.flags = 0x4000i32 as libc::c_uint;
    setupMenuInfo.framel.generic.x = 0i32;
    setupMenuInfo.framel.generic.y = 78i32;
    setupMenuInfo.framel.width = 256i32;
    setupMenuInfo.framel.height = 329i32;
    setupMenuInfo.framer.generic.type_0 = 6i32;
    setupMenuInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    setupMenuInfo.framer.generic.flags = 0x4000i32 as libc::c_uint;
    setupMenuInfo.framer.generic.x = 376i32;
    setupMenuInfo.framer.generic.y = 76i32;
    setupMenuInfo.framer.width = 256i32;
    setupMenuInfo.framer.height = 334i32;
    y = 134i32;
    setupMenuInfo.setupplayer.generic.type_0 = 9i32;
    setupMenuInfo.setupplayer.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    setupMenuInfo.setupplayer.generic.x = 320i32;
    setupMenuInfo.setupplayer.generic.y = y;
    setupMenuInfo.setupplayer.generic.id = 10i32;
    setupMenuInfo.setupplayer.generic.callback = Some(UI_SetupMenu_Event);
    setupMenuInfo.setupplayer.string =
        b"PLAYER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.setupplayer.color = color_red.as_mut_ptr();
    setupMenuInfo.setupplayer.style = 0x1i32;
    y += 34i32;
    setupMenuInfo.setupcontrols.generic.type_0 = 9i32;
    setupMenuInfo.setupcontrols.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    setupMenuInfo.setupcontrols.generic.x = 320i32;
    setupMenuInfo.setupcontrols.generic.y = y;
    setupMenuInfo.setupcontrols.generic.id = 11i32;
    setupMenuInfo.setupcontrols.generic.callback = Some(UI_SetupMenu_Event);
    setupMenuInfo.setupcontrols.string =
        b"CONTROLS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.setupcontrols.color = color_red.as_mut_ptr();
    setupMenuInfo.setupcontrols.style = 0x1i32;
    y += 34i32;
    setupMenuInfo.setupsystem.generic.type_0 = 9i32;
    setupMenuInfo.setupsystem.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    setupMenuInfo.setupsystem.generic.x = 320i32;
    setupMenuInfo.setupsystem.generic.y = y;
    setupMenuInfo.setupsystem.generic.id = 12i32;
    setupMenuInfo.setupsystem.generic.callback = Some(UI_SetupMenu_Event);
    setupMenuInfo.setupsystem.string =
        b"SYSTEM\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.setupsystem.color = color_red.as_mut_ptr();
    setupMenuInfo.setupsystem.style = 0x1i32;
    y += 34i32;
    setupMenuInfo.game.generic.type_0 = 9i32;
    setupMenuInfo.game.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    setupMenuInfo.game.generic.x = 320i32;
    setupMenuInfo.game.generic.y = y;
    setupMenuInfo.game.generic.id = 13i32;
    setupMenuInfo.game.generic.callback = Some(UI_SetupMenu_Event);
    setupMenuInfo.game.string =
        b"GAME OPTIONS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.game.color = color_red.as_mut_ptr();
    setupMenuInfo.game.style = 0x1i32;
    y += 34i32;
    setupMenuInfo.cdkey.generic.type_0 = 9i32;
    setupMenuInfo.cdkey.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    setupMenuInfo.cdkey.generic.x = 320i32;
    setupMenuInfo.cdkey.generic.y = y;
    setupMenuInfo.cdkey.generic.id = 14i32;
    setupMenuInfo.cdkey.generic.callback = Some(UI_SetupMenu_Event);
    setupMenuInfo.cdkey.string =
        b"CD Key\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    setupMenuInfo.cdkey.color = color_red.as_mut_ptr();
    setupMenuInfo.cdkey.style = 0x1i32;
    if 0. == trap_Cvar_VariableValue(b"cl_paused\x00" as *const u8 as *const libc::c_char) {
        y += 34i32;
        setupMenuInfo.defaults.generic.type_0 = 9i32;
        setupMenuInfo.defaults.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
        setupMenuInfo.defaults.generic.x = 320i32;
        setupMenuInfo.defaults.generic.y = y;
        setupMenuInfo.defaults.generic.id = 17i32;
        setupMenuInfo.defaults.generic.callback = Some(UI_SetupMenu_Event);
        setupMenuInfo.defaults.string =
            b"DEFAULTS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        setupMenuInfo.defaults.color = color_red.as_mut_ptr();
        setupMenuInfo.defaults.style = 0x1i32
    }
    setupMenuInfo.back.generic.type_0 = 6i32;
    setupMenuInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    setupMenuInfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    setupMenuInfo.back.generic.id = 18i32;
    setupMenuInfo.back.generic.callback = Some(UI_SetupMenu_Event);
    setupMenuInfo.back.generic.x = 0i32;
    setupMenuInfo.back.generic.y = 480i32 - 64i32;
    setupMenuInfo.back.width = 128i32;
    setupMenuInfo.back.height = 64i32;
    setupMenuInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut setupMenuInfo.menu,
        &mut setupMenuInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu,
        &mut setupMenuInfo.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu,
        &mut setupMenuInfo.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu,
        &mut setupMenuInfo.setupplayer as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu,
        &mut setupMenuInfo.setupcontrols as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu,
        &mut setupMenuInfo.setupsystem as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu,
        &mut setupMenuInfo.game as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut setupMenuInfo.menu,
        &mut setupMenuInfo.cdkey as *mut menutext_s as *mut libc::c_void,
    );
    if 0. == trap_Cvar_VariableValue(b"cl_paused\x00" as *const u8 as *const libc::c_char) {
        Menu_AddItem(
            &mut setupMenuInfo.menu,
            &mut setupMenuInfo.defaults as *mut menutext_s as *mut libc::c_void,
        );
    }
    Menu_AddItem(
        &mut setupMenuInfo.menu,
        &mut setupMenuInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
===============
UI_SetupMenu_Event
===============
*/
unsafe extern "C" fn UI_SetupMenu_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        10 => {
            UI_PlayerSettingsMenu();
        }
        11 => {
            UI_ControlsMenu();
        }
        12 => {
            UI_GraphicsOptionsMenu();
        }
        13 => {
            UI_PreferencesMenu();
        }
        14 => {
            UI_CDKeyMenu();
        }
        17 => {
            UI_ConfirmMenu(
                b"SET TO DEFAULTS?\x00" as *const u8 as *const libc::c_char,
                Some(Setup_ResetDefaults_Draw),
                Some(Setup_ResetDefaults_Action),
            );
        }
        18 => {
            UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
Setup_ResetDefaults_Action
=================
*/
unsafe extern "C" fn Setup_ResetDefaults_Action(mut result: qboolean) {
    if 0 == result as u64 {
        return;
    }
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        b"exec default.cfg\n\x00" as *const u8 as *const libc::c_char,
    );
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        b"cvar_restart\n\x00" as *const u8 as *const libc::c_char,
    );
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        b"vid_restart\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
Setup_ResetDefaults_Draw
=================
*/
unsafe extern "C" fn Setup_ResetDefaults_Draw() {
    UI_DrawProportionalString(
        640i32 / 2i32,
        356i32 + 27i32 * 0i32,
        b"WARNING: This will reset *ALL*\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_yellow.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640i32 / 2i32,
        356i32 + 27i32 * 1i32,
        b"options to their default values.\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_yellow.as_mut_ptr(),
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct setupMenuInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub setupplayer: menutext_s,
    pub setupcontrols: menutext_s,
    pub setupsystem: menutext_s,
    pub game: menutext_s,
    pub cdkey: menutext_s,
    pub defaults: menutext_s,
    pub back: menubitmap_s,
}
