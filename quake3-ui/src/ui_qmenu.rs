use keycodes_h::{
    unnamed_0, K_ALT, K_AUX1, K_AUX10, K_AUX11, K_AUX12, K_AUX13, K_AUX14, K_AUX15, K_AUX16,
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
use libc;
use q_shared_h::{
    colorMdGrey, colorWhite, colorYellow, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed,
    va, vec4_t, vec_t, Q_isprint, Q_isupper, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::{sin, strcat, strcpy, strlen};
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
    _tag_menuframework, menuaction_s, menubitmap_s, menucommon_s, menufield_s, menuframework_s,
    menulist_s, menuradiobutton_s, menuslider_s, menutext_s, mfield_t, trap_Cmd_ExecuteText,
    trap_Error, trap_R_RegisterShaderNoMip, trap_R_SetColor, trap_S_RegisterSound, uiStatic_t,
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
pub unsafe extern "C" fn Menu_Cache() {
    uis.charset =
        trap_R_RegisterShaderNoMip(b"gfx/2d/bigchars\x00" as *const u8 as *const libc::c_char);
    uis.charsetProp = trap_R_RegisterShaderNoMip(
        b"menu/art/font1_prop.tga\x00" as *const u8 as *const libc::c_char,
    );
    uis.charsetPropGlow = trap_R_RegisterShaderNoMip(
        b"menu/art/font1_prop_glo.tga\x00" as *const u8 as *const libc::c_char,
    );
    uis.charsetPropB = trap_R_RegisterShaderNoMip(
        b"menu/art/font2_prop.tga\x00" as *const u8 as *const libc::c_char,
    );
    uis.cursor =
        trap_R_RegisterShaderNoMip(b"menu/art/3_cursor2\x00" as *const u8 as *const libc::c_char);
    uis.rb_on =
        trap_R_RegisterShaderNoMip(b"menu/art/switch_on\x00" as *const u8 as *const libc::c_char);
    uis.rb_off =
        trap_R_RegisterShaderNoMip(b"menu/art/switch_off\x00" as *const u8 as *const libc::c_char);
    uis.whiteShader = trap_R_RegisterShaderNoMip(b"white\x00" as *const u8 as *const libc::c_char);
    if uis.glconfig.hardwareType as libc::c_uint == GLHW_RAGEPRO as libc::c_int as libc::c_uint {
        uis.menuBackShader =
            trap_R_RegisterShaderNoMip(b"menubackRagePro\x00" as *const u8 as *const libc::c_char)
    } else {
        uis.menuBackShader =
            trap_R_RegisterShaderNoMip(b"menuback\x00" as *const u8 as *const libc::c_char)
    }
    uis.menuBackNoLogoShader =
        trap_R_RegisterShaderNoMip(b"menubacknologo\x00" as *const u8 as *const libc::c_char);
    menu_in_sound = trap_S_RegisterSound(
        b"sound/misc/menu1.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    menu_move_sound = trap_S_RegisterSound(
        b"sound/misc/menu2.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    menu_out_sound = trap_S_RegisterSound(
        b"sound/misc/menu3.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    menu_buzz_sound = trap_S_RegisterSound(
        b"sound/misc/menu4.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    weaponChangeSound = trap_S_RegisterSound(
        b"sound/weapons/change.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    menu_null_sound = -1i32;
    sliderBar =
        trap_R_RegisterShaderNoMip(b"menu/art/slider2\x00" as *const u8 as *const libc::c_char);
    sliderButton_0 = trap_R_RegisterShaderNoMip(
        b"menu/art/sliderbutt_0\x00" as *const u8 as *const libc::c_char,
    );
    sliderButton_1 = trap_R_RegisterShaderNoMip(
        b"menu/art/sliderbutt_1\x00" as *const u8 as *const libc::c_char,
    );
}
static mut sliderButton_1: qhandle_t = 0;
static mut sliderButton_0: qhandle_t = 0;
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
/**********************************************************************
    UI_QMENU.C

    Quake's menu framework system.
**********************************************************************/
static mut sliderBar: qhandle_t = 0;
#[no_mangle]
pub static mut menu_null_sound: sfxHandle_t = 0;
#[no_mangle]
pub static mut weaponChangeSound: sfxHandle_t = 0;
#[no_mangle]
pub static mut menu_buzz_sound: sfxHandle_t = 0;
#[no_mangle]
pub static mut menu_out_sound: sfxHandle_t = 0;
#[no_mangle]
pub static mut menu_move_sound: sfxHandle_t = 0;
#[no_mangle]
pub static mut menu_in_sound: sfxHandle_t = 0;
#[no_mangle]
pub unsafe extern "C" fn Menu_AddItem(mut menu: *mut menuframework_s, mut item: *mut libc::c_void) {
    let mut itemptr: *mut menucommon_s = 0 as *mut menucommon_s;
    if (*menu).nitems >= 64i32 {
        trap_Error(b"Menu_AddItem: excessive items\x00" as *const u8 as *const libc::c_char);
    }
    (*menu).items[(*menu).nitems as usize] = item;
    let ref mut fresh0 = (*((*menu).items[(*menu).nitems as usize] as *mut menucommon_s)).parent;
    *fresh0 = menu;
    (*((*menu).items[(*menu).nitems as usize] as *mut menucommon_s)).menuPosition = (*menu).nitems;
    (*((*menu).items[(*menu).nitems as usize] as *mut menucommon_s)).flags &=
        !(0x200i32 as libc::c_uint);
    itemptr = item as *mut menucommon_s;
    if 0 == (*itemptr).flags & 0x8000i32 as libc::c_uint {
        match (*itemptr).type_0 {
            2 => {
                Action_Init(item as *mut menuaction_s);
            }
            4 => {
                MenuField_Init(item as *mut menufield_s);
            }
            3 => {
                SpinControl_Init(item as *mut menulist_s);
            }
            5 => {
                RadioButton_Init(item as *mut menuradiobutton_s);
            }
            1 => {
                Slider_Init(item as *mut menuslider_s);
            }
            6 => {
                Bitmap_Init(item as *mut menubitmap_s);
            }
            7 => {
                Text_Init(item as *mut menutext_s);
            }
            8 => {
                ScrollList_Init(item as *mut menulist_s);
            }
            9 => {
                PText_Init(item as *mut menutext_s);
            }
            10 => {
                BText_Init(item as *mut menutext_s);
            }
            _ => {
                trap_Error(va(
                    b"Menu_Init: unknown type %d\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    (*itemptr).type_0,
                ));
            }
        }
    }
    (*menu).nitems += 1;
}
// proportional banner text widget
unsafe extern "C" fn BText_Init(mut t: *mut menutext_s) {
    (*t).generic.flags |= 0x4000i32 as libc::c_uint;
}
// proportional text widget
unsafe extern "C" fn PText_Init(mut t: *mut menutext_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut sizeScale: libc::c_float = 0.;
    sizeScale = UI_ProportionalSizeScale((*t).style);
    x = (*t).generic.x;
    y = (*t).generic.y;
    w = (UI_ProportionalStringWidth((*t).string) as libc::c_float * sizeScale) as libc::c_int;
    h = (27i32 as libc::c_float * sizeScale) as libc::c_int;
    if 0 != (*t).generic.flags & 0x10i32 as libc::c_uint {
        x -= w
    } else if 0 != (*t).generic.flags & 0x8i32 as libc::c_uint {
        x -= w / 2i32
    }
    (*t).generic.left = (x as libc::c_float - 3i32 as libc::c_float * sizeScale) as libc::c_int;
    (*t).generic.right =
        ((x + w) as libc::c_float + 3i32 as libc::c_float * sizeScale) as libc::c_int;
    (*t).generic.top = y;
    (*t).generic.bottom = y + h;
}
// scrolllist widget
unsafe extern "C" fn ScrollList_Init(mut l: *mut menulist_s) {
    let mut w: libc::c_int = 0;
    (*l).oldvalue = 0i32;
    (*l).curvalue = 0i32;
    (*l).top = 0i32;
    if 0 == (*l).columns {
        (*l).columns = 1i32;
        (*l).separation = 0i32
    } else if 0 == (*l).separation {
        (*l).separation = 3i32
    }
    w = (((*l).width + (*l).separation) * (*l).columns - (*l).separation) * 8i32;
    (*l).generic.left = (*l).generic.x;
    (*l).generic.top = (*l).generic.y;
    (*l).generic.right = (*l).generic.x + w;
    (*l).generic.bottom = (*l).generic.y + (*l).height * 16i32;
    if 0 != (*l).generic.flags & 0x8i32 as libc::c_uint {
        (*l).generic.left -= w / 2i32;
        (*l).generic.right -= w / 2i32
    };
}
// text widget
unsafe extern "C" fn Text_Init(mut t: *mut menutext_s) {
    (*t).generic.flags |= 0x4000i32 as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn Bitmap_Init(mut b: *mut menubitmap_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    x = (*b).generic.x;
    y = (*b).generic.y;
    w = (*b).width;
    h = (*b).height;
    if w < 0i32 {
        w = -w
    }
    if h < 0i32 {
        h = -h
    }
    if 0 != (*b).generic.flags & 0x10i32 as libc::c_uint {
        x = x - w
    } else if 0 != (*b).generic.flags & 0x8i32 as libc::c_uint {
        x = x - w / 2i32
    }
    (*b).generic.left = x;
    (*b).generic.right = x + w;
    (*b).generic.top = y;
    (*b).generic.bottom = y + h;
    (*b).shader = 0i32;
    (*b).focusshader = 0i32;
}
// slider widget
unsafe extern "C" fn Slider_Init(mut s: *mut menuslider_s) {
    let mut len: libc::c_int = 0;
    if !(*s).generic.name.is_null() {
        len = strlen((*s).generic.name) as libc::c_int
    } else {
        len = 0i32
    }
    (*s).generic.left = (*s).generic.x - (len + 1i32) * 8i32;
    (*s).generic.right = (*s).generic.x + (10i32 + 2i32 + 1i32) * 8i32;
    (*s).generic.top = (*s).generic.y;
    (*s).generic.bottom = (*s).generic.y + 16i32;
}
// radio button widget
unsafe extern "C" fn RadioButton_Init(mut rb: *mut menuradiobutton_s) {
    let mut len: libc::c_int = 0;
    if !(*rb).generic.name.is_null() {
        len = strlen((*rb).generic.name) as libc::c_int
    } else {
        len = 0i32
    }
    (*rb).generic.left = (*rb).generic.x - (len + 1i32) * 8i32;
    (*rb).generic.right = (*rb).generic.x + 6i32 * 8i32;
    (*rb).generic.top = (*rb).generic.y;
    (*rb).generic.bottom = (*rb).generic.y + 16i32;
}
// spin control widget
unsafe extern "C" fn SpinControl_Init(mut s: *mut menulist_s) {
    let mut len: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    if !(*s).generic.name.is_null() {
        len = strlen((*s).generic.name).wrapping_mul(8i32 as libc::c_ulong) as libc::c_int
    } else {
        len = 0i32
    }
    (*s).generic.left = (*s).generic.x - 8i32 - len;
    (*s).numitems = 0i32;
    len = (*s).numitems;
    loop {
        str = *(*s).itemnames.offset((*s).numitems as isize);
        if str.is_null() {
            break;
        }
        l = strlen(str) as libc::c_int;
        if l > len {
            len = l
        }
        (*s).numitems += 1
    }
    (*s).generic.top = (*s).generic.y;
    (*s).generic.right = (*s).generic.x + (len + 1i32) * 8i32;
    (*s).generic.bottom = (*s).generic.y + 16i32;
}
// action widget
unsafe extern "C" fn Action_Init(mut a: *mut menuaction_s) {
    let mut len: libc::c_int = 0;
    if !(*a).generic.name.is_null() {
        len = strlen((*a).generic.name) as libc::c_int
    } else {
        len = 0i32
    }
    (*a).generic.left = (*a).generic.x;
    (*a).generic.right = (*a).generic.x + len * 16i32;
    (*a).generic.top = (*a).generic.y;
    (*a).generic.bottom = (*a).generic.y + 16i32;
}
#[no_mangle]
pub unsafe extern "C" fn Menu_AdjustCursor(mut m: *mut menuframework_s, mut dir: libc::c_int) {
    let mut item: *mut menucommon_s = 0 as *mut menucommon_s;
    let mut wrapped: qboolean = qfalse;
    loop {
        while (*m).cursor >= 0i32 && (*m).cursor < (*m).nitems {
            item = (*m).items[(*m).cursor as usize] as *mut menucommon_s;
            if !(0
                != (*item).flags
                    & (0x2000i32 as libc::c_uint
                        | 0x800i32 as libc::c_uint
                        | 0x4000i32 as libc::c_uint))
            {
                break;
            }
            (*m).cursor += dir
        }
        if dir == 1i32 {
            if !((*m).cursor >= (*m).nitems) {
                break;
            }
            if 0 != (*m).wrapAround as u64 {
                if 0 != wrapped as u64 {
                    (*m).cursor = (*m).cursor_prev;
                    return;
                }
                (*m).cursor = 0i32;
                wrapped = qtrue
            } else {
                (*m).cursor = (*m).cursor_prev;
                break;
            }
        } else {
            if !((*m).cursor < 0i32) {
                break;
            }
            if 0 != (*m).wrapAround as u64 {
                if 0 != wrapped as u64 {
                    (*m).cursor = (*m).cursor_prev;
                    return;
                }
                (*m).cursor = (*m).nitems - 1i32;
                wrapped = qtrue
            } else {
                (*m).cursor = (*m).cursor_prev;
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Menu_Draw(mut menu: *mut menuframework_s) {
    let mut i: libc::c_int = 0;
    let mut itemptr: *mut menucommon_s = 0 as *mut menucommon_s;
    i = 0i32;
    while i < (*menu).nitems {
        itemptr = (*menu).items[i as usize] as *mut menucommon_s;
        if !(0 != (*itemptr).flags & 0x1000i32 as libc::c_uint) {
            if (*itemptr).ownerdraw.is_some() {
                (*itemptr).ownerdraw.expect("non-null function pointer")(
                    itemptr as *mut libc::c_void,
                );
            } else {
                match (*itemptr).type_0 {
                    5 => {
                        RadioButton_Draw(itemptr as *mut menuradiobutton_s);
                    }
                    4 => {
                        MenuField_Draw(itemptr as *mut menufield_s);
                    }
                    1 => {
                        Slider_Draw(itemptr as *mut menuslider_s);
                    }
                    3 => {
                        SpinControl_Draw(itemptr as *mut menulist_s);
                    }
                    2 => {
                        Action_Draw(itemptr as *mut menuaction_s);
                    }
                    6 => {
                        Bitmap_Draw(itemptr as *mut menubitmap_s);
                    }
                    7 => {
                        Text_Draw(itemptr as *mut menutext_s);
                    }
                    8 => {
                        ScrollList_Draw(itemptr as *mut menulist_s);
                    }
                    9 => {
                        PText_Draw(itemptr as *mut menutext_s);
                    }
                    10 => {
                        BText_Draw(itemptr as *mut menutext_s);
                    }
                    _ => {
                        trap_Error(va(
                            b"Menu_Draw: unknown type %d\x00" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            (*itemptr).type_0,
                        ));
                    }
                }
            }
            if 0 != uis.debug as u64 {
                let mut x: libc::c_int = 0;
                let mut y: libc::c_int = 0;
                let mut w: libc::c_int = 0;
                let mut h: libc::c_int = 0;
                if 0 == (*itemptr).flags & 0x4000i32 as libc::c_uint {
                    x = (*itemptr).left;
                    y = (*itemptr).top;
                    w = (*itemptr).right - (*itemptr).left + 1i32;
                    h = (*itemptr).bottom - (*itemptr).top + 1i32;
                    if 0 != (*itemptr).flags & 0x200i32 as libc::c_uint {
                        UI_DrawRect(
                            x as libc::c_float,
                            y as libc::c_float,
                            w as libc::c_float,
                            h as libc::c_float,
                            colorYellow.as_mut_ptr(),
                        );
                    } else {
                        UI_DrawRect(
                            x as libc::c_float,
                            y as libc::c_float,
                            w as libc::c_float,
                            h as libc::c_float,
                            colorWhite.as_mut_ptr(),
                        );
                    }
                }
            }
        }
        i += 1
    }
    itemptr = Menu_ItemAtCursor(menu) as *mut menucommon_s;
    if !itemptr.is_null() && (*itemptr).statusbar.is_some() {
        (*itemptr).statusbar.expect("non-null function pointer")(itemptr as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn Menu_ItemAtCursor(mut m: *mut menuframework_s) -> *mut libc::c_void {
    if (*m).cursor < 0i32 || (*m).cursor >= (*m).nitems {
        return 0 as *mut libc::c_void;
    }
    return (*m).items[(*m).cursor as usize];
}
unsafe extern "C" fn BText_Draw(mut t: *mut menutext_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    x = (*t).generic.x;
    y = (*t).generic.y;
    if 0 != (*t).generic.flags & 0x2000i32 as libc::c_uint {
        color = text_color_disabled.as_mut_ptr()
    } else {
        color = (*t).color
    }
    UI_DrawBannerString(x, y, (*t).string, (*t).style, color);
}
#[no_mangle]
pub static mut text_color_disabled: vec4_t = [0.50f32, 0.50f32, 0.50f32, 1.00f32];
unsafe extern "C" fn PText_Draw(mut t: *mut menutext_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut style: libc::c_int = 0;
    x = (*t).generic.x;
    y = (*t).generic.y;
    if 0 != (*t).generic.flags & 0x2000i32 as libc::c_uint {
        color = text_color_disabled.as_mut_ptr()
    } else {
        color = (*t).color
    }
    style = (*t).style;
    if 0 != (*t).generic.flags & 0x100i32 as libc::c_uint {
        if Menu_ItemAtCursor((*t).generic.parent) == t as *mut libc::c_void {
            style |= 0x4000i32
        } else {
            style |= 0x2000i32
        }
    }
    UI_DrawProportionalString(x, y, (*t).string, style, color);
}
#[no_mangle]
pub unsafe extern "C" fn ScrollList_Draw(mut l: *mut menulist_s) {
    let mut x: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut base: libc::c_int = 0;
    let mut column: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut hasfocus: qboolean = qfalse;
    let mut style: libc::c_int = 0;
    hasfocus =
        ((*(*l).generic.parent).cursor == (*l).generic.menuPosition) as libc::c_int as qboolean;
    x = (*l).generic.x;
    column = 0i32;
    while column < (*l).columns {
        y = (*l).generic.y;
        base = (*l).top + column * (*l).height;
        i = base;
        while i < base + (*l).height {
            if i >= (*l).numitems {
                break;
            }
            if i == (*l).curvalue {
                u = x - 2i32;
                if 0 != (*l).generic.flags & 0x8i32 as libc::c_uint {
                    u -= (*l).width * 8i32 / 2i32 + 1i32
                }
                UI_FillRect(
                    u as libc::c_float,
                    y as libc::c_float,
                    ((*l).width * 8i32) as libc::c_float,
                    (16i32 + 2i32) as libc::c_float,
                    listbar_color.as_mut_ptr(),
                );
                color = text_color_highlight.as_mut_ptr();
                if 0 != hasfocus as u64 {
                    style = 0x4000i32 | 0i32 | 0x10i32
                } else {
                    style = 0i32 | 0x10i32
                }
            } else {
                color = text_color_normal.as_mut_ptr();
                style = 0i32 | 0x10i32
            }
            if 0 != (*l).generic.flags & 0x8i32 as libc::c_uint {
                style |= 0x1i32
            }
            UI_DrawString(x, y, *(*l).itemnames.offset(i as isize), style, color);
            y += 16i32;
            i += 1
        }
        x += ((*l).width + (*l).separation) * 8i32;
        column += 1
    }
}
#[no_mangle]
pub static mut text_color_normal: vec4_t = [1.00f32, 0.43f32, 0.00f32, 1.00f32];
#[no_mangle]
pub static mut text_color_highlight: vec4_t = [1.00f32, 1.00f32, 0.00f32, 1.00f32];
#[no_mangle]
pub static mut listbar_color: vec4_t = [1.00f32, 0.43f32, 0.00f32, 0.30f32];
unsafe extern "C" fn Text_Draw(mut t: *mut menutext_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut buff: [libc::c_char; 512] = [0; 512];
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    x = (*t).generic.x;
    y = (*t).generic.y;
    buff[0usize] = '\u{0}' as i32 as libc::c_char;
    if !(*t).generic.name.is_null() {
        strcpy(buff.as_mut_ptr(), (*t).generic.name);
    }
    if !(*t).string.is_null() {
        strcat(buff.as_mut_ptr(), (*t).string);
    }
    if 0 != (*t).generic.flags & 0x2000i32 as libc::c_uint {
        color = text_color_disabled.as_mut_ptr()
    } else {
        color = (*t).color
    }
    UI_DrawString(x, y, buff.as_mut_ptr(), (*t).style, color);
}
#[no_mangle]
pub unsafe extern "C" fn Bitmap_Draw(mut b: *mut menubitmap_s) {
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut tempcolor: vec4_t = [0.; 4];
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    x = (*b).generic.x as libc::c_float;
    y = (*b).generic.y as libc::c_float;
    w = (*b).width as libc::c_float;
    h = (*b).height as libc::c_float;
    if 0 != (*b).generic.flags & 0x10i32 as libc::c_uint {
        x = x - w
    } else if 0 != (*b).generic.flags & 0x8i32 as libc::c_uint {
        x = x - w / 2i32 as libc::c_float
    }
    if !(*b).generic.name.is_null() && 0 == (*b).shader {
        (*b).shader = trap_R_RegisterShaderNoMip((*b).generic.name);
        if 0 == (*b).shader && !(*b).errorpic.is_null() {
            (*b).shader = trap_R_RegisterShaderNoMip((*b).errorpic)
        }
    }
    if !(*b).focuspic.is_null() && 0 == (*b).focusshader {
        (*b).focusshader = trap_R_RegisterShaderNoMip((*b).focuspic)
    }
    if 0 != (*b).generic.flags & 0x2000i32 as libc::c_uint {
        if 0 != (*b).shader {
            trap_R_SetColor(colorMdGrey.as_mut_ptr());
            UI_DrawHandlePic(x, y, w, h, (*b).shader);
            trap_R_SetColor(0 as *const libc::c_float);
        }
    } else {
        if 0 != (*b).shader {
            UI_DrawHandlePic(x, y, w, h, (*b).shader);
        }
        if (0 != (*b).generic.flags & 0x20000i32 as libc::c_uint
            || 0 != (*b).generic.flags & 0x100i32 as libc::c_uint)
            && Menu_ItemAtCursor((*b).generic.parent) == b as *mut libc::c_void
        {
            if !(*b).focuscolor.is_null() {
                tempcolor[0usize] = *(*b).focuscolor.offset(0isize);
                tempcolor[1usize] = *(*b).focuscolor.offset(1isize);
                tempcolor[2usize] = *(*b).focuscolor.offset(2isize);
                color = tempcolor.as_mut_ptr()
            } else {
                color = pulse_color.as_mut_ptr()
            }
            *color.offset(3isize) =
                (0.5f64 + 0.5f64 * sin((uis.realtime / 75i32) as libc::c_double)) as libc::c_float;
            trap_R_SetColor(color);
            UI_DrawHandlePic(x, y, w, h, (*b).focusshader);
            trap_R_SetColor(0 as *const libc::c_float);
        } else if 0 != (*b).generic.flags & 0x40i32 as libc::c_uint
            || 0 != (*b).generic.flags & 0x80i32 as libc::c_uint
                && Menu_ItemAtCursor((*b).generic.parent) == b as *mut libc::c_void
        {
            if !(*b).focuscolor.is_null() {
                trap_R_SetColor((*b).focuscolor);
                UI_DrawHandlePic(x, y, w, h, (*b).focusshader);
                trap_R_SetColor(0 as *const libc::c_float);
            } else {
                UI_DrawHandlePic(x, y, w, h, (*b).focusshader);
            }
        }
    };
}
// current color scheme
#[no_mangle]
pub static mut pulse_color: vec4_t = [1.00f32, 1.00f32, 1.00f32, 1.00f32];
unsafe extern "C" fn Action_Draw(mut a: *mut menuaction_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    style = 0i32;
    color = menu_text_color.as_mut_ptr();
    if 0 != (*a).generic.flags & 0x2000i32 as libc::c_uint {
        color = text_color_disabled.as_mut_ptr()
    } else if 0 != (*a).generic.flags & 0x100i32 as libc::c_uint
        && (*(*a).generic.parent).cursor == (*a).generic.menuPosition
    {
        color = text_color_highlight.as_mut_ptr();
        style = 0x4000i32
    } else if 0 != (*a).generic.flags & 0x80i32 as libc::c_uint
        && (*(*a).generic.parent).cursor == (*a).generic.menuPosition
    {
        color = text_color_highlight.as_mut_ptr()
    } else if 0 != (*a).generic.flags & 0x1i32 as libc::c_uint {
        style = 0x1000i32;
        color = text_color_highlight.as_mut_ptr()
    }
    x = (*a).generic.x;
    y = (*a).generic.y;
    UI_DrawString(x, y, (*a).generic.name, 0i32 | style, color);
    if (*(*a).generic.parent).cursor == (*a).generic.menuPosition {
        UI_DrawChar(x - 16i32, y, 13i32, 0i32 | 0x1000i32, color);
    };
}
#[no_mangle]
pub static mut menu_text_color: vec4_t = [1.0f32, 1.0f32, 1.0f32, 1.0f32];
unsafe extern "C" fn SpinControl_Draw(mut s: *mut menulist_s) {
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut focus: qboolean = qfalse;
    x = (*s).generic.x;
    y = (*s).generic.y;
    style = 0x10i32;
    focus = ((*(*s).generic.parent).cursor == (*s).generic.menuPosition) as libc::c_int as qboolean;
    if 0 != (*s).generic.flags & 0x2000i32 as libc::c_uint {
        color = text_color_disabled.as_mut_ptr()
    } else if 0 != focus as u64 {
        color = text_color_highlight.as_mut_ptr();
        style |= 0x4000i32
    } else if 0 != (*s).generic.flags & 0x1i32 as libc::c_uint {
        color = text_color_highlight.as_mut_ptr();
        style |= 0x1000i32
    } else {
        color = text_color_normal.as_mut_ptr()
    }
    if 0 != focus as u64 {
        UI_FillRect(
            (*s).generic.left as libc::c_float,
            (*s).generic.top as libc::c_float,
            ((*s).generic.right - (*s).generic.left + 1i32) as libc::c_float,
            ((*s).generic.bottom - (*s).generic.top + 1i32) as libc::c_float,
            listbar_color.as_mut_ptr(),
        );
        UI_DrawChar(x, y, 13i32, 0x1i32 | 0x1000i32 | 0x10i32, color);
    }
    UI_DrawString(x - 8i32, y, (*s).generic.name, style | 0x2i32, color);
    UI_DrawString(
        x + 8i32,
        y,
        *(*s).itemnames.offset((*s).curvalue as isize),
        style | 0i32,
        color,
    );
}
unsafe extern "C" fn Slider_Draw(mut s: *mut menuslider_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut button: libc::c_int = 0;
    let mut focus: qboolean = qfalse;
    x = (*s).generic.x;
    y = (*s).generic.y;
    focus = ((*(*s).generic.parent).cursor == (*s).generic.menuPosition) as libc::c_int as qboolean;
    if 0 != (*s).generic.flags & 0x2000i32 as libc::c_uint {
        color = text_color_disabled.as_mut_ptr();
        style = 0x10i32
    } else if 0 != focus as u64 {
        color = text_color_highlight.as_mut_ptr();
        style = 0x10i32 | 0x4000i32
    } else {
        color = text_color_normal.as_mut_ptr();
        style = 0x10i32
    }
    UI_DrawString(x - 8i32, y, (*s).generic.name, 0x2i32 | style, color);
    UI_SetColor(color);
    UI_DrawHandlePic(
        (x + 8i32) as libc::c_float,
        y as libc::c_float,
        96i32 as libc::c_float,
        16i32 as libc::c_float,
        sliderBar,
    );
    UI_SetColor(0 as *const libc::c_float);
    if (*s).maxvalue > (*s).minvalue {
        (*s).range = ((*s).curvalue - (*s).minvalue) / ((*s).maxvalue - (*s).minvalue);
        if (*s).range < 0i32 as libc::c_float {
            (*s).range = 0i32 as libc::c_float
        } else if (*s).range > 1i32 as libc::c_float {
            (*s).range = 1i32 as libc::c_float
        }
    } else {
        (*s).range = 0i32 as libc::c_float
    }
    if 0 != style & 0x4000i32 {
        button = sliderButton_1
    } else {
        button = sliderButton_0
    }
    UI_DrawHandlePic(
        (((x + 2i32 * 8i32) as libc::c_float
            + ((10i32 - 1i32) * 8i32) as libc::c_float * (*s).range) as libc::c_int
            - 2i32) as libc::c_float,
        (y - 2i32) as libc::c_float,
        12i32 as libc::c_float,
        20i32 as libc::c_float,
        button,
    );
}
unsafe extern "C" fn RadioButton_Draw(mut rb: *mut menuradiobutton_s) {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut style: libc::c_int = 0;
    let mut focus: qboolean = qfalse;
    x = (*rb).generic.x;
    y = (*rb).generic.y;
    focus =
        ((*(*rb).generic.parent).cursor == (*rb).generic.menuPosition) as libc::c_int as qboolean;
    if 0 != (*rb).generic.flags & 0x2000i32 as libc::c_uint {
        color = text_color_disabled.as_mut_ptr();
        style = 0i32 | 0x10i32
    } else if 0 != focus as u64 {
        color = text_color_highlight.as_mut_ptr();
        style = 0i32 | 0x4000i32 | 0x10i32
    } else {
        color = text_color_normal.as_mut_ptr();
        style = 0i32 | 0x10i32
    }
    if 0 != focus as u64 {
        UI_FillRect(
            (*rb).generic.left as libc::c_float,
            (*rb).generic.top as libc::c_float,
            ((*rb).generic.right - (*rb).generic.left + 1i32) as libc::c_float,
            ((*rb).generic.bottom - (*rb).generic.top + 1i32) as libc::c_float,
            listbar_color.as_mut_ptr(),
        );
        UI_DrawChar(x, y, 13i32, 0x1i32 | 0x1000i32 | 0x10i32, color);
    }
    if !(*rb).generic.name.is_null() {
        UI_DrawString(x - 8i32, y, (*rb).generic.name, 0x2i32 | 0x10i32, color);
    }
    if 0 == (*rb).curvalue {
        UI_DrawHandlePic(
            (x + 8i32) as libc::c_float,
            (y + 2i32) as libc::c_float,
            16i32 as libc::c_float,
            16i32 as libc::c_float,
            uis.rb_off,
        );
        UI_DrawString(
            x + 8i32 + 16i32,
            y,
            b"off\x00" as *const u8 as *const libc::c_char,
            style,
            color,
        );
    } else {
        UI_DrawHandlePic(
            (x + 8i32) as libc::c_float,
            (y + 2i32) as libc::c_float,
            16i32 as libc::c_float,
            16i32 as libc::c_float,
            uis.rb_on,
        );
        UI_DrawString(
            x + 8i32 + 16i32,
            y,
            b"on\x00" as *const u8 as *const libc::c_char,
            style,
            color,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Menu_ActivateItem(
    mut s: *mut menuframework_s,
    mut item: *mut menucommon_s,
) -> sfxHandle_t {
    if (*item).callback.is_some() {
        (*item).callback.expect("non-null function pointer")(item as *mut libc::c_void, 3i32);
        if 0 == (*item).flags & 0x100000i32 as libc::c_uint {
            return menu_move_sound;
        }
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn Menu_SetCursor(mut m: *mut menuframework_s, mut cursor: libc::c_int) {
    if 0 != (*((*m).items[cursor as usize] as *mut menucommon_s)).flags
        & (0x2000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    {
        return;
    }
    (*m).cursor_prev = (*m).cursor;
    (*m).cursor = cursor;
    Menu_CursorMoved(m);
}
/*
=================
Menu_CursorMoved
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Menu_CursorMoved(mut m: *mut menuframework_s) {
    let mut callback: Option<unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> ()> =
        None;
    if (*m).cursor_prev == (*m).cursor {
        return;
    }
    if (*m).cursor_prev >= 0i32 && (*m).cursor_prev < (*m).nitems {
        callback = (*((*m).items[(*m).cursor_prev as usize] as *mut menucommon_s)).callback;
        if callback.is_some() {
            callback.expect("non-null function pointer")(
                (*m).items[(*m).cursor_prev as usize],
                2i32,
            );
        }
    }
    if (*m).cursor >= 0i32 && (*m).cursor < (*m).nitems {
        callback = (*((*m).items[(*m).cursor as usize] as *mut menucommon_s)).callback;
        if callback.is_some() {
            callback.expect("non-null function pointer")((*m).items[(*m).cursor as usize], 1i32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Menu_SetCursorToItem(
    mut m: *mut menuframework_s,
    mut ptr: *mut libc::c_void,
) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < (*m).nitems {
        if (*m).items[i as usize] == ptr {
            Menu_SetCursor(m, i);
            return;
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn Menu_DefaultKey(
    mut m: *mut menuframework_s,
    mut key: libc::c_int,
) -> sfxHandle_t {
    let mut sound: sfxHandle_t = 0i32;
    let mut item: *mut menucommon_s = 0 as *mut menucommon_s;
    let mut cursor_prev: libc::c_int = 0;
    match key {
        179 | 27 => {
            UI_PopMenu();
            return menu_out_sound;
        }
        _ => {}
    }
    if m.is_null() || 0 == (*m).nitems {
        return 0i32;
    }
    item = Menu_ItemAtCursor(m) as *mut menucommon_s;
    if !item.is_null()
        && 0 == (*item).flags & (0x2000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    {
        match (*item).type_0 {
            3 => sound = SpinControl_Key(item as *mut menulist_s, key),
            5 => sound = RadioButton_Key(item as *mut menuradiobutton_s, key),
            1 => sound = Slider_Key(item as *mut menuslider_s, key),
            8 => sound = ScrollList_Key(item as *mut menulist_s, key),
            4 => sound = MenuField_Key(item as *mut menufield_s, &mut key),
            _ => {}
        }
        if 0 != sound {
            return sound;
        }
    }
    match key {
        155 => {
            uis.debug = ::std::mem::transmute::<libc::c_uint, qboolean>(
                uis.debug as libc::c_uint ^ 1i32 as libc::c_uint,
            )
        }
        156 => {
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                b"screenshot\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        161 | 132 => {
            cursor_prev = (*m).cursor;
            (*m).cursor_prev = (*m).cursor;
            (*m).cursor -= 1;
            Menu_AdjustCursor(m, -1i32);
            if cursor_prev != (*m).cursor {
                Menu_CursorMoved(m);
                sound = menu_move_sound
            }
        }
        9 | 167 | 133 => {
            cursor_prev = (*m).cursor;
            (*m).cursor_prev = (*m).cursor;
            (*m).cursor += 1;
            Menu_AdjustCursor(m, 1i32);
            if cursor_prev != (*m).cursor {
                Menu_CursorMoved(m);
                sound = menu_move_sound
            }
        }
        178 | 180 => {
            if !item.is_null() {
                if 0 != (*item).flags & 0x200i32 as libc::c_uint
                    && 0 == (*item).flags & (0x2000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
                {
                    return Menu_ActivateItem(m, item);
                }
            }
        }
        185 | 186 | 187 | 188 | 217 | 218 | 219 | 220 | 221 | 222 | 223 | 224 | 225 | 226 | 227
        | 228 | 229 | 230 | 231 | 232 | 169 | 13 => {
            if !item.is_null() {
                if 0 == (*item).flags
                    & (0x800i32 as libc::c_uint
                        | 0x2000i32 as libc::c_uint
                        | 0x4000i32 as libc::c_uint)
                {
                    return Menu_ActivateItem(m, item);
                }
            }
        }
        _ => {}
    }
    return sound;
}
#[no_mangle]
pub unsafe extern "C" fn ScrollList_Key(
    mut l: *mut menulist_s,
    mut key: libc::c_int,
) -> sfxHandle_t {
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cursorx: libc::c_int = 0;
    let mut cursory: libc::c_int = 0;
    let mut column: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    match key {
        178 => {
            if 0 != (*l).generic.flags & 0x200i32 as libc::c_uint {
                x = (*l).generic.x;
                y = (*l).generic.y;
                w = (((*l).width + (*l).separation) * (*l).columns - (*l).separation) * 8i32;
                if 0 != (*l).generic.flags & 0x8i32 as libc::c_uint {
                    x -= w / 2i32
                }
                if 0 != UI_CursorInRect(x, y, w, (*l).height * 16i32) as u64 {
                    cursorx = (uis.cursorx - x) / 8i32;
                    column = cursorx / ((*l).width + (*l).separation);
                    cursory = (uis.cursory - y) / 16i32;
                    index = column * (*l).height + cursory;
                    if (*l).top + index < (*l).numitems {
                        (*l).oldvalue = (*l).curvalue;
                        (*l).curvalue = (*l).top + index;
                        if (*l).oldvalue != (*l).curvalue && (*l).generic.callback.is_some() {
                            (*l).generic.callback.expect("non-null function pointer")(
                                l as *mut libc::c_void,
                                1i32,
                            );
                            return menu_move_sound;
                        }
                    }
                }
                return menu_null_sound;
            }
        }
        160 | 143 => {
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue = 0i32;
            (*l).top = 0i32;
            if (*l).oldvalue != (*l).curvalue && (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1i32,
                );
                return menu_move_sound;
            }
            return menu_buzz_sound;
        }
        166 | 144 => {
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue = (*l).numitems - 1i32;
            if (*l).columns > 1i32 {
                c = ((*l).curvalue / (*l).height + 1i32) * (*l).height;
                (*l).top = c - (*l).columns * (*l).height
            } else {
                (*l).top = (*l).curvalue - ((*l).height - 1i32)
            }
            if (*l).top < 0i32 {
                (*l).top = 0i32
            }
            if (*l).oldvalue != (*l).curvalue && (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1i32,
                );
                return menu_move_sound;
            }
            return menu_buzz_sound;
        }
        142 | 162 => {
            if (*l).columns > 1i32 {
                return menu_null_sound;
            }
            if (*l).curvalue > 0i32 {
                (*l).oldvalue = (*l).curvalue;
                (*l).curvalue -= (*l).height - 1i32;
                if (*l).curvalue < 0i32 {
                    (*l).curvalue = 0i32
                }
                (*l).top = (*l).curvalue;
                if (*l).top < 0i32 {
                    (*l).top = 0i32
                }
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        1i32,
                    );
                }
                return menu_move_sound;
            }
            return menu_buzz_sound;
        }
        141 | 168 => {
            if (*l).columns > 1i32 {
                return menu_null_sound;
            }
            if (*l).curvalue < (*l).numitems - 1i32 {
                (*l).oldvalue = (*l).curvalue;
                (*l).curvalue += (*l).height - 1i32;
                if (*l).curvalue > (*l).numitems - 1i32 {
                    (*l).curvalue = (*l).numitems - 1i32
                }
                (*l).top = (*l).curvalue - ((*l).height - 1i32);
                if (*l).top < 0i32 {
                    (*l).top = 0i32
                }
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        1i32,
                    );
                }
                return menu_move_sound;
            }
            return menu_buzz_sound;
        }
        184 => {
            if (*l).columns > 1i32 {
                return menu_null_sound;
            }
            if (*l).top > 0i32 {
                let mut scroll: libc::c_int = if (*l).height < 6i32 { 1i32 } else { 3i32 };
                (*l).top -= scroll;
                if (*l).top < 0i32 {
                    (*l).top = 0i32
                }
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        1i32,
                    );
                }
                return menu_null_sound;
            }
            return menu_buzz_sound;
        }
        183 => {
            if (*l).columns > 1i32 {
                return menu_null_sound;
            }
            if (*l).top < (*l).numitems - (*l).height {
                let mut scroll_0: libc::c_int = if (*l).height < 6i32 { 1i32 } else { 3i32 };
                (*l).top += scroll_0;
                if (*l).top > (*l).numitems - (*l).height {
                    (*l).top = (*l).numitems - (*l).height
                }
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        1i32,
                    );
                }
                return menu_null_sound;
            }
            return menu_buzz_sound;
        }
        161 | 132 => {
            if (*l).curvalue == 0i32 {
                return menu_buzz_sound;
            }
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue -= 1;
            if (*l).curvalue < (*l).top {
                if (*l).columns == 1i32 {
                    (*l).top -= 1
                } else {
                    (*l).top -= (*l).height
                }
            }
            if (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1i32,
                );
            }
            return menu_move_sound;
        }
        167 | 133 => {
            if (*l).curvalue == (*l).numitems - 1i32 {
                return menu_buzz_sound;
            }
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue += 1;
            if (*l).curvalue >= (*l).top + (*l).columns * (*l).height {
                if (*l).columns == 1i32 {
                    (*l).top += 1
                } else {
                    (*l).top += (*l).height
                }
            }
            if (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1i32,
                );
            }
            return menu_move_sound;
        }
        163 | 134 => {
            if (*l).columns == 1i32 {
                return menu_null_sound;
            }
            if (*l).curvalue < (*l).height {
                return menu_buzz_sound;
            }
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue -= (*l).height;
            if (*l).curvalue < (*l).top {
                (*l).top -= (*l).height
            }
            if (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1i32,
                );
            }
            return menu_move_sound;
        }
        165 | 135 => {
            if (*l).columns == 1i32 {
                return menu_null_sound;
            }
            c = (*l).curvalue + (*l).height;
            if c >= (*l).numitems {
                return menu_buzz_sound;
            }
            (*l).oldvalue = (*l).curvalue;
            (*l).curvalue = c;
            if (*l).curvalue > (*l).top + (*l).columns * (*l).height - 1i32 {
                (*l).top += (*l).height
            }
            if (*l).generic.callback.is_some() {
                (*l).generic.callback.expect("non-null function pointer")(
                    l as *mut libc::c_void,
                    1i32,
                );
            }
            return menu_move_sound;
        }
        _ => {}
    }
    if 0 == Q_isprint(key) {
        return 0i32;
    }
    if 0 != Q_isupper(key) {
        key -= 'A' as i32 - 'a' as i32
    }
    i = 1i32;
    while i <= (*l).numitems {
        j = ((*l).curvalue + i) % (*l).numitems;
        c = *(*(*l).itemnames.offset(j as isize)).offset(0isize) as libc::c_int;
        if 0 != Q_isupper(c) {
            c -= 'A' as i32 - 'a' as i32
        }
        if c == key {
            if j < (*l).top {
                (*l).top = j
            } else if j > (*l).top + (*l).height - 1i32 {
                (*l).top = j + 1i32 - (*l).height
            }
            if (*l).curvalue != j {
                (*l).oldvalue = (*l).curvalue;
                (*l).curvalue = j;
                if (*l).generic.callback.is_some() {
                    (*l).generic.callback.expect("non-null function pointer")(
                        l as *mut libc::c_void,
                        1i32,
                    );
                }
                return menu_move_sound;
            }
            return menu_buzz_sound;
        }
        i += 1
    }
    return menu_buzz_sound;
}
unsafe extern "C" fn Slider_Key(mut s: *mut menuslider_s, mut key: libc::c_int) -> sfxHandle_t {
    let mut sound: sfxHandle_t = 0;
    let mut x: libc::c_int = 0;
    let mut oldvalue: libc::c_int = 0;
    match key {
        178 => {
            x = uis.cursorx - (*s).generic.x - 2i32 * 8i32;
            oldvalue = (*s).curvalue as libc::c_int;
            (*s).curvalue = x as libc::c_float / (10i32 * 8i32) as libc::c_float
                * ((*s).maxvalue - (*s).minvalue)
                + (*s).minvalue;
            if (*s).curvalue < (*s).minvalue {
                (*s).curvalue = (*s).minvalue
            } else if (*s).curvalue > (*s).maxvalue {
                (*s).curvalue = (*s).maxvalue
            }
            if (*s).curvalue != oldvalue as libc::c_float {
                sound = menu_move_sound
            } else {
                sound = 0i32
            }
        }
        163 | 134 => {
            if (*s).curvalue > (*s).minvalue {
                (*s).curvalue -= 1.;
                sound = menu_move_sound
            } else {
                sound = menu_buzz_sound
            }
        }
        165 | 135 => {
            if (*s).curvalue < (*s).maxvalue {
                (*s).curvalue += 1.;
                sound = menu_move_sound
            } else {
                sound = menu_buzz_sound
            }
        }
        _ => sound = 0i32,
    }
    if 0 != sound && (*s).generic.callback.is_some() {
        (*s).generic.callback.expect("non-null function pointer")(s as *mut libc::c_void, 3i32);
    }
    return sound;
}
unsafe extern "C" fn RadioButton_Key(
    mut rb: *mut menuradiobutton_s,
    mut key: libc::c_int,
) -> sfxHandle_t {
    let mut current_block_3: u64;
    match key {
        178 => {
            if 0 == (*rb).generic.flags & 0x200i32 as libc::c_uint {
                current_block_3 = 3640593987805443782;
            } else {
                current_block_3 = 9343145072321788094;
            }
        }
        185 | 186 | 187 | 188 | 13 | 169 | 163 | 134 | 165 | 135 => {
            current_block_3 = 9343145072321788094;
        }
        _ => {
            current_block_3 = 3640593987805443782;
        }
    }
    match current_block_3 {
        3640593987805443782 => {}
        _ => {
            (*rb).curvalue = (0 == (*rb).curvalue) as libc::c_int;
            if (*rb).generic.callback.is_some() {
                (*rb).generic.callback.expect("non-null function pointer")(
                    rb as *mut libc::c_void,
                    3i32,
                );
            }
            return menu_move_sound;
        }
    }
    return 0i32;
}
unsafe extern "C" fn SpinControl_Key(mut s: *mut menulist_s, mut key: libc::c_int) -> sfxHandle_t {
    let mut sound: sfxHandle_t = 0;
    sound = 0i32;
    match key {
        165 | 135 | 178 => {
            (*s).curvalue += 1;
            if (*s).curvalue >= (*s).numitems {
                (*s).curvalue = 0i32
            }
            sound = menu_move_sound
        }
        163 | 134 => {
            (*s).curvalue -= 1;
            if (*s).curvalue < 0i32 {
                (*s).curvalue = (*s).numitems - 1i32
            }
            sound = menu_move_sound
        }
        _ => {}
    }
    if 0 != sound && (*s).generic.callback.is_some() {
        (*s).generic.callback.expect("non-null function pointer")(s as *mut libc::c_void, 3i32);
    }
    return sound;
}
#[no_mangle]
pub static mut menu_dim_color: vec4_t = [0.0f32, 0.0f32, 0.0f32, 0.75f32];
#[no_mangle]
pub static mut color_black: vec4_t = [0.00f32, 0.00f32, 0.00f32, 1.00f32];
#[no_mangle]
pub static mut color_white: vec4_t = [1.00f32, 1.00f32, 1.00f32, 1.00f32];
#[no_mangle]
pub static mut color_yellow: vec4_t = [1.00f32, 1.00f32, 0.00f32, 1.00f32];
#[no_mangle]
pub static mut color_blue: vec4_t = [0.00f32, 0.00f32, 1.00f32, 1.00f32];
#[no_mangle]
pub static mut color_orange: vec4_t = [1.00f32, 0.43f32, 0.00f32, 1.00f32];
#[no_mangle]
pub static mut color_red: vec4_t = [1.00f32, 0.00f32, 0.00f32, 1.00f32];
#[no_mangle]
pub static mut color_dim: vec4_t = [0.00f32, 0.00f32, 0.00f32, 0.25f32];
#[no_mangle]
pub static mut color_lightOrange: vec4_t = [1.00f32, 0.68f32, 0.00f32, 1.00f32];
// light gray
// light orange
// bright yellow
// transluscent orange
// bright white
#[no_mangle]
pub static mut text_color_status: vec4_t = [1.00f32, 1.00f32, 1.00f32, 1.00f32];
