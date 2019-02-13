use bg_misc::bg_itemlist;
use libc;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
    AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
    AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t,
    COM_StripExtension, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
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
    mfield_t, trap_Cmd_ExecuteText, trap_R_RegisterShaderNoMip,
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
// ui_saveconfig.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_SaveConfigMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/save_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/save_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_SaveConfigMenu() {
    UI_SaveConfigMenu_Init();
    UI_PushMenu(&mut saveConfig.menu);
}
static mut saveConfig: saveConfig_t = saveConfig_t {
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
    background: menubitmap_s {
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
    savename: menufield_s {
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
    save: menubitmap_s {
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
=================
UI_SaveConfigMenu_Init
=================
*/
unsafe extern "C" fn UI_SaveConfigMenu_Init() {
    memset(
        &mut saveConfig as *mut saveConfig_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<saveConfig_t>() as libc::c_ulong,
    );
    UI_SaveConfigMenu_Cache();
    saveConfig.menu.wrapAround = qtrue;
    saveConfig.menu.fullscreen = qtrue;
    saveConfig.banner.generic.type_0 = 10i32;
    saveConfig.banner.generic.x = 320i32;
    saveConfig.banner.generic.y = 16i32;
    saveConfig.banner.string =
        b"SAVE CONFIG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    saveConfig.banner.color = color_white.as_mut_ptr();
    saveConfig.banner.style = 0x1i32;
    saveConfig.background.generic.type_0 = 6i32;
    saveConfig.background.generic.name =
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char;
    saveConfig.background.generic.flags = 0x4000i32 as libc::c_uint;
    saveConfig.background.generic.x = 142i32;
    saveConfig.background.generic.y = 118i32;
    saveConfig.background.width = 359i32;
    saveConfig.background.height = 256i32;
    saveConfig.savename.generic.type_0 = 4i32;
    saveConfig.savename.generic.flags = 0x8000i32 as libc::c_uint | 0x80000i32 as libc::c_uint;
    saveConfig.savename.generic.ownerdraw = Some(UI_SaveConfigMenu_SavenameDraw);
    saveConfig.savename.field.widthInChars = 20i32;
    saveConfig.savename.field.maxchars = 20i32;
    saveConfig.savename.generic.x = 240i32;
    saveConfig.savename.generic.y = 155i32 + 72i32;
    saveConfig.savename.generic.left = 240i32;
    saveConfig.savename.generic.top = 155i32 + 72i32;
    saveConfig.savename.generic.right = 233i32 + 20i32 * 8i32;
    saveConfig.savename.generic.bottom = 155i32 + 72i32 + 16i32 + 2i32;
    saveConfig.back.generic.type_0 = 6i32;
    saveConfig.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    saveConfig.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    saveConfig.back.generic.id = 11i32;
    saveConfig.back.generic.callback = Some(UI_SaveConfigMenu_BackEvent);
    saveConfig.back.generic.x = 0i32;
    saveConfig.back.generic.y = 480i32 - 64i32;
    saveConfig.back.width = 128i32;
    saveConfig.back.height = 64i32;
    saveConfig.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    saveConfig.save.generic.type_0 = 6i32;
    saveConfig.save.generic.name = b"menu/art/save_0\x00" as *const u8 as *const libc::c_char;
    saveConfig.save.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    saveConfig.save.generic.id = 12i32;
    saveConfig.save.generic.callback = Some(UI_SaveConfigMenu_SaveEvent);
    saveConfig.save.generic.x = 640i32;
    saveConfig.save.generic.y = 480i32 - 64i32;
    saveConfig.save.width = 128i32;
    saveConfig.save.height = 64i32;
    saveConfig.save.focuspic =
        b"menu/art/save_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut saveConfig.menu,
        &mut saveConfig.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut saveConfig.menu,
        &mut saveConfig.background as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut saveConfig.menu,
        &mut saveConfig.savename as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut saveConfig.menu,
        &mut saveConfig.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut saveConfig.menu,
        &mut saveConfig.save as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
===============
UI_SaveConfigMenu_SaveEvent
===============
*/
unsafe extern "C" fn UI_SaveConfigMenu_SaveEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    let mut configname: [libc::c_char; 64] = [0; 64];
    if event != 3i32 {
        return;
    }
    if 0 == saveConfig.savename.field.buffer[0usize] {
        return;
    }
    COM_StripExtension(
        saveConfig.savename.field.buffer.as_mut_ptr(),
        configname.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        va(
            b"writeconfig %s.cfg\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            configname.as_mut_ptr(),
        ),
    );
    UI_PopMenu();
}
/*
===============
UI_SaveConfigMenu_BackEvent
===============
*/
unsafe extern "C" fn UI_SaveConfigMenu_BackEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    UI_PopMenu();
}
/*
===============
UI_SaveConfigMenu_SavenameDraw
===============
*/
unsafe extern "C" fn UI_SaveConfigMenu_SavenameDraw(mut self_0: *mut libc::c_void) {
    let mut f: *mut menufield_s = 0 as *mut menufield_s;
    let mut style: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    f = self_0 as *mut menufield_s;
    if f == Menu_ItemAtCursor(&mut saveConfig.menu) as *mut menufield_s {
        style = 0i32 | 0x4000i32 | 0x10i32;
        color = text_color_highlight.as_mut_ptr()
    } else {
        style = 0i32 | 0x10i32;
        color = colorRed.as_mut_ptr()
    }
    UI_DrawProportionalString(
        320i32,
        192i32,
        b"Enter filename:\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_orange.as_mut_ptr(),
    );
    UI_FillRect(
        (*f).generic.x as libc::c_float,
        (*f).generic.y as libc::c_float,
        ((*f).field.widthInChars * 8i32) as libc::c_float,
        16i32 as libc::c_float,
        colorBlack.as_mut_ptr(),
    );
    MField_Draw(
        &mut (*f).field,
        (*f).generic.x,
        (*f).generic.y,
        style,
        color,
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saveConfig_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub background: menubitmap_s,
    pub savename: menufield_s,
    pub back: menubitmap_s,
    pub save: menubitmap_s,
}
