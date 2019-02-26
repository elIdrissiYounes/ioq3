#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use bg_misc::bg_itemlist;
use bg_public_h::{
    unnamed_1, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT,
};
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
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, colorYellow, g_color_table, vec3_origin,
    vectoangles, AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract,
    AnglesToAxis, AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    cvarHandle_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t,
    vmCvar_t, Com_Clamp, Com_sprintf, Info_SetValueForKey, Info_ValueForKey, Q_CleanStr, Q_stricmp,
    Q_strncpyz, Q_strupr, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stddef_h::size_t;
use stdlib::{__compar_fn_t, atoi, memcpy, memset, qsort, strcat, strcpy, strlen};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, textureCompression_t, GLDRV_ICD,
    GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC, GLHW_PERMEDIA2, GLHW_RAGEPRO,
    GLHW_RIVA128, TC_NONE, TC_S3TC, TC_S3TC_ARB,
};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menulist_s, menuradiobutton_s,
    menutext_s, trap_Cmd_ExecuteText, trap_Cvar_Register, trap_Cvar_Set, trap_Cvar_SetValue,
    trap_Cvar_VariableStringBuffer, trap_Cvar_VariableValue, trap_LAN_ClearPing, trap_LAN_GetPing,
    trap_LAN_GetPingInfo, trap_LAN_GetPingQueueCount, trap_LAN_GetServerAddressString,
    trap_LAN_GetServerCount, trap_R_RegisterShaderNoMip, trap_SetPbClStatus, uiStatic_t,
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
// ui_servers2.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_ArenaServersMenu() {
    ArenaServers_MenuInit();
    UI_PushMenu(&mut g_arenaservers.menu);
}
static mut g_arenaservers: arenaservers_t = arenaservers_t {
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
    master: menulist_s {
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
    gametype: menulist_s {
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
    sortkey: menulist_s {
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
    showfull: menuradiobutton_s {
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
        curvalue: 0,
    },
    showempty: menuradiobutton_s {
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
        curvalue: 0,
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
    mappic: menubitmap_s {
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
    up: menubitmap_s {
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
    down: menubitmap_s {
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
    status: menutext_s {
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
    statusbar: menutext_s {
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
    remove: menubitmap_s {
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
    refresh: menubitmap_s {
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
    specify: menubitmap_s {
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
    create: menubitmap_s {
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
    pinglist: [pinglist_t {
        adrstr: [0; 64],
        start: 0,
    }; 32],
    table: [table_t {
        buff: [0; 68],
        servernode: 0 as *const servernode_t as *mut servernode_t,
    }; 128],
    items: [0 as *const libc::c_char as *mut libc::c_char; 128],
    numqueriedservers: 0,
    numservers: 0 as *const libc::c_int as *mut libc::c_int,
    serverlist: 0 as *const servernode_t as *mut servernode_t,
    currentping: 0,
    refreshservers: qfalse,
    nextpingtime: 0,
    maxservers: 0,
    refreshtime: 0,
    favoriteaddresses: [[0; 64]; 16],
    numfavoriteaddresses: 0,
    punkbuster: menulist_s {
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
    pblogo: menubitmap_s {
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
ArenaServers_MenuInit
=================
*/
unsafe extern "C" fn ArenaServers_MenuInit() {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    static mut statusbuffer: [libc::c_char; 64] = [0; 64];
    memset(
        &mut g_arenaservers as *mut arenaservers_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<arenaservers_t>() as libc::c_ulong,
    );
    ArenaServers_Cache();
    g_arenaservers.menu.fullscreen = qtrue;
    g_arenaservers.menu.wrapAround = qtrue;
    g_arenaservers.menu.draw = Some(ArenaServers_MenuDraw);
    g_arenaservers.menu.key = Some(ArenaServers_MenuKey);
    g_arenaservers.banner.generic.type_0 = 10i32;
    g_arenaservers.banner.generic.flags = 0x8i32 as libc::c_uint;
    g_arenaservers.banner.generic.x = 320i32;
    g_arenaservers.banner.generic.y = 16i32;
    g_arenaservers.banner.string =
        b"ARENA SERVERS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.banner.style = 0x1i32;
    g_arenaservers.banner.color = color_white.as_mut_ptr();
    y = 80i32;
    g_arenaservers.master.generic.type_0 = 3i32;
    g_arenaservers.master.generic.name = b"Servers:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.master.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    g_arenaservers.master.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.master.generic.id = 10i32;
    g_arenaservers.master.generic.x = 320i32;
    g_arenaservers.master.generic.y = y;
    g_arenaservers.master.itemnames = master_items.as_mut_ptr();
    y += 16i32;
    g_arenaservers.gametype.generic.type_0 = 3i32;
    g_arenaservers.gametype.generic.name = b"Game Type:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.gametype.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    g_arenaservers.gametype.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.gametype.generic.id = 11i32;
    g_arenaservers.gametype.generic.x = 320i32;
    g_arenaservers.gametype.generic.y = y;
    g_arenaservers.gametype.itemnames = servertype_items.as_mut_ptr();
    y += 16i32;
    g_arenaservers.sortkey.generic.type_0 = 3i32;
    g_arenaservers.sortkey.generic.name = b"Sort By:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.sortkey.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    g_arenaservers.sortkey.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.sortkey.generic.id = 12i32;
    g_arenaservers.sortkey.generic.x = 320i32;
    g_arenaservers.sortkey.generic.y = y;
    g_arenaservers.sortkey.itemnames = sortkey_items.as_mut_ptr();
    y += 16i32;
    g_arenaservers.showfull.generic.type_0 = 5i32;
    g_arenaservers.showfull.generic.name = b"Show Full:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.showfull.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    g_arenaservers.showfull.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.showfull.generic.id = 13i32;
    g_arenaservers.showfull.generic.x = 320i32;
    g_arenaservers.showfull.generic.y = y;
    y += 16i32;
    g_arenaservers.showempty.generic.type_0 = 5i32;
    g_arenaservers.showempty.generic.name = b"Show Empty:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.showempty.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    g_arenaservers.showempty.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.showempty.generic.id = 14i32;
    g_arenaservers.showempty.generic.x = 320i32;
    g_arenaservers.showempty.generic.y = y;
    y += 3i32 * 16i32;
    g_arenaservers.list.generic.type_0 = 8i32;
    g_arenaservers.list.generic.flags = 0x80i32 as libc::c_uint;
    g_arenaservers.list.generic.id = 15i32;
    g_arenaservers.list.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.list.generic.x = 72i32;
    g_arenaservers.list.generic.y = y;
    g_arenaservers.list.width = 68i32;
    g_arenaservers.list.height = 11i32;
    g_arenaservers.list.itemnames = g_arenaservers.items.as_mut_ptr() as *mut *const libc::c_char;
    i = 0i32;
    while i < 128i32 {
        g_arenaservers.items[i as usize] = g_arenaservers.table[i as usize].buff.as_mut_ptr();
        i += 1
    }
    g_arenaservers.mappic.generic.type_0 = 6i32;
    g_arenaservers.mappic.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    g_arenaservers.mappic.generic.x = 72i32;
    g_arenaservers.mappic.generic.y = 80i32;
    g_arenaservers.mappic.width = 128i32;
    g_arenaservers.mappic.height = 96i32;
    g_arenaservers.mappic.errorpic =
        b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.arrows.generic.type_0 = 6i32;
    g_arenaservers.arrows.generic.name =
        b"menu/art/arrows_vert_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.arrows.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    g_arenaservers.arrows.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.arrows.generic.x = 512i32 + 48i32;
    g_arenaservers.arrows.generic.y = 240i32 - 64i32 + 16i32;
    g_arenaservers.arrows.width = 64i32;
    g_arenaservers.arrows.height = 128i32;
    g_arenaservers.up.generic.type_0 = 6i32;
    g_arenaservers.up.generic.flags =
        0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x800i32 as libc::c_uint;
    g_arenaservers.up.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.up.generic.id = 16i32;
    g_arenaservers.up.generic.x = 512i32 + 48i32;
    g_arenaservers.up.generic.y = 240i32 - 64i32 + 16i32;
    g_arenaservers.up.width = 64i32;
    g_arenaservers.up.height = 64i32;
    g_arenaservers.up.focuspic =
        b"menu/art/arrows_vert_top\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.down.generic.type_0 = 6i32;
    g_arenaservers.down.generic.flags =
        0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x800i32 as libc::c_uint;
    g_arenaservers.down.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.down.generic.id = 17i32;
    g_arenaservers.down.generic.x = 512i32 + 48i32;
    g_arenaservers.down.generic.y = 240i32 + 16i32;
    g_arenaservers.down.width = 64i32;
    g_arenaservers.down.height = 64i32;
    g_arenaservers.down.focuspic =
        b"menu/art/arrows_vert_bot\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    y = 376i32;
    g_arenaservers.status.generic.type_0 = 7i32;
    g_arenaservers.status.generic.x = 320i32;
    g_arenaservers.status.generic.y = y;
    g_arenaservers.status.string = statusbuffer.as_mut_ptr();
    g_arenaservers.status.style = 0x1i32 | 0x10i32;
    g_arenaservers.status.color = menu_text_color.as_mut_ptr();
    y += 16i32;
    g_arenaservers.statusbar.generic.type_0 = 7i32;
    g_arenaservers.statusbar.generic.x = 320i32;
    g_arenaservers.statusbar.generic.y = y;
    g_arenaservers.statusbar.string =
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.statusbar.style = 0x1i32 | 0x10i32;
    g_arenaservers.statusbar.color = text_color_normal.as_mut_ptr();
    g_arenaservers.remove.generic.type_0 = 6i32;
    g_arenaservers.remove.generic.name =
        b"menu/art/delete_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.remove.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    g_arenaservers.remove.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.remove.generic.id = 23i32;
    g_arenaservers.remove.generic.x = 450i32;
    g_arenaservers.remove.generic.y = 86i32;
    g_arenaservers.remove.width = 96i32;
    g_arenaservers.remove.height = 48i32;
    g_arenaservers.remove.focuspic =
        b"menu/art/delete_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.back.generic.type_0 = 6i32;
    g_arenaservers.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    g_arenaservers.back.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.back.generic.id = 18i32;
    g_arenaservers.back.generic.x = 0i32;
    g_arenaservers.back.generic.y = 480i32 - 64i32;
    g_arenaservers.back.width = 128i32;
    g_arenaservers.back.height = 64i32;
    g_arenaservers.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.specify.generic.type_0 = 6i32;
    g_arenaservers.specify.generic.name =
        b"menu/art/specify_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.specify.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    g_arenaservers.specify.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.specify.generic.id = 20i32;
    g_arenaservers.specify.generic.x = 128i32;
    g_arenaservers.specify.generic.y = 480i32 - 64i32;
    g_arenaservers.specify.width = 128i32;
    g_arenaservers.specify.height = 64i32;
    g_arenaservers.specify.focuspic =
        b"menu/art/specify_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.refresh.generic.type_0 = 6i32;
    g_arenaservers.refresh.generic.name =
        b"menu/art/refresh_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.refresh.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    g_arenaservers.refresh.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.refresh.generic.id = 19i32;
    g_arenaservers.refresh.generic.x = 256i32;
    g_arenaservers.refresh.generic.y = 480i32 - 64i32;
    g_arenaservers.refresh.width = 128i32;
    g_arenaservers.refresh.height = 64i32;
    g_arenaservers.refresh.focuspic =
        b"menu/art/refresh_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.create.generic.type_0 = 6i32;
    g_arenaservers.create.generic.name =
        b"menu/art/create_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.create.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    g_arenaservers.create.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.create.generic.id = 21i32;
    g_arenaservers.create.generic.x = 384i32;
    g_arenaservers.create.generic.y = 480i32 - 64i32;
    g_arenaservers.create.width = 128i32;
    g_arenaservers.create.height = 64i32;
    g_arenaservers.create.focuspic =
        b"menu/art/create_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.go.generic.type_0 = 6i32;
    g_arenaservers.go.generic.name = b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.go.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    g_arenaservers.go.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.go.generic.id = 22i32;
    g_arenaservers.go.generic.x = 640i32;
    g_arenaservers.go.generic.y = 480i32 - 64i32;
    g_arenaservers.go.width = 128i32;
    g_arenaservers.go.height = 64i32;
    g_arenaservers.go.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_arenaservers.punkbuster.generic.type_0 = 3i32;
    g_arenaservers.punkbuster.generic.name = b"Punkbuster:\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.punkbuster.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    g_arenaservers.punkbuster.generic.callback = Some(ArenaServers_Event);
    g_arenaservers.punkbuster.generic.id = 24i32;
    g_arenaservers.punkbuster.generic.x = 480i32 + 32i32;
    g_arenaservers.punkbuster.generic.y = 144i32;
    g_arenaservers.punkbuster.itemnames = punkbuster_items.as_mut_ptr();
    g_arenaservers.pblogo.generic.type_0 = 6i32;
    g_arenaservers.pblogo.generic.name = b"menu/art/pblogo\x00" as *const u8 as *const libc::c_char;
    g_arenaservers.pblogo.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    g_arenaservers.pblogo.generic.x = 526i32;
    g_arenaservers.pblogo.generic.y = 176i32;
    g_arenaservers.pblogo.width = 32i32;
    g_arenaservers.pblogo.height = 16i32;
    g_arenaservers.pblogo.errorpic =
        b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.master as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.gametype as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.sortkey as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.showfull as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.showempty as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.mappic as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.status as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.statusbar as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.up as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.down as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.list as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.remove as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.specify as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.refresh as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.create as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.go as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.punkbuster as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.pblogo as *mut menubitmap_s as *mut libc::c_void,
    );
    ArenaServers_LoadFavorites();
    g_servertype = Com_Clamp(
        0i32 as libc::c_float,
        (8i32 - 1i32) as libc::c_float,
        ui_browserMaster.integer as libc::c_float,
    ) as libc::c_int;
    g_arenaservers.master.curvalue = g_servertype;
    g_gametype = Com_Clamp(
        0i32 as libc::c_float,
        (5i32 - 1i32) as libc::c_float,
        ui_browserGameType.integer as libc::c_float,
    ) as libc::c_int;
    g_arenaservers.gametype.curvalue = g_gametype;
    g_sortkey = Com_Clamp(
        0i32 as libc::c_float,
        (5i32 - 1i32) as libc::c_float,
        ui_browserSortKey.integer as libc::c_float,
    ) as libc::c_int;
    g_arenaservers.sortkey.curvalue = g_sortkey;
    g_fullservers = Com_Clamp(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        ui_browserShowFull.integer as libc::c_float,
    ) as libc::c_int;
    g_arenaservers.showfull.curvalue = g_fullservers;
    g_emptyservers = Com_Clamp(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        ui_browserShowEmpty.integer as libc::c_float,
    ) as libc::c_int;
    g_arenaservers.showempty.curvalue = g_emptyservers;
    g_arenaservers.punkbuster.curvalue = Com_Clamp(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        trap_Cvar_VariableValue(b"cl_punkbuster\x00" as *const u8 as *const libc::c_char),
    ) as libc::c_int;
    g_servertype = ArenaServers_SetType(g_servertype);
    g_arenaservers.master.curvalue = g_servertype;
    trap_Cvar_Register(
        0 as *mut vmCvar_t,
        b"debug_protocol\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0i32,
    );
}
static mut g_servertype: libc::c_int = 0;
/*
=================
ArenaServers_SetType
=================
*/
#[no_mangle]
pub unsafe extern "C" fn ArenaServers_SetType(mut type_0: libc::c_int) -> libc::c_int {
    ArenaServers_StopRefresh();
    if type_0 >= 2i32 && type_0 <= 6i32 {
        let mut masterstr: [libc::c_char; 2] = [0; 2];
        let mut cvarname: [libc::c_char; 11] = [0; 11];
        let mut direction: libc::c_int = 0;
        if type_0 == g_servertype || type_0 == (g_servertype + 1i32) % 8i32 {
            direction = 1i32
        } else {
            direction = -1i32
        }
        while type_0 >= 2i32 && type_0 <= 6i32 {
            Com_sprintf(
                cvarname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 11]>() as libc::c_ulong as libc::c_int,
                b"sv_master%d\x00" as *const u8 as *const libc::c_char,
                type_0 - 1i32,
            );
            trap_Cvar_VariableStringBuffer(
                cvarname.as_mut_ptr(),
                masterstr.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as libc::c_int,
            );
            if 0 != *masterstr.as_mut_ptr() {
                break;
            }
            type_0 += direction
        }
    }
    g_servertype = type_0;
    match type_0 {
        1 | 2 | 3 | 4 | 5 | 6 => {
            g_arenaservers.remove.generic.flags |=
                0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint;
            g_arenaservers.serverlist = g_globalserverlist[(type_0 - 1i32) as usize].as_mut_ptr();
            g_arenaservers.numservers = &mut *g_numglobalservers
                .as_mut_ptr()
                .offset((type_0 - 1i32) as isize)
                as *mut libc::c_int;
            g_arenaservers.maxservers = 128i32
        }
        7 => {
            g_arenaservers.remove.generic.flags &=
                !(0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint);
            g_arenaservers.serverlist = g_favoriteserverlist.as_mut_ptr();
            g_arenaservers.numservers = &mut g_numfavoriteservers;
            g_arenaservers.maxservers = 16i32
        }
        0 | _ => {
            g_arenaservers.remove.generic.flags |=
                0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint;
            g_arenaservers.serverlist = g_localserverlist.as_mut_ptr();
            g_arenaservers.numservers = &mut g_numlocalservers;
            g_arenaservers.maxservers = 128i32
        }
    }
    if 0 == *g_arenaservers.numservers {
        ArenaServers_StartRefresh();
    } else {
        g_arenaservers.currentping = *g_arenaservers.numservers;
        g_arenaservers.numqueriedservers = *g_arenaservers.numservers;
        ArenaServers_UpdateMenu();
        strcpy(
            g_arenaservers.status.string,
            b"hit refresh to update\x00" as *const u8 as *const libc::c_char,
        );
    }
    return type_0;
}
/*
=================
ArenaServers_UpdateMenu
=================
*/
unsafe extern "C" fn ArenaServers_UpdateMenu() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut buff: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    let mut tableptr: *mut table_t = 0 as *mut table_t;
    let mut pingColor: *mut libc::c_char = 0 as *mut libc::c_char;
    if g_arenaservers.numqueriedservers > 0i32 {
        if 0 != g_arenaservers.refreshservers as libc::c_uint
            && g_arenaservers.currentping <= g_arenaservers.numqueriedservers
        {
            Com_sprintf(
                g_arenaservers.status.string,
                64i32,
                b"%d of %d Arena Servers.\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.currentping,
                g_arenaservers.numqueriedservers,
            );
            g_arenaservers.statusbar.string =
                b"Press SPACE to stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
            qsort(
                g_arenaservers.serverlist as *mut libc::c_void,
                *g_arenaservers.numservers as size_t,
                ::std::mem::size_of::<servernode_t>() as libc::c_ulong,
                Some(ArenaServers_Compare),
            );
        } else {
            g_arenaservers.gametype.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.sortkey.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.showempty.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.showfull.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.list.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.refresh.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.go.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.punkbuster.generic.flags &= !(0x2000i32 as libc::c_uint);
            if g_servertype >= 1i32 && g_servertype <= 6i32 {
                g_arenaservers.statusbar.string = quake3worldMessage.as_mut_ptr()
            } else {
                g_arenaservers.statusbar.string =
                    b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
        }
    } else {
        if 0 != g_arenaservers.refreshservers as u64 {
            strcpy(
                g_arenaservers.status.string,
                b"Scanning For Servers.\x00" as *const u8 as *const libc::c_char,
            );
            g_arenaservers.statusbar.string =
                b"Press SPACE to stop\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
            g_arenaservers.gametype.generic.flags |= 0x2000i32 as libc::c_uint;
            g_arenaservers.sortkey.generic.flags |= 0x2000i32 as libc::c_uint;
            g_arenaservers.showempty.generic.flags |= 0x2000i32 as libc::c_uint;
            g_arenaservers.showfull.generic.flags |= 0x2000i32 as libc::c_uint;
            g_arenaservers.list.generic.flags |= 0x2000i32 as libc::c_uint;
            g_arenaservers.refresh.generic.flags |= 0x2000i32 as libc::c_uint;
            g_arenaservers.go.generic.flags |= 0x2000i32 as libc::c_uint;
            g_arenaservers.punkbuster.generic.flags |= 0x2000i32 as libc::c_uint
        } else {
            if g_arenaservers.numqueriedservers < 0i32 {
                strcpy(
                    g_arenaservers.status.string,
                    b"No Response From Master Server.\x00" as *const u8 as *const libc::c_char,
                );
            } else {
                strcpy(
                    g_arenaservers.status.string,
                    b"No Servers Found.\x00" as *const u8 as *const libc::c_char,
                );
            }
            if g_servertype >= 1i32 && g_servertype <= 6i32 {
                g_arenaservers.statusbar.string = quake3worldMessage.as_mut_ptr()
            } else {
                g_arenaservers.statusbar.string =
                    b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
            g_arenaservers.master.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.gametype.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.sortkey.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.showempty.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.showfull.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.list.generic.flags |= 0x2000i32 as libc::c_uint;
            g_arenaservers.refresh.generic.flags &= !(0x2000i32 as libc::c_uint);
            g_arenaservers.go.generic.flags |= 0x2000i32 as libc::c_uint;
            g_arenaservers.punkbuster.generic.flags &= !(0x2000i32 as libc::c_uint)
        }
        g_arenaservers.list.numitems = 0i32;
        g_arenaservers.list.curvalue = 0i32;
        g_arenaservers.list.top = 0i32;
        ArenaServers_UpdatePicture();
        return;
    }
    servernodeptr = g_arenaservers.serverlist;
    count = *g_arenaservers.numservers;
    let mut current_block_80: u64;
    i = 0i32;
    j = 0i32;
    while i < count {
        tableptr = &mut *g_arenaservers.table.as_mut_ptr().offset(j as isize) as *mut table_t;
        (*tableptr).servernode = servernodeptr;
        buff = (*tableptr).buff.as_mut_ptr();
        // can only cull valid results
        if !(0 == g_emptyservers && 0 == (*servernodeptr).numclients) {
            if !(0 == g_fullservers && (*servernodeptr).numclients == (*servernodeptr).maxclients) {
                match g_gametype {
                    1 => {
                        if (*servernodeptr).gametype != GT_FFA as libc::c_int {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    2 => {
                        if (*servernodeptr).gametype != GT_TEAM as libc::c_int {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    3 => {
                        if (*servernodeptr).gametype != GT_TOURNAMENT as libc::c_int {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    4 => {
                        if (*servernodeptr).gametype != GT_CTF as libc::c_int {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    0 | _ => {
                        current_block_80 = 17441561948628420366;
                    }
                }
                match current_block_80 {
                    13325891313334703151 => {}
                    _ => {
                        if (*servernodeptr).pingtime < (*servernodeptr).minPing {
                            pingColor =
                                b"^4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                        } else if 0 != (*servernodeptr).maxPing
                            && (*servernodeptr).pingtime > (*servernodeptr).maxPing
                        {
                            pingColor =
                                b"^4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                        } else if (*servernodeptr).pingtime < 200i32 {
                            pingColor =
                                b"^2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                        } else if (*servernodeptr).pingtime < 400i32 {
                            pingColor =
                                b"^3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                        } else {
                            pingColor =
                                b"^1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
                        }
                        Com_sprintf(
                            buff,
                            68i32,
                            b"%-20.20s %-12.12s %2d/%2d %-8.8s %4s%s%3d ^3%s\x00" as *const u8
                                as *const libc::c_char,
                            (*servernodeptr).hostname.as_mut_ptr(),
                            (*servernodeptr).mapname.as_mut_ptr(),
                            (*servernodeptr).numclients,
                            (*servernodeptr).maxclients,
                            (*servernodeptr).gamename.as_mut_ptr(),
                            netnames[(*servernodeptr).nettype as usize],
                            pingColor,
                            (*servernodeptr).pingtime,
                            if 0 != (*servernodeptr).bPB as libc::c_uint {
                                b"Yes\x00" as *const u8 as *const libc::c_char
                            } else {
                                b"No\x00" as *const u8 as *const libc::c_char
                            },
                        );
                        j += 1
                    }
                }
            }
        }
        i += 1;
        servernodeptr = servernodeptr.offset(1isize)
    }
    g_arenaservers.list.numitems = j;
    g_arenaservers.list.curvalue = 0i32;
    g_arenaservers.list.top = 0i32;
    ArenaServers_UpdatePicture();
}
/*
=================
ArenaServers_UpdatePicture
=================
*/
unsafe extern "C" fn ArenaServers_UpdatePicture() {
    static mut picname: [libc::c_char; 64] = [0; 64];
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    if 0 == g_arenaservers.list.numitems {
        g_arenaservers.mappic.generic.name = 0 as *const libc::c_char
    } else {
        servernodeptr = g_arenaservers.table[g_arenaservers.list.curvalue as usize].servernode;
        Com_sprintf(
            picname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"levelshots/%s.tga\x00" as *const u8 as *const libc::c_char,
            (*servernodeptr).mapname.as_mut_ptr(),
        );
        g_arenaservers.mappic.generic.name = picname.as_mut_ptr()
    }
    g_arenaservers.mappic.shader = 0i32;
}
// deathmatch
// tournament
// single player
// team deathmatch
// capture the flag
// one flag ctf
// Overload
// Harvester
// Rocket Arena 3
// Q3F
// Urban Terror
// Orange Smoothie Productions
// unknown
static mut netnames: [*mut libc::c_char; 4] = [
    b"??? \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"UDP \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"UDP6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
static mut g_gametype: libc::c_int = 0;
static mut g_fullservers: libc::c_int = 0;
static mut g_emptyservers: libc::c_int = 0;
static mut quake3worldMessage: [libc::c_char; 59] = [
    86, 105, 115, 105, 116, 32, 119, 119, 119, 46, 113, 117, 97, 107, 101, 51, 119, 111, 114, 108,
    100, 46, 99, 111, 109, 32, 45, 32, 78, 101, 119, 115, 44, 32, 67, 111, 109, 109, 117, 110, 105,
    116, 121, 44, 32, 69, 118, 101, 110, 116, 115, 44, 32, 70, 105, 108, 101, 115, 0,
];
/*
=================
ArenaServers_Compare
=================
*/
unsafe extern "C" fn ArenaServers_Compare(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> libc::c_int {
    let mut f1: libc::c_float = 0.;
    let mut f2: libc::c_float = 0.;
    let mut t1: *mut servernode_t = 0 as *mut servernode_t;
    let mut t2: *mut servernode_t = 0 as *mut servernode_t;
    t1 = arg1 as *mut servernode_t;
    t2 = arg2 as *mut servernode_t;
    match g_sortkey {
        0 => return Q_stricmp((*t1).hostname.as_mut_ptr(), (*t2).hostname.as_mut_ptr()),
        1 => return Q_stricmp((*t1).mapname.as_mut_ptr(), (*t2).mapname.as_mut_ptr()),
        2 => {
            f1 = ((*t1).maxclients - (*t1).numclients) as libc::c_float;
            if f1 < 0i32 as libc::c_float {
                f1 = 0i32 as libc::c_float
            }
            f2 = ((*t2).maxclients - (*t2).numclients) as libc::c_float;
            if f2 < 0i32 as libc::c_float {
                f2 = 0i32 as libc::c_float
            }
            if f1 < f2 {
                return 1i32;
            }
            if f1 == f2 {
                return 0i32;
            }
            return -1i32;
        }
        3 => {
            if (*t1).gametype < (*t2).gametype {
                return -1i32;
            }
            if (*t1).gametype == (*t2).gametype {
                return 0i32;
            }
            return 1i32;
        }
        4 => {
            if (*t1).pingtime < (*t2).pingtime {
                return -1i32;
            }
            if (*t1).pingtime > (*t2).pingtime {
                return 1i32;
            }
            return Q_stricmp((*t1).hostname.as_mut_ptr(), (*t2).hostname.as_mut_ptr());
        }
        _ => {}
    }
    return 0i32;
}
static mut g_sortkey: libc::c_int = 0;
/*
=================
ArenaServers_StartRefresh
=================
*/
unsafe extern "C" fn ArenaServers_StartRefresh() {
    let mut i: libc::c_int = 0;
    let mut myargs: [libc::c_char; 32] = [0; 32];
    let mut protocol: [libc::c_char; 32] = [0; 32];
    memset(
        g_arenaservers.serverlist as *mut libc::c_void,
        0i32,
        (g_arenaservers.maxservers as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<table_t>() as libc::c_ulong),
    );
    i = 0i32;
    while i < 32i32 {
        g_arenaservers.pinglist[i as usize].adrstr[0usize] = '\u{0}' as i32 as libc::c_char;
        trap_LAN_ClearPing(i);
        i += 1
    }
    g_arenaservers.refreshservers = qtrue;
    g_arenaservers.currentping = 0i32;
    g_arenaservers.nextpingtime = 0i32;
    *g_arenaservers.numservers = 0i32;
    g_arenaservers.numqueriedservers = 0i32;
    g_arenaservers.refreshtime = uis.realtime + 5000i32;
    ArenaServers_UpdateMenu();
    if g_servertype == 0i32 {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            b"localservers\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if g_servertype >= 1i32 && g_servertype <= 6i32 {
        match g_arenaservers.gametype.curvalue {
            1 => {
                strcpy(
                    myargs.as_mut_ptr(),
                    b" ffa\x00" as *const u8 as *const libc::c_char,
                );
            }
            2 => {
                strcpy(
                    myargs.as_mut_ptr(),
                    b" team\x00" as *const u8 as *const libc::c_char,
                );
            }
            3 => {
                strcpy(
                    myargs.as_mut_ptr(),
                    b" tourney\x00" as *const u8 as *const libc::c_char,
                );
            }
            4 => {
                strcpy(
                    myargs.as_mut_ptr(),
                    b" ctf\x00" as *const u8 as *const libc::c_char,
                );
            }
            0 | _ => myargs[0usize] = 0i32 as libc::c_char,
        }
        if 0 != g_emptyservers {
            strcat(
                myargs.as_mut_ptr(),
                b" empty\x00" as *const u8 as *const libc::c_char,
            );
        }
        if 0 != g_fullservers {
            strcat(
                myargs.as_mut_ptr(),
                b" full\x00" as *const u8 as *const libc::c_char,
            );
        }
        protocol[0usize] = '\u{0}' as i32 as libc::c_char;
        trap_Cvar_VariableStringBuffer(
            b"debug_protocol\x00" as *const u8 as *const libc::c_char,
            protocol.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        );
        if 0 != strlen(protocol.as_mut_ptr()) {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                va(
                    b"globalservers %d %s%s\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g_servertype - 1i32,
                    protocol.as_mut_ptr(),
                    myargs.as_mut_ptr(),
                ),
            );
        } else {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                va(
                    b"globalservers %d %d%s\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    g_servertype - 1i32,
                    trap_Cvar_VariableValue(b"protocol\x00" as *const u8 as *const libc::c_char)
                        as libc::c_int,
                    myargs.as_mut_ptr(),
                ),
            );
        }
    };
}
static mut g_numfavoriteservers: libc::c_int = 0;
static mut g_favoriteserverlist: [servernode_t; 16] = [servernode_s {
    adrstr: [0; 64],
    hostname: [0; 25],
    mapname: [0; 16],
    numclients: 0,
    maxclients: 0,
    pingtime: 0,
    gametype: 0,
    gamename: [0; 12],
    nettype: 0,
    minPing: 0,
    maxPing: 0,
    bPB: qfalse,
}; 16];
static mut g_numglobalservers: [libc::c_int; 6] = [0; 6];
static mut g_globalserverlist: [[servernode_t; 128]; 6] = [[servernode_s {
    adrstr: [0; 64],
    hostname: [0; 25],
    mapname: [0; 16],
    numclients: 0,
    maxclients: 0,
    pingtime: 0,
    gametype: 0,
    gamename: [0; 12],
    nettype: 0,
    minPing: 0,
    maxPing: 0,
    bPB: qfalse,
}; 128]; 6];
static mut g_numlocalservers: libc::c_int = 0;
static mut g_localserverlist: [servernode_t; 128] = [servernode_s {
    adrstr: [0; 64],
    hostname: [0; 25],
    mapname: [0; 16],
    numclients: 0,
    maxclients: 0,
    pingtime: 0,
    gametype: 0,
    gamename: [0; 12],
    nettype: 0,
    minPing: 0,
    maxPing: 0,
    bPB: qfalse,
}; 128];
/*
=================
ArenaServers_StopRefresh
=================
*/
unsafe extern "C" fn ArenaServers_StopRefresh() {
    if 0 == g_arenaservers.refreshservers as u64 {
        return;
    }
    g_arenaservers.refreshservers = qfalse;
    if g_arenaservers.numqueriedservers >= 0i32 {
        g_arenaservers.currentping = *g_arenaservers.numservers;
        g_arenaservers.numqueriedservers = *g_arenaservers.numservers
    }
    qsort(
        g_arenaservers.serverlist as *mut libc::c_void,
        *g_arenaservers.numservers as size_t,
        ::std::mem::size_of::<servernode_t>() as libc::c_ulong,
        Some(ArenaServers_Compare),
    );
    ArenaServers_UpdateMenu();
}
/*
=================
ArenaServers_LoadFavorites

Load cvar address book entries into local lists.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn ArenaServers_LoadFavorites() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut numtempitems: libc::c_int = 0;
    let mut adrstr: [libc::c_char; 64] = [0; 64];
    let mut templist: [servernode_t; 16] = [servernode_s {
        adrstr: [0; 64],
        hostname: [0; 25],
        mapname: [0; 16],
        numclients: 0,
        maxclients: 0,
        pingtime: 0,
        gametype: 0,
        gamename: [0; 12],
        nettype: 0,
        minPing: 0,
        maxPing: 0,
        bPB: qfalse,
    }; 16];
    let mut found: qboolean = qfalse;
    found = qfalse;
    memcpy(
        templist.as_mut_ptr() as *mut libc::c_void,
        g_favoriteserverlist.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<servernode_t>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong),
    );
    numtempitems = g_numfavoriteservers;
    memset(
        g_favoriteserverlist.as_mut_ptr() as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<servernode_t>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong),
    );
    g_numfavoriteservers = 0i32;
    i = 0i32;
    while i < 16i32 {
        trap_Cvar_VariableStringBuffer(
            va(
                b"server%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i + 1i32,
            ),
            adrstr.as_mut_ptr(),
            64i32,
        );
        if !(0 == adrstr[0usize]) {
            strcpy(
                g_arenaservers.favoriteaddresses[g_numfavoriteservers as usize].as_mut_ptr(),
                adrstr.as_mut_ptr(),
            );
            j = 0i32;
            while j < numtempitems {
                if 0 == Q_stricmp(
                    templist[j as usize].adrstr.as_mut_ptr(),
                    adrstr.as_mut_ptr(),
                ) {
                    break;
                }
                j += 1
            }
            if j < numtempitems {
                memcpy(
                    &mut *g_favoriteserverlist
                        .as_mut_ptr()
                        .offset(g_numfavoriteservers as isize)
                        as *mut servernode_t as *mut libc::c_void,
                    &mut *templist.as_mut_ptr().offset(j as isize) as *mut servernode_t
                        as *const libc::c_void,
                    ::std::mem::size_of::<servernode_t>() as libc::c_ulong,
                );
                found = qtrue
            } else {
                Q_strncpyz(
                    g_favoriteserverlist[g_numfavoriteservers as usize]
                        .adrstr
                        .as_mut_ptr(),
                    adrstr.as_mut_ptr(),
                    64i32,
                );
                g_favoriteserverlist[g_numfavoriteservers as usize].pingtime =
                    ArenaServers_MaxPing()
            }
            g_numfavoriteservers += 1
        }
        i += 1
    }
    g_arenaservers.numfavoriteaddresses = g_numfavoriteservers;
    if 0 == found as u64 {
        g_numfavoriteservers = 0i32
    };
}
/*
=================
ArenaServers_MaxPing
=================
*/
unsafe extern "C" fn ArenaServers_MaxPing() -> libc::c_int {
    let mut maxPing: libc::c_int = 0;
    maxPing = trap_Cvar_VariableValue(b"cl_maxPing\x00" as *const u8 as *const libc::c_char)
        as libc::c_int;
    if maxPing < 100i32 {
        maxPing = 100i32
    }
    return maxPing;
}
#[no_mangle]
pub static mut punkbuster_items: [*const libc::c_char; 3] = [
    b"Disabled\x00" as *const u8 as *const libc::c_char,
    b"Enabled\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
ArenaServers_Event
=================
*/
unsafe extern "C" fn ArenaServers_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    let mut id: libc::c_int = 0;
    id = (*(ptr as *mut menucommon_s)).id;
    if event != 3i32 && id != 15i32 {
        return;
    }
    match id {
        10 => {
            g_arenaservers.master.curvalue = ArenaServers_SetType(g_arenaservers.master.curvalue);
            trap_Cvar_SetValue(
                b"ui_browserMaster\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.master.curvalue as libc::c_float,
            );
        }
        11 => {
            trap_Cvar_SetValue(
                b"ui_browserGameType\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.gametype.curvalue as libc::c_float,
            );
            g_gametype = g_arenaservers.gametype.curvalue;
            ArenaServers_UpdateMenu();
        }
        12 => {
            trap_Cvar_SetValue(
                b"ui_browserSortKey\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.sortkey.curvalue as libc::c_float,
            );
            ArenaServers_Sort(g_arenaservers.sortkey.curvalue);
            ArenaServers_UpdateMenu();
        }
        13 => {
            trap_Cvar_SetValue(
                b"ui_browserShowFull\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.showfull.curvalue as libc::c_float,
            );
            g_fullservers = g_arenaservers.showfull.curvalue;
            ArenaServers_UpdateMenu();
        }
        14 => {
            trap_Cvar_SetValue(
                b"ui_browserShowEmpty\x00" as *const u8 as *const libc::c_char,
                g_arenaservers.showempty.curvalue as libc::c_float,
            );
            g_emptyservers = g_arenaservers.showempty.curvalue;
            ArenaServers_UpdateMenu();
        }
        15 => {
            if event == 1i32 {
                ArenaServers_UpdatePicture();
            }
        }
        16 => {
            ScrollList_Key(&mut g_arenaservers.list, K_UPARROW as libc::c_int);
        }
        17 => {
            ScrollList_Key(&mut g_arenaservers.list, K_DOWNARROW as libc::c_int);
        }
        18 => {
            ArenaServers_StopRefresh();
            ArenaServers_SaveChanges();
            UI_PopMenu();
        }
        19 => {
            ArenaServers_StartRefresh();
        }
        20 => {
            UI_SpecifyServerMenu();
        }
        21 => {
            UI_StartServerMenu(qtrue);
        }
        22 => {
            ArenaServers_Go();
        }
        23 => {
            ArenaServers_Remove();
            ArenaServers_UpdateMenu();
        }
        24 => {
            if 0 != g_arenaservers.punkbuster.curvalue {
                UI_ConfirmMenu_Style(
                    b"Enable Punkbuster?\x00" as *const u8 as *const libc::c_char,
                    0x1i32 | 0x2000i32 | 0x10i32,
                    None,
                    Some(Punkbuster_ConfirmEnable),
                );
            } else {
                UI_ConfirmMenu_Style(
                    b"Disable Punkbuster?\x00" as *const u8 as *const libc::c_char,
                    0x1i32 | 0x2000i32 | 0x10i32,
                    None,
                    Some(Punkbuster_ConfirmDisable),
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn Punkbuster_ConfirmDisable(mut result: qboolean) {
    if 0 != result as u64 {
        trap_SetPbClStatus(0i32);
        UI_Message(punkbuster_msg.as_mut_ptr());
    }
    g_arenaservers.punkbuster.curvalue = Com_Clamp(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        trap_Cvar_VariableValue(b"cl_punkbuster\x00" as *const u8 as *const libc::c_char),
    ) as libc::c_int;
}
#[no_mangle]
pub static mut punkbuster_msg: [*const libc::c_char; 5] = [
    b"PunkBuster will be\x00" as *const u8 as *const libc::c_char,
    b"disabled the next time\x00" as *const u8 as *const libc::c_char,
    b"Quake III Arena\x00" as *const u8 as *const libc::c_char,
    b"is started.\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
PunkBuster_Confirm
=================
*/
unsafe extern "C" fn Punkbuster_ConfirmEnable(mut result: qboolean) {
    if 0 != result as u64 {
        trap_SetPbClStatus(1i32);
    }
    g_arenaservers.punkbuster.curvalue = Com_Clamp(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        trap_Cvar_VariableValue(b"cl_punkbuster\x00" as *const u8 as *const libc::c_char),
    ) as libc::c_int;
}
/*
=================
ArenaServers_Remove
=================
*/
unsafe extern "C" fn ArenaServers_Remove() {
    let mut i: libc::c_int = 0;
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    let mut tableptr: *mut table_t = 0 as *mut table_t;
    if 0 == g_arenaservers.list.numitems {
        return;
    }
    tableptr = &mut *g_arenaservers
        .table
        .as_mut_ptr()
        .offset(g_arenaservers.list.curvalue as isize) as *mut table_t;
    servernodeptr = (*tableptr).servernode;
    i = 0i32;
    while i < g_arenaservers.numfavoriteaddresses {
        if 0 == Q_stricmp(
            g_arenaservers.favoriteaddresses[i as usize].as_mut_ptr(),
            (*servernodeptr).adrstr.as_mut_ptr(),
        ) {
            if i < g_arenaservers.numfavoriteaddresses - 1i32 {
                memcpy(
                    &mut *g_arenaservers
                        .favoriteaddresses
                        .as_mut_ptr()
                        .offset(i as isize) as *mut [libc::c_char; 64]
                        as *mut libc::c_void,
                    &mut *g_arenaservers
                        .favoriteaddresses
                        .as_mut_ptr()
                        .offset((i + 1i32) as isize) as *mut [libc::c_char; 64]
                        as *const libc::c_void,
                    ((g_arenaservers.numfavoriteaddresses - i - 1i32) * 64i32) as libc::c_ulong,
                );
            }
            g_arenaservers.numfavoriteaddresses -= 1;
            memset(
                &mut *g_arenaservers
                    .favoriteaddresses
                    .as_mut_ptr()
                    .offset(g_arenaservers.numfavoriteaddresses as isize)
                    as *mut [libc::c_char; 64] as *mut libc::c_void,
                0i32,
                64i32 as libc::c_ulong,
            );
            break;
        } else {
            i += 1
        }
    }
    i = 0i32;
    while i < g_numfavoriteservers {
        if &mut *g_favoriteserverlist.as_mut_ptr().offset(i as isize) as *mut servernode_t
            == servernodeptr
        {
            if i < g_numfavoriteservers - 1i32 {
                memcpy(
                    &mut *g_favoriteserverlist.as_mut_ptr().offset(i as isize) as *mut servernode_t
                        as *mut libc::c_void,
                    &mut *g_favoriteserverlist
                        .as_mut_ptr()
                        .offset((i + 1i32) as isize) as *mut servernode_t
                        as *const libc::c_void,
                    ((g_numfavoriteservers - i - 1i32) as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<servernode_t>() as libc::c_ulong),
                );
            }
            g_numfavoriteservers -= 1;
            memset(
                &mut *g_favoriteserverlist
                    .as_mut_ptr()
                    .offset(g_numfavoriteservers as isize) as *mut servernode_t
                    as *mut libc::c_void,
                0i32,
                ::std::mem::size_of::<servernode_t>() as libc::c_ulong,
            );
            break;
        } else {
            i += 1
        }
    }
    g_arenaservers.numqueriedservers = g_arenaservers.numfavoriteaddresses;
    g_arenaservers.currentping = g_arenaservers.numfavoriteaddresses;
}
/*
=================
ArenaServers_Go
=================
*/
unsafe extern "C" fn ArenaServers_Go() {
    let mut servernode: *mut servernode_t = 0 as *mut servernode_t;
    servernode = g_arenaservers.table[g_arenaservers.list.curvalue as usize].servernode;
    if !servernode.is_null() {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            va(
                b"connect %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*servernode).adrstr.as_mut_ptr(),
            ),
        );
    };
}
/*
=================
ArenaServers_SaveChanges
=================
*/
#[no_mangle]
pub unsafe extern "C" fn ArenaServers_SaveChanges() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < g_arenaservers.numfavoriteaddresses {
        trap_Cvar_Set(
            va(
                b"server%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i + 1i32,
            ),
            g_arenaservers.favoriteaddresses[i as usize].as_mut_ptr(),
        );
        i += 1
    }
    while i < 16i32 {
        trap_Cvar_Set(
            va(
                b"server%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                i + 1i32,
            ),
            b"\x00" as *const u8 as *const libc::c_char,
        );
        i += 1
    }
}
/*
=================
ArenaServers_Sort
=================
*/
#[no_mangle]
pub unsafe extern "C" fn ArenaServers_Sort(mut type_0: libc::c_int) {
    if g_sortkey == type_0 {
        return;
    }
    g_sortkey = type_0;
    qsort(
        g_arenaservers.serverlist as *mut libc::c_void,
        *g_arenaservers.numservers as size_t,
        ::std::mem::size_of::<servernode_t>() as libc::c_ulong,
        Some(ArenaServers_Compare),
    );
}
static mut sortkey_items: [*const libc::c_char; 6] = [
    b"Server Name\x00" as *const u8 as *const libc::c_char,
    b"Map Name\x00" as *const u8 as *const libc::c_char,
    b"Open Player Spots\x00" as *const u8 as *const libc::c_char,
    b"Game Type\x00" as *const u8 as *const libc::c_char,
    b"Ping Time\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut servertype_items: [*const libc::c_char; 6] = [
    b"All\x00" as *const u8 as *const libc::c_char,
    b"Free For All\x00" as *const u8 as *const libc::c_char,
    b"Team Deathmatch\x00" as *const u8 as *const libc::c_char,
    b"Tournament\x00" as *const u8 as *const libc::c_char,
    b"Capture the Flag\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
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

MULTIPLAYER MENU (SERVER BROWSER)

=======================================================================
*/
static mut master_items: [*const libc::c_char; 9] = [
    b"Local\x00" as *const u8 as *const libc::c_char,
    b"Internet\x00" as *const u8 as *const libc::c_char,
    b"Master1\x00" as *const u8 as *const libc::c_char,
    b"Master2\x00" as *const u8 as *const libc::c_char,
    b"Master3\x00" as *const u8 as *const libc::c_char,
    b"Master4\x00" as *const u8 as *const libc::c_char,
    b"Master5\x00" as *const u8 as *const libc::c_char,
    b"Favorites\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
ArenaServers_MenuKey
=================
*/
unsafe extern "C" fn ArenaServers_MenuKey(mut key: libc::c_int) -> sfxHandle_t {
    if key == K_SPACE as libc::c_int && 0 != g_arenaservers.refreshservers as libc::c_uint {
        ArenaServers_StopRefresh();
        return menu_move_sound;
    }
    if (key == K_DEL as libc::c_int || key == K_KP_DEL as libc::c_int)
        && g_servertype == 7i32
        && Menu_ItemAtCursor(&mut g_arenaservers.menu)
            == &mut g_arenaservers.list as *mut menulist_s as *mut libc::c_void
    {
        ArenaServers_Remove();
        ArenaServers_UpdateMenu();
        return menu_move_sound;
    }
    if key == K_MOUSE2 as libc::c_int || key == K_ESCAPE as libc::c_int {
        ArenaServers_StopRefresh();
        ArenaServers_SaveChanges();
    }
    return Menu_DefaultKey(&mut g_arenaservers.menu, key);
}
/*
=================
ArenaServers_MenuDraw
=================
*/
unsafe extern "C" fn ArenaServers_MenuDraw() {
    if 0 != g_arenaservers.refreshservers as u64 {
        ArenaServers_DoRefresh();
    }
    Menu_Draw(&mut g_arenaservers.menu);
}
/*
=================
ArenaServers_DoRefresh
=================
*/
unsafe extern "C" fn ArenaServers_DoRefresh() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut time: libc::c_int = 0;
    let mut maxPing: libc::c_int = 0;
    let mut adrstr: [libc::c_char; 64] = [0; 64];
    let mut info: [libc::c_char; 1024] = [0; 1024];
    if uis.realtime < g_arenaservers.refreshtime {
        if g_servertype != 7i32 {
            if g_servertype == 0i32 {
                if 0 == trap_LAN_GetServerCount(0i32) {
                    return;
                }
            }
            if trap_LAN_GetServerCount(ArenaServers_SourceForLAN()) < 0i32 {
                return;
            }
        }
    } else if g_servertype == 0i32 {
        if 0 == trap_LAN_GetServerCount(0i32) {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                b"localservers\n\x00" as *const u8 as *const libc::c_char,
            );
            g_arenaservers.refreshtime = uis.realtime + 5000i32;
            return;
        }
    }
    if uis.realtime < g_arenaservers.nextpingtime {
        return;
    }
    g_arenaservers.nextpingtime = uis.realtime + 10i32;
    maxPing = ArenaServers_MaxPing();
    let mut current_block_41: u64;
    i = 0i32;
    while i < 32i32 {
        trap_LAN_GetPing(i, adrstr.as_mut_ptr(), 64i32, &mut time);
        if !(0 == adrstr[0usize]) {
            // ignore empty or pending pings
            j = 0i32;
            while j < 32i32 {
                if 0 == Q_stricmp(
                    adrstr.as_mut_ptr(),
                    g_arenaservers.pinglist[j as usize].adrstr.as_mut_ptr(),
                ) {
                    break;
                }
                j += 1
            }
            if j < 32i32 {
                // found it
                if 0 == time {
                    time = uis.realtime - g_arenaservers.pinglist[j as usize].start;
                    if time < maxPing {
                        // still waiting
                        current_block_41 = 9828876828309294594;
                    } else {
                        current_block_41 = 6417057564578538666;
                    }
                } else {
                    current_block_41 = 6417057564578538666;
                }
                match current_block_41 {
                    9828876828309294594 => {}
                    _ => {
                        if time > maxPing {
                            info[0usize] = '\u{0}' as i32 as libc::c_char;
                            time = maxPing;
                            if g_servertype == 7i32 {
                                Info_SetValueForKey(
                                    info.as_mut_ptr(),
                                    b"hostname\x00" as *const u8 as *const libc::c_char,
                                    adrstr.as_mut_ptr(),
                                );
                                Info_SetValueForKey(
                                    info.as_mut_ptr(),
                                    b"game\x00" as *const u8 as *const libc::c_char,
                                    b"???\x00" as *const u8 as *const libc::c_char,
                                );
                            }
                        } else {
                            trap_LAN_GetPingInfo(i, info.as_mut_ptr(), 1024i32);
                        }
                        ArenaServers_Insert(adrstr.as_mut_ptr(), info.as_mut_ptr(), time);
                        g_arenaservers.pinglist[j as usize].adrstr[0usize] =
                            '\u{0}' as i32 as libc::c_char;
                        current_block_41 = 1924505913685386279;
                    }
                }
            } else {
                current_block_41 = 1924505913685386279;
            }
            match current_block_41 {
                9828876828309294594 => {}
                _ => {
                    trap_LAN_ClearPing(i);
                }
            }
        }
        i += 1
    }
    if g_servertype == 7i32 {
        g_arenaservers.numqueriedservers = g_arenaservers.numfavoriteaddresses
    } else {
        g_arenaservers.numqueriedservers = trap_LAN_GetServerCount(ArenaServers_SourceForLAN())
    }
    i = 0i32;
    while i < 32i32 && g_arenaservers.currentping < g_arenaservers.numqueriedservers {
        if trap_LAN_GetPingQueueCount() >= 32i32 {
            // ping queue is full
            break;
        } else {
            j = 0i32;
            while j < 32i32 {
                if 0 == g_arenaservers.pinglist[j as usize].adrstr[0usize] {
                    break;
                }
                j += 1
            }
            if j >= 32i32 {
                // no empty slots available yet - wait for timeout
                break;
            } else {
                if g_servertype == 7i32 {
                    strcpy(
                        adrstr.as_mut_ptr(),
                        g_arenaservers.favoriteaddresses[g_arenaservers.currentping as usize]
                            .as_mut_ptr(),
                    );
                } else {
                    trap_LAN_GetServerAddressString(
                        ArenaServers_SourceForLAN(),
                        g_arenaservers.currentping,
                        adrstr.as_mut_ptr(),
                        64i32,
                    );
                }
                strcpy(
                    g_arenaservers.pinglist[j as usize].adrstr.as_mut_ptr(),
                    adrstr.as_mut_ptr(),
                );
                g_arenaservers.pinglist[j as usize].start = uis.realtime;
                trap_Cmd_ExecuteText(
                    EXEC_NOW as libc::c_int,
                    va(
                        b"ping %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        adrstr.as_mut_ptr(),
                    ),
                );
                g_arenaservers.currentping += 1;
                i += 1
            }
        }
    }
    if 0 == trap_LAN_GetPingQueueCount() {
        ArenaServers_StopRefresh();
        return;
    }
    ArenaServers_UpdateMenu();
}
/*
=================
ArenaServers_SourceForLAN

Convert ui's g_servertype to AS_* used by trap calls.
=================
*/
#[no_mangle]
pub unsafe extern "C" fn ArenaServers_SourceForLAN() -> libc::c_int {
    match g_servertype {
        1 | 2 | 3 | 4 | 5 | 6 => return 2i32,
        7 => return 3i32,
        0 | _ => return 0i32,
    };
}
/*
=================
ArenaServers_Insert
=================
*/
unsafe extern "C" fn ArenaServers_Insert(
    mut adrstr: *mut libc::c_char,
    mut info: *mut libc::c_char,
    mut pingtime: libc::c_int,
) {
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if pingtime >= ArenaServers_MaxPing() && g_servertype != 7i32 {
        return;
    }
    if *g_arenaservers.numservers >= g_arenaservers.maxservers {
        servernodeptr = g_arenaservers
            .serverlist
            .offset(*g_arenaservers.numservers as isize)
            .offset(-1isize)
    } else {
        servernodeptr = g_arenaservers
            .serverlist
            .offset(*g_arenaservers.numservers as isize);
        *g_arenaservers.numservers += 1
    }
    Q_strncpyz((*servernodeptr).adrstr.as_mut_ptr(), adrstr, 64i32);
    Q_strncpyz(
        (*servernodeptr).hostname.as_mut_ptr(),
        Info_ValueForKey(info, b"hostname\x00" as *const u8 as *const libc::c_char),
        22i32,
    );
    Q_CleanStr((*servernodeptr).hostname.as_mut_ptr());
    Q_strupr((*servernodeptr).hostname.as_mut_ptr());
    Q_strncpyz(
        (*servernodeptr).mapname.as_mut_ptr(),
        Info_ValueForKey(info, b"mapname\x00" as *const u8 as *const libc::c_char),
        16i32,
    );
    Q_CleanStr((*servernodeptr).mapname.as_mut_ptr());
    Q_strupr((*servernodeptr).mapname.as_mut_ptr());
    (*servernodeptr).numclients = atoi(Info_ValueForKey(
        info,
        b"clients\x00" as *const u8 as *const libc::c_char,
    ));
    (*servernodeptr).maxclients = atoi(Info_ValueForKey(
        info,
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
    ));
    (*servernodeptr).pingtime = pingtime;
    (*servernodeptr).minPing = atoi(Info_ValueForKey(
        info,
        b"minPing\x00" as *const u8 as *const libc::c_char,
    ));
    (*servernodeptr).maxPing = atoi(Info_ValueForKey(
        info,
        b"maxPing\x00" as *const u8 as *const libc::c_char,
    ));
    (*servernodeptr).bPB = atoi(Info_ValueForKey(
        info,
        b"punkbuster\x00" as *const u8 as *const libc::c_char,
    )) as qboolean;
    (*servernodeptr).nettype = atoi(Info_ValueForKey(
        info,
        b"nettype\x00" as *const u8 as *const libc::c_char,
    ));
    if (*servernodeptr).nettype < 0i32
        || (*servernodeptr).nettype as libc::c_ulong
            >= (::std::mem::size_of::<[*mut libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
    {
        (*servernodeptr).nettype = 0i32
    }
    s = Info_ValueForKey(info, b"game\x00" as *const u8 as *const libc::c_char);
    i = atoi(Info_ValueForKey(
        info,
        b"gametype\x00" as *const u8 as *const libc::c_char,
    ));
    if i < 0i32 {
        i = 0i32
    } else if i > 11i32 {
        i = 12i32
    }
    if 0 != *s {
        (*servernodeptr).gametype = i;
        Q_strncpyz(
            (*servernodeptr).gamename.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as libc::c_int,
        );
    } else {
        (*servernodeptr).gametype = i;
        Q_strncpyz(
            (*servernodeptr).gamename.as_mut_ptr(),
            gamenames[i as usize],
            ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong as libc::c_int,
        );
    };
}
static mut gamenames: [*mut libc::c_char; 14] = [
    b"DM \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"1v1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"SP \x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Team DM\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"CTF\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"One Flag CTF\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"OverLoad\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Harvester\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Rocket Arena 3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Q3F\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Urban Terror\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"OSP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"???\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn ArenaServers_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/create_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/create_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/specify_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/specify_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/refresh_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/refresh_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/arrows_vert_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/arrows_vert_top\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/arrows_vert_bot\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/pblogo\x00" as *const u8 as *const libc::c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arenaservers_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub master: menulist_s,
    pub gametype: menulist_s,
    pub sortkey: menulist_s,
    pub showfull: menuradiobutton_s,
    pub showempty: menuradiobutton_s,
    pub list: menulist_s,
    pub mappic: menubitmap_s,
    pub arrows: menubitmap_s,
    pub up: menubitmap_s,
    pub down: menubitmap_s,
    pub status: menutext_s,
    pub statusbar: menutext_s,
    pub remove: menubitmap_s,
    pub back: menubitmap_s,
    pub refresh: menubitmap_s,
    pub specify: menubitmap_s,
    pub create: menubitmap_s,
    pub go: menubitmap_s,
    pub pinglist: [pinglist_t; 32],
    pub table: [table_t; 128],
    pub items: [*mut libc::c_char; 128],
    pub numqueriedservers: libc::c_int,
    pub numservers: *mut libc::c_int,
    pub serverlist: *mut servernode_t,
    pub currentping: libc::c_int,
    pub refreshservers: qboolean,
    pub nextpingtime: libc::c_int,
    pub maxservers: libc::c_int,
    pub refreshtime: libc::c_int,
    pub favoriteaddresses: [[libc::c_char; 64]; 16],
    pub numfavoriteaddresses: libc::c_int,
    pub punkbuster: menulist_s,
    pub pblogo: menubitmap_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinglist_t {
    pub adrstr: [libc::c_char; 64],
    pub start: libc::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct table_t {
    pub buff: [libc::c_char; 68],
    pub servernode: *mut servernode_t,
}
pub type servernode_t = servernode_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct servernode_s {
    pub adrstr: [libc::c_char; 64],
    pub hostname: [libc::c_char; 25],
    pub mapname: [libc::c_char; 16],
    pub numclients: libc::c_int,
    pub maxclients: libc::c_int,
    pub pingtime: libc::c_int,
    pub gametype: libc::c_int,
    pub gamename: [libc::c_char; 12],
    pub nettype: libc::c_int,
    pub minPing: libc::c_int,
    pub maxPing: libc::c_int,
    pub bPB: qboolean,
}
