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
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t, Com_sprintf,
    EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::{memset, strlen};
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
    trap_Cmd_ExecuteText, trap_Cvar_VariableValue, trap_FS_GetFileList, trap_R_RegisterShaderNoMip,
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
// ui_demo2.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_DemosMenu() {
    Demos_MenuInit();
    UI_PushMenu(&mut s_demos.menu);
}
static mut s_demos: demos_t = demos_t {
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
    numDemos: 0,
    names: [0; 32768],
    demolist: [0 as *const libc::c_char as *mut libc::c_char; 1024],
};
/*
===============
Demos_MenuInit
===============
*/
unsafe extern "C" fn Demos_MenuInit() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut demoname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extension: [libc::c_char; 32] = [0; 32];
    let mut protocol: libc::c_int = 0;
    let mut protocolLegacy: libc::c_int = 0;
    memset(
        &mut s_demos as *mut demos_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<demos_t>() as libc::c_ulong,
    );
    Demos_Cache();
    s_demos.menu.fullscreen = qtrue;
    s_demos.menu.wrapAround = qtrue;
    s_demos.banner.generic.type_0 = 10i32;
    s_demos.banner.generic.x = 320i32;
    s_demos.banner.generic.y = 16i32;
    s_demos.banner.string = b"DEMOS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_demos.banner.color = color_white.as_mut_ptr();
    s_demos.banner.style = 0x1i32;
    s_demos.framel.generic.type_0 = 6i32;
    s_demos.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_demos.framel.generic.flags = 0x4000i32 as libc::c_uint;
    s_demos.framel.generic.x = 0i32;
    s_demos.framel.generic.y = 78i32;
    s_demos.framel.width = 256i32;
    s_demos.framel.height = 329i32;
    s_demos.framer.generic.type_0 = 6i32;
    s_demos.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_demos.framer.generic.flags = 0x4000i32 as libc::c_uint;
    s_demos.framer.generic.x = 376i32;
    s_demos.framer.generic.y = 76i32;
    s_demos.framer.width = 256i32;
    s_demos.framer.height = 334i32;
    s_demos.arrows.generic.type_0 = 6i32;
    s_demos.arrows.generic.name = b"menu/art/arrows_horz_0\x00" as *const u8 as *const libc::c_char;
    s_demos.arrows.generic.flags = 0x4000i32 as libc::c_uint;
    s_demos.arrows.generic.x = 320i32 - 128i32 / 2i32;
    s_demos.arrows.generic.y = 400i32;
    s_demos.arrows.width = 128i32;
    s_demos.arrows.height = 48i32;
    s_demos.left.generic.type_0 = 6i32;
    s_demos.left.generic.flags =
        0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x800i32 as libc::c_uint;
    s_demos.left.generic.x = 320i32 - 128i32 / 2i32;
    s_demos.left.generic.y = 400i32;
    s_demos.left.generic.id = 14i32;
    s_demos.left.generic.callback = Some(Demos_MenuEvent);
    s_demos.left.width = 128i32 / 2i32;
    s_demos.left.height = 48i32;
    s_demos.left.focuspic =
        b"menu/art/arrows_horz_left\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_demos.right.generic.type_0 = 6i32;
    s_demos.right.generic.flags =
        0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x800i32 as libc::c_uint;
    s_demos.right.generic.x = 320i32;
    s_demos.right.generic.y = 400i32;
    s_demos.right.generic.id = 13i32;
    s_demos.right.generic.callback = Some(Demos_MenuEvent);
    s_demos.right.width = 128i32 / 2i32;
    s_demos.right.height = 48i32;
    s_demos.right.focuspic =
        b"menu/art/arrows_horz_right\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_demos.back.generic.type_0 = 6i32;
    s_demos.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_demos.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_demos.back.generic.id = 10i32;
    s_demos.back.generic.callback = Some(Demos_MenuEvent);
    s_demos.back.generic.x = 0i32;
    s_demos.back.generic.y = 480i32 - 64i32;
    s_demos.back.width = 128i32;
    s_demos.back.height = 64i32;
    s_demos.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_demos.go.generic.type_0 = 6i32;
    s_demos.go.generic.name = b"menu/art/play_0\x00" as *const u8 as *const libc::c_char;
    s_demos.go.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_demos.go.generic.id = 11i32;
    s_demos.go.generic.callback = Some(Demos_MenuEvent);
    s_demos.go.generic.x = 640i32;
    s_demos.go.generic.y = 480i32 - 64i32;
    s_demos.go.width = 128i32;
    s_demos.go.height = 64i32;
    s_demos.go.focuspic =
        b"menu/art/play_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_demos.list.generic.type_0 = 8i32;
    s_demos.list.generic.flags = 0x100i32 as libc::c_uint;
    s_demos.list.generic.callback = Some(Demos_MenuEvent);
    s_demos.list.generic.id = 12i32;
    s_demos.list.generic.x = 118i32;
    s_demos.list.generic.y = 130i32;
    s_demos.list.width = 16i32;
    s_demos.list.height = 14i32;
    s_demos.list.itemnames = s_demos.demolist.as_mut_ptr() as *mut *const libc::c_char;
    s_demos.list.columns = 3i32;
    protocolLegacy =
        trap_Cvar_VariableValue(b"com_legacyprotocol\x00" as *const u8 as *const libc::c_char)
            as libc::c_int;
    protocol = trap_Cvar_VariableValue(b"com_protocol\x00" as *const u8 as *const libc::c_char)
        as libc::c_int;
    if 0 == protocol {
        protocol = trap_Cvar_VariableValue(b"protocol\x00" as *const u8 as *const libc::c_char)
            as libc::c_int
    }
    if protocolLegacy == protocol {
        protocolLegacy = 0i32
    }
    Com_sprintf(
        extension.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        b".%s%d\x00" as *const u8 as *const libc::c_char,
        b"dm_\x00" as *const u8 as *const libc::c_char,
        protocol,
    );
    s_demos.numDemos = trap_FS_GetFileList(
        b"demos\x00" as *const u8 as *const libc::c_char,
        extension.as_mut_ptr(),
        s_demos.names.as_mut_ptr(),
        (::std::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    );
    demoname = s_demos.names.as_mut_ptr();
    i = 0i32;
    j = 0i32;
    while j < 2i32 {
        if s_demos.numDemos > 1024i32 {
            s_demos.numDemos = 1024i32
        }
        while i < s_demos.numDemos {
            let ref mut fresh0 = *s_demos.list.itemnames.offset(i as isize);
            *fresh0 = demoname;
            len = strlen(demoname) as libc::c_int;
            demoname = demoname.offset((len + 1i32) as isize);
            i += 1
        }
        if 0 == j {
            if !(protocolLegacy > 0i32 && s_demos.numDemos < 1024i32) {
                break;
            }
            Com_sprintf(
                extension.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
                b".%s%d\x00" as *const u8 as *const libc::c_char,
                b"dm_\x00" as *const u8 as *const libc::c_char,
                protocolLegacy,
            );
            s_demos.numDemos += trap_FS_GetFileList(
                b"demos\x00" as *const u8 as *const libc::c_char,
                extension.as_mut_ptr(),
                demoname,
                (::std::mem::size_of::<[libc::c_char; 32768]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(demoname.wrapping_offset_from(s_demos.names.as_mut_ptr())
                        as libc::c_long as libc::c_ulong) as libc::c_int,
            )
        }
        j += 1
    }
    s_demos.list.numitems = s_demos.numDemos;
    if 0 == s_demos.numDemos {
        let ref mut fresh1 = *s_demos.list.itemnames.offset(0isize);
        *fresh1 = b"No Demos Found.\x00" as *const u8 as *const libc::c_char;
        s_demos.list.numitems = 1i32;
        s_demos.go.generic.flags |= 0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint
    }
    Menu_AddItem(
        &mut s_demos.menu,
        &mut s_demos.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_demos.menu,
        &mut s_demos.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_demos.menu,
        &mut s_demos.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_demos.menu,
        &mut s_demos.list as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_demos.menu,
        &mut s_demos.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_demos.menu,
        &mut s_demos.left as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_demos.menu,
        &mut s_demos.right as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_demos.menu,
        &mut s_demos.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_demos.menu,
        &mut s_demos.go as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
===============
Demos_MenuEvent
===============
*/
unsafe extern "C" fn Demos_MenuEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        11 => {
            UI_ForceMenuOff();
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                va(
                    b"demo %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    *s_demos
                        .list
                        .itemnames
                        .offset(s_demos.list.curvalue as isize),
                ),
            );
        }
        10 => {
            UI_PopMenu();
        }
        14 => {
            ScrollList_Key(&mut s_demos.list, K_LEFTARROW as libc::c_int);
        }
        13 => {
            ScrollList_Key(&mut s_demos.list, K_RIGHTARROW as libc::c_int);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn Demos_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/play_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/play_1\x00" as *const u8 as *const libc::c_char);
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct demos_t {
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
    pub numDemos: libc::c_int,
    pub names: [libc::c_char; 32768],
    pub demolist: [*mut libc::c_char; 1024],
}
