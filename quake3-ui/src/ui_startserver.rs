use bg_public_h::{
    unnamed_0, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT,
};
use libc;
use q_shared_h::{
    colorBlack, colorRed, colorWhite, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va,
    vec4_t, vec_t, COM_ParseExt, Com_Clamp, Com_sprintf, Info_ValueForKey, Q_CleanStr, Q_stricmp,
    Q_strncpyz, Q_strupr, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stddef_h::size_t;
use stdlib::{__compar_fn_t, atoi, memset, qsort, strcpy, strlen, strrchr};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menufield_s, menuframework_s, menulist_s,
    menuradiobutton_s, menutext_s, mfield_t, trap_Cmd_ExecuteText, trap_Cvar_Set,
    trap_Cvar_SetValue, trap_Cvar_VariableStringBuffer, trap_Cvar_VariableValue,
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
use ui_sound::{UI_SoundOptionsMenu, UI_SoundOptionsMenu_Cache};
use ui_sparena::UI_SPArena_Start;
use ui_specifyserver::{SpecifyServer_Cache, UI_SpecifyServerMenu};
use ui_splevel::{UI_SPLevelMenu, UI_SPLevelMenu_Cache, UI_SPLevelMenu_ReInit, UI_SPLevelMenu_f};
use ui_sppostgame::{
    ui_medalPicNames, ui_medalSounds, UI_SPPostgameMenu_Cache, UI_SPPostgameMenu_f,
};
use ui_spskill::{UI_SPSkillMenu, UI_SPSkillMenu_Cache};
use ui_team::{TeamMain_Cache, UI_TeamMainMenu};
use ui_teamorders::{UI_TeamOrdersMenu, UI_TeamOrdersMenu_f};
use ui_video::{DriverInfo_Cache, GraphicsOptions_Cache, UI_GraphicsOptionsMenu};

//
// ui_startserver.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_StartServerMenu(mut multiplayer: qboolean) {
    StartServer_MenuInit();
    s_startserver.multiplayer = multiplayer;
    UI_PushMenu(&mut s_startserver.menu);
}
static mut s_startserver: startserver_t = startserver_t {
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
    gametype: menulist_s {
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
    mappics: [menubitmap_s {
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
    }; 4],
    mapbuttons: [menubitmap_s {
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
    }; 4],
    arrows: menubitmap_s {
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
    prevpage: menubitmap_s {
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
    nextpage: menubitmap_s {
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
    next: menubitmap_s {
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
    mapname: menutext_s {
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
    item_null: menubitmap_s {
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
    multiplayer: qfalse,
    currentmap: 0,
    nummaps: 0,
    page: 0,
    maxpages: 0,
    maplist: [0; 1024],
};
/*
=================
StartServer_MenuInit
=================
*/
unsafe extern "C" fn StartServer_MenuInit() {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    static mut mapnamebuffer: [libc::c_char; 64] = [0; 64];
    memset(
        &mut s_startserver as *mut startserver_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<startserver_t>() as libc::c_ulong,
    );
    StartServer_Cache();
    s_startserver.menu.wrapAround = qtrue;
    s_startserver.menu.fullscreen = qtrue;
    s_startserver.banner.generic.type_0 = 10i32;
    s_startserver.banner.generic.x = 320i32;
    s_startserver.banner.generic.y = 16i32;
    s_startserver.banner.string =
        b"GAME SERVER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_startserver.banner.color = color_white.as_mut_ptr();
    s_startserver.banner.style = 0x1i32;
    s_startserver.framel.generic.type_0 = 6i32;
    s_startserver.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_startserver.framel.generic.flags = 0x4000i32 as libc::c_uint;
    s_startserver.framel.generic.x = 0i32;
    s_startserver.framel.generic.y = 78i32;
    s_startserver.framel.width = 256i32;
    s_startserver.framel.height = 329i32;
    s_startserver.framer.generic.type_0 = 6i32;
    s_startserver.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_startserver.framer.generic.flags = 0x4000i32 as libc::c_uint;
    s_startserver.framer.generic.x = 376i32;
    s_startserver.framer.generic.y = 76i32;
    s_startserver.framer.width = 256i32;
    s_startserver.framer.height = 334i32;
    s_startserver.gametype.generic.type_0 = 3i32;
    s_startserver.gametype.generic.name = b"Game Type:\x00" as *const u8 as *const libc::c_char;
    s_startserver.gametype.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_startserver.gametype.generic.callback = Some(StartServer_GametypeEvent);
    s_startserver.gametype.generic.id = 10i32;
    s_startserver.gametype.generic.x = 320i32 - 24i32;
    s_startserver.gametype.generic.y = 368i32;
    s_startserver.gametype.itemnames = gametype_items.as_mut_ptr();
    i = 0i32;
    while i < 4i32 {
        x = i % 2i32 * (128i32 + 8i32) + 188i32;
        y = i / 2i32 * (128i32 + 8i32) + 96i32;
        s_startserver.mappics[i as usize].generic.type_0 = 6i32;
        s_startserver.mappics[i as usize].generic.flags =
            0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
        s_startserver.mappics[i as usize].generic.x = x;
        s_startserver.mappics[i as usize].generic.y = y;
        s_startserver.mappics[i as usize].generic.id = 11i32 + i;
        s_startserver.mappics[i as usize].width = 128i32;
        s_startserver.mappics[i as usize].height = 96i32;
        s_startserver.mappics[i as usize].focuspic =
            b"menu/art/maps_selected\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        s_startserver.mappics[i as usize].errorpic =
            b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        s_startserver.mappics[i as usize].generic.ownerdraw = Some(StartServer_LevelshotDraw);
        s_startserver.mapbuttons[i as usize].generic.type_0 = 6i32;
        s_startserver.mapbuttons[i as usize].generic.flags =
            0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x8000i32 as libc::c_uint;
        s_startserver.mapbuttons[i as usize].generic.id = 11i32 + i;
        s_startserver.mapbuttons[i as usize].generic.callback = Some(StartServer_MapEvent);
        s_startserver.mapbuttons[i as usize].generic.x = x - 30i32;
        s_startserver.mapbuttons[i as usize].generic.y = y - 32i32;
        s_startserver.mapbuttons[i as usize].width = 256i32;
        s_startserver.mapbuttons[i as usize].height = 248i32;
        s_startserver.mapbuttons[i as usize].generic.left = x;
        s_startserver.mapbuttons[i as usize].generic.top = y;
        s_startserver.mapbuttons[i as usize].generic.right = x + 128i32;
        s_startserver.mapbuttons[i as usize].generic.bottom = y + 128i32;
        s_startserver.mapbuttons[i as usize].focuspic =
            b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        i += 1
    }
    s_startserver.arrows.generic.type_0 = 6i32;
    s_startserver.arrows.generic.name =
        b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char;
    s_startserver.arrows.generic.flags = 0x4000i32 as libc::c_uint;
    s_startserver.arrows.generic.x = 260i32;
    s_startserver.arrows.generic.y = 400i32;
    s_startserver.arrows.width = 128i32;
    s_startserver.arrows.height = 32i32;
    s_startserver.prevpage.generic.type_0 = 6i32;
    s_startserver.prevpage.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_startserver.prevpage.generic.callback = Some(StartServer_MenuEvent);
    s_startserver.prevpage.generic.id = 15i32;
    s_startserver.prevpage.generic.x = 260i32;
    s_startserver.prevpage.generic.y = 400i32;
    s_startserver.prevpage.width = 64i32;
    s_startserver.prevpage.height = 32i32;
    s_startserver.prevpage.focuspic =
        b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_startserver.nextpage.generic.type_0 = 6i32;
    s_startserver.nextpage.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_startserver.nextpage.generic.callback = Some(StartServer_MenuEvent);
    s_startserver.nextpage.generic.id = 16i32;
    s_startserver.nextpage.generic.x = 321i32;
    s_startserver.nextpage.generic.y = 400i32;
    s_startserver.nextpage.width = 64i32;
    s_startserver.nextpage.height = 32i32;
    s_startserver.nextpage.focuspic =
        b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_startserver.mapname.generic.type_0 = 9i32;
    s_startserver.mapname.generic.flags = 0x8i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_startserver.mapname.generic.x = 320i32;
    s_startserver.mapname.generic.y = 440i32;
    s_startserver.mapname.string = mapnamebuffer.as_mut_ptr();
    s_startserver.mapname.style = 0x1i32 | 0x20i32;
    s_startserver.mapname.color = text_color_normal.as_mut_ptr();
    s_startserver.back.generic.type_0 = 6i32;
    s_startserver.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_startserver.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_startserver.back.generic.callback = Some(StartServer_MenuEvent);
    s_startserver.back.generic.id = 17i32;
    s_startserver.back.generic.x = 0i32;
    s_startserver.back.generic.y = 480i32 - 64i32;
    s_startserver.back.width = 128i32;
    s_startserver.back.height = 64i32;
    s_startserver.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_startserver.next.generic.type_0 = 6i32;
    s_startserver.next.generic.name = b"menu/art/next_0\x00" as *const u8 as *const libc::c_char;
    s_startserver.next.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_startserver.next.generic.callback = Some(StartServer_MenuEvent);
    s_startserver.next.generic.id = 18i32;
    s_startserver.next.generic.x = 640i32;
    s_startserver.next.generic.y = 480i32 - 64i32;
    s_startserver.next.width = 128i32;
    s_startserver.next.height = 64i32;
    s_startserver.next.focuspic =
        b"menu/art/next_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_startserver.item_null.generic.type_0 = 6i32;
    s_startserver.item_null.generic.flags =
        0x4i32 as libc::c_uint | 0x800i32 as libc::c_uint | 0x100000i32 as libc::c_uint;
    s_startserver.item_null.generic.x = 0i32;
    s_startserver.item_null.generic.y = 0i32;
    s_startserver.item_null.width = 640i32;
    s_startserver.item_null.height = 480i32;
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.gametype as *mut menulist_s as *mut libc::c_void,
    );
    i = 0i32;
    while i < 4i32 {
        Menu_AddItem(
            &mut s_startserver.menu,
            &mut s_startserver.mappics[i as usize] as *mut menubitmap_s as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut s_startserver.menu,
            &mut s_startserver.mapbuttons[i as usize] as *mut menubitmap_s as *mut libc::c_void,
        );
        i += 1
    }
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.prevpage as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.nextpage as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.next as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.mapname as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.item_null as *mut menubitmap_s as *mut libc::c_void,
    );
    StartServer_GametypeEvent(0 as *mut libc::c_void, 3i32);
}
/*
=================
StartServer_GametypeEvent
=================
*/
unsafe extern "C" fn StartServer_GametypeEvent(mut _ptr: *mut libc::c_void, mut event: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut gamebits: libc::c_int = 0;
    let mut matchbits: libc::c_int = 0;
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    if event != 3i32 {
        return;
    }
    count = UI_GetNumArenas();
    s_startserver.nummaps = 0i32;
    matchbits = 1i32 << gametype_remap[s_startserver.gametype.curvalue as usize];
    if gametype_remap[s_startserver.gametype.curvalue as usize] == GT_FFA as libc::c_int {
        matchbits |= 1i32 << GT_SINGLE_PLAYER as libc::c_int
    }
    i = 0i32;
    while i < count {
        info = UI_GetArenaInfoByNumber(i);
        gamebits = GametypeBits(Info_ValueForKey(
            info,
            b"type\x00" as *const u8 as *const libc::c_char,
        ));
        if !(0 == gamebits & matchbits) {
            s_startserver.maplist[s_startserver.nummaps as usize] = i;
            s_startserver.nummaps += 1
        }
        i += 1
    }
    s_startserver.maxpages = (s_startserver.nummaps + 4i32 - 1i32) / 4i32;
    s_startserver.page = 0i32;
    s_startserver.currentmap = 0i32;
    StartServer_Update();
}
/*
=================
StartServer_Update
=================
*/
unsafe extern "C" fn StartServer_Update() {
    let mut i: libc::c_int = 0;
    let mut top: libc::c_int = 0;
    static mut picname: [[libc::c_char; 64]; 4] = [[0; 64]; 4];
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapname: [libc::c_char; 16] = [0; 16];
    top = s_startserver.page * 4i32;
    i = 0i32;
    while i < 4i32 {
        if top + i >= s_startserver.nummaps {
            break;
        }
        info = UI_GetArenaInfoByNumber(s_startserver.maplist[(top + i) as usize]);
        Q_strncpyz(
            mapname.as_mut_ptr(),
            Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
            16i32,
        );
        Q_strupr(mapname.as_mut_ptr());
        Com_sprintf(
            picname[i as usize].as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"levelshots/%s\x00" as *const u8 as *const libc::c_char,
            mapname.as_mut_ptr(),
        );
        s_startserver.mappics[i as usize].generic.flags &= !(0x40i32 as libc::c_uint);
        s_startserver.mappics[i as usize].generic.name = picname[i as usize].as_mut_ptr();
        s_startserver.mappics[i as usize].shader = 0i32;
        s_startserver.mapbuttons[i as usize].generic.flags |= 0x100i32 as libc::c_uint;
        s_startserver.mapbuttons[i as usize].generic.flags &= !(0x4000i32 as libc::c_uint);
        i += 1
    }
    while i < 4i32 {
        s_startserver.mappics[i as usize].generic.flags &= !(0x40i32 as libc::c_uint);
        s_startserver.mappics[i as usize].generic.name = 0 as *const libc::c_char;
        s_startserver.mappics[i as usize].shader = 0i32;
        s_startserver.mapbuttons[i as usize].generic.flags &= !(0x100i32 as libc::c_uint);
        s_startserver.mapbuttons[i as usize].generic.flags |= 0x4000i32 as libc::c_uint;
        i += 1
    }
    if 0 == s_startserver.nummaps {
        s_startserver.next.generic.flags |= 0x4000i32 as libc::c_uint;
        strcpy(
            s_startserver.mapname.string,
            b"NO MAPS FOUND\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        s_startserver.next.generic.flags &= !(0x4000i32 as libc::c_uint);
        i = s_startserver.currentmap - top;
        if i >= 0i32 && i < 4i32 {
            s_startserver.mappics[i as usize].generic.flags |= 0x40i32 as libc::c_uint;
            s_startserver.mapbuttons[i as usize].generic.flags &= !(0x100i32 as libc::c_uint)
        }
        info = UI_GetArenaInfoByNumber(s_startserver.maplist[s_startserver.currentmap as usize]);
        Q_strncpyz(
            s_startserver.mapname.string,
            Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
            16i32,
        );
    }
    Q_strupr(s_startserver.mapname.string);
}
/*
=================
GametypeBits
=================
*/
unsafe extern "C" fn GametypeBits(mut string: *mut libc::c_char) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    bits = 0i32;
    p = string;
    loop {
        token = COM_ParseExt(&mut p, qfalse);
        if 0 == *token.offset(0isize) {
            break;
        }
        if Q_stricmp(token, b"ffa\x00" as *const u8 as *const libc::c_char) == 0i32 {
            bits |= 1i32 << GT_FFA as libc::c_int
        } else if Q_stricmp(token, b"tourney\x00" as *const u8 as *const libc::c_char) == 0i32 {
            bits |= 1i32 << GT_TOURNAMENT as libc::c_int
        } else if Q_stricmp(token, b"single\x00" as *const u8 as *const libc::c_char) == 0i32 {
            bits |= 1i32 << GT_SINGLE_PLAYER as libc::c_int
        } else if Q_stricmp(token, b"team\x00" as *const u8 as *const libc::c_char) == 0i32 {
            bits |= 1i32 << GT_TEAM as libc::c_int
        } else {
            if !(Q_stricmp(token, b"ctf\x00" as *const u8 as *const libc::c_char) == 0i32) {
                continue;
            }
            bits |= 1i32 << GT_CTF as libc::c_int
        }
    }
    return bits;
}
static mut gametype_remap: [libc::c_int; 4] = [
    GT_FFA as libc::c_int,
    GT_TEAM as libc::c_int,
    GT_TOURNAMENT as libc::c_int,
    GT_CTF as libc::c_int,
];
/*
=================
StartServer_MenuEvent
=================
*/
unsafe extern "C" fn StartServer_MenuEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        15 => {
            if s_startserver.page > 0i32 {
                s_startserver.page -= 1;
                StartServer_Update();
            }
        }
        16 => {
            if s_startserver.page < s_startserver.maxpages - 1i32 {
                s_startserver.page += 1;
                StartServer_Update();
            }
        }
        18 => {
            trap_Cvar_SetValue(
                b"g_gameType\x00" as *const u8 as *const libc::c_char,
                gametype_remap[s_startserver.gametype.curvalue as usize] as libc::c_float,
            );
            UI_ServerOptionsMenu(s_startserver.multiplayer);
        }
        17 => {
            UI_PopMenu();
        }
        _ => {}
    };
}
unsafe extern "C" fn UI_ServerOptionsMenu(mut multiplayer: qboolean) {
    ServerOptions_MenuInit(multiplayer);
    UI_PushMenu(&mut s_serveroptions.menu);
}
static mut s_serveroptions: serveroptions_t = serveroptions_t {
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
    mappic: menubitmap_s {
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
    picframe: menubitmap_s {
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
    dedicated: menulist_s {
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
    timelimit: menufield_s {
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
    fraglimit: menufield_s {
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
    flaglimit: menufield_s {
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
    friendlyfire: menuradiobutton_s {
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
    hostname: menufield_s {
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
    pure_0: menuradiobutton_s {
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
    botSkill: menulist_s {
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
    player0: menutext_s {
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
    playerType: [menulist_s {
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
    }; 12],
    playerName: [menutext_s {
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
    }; 12],
    playerTeam: [menulist_s {
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
    }; 12],
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
    next: menubitmap_s {
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
    multiplayer: qfalse,
    gametype: 0,
    mapnamebuffer: [0; 32],
    playerNameBuffers: [[0; 16]; 12],
    newBot: qfalse,
    newBotIndex: 0,
    newBotName: [0; 16],
    punkbuster: menulist_s {
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
};
/*
=================
ServerOptions_MenuInit
=================
*/
unsafe extern "C" fn ServerOptions_MenuInit(mut multiplayer: qboolean) {
    let mut y: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    memset(
        &mut s_serveroptions as *mut serveroptions_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<serveroptions_t>() as libc::c_ulong,
    );
    s_serveroptions.multiplayer = multiplayer;
    s_serveroptions.gametype = Com_Clamp(
        0i32 as libc::c_float,
        (::std::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_float,
        trap_Cvar_VariableValue(b"g_gametype\x00" as *const u8 as *const libc::c_char),
    ) as libc::c_int;
    s_serveroptions.punkbuster.curvalue = Com_Clamp(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        trap_Cvar_VariableValue(b"sv_punkbuster\x00" as *const u8 as *const libc::c_char),
    ) as libc::c_int;
    ServerOptions_Cache();
    s_serveroptions.menu.wrapAround = qtrue;
    s_serveroptions.menu.fullscreen = qtrue;
    s_serveroptions.banner.generic.type_0 = 10i32;
    s_serveroptions.banner.generic.x = 320i32;
    s_serveroptions.banner.generic.y = 16i32;
    s_serveroptions.banner.string =
        b"GAME SERVER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serveroptions.banner.color = color_white.as_mut_ptr();
    s_serveroptions.banner.style = 0x1i32;
    s_serveroptions.mappic.generic.type_0 = 6i32;
    s_serveroptions.mappic.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_serveroptions.mappic.generic.x = 352i32;
    s_serveroptions.mappic.generic.y = 80i32;
    s_serveroptions.mappic.width = 160i32;
    s_serveroptions.mappic.height = 120i32;
    s_serveroptions.mappic.errorpic =
        b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serveroptions.mappic.generic.ownerdraw = Some(ServerOptions_LevelshotDraw);
    s_serveroptions.picframe.generic.type_0 = 6i32;
    s_serveroptions.picframe.generic.flags =
        0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint | 0x40i32 as libc::c_uint;
    s_serveroptions.picframe.generic.x = 352i32 - 38i32;
    s_serveroptions.picframe.generic.y = 80i32 - 40i32;
    s_serveroptions.picframe.width = 320i32;
    s_serveroptions.picframe.height = 320i32;
    s_serveroptions.picframe.focuspic =
        b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    y = 272i32;
    if s_serveroptions.gametype != GT_CTF as libc::c_int {
        s_serveroptions.fraglimit.generic.type_0 = 4i32;
        s_serveroptions.fraglimit.generic.name =
            b"Frag Limit:\x00" as *const u8 as *const libc::c_char;
        s_serveroptions.fraglimit.generic.flags =
            0x20i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
        s_serveroptions.fraglimit.generic.x = 456i32;
        s_serveroptions.fraglimit.generic.y = y;
        s_serveroptions.fraglimit.generic.statusbar = Some(ServerOptions_StatusBar);
        s_serveroptions.fraglimit.field.widthInChars = 3i32;
        s_serveroptions.fraglimit.field.maxchars = 3i32
    } else {
        s_serveroptions.flaglimit.generic.type_0 = 4i32;
        s_serveroptions.flaglimit.generic.name =
            b"Capture Limit:\x00" as *const u8 as *const libc::c_char;
        s_serveroptions.flaglimit.generic.flags =
            0x20i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
        s_serveroptions.flaglimit.generic.x = 456i32;
        s_serveroptions.flaglimit.generic.y = y;
        s_serveroptions.flaglimit.generic.statusbar = Some(ServerOptions_StatusBar);
        s_serveroptions.flaglimit.field.widthInChars = 3i32;
        s_serveroptions.flaglimit.field.maxchars = 3i32
    }
    y += 16i32 + 2i32;
    s_serveroptions.timelimit.generic.type_0 = 4i32;
    s_serveroptions.timelimit.generic.name = b"Time Limit:\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.timelimit.generic.flags =
        0x20i32 as libc::c_uint | 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_serveroptions.timelimit.generic.x = 456i32;
    s_serveroptions.timelimit.generic.y = y;
    s_serveroptions.timelimit.generic.statusbar = Some(ServerOptions_StatusBar);
    s_serveroptions.timelimit.field.widthInChars = 3i32;
    s_serveroptions.timelimit.field.maxchars = 3i32;
    if s_serveroptions.gametype >= GT_TEAM as libc::c_int {
        y += 16i32 + 2i32;
        s_serveroptions.friendlyfire.generic.type_0 = 5i32;
        s_serveroptions.friendlyfire.generic.flags =
            0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
        s_serveroptions.friendlyfire.generic.x = 456i32;
        s_serveroptions.friendlyfire.generic.y = y;
        s_serveroptions.friendlyfire.generic.name =
            b"Friendly Fire:\x00" as *const u8 as *const libc::c_char
    }
    y += 16i32 + 2i32;
    s_serveroptions.pure_0.generic.type_0 = 5i32;
    s_serveroptions.pure_0.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_serveroptions.pure_0.generic.x = 456i32;
    s_serveroptions.pure_0.generic.y = y;
    s_serveroptions.pure_0.generic.name = b"Pure Server:\x00" as *const u8 as *const libc::c_char;
    if 0 != s_serveroptions.multiplayer as u64 {
        y += 16i32 + 2i32;
        s_serveroptions.dedicated.generic.type_0 = 3i32;
        s_serveroptions.dedicated.generic.id = 22i32;
        s_serveroptions.dedicated.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
        s_serveroptions.dedicated.generic.callback = Some(ServerOptions_Event);
        s_serveroptions.dedicated.generic.x = 456i32;
        s_serveroptions.dedicated.generic.y = y;
        s_serveroptions.dedicated.generic.name =
            b"Dedicated:\x00" as *const u8 as *const libc::c_char;
        s_serveroptions.dedicated.itemnames = dedicated_list.as_mut_ptr()
    }
    if 0 != s_serveroptions.multiplayer as u64 {
        y += 16i32 + 2i32;
        s_serveroptions.hostname.generic.type_0 = 4i32;
        s_serveroptions.hostname.generic.name =
            b"Hostname:\x00" as *const u8 as *const libc::c_char;
        s_serveroptions.hostname.generic.flags = 0x2i32 as libc::c_uint;
        s_serveroptions.hostname.generic.x = 456i32;
        s_serveroptions.hostname.generic.y = y;
        s_serveroptions.hostname.field.widthInChars = 18i32;
        s_serveroptions.hostname.field.maxchars = 64i32
    }
    y += 16i32 + 2i32;
    s_serveroptions.punkbuster.generic.type_0 = 3i32;
    s_serveroptions.punkbuster.generic.name =
        b"Punkbuster:\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.punkbuster.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_serveroptions.punkbuster.generic.id = 0i32;
    s_serveroptions.punkbuster.generic.x = 456i32;
    s_serveroptions.punkbuster.generic.y = y;
    s_serveroptions.punkbuster.itemnames = punkbuster_items.as_mut_ptr();
    y = 80i32;
    s_serveroptions.botSkill.generic.type_0 = 3i32;
    s_serveroptions.botSkill.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    s_serveroptions.botSkill.generic.name = b"Bot Skill:\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.botSkill.generic.x = (32i32 as libc::c_ulong).wrapping_add(
        strlen(s_serveroptions.botSkill.generic.name)
            .wrapping_add(2i32 as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong),
    ) as libc::c_int;
    s_serveroptions.botSkill.generic.y = y;
    s_serveroptions.botSkill.itemnames = botSkill_list.as_mut_ptr();
    s_serveroptions.botSkill.curvalue = 1i32;
    y += 2i32 * 16i32;
    s_serveroptions.player0.generic.type_0 = 7i32;
    s_serveroptions.player0.generic.flags = 0x2i32 as libc::c_uint;
    s_serveroptions.player0.generic.x = 32i32 + 8i32;
    s_serveroptions.player0.generic.y = y;
    s_serveroptions.player0.color = color_orange.as_mut_ptr();
    s_serveroptions.player0.style = 0i32 | 0x10i32;
    n = 0i32;
    while n < 12i32 {
        s_serveroptions.playerType[n as usize].generic.type_0 = 3i32;
        s_serveroptions.playerType[n as usize].generic.flags = 0x2i32 as libc::c_uint;
        s_serveroptions.playerType[n as usize].generic.id = 20i32;
        s_serveroptions.playerType[n as usize].generic.callback = Some(ServerOptions_Event);
        s_serveroptions.playerType[n as usize].generic.x = 32i32;
        s_serveroptions.playerType[n as usize].generic.y = y;
        s_serveroptions.playerType[n as usize].itemnames = playerType_list.as_mut_ptr();
        s_serveroptions.playerName[n as usize].generic.type_0 = 7i32;
        s_serveroptions.playerName[n as usize].generic.flags = 0x2i32 as libc::c_uint;
        s_serveroptions.playerName[n as usize].generic.x = 96i32;
        s_serveroptions.playerName[n as usize].generic.y = y;
        s_serveroptions.playerName[n as usize].generic.callback =
            Some(ServerOptions_PlayerNameEvent);
        s_serveroptions.playerName[n as usize].generic.id = n;
        s_serveroptions.playerName[n as usize].generic.ownerdraw = Some(PlayerName_Draw);
        s_serveroptions.playerName[n as usize].color = color_orange.as_mut_ptr();
        s_serveroptions.playerName[n as usize].style = 0x10i32;
        s_serveroptions.playerName[n as usize].string =
            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr();
        s_serveroptions.playerName[n as usize].generic.top =
            s_serveroptions.playerName[n as usize].generic.y;
        s_serveroptions.playerName[n as usize].generic.bottom =
            s_serveroptions.playerName[n as usize].generic.y + 16i32;
        s_serveroptions.playerName[n as usize].generic.left =
            s_serveroptions.playerName[n as usize].generic.x - 16i32 / 2i32;
        s_serveroptions.playerName[n as usize].generic.right =
            s_serveroptions.playerName[n as usize].generic.x + 16i32 * 8i32;
        s_serveroptions.playerTeam[n as usize].generic.type_0 = 3i32;
        s_serveroptions.playerTeam[n as usize].generic.flags = 0x2i32 as libc::c_uint;
        s_serveroptions.playerTeam[n as usize].generic.x = 240i32;
        s_serveroptions.playerTeam[n as usize].generic.y = y;
        s_serveroptions.playerTeam[n as usize].itemnames = playerTeam_list.as_mut_ptr();
        y += 16i32 + 4i32;
        n += 1
    }
    s_serveroptions.back.generic.type_0 = 6i32;
    s_serveroptions.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_serveroptions.back.generic.callback = Some(ServerOptions_Event);
    s_serveroptions.back.generic.id = 24i32;
    s_serveroptions.back.generic.x = 0i32;
    s_serveroptions.back.generic.y = 480i32 - 64i32;
    s_serveroptions.back.width = 128i32;
    s_serveroptions.back.height = 64i32;
    s_serveroptions.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serveroptions.next.generic.type_0 = 6i32;
    s_serveroptions.next.generic.name = b"menu/art/next_0\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.next.generic.flags = 0x10i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x4000i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_serveroptions.next.generic.callback = Some(ServerOptions_Event);
    s_serveroptions.next.generic.id = 18i32;
    s_serveroptions.next.generic.x = 640i32;
    s_serveroptions.next.generic.y = 480i32 - 64i32 - 72i32;
    s_serveroptions.next.generic.statusbar = Some(ServerOptions_StatusBar);
    s_serveroptions.next.width = 128i32;
    s_serveroptions.next.height = 64i32;
    s_serveroptions.next.focuspic =
        b"menu/art/next_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_serveroptions.go.generic.type_0 = 6i32;
    s_serveroptions.go.generic.name = b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    s_serveroptions.go.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_serveroptions.go.generic.callback = Some(ServerOptions_Event);
    s_serveroptions.go.generic.id = 23i32;
    s_serveroptions.go.generic.x = 640i32;
    s_serveroptions.go.generic.y = 480i32 - 64i32;
    s_serveroptions.go.width = 128i32;
    s_serveroptions.go.height = 64i32;
    s_serveroptions.go.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.mappic as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.picframe as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.botSkill as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.player0 as *mut menutext_s as *mut libc::c_void,
    );
    n = 0i32;
    while n < 12i32 {
        if n != 0i32 {
            Menu_AddItem(
                &mut s_serveroptions.menu,
                &mut s_serveroptions.playerType[n as usize] as *mut menulist_s as *mut libc::c_void,
            );
        }
        Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.playerName[n as usize] as *mut menutext_s as *mut libc::c_void,
        );
        if s_serveroptions.gametype >= GT_TEAM as libc::c_int {
            Menu_AddItem(
                &mut s_serveroptions.menu,
                &mut s_serveroptions.playerTeam[n as usize] as *mut menulist_s as *mut libc::c_void,
            );
        }
        n += 1
    }
    if s_serveroptions.gametype != GT_CTF as libc::c_int {
        Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.fraglimit as *mut menufield_s as *mut libc::c_void,
        );
    } else {
        Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.flaglimit as *mut menufield_s as *mut libc::c_void,
        );
    }
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.timelimit as *mut menufield_s as *mut libc::c_void,
    );
    if s_serveroptions.gametype >= GT_TEAM as libc::c_int {
        Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.friendlyfire as *mut menuradiobutton_s as *mut libc::c_void,
        );
    }
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.pure_0 as *mut menuradiobutton_s as *mut libc::c_void,
    );
    if 0 != s_serveroptions.multiplayer as u64 {
        Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.dedicated as *mut menulist_s as *mut libc::c_void,
        );
    }
    if 0 != s_serveroptions.multiplayer as u64 {
        Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.hostname as *mut menufield_s as *mut libc::c_void,
        );
    }
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.next as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.go as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.punkbuster as *mut menulist_s as *mut libc::c_void,
    );
    ServerOptions_SetMenuItems();
}
/*
=================
ServerOptions_SetMenuItems
=================
*/
unsafe extern "C" fn ServerOptions_SetMenuItems() {
    static mut picname: [libc::c_char; 64] = [0; 64];
    let mut mapname: [libc::c_char; 16] = [0; 16];
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    match s_serveroptions.gametype {
        1 => {
            Com_sprintf(
                s_serveroptions.fraglimit.field.buffer.as_mut_ptr(),
                4i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0i32 as libc::c_float,
                    999i32 as libc::c_float,
                    trap_Cvar_VariableValue(
                        b"ui_tourney_fraglimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as libc::c_int,
            );
            Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0i32 as libc::c_float,
                    999i32 as libc::c_float,
                    trap_Cvar_VariableValue(
                        b"ui_tourney_timelimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as libc::c_int,
            );
        }
        3 => {
            Com_sprintf(
                s_serveroptions.fraglimit.field.buffer.as_mut_ptr(),
                4i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0i32 as libc::c_float,
                    999i32 as libc::c_float,
                    trap_Cvar_VariableValue(
                        b"ui_team_fraglimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as libc::c_int,
            );
            Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0i32 as libc::c_float,
                    999i32 as libc::c_float,
                    trap_Cvar_VariableValue(
                        b"ui_team_timelimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as libc::c_int,
            );
            s_serveroptions.friendlyfire.curvalue = Com_Clamp(
                0i32 as libc::c_float,
                1i32 as libc::c_float,
                trap_Cvar_VariableValue(
                    b"ui_team_friendly\x00" as *const u8 as *const libc::c_char,
                ),
            ) as libc::c_int
        }
        4 => {
            Com_sprintf(
                s_serveroptions.flaglimit.field.buffer.as_mut_ptr(),
                4i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0i32 as libc::c_float,
                    100i32 as libc::c_float,
                    trap_Cvar_VariableValue(
                        b"ui_ctf_capturelimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as libc::c_int,
            );
            Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0i32 as libc::c_float,
                    999i32 as libc::c_float,
                    trap_Cvar_VariableValue(
                        b"ui_ctf_timelimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as libc::c_int,
            );
            s_serveroptions.friendlyfire.curvalue = Com_Clamp(
                0i32 as libc::c_float,
                1i32 as libc::c_float,
                trap_Cvar_VariableValue(b"ui_ctf_friendly\x00" as *const u8 as *const libc::c_char),
            ) as libc::c_int
        }
        0 | _ => {
            Com_sprintf(
                s_serveroptions.fraglimit.field.buffer.as_mut_ptr(),
                4i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0i32 as libc::c_float,
                    999i32 as libc::c_float,
                    trap_Cvar_VariableValue(
                        b"ui_ffa_fraglimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as libc::c_int,
            );
            Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4i32,
                b"%i\x00" as *const u8 as *const libc::c_char,
                Com_Clamp(
                    0i32 as libc::c_float,
                    999i32 as libc::c_float,
                    trap_Cvar_VariableValue(
                        b"ui_ffa_timelimit\x00" as *const u8 as *const libc::c_char,
                    ),
                ) as libc::c_int,
            );
        }
    }
    Q_strncpyz(
        s_serveroptions.hostname.field.buffer.as_mut_ptr(),
        UI_Cvar_VariableString(b"sv_hostname\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    s_serveroptions.pure_0.curvalue = Com_Clamp(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        trap_Cvar_VariableValue(b"sv_pure\x00" as *const u8 as *const libc::c_char),
    ) as libc::c_int;
    info = UI_GetArenaInfoByNumber(s_startserver.maplist[s_startserver.currentmap as usize]);
    Q_strncpyz(
        mapname.as_mut_ptr(),
        Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
        16i32,
    );
    Q_strupr(mapname.as_mut_ptr());
    Com_sprintf(
        picname.as_mut_ptr(),
        64i32,
        b"levelshots/%s\x00" as *const u8 as *const libc::c_char,
        mapname.as_mut_ptr(),
    );
    s_serveroptions.mappic.generic.name = picname.as_mut_ptr();
    strcpy(
        s_serveroptions.mapnamebuffer.as_mut_ptr(),
        s_startserver.mapname.string,
    );
    Q_strupr(s_serveroptions.mapnamebuffer.as_mut_ptr());
    ServerOptions_InitPlayerItems();
    ServerOptions_SetPlayerItems();
    ServerOptions_InitBotNames();
    ServerOptions_SetPlayerItems();
}
/*
=================
ServerOptions_SetPlayerItems
=================
*/
unsafe extern "C" fn ServerOptions_SetPlayerItems() {
    let mut start: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if s_serveroptions.dedicated.curvalue == 0i32 {
        s_serveroptions.player0.string =
            b"Human\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        s_serveroptions.playerName[0usize].generic.flags &= !(0x1000i32 as libc::c_uint);
        start = 1i32
    } else {
        s_serveroptions.player0.string =
            b"Open\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
        start = 0i32
    }
    n = start;
    while n < 12i32 {
        if s_serveroptions.playerType[n as usize].curvalue == 1i32 {
            s_serveroptions.playerName[n as usize].generic.flags &=
                !(0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint)
        } else {
            s_serveroptions.playerName[n as usize].generic.flags |=
                0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint
        }
        n += 1
    }
    if s_serveroptions.gametype < GT_TEAM as libc::c_int {
        return;
    }
    n = start;
    while n < 12i32 {
        if s_serveroptions.playerType[n as usize].curvalue == 2i32 {
            s_serveroptions.playerTeam[n as usize].generic.flags |=
                0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint
        } else {
            s_serveroptions.playerTeam[n as usize].generic.flags &=
                !(0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint)
        }
        n += 1
    }
}
unsafe extern "C" fn ServerOptions_InitBotNames() {
    let mut count: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut arenaInfo: *const libc::c_char = 0 as *const libc::c_char;
    let mut botInfo: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bots: [libc::c_char; 1024] = [0; 1024];
    if s_serveroptions.gametype >= GT_TEAM as libc::c_int {
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[1usize].as_mut_ptr(),
            b"grunt\x00" as *const u8 as *const libc::c_char,
            16i32,
        );
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[2usize].as_mut_ptr(),
            b"major\x00" as *const u8 as *const libc::c_char,
            16i32,
        );
        if s_serveroptions.gametype == GT_TEAM as libc::c_int {
            Q_strncpyz(
                s_serveroptions.playerNameBuffers[3usize].as_mut_ptr(),
                b"visor\x00" as *const u8 as *const libc::c_char,
                16i32,
            );
        } else {
            s_serveroptions.playerType[3usize].curvalue = 2i32
        }
        s_serveroptions.playerType[4usize].curvalue = 2i32;
        s_serveroptions.playerType[5usize].curvalue = 2i32;
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[6usize].as_mut_ptr(),
            b"sarge\x00" as *const u8 as *const libc::c_char,
            16i32,
        );
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[7usize].as_mut_ptr(),
            b"grunt\x00" as *const u8 as *const libc::c_char,
            16i32,
        );
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[8usize].as_mut_ptr(),
            b"major\x00" as *const u8 as *const libc::c_char,
            16i32,
        );
        if s_serveroptions.gametype == GT_TEAM as libc::c_int {
            Q_strncpyz(
                s_serveroptions.playerNameBuffers[9usize].as_mut_ptr(),
                b"visor\x00" as *const u8 as *const libc::c_char,
                16i32,
            );
        } else {
            s_serveroptions.playerType[9usize].curvalue = 2i32
        }
        s_serveroptions.playerType[10usize].curvalue = 2i32;
        s_serveroptions.playerType[11usize].curvalue = 2i32;
        return;
    }
    count = 1i32;
    arenaInfo = UI_GetArenaInfoByMap(s_serveroptions.mapnamebuffer.as_mut_ptr());
    Q_strncpyz(
        bots.as_mut_ptr(),
        Info_ValueForKey(arenaInfo, b"bots\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    p = &mut bots[0usize] as *mut libc::c_char;
    while 0 != *p as libc::c_int && count < 12i32 {
        while 0 != *p as libc::c_int && *p as libc::c_int == ' ' as i32 {
            p = p.offset(1isize)
        }
        if 0 == *p {
            break;
        }
        bot = p;
        while 0 != *p as libc::c_int && *p as libc::c_int != ' ' as i32 {
            p = p.offset(1isize)
        }
        if 0 != *p {
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = 0i32 as libc::c_char
        }
        botInfo = UI_GetBotInfoByName(bot);
        if botInfo.is_null() {
            botInfo = UI_GetBotInfoByNumber(count)
        }
        bot = Info_ValueForKey(botInfo, b"name\x00" as *const u8 as *const libc::c_char);
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[count as usize].as_mut_ptr(),
            bot,
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        );
        count += 1
    }
    n = count;
    while n < 12i32 {
        strcpy(
            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
            b"--------\x00" as *const u8 as *const libc::c_char,
        );
        n += 1
    }
    while count < 8i32 {
        s_serveroptions.playerType[count as usize].curvalue = 0i32;
        count += 1
    }
    while count < 12i32 {
        if s_serveroptions.playerType[count as usize].curvalue == 1i32 {
            s_serveroptions.playerType[count as usize].curvalue = 2i32
        }
        count += 1
    }
}
/*
=================
ServerOptions_InitPlayerItems
=================
*/
unsafe extern "C" fn ServerOptions_InitPlayerItems() {
    let mut n: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    if 0 != s_serveroptions.multiplayer as u64 {
        v = 0i32
    } else {
        v = 1i32
    }
    n = 0i32;
    while n < 12i32 {
        s_serveroptions.playerType[n as usize].curvalue = v;
        n += 1
    }
    if 0 != s_serveroptions.multiplayer as libc::c_uint
        && s_serveroptions.gametype < GT_TEAM as libc::c_int
    {
        n = 8i32;
        while n < 12i32 {
            s_serveroptions.playerType[n as usize].curvalue = 2i32;
            n += 1
        }
    }
    if s_serveroptions.dedicated.curvalue == 0i32 {
        s_serveroptions.playerType[0usize].generic.flags |= 0x4000i32 as libc::c_uint;
        s_serveroptions.playerType[0usize].curvalue = 0i32;
        trap_Cvar_VariableStringBuffer(
            b"name\x00" as *const u8 as *const libc::c_char,
            s_serveroptions.playerNameBuffers[0usize].as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
        );
        Q_CleanStr(s_serveroptions.playerNameBuffers[0usize].as_mut_ptr());
    }
    if s_serveroptions.gametype >= GT_TEAM as libc::c_int {
        n = 0i32;
        while n < 12i32 / 2i32 {
            s_serveroptions.playerTeam[n as usize].curvalue = 0i32;
            n += 1
        }
        while n < 12i32 {
            s_serveroptions.playerTeam[n as usize].curvalue = 1i32;
            n += 1
        }
    } else {
        n = 0i32;
        while n < 12i32 {
            s_serveroptions.playerTeam[n as usize].generic.flags |=
                0x4000i32 as libc::c_uint | 0x1000i32 as libc::c_uint;
            n += 1
        }
    };
}
/*
=================
ServerOptions_Event
=================
*/
unsafe extern "C" fn ServerOptions_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    match (*(ptr as *mut menucommon_s)).id {
        20 => {
            if !(event != 3i32) {
                ServerOptions_SetPlayerItems();
            }
        }
        21 | 22 => {
            ServerOptions_SetPlayerItems();
        }
        23 => {
            if !(event != 3i32) {
                ServerOptions_Start();
            }
        }
        18 => {
            event != 3i32;
        }
        24 => {
            if !(event != 3i32) {
                UI_PopMenu();
            }
        }
        _ => {}
    };
}
/*
=================
ServerOptions_Start
=================
*/
unsafe extern "C" fn ServerOptions_Start() {
    let mut timelimit: libc::c_int = 0;
    let mut fraglimit: libc::c_int = 0;
    let mut maxclients: libc::c_int = 0;
    let mut dedicated: libc::c_int = 0;
    let mut friendlyfire: libc::c_int = 0;
    let mut flaglimit: libc::c_int = 0;
    let mut pure_0: libc::c_int = 0;
    let mut skill: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    timelimit = atoi(s_serveroptions.timelimit.field.buffer.as_mut_ptr());
    fraglimit = atoi(s_serveroptions.fraglimit.field.buffer.as_mut_ptr());
    flaglimit = atoi(s_serveroptions.flaglimit.field.buffer.as_mut_ptr());
    dedicated = s_serveroptions.dedicated.curvalue;
    friendlyfire = s_serveroptions.friendlyfire.curvalue;
    pure_0 = s_serveroptions.pure_0.curvalue;
    skill = s_serveroptions.botSkill.curvalue + 1i32;
    n = 0i32;
    maxclients = 0i32;
    while n < 12i32 {
        if !(s_serveroptions.playerType[n as usize].curvalue == 2i32) {
            if !(s_serveroptions.playerType[n as usize].curvalue == 1i32
                && s_serveroptions.playerNameBuffers[n as usize][0usize] as libc::c_int == 0i32)
            {
                maxclients += 1
            }
        }
        n += 1
    }
    match s_serveroptions.gametype {
        1 => {
            trap_Cvar_SetValue(
                b"ui_tourney_fraglimit\x00" as *const u8 as *const libc::c_char,
                fraglimit as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"ui_tourney_timelimit\x00" as *const u8 as *const libc::c_char,
                timelimit as libc::c_float,
            );
        }
        3 => {
            trap_Cvar_SetValue(
                b"ui_team_fraglimit\x00" as *const u8 as *const libc::c_char,
                fraglimit as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"ui_team_timelimit\x00" as *const u8 as *const libc::c_char,
                timelimit as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"ui_team_friendly\x00" as *const u8 as *const libc::c_char,
                friendlyfire as libc::c_float,
            );
        }
        4 => {
            trap_Cvar_SetValue(
                b"ui_ctf_capturelimit\x00" as *const u8 as *const libc::c_char,
                flaglimit as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"ui_ctf_timelimit\x00" as *const u8 as *const libc::c_char,
                timelimit as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"ui_ctf_friendly\x00" as *const u8 as *const libc::c_char,
                friendlyfire as libc::c_float,
            );
        }
        0 | _ => {
            trap_Cvar_SetValue(
                b"ui_ffa_fraglimit\x00" as *const u8 as *const libc::c_char,
                fraglimit as libc::c_float,
            );
            trap_Cvar_SetValue(
                b"ui_ffa_timelimit\x00" as *const u8 as *const libc::c_char,
                timelimit as libc::c_float,
            );
        }
    }
    trap_Cvar_SetValue(
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
        Com_Clamp(
            0i32 as libc::c_float,
            12i32 as libc::c_float,
            maxclients as libc::c_float,
        ),
    );
    trap_Cvar_SetValue(
        b"dedicated\x00" as *const u8 as *const libc::c_char,
        Com_Clamp(
            0i32 as libc::c_float,
            2i32 as libc::c_float,
            dedicated as libc::c_float,
        ),
    );
    trap_Cvar_SetValue(
        b"timelimit\x00" as *const u8 as *const libc::c_char,
        Com_Clamp(
            0i32 as libc::c_float,
            timelimit as libc::c_float,
            timelimit as libc::c_float,
        ),
    );
    trap_Cvar_SetValue(
        b"fraglimit\x00" as *const u8 as *const libc::c_char,
        Com_Clamp(
            0i32 as libc::c_float,
            fraglimit as libc::c_float,
            fraglimit as libc::c_float,
        ),
    );
    trap_Cvar_SetValue(
        b"capturelimit\x00" as *const u8 as *const libc::c_char,
        Com_Clamp(
            0i32 as libc::c_float,
            flaglimit as libc::c_float,
            flaglimit as libc::c_float,
        ),
    );
    trap_Cvar_SetValue(
        b"g_friendlyfire\x00" as *const u8 as *const libc::c_char,
        friendlyfire as libc::c_float,
    );
    trap_Cvar_SetValue(
        b"sv_pure\x00" as *const u8 as *const libc::c_char,
        pure_0 as libc::c_float,
    );
    trap_Cvar_Set(
        b"sv_hostname\x00" as *const u8 as *const libc::c_char,
        s_serveroptions.hostname.field.buffer.as_mut_ptr(),
    );
    trap_Cvar_SetValue(
        b"sv_punkbuster\x00" as *const u8 as *const libc::c_char,
        s_serveroptions.punkbuster.curvalue as libc::c_float,
    );
    info = UI_GetArenaInfoByNumber(s_startserver.maplist[s_startserver.currentmap as usize]);
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        va(
            b"wait ; wait ; map %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
        ),
    );
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        b"wait 3\n\x00" as *const u8 as *const libc::c_char,
    );
    n = 1i32;
    while n < 12i32 {
        if !(s_serveroptions.playerType[n as usize].curvalue != 1i32) {
            if !(s_serveroptions.playerNameBuffers[n as usize][0usize] as libc::c_int == 0i32) {
                if !(s_serveroptions.playerNameBuffers[n as usize][0usize] as libc::c_int
                    == '-' as i32)
                {
                    if s_serveroptions.gametype >= GT_TEAM as libc::c_int {
                        Com_sprintf(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                                as libc::c_int,
                            b"addbot %s %i %s\n\x00" as *const u8 as *const libc::c_char,
                            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
                            skill,
                            playerTeam_list
                                [s_serveroptions.playerTeam[n as usize].curvalue as usize],
                        );
                    } else {
                        Com_sprintf(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong
                                as libc::c_int,
                            b"addbot %s %i\n\x00" as *const u8 as *const libc::c_char,
                            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
                            skill,
                        );
                    }
                    trap_Cmd_ExecuteText(EXEC_APPEND as libc::c_int, buf.as_mut_ptr());
                }
            }
        }
        n += 1
    }
    if dedicated == 0i32 && s_serveroptions.gametype >= GT_TEAM as libc::c_int {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            va(
                b"wait 5; team %s\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                playerTeam_list[s_serveroptions.playerTeam[0usize].curvalue as usize],
            ),
        );
        trap_Cvar_Set(
            b"g_localTeamPref\x00" as *const u8 as *const libc::c_char,
            playerTeam_list[s_serveroptions.playerTeam[0usize].curvalue as usize],
        );
    };
}
static mut playerTeam_list: [*const libc::c_char; 3] = [
    b"Blue\x00" as *const u8 as *const libc::c_char,
    b"Red\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
ServerOptions_StatusBar
=================
*/
unsafe extern "C" fn ServerOptions_StatusBar(mut ptr: *mut libc::c_void) {
    match (*(ptr as *mut menucommon_s)).id {
        _ => {}
    }
    UI_DrawString(
        320i32,
        440i32,
        b"0 = NO LIMIT\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        colorWhite.as_mut_ptr(),
    );
}
/*
=================
PlayerName_Draw
=================
*/
unsafe extern "C" fn PlayerName_Draw(mut item: *mut libc::c_void) {
    let mut s: *mut menutext_s = 0 as *mut menutext_s;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut style: libc::c_int = 0;
    let mut focus: qboolean = qfalse;
    s = item as *mut menutext_s;
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
    UI_DrawString(x + 8i32, y, (*s).string, style | 0i32, color);
}
unsafe extern "C" fn ServerOptions_PlayerNameEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    if event != 3i32 {
        return;
    }
    n = (*(ptr as *mut menutext_s)).generic.id;
    s_serveroptions.newBotIndex = n;
    UI_BotSelectMenu(s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn UI_BotSelectMenu(mut bot: *mut libc::c_char) {
    UI_BotSelectMenu_Init(bot);
    UI_PushMenu(&mut botSelectInfo.menu);
}
static mut botSelectInfo: botSelectInfo_t = botSelectInfo_t {
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
    pics: [menubitmap_s {
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
    }; 16],
    picbuttons: [menubitmap_s {
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
    }; 16],
    picnames: [menutext_s {
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
    }; 16],
    arrows: menubitmap_s {
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
    left: menubitmap_s {
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
    right: menubitmap_s {
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
    numBots: 0,
    modelpage: 0,
    numpages: 0,
    selectedmodel: 0,
    sortedBotNums: [0; 1024],
    boticons: [[0; 64]; 16],
    botnames: [[0; 16]; 16],
};
unsafe extern "C" fn UI_BotSelectMenu_Init(mut bot: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    memset(
        &mut botSelectInfo as *mut botSelectInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<botSelectInfo_t>() as libc::c_ulong,
    );
    botSelectInfo.menu.wrapAround = qtrue;
    botSelectInfo.menu.fullscreen = qtrue;
    UI_BotSelectMenu_Cache();
    botSelectInfo.banner.generic.type_0 = 10i32;
    botSelectInfo.banner.generic.x = 320i32;
    botSelectInfo.banner.generic.y = 16i32;
    botSelectInfo.banner.string =
        b"SELECT BOT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    botSelectInfo.banner.color = color_white.as_mut_ptr();
    botSelectInfo.banner.style = 0x1i32;
    y = 80i32;
    i = 0i32;
    k = 0i32;
    while i < 4i32 {
        x = 180i32;
        j = 0i32;
        while j < 4i32 {
            botSelectInfo.pics[k as usize].generic.type_0 = 6i32;
            botSelectInfo.pics[k as usize].generic.flags =
                0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
            botSelectInfo.pics[k as usize].generic.x = x;
            botSelectInfo.pics[k as usize].generic.y = y;
            botSelectInfo.pics[k as usize].generic.name =
                botSelectInfo.boticons[k as usize].as_mut_ptr();
            botSelectInfo.pics[k as usize].width = 64i32;
            botSelectInfo.pics[k as usize].height = 64i32;
            botSelectInfo.pics[k as usize].focuspic =
                b"menu/art/opponents_selected\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            botSelectInfo.pics[k as usize].focuscolor = colorRed.as_mut_ptr();
            botSelectInfo.picbuttons[k as usize].generic.type_0 = 6i32;
            botSelectInfo.picbuttons[k as usize].generic.flags =
                0x4i32 as libc::c_uint | 0x8000i32 as libc::c_uint | 0x100i32 as libc::c_uint;
            botSelectInfo.picbuttons[k as usize].generic.callback = Some(UI_BotSelectMenu_BotEvent);
            botSelectInfo.picbuttons[k as usize].generic.id = k;
            botSelectInfo.picbuttons[k as usize].generic.x = x - 16i32;
            botSelectInfo.picbuttons[k as usize].generic.y = y - 16i32;
            botSelectInfo.picbuttons[k as usize].generic.left = x;
            botSelectInfo.picbuttons[k as usize].generic.top = y;
            botSelectInfo.picbuttons[k as usize].generic.right = x + 64i32;
            botSelectInfo.picbuttons[k as usize].generic.bottom = y + 64i32;
            botSelectInfo.picbuttons[k as usize].width = 128i32;
            botSelectInfo.picbuttons[k as usize].height = 128i32;
            botSelectInfo.picbuttons[k as usize].focuspic =
                b"menu/art/opponents_select\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            botSelectInfo.picbuttons[k as usize].focuscolor = colorRed.as_mut_ptr();
            botSelectInfo.picnames[k as usize].generic.type_0 = 7i32;
            botSelectInfo.picnames[k as usize].generic.flags = 0x2i32 as libc::c_uint;
            botSelectInfo.picnames[k as usize].generic.x = x + 32i32;
            botSelectInfo.picnames[k as usize].generic.y = y + 64i32;
            botSelectInfo.picnames[k as usize].string =
                botSelectInfo.botnames[k as usize].as_mut_ptr();
            botSelectInfo.picnames[k as usize].color = color_orange.as_mut_ptr();
            botSelectInfo.picnames[k as usize].style = 0x1i32 | 0x10i32;
            x += 64i32 + 6i32;
            j += 1;
            k += 1
        }
        y += 64i32 + 16i32 + 6i32;
        i += 1
    }
    botSelectInfo.arrows.generic.type_0 = 6i32;
    botSelectInfo.arrows.generic.name =
        b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char;
    botSelectInfo.arrows.generic.flags = 0x4000i32 as libc::c_uint;
    botSelectInfo.arrows.generic.x = 260i32;
    botSelectInfo.arrows.generic.y = 440i32;
    botSelectInfo.arrows.width = 128i32;
    botSelectInfo.arrows.height = 32i32;
    botSelectInfo.left.generic.type_0 = 6i32;
    botSelectInfo.left.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    botSelectInfo.left.generic.callback = Some(UI_BotSelectMenu_LeftEvent);
    botSelectInfo.left.generic.x = 260i32;
    botSelectInfo.left.generic.y = 440i32;
    botSelectInfo.left.width = 64i32;
    botSelectInfo.left.height = 32i32;
    botSelectInfo.left.focuspic =
        b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    botSelectInfo.right.generic.type_0 = 6i32;
    botSelectInfo.right.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    botSelectInfo.right.generic.callback = Some(UI_BotSelectMenu_RightEvent);
    botSelectInfo.right.generic.x = 321i32;
    botSelectInfo.right.generic.y = 440i32;
    botSelectInfo.right.width = 64i32;
    botSelectInfo.right.height = 32i32;
    botSelectInfo.right.focuspic =
        b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    botSelectInfo.back.generic.type_0 = 6i32;
    botSelectInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    botSelectInfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    botSelectInfo.back.generic.callback = Some(UI_BotSelectMenu_BackEvent);
    botSelectInfo.back.generic.x = 0i32;
    botSelectInfo.back.generic.y = 480i32 - 64i32;
    botSelectInfo.back.width = 128i32;
    botSelectInfo.back.height = 64i32;
    botSelectInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    botSelectInfo.go.generic.type_0 = 6i32;
    botSelectInfo.go.generic.name = b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char;
    botSelectInfo.go.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    botSelectInfo.go.generic.callback = Some(UI_BotSelectMenu_SelectEvent);
    botSelectInfo.go.generic.x = 640i32;
    botSelectInfo.go.generic.y = 480i32 - 64i32;
    botSelectInfo.go.width = 128i32;
    botSelectInfo.go.height = 64i32;
    botSelectInfo.go.focuspic =
        b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    i = 0i32;
    while i < 4i32 * 4i32 {
        Menu_AddItem(
            &mut botSelectInfo.menu,
            &mut botSelectInfo.pics[i as usize] as *mut menubitmap_s as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut botSelectInfo.menu,
            &mut botSelectInfo.picbuttons[i as usize] as *mut menubitmap_s as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut botSelectInfo.menu,
            &mut botSelectInfo.picnames[i as usize] as *mut menutext_s as *mut libc::c_void,
        );
        i += 1
    }
    Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.left as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.right as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.go as *mut menubitmap_s as *mut libc::c_void,
    );
    UI_BotSelectMenu_BuildList();
    UI_BotSelectMenu_Default(bot);
    botSelectInfo.modelpage = botSelectInfo.selectedmodel / (4i32 * 4i32);
    UI_BotSelectMenu_UpdateGrid();
}
/*
=================
UI_BotSelectMenu_UpdateGrid
=================
*/
unsafe extern "C" fn UI_BotSelectMenu_UpdateGrid() {
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = botSelectInfo.modelpage * (4i32 * 4i32);
    i = 0i32;
    while i < 4i32 * 4i32 {
        if j < botSelectInfo.numBots {
            info = UI_GetBotInfoByNumber(botSelectInfo.sortedBotNums[j as usize]);
            ServerPlayerIcon(
                Info_ValueForKey(info, b"model\x00" as *const u8 as *const libc::c_char),
                botSelectInfo.boticons[i as usize].as_mut_ptr(),
                64i32,
            );
            Q_strncpyz(
                botSelectInfo.botnames[i as usize].as_mut_ptr(),
                Info_ValueForKey(info, b"name\x00" as *const u8 as *const libc::c_char),
                16i32,
            );
            Q_CleanStr(botSelectInfo.botnames[i as usize].as_mut_ptr());
            botSelectInfo.pics[i as usize].generic.name =
                botSelectInfo.boticons[i as usize].as_mut_ptr();
            if 0 != BotAlreadySelected(botSelectInfo.botnames[i as usize].as_mut_ptr()) as u64 {
                botSelectInfo.picnames[i as usize].color = color_red.as_mut_ptr()
            } else {
                botSelectInfo.picnames[i as usize].color = color_orange.as_mut_ptr()
            }
            botSelectInfo.picbuttons[i as usize].generic.flags &= !(0x4000i32 as libc::c_uint)
        } else {
            botSelectInfo.pics[i as usize].generic.name = 0 as *const libc::c_char;
            botSelectInfo.picbuttons[i as usize].generic.flags |= 0x4000i32 as libc::c_uint;
            botSelectInfo.botnames[i as usize][0usize] = 0i32 as libc::c_char
        }
        botSelectInfo.pics[i as usize].generic.flags &= !(0x40i32 as libc::c_uint);
        botSelectInfo.pics[i as usize].shader = 0i32;
        botSelectInfo.picbuttons[i as usize].generic.flags |= 0x100i32 as libc::c_uint;
        i += 1;
        j += 1
    }
    i = botSelectInfo.selectedmodel % (4i32 * 4i32);
    botSelectInfo.pics[i as usize].generic.flags |= 0x40i32 as libc::c_uint;
    botSelectInfo.picbuttons[i as usize].generic.flags &= !(0x100i32 as libc::c_uint);
    if botSelectInfo.numpages > 1i32 {
        if botSelectInfo.modelpage > 0i32 {
            botSelectInfo.left.generic.flags &= !(0x4000i32 as libc::c_uint)
        } else {
            botSelectInfo.left.generic.flags |= 0x4000i32 as libc::c_uint
        }
        if botSelectInfo.modelpage < botSelectInfo.numpages - 1i32 {
            botSelectInfo.right.generic.flags &= !(0x4000i32 as libc::c_uint)
        } else {
            botSelectInfo.right.generic.flags |= 0x4000i32 as libc::c_uint
        }
    } else {
        botSelectInfo.left.generic.flags |= 0x4000i32 as libc::c_uint;
        botSelectInfo.right.generic.flags |= 0x4000i32 as libc::c_uint
    };
}
/*
=================
BotAlreadySelected
=================
*/
unsafe extern "C" fn BotAlreadySelected(mut checkName: *const libc::c_char) -> qboolean {
    let mut n: libc::c_int = 0;
    n = 1i32;
    while n < 12i32 {
        if !(s_serveroptions.playerType[n as usize].curvalue != 1i32) {
            if !(s_serveroptions.gametype >= GT_TEAM as libc::c_int
                && s_serveroptions.playerTeam[n as usize].curvalue
                    != s_serveroptions.playerTeam[s_serveroptions.newBotIndex as usize].curvalue)
            {
                if Q_stricmp(
                    checkName,
                    s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
                ) == 0i32
                {
                    return qtrue;
                }
            }
        }
        n += 1
    }
    return qfalse;
}
/*
=================
ServerPlayerIcon
=================
*/
unsafe extern "C" fn ServerPlayerIcon(
    mut modelAndSkin: *const libc::c_char,
    mut iconName: *mut libc::c_char,
    mut iconNameMaxSize: libc::c_int,
) {
    let mut skin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut model: [libc::c_char; 64] = [0; 64];
    Q_strncpyz(
        model.as_mut_ptr(),
        modelAndSkin,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    skin = strrchr(model.as_mut_ptr(), '/' as i32);
    if !skin.is_null() {
        let fresh1 = skin;
        skin = skin.offset(1);
        *fresh1 = '\u{0}' as i32 as libc::c_char
    } else {
        skin = b"default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    Com_sprintf(
        iconName,
        iconNameMaxSize,
        b"models/players/%s/icon_%s.tga\x00" as *const u8 as *const libc::c_char,
        model.as_mut_ptr(),
        skin,
    );
    if 0 == trap_R_RegisterShaderNoMip(iconName)
        && Q_stricmp(skin, b"default\x00" as *const u8 as *const libc::c_char) != 0i32
    {
        Com_sprintf(
            iconName,
            iconNameMaxSize,
            b"models/players/%s/icon_default.tga\x00" as *const u8 as *const libc::c_char,
            model.as_mut_ptr(),
        );
    };
}
/*
=================
UI_BotSelectMenu_Default
=================
*/
unsafe extern "C" fn UI_BotSelectMenu_Default(mut bot: *mut libc::c_char) {
    let mut botInfo: *const libc::c_char = 0 as *const libc::c_char;
    let mut test: *const libc::c_char = 0 as *const libc::c_char;
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    n = 0i32;
    while n < botSelectInfo.numBots {
        botInfo = UI_GetBotInfoByNumber(n);
        test = Info_ValueForKey(botInfo, b"name\x00" as *const u8 as *const libc::c_char);
        if Q_stricmp(bot, test) == 0i32 {
            break;
        }
        n += 1
    }
    if n == botSelectInfo.numBots {
        botSelectInfo.selectedmodel = 0i32;
        return;
    }
    i = 0i32;
    while i < botSelectInfo.numBots {
        if botSelectInfo.sortedBotNums[i as usize] == n {
            break;
        }
        i += 1
    }
    if i == botSelectInfo.numBots {
        botSelectInfo.selectedmodel = 0i32;
        return;
    }
    botSelectInfo.selectedmodel = i;
}
/*
=================
UI_BotSelectMenu_BuildList
=================
*/
unsafe extern "C" fn UI_BotSelectMenu_BuildList() {
    let mut n: libc::c_int = 0;
    botSelectInfo.modelpage = 0i32;
    botSelectInfo.numBots = UI_GetNumBots();
    botSelectInfo.numpages = botSelectInfo.numBots / (4i32 * 4i32);
    if 0 != botSelectInfo.numBots % (4i32 * 4i32) {
        botSelectInfo.numpages += 1
    }
    n = 0i32;
    while n < botSelectInfo.numBots {
        botSelectInfo.sortedBotNums[n as usize] = n;
        n += 1
    }
    qsort(
        botSelectInfo.sortedBotNums.as_mut_ptr() as *mut libc::c_void,
        botSelectInfo.numBots as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(UI_BotSelectMenu_SortCompare),
    );
}
/*
=================
UI_BotSelectMenu_SortCompare
=================
*/
unsafe extern "C" fn UI_BotSelectMenu_SortCompare(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> libc::c_int {
    let mut num1: libc::c_int = 0;
    let mut num2: libc::c_int = 0;
    let mut info1: *const libc::c_char = 0 as *const libc::c_char;
    let mut info2: *const libc::c_char = 0 as *const libc::c_char;
    let mut name1: *const libc::c_char = 0 as *const libc::c_char;
    let mut name2: *const libc::c_char = 0 as *const libc::c_char;
    num1 = *(arg1 as *mut libc::c_int);
    num2 = *(arg2 as *mut libc::c_int);
    info1 = UI_GetBotInfoByNumber(num1);
    info2 = UI_GetBotInfoByNumber(num2);
    name1 = Info_ValueForKey(info1, b"name\x00" as *const u8 as *const libc::c_char);
    name2 = Info_ValueForKey(info2, b"name\x00" as *const u8 as *const libc::c_char);
    return Q_stricmp(name1, name2);
}
/*
=================
UI_BotSelectMenu_SelectEvent
=================
*/
unsafe extern "C" fn UI_BotSelectMenu_SelectEvent(
    mut _ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    UI_PopMenu();
    s_serveroptions.newBot = qtrue;
    Q_strncpyz(
        s_serveroptions.newBotName.as_mut_ptr(),
        botSelectInfo.botnames[(botSelectInfo.selectedmodel % (4i32 * 4i32)) as usize].as_mut_ptr(),
        16i32,
    );
}
/*
=================
UI_BotSelectMenu_BackEvent
=================
*/
unsafe extern "C" fn UI_BotSelectMenu_BackEvent(
    mut _ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    UI_PopMenu();
}
/*
=================
UI_BotSelectMenu_RightEvent
=================
*/
unsafe extern "C" fn UI_BotSelectMenu_RightEvent(
    mut _ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    if botSelectInfo.modelpage < botSelectInfo.numpages - 1i32 {
        botSelectInfo.modelpage += 1;
        botSelectInfo.selectedmodel = botSelectInfo.modelpage * (4i32 * 4i32);
        UI_BotSelectMenu_UpdateGrid();
    };
}
/*
=================
UI_BotSelectMenu_LeftEvent
=================
*/
unsafe extern "C" fn UI_BotSelectMenu_LeftEvent(
    mut _ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    if botSelectInfo.modelpage > 0i32 {
        botSelectInfo.modelpage -= 1;
        botSelectInfo.selectedmodel = botSelectInfo.modelpage * (4i32 * 4i32);
        UI_BotSelectMenu_UpdateGrid();
    };
}
/*
=================
UI_BotSelectMenu_BotEvent
=================
*/
unsafe extern "C" fn UI_BotSelectMenu_BotEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    let mut i: libc::c_int = 0;
    if event != 3i32 {
        return;
    }
    i = 0i32;
    while i < 4i32 * 4i32 {
        botSelectInfo.pics[i as usize].generic.flags &= !(0x40i32 as libc::c_uint);
        botSelectInfo.picbuttons[i as usize].generic.flags |= 0x100i32 as libc::c_uint;
        i += 1
    }
    i = (*(ptr as *mut menucommon_s)).id;
    botSelectInfo.pics[i as usize].generic.flags |= 0x40i32 as libc::c_uint;
    botSelectInfo.picbuttons[i as usize].generic.flags &= !(0x100i32 as libc::c_uint);
    botSelectInfo.selectedmodel = botSelectInfo.modelpage * (4i32 * 4i32) + i;
}
#[no_mangle]
pub unsafe extern "C" fn UI_BotSelectMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(
        b"menu/art/opponents_select\x00" as *const u8 as *const libc::c_char,
    );
    trap_R_RegisterShaderNoMip(
        b"menu/art/opponents_selected\x00" as *const u8 as *const libc::c_char,
    );
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char);
}
static mut playerType_list: [*const libc::c_char; 4] = [
    b"Open\x00" as *const u8 as *const libc::c_char,
    b"Bot\x00" as *const u8 as *const libc::c_char,
    b"----\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut botSkill_list: [*const libc::c_char; 6] = [
    b"I Can Win\x00" as *const u8 as *const libc::c_char,
    b"Bring It On\x00" as *const u8 as *const libc::c_char,
    b"Hurt Me Plenty\x00" as *const u8 as *const libc::c_char,
    b"Hardcore\x00" as *const u8 as *const libc::c_char,
    b"Nightmare!\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut dedicated_list: [*const libc::c_char; 4] = [
    b"No\x00" as *const u8 as *const libc::c_char,
    b"LAN\x00" as *const u8 as *const libc::c_char,
    b"Internet\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
===============
ServerOptions_LevelshotDraw
===============
*/
unsafe extern "C" fn ServerOptions_LevelshotDraw(mut self_0: *mut libc::c_void) {
    let mut b: *mut menubitmap_s = 0 as *mut menubitmap_s;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    if 0 != s_serveroptions.newBot as u64 {
        Q_strncpyz(
            s_serveroptions.playerNameBuffers[s_serveroptions.newBotIndex as usize].as_mut_ptr(),
            s_serveroptions.newBotName.as_mut_ptr(),
            16i32,
        );
        s_serveroptions.newBot = qfalse
    }
    b = self_0 as *mut menubitmap_s;
    Bitmap_Draw(b);
    x = (*b).generic.x;
    y = (*b).generic.y + (*b).height;
    UI_FillRect(
        x as libc::c_float,
        y as libc::c_float,
        (*b).width as libc::c_float,
        40i32 as libc::c_float,
        colorBlack.as_mut_ptr(),
    );
    x += (*b).width / 2i32;
    y += 4i32;
    UI_DrawString(
        x,
        y,
        s_serveroptions.mapnamebuffer.as_mut_ptr(),
        0x1i32 | 0x10i32,
        color_orange.as_mut_ptr(),
    );
    y += 16i32;
    UI_DrawString(
        x,
        y,
        gametype_items[gametype_remap2[s_serveroptions.gametype as usize] as usize],
        0x1i32 | 0x10i32,
        color_orange.as_mut_ptr(),
    );
}
static mut gametype_remap2: [libc::c_int; 5] = [0i32, 2i32, 0i32, 1i32, 3i32];
static mut gametype_items: [*const libc::c_char; 5] = [
    b"Free For All\x00" as *const u8 as *const libc::c_char,
    b"Team Deathmatch\x00" as *const u8 as *const libc::c_char,
    b"Tournament\x00" as *const u8 as *const libc::c_char,
    b"Capture the Flag\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn ServerOptions_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char);
}
/*
=================
StartServer_MapEvent
=================
*/
unsafe extern "C" fn StartServer_MapEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    s_startserver.currentmap =
        s_startserver.page * 4i32 + ((*(ptr as *mut menucommon_s)).id - 11i32);
    StartServer_Update();
}
/*
===============
StartServer_LevelshotDraw
===============
*/
unsafe extern "C" fn StartServer_LevelshotDraw(mut self_0: *mut libc::c_void) {
    let mut b: *mut menubitmap_s = 0 as *mut menubitmap_s;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut mapname: [libc::c_char; 16] = [0; 16];
    b = self_0 as *mut menubitmap_s;
    if (*b).generic.name.is_null() {
        return;
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
    x = (*b).generic.x;
    y = (*b).generic.y;
    w = (*b).width;
    h = (*b).height;
    if 0 != (*b).shader {
        UI_DrawHandlePic(
            x as libc::c_float,
            y as libc::c_float,
            w as libc::c_float,
            h as libc::c_float,
            (*b).shader,
        );
    }
    x = (*b).generic.x;
    y = (*b).generic.y + (*b).height;
    UI_FillRect(
        x as libc::c_float,
        y as libc::c_float,
        (*b).width as libc::c_float,
        28i32 as libc::c_float,
        colorBlack.as_mut_ptr(),
    );
    x += (*b).width / 2i32;
    y += 4i32;
    n = s_startserver.page * 4i32 + (*b).generic.id - 11i32;
    info = UI_GetArenaInfoByNumber(s_startserver.maplist[n as usize]);
    Q_strncpyz(
        mapname.as_mut_ptr(),
        Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
        16i32,
    );
    Q_strupr(mapname.as_mut_ptr());
    UI_DrawString(
        x,
        y,
        mapname.as_mut_ptr(),
        0x1i32 | 0x10i32,
        color_orange.as_mut_ptr(),
    );
    x = (*b).generic.x;
    y = (*b).generic.y;
    w = (*b).width;
    h = (*b).height + 28i32;
    if 0 != (*b).generic.flags & 0x40i32 as libc::c_uint {
        UI_DrawHandlePic(
            x as libc::c_float,
            y as libc::c_float,
            w as libc::c_float,
            h as libc::c_float,
            (*b).focusshader,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn StartServer_Cache() {
    let mut i: libc::c_int = 0;
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    let mut precache: qboolean = qfalse;
    let mut picname: [libc::c_char; 64] = [0; 64];
    let mut mapname: [libc::c_char; 16] = [0; 16];
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/next_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/next_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/maps_selected\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char);
    precache = trap_Cvar_VariableValue(b"com_buildscript\x00" as *const u8 as *const libc::c_char)
        as qboolean;
    if 0 != precache as u64 {
        i = 0i32;
        while i < UI_GetNumArenas() {
            info = UI_GetArenaInfoByNumber(i);
            Q_strncpyz(
                mapname.as_mut_ptr(),
                Info_ValueForKey(info, b"map\x00" as *const u8 as *const libc::c_char),
                16i32,
            );
            Q_strupr(mapname.as_mut_ptr());
            Com_sprintf(
                picname.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                b"levelshots/%s\x00" as *const u8 as *const libc::c_char,
                mapname.as_mut_ptr(),
            );
            trap_R_RegisterShaderNoMip(picname.as_mut_ptr());
            i += 1
        }
    };
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serveroptions_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub mappic: menubitmap_s,
    pub picframe: menubitmap_s,
    pub dedicated: menulist_s,
    pub timelimit: menufield_s,
    pub fraglimit: menufield_s,
    pub flaglimit: menufield_s,
    pub friendlyfire: menuradiobutton_s,
    pub hostname: menufield_s,
    pub pure_0: menuradiobutton_s,
    pub botSkill: menulist_s,
    pub player0: menutext_s,
    pub playerType: [menulist_s; 12],
    pub playerName: [menutext_s; 12],
    pub playerTeam: [menulist_s; 12],
    pub go: menubitmap_s,
    pub next: menubitmap_s,
    pub back: menubitmap_s,
    pub multiplayer: qboolean,
    pub gametype: libc::c_int,
    pub mapnamebuffer: [libc::c_char; 32],
    pub playerNameBuffers: [[libc::c_char; 16]; 12],
    pub newBot: qboolean,
    pub newBotIndex: libc::c_int,
    pub newBotName: [libc::c_char; 16],
    pub punkbuster: menulist_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct startserver_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub gametype: menulist_s,
    pub mappics: [menubitmap_s; 4],
    pub mapbuttons: [menubitmap_s; 4],
    pub arrows: menubitmap_s,
    pub prevpage: menubitmap_s,
    pub nextpage: menubitmap_s,
    pub back: menubitmap_s,
    pub next: menubitmap_s,
    pub mapname: menutext_s,
    pub item_null: menubitmap_s,
    pub multiplayer: qboolean,
    pub currentmap: libc::c_int,
    pub nummaps: libc::c_int,
    pub page: libc::c_int,
    pub maxpages: libc::c_int,
    pub maplist: [libc::c_int; 1024],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct botSelectInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub pics: [menubitmap_s; 16],
    pub picbuttons: [menubitmap_s; 16],
    pub picnames: [menutext_s; 16],
    pub arrows: menubitmap_s,
    pub left: menubitmap_s,
    pub right: menubitmap_s,
    pub go: menubitmap_s,
    pub back: menubitmap_s,
    pub numBots: libc::c_int,
    pub modelpage: libc::c_int,
    pub numpages: libc::c_int,
    pub selectedmodel: libc::c_int,
    pub sortedBotNums: [libc::c_int; 1024],
    pub boticons: [[libc::c_char; 64]; 16],
    pub botnames: [[libc::c_char; 16]; 16],
}
