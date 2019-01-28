use libc;
use q_shared_h::{unnamed, va, Info_ValueForKey, Q_stricmp, EXEC_APPEND, EXEC_INSERT, EXEC_NOW};
use stdlib::atoi;
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
use ui_local_h::{trap_Cmd_ExecuteText, trap_Cvar_SetValue, trap_Cvar_VariableValue};
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
// ui_spArena.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_SPArena_Start(mut arenaInfo: *const libc::c_char) {
    let mut map: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut level: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut txt: *mut libc::c_char = 0 as *mut libc::c_char;
    n = trap_Cvar_VariableValue(b"sv_maxclients\x00" as *const u8 as *const libc::c_char)
        as libc::c_int;
    if n < 8i32 {
        trap_Cvar_SetValue(
            b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
            8i32 as libc::c_float,
        );
    }
    level = atoi(Info_ValueForKey(
        arenaInfo,
        b"num\x00" as *const u8 as *const libc::c_char,
    ));
    txt = Info_ValueForKey(
        arenaInfo,
        b"special\x00" as *const u8 as *const libc::c_char,
    );
    if 0 != *txt.offset(0isize) {
        if Q_stricmp(txt, b"training\x00" as *const u8 as *const libc::c_char) == 0i32 {
            level = -4i32
        } else if Q_stricmp(txt, b"final\x00" as *const u8 as *const libc::c_char) == 0i32 {
            level = UI_GetNumSPTiers() * 4i32
        }
    }
    trap_Cvar_SetValue(
        b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
        level as libc::c_float,
    );
    map = Info_ValueForKey(arenaInfo, b"map\x00" as *const u8 as *const libc::c_char);
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        va(
            b"spmap %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            map,
        ),
    );
}
