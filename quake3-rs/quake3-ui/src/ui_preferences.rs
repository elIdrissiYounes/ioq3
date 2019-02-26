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
use q_shared_h::{qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, va, vec4_t, vec_t, Com_Clamp};
use stdlib::{memset, strlen};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menulist_s, menuradiobutton_s,
    menutext_s, trap_Cvar_Reset, trap_Cvar_SetValue, trap_Cvar_VariableValue,
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
// ui_preferences.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_PreferencesMenu() {
    Preferences_MenuInit();
    UI_PushMenu(&mut s_preferences.menu);
}
static mut s_preferences: preferences_t = preferences_t {
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
    crosshair: menulist_s {
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
    simpleitems: menuradiobutton_s {
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
        curvalue: 0,
    },
    brass: menuradiobutton_s {
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
        curvalue: 0,
    },
    wallmarks: menuradiobutton_s {
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
        curvalue: 0,
    },
    dynamiclights: menuradiobutton_s {
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
        curvalue: 0,
    },
    identifytarget: menuradiobutton_s {
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
        curvalue: 0,
    },
    highqualitysky: menuradiobutton_s {
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
        curvalue: 0,
    },
    synceveryframe: menuradiobutton_s {
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
        curvalue: 0,
    },
    forcemodel: menuradiobutton_s {
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
        curvalue: 0,
    },
    drawteamoverlay: menulist_s {
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
    allowdownload: menuradiobutton_s {
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
        curvalue: 0,
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
    crosshairShader: [0; 10],
};
unsafe extern "C" fn Preferences_MenuInit() {
    let mut y: libc::c_int = 0;
    memset(
        &mut s_preferences as *mut preferences_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<preferences_t>() as libc::c_ulong,
    );
    Preferences_Cache();
    s_preferences.menu.wrapAround = qtrue;
    s_preferences.menu.fullscreen = qtrue;
    s_preferences.banner.generic.type_0 = 10i32;
    s_preferences.banner.generic.x = 320i32;
    s_preferences.banner.generic.y = 16i32;
    s_preferences.banner.string =
        b"GAME OPTIONS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_preferences.banner.color = color_white.as_mut_ptr();
    s_preferences.banner.style = 0x1i32;
    s_preferences.framel.generic.type_0 = 6i32;
    s_preferences.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_preferences.framel.generic.flags = 0x4000i32 as libc::c_uint;
    s_preferences.framel.generic.x = 0i32;
    s_preferences.framel.generic.y = 78i32;
    s_preferences.framel.width = 256i32;
    s_preferences.framel.height = 329i32;
    s_preferences.framer.generic.type_0 = 6i32;
    s_preferences.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_preferences.framer.generic.flags = 0x4000i32 as libc::c_uint;
    s_preferences.framer.generic.x = 376i32;
    s_preferences.framer.generic.y = 76i32;
    s_preferences.framer.width = 256i32;
    s_preferences.framer.height = 334i32;
    y = 144i32;
    s_preferences.crosshair.generic.type_0 = 3i32;
    s_preferences.crosshair.generic.flags = 0x100i32 as libc::c_uint
        | 0x2i32 as libc::c_uint
        | 0x8000i32 as libc::c_uint
        | 0x10000i32 as libc::c_uint;
    s_preferences.crosshair.generic.x = 360i32;
    s_preferences.crosshair.generic.y = y;
    s_preferences.crosshair.generic.name = b"Crosshair:\x00" as *const u8 as *const libc::c_char;
    s_preferences.crosshair.generic.callback = Some(Preferences_Event);
    s_preferences.crosshair.generic.ownerdraw = Some(Crosshair_Draw);
    s_preferences.crosshair.generic.id = 127i32;
    s_preferences.crosshair.generic.top = y - 4i32;
    s_preferences.crosshair.generic.bottom = y + 20i32;
    s_preferences.crosshair.generic.left = (360i32 as libc::c_ulong).wrapping_sub(
        strlen(s_preferences.crosshair.generic.name)
            .wrapping_add(1i32 as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong),
    ) as libc::c_int;
    s_preferences.crosshair.generic.right = 360i32 + 48i32;
    s_preferences.crosshair.numitems = 10i32;
    y += 16i32 + 2i32 + 4i32;
    s_preferences.simpleitems.generic.type_0 = 5i32;
    s_preferences.simpleitems.generic.name =
        b"Simple Items:\x00" as *const u8 as *const libc::c_char;
    s_preferences.simpleitems.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_preferences.simpleitems.generic.callback = Some(Preferences_Event);
    s_preferences.simpleitems.generic.id = 128i32;
    s_preferences.simpleitems.generic.x = 360i32;
    s_preferences.simpleitems.generic.y = y;
    y += 16i32;
    s_preferences.wallmarks.generic.type_0 = 5i32;
    s_preferences.wallmarks.generic.name =
        b"Marks on Walls:\x00" as *const u8 as *const libc::c_char;
    s_preferences.wallmarks.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_preferences.wallmarks.generic.callback = Some(Preferences_Event);
    s_preferences.wallmarks.generic.id = 131i32;
    s_preferences.wallmarks.generic.x = 360i32;
    s_preferences.wallmarks.generic.y = y;
    y += 16i32 + 2i32;
    s_preferences.brass.generic.type_0 = 5i32;
    s_preferences.brass.generic.name = b"Ejecting Brass:\x00" as *const u8 as *const libc::c_char;
    s_preferences.brass.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_preferences.brass.generic.callback = Some(Preferences_Event);
    s_preferences.brass.generic.id = 130i32;
    s_preferences.brass.generic.x = 360i32;
    s_preferences.brass.generic.y = y;
    y += 16i32 + 2i32;
    s_preferences.dynamiclights.generic.type_0 = 5i32;
    s_preferences.dynamiclights.generic.name =
        b"Dynamic Lights:\x00" as *const u8 as *const libc::c_char;
    s_preferences.dynamiclights.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_preferences.dynamiclights.generic.callback = Some(Preferences_Event);
    s_preferences.dynamiclights.generic.id = 132i32;
    s_preferences.dynamiclights.generic.x = 360i32;
    s_preferences.dynamiclights.generic.y = y;
    y += 16i32 + 2i32;
    s_preferences.identifytarget.generic.type_0 = 5i32;
    s_preferences.identifytarget.generic.name =
        b"Identify Target:\x00" as *const u8 as *const libc::c_char;
    s_preferences.identifytarget.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_preferences.identifytarget.generic.callback = Some(Preferences_Event);
    s_preferences.identifytarget.generic.id = 133i32;
    s_preferences.identifytarget.generic.x = 360i32;
    s_preferences.identifytarget.generic.y = y;
    y += 16i32 + 2i32;
    s_preferences.highqualitysky.generic.type_0 = 5i32;
    s_preferences.highqualitysky.generic.name =
        b"High Quality Sky:\x00" as *const u8 as *const libc::c_char;
    s_preferences.highqualitysky.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_preferences.highqualitysky.generic.callback = Some(Preferences_Event);
    s_preferences.highqualitysky.generic.id = 129i32;
    s_preferences.highqualitysky.generic.x = 360i32;
    s_preferences.highqualitysky.generic.y = y;
    y += 16i32 + 2i32;
    s_preferences.synceveryframe.generic.type_0 = 5i32;
    s_preferences.synceveryframe.generic.name =
        b"Sync Every Frame:\x00" as *const u8 as *const libc::c_char;
    s_preferences.synceveryframe.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_preferences.synceveryframe.generic.callback = Some(Preferences_Event);
    s_preferences.synceveryframe.generic.id = 134i32;
    s_preferences.synceveryframe.generic.x = 360i32;
    s_preferences.synceveryframe.generic.y = y;
    y += 16i32 + 2i32;
    s_preferences.forcemodel.generic.type_0 = 5i32;
    s_preferences.forcemodel.generic.name =
        b"Force Player Models:\x00" as *const u8 as *const libc::c_char;
    s_preferences.forcemodel.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_preferences.forcemodel.generic.callback = Some(Preferences_Event);
    s_preferences.forcemodel.generic.id = 135i32;
    s_preferences.forcemodel.generic.x = 360i32;
    s_preferences.forcemodel.generic.y = y;
    y += 16i32 + 2i32;
    s_preferences.drawteamoverlay.generic.type_0 = 3i32;
    s_preferences.drawteamoverlay.generic.name =
        b"Draw Team Overlay:\x00" as *const u8 as *const libc::c_char;
    s_preferences.drawteamoverlay.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_preferences.drawteamoverlay.generic.callback = Some(Preferences_Event);
    s_preferences.drawteamoverlay.generic.id = 136i32;
    s_preferences.drawteamoverlay.generic.x = 360i32;
    s_preferences.drawteamoverlay.generic.y = y;
    s_preferences.drawteamoverlay.itemnames = teamoverlay_names.as_mut_ptr();
    y += 16i32 + 2i32;
    s_preferences.allowdownload.generic.type_0 = 5i32;
    s_preferences.allowdownload.generic.name =
        b"Automatic Downloading:\x00" as *const u8 as *const libc::c_char;
    s_preferences.allowdownload.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_preferences.allowdownload.generic.callback = Some(Preferences_Event);
    s_preferences.allowdownload.generic.id = 137i32;
    s_preferences.allowdownload.generic.x = 360i32;
    s_preferences.allowdownload.generic.y = y;
    s_preferences.back.generic.type_0 = 6i32;
    s_preferences.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_preferences.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_preferences.back.generic.callback = Some(Preferences_Event);
    s_preferences.back.generic.id = 138i32;
    s_preferences.back.generic.x = 0i32;
    s_preferences.back.generic.y = 480i32 - 64i32;
    s_preferences.back.width = 128i32;
    s_preferences.back.height = 64i32;
    s_preferences.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.crosshair as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.simpleitems as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.wallmarks as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.brass as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.dynamiclights as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.identifytarget as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.highqualitysky as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.synceveryframe as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.forcemodel as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.drawteamoverlay as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.allowdownload as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_preferences.menu,
        &mut s_preferences.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Preferences_SetMenuItems();
}
unsafe extern "C" fn Preferences_SetMenuItems() {
    s_preferences.crosshair.curvalue =
        trap_Cvar_VariableValue(b"cg_drawCrosshair\x00" as *const u8 as *const libc::c_char)
            as libc::c_int
            % 10i32;
    s_preferences.simpleitems.curvalue =
        (trap_Cvar_VariableValue(b"cg_simpleItems\x00" as *const u8 as *const libc::c_char)
            != 0i32 as libc::c_float) as libc::c_int;
    s_preferences.brass.curvalue =
        (trap_Cvar_VariableValue(b"cg_brassTime\x00" as *const u8 as *const libc::c_char)
            != 0i32 as libc::c_float) as libc::c_int;
    s_preferences.wallmarks.curvalue =
        (trap_Cvar_VariableValue(b"cg_marks\x00" as *const u8 as *const libc::c_char)
            != 0i32 as libc::c_float) as libc::c_int;
    s_preferences.identifytarget.curvalue =
        (trap_Cvar_VariableValue(b"cg_drawCrosshairNames\x00" as *const u8 as *const libc::c_char)
            != 0i32 as libc::c_float) as libc::c_int;
    s_preferences.dynamiclights.curvalue =
        (trap_Cvar_VariableValue(b"r_dynamiclight\x00" as *const u8 as *const libc::c_char)
            != 0i32 as libc::c_float) as libc::c_int;
    s_preferences.highqualitysky.curvalue =
        (trap_Cvar_VariableValue(b"r_fastsky\x00" as *const u8 as *const libc::c_char)
            == 0i32 as libc::c_float) as libc::c_int;
    s_preferences.synceveryframe.curvalue =
        (trap_Cvar_VariableValue(b"r_finish\x00" as *const u8 as *const libc::c_char)
            != 0i32 as libc::c_float) as libc::c_int;
    s_preferences.forcemodel.curvalue =
        (trap_Cvar_VariableValue(b"cg_forcemodel\x00" as *const u8 as *const libc::c_char)
            != 0i32 as libc::c_float) as libc::c_int;
    s_preferences.drawteamoverlay.curvalue = Com_Clamp(
        0i32 as libc::c_float,
        3i32 as libc::c_float,
        trap_Cvar_VariableValue(b"cg_drawTeamOverlay\x00" as *const u8 as *const libc::c_char),
    ) as libc::c_int;
    s_preferences.allowdownload.curvalue =
        (trap_Cvar_VariableValue(b"cl_allowDownload\x00" as *const u8 as *const libc::c_char)
            != 0i32 as libc::c_float) as libc::c_int;
}
unsafe extern "C" fn Preferences_Event(mut ptr: *mut libc::c_void, mut notification: libc::c_int) {
    if notification != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        127 => {
            trap_Cvar_SetValue(
                b"cg_drawCrosshair\x00" as *const u8 as *const libc::c_char,
                s_preferences.crosshair.curvalue as libc::c_float,
            );
        }
        128 => {
            trap_Cvar_SetValue(
                b"cg_simpleItems\x00" as *const u8 as *const libc::c_char,
                s_preferences.simpleitems.curvalue as libc::c_float,
            );
        }
        129 => {
            trap_Cvar_SetValue(
                b"r_fastsky\x00" as *const u8 as *const libc::c_char,
                (0 == s_preferences.highqualitysky.curvalue) as libc::c_int as libc::c_float,
            );
        }
        130 => {
            if 0 != s_preferences.brass.curvalue {
                trap_Cvar_Reset(b"cg_brassTime\x00" as *const u8 as *const libc::c_char);
            } else {
                trap_Cvar_SetValue(
                    b"cg_brassTime\x00" as *const u8 as *const libc::c_char,
                    0i32 as libc::c_float,
                );
            }
        }
        131 => {
            trap_Cvar_SetValue(
                b"cg_marks\x00" as *const u8 as *const libc::c_char,
                s_preferences.wallmarks.curvalue as libc::c_float,
            );
        }
        132 => {
            trap_Cvar_SetValue(
                b"r_dynamiclight\x00" as *const u8 as *const libc::c_char,
                s_preferences.dynamiclights.curvalue as libc::c_float,
            );
        }
        133 => {
            trap_Cvar_SetValue(
                b"cg_drawCrosshairNames\x00" as *const u8 as *const libc::c_char,
                s_preferences.identifytarget.curvalue as libc::c_float,
            );
        }
        134 => {
            trap_Cvar_SetValue(
                b"r_finish\x00" as *const u8 as *const libc::c_char,
                s_preferences.synceveryframe.curvalue as libc::c_float,
            );
        }
        135 => {
            trap_Cvar_SetValue(
                b"cg_forcemodel\x00" as *const u8 as *const libc::c_char,
                s_preferences.forcemodel.curvalue as libc::c_float,
            );
        }
        136 => {
            trap_Cvar_SetValue(
                b"cg_drawTeamOverlay\x00" as *const u8 as *const libc::c_char,
                s_preferences.drawteamoverlay.curvalue as libc::c_float,
            );
        }
        137 => {
            trap_Cvar_SetValue(
                b"cl_allowDownload\x00" as *const u8 as *const libc::c_char,
                s_preferences.allowdownload.curvalue as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"sv_allowDownload\x00" as *const u8 as *const libc::c_char,
                s_preferences.allowdownload.curvalue as libc::c_float,
            );
        }
        138 => {
            UI_PopMenu();
        }
        _ => {}
    };
}
static mut teamoverlay_names: [*const libc::c_char; 5] = [
    b"off\x00" as *const u8 as *const libc::c_char,
    b"upper right\x00" as *const u8 as *const libc::c_char,
    b"lower right\x00" as *const u8 as *const libc::c_char,
    b"lower left\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
Crosshair_Draw
=================
*/
unsafe extern "C" fn Crosshair_Draw(mut self_0: *mut libc::c_void) {
    let mut s: *mut menulist_s = 0 as *mut menulist_s;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut focus: qboolean = qfalse;
    s = self_0 as *mut menulist_s;
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
    if 0 == (*s).curvalue {
        return;
    }
    UI_DrawHandlePic(
        (x + 8i32) as libc::c_float,
        (y - 4i32) as libc::c_float,
        24i32 as libc::c_float,
        24i32 as libc::c_float,
        s_preferences.crosshairShader[(*s).curvalue as usize],
    );
}
#[no_mangle]
pub unsafe extern "C" fn Preferences_Cache() {
    let mut n: libc::c_int = 0;
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    n = 0i32;
    while n < 10i32 {
        s_preferences.crosshairShader[n as usize] = trap_R_RegisterShaderNoMip(va(
            b"gfx/2d/crosshair%c\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            'a' as i32 + n,
        ));
        n += 1
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct preferences_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub crosshair: menulist_s,
    pub simpleitems: menuradiobutton_s,
    pub brass: menuradiobutton_s,
    pub wallmarks: menuradiobutton_s,
    pub dynamiclights: menuradiobutton_s,
    pub identifytarget: menuradiobutton_s,
    pub highqualitysky: menuradiobutton_s,
    pub synceveryframe: menuradiobutton_s,
    pub forcemodel: menuradiobutton_s,
    pub drawteamoverlay: menulist_s,
    pub allowdownload: menuradiobutton_s,
    pub back: menubitmap_s,
    pub crosshairShader: [qhandle_t; 10],
}
