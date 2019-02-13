use bg_misc::bg_itemlist;
use bg_public_h::{
    unnamed_0, unnamed_1, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED,
    TEAM_SPECTATOR,
};
use libc;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
    AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
    AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    connstate_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, vec4_t, vec_t,
    Info_ValueForKey, CA_ACTIVE, CA_AUTHORIZING, CA_CHALLENGING, CA_CINEMATIC, CA_CONNECTED,
    CA_CONNECTING, CA_DISCONNECTED, CA_LOADING, CA_PRIMED, CA_UNINITIALIZED, EXEC_APPEND,
    EXEC_INSERT, EXEC_NOW,
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
use ui_local_h::{
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menutext_s,
    trap_Cmd_ExecuteText, trap_Cvar_VariableValue, trap_GetClientState, trap_GetConfigString,
    trap_R_RegisterShaderNoMip, uiStatic_t,
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

unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10i32,
    ) as libc::c_int;
}
//
// ui_ingame.c
//
#[no_mangle]
pub unsafe extern "C" fn InGame_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_InGameMenu() {
    uis.menusp = 0i32;
    uis.cursorx = 319i32;
    uis.cursory = 80i32;
    InGame_MenuInit();
    UI_PushMenu(&mut s_ingame.menu);
}
static mut s_ingame: ingamemenu_t = ingamemenu_t {
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
    team: menutext_s {
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
    setup: menutext_s {
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
    server: menutext_s {
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
    leave: menutext_s {
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
    restart: menutext_s {
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
    addbots: menutext_s {
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
    removebots: menutext_s {
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
    teamorders: menutext_s {
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
    quit: menutext_s {
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
    resume: menutext_s {
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
};
/*
=================
InGame_MenuInit
=================
*/
#[no_mangle]
pub unsafe extern "C" fn InGame_MenuInit() {
    let mut y: libc::c_int = 0;
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
    memset(
        &mut s_ingame as *mut ingamemenu_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<ingamemenu_t>() as libc::c_ulong,
    );
    InGame_Cache();
    s_ingame.menu.wrapAround = qtrue;
    s_ingame.menu.fullscreen = qfalse;
    s_ingame.frame.generic.type_0 = 6i32;
    s_ingame.frame.generic.flags = 0x4000i32 as libc::c_uint;
    s_ingame.frame.generic.name = b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char;
    s_ingame.frame.generic.x = 320i32 - 233i32;
    s_ingame.frame.generic.y = 240i32 - 166i32;
    s_ingame.frame.width = 466i32;
    s_ingame.frame.height = 332i32;
    y = 88i32;
    s_ingame.team.generic.type_0 = 9i32;
    s_ingame.team.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_ingame.team.generic.x = 320i32;
    s_ingame.team.generic.y = y;
    s_ingame.team.generic.id = 10i32;
    s_ingame.team.generic.callback = Some(InGame_Event);
    s_ingame.team.string = b"START\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.team.color = color_red.as_mut_ptr();
    s_ingame.team.style = 0x1i32 | 0x10i32;
    y += 28i32;
    s_ingame.addbots.generic.type_0 = 9i32;
    s_ingame.addbots.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_ingame.addbots.generic.x = 320i32;
    s_ingame.addbots.generic.y = y;
    s_ingame.addbots.generic.id = 11i32;
    s_ingame.addbots.generic.callback = Some(InGame_Event);
    s_ingame.addbots.string =
        b"ADD BOTS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.addbots.color = color_red.as_mut_ptr();
    s_ingame.addbots.style = 0x1i32 | 0x10i32;
    if 0. == trap_Cvar_VariableValue(b"sv_running\x00" as *const u8 as *const libc::c_char)
        || 0. == trap_Cvar_VariableValue(b"bot_enable\x00" as *const u8 as *const libc::c_char)
        || trap_Cvar_VariableValue(b"g_gametype\x00" as *const u8 as *const libc::c_char)
            == GT_SINGLE_PLAYER as libc::c_int as libc::c_float
    {
        s_ingame.addbots.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 28i32;
    s_ingame.removebots.generic.type_0 = 9i32;
    s_ingame.removebots.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_ingame.removebots.generic.x = 320i32;
    s_ingame.removebots.generic.y = y;
    s_ingame.removebots.generic.id = 12i32;
    s_ingame.removebots.generic.callback = Some(InGame_Event);
    s_ingame.removebots.string =
        b"REMOVE BOTS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.removebots.color = color_red.as_mut_ptr();
    s_ingame.removebots.style = 0x1i32 | 0x10i32;
    if 0. == trap_Cvar_VariableValue(b"sv_running\x00" as *const u8 as *const libc::c_char)
        || 0. == trap_Cvar_VariableValue(b"bot_enable\x00" as *const u8 as *const libc::c_char)
        || trap_Cvar_VariableValue(b"g_gametype\x00" as *const u8 as *const libc::c_char)
            == GT_SINGLE_PLAYER as libc::c_int as libc::c_float
    {
        s_ingame.removebots.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 28i32;
    s_ingame.teamorders.generic.type_0 = 9i32;
    s_ingame.teamorders.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_ingame.teamorders.generic.x = 320i32;
    s_ingame.teamorders.generic.y = y;
    s_ingame.teamorders.generic.id = 19i32;
    s_ingame.teamorders.generic.callback = Some(InGame_Event);
    s_ingame.teamorders.string =
        b"TEAM ORDERS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.teamorders.color = color_red.as_mut_ptr();
    s_ingame.teamorders.style = 0x1i32 | 0x10i32;
    if !(trap_Cvar_VariableValue(b"g_gametype\x00" as *const u8 as *const libc::c_char)
        >= GT_TEAM as libc::c_int as libc::c_float)
    {
        s_ingame.teamorders.generic.flags |= 0x2000i32 as libc::c_uint
    } else {
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
            s_ingame.teamorders.generic.flags |= 0x2000i32 as libc::c_uint
        }
    }
    y += 28i32;
    s_ingame.setup.generic.type_0 = 9i32;
    s_ingame.setup.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_ingame.setup.generic.x = 320i32;
    s_ingame.setup.generic.y = y;
    s_ingame.setup.generic.id = 13i32;
    s_ingame.setup.generic.callback = Some(InGame_Event);
    s_ingame.setup.string = b"SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.setup.color = color_red.as_mut_ptr();
    s_ingame.setup.style = 0x1i32 | 0x10i32;
    y += 28i32;
    s_ingame.server.generic.type_0 = 9i32;
    s_ingame.server.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_ingame.server.generic.x = 320i32;
    s_ingame.server.generic.y = y;
    s_ingame.server.generic.id = 14i32;
    s_ingame.server.generic.callback = Some(InGame_Event);
    s_ingame.server.string =
        b"SERVER INFO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.server.color = color_red.as_mut_ptr();
    s_ingame.server.style = 0x1i32 | 0x10i32;
    y += 28i32;
    s_ingame.restart.generic.type_0 = 9i32;
    s_ingame.restart.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_ingame.restart.generic.x = 320i32;
    s_ingame.restart.generic.y = y;
    s_ingame.restart.generic.id = 16i32;
    s_ingame.restart.generic.callback = Some(InGame_Event);
    s_ingame.restart.string =
        b"RESTART ARENA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.restart.color = color_red.as_mut_ptr();
    s_ingame.restart.style = 0x1i32 | 0x10i32;
    if 0. == trap_Cvar_VariableValue(b"sv_running\x00" as *const u8 as *const libc::c_char) {
        s_ingame.restart.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 28i32;
    s_ingame.resume.generic.type_0 = 9i32;
    s_ingame.resume.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_ingame.resume.generic.x = 320i32;
    s_ingame.resume.generic.y = y;
    s_ingame.resume.generic.id = 18i32;
    s_ingame.resume.generic.callback = Some(InGame_Event);
    s_ingame.resume.string =
        b"RESUME GAME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.resume.color = color_red.as_mut_ptr();
    s_ingame.resume.style = 0x1i32 | 0x10i32;
    y += 28i32;
    s_ingame.leave.generic.type_0 = 9i32;
    s_ingame.leave.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_ingame.leave.generic.x = 320i32;
    s_ingame.leave.generic.y = y;
    s_ingame.leave.generic.id = 15i32;
    s_ingame.leave.generic.callback = Some(InGame_Event);
    s_ingame.leave.string =
        b"LEAVE ARENA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.leave.color = color_red.as_mut_ptr();
    s_ingame.leave.style = 0x1i32 | 0x10i32;
    y += 28i32;
    s_ingame.quit.generic.type_0 = 9i32;
    s_ingame.quit.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_ingame.quit.generic.x = 320i32;
    s_ingame.quit.generic.y = y;
    s_ingame.quit.generic.id = 17i32;
    s_ingame.quit.generic.callback = Some(InGame_Event);
    s_ingame.quit.string =
        b"EXIT GAME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_ingame.quit.color = color_red.as_mut_ptr();
    s_ingame.quit.style = 0x1i32 | 0x10i32;
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.frame as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.team as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.addbots as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.removebots as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.teamorders as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.setup as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.server as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.restart as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.resume as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.leave as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_ingame.menu,
        &mut s_ingame.quit as *mut menutext_s as *mut libc::c_void,
    );
}
/*
=================
InGame_Event
=================
*/
#[no_mangle]
pub unsafe extern "C" fn InGame_Event(mut ptr: *mut libc::c_void, mut notification: libc::c_int) {
    if notification != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        10 => {
            UI_TeamMainMenu();
        }
        13 => {
            UI_SetupMenu();
        }
        15 => {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                b"disconnect\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        16 => {
            UI_ConfirmMenu(
                b"RESTART ARENA?\x00" as *const u8 as *const libc::c_char,
                None,
                Some(InGame_RestartAction),
            );
        }
        17 => {
            UI_ConfirmMenu(
                b"EXIT GAME?\x00" as *const u8 as *const libc::c_char,
                None,
                Some(InGame_QuitAction),
            );
        }
        14 => {
            UI_ServerInfoMenu();
        }
        11 => {
            UI_AddBotsMenu();
        }
        12 => {
            UI_RemoveBotsMenu();
        }
        19 => {
            UI_TeamOrdersMenu();
        }
        18 => {
            UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
InGame_QuitAction
=================
*/
unsafe extern "C" fn InGame_QuitAction(mut result: qboolean) {
    if 0 == result as u64 {
        return;
    }
    UI_PopMenu();
    UI_CreditMenu();
}
/*
=================
InGame_RestartAction
=================
*/
unsafe extern "C" fn InGame_RestartAction(mut result: qboolean) {
    if 0 == result as u64 {
        return;
    }
    UI_PopMenu();
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        b"map_restart 0\n\x00" as *const u8 as *const libc::c_char,
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingamemenu_t {
    pub menu: menuframework_s,
    pub frame: menubitmap_s,
    pub team: menutext_s,
    pub setup: menutext_s,
    pub server: menutext_s,
    pub leave: menutext_s,
    pub restart: menutext_s,
    pub addbots: menutext_s,
    pub removebots: menutext_s,
    pub teamorders: menutext_s,
    pub quit: menutext_s,
    pub resume: menutext_s,
}
