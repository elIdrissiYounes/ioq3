use libc;
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t, Com_sprintf,
    Info_ValueForKey, Q_CleanStr, Q_stricmp, Q_strncpyz, Q_strupr, CHAN_ANNOUNCER, CHAN_AUTO,
    CHAN_BODY, CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE, CHAN_WEAPON,
};
use stdlib::{atoi, memset, sin, strcpy, strrchr};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menutext_s, trap_Cvar_Set,
    trap_Cvar_SetValue, trap_Cvar_VariableStringBuffer, trap_Cvar_VariableValue,
    trap_Key_SetCatcher, trap_R_RegisterShaderNoMip, trap_R_SetColor, trap_S_RegisterSound,
    trap_S_StartLocalSound, uiStatic_t, unnamed_0, AWARD_ACCURACY, AWARD_EXCELLENT, AWARD_FRAGS,
    AWARD_GAUNTLET, AWARD_IMPRESSIVE, AWARD_PERFECT,
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
// ui_spLevel.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_SPLevelMenu_Cache() {
    let mut n: libc::c_int = 0;
    trap_R_RegisterShaderNoMip(b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/maps_selected\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/narrow_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/narrow_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/level_complete1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/level_complete2\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/level_complete3\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/level_complete4\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/level_complete5\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/reset_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/reset_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/skirmish_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/skirmish_1\x00" as *const u8 as *const libc::c_char);
    n = 0i32;
    while n < 6i32 {
        trap_R_RegisterShaderNoMip(*ui_medalPicNames.as_mut_ptr().offset(n as isize));
        levelMenuInfo.awardSounds[n as usize] =
            trap_S_RegisterSound(*ui_medalSounds.as_mut_ptr().offset(n as isize), qfalse);
        n += 1
    }
    levelMenuInfo.levelSelectedPic = trap_R_RegisterShaderNoMip(
        b"menu/art/maps_selected\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelFocusPic =
        trap_R_RegisterShaderNoMip(b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char);
    levelMenuInfo.levelCompletePic[0usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete1\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelCompletePic[1usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete2\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelCompletePic[2usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete3\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelCompletePic[3usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete4\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelCompletePic[4usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete5\x00" as *const u8 as *const libc::c_char,
    );
}
static mut levelMenuInfo: levelMenuInfo_t = levelMenuInfo_t {
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
    item_banner: menutext_s {
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
    item_leftarrow: menubitmap_s {
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
    item_maps: [menubitmap_s {
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
    }; 4],
    item_rightarrow: menubitmap_s {
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
    item_player: menubitmap_s {
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
    item_awards: [menubitmap_s {
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
    }; 6],
    item_back: menubitmap_s {
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
    item_reset: menubitmap_s {
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
    item_custom: menubitmap_s {
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
    item_next: menubitmap_s {
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
    item_null: menubitmap_s {
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
    reinit: qfalse,
    selectedArenaInfo: 0 as *const libc::c_char,
    numMaps: 0,
    levelPicNames: [[0; 64]; 4],
    levelNames: [[0; 16]; 4],
    levelScores: [0; 4],
    levelScoresSkill: [0; 4],
    levelSelectedPic: 0,
    levelFocusPic: 0,
    levelCompletePic: [0; 5],
    playerModel: [0; 64],
    playerPicName: [0; 64],
    awardLevels: [0; 6],
    awardSounds: [0; 6],
    numBots: 0,
    botPics: [0; 7],
    botNames: [[0; 10]; 7],
};
#[no_mangle]
pub unsafe extern "C" fn UI_SPLevelMenu() {
    let mut level: libc::c_int = 0;
    let mut trainingLevel: libc::c_int = 0;
    let mut arenaInfo: *const libc::c_char = 0 as *const libc::c_char;
    trainingTier = -1i32;
    arenaInfo = UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const libc::c_char);
    if !arenaInfo.is_null() {
        minTier = trainingTier;
        trainingLevel = atoi(Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const libc::c_char,
        ))
    } else {
        minTier = 0i32;
        trainingLevel = -2i32
    }
    finalTier = UI_GetNumSPTiers();
    arenaInfo = UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const libc::c_char);
    if !arenaInfo.is_null() {
        maxTier = finalTier
    } else {
        maxTier = finalTier - 1i32;
        if maxTier < minTier {
            maxTier = minTier
        }
    }
    level = UI_GetCurrentGame();
    if level == -1i32 {
        level = UI_GetNumSPArenas() - 1i32;
        if maxTier == finalTier {
            level += 1
        }
    }
    if level == trainingLevel {
        currentSet = -1i32;
        currentGame = 0i32
    } else {
        currentSet = level / 4i32;
        currentGame = level % 4i32
    }
    UI_SPLevelMenu_Init();
    UI_PushMenu(&mut levelMenuInfo.menu);
    Menu_SetCursorToItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_next as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_SPLevelMenu_Init
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_Init() {
    let mut skill: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    skill = trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const libc::c_char)
        as libc::c_int;
    if skill < 1i32 || skill > 5i32 {
        trap_Cvar_Set(
            b"g_spSkill\x00" as *const u8 as *const libc::c_char,
            b"2\x00" as *const u8 as *const libc::c_char,
        );
    }
    memset(
        &mut levelMenuInfo as *mut levelMenuInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<levelMenuInfo_t>() as libc::c_ulong,
    );
    levelMenuInfo.menu.fullscreen = qtrue;
    levelMenuInfo.menu.wrapAround = qtrue;
    levelMenuInfo.menu.draw = Some(UI_SPLevelMenu_MenuDraw);
    UI_SPLevelMenu_Cache();
    levelMenuInfo.item_banner.generic.type_0 = 10i32;
    levelMenuInfo.item_banner.generic.x = 320i32;
    levelMenuInfo.item_banner.generic.y = 16i32;
    levelMenuInfo.item_banner.string =
        b"CHOOSE LEVEL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_banner.color = color_red.as_mut_ptr();
    levelMenuInfo.item_banner.style = 0x1i32;
    levelMenuInfo.item_leftarrow.generic.type_0 = 6i32;
    levelMenuInfo.item_leftarrow.generic.name =
        b"menu/art/narrow_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_leftarrow.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    levelMenuInfo.item_leftarrow.generic.x = 18i32;
    levelMenuInfo.item_leftarrow.generic.y = 64i32;
    levelMenuInfo.item_leftarrow.generic.callback = Some(UI_SPLevelMenu_LeftArrowEvent);
    levelMenuInfo.item_leftarrow.generic.id = 10i32;
    levelMenuInfo.item_leftarrow.width = 16i32;
    levelMenuInfo.item_leftarrow.height = 114i32;
    levelMenuInfo.item_leftarrow.focuspic =
        b"menu/art/narrow_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_maps[0usize].generic.type_0 = 6i32;
    levelMenuInfo.item_maps[0usize].generic.name = levelMenuInfo.levelPicNames[0usize].as_mut_ptr();
    levelMenuInfo.item_maps[0usize].generic.flags = 0x4i32 as libc::c_uint;
    levelMenuInfo.item_maps[0usize].generic.x = 46i32;
    levelMenuInfo.item_maps[0usize].generic.y = 64i32;
    levelMenuInfo.item_maps[0usize].generic.id = 11i32;
    levelMenuInfo.item_maps[0usize].generic.callback = Some(UI_SPLevelMenu_LevelEvent);
    levelMenuInfo.item_maps[0usize].width = 128i32;
    levelMenuInfo.item_maps[0usize].height = 96i32;
    levelMenuInfo.item_maps[1usize].generic.type_0 = 6i32;
    levelMenuInfo.item_maps[1usize].generic.name = levelMenuInfo.levelPicNames[1usize].as_mut_ptr();
    levelMenuInfo.item_maps[1usize].generic.flags = 0x4i32 as libc::c_uint;
    levelMenuInfo.item_maps[1usize].generic.x = 186i32;
    levelMenuInfo.item_maps[1usize].generic.y = 64i32;
    levelMenuInfo.item_maps[1usize].generic.id = 12i32;
    levelMenuInfo.item_maps[1usize].generic.callback = Some(UI_SPLevelMenu_LevelEvent);
    levelMenuInfo.item_maps[1usize].width = 128i32;
    levelMenuInfo.item_maps[1usize].height = 96i32;
    levelMenuInfo.item_maps[2usize].generic.type_0 = 6i32;
    levelMenuInfo.item_maps[2usize].generic.name = levelMenuInfo.levelPicNames[2usize].as_mut_ptr();
    levelMenuInfo.item_maps[2usize].generic.flags = 0x4i32 as libc::c_uint;
    levelMenuInfo.item_maps[2usize].generic.x = 326i32;
    levelMenuInfo.item_maps[2usize].generic.y = 64i32;
    levelMenuInfo.item_maps[2usize].generic.id = 13i32;
    levelMenuInfo.item_maps[2usize].generic.callback = Some(UI_SPLevelMenu_LevelEvent);
    levelMenuInfo.item_maps[2usize].width = 128i32;
    levelMenuInfo.item_maps[2usize].height = 96i32;
    levelMenuInfo.item_maps[3usize].generic.type_0 = 6i32;
    levelMenuInfo.item_maps[3usize].generic.name = levelMenuInfo.levelPicNames[3usize].as_mut_ptr();
    levelMenuInfo.item_maps[3usize].generic.flags = 0x4i32 as libc::c_uint;
    levelMenuInfo.item_maps[3usize].generic.x = 466i32;
    levelMenuInfo.item_maps[3usize].generic.y = 64i32;
    levelMenuInfo.item_maps[3usize].generic.id = 14i32;
    levelMenuInfo.item_maps[3usize].generic.callback = Some(UI_SPLevelMenu_LevelEvent);
    levelMenuInfo.item_maps[3usize].width = 128i32;
    levelMenuInfo.item_maps[3usize].height = 96i32;
    levelMenuInfo.item_rightarrow.generic.type_0 = 6i32;
    levelMenuInfo.item_rightarrow.generic.name =
        b"menu/art/narrow_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_rightarrow.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    levelMenuInfo.item_rightarrow.generic.x = 606i32;
    levelMenuInfo.item_rightarrow.generic.y = 64i32;
    levelMenuInfo.item_rightarrow.generic.callback = Some(UI_SPLevelMenu_RightArrowEvent);
    levelMenuInfo.item_rightarrow.generic.id = 15i32;
    levelMenuInfo.item_rightarrow.width = -16i32;
    levelMenuInfo.item_rightarrow.height = 114i32;
    levelMenuInfo.item_rightarrow.focuspic =
        b"menu/art/narrow_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        levelMenuInfo.playerModel.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    PlayerIcon(
        levelMenuInfo.playerModel.as_mut_ptr(),
        levelMenuInfo.playerPicName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    levelMenuInfo.item_player.generic.type_0 = 6i32;
    levelMenuInfo.item_player.generic.name = levelMenuInfo.playerPicName.as_mut_ptr();
    levelMenuInfo.item_player.generic.flags = 0x4i32 as libc::c_uint | 0x800i32 as libc::c_uint;
    levelMenuInfo.item_player.generic.x = 288i32;
    levelMenuInfo.item_player.generic.y = 314i32 + 26i32;
    levelMenuInfo.item_player.generic.id = 16i32;
    levelMenuInfo.item_player.generic.callback = Some(UI_SPLevelMenu_PlayerEvent);
    levelMenuInfo.item_player.width = 64i32;
    levelMenuInfo.item_player.height = 64i32;
    n = 0i32;
    while n < 6i32 {
        levelMenuInfo.awardLevels[n as usize] = UI_GetAwardLevel(n);
        n += 1
    }
    levelMenuInfo.awardLevels[AWARD_FRAGS as libc::c_int as usize] =
        100i32 * (levelMenuInfo.awardLevels[AWARD_FRAGS as libc::c_int as usize] / 100i32);
    y = 314i32 + 26i32;
    count = 0i32;
    n = 0i32;
    while n < 6i32 {
        if 0 != levelMenuInfo.awardLevels[n as usize] {
            if 0 != count & 1i32 {
                x = 224i32 - (count - 1i32) / 2i32 * (48i32 + 16i32)
            } else {
                x = 368i32 + count / 2i32 * (48i32 + 16i32)
            }
            levelMenuInfo.item_awards[count as usize].generic.type_0 = 6i32;
            levelMenuInfo.item_awards[count as usize].generic.name =
                *ui_medalPicNames.as_mut_ptr().offset(n as isize);
            levelMenuInfo.item_awards[count as usize].generic.flags =
                0x4i32 as libc::c_uint | 0x100000i32 as libc::c_uint | 0x800i32 as libc::c_uint;
            levelMenuInfo.item_awards[count as usize].generic.x = x;
            levelMenuInfo.item_awards[count as usize].generic.y = y;
            levelMenuInfo.item_awards[count as usize].generic.id = 17i32 + n;
            levelMenuInfo.item_awards[count as usize].generic.callback =
                Some(UI_SPLevelMenu_AwardEvent);
            levelMenuInfo.item_awards[count as usize].width = 48i32;
            levelMenuInfo.item_awards[count as usize].height = 48i32;
            count += 1
        }
        n += 1
    }
    levelMenuInfo.item_back.generic.type_0 = 6i32;
    levelMenuInfo.item_back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    levelMenuInfo.item_back.generic.x = 0i32;
    levelMenuInfo.item_back.generic.y = 480i32 - 64i32;
    levelMenuInfo.item_back.generic.callback = Some(UI_SPLevelMenu_BackEvent);
    levelMenuInfo.item_back.generic.id = 23i32;
    levelMenuInfo.item_back.width = 128i32;
    levelMenuInfo.item_back.height = 64i32;
    levelMenuInfo.item_back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_reset.generic.type_0 = 6i32;
    levelMenuInfo.item_reset.generic.name =
        b"menu/art/reset_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_reset.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    levelMenuInfo.item_reset.generic.x = 170i32;
    levelMenuInfo.item_reset.generic.y = 480i32 - 64i32;
    levelMenuInfo.item_reset.generic.callback = Some(UI_SPLevelMenu_ResetEvent);
    levelMenuInfo.item_reset.generic.id = 24i32;
    levelMenuInfo.item_reset.width = 128i32;
    levelMenuInfo.item_reset.height = 64i32;
    levelMenuInfo.item_reset.focuspic =
        b"menu/art/reset_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_custom.generic.type_0 = 6i32;
    levelMenuInfo.item_custom.generic.name =
        b"menu/art/skirmish_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_custom.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    levelMenuInfo.item_custom.generic.x = 342i32;
    levelMenuInfo.item_custom.generic.y = 480i32 - 64i32;
    levelMenuInfo.item_custom.generic.callback = Some(UI_SPLevelMenu_CustomEvent);
    levelMenuInfo.item_custom.generic.id = 25i32;
    levelMenuInfo.item_custom.width = 128i32;
    levelMenuInfo.item_custom.height = 64i32;
    levelMenuInfo.item_custom.focuspic =
        b"menu/art/skirmish_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_next.generic.type_0 = 6i32;
    levelMenuInfo.item_next.generic.name =
        b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_next.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    levelMenuInfo.item_next.generic.x = 640i32;
    levelMenuInfo.item_next.generic.y = 480i32 - 64i32;
    levelMenuInfo.item_next.generic.callback = Some(UI_SPLevelMenu_NextEvent);
    levelMenuInfo.item_next.generic.id = 26i32;
    levelMenuInfo.item_next.width = 128i32;
    levelMenuInfo.item_next.height = 64i32;
    levelMenuInfo.item_next.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_null.generic.type_0 = 6i32;
    levelMenuInfo.item_null.generic.flags =
        0x4i32 as libc::c_uint | 0x800i32 as libc::c_uint | 0x100000i32 as libc::c_uint;
    levelMenuInfo.item_null.generic.x = 0i32;
    levelMenuInfo.item_null.generic.y = 0i32;
    levelMenuInfo.item_null.width = 640i32;
    levelMenuInfo.item_null.height = 480i32;
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_leftarrow as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_maps[0usize] as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_maps[1usize] as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_maps[2usize] as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_maps[3usize] as *mut menubitmap_s as *mut libc::c_void,
    );
    levelMenuInfo.item_maps[0usize].generic.bottom += 18i32;
    levelMenuInfo.item_maps[1usize].generic.bottom += 18i32;
    levelMenuInfo.item_maps[2usize].generic.bottom += 18i32;
    levelMenuInfo.item_maps[3usize].generic.bottom += 18i32;
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_rightarrow as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_player as *mut menubitmap_s as *mut libc::c_void,
    );
    n = 0i32;
    while n < count {
        Menu_AddItem(
            &mut levelMenuInfo.menu,
            &mut levelMenuInfo.item_awards[n as usize] as *mut menubitmap_s as *mut libc::c_void,
        );
        n += 1
    }
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_reset as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_custom as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_next as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_null as *mut menubitmap_s as *mut libc::c_void,
    );
    trap_Cvar_VariableStringBuffer(
        b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != *buf.as_mut_ptr() {
        n = atoi(buf.as_mut_ptr());
        selectedArenaSet = n / 4i32;
        selectedArena = n % 4i32
    } else {
        selectedArenaSet = currentSet;
        selectedArena = currentGame
    }
    UI_SPLevelMenu_SetMenuItems();
}
unsafe extern "C" fn UI_SPLevelMenu_SetMenuItems() {
    let mut n: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut arenaInfo: *const libc::c_char = 0 as *const libc::c_char;
    if selectedArenaSet > currentSet {
        selectedArena = -1i32
    } else if selectedArena == -1i32 {
        selectedArena = 0i32
    }
    if selectedArenaSet == trainingTier || selectedArenaSet == finalTier {
        selectedArena = 0i32
    }
    if selectedArena != -1i32 {
        trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
            (selectedArenaSet * 4i32 + selectedArena) as libc::c_float,
        );
    }
    if selectedArenaSet == trainingTier {
        arenaInfo = UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const libc::c_char);
        level = atoi(Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const libc::c_char,
        ));
        UI_SPLevelMenu_SetMenuArena(0i32, level, arenaInfo);
        levelMenuInfo.selectedArenaInfo = arenaInfo;
        levelMenuInfo.item_maps[0usize].generic.x = 256i32;
        Bitmap_Init(&mut levelMenuInfo.item_maps[0usize]);
        levelMenuInfo.item_maps[0usize].generic.bottom += 32i32;
        levelMenuInfo.numMaps = 1i32;
        levelMenuInfo.item_maps[1usize].generic.flags |= 0x4000i32 as libc::c_uint;
        levelMenuInfo.item_maps[2usize].generic.flags |= 0x4000i32 as libc::c_uint;
        levelMenuInfo.item_maps[3usize].generic.flags |= 0x4000i32 as libc::c_uint;
        levelMenuInfo.levelPicNames[1usize][0usize] = 0i32 as libc::c_char;
        levelMenuInfo.levelPicNames[2usize][0usize] = 0i32 as libc::c_char;
        levelMenuInfo.levelPicNames[3usize][0usize] = 0i32 as libc::c_char;
        levelMenuInfo.item_maps[1usize].shader = 0i32;
        levelMenuInfo.item_maps[2usize].shader = 0i32;
        levelMenuInfo.item_maps[3usize].shader = 0i32
    } else if selectedArenaSet == finalTier {
        arenaInfo = UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const libc::c_char);
        level = atoi(Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const libc::c_char,
        ));
        UI_SPLevelMenu_SetMenuArena(0i32, level, arenaInfo);
        levelMenuInfo.selectedArenaInfo = arenaInfo;
        levelMenuInfo.item_maps[0usize].generic.x = 256i32;
        Bitmap_Init(&mut levelMenuInfo.item_maps[0usize]);
        levelMenuInfo.item_maps[0usize].generic.bottom += 32i32;
        levelMenuInfo.numMaps = 1i32;
        levelMenuInfo.item_maps[1usize].generic.flags |= 0x4000i32 as libc::c_uint;
        levelMenuInfo.item_maps[2usize].generic.flags |= 0x4000i32 as libc::c_uint;
        levelMenuInfo.item_maps[3usize].generic.flags |= 0x4000i32 as libc::c_uint;
        levelMenuInfo.levelPicNames[1usize][0usize] = 0i32 as libc::c_char;
        levelMenuInfo.levelPicNames[2usize][0usize] = 0i32 as libc::c_char;
        levelMenuInfo.levelPicNames[3usize][0usize] = 0i32 as libc::c_char;
        levelMenuInfo.item_maps[1usize].shader = 0i32;
        levelMenuInfo.item_maps[2usize].shader = 0i32;
        levelMenuInfo.item_maps[3usize].shader = 0i32
    } else {
        levelMenuInfo.item_maps[0usize].generic.x = 46i32;
        Bitmap_Init(&mut levelMenuInfo.item_maps[0usize]);
        levelMenuInfo.item_maps[0usize].generic.bottom += 18i32;
        levelMenuInfo.numMaps = 4i32;
        n = 0i32;
        while n < 4i32 {
            level = selectedArenaSet * 4i32 + n;
            arenaInfo = UI_GetArenaInfoByNumber(level);
            UI_SPLevelMenu_SetMenuArena(n, level, arenaInfo);
            n += 1
        }
        if selectedArena != -1i32 {
            levelMenuInfo.selectedArenaInfo =
                UI_GetArenaInfoByNumber(selectedArenaSet * 4i32 + selectedArena)
        }
    }
    if selectedArenaSet == minTier {
        levelMenuInfo.item_leftarrow.generic.flags |=
            0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint
    } else {
        levelMenuInfo.item_leftarrow.generic.flags &=
            !(0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint)
    }
    if selectedArenaSet == maxTier {
        levelMenuInfo.item_rightarrow.generic.flags |=
            0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint
    } else {
        levelMenuInfo.item_rightarrow.generic.flags &=
            !(0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint)
    }
    UI_SPLevelMenu_SetBots();
}
/*
=================
UI_SPLevelMenu_SetBots
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_SetBots() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut botInfo: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bots: [libc::c_char; 1024] = [0; 1024];
    levelMenuInfo.numBots = 0i32;
    if selectedArenaSet > currentSet {
        return;
    }
    Q_strncpyz(
        bots.as_mut_ptr(),
        Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"bots\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    p = &mut bots[0usize] as *mut libc::c_char;
    while 0 != *p as libc::c_int && levelMenuInfo.numBots < 7i32 {
        while 0 != *p as libc::c_int && *p as libc::c_int == ' ' as i32 {
            p = p.offset(1isize)
        }
        if 0 == *p {
            break;
        }
        bot = p;
        while 0 != *p as libc::c_int && *p as libc::c_int != ' ' as i32 {
            p = p.offset(1isize)
        }
        if 0 != *p {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = 0i32 as libc::c_char
        }
        botInfo = UI_GetBotInfoByName(bot);
        if botInfo.is_null() {
            botInfo = UI_GetBotInfoByNumber(levelMenuInfo.numBots)
        }
        if !botInfo.is_null() {
            levelMenuInfo.botPics[levelMenuInfo.numBots as usize] = PlayerIconHandle(
                Info_ValueForKey(botInfo, b"model\x00" as *const u8 as *const libc::c_char),
            );
            Q_strncpyz(
                levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr(),
                Info_ValueForKey(botInfo, b"name\x00" as *const u8 as *const libc::c_char),
                10i32,
            );
        } else {
            levelMenuInfo.botPics[levelMenuInfo.numBots as usize] = 0i32;
            Q_strncpyz(
                levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr(),
                bot,
                10i32,
            );
        }
        Q_CleanStr(levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr());
        levelMenuInfo.numBots += 1
    }
}
/*
=================
PlayerIconhandle
=================
*/
unsafe extern "C" fn PlayerIconHandle(mut modelAndSkin: *const libc::c_char) -> qhandle_t {
    let mut iconName: [libc::c_char; 64] = [0; 64];
    PlayerIcon(
        modelAndSkin,
        iconName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    return trap_R_RegisterShaderNoMip(iconName.as_mut_ptr());
}
/*
=================
PlayerIcon
=================
*/
unsafe extern "C" fn PlayerIcon(
    mut modelAndSkin: *const libc::c_char,
    mut iconName: *mut libc::c_char,
    mut iconNameMaxSize: libc::c_int,
) {
    let mut skin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut model: [libc::c_char; 64] = [0; 64];
    Q_strncpyz(
        model.as_mut_ptr(),
        modelAndSkin,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    skin = strrchr(model.as_mut_ptr(), '/' as i32);
    if !skin.is_null() {
        let fresh1 = skin;
        skin = skin.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char
    } else {
        skin = b"default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Com_sprintf(
        iconName,
        iconNameMaxSize,
        b"models/players/%s/icon_%s.tga\x00" as *const u8 as *const libc::c_char,
        model.as_mut_ptr(),
        skin,
    );
    if 0 == trap_R_RegisterShaderNoMip(iconName)
        && Q_stricmp(skin, b"default\x00" as *const u8 as *const libc::c_char) != 0i32
    {
        Com_sprintf(
            iconName,
            iconNameMaxSize,
            b"models/players/%s/icon_default.tga\x00" as *const u8 as *const libc::c_char,
            model.as_mut_ptr(),
        );
    };
}
static mut currentSet: libc::c_int = 0;
static mut selectedArenaSet: libc::c_int = 0;
static mut maxTier: libc::c_int = 0;
static mut minTier: libc::c_int = 0;
static mut selectedArena: libc::c_int = 0;
/*
=================
UI_SPLevelMenu_SetMenuItems
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_SetMenuArena(
    mut n: libc::c_int,
    mut level: libc::c_int,
    mut arenaInfo: *const libc::c_char,
) {
    let mut map: [libc::c_char; 64] = [0; 64];
    Q_strncpyz(
        map.as_mut_ptr(),
        Info_ValueForKey(arenaInfo, b"map\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    Q_strncpyz(
        levelMenuInfo.levelNames[n as usize].as_mut_ptr(),
        map.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
    );
    Q_strupr(levelMenuInfo.levelNames[n as usize].as_mut_ptr());
    UI_GetBestScore(
        level,
        &mut levelMenuInfo.levelScores[n as usize],
        &mut levelMenuInfo.levelScoresSkill[n as usize],
    );
    if levelMenuInfo.levelScores[n as usize] > 8i32 {
        levelMenuInfo.levelScores[n as usize] = 8i32
    }
    Com_sprintf(
        levelMenuInfo.levelPicNames[n as usize].as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"levelshots/%s.tga\x00" as *const u8 as *const libc::c_char,
        map.as_mut_ptr(),
    );
    if 0 == trap_R_RegisterShaderNoMip(levelMenuInfo.levelPicNames[n as usize].as_mut_ptr()) {
        strcpy(
            levelMenuInfo.levelPicNames[n as usize].as_mut_ptr(),
            b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char,
        );
    }
    levelMenuInfo.item_maps[n as usize].shader = 0i32;
    if selectedArenaSet > currentSet {
        levelMenuInfo.item_maps[n as usize].generic.flags |= 0x2000i32 as libc::c_uint
    } else {
        levelMenuInfo.item_maps[n as usize].generic.flags &= !(0x2000i32 as libc::c_uint)
    }
    levelMenuInfo.item_maps[n as usize].generic.flags &= !(0x4000i32 as libc::c_uint);
}
static mut finalTier: libc::c_int = 0;
static mut trainingTier: libc::c_int = 0;
static mut currentGame: libc::c_int = 0;
/*
=================
UI_SPLevelMenu_NextEvent
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_NextEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3i32 {
        return;
    }
    if selectedArenaSet > currentSet {
        return;
    }
    if selectedArena == -1i32 {
        selectedArena = 0i32
    }
    UI_SPSkillMenu(levelMenuInfo.selectedArenaInfo);
}
/*
=================
UI_SPLevelMenu_CustomEvent
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_CustomEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3i32 {
        return;
    }
    UI_StartServerMenu(qfalse);
}
unsafe extern "C" fn UI_SPLevelMenu_ResetEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    UI_ConfirmMenu(
        b"RESET GAME?\x00" as *const u8 as *const libc::c_char,
        Some(UI_SPLevelMenu_ResetDraw),
        Some(UI_SPLevelMenu_ResetAction),
    );
}
unsafe extern "C" fn UI_SPLevelMenu_ResetAction(mut result: qboolean) {
    if 0 == result as u64 {
        return;
    }
    UI_NewGame();
    if !UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const libc::c_char).is_null() {
        trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
            -4i32 as libc::c_float,
        );
    } else {
        trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
            0i32 as libc::c_float,
        );
    }
    UI_PopMenu();
    UI_SPLevelMenu();
}
/*
=================
UI_SPLevelMenu_ResetEvent
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_ResetDraw() {
    UI_DrawProportionalString(
        640i32 / 2i32,
        356i32 + 27i32 * 0i32,
        b"WARNING: This resets all of the\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_yellow.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640i32 / 2i32,
        356i32 + 27i32 * 1i32,
        b"single player game variables.\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_yellow.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640i32 / 2i32,
        356i32 + 27i32 * 2i32,
        b"Do this only if you want to\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_yellow.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640i32 / 2i32,
        356i32 + 27i32 * 3i32,
        b"start over from the beginning.\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_yellow.as_mut_ptr(),
    );
}
/*
=================
UI_SPLevelMenu_BackEvent
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_BackEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3i32 {
        return;
    }
    if selectedArena == -1i32 {
        selectedArena = 0i32
    }
    UI_PopMenu();
}
/*
=================
UI_SPLevelMenu_AwardEvent
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_AwardEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    if notification != 3i32 {
        return;
    }
    n = (*(ptr as *mut menucommon_s)).id - 17i32;
    trap_S_StartLocalSound(
        levelMenuInfo.awardSounds[n as usize],
        CHAN_ANNOUNCER as libc::c_int,
    );
}
/*
=================
UI_SPLevelMenu_PlayerEvent
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_PlayerEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3i32 {
        return;
    }
    UI_PlayerSettingsMenu();
}
/*
=================
UI_SPLevelMenu_RightArrowEvent
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_RightArrowEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3i32 {
        return;
    }
    if selectedArenaSet == maxTier {
        return;
    }
    selectedArenaSet += 1;
    UI_SPLevelMenu_SetMenuItems();
}
/*
=================
UI_SPLevelMenu_LevelEvent
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_LevelEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3i32 {
        return;
    }
    if selectedArenaSet == trainingTier || selectedArenaSet == finalTier {
        return;
    }
    selectedArena = (*(ptr as *mut menucommon_s)).id - 11i32;
    levelMenuInfo.selectedArenaInfo =
        UI_GetArenaInfoByNumber(selectedArenaSet * 4i32 + selectedArena);
    UI_SPLevelMenu_SetBots();
    trap_Cvar_SetValue(
        b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
        (selectedArenaSet * 4i32 + selectedArena) as libc::c_float,
    );
}
/*
=================
UI_SPLevelMenu_LeftArrowEvent
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_LeftArrowEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3i32 {
        return;
    }
    if selectedArenaSet == minTier {
        return;
    }
    selectedArenaSet -= 1;
    UI_SPLevelMenu_SetMenuItems();
}
/*
=================
UI_SPLevelMenu_MenuDraw
=================
*/
unsafe extern "C" fn UI_SPLevelMenu_MenuDraw() {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut color: vec4_t = [0.; 4];
    let mut level: libc::c_int = 0;
    //	int				fraglimit;
    let mut pad: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 64] = [0; 64];
    if 0 != levelMenuInfo.reinit as u64 {
        UI_PopMenu();
        UI_SPLevelMenu();
        return;
    }
    trap_Cvar_VariableStringBuffer(
        b"name\x00" as *const u8 as *const libc::c_char,
        string.as_mut_ptr(),
        32i32,
    );
    Q_CleanStr(string.as_mut_ptr());
    UI_DrawProportionalString(
        320i32,
        314i32,
        string.as_mut_ptr(),
        0x1i32 | 0x10i32,
        color_orange.as_mut_ptr(),
    );
    trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if Q_stricmp(buf.as_mut_ptr(), levelMenuInfo.playerModel.as_mut_ptr()) != 0i32 {
        Q_strncpyz(
            levelMenuInfo.playerModel.as_mut_ptr(),
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        PlayerIcon(
            levelMenuInfo.playerModel.as_mut_ptr(),
            levelMenuInfo.playerPicName.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        levelMenuInfo.item_player.shader = 0i32
    }
    Menu_Draw(&mut levelMenuInfo.menu);
    y = 314i32 + 26i32;
    i = 0i32;
    n = 0i32;
    while n < 6i32 {
        level = levelMenuInfo.awardLevels[n as usize];
        if level > 0i32 {
            if 0 != i & 1i32 {
                x = 224i32 - (i - 1i32) / 2i32 * (48i32 + 16i32)
            } else {
                x = 368i32 + i / 2i32 * (48i32 + 16i32)
            }
            i += 1;
            if !(level == 1i32) {
                if level >= 1000000i32 {
                    Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                        b"%im\x00" as *const u8 as *const libc::c_char,
                        level / 1000000i32,
                    );
                } else if level >= 1000i32 {
                    Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                        b"%ik\x00" as *const u8 as *const libc::c_char,
                        level / 1000i32,
                    );
                } else {
                    Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                        b"%i\x00" as *const u8 as *const libc::c_char,
                        level,
                    );
                }
                UI_DrawString(
                    x + 24i32,
                    y + 48i32,
                    string.as_mut_ptr(),
                    0x1i32,
                    color_yellow.as_mut_ptr(),
                );
            }
        }
        n += 1
    }
    UI_DrawProportionalString(
        18i32,
        38i32,
        va(
            b"Tier %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            selectedArenaSet + 1i32,
        ),
        0i32 | 0x10i32,
        color_orange.as_mut_ptr(),
    );
    n = 0i32;
    while n < levelMenuInfo.numMaps {
        x = levelMenuInfo.item_maps[n as usize].generic.x;
        y = levelMenuInfo.item_maps[n as usize].generic.y;
        UI_FillRect(
            x as libc::c_float,
            (y + 96i32) as libc::c_float,
            128i32 as libc::c_float,
            18i32 as libc::c_float,
            color_black.as_mut_ptr(),
        );
        n += 1
    }
    if selectedArenaSet > currentSet {
        UI_DrawProportionalString(
            320i32,
            216i32,
            b"ACCESS DENIED\x00" as *const u8 as *const libc::c_char,
            0x1i32 | 0x20i32,
            color_red.as_mut_ptr(),
        );
        return;
    }
    color[0usize] = color_white[0usize];
    color[1usize] = color_white[1usize];
    color[2usize] = color_white[2usize];
    color[3usize] = color_white[3usize];
    color[3usize] = (0.5f64 + 0.5f64 * sin((uis.realtime / 75i32) as libc::c_double)) as vec_t;
    n = 0i32;
    while n < levelMenuInfo.numMaps {
        x = levelMenuInfo.item_maps[n as usize].generic.x;
        y = levelMenuInfo.item_maps[n as usize].generic.y;
        UI_DrawString(
            x + 64i32,
            y + 96i32,
            levelMenuInfo.levelNames[n as usize].as_mut_ptr(),
            0x1i32 | 0x10i32,
            color_orange.as_mut_ptr(),
        );
        if levelMenuInfo.levelScores[n as usize] == 1i32 {
            UI_DrawHandlePic(
                x as libc::c_float,
                y as libc::c_float,
                128i32 as libc::c_float,
                96i32 as libc::c_float,
                levelMenuInfo.levelCompletePic
                    [(levelMenuInfo.levelScoresSkill[n as usize] - 1i32) as usize],
            );
        }
        if n == selectedArena {
            if Menu_ItemAtCursor(&mut levelMenuInfo.menu)
                == &mut levelMenuInfo.item_maps[n as usize] as *mut menubitmap_s
                    as *mut libc::c_void
            {
                trap_R_SetColor(color.as_mut_ptr());
            }
            UI_DrawHandlePic(
                (x - 1i32) as libc::c_float,
                (y - 1i32) as libc::c_float,
                130i32 as libc::c_float,
                (130i32 - 14i32) as libc::c_float,
                levelMenuInfo.levelSelectedPic,
            );
            trap_R_SetColor(0 as *const libc::c_float);
        } else if Menu_ItemAtCursor(&mut levelMenuInfo.menu)
            == &mut levelMenuInfo.item_maps[n as usize] as *mut menubitmap_s as *mut libc::c_void
        {
            trap_R_SetColor(color.as_mut_ptr());
            UI_DrawHandlePic(
                (x - 31i32) as libc::c_float,
                (y - 30i32) as libc::c_float,
                256i32 as libc::c_float,
                (256i32 - 27i32) as libc::c_float,
                levelMenuInfo.levelFocusPic,
            );
            trap_R_SetColor(0 as *const libc::c_float);
        }
        n += 1
    }
    y = 192i32;
    Q_strncpyz(
        buf.as_mut_ptr(),
        Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"map\x00" as *const u8 as *const libc::c_char,
        ),
        20i32,
    );
    Q_strupr(buf.as_mut_ptr());
    Com_sprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"longname\x00" as *const u8 as *const libc::c_char,
        ),
    );
    UI_DrawProportionalString(
        320i32,
        y,
        string.as_mut_ptr(),
        0x1i32 | 0x10i32,
        color_orange.as_mut_ptr(),
    );
    y += 24i32;
    pad = (7i32 - levelMenuInfo.numBots) * (64i32 + 26i32) / 2i32;
    n = 0i32;
    while n < levelMenuInfo.numBots {
        x = 18i32 + pad + (64i32 + 26i32) * n;
        if 0 != levelMenuInfo.botPics[n as usize] {
            UI_DrawHandlePic(
                x as libc::c_float,
                y as libc::c_float,
                64i32 as libc::c_float,
                64i32 as libc::c_float,
                levelMenuInfo.botPics[n as usize],
            );
        } else {
            UI_FillRect(
                x as libc::c_float,
                y as libc::c_float,
                64i32 as libc::c_float,
                64i32 as libc::c_float,
                color_black.as_mut_ptr(),
            );
            UI_DrawProportionalString(
                x + 22i32,
                y + 18i32,
                b"?\x00" as *const u8 as *const libc::c_char,
                0x20i32,
                color_orange.as_mut_ptr(),
            );
        }
        UI_DrawString(
            x,
            y + 64i32,
            levelMenuInfo.botNames[n as usize].as_mut_ptr(),
            0x10i32 | 0i32,
            color_orange.as_mut_ptr(),
        );
        n += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn UI_SPLevelMenu_f() {
    trap_Key_SetCatcher(0x2i32);
    uis.menusp = 0i32;
    UI_SPLevelMenu();
}
#[no_mangle]
pub unsafe extern "C" fn UI_SPLevelMenu_ReInit() {
    levelMenuInfo.reinit = qtrue;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct levelMenuInfo_t {
    pub menu: menuframework_s,
    pub item_banner: menutext_s,
    pub item_leftarrow: menubitmap_s,
    pub item_maps: [menubitmap_s; 4],
    pub item_rightarrow: menubitmap_s,
    pub item_player: menubitmap_s,
    pub item_awards: [menubitmap_s; 6],
    pub item_back: menubitmap_s,
    pub item_reset: menubitmap_s,
    pub item_custom: menubitmap_s,
    pub item_next: menubitmap_s,
    pub item_null: menubitmap_s,
    pub reinit: qboolean,
    pub selectedArenaInfo: *const libc::c_char,
    pub numMaps: libc::c_int,
    pub levelPicNames: [[libc::c_char; 64]; 4],
    pub levelNames: [[libc::c_char; 16]; 4],
    pub levelScores: [libc::c_int; 4],
    pub levelScoresSkill: [libc::c_int; 4],
    pub levelSelectedPic: qhandle_t,
    pub levelFocusPic: qhandle_t,
    pub levelCompletePic: [qhandle_t; 5],
    pub playerModel: [libc::c_char; 64],
    pub playerPicName: [libc::c_char; 64],
    pub awardLevels: [libc::c_int; 6],
    pub awardSounds: [sfxHandle_t; 6],
    pub numBots: libc::c_int,
    pub botPics: [qhandle_t; 7],
    pub botNames: [[libc::c_char; 10]; 7],
}
