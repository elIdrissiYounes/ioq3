#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use bg_misc::bg_itemlist;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, colorYellow, g_color_table, vec3_origin,
    vectoangles, AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract,
    AnglesToAxis, AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    cvarHandle_t, fileHandle_t, fsMode_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, va,
    vmCvar_t, COM_Parse, COM_ParseExt, Com_Printf, Com_sprintf, Info_SetValueForKey,
    Info_ValueForKey, Q_stricmp, Q_strncpyz, FS_APPEND, FS_APPEND_SYNC, FS_READ, FS_WRITE,
};
use stdlib::{atoi, strcat, strcmp, strcpy, strlen, strstr};
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
use ui_ingame::{InGame_Cache, UI_InGameMenu};
use ui_local_h::{
    _tag_menuframework, menuframework_s, trap_Cvar_Register, trap_Cvar_Set,
    trap_Cvar_VariableStringBuffer, trap_Cvar_VariableValue, trap_FS_FCloseFile, trap_FS_FOpenFile,
    trap_FS_GetFileList, trap_FS_Read, trap_Print, uiStatic_t, unnamed, AWARD_ACCURACY,
    AWARD_EXCELLENT, AWARD_FRAGS, AWARD_GAUNTLET, AWARD_IMPRESSIVE, AWARD_PERFECT,
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
use ui_team::{TeamMain_Cache, UI_TeamMainMenu};
use ui_teamorders::{UI_TeamOrdersMenu, UI_TeamOrdersMenu_f};
use ui_video::{DriverInfo_Cache, GraphicsOptions_Cache, UI_GraphicsOptionsMenu};
extern crate libc;

#[no_mangle]
pub unsafe extern "C" fn UI_GetArenaInfoByNumber(mut num: libc::c_int) -> *const libc::c_char {
    let mut n: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    if num < 0i32 || num >= ui_numArenas {
        trap_Print(va(
            b"^1Invalid arena number: %i\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            num,
        ));
        return 0 as *const libc::c_char;
    }
    n = 0i32;
    while n < ui_numArenas {
        value = Info_ValueForKey(
            ui_arenaInfos[n as usize],
            b"num\x00" as *const u8 as *const libc::c_char,
        );
        if 0 != *value as libc::c_int && atoi(value) == num {
            return ui_arenaInfos[n as usize];
        }
        n += 1
    }
    return 0 as *const libc::c_char;
}
static mut ui_arenaInfos: [*mut libc::c_char; 1024] =
    [0 as *const libc::c_char as *mut libc::c_char; 1024];
static mut ui_numArenas: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn UI_GetArenaInfoByMap(mut map: *const libc::c_char) -> *const libc::c_char {
    let mut n: libc::c_int = 0;
    n = 0i32;
    while n < ui_numArenas {
        if Q_stricmp(
            Info_ValueForKey(
                ui_arenaInfos[n as usize],
                b"map\x00" as *const u8 as *const libc::c_char,
            ),
            map,
        ) == 0i32
        {
            return ui_arenaInfos[n as usize];
        }
        n += 1
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn UI_GetSpecialArenaInfo(
    mut tag: *const libc::c_char,
) -> *const libc::c_char {
    let mut n: libc::c_int = 0;
    n = 0i32;
    while n < ui_numArenas {
        if Q_stricmp(
            Info_ValueForKey(
                ui_arenaInfos[n as usize],
                b"special\x00" as *const u8 as *const libc::c_char,
            ),
            tag,
        ) == 0i32
        {
            return ui_arenaInfos[n as usize];
        }
        n += 1
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn UI_GetNumArenas() -> libc::c_int {
    return ui_numArenas;
}
#[no_mangle]
pub unsafe extern "C" fn UI_GetNumSPArenas() -> libc::c_int {
    return ui_numSinglePlayerArenas;
}
static mut ui_numSinglePlayerArenas: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn UI_GetNumSPTiers() -> libc::c_int {
    return ui_numSinglePlayerArenas / 4i32;
}
#[no_mangle]
pub unsafe extern "C" fn UI_GetBotInfoByNumber(mut num: libc::c_int) -> *mut libc::c_char {
    if num < 0i32 || num >= ui_numBots {
        trap_Print(va(
            b"^1Invalid bot number: %i\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            num,
        ));
        return 0 as *mut libc::c_char;
    }
    return ui_botInfos[num as usize];
}
static mut ui_botInfos: [*mut libc::c_char; 1024] =
    [0 as *const libc::c_char as *mut libc::c_char; 1024];
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
//
// gameinfo.c
//
//
// arena and bot info
//
#[no_mangle]
pub static mut ui_numBots: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn UI_GetBotInfoByName(mut name: *const libc::c_char) -> *mut libc::c_char {
    let mut n: libc::c_int = 0;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    n = 0i32;
    while n < ui_numBots {
        value = Info_ValueForKey(
            ui_botInfos[n as usize],
            b"name\x00" as *const u8 as *const libc::c_char,
        );
        if 0 == Q_stricmp(value, name) {
            return ui_botInfos[n as usize];
        }
        n += 1
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn UI_GetNumBots() -> libc::c_int {
    return ui_numBots;
}
#[no_mangle]
pub unsafe extern "C" fn UI_GetBestScore(
    mut level: libc::c_int,
    mut score: *mut libc::c_int,
    mut skill: *mut libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut skillScore: libc::c_int = 0;
    let mut bestScore: libc::c_int = 0;
    let mut bestScoreSkill: libc::c_int = 0;
    let mut arenaKey: [libc::c_char; 16] = [0; 16];
    let mut scores: [libc::c_char; 1024] = [0; 1024];
    if score.is_null() || skill.is_null() {
        return;
    }
    if level < 0i32 || level > ui_numArenas {
        return;
    }
    bestScore = 0i32;
    bestScoreSkill = 0i32;
    n = 1i32;
    while n <= 5i32 {
        trap_Cvar_VariableStringBuffer(
            va(
                b"g_spScores%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                n,
            ),
            scores.as_mut_ptr(),
            1024i32,
        );
        Com_sprintf(
            arenaKey.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
            b"l%i\x00" as *const u8 as *const libc::c_char,
            level,
        );
        skillScore = atoi(Info_ValueForKey(scores.as_mut_ptr(), arenaKey.as_mut_ptr()));
        if !(skillScore < 1i32 || skillScore > 8i32) {
            if 0 == bestScore || skillScore <= bestScore {
                bestScore = skillScore;
                bestScoreSkill = n
            }
        }
        n += 1
    }
    *score = bestScore;
    *skill = bestScoreSkill;
}
#[no_mangle]
pub unsafe extern "C" fn UI_SetBestScore(mut level: libc::c_int, mut score: libc::c_int) {
    let mut skill: libc::c_int = 0;
    let mut oldScore: libc::c_int = 0;
    let mut arenaKey: [libc::c_char; 16] = [0; 16];
    let mut scores: [libc::c_char; 1024] = [0; 1024];
    if score < 1i32 || score > 8i32 {
        return;
    }
    skill = trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const libc::c_char)
        as libc::c_int;
    if skill < 1i32 || skill > 5i32 {
        return;
    }
    trap_Cvar_VariableStringBuffer(
        va(
            b"g_spScores%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            skill,
        ),
        scores.as_mut_ptr(),
        1024i32,
    );
    Com_sprintf(
        arenaKey.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"l%i\x00" as *const u8 as *const libc::c_char,
        level,
    );
    oldScore = atoi(Info_ValueForKey(scores.as_mut_ptr(), arenaKey.as_mut_ptr()));
    if 0 != oldScore && oldScore <= score {
        return;
    }
    Info_SetValueForKey(
        scores.as_mut_ptr(),
        arenaKey.as_mut_ptr(),
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            score,
        ),
    );
    trap_Cvar_Set(
        va(
            b"g_spScores%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            skill,
        ),
        scores.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn UI_TierCompleted(mut levelWon: libc::c_int) -> libc::c_int {
    let mut level: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut tier: libc::c_int = 0;
    let mut score: libc::c_int = 0;
    let mut skill: libc::c_int = 0;
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    tier = levelWon / 4i32;
    level = tier * 4i32;
    if tier == UI_GetNumSPTiers() {
        info = UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const libc::c_char);
        if levelWon
            == atoi(Info_ValueForKey(
                info,
                b"num\x00" as *const u8 as *const libc::c_char,
            ))
        {
            return 0i32;
        }
        info = UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const libc::c_char);
        if info.is_null()
            || levelWon
                == atoi(Info_ValueForKey(
                    info,
                    b"num\x00" as *const u8 as *const libc::c_char,
                ))
        {
            return tier + 1i32;
        }
        return -1i32;
    }
    n = 0i32;
    while n < 4i32 {
        UI_GetBestScore(level, &mut score, &mut skill);
        if score != 1i32 {
            return -1i32;
        }
        n += 1;
        level += 1
    }
    return tier + 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn UI_ShowTierVideo(mut tier: libc::c_int) -> qboolean {
    let mut key: [libc::c_char; 16] = [0; 16];
    let mut videos: [libc::c_char; 1024] = [0; 1024];
    if tier <= 0i32 {
        return qfalse;
    }
    trap_Cvar_VariableStringBuffer(
        b"g_spVideos\x00" as *const u8 as *const libc::c_char,
        videos.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"tier%i\x00" as *const u8 as *const libc::c_char,
        tier,
    );
    if 0 != atoi(Info_ValueForKey(videos.as_mut_ptr(), key.as_mut_ptr())) {
        return qfalse;
    }
    Info_SetValueForKey(
        videos.as_mut_ptr(),
        key.as_mut_ptr(),
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1i32,
        ),
    );
    trap_Cvar_Set(
        b"g_spVideos\x00" as *const u8 as *const libc::c_char,
        videos.as_mut_ptr(),
    );
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn UI_CanShowTierVideo(mut tier: libc::c_int) -> qboolean {
    let mut key: [libc::c_char; 16] = [0; 16];
    let mut videos: [libc::c_char; 1024] = [0; 1024];
    if 0 == tier {
        return qfalse;
    }
    if 0 != uis.demoversion as libc::c_uint && tier != 8i32 {
        return qfalse;
    }
    trap_Cvar_VariableStringBuffer(
        b"g_spVideos\x00" as *const u8 as *const libc::c_char,
        videos.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"tier%i\x00" as *const u8 as *const libc::c_char,
        tier,
    );
    if 0 != atoi(Info_ValueForKey(videos.as_mut_ptr(), key.as_mut_ptr())) {
        return qtrue;
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn UI_GetCurrentGame() -> libc::c_int {
    let mut level: libc::c_int = 0;
    let mut rank: libc::c_int = 0i32;
    let mut skill: libc::c_int = 0;
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    info = UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const libc::c_char);
    if !info.is_null() {
        level = atoi(Info_ValueForKey(
            info,
            b"num\x00" as *const u8 as *const libc::c_char,
        ));
        UI_GetBestScore(level, &mut rank, &mut skill);
        if 0 == rank || rank > 1i32 {
            return level;
        }
    }
    level = 0i32;
    while level < ui_numSinglePlayerArenas {
        UI_GetBestScore(level, &mut rank, &mut skill);
        if 0 == rank || rank > 1i32 {
            return level;
        }
        level += 1
    }
    info = UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const libc::c_char);
    if info.is_null() {
        return -1i32;
    }
    return atoi(Info_ValueForKey(
        info,
        b"num\x00" as *const u8 as *const libc::c_char,
    ));
}
#[no_mangle]
pub unsafe extern "C" fn UI_NewGame() {
    trap_Cvar_Set(
        b"g_spScores1\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    trap_Cvar_Set(
        b"g_spScores2\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    trap_Cvar_Set(
        b"g_spScores3\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    trap_Cvar_Set(
        b"g_spScores4\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    trap_Cvar_Set(
        b"g_spScores5\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    trap_Cvar_Set(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    trap_Cvar_Set(
        b"g_spVideos\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn UI_LogAwardData(mut award: libc::c_int, mut data: libc::c_int) {
    let mut key: [libc::c_char; 16] = [0; 16];
    let mut awardData: [libc::c_char; 1024] = [0; 1024];
    let mut oldValue: libc::c_int = 0;
    if data == 0i32 {
        return;
    }
    if award > AWARD_PERFECT as libc::c_int {
        trap_Print(va(
            b"^1Bad award %i in UI_LogAwardData\n\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            award,
        ));
        return;
    }
    trap_Cvar_VariableStringBuffer(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        awardData.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"a%i\x00" as *const u8 as *const libc::c_char,
        award,
    );
    oldValue = atoi(Info_ValueForKey(awardData.as_mut_ptr(), key.as_mut_ptr()));
    Info_SetValueForKey(
        awardData.as_mut_ptr(),
        key.as_mut_ptr(),
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            oldValue + data,
        ),
    );
    trap_Cvar_Set(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        awardData.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn UI_GetAwardLevel(mut award: libc::c_int) -> libc::c_int {
    let mut key: [libc::c_char; 16] = [0; 16];
    let mut awardData: [libc::c_char; 1024] = [0; 1024];
    trap_Cvar_VariableStringBuffer(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        awardData.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        b"a%i\x00" as *const u8 as *const libc::c_char,
        award,
    );
    return atoi(Info_ValueForKey(awardData.as_mut_ptr(), key.as_mut_ptr()));
}
#[no_mangle]
pub unsafe extern "C" fn UI_SPUnlock_f() {
    let mut arenaKey: [libc::c_char; 16] = [0; 16];
    let mut scores: [libc::c_char; 1024] = [0; 1024];
    let mut level: libc::c_int = 0;
    let mut tier: libc::c_int = 0;
    trap_Cvar_VariableStringBuffer(
        b"g_spScores1\x00" as *const u8 as *const libc::c_char,
        scores.as_mut_ptr(),
        1024i32,
    );
    level = 0i32;
    while level < ui_numSinglePlayerArenas + ui_numSpecialSinglePlayerArenas {
        Com_sprintf(
            arenaKey.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
            b"l%i\x00" as *const u8 as *const libc::c_char,
            level,
        );
        Info_SetValueForKey(
            scores.as_mut_ptr(),
            arenaKey.as_mut_ptr(),
            b"1\x00" as *const u8 as *const libc::c_char,
        );
        level += 1
    }
    trap_Cvar_Set(
        b"g_spScores1\x00" as *const u8 as *const libc::c_char,
        scores.as_mut_ptr(),
    );
    tier = 1i32;
    while tier <= 8i32 {
        UI_ShowTierVideo(tier);
        tier += 1
    }
    trap_Print(b"All levels unlocked at skill level 1\n\x00" as *const u8 as *const libc::c_char);
    UI_SPLevelMenu_ReInit();
}
static mut ui_numSpecialSinglePlayerArenas: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn UI_SPUnlockMedals_f() {
    let mut n: libc::c_int = 0;
    let mut key: [libc::c_char; 16] = [0; 16];
    let mut awardData: [libc::c_char; 1024] = [0; 1024];
    trap_Cvar_VariableStringBuffer(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        awardData.as_mut_ptr(),
        1024i32,
    );
    n = 0i32;
    while n < 6i32 {
        Com_sprintf(
            key.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
            b"a%i\x00" as *const u8 as *const libc::c_char,
            n,
        );
        Info_SetValueForKey(
            awardData.as_mut_ptr(),
            key.as_mut_ptr(),
            b"100\x00" as *const u8 as *const libc::c_char,
        );
        n += 1
    }
    trap_Cvar_Set(
        b"g_spAwards\x00" as *const u8 as *const libc::c_char,
        awardData.as_mut_ptr(),
    );
    trap_Print(b"All awards unlocked at 100\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_InitGameinfo() {
    UI_InitMemory();
    UI_LoadArenas();
    UI_LoadBots();
    uis.demoversion = qfalse;
}
/*
===============
UI_LoadBots
===============
*/
unsafe extern "C" fn UI_LoadBots() {
    let mut botsFile: vmCvar_t = vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
    let mut numdirs: libc::c_int = 0;
    let mut filename: [libc::c_char; 128] = [0; 128];
    let mut dirlist: [libc::c_char; 1024] = [0; 1024];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut dirlen: libc::c_int = 0;
    ui_numBots = 0i32;
    trap_Cvar_Register(
        &mut botsFile,
        b"g_botsFile\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10i32 | 0x40i32,
    );
    if 0 != *botsFile.string.as_mut_ptr() {
        UI_LoadBotsFromFile(botsFile.string.as_mut_ptr());
    } else {
        UI_LoadBotsFromFile(
            b"scripts/bots.txt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    numdirs = trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const libc::c_char,
        b".bot\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        1024i32,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0i32;
    while i < numdirs {
        dirlen = strlen(dirptr) as libc::c_int;
        strcpy(
            filename.as_mut_ptr(),
            b"scripts/\x00" as *const u8 as *const libc::c_char,
        );
        strcat(filename.as_mut_ptr(), dirptr);
        UI_LoadBotsFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1i32) as isize)
    }
    trap_Print(va(
        b"%i bots parsed\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ui_numBots,
    ));
}
/*
===============
UI_LoadBotsFromFile
===============
*/
unsafe extern "C" fn UI_LoadBotsFromFile(mut filename: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut f: fileHandle_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    len = trap_FS_FOpenFile(filename, &mut f, FS_READ);
    if 0 == f {
        trap_Print(va(
            b"^1file not found: %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        ));
        return;
    }
    if len >= 8192i32 {
        trap_Print(va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            filename,
            len,
            8192i32,
        ));
        trap_FS_FCloseFile(f);
        return;
    }
    trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0i32 as libc::c_char;
    trap_FS_FCloseFile(f);
    ui_numBots += UI_ParseInfos(
        buf.as_mut_ptr(),
        1024i32 - ui_numBots,
        &mut *ui_botInfos.as_mut_ptr().offset(ui_numBots as isize),
    );
    if 0 != outOfMemory {
        trap_Print(
            b"^3WARNING: not enough memory in pool to load all bots\n\x00" as *const u8
                as *const libc::c_char,
        );
    };
}
static mut outOfMemory: libc::c_int = 0;
/*
===============
UI_ParseInfos
===============
*/
#[no_mangle]
pub unsafe extern "C" fn UI_ParseInfos(
    mut buf: *mut libc::c_char,
    mut max: libc::c_int,
    mut infos: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut count: libc::c_int = 0;
    let mut key: [libc::c_char; 1024] = [0; 1024];
    let mut info: [libc::c_char; 1024] = [0; 1024];
    count = 0i32;
    loop {
        token = COM_Parse(&mut buf);
        if 0 == *token.offset(0isize) {
            break;
        }
        if 0 != strcmp(token, b"{\x00" as *const u8 as *const libc::c_char) {
            Com_Printf(b"Missing { in info file\n\x00" as *const u8 as *const libc::c_char);
            break;
        } else if count == max {
            Com_Printf(b"Max infos exceeded\n\x00" as *const u8 as *const libc::c_char);
            break;
        } else {
            info[0usize] = '\u{0}' as i32 as libc::c_char;
            loop {
                token = COM_ParseExt(&mut buf, qtrue);
                if 0 == *token.offset(0isize) {
                    Com_Printf(
                        b"Unexpected end of info file\n\x00" as *const u8 as *const libc::c_char,
                    );
                    break;
                } else {
                    if 0 == strcmp(token, b"}\x00" as *const u8 as *const libc::c_char) {
                        break;
                    }
                    Q_strncpyz(
                        key.as_mut_ptr(),
                        token,
                        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                            as libc::c_int,
                    );
                    token = COM_ParseExt(&mut buf, qfalse);
                    if 0 == *token.offset(0isize) {
                        strcpy(token, b"<NULL>\x00" as *const u8 as *const libc::c_char);
                    }
                    Info_SetValueForKey(info.as_mut_ptr(), key.as_mut_ptr(), token);
                }
            }
            let ref mut fresh0 = *infos.offset(count as isize);
            *fresh0 = UI_Alloc(
                strlen(info.as_mut_ptr())
                    .wrapping_add(strlen(b"\\num\\\x00" as *const u8 as *const libc::c_char))
                    .wrapping_add(strlen(va(
                        b"%d\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        1024i32,
                    )))
                    .wrapping_add(1i32 as libc::c_ulong) as libc::c_int,
            ) as *mut libc::c_char;
            if !(*infos.offset(count as isize)).is_null() {
                strcpy(*infos.offset(count as isize), info.as_mut_ptr());
                count += 1
            }
        }
    }
    return count;
}
/*
===============
UI_Alloc
===============
*/
#[no_mangle]
pub unsafe extern "C" fn UI_Alloc(mut size: libc::c_int) -> *mut libc::c_void {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if allocPoint + size > 128i32 * 1024i32 {
        outOfMemory = qtrue as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    p = &mut *memoryPool.as_mut_ptr().offset(allocPoint as isize) as *mut libc::c_char;
    allocPoint += size + 31i32 & !31i32;
    return p as *mut libc::c_void;
}
static mut allocPoint: libc::c_int = 0;
static mut memoryPool: [libc::c_char; 131072] = [0; 131072];
/*
===============
UI_LoadArenas
===============
*/
unsafe extern "C" fn UI_LoadArenas() {
    let mut numdirs: libc::c_int = 0;
    let mut arenasFile: vmCvar_t = vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
    let mut filename: [libc::c_char; 128] = [0; 128];
    let mut dirlist: [libc::c_char; 4096] = [0; 4096];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut dirlen: libc::c_int = 0;
    let mut type_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tag: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut singlePlayerNum: libc::c_int = 0;
    let mut specialNum: libc::c_int = 0;
    let mut otherNum: libc::c_int = 0;
    ui_numArenas = 0i32;
    trap_Cvar_Register(
        &mut arenasFile,
        b"g_arenasFile\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        0x10i32 | 0x40i32,
    );
    if 0 != *arenasFile.string.as_mut_ptr() {
        UI_LoadArenasFromFile(arenasFile.string.as_mut_ptr());
    } else {
        UI_LoadArenasFromFile(
            b"scripts/arenas.txt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    numdirs = trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const libc::c_char,
        b".arena\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        4096i32,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0i32;
    while i < numdirs {
        dirlen = strlen(dirptr) as libc::c_int;
        strcpy(
            filename.as_mut_ptr(),
            b"scripts/\x00" as *const u8 as *const libc::c_char,
        );
        strcat(filename.as_mut_ptr(), dirptr);
        UI_LoadArenasFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1i32) as isize)
    }
    trap_Print(va(
        b"%i arenas parsed\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ui_numArenas,
    ));
    if 0 != outOfMemory {
        trap_Print(
            b"^3WARNING: not enough memory in pool to load all arenas\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    n = 0i32;
    while n < ui_numArenas {
        Info_SetValueForKey(
            ui_arenaInfos[n as usize],
            b"num\x00" as *const u8 as *const libc::c_char,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                n,
            ),
        );
        n += 1
    }
    ui_numSinglePlayerArenas = 0i32;
    ui_numSpecialSinglePlayerArenas = 0i32;
    n = 0i32;
    while n < ui_numArenas {
        type_0 = Info_ValueForKey(
            ui_arenaInfos[n as usize],
            b"type\x00" as *const u8 as *const libc::c_char,
        );
        // if no type specified, it will be treated as "ffa"
        if !(0 == *type_0) {
            if !strstr(type_0, b"single\x00" as *const u8 as *const libc::c_char).is_null() {
                tag = Info_ValueForKey(
                    ui_arenaInfos[n as usize],
                    b"special\x00" as *const u8 as *const libc::c_char,
                );
                if 0 != *tag {
                    ui_numSpecialSinglePlayerArenas += 1
                } else {
                    ui_numSinglePlayerArenas += 1
                }
            }
        }
        n += 1
    }
    n = ui_numSinglePlayerArenas % 4i32;
    if n != 0i32 {
        ui_numSinglePlayerArenas -= n;
        trap_Print(va(
            b"%i arenas ignored to make count divisible by %i\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            n,
            4i32,
        ));
    }
    singlePlayerNum = 0i32;
    specialNum = singlePlayerNum + ui_numSinglePlayerArenas;
    otherNum = specialNum + ui_numSpecialSinglePlayerArenas;
    let mut current_block_44: u64;
    n = 0i32;
    while n < ui_numArenas {
        type_0 = Info_ValueForKey(
            ui_arenaInfos[n as usize],
            b"type\x00" as *const u8 as *const libc::c_char,
        );
        // if no type specified, it will be treated as "ffa"
        if 0 != *type_0 {
            if !strstr(type_0, b"single\x00" as *const u8 as *const libc::c_char).is_null() {
                tag = Info_ValueForKey(
                    ui_arenaInfos[n as usize],
                    b"special\x00" as *const u8 as *const libc::c_char,
                );
                if 0 != *tag {
                    let fresh1 = specialNum;
                    specialNum = specialNum + 1;
                    Info_SetValueForKey(
                        ui_arenaInfos[n as usize],
                        b"num\x00" as *const u8 as *const libc::c_char,
                        va(
                            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            fresh1,
                        ),
                    );
                } else {
                    let fresh2 = singlePlayerNum;
                    singlePlayerNum = singlePlayerNum + 1;
                    Info_SetValueForKey(
                        ui_arenaInfos[n as usize],
                        b"num\x00" as *const u8 as *const libc::c_char,
                        va(
                            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                            fresh2,
                        ),
                    );
                }
                current_block_44 = 14775119014532381840;
            } else {
                current_block_44 = 16415152177862271243;
            }
        } else {
            current_block_44 = 16415152177862271243;
        }
        match current_block_44 {
            16415152177862271243 => {
                let fresh3 = otherNum;
                otherNum = otherNum + 1;
                Info_SetValueForKey(
                    ui_arenaInfos[n as usize],
                    b"num\x00" as *const u8 as *const libc::c_char,
                    va(
                        b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        fresh3,
                    ),
                );
            }
            _ => {}
        }
        n += 1
    }
}
/*
===============
UI_LoadArenasFromFile
===============
*/
unsafe extern "C" fn UI_LoadArenasFromFile(mut filename: *mut libc::c_char) {
    let mut len: libc::c_int = 0;
    let mut f: fileHandle_t = 0;
    let mut buf: [libc::c_char; 8192] = [0; 8192];
    len = trap_FS_FOpenFile(filename, &mut f, FS_READ);
    if 0 == f {
        trap_Print(va(
            b"^1file not found: %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            filename,
        ));
        return;
    }
    if len >= 8192i32 {
        trap_Print(va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            filename,
            len,
            8192i32,
        ));
        trap_FS_FCloseFile(f);
        return;
    }
    trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0i32 as libc::c_char;
    trap_FS_FCloseFile(f);
    ui_numArenas += UI_ParseInfos(
        buf.as_mut_ptr(),
        1024i32 - ui_numArenas,
        &mut *ui_arenaInfos.as_mut_ptr().offset(ui_numArenas as isize),
    );
}
/*
===============
UI_InitMemory
===============
*/
#[no_mangle]
pub unsafe extern "C" fn UI_InitMemory() {
    allocPoint = 0i32;
    outOfMemory = qfalse as libc::c_int;
}
