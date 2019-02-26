#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use bg_misc::bg_itemlist;
use keycodes_h::{
    unnamed_1, K_ALT, K_AUX1, K_AUX10, K_AUX11, K_AUX12, K_AUX13, K_AUX14, K_AUX15, K_AUX16,
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
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, unnamed_0, va, vec4_t, vec_t,
    Com_sprintf, Info_ValueForKey, Q_CleanStr, Q_strncpyz, CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY,
    CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE, CHAN_WEAPON, EXEC_APPEND, EXEC_INSERT,
    EXEC_NOW,
};
use stdlib::{atoi, memset, strlen};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, trap_Cmd_ExecuteText,
    trap_Cvar_Set, trap_Cvar_SetValue, trap_Cvar_VariableValue, trap_GetConfigString,
    trap_Key_SetCatcher, trap_R_RegisterShaderNoMip, trap_S_RegisterSound, trap_S_StartLocalSound,
    uiStatic_t, unnamed_2, AWARD_ACCURACY, AWARD_EXCELLENT, AWARD_FRAGS, AWARD_GAUNTLET,
    AWARD_IMPRESSIVE, AWARD_PERFECT,
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
use ui_spskill::{UI_SPSkillMenu, UI_SPSkillMenu_Cache};
use ui_startserver::{
    ServerOptions_Cache, StartServer_Cache, UI_BotSelectMenu_Cache, UI_StartServerMenu,
};
use ui_team::{TeamMain_Cache, UI_TeamMainMenu};
use ui_teamorders::{UI_TeamOrdersMenu, UI_TeamOrdersMenu_f};
use ui_video::{DriverInfo_Cache, GraphicsOptions_Cache, UI_GraphicsOptionsMenu};
extern crate libc;

#[no_mangle]
pub static mut ui_medalNames: [*mut libc::c_char; 6] = [
    b"Accuracy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Impressive\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Excellent\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Frags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Perfect\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut ui_medalPicNames: [*mut libc::c_char; 6] = [
    b"menu/medals/medal_accuracy\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/medals/medal_impressive\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/medals/medal_excellent\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/medals/medal_gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/medals/medal_frags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/medals/medal_victory\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut ui_medalSounds: [*mut libc::c_char; 6] = [
    b"sound/feedback/accuracy.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sound/feedback/impressive_a.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sound/feedback/excellent_a.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sound/feedback/gauntlet.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sound/feedback/frags.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"sound/feedback/perfect.wav\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
//
// ui_spPostgame.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_SPPostgameMenu_Cache() {
    let mut n: libc::c_int = 0;
    let mut buildscript: qboolean = qfalse;
    buildscript =
        trap_Cvar_VariableValue(b"com_buildscript\x00" as *const u8 as *const libc::c_char)
            as qboolean;
    trap_R_RegisterShaderNoMip(b"menu/art/menu_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/menu_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/replay_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/replay_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/next_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/next_1\x00" as *const u8 as *const libc::c_char);
    n = 0i32;
    while n < 6i32 {
        trap_R_RegisterShaderNoMip(ui_medalPicNames[n as usize]);
        trap_S_RegisterSound(ui_medalSounds[n as usize], qfalse);
        n += 1
    }
    if 0 != buildscript as u64 {
        trap_S_RegisterSound(
            b"music/loss.wav\x00" as *const u8 as *const libc::c_char,
            qfalse,
        );
        trap_S_RegisterSound(
            b"music/win.wav\x00" as *const u8 as *const libc::c_char,
            qfalse,
        );
        trap_S_RegisterSound(
            b"sound/player/announce/youwin.wav\x00" as *const u8 as *const libc::c_char,
            qfalse,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_SPPostgameMenu_f() {
    let mut playerGameRank: libc::c_int = 0;
    let mut playerClientNum: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut oldFrags: libc::c_int = 0;
    let mut newFrags: libc::c_int = 0;
    let mut arena: *const libc::c_char = 0 as *const libc::c_char;
    let mut awardValues: [libc::c_int; 6] = [0; 6];
    let mut map: [libc::c_char; 64] = [0; 64];
    let mut info: [libc::c_char; 1024] = [0; 1024];
    memset(
        &mut postgameMenuInfo as *mut postgameMenuInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<postgameMenuInfo_t>() as libc::c_ulong,
    );
    trap_GetConfigString(
        1i32,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    postgameMenuInfo.serverId = atoi(Info_ValueForKey(
        info.as_mut_ptr(),
        b"sv_serverid\x00" as *const u8 as *const libc::c_char,
    ));
    trap_GetConfigString(
        0i32,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Q_strncpyz(
        map.as_mut_ptr(),
        Info_ValueForKey(
            info.as_mut_ptr(),
            b"mapname\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    arena = UI_GetArenaInfoByMap(map.as_mut_ptr());
    if arena.is_null() {
        return;
    }
    Q_strncpyz(
        arenainfo.as_mut_ptr(),
        arena,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    postgameMenuInfo.level = atoi(Info_ValueForKey(
        arenainfo.as_mut_ptr(),
        b"num\x00" as *const u8 as *const libc::c_char,
    ));
    postgameMenuInfo.numClients = atoi(UI_Argv(1i32));
    playerClientNum = atoi(UI_Argv(2i32));
    playerGameRank = 8i32;
    if postgameMenuInfo.numClients > 8i32 {
        postgameMenuInfo.numClients = 8i32
    }
    n = 0i32;
    while n < postgameMenuInfo.numClients {
        postgameMenuInfo.clientNums[n as usize] = atoi(UI_Argv(8i32 + n * 3i32 + 1i32));
        postgameMenuInfo.ranks[n as usize] = atoi(UI_Argv(8i32 + n * 3i32 + 2i32));
        postgameMenuInfo.scores[n as usize] = atoi(UI_Argv(8i32 + n * 3i32 + 3i32));
        if postgameMenuInfo.clientNums[n as usize] == playerClientNum {
            playerGameRank = (postgameMenuInfo.ranks[n as usize] & !0x4000i32) + 1i32
        }
        n += 1
    }
    UI_SetBestScore(postgameMenuInfo.level, playerGameRank);
    awardValues[AWARD_ACCURACY as libc::c_int as usize] = atoi(UI_Argv(3i32));
    awardValues[AWARD_IMPRESSIVE as libc::c_int as usize] = atoi(UI_Argv(4i32));
    awardValues[AWARD_EXCELLENT as libc::c_int as usize] = atoi(UI_Argv(5i32));
    awardValues[AWARD_GAUNTLET as libc::c_int as usize] = atoi(UI_Argv(6i32));
    awardValues[AWARD_FRAGS as libc::c_int as usize] = atoi(UI_Argv(7i32));
    awardValues[AWARD_PERFECT as libc::c_int as usize] = atoi(UI_Argv(8i32));
    postgameMenuInfo.numAwards = 0i32;
    if awardValues[AWARD_ACCURACY as libc::c_int as usize] >= 50i32 {
        UI_LogAwardData(AWARD_ACCURACY as libc::c_int, 1i32);
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            AWARD_ACCURACY as libc::c_int;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] =
            awardValues[AWARD_ACCURACY as libc::c_int as usize];
        postgameMenuInfo.numAwards += 1
    }
    if 0 != awardValues[AWARD_IMPRESSIVE as libc::c_int as usize] {
        UI_LogAwardData(
            AWARD_IMPRESSIVE as libc::c_int,
            awardValues[AWARD_IMPRESSIVE as libc::c_int as usize],
        );
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            AWARD_IMPRESSIVE as libc::c_int;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] =
            awardValues[AWARD_IMPRESSIVE as libc::c_int as usize];
        postgameMenuInfo.numAwards += 1
    }
    if 0 != awardValues[AWARD_EXCELLENT as libc::c_int as usize] {
        UI_LogAwardData(
            AWARD_EXCELLENT as libc::c_int,
            awardValues[AWARD_EXCELLENT as libc::c_int as usize],
        );
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            AWARD_EXCELLENT as libc::c_int;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] =
            awardValues[AWARD_EXCELLENT as libc::c_int as usize];
        postgameMenuInfo.numAwards += 1
    }
    if 0 != awardValues[AWARD_GAUNTLET as libc::c_int as usize] {
        UI_LogAwardData(
            AWARD_GAUNTLET as libc::c_int,
            awardValues[AWARD_GAUNTLET as libc::c_int as usize],
        );
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            AWARD_GAUNTLET as libc::c_int;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] =
            awardValues[AWARD_GAUNTLET as libc::c_int as usize];
        postgameMenuInfo.numAwards += 1
    }
    oldFrags = UI_GetAwardLevel(AWARD_FRAGS as libc::c_int) / 100i32;
    UI_LogAwardData(
        AWARD_FRAGS as libc::c_int,
        awardValues[AWARD_FRAGS as libc::c_int as usize],
    );
    newFrags = UI_GetAwardLevel(AWARD_FRAGS as libc::c_int) / 100i32;
    if newFrags > oldFrags {
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            AWARD_FRAGS as libc::c_int;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] = newFrags * 100i32;
        postgameMenuInfo.numAwards += 1
    }
    if 0 != awardValues[AWARD_PERFECT as libc::c_int as usize] {
        UI_LogAwardData(AWARD_PERFECT as libc::c_int, 1i32);
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            AWARD_PERFECT as libc::c_int;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] = 1i32;
        postgameMenuInfo.numAwards += 1
    }
    if playerGameRank == 1i32 {
        postgameMenuInfo.won = UI_TierCompleted(postgameMenuInfo.level)
    } else {
        postgameMenuInfo.won = -1i32
    }
    postgameMenuInfo.starttime = uis.realtime;
    postgameMenuInfo.scoreboardtime = uis.realtime;
    trap_Key_SetCatcher(0x2i32);
    uis.menusp = 0i32;
    UI_SPPostgameMenu_Init();
    UI_PushMenu(&mut postgameMenuInfo.menu);
    if playerGameRank == 1i32 {
        Menu_SetCursorToItem(
            &mut postgameMenuInfo.menu,
            &mut postgameMenuInfo.item_next as *mut menubitmap_s as *mut libc::c_void,
        );
    } else {
        Menu_SetCursorToItem(
            &mut postgameMenuInfo.menu,
            &mut postgameMenuInfo.item_again as *mut menubitmap_s as *mut libc::c_void,
        );
    }
    Prepname(0i32);
    Prepname(1i32);
    Prepname(2i32);
    if playerGameRank != 1i32 {
        postgameMenuInfo.winnerSound = trap_S_RegisterSound(
            va(
                b"sound/player/announce/%s_wins.wav\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                postgameMenuInfo.placeNames[0usize].as_mut_ptr(),
            ),
            qfalse,
        );
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            b"music music/loss\n\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        postgameMenuInfo.winnerSound = trap_S_RegisterSound(
            b"sound/player/announce/youwin.wav\x00" as *const u8 as *const libc::c_char,
            qfalse,
        );
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            b"music music/win\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    postgameMenuInfo.phase = 1i32;
    postgameMenuInfo.lastTier = UI_GetNumSPTiers();
    if !UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const libc::c_char).is_null() {
        postgameMenuInfo.lastTier += 1
    };
}
static mut postgameMenuInfo: postgameMenuInfo_t = postgameMenuInfo_t {
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
    item_again: menubitmap_s {
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
    item_menu: menubitmap_s {
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
    phase: 0,
    ignoreKeysTime: 0,
    starttime: 0,
    scoreboardtime: 0,
    serverId: 0,
    clientNums: [0; 8],
    ranks: [0; 8],
    scores: [0; 8],
    placeNames: [[0; 64]; 3],
    level: 0,
    numClients: 0,
    won: 0,
    numAwards: 0,
    awardsEarned: [0; 6],
    awardsLevels: [0; 6],
    playedSound: [qfalse; 6],
    lastTier: 0,
    winnerSound: 0,
};
unsafe extern "C" fn Prepname(mut index: libc::c_int) {
    let mut len: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut info: [libc::c_char; 1024] = [0; 1024];
    trap_GetConfigString(
        32i32 + 256i32 + 256i32 + postgameMenuInfo.clientNums[index as usize],
        info.as_mut_ptr(),
        1024i32,
    );
    Q_strncpyz(
        name.as_mut_ptr(),
        Info_ValueForKey(
            info.as_mut_ptr(),
            b"n\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    Q_CleanStr(name.as_mut_ptr());
    len = strlen(name.as_mut_ptr()) as libc::c_int;
    while 0 != len && UI_ProportionalStringWidth(name.as_mut_ptr()) > 256i32 {
        len -= 1;
        name[len as usize] = 0i32 as libc::c_char
    }
    Q_strncpyz(
        postgameMenuInfo.placeNames[index as usize].as_mut_ptr(),
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
}
/*
=================
UI_SPPostgameMenu_Init
=================
*/
unsafe extern "C" fn UI_SPPostgameMenu_Init() {
    postgameMenuInfo.menu.wrapAround = qtrue;
    postgameMenuInfo.menu.key = Some(UI_SPPostgameMenu_MenuKey);
    postgameMenuInfo.menu.draw = Some(UI_SPPostgameMenu_MenuDraw);
    postgameMenuInfo.ignoreKeysTime = uis.realtime + 1500i32;
    UI_SPPostgameMenu_Cache();
    postgameMenuInfo.item_menu.generic.type_0 = 6i32;
    postgameMenuInfo.item_menu.generic.name =
        b"menu/art/menu_0\x00" as *const u8 as *const libc::c_char;
    postgameMenuInfo.item_menu.generic.flags =
        0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    postgameMenuInfo.item_menu.generic.x = 0i32;
    postgameMenuInfo.item_menu.generic.y = 480i32 - 64i32;
    postgameMenuInfo.item_menu.generic.callback = Some(UI_SPPostgameMenu_MenuEvent);
    postgameMenuInfo.item_menu.generic.id = 12i32;
    postgameMenuInfo.item_menu.width = 128i32;
    postgameMenuInfo.item_menu.height = 64i32;
    postgameMenuInfo.item_menu.focuspic =
        b"menu/art/menu_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    postgameMenuInfo.item_again.generic.type_0 = 6i32;
    postgameMenuInfo.item_again.generic.name =
        b"menu/art/replay_0\x00" as *const u8 as *const libc::c_char;
    postgameMenuInfo.item_again.generic.flags =
        0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    postgameMenuInfo.item_again.generic.x = 320i32;
    postgameMenuInfo.item_again.generic.y = 480i32 - 64i32;
    postgameMenuInfo.item_again.generic.callback = Some(UI_SPPostgameMenu_AgainEvent);
    postgameMenuInfo.item_again.generic.id = 10i32;
    postgameMenuInfo.item_again.width = 128i32;
    postgameMenuInfo.item_again.height = 64i32;
    postgameMenuInfo.item_again.focuspic =
        b"menu/art/replay_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    postgameMenuInfo.item_next.generic.type_0 = 6i32;
    postgameMenuInfo.item_next.generic.name =
        b"menu/art/next_0\x00" as *const u8 as *const libc::c_char;
    postgameMenuInfo.item_next.generic.flags =
        0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    postgameMenuInfo.item_next.generic.x = 640i32;
    postgameMenuInfo.item_next.generic.y = 480i32 - 64i32;
    postgameMenuInfo.item_next.generic.callback = Some(UI_SPPostgameMenu_NextEvent);
    postgameMenuInfo.item_next.generic.id = 11i32;
    postgameMenuInfo.item_next.width = 128i32;
    postgameMenuInfo.item_next.height = 64i32;
    postgameMenuInfo.item_next.focuspic =
        b"menu/art/next_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut postgameMenuInfo.menu,
        &mut postgameMenuInfo.item_menu as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut postgameMenuInfo.menu,
        &mut postgameMenuInfo.item_again as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut postgameMenuInfo.menu,
        &mut postgameMenuInfo.item_next as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_SPPostgameMenu_NextEvent
=================
*/
unsafe extern "C" fn UI_SPPostgameMenu_NextEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    let mut currentSet: libc::c_int = 0;
    let mut levelSet: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut currentLevel: libc::c_int = 0;
    let mut arenaInfo: *const libc::c_char = 0 as *const libc::c_char;
    if event != 3i32 {
        return;
    }
    UI_PopMenu();
    if postgameMenuInfo.won == 0i32 {
        level = 0i32
    } else {
        level = postgameMenuInfo.level + 1i32
    }
    levelSet = level / 4i32;
    currentLevel = UI_GetCurrentGame();
    if currentLevel == -1i32 {
        currentLevel = postgameMenuInfo.level
    }
    currentSet = currentLevel / 4i32;
    if levelSet > currentSet || levelSet == UI_GetNumSPTiers() {
        level = currentLevel
    }
    arenaInfo = UI_GetArenaInfoByNumber(level);
    if arenaInfo.is_null() {
        return;
    }
    UI_SPArena_Start(arenaInfo);
}
/*
=================
UI_SPPostgameMenu_AgainEvent
=================
*/
unsafe extern "C" fn UI_SPPostgameMenu_AgainEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    UI_PopMenu();
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        b"map_restart 0\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
UI_SPPostgameMenu_MenuEvent
=================
*/
unsafe extern "C" fn UI_SPPostgameMenu_MenuEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    UI_PopMenu();
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        b"disconnect; levelselect\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
UI_SPPostgameMenu_MenuDraw
=================
*/
unsafe extern "C" fn UI_SPPostgameMenu_MenuDraw() {
    let mut timer: libc::c_int = 0;
    let mut serverId: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    trap_GetConfigString(
        1i32,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    serverId = atoi(Info_ValueForKey(
        info.as_mut_ptr(),
        b"sv_serverid\x00" as *const u8 as *const libc::c_char,
    ));
    if serverId != postgameMenuInfo.serverId {
        UI_PopMenu();
        return;
    }
    if postgameMenuInfo.numClients > 2i32 {
        UI_DrawProportionalString(
            510i32,
            480i32 - 64i32 - 27i32,
            postgameMenuInfo.placeNames[2usize].as_mut_ptr(),
            0x1i32,
            color_white.as_mut_ptr(),
        );
    }
    UI_DrawProportionalString(
        130i32,
        480i32 - 64i32 - 27i32,
        postgameMenuInfo.placeNames[1usize].as_mut_ptr(),
        0x1i32,
        color_white.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        320i32,
        480i32 - 64i32 - 2i32 * 27i32,
        postgameMenuInfo.placeNames[0usize].as_mut_ptr(),
        0x1i32,
        color_white.as_mut_ptr(),
    );
    if postgameMenuInfo.phase == 1i32 {
        timer = uis.realtime - postgameMenuInfo.starttime;
        if timer >= 1000i32 && 0 != postgameMenuInfo.winnerSound {
            trap_S_StartLocalSound(postgameMenuInfo.winnerSound, CHAN_ANNOUNCER as libc::c_int);
            postgameMenuInfo.winnerSound = 0i32
        }
        if timer < 5000i32 {
            return;
        }
        postgameMenuInfo.phase = 2i32;
        postgameMenuInfo.starttime = uis.realtime
    }
    if postgameMenuInfo.phase == 2i32 {
        timer = uis.realtime - postgameMenuInfo.starttime;
        if timer >= postgameMenuInfo.numAwards * 2000i32 {
            if timer < 5000i32 {
                return;
            }
            postgameMenuInfo.phase = 3i32;
            postgameMenuInfo.starttime = uis.realtime
        } else {
            UI_SPPostgameMenu_DrawAwardsPresentation(timer);
        }
    }
    if postgameMenuInfo.phase == 3i32 {
        if 0 != uis.demoversion as u64 {
            if postgameMenuInfo.won == 1i32 && 0 != UI_ShowTierVideo(8i32) as libc::c_uint {
                trap_Cvar_Set(
                    b"nextmap\x00" as *const u8 as *const libc::c_char,
                    b"\x00" as *const u8 as *const libc::c_char,
                );
                trap_Cmd_ExecuteText(
                    EXEC_APPEND as libc::c_int,
                    b"disconnect; cinematic demoEnd.RoQ\n\x00" as *const u8 as *const libc::c_char,
                );
                return;
            }
        } else if postgameMenuInfo.won > -1i32
            && 0 != UI_ShowTierVideo(postgameMenuInfo.won + 1i32) as libc::c_uint
        {
            if postgameMenuInfo.won == postgameMenuInfo.lastTier {
                trap_Cvar_Set(
                    b"nextmap\x00" as *const u8 as *const libc::c_char,
                    b"\x00" as *const u8 as *const libc::c_char,
                );
                trap_Cmd_ExecuteText(
                    EXEC_APPEND as libc::c_int,
                    b"disconnect; cinematic end.RoQ\n\x00" as *const u8 as *const libc::c_char,
                );
                return;
            }
            trap_Cvar_SetValue(
                b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
                (postgameMenuInfo.won * 4i32) as libc::c_float,
            );
            trap_Cvar_Set(
                b"nextmap\x00" as *const u8 as *const libc::c_char,
                b"levelselect\x00" as *const u8 as *const libc::c_char,
            );
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                va(
                    b"disconnect; cinematic tier%i.RoQ\n\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    postgameMenuInfo.won + 1i32,
                ),
            );
            return;
        }
        postgameMenuInfo.item_again.generic.flags &= !(0x4000i32 as libc::c_uint);
        postgameMenuInfo.item_next.generic.flags &= !(0x4000i32 as libc::c_uint);
        postgameMenuInfo.item_menu.generic.flags &= !(0x4000i32 as libc::c_uint);
        UI_SPPostgameMenu_DrawAwardsMedals(postgameMenuInfo.numAwards);
        Menu_Draw(&mut postgameMenuInfo.menu);
    }
    if 0. == trap_Cvar_VariableValue(b"ui_spScoreboard\x00" as *const u8 as *const libc::c_char) {
        return;
    }
    timer = uis.realtime - postgameMenuInfo.scoreboardtime;
    if postgameMenuInfo.numClients <= 3i32 {
        n = 0i32
    } else {
        n = timer / 1500i32 % (postgameMenuInfo.numClients + 2i32)
    }
    UI_SPPostgameMenu_MenuDrawScoreLine(n, 0i32);
    UI_SPPostgameMenu_MenuDrawScoreLine(n + 1i32, 0i32 + 16i32);
    UI_SPPostgameMenu_MenuDrawScoreLine(n + 2i32, 0i32 + 2i32 * 16i32);
}
/*
=================
UI_SPPostgameMenu_MenuDrawScoreLine
=================
*/
unsafe extern "C" fn UI_SPPostgameMenu_MenuDrawScoreLine(mut n: libc::c_int, mut y: libc::c_int) {
    let mut rank: libc::c_int = 0;
    let mut name: [libc::c_char; 64] = [0; 64];
    let mut info: [libc::c_char; 1024] = [0; 1024];
    if n > postgameMenuInfo.numClients + 1i32 {
        n -= postgameMenuInfo.numClients + 2i32
    }
    if n >= postgameMenuInfo.numClients {
        return;
    }
    rank = postgameMenuInfo.ranks[n as usize];
    if 0 != rank & 0x4000i32 {
        UI_DrawString(
            640i32 - 31i32 * 8i32,
            y,
            b"(tie)\x00" as *const u8 as *const libc::c_char,
            0i32 | 0x10i32,
            color_white.as_mut_ptr(),
        );
        rank &= !0x4000i32
    }
    trap_GetConfigString(
        32i32 + 256i32 + 256i32 + postgameMenuInfo.clientNums[n as usize],
        info.as_mut_ptr(),
        1024i32,
    );
    Q_strncpyz(
        name.as_mut_ptr(),
        Info_ValueForKey(
            info.as_mut_ptr(),
            b"n\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    Q_CleanStr(name.as_mut_ptr());
    UI_DrawString(
        640i32 - 25i32 * 8i32,
        y,
        va(
            b"#%i: %-16s %2i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            rank + 1i32,
            name.as_mut_ptr(),
            postgameMenuInfo.scores[n as usize],
        ),
        0i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
}
unsafe extern "C" fn UI_SPPostgameMenu_DrawAwardsMedals(mut max: libc::c_int) {
    let mut n: libc::c_int = 0;
    let mut medal: libc::c_int = 0;
    let mut amount: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut buf: [libc::c_char; 16] = [0; 16];
    let mut current_block_9: u64;
    n = 0i32;
    while n < max {
        x = medalLocations[n as usize];
        y = 64i32;
        medal = postgameMenuInfo.awardsEarned[n as usize];
        amount = postgameMenuInfo.awardsLevels[n as usize];
        UI_DrawNamedPic(
            x as libc::c_float,
            y as libc::c_float,
            48i32 as libc::c_float,
            48i32 as libc::c_float,
            ui_medalPicNames[medal as usize],
        );
        if medal == AWARD_ACCURACY as libc::c_int {
            Com_sprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
                b"%i%%\x00" as *const u8 as *const libc::c_char,
                amount,
            );
            current_block_9 = 8236137900636309791;
        } else if amount == 1i32 {
            current_block_9 = 4644295000439058019;
        } else {
            Com_sprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
                b"%i\x00" as *const u8 as *const libc::c_char,
                amount,
            );
            current_block_9 = 8236137900636309791;
        }
        match current_block_9 {
            8236137900636309791 => {
                UI_DrawString(
                    x + 24i32,
                    y + 52i32,
                    buf.as_mut_ptr(),
                    0x1i32,
                    color_yellow.as_mut_ptr(),
                );
            }
            _ => {}
        }
        n += 1
    }
}
static mut medalLocations: [libc::c_int; 6] = [144i32, 448i32, 88i32, 504i32, 32i32, 560i32];
unsafe extern "C" fn UI_SPPostgameMenu_DrawAwardsPresentation(mut timer: libc::c_int) {
    let mut awardNum: libc::c_int = 0;
    let mut atimer: libc::c_int = 0;
    let mut color: vec4_t = [0.; 4];
    awardNum = timer / 2000i32;
    atimer = timer % 2000i32;
    color[2usize] = 1.0f32;
    color[1usize] = color[2usize];
    color[0usize] = color[1usize];
    color[3usize] = (2000i32 - atimer) as libc::c_float / 2000i32 as libc::c_float;
    UI_DrawProportionalString(
        320i32,
        64i32,
        ui_medalNames[postgameMenuInfo.awardsEarned[awardNum as usize] as usize],
        0x1i32,
        color.as_mut_ptr(),
    );
    UI_SPPostgameMenu_DrawAwardsMedals(awardNum + 1i32);
    if 0 == postgameMenuInfo.playedSound[awardNum as usize] as u64 {
        postgameMenuInfo.playedSound[awardNum as usize] = qtrue;
        trap_S_StartLocalSound(
            trap_S_RegisterSound(
                ui_medalSounds[postgameMenuInfo.awardsEarned[awardNum as usize] as usize],
                qfalse,
            ),
            CHAN_ANNOUNCER as libc::c_int,
        );
    };
}
/*
=================
UI_SPPostgameMenu_MenuKey
=================
*/
unsafe extern "C" fn UI_SPPostgameMenu_MenuKey(mut key: libc::c_int) -> sfxHandle_t {
    if uis.realtime < postgameMenuInfo.ignoreKeysTime {
        return 0i32;
    }
    if postgameMenuInfo.phase == 1i32 {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            b"abort_podium\n\x00" as *const u8 as *const libc::c_char,
        );
        postgameMenuInfo.phase = 2i32;
        postgameMenuInfo.starttime = uis.realtime;
        postgameMenuInfo.ignoreKeysTime = uis.realtime + 250i32;
        return 0i32;
    }
    if postgameMenuInfo.phase == 2i32 {
        postgameMenuInfo.phase = 3i32;
        postgameMenuInfo.starttime = uis.realtime;
        postgameMenuInfo.ignoreKeysTime = uis.realtime + 250i32;
        return 0i32;
    }
    if key == K_ESCAPE as libc::c_int || key == K_MOUSE2 as libc::c_int {
        return 0i32;
    }
    return Menu_DefaultKey(&mut postgameMenuInfo.menu, key);
}
static mut arenainfo: [libc::c_char; 1024] = [0; 1024];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct postgameMenuInfo_t {
    pub menu: menuframework_s,
    pub item_again: menubitmap_s,
    pub item_next: menubitmap_s,
    pub item_menu: menubitmap_s,
    pub phase: libc::c_int,
    pub ignoreKeysTime: libc::c_int,
    pub starttime: libc::c_int,
    pub scoreboardtime: libc::c_int,
    pub serverId: libc::c_int,
    pub clientNums: [libc::c_int; 8],
    pub ranks: [libc::c_int; 8],
    pub scores: [libc::c_int; 8],
    pub placeNames: [[libc::c_char; 64]; 3],
    pub level: libc::c_int,
    pub numClients: libc::c_int,
    pub won: libc::c_int,
    pub numAwards: libc::c_int,
    pub awardsEarned: [libc::c_int; 6],
    pub awardsLevels: [libc::c_int; 6],
    pub playedSound: [qboolean; 6],
    pub lastTier: libc::c_int,
    pub winnerSound: sfxHandle_t,
}
