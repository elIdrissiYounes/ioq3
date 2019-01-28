use libc;
use q_shared_h::{cvarHandle_t, qboolean, qfalse, qtrue, vmCvar_t};
use stdlib::intptr_t;
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
use ui_local_h::{trap_Cvar_Register, trap_Cvar_Update};
use ui_menu::{MainMenu_Cache, UI_MainMenu};
use ui_mfield::{MField_Draw, MenuField_Draw, MenuField_Init, MenuField_Key};
use ui_mods::{UI_ModsMenu, UI_ModsMenu_Cache};
use ui_network::{UI_NetworkOptionsMenu, UI_NetworkOptionsMenu_Cache};
use ui_playermodel::{PlayerModel_Cache, UI_PlayerModelMenu};
use ui_players::{UI_DrawPlayer, UI_PlayerInfo_SetInfo, UI_PlayerInfo_SetModel};
use ui_playersettings::{PlayerSettings_Cache, UI_PlayerSettingsMenu};
use ui_preferences::{Preferences_Cache, UI_PreferencesMenu};
use ui_public_h::{
    uiMenuCommand_t, unnamed, UIMENU_BAD_CD_KEY, UIMENU_INGAME, UIMENU_MAIN, UIMENU_NEED_CD,
    UIMENU_NONE, UIMENU_POSTGAME, UIMENU_TEAM, UI_CONSOLE_COMMAND, UI_DRAW_CONNECT_SCREEN,
    UI_GETAPIVERSION, UI_HASUNIQUECDKEY, UI_INIT, UI_IS_FULLSCREEN, UI_KEY_EVENT, UI_MOUSE_EVENT,
    UI_REFRESH, UI_SET_ACTIVE_MENU, UI_SHUTDOWN,
};
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

#[no_mangle]
pub static mut ui_ffa_fraglimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_ffa_timelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_tourney_fraglimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_tourney_timelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_team_fraglimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_team_timelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_team_friendly: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_ctf_capturelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_ctf_timelimit: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_ctf_friendly: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_arenasFile: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_botsFile: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_spScores1: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_spScores2: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_spScores3: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_spScores4: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_spScores5: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_spAwards: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_spVideos: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_spSkill: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_spSelection: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_browserMaster: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_browserGameType: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_browserSortKey: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_browserShowFull: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_browserShowEmpty: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_brassTime: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_drawCrosshair: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_drawCrosshairNames: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_marks: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server1: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server2: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server3: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server4: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server5: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server6: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server7: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server8: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server9: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server10: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server11: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server12: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server13: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server14: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server15: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_server16: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_cdkeychecked: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub static mut ui_ioq3: vmCvar_t = vmCvar_t {
    handle: 0,
    modificationCount: 0,
    value: 0.,
    integer: 0,
    string: [0; 256],
};
#[no_mangle]
pub unsafe extern "C" fn UI_RegisterCvars() {
    let mut i: libc::c_int = 0;
    let mut cv: *mut cvarTable_t = 0 as *mut cvarTable_t;
    i = 0i32;
    cv = cvarTable.as_mut_ptr();
    while i < cvarTableSize {
        trap_Cvar_Register(
            (*cv).vmCvar,
            (*cv).cvarName,
            (*cv).defaultString,
            (*cv).cvarFlags,
        );
        i += 1;
        cv = cv.offset(1isize)
    }
}
// Initialized in run_static_initializers
static mut cvarTableSize: libc::c_int = 0;
static mut cvarTable: [cvarTable_t; 49] = unsafe {
    [
        cvarTable_t {
            vmCvar: &ui_ffa_fraglimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_ffa_fraglimit\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"20\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_ffa_timelimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_ffa_timelimit\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_tourney_fraglimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_tourney_fraglimit\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_tourney_timelimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_tourney_timelimit\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"15\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_team_fraglimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_team_fraglimit\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_team_timelimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_team_timelimit\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"20\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_team_friendly as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_team_friendly\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_ctf_capturelimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_ctf_capturelimit\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_ctf_timelimit as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_ctf_timelimit\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"30\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_ctf_friendly as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_ctf_friendly\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_arenasFile as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_arenasFile\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x10i32 | 0x40i32,
        },
        cvarTable_t {
            vmCvar: &ui_botsFile as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_botsFile\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x10i32 | 0x40i32,
        },
        cvarTable_t {
            vmCvar: &ui_spScores1 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_spScores1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_spScores2 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_spScores2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_spScores3 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_spScores3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_spScores4 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_spScores4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_spScores5 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_spScores5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_spAwards as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_spAwards\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_spVideos as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_spVideos\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_spSkill as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_spSkill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32 | 0x20i32,
        },
        cvarTable_t {
            vmCvar: &ui_spSelection as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_spSelection\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x40i32,
        },
        cvarTable_t {
            vmCvar: &ui_browserMaster as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_browserMaster\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_browserGameType as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_browserGameType\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_browserSortKey as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_browserSortKey\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_browserShowFull as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_browserShowFull\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_browserShowEmpty as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_browserShowEmpty\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_brassTime as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_brassTime\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"2500\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_drawCrosshair as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawCrosshair\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_drawCrosshairNames as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_drawCrosshairNames\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_marks as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"cg_marks\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server1 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server2 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server3 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server4 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server5 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server6 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server7 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server8 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server9 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server10 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server10\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server11 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server11\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server12 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server12\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server13 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server13\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server14 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server14\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server15 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server15\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_server16 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"server16\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x1i32,
        },
        cvarTable_t {
            vmCvar: &ui_cdkeychecked as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_cdkeychecked\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x40i32,
        },
        cvarTable_t {
            vmCvar: &ui_ioq3 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"ui_ioq3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            defaultString: b"1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0x40i32,
        },
        cvarTable_t {
            vmCvar: 0 as *const vmCvar_t as *mut vmCvar_t,
            cvarName: b"g_localTeamPref\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            defaultString: b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cvarFlags: 0i32,
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn UI_UpdateCvars() {
    let mut i: libc::c_int = 0;
    let mut cv: *mut cvarTable_t = 0 as *mut cvarTable_t;
    i = 0i32;
    cv = cvarTable.as_mut_ptr();
    while i < cvarTableSize {
        if !(*cv).vmCvar.is_null() {
            trap_Cvar_Update((*cv).vmCvar);
        }
        i += 1;
        cv = cv.offset(1isize)
    }
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

USER INTERFACE MAIN

=======================================================================
*/
/*
================
vmMain

This is the only way control passes into the module.
This must be the very first function compiled into the .qvm file
================
*/
#[no_mangle]
pub unsafe extern "C" fn vmMain(
    mut command: libc::c_int,
    mut arg0: libc::c_int,
    mut arg1: libc::c_int,
    mut arg2: libc::c_int,
    mut arg3: libc::c_int,
    mut arg4: libc::c_int,
    mut arg5: libc::c_int,
    mut arg6: libc::c_int,
    mut arg7: libc::c_int,
    mut arg8: libc::c_int,
    mut arg9: libc::c_int,
    mut arg10: libc::c_int,
    mut arg11: libc::c_int,
) -> intptr_t {
    match command {
        0 => return 4i32 as intptr_t,
        1 => {
            UI_Init();
            return 0i32 as intptr_t;
        }
        2 => {
            UI_Shutdown();
            return 0i32 as intptr_t;
        }
        3 => {
            UI_KeyEvent(arg0, arg1);
            return 0i32 as intptr_t;
        }
        4 => {
            UI_MouseEvent(arg0, arg1);
            return 0i32 as intptr_t;
        }
        5 => {
            UI_Refresh(arg0);
            return 0i32 as intptr_t;
        }
        6 => return UI_IsFullscreen() as intptr_t,
        7 => {
            UI_SetActiveMenu(arg0 as uiMenuCommand_t);
            return 0i32 as intptr_t;
        }
        8 => return UI_ConsoleCommand(arg0) as intptr_t,
        9 => {
            UI_DrawConnectScreen(arg0 as qboolean);
            return 0i32 as intptr_t;
        }
        10 => return qtrue as libc::c_int as intptr_t,
        _ => {}
    }
    return -1i32 as intptr_t;
}
unsafe extern "C" fn run_static_initializers() {
    cvarTableSize = (::std::mem::size_of::<[cvarTable_t; 49]>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<cvarTable_t>() as libc::c_ulong)
        as libc::c_int
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cvarTable_t {
    pub vmCvar: *mut vmCvar_t,
    pub cvarName: *mut libc::c_char,
    pub defaultString: *mut libc::c_char,
    pub cvarFlags: libc::c_int,
}
