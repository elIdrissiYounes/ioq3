use bg_misc::bg_itemlist;
use libc;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
    AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
    AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, vec4_t, vec_t, EXEC_APPEND,
    EXEC_INSERT, EXEC_NOW,
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menulist_s, menuslider_s,
    menutext_s, trap_Cmd_ExecuteText, trap_Cvar_SetValue, trap_Cvar_VariableValue,
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
// ui_sound.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_SoundOptionsMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_SoundOptionsMenu() {
    UI_SoundOptionsMenu_Init();
    UI_PushMenu(&mut soundOptionsInfo.menu);
    Menu_SetCursorToItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.sound as *mut menutext_s as *mut libc::c_void,
    );
}
static mut soundOptionsInfo: soundOptionsInfo_t = soundOptionsInfo_t {
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
    framel: menubitmap_s {
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
    framer: menubitmap_s {
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
    graphics: menutext_s {
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
    display: menutext_s {
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
    sound: menutext_s {
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
    network: menutext_s {
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
    sfxvolume: menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    musicvolume: menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    soundSystem: menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    quality: menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
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
    apply: menubitmap_s {
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
    sfxvolume_original: 0.,
    musicvolume_original: 0.,
    soundSystem_original: 0,
    quality_original: 0,
};
/*
===============
UI_SoundOptionsMenu_Init
===============
*/
unsafe extern "C" fn UI_SoundOptionsMenu_Init() {
    let mut y: libc::c_int = 0;
    let mut speed: libc::c_int = 0;
    memset(
        &mut soundOptionsInfo as *mut soundOptionsInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<soundOptionsInfo_t>() as libc::c_ulong,
    );
    UI_SoundOptionsMenu_Cache();
    soundOptionsInfo.menu.wrapAround = qtrue;
    soundOptionsInfo.menu.fullscreen = qtrue;
    soundOptionsInfo.menu.draw = Some(SoundOptions_MenuDraw);
    soundOptionsInfo.banner.generic.type_0 = 10i32;
    soundOptionsInfo.banner.generic.flags = 0x8i32 as libc::c_uint;
    soundOptionsInfo.banner.generic.x = 320i32;
    soundOptionsInfo.banner.generic.y = 16i32;
    soundOptionsInfo.banner.string =
        b"SYSTEM SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.banner.color = color_white.as_mut_ptr();
    soundOptionsInfo.banner.style = 0x1i32;
    soundOptionsInfo.framel.generic.type_0 = 6i32;
    soundOptionsInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.framel.generic.flags = 0x4000i32 as libc::c_uint;
    soundOptionsInfo.framel.generic.x = 0i32;
    soundOptionsInfo.framel.generic.y = 78i32;
    soundOptionsInfo.framel.width = 256i32;
    soundOptionsInfo.framel.height = 329i32;
    soundOptionsInfo.framer.generic.type_0 = 6i32;
    soundOptionsInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.framer.generic.flags = 0x4000i32 as libc::c_uint;
    soundOptionsInfo.framer.generic.x = 376i32;
    soundOptionsInfo.framer.generic.y = 76i32;
    soundOptionsInfo.framer.width = 256i32;
    soundOptionsInfo.framer.height = 334i32;
    soundOptionsInfo.graphics.generic.type_0 = 9i32;
    soundOptionsInfo.graphics.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    soundOptionsInfo.graphics.generic.id = 10i32;
    soundOptionsInfo.graphics.generic.callback = Some(UI_SoundOptionsMenu_Event);
    soundOptionsInfo.graphics.generic.x = 216i32;
    soundOptionsInfo.graphics.generic.y = 240i32 - 2i32 * 27i32;
    soundOptionsInfo.graphics.string =
        b"GRAPHICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.graphics.style = 0x2i32;
    soundOptionsInfo.graphics.color = color_red.as_mut_ptr();
    soundOptionsInfo.display.generic.type_0 = 9i32;
    soundOptionsInfo.display.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    soundOptionsInfo.display.generic.id = 11i32;
    soundOptionsInfo.display.generic.callback = Some(UI_SoundOptionsMenu_Event);
    soundOptionsInfo.display.generic.x = 216i32;
    soundOptionsInfo.display.generic.y = 240i32 - 27i32;
    soundOptionsInfo.display.string =
        b"DISPLAY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.display.style = 0x2i32;
    soundOptionsInfo.display.color = color_red.as_mut_ptr();
    soundOptionsInfo.sound.generic.type_0 = 9i32;
    soundOptionsInfo.sound.generic.flags = 0x10i32 as libc::c_uint;
    soundOptionsInfo.sound.generic.id = 12i32;
    soundOptionsInfo.sound.generic.callback = Some(UI_SoundOptionsMenu_Event);
    soundOptionsInfo.sound.generic.x = 216i32;
    soundOptionsInfo.sound.generic.y = 240i32;
    soundOptionsInfo.sound.string =
        b"SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.sound.style = 0x2i32;
    soundOptionsInfo.sound.color = color_red.as_mut_ptr();
    soundOptionsInfo.network.generic.type_0 = 9i32;
    soundOptionsInfo.network.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    soundOptionsInfo.network.generic.id = 13i32;
    soundOptionsInfo.network.generic.callback = Some(UI_SoundOptionsMenu_Event);
    soundOptionsInfo.network.generic.x = 216i32;
    soundOptionsInfo.network.generic.y = 240i32 + 27i32;
    soundOptionsInfo.network.string =
        b"NETWORK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.network.style = 0x2i32;
    soundOptionsInfo.network.color = color_red.as_mut_ptr();
    y = 240i32 - 2i32 * (16i32 + 2i32);
    soundOptionsInfo.sfxvolume.generic.type_0 = 1i32;
    soundOptionsInfo.sfxvolume.generic.name =
        b"Effects Volume:\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.sfxvolume.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    soundOptionsInfo.sfxvolume.generic.callback = Some(UI_SoundOptionsMenu_Event);
    soundOptionsInfo.sfxvolume.generic.id = 14i32;
    soundOptionsInfo.sfxvolume.generic.x = 400i32;
    soundOptionsInfo.sfxvolume.generic.y = y;
    soundOptionsInfo.sfxvolume.minvalue = 0i32 as libc::c_float;
    soundOptionsInfo.sfxvolume.maxvalue = 10i32 as libc::c_float;
    y += 16i32 + 2i32;
    soundOptionsInfo.musicvolume.generic.type_0 = 1i32;
    soundOptionsInfo.musicvolume.generic.name =
        b"Music Volume:\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.musicvolume.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    soundOptionsInfo.musicvolume.generic.callback = Some(UI_SoundOptionsMenu_Event);
    soundOptionsInfo.musicvolume.generic.id = 15i32;
    soundOptionsInfo.musicvolume.generic.x = 400i32;
    soundOptionsInfo.musicvolume.generic.y = y;
    soundOptionsInfo.musicvolume.minvalue = 0i32 as libc::c_float;
    soundOptionsInfo.musicvolume.maxvalue = 10i32 as libc::c_float;
    y += 16i32 + 2i32;
    soundOptionsInfo.soundSystem.generic.type_0 = 3i32;
    soundOptionsInfo.soundSystem.generic.name =
        b"Sound System:\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.soundSystem.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    soundOptionsInfo.soundSystem.generic.callback = Some(UI_SoundOptionsMenu_Event);
    soundOptionsInfo.soundSystem.generic.id = 17i32;
    soundOptionsInfo.soundSystem.generic.x = 400i32;
    soundOptionsInfo.soundSystem.generic.y = y;
    soundOptionsInfo.soundSystem.itemnames = soundSystem_items.as_mut_ptr();
    y += 16i32 + 2i32;
    soundOptionsInfo.quality.generic.type_0 = 3i32;
    soundOptionsInfo.quality.generic.name =
        b"SDL Sound Quality:\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.quality.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    soundOptionsInfo.quality.generic.callback = Some(UI_SoundOptionsMenu_Event);
    soundOptionsInfo.quality.generic.id = 16i32;
    soundOptionsInfo.quality.generic.x = 400i32;
    soundOptionsInfo.quality.generic.y = y;
    soundOptionsInfo.quality.itemnames = quality_items.as_mut_ptr();
    soundOptionsInfo.back.generic.type_0 = 6i32;
    soundOptionsInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    soundOptionsInfo.back.generic.callback = Some(UI_SoundOptionsMenu_Event);
    soundOptionsInfo.back.generic.id = 19i32;
    soundOptionsInfo.back.generic.x = 0i32;
    soundOptionsInfo.back.generic.y = 480i32 - 64i32;
    soundOptionsInfo.back.width = 128i32;
    soundOptionsInfo.back.height = 64i32;
    soundOptionsInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.apply.generic.type_0 = 6i32;
    soundOptionsInfo.apply.generic.name =
        b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.apply.generic.flags = 0x10i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint
        | 0x4000i32 as libc::c_uint;
    soundOptionsInfo.apply.generic.callback = Some(UI_SoundOptionsMenu_Event);
    soundOptionsInfo.apply.generic.id = 20i32;
    soundOptionsInfo.apply.generic.x = 640i32;
    soundOptionsInfo.apply.generic.y = 480i32 - 64i32;
    soundOptionsInfo.apply.width = 128i32;
    soundOptionsInfo.apply.height = 64i32;
    soundOptionsInfo.apply.focuspic =
        b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.graphics as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.display as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.sound as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.network as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.sfxvolume as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.musicvolume as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.soundSystem as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.quality as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut soundOptionsInfo.menu,
        &mut soundOptionsInfo.apply as *mut menubitmap_s as *mut libc::c_void,
    );
    soundOptionsInfo.sfxvolume_original =
        trap_Cvar_VariableValue(b"s_volume\x00" as *const u8 as *const libc::c_char)
            * 10i32 as libc::c_float;
    soundOptionsInfo.sfxvolume.curvalue = soundOptionsInfo.sfxvolume_original;
    soundOptionsInfo.musicvolume_original =
        trap_Cvar_VariableValue(b"s_musicvolume\x00" as *const u8 as *const libc::c_char)
            * 10i32 as libc::c_float;
    soundOptionsInfo.musicvolume.curvalue = soundOptionsInfo.musicvolume_original;
    if 0. != trap_Cvar_VariableValue(b"s_useOpenAL\x00" as *const u8 as *const libc::c_char) {
        soundOptionsInfo.soundSystem_original = 1i32
    } else {
        soundOptionsInfo.soundSystem_original = 0i32
    }
    soundOptionsInfo.soundSystem.curvalue = soundOptionsInfo.soundSystem_original;
    speed = trap_Cvar_VariableValue(b"s_sdlSpeed\x00" as *const u8 as *const libc::c_char)
        as libc::c_int;
    if 0 == speed {
        speed = 22050i32
    }
    if speed <= 11025i32 {
        soundOptionsInfo.quality_original = 0i32
    } else if speed <= 22050i32 {
        soundOptionsInfo.quality_original = 1i32
    } else {
        soundOptionsInfo.quality_original = 2i32
    }
    soundOptionsInfo.quality.curvalue = soundOptionsInfo.quality_original;
}
/*
=================
UI_SoundOptionsMenu_Event
=================
*/
unsafe extern "C" fn UI_SoundOptionsMenu_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        10 => {
            UI_PopMenu();
            UI_GraphicsOptionsMenu();
        }
        11 => {
            UI_PopMenu();
            UI_DisplayOptionsMenu();
        }
        13 => {
            UI_PopMenu();
            UI_NetworkOptionsMenu();
        }
        19 => {
            UI_PopMenu();
        }
        20 => {
            trap_Cvar_SetValue(
                b"s_volume\x00" as *const u8 as *const libc::c_char,
                soundOptionsInfo.sfxvolume.curvalue / 10i32 as libc::c_float,
            );
            soundOptionsInfo.sfxvolume_original = soundOptionsInfo.sfxvolume.curvalue;
            trap_Cvar_SetValue(
                b"s_musicvolume\x00" as *const u8 as *const libc::c_char,
                soundOptionsInfo.musicvolume.curvalue / 10i32 as libc::c_float,
            );
            soundOptionsInfo.musicvolume_original = soundOptionsInfo.musicvolume.curvalue;
            if soundOptionsInfo.quality_original != soundOptionsInfo.quality.curvalue
                || soundOptionsInfo.soundSystem_original != soundOptionsInfo.soundSystem.curvalue
            {
                let mut speed: libc::c_int = 0;
                match soundOptionsInfo.quality.curvalue {
                    1 => speed = 22050i32,
                    2 => speed = 44100i32,
                    0 | _ => speed = 11025i32,
                }
                if speed == 22050i32 {
                    speed = 0i32
                }
                trap_Cvar_SetValue(
                    b"s_sdlSpeed\x00" as *const u8 as *const libc::c_char,
                    speed as libc::c_float,
                );
                soundOptionsInfo.quality_original = soundOptionsInfo.quality.curvalue;
                trap_Cvar_SetValue(
                    b"s_useOpenAL\x00" as *const u8 as *const libc::c_char,
                    (soundOptionsInfo.soundSystem.curvalue == 1i32) as libc::c_int as libc::c_float,
                );
                soundOptionsInfo.soundSystem_original = soundOptionsInfo.soundSystem.curvalue;
                UI_ForceMenuOff();
                trap_Cmd_ExecuteText(
                    EXEC_APPEND as libc::c_int,
                    b"snd_restart\n\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        12 | _ => {}
    };
}
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
/*
=======================================================================

SOUND OPTIONS MENU

=======================================================================
*/
//#define ID_A3D				18
static mut quality_items: [*const libc::c_char; 4] = [
    b"Low\x00" as *const u8 as *const libc::c_char,
    b"Medium\x00" as *const u8 as *const libc::c_char,
    b"High\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut soundSystem_items: [*const libc::c_char; 3] = [
    b"SDL\x00" as *const u8 as *const libc::c_char,
    b"OpenAL\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
================
SoundOptions_MenuDraw
================
*/
#[no_mangle]
pub unsafe extern "C" fn SoundOptions_MenuDraw() {
    SoundOptions_UpdateMenuItems();
    Menu_Draw(&mut soundOptionsInfo.menu);
}
/*
=================
SoundOptions_UpdateMenuItems
=================
*/
unsafe extern "C" fn SoundOptions_UpdateMenuItems() {
    if soundOptionsInfo.soundSystem.curvalue == 0i32 {
        soundOptionsInfo.quality.generic.flags &= !(0x2000i32 as libc::c_uint)
    } else {
        soundOptionsInfo.quality.generic.flags |= 0x2000i32 as libc::c_uint
    }
    soundOptionsInfo.apply.generic.flags |= 0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    if soundOptionsInfo.sfxvolume_original != soundOptionsInfo.sfxvolume.curvalue {
        soundOptionsInfo.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if soundOptionsInfo.musicvolume_original != soundOptionsInfo.musicvolume.curvalue {
        soundOptionsInfo.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if soundOptionsInfo.soundSystem_original != soundOptionsInfo.soundSystem.curvalue {
        soundOptionsInfo.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    }
    if soundOptionsInfo.quality_original != soundOptionsInfo.quality.curvalue {
        soundOptionsInfo.apply.generic.flags &=
            !(0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
    };
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundOptionsInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub graphics: menutext_s,
    pub display: menutext_s,
    pub sound: menutext_s,
    pub network: menutext_s,
    pub sfxvolume: menuslider_s,
    pub musicvolume: menuslider_s,
    pub soundSystem: menulist_s,
    pub quality: menulist_s,
    pub back: menubitmap_s,
    pub apply: menubitmap_s,
    pub sfxvolume_original: libc::c_float,
    pub musicvolume_original: libc::c_float,
    pub soundSystem_original: libc::c_int,
    pub quality_original: libc::c_int,
}
