use bg_misc::bg_itemlist;
use bg_public_h::{
    unnamed_1, unnamed_2, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED,
    TEAM_SPECTATOR,
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
use libc;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
    AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
    AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    connstate_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t,
    Com_sprintf, Info_ValueForKey, Q_CleanStr, Q_strncpyz, CA_ACTIVE, CA_AUTHORIZING,
    CA_CHALLENGING, CA_CINEMATIC, CA_CONNECTED, CA_CONNECTING, CA_DISCONNECTED, CA_LOADING,
    CA_PRIMED, CA_UNINITIALIZED, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::{memset, strtol};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menulist_s, menutext_s,
    trap_Cmd_ExecuteText, trap_GetClientState, trap_GetConfigString, trap_R_RegisterShaderNoMip,
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
use ui_team::{teamOrdersMenuInfo_t, TeamMain_Cache, UI_TeamMainMenu};
use ui_video::{DriverInfo_Cache, GraphicsOptions_Cache, UI_GraphicsOptionsMenu};

unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10i32,
    ) as libc::c_int;
}
//
// ui_teamorders.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_TeamOrdersMenu() {
    UI_TeamOrdersMenu_Init();
    UI_PushMenu(&mut teamOrdersMenuInfo.menu);
}
static mut teamOrdersMenuInfo: teamOrdersMenuInfo_t = teamOrdersMenuInfo_t {
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
    frame: menubitmap_s {
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
    gametype: 0,
    numBots: 0,
    selectedBot: 0,
    bots: [0 as *const libc::c_char as *mut libc::c_char; 9],
    botNames: [[0; 16]; 9],
};
/*
===============
UI_TeamOrdersMenu_Init
===============
*/
unsafe extern "C" fn UI_TeamOrdersMenu_Init() {
    UI_TeamOrdersMenu_Cache();
    memset(
        &mut teamOrdersMenuInfo as *mut teamOrdersMenuInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<teamOrdersMenuInfo_t>() as libc::c_ulong,
    );
    teamOrdersMenuInfo.menu.fullscreen = qfalse;
    teamOrdersMenuInfo.menu.key = Some(UI_TeamOrdersMenu_Key);
    UI_TeamOrdersMenu_BuildBotList();
    teamOrdersMenuInfo.banner.generic.type_0 = 10i32;
    teamOrdersMenuInfo.banner.generic.x = 320i32;
    teamOrdersMenuInfo.banner.generic.y = 16i32;
    teamOrdersMenuInfo.banner.string =
        b"TEAM ORDERS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    teamOrdersMenuInfo.banner.color = color_white.as_mut_ptr();
    teamOrdersMenuInfo.banner.style = 0x1i32;
    teamOrdersMenuInfo.frame.generic.type_0 = 6i32;
    teamOrdersMenuInfo.frame.generic.flags = 0x4000i32 as libc::c_uint;
    teamOrdersMenuInfo.frame.generic.name =
        b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char;
    teamOrdersMenuInfo.frame.generic.x = 320i32 - 233i32;
    teamOrdersMenuInfo.frame.generic.y = 240i32 - 166i32;
    teamOrdersMenuInfo.frame.width = 466i32;
    teamOrdersMenuInfo.frame.height = 332i32;
    teamOrdersMenuInfo.list.generic.type_0 = 8i32;
    teamOrdersMenuInfo.list.generic.flags = 0x100i32 as libc::c_uint;
    teamOrdersMenuInfo.list.generic.ownerdraw = Some(UI_TeamOrdersMenu_ListDraw);
    teamOrdersMenuInfo.list.generic.callback = Some(UI_TeamOrdersMenu_ListEvent);
    teamOrdersMenuInfo.list.generic.x = 320i32 - 64i32;
    teamOrdersMenuInfo.list.generic.y = 120i32;
    teamOrdersMenuInfo.back.generic.type_0 = 6i32;
    teamOrdersMenuInfo.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    teamOrdersMenuInfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    teamOrdersMenuInfo.back.generic.callback = Some(UI_TeamOrdersMenu_BackEvent);
    teamOrdersMenuInfo.back.generic.x = 0i32;
    teamOrdersMenuInfo.back.generic.y = 480i32 - 64i32;
    teamOrdersMenuInfo.back.width = 128i32;
    teamOrdersMenuInfo.back.height = 64i32;
    teamOrdersMenuInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut teamOrdersMenuInfo.menu,
        &mut teamOrdersMenuInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut teamOrdersMenuInfo.menu,
        &mut teamOrdersMenuInfo.frame as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut teamOrdersMenuInfo.menu,
        &mut teamOrdersMenuInfo.list as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut teamOrdersMenuInfo.menu,
        &mut teamOrdersMenuInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
    teamOrdersMenuInfo.list.generic.left = 220i32;
    teamOrdersMenuInfo.list.generic.top = teamOrdersMenuInfo.list.generic.y;
    teamOrdersMenuInfo.list.generic.right = 420i32;
    UI_TeamOrdersMenu_SetList(10i32);
}
/*
===============
UI_TeamOrdersMenu_SetList
===============
*/
unsafe extern "C" fn UI_TeamOrdersMenu_SetList(mut id: libc::c_int) {
    match id {
        11 => {
            teamOrdersMenuInfo.list.generic.id = id;
            teamOrdersMenuInfo.list.numitems = 7i32;
            teamOrdersMenuInfo.list.itemnames = ctfOrders.as_mut_ptr()
        }
        12 => {
            teamOrdersMenuInfo.list.generic.id = id;
            teamOrdersMenuInfo.list.numitems = 6i32;
            teamOrdersMenuInfo.list.itemnames = teamOrders.as_mut_ptr()
        }
        10 | _ => {
            teamOrdersMenuInfo.list.generic.id = id;
            teamOrdersMenuInfo.list.numitems = teamOrdersMenuInfo.numBots;
            teamOrdersMenuInfo.list.itemnames =
                teamOrdersMenuInfo.bots.as_mut_ptr() as *mut *const libc::c_char
        }
    }
    teamOrdersMenuInfo.list.generic.bottom =
        teamOrdersMenuInfo.list.generic.top + teamOrdersMenuInfo.list.numitems * 27i32;
}
static mut teamOrders: [*const libc::c_char; 7] = [
    b"I Am the Leader\x00" as *const u8 as *const libc::c_char,
    b"Follow Me\x00" as *const u8 as *const libc::c_char,
    b"Roam\x00" as *const u8 as *const libc::c_char,
    b"Camp Here\x00" as *const u8 as *const libc::c_char,
    b"Report\x00" as *const u8 as *const libc::c_char,
    b"I Relinquish Command\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut ctfOrders: [*const libc::c_char; 8] = [
    b"I Am the Leader\x00" as *const u8 as *const libc::c_char,
    b"Defend the Base\x00" as *const u8 as *const libc::c_char,
    b"Follow Me\x00" as *const u8 as *const libc::c_char,
    b"Get Enemy Flag\x00" as *const u8 as *const libc::c_char,
    b"Camp Here\x00" as *const u8 as *const libc::c_char,
    b"Report\x00" as *const u8 as *const libc::c_char,
    b"I Relinquish Command\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
===============
UI_TeamOrdersMenu_BackEvent
===============
*/
unsafe extern "C" fn UI_TeamOrdersMenu_BackEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    UI_PopMenu();
}
/*
===============
UI_TeamOrdersMenu_ListEvent
===============
*/
unsafe extern "C" fn UI_TeamOrdersMenu_ListEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    let mut id: libc::c_int = 0;
    let mut selection: libc::c_int = 0;
    let mut message: [libc::c_char; 256] = [0; 256];
    if event != 3i32 {
        return;
    }
    id = (*(ptr as *mut menulist_s)).generic.id;
    selection = (*(ptr as *mut menulist_s)).curvalue;
    if id == 10i32 {
        teamOrdersMenuInfo.selectedBot = selection;
        if teamOrdersMenuInfo.gametype == GT_CTF as libc::c_int {
            UI_TeamOrdersMenu_SetList(11i32);
        } else {
            UI_TeamOrdersMenu_SetList(12i32);
        }
        return;
    }
    if id == 11i32 {
        Com_sprintf(
            message.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            ctfMessages[selection as usize],
            teamOrdersMenuInfo.botNames[teamOrdersMenuInfo.selectedBot as usize].as_mut_ptr(),
        );
    } else {
        Com_sprintf(
            message.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
            teamMessages[selection as usize],
            teamOrdersMenuInfo.botNames[teamOrdersMenuInfo.selectedBot as usize].as_mut_ptr(),
        );
    }
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        va(
            b"say_team \"%s\"\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            message.as_mut_ptr(),
        ),
    );
    UI_PopMenu();
}
static mut teamMessages: [*const libc::c_char; 7] = [
    b"i am the leader\x00" as *const u8 as *const libc::c_char,
    b"%s follow me\x00" as *const u8 as *const libc::c_char,
    b"%s roam\x00" as *const u8 as *const libc::c_char,
    b"%s camp here\x00" as *const u8 as *const libc::c_char,
    b"%s report\x00" as *const u8 as *const libc::c_char,
    b"i stop being the leader\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut ctfMessages: [*const libc::c_char; 8] = [
    b"i am the leader\x00" as *const u8 as *const libc::c_char,
    b"%s defend the base\x00" as *const u8 as *const libc::c_char,
    b"%s follow me\x00" as *const u8 as *const libc::c_char,
    b"%s get enemy flag\x00" as *const u8 as *const libc::c_char,
    b"%s camp here\x00" as *const u8 as *const libc::c_char,
    b"%s report\x00" as *const u8 as *const libc::c_char,
    b"i stop being the leader\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
UI_TeamOrdersMenu_ListDraw
=================
*/
unsafe extern "C" fn UI_TeamOrdersMenu_ListDraw(mut self_0: *mut libc::c_void) {
    let mut l: *mut menulist_s = 0 as *mut menulist_s;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut hasfocus: qboolean = qfalse;
    let mut style: libc::c_int = 0;
    l = self_0 as *mut menulist_s;
    hasfocus =
        ((*(*l).generic.parent).cursor == (*l).generic.menuPosition) as libc::c_int as qboolean;
    x = 320i32;
    y = (*l).generic.y;
    i = 0i32;
    while i < (*l).numitems {
        style = 0i32 | 0x10i32 | 0x1i32;
        if i == (*l).curvalue {
            color = color_yellow.as_mut_ptr();
            if 0 != hasfocus as u64 {
                style |= 0x4000i32
            }
        } else {
            color = color_orange.as_mut_ptr()
        }
        UI_DrawProportionalString(x, y, *(*l).itemnames.offset(i as isize), style, color);
        y += 27i32;
        i += 1
    }
}
/*
===============
UI_TeamOrdersMenu_BuildBotList
===============
*/
unsafe extern "C" fn UI_TeamOrdersMenu_BuildBotList() {
    let mut cs: uiClientState_t = uiClientState_t {
        connState: CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    let mut numPlayers: libc::c_int = 0;
    let mut isBot: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut playerTeam: libc::c_char = '3' as i32 as libc::c_char;
    let mut botTeam: libc::c_char = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    n = 0i32;
    while n < 9i32 {
        teamOrdersMenuInfo.bots[n as usize] = teamOrdersMenuInfo.botNames[n as usize].as_mut_ptr();
        n += 1
    }
    trap_GetClientState(&mut cs);
    Q_strncpyz(
        teamOrdersMenuInfo.botNames[0usize].as_mut_ptr(),
        b"Everyone\x00" as *const u8 as *const libc::c_char,
        16i32,
    );
    teamOrdersMenuInfo.numBots = 1i32;
    trap_GetConfigString(
        0i32,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    numPlayers = atoi(Info_ValueForKey(
        info.as_mut_ptr(),
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
    ));
    teamOrdersMenuInfo.gametype = atoi(Info_ValueForKey(
        info.as_mut_ptr(),
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
    ));
    n = 0i32;
    while n < numPlayers && teamOrdersMenuInfo.numBots < 9i32 {
        trap_GetConfigString(32i32 + 256i32 + 256i32 + n, info.as_mut_ptr(), 1024i32);
        if n == cs.clientNum {
            playerTeam = *Info_ValueForKey(
                info.as_mut_ptr(),
                b"t\x00" as *const u8 as *const libc::c_char,
            )
        } else {
            isBot = atoi(Info_ValueForKey(
                info.as_mut_ptr(),
                b"skill\x00" as *const u8 as *const libc::c_char,
            ));
            if !(0 == isBot) {
                botTeam = *Info_ValueForKey(
                    info.as_mut_ptr(),
                    b"t\x00" as *const u8 as *const libc::c_char,
                );
                if !(botTeam as libc::c_int != playerTeam as libc::c_int) {
                    Q_strncpyz(
                        teamOrdersMenuInfo.botNames[teamOrdersMenuInfo.numBots as usize]
                            .as_mut_ptr(),
                        Info_ValueForKey(
                            info.as_mut_ptr(),
                            b"n\x00" as *const u8 as *const libc::c_char,
                        ),
                        16i32,
                    );
                    Q_CleanStr(
                        teamOrdersMenuInfo.botNames[teamOrdersMenuInfo.numBots as usize]
                            .as_mut_ptr(),
                    );
                    teamOrdersMenuInfo.numBots += 1
                }
            }
        }
        n += 1
    }
}
/*
=================
UI_TeamOrdersMenu_Key
=================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_TeamOrdersMenu_Key(mut key: libc::c_int) -> sfxHandle_t {
    let mut l: *mut menulist_s = 0 as *mut menulist_s;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    l = Menu_ItemAtCursor(&mut teamOrdersMenuInfo.menu) as *mut menulist_s;
    if l != &mut teamOrdersMenuInfo.list as *mut menulist_s {
        return Menu_DefaultKey(&mut teamOrdersMenuInfo.menu, key);
    }
    match key {
        178 => {
            x = (*l).generic.left;
            y = (*l).generic.top;
            if 0 != UI_CursorInRect(x, y, (*l).generic.right - x, (*l).generic.bottom - y) as u64 {
                index = (uis.cursory - y) / 27i32;
                (*l).oldvalue = (*l).curvalue;
                (*l).curvalue = index;
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        3i32,
                    );
                    return menu_move_sound;
                }
            }
            return menu_null_sound;
        }
        161 | 132 => {
            (*l).oldvalue = (*l).curvalue;
            if (*l).curvalue == 0i32 {
                (*l).curvalue = (*l).numitems - 1i32
            } else {
                (*l).curvalue -= 1
            }
            return menu_move_sound;
        }
        167 | 133 => {
            (*l).oldvalue = (*l).curvalue;
            if (*l).curvalue == (*l).numitems - 1i32 {
                (*l).curvalue = 0i32
            } else {
                (*l).curvalue += 1
            }
            return menu_move_sound;
        }
        _ => {}
    }
    return Menu_DefaultKey(&mut teamOrdersMenuInfo.menu, key);
}
#[no_mangle]
pub unsafe extern "C" fn UI_TeamOrdersMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_TeamOrdersMenu_f() {
    let mut cs: uiClientState_t = uiClientState_t {
        connState: CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    let mut info: [libc::c_char; 1024] = [0; 1024];
    let mut team: libc::c_int = 0;
    trap_GetConfigString(
        0i32,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    teamOrdersMenuInfo.gametype = atoi(Info_ValueForKey(
        info.as_mut_ptr(),
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
    ));
    if teamOrdersMenuInfo.gametype < GT_TEAM as libc::c_int {
        return;
    }
    trap_GetClientState(&mut cs);
    trap_GetConfigString(
        32i32 + 256i32 + 256i32 + cs.clientNum,
        info.as_mut_ptr(),
        1024i32,
    );
    team = atoi(Info_ValueForKey(
        info.as_mut_ptr(),
        b"t\x00" as *const u8 as *const libc::c_char,
    ));
    if team == TEAM_SPECTATOR as libc::c_int {
        return;
    }
    UI_TeamOrdersMenu();
}
