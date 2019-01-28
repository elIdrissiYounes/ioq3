use bg_public_h::{
    unnamed_0, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT,
};
use libc;
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t, Com_Clamp,
    Info_ValueForKey, Q_stricmp, Q_strncpyz, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stddef_h::size_t;
use stdlib::{__compar_fn_t, atoi, memset, qsort};
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
    _tag_menuframework, menubitmap_s, menucommon_s, menuframework_s, menulist_s, menutext_s,
    trap_Cmd_ExecuteText, trap_Cvar_VariableValue, trap_GetConfigString,
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
use ui_startserver::{
    ServerOptions_Cache, StartServer_Cache, UI_BotSelectMenu_Cache, UI_StartServerMenu,
};
use ui_team::{TeamMain_Cache, UI_TeamMainMenu};
use ui_teamorders::{UI_TeamOrdersMenu, UI_TeamOrdersMenu_f};
use ui_video::{DriverInfo_Cache, GraphicsOptions_Cache, UI_GraphicsOptionsMenu};

//
// ui_addbots.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_AddBots_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/arrows_vert_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/arrows_vert_top\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/arrows_vert_bot\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_AddBotsMenu() {
    UI_AddBotsMenu_Init();
    UI_PushMenu(&mut addBotsMenuInfo.menu);
}
static mut addBotsMenuInfo: addBotsMenuInfo_t = addBotsMenuInfo_t {
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
    skill: menulist_s {
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
    team: menulist_s {
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
    delay: 0,
    baseBotNum: 0,
    selectedBotNum: 0,
    sortedBotNums: [0; 1024],
    botnames: [[0; 32]; 7],
};
unsafe extern "C" fn UI_AddBotsMenu_Init() {
    let mut n: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut gametype: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    trap_GetConfigString(0i32, info.as_mut_ptr(), 1024i32);
    gametype = atoi(Info_ValueForKey(
        info.as_mut_ptr(),
        b"g_gametype\x00" as *const u8 as *const libc::c_char,
    ));
    memset(
        &mut addBotsMenuInfo as *mut addBotsMenuInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<addBotsMenuInfo_t>() as libc::c_ulong,
    );
    addBotsMenuInfo.menu.fullscreen = qfalse;
    addBotsMenuInfo.menu.wrapAround = qtrue;
    addBotsMenuInfo.delay = 1000i32;
    UI_AddBots_Cache();
    addBotsMenuInfo.numBots = UI_GetNumBots();
    count = if addBotsMenuInfo.numBots < 7i32 {
        addBotsMenuInfo.numBots
    } else {
        7i32
    };
    addBotsMenuInfo.banner.generic.type_0 = 10i32;
    addBotsMenuInfo.banner.generic.x = 320i32;
    addBotsMenuInfo.banner.generic.y = 16i32;
    addBotsMenuInfo.banner.string =
        b"ADD BOTS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    addBotsMenuInfo.banner.color = color_white.as_mut_ptr();
    addBotsMenuInfo.banner.style = 0x1i32;
    addBotsMenuInfo.background.generic.type_0 = 6i32;
    addBotsMenuInfo.background.generic.name =
        b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char;
    addBotsMenuInfo.background.generic.flags = 0x4000i32 as libc::c_uint;
    addBotsMenuInfo.background.generic.x = 320i32 - 233i32;
    addBotsMenuInfo.background.generic.y = 240i32 - 166i32;
    addBotsMenuInfo.background.width = 466i32;
    addBotsMenuInfo.background.height = 332i32;
    addBotsMenuInfo.arrows.generic.type_0 = 6i32;
    addBotsMenuInfo.arrows.generic.name =
        b"menu/art/arrows_vert_0\x00" as *const u8 as *const libc::c_char;
    addBotsMenuInfo.arrows.generic.flags = 0x4000i32 as libc::c_uint;
    addBotsMenuInfo.arrows.generic.x = 200i32;
    addBotsMenuInfo.arrows.generic.y = 128i32;
    addBotsMenuInfo.arrows.width = 64i32;
    addBotsMenuInfo.arrows.height = 128i32;
    addBotsMenuInfo.up.generic.type_0 = 6i32;
    addBotsMenuInfo.up.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    addBotsMenuInfo.up.generic.x = 200i32;
    addBotsMenuInfo.up.generic.y = 128i32;
    addBotsMenuInfo.up.generic.id = 13i32;
    addBotsMenuInfo.up.generic.callback = Some(UI_AddBotsMenu_UpEvent);
    addBotsMenuInfo.up.width = 64i32;
    addBotsMenuInfo.up.height = 64i32;
    addBotsMenuInfo.up.focuspic =
        b"menu/art/arrows_vert_top\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    addBotsMenuInfo.down.generic.type_0 = 6i32;
    addBotsMenuInfo.down.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    addBotsMenuInfo.down.generic.x = 200i32;
    addBotsMenuInfo.down.generic.y = 128i32 + 64i32;
    addBotsMenuInfo.down.generic.id = 14i32;
    addBotsMenuInfo.down.generic.callback = Some(UI_AddBotsMenu_DownEvent);
    addBotsMenuInfo.down.width = 64i32;
    addBotsMenuInfo.down.height = 64i32;
    addBotsMenuInfo.down.focuspic =
        b"menu/art/arrows_vert_bot\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    n = 0i32;
    y = 120i32;
    while n < count {
        addBotsMenuInfo.bots[n as usize].generic.type_0 = 9i32;
        addBotsMenuInfo.bots[n as usize].generic.flags =
            0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
        addBotsMenuInfo.bots[n as usize].generic.id = 20i32 + n;
        addBotsMenuInfo.bots[n as usize].generic.x = 320i32 - 56i32;
        addBotsMenuInfo.bots[n as usize].generic.y = y;
        addBotsMenuInfo.bots[n as usize].generic.callback = Some(UI_AddBotsMenu_BotEvent);
        addBotsMenuInfo.bots[n as usize].string = addBotsMenuInfo.botnames[n as usize].as_mut_ptr();
        addBotsMenuInfo.bots[n as usize].color = color_orange.as_mut_ptr();
        addBotsMenuInfo.bots[n as usize].style = 0i32 | 0x10i32;
        n += 1;
        y += 20i32
    }
    y += 12i32;
    addBotsMenuInfo.skill.generic.type_0 = 3i32;
    addBotsMenuInfo.skill.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    addBotsMenuInfo.skill.generic.x = 320i32;
    addBotsMenuInfo.skill.generic.y = y;
    addBotsMenuInfo.skill.generic.name = b"Skill:\x00" as *const u8 as *const libc::c_char;
    addBotsMenuInfo.skill.generic.id = 15i32;
    addBotsMenuInfo.skill.itemnames = skillNames.as_mut_ptr();
    addBotsMenuInfo.skill.curvalue = Com_Clamp(
        0i32 as libc::c_float,
        4i32 as libc::c_float,
        (trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const libc::c_char)
            as libc::c_int
            - 1i32) as libc::c_float,
    ) as libc::c_int;
    y += 16i32;
    addBotsMenuInfo.team.generic.type_0 = 3i32;
    addBotsMenuInfo.team.generic.flags = 0x100i32 as libc::c_uint | 0x2i32 as libc::c_uint;
    addBotsMenuInfo.team.generic.x = 320i32;
    addBotsMenuInfo.team.generic.y = y;
    addBotsMenuInfo.team.generic.name = b"Team: \x00" as *const u8 as *const libc::c_char;
    addBotsMenuInfo.team.generic.id = 16i32;
    if gametype >= GT_TEAM as libc::c_int {
        addBotsMenuInfo.team.itemnames = teamNames2.as_mut_ptr()
    } else {
        addBotsMenuInfo.team.itemnames = teamNames1.as_mut_ptr();
        addBotsMenuInfo.team.generic.flags = 0x2000i32 as libc::c_uint
    }
    addBotsMenuInfo.go.generic.type_0 = 6i32;
    addBotsMenuInfo.go.generic.name = b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char;
    addBotsMenuInfo.go.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    addBotsMenuInfo.go.generic.id = 11i32;
    addBotsMenuInfo.go.generic.callback = Some(UI_AddBotsMenu_FightEvent);
    addBotsMenuInfo.go.generic.x = 320i32 + 128i32 - 128i32;
    addBotsMenuInfo.go.generic.y = 256i32 + 128i32 - 64i32;
    addBotsMenuInfo.go.width = 128i32;
    addBotsMenuInfo.go.height = 64i32;
    addBotsMenuInfo.go.focuspic =
        b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    addBotsMenuInfo.back.generic.type_0 = 6i32;
    addBotsMenuInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    addBotsMenuInfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    addBotsMenuInfo.back.generic.id = 10i32;
    addBotsMenuInfo.back.generic.callback = Some(UI_AddBotsMenu_BackEvent);
    addBotsMenuInfo.back.generic.x = 320i32 - 128i32;
    addBotsMenuInfo.back.generic.y = 256i32 + 128i32 - 64i32;
    addBotsMenuInfo.back.width = 128i32;
    addBotsMenuInfo.back.height = 64i32;
    addBotsMenuInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    addBotsMenuInfo.baseBotNum = 0i32;
    addBotsMenuInfo.selectedBotNum = 0i32;
    addBotsMenuInfo.bots[0usize].color = color_white.as_mut_ptr();
    UI_AddBotsMenu_GetSortedBotNums();
    UI_AddBotsMenu_SetBotNames();
    Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.background as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.up as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.down as *mut menubitmap_s as *mut libc::c_void,
    );
    n = 0i32;
    while n < count {
        Menu_AddItem(
            &mut addBotsMenuInfo.menu,
            &mut addBotsMenuInfo.bots[n as usize] as *mut menutext_s as *mut libc::c_void,
        );
        n += 1
    }
    Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.skill as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.team as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.go as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_AddBotsMenu_SetBotNames
=================
*/
unsafe extern "C" fn UI_AddBotsMenu_SetBotNames() {
    let mut n: libc::c_int = 0;
    let mut info: *const libc::c_char = 0 as *const libc::c_char;
    n = 0i32;
    while n < 7i32 {
        info = UI_GetBotInfoByNumber(
            addBotsMenuInfo.sortedBotNums[(addBotsMenuInfo.baseBotNum + n) as usize],
        );
        Q_strncpyz(
            addBotsMenuInfo.botnames[n as usize].as_mut_ptr(),
            Info_ValueForKey(info, b"name\x00" as *const u8 as *const libc::c_char),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        );
        n += 1
    }
}
unsafe extern "C" fn UI_AddBotsMenu_GetSortedBotNums() {
    let mut n: libc::c_int = 0;
    n = 0i32;
    while n < addBotsMenuInfo.numBots {
        addBotsMenuInfo.sortedBotNums[n as usize] = n;
        n += 1
    }
    qsort(
        addBotsMenuInfo.sortedBotNums.as_mut_ptr() as *mut libc::c_void,
        addBotsMenuInfo.numBots as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(UI_AddBotsMenu_SortCompare),
    );
}
/*
=================
UI_AddBotsMenu_GetSortedBotNums
=================
*/
unsafe extern "C" fn UI_AddBotsMenu_SortCompare(
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
UI_AddBotsMenu_BackEvent
=================
*/
unsafe extern "C" fn UI_AddBotsMenu_BackEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    UI_PopMenu();
}
/*
=================
UI_AddBotsMenu_FightEvent
=================
*/
unsafe extern "C" fn UI_AddBotsMenu_FightEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    let mut team: *const libc::c_char = 0 as *const libc::c_char;
    let mut skill: libc::c_int = 0;
    if event != 3i32 {
        return;
    }
    team = *addBotsMenuInfo
        .team
        .itemnames
        .offset(addBotsMenuInfo.team.curvalue as isize);
    skill = addBotsMenuInfo.skill.curvalue + 1i32;
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        va(
            b"addbot %s %i %s %i\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            addBotsMenuInfo.botnames[addBotsMenuInfo.selectedBotNum as usize].as_mut_ptr(),
            skill,
            team,
            addBotsMenuInfo.delay,
        ),
    );
    addBotsMenuInfo.delay += 1500i32;
}
static mut teamNames1: [*const libc::c_char; 2] = [
    b"Free\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
static mut teamNames2: [*const libc::c_char; 3] = [
    b"Red\x00" as *const u8 as *const libc::c_char,
    b"Blue\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
UI_AddBotsMenu_Init
=================
*/
static mut skillNames: [*const libc::c_char; 6] = [
    b"I Can Win\x00" as *const u8 as *const libc::c_char,
    b"Bring It On\x00" as *const u8 as *const libc::c_char,
    b"Hurt Me Plenty\x00" as *const u8 as *const libc::c_char,
    b"Hardcore\x00" as *const u8 as *const libc::c_char,
    b"Nightmare!\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
UI_AddBotsMenu_BotEvent
=================
*/
unsafe extern "C" fn UI_AddBotsMenu_BotEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    addBotsMenuInfo.bots[addBotsMenuInfo.selectedBotNum as usize].color = color_orange.as_mut_ptr();
    addBotsMenuInfo.selectedBotNum = (*(ptr as *mut menucommon_s)).id - 20i32;
    addBotsMenuInfo.bots[addBotsMenuInfo.selectedBotNum as usize].color = color_white.as_mut_ptr();
}
/*
=================
UI_AddBotsMenu_DownEvent
=================
*/
unsafe extern "C" fn UI_AddBotsMenu_DownEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    if addBotsMenuInfo.baseBotNum + 7i32 < addBotsMenuInfo.numBots {
        addBotsMenuInfo.baseBotNum += 1;
        UI_AddBotsMenu_SetBotNames();
    };
}
/*
=================
UI_AddBotsMenu_UpEvent
=================
*/
unsafe extern "C" fn UI_AddBotsMenu_UpEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    if addBotsMenuInfo.baseBotNum > 0i32 {
        addBotsMenuInfo.baseBotNum -= 1;
        UI_AddBotsMenu_SetBotNames();
    };
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct addBotsMenuInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub background: menubitmap_s,
    pub arrows: menubitmap_s,
    pub up: menubitmap_s,
    pub down: menubitmap_s,
    pub bots: [menutext_s; 7],
    pub skill: menulist_s,
    pub team: menulist_s,
    pub go: menubitmap_s,
    pub back: menubitmap_s,
    pub numBots: libc::c_int,
    pub delay: libc::c_int,
    pub baseBotNum: libc::c_int,
    pub selectedBotNum: libc::c_int,
    pub sortedBotNums: [libc::c_int; 1024],
    pub botnames: [[libc::c_char; 32]; 7],
}
