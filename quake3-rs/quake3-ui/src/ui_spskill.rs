use keycodes_h::{
    unnamed_0, K_ALT, K_AUX1, K_AUX10, K_AUX11, K_AUX12, K_AUX13, K_AUX14, K_AUX15, K_AUX16,
    K_AUX2, K_AUX3, K_AUX4, K_AUX5, K_AUX6, K_AUX7, K_AUX8, K_AUX9, K_BACKSPACE, K_BREAK,
    K_CAPSLOCK, K_COMMAND, K_COMPOSE, K_CONSOLE, K_CTRL, K_DEL, K_DOWNARROW, K_END, K_ENTER,
    K_ESCAPE, K_EURO, K_F1, K_F10, K_F11, K_F12, K_F13, K_F14, K_F15, K_F2, K_F3, K_F4, K_F5, K_F6,
    K_F7, K_F8, K_F9, K_HELP, K_HOME, K_INS, K_JOY1, K_JOY10, K_JOY11, K_JOY12, K_JOY13, K_JOY14,
    K_JOY15, K_JOY16, K_JOY17, K_JOY18, K_JOY19, K_JOY2, K_JOY20, K_JOY21, K_JOY22, K_JOY23,
    K_JOY24, K_JOY25, K_JOY26, K_JOY27, K_JOY28, K_JOY29, K_JOY3, K_JOY30, K_JOY31, K_JOY32,
    K_JOY4, K_JOY5, K_JOY6, K_JOY7, K_JOY8, K_JOY9, K_KP_5, K_KP_DEL, K_KP_DOWNARROW, K_KP_END,
    K_KP_ENTER, K_KP_EQUALS, K_KP_HOME, K_KP_INS, K_KP_LEFTARROW, K_KP_MINUS, K_KP_NUMLOCK,
    K_KP_PGDN, K_KP_PGUP, K_KP_PLUS, K_KP_RIGHTARROW, K_KP_SLASH, K_KP_STAR, K_KP_UPARROW,
    K_LEFTARROW, K_MENU, K_MODE, K_MOUSE1, K_MOUSE2, K_MOUSE3, K_MOUSE4, K_MOUSE5, K_MWHEELDOWN,
    K_MWHEELUP, K_PAD0_A, K_PAD0_B, K_PAD0_BACK, K_PAD0_DPAD_DOWN, K_PAD0_DPAD_LEFT,
    K_PAD0_DPAD_RIGHT, K_PAD0_DPAD_UP, K_PAD0_GUIDE, K_PAD0_LEFTSHOULDER, K_PAD0_LEFTSTICK_CLICK,
    K_PAD0_LEFTSTICK_DOWN, K_PAD0_LEFTSTICK_LEFT, K_PAD0_LEFTSTICK_RIGHT, K_PAD0_LEFTSTICK_UP,
    K_PAD0_LEFTTRIGGER, K_PAD0_RIGHTSHOULDER, K_PAD0_RIGHTSTICK_CLICK, K_PAD0_RIGHTSTICK_DOWN,
    K_PAD0_RIGHTSTICK_LEFT, K_PAD0_RIGHTSTICK_RIGHT, K_PAD0_RIGHTSTICK_UP, K_PAD0_RIGHTTRIGGER,
    K_PAD0_START, K_PAD0_X, K_PAD0_Y, K_PAUSE, K_PGDN, K_PGUP, K_POWER, K_PRINT, K_RIGHTARROW,
    K_SCROLLOCK, K_SHIFT, K_SPACE, K_SUPER, K_SYSREQ, K_TAB, K_UNDO, K_UPARROW, K_WORLD_0,
    K_WORLD_1, K_WORLD_10, K_WORLD_11, K_WORLD_12, K_WORLD_13, K_WORLD_14, K_WORLD_15, K_WORLD_16,
    K_WORLD_17, K_WORLD_18, K_WORLD_19, K_WORLD_2, K_WORLD_20, K_WORLD_21, K_WORLD_22, K_WORLD_23,
    K_WORLD_24, K_WORLD_25, K_WORLD_26, K_WORLD_27, K_WORLD_28, K_WORLD_29, K_WORLD_3, K_WORLD_30,
    K_WORLD_31, K_WORLD_32, K_WORLD_33, K_WORLD_34, K_WORLD_35, K_WORLD_36, K_WORLD_37, K_WORLD_38,
    K_WORLD_39, K_WORLD_4, K_WORLD_40, K_WORLD_41, K_WORLD_42, K_WORLD_43, K_WORLD_44, K_WORLD_45,
    K_WORLD_46, K_WORLD_47, K_WORLD_48, K_WORLD_49, K_WORLD_5, K_WORLD_50, K_WORLD_51, K_WORLD_52,
    K_WORLD_53, K_WORLD_54, K_WORLD_55, K_WORLD_56, K_WORLD_57, K_WORLD_58, K_WORLD_59, K_WORLD_6,
    K_WORLD_60, K_WORLD_61, K_WORLD_62, K_WORLD_63, K_WORLD_64, K_WORLD_65, K_WORLD_66, K_WORLD_67,
    K_WORLD_68, K_WORLD_69, K_WORLD_7, K_WORLD_70, K_WORLD_71, K_WORLD_72, K_WORLD_73, K_WORLD_74,
    K_WORLD_75, K_WORLD_76, K_WORLD_77, K_WORLD_78, K_WORLD_79, K_WORLD_8, K_WORLD_80, K_WORLD_81,
    K_WORLD_82, K_WORLD_83, K_WORLD_84, K_WORLD_85, K_WORLD_86, K_WORLD_87, K_WORLD_88, K_WORLD_89,
    K_WORLD_9, K_WORLD_90, K_WORLD_91, K_WORLD_92, K_WORLD_93, K_WORLD_94, K_WORLD_95, MAX_KEYS,
};
use libc;
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, vec4_t, vec_t, Com_Clamp,
    CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY, CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE,
    CHAN_WEAPON,
};
use stdlib::memset;
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
    trap_Cvar_SetValue, trap_Cvar_VariableValue, trap_R_RegisterShaderNoMip, trap_S_RegisterSound,
    trap_S_StartLocalSound,
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
use ui_startserver::{
    ServerOptions_Cache, StartServer_Cache, UI_BotSelectMenu_Cache, UI_StartServerMenu,
};
use ui_team::{TeamMain_Cache, UI_TeamMainMenu};
use ui_teamorders::{UI_TeamOrdersMenu, UI_TeamOrdersMenu_f};
use ui_video::{DriverInfo_Cache, GraphicsOptions_Cache, UI_GraphicsOptionsMenu};

//
// ui_spSkill.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_SPSkillMenu(mut arenaInfo: *const libc::c_char) {
    UI_SPSkillMenu_Init();
    skillMenuInfo.arenaInfo = arenaInfo;
    UI_PushMenu(&mut skillMenuInfo.menu);
    Menu_SetCursorToItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.item_fight as *mut menubitmap_s as *mut libc::c_void,
    );
}
static mut skillMenuInfo: skillMenuInfo_t = skillMenuInfo_t {
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
    art_frame: menubitmap_s {
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
    art_banner: menutext_s {
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
    item_baby: menutext_s {
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
    item_easy: menutext_s {
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
    item_medium: menutext_s {
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
    item_hard: menutext_s {
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
    item_nightmare: menutext_s {
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
    art_skillPic: menubitmap_s {
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
    item_back: menubitmap_s {
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
    item_fight: menubitmap_s {
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
    arenaInfo: 0 as *const libc::c_char,
    skillpics: [0; 5],
    nightmareSound: 0,
    silenceSound: 0,
};
/*
=================
UI_SPSkillMenu_Init
=================
*/
unsafe extern "C" fn UI_SPSkillMenu_Init() {
    let mut skill: libc::c_int = 0;
    memset(
        &mut skillMenuInfo as *mut skillMenuInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<skillMenuInfo_t>() as libc::c_ulong,
    );
    skillMenuInfo.menu.fullscreen = qtrue;
    skillMenuInfo.menu.key = Some(UI_SPSkillMenu_Key);
    UI_SPSkillMenu_Cache();
    skillMenuInfo.art_frame.generic.type_0 = 6i32;
    skillMenuInfo.art_frame.generic.name =
        b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char;
    skillMenuInfo.art_frame.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    skillMenuInfo.art_frame.generic.x = 142i32;
    skillMenuInfo.art_frame.generic.y = 118i32;
    skillMenuInfo.art_frame.width = 359i32;
    skillMenuInfo.art_frame.height = 256i32;
    skillMenuInfo.art_banner.generic.type_0 = 10i32;
    skillMenuInfo.art_banner.generic.flags = 0x8i32 as libc::c_uint;
    skillMenuInfo.art_banner.generic.x = 320i32;
    skillMenuInfo.art_banner.generic.y = 16i32;
    skillMenuInfo.art_banner.string =
        b"DIFFICULTY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.art_banner.color = color_white.as_mut_ptr();
    skillMenuInfo.art_banner.style = 0x1i32;
    skillMenuInfo.item_baby.generic.type_0 = 9i32;
    skillMenuInfo.item_baby.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    skillMenuInfo.item_baby.generic.x = 320i32;
    skillMenuInfo.item_baby.generic.y = 170i32;
    skillMenuInfo.item_baby.generic.callback = Some(UI_SPSkillMenu_SkillEvent);
    skillMenuInfo.item_baby.generic.id = 10i32;
    skillMenuInfo.item_baby.string =
        b"I Can Win\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.item_baby.color = color_red.as_mut_ptr();
    skillMenuInfo.item_baby.style = 0x1i32;
    skillMenuInfo.item_easy.generic.type_0 = 9i32;
    skillMenuInfo.item_easy.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    skillMenuInfo.item_easy.generic.x = 320i32;
    skillMenuInfo.item_easy.generic.y = 198i32;
    skillMenuInfo.item_easy.generic.callback = Some(UI_SPSkillMenu_SkillEvent);
    skillMenuInfo.item_easy.generic.id = 11i32;
    skillMenuInfo.item_easy.string =
        b"Bring It On\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.item_easy.color = color_red.as_mut_ptr();
    skillMenuInfo.item_easy.style = 0x1i32;
    skillMenuInfo.item_medium.generic.type_0 = 9i32;
    skillMenuInfo.item_medium.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    skillMenuInfo.item_medium.generic.x = 320i32;
    skillMenuInfo.item_medium.generic.y = 227i32;
    skillMenuInfo.item_medium.generic.callback = Some(UI_SPSkillMenu_SkillEvent);
    skillMenuInfo.item_medium.generic.id = 12i32;
    skillMenuInfo.item_medium.string =
        b"Hurt Me Plenty\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.item_medium.color = color_red.as_mut_ptr();
    skillMenuInfo.item_medium.style = 0x1i32;
    skillMenuInfo.item_hard.generic.type_0 = 9i32;
    skillMenuInfo.item_hard.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    skillMenuInfo.item_hard.generic.x = 320i32;
    skillMenuInfo.item_hard.generic.y = 255i32;
    skillMenuInfo.item_hard.generic.callback = Some(UI_SPSkillMenu_SkillEvent);
    skillMenuInfo.item_hard.generic.id = 13i32;
    skillMenuInfo.item_hard.string =
        b"Hardcore\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.item_hard.color = color_red.as_mut_ptr();
    skillMenuInfo.item_hard.style = 0x1i32;
    skillMenuInfo.item_nightmare.generic.type_0 = 9i32;
    skillMenuInfo.item_nightmare.generic.flags = 0x8i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    skillMenuInfo.item_nightmare.generic.x = 320i32;
    skillMenuInfo.item_nightmare.generic.y = 283i32;
    skillMenuInfo.item_nightmare.generic.callback = Some(UI_SPSkillMenu_SkillEvent);
    skillMenuInfo.item_nightmare.generic.id = 14i32;
    skillMenuInfo.item_nightmare.string =
        b"NIGHTMARE!\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.item_nightmare.color = color_red.as_mut_ptr();
    skillMenuInfo.item_nightmare.style = 0x1i32;
    skillMenuInfo.item_back.generic.type_0 = 6i32;
    skillMenuInfo.item_back.generic.name =
        b"menu/art/back_0.tga\x00" as *const u8 as *const libc::c_char;
    skillMenuInfo.item_back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    skillMenuInfo.item_back.generic.x = 0i32;
    skillMenuInfo.item_back.generic.y = 480i32 - 64i32;
    skillMenuInfo.item_back.generic.callback = Some(UI_SPSkillMenu_BackEvent);
    skillMenuInfo.item_back.generic.id = 15i32;
    skillMenuInfo.item_back.width = 128i32;
    skillMenuInfo.item_back.height = 64i32;
    skillMenuInfo.item_back.focuspic =
        b"menu/art/back_1.tga\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    skillMenuInfo.art_skillPic.generic.type_0 = 6i32;
    skillMenuInfo.art_skillPic.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    skillMenuInfo.art_skillPic.generic.x = 320i32 - 64i32;
    skillMenuInfo.art_skillPic.generic.y = 368i32;
    skillMenuInfo.art_skillPic.width = 128i32;
    skillMenuInfo.art_skillPic.height = 96i32;
    skillMenuInfo.item_fight.generic.type_0 = 6i32;
    skillMenuInfo.item_fight.generic.name =
        b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    skillMenuInfo.item_fight.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    skillMenuInfo.item_fight.generic.callback = Some(UI_SPSkillMenu_FightEvent);
    skillMenuInfo.item_fight.generic.id = 16i32;
    skillMenuInfo.item_fight.generic.x = 640i32;
    skillMenuInfo.item_fight.generic.y = 480i32 - 64i32;
    skillMenuInfo.item_fight.width = 128i32;
    skillMenuInfo.item_fight.height = 64i32;
    skillMenuInfo.item_fight.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.art_frame as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.art_banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.item_baby as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.item_easy as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.item_medium as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.item_hard as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.item_nightmare as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.art_skillPic as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.item_back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut skillMenuInfo.menu,
        &mut skillMenuInfo.item_fight as *mut menubitmap_s as *mut libc::c_void,
    );
    skill = Com_Clamp(
        1i32 as libc::c_float,
        5i32 as libc::c_float,
        trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const libc::c_char),
    ) as libc::c_int;
    SetSkillColor(skill, color_white.as_mut_ptr());
    skillMenuInfo.art_skillPic.shader = skillMenuInfo.skillpics[(skill - 1i32) as usize];
    if skill == 5i32 {
        trap_S_StartLocalSound(skillMenuInfo.nightmareSound, CHAN_ANNOUNCER as libc::c_int);
    };
}
unsafe extern "C" fn SetSkillColor(mut skill: libc::c_int, mut color: *mut vec_t) {
    match skill {
        1 => skillMenuInfo.item_baby.color = color,
        2 => skillMenuInfo.item_easy.color = color,
        3 => skillMenuInfo.item_medium.color = color,
        4 => skillMenuInfo.item_hard.color = color,
        5 => skillMenuInfo.item_nightmare.color = color,
        _ => {}
    };
}
/*
=================
UI_SPSkillMenu_FightEvent
=================
*/
unsafe extern "C" fn UI_SPSkillMenu_FightEvent(
    mut _ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3i32 {
        return;
    }
    UI_SPArena_Start(skillMenuInfo.arenaInfo);
}
/*
=================
UI_SPSkillMenu_BackEvent
=================
*/
unsafe extern "C" fn UI_SPSkillMenu_BackEvent(
    mut _ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3i32 {
        return;
    }
    trap_S_StartLocalSound(skillMenuInfo.silenceSound, CHAN_ANNOUNCER as libc::c_int);
    UI_PopMenu();
}
/*
=================
UI_SPSkillMenu_SkillEvent
=================
*/
unsafe extern "C" fn UI_SPSkillMenu_SkillEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    let mut id: libc::c_int = 0;
    let mut skill: libc::c_int = 0;
    if notification != 3i32 {
        return;
    }
    SetSkillColor(
        trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const libc::c_char)
            as libc::c_int,
        color_red.as_mut_ptr(),
    );
    id = (*(ptr as *mut menucommon_s)).id;
    skill = id - 10i32 + 1i32;
    trap_Cvar_SetValue(
        b"g_spSkill\x00" as *const u8 as *const libc::c_char,
        skill as libc::c_float,
    );
    SetSkillColor(skill, color_white.as_mut_ptr());
    skillMenuInfo.art_skillPic.shader = skillMenuInfo.skillpics[(skill - 1i32) as usize];
    if id == 14i32 {
        trap_S_StartLocalSound(skillMenuInfo.nightmareSound, CHAN_ANNOUNCER as libc::c_int);
    } else {
        trap_S_StartLocalSound(skillMenuInfo.silenceSound, CHAN_ANNOUNCER as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_SPSkillMenu_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/cut_frame\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0.tga\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1.tga\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char);
    skillMenuInfo.skillpics[0usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete1\x00" as *const u8 as *const libc::c_char,
    );
    skillMenuInfo.skillpics[1usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete2\x00" as *const u8 as *const libc::c_char,
    );
    skillMenuInfo.skillpics[2usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete3\x00" as *const u8 as *const libc::c_char,
    );
    skillMenuInfo.skillpics[3usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete4\x00" as *const u8 as *const libc::c_char,
    );
    skillMenuInfo.skillpics[4usize] = trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete5\x00" as *const u8 as *const libc::c_char,
    );
    skillMenuInfo.nightmareSound = trap_S_RegisterSound(
        b"sound/misc/nightmare.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
    skillMenuInfo.silenceSound = trap_S_RegisterSound(
        b"sound/misc/silence.wav\x00" as *const u8 as *const libc::c_char,
        qfalse,
    );
}
/*
=================
UI_SPSkillMenu_Key
=================
*/
unsafe extern "C" fn UI_SPSkillMenu_Key(mut key: libc::c_int) -> sfxHandle_t {
    if key == K_MOUSE2 as libc::c_int || key == K_ESCAPE as libc::c_int {
        trap_S_StartLocalSound(skillMenuInfo.silenceSound, CHAN_ANNOUNCER as libc::c_int);
    }
    return Menu_DefaultKey(&mut skillMenuInfo.menu, key);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skillMenuInfo_t {
    pub menu: menuframework_s,
    pub art_frame: menubitmap_s,
    pub art_banner: menutext_s,
    pub item_baby: menutext_s,
    pub item_easy: menutext_s,
    pub item_medium: menutext_s,
    pub item_hard: menutext_s,
    pub item_nightmare: menutext_s,
    pub art_skillPic: menubitmap_s,
    pub item_back: menubitmap_s,
    pub item_fight: menubitmap_s,
    pub arenaInfo: *const libc::c_char,
    pub skillpics: [qhandle_t; 5],
    pub nightmareSound: sfxHandle_t,
    pub silenceSound: sfxHandle_t,
}
