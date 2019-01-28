use libc;
use q_shared_h::{qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, vec4_t, vec_t};
use stdlib::{memset, strlen};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menufield_s, menuframework_s, menutext_s,
    mfield_t, trap_Cvar_Set, trap_GetCDKey, trap_Key_GetOverstrikeMode, trap_R_RegisterShaderNoMip,
    trap_SetCDKey, trap_VerifyCDKey, uiStatic_t,
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

//
// ui_cdkey.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_CDKeyMenu() {
    UI_CDKeyMenu_Init();
    UI_PushMenu(&mut cdkeyMenuInfo.menu);
}
static mut cdkeyMenuInfo: cdkeyMenuInfo_t = cdkeyMenuInfo_t {
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
    banner: menutext_s {
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
    frame: menubitmap_s {
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
    cdkey: menufield_s {
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
        field: mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    accept: menubitmap_s {
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
    back: menubitmap_s {
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
};
/*
===============
UI_CDKeyMenu_Init
===============
*/
unsafe extern "C" fn UI_CDKeyMenu_Init() {
    trap_Cvar_Set(
        b"ui_cdkeychecked\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
    UI_CDKeyMenu_Cache();
    memset(
        &mut cdkeyMenuInfo as *mut cdkeyMenuInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<cdkeyMenuInfo_t>() as libc::c_ulong,
    );
    cdkeyMenuInfo.menu.wrapAround = qtrue;
    cdkeyMenuInfo.menu.fullscreen = qtrue;
    cdkeyMenuInfo.banner.generic.type_0 = 10i32;
    cdkeyMenuInfo.banner.generic.x = 320i32;
    cdkeyMenuInfo.banner.generic.y = 16i32;
    cdkeyMenuInfo.banner.string =
        b"CD KEY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cdkeyMenuInfo.banner.color = color_white.as_mut_ptr();
    cdkeyMenuInfo.banner.style = 0x1i32;
    cdkeyMenuInfo.frame.generic.type_0 = 6i32;
    cdkeyMenuInfo.frame.generic.name =
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char;
    cdkeyMenuInfo.frame.generic.flags = 0x4000i32 as libc::c_uint;
    cdkeyMenuInfo.frame.generic.x = 142i32;
    cdkeyMenuInfo.frame.generic.y = 118i32;
    cdkeyMenuInfo.frame.width = 359i32;
    cdkeyMenuInfo.frame.height = 256i32;
    cdkeyMenuInfo.cdkey.generic.type_0 = 4i32;
    cdkeyMenuInfo.cdkey.generic.name = b"CD Key:\x00" as *const u8 as *const libc::c_char;
    cdkeyMenuInfo.cdkey.generic.flags = 0x40000i32 as libc::c_uint;
    cdkeyMenuInfo.cdkey.generic.x =
        (320i32 as libc::c_double - 16i32 as libc::c_double * 2.5f64) as libc::c_int;
    cdkeyMenuInfo.cdkey.generic.y = 240i32 - 16i32 / 2i32;
    cdkeyMenuInfo.cdkey.field.widthInChars = 16i32;
    cdkeyMenuInfo.cdkey.field.maxchars = 16i32;
    cdkeyMenuInfo.cdkey.generic.ownerdraw = Some(UI_CDKeyMenu_DrawKey);
    cdkeyMenuInfo.accept.generic.type_0 = 6i32;
    cdkeyMenuInfo.accept.generic.name =
        b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char;
    cdkeyMenuInfo.accept.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cdkeyMenuInfo.accept.generic.id = 11i32;
    cdkeyMenuInfo.accept.generic.callback = Some(UI_CDKeyMenu_Event);
    cdkeyMenuInfo.accept.generic.x = 640i32;
    cdkeyMenuInfo.accept.generic.y = 480i32 - 64i32;
    cdkeyMenuInfo.accept.width = 128i32;
    cdkeyMenuInfo.accept.height = 64i32;
    cdkeyMenuInfo.accept.focuspic =
        b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cdkeyMenuInfo.back.generic.type_0 = 6i32;
    cdkeyMenuInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    cdkeyMenuInfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cdkeyMenuInfo.back.generic.id = 12i32;
    cdkeyMenuInfo.back.generic.callback = Some(UI_CDKeyMenu_Event);
    cdkeyMenuInfo.back.generic.x = 0i32;
    cdkeyMenuInfo.back.generic.y = 480i32 - 64i32;
    cdkeyMenuInfo.back.width = 128i32;
    cdkeyMenuInfo.back.height = 64i32;
    cdkeyMenuInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut cdkeyMenuInfo.menu,
        &mut cdkeyMenuInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cdkeyMenuInfo.menu,
        &mut cdkeyMenuInfo.frame as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cdkeyMenuInfo.menu,
        &mut cdkeyMenuInfo.cdkey as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cdkeyMenuInfo.menu,
        &mut cdkeyMenuInfo.accept as *mut menubitmap_s as *mut libc::c_void,
    );
    if 0 != uis.menusp {
        Menu_AddItem(
            &mut cdkeyMenuInfo.menu,
            &mut cdkeyMenuInfo.back as *mut menubitmap_s as *mut libc::c_void,
        );
    }
    trap_GetCDKey(
        cdkeyMenuInfo.cdkey.field.buffer.as_mut_ptr(),
        cdkeyMenuInfo.cdkey.field.maxchars + 1i32,
    );
    if trap_VerifyCDKey(
        cdkeyMenuInfo.cdkey.field.buffer.as_mut_ptr(),
        0 as *const libc::c_char,
    ) as libc::c_uint
        == qfalse as libc::c_int as libc::c_uint
    {
        cdkeyMenuInfo.cdkey.field.buffer[0usize] = 0i32 as libc::c_char
    };
}
/*
===============
UI_CDKeyMenu_Event
===============
*/
unsafe extern "C" fn UI_CDKeyMenu_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        11 => {
            if 0 != cdkeyMenuInfo.cdkey.field.buffer[0usize] {
                trap_SetCDKey(cdkeyMenuInfo.cdkey.field.buffer.as_mut_ptr());
            }
            UI_PopMenu();
        }
        12 => {
            UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
UI_CDKeyMenu_DrawKey
=================
*/
unsafe extern "C" fn UI_CDKeyMenu_DrawKey(mut self_0: *mut libc::c_void) {
    let mut f: *mut menufield_s = 0 as *mut menufield_s;
    let mut focus: qboolean = qfalse;
    let mut style: libc::c_int = 0;
    let mut c: libc::c_char = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    f = self_0 as *mut menufield_s;
    focus = ((*(*f).generic.parent).cursor == (*f).generic.menuPosition) as libc::c_int as qboolean;
    style = 0i32;
    if 0 != focus as u64 {
        color = color_yellow.as_mut_ptr()
    } else {
        color = color_orange.as_mut_ptr()
    }
    x = 320i32 - 8i32 * 16i32;
    y = 240i32 - 16i32 / 2i32;
    UI_FillRect(
        x as libc::c_float,
        y as libc::c_float,
        (16i32 * 16i32) as libc::c_float,
        16i32 as libc::c_float,
        listbar_color.as_mut_ptr(),
    );
    UI_DrawString(x, y, (*f).field.buffer.as_mut_ptr(), style, color);
    if 0 != focus as u64 {
        if 0 != trap_Key_GetOverstrikeMode() as u64 {
            c = 11i32 as libc::c_char
        } else {
            c = 10i32 as libc::c_char
        }
        style &= !0x4000i32;
        style |= 0x1000i32;
        UI_DrawChar(
            x + (*f).field.cursor * 16i32,
            y,
            c as libc::c_int,
            style,
            color_white.as_mut_ptr(),
        );
    }
    val = UI_CDKeyMenu_PreValidateKey((*f).field.buffer.as_mut_ptr());
    if val == 1i32 {
        UI_DrawProportionalString(
            320i32,
            376i32,
            b"Please enter your CD Key\x00" as *const u8 as *const libc::c_char,
            0x1i32 | 0x10i32,
            color_yellow.as_mut_ptr(),
        );
    } else if val == 0i32 {
        UI_DrawProportionalString(
            320i32,
            376i32,
            b"The CD Key appears to be valid, thank you\x00" as *const u8 as *const libc::c_char,
            0x1i32 | 0x10i32,
            color_white.as_mut_ptr(),
        );
    } else {
        UI_DrawProportionalString(
            320i32,
            376i32,
            b"The CD Key is not valid\x00" as *const u8 as *const libc::c_char,
            0x1i32 | 0x10i32,
            color_red.as_mut_ptr(),
        );
    };
}
/*
=================
UI_CDKeyMenu_PreValidateKey
=================
*/
unsafe extern "C" fn UI_CDKeyMenu_PreValidateKey(mut key: *const libc::c_char) -> libc::c_int {
    let mut ch: libc::c_char = 0;
    if strlen(key) != 16i32 as libc::c_ulong {
        return 1i32;
    }
    loop {
        let fresh0 = key;
        key = key.offset(1);
        ch = *fresh0;
        if !(0 != ch) {
            break;
        }
        match ch as libc::c_int {
            50 | 51 | 55 | 97 | 98 | 99 | 100 | 103 | 104 | 106 | 108 | 112 | 114 | 115 | 116
            | 119 => {
                continue;
            }
            _ => {}
        }
        return -1i32;
    }
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn UI_CDKeyMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_CDKeyMenu_f() {
    UI_CDKeyMenu();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdkeyMenuInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub frame: menubitmap_s,
    pub cdkey: menufield_s,
    pub accept: menubitmap_s,
    pub back: menubitmap_s,
}
