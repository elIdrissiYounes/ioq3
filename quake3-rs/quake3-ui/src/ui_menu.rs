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
    byte, cvarHandle_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, vec3_t, vec4_t,
    vec_t, vmCvar_t, Q_stricmp, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::{memset, sin, strlen};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, refEntityType_t, refEntity_t, refdef_t,
    textureCompression_t, GLDRV_ICD, GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC,
    GLHW_PERMEDIA2, GLHW_RAGEPRO, GLHW_RIVA128, RT_BEAM, RT_LIGHTNING, RT_MAX_REF_ENTITY_TYPE,
    RT_MODEL, RT_POLY, RT_PORTALSURFACE, RT_RAIL_CORE, RT_RAIL_RINGS, RT_SPRITE, TC_NONE, TC_S3TC,
    TC_S3TC_ARB,
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
    _tag_menuframework, menucommon_s, menuframework_s, menutext_s, trap_Cmd_ExecuteText,
    trap_Cvar_Set, trap_Cvar_VariableStringBuffer, trap_FS_GetFileList, trap_GetCDKey,
    trap_Key_SetCatcher, trap_R_AddRefEntityToScene, trap_R_ClearScene, trap_R_RegisterModel,
    trap_R_RenderScene, trap_VerifyCDKey, uiStatic_t,
};
use ui_main::{
    ui_browserGameType, ui_browserMaster, ui_browserShowEmpty, ui_browserShowFull,
    ui_browserSortKey, ui_cdkeychecked, UI_RegisterCvars, UI_UpdateCvars,
};
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

//
// ui_menu.c
//
#[no_mangle]
pub unsafe extern "C" fn MainMenu_Cache() {
    s_main.bannerModel = trap_R_RegisterModel(
        b"models/mapobjects/banner/banner5.md3\x00" as *const u8 as *const libc::c_char,
    );
}
static mut s_main: mainmenu_t = mainmenu_t {
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
    singleplayer: menutext_s {
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
    multiplayer: menutext_s {
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
    setup: menutext_s {
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
    demos: menutext_s {
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
    cinematics: menutext_s {
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
    teamArena: menutext_s {
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
    mods: menutext_s {
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
    exit: menutext_s {
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
    bannerModel: 0,
};
#[no_mangle]
pub unsafe extern "C" fn UI_MainMenu() {
    let mut y: libc::c_int = 0;
    let mut teamArena: qboolean = qfalse;
    let mut style: libc::c_int = 0x1i32 | 0x800i32;
    trap_Cvar_Set(
        b"sv_killserver\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
    );
    if 0 == uis.demoversion as u64 && 0 == ui_cdkeychecked.integer {
        let mut key: [libc::c_char; 17] = [0; 17];
        trap_GetCDKey(
            key.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 17]>() as libc::c_ulong as libc::c_int,
        );
        if trap_VerifyCDKey(key.as_mut_ptr(), 0 as *const libc::c_char) as libc::c_uint
            == qfalse as libc::c_int as libc::c_uint
        {
            UI_CDKeyMenu();
            return;
        }
    }
    memset(
        &mut s_main as *mut mainmenu_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<mainmenu_t>() as libc::c_ulong,
    );
    memset(
        &mut s_errorMessage as *mut errorMessage_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<errorMessage_t>() as libc::c_ulong,
    );
    MainMenu_Cache();
    trap_Cvar_VariableStringBuffer(
        b"com_errorMessage\x00" as *const u8 as *const libc::c_char,
        s_errorMessage.errorMessage.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
    );
    if 0 != strlen(s_errorMessage.errorMessage.as_mut_ptr()) {
        s_errorMessage.menu.draw = Some(Main_MenuDraw);
        s_errorMessage.menu.key = Some(ErrorMessage_Key);
        s_errorMessage.menu.fullscreen = qtrue;
        s_errorMessage.menu.wrapAround = qtrue;
        s_errorMessage.menu.showlogo = qtrue;
        trap_Key_SetCatcher(0x2i32);
        uis.menusp = 0i32;
        UI_PushMenu(&mut s_errorMessage.menu);
        return;
    }
    s_main.menu.draw = Some(Main_MenuDraw);
    s_main.menu.fullscreen = qtrue;
    s_main.menu.wrapAround = qtrue;
    s_main.menu.showlogo = qtrue;
    y = 134i32;
    s_main.singleplayer.generic.type_0 = 9i32;
    s_main.singleplayer.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_main.singleplayer.generic.x = 320i32;
    s_main.singleplayer.generic.y = y;
    s_main.singleplayer.generic.id = 10i32;
    s_main.singleplayer.generic.callback = Some(Main_MenuEvent);
    s_main.singleplayer.string =
        b"SINGLE PLAYER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.singleplayer.color = color_red.as_mut_ptr();
    s_main.singleplayer.style = style;
    y += 34i32;
    s_main.multiplayer.generic.type_0 = 9i32;
    s_main.multiplayer.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_main.multiplayer.generic.x = 320i32;
    s_main.multiplayer.generic.y = y;
    s_main.multiplayer.generic.id = 11i32;
    s_main.multiplayer.generic.callback = Some(Main_MenuEvent);
    s_main.multiplayer.string =
        b"MULTIPLAYER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.multiplayer.color = color_red.as_mut_ptr();
    s_main.multiplayer.style = style;
    y += 34i32;
    s_main.setup.generic.type_0 = 9i32;
    s_main.setup.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_main.setup.generic.x = 320i32;
    s_main.setup.generic.y = y;
    s_main.setup.generic.id = 12i32;
    s_main.setup.generic.callback = Some(Main_MenuEvent);
    s_main.setup.string = b"SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.setup.color = color_red.as_mut_ptr();
    s_main.setup.style = style;
    y += 34i32;
    s_main.demos.generic.type_0 = 9i32;
    s_main.demos.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_main.demos.generic.x = 320i32;
    s_main.demos.generic.y = y;
    s_main.demos.generic.id = 13i32;
    s_main.demos.generic.callback = Some(Main_MenuEvent);
    s_main.demos.string = b"DEMOS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.demos.color = color_red.as_mut_ptr();
    s_main.demos.style = style;
    y += 34i32;
    s_main.cinematics.generic.type_0 = 9i32;
    s_main.cinematics.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_main.cinematics.generic.x = 320i32;
    s_main.cinematics.generic.y = y;
    s_main.cinematics.generic.id = 14i32;
    s_main.cinematics.generic.callback = Some(Main_MenuEvent);
    s_main.cinematics.string =
        b"CINEMATICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.cinematics.color = color_red.as_mut_ptr();
    s_main.cinematics.style = style;
    if 0 == uis.demoversion as u64 && 0 != UI_TeamArenaExists() as libc::c_uint {
        teamArena = qtrue;
        y += 34i32;
        s_main.teamArena.generic.type_0 = 9i32;
        s_main.teamArena.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
        s_main.teamArena.generic.x = 320i32;
        s_main.teamArena.generic.y = y;
        s_main.teamArena.generic.id = 15i32;
        s_main.teamArena.generic.callback = Some(Main_MenuEvent);
        s_main.teamArena.string =
            b"TEAM ARENA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        s_main.teamArena.color = color_red.as_mut_ptr();
        s_main.teamArena.style = style
    }
    if 0 == uis.demoversion as u64 {
        y += 34i32;
        s_main.mods.generic.type_0 = 9i32;
        s_main.mods.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
        s_main.mods.generic.x = 320i32;
        s_main.mods.generic.y = y;
        s_main.mods.generic.id = 16i32;
        s_main.mods.generic.callback = Some(Main_MenuEvent);
        s_main.mods.string = b"MODS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        s_main.mods.color = color_red.as_mut_ptr();
        s_main.mods.style = style
    }
    y += 34i32;
    s_main.exit.generic.type_0 = 9i32;
    s_main.exit.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_main.exit.generic.x = 320i32;
    s_main.exit.generic.y = y;
    s_main.exit.generic.id = 17i32;
    s_main.exit.generic.callback = Some(Main_MenuEvent);
    s_main.exit.string = b"EXIT QUAKE3-RS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_main.exit.color = color_orange.as_mut_ptr();
    s_main.exit.style = style;
    Menu_AddItem(
        &mut s_main.menu,
        &mut s_main.singleplayer as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_main.menu,
        &mut s_main.multiplayer as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_main.menu,
        &mut s_main.setup as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_main.menu,
        &mut s_main.demos as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_main.menu,
        &mut s_main.cinematics as *mut menutext_s as *mut libc::c_void,
    );
    if 0 != teamArena as u64 {
        Menu_AddItem(
            &mut s_main.menu,
            &mut s_main.teamArena as *mut menutext_s as *mut libc::c_void,
        );
    }
    if 0 == uis.demoversion as u64 {
        Menu_AddItem(
            &mut s_main.menu,
            &mut s_main.mods as *mut menutext_s as *mut libc::c_void,
        );
    }
    Menu_AddItem(
        &mut s_main.menu,
        &mut s_main.exit as *mut menutext_s as *mut libc::c_void,
    );
    trap_Key_SetCatcher(0x2i32);
    uis.menusp = 0i32;
    UI_PushMenu(&mut s_main.menu);
}
/*
=================
Main_MenuEvent
=================
*/
#[no_mangle]
pub unsafe extern "C" fn Main_MenuEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        10 => {
            UI_SPLevelMenu();
        }
        11 => {
            UI_ArenaServersMenu();
        }
        12 => {
            UI_SetupMenu();
        }
        13 => {
            UI_DemosMenu();
        }
        14 => {
            UI_CinematicsMenu();
        }
        16 => {
            UI_ModsMenu();
        }
        15 => {
            trap_Cvar_Set(
                b"fs_game\x00" as *const u8 as *const libc::c_char,
                b"missionpack\x00" as *const u8 as *const libc::c_char,
            );
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                b"vid_restart;\x00" as *const u8 as *const libc::c_char,
            );
        }
        17 => {
            UI_ConfirmMenu(
                b"EXIT GAME?\x00" as *const u8 as *const libc::c_char,
                None,
                Some(MainMenu_ExitAction),
            );
        }
        _ => {}
    };
}
/*
=================
MainMenu_ExitAction
=================
*/
unsafe extern "C" fn MainMenu_ExitAction(mut result: qboolean) {
    if 0 == result as u64 {
        return;
    }
    UI_PopMenu();
    UI_CreditMenu();
}
/*
===============
UI_TeamArenaExists
===============
*/
unsafe extern "C" fn UI_TeamArenaExists() -> qboolean {
    let mut numdirs: libc::c_int = 0;
    let mut dirlist: [libc::c_char; 2048] = [0; 2048];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut descptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut dirlen: libc::c_int = 0;
    numdirs = trap_FS_GetFileList(
        b"$modlist\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong as libc::c_int,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0i32;
    while i < numdirs {
        dirlen = strlen(dirptr).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
        descptr = dirptr.offset(dirlen as isize);
        if Q_stricmp(
            dirptr,
            b"missionpack\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            return qtrue;
        }
        dirptr = dirptr.offset(
            (dirlen as libc::c_ulong)
                .wrapping_add(strlen(descptr))
                .wrapping_add(1i32 as libc::c_ulong) as isize,
        );
        i += 1
    }
    return qfalse;
}
/*
===============
Main_MenuDraw
TTimo: this function is common to the main menu and errorMessage menu
===============
*/
unsafe extern "C" fn Main_MenuDraw() {
    let mut refdef: refdef_t = refdef_t {
        x: 0,
        y: 0,
        width: 0,
        height: 0,
        fov_x: 0.,
        fov_y: 0.,
        vieworg: [0.; 3],
        viewaxis: [[0.; 3]; 3],
        time: 0,
        rdflags: 0,
        areamask: [0; 32],
        text: [[0; 32]; 8],
    };
    let mut ent: refEntity_t = refEntity_t {
        reType: RT_MODEL,
        renderfx: 0,
        hModel: 0,
        lightingOrigin: [0.; 3],
        shadowPlane: 0.,
        axis: [[0.; 3]; 3],
        nonNormalizedAxes: qfalse,
        origin: [0.; 3],
        frame: 0,
        oldorigin: [0.; 3],
        oldframe: 0,
        backlerp: 0.,
        skinNum: 0,
        customSkin: 0,
        customShader: 0,
        shaderRGBA: [0; 4],
        shaderTexCoord: [0.; 2],
        shaderTime: 0.,
        radius: 0.,
        rotation: 0.,
    };
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    let mut adjust: libc::c_float = 0.;
    let mut x: libc::c_float = 0.;
    let mut y: libc::c_float = 0.;
    let mut w: libc::c_float = 0.;
    let mut h: libc::c_float = 0.;
    let mut color: vec4_t = [0.5f64 as vec_t, 0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    memset(
        &mut refdef as *mut refdef_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refdef_t>() as libc::c_ulong,
    );
    refdef.rdflags = 0x1i32;
    AxisClear(refdef.viewaxis.as_mut_ptr());
    x = 0i32 as libc::c_float;
    y = 0i32 as libc::c_float;
    w = 640i32 as libc::c_float;
    h = 120i32 as libc::c_float;
    UI_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    refdef.x = x as libc::c_int;
    refdef.y = y as libc::c_int;
    refdef.width = w as libc::c_int;
    refdef.height = h as libc::c_int;
    adjust = 0i32 as libc::c_float;
    refdef.fov_x = 60i32 as libc::c_float + adjust;
    refdef.fov_y = (19.6875f64 + adjust as libc::c_double) as libc::c_float;
    refdef.time = uis.realtime;
    origin[0usize] = 300i32 as vec_t;
    origin[1usize] = 0i32 as vec_t;
    origin[2usize] = -32i32 as vec_t;
    trap_R_ClearScene();
    memset(
        &mut ent as *mut refEntity_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<refEntity_t>() as libc::c_ulong,
    );
    adjust = (5.0f64
        * sin((uis.realtime as libc::c_float / 5000i32 as libc::c_float) as libc::c_double))
        as libc::c_float;
    angles[0usize] = 0i32 as vec_t;
    angles[1usize] = 180i32 as libc::c_float + adjust;
    angles[2usize] = 0i32 as vec_t;
    AnglesToAxis(angles.as_mut_ptr() as *const vec_t, ent.axis.as_mut_ptr());
    ent.hModel = s_main.bannerModel;
    ent.origin[0usize] = origin[0usize];
    ent.origin[1usize] = origin[1usize];
    ent.origin[2usize] = origin[2usize];
    ent.lightingOrigin[0usize] = origin[0usize];
    ent.lightingOrigin[1usize] = origin[1usize];
    ent.lightingOrigin[2usize] = origin[2usize];
    ent.renderfx = 0x80i32 | 0x40i32;
    ent.oldorigin[0usize] = ent.origin[0usize];
    ent.oldorigin[1usize] = ent.origin[1usize];
    ent.oldorigin[2usize] = ent.origin[2usize];
    trap_R_AddRefEntityToScene(&mut ent);
    trap_R_RenderScene(&mut refdef);
    if 0 != strlen(s_errorMessage.errorMessage.as_mut_ptr()) {
        UI_DrawProportionalString_AutoWrapped(
            320i32,
            192i32,
            600i32,
            20i32,
            s_errorMessage.errorMessage.as_mut_ptr(),
            0x1i32 | 0x10i32 | 0x800i32,
            menu_text_color.as_mut_ptr(),
        );
    } else {
        Menu_Draw(&mut s_main.menu);
    }
    if 0 != uis.demoversion as u64 {
        UI_DrawProportionalString(
            320i32,
            372i32,
            b"DEMO      FOR MATURE AUDIENCES      DEMO\x00" as *const u8 as *const libc::c_char,
            0x1i32 | 0x10i32,
            color.as_mut_ptr(),
        );
        UI_DrawString(
            320i32,
            400i32,
            b"Quake III Arena(c) 1999-2000, Id Software, Inc.  All Rights Reserved\x00" as *const u8
                as *const libc::c_char,
            0x1i32 | 0x10i32,
            color.as_mut_ptr(),
        );
    } else {
        UI_DrawString(
            320i32,
            450i32,
            b"Quake III Arena(c) 1999-2000, Id Software, Inc.  All Rights Reserved\x00" as *const u8
                as *const libc::c_char,
            0x1i32 | 0x10i32,
            color.as_mut_ptr(),
        );
    };
}
static mut s_errorMessage: errorMessage_t = errorMessage_t {
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
    errorMessage: [0; 4096],
};
#[no_mangle]
pub unsafe extern "C" fn ErrorMessage_Key(mut key: libc::c_int) -> sfxHandle_t {
    trap_Cvar_Set(
        b"com_errorMessage\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
    );
    UI_MainMenu();
    return menu_null_sound;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mainmenu_t {
    pub menu: menuframework_s,
    pub singleplayer: menutext_s,
    pub multiplayer: menutext_s,
    pub setup: menutext_s,
    pub demos: menutext_s,
    pub cinematics: menutext_s,
    pub teamArena: menutext_s,
    pub mods: menutext_s,
    pub exit: menutext_s,
    pub bannerModel: qhandle_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct errorMessage_t {
    pub menu: menuframework_s,
    pub errorMessage: [libc::c_char; 4096],
}
