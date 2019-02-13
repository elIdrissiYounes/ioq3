#![feature(libc)]
#![feature(extern_types)]
#![feature(asm)]
#![feature(ptr_wrapping_offset_from)]
#![feature(label_break_value)]
#![feature(const_raw_ptr_to_usize_cast)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]
#![feature(custom_attribute)]

use bg_misc::bg_itemlist;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
    AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
    AxisClear, MatrixMultiply, Q_fabs,
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
extern crate libc;

#[path = "src/bg_lib.rs"]
pub mod bg_lib;
#[path = "src/bg_misc.rs"]
pub mod bg_misc;
#[path = "src/q_math.rs"]
pub mod q_math;
#[path = "src/ui_addbots.rs"]
pub mod ui_addbots;
#[path = "src/ui_atoms.rs"]
pub mod ui_atoms;
#[path = "src/ui_cdkey.rs"]
pub mod ui_cdkey;
#[path = "src/ui_cinematics.rs"]
pub mod ui_cinematics;
#[path = "src/ui_confirm.rs"]
pub mod ui_confirm;
#[path = "src/ui_connect.rs"]
pub mod ui_connect;
#[path = "src/ui_controls2.rs"]
pub mod ui_controls2;
#[path = "src/ui_credits.rs"]
pub mod ui_credits;
#[path = "src/ui_demo2.rs"]
pub mod ui_demo2;
#[path = "src/ui_display.rs"]
pub mod ui_display;
#[path = "src/ui_gameinfo.rs"]
pub mod ui_gameinfo;
#[path = "src/ui_ingame.rs"]
pub mod ui_ingame;
#[path = "src/ui_loadconfig.rs"]
pub mod ui_loadconfig;
#[path = "src/ui_main.rs"]
pub mod ui_main;
#[path = "src/ui_menu.rs"]
pub mod ui_menu;
#[path = "src/ui_mfield.rs"]
pub mod ui_mfield;
#[path = "src/ui_mods.rs"]
pub mod ui_mods;
#[path = "src/ui_network.rs"]
pub mod ui_network;
#[path = "src/ui_options.rs"]
pub mod ui_options;
#[path = "src/ui_playermodel.rs"]
pub mod ui_playermodel;
#[path = "src/ui_players.rs"]
pub mod ui_players;
#[path = "src/ui_playersettings.rs"]
pub mod ui_playersettings;
#[path = "src/ui_preferences.rs"]
pub mod ui_preferences;
#[path = "src/ui_qmenu.rs"]
pub mod ui_qmenu;
#[path = "src/ui_removebots.rs"]
pub mod ui_removebots;
#[path = "src/ui_saveconfig.rs"]
pub mod ui_saveconfig;
#[path = "src/ui_serverinfo.rs"]
pub mod ui_serverinfo;
#[path = "src/ui_servers2.rs"]
pub mod ui_servers2;
#[path = "src/ui_setup.rs"]
pub mod ui_setup;
#[path = "src/ui_sound.rs"]
pub mod ui_sound;
#[path = "src/ui_sparena.rs"]
pub mod ui_sparena;
#[path = "src/ui_specifyserver.rs"]
pub mod ui_specifyserver;
#[path = "src/ui_splevel.rs"]
pub mod ui_splevel;
#[path = "src/ui_sppostgame.rs"]
pub mod ui_sppostgame;
#[path = "src/ui_spskill.rs"]
pub mod ui_spskill;
#[path = "src/ui_startserver.rs"]
pub mod ui_startserver;
#[path = "src/ui_team.rs"]
pub mod ui_team;
#[path = "src/ui_teamorders.rs"]
pub mod ui_teamorders;
#[path = "src/ui_video.rs"]
pub mod ui_video;
mod ui_public_h {
    use bg_misc::bg_itemlist;
    use q_math::{
        colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
        AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
        AxisClear, MatrixMultiply, Q_fabs,
    };
    use q_shared_h::connstate_t;
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent,
        UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale, UI_ProportionalStringWidth,
        UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor, UI_Shutdown,
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
        UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers,
        UI_GetSpecialArenaInfo, UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f,
        UI_SPUnlock_f, UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
    };
    use ui_ingame::{InGame_Cache, UI_InGameMenu};
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
    use ui_splevel::{
        UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f,
    };
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
    pub const UI_DRAW_CONNECT_SCREEN: unnamed = 9;
    pub const UI_KEY_EVENT: unnamed = 3;
    pub const UI_HASUNIQUECDKEY: unnamed = 10;
    pub const UIMENU_POSTGAME: uiMenuCommand_t = 6;
    pub const UIMENU_TEAM: uiMenuCommand_t = 5;
    pub const UI_INIT: unnamed = 1;
    pub const UI_SET_ACTIVE_MENU: unnamed = 7;
    pub const UI_IS_FULLSCREEN: unnamed = 6;
    pub const UI_MOUSE_EVENT: unnamed = 4;
    pub const UIMENU_NONE: uiMenuCommand_t = 0;
    pub const UI_REFRESH: unnamed = 5;
    pub const UI_GETAPIVERSION: unnamed = 0;
    pub const UIMENU_INGAME: uiMenuCommand_t = 2;
    pub const UI_SHUTDOWN: unnamed = 2;
    pub const UI_CONSOLE_COMMAND: unnamed = 8;
    pub const UIMENU_BAD_CD_KEY: uiMenuCommand_t = 4;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct uiClientState_t {
        pub connState: connstate_t,
        pub connectPacketCount: libc::c_int,
        pub clientNum: libc::c_int,
        pub servername: [libc::c_char; 1024],
        pub updateInfoString: [libc::c_char; 1024],
        pub messageString: [libc::c_char; 1024],
    }
    pub type unnamed = libc::c_uint;
    pub type uiMenuCommand_t = libc::c_uint;
    pub const UIMENU_MAIN: uiMenuCommand_t = 1;
    pub const UIMENU_NEED_CD: uiMenuCommand_t = 3;
    use libc;
}
mod stddef_h {
    use bg_misc::bg_itemlist;
    use q_math::{
        colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
        AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
        AxisClear, MatrixMultiply, Q_fabs,
    };
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent,
        UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale, UI_ProportionalStringWidth,
        UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor, UI_Shutdown,
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
        UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers,
        UI_GetSpecialArenaInfo, UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f,
        UI_SPUnlock_f, UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
    };
    use ui_ingame::{InGame_Cache, UI_InGameMenu};
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
    use ui_splevel::{
        UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f,
    };
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
    pub type size_t = libc::c_ulong;
    use libc;
}
mod keycodes_h {
    use bg_misc::bg_itemlist;
    use q_math::{
        colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
        AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
        AxisClear, MatrixMultiply, Q_fabs,
    };
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent,
        UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale, UI_ProportionalStringWidth,
        UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor, UI_Shutdown,
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
        UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers,
        UI_GetSpecialArenaInfo, UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f,
        UI_SPUnlock_f, UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
    };
    use ui_ingame::{InGame_Cache, UI_InGameMenu};
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
    use ui_splevel::{
        UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f,
    };
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
    pub type unnamed = libc::c_uint;
    pub const K_F10: unnamed_0 = 154;
    pub const K_PAD0_RIGHTSTICK_DOWN: unnamed = 362;
    pub const K_JOY19: unnamed_0 = 203;
    pub const K_JOY30: unnamed = 214;
    pub const K_WORLD_81: unnamed_0 = 314;
    pub const K_WORLD_41: unnamed_0 = 274;
    pub const K_PAD0_DPAD_UP: unnamed_0 = 351;
    pub const K_WORLD_48: unnamed_0 = 281;
    pub const K_MENU: unnamed_0 = 337;
    pub const K_WORLD_22: unnamed = 255;
    pub const K_WORLD_68: unnamed_0 = 301;
    pub const K_WORLD_95: unnamed_0 = 328;
    pub const K_JOY13: unnamed_0 = 197;
    pub const K_JOY29: unnamed_0 = 213;
    pub const K_TAB: unnamed = 9;
    pub const K_WORLD_88: unnamed_0 = 321;
    pub const K_WORLD_56: unnamed_0 = 289;
    pub const K_KP_UPARROW: unnamed_1 = 161;
    pub const K_WORLD_87: unnamed = 320;
    pub const K_KP_SLASH: unnamed = 172;
    pub const K_WORLD_83: unnamed = 316;
    pub const K_WORLD_9: unnamed_0 = 242;
    pub const K_WORLD_40: unnamed_1 = 273;
    pub const K_WORLD_25: unnamed = 258;
    pub const K_AUX8: unnamed_0 = 224;
    pub const K_PAD0_LEFTSTICK_UP: unnamed_0 = 357;
    pub const K_WORLD_15: unnamed_0 = 248;
    pub const K_KP_INS: unnamed = 170;
    pub const K_AUX9: unnamed_0 = 225;
    pub const K_PRINT: unnamed_0 = 333;
    pub const K_SCROLLOCK: unnamed_0 = 335;
    pub const K_WORLD_46: unnamed_0 = 279;
    pub const K_WORLD_57: unnamed_0 = 290;
    pub const K_SYSREQ: unnamed_0 = 334;
    pub const K_PAD0_LEFTSTICK_RIGHT: unnamed_0 = 356;
    pub type unnamed_0 = libc::c_uint;
    pub const K_WORLD_78: unnamed = 311;
    pub const K_F14: unnamed_0 = 158;
    pub const K_WORLD_93: unnamed = 326;
    pub const K_MOUSE3: unnamed = 180;
    pub const K_AUX7: unnamed = 223;
    pub const K_PAD0_LEFTSTICK_LEFT: unnamed = 355;
    pub const K_PAD0_DPAD_LEFT: unnamed = 353;
    pub const K_JOY32: unnamed = 216;
    pub const K_WORLD_91: unnamed_0 = 324;
    pub const K_WORLD_54: unnamed = 287;
    pub const K_AUX12: unnamed_1 = 228;
    pub const K_WORLD_50: unnamed = 283;
    pub const K_WORLD_7: unnamed_0 = 240;
    pub const K_KP_PGDN: unnamed = 168;
    pub const K_WORLD_29: unnamed_0 = 262;
    pub const K_JOY28: unnamed_0 = 212;
    pub const K_WORLD_75: unnamed_0 = 308;
    pub const K_WORLD_61: unnamed_1 = 294;
    pub const K_AUX15: unnamed_0 = 231;
    pub const K_END: unnamed_0 = 144;
    pub const K_ESCAPE: unnamed = 27;
    pub const K_AUX13: unnamed_0 = 229;
    pub const K_PAD0_RIGHTSTICK_CLICK: unnamed_0 = 348;
    pub const K_KP_RIGHTARROW: unnamed = 165;
    pub const K_PAD0_DPAD_DOWN: unnamed = 352;
    pub const K_WORLD_12: unnamed_0 = 245;
    pub const K_DOWNARROW: unnamed_0 = 133;
    pub const K_WORLD_3: unnamed_0 = 236;
    pub const K_WORLD_51: unnamed = 284;
    pub const K_AUX2: unnamed = 218;
    pub const K_WORLD_34: unnamed_0 = 267;
    pub const K_DEL: unnamed_0 = 140;
    pub const K_JOY23: unnamed = 207;
    pub const K_WORLD_55: unnamed_1 = 288;
    pub const K_WORLD_19: unnamed = 252;
    pub const K_F5: unnamed = 149;
    pub const K_WORLD_39: unnamed = 272;
    pub const K_WORLD_20: unnamed = 253;
    pub const K_WORLD_90: unnamed = 323;
    pub const K_RIGHTARROW: unnamed = 135;
    pub const K_JOY26: unnamed_0 = 210;
    pub const K_WORLD_23: unnamed = 256;
    pub const K_BACKSPACE: unnamed = 127;
    pub const K_WORLD_58: unnamed_0 = 291;
    pub const K_KP_LEFTARROW: unnamed_0 = 163;
    pub const K_PAD0_RIGHTTRIGGER: unnamed = 364;
    pub const K_WORLD_11: unnamed_1 = 244;
    pub const K_WORLD_17: unnamed_0 = 250;
    pub const K_POWER: unnamed_0 = 130;
    pub const K_F7: unnamed_0 = 151;
    pub const K_WORLD_66: unnamed_0 = 299;
    pub const K_PAD0_Y: unnamed_0 = 343;
    pub const K_WORLD_16: unnamed = 249;
    pub const K_JOY17: unnamed = 201;
    pub const K_JOY3: unnamed_0 = 187;
    pub const K_JOY27: unnamed_0 = 211;
    pub const K_WORLD_31: unnamed = 264;
    pub const K_F8: unnamed_0 = 152;
    pub const K_PAD0_A: unnamed_0 = 340;
    pub const K_WORLD_76: unnamed_0 = 309;
    pub const K_WORLD_82: unnamed_0 = 315;
    pub const K_PAD0_DPAD_RIGHT: unnamed_0 = 354;
    pub const K_JOY20: unnamed_0 = 204;
    pub const K_AUX6: unnamed = 222;
    pub const K_JOY9: unnamed = 193;
    pub const K_KP_STAR: unnamed_0 = 176;
    pub const K_WORLD_77: unnamed_0 = 310;
    pub const K_PAD0_RIGHTSTICK_LEFT: unnamed_0 = 359;
    pub const K_AUX10: unnamed_0 = 226;
    pub const K_F12: unnamed = 156;
    pub const K_COMPOSE: unnamed_1 = 330;
    pub const K_F4: unnamed = 148;
    pub const K_JOY5: unnamed_0 = 189;
    pub const K_WORLD_24: unnamed_1 = 257;
    pub const K_AUX4: unnamed = 220;
    pub const K_PAD0_GUIDE: unnamed_0 = 345;
    pub const K_ALT: unnamed_0 = 136;
    pub const K_JOY15: unnamed_0 = 199;
    pub const K_PAD0_BACK: unnamed = 344;
    pub const K_WORLD_21: unnamed_0 = 254;
    pub const K_KP_5: unnamed = 164;
    pub const K_WORLD_2: unnamed_0 = 235;
    pub const K_WORLD_53: unnamed_0 = 286;
    pub const K_JOY4: unnamed_0 = 188;
    pub const K_PGDN: unnamed_0 = 141;
    pub const K_WORLD_74: unnamed = 307;
    pub const K_CONSOLE: unnamed_0 = 365;
    pub const K_HELP: unnamed_0 = 332;
    pub const K_WORLD_47: unnamed_0 = 280;
    pub const K_WORLD_18: unnamed = 251;
    pub const K_WORLD_10: unnamed = 243;
    pub const K_JOY22: unnamed_0 = 206;
    pub const K_KP_END: unnamed_0 = 166;
    pub const K_SHIFT: unnamed = 138;
    pub const K_MOUSE1: unnamed_0 = 178;
    pub const K_WORLD_72: unnamed_0 = 305;
    pub const K_WORLD_42: unnamed_0 = 275;
    pub const K_JOY25: unnamed_0 = 209;
    pub const K_WORLD_28: unnamed_1 = 261;
    pub const K_WORLD_44: unnamed = 277;
    pub const K_F2: unnamed = 146;
    pub const K_WORLD_37: unnamed_1 = 270;
    pub const K_WORLD_73: unnamed_0 = 306;
    pub const K_WORLD_59: unnamed = 292;
    pub const K_JOY21: unnamed_0 = 205;
    pub const K_KP_PGUP: unnamed = 162;
    pub const K_PAD0_LEFTSTICK_DOWN: unnamed = 358;
    pub const K_WORLD_43: unnamed_0 = 276;
    pub const K_HOME: unnamed_0 = 143;
    pub const K_WORLD_0: unnamed_0 = 233;
    pub const K_F9: unnamed = 153;
    pub const K_SUPER: unnamed = 329;
    pub const K_JOY24: unnamed_0 = 208;
    pub const K_JOY1: unnamed_0 = 185;
    pub const K_WORLD_38: unnamed_0 = 271;
    pub const K_MOUSE2: unnamed = 179;
    pub const K_PAD0_LEFTSTICK_CLICK: unnamed_0 = 347;
    pub const K_PAD0_B: unnamed = 341;
    pub const K_F3: unnamed = 147;
    pub const K_F11: unnamed = 155;
    pub const K_F6: unnamed = 150;
    pub const K_WORLD_1: unnamed_0 = 234;
    pub const K_WORLD_60: unnamed_0 = 293;
    pub const K_WORLD_70: unnamed_0 = 303;
    pub const K_WORLD_35: unnamed_0 = 268;
    pub const K_WORLD_85: unnamed_0 = 318;
    pub const K_KP_HOME: unnamed_0 = 160;
    pub const K_PAD0_LEFTSHOULDER: unnamed_0 = 349;
    pub const MAX_KEYS: unnamed = 366;
    pub const K_WORLD_71: unnamed_0 = 304;
    pub const K_WORLD_80: unnamed_0 = 313;
    pub const K_KP_MINUS: unnamed_0 = 173;
    pub const K_MODE: unnamed = 331;
    pub const K_WORLD_69: unnamed = 302;
    pub const K_WORLD_63: unnamed_0 = 296;
    pub const K_LEFTARROW: unnamed = 134;
    pub const K_PAD0_RIGHTSHOULDER: unnamed_0 = 350;
    pub const K_WORLD_94: unnamed = 327;
    pub const K_PAD0_LEFTTRIGGER: unnamed = 363;
    pub const K_JOY31: unnamed_0 = 215;
    pub const K_AUX14: unnamed = 230;
    pub const K_WORLD_62: unnamed_0 = 295;
    pub const K_JOY14: unnamed_0 = 198;
    pub const K_PAD0_X: unnamed = 342;
    pub const K_WORLD_92: unnamed_0 = 325;
    pub const K_AUX16: unnamed_0 = 232;
    pub const K_JOY2: unnamed_0 = 186;
    pub const K_INS: unnamed_0 = 139;
    pub const K_CTRL: unnamed_0 = 137;
    pub const K_CAPSLOCK: unnamed = 129;
    pub const K_KP_PLUS: unnamed = 174;
    pub const K_WORLD_89: unnamed = 322;
    pub const K_COMMAND: unnamed = 128;
    pub const K_WORLD_79: unnamed_0 = 312;
    pub const K_AUX3: unnamed = 219;
    pub const K_PAD0_START: unnamed = 346;
    pub const K_BREAK: unnamed = 336;
    pub const K_MWHEELUP: unnamed_0 = 184;
    pub const K_WORLD_32: unnamed_0 = 265;
    pub const K_UNDO: unnamed_0 = 339;
    pub const K_PAD0_RIGHTSTICK_RIGHT: unnamed = 360;
    pub const K_JOY16: unnamed_0 = 200;
    pub const K_WORLD_86: unnamed = 319;
    pub const K_PAUSE: unnamed = 131;
    pub const K_WORLD_49: unnamed = 282;
    pub const K_WORLD_14: unnamed_0 = 247;
    pub const K_JOY8: unnamed_0 = 192;
    pub const K_MWHEELDOWN: unnamed_0 = 183;
    pub const K_KP_DEL: unnamed = 171;
    pub const K_WORLD_5: unnamed = 238;
    pub const K_WORLD_84: unnamed_0 = 317;
    pub const K_ENTER: unnamed = 13;
    pub const K_PGUP: unnamed_0 = 142;
    pub const K_WORLD_6: unnamed_0 = 239;
    pub const K_JOY10: unnamed_0 = 194;
    pub const K_UPARROW: unnamed_0 = 132;
    pub const K_F1: unnamed = 145;
    pub const K_KP_NUMLOCK: unnamed_0 = 175;
    pub const K_JOY12: unnamed = 196;
    pub const K_MOUSE4: unnamed_0 = 181;
    pub const K_JOY11: unnamed = 195;
    pub const K_WORLD_30: unnamed_0 = 263;
    pub const K_F13: unnamed_0 = 157;
    pub type unnamed_1 = libc::c_uint;
    pub const K_SPACE: unnamed = 32;
    pub const K_WORLD_45: unnamed_0 = 278;
    pub const K_KP_DOWNARROW: unnamed_1 = 167;
    pub const K_AUX11: unnamed_0 = 227;
    pub const K_AUX1: unnamed = 217;
    pub const K_WORLD_26: unnamed_0 = 259;
    pub const K_WORLD_33: unnamed_0 = 266;
    pub const K_AUX5: unnamed_0 = 221;
    pub const K_WORLD_67: unnamed_0 = 300;
    pub const K_WORLD_4: unnamed = 237;
    pub const K_MOUSE5: unnamed_0 = 182;
    pub const K_JOY18: unnamed_1 = 202;
    pub const K_WORLD_65: unnamed = 298;
    pub const K_WORLD_64: unnamed_0 = 297;
    pub const K_EURO: unnamed = 338;
    pub const K_WORLD_27: unnamed = 260;
    pub const K_WORLD_36: unnamed_0 = 269;
    pub const K_WORLD_52: unnamed_0 = 285;
    pub const K_KP_EQUALS: unnamed_0 = 177;
    pub const K_JOY7: unnamed_0 = 191;
    pub const K_WORLD_13: unnamed = 246;
    pub const K_PAD0_RIGHTSTICK_UP: unnamed_0 = 361;
    pub const K_F15: unnamed_0 = 159;
    pub const K_JOY6: unnamed = 190;
    pub const K_WORLD_8: unnamed_0 = 241;
    pub const K_KP_ENTER: unnamed_0 = 169;
    use libc;
}
mod bg_public_h {
    use bg_misc::bg_itemlist;
    use q_math::{
        colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
        AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
        AxisClear, MatrixMultiply, Q_fabs,
    };
    use q_shared_h::{entityState_t, playerState_t, qboolean, trajectory_t, vec_t};
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent,
        UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale, UI_ProportionalStringWidth,
        UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor, UI_Shutdown,
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
        UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers,
        UI_GetSpecialArenaInfo, UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f,
        UI_SPUnlock_f, UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
    };
    use ui_ingame::{InGame_Cache, UI_InGameMenu};
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
    use ui_splevel::{
        UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f,
    };
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
    pub const PW_DOUBLER: powerup_t = 12;
    pub const EV_NOAMMO: unnamed_4 = 21;
    pub const EV_STEP_4: unnamed_4 = 6;
    pub const PM_SPINTERMISSION: unnamed_1 = 6;
    pub const EV_TAUNT_PATROL: unnamed_4 = 82;
    pub const EV_TAUNT_FOLLOWME: unnamed_4 = 79;
    pub const EV_JUMP: unnamed_4 = 14;
    pub const EV_FOOTWADE: unnamed_4 = 4;
    pub const PERS_DEFEND_COUNT: unnamed_3 = 11;
    pub const PM_NOCLIP: unnamed_1 = 1;
    pub const ET_PLAYER: unnamed_6 = 1;
    pub const STAT_CLIENTS_READY: unnamed_2 = 5;
    pub const EV_MISSILE_MISS: unnamed_4 = 51;
    pub const HI_KAMIKAZE: holdable_t = 3;
    pub const EV_STEP_16: unnamed_4 = 9;
    pub const PERS_GAUNTLET_FRAG_COUNT: unnamed_3 = 13;
    pub const EV_GLOBAL_TEAM_SOUND: unnamed_4 = 47;
    pub const EV_TAUNT: unnamed_4 = 76;
    pub const STAT_ARMOR: unnamed_2 = 3;
    pub const EV_BULLET_HIT_WALL: unnamed_4 = 49;
    pub const PW_REDFLAG: powerup_t = 7;
    pub const HI_NONE: holdable_t = 0;
    pub const EV_TAUNT_NO: unnamed_4 = 78;
    pub const EV_KAMIKAZE: unnamed_4 = 68;
    pub const EV_BULLET: unnamed_4 = 55;
    pub const EV_PLAYER_TELEPORT_IN: unnamed_4 = 42;
    pub const IT_HEALTH: itemType_t = 4;
    pub const EV_DEATH3: unnamed_4 = 59;
    pub type powerup_t = libc::c_uint;
    pub const EV_USE_ITEM14: unnamed_4 = 38;
    pub const EV_ITEM_PICKUP: unnamed_4 = 19;
    pub const STAT_DEAD_YAW: unnamed_2 = 4;
    pub const PM_NORMAL: unnamed_1 = 0;
    pub const EV_GENERAL_SOUND: unnamed_4 = 45;
    pub const EV_USE_ITEM0: unnamed_4 = 24;
    pub const PM_DEAD: unnamed_1 = 3;
    pub const PERS_CAPTURES: unnamed_3 = 14;
    pub const EV_FALL_FAR: unnamed_4 = 12;
    pub type unnamed_3 = libc::c_uint;
    pub const EV_SWIM: unnamed_4 = 5;
    pub const EV_FIRE_WEAPON: unnamed_4 = 23;
    pub const EV_OBELISKEXPLODE: unnamed_4 = 69;
    pub const PW_INVULNERABILITY: powerup_t = 14;
    pub const EV_RAILTRAIL: unnamed_4 = 53;
    pub type unnamed_6 = libc::c_uint;
    pub const EV_STOPLOOPINGSOUND: unnamed_4 = 75;
    pub const PW_BATTLESUIT: powerup_t = 2;
    pub const ET_INVISIBLE: unnamed_6 = 10;
    pub const TORSO_PATROL: unnamed_0 = 27;
    pub const PERS_ATTACKEE_ARMOR: unnamed_3 = 7;
    pub const STAT_HOLDABLE_ITEM: unnamed_2 = 1;
    pub const PERS_IMPRESSIVE_COUNT: unnamed_3 = 9;
    pub const EV_GRENADE_BOUNCE: unnamed_4 = 44;
    pub const ET_TEAM: unnamed_6 = 12;
    pub const EV_TAUNT_YES: unnamed_4 = 77;
    pub const EV_PROXIMITY_MINE_TRIGGER: unnamed_4 = 67;
    pub const EV_USE_ITEM9: unnamed_4 = 33;
    pub const PERS_RANK: unnamed_3 = 2;
    pub const TORSO_STAND: unnamed_0 = 11;
    pub const EV_FALL_MEDIUM: unnamed_4 = 11;
    pub const EV_USE_ITEM1: unnamed_4 = 25;
    pub const EV_DEATH1: unnamed_4 = 57;
    pub const EV_WATER_TOUCH: unnamed_4 = 15;
    pub const EV_WATER_LEAVE: unnamed_4 = 16;
    pub const EV_USE_ITEM13: unnamed_4 = 37;
    pub const EV_DEBUG_LINE: unnamed_4 = 74;
    pub const EV_USE_ITEM5: unnamed_4 = 29;
    pub const EV_FOOTSTEP_METAL: unnamed_4 = 2;
    pub const PW_AMMOREGEN: powerup_t = 13;
    pub const PW_FLIGHT: powerup_t = 6;
    pub const PW_NUM_POWERUPS: powerup_t = 15;
    pub const PW_BLUEFLAG: powerup_t = 8;
    pub const EV_TAUNT_GUARDBASE: unnamed_4 = 81;
    pub const HI_TELEPORTER: holdable_t = 1;
    pub const STAT_MAX_HEALTH: unnamed_2 = 6;
    pub const HI_PORTAL: holdable_t = 4;
    pub const PERS_KILLED: unnamed_3 = 8;
    pub const BOTH_DEAD1: unnamed_0 = 1;
    pub const PW_SCOUT: powerup_t = 10;
    pub const IT_HOLDABLE: itemType_t = 6;
    pub const ET_ITEM: unnamed_6 = 2;
    pub const WP_BFG: weapon_t = 9;
    pub const LEGS_BACK: unnamed_0 = 16;
    pub const EV_POWERUP_REGEN: unnamed_4 = 63;
    pub const PERS_SCORE: unnamed_3 = 0;
    pub const EV_WATER_UNDER: unnamed_4 = 17;
    pub const PW_QUAD: powerup_t = 1;
    pub const WP_NUM_WEAPONS: weapon_t = 11;
    pub const EV_USE_ITEM15: unnamed_4 = 39;
    pub const EV_ITEM_POP: unnamed_4 = 41;
    pub type unnamed_5 = libc::c_uint;
    pub const ET_PUSH_TRIGGER: unnamed_6 = 8;
    pub const MAX_ANIMATIONS: unnamed_0 = 31;
    pub const EV_USE_ITEM6: unnamed_4 = 30;
    pub const PERS_TEAM: unnamed_3 = 3;
    pub const ET_BEAM: unnamed_6 = 5;
    pub const IT_TEAM: itemType_t = 8;
    pub const WP_GRAPPLING_HOOK: weapon_t = 10;
    pub const ET_GRAPPLE: unnamed_6 = 11;
    pub const EV_LIGHTNINGBOLT: unnamed_4 = 73;
    pub const EV_BULLET_HIT_FLESH: unnamed_4 = 48;
    pub const ET_TELEPORT_TRIGGER: unnamed_6 = 9;
    pub const TEAM_BLUE: unnamed_1 = 2;
    pub const PERS_SPAWN_COUNT: unnamed_3 = 4;
    pub const TORSO_GESTURE: unnamed_0 = 6;
    pub const EV_DEATH2: unnamed_4 = 58;
    pub const EV_MISSILE_MISS_METAL: unnamed_4 = 52;
    pub const LEGS_BACKWALK: unnamed_1 = 33;
    pub const FLAG_RUN: unnamed_0 = 34;
    pub const PERS_HITS: unnamed_3 = 1;
    pub const BOTH_DEATH1: unnamed_0 = 0;
    pub const TORSO_STAND2: unnamed_1 = 12;
    pub const WP_SHOTGUN: weapon_t = 3;
    pub const GT_FFA: unnamed_0 = 0;
    pub type weapon_t = libc::c_uint;
    pub type unnamed_2 = libc::c_uint;
    pub const EV_CHANGE_WEAPON: unnamed_4 = 22;
    pub const ET_GENERAL: unnamed_6 = 0;
    pub const EV_PROXIMITY_MINE_STICK: unnamed_4 = 66;
    pub const LEGS_WALKCR: unnamed_0 = 13;
    pub const LEGS_IDLE: unnamed_1 = 22;
    pub const EV_USE_ITEM12: unnamed_4 = 36;
    pub const EV_POWERUP_QUAD: unnamed_4 = 61;
    pub const EV_FOOTSTEP: unnamed_4 = 1;
    pub const EV_SHOTGUN: unnamed_4 = 54;
    pub const EV_INVUL_IMPACT: unnamed_4 = 71;
    pub const HI_INVULNERABILITY: holdable_t = 5;
    pub const GT_1FCTF: unnamed_1 = 5;
    pub const EV_OBITUARY: unnamed_4 = 60;
    pub const PW_NEUTRALFLAG: powerup_t = 9;
    pub const PM_FREEZE: unnamed_1 = 4;
    pub const PW_HASTE: powerup_t = 3;
    pub const EV_FOOTSPLASH: unnamed_4 = 3;
    pub type animation_t = animation_s;
    pub const PW_GUARD: powerup_t = 11;
    pub const GT_MAX_GAME_TYPE: unnamed_0 = 8;
    pub const EV_TAUNT_GETFLAG: unnamed_4 = 80;
    pub const EV_ITEM_RESPAWN: unnamed_4 = 40;
    pub const BOTH_DEATH2: unnamed_0 = 2;
    pub const PW_INVIS: powerup_t = 4;
    pub const PERS_PLAYEREVENTS: unnamed_3 = 5;
    pub const EV_JUMP_PAD: unnamed_4 = 13;
    pub const TORSO_GETFLAG: unnamed_0 = 25;
    pub const IT_AMMO: itemType_t = 2;
    pub const IT_PERSISTANT_POWERUP: itemType_t = 7;
    pub const PM_SPECTATOR: unnamed_1 = 2;
    pub const EV_USE_ITEM3: unnamed_4 = 27;
    pub const EV_GLOBAL_ITEM_PICKUP: unnamed_4 = 20;
    pub const WP_NONE: weapon_t = 0;
    pub const EV_USE_ITEM11: unnamed_4 = 35;
    pub const EV_STEP_8: unnamed_4 = 7;
    pub const PM_INTERMISSION: unnamed_1 = 5;
    pub const LEGS_TURN: unnamed_0 = 24;
    pub const WP_GAUNTLET: weapon_t = 1;
    pub type holdable_t = libc::c_uint;
    pub const ET_MISSILE: unnamed_6 = 3;
    pub const BOTH_DEAD2: unnamed_1 = 3;
    pub const LEGS_IDLECR: unnamed_0 = 23;
    pub const IT_ARMOR: itemType_t = 3;
    pub const TORSO_FOLLOWME: unnamed_1 = 28;
    pub const GT_HARVESTER: unnamed_1 = 7;
    pub const GT_TEAM: unnamed_1 = 3;
    pub const TORSO_ATTACK: unnamed_1 = 7;
    pub const HI_NUM_HOLDABLE: holdable_t = 6;
    pub const TEAM_FREE: unnamed_5 = 0;
    pub const PERS_ASSIST_COUNT: unnamed_3 = 12;
    pub const EV_NONE: unnamed_4 = 0;
    pub const IT_BAD: itemType_t = 0;
    pub const LEGS_LANDB: unnamed_0 = 21;
    pub const STAT_HEALTH: unnamed_2 = 0;
    pub const EV_POWERUP_BATTLESUIT: unnamed_4 = 62;
    pub const GT_TOURNAMENT: unnamed_0 = 1;
    pub const WP_RAILGUN: weapon_t = 7;
    pub const WP_GRENADE_LAUNCHER: weapon_t = 4;
    pub const EV_MISSILE_HIT: unnamed_4 = 50;
    pub type gitem_t = gitem_s;
    pub const LEGS_WALK: unnamed_0 = 14;
    pub type unnamed_1 = libc::c_uint;
    pub const GT_SINGLE_PLAYER: unnamed_0 = 2;
    pub const IT_POWERUP: itemType_t = 5;
    pub const EV_PAIN: unnamed_4 = 56;
    pub const EV_USE_ITEM7: unnamed_4 = 31;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct animation_s {
        pub firstFrame: libc::c_int,
        pub numFrames: libc::c_int,
        pub loopFrames: libc::c_int,
        pub frameLerp: libc::c_int,
        pub initialLerp: libc::c_int,
        pub reversed: libc::c_int,
        pub flipflop: libc::c_int,
    }
    pub const BOTH_DEAD3: unnamed_0 = 5;
    pub const LEGS_JUMPB: unnamed_0 = 20;
    pub type itemType_t = libc::c_uint;
    pub const EV_WATER_CLEAR: unnamed_4 = 18;
    pub const BOTH_DEATH3: unnamed_1 = 4;
    pub const WP_PLASMAGUN: weapon_t = 8;
    pub const ET_MOVER: unnamed_6 = 4;
    pub const LEGS_BACKCR: unnamed_1 = 32;
    pub type unnamed_0 = libc::c_uint;
    pub const MAX_TOTALANIMATIONS: unnamed_0 = 37;
    pub const TEAM_SPECTATOR: unnamed_1 = 3;
    pub const TORSO_RAISE: unnamed_0 = 10;
    pub const PW_NONE: powerup_t = 0;
    pub const ET_PORTAL: unnamed_6 = 6;
    pub type unnamed_4 = libc::c_uint;
    pub const TORSO_DROP: unnamed_0 = 9;
    pub const GT_CTF: unnamed_0 = 4;
    pub const TORSO_GUARDBASE: unnamed_1 = 26;
    pub const EV_STEP_12: unnamed_4 = 8;
    pub const STAT_WEAPONS: unnamed_2 = 2;
    pub const PW_REGEN: powerup_t = 5;
    pub const EV_USE_ITEM10: unnamed_4 = 34;
    pub const EV_GLOBAL_SOUND: unnamed_4 = 46;
    pub const EV_OBELISKPAIN: unnamed_4 = 70;
    pub const TEAM_RED: unnamed_2 = 1;
    pub const EV_USE_ITEM8: unnamed_4 = 32;
    pub const IT_WEAPON: itemType_t = 1;
    pub const LEGS_SWIM: unnamed_1 = 17;
    pub const LEGS_RUN: unnamed_0 = 15;
    pub const EV_FALL_SHORT: unnamed_4 = 10;
    pub const EV_PLAYER_TELEPORT_OUT: unnamed_4 = 43;
    pub const EV_USE_ITEM4: unnamed_4 = 28;
    pub const TORSO_NEGATIVE: unnamed_0 = 30;
    pub const PERS_EXCELLENT_COUNT: unnamed_3 = 10;
    pub const GT_OBELISK: unnamed_0 = 6;
    pub const LEGS_JUMP: unnamed_0 = 18;
    pub const ET_EVENTS: unnamed_6 = 13;
    pub const LEGS_LAND: unnamed_0 = 19;
    pub const PERS_ATTACKER: unnamed_3 = 6;
    pub const HI_MEDKIT: holdable_t = 2;
    pub const FLAG_STAND: unnamed_0 = 35;
    pub const TORSO_AFFIRMATIVE: unnamed_0 = 29;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct gitem_s {
        pub classname: *mut libc::c_char,
        pub pickup_sound: *mut libc::c_char,
        pub world_model: [*mut libc::c_char; 4],
        pub icon: *mut libc::c_char,
        pub pickup_name: *mut libc::c_char,
        pub quantity: libc::c_int,
        pub giType: itemType_t,
        pub giTag: libc::c_int,
        pub precaches: *mut libc::c_char,
        pub sounds: *mut libc::c_char,
    }
    pub const TEAM_NUM_TEAMS: unnamed_1 = 4;
    pub const EV_SCOREPLUM: unnamed_4 = 65;
    pub const TORSO_ATTACK2: unnamed_1 = 8;
    pub const ET_SPEAKER: unnamed_6 = 7;
    pub const WP_ROCKET_LAUNCHER: weapon_t = 5;
    pub const WP_LIGHTNING: weapon_t = 6;
    pub const EV_GIB_PLAYER: unnamed_4 = 64;
    pub const FLAG_STAND2RUN: unnamed_0 = 36;
    pub const EV_JUICED: unnamed_4 = 72;
    pub const EV_USE_ITEM2: unnamed_4 = 26;
    pub const WP_MACHINEGUN: weapon_t = 2;
    use libc;
}
mod stdlib {
    use bg_misc::bg_itemlist;
    use q_math::{
        colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
        AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
        AxisClear, MatrixMultiply, Q_fabs,
    };
    use stddef_h::size_t;
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent,
        UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale, UI_ProportionalStringWidth,
        UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor, UI_Shutdown,
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
        UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers,
        UI_GetSpecialArenaInfo, UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f,
        UI_SPUnlock_f, UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
    };
    use ui_ingame::{InGame_Cache, UI_InGameMenu};
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
    use ui_splevel::{
        UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f,
    };
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
    extern "C" {

        #[no_mangle]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;

    }
    extern "C" {
        #[no_mangle]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;

    }
    extern "C" {

        #[no_mangle]
        pub fn tan(_: libc::c_double) -> libc::c_double;

    }
    extern "C" {

        #[no_mangle]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

    }
    pub type intptr_t = libc::c_long;
    extern "C" {

        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

    }
    extern "C" {
        #[no_mangle]
        pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
    }
    pub type __compar_fn_t =
        Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
    pub type __int32_t = libc::c_int;
    extern "C" {
        #[no_mangle]
        pub fn strtod(
            __nptr: *const libc::c_char,
            __endptr: *mut *mut libc::c_char,
        ) -> libc::c_double;

        #[no_mangle]
        pub fn rand() -> libc::c_int;
    }
    extern "C" {

        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

    }
    extern "C" {

        #[no_mangle]
        pub fn qsort(
            __base: *mut libc::c_void,
            __nmemb: size_t,
            __size: size_t,
            __compar: __compar_fn_t,
        );
    }
    extern "C" {
        #[no_mangle]
        pub fn acos(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;

    }
    extern "C" {

        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
    extern "C" {
        #[no_mangle]
        pub fn cos(_: libc::c_double) -> libc::c_double;

        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
    }
    extern "C" {
        #[no_mangle]
        pub fn strtol(
            __nptr: *const libc::c_char,
            __endptr: *mut *mut libc::c_char,
            __base: libc::c_int,
        ) -> libc::c_long;
    }
    extern "C" {

        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

    }
    extern "C" {

        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
    extern "C" {
        #[no_mangle]
        pub fn sin(_: libc::c_double) -> libc::c_double;
    }
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    }
    use libc;
}
mod tr_types_h {
    use bg_misc::bg_itemlist;
    use q_math::{
        colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
        AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
        AxisClear, MatrixMultiply, Q_fabs,
    };
    use q_shared_h::{byte, qboolean, qhandle_t, vec3_t};
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent,
        UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale, UI_ProportionalStringWidth,
        UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor, UI_Shutdown,
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
        UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers,
        UI_GetSpecialArenaInfo, UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f,
        UI_SPUnlock_f, UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
    };
    use ui_ingame::{InGame_Cache, UI_InGameMenu};
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
    use ui_splevel::{
        UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f,
    };
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
    pub const RT_SPRITE: refEntityType_t = 2;
    pub const RT_RAIL_RINGS: refEntityType_t = 5;
    pub const RT_MAX_REF_ENTITY_TYPE: refEntityType_t = 8;
    pub const RT_PORTALSURFACE: refEntityType_t = 7;
    pub type refEntityType_t = libc::c_uint;
    pub const RT_RAIL_CORE: refEntityType_t = 4;
    pub const RT_POLY: refEntityType_t = 1;
    pub const RT_BEAM: refEntityType_t = 3;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct refEntity_t {
        pub reType: refEntityType_t,
        pub renderfx: libc::c_int,
        pub hModel: qhandle_t,
        pub lightingOrigin: vec3_t,
        pub shadowPlane: libc::c_float,
        pub axis: [vec3_t; 3],
        pub nonNormalizedAxes: qboolean,
        pub origin: [libc::c_float; 3],
        pub frame: libc::c_int,
        pub oldorigin: [libc::c_float; 3],
        pub oldframe: libc::c_int,
        pub backlerp: libc::c_float,
        pub skinNum: libc::c_int,
        pub customSkin: qhandle_t,
        pub customShader: qhandle_t,
        pub shaderRGBA: [byte; 4],
        pub shaderTexCoord: [libc::c_float; 2],
        pub shaderTime: libc::c_float,
        pub radius: libc::c_float,
        pub rotation: libc::c_float,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct glconfig_t {
        pub renderer_string: [libc::c_char; 1024],
        pub vendor_string: [libc::c_char; 1024],
        pub version_string: [libc::c_char; 1024],
        pub extensions_string: [libc::c_char; 8192],
        pub maxTextureSize: libc::c_int,
        pub numTextureUnits: libc::c_int,
        pub colorBits: libc::c_int,
        pub depthBits: libc::c_int,
        pub stencilBits: libc::c_int,
        pub driverType: glDriverType_t,
        pub hardwareType: glHardwareType_t,
        pub deviceSupportsGamma: qboolean,
        pub textureCompression: textureCompression_t,
        pub textureEnvAddAvailable: qboolean,
        pub vidWidth: libc::c_int,
        pub vidHeight: libc::c_int,
        pub windowAspect: libc::c_float,
        pub displayFrequency: libc::c_int,
        pub isFullscreen: qboolean,
        pub stereoEnabled: qboolean,
        pub smpActive: qboolean,
    }
    pub const GLDRV_VOODOO: glDriverType_t = 2;
    pub const RT_MODEL: refEntityType_t = 0;
    pub const GLHW_RAGEPRO: glHardwareType_t = 3;
    pub type glDriverType_t = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct refdef_t {
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub fov_x: libc::c_float,
        pub fov_y: libc::c_float,
        pub vieworg: vec3_t,
        pub viewaxis: [vec3_t; 3],
        pub time: libc::c_int,
        pub rdflags: libc::c_int,
        pub areamask: [byte; 32],
        pub text: [[libc::c_char; 32]; 8],
    }
    pub const GLHW_3DFX_2D3D: glHardwareType_t = 1;
    pub const TC_NONE: textureCompression_t = 0;
    pub type glHardwareType_t = libc::c_uint;
    pub const GLDRV_STANDALONE: glDriverType_t = 1;
    pub type textureCompression_t = libc::c_uint;
    pub const TC_S3TC: textureCompression_t = 1;
    pub const RT_LIGHTNING: refEntityType_t = 6;
    pub const GLHW_RIVA128: glHardwareType_t = 2;
    pub const GLDRV_ICD: glDriverType_t = 0;
    pub const TC_S3TC_ARB: textureCompression_t = 2;
    pub const GLHW_GENERIC: glHardwareType_t = 0;
    pub const GLHW_PERMEDIA2: glHardwareType_t = 4;
    use libc;
}
mod q_shared_h {
    use bg_misc::bg_itemlist;
    use q_math::{
        colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
        AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
        AxisClear, MatrixMultiply, Q_fabs,
    };
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent,
        UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale, UI_ProportionalStringWidth,
        UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor, UI_Shutdown,
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
        UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers,
        UI_GetSpecialArenaInfo, UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f,
        UI_SPUnlock_f, UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
    };
    use ui_ingame::{InGame_Cache, UI_InGameMenu};
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
    use ui_splevel::{
        UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f,
    };
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

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct playerState_s {
        pub commandTime: libc::c_int,
        pub pm_type: libc::c_int,
        pub bobCycle: libc::c_int,
        pub pm_flags: libc::c_int,
        pub pm_time: libc::c_int,
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub weaponTime: libc::c_int,
        pub gravity: libc::c_int,
        pub speed: libc::c_int,
        pub delta_angles: [libc::c_int; 3],
        pub groundEntityNum: libc::c_int,
        pub legsTimer: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoTimer: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub movementDir: libc::c_int,
        pub grapplePoint: vec3_t,
        pub eFlags: libc::c_int,
        pub eventSequence: libc::c_int,
        pub events: [libc::c_int; 2],
        pub eventParms: [libc::c_int; 2],
        pub externalEvent: libc::c_int,
        pub externalEventParm: libc::c_int,
        pub externalEventTime: libc::c_int,
        pub clientNum: libc::c_int,
        pub weapon: libc::c_int,
        pub weaponstate: libc::c_int,
        pub viewangles: vec3_t,
        pub viewheight: libc::c_int,
        pub damageEvent: libc::c_int,
        pub damageYaw: libc::c_int,
        pub damagePitch: libc::c_int,
        pub damageCount: libc::c_int,
        pub stats: [libc::c_int; 16],
        pub persistant: [libc::c_int; 16],
        pub powerups: [libc::c_int; 16],
        pub ammo: [libc::c_int; 16],
        pub generic1: libc::c_int,
        pub loopSound: libc::c_int,
        pub jumppad_ent: libc::c_int,
        pub ping: libc::c_int,
        pub pmove_framecount: libc::c_int,
        pub jumppad_frame: libc::c_int,
        pub entityEventSequence: libc::c_int,
    }
    pub type cplane_t = cplane_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct trajectory_t {
        pub trType: trType_t,
        pub trTime: libc::c_int,
        pub trDuration: libc::c_int,
        pub trBase: vec3_t,
        pub trDelta: vec3_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct orientation_t {
        pub origin: vec3_t,
        pub axis: [vec3_t; 3],
    }
    extern "C" {

        //=============================================
        #[no_mangle]
        pub fn Q_isprint(c: libc::c_int) -> libc::c_int;

    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    pub const ERR_SERVERDISCONNECT: unnamed = 2;
    pub const TR_STATIONARY: trType_t = 0;
    pub const ERR_DROP: unnamed = 1;
    pub const TR_LINEAR_STOP: trType_t = 3;
    pub const TR_SINE: trType_t = 4;
    pub type clipHandle_t = libc::c_int;
    pub type playerState_t = playerState_s;
    pub const TR_GRAVITY: trType_t = 5;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union floatint_t {
        pub f: libc::c_float,
        pub i: libc::c_int,
        pub ui: libc::c_uint,
    }
    pub const ERR_DISCONNECT: unnamed = 3;
    extern "C" {
        #[no_mangle]
        pub fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn COM_ParseExt(
            data_p: *mut *mut libc::c_char,
            allowLineBreak: qboolean,
        ) -> *mut libc::c_char;

        // portable case insensitive compare

        // buffer size safe library replacements

        //=============================================
        /*
        short	BigShort(short l);
        short	LittleShort(short l);
        int		BigLong (int l);
        int		LittleLong (int l);
        qint64  BigLong64 (qint64 l);
        qint64  LittleLong64 (qint64 l);
        float	BigFloat (const float *l);
        float	LittleFloat (const float *l);

        void	Swap_Init (void);
        */

        //=============================================
        //
        // key / value info strings
        //

        #[no_mangle]
        pub fn Info_SetValueForKey(
            s: *mut libc::c_char,
            key: *const libc::c_char,
            value: *const libc::c_char,
        );
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
    pub const TR_INTERPOLATE: trType_t = 1;
    pub const ERR_NEED_CD: unnamed = 4;
    pub const ERR_FATAL: unnamed = 0;
    pub const CHAN_LOCAL: unnamed_0 = 1;
    pub const CA_PRIMED: connstate_t = 7;
    extern "C" {
        // portable case insensitive compare

        //=============================================
        /*
        short	BigShort(short l);
        short	LittleShort(short l);
        int		BigLong (int l);
        int		LittleLong (int l);
        qint64  BigLong64 (qint64 l);
        qint64  LittleLong64 (qint64 l);
        float	BigFloat (const float *l);
        float	LittleFloat (const float *l);

        void	Swap_Init (void);
        */

        #[no_mangle]
        pub fn Info_NextPair(
            s: *mut *const libc::c_char,
            key: *mut libc::c_char,
            value: *mut libc::c_char,
        );
    }
    pub const TR_LINEAR: trType_t = 2;
    extern "C" {

        // portable case insensitive compare

        #[no_mangle]
        pub fn Q_stricmpn(
            s1: *const libc::c_char,
            s2: *const libc::c_char,
            n: libc::c_int,
        ) -> libc::c_int;

        // buffer size safe library replacements

        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int, src: *const libc::c_char);
    // removes color sequences from string

    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    pub const FS_READ: fsMode_t = 0;
    extern "C" {
        #[no_mangle]
        pub fn Q_islower(c: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Q_isupper(c: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Q_isalpha(c: libc::c_int) -> libc::c_int;
    }
    pub const FS_APPEND: fsMode_t = 2;
    pub const CHAN_WEAPON: unnamed = 2;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct entityState_s {
        pub number: libc::c_int,
        pub eType: libc::c_int,
        pub eFlags: libc::c_int,
        pub pos: trajectory_t,
        pub apos: trajectory_t,
        pub time: libc::c_int,
        pub time2: libc::c_int,
        pub origin: vec3_t,
        pub origin2: vec3_t,
        pub angles: vec3_t,
        pub angles2: vec3_t,
        pub otherEntityNum: libc::c_int,
        pub otherEntityNum2: libc::c_int,
        pub groundEntityNum: libc::c_int,
        pub constantLight: libc::c_int,
        pub loopSound: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub clientNum: libc::c_int,
        pub frame: libc::c_int,
        pub solid: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub generic1: libc::c_int,
    }
    pub const CHAN_BODY: unnamed_0 = 5;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cplane_s {
        pub normal: vec3_t,
        pub dist: libc::c_float,
        pub type_0: byte,
        pub signbits: byte,
        pub pad: [byte; 2],
    }
    pub const CA_CHALLENGING: connstate_t = 4;
    pub const CHAN_VOICE: unnamed = 3;
    pub const CA_DISCONNECTED: connstate_t = 1;
    pub const CHAN_ITEM: unnamed = 4;
    pub const CHAN_ANNOUNCER: unnamed = 7;
    pub const FS_WRITE: fsMode_t = 1;
    extern "C" {

        // portable case insensitive compare

        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...) -> !;
    }
    extern "C" {
        // all drawing is done to a 640*480 virtual screen size
        // and will be automatically scaled to the real resolution

        #[no_mangle]
        pub fn COM_StripExtension(
            in_0: *const libc::c_char,
            out: *mut libc::c_char,
            destsize: libc::c_int,
        );
    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    extern "C" {
        #[no_mangle]
        pub fn Q_IsColorString(p: *const libc::c_char) -> qboolean;

        //=============================================
        #[no_mangle]
        pub fn Com_Clamp(
            min: libc::c_float,
            max: libc::c_float,
            value: libc::c_float,
        ) -> libc::c_float;
    // buffer size safe library replacements

    // removes color sequences from string

    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    pub type trType_t = libc::c_uint;
    pub const CA_AUTHORIZING: connstate_t = 2;
    pub type cvarHandle_t = libc::c_int;
    pub type byte = libc::c_uchar;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vmCvar_t {
        pub handle: cvarHandle_t,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub string: [libc::c_char; 256],
    }
    pub const CA_CONNECTED: connstate_t = 5;
    pub type connstate_t = libc::c_uint;
    pub type unnamed_0 = libc::c_uint;
    pub const EXEC_APPEND: unnamed = 2;
    pub const CA_LOADING: connstate_t = 6;
    pub const CA_UNINITIALIZED: connstate_t = 0;
    pub type unnamed = libc::c_uint;
    pub type fsMode_t = libc::c_uint;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const CHAN_AUTO: unnamed_0 = 0;
    pub const CA_CONNECTING: connstate_t = 3;
    pub type vec3_t = [vec_t; 3];
    pub const EXEC_INSERT: unnamed = 1;
    pub type qboolean = libc::c_uint;
    pub type fileHandle_t = libc::c_int;
    pub const qfalse: qboolean = 0;
    pub type sfxHandle_t = libc::c_int;
    pub const CA_ACTIVE: connstate_t = 8;
    extern "C" {
        #[no_mangle]
        pub fn Com_sprintf(
            dest: *mut libc::c_char,
            size: libc::c_int,
            fmt: *const libc::c_char,
            ...
        ) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn Q_strupr(s1: *mut libc::c_char) -> *mut libc::c_char;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char, destsize: libc::c_int);
        // removes color sequences from string
        #[no_mangle]
        pub fn Q_CleanStr(string: *mut libc::c_char) -> *mut libc::c_char;
        //=============================================
        /*
        short	BigShort(short l);
        short	LittleShort(short l);
        int		BigLong (int l);
        int		LittleLong (int l);
        qint64  BigLong64 (qint64 l);
        qint64  LittleLong64 (qint64 l);
        float	BigFloat (const float *l);
        float	LittleFloat (const float *l);

        void	Swap_Init (void);
        */
        #[no_mangle]
        pub fn va(format: *mut libc::c_char, ...) -> *mut libc::c_char;
        //=============================================
        //
        // key / value info strings
        //
        #[no_mangle]
        pub fn Info_ValueForKey(
            s: *const libc::c_char,
            key: *const libc::c_char,
        ) -> *mut libc::c_char;
    }
    pub const EXEC_NOW: unnamed = 0;
    pub const CHAN_LOCAL_SOUND: unnamed_0 = 6;
    pub type qhandle_t = libc::c_int;
    pub const qtrue: qboolean = 1;
    pub type vec4_t = [vec_t; 4];
    pub const CA_CINEMATIC: connstate_t = 9;
    pub type entityState_t = entityState_s;
    pub type vec_t = libc::c_float;
    use libc;
}
mod ui_local_h {
    use bg_misc::bg_itemlist;
    use bg_public_h::{animation_t, weapon_t};
    use q_math::{
        colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
        AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
        AxisClear, MatrixMultiply, Q_fabs,
    };
    use q_shared_h::{
        byte, clipHandle_t, fileHandle_t, fsMode_t, orientation_t, qboolean, qhandle_t,
        sfxHandle_t, vec3_t, vec4_t, vec_t, vmCvar_t,
    };
    use tr_types_h::{glconfig_t, refEntity_t, refdef_t};
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen, UI_KeyEvent,
        UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale, UI_ProportionalStringWidth,
        UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor, UI_Shutdown,
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
        UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers,
        UI_GetSpecialArenaInfo, UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f,
        UI_SPUnlock_f, UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
    };
    use ui_ingame::{InGame_Cache, UI_InGameMenu};
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
    use ui_public_h::{uiClientState_t, uiMenuCommand_t};
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
    use ui_splevel::{
        UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f,
    };
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
    extern "C" {

        #[no_mangle]
        pub fn trap_R_AddLightToScene(
            org: *const vec_t,
            intensity: libc::c_float,
            r: libc::c_float,
            g: libc::c_float,
            b: libc::c_float,
        );

        #[no_mangle]
        pub fn trap_CM_LerpTag(
            tag: *mut orientation_t,
            mod_0: clipHandle_t,
            startFrame: libc::c_int,
            endFrame: libc::c_int,
            frac: libc::c_float,
            tagName: *const libc::c_char,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn trap_R_RegisterSkin(name: *const libc::c_char) -> qhandle_t;

    }
    extern "C" {

        //
        // ui_credits.c
        //

        //
        // ui_mods.c
        //

        //
        // ui_cinematics.c
        //

        //
        // ui_demo2.c
        //

        //
        // ui_servers2.c
        //

        #[no_mangle]
        pub fn trap_R_RenderScene(fd: *const refdef_t);
        #[no_mangle]
        pub fn trap_R_AddRefEntityToScene(re: *const refEntity_t);
        #[no_mangle]
        pub fn trap_R_ClearScene();

    //
    // ui_cdkey.c
    //

    }
    pub type unnamed_0 = libc::c_uint;
    extern "C" {

        #[no_mangle]
        pub fn trap_MemoryRemaining() -> libc::c_int;

    }
    extern "C" {

        //
        // ui_menu.c
        //

        //
        // ui_ingame.c
        //

        //
        // ui_confirm.c
        //

        //
        // ui_setup.c
        //

        #[no_mangle]
        pub fn trap_GetGlconfig(glconfig: *mut glconfig_t);

        #[no_mangle]
        pub fn trap_R_DrawStretchPic(
            x: libc::c_float,
            y: libc::c_float,
            w: libc::c_float,
            h: libc::c_float,
            s1: libc::c_float,
            t1: libc::c_float,
            s2: libc::c_float,
            t2: libc::c_float,
            hShader: qhandle_t,
        );

        #[no_mangle]
        pub fn trap_Key_GetCatcher() -> libc::c_int;

        //
        // ui_removebots.c
        //

        //
        // ui_addbots.c
        //

        //
        // ui_spPostgame.c
        //

        //
        // ui_spLevel.c
        //

        //
        // ui_network.c
        //

        //
        // ui_sound.c
        //

        //
        // ui_display.c
        //

        #[no_mangle]
        pub fn trap_Argv(n: libc::c_int, buffer: *mut libc::c_char, bufferLength: libc::c_int);

        #[no_mangle]
        pub fn trap_UpdateScreen();

        #[no_mangle]
        pub fn trap_Key_ClearStates();

    }
    pub const AWARD_GAUNTLET: unnamed_2 = 3;
    extern "C" {

        #[no_mangle]
        pub fn trap_VerifyCDKey(key: *const libc::c_char, chksum: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn trap_GetCDKey(buf: *mut libc::c_char, buflen: libc::c_int);

        #[no_mangle]
        pub fn trap_SetCDKey(buf: *mut libc::c_char);

    }
    extern "C" {

        #[no_mangle]
        pub fn trap_R_RegisterModel(name: *const libc::c_char) -> qhandle_t;

        #[no_mangle]
        pub fn trap_Key_GetBindingBuf(
            keynum: libc::c_int,
            buf: *mut libc::c_char,
            buflen: libc::c_int,
        );

        #[no_mangle]
        pub fn trap_Key_SetBinding(keynum: libc::c_int, binding: *const libc::c_char);

        #[no_mangle]
        pub fn trap_Key_KeynumToStringBuf(
            keynum: libc::c_int,
            buf: *mut libc::c_char,
            buflen: libc::c_int,
        );

    }
    pub type unnamed_2 = libc::c_uint;
    extern "C" {

        //
        // ui_specifyserver.c
        //

        #[no_mangle]
        pub fn trap_LAN_ClearPing(n: libc::c_int);
        #[no_mangle]
        pub fn trap_SetPbClStatus(status: libc::c_int);
        //
        // ui_startserver.c
        //

        #[no_mangle]
        pub fn trap_LAN_GetPingQueueCount() -> libc::c_int;
        #[no_mangle]
        pub fn trap_LAN_GetServerAddressString(
            source: libc::c_int,
            n: libc::c_int,
            buf: *mut libc::c_char,
            buflen: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_LAN_GetServerCount(source: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn trap_LAN_GetPingInfo(n: libc::c_int, buf: *mut libc::c_char, buflen: libc::c_int);
        #[no_mangle]
        pub fn trap_LAN_GetPing(
            n: libc::c_int,
            buf: *mut libc::c_char,
            buflen: libc::c_int,
            pingtime: *mut libc::c_int,
        );

    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menuaction_s {
        pub generic: menucommon_s,
    }
    extern "C" {

        #[no_mangle]
        pub fn trap_Cvar_Update(vmCvar: *mut vmCvar_t);
    //
    // ui_connect.c
    //

    }
    extern "C" {

        #[no_mangle]
        pub fn trap_Cvar_Reset(name: *const libc::c_char);

    }
    pub const AWARD_IMPRESSIVE: unnamed_2 = 1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menuslider_s {
        pub generic: menucommon_s,
        pub minvalue: libc::c_float,
        pub maxvalue: libc::c_float,
        pub curvalue: libc::c_float,
        pub range: libc::c_float,
    }
    pub const AWARD_FRAGS: unnamed = 4;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menufield_s {
        pub generic: menucommon_s,
        pub field: mfield_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menuradiobutton_s {
        pub generic: menucommon_s,
        pub curvalue: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct uiStatic_t {
        pub frametime: libc::c_int,
        pub realtime: libc::c_int,
        pub cursorx: libc::c_int,
        pub cursory: libc::c_int,
        pub menusp: libc::c_int,
        pub activemenu: *mut menuframework_s,
        pub stack: [*mut menuframework_s; 8],
        pub glconfig: glconfig_t,
        pub debug: qboolean,
        pub whiteShader: qhandle_t,
        pub menuBackShader: qhandle_t,
        pub menuBackNoLogoShader: qhandle_t,
        pub charset: qhandle_t,
        pub charsetProp: qhandle_t,
        pub charsetPropGlow: qhandle_t,
        pub charsetPropB: qhandle_t,
        pub cursor: qhandle_t,
        pub rb_on: qhandle_t,
        pub rb_off: qhandle_t,
        pub xscale: libc::c_float,
        pub yscale: libc::c_float,
        pub bias: libc::c_float,
        pub demoversion: qboolean,
        pub firstdraw: qboolean,
    }
    extern "C" {

        //
        // ui_playersettings.c
        //

        //
        // ui_startserver.c
        //

        //
        // ui_spSkill.c
        //

        #[no_mangle]
        pub fn trap_R_SetColor(rgba: *const libc::c_float);

        #[no_mangle]
        pub fn trap_Key_SetCatcher(catcher: libc::c_int);
    }
    extern "C" {

        //
        // ui_spArena.c
        //

        #[no_mangle]
        pub fn trap_S_StartLocalSound(sfx: sfxHandle_t, channelNum: libc::c_int);

        #[no_mangle]
        pub fn trap_Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);
        #[no_mangle]
        pub fn trap_S_RegisterSound(
            sample: *const libc::c_char,
            compressed: qboolean,
        ) -> sfxHandle_t;

    }
    pub type unnamed = libc::c_uint;
    pub const AWARD_EXCELLENT: unnamed_0 = 2;
    extern "C" {

        #[no_mangle]
        pub fn trap_Key_GetOverstrikeMode() -> qboolean;
        #[no_mangle]
        pub fn trap_Key_SetOverstrikeMode(state: qboolean);
        #[no_mangle]
        pub fn trap_Key_IsDown(keynum: libc::c_int) -> qboolean;
        #[no_mangle]
        pub fn trap_GetClipboardData(buf: *mut libc::c_char, bufsize: libc::c_int);

        #[no_mangle]
        pub fn trap_Error(string: *const libc::c_char) -> !;

    }
    pub const AWARD_PERFECT: unnamed = 5;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menulist_s {
        pub generic: menucommon_s,
        pub oldvalue: libc::c_int,
        pub curvalue: libc::c_int,
        pub numitems: libc::c_int,
        pub top: libc::c_int,
        pub itemnames: *mut *const libc::c_char,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub columns: libc::c_int,
        pub separation: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct lerpFrame_t {
        pub oldFrame: libc::c_int,
        pub oldFrameTime: libc::c_int,
        pub frame: libc::c_int,
        pub frameTime: libc::c_int,
        pub backlerp: libc::c_float,
        pub yawAngle: libc::c_float,
        pub yawing: qboolean,
        pub pitchAngle: libc::c_float,
        pub pitching: qboolean,
        pub animationNumber: libc::c_int,
        pub animation: *mut animation_t,
        pub animationTime: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menutext_s {
        pub generic: menucommon_s,
        pub string: *mut libc::c_char,
        pub style: libc::c_int,
        pub color: *mut libc::c_float,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct mfield_t {
        pub cursor: libc::c_int,
        pub scroll: libc::c_int,
        pub widthInChars: libc::c_int,
        pub buffer: [libc::c_char; 256],
        pub maxchars: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct playerInfo_t {
        pub legsModel: qhandle_t,
        pub legsSkin: qhandle_t,
        pub legs: lerpFrame_t,
        pub torsoModel: qhandle_t,
        pub torsoSkin: qhandle_t,
        pub torso: lerpFrame_t,
        pub headModel: qhandle_t,
        pub headSkin: qhandle_t,
        pub animations: [animation_t; 31],
        pub fixedlegs: qboolean,
        pub fixedtorso: qboolean,
        pub weaponModel: qhandle_t,
        pub barrelModel: qhandle_t,
        pub flashModel: qhandle_t,
        pub flashDlightColor: vec3_t,
        pub muzzleFlashTime: libc::c_int,
        pub color1: vec3_t,
        pub c1RGBA: [byte; 4],
        pub viewAngles: vec3_t,
        pub moveAngles: vec3_t,
        pub currentWeapon: weapon_t,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub weapon: weapon_t,
        pub lastWeapon: weapon_t,
        pub pendingWeapon: weapon_t,
        pub weaponTimer: libc::c_int,
        pub pendingLegsAnim: libc::c_int,
        pub torsoAnimationTimer: libc::c_int,
        pub pendingTorsoAnim: libc::c_int,
        pub legsAnimationTimer: libc::c_int,
        pub chat: qboolean,
        pub newModel: qboolean,
        pub barrelSpinning: qboolean,
        pub barrelAngle: libc::c_float,
        pub barrelTime: libc::c_int,
        pub realWeapon: libc::c_int,
    }
    extern "C" {

        #[no_mangle]
        pub fn trap_Cmd_ExecuteText(exec_when: libc::c_int, text: *const libc::c_char);
        #[no_mangle]
        pub fn trap_R_RegisterShaderNoMip(name: *const libc::c_char) -> qhandle_t;
        #[no_mangle]
        pub fn trap_GetClientState(state: *mut uiClientState_t);
        #[no_mangle]
        pub fn trap_GetConfigString(
            index: libc::c_int,
            buff: *mut libc::c_char,
            buffsize: libc::c_int,
        ) -> libc::c_int;
    }
    extern "C" {

        //
        // ui_syscalls.c
        //
        #[no_mangle]
        pub fn trap_Print(string: *const libc::c_char);
        #[no_mangle]
        pub fn trap_Cvar_Register(
            vmCvar: *mut vmCvar_t,
            varName: *const libc::c_char,
            defaultValue: *const libc::c_char,
            flags: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
        #[no_mangle]
        pub fn trap_Cvar_VariableValue(var_name: *const libc::c_char) -> libc::c_float;
        #[no_mangle]
        pub fn trap_Cvar_VariableStringBuffer(
            var_name: *const libc::c_char,
            buffer: *mut libc::c_char,
            bufsize: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_FS_FOpenFile(
            qpath: *const libc::c_char,
            f: *mut fileHandle_t,
            mode: fsMode_t,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_FS_Read(buffer: *mut libc::c_void, len: libc::c_int, f: fileHandle_t);
        #[no_mangle]
        pub fn trap_FS_FCloseFile(f: fileHandle_t);
        #[no_mangle]
        pub fn trap_FS_GetFileList(
            path: *const libc::c_char,
            extension: *const libc::c_char,
            listbuf: *mut libc::c_char,
            bufsize: libc::c_int,
        ) -> libc::c_int;
    }
    pub type menuframework_s = _tag_menuframework;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct _tag_menuframework {
        pub cursor: libc::c_int,
        pub cursor_prev: libc::c_int,
        pub nitems: libc::c_int,
        pub items: [*mut libc::c_void; 64],
        pub draw: Option<unsafe extern "C" fn() -> ()>,
        pub key: Option<unsafe extern "C" fn(_: libc::c_int) -> sfxHandle_t>,
        pub wrapAround: qboolean,
        pub fullscreen: qboolean,
        pub showlogo: qboolean,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menubitmap_s {
        pub generic: menucommon_s,
        pub focuspic: *mut libc::c_char,
        pub errorpic: *mut libc::c_char,
        pub shader: qhandle_t,
        pub focusshader: qhandle_t,
        pub width: libc::c_int,
        pub height: libc::c_int,
        pub focuscolor: *mut libc::c_float,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menucommon_s {
        pub type_0: libc::c_int,
        pub name: *const libc::c_char,
        pub id: libc::c_int,
        pub x: libc::c_int,
        pub y: libc::c_int,
        pub left: libc::c_int,
        pub top: libc::c_int,
        pub right: libc::c_int,
        pub bottom: libc::c_int,
        pub parent: *mut menuframework_s,
        pub menuPosition: libc::c_int,
        pub flags: libc::c_uint,
        pub callback: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ()>,
        pub statusbar: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
        pub ownerdraw: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>,
    }
    pub const AWARD_ACCURACY: unnamed_2 = 0;
    use libc;
}
