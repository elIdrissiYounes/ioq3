use bg_misc::bg_itemlist;
use libc;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
    AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
    AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    qboolean, qfalse, qtrue, sfxHandle_t, unnamed, vec4_t, vec_t, EXEC_APPEND, EXEC_INSERT,
    EXEC_NOW,
};
use stdlib::memset;
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
use ui_local_h::{_tag_menuframework, menuframework_s, trap_Cmd_ExecuteText};
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

//
// ui_credits.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_CreditMenu() {
    memset(
        &mut s_credits as *mut creditsmenu_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<creditsmenu_t>() as libc::c_ulong,
    );
    s_credits.menu.draw = Some(UI_CreditMenu_Draw);
    s_credits.menu.key = Some(UI_CreditMenu_Key);
    s_credits.menu.fullscreen = qtrue;
    UI_PushMenu(&mut s_credits.menu);
}
static mut s_credits: creditsmenu_t = creditsmenu_t {
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
    frame: 0,
};
/*
=================
UI_CreditMenu_Key
=================
*/
unsafe extern "C" fn UI_CreditMenu_Key(mut key: libc::c_int) -> sfxHandle_t {
    if 0 != key & 1024i32 {
        return 0i32;
    }
    s_credits.frame += 1;
    if s_credits.frame == 1i32 {
        s_credits.menu.draw = Some(UI_CreditMenu_Draw_ioq3)
    } else {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            b"quit\n\x00" as *const u8 as *const libc::c_char,
        );
    }
    return 0i32;
}
/*
===============
UI_CreditMenu_Draw_ioq3
===============
*/
unsafe extern "C" fn UI_CreditMenu_Draw_ioq3() {
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    // These are all people that have made commits to Subversion, and thus
    //  probably incomplete.
    // (These are in alphabetical order, for the defense of everyone's egos.)
    static mut names: [*const libc::c_char; 14] = [
        b"Tim Angus\x00" as *const u8 as *const libc::c_char,
        b"James Canete\x00" as *const u8 as *const libc::c_char,
        b"Vincent Cojot\x00" as *const u8 as *const libc::c_char,
        b"Ryan C. Gordon\x00" as *const u8 as *const libc::c_char,
        b"Aaron Gyes\x00" as *const u8 as *const libc::c_char,
        b"Zack Middleton\x00" as *const u8 as *const libc::c_char,
        b"Ludwig Nussel\x00" as *const u8 as *const libc::c_char,
        b"Julian Priestley\x00" as *const u8 as *const libc::c_char,
        b"Scirocco Six\x00" as *const u8 as *const libc::c_char,
        b"Thilo Schulz\x00" as *const u8 as *const libc::c_char,
        b"Zachary J. Slater\x00" as *const u8 as *const libc::c_char,
        b"Tony J. White\x00" as *const u8 as *const libc::c_char,
        b"...and many, many others!\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    y = ((480i32 as libc::c_double
        - (::std::mem::size_of::<[*const libc::c_char; 14]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
            as libc::c_double
            * (1.42f64 * 27i32 as libc::c_double * 0.75f64))
        / 2i32 as libc::c_double) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"ioquake3 contributors:\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 1.42f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    i = 0i32;
    while !names[i as usize].is_null() {
        UI_DrawProportionalString(
            320i32,
            y,
            names[i as usize],
            0x1i32 | 0x10i32,
            color_white.as_mut_ptr(),
        );
        y = (y as libc::c_double + 1.42f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
        i += 1
    }
    UI_DrawString(
        320i32,
        459i32,
        b"http://www.ioquake3.org/\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_red.as_mut_ptr(),
    );
}
/*
===============
UI_CreditMenu_Draw
===============
*/
unsafe extern "C" fn UI_CreditMenu_Draw() {
    let mut y: libc::c_int = 0;
    y = 12i32;
    UI_DrawProportionalString(
        320i32,
        y,
        b"id Software is:\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 1.42f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Programming\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"John Carmack, Robert A. Duffy, Jim Dose\'\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 1.42f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Art\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Adrian Carmack, Kevin Cloud,\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Kenneth Scott, Seneca Menard, Fred Nilsson\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 1.42f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Game Designer\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Graeme Devine\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 1.42f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Level Design\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Tim Willits, Christian Antkow, Paul Jaquays\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 1.42f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"CEO\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Todd Hollenshead\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 1.42f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Director of Business Development\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Marty Stratton\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 1.42f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Biz Assist and id Mom\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Donna Jackson\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 1.42f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Development Assistance\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawProportionalString(
        320i32,
        y,
        b"Eric Webb\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_white.as_mut_ptr(),
    );
    y = (y as libc::c_double + 1.35f64 * 27i32 as libc::c_double * 0.75f64) as libc::c_int;
    UI_DrawString(
        320i32,
        y,
        b"To order: 1-800-idgames     www.quake3arena.com     www.idsoftware.com\x00" as *const u8
            as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_red.as_mut_ptr(),
    );
    y += 16i32;
    UI_DrawString(
        320i32,
        y,
        b"Quake III Arena(c) 1999-2000, Id Software, Inc.  All Rights Reserved\x00" as *const u8
            as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_red.as_mut_ptr(),
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct creditsmenu_t {
    pub menu: menuframework_s,
    pub frame: libc::c_int,
}
