use libc;
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t, EXEC_APPEND,
    EXEC_INSERT, EXEC_NOW,
};
use stdlib::{atoi, memset};
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
use ui_cdkey::{UI_CDKeyMenu, UI_CDKeyMenu_Cache, UI_CDKeyMenu_f};
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
    trap_Cmd_ExecuteText, trap_Cvar_Set, trap_R_RegisterShaderNoMip, uiStatic_t,
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
// ui_cinematics.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_CinematicsMenu() {
    UI_CinematicsMenu_Init();
    UI_PushMenu(&mut cinematicsMenuInfo.menu);
}
static mut cinematicsMenuInfo: cinematicsMenuInfo_t = cinematicsMenuInfo_t {
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
    cin_idlogo: menutext_s {
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
    cin_intro: menutext_s {
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
    cin_tier1: menutext_s {
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
    cin_tier2: menutext_s {
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
    cin_tier3: menutext_s {
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
    cin_tier4: menutext_s {
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
    cin_tier5: menutext_s {
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
    cin_tier6: menutext_s {
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
    cin_tier7: menutext_s {
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
    cin_end: menutext_s {
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
UI_CinematicsMenu_Init
===============
*/
unsafe extern "C" fn UI_CinematicsMenu_Init() {
    let mut y: libc::c_int = 0;
    UI_CinematicsMenu_Cache();
    memset(
        &mut cinematicsMenuInfo as *mut cinematicsMenuInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<cinematicsMenuInfo_t>() as libc::c_ulong,
    );
    cinematicsMenuInfo.menu.fullscreen = qtrue;
    cinematicsMenuInfo.banner.generic.type_0 = 10i32;
    cinematicsMenuInfo.banner.generic.x = 320i32;
    cinematicsMenuInfo.banner.generic.y = 16i32;
    cinematicsMenuInfo.banner.string =
        b"CINEMATICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.banner.color = color_white.as_mut_ptr();
    cinematicsMenuInfo.banner.style = 0x1i32;
    cinematicsMenuInfo.framel.generic.type_0 = 6i32;
    cinematicsMenuInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    cinematicsMenuInfo.framel.generic.flags = 0x4000i32 as libc::c_uint;
    cinematicsMenuInfo.framel.generic.x = 0i32;
    cinematicsMenuInfo.framel.generic.y = 78i32;
    cinematicsMenuInfo.framel.width = 256i32;
    cinematicsMenuInfo.framel.height = 329i32;
    cinematicsMenuInfo.framer.generic.type_0 = 6i32;
    cinematicsMenuInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    cinematicsMenuInfo.framer.generic.flags = 0x4000i32 as libc::c_uint;
    cinematicsMenuInfo.framer.generic.x = 376i32;
    cinematicsMenuInfo.framer.generic.y = 76i32;
    cinematicsMenuInfo.framer.width = 256i32;
    cinematicsMenuInfo.framer.height = 334i32;
    y = 100i32;
    cinematicsMenuInfo.cin_idlogo.generic.type_0 = 9i32;
    cinematicsMenuInfo.cin_idlogo.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.cin_idlogo.generic.x = 320i32;
    cinematicsMenuInfo.cin_idlogo.generic.y = y;
    cinematicsMenuInfo.cin_idlogo.generic.id = 11i32;
    cinematicsMenuInfo.cin_idlogo.generic.callback = Some(UI_CinematicsMenu_Event);
    cinematicsMenuInfo.cin_idlogo.string =
        b"ID LOGO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_idlogo.color = color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_idlogo.style = 0x1i32;
    y += 30i32;
    cinematicsMenuInfo.cin_intro.generic.type_0 = 9i32;
    cinematicsMenuInfo.cin_intro.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.cin_intro.generic.x = 320i32;
    cinematicsMenuInfo.cin_intro.generic.y = y;
    cinematicsMenuInfo.cin_intro.generic.id = 12i32;
    cinematicsMenuInfo.cin_intro.generic.callback = Some(UI_CinematicsMenu_Event);
    cinematicsMenuInfo.cin_intro.string =
        b"INTRO\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_intro.color = color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_intro.style = 0x1i32;
    if 0 != uis.demoversion as u64 {
        cinematicsMenuInfo.cin_intro.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 30i32;
    cinematicsMenuInfo.cin_tier1.generic.type_0 = 9i32;
    cinematicsMenuInfo.cin_tier1.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.cin_tier1.generic.x = 320i32;
    cinematicsMenuInfo.cin_tier1.generic.y = y;
    cinematicsMenuInfo.cin_tier1.generic.id = 13i32;
    cinematicsMenuInfo.cin_tier1.generic.callback = Some(UI_CinematicsMenu_Event);
    cinematicsMenuInfo.cin_tier1.string =
        b"Tier 1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier1.color = color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier1.style = 0x1i32;
    if 0 == UI_CanShowTierVideo(1i32) as u64 {
        cinematicsMenuInfo.cin_tier1.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 30i32;
    cinematicsMenuInfo.cin_tier2.generic.type_0 = 9i32;
    cinematicsMenuInfo.cin_tier2.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.cin_tier2.generic.x = 320i32;
    cinematicsMenuInfo.cin_tier2.generic.y = y;
    cinematicsMenuInfo.cin_tier2.generic.id = 14i32;
    cinematicsMenuInfo.cin_tier2.generic.callback = Some(UI_CinematicsMenu_Event);
    cinematicsMenuInfo.cin_tier2.string =
        b"Tier 2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier2.color = color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier2.style = 0x1i32;
    if 0 == UI_CanShowTierVideo(2i32) as u64 {
        cinematicsMenuInfo.cin_tier2.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 30i32;
    cinematicsMenuInfo.cin_tier3.generic.type_0 = 9i32;
    cinematicsMenuInfo.cin_tier3.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.cin_tier3.generic.x = 320i32;
    cinematicsMenuInfo.cin_tier3.generic.y = y;
    cinematicsMenuInfo.cin_tier3.generic.id = 15i32;
    cinematicsMenuInfo.cin_tier3.generic.callback = Some(UI_CinematicsMenu_Event);
    cinematicsMenuInfo.cin_tier3.string =
        b"Tier 3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier3.color = color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier3.style = 0x1i32;
    if 0 == UI_CanShowTierVideo(3i32) as u64 {
        cinematicsMenuInfo.cin_tier3.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 30i32;
    cinematicsMenuInfo.cin_tier4.generic.type_0 = 9i32;
    cinematicsMenuInfo.cin_tier4.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.cin_tier4.generic.x = 320i32;
    cinematicsMenuInfo.cin_tier4.generic.y = y;
    cinematicsMenuInfo.cin_tier4.generic.id = 16i32;
    cinematicsMenuInfo.cin_tier4.generic.callback = Some(UI_CinematicsMenu_Event);
    cinematicsMenuInfo.cin_tier4.string =
        b"Tier 4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier4.color = color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier4.style = 0x1i32;
    if 0 == UI_CanShowTierVideo(4i32) as u64 {
        cinematicsMenuInfo.cin_tier4.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 30i32;
    cinematicsMenuInfo.cin_tier5.generic.type_0 = 9i32;
    cinematicsMenuInfo.cin_tier5.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.cin_tier5.generic.x = 320i32;
    cinematicsMenuInfo.cin_tier5.generic.y = y;
    cinematicsMenuInfo.cin_tier5.generic.id = 17i32;
    cinematicsMenuInfo.cin_tier5.generic.callback = Some(UI_CinematicsMenu_Event);
    cinematicsMenuInfo.cin_tier5.string =
        b"Tier 5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier5.color = color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier5.style = 0x1i32;
    if 0 == UI_CanShowTierVideo(5i32) as u64 {
        cinematicsMenuInfo.cin_tier5.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 30i32;
    cinematicsMenuInfo.cin_tier6.generic.type_0 = 9i32;
    cinematicsMenuInfo.cin_tier6.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.cin_tier6.generic.x = 320i32;
    cinematicsMenuInfo.cin_tier6.generic.y = y;
    cinematicsMenuInfo.cin_tier6.generic.id = 18i32;
    cinematicsMenuInfo.cin_tier6.generic.callback = Some(UI_CinematicsMenu_Event);
    cinematicsMenuInfo.cin_tier6.string =
        b"Tier 6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier6.color = color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier6.style = 0x1i32;
    if 0 == UI_CanShowTierVideo(6i32) as u64 {
        cinematicsMenuInfo.cin_tier6.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 30i32;
    cinematicsMenuInfo.cin_tier7.generic.type_0 = 9i32;
    cinematicsMenuInfo.cin_tier7.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.cin_tier7.generic.x = 320i32;
    cinematicsMenuInfo.cin_tier7.generic.y = y;
    cinematicsMenuInfo.cin_tier7.generic.id = 19i32;
    cinematicsMenuInfo.cin_tier7.generic.callback = Some(UI_CinematicsMenu_Event);
    cinematicsMenuInfo.cin_tier7.string =
        b"Tier 7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_tier7.color = color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_tier7.style = 0x1i32;
    if 0 == UI_CanShowTierVideo(7i32) as u64 {
        cinematicsMenuInfo.cin_tier7.generic.flags |= 0x2000i32 as libc::c_uint
    }
    y += 30i32;
    cinematicsMenuInfo.cin_end.generic.type_0 = 9i32;
    cinematicsMenuInfo.cin_end.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.cin_end.generic.x = 320i32;
    cinematicsMenuInfo.cin_end.generic.y = y;
    cinematicsMenuInfo.cin_end.generic.id = 20i32;
    cinematicsMenuInfo.cin_end.generic.callback = Some(UI_CinematicsMenu_Event);
    cinematicsMenuInfo.cin_end.string =
        b"END\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    cinematicsMenuInfo.cin_end.color = color_red.as_mut_ptr();
    cinematicsMenuInfo.cin_end.style = 0x1i32;
    if 0 == UI_CanShowTierVideo(8i32) as u64 {
        cinematicsMenuInfo.cin_end.generic.flags |= 0x2000i32 as libc::c_uint
    }
    cinematicsMenuInfo.back.generic.type_0 = 6i32;
    cinematicsMenuInfo.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    cinematicsMenuInfo.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    cinematicsMenuInfo.back.generic.id = 10i32;
    cinematicsMenuInfo.back.generic.callback = Some(UI_CinematicsMenu_BackEvent);
    cinematicsMenuInfo.back.generic.x = 0i32;
    cinematicsMenuInfo.back.generic.y = 480i32 - 64i32;
    cinematicsMenuInfo.back.width = 128i32;
    cinematicsMenuInfo.back.height = 64i32;
    cinematicsMenuInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.cin_idlogo as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.cin_intro as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.cin_tier1 as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.cin_tier2 as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.cin_tier3 as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.cin_tier4 as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.cin_tier5 as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.cin_tier6 as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.cin_tier7 as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.cin_end as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut cinematicsMenuInfo.menu,
        &mut cinematicsMenuInfo.back as *mut menubitmap_s as *mut libc::c_void,
    );
}
/*
===============
UI_CinematicsMenu_BackEvent
===============
*/
unsafe extern "C" fn UI_CinematicsMenu_BackEvent(
    mut _ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3i32 {
        return;
    }
    UI_PopMenu();
}
/*
===============
UI_CinematicsMenu_Event
===============
*/
unsafe extern "C" fn UI_CinematicsMenu_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    let mut n: libc::c_int = 0;
    if event != 3i32 {
        return;
    }
    n = (*(ptr as *mut menucommon_s)).id - 11i32;
    trap_Cvar_Set(
        b"nextmap\x00" as *const u8 as *const libc::c_char,
        va(
            b"ui_cinematics %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            n,
        ),
    );
    if 0 != uis.demoversion as libc::c_uint && (*(ptr as *mut menucommon_s)).id == 20i32 {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            b"disconnect; cinematic demoEnd.RoQ 1\n\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            va(
                b"disconnect; cinematic %s.RoQ\n\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cinematics[n as usize],
            ),
        );
    };
}
static mut cinematics: [*mut libc::c_char; 10] = [
    b"idlogo\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"intro\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"tier7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"end\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn UI_CinematicsMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn UI_CinematicsMenu_f() {
    let mut n: libc::c_int = 0;
    n = atoi(UI_Argv(1i32));
    UI_CinematicsMenu();
    Menu_SetCursorToItem(
        &mut cinematicsMenuInfo.menu,
        cinematicsMenuInfo.menu.items[(n + 3i32) as usize],
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cinematicsMenuInfo_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub cin_idlogo: menutext_s,
    pub cin_intro: menutext_s,
    pub cin_tier1: menutext_s,
    pub cin_tier2: menutext_s,
    pub cin_tier3: menutext_s,
    pub cin_tier4: menutext_s,
    pub cin_tier5: menutext_s,
    pub cin_tier6: menutext_s,
    pub cin_tier7: menutext_s,
    pub cin_end: menutext_s,
    pub back: menubitmap_s,
}
