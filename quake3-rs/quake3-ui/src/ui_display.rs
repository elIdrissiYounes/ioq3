use bg_misc::bg_itemlist;
use libc;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
    AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
    AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, vec4_t, vec_t};
use stdlib::memset;
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, textureCompression_t, GLDRV_ICD,
    GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC, GLHW_PERMEDIA2, GLHW_RAGEPRO,
    GLHW_RIVA128, TC_NONE, TC_S3TC, TC_S3TC_ARB,
};
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
use ui_gameinfo::{
    UI_CanShowTierVideo, UI_GetArenaInfoByMap, UI_GetArenaInfoByNumber, UI_GetAwardLevel,
    UI_GetBestScore, UI_GetBotInfoByName, UI_GetBotInfoByNumber, UI_GetCurrentGame,
    UI_GetNumArenas, UI_GetNumBots, UI_GetNumSPArenas, UI_GetNumSPTiers, UI_GetSpecialArenaInfo,
    UI_InitGameinfo, UI_LogAwardData, UI_NewGame, UI_SPUnlockMedals_f, UI_SPUnlock_f,
    UI_SetBestScore, UI_ShowTierVideo, UI_TierCompleted,
};
use ui_ingame::{InGame_Cache, UI_InGameMenu};
use ui_local_h::{
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menuslider_s, menutext_s,
    trap_Cvar_SetValue, trap_Cvar_VariableValue, trap_R_RegisterShaderNoMip, uiStatic_t,
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
// ui_display.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_DisplayOptionsMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_DisplayOptionsMenu() {
    UI_DisplayOptionsMenu_Init();
    UI_PushMenu(&mut displayOptionsInfo.menu);
    Menu_SetCursorToItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.display as *mut menutext_s as *mut libc::c_void,
    );
}
static mut displayOptionsInfo: displayOptionsInfo_t = displayOptionsInfo_t {
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
    brightness: menuslider_s {
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
    screensize: menuslider_s {
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
UI_DisplayOptionsMenu_Init
===============
*/
unsafe extern "C" fn UI_DisplayOptionsMenu_Init() {
    let mut y: libc::c_int = 0;
    memset(
        &mut displayOptionsInfo as *mut displayOptionsInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<displayOptionsInfo_t>() as libc::c_ulong,
    );
    UI_DisplayOptionsMenu_Cache();
    displayOptionsInfo.menu.wrapAround = qtrue;
    displayOptionsInfo.menu.fullscreen = qtrue;
    displayOptionsInfo.banner.generic.type_0 = 10i32;
    displayOptionsInfo.banner.generic.flags = 0x8i32 as libc::c_uint;
    displayOptionsInfo.banner.generic.x = 320i32;
    displayOptionsInfo.banner.generic.y = 16i32;
    displayOptionsInfo.banner.string =
        b"SYSTEM SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    displayOptionsInfo.banner.color = color_white.as_mut_ptr();
    displayOptionsInfo.banner.style = 0x1i32;
    displayOptionsInfo.framel.generic.type_0 = 6i32;
    displayOptionsInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    displayOptionsInfo.framel.generic.flags = 0x4000i32 as libc::c_uint;
    displayOptionsInfo.framel.generic.x = 0i32;
    displayOptionsInfo.framel.generic.y = 78i32;
    displayOptionsInfo.framel.width = 256i32;
    displayOptionsInfo.framel.height = 329i32;
    displayOptionsInfo.framer.generic.type_0 = 6i32;
    displayOptionsInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    displayOptionsInfo.framer.generic.flags = 0x4000i32 as libc::c_uint;
    displayOptionsInfo.framer.generic.x = 376i32;
    displayOptionsInfo.framer.generic.y = 76i32;
    displayOptionsInfo.framer.width = 256i32;
    displayOptionsInfo.framer.height = 334i32;
    displayOptionsInfo.graphics.generic.type_0 = 9i32;
    displayOptionsInfo.graphics.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    displayOptionsInfo.graphics.generic.id = 10i32;
    displayOptionsInfo.graphics.generic.callback = Some(UI_DisplayOptionsMenu_Event);
    displayOptionsInfo.graphics.generic.x = 216i32;
    displayOptionsInfo.graphics.generic.y = 240i32 - 2i32 * 27i32;
    displayOptionsInfo.graphics.string =
        b"GRAPHICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    displayOptionsInfo.graphics.style = 0x2i32;
    displayOptionsInfo.graphics.color = color_red.as_mut_ptr();
    displayOptionsInfo.display.generic.type_0 = 9i32;
    displayOptionsInfo.display.generic.flags = 0x10i32 as libc::c_uint;
    displayOptionsInfo.display.generic.id = 11i32;
    displayOptionsInfo.display.generic.callback = Some(UI_DisplayOptionsMenu_Event);
    displayOptionsInfo.display.generic.x = 216i32;
    displayOptionsInfo.display.generic.y = 240i32 - 27i32;
    displayOptionsInfo.display.string =
        b"DISPLAY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    displayOptionsInfo.display.style = 0x2i32;
    displayOptionsInfo.display.color = color_red.as_mut_ptr();
    displayOptionsInfo.sound.generic.type_0 = 9i32;
    displayOptionsInfo.sound.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    displayOptionsInfo.sound.generic.id = 12i32;
    displayOptionsInfo.sound.generic.callback = Some(UI_DisplayOptionsMenu_Event);
    displayOptionsInfo.sound.generic.x = 216i32;
    displayOptionsInfo.sound.generic.y = 240i32;
    displayOptionsInfo.sound.string =
        b"SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    displayOptionsInfo.sound.style = 0x2i32;
    displayOptionsInfo.sound.color = color_red.as_mut_ptr();
    displayOptionsInfo.network.generic.type_0 = 9i32;
    displayOptionsInfo.network.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    displayOptionsInfo.network.generic.id = 13i32;
    displayOptionsInfo.network.generic.callback = Some(UI_DisplayOptionsMenu_Event);
    displayOptionsInfo.network.generic.x = 216i32;
    displayOptionsInfo.network.generic.y = 240i32 + 27i32;
    displayOptionsInfo.network.string =
        b"NETWORK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    displayOptionsInfo.network.style = 0x2i32;
    displayOptionsInfo.network.color = color_red.as_mut_ptr();
    y = 240i32 - 1i32 * (16i32 + 2i32);
    displayOptionsInfo.brightness.generic.type_0 = 1i32;
    displayOptionsInfo.brightness.generic.name =
        b"Brightness:\x00" as *const u8 as *const libc::c_char;
    displayOptionsInfo.brightness.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    displayOptionsInfo.brightness.generic.callback = Some(UI_DisplayOptionsMenu_Event);
    displayOptionsInfo.brightness.generic.id = 14i32;
    displayOptionsInfo.brightness.generic.x = 400i32;
    displayOptionsInfo.brightness.generic.y = y;
    displayOptionsInfo.brightness.minvalue = 5i32 as libc::c_float;
    displayOptionsInfo.brightness.maxvalue = 20i32 as libc::c_float;
    if 0 == uis.glconfig.deviceSupportsGamma as u64 {
        displayOptionsInfo.brightness.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 16i32 + 2i32;
    displayOptionsInfo.screensize.generic.type_0 = 1i32;
    displayOptionsInfo.screensize.generic.name =
        b"Screen Size:\x00" as *const u8 as *const libc::c_char;
    displayOptionsInfo.screensize.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    displayOptionsInfo.screensize.generic.callback = Some(UI_DisplayOptionsMenu_Event);
    displayOptionsInfo.screensize.generic.id = 15i32;
    displayOptionsInfo.screensize.generic.x = 400i32;
    displayOptionsInfo.screensize.generic.y = y;
    displayOptionsInfo.screensize.minvalue = 3i32 as libc::c_float;
    displayOptionsInfo.screensize.maxvalue = 10i32 as libc::c_float;
    displayOptionsInfo.back.generic.type_0 = 6i32;
    displayOptionsInfo.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    displayOptionsInfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    displayOptionsInfo.back.generic.callback = Some(UI_DisplayOptionsMenu_Event);
    displayOptionsInfo.back.generic.id = 16i32;
    displayOptionsInfo.back.generic.x = 0i32;
    displayOptionsInfo.back.generic.y = 480i32 - 64i32;
    displayOptionsInfo.back.width = 128i32;
    displayOptionsInfo.back.height = 64i32;
    displayOptionsInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.graphics as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.display as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.sound as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.network as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.brightness as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.screensize as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut displayOptionsInfo.menu,
        &mut displayOptionsInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
    displayOptionsInfo.brightness.curvalue =
        trap_Cvar_VariableValue(b"r_gamma\x00" as *const u8 as *const libc::c_char)
            * 10i32 as libc::c_float;
    displayOptionsInfo.screensize.curvalue =
        trap_Cvar_VariableValue(b"cg_viewsize\x00" as *const u8 as *const libc::c_char)
            / 10i32 as libc::c_float;
}
/*
=================
UI_DisplayOptionsMenu_Event
=================
*/
unsafe extern "C" fn UI_DisplayOptionsMenu_Event(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        10 => {
            UI_PopMenu();
            UI_GraphicsOptionsMenu();
        }
        12 => {
            UI_PopMenu();
            UI_SoundOptionsMenu();
        }
        13 => {
            UI_PopMenu();
            UI_NetworkOptionsMenu();
        }
        14 => {
            trap_Cvar_SetValue(
                b"r_gamma\x00" as *const u8 as *const libc::c_char,
                displayOptionsInfo.brightness.curvalue / 10.0f32,
            );
        }
        15 => {
            trap_Cvar_SetValue(
                b"cg_viewsize\x00" as *const u8 as *const libc::c_char,
                displayOptionsInfo.screensize.curvalue * 10i32 as libc::c_float,
            );
        }
        16 => {
            UI_PopMenu();
        }
        11 | _ => {}
    };
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct displayOptionsInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub graphics: menutext_s,
    pub display: menutext_s,
    pub sound: menutext_s,
    pub network: menutext_s,
    pub brightness: menuslider_s,
    pub screensize: menuslider_s,
    pub back: menubitmap_s,
}
