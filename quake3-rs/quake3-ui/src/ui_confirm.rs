#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
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
    connstate_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, vec4_t, vec_t, CA_ACTIVE,
    CA_AUTHORIZING, CA_CHALLENGING, CA_CINEMATIC, CA_CONNECTED, CA_CONNECTING, CA_DISCONNECTED,
    CA_LOADING, CA_PRIMED, CA_UNINITIALIZED,
};
use stdlib::memset;
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
    _tag_menuframework, menucommon_s, menuframework_s, menutext_s, trap_GetClientState,
    trap_R_RegisterShaderNoMip,
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
use ui_public_h::uiClientState_t;
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
// ui_confirm.c
//
#[no_mangle]
pub unsafe extern "C" fn ConfirmMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_ConfirmMenu(
    mut question: *const libc::c_char,
    mut draw: Option<unsafe extern "C" fn() -> ()>,
    mut action: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
) {
    UI_ConfirmMenu_Style(question, 0x1i32 | 0x2000i32, draw, action);
}
#[no_mangle]
pub unsafe extern "C" fn UI_ConfirmMenu_Style(
    mut question: *const libc::c_char,
    mut style: libc::c_int,
    mut draw: Option<unsafe extern "C" fn() -> ()>,
    mut action: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
) {
    let mut cstate: uiClientState_t = uiClientState_t {
        connState: CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut n3: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    let mut l3: libc::c_int = 0;
    memset(
        &mut s_confirm as *mut confirmMenu_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<confirmMenu_t>() as libc::c_ulong,
    );
    ConfirmMenu_Cache();
    n1 = UI_ProportionalStringWidth(b"YES/NO\x00" as *const u8 as *const libc::c_char);
    n2 = UI_ProportionalStringWidth(b"YES\x00" as *const u8 as *const libc::c_char) + 3i32;
    n3 = UI_ProportionalStringWidth(b"/\x00" as *const u8 as *const libc::c_char) + 3i32;
    l1 = 320i32 - n1 / 2i32;
    l2 = l1 + n2;
    l3 = l2 + n3;
    s_confirm.slashX = l2;
    s_confirm.question = question;
    s_confirm.draw = draw;
    s_confirm.action = action;
    s_confirm.style = style;
    s_confirm.menu.draw = Some(ConfirmMenu_Draw);
    s_confirm.menu.key = Some(ConfirmMenu_Key);
    s_confirm.menu.wrapAround = qtrue;
    trap_GetClientState(&mut cstate);
    if cstate.connState as libc::c_uint >= CA_CONNECTED as libc::c_int as libc::c_uint {
        s_confirm.menu.fullscreen = qfalse
    } else {
        s_confirm.menu.fullscreen = qtrue
    }
    s_confirm.yes.generic.type_0 = 9i32;
    s_confirm.yes.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_confirm.yes.generic.callback = Some(ConfirmMenu_Event);
    s_confirm.yes.generic.id = 11i32;
    s_confirm.yes.generic.x = l1;
    s_confirm.yes.generic.y = 264i32;
    s_confirm.yes.string = b"YES\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_confirm.yes.color = color_red.as_mut_ptr();
    s_confirm.yes.style = 0i32;
    s_confirm.no.generic.type_0 = 9i32;
    s_confirm.no.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_confirm.no.generic.callback = Some(ConfirmMenu_Event);
    s_confirm.no.generic.id = 10i32;
    s_confirm.no.generic.x = l3;
    s_confirm.no.generic.y = 264i32;
    s_confirm.no.string = b"NO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_confirm.no.color = color_red.as_mut_ptr();
    s_confirm.no.style = 0i32;
    Menu_AddItem(
        &mut s_confirm.menu,
        &mut s_confirm.yes as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_confirm.menu,
        &mut s_confirm.no as *mut menutext_s as *mut libc::c_void,
    );
    UI_PushMenu(&mut s_confirm.menu);
    Menu_SetCursorToItem(
        &mut s_confirm.menu,
        &mut s_confirm.no as *mut menutext_s as *mut libc::c_void,
    );
}
static mut s_confirm: confirmMenu_t = confirmMenu_t {
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
    no: menutext_s {
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
    yes: menutext_s {
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
    slashX: 0,
    question: 0 as *const libc::c_char,
    draw: None,
    action: None,
    style: 0,
    lines: 0 as *const *const libc::c_char as *mut *const libc::c_char,
};
/*
=================
ConfirmMenu_Event
=================
*/
unsafe extern "C" fn ConfirmMenu_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    let mut result: qboolean = qfalse;
    if event != 3i32 {
        return;
    }
    UI_PopMenu();
    if (*(ptr as *mut menucommon_s)).id == 10i32 {
        result = qfalse
    } else {
        result = qtrue
    }
    if s_confirm.action.is_some() {
        s_confirm.action.expect("non-null function pointer")(result);
    };
}
/*
=================
ConfirmMenu_Key
=================
*/
unsafe extern "C" fn ConfirmMenu_Key(mut key: libc::c_int) -> sfxHandle_t {
    match key {
        163 | 134 | 165 | 135 => key = K_TAB as libc::c_int,
        110 | 78 => {
            ConfirmMenu_Event(
                &mut s_confirm.no as *mut menutext_s as *mut libc::c_void,
                3i32,
            );
        }
        121 | 89 => {
            ConfirmMenu_Event(
                &mut s_confirm.yes as *mut menutext_s as *mut libc::c_void,
                3i32,
            );
        }
        _ => {}
    }
    return Menu_DefaultKey(&mut s_confirm.menu, key);
}
/*
=================
ConfirmMenu_Draw
=================
*/
unsafe extern "C" fn ConfirmMenu_Draw() {
    UI_DrawNamedPic(
        142i32 as libc::c_float,
        118i32 as libc::c_float,
        359i32 as libc::c_float,
        256i32 as libc::c_float,
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char,
    );
    UI_DrawProportionalString(
        320i32,
        204i32,
        s_confirm.question,
        s_confirm.style,
        color_red.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        s_confirm.slashX,
        265i32,
        b"/\x00" as *const u8 as *const libc::c_char,
        0i32 | 0x2000i32,
        color_red.as_mut_ptr(),
    );
    Menu_Draw(&mut s_confirm.menu);
    if s_confirm.draw.is_some() {
        s_confirm.draw.expect("non-null function pointer")();
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_Message(mut lines: *mut *const libc::c_char) {
    let mut cstate: uiClientState_t = uiClientState_t {
        connState: CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    let mut n1: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    memset(
        &mut s_confirm as *mut confirmMenu_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<confirmMenu_t>() as libc::c_ulong,
    );
    ConfirmMenu_Cache();
    n1 = UI_ProportionalStringWidth(b"OK\x00" as *const u8 as *const libc::c_char);
    l1 = 320i32 - n1 / 2i32;
    s_confirm.lines = lines;
    s_confirm.style = 0x1i32 | 0x2000i32 | 0x10i32;
    s_confirm.menu.draw = Some(MessageMenu_Draw);
    s_confirm.menu.key = Some(ConfirmMenu_Key);
    s_confirm.menu.wrapAround = qtrue;
    trap_GetClientState(&mut cstate);
    if cstate.connState as libc::c_uint >= CA_CONNECTED as libc::c_int as libc::c_uint {
        s_confirm.menu.fullscreen = qfalse
    } else {
        s_confirm.menu.fullscreen = qtrue
    }
    s_confirm.yes.generic.type_0 = 9i32;
    s_confirm.yes.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_confirm.yes.generic.callback = Some(ConfirmMenu_Event);
    s_confirm.yes.generic.id = 11i32;
    s_confirm.yes.generic.x = l1;
    s_confirm.yes.generic.y = 280i32;
    s_confirm.yes.string = b"OK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_confirm.yes.color = color_red.as_mut_ptr();
    s_confirm.yes.style = 0i32;
    Menu_AddItem(
        &mut s_confirm.menu,
        &mut s_confirm.yes as *mut menutext_s as *mut libc::c_void,
    );
    UI_PushMenu(&mut s_confirm.menu);
    Menu_SetCursorToItem(
        &mut s_confirm.menu,
        &mut s_confirm.yes as *mut menutext_s as *mut libc::c_void,
    );
}
/*
=================
MessaheMenu_Draw
=================
*/
unsafe extern "C" fn MessageMenu_Draw() {
    let mut i: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    UI_DrawNamedPic(
        142i32 as libc::c_float,
        118i32 as libc::c_float,
        359i32 as libc::c_float,
        256i32 as libc::c_float,
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char,
    );
    y = 188i32;
    i = 0i32;
    while !(*s_confirm.lines.offset(i as isize)).is_null() {
        UI_DrawProportionalString(
            320i32,
            y,
            *s_confirm.lines.offset(i as isize),
            s_confirm.style,
            color_red.as_mut_ptr(),
        );
        y += 18i32;
        i += 1
    }
    Menu_Draw(&mut s_confirm.menu);
    if s_confirm.draw.is_some() {
        s_confirm.draw.expect("non-null function pointer")();
    };
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct confirmMenu_t {
    pub menu: menuframework_s,
    pub no: menutext_s,
    pub yes: menutext_s,
    pub slashX: libc::c_int,
    pub question: *const libc::c_char,
    pub draw: Option<unsafe extern "C" fn() -> ()>,
    pub action: Option<unsafe extern "C" fn(_: qboolean) -> ()>,
    pub style: libc::c_int,
    pub lines: *mut *const libc::c_char,
}
