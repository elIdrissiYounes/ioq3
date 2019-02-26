#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(custom_attribute, libc)]
use bg_misc::bg_itemlist;
use keycodes_h::{
    unnamed, K_ALT, K_AUX1, K_AUX10, K_AUX11, K_AUX12, K_AUX13, K_AUX14, K_AUX15, K_AUX16, K_AUX2,
    K_AUX3, K_AUX4, K_AUX5, K_AUX6, K_AUX7, K_AUX8, K_AUX9, K_BACKSPACE, K_BREAK, K_CAPSLOCK,
    K_COMMAND, K_COMPOSE, K_CONSOLE, K_CTRL, K_DEL, K_DOWNARROW, K_END, K_ENTER, K_ESCAPE, K_EURO,
    K_F1, K_F10, K_F11, K_F12, K_F13, K_F14, K_F15, K_F2, K_F3, K_F4, K_F5, K_F6, K_F7, K_F8, K_F9,
    K_HELP, K_HOME, K_INS, K_JOY1, K_JOY10, K_JOY11, K_JOY12, K_JOY13, K_JOY14, K_JOY15, K_JOY16,
    K_JOY17, K_JOY18, K_JOY19, K_JOY2, K_JOY20, K_JOY21, K_JOY22, K_JOY23, K_JOY24, K_JOY25,
    K_JOY26, K_JOY27, K_JOY28, K_JOY29, K_JOY3, K_JOY30, K_JOY31, K_JOY32, K_JOY4, K_JOY5, K_JOY6,
    K_JOY7, K_JOY8, K_JOY9, K_KP_5, K_KP_DEL, K_KP_DOWNARROW, K_KP_END, K_KP_ENTER, K_KP_EQUALS,
    K_KP_HOME, K_KP_INS, K_KP_LEFTARROW, K_KP_MINUS, K_KP_NUMLOCK, K_KP_PGDN, K_KP_PGUP, K_KP_PLUS,
    K_KP_RIGHTARROW, K_KP_SLASH, K_KP_STAR, K_KP_UPARROW, K_LEFTARROW, K_MENU, K_MODE, K_MOUSE1,
    K_MOUSE2, K_MOUSE3, K_MOUSE4, K_MOUSE5, K_MWHEELDOWN, K_MWHEELUP, K_PAD0_A, K_PAD0_B,
    K_PAD0_BACK, K_PAD0_DPAD_DOWN, K_PAD0_DPAD_LEFT, K_PAD0_DPAD_RIGHT, K_PAD0_DPAD_UP,
    K_PAD0_GUIDE, K_PAD0_LEFTSHOULDER, K_PAD0_LEFTSTICK_CLICK, K_PAD0_LEFTSTICK_DOWN,
    K_PAD0_LEFTSTICK_LEFT, K_PAD0_LEFTSTICK_RIGHT, K_PAD0_LEFTSTICK_UP, K_PAD0_LEFTTRIGGER,
    K_PAD0_RIGHTSHOULDER, K_PAD0_RIGHTSTICK_CLICK, K_PAD0_RIGHTSTICK_DOWN, K_PAD0_RIGHTSTICK_LEFT,
    K_PAD0_RIGHTSTICK_RIGHT, K_PAD0_RIGHTSTICK_UP, K_PAD0_RIGHTTRIGGER, K_PAD0_START, K_PAD0_X,
    K_PAD0_Y, K_PAUSE, K_PGDN, K_PGUP, K_POWER, K_PRINT, K_RIGHTARROW, K_SCROLLOCK, K_SHIFT,
    K_SPACE, K_SUPER, K_SYSREQ, K_TAB, K_UNDO, K_UPARROW, K_WORLD_0, K_WORLD_1, K_WORLD_10,
    K_WORLD_11, K_WORLD_12, K_WORLD_13, K_WORLD_14, K_WORLD_15, K_WORLD_16, K_WORLD_17, K_WORLD_18,
    K_WORLD_19, K_WORLD_2, K_WORLD_20, K_WORLD_21, K_WORLD_22, K_WORLD_23, K_WORLD_24, K_WORLD_25,
    K_WORLD_26, K_WORLD_27, K_WORLD_28, K_WORLD_29, K_WORLD_3, K_WORLD_30, K_WORLD_31, K_WORLD_32,
    K_WORLD_33, K_WORLD_34, K_WORLD_35, K_WORLD_36, K_WORLD_37, K_WORLD_38, K_WORLD_39, K_WORLD_4,
    K_WORLD_40, K_WORLD_41, K_WORLD_42, K_WORLD_43, K_WORLD_44, K_WORLD_45, K_WORLD_46, K_WORLD_47,
    K_WORLD_48, K_WORLD_49, K_WORLD_5, K_WORLD_50, K_WORLD_51, K_WORLD_52, K_WORLD_53, K_WORLD_54,
    K_WORLD_55, K_WORLD_56, K_WORLD_57, K_WORLD_58, K_WORLD_59, K_WORLD_6, K_WORLD_60, K_WORLD_61,
    K_WORLD_62, K_WORLD_63, K_WORLD_64, K_WORLD_65, K_WORLD_66, K_WORLD_67, K_WORLD_68, K_WORLD_69,
    K_WORLD_7, K_WORLD_70, K_WORLD_71, K_WORLD_72, K_WORLD_73, K_WORLD_74, K_WORLD_75, K_WORLD_76,
    K_WORLD_77, K_WORLD_78, K_WORLD_79, K_WORLD_8, K_WORLD_80, K_WORLD_81, K_WORLD_82, K_WORLD_83,
    K_WORLD_84, K_WORLD_85, K_WORLD_86, K_WORLD_87, K_WORLD_88, K_WORLD_89, K_WORLD_9, K_WORLD_90,
    K_WORLD_91, K_WORLD_92, K_WORLD_93, K_WORLD_94, K_WORLD_95, MAX_KEYS,
};
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, colorYellow, g_color_table, vec3_origin,
    vectoangles, AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract,
    AnglesToAxis, AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    qboolean, qfalse, qtrue, sfxHandle_t, vec4_t, vec_t, Q_isalpha, Q_islower, Q_isupper,
};
use stdlib::{memcpy, memmove, strlen, tolower};
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
    _tag_menuframework, menucommon_s, menufield_s, menuframework_s, mfield_t, trap_Error,
    trap_GetClipboardData, trap_Key_GetOverstrikeMode, trap_Key_IsDown, trap_Key_SetOverstrikeMode,
};
use ui_main::{
    ui_browserGameType, ui_browserMaster, ui_browserShowEmpty, ui_browserShowFull,
    ui_browserSortKey, ui_cdkeychecked, UI_RegisterCvars, UI_UpdateCvars,
};
use ui_menu::{MainMenu_Cache, UI_MainMenu};
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

//
// ui_mfield.c
//
#[no_mangle]
pub unsafe extern "C" fn MField_Clear(mut edit: *mut mfield_t) {
    (*edit).buffer[0usize] = 0i32 as libc::c_char;
    (*edit).cursor = 0i32;
    (*edit).scroll = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn MField_KeyDownEvent(mut edit: *mut mfield_t, mut key: libc::c_int) {
    let mut len: libc::c_int = 0;
    if (key == K_INS as libc::c_int || key == K_KP_INS as libc::c_int)
        && 0 != trap_Key_IsDown(K_SHIFT as libc::c_int) as libc::c_uint
    {
        MField_Paste(edit);
        return;
    }
    len = strlen((*edit).buffer.as_mut_ptr()) as libc::c_int;
    if key == K_DEL as libc::c_int || key == K_KP_DEL as libc::c_int {
        if (*edit).cursor < len {
            memmove(
                (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize) as *mut libc::c_void,
                (*edit)
                    .buffer
                    .as_mut_ptr()
                    .offset((*edit).cursor as isize)
                    .offset(1isize) as *const libc::c_void,
                (len - (*edit).cursor) as libc::c_ulong,
            );
        }
        return;
    }
    if key == K_RIGHTARROW as libc::c_int || key == K_KP_RIGHTARROW as libc::c_int {
        if (*edit).cursor < len {
            (*edit).cursor += 1
        }
        if (*edit).cursor >= (*edit).scroll + (*edit).widthInChars && (*edit).cursor <= len {
            (*edit).scroll += 1
        }
        return;
    }
    if key == K_LEFTARROW as libc::c_int || key == K_KP_LEFTARROW as libc::c_int {
        if (*edit).cursor > 0i32 {
            (*edit).cursor -= 1
        }
        if (*edit).cursor < (*edit).scroll {
            (*edit).scroll -= 1
        }
        return;
    }
    if key == K_HOME as libc::c_int
        || key == K_KP_HOME as libc::c_int
        || tolower(key) == 'a' as i32 && 0 != trap_Key_IsDown(K_CTRL as libc::c_int) as libc::c_uint
    {
        (*edit).cursor = 0i32;
        (*edit).scroll = 0i32;
        return;
    }
    if key == K_END as libc::c_int
        || key == K_KP_END as libc::c_int
        || tolower(key) == 'e' as i32 && 0 != trap_Key_IsDown(K_CTRL as libc::c_int) as libc::c_uint
    {
        (*edit).cursor = len;
        (*edit).scroll = len - (*edit).widthInChars + 1i32;
        if (*edit).scroll < 0i32 {
            (*edit).scroll = 0i32
        }
        return;
    }
    if key == K_INS as libc::c_int || key == K_KP_INS as libc::c_int {
        trap_Key_SetOverstrikeMode(
            (0 == trap_Key_GetOverstrikeMode() as u64) as libc::c_int as qboolean,
        );
        return;
    };
}
/*
================
MField_Paste
================
*/
#[no_mangle]
pub unsafe extern "C" fn MField_Paste(mut edit: *mut mfield_t) {
    let mut pasteBuffer: [libc::c_char; 64] = [0; 64];
    let mut pasteLen: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    trap_GetClipboardData(pasteBuffer.as_mut_ptr(), 64i32);
    pasteLen = strlen(pasteBuffer.as_mut_ptr()) as libc::c_int;
    i = 0i32;
    while i < pasteLen {
        MField_CharEvent(edit, pasteBuffer[i as usize] as libc::c_int);
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn MField_CharEvent(mut edit: *mut mfield_t, mut ch: libc::c_int) {
    let mut len: libc::c_int = 0;
    if ch == 'v' as i32 - 'a' as i32 + 1i32 {
        MField_Paste(edit);
        return;
    }
    if ch == 'c' as i32 - 'a' as i32 + 1i32 {
        MField_Clear(edit);
        return;
    }
    len = strlen((*edit).buffer.as_mut_ptr()) as libc::c_int;
    if ch == 'h' as i32 - 'a' as i32 + 1i32 {
        if (*edit).cursor > 0i32 {
            memmove(
                (*edit)
                    .buffer
                    .as_mut_ptr()
                    .offset((*edit).cursor as isize)
                    .offset(-1isize) as *mut libc::c_void,
                (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize) as *const libc::c_void,
                (len + 1i32 - (*edit).cursor) as libc::c_ulong,
            );
            (*edit).cursor -= 1;
            if (*edit).cursor < (*edit).scroll {
                (*edit).scroll -= 1
            }
        }
        return;
    }
    if ch == 'a' as i32 - 'a' as i32 + 1i32 {
        (*edit).cursor = 0i32;
        (*edit).scroll = 0i32;
        return;
    }
    if ch == 'e' as i32 - 'a' as i32 + 1i32 {
        (*edit).cursor = len;
        (*edit).scroll = (*edit).cursor - (*edit).widthInChars + 1i32;
        if (*edit).scroll < 0i32 {
            (*edit).scroll = 0i32
        }
        return;
    }
    if ch < 32i32 {
        return;
    }
    if 0 != trap_Key_GetOverstrikeMode() as u64 {
        if (*edit).cursor == 256i32 - 1i32
            || 0 != (*edit).maxchars && (*edit).cursor >= (*edit).maxchars
        {
            return;
        }
    } else {
        if len == 256i32 - 1i32 || 0 != (*edit).maxchars && len >= (*edit).maxchars {
            return;
        }
        memmove(
            (*edit)
                .buffer
                .as_mut_ptr()
                .offset((*edit).cursor as isize)
                .offset(1isize) as *mut libc::c_void,
            (*edit).buffer.as_mut_ptr().offset((*edit).cursor as isize) as *const libc::c_void,
            (len + 1i32 - (*edit).cursor) as libc::c_ulong,
        );
    }
    (*edit).buffer[(*edit).cursor as usize] = ch as libc::c_char;
    if 0 == (*edit).maxchars || (*edit).cursor < (*edit).maxchars - 1i32 {
        (*edit).cursor += 1
    }
    if (*edit).cursor >= (*edit).widthInChars {
        (*edit).scroll += 1
    }
    if (*edit).cursor == len + 1i32 {
        (*edit).buffer[(*edit).cursor as usize] = 0i32 as libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn MField_Draw(
    mut edit: *mut mfield_t,
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut style: libc::c_int,
    mut color: *mut vec_t,
) {
    let mut len: libc::c_int = 0;
    let mut charw: libc::c_int = 0;
    let mut drawLen: libc::c_int = 0;
    let mut prestep: libc::c_int = 0;
    let mut cursorChar: libc::c_int = 0;
    let mut str: [libc::c_char; 1024] = [0; 1024];
    drawLen = (*edit).widthInChars;
    len = strlen((*edit).buffer.as_mut_ptr()).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
    if len <= drawLen {
        prestep = 0i32
    } else {
        if (*edit).scroll + drawLen > len {
            (*edit).scroll = len - drawLen;
            if (*edit).scroll < 0i32 {
                (*edit).scroll = 0i32
            }
        }
        prestep = (*edit).scroll
    }
    if prestep + drawLen > len {
        drawLen = len - prestep
    }
    if drawLen >= 1024i32 {
        trap_Error(b"drawLen >= MAX_STRING_CHARS\x00" as *const u8 as *const libc::c_char);
    }
    memcpy(
        str.as_mut_ptr() as *mut libc::c_void,
        (*edit).buffer.as_mut_ptr().offset(prestep as isize) as *const libc::c_void,
        drawLen as libc::c_ulong,
    );
    str[drawLen as usize] = 0i32 as libc::c_char;
    UI_DrawString(x, y, str.as_mut_ptr(), style, color);
    if 0 == style & 0x4000i32 {
        return;
    }
    if 0 != trap_Key_GetOverstrikeMode() as u64 {
        cursorChar = 11i32
    } else {
        cursorChar = 10i32
    }
    style &= !0x4000i32;
    style |= 0x1000i32;
    if 0 != style & 0x10i32 {
        charw = 8i32
    } else if 0 != style & 0x40i32 {
        charw = 32i32
    } else {
        charw = 16i32
    }
    if 0 != style & 0x1i32 {
        len = strlen(str.as_mut_ptr()) as libc::c_int;
        x = x - len * charw / 2i32
    } else if 0 != style & 0x2i32 {
        len = strlen(str.as_mut_ptr()) as libc::c_int;
        x = x - len * charw
    }
    UI_DrawChar(
        x + ((*edit).cursor - prestep) * charw,
        y,
        cursorChar,
        style & !(0x1i32 | 0x2i32),
        color,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MenuField_Init(mut m: *mut menufield_s) {
    let mut l: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    MField_Clear(&mut (*m).field);
    if 0 != (*m).generic.flags & 0x2i32 as libc::c_uint {
        w = 8i32;
        h = 16i32
    } else {
        w = 16i32;
        h = 16i32
    }
    if !(*m).generic.name.is_null() {
        l = strlen((*m).generic.name)
            .wrapping_add(1i32 as libc::c_ulong)
            .wrapping_mul(w as libc::c_ulong) as libc::c_int
    } else {
        l = 0i32
    }
    (*m).generic.left = (*m).generic.x - l;
    (*m).generic.top = (*m).generic.y;
    (*m).generic.right = (*m).generic.x + w + (*m).field.widthInChars * w;
    (*m).generic.bottom = (*m).generic.y + h;
}
#[no_mangle]
pub unsafe extern "C" fn MenuField_Draw(mut f: *mut menufield_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut focus: qboolean = qfalse;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    x = (*f).generic.x;
    y = (*f).generic.y;
    if 0 != (*f).generic.flags & 0x2i32 as libc::c_uint {
        w = 8i32;
        style = 0x10i32
    } else {
        w = 16i32;
        style = 0x20i32
    }
    if Menu_ItemAtCursor((*f).generic.parent) == f as *mut libc::c_void {
        focus = qtrue;
        style |= 0x4000i32
    } else {
        focus = qfalse
    }
    if 0 != (*f).generic.flags & 0x2000i32 as libc::c_uint {
        color = text_color_disabled.as_mut_ptr()
    } else if 0 != focus as u64 {
        color = text_color_highlight.as_mut_ptr()
    } else {
        color = text_color_normal.as_mut_ptr()
    }
    if 0 != focus as u64 {
        UI_FillRect(
            (*f).generic.left as libc::c_float,
            (*f).generic.top as libc::c_float,
            ((*f).generic.right - (*f).generic.left + 1i32) as libc::c_float,
            ((*f).generic.bottom - (*f).generic.top + 1i32) as libc::c_float,
            listbar_color.as_mut_ptr(),
        );
        UI_DrawChar(x, y, 13i32, 0x1i32 | 0x1000i32 | style, color);
    }
    if !(*f).generic.name.is_null() {
        UI_DrawString(x - w, y, (*f).generic.name, style | 0x2i32, color);
    }
    MField_Draw(&mut (*f).field, x + w, y, style, color);
}
#[no_mangle]
pub unsafe extern "C" fn MenuField_Key(
    mut m: *mut menufield_s,
    mut key: *mut libc::c_int,
) -> sfxHandle_t {
    let mut keycode: libc::c_int = 0;
    keycode = *key;
    match keycode {
        169 | 13 | 185 | 186 | 187 | 188 => *key = K_TAB as libc::c_int,
        9 | 167 | 133 | 161 | 132 => {}
        _ => {
            if 0 != keycode & 1024i32 {
                keycode &= !1024i32;
                if 0 != (*m).generic.flags & 0x80000i32 as libc::c_uint && 0 != Q_islower(keycode) {
                    keycode -= 'a' as i32 - 'A' as i32
                } else if 0 != (*m).generic.flags & 0x40000i32 as libc::c_uint
                    && 0 != Q_isupper(keycode)
                {
                    keycode -= 'A' as i32 - 'a' as i32
                } else if 0 != (*m).generic.flags & 0x20i32 as libc::c_uint
                    && 0 != Q_isalpha(keycode)
                {
                    return menu_buzz_sound;
                }
                MField_CharEvent(&mut (*m).field, keycode);
            } else {
                MField_KeyDownEvent(&mut (*m).field, keycode);
            }
        }
    }
    return 0i32;
}
