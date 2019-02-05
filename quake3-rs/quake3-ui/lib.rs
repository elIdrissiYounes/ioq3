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

mod tr_types_h {
    use q_shared_h::{byte, qboolean, qhandle_t, vec3_t};
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawRect, UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen,
        UI_KeyEvent, UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale,
        UI_ProportionalStringWidth, UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor,
        UI_Shutdown,
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
    pub const RT_RAIL_RINGS: refEntityType_t = 5;
    pub const RT_PORTALSURFACE: refEntityType_t = 7;

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
    pub type refEntityType_t = libc::c_uint;
    pub const RT_RAIL_CORE: refEntityType_t = 4;
    pub const RT_SPRITE: refEntityType_t = 2;
    pub const RT_MODEL: refEntityType_t = 0;
    pub type glDriverType_t = libc::c_uint;
    pub const RT_POLY: refEntityType_t = 1;
    pub type textureCompression_t = libc::c_uint;

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
    pub const RT_MAX_REF_ENTITY_TYPE: refEntityType_t = 8;
    pub const GLHW_RAGEPRO: glHardwareType_t = 3;
    pub const TC_S3TC_ARB: textureCompression_t = 2;

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
    pub type glHardwareType_t = libc::c_uint;
    pub const GLHW_PERMEDIA2: glHardwareType_t = 4;
    pub const GLHW_3DFX_2D3D: glHardwareType_t = 1;
    pub const RT_BEAM: refEntityType_t = 3;
    pub const GLDRV_ICD: glDriverType_t = 0;
    pub const RT_LIGHTNING: refEntityType_t = 6;
    pub const GLHW_GENERIC: glHardwareType_t = 0;
    pub const GLHW_RIVA128: glHardwareType_t = 2;
    pub const TC_S3TC: textureCompression_t = 1;
    pub const GLDRV_STANDALONE: glDriverType_t = 1;
    pub const GLDRV_VOODOO: glDriverType_t = 2;
    pub const TC_NONE: textureCompression_t = 0;
    use libc;
}
mod keycodes_h {
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawRect, UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen,
        UI_KeyEvent, UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale,
        UI_ProportionalStringWidth, UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor,
        UI_Shutdown,
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
    pub const K_WORLD_92: unnamed_0 = 325;
    pub const K_WORLD_42: unnamed = 275;
    pub const K_WORLD_45: unnamed_0 = 278;
    pub const K_WORLD_44: unnamed_0 = 277;
    pub const K_JOY17: unnamed = 201;
    pub const K_KP_ENTER: unnamed = 169;
    pub const K_HELP: unnamed = 332;
    pub type unnamed_1 = libc::c_uint;
    pub const K_WORLD_62: unnamed_0 = 295;
    pub const K_WORLD_87: unnamed = 320;
    pub const K_WORLD_68: unnamed = 301;
    pub const K_JOY7: unnamed_0 = 191;
    pub const K_CTRL: unnamed = 137;
    pub const K_F4: unnamed = 148;
    pub const K_PAD0_RIGHTSTICK_LEFT: unnamed_0 = 359;
    pub const K_KP_HOME: unnamed_0 = 160;
    pub const K_WORLD_29: unnamed_0 = 262;
    pub const K_WORLD_65: unnamed_0 = 298;
    pub const K_SHIFT: unnamed_0 = 138;
    pub const K_F9: unnamed_0 = 153;
    pub const K_PAD0_LEFTSTICK_RIGHT: unnamed_0 = 356;
    pub const K_WORLD_27: unnamed_1 = 260;
    pub const K_WORLD_41: unnamed = 274;
    pub const K_WORLD_2: unnamed_0 = 235;
    pub const K_JOY14: unnamed_0 = 198;
    pub const K_JOY11: unnamed_1 = 195;
    pub const K_PAD0_LEFTSTICK_DOWN: unnamed_0 = 358;
    pub const K_MENU: unnamed_0 = 337;
    pub const K_WORLD_59: unnamed_0 = 292;
    pub const K_PGUP: unnamed_0 = 142;
    pub const K_WORLD_55: unnamed_0 = 288;
    pub const K_KP_DOWNARROW: unnamed_0 = 167;
    pub const K_KP_MINUS: unnamed_0 = 173;
    pub const K_JOY13: unnamed_1 = 197;
    pub const K_MOUSE2: unnamed_0 = 179;
    pub const K_AUX3: unnamed = 219;
    pub const K_WORLD_26: unnamed_0 = 259;
    pub const K_PAD0_RIGHTSTICK_UP: unnamed = 361;
    pub const K_JOY4: unnamed_0 = 188;
    pub const K_WORLD_10: unnamed_0 = 243;
    pub const K_WORLD_1: unnamed_0 = 234;
    pub const K_WORLD_75: unnamed_0 = 308;
    pub const K_PRINT: unnamed_0 = 333;
    pub const K_TAB: unnamed_1 = 9;
    pub const K_WORLD_37: unnamed_0 = 270;
    pub const K_JOY22: unnamed = 206;
    pub const K_JOY16: unnamed_0 = 200;
    pub const K_KP_DEL: unnamed_0 = 171;
    pub const K_WORLD_11: unnamed = 244;
    pub const K_PAD0_RIGHTTRIGGER: unnamed_0 = 364;
    pub const K_KP_INS: unnamed = 170;
    pub const K_WORLD_58: unnamed = 291;
    pub const K_WORLD_79: unnamed_0 = 312;
    pub const K_WORLD_94: unnamed_0 = 327;
    pub const K_JOY12: unnamed_0 = 196;
    pub const K_WORLD_71: unnamed_0 = 304;
    pub const K_JOY31: unnamed_0 = 215;
    pub const K_JOY28: unnamed_1 = 212;
    pub const K_JOY32: unnamed_0 = 216;
    pub const K_AUX2: unnamed_0 = 218;
    pub const K_WORLD_56: unnamed_0 = 289;
    pub const K_JOY21: unnamed_0 = 205;
    pub type unnamed = libc::c_uint;
    pub const K_JOY24: unnamed_0 = 208;
    pub const K_WORLD_36: unnamed_0 = 269;
    pub const K_PAD0_BACK: unnamed = 344;
    pub const K_JOY18: unnamed = 202;
    pub const K_WORLD_8: unnamed_0 = 241;
    pub const K_AUX8: unnamed_0 = 224;
    pub const K_UNDO: unnamed_0 = 339;
    pub const K_AUX7: unnamed = 223;
    pub const K_DEL: unnamed = 140;
    pub const K_KP_UPARROW: unnamed_0 = 161;
    pub const K_WORLD_63: unnamed_0 = 296;
    pub const K_F3: unnamed_0 = 147;
    pub const K_WORLD_74: unnamed_0 = 307;
    pub const K_AUX5: unnamed_0 = 221;
    pub const K_PAD0_RIGHTSTICK_RIGHT: unnamed_0 = 360;
    pub const K_KP_SLASH: unnamed_0 = 172;
    pub const K_KP_PGDN: unnamed_0 = 168;
    pub const K_WORLD_22: unnamed = 255;
    pub const K_PAD0_GUIDE: unnamed_0 = 345;
    pub const K_PGDN: unnamed_0 = 141;
    pub const K_PAUSE: unnamed = 131;
    pub const K_RIGHTARROW: unnamed_0 = 135;
    pub const K_WORLD_20: unnamed_0 = 253;
    pub const K_JOY27: unnamed_1 = 211;
    pub const K_WORLD_53: unnamed = 286;
    pub const K_EURO: unnamed_0 = 338;
    pub const K_DOWNARROW: unnamed = 133;
    pub const K_F11: unnamed_0 = 155;
    pub const K_WORLD_91: unnamed_0 = 324;
    pub const K_WORLD_86: unnamed_0 = 319;
    pub const K_JOY6: unnamed_0 = 190;
    pub const K_AUX16: unnamed_0 = 232;
    pub const K_CONSOLE: unnamed = 365;
    pub const K_WORLD_31: unnamed = 264;
    pub const K_MWHEELUP: unnamed_0 = 184;
    pub const K_AUX6: unnamed_0 = 222;
    pub const K_KP_EQUALS: unnamed_1 = 177;
    pub const K_WORLD_47: unnamed = 280;
    pub const K_WORLD_57: unnamed = 290;
    pub const K_WORLD_95: unnamed_0 = 328;
    pub const K_PAD0_A: unnamed = 340;
    pub const K_WORLD_40: unnamed_0 = 273;
    pub const K_WORLD_16: unnamed_0 = 249;
    pub const K_MWHEELDOWN: unnamed = 183;
    pub const K_WORLD_85: unnamed_0 = 318;
    pub const K_WORLD_67: unnamed_0 = 300;
    pub const K_AUX14: unnamed = 230;
    pub const K_WORLD_76: unnamed_1 = 309;
    pub const K_PAD0_X: unnamed = 342;
    pub const K_PAD0_DPAD_UP: unnamed_0 = 351;
    pub const K_WORLD_6: unnamed_0 = 239;
    pub const K_KP_PGUP: unnamed_0 = 162;
    pub const K_KP_LEFTARROW: unnamed_0 = 163;
    pub const K_ESCAPE: unnamed_0 = 27;
    pub const K_WORLD_35: unnamed_0 = 268;
    pub const K_WORLD_48: unnamed_0 = 281;
    pub const K_WORLD_69: unnamed = 302;
    pub const K_UPARROW: unnamed_0 = 132;
    pub const K_JOY19: unnamed_0 = 203;
    pub const K_F2: unnamed = 146;
    pub const K_WORLD_73: unnamed_1 = 306;
    pub const MAX_KEYS: unnamed_0 = 366;
    pub const K_WORLD_82: unnamed = 315;
    pub const K_JOY30: unnamed_0 = 214;
    pub const K_MOUSE4: unnamed = 181;
    pub const K_WORLD_24: unnamed_0 = 257;
    pub const K_BREAK: unnamed = 336;
    pub const K_KP_NUMLOCK: unnamed_0 = 175;
    pub const K_PAD0_B: unnamed_0 = 341;
    pub const K_WORLD_89: unnamed_0 = 322;
    pub const K_F15: unnamed_0 = 159;
    pub const K_WORLD_33: unnamed_0 = 266;
    pub const K_WORLD_7: unnamed = 240;
    pub type unnamed_0 = libc::c_uint;
    pub const K_PAD0_LEFTSTICK_LEFT: unnamed_0 = 355;
    pub const K_END: unnamed = 144;
    pub const K_AUX4: unnamed_0 = 220;
    pub const K_WORLD_70: unnamed_0 = 303;
    pub const K_AUX10: unnamed = 226;
    pub const K_PAD0_LEFTSTICK_CLICK: unnamed_1 = 347;
    pub const K_JOY8: unnamed = 192;
    pub const K_HOME: unnamed_0 = 143;
    pub const K_KP_STAR: unnamed_0 = 176;
    pub const K_MOUSE5: unnamed_0 = 182;
    pub const K_WORLD_5: unnamed_1 = 238;
    pub const K_MOUSE1: unnamed_0 = 178;
    pub const K_JOY10: unnamed = 194;
    pub const K_WORLD_25: unnamed_1 = 258;
    pub const K_COMMAND: unnamed_0 = 128;
    pub const K_WORLD_78: unnamed = 311;
    pub const K_AUX11: unnamed_0 = 227;
    pub const K_MODE: unnamed_0 = 331;
    pub const K_WORLD_13: unnamed_1 = 246;
    pub const K_WORLD_28: unnamed = 261;
    pub const K_F6: unnamed = 150;
    pub const K_JOY15: unnamed_0 = 199;
    pub const K_COMPOSE: unnamed_1 = 330;
    pub const K_WORLD_80: unnamed_0 = 313;
    pub const K_AUX12: unnamed_0 = 228;
    pub const K_WORLD_21: unnamed = 254;
    pub const K_JOY26: unnamed_0 = 210;
    pub const K_WORLD_64: unnamed_0 = 297;
    pub const K_WORLD_84: unnamed_1 = 317;
    pub const K_WORLD_43: unnamed_1 = 276;
    pub const K_PAD0_LEFTSTICK_UP: unnamed = 357;
    pub const K_PAD0_LEFTSHOULDER: unnamed_0 = 349;
    pub const K_WORLD_50: unnamed = 283;
    pub const K_KP_5: unnamed_0 = 164;
    pub const K_ENTER: unnamed_0 = 13;
    pub const K_KP_END: unnamed_0 = 166;
    pub const K_WORLD_30: unnamed_1 = 263;
    pub const K_WORLD_23: unnamed = 256;
    pub const K_BACKSPACE: unnamed_0 = 127;
    pub const K_F1: unnamed_0 = 145;
    pub const K_WORLD_81: unnamed_0 = 314;
    pub const K_CAPSLOCK: unnamed = 129;
    pub const K_INS: unnamed_0 = 139;
    pub const K_WORLD_90: unnamed_0 = 323;
    pub const K_WORLD_4: unnamed_0 = 237;
    pub const K_WORLD_49: unnamed_0 = 282;
    pub const K_SCROLLOCK: unnamed_0 = 335;
    pub const K_KP_PLUS: unnamed_0 = 174;
    pub const K_AUX13: unnamed = 229;
    pub const K_WORLD_72: unnamed = 305;
    pub const K_WORLD_18: unnamed = 251;
    pub const K_WORLD_19: unnamed_0 = 252;
    pub const K_JOY3: unnamed = 187;
    pub const K_PAD0_DPAD_RIGHT: unnamed_0 = 354;
    pub const K_PAD0_RIGHTSTICK_CLICK: unnamed_0 = 348;
    pub const K_PAD0_START: unnamed = 346;
    pub const K_JOY2: unnamed_0 = 186;
    pub const K_WORLD_60: unnamed_0 = 293;
    pub const K_WORLD_0: unnamed_0 = 233;
    pub const K_JOY1: unnamed_0 = 185;
    pub const K_PAD0_LEFTTRIGGER: unnamed_0 = 363;
    pub const K_WORLD_61: unnamed_0 = 294;
    pub const K_WORLD_32: unnamed_0 = 265;
    pub const K_WORLD_14: unnamed = 247;
    pub const K_WORLD_34: unnamed_0 = 267;
    pub const K_PAD0_RIGHTSHOULDER: unnamed = 350;
    pub const K_WORLD_52: unnamed = 285;
    pub const K_JOY29: unnamed_0 = 213;
    pub const K_SYSREQ: unnamed_0 = 334;
    pub const K_JOY23: unnamed_0 = 207;
    pub const K_JOY5: unnamed = 189;
    pub const K_LEFTARROW: unnamed_0 = 134;
    pub const K_PAD0_DPAD_DOWN: unnamed = 352;
    pub const K_WORLD_93: unnamed = 326;
    pub const K_WORLD_77: unnamed = 310;
    pub const K_KP_RIGHTARROW: unnamed_0 = 165;
    pub const K_WORLD_88: unnamed_0 = 321;
    pub const K_AUX9: unnamed_0 = 225;
    pub const K_F12: unnamed_0 = 156;
    pub const K_PAD0_Y: unnamed_0 = 343;
    pub const K_WORLD_17: unnamed_0 = 250;
    pub const K_WORLD_39: unnamed = 272;
    pub const K_ALT: unnamed_0 = 136;
    pub const K_WORLD_15: unnamed_0 = 248;
    pub const K_F13: unnamed_0 = 157;
    pub const K_WORLD_83: unnamed_0 = 316;
    pub const K_WORLD_38: unnamed_0 = 271;
    pub const K_WORLD_46: unnamed = 279;
    pub const K_MOUSE3: unnamed_0 = 180;
    pub const K_PAD0_DPAD_LEFT: unnamed_0 = 353;
    pub const K_SPACE: unnamed_0 = 32;
    pub const K_POWER: unnamed_0 = 130;
    pub const K_JOY9: unnamed = 193;
    pub const K_WORLD_3: unnamed = 236;
    pub const K_WORLD_66: unnamed_0 = 299;
    pub const K_F5: unnamed = 149;
    pub const K_WORLD_12: unnamed_0 = 245;
    pub const K_JOY25: unnamed_0 = 209;
    pub const K_WORLD_54: unnamed_0 = 287;
    pub const K_PAD0_RIGHTSTICK_DOWN: unnamed = 362;
    pub const K_F10: unnamed_0 = 154;
    pub const K_JOY20: unnamed_1 = 204;
    pub const K_AUX15: unnamed_0 = 231;
    pub const K_SUPER: unnamed_0 = 329;
    pub const K_WORLD_51: unnamed_0 = 284;
    pub const K_F14: unnamed_0 = 158;
    pub const K_AUX1: unnamed = 217;
    pub const K_WORLD_9: unnamed_0 = 242;
    pub const K_F7: unnamed_1 = 151;
    pub const K_F8: unnamed_1 = 152;
    use libc;
}
mod stdlib {
    use stddef_h::size_t;
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawRect, UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen,
        UI_KeyEvent, UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale,
        UI_ProportionalStringWidth, UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor,
        UI_Shutdown,
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
        pub fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;

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
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

    }
    extern "C" {
        #[no_mangle]
        pub fn sin(_: libc::c_double) -> libc::c_double;
    }
    extern "C" {
        #[no_mangle]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
    }
    extern "C" {

        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;

        #[no_mangle]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

    }
    extern "C" {
        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
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
    pub type __compar_fn_t =
        Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
    extern "C" {
        #[no_mangle]
        pub fn atof(__nptr: *const libc::c_char) -> libc::c_double;

        #[no_mangle]
        pub fn rand() -> libc::c_int;
    }
    extern "C" {

        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
    extern "C" {
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    }
    use libc;
}
mod stddef_h {
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawRect, UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen,
        UI_KeyEvent, UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale,
        UI_ProportionalStringWidth, UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor,
        UI_Shutdown,
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
mod bg_public_h {
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawRect, UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen,
        UI_KeyEvent, UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale,
        UI_ProportionalStringWidth, UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor,
        UI_Shutdown,
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
    pub const TEAM_NUM_TEAMS: unnamed_1 = 4;
    pub const IT_HOLDABLE: itemType_t = 6;
    pub const TEAM_BLUE: unnamed_2 = 2;
    pub const IT_HEALTH: itemType_t = 4;
    pub const TEAM_FREE: unnamed_1 = 0;
    pub const TORSO_NEGATIVE: unnamed_0 = 30;
    pub const BOTH_DEAD1: unnamed_0 = 1;
    pub type unnamed_2 = libc::c_uint;
    pub const IT_POWERUP: itemType_t = 5;
    pub type animation_t = animation_s;
    pub const IT_TEAM: itemType_t = 8;
    pub const TEAM_SPECTATOR: unnamed_1 = 3;
    pub const TORSO_ATTACK2: unnamed_0 = 8;
    pub const LEGS_JUMP: unnamed_0 = 18;

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
    pub const LEGS_WALK: unnamed_0 = 14;
    pub const LEGS_WALKCR: unnamed_1 = 13;
    pub const LEGS_BACKWALK: unnamed_1 = 33;
    pub const WP_GRAPPLING_HOOK: weapon_t = 10;
    pub const LEGS_BACKCR: unnamed_0 = 32;
    pub const TORSO_DROP: unnamed_0 = 9;
    pub const TORSO_FOLLOWME: unnamed_0 = 28;
    pub const FLAG_STAND2RUN: unnamed_1 = 36;
    pub const FLAG_RUN: unnamed_0 = 34;
    pub const TORSO_PATROL: unnamed_0 = 27;
    pub const WP_GRENADE_LAUNCHER: weapon_t = 4;
    pub const LEGS_LAND: unnamed_0 = 19;
    pub const TORSO_AFFIRMATIVE: unnamed_1 = 29;
    pub const WP_SHOTGUN: weapon_t = 3;
    pub const GT_SINGLE_PLAYER: unnamed_0 = 2;
    extern "C" {
        // included in both the game dll and the client
        #[no_mangle]
        pub static mut bg_itemlist: [gitem_t; 0];
    }
    pub const WP_PLASMAGUN: weapon_t = 8;
    pub const FLAG_STAND: unnamed_0 = 35;
    pub const TEAM_RED: unnamed_2 = 1;
    pub const BOTH_DEATH1: unnamed_0 = 0;
    pub const WP_MACHINEGUN: weapon_t = 2;
    pub const IT_AMMO: itemType_t = 2;
    pub const LEGS_SWIM: unnamed_0 = 17;
    pub const GT_TOURNAMENT: unnamed_1 = 1;
    pub const TORSO_STAND2: unnamed_0 = 12;
    pub const GT_FFA: unnamed_1 = 0;
    pub const WP_NONE: weapon_t = 0;
    pub const BOTH_DEATH3: unnamed_0 = 4;
    pub const WP_LIGHTNING: weapon_t = 6;
    pub const BOTH_DEATH2: unnamed_0 = 2;
    pub const MAX_TOTALANIMATIONS: unnamed_0 = 37;
    pub type gitem_t = gitem_s;
    pub const TORSO_GUARDBASE: unnamed_0 = 26;
    pub type weapon_t = libc::c_uint;
    pub const GT_OBELISK: unnamed_0 = 6;
    pub const WP_NUM_WEAPONS: weapon_t = 11;
    pub const LEGS_RUN: unnamed_1 = 15;
    pub const IT_WEAPON: itemType_t = 1;
    pub const WP_BFG: weapon_t = 9;
    pub const WP_ROCKET_LAUNCHER: weapon_t = 5;
    pub const GT_MAX_GAME_TYPE: unnamed_0 = 8;
    pub const LEGS_IDLE: unnamed_1 = 22;
    pub const TORSO_STAND: unnamed_0 = 11;
    pub const BOTH_DEAD3: unnamed_0 = 5;
    pub const LEGS_LANDB: unnamed_0 = 21;
    pub const TORSO_RAISE: unnamed_1 = 10;
    pub const GT_HARVESTER: unnamed_1 = 7;
    pub const IT_ARMOR: itemType_t = 3;
    pub const LEGS_IDLECR: unnamed_1 = 23;
    pub type unnamed_1 = libc::c_uint;
    pub const LEGS_BACK: unnamed_0 = 16;
    pub type unnamed_0 = libc::c_uint;
    pub const BOTH_DEAD2: unnamed_0 = 3;
    pub const IT_PERSISTANT_POWERUP: itemType_t = 7;
    pub const TORSO_GESTURE: unnamed_1 = 6;

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
    pub type itemType_t = libc::c_uint;
    pub const GT_TEAM: unnamed_0 = 3;
    pub const IT_BAD: itemType_t = 0;
    pub const LEGS_TURN: unnamed_0 = 24;
    pub const GT_1FCTF: unnamed_1 = 5;
    pub const WP_RAILGUN: weapon_t = 7;
    pub const MAX_ANIMATIONS: unnamed_0 = 31;
    pub const TORSO_ATTACK: unnamed_0 = 7;
    pub const TORSO_GETFLAG: unnamed_0 = 25;
    pub const GT_CTF: unnamed_1 = 4;
    pub const WP_GAUNTLET: weapon_t = 1;
    pub const LEGS_JUMPB: unnamed_0 = 20;
    use libc;
}
mod ui_local_h {
    use bg_public_h::{animation_t, weapon_t};
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
        UI_DrawRect, UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen,
        UI_KeyEvent, UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale,
        UI_ProportionalStringWidth, UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor,
        UI_Shutdown,
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
        pub fn trap_MemoryRemaining() -> libc::c_int;

    }
    pub type unnamed = libc::c_uint;
    extern "C" {

        #[no_mangle]
        pub fn trap_R_RenderScene(fd: *const refdef_t);
        #[no_mangle]
        pub fn trap_R_AddLightToScene(
            org: *const vec_t,
            intensity: libc::c_float,
            r: libc::c_float,
            g: libc::c_float,
            b: libc::c_float,
        );

        #[no_mangle]
        pub fn trap_R_AddRefEntityToScene(re: *const refEntity_t);
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
        pub fn trap_R_ClearScene();

        #[no_mangle]
        pub fn trap_R_RegisterSkin(name: *const libc::c_char) -> qhandle_t;

    }
    extern "C" {

        //
        // ui_syscalls.c
        //

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

    }
    pub type unnamed_2 = libc::c_uint;
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
    pub type unnamed_0 = libc::c_uint;
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
        #[no_mangle]
        pub fn trap_Key_SetCatcher(catcher: libc::c_int);

    }
    extern "C" {

        #[no_mangle]
        pub fn trap_Cvar_Reset(name: *const libc::c_char);

    }
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
    extern "C" {

        //
        // ui_syscalls.c
        //
        #[no_mangle]
        pub fn trap_Print(string: *const libc::c_char);

    }
    pub const AWARD_EXCELLENT: unnamed_0 = 2;
    extern "C" {

        //
        // ui_spArena.c
        //

        #[no_mangle]
        pub fn trap_S_StartLocalSound(sfx: sfxHandle_t, channelNum: libc::c_int);

    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menuaction_s {
        pub generic: menucommon_s,
    }
    pub const AWARD_IMPRESSIVE: unnamed = 1;
    extern "C" {

        #[no_mangle]
        pub fn trap_VerifyCDKey(key: *const libc::c_char, chksum: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub fn trap_GetCDKey(buf: *mut libc::c_char, buflen: libc::c_int);

        #[no_mangle]
        pub fn trap_SetCDKey(buf: *mut libc::c_char);

        #[no_mangle]
        pub fn trap_Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
    }
    extern "C" {

        #[no_mangle]
        pub fn trap_FS_GetFileList(
            path: *const libc::c_char,
            extension: *const libc::c_char,
            listbuf: *mut libc::c_char,
            bufsize: libc::c_int,
        ) -> libc::c_int;

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

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menuradiobutton_s {
        pub generic: menucommon_s,
        pub curvalue: libc::c_int,
    }
    extern "C" {

        #[no_mangle]
        pub fn trap_S_RegisterSound(
            sample: *const libc::c_char,
            compressed: qboolean,
        ) -> sfxHandle_t;

        #[no_mangle]
        pub fn trap_R_SetColor(rgba: *const libc::c_float);

    }
    pub const AWARD_GAUNTLET: unnamed = 3;
    extern "C" {
        #[no_mangle]
        pub fn trap_Cvar_Register(
            vmCvar: *mut vmCvar_t,
            varName: *const libc::c_char,
            defaultValue: *const libc::c_char,
            flags: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_Cvar_Update(vmCvar: *mut vmCvar_t);
    //
    // ui_connect.c
    //

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

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menuslider_s {
        pub generic: menucommon_s,
        pub minvalue: libc::c_float,
        pub maxvalue: libc::c_float,
        pub curvalue: libc::c_float,
        pub range: libc::c_float,
    }
    pub const AWARD_PERFECT: unnamed_0 = 5;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct menufield_s {
        pub generic: menucommon_s,
        pub field: mfield_t,
    }
    extern "C" {

        //
        // ui_video.c
        //

        #[no_mangle]
        pub fn trap_Cvar_SetValue(var_name: *const libc::c_char, value: libc::c_float);

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
    pub type menuframework_s = _tag_menuframework;

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

        #[no_mangle]
        pub fn trap_R_RegisterShaderNoMip(name: *const libc::c_char) -> qhandle_t;
    }
    pub const AWARD_FRAGS: unnamed_2 = 4;

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
    extern "C" {

        #[no_mangle]
        pub fn trap_Cvar_VariableValue(var_name: *const libc::c_char) -> libc::c_float;
        #[no_mangle]
        pub fn trap_Cvar_VariableStringBuffer(
            var_name: *const libc::c_char,
            buffer: *mut libc::c_char,
            bufsize: libc::c_int,
        );

        #[no_mangle]
        pub fn trap_GetConfigString(
            index: libc::c_int,
            buff: *mut libc::c_char,
            buffsize: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_GetClientState(state: *mut uiClientState_t);

        #[no_mangle]
        pub fn trap_Cmd_ExecuteText(exec_when: libc::c_int, text: *const libc::c_char);
    }

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
    pub const AWARD_ACCURACY: unnamed_2 = 0;
    use libc;
}
mod ui_public_h {
    use q_shared_h::connstate_t;
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawRect, UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen,
        UI_KeyEvent, UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale,
        UI_ProportionalStringWidth, UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor,
        UI_Shutdown,
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
    pub const UI_SET_ACTIVE_MENU: unnamed = 7;
    pub const UI_HASUNIQUECDKEY: unnamed = 10;
    pub const UI_MOUSE_EVENT: unnamed = 4;
    pub const UI_REFRESH: unnamed = 5;
    pub const UI_CONSOLE_COMMAND: unnamed = 8;
    pub const UIMENU_INGAME: uiMenuCommand_t = 2;
    pub const UI_KEY_EVENT: unnamed = 3;
    pub const UI_SHUTDOWN: unnamed = 2;
    pub type uiMenuCommand_t = libc::c_uint;
    pub const UI_INIT: unnamed = 1;
    pub const UIMENU_BAD_CD_KEY: uiMenuCommand_t = 4;
    pub const UI_DRAW_CONNECT_SCREEN: unnamed = 9;
    pub const UI_GETAPIVERSION: unnamed = 0;
    pub const UIMENU_NONE: uiMenuCommand_t = 0;
    pub const UIMENU_TEAM: uiMenuCommand_t = 5;
    pub const UIMENU_NEED_CD: uiMenuCommand_t = 3;
    pub const UIMENU_POSTGAME: uiMenuCommand_t = 6;
    pub type unnamed = libc::c_uint;
    pub const UI_IS_FULLSCREEN: unnamed = 6;
    pub const UIMENU_MAIN: uiMenuCommand_t = 1;

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
    use libc;
}
mod q_shared_h {
    use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
    use ui_atoms::{
        uis, UI_AdjustFrom640, UI_Argv, UI_ClampCvar, UI_ConsoleCommand, UI_CursorInRect,
        UI_Cvar_VariableString, UI_DrawBannerString, UI_DrawChar, UI_DrawHandlePic,
        UI_DrawNamedPic, UI_DrawProportionalString, UI_DrawProportionalString_AutoWrapped,
        UI_DrawRect, UI_DrawString, UI_FillRect, UI_ForceMenuOff, UI_Init, UI_IsFullscreen,
        UI_KeyEvent, UI_MouseEvent, UI_PopMenu, UI_ProportionalSizeScale,
        UI_ProportionalStringWidth, UI_PushMenu, UI_Refresh, UI_SetActiveMenu, UI_SetColor,
        UI_Shutdown,
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

        /*
        // if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
        // or write a mail to the ioq3 mailing list.
        #else
          #define Q_ftol(v) ((long) (v))
          #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
          #define Q_SnapVector(vec) \
            do\
            {\
                vec3_t *temp = (vec);\
                \
                Q_round((*temp)[0]);\
                Q_round((*temp)[1]);\
                Q_round((*temp)[2]);\
            } while(0)
        #endif
        */
        #[no_mangle]
        pub fn Q_fabs(f: libc::c_float) -> libc::c_float;
        #[no_mangle]
        pub fn AnglesToAxis(angles: *const vec_t, axis: *mut vec3_t);
        #[no_mangle]
        pub fn AxisClear(axis: *mut vec3_t);
        #[no_mangle]
        pub fn AngleMod(a: libc::c_float) -> libc::c_float;
        #[no_mangle]
        pub fn AngleSubtract(a1: libc::c_float, a2: libc::c_float) -> libc::c_float;
        #[no_mangle]
        pub fn AnglesSubtract(v1: *mut vec_t, v2: *mut vec_t, v3: *mut vec_t);
        // perpendicular vector could be replaced by this
        //int	PlaneTypeForNormal (vec3_t normal);
        #[no_mangle]
        pub fn MatrixMultiply(
            in1: *mut [libc::c_float; 3],
            in2: *mut [libc::c_float; 3],
            out: *mut [libc::c_float; 3],
        );
        #[no_mangle]
        pub fn AngleVectors(
            angles: *const vec_t,
            forward: *mut vec_t,
            right: *mut vec_t,
            up: *mut vec_t,
        );

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

    }
    extern "C" {

        #[no_mangle]
        pub static mut vec3_origin: vec3_t;
    //=============================================

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
    pub type fileHandle_t = libc::c_int;
    pub type clipHandle_t = libc::c_int;
    extern "C" {
        #[no_mangle]
        pub fn Q_islower(c: libc::c_int) -> libc::c_int;

        #[no_mangle]
        pub fn Q_isalpha(c: libc::c_int) -> libc::c_int;
    }
    pub const FS_READ: fsMode_t = 0;
    pub const CHAN_ITEM: unnamed = 4;
    pub type cvarHandle_t = libc::c_int;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub type byte = libc::c_uchar;
    pub type fsMode_t = libc::c_uint;
    pub const FS_WRITE: fsMode_t = 1;
    pub const CA_DISCONNECTED: connstate_t = 1;
    pub const CHAN_BODY: unnamed = 5;
    pub type unnamed_0 = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct orientation_t {
        pub origin: vec3_t,
        pub axis: [vec3_t; 3],
    }
    extern "C" {
        #[no_mangle]
        pub static mut colorYellow: vec4_t;

        #[no_mangle]
        pub static mut colorMdGrey: vec4_t;
        //=============================================
        #[no_mangle]
        pub fn Q_isprint(c: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn Q_isupper(c: libc::c_int) -> libc::c_int;
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
    pub const CA_CHALLENGING: connstate_t = 4;
    pub const CA_CONNECTED: connstate_t = 5;
    extern "C" {
        #[no_mangle]
        pub fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;

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

    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vmCvar_t {
        pub handle: cvarHandle_t,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub string: [libc::c_char; 256],
    }
    pub const FS_APPEND: fsMode_t = 2;
    extern "C" {
        // all drawing is done to a 640*480 virtual screen size
        // and will be automatically scaled to the real resolution

        //=============================================

        #[no_mangle]
        pub fn COM_ParseExt(
            data_p: *mut *mut libc::c_char,
            allowLineBreak: qboolean,
        ) -> *mut libc::c_char;

    // portable case insensitive compare

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

    //=============================================
    //
    // key / value info strings
    //

    }
    pub const CHAN_LOCAL_SOUND: unnamed_0 = 6;
    extern "C" {
        // all drawing is done to a 640*480 virtual screen size
        // and will be automatically scaled to the real resolution
        #[no_mangle]
        pub static mut colorBlack: vec4_t;

        #[no_mangle]
        pub fn Q_IsColorString(p: *const libc::c_char) -> qboolean;
        #[no_mangle]
        pub static mut g_color_table: [vec4_t; 8];
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

        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
    pub type unnamed = libc::c_uint;
    pub const CA_PRIMED: connstate_t = 7;
    pub const CHAN_AUTO: unnamed = 0;
    extern "C" {
        #[no_mangle]
        pub static mut colorRed: vec4_t;
        #[no_mangle]
        pub fn COM_StripExtension(
            in_0: *const libc::c_char,
            out: *mut libc::c_char,
            destsize: libc::c_int,
        );

        // portable case insensitive compare

        #[no_mangle]
        pub fn Q_stricmpn(
            s1: *const libc::c_char,
            s2: *const libc::c_char,
            n: libc::c_int,
        ) -> libc::c_int;

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
    pub const CHAN_VOICE: unnamed = 3;
    pub const CA_LOADING: connstate_t = 6;
    pub const CHAN_LOCAL: unnamed = 1;
    pub const EXEC_APPEND: unnamed = 2;
    pub type vec4_t = [vec_t; 4];
    pub const CA_CINEMATIC: connstate_t = 9;
    extern "C" {
        #[no_mangle]
        pub static mut colorWhite: vec4_t;
    // portable case insensitive compare

    // removes color sequences from string

    }
    pub const CHAN_WEAPON: unnamed = 2;
    extern "C" {
        // portable case insensitive compare

        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int, src: *const libc::c_char);
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
    pub type vec3_t = [vec_t; 3];
    extern "C" {
        //=============================================
        #[no_mangle]
        pub fn Com_Clamp(
            min: libc::c_float,
            max: libc::c_float,
            value: libc::c_float,
        ) -> libc::c_float;
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
        #[no_mangle]
        pub fn Info_SetValueForKey(
            s: *mut libc::c_char,
            key: *const libc::c_char,
            value: *const libc::c_char,
        );
    }
    pub type connstate_t = libc::c_uint;
    pub const CA_AUTHORIZING: connstate_t = 2;
    pub type qboolean = libc::c_uint;
    pub const EXEC_NOW: unnamed = 0;
    pub const CA_UNINITIALIZED: connstate_t = 0;
    pub const qtrue: qboolean = 1;
    pub type qhandle_t = libc::c_int;
    pub const CA_ACTIVE: connstate_t = 8;
    pub type sfxHandle_t = libc::c_int;
    pub const CA_CONNECTING: connstate_t = 3;
    pub type vec_t = libc::c_float;
    pub const EXEC_INSERT: unnamed = 1;
    pub const CHAN_ANNOUNCER: unnamed_0 = 7;
    pub const qfalse: qboolean = 0;
    use libc;
}
