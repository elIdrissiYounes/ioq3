use bg_misc::bg_itemlist;
use bg_public_h::{
    unnamed_0, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT,
};
use libc;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
    AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
    AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, vec4_t, vec_t, Info_ValueForKey,
    EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::{memset, strtol};
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
    trap_Cmd_ExecuteText, trap_GetConfigString, trap_R_RegisterShaderNoMip,
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
// ui_team.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_TeamMainMenu() {
    TeamMain_MenuInit();
    UI_PushMenu(&mut s_teammain.menu);
}
static mut s_teammain: teammain_t = teammain_t {
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
    joinred: menutext_s {
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
    joinblue: menutext_s {
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
    joingame: menutext_s {
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
    spectate: menutext_s {
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
===============
TeamMain_MenuInit
===============
*/
#[no_mangle]
pub unsafe extern "C" fn TeamMain_MenuInit() {
    let mut y: libc::c_int = 0;
    let mut gametype: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    memset(
        &mut s_teammain as *mut teammain_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<teammain_t>() as libc::c_ulong,
    );
    TeamMain_Cache();
    s_teammain.menu.wrapAround = qtrue;
    s_teammain.menu.fullscreen = qfalse;
    s_teammain.frame.generic.type_0 = 6i32;
    s_teammain.frame.generic.flags = 0x4000i32 as libc::c_uint;
    s_teammain.frame.generic.name = b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char;
    s_teammain.frame.generic.x = 142i32;
    s_teammain.frame.generic.y = 118i32;
    s_teammain.frame.width = 359i32;
    s_teammain.frame.height = 256i32;
    y = 194i32;
    s_teammain.joinred.generic.type_0 = 9i32;
    s_teammain.joinred.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_teammain.joinred.generic.id = 100i32;
    s_teammain.joinred.generic.callback = Some(TeamMain_MenuEvent);
    s_teammain.joinred.generic.x = 320i32;
    s_teammain.joinred.generic.y = y;
    s_teammain.joinred.string =
        b"JOIN RED\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_teammain.joinred.style = 0x1i32 | 0x10i32;
    s_teammain.joinred.color = colorRed.as_mut_ptr();
    y += 20i32;
    s_teammain.joinblue.generic.type_0 = 9i32;
    s_teammain.joinblue.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_teammain.joinblue.generic.id = 101i32;
    s_teammain.joinblue.generic.callback = Some(TeamMain_MenuEvent);
    s_teammain.joinblue.generic.x = 320i32;
    s_teammain.joinblue.generic.y = y;
    s_teammain.joinblue.string =
        b"JOIN BLUE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_teammain.joinblue.style = 0x1i32 | 0x10i32;
    s_teammain.joinblue.color = colorRed.as_mut_ptr();
    y += 20i32;
    s_teammain.joingame.generic.type_0 = 9i32;
    s_teammain.joingame.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_teammain.joingame.generic.id = 102i32;
    s_teammain.joingame.generic.callback = Some(TeamMain_MenuEvent);
    s_teammain.joingame.generic.x = 320i32;
    s_teammain.joingame.generic.y = y;
    s_teammain.joingame.string =
        b"JOIN GAME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_teammain.joingame.style = 0x1i32 | 0x10i32;
    s_teammain.joingame.color = colorRed.as_mut_ptr();
    y += 20i32;
    s_teammain.spectate.generic.type_0 = 9i32;
    s_teammain.spectate.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_teammain.spectate.generic.id = 103i32;
    s_teammain.spectate.generic.callback = Some(TeamMain_MenuEvent);
    s_teammain.spectate.generic.x = 320i32;
    s_teammain.spectate.generic.y = y;
    s_teammain.spectate.string =
        b"SPECTATE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_teammain.spectate.style = 0x1i32 | 0x10i32;
    s_teammain.spectate.color = colorRed.as_mut_ptr();
    trap_GetConfigString(0i32, info.as_mut_ptr(), 1024i32);
    gametype = atoi(Info_ValueForKey(
        info.as_mut_ptr(),
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
    ));
    match gametype {
        2 | 0 | 1 => {
            s_teammain.joinred.generic.flags |= 0x2000i32 as libc::c_uint;
            s_teammain.joinblue.generic.flags |= 0x2000i32 as libc::c_uint
        }
        3 | 4 | _ => s_teammain.joingame.generic.flags |= 0x2000i32 as libc::c_uint,
    }
    Menu_AddItem(
        &mut s_teammain.menu,
        &mut s_teammain.frame as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_teammain.menu,
        &mut s_teammain.joinred as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_teammain.menu,
        &mut s_teammain.joinblue as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_teammain.menu,
        &mut s_teammain.joingame as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_teammain.menu,
        &mut s_teammain.spectate as *mut menutext_s as *mut libc::c_void,
    );
}
/*
===============
TeamMain_MenuEvent
===============
*/
unsafe extern "C" fn TeamMain_MenuEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        100 => {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                b"cmd team red\n\x00" as *const u8 as *const libc::c_char,
            );
            UI_ForceMenuOff();
        }
        101 => {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                b"cmd team blue\n\x00" as *const u8 as *const libc::c_char,
            );
            UI_ForceMenuOff();
        }
        102 => {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                b"cmd team free\n\x00" as *const u8 as *const libc::c_char,
            );
            UI_ForceMenuOff();
        }
        103 => {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                b"cmd team spectator\n\x00" as *const u8 as *const libc::c_char,
            );
            UI_ForceMenuOff();
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn TeamMain_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct teamOrdersMenuInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub frame: menubitmap_s,
    pub list: menulist_s,
    pub back: menubitmap_s,
    pub gametype: libc::c_int,
    pub numBots: libc::c_int,
    pub selectedBot: libc::c_int,
    pub bots: [*mut libc::c_char; 9],
    pub botNames: [[libc::c_char; 16]; 9],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct teammain_t {
    pub menu: menuframework_s,
    pub frame: menubitmap_s,
    pub joinred: menutext_s,
    pub joinblue: menutext_s,
    pub joingame: menutext_s,
    pub spectate: menutext_s,
}
