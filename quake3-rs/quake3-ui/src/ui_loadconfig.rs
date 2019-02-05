use keycodes_h::{
    unnamed_0, K_ALT, K_AUX1, K_AUX10, K_AUX11, K_AUX12, K_AUX13, K_AUX14, K_AUX15, K_AUX16,
    K_AUX2, K_AUX3, K_AUX4, K_AUX5, K_AUX6, K_AUX7, K_AUX8, K_AUX9, K_BACKSPACE, K_BREAK,
    K_CAPSLOCK, K_COMMAND, K_COMPOSE, K_CONSOLE, K_CTRL, K_DEL, K_DOWNARROW, K_END, K_ENTER,
    K_ESCAPE, K_EURO, K_F1, K_F10, K_F11, K_F12, K_F13, K_F14, K_F15, K_F2, K_F3, K_F4, K_F5, K_F6,
    K_F7, K_F8, K_F9, K_HELP, K_HOME, K_INS, K_JOY1, K_JOY10, K_JOY11, K_JOY12, K_JOY13, K_JOY14,
    K_JOY15, K_JOY16, K_JOY17, K_JOY18, K_JOY19, K_JOY2, K_JOY20, K_JOY21, K_JOY22, K_JOY23,
    K_JOY24, K_JOY25, K_JOY26, K_JOY27, K_JOY28, K_JOY29, K_JOY3, K_JOY30, K_JOY31, K_JOY32,
    K_JOY4, K_JOY5, K_JOY6, K_JOY7, K_JOY8, K_JOY9, K_KP_5, K_KP_DEL, K_KP_DOWNARROW, K_KP_END,
    K_KP_ENTER, K_KP_EQUALS, K_KP_HOME, K_KP_INS, K_KP_LEFTARROW, K_KP_MINUS, K_KP_NUMLOCK,
    K_KP_PGDN, K_KP_PGUP, K_KP_PLUS, K_KP_RIGHTARROW, K_KP_SLASH, K_KP_STAR, K_KP_UPARROW,
    K_LEFTARROW, K_MENU, K_MODE, K_MOUSE1, K_MOUSE2, K_MOUSE3, K_MOUSE4, K_MOUSE5, K_MWHEELDOWN,
    K_MWHEELUP, K_PAD0_A, K_PAD0_B, K_PAD0_BACK, K_PAD0_DPAD_DOWN, K_PAD0_DPAD_LEFT,
    K_PAD0_DPAD_RIGHT, K_PAD0_DPAD_UP, K_PAD0_GUIDE, K_PAD0_LEFTSHOULDER, K_PAD0_LEFTSTICK_CLICK,
    K_PAD0_LEFTSTICK_DOWN, K_PAD0_LEFTSTICK_LEFT, K_PAD0_LEFTSTICK_RIGHT, K_PAD0_LEFTSTICK_UP,
    K_PAD0_LEFTTRIGGER, K_PAD0_RIGHTSHOULDER, K_PAD0_RIGHTSTICK_CLICK, K_PAD0_RIGHTSTICK_DOWN,
    K_PAD0_RIGHTSTICK_LEFT, K_PAD0_RIGHTSTICK_RIGHT, K_PAD0_RIGHTSTICK_UP, K_PAD0_RIGHTTRIGGER,
    K_PAD0_START, K_PAD0_X, K_PAD0_Y, K_PAUSE, K_PGDN, K_PGUP, K_POWER, K_PRINT, K_RIGHTARROW,
    K_SCROLLOCK, K_SHIFT, K_SPACE, K_SUPER, K_SYSREQ, K_TAB, K_UNDO, K_UPARROW, K_WORLD_0,
    K_WORLD_1, K_WORLD_10, K_WORLD_11, K_WORLD_12, K_WORLD_13, K_WORLD_14, K_WORLD_15, K_WORLD_16,
    K_WORLD_17, K_WORLD_18, K_WORLD_19, K_WORLD_2, K_WORLD_20, K_WORLD_21, K_WORLD_22, K_WORLD_23,
    K_WORLD_24, K_WORLD_25, K_WORLD_26, K_WORLD_27, K_WORLD_28, K_WORLD_29, K_WORLD_3, K_WORLD_30,
    K_WORLD_31, K_WORLD_32, K_WORLD_33, K_WORLD_34, K_WORLD_35, K_WORLD_36, K_WORLD_37, K_WORLD_38,
    K_WORLD_39, K_WORLD_4, K_WORLD_40, K_WORLD_41, K_WORLD_42, K_WORLD_43, K_WORLD_44, K_WORLD_45,
    K_WORLD_46, K_WORLD_47, K_WORLD_48, K_WORLD_49, K_WORLD_5, K_WORLD_50, K_WORLD_51, K_WORLD_52,
    K_WORLD_53, K_WORLD_54, K_WORLD_55, K_WORLD_56, K_WORLD_57, K_WORLD_58, K_WORLD_59, K_WORLD_6,
    K_WORLD_60, K_WORLD_61, K_WORLD_62, K_WORLD_63, K_WORLD_64, K_WORLD_65, K_WORLD_66, K_WORLD_67,
    K_WORLD_68, K_WORLD_69, K_WORLD_7, K_WORLD_70, K_WORLD_71, K_WORLD_72, K_WORLD_73, K_WORLD_74,
    K_WORLD_75, K_WORLD_76, K_WORLD_77, K_WORLD_78, K_WORLD_79, K_WORLD_8, K_WORLD_80, K_WORLD_81,
    K_WORLD_82, K_WORLD_83, K_WORLD_84, K_WORLD_85, K_WORLD_86, K_WORLD_87, K_WORLD_88, K_WORLD_89,
    K_WORLD_9, K_WORLD_90, K_WORLD_91, K_WORLD_92, K_WORLD_93, K_WORLD_94, K_WORLD_95, MAX_KEYS,
};
use libc;
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t, Q_stricmp,
    Q_strupr, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menulist_s, menutext_s,
    trap_Cmd_ExecuteText, trap_FS_GetFileList, trap_R_RegisterShaderNoMip,
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
use ui_video::{DriverInfo_Cache, GraphicsOptions_Cache, UI_GraphicsOptionsMenu};

//
// ui_loadconfig.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_LoadConfig_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/load_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/load_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/arrows_horz_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_horz_left\x00" as *const u8 as *const libc::c_char,
    );
    trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_horz_right\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn UI_LoadConfigMenu() {
    LoadConfig_MenuInit();
    UI_PushMenu(&mut s_configs.menu);
}
static mut s_configs: configs_t = configs_t {
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
    arrows: menubitmap_s {
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
    left: menubitmap_s {
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
    right: menubitmap_s {
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
    names: [0; 2048],
    configlist: [0 as *const libc::c_char as *mut libc::c_char; 128],
};
/*
===============
LoadConfig_MenuInit
===============
*/
unsafe extern "C" fn LoadConfig_MenuInit() {
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut configname: *mut libc::c_char = 0 as *mut libc::c_char;
    UI_LoadConfig_Cache();
    memset(
        &mut s_configs as *mut configs_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<configs_t>() as libc::c_ulong,
    );
    s_configs.menu.wrapAround = qtrue;
    s_configs.menu.fullscreen = qtrue;
    s_configs.banner.generic.type_0 = 10i32;
    s_configs.banner.generic.x = 320i32;
    s_configs.banner.generic.y = 16i32;
    s_configs.banner.string =
        b"LOAD CONFIG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_configs.banner.color = color_white.as_mut_ptr();
    s_configs.banner.style = 0x1i32;
    s_configs.framel.generic.type_0 = 6i32;
    s_configs.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_configs.framel.generic.flags = 0x4000i32 as libc::c_uint;
    s_configs.framel.generic.x = 0i32;
    s_configs.framel.generic.y = 78i32;
    s_configs.framel.width = 256i32;
    s_configs.framel.height = 329i32;
    s_configs.framer.generic.type_0 = 6i32;
    s_configs.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_configs.framer.generic.flags = 0x4000i32 as libc::c_uint;
    s_configs.framer.generic.x = 376i32;
    s_configs.framer.generic.y = 76i32;
    s_configs.framer.width = 256i32;
    s_configs.framer.height = 334i32;
    s_configs.arrows.generic.type_0 = 6i32;
    s_configs.arrows.generic.name =
        b"menu/art/arrows_horz_0\x00" as *const u8 as *const libc::c_char;
    s_configs.arrows.generic.flags = 0x4000i32 as libc::c_uint;
    s_configs.arrows.generic.x = 320i32 - 128i32 / 2i32;
    s_configs.arrows.generic.y = 400i32;
    s_configs.arrows.width = 128i32;
    s_configs.arrows.height = 48i32;
    s_configs.left.generic.type_0 = 6i32;
    s_configs.left.generic.flags =
        0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x800i32 as libc::c_uint;
    s_configs.left.generic.x = 320i32 - 128i32 / 2i32;
    s_configs.left.generic.y = 400i32;
    s_configs.left.generic.id = 13i32;
    s_configs.left.generic.callback = Some(LoadConfig_MenuEvent);
    s_configs.left.width = 128i32 / 2i32;
    s_configs.left.height = 48i32;
    s_configs.left.focuspic =
        b"menu/art/arrows_horz_left\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_configs.right.generic.type_0 = 6i32;
    s_configs.right.generic.flags =
        0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x800i32 as libc::c_uint;
    s_configs.right.generic.x = 320i32;
    s_configs.right.generic.y = 400i32;
    s_configs.right.generic.id = 14i32;
    s_configs.right.generic.callback = Some(LoadConfig_MenuEvent);
    s_configs.right.width = 128i32 / 2i32;
    s_configs.right.height = 48i32;
    s_configs.right.focuspic =
        b"menu/art/arrows_horz_right\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_configs.back.generic.type_0 = 6i32;
    s_configs.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_configs.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_configs.back.generic.id = 10i32;
    s_configs.back.generic.callback = Some(LoadConfig_MenuEvent);
    s_configs.back.generic.x = 0i32;
    s_configs.back.generic.y = 480i32 - 64i32;
    s_configs.back.width = 128i32;
    s_configs.back.height = 64i32;
    s_configs.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_configs.go.generic.type_0 = 6i32;
    s_configs.go.generic.name = b"menu/art/load_0\x00" as *const u8 as *const libc::c_char;
    s_configs.go.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_configs.go.generic.id = 11i32;
    s_configs.go.generic.callback = Some(LoadConfig_MenuEvent);
    s_configs.go.generic.x = 640i32;
    s_configs.go.generic.y = 480i32 - 64i32;
    s_configs.go.width = 128i32;
    s_configs.go.height = 64i32;
    s_configs.go.focuspic =
        b"menu/art/load_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_configs.list.generic.type_0 = 8i32;
    s_configs.list.generic.flags = 0x100i32 as libc::c_uint;
    s_configs.list.generic.callback = Some(LoadConfig_MenuEvent);
    s_configs.list.generic.id = 12i32;
    s_configs.list.generic.x = 118i32;
    s_configs.list.generic.y = 130i32;
    s_configs.list.width = 16i32;
    s_configs.list.height = 14i32;
    s_configs.list.numitems = trap_FS_GetFileList(
        b"\x00" as *const u8 as *const libc::c_char,
        b"cfg\x00" as *const u8 as *const libc::c_char,
        s_configs.names.as_mut_ptr(),
        128i32 * 16i32,
    );
    s_configs.list.itemnames = s_configs.configlist.as_mut_ptr() as *mut *const libc::c_char;
    s_configs.list.columns = 3i32;
    if 0 == s_configs.list.numitems {
        strcpy(
            s_configs.names.as_mut_ptr(),
            b"No Files Found.\x00" as *const u8 as *const libc::c_char,
        );
        s_configs.list.numitems = 1i32;
        s_configs.go.generic.flags |= 0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint
    } else if s_configs.list.numitems > 128i32 {
        s_configs.list.numitems = 128i32
    }
    configname = s_configs.names.as_mut_ptr();
    i = 0i32;
    while i < s_configs.list.numitems {
        let ref mut fresh0 = *s_configs.list.itemnames.offset(i as isize);
        *fresh0 = configname;
        len = strlen(configname) as libc::c_int;
        if 0 == Q_stricmp(
            configname.offset(len as isize).offset(-4isize),
            b".cfg\x00" as *const u8 as *const libc::c_char,
        ) {
            *configname.offset((len - 4i32) as isize) = '\u{0}' as i32 as libc::c_char
        }
        Q_strupr(configname);
        configname = configname.offset((len + 1i32) as isize);
        i += 1
    }
    Menu_AddItem(
        &mut s_configs.menu,
        &mut s_configs.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu,
        &mut s_configs.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu,
        &mut s_configs.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu,
        &mut s_configs.list as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu,
        &mut s_configs.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu,
        &mut s_configs.left as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu,
        &mut s_configs.right as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu,
        &mut s_configs.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_configs.menu,
        &mut s_configs.go as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
===============
LoadConfig_MenuEvent
===============
*/
unsafe extern "C" fn LoadConfig_MenuEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        11 => {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                va(
                    b"exec %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *s_configs
                        .list
                        .itemnames
                        .offset(s_configs.list.curvalue as isize),
                ),
            );
            UI_PopMenu();
        }
        10 => {
            UI_PopMenu();
        }
        13 => {
            ScrollList_Key(&mut s_configs.list, K_LEFTARROW as libc::c_int);
        }
        14 => {
            ScrollList_Key(&mut s_configs.list, K_RIGHTARROW as libc::c_int);
        }
        _ => {}
    };
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct configs_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub list: menulist_s,
    pub arrows: menubitmap_s,
    pub left: menubitmap_s,
    pub right: menubitmap_s,
    pub back: menubitmap_s,
    pub go: menubitmap_s,
    pub names: [libc::c_char; 2048],
    pub configlist: [*mut libc::c_char; 128],
}
