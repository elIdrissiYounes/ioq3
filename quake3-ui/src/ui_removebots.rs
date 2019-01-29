use libc;
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t, Info_ValueForKey,
    Q_CleanStr, Q_strncpyz, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::{atoi, memset};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menutext_s,
    trap_Cmd_ExecuteText, trap_GetConfigString, trap_R_RegisterShaderNoMip,
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
// ui_removebots.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_RemoveBots_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/delete_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/delete_1\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_RemoveBotsMenu() {
    UI_RemoveBotsMenu_Init();
    UI_PushMenu(&mut removeBotsMenuInfo.menu);
}
static mut removeBotsMenuInfo: removeBotsMenuInfo_t = removeBotsMenuInfo_t {
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
    up: menubitmap_s {
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
    down: menubitmap_s {
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
    bots: [menutext_s {
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
    }; 7],
    delete: menubitmap_s {
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
    baseBotNum: 0,
    selectedBotNum: 0,
    botnames: [[0; 32]; 7],
    botClientNums: [0; 1024],
};
/*
=================
UI_RemoveBotsMenu_Init
=================
*/
unsafe extern "C" fn UI_RemoveBotsMenu_Init() {
    let mut n: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    memset(
        &mut removeBotsMenuInfo as *mut removeBotsMenuInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<removeBotsMenuInfo_t>() as libc::c_ulong,
    );
    removeBotsMenuInfo.menu.fullscreen = qfalse;
    removeBotsMenuInfo.menu.wrapAround = qtrue;
    UI_RemoveBots_Cache();
    UI_RemoveBotsMenu_GetBots();
    UI_RemoveBotsMenu_SetBotNames();
    count = if removeBotsMenuInfo.numBots < 7i32 {
        removeBotsMenuInfo.numBots
    } else {
        7i32
    };
    removeBotsMenuInfo.banner.generic.type_0 = 10i32;
    removeBotsMenuInfo.banner.generic.x = 320i32;
    removeBotsMenuInfo.banner.generic.y = 16i32;
    removeBotsMenuInfo.banner.string =
        b"REMOVE BOTS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    removeBotsMenuInfo.banner.color = color_white.as_mut_ptr();
    removeBotsMenuInfo.banner.style = 0x1i32;
    removeBotsMenuInfo.background.generic.type_0 = 6i32;
    removeBotsMenuInfo.background.generic.name =
        b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char;
    removeBotsMenuInfo.background.generic.flags = 0x4000i32 as libc::c_uint;
    removeBotsMenuInfo.background.generic.x = 320i32 - 233i32;
    removeBotsMenuInfo.background.generic.y = 240i32 - 166i32;
    removeBotsMenuInfo.background.width = 466i32;
    removeBotsMenuInfo.background.height = 332i32;
    removeBotsMenuInfo.arrows.generic.type_0 = 6i32;
    removeBotsMenuInfo.arrows.generic.name =
        b"menu/art/arrows_vert_0\x00" as *const u8 as *const libc::c_char;
    removeBotsMenuInfo.arrows.generic.flags = 0x4000i32 as libc::c_uint;
    removeBotsMenuInfo.arrows.generic.x = 200i32;
    removeBotsMenuInfo.arrows.generic.y = 128i32;
    removeBotsMenuInfo.arrows.width = 64i32;
    removeBotsMenuInfo.arrows.height = 128i32;
    removeBotsMenuInfo.up.generic.type_0 = 6i32;
    removeBotsMenuInfo.up.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    removeBotsMenuInfo.up.generic.x = 200i32;
    removeBotsMenuInfo.up.generic.y = 128i32;
    removeBotsMenuInfo.up.generic.id = 10i32;
    removeBotsMenuInfo.up.generic.callback = Some(UI_RemoveBotsMenu_UpEvent);
    removeBotsMenuInfo.up.width = 64i32;
    removeBotsMenuInfo.up.height = 64i32;
    removeBotsMenuInfo.up.focuspic =
        b"menu/art/arrows_vert_top\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    removeBotsMenuInfo.down.generic.type_0 = 6i32;
    removeBotsMenuInfo.down.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    removeBotsMenuInfo.down.generic.x = 200i32;
    removeBotsMenuInfo.down.generic.y = 128i32 + 64i32;
    removeBotsMenuInfo.down.generic.id = 11i32;
    removeBotsMenuInfo.down.generic.callback = Some(UI_RemoveBotsMenu_DownEvent);
    removeBotsMenuInfo.down.width = 64i32;
    removeBotsMenuInfo.down.height = 64i32;
    removeBotsMenuInfo.down.focuspic =
        b"menu/art/arrows_vert_bot\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    n = 0i32;
    y = 120i32;
    while n < count {
        removeBotsMenuInfo.bots[n as usize].generic.type_0 = 9i32;
        removeBotsMenuInfo.bots[n as usize].generic.flags =
            0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
        removeBotsMenuInfo.bots[n as usize].generic.id = 20i32 + n;
        removeBotsMenuInfo.bots[n as usize].generic.x = 320i32 - 56i32;
        removeBotsMenuInfo.bots[n as usize].generic.y = y;
        removeBotsMenuInfo.bots[n as usize].generic.callback = Some(UI_RemoveBotsMenu_BotEvent);
        removeBotsMenuInfo.bots[n as usize].string =
            removeBotsMenuInfo.botnames[n as usize].as_mut_ptr();
        removeBotsMenuInfo.bots[n as usize].color = color_orange.as_mut_ptr();
        removeBotsMenuInfo.bots[n as usize].style = 0i32 | 0x10i32;
        n += 1;
        y += 20i32
    }
    removeBotsMenuInfo.delete.generic.type_0 = 6i32;
    removeBotsMenuInfo.delete.generic.name =
        b"menu/art/delete_0\x00" as *const u8 as *const libc::c_char;
    removeBotsMenuInfo.delete.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    removeBotsMenuInfo.delete.generic.id = 12i32;
    removeBotsMenuInfo.delete.generic.callback = Some(UI_RemoveBotsMenu_DeleteEvent);
    removeBotsMenuInfo.delete.generic.x = 320i32 + 128i32 - 128i32;
    removeBotsMenuInfo.delete.generic.y = 256i32 + 128i32 - 64i32;
    removeBotsMenuInfo.delete.width = 128i32;
    removeBotsMenuInfo.delete.height = 64i32;
    removeBotsMenuInfo.delete.focuspic =
        b"menu/art/delete_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    removeBotsMenuInfo.back.generic.type_0 = 6i32;
    removeBotsMenuInfo.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    removeBotsMenuInfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    removeBotsMenuInfo.back.generic.id = 13i32;
    removeBotsMenuInfo.back.generic.callback = Some(UI_RemoveBotsMenu_BackEvent);
    removeBotsMenuInfo.back.generic.x = 320i32 - 128i32;
    removeBotsMenuInfo.back.generic.y = 256i32 + 128i32 - 64i32;
    removeBotsMenuInfo.back.width = 128i32;
    removeBotsMenuInfo.back.height = 64i32;
    removeBotsMenuInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut removeBotsMenuInfo.menu,
        &mut removeBotsMenuInfo.background as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut removeBotsMenuInfo.menu,
        &mut removeBotsMenuInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut removeBotsMenuInfo.menu,
        &mut removeBotsMenuInfo.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut removeBotsMenuInfo.menu,
        &mut removeBotsMenuInfo.up as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut removeBotsMenuInfo.menu,
        &mut removeBotsMenuInfo.down as *mut menubitmap_s as *mut libc::c_void,
    );
    n = 0i32;
    while n < count {
        Menu_AddItem(
            &mut removeBotsMenuInfo.menu,
            &mut removeBotsMenuInfo.bots[n as usize] as *mut menutext_s as *mut libc::c_void,
        );
        n += 1
    }
    Menu_AddItem(
        &mut removeBotsMenuInfo.menu,
        &mut removeBotsMenuInfo.delete as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut removeBotsMenuInfo.menu,
        &mut removeBotsMenuInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
    removeBotsMenuInfo.baseBotNum = 0i32;
    removeBotsMenuInfo.selectedBotNum = 0i32;
    removeBotsMenuInfo.bots[0usize].color = color_white.as_mut_ptr();
}
/*
=================
UI_RemoveAddBotsMenu_BackEvent
=================
*/
unsafe extern "C" fn UI_RemoveBotsMenu_BackEvent(
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
UI_RemoveBotsMenu_DeleteEvent
=================
*/
unsafe extern "C" fn UI_RemoveBotsMenu_DeleteEvent(
    mut _ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        va(
            b"clientkick %i\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            removeBotsMenuInfo.botClientNums
                [(removeBotsMenuInfo.baseBotNum + removeBotsMenuInfo.selectedBotNum) as usize],
        ),
    );
}
/*
=================
UI_RemoveBotsMenu_BotEvent
=================
*/
unsafe extern "C" fn UI_RemoveBotsMenu_BotEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    removeBotsMenuInfo.bots[removeBotsMenuInfo.selectedBotNum as usize].color =
        color_orange.as_mut_ptr();
    removeBotsMenuInfo.selectedBotNum = (*(ptr as *mut menucommon_s)).id - 20i32;
    removeBotsMenuInfo.bots[removeBotsMenuInfo.selectedBotNum as usize].color =
        color_white.as_mut_ptr();
}
/*
=================
UI_RemoveBotsMenu_DownEvent
=================
*/
unsafe extern "C" fn UI_RemoveBotsMenu_DownEvent(
    mut _ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    if removeBotsMenuInfo.baseBotNum + 7i32 < removeBotsMenuInfo.numBots {
        removeBotsMenuInfo.baseBotNum += 1;
        UI_RemoveBotsMenu_SetBotNames();
    };
}
/*
=================
UI_RemoveBotsMenu_SetBotNames
=================
*/
unsafe extern "C" fn UI_RemoveBotsMenu_SetBotNames() {
    let mut n: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    n = 0i32;
    while n < 7i32 && removeBotsMenuInfo.baseBotNum + n < removeBotsMenuInfo.numBots {
        trap_GetConfigString(
            32i32
                + 256i32
                + 256i32
                + removeBotsMenuInfo.botClientNums[(removeBotsMenuInfo.baseBotNum + n) as usize],
            info.as_mut_ptr(),
            1024i32,
        );
        Q_strncpyz(
            removeBotsMenuInfo.botnames[n as usize].as_mut_ptr(),
            Info_ValueForKey(
                info.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        );
        Q_CleanStr(removeBotsMenuInfo.botnames[n as usize].as_mut_ptr());
        n += 1
    }
}
/*
=================
UI_RemoveBotsMenu_UpEvent
=================
*/
unsafe extern "C" fn UI_RemoveBotsMenu_UpEvent(mut _ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    if removeBotsMenuInfo.baseBotNum > 0i32 {
        removeBotsMenuInfo.baseBotNum -= 1;
        UI_RemoveBotsMenu_SetBotNames();
    };
}
/*
=================
UI_RemoveBotsMenu_GetBots
=================
*/
unsafe extern "C" fn UI_RemoveBotsMenu_GetBots() {
    let mut numPlayers: libc::c_int = 0;
    let mut isBot: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    trap_GetConfigString(
        0i32,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    numPlayers = atoi(Info_ValueForKey(
        info.as_mut_ptr(),
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
    ));
    removeBotsMenuInfo.numBots = 0i32;
    n = 0i32;
    while n < numPlayers {
        trap_GetConfigString(32i32 + 256i32 + 256i32 + n, info.as_mut_ptr(), 1024i32);
        isBot = atoi(Info_ValueForKey(
            info.as_mut_ptr(),
            b"skill\x00" as *const u8 as *const libc::c_char,
        ));
        if !(0 == isBot) {
            removeBotsMenuInfo.botClientNums[removeBotsMenuInfo.numBots as usize] = n;
            removeBotsMenuInfo.numBots += 1
        }
        n += 1
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct removeBotsMenuInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub background: menubitmap_s,
    pub arrows: menubitmap_s,
    pub up: menubitmap_s,
    pub down: menubitmap_s,
    pub bots: [menutext_s; 7],
    pub delete: menubitmap_s,
    pub back: menubitmap_s,
    pub numBots: libc::c_int,
    pub baseBotNum: libc::c_int,
    pub selectedBotNum: libc::c_int,
    pub botnames: [[libc::c_char; 32]; 7],
    pub botClientNums: [libc::c_int; 1024],
}
