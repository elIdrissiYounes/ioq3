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
    connstate_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, vec4_t, vec_t, CA_ACTIVE,
    CA_AUTHORIZING, CA_CHALLENGING, CA_CINEMATIC, CA_CONNECTED, CA_CONNECTING, CA_DISCONNECTED,
    CA_LOADING, CA_PRIMED, CA_UNINITIALIZED,
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
    trap_GetClientState, trap_R_RegisterShaderNoMip,
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
use ui_public_h::uiClientState_t;
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

static mut s_options: optionsmenu_t = optionsmenu_t {
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
Options_Event
=================
*/
unsafe extern "C" fn Options_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        10 => {
            UI_GraphicsOptionsMenu();
        }
        11 => {
            UI_DisplayOptionsMenu();
        }
        12 => {
            UI_SoundOptionsMenu();
        }
        13 => {
            UI_NetworkOptionsMenu();
        }
        14 => {
            UI_PopMenu();
        }
        _ => {}
    };
}
/*
===============
SystemConfig_Cache
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SystemConfig_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
}
/*
===============
Options_MenuInit
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Options_MenuInit() {
    let mut y: libc::c_int = 0;
    let mut cstate: uiClientState_t = uiClientState_t {
        connState: CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    memset(
        &mut s_options as *mut optionsmenu_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<optionsmenu_t>() as libc::c_ulong,
    );
    SystemConfig_Cache();
    s_options.menu.wrapAround = qtrue;
    trap_GetClientState(&mut cstate);
    if cstate.connState as libc::c_uint >= CA_CONNECTED as libc::c_int as libc::c_uint {
        s_options.menu.fullscreen = qfalse
    } else {
        s_options.menu.fullscreen = qtrue
    }
    s_options.banner.generic.type_0 = 10i32;
    s_options.banner.generic.flags = 0x8i32 as libc::c_uint;
    s_options.banner.generic.x = 320i32;
    s_options.banner.generic.y = 16i32;
    s_options.banner.string =
        b"SYSTEM SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_options.banner.color = color_white.as_mut_ptr();
    s_options.banner.style = 0x1i32;
    s_options.framel.generic.type_0 = 6i32;
    s_options.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_options.framel.generic.flags = 0x4000i32 as libc::c_uint;
    s_options.framel.generic.x = 8i32;
    s_options.framel.generic.y = 76i32;
    s_options.framel.width = 256i32;
    s_options.framel.height = 334i32;
    s_options.framer.generic.type_0 = 6i32;
    s_options.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_options.framer.generic.flags = 0x4000i32 as libc::c_uint;
    s_options.framer.generic.x = 376i32;
    s_options.framer.generic.y = 76i32;
    s_options.framer.width = 256i32;
    s_options.framer.height = 334i32;
    y = 168i32;
    s_options.graphics.generic.type_0 = 9i32;
    s_options.graphics.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_options.graphics.generic.callback = Some(Options_Event);
    s_options.graphics.generic.id = 10i32;
    s_options.graphics.generic.x = 320i32;
    s_options.graphics.generic.y = y;
    s_options.graphics.string =
        b"GRAPHICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_options.graphics.color = color_red.as_mut_ptr();
    s_options.graphics.style = 0x1i32;
    y += 34i32;
    s_options.display.generic.type_0 = 9i32;
    s_options.display.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_options.display.generic.callback = Some(Options_Event);
    s_options.display.generic.id = 11i32;
    s_options.display.generic.x = 320i32;
    s_options.display.generic.y = y;
    s_options.display.string =
        b"DISPLAY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_options.display.color = color_red.as_mut_ptr();
    s_options.display.style = 0x1i32;
    y += 34i32;
    s_options.sound.generic.type_0 = 9i32;
    s_options.sound.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_options.sound.generic.callback = Some(Options_Event);
    s_options.sound.generic.id = 12i32;
    s_options.sound.generic.x = 320i32;
    s_options.sound.generic.y = y;
    s_options.sound.string = b"SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_options.sound.color = color_red.as_mut_ptr();
    s_options.sound.style = 0x1i32;
    y += 34i32;
    s_options.network.generic.type_0 = 9i32;
    s_options.network.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_options.network.generic.callback = Some(Options_Event);
    s_options.network.generic.id = 13i32;
    s_options.network.generic.x = 320i32;
    s_options.network.generic.y = y;
    s_options.network.string =
        b"NETWORK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_options.network.color = color_red.as_mut_ptr();
    s_options.network.style = 0x1i32;
    s_options.back.generic.type_0 = 6i32;
    s_options.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_options.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_options.back.generic.callback = Some(Options_Event);
    s_options.back.generic.id = 14i32;
    s_options.back.generic.x = 0i32;
    s_options.back.generic.y = 480i32 - 64i32;
    s_options.back.width = 128i32;
    s_options.back.height = 64i32;
    s_options.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut s_options.menu,
        &mut s_options.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu,
        &mut s_options.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu,
        &mut s_options.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu,
        &mut s_options.graphics as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu,
        &mut s_options.display as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu,
        &mut s_options.sound as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu,
        &mut s_options.network as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_options.menu,
        &mut s_options.back as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
===============
UI_SystemConfigMenu
===============
*/
#[no_mangle]
pub unsafe extern "C" fn UI_SystemConfigMenu() {
    Options_MenuInit();
    UI_PushMenu(&mut s_options.menu);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct optionsmenu_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub graphics: menutext_s,
    pub display: menutext_s,
    pub sound: menutext_s,
    pub network: menutext_s,
    pub back: menubitmap_s,
}
