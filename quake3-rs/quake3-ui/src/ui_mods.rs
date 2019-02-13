use bg_misc::bg_itemlist;
use libc;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, g_color_table, vec3_origin, vectoangles,
    AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract, AnglesToAxis,
    AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t, Q_strncpyz,
    EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::{memset, strlen};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menulist_s, menutext_s,
    trap_Cmd_ExecuteText, trap_Cvar_Set, trap_FS_GetFileList, trap_Print,
    trap_R_RegisterShaderNoMip,
};
use ui_main::{
    ui_browserGameType, ui_browserMaster, ui_browserShowEmpty, ui_browserShowFull,
    ui_browserSortKey, ui_cdkeychecked, UI_RegisterCvars, UI_UpdateCvars,
};
use ui_menu::{MainMenu_Cache, UI_MainMenu};
use ui_mfield::{MField_Draw, MenuField_Draw, MenuField_Init, MenuField_Key};
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
// ui_mods.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_ModsMenu() {
    UI_Mods_MenuInit();
    UI_PushMenu(&mut s_mods.menu);
}
static mut s_mods: mods_t = mods_t {
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
    list: menulist_s {
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
    go: menubitmap_s {
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
    description: [0; 3072],
    fs_game: [0; 1024],
    descriptionPtr: 0 as *const libc::c_char as *mut libc::c_char,
    fs_gamePtr: 0 as *const libc::c_char as *mut libc::c_char,
    descriptionList: [0 as *const libc::c_char as *mut libc::c_char; 64],
    fs_gameList: [0 as *const libc::c_char as *mut libc::c_char; 64],
};
/*
===============
UI_Mods_MenuInit
===============
*/
unsafe extern "C" fn UI_Mods_MenuInit() {
    UI_ModsMenu_Cache();
    memset(
        &mut s_mods as *mut mods_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<mods_t>() as libc::c_ulong,
    );
    s_mods.menu.wrapAround = qtrue;
    s_mods.menu.fullscreen = qtrue;
    s_mods.banner.generic.type_0 = 10i32;
    s_mods.banner.generic.x = 320i32;
    s_mods.banner.generic.y = 16i32;
    s_mods.banner.string = b"MODS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_mods.banner.color = color_white.as_mut_ptr();
    s_mods.banner.style = 0x1i32;
    s_mods.framel.generic.type_0 = 6i32;
    s_mods.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_mods.framel.generic.flags = 0x4000i32 as libc::c_uint;
    s_mods.framel.generic.x = 0i32;
    s_mods.framel.generic.y = 78i32;
    s_mods.framel.width = 256i32;
    s_mods.framel.height = 329i32;
    s_mods.framer.generic.type_0 = 6i32;
    s_mods.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_mods.framer.generic.flags = 0x4000i32 as libc::c_uint;
    s_mods.framer.generic.x = 376i32;
    s_mods.framer.generic.y = 76i32;
    s_mods.framer.width = 256i32;
    s_mods.framer.height = 334i32;
    s_mods.back.generic.type_0 = 6i32;
    s_mods.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_mods.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_mods.back.generic.id = 10i32;
    s_mods.back.generic.callback = Some(UI_Mods_MenuEvent);
    s_mods.back.generic.x = 0i32;
    s_mods.back.generic.y = 480i32 - 64i32;
    s_mods.back.width = 128i32;
    s_mods.back.height = 64i32;
    s_mods.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_mods.go.generic.type_0 = 6i32;
    s_mods.go.generic.name = b"menu/art/load_0\x00" as *const u8 as *const libc::c_char;
    s_mods.go.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_mods.go.generic.id = 11i32;
    s_mods.go.generic.callback = Some(UI_Mods_MenuEvent);
    s_mods.go.generic.x = 640i32;
    s_mods.go.generic.y = 480i32 - 64i32;
    s_mods.go.width = 128i32;
    s_mods.go.height = 64i32;
    s_mods.go.focuspic =
        b"menu/art/load_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_mods.list.generic.type_0 = 8i32;
    s_mods.list.generic.flags = 0x100i32 as libc::c_uint | 0x8i32 as libc::c_uint;
    s_mods.list.generic.callback = Some(UI_Mods_MenuEvent);
    s_mods.list.generic.id = 12i32;
    s_mods.list.generic.x = 320i32;
    s_mods.list.generic.y = 130i32;
    s_mods.list.width = 48i32;
    s_mods.list.height = 14i32;
    UI_Mods_LoadMods();
    Menu_AddItem(
        &mut s_mods.menu,
        &mut s_mods.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_mods.menu,
        &mut s_mods.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_mods.menu,
        &mut s_mods.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_mods.menu,
        &mut s_mods.list as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_mods.menu,
        &mut s_mods.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_mods.menu,
        &mut s_mods.go as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
===============
UI_Mods_LoadMods
===============
*/
unsafe extern "C" fn UI_Mods_LoadMods() {
    let mut numdirs: libc::c_int = 0;
    let mut dirlist: [libc::c_char; 2048] = [0; 2048];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut descptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut dirlen: libc::c_int = 0;
    s_mods.list.itemnames = s_mods.descriptionList.as_mut_ptr() as *mut *const libc::c_char;
    s_mods.descriptionPtr = s_mods.description.as_mut_ptr();
    s_mods.fs_gamePtr = s_mods.fs_game.as_mut_ptr();
    s_mods.list.numitems = 1i32;
    let ref mut fresh0 = *s_mods.list.itemnames.offset(0isize);
    s_mods.descriptionList[0usize] =
        b"Quake III Arena\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    *fresh0 = s_mods.descriptionList[0usize];
    s_mods.fs_gameList[0usize] = b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
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
        UI_Mods_ParseInfos(dirptr, descptr);
        dirptr = dirptr.offset(
            (dirlen as libc::c_ulong)
                .wrapping_add(strlen(descptr))
                .wrapping_add(1i32 as libc::c_ulong) as isize,
        );
        i += 1
    }
    trap_Print(va(
        b"%i mods parsed\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s_mods.list.numitems,
    ));
    if s_mods.list.numitems > 64i32 {
        s_mods.list.numitems = 64i32
    };
}
/*
===============
UI_Mods_ParseInfos
===============
*/
unsafe extern "C" fn UI_Mods_ParseInfos(
    mut modDir: *mut libc::c_char,
    mut modDesc: *mut libc::c_char,
) {
    s_mods.fs_gameList[s_mods.list.numitems as usize] = s_mods.fs_gamePtr;
    Q_strncpyz(s_mods.fs_gamePtr, modDir, 16i32);
    s_mods.descriptionList[s_mods.list.numitems as usize] = s_mods.descriptionPtr;
    Q_strncpyz(s_mods.descriptionPtr, modDesc, 48i32);
    let ref mut fresh1 = *s_mods.list.itemnames.offset(s_mods.list.numitems as isize);
    *fresh1 = s_mods.descriptionPtr;
    s_mods.descriptionPtr = s_mods
        .descriptionPtr
        .offset(strlen(s_mods.descriptionPtr).wrapping_add(1i32 as libc::c_ulong) as isize);
    s_mods.fs_gamePtr = s_mods
        .fs_gamePtr
        .offset(strlen(s_mods.fs_gamePtr).wrapping_add(1i32 as libc::c_ulong) as isize);
    s_mods.list.numitems += 1;
}
/*
===============
UI_Mods_MenuEvent
===============
*/
unsafe extern "C" fn UI_Mods_MenuEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        11 => {
            trap_Cvar_Set(
                b"fs_game\x00" as *const u8 as *const libc::c_char,
                s_mods.fs_gameList[s_mods.list.curvalue as usize],
            );
            trap_Cmd_ExecuteText(
                EXEC_APPEND as libc::c_int,
                b"vid_restart;\x00" as *const u8 as *const libc::c_char,
            );
            UI_PopMenu();
        }
        10 => {
            UI_PopMenu();
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_ModsMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/load_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/load_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mods_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub list: menulist_s,
    pub back: menubitmap_s,
    pub go: menubitmap_s,
    pub description: [libc::c_char; 3072],
    pub fs_game: [libc::c_char; 1024],
    pub descriptionPtr: *mut libc::c_char,
    pub fs_gamePtr: *mut libc::c_char,
    pub descriptionList: [*mut libc::c_char; 64],
    pub fs_gameList: [*mut libc::c_char; 64],
}
