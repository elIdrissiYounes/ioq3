use bg_public_h::{
    animation_s, animation_t, bg_itemlist, gitem_s, gitem_t, itemType_t, unnamed_1, weapon_t,
    BOTH_DEAD1, BOTH_DEAD2, BOTH_DEAD3, BOTH_DEATH1, BOTH_DEATH2, BOTH_DEATH3, FLAG_RUN,
    FLAG_STAND, FLAG_STAND2RUN, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE,
    IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, LEGS_BACK, LEGS_BACKCR, LEGS_BACKWALK,
    LEGS_IDLE, LEGS_IDLECR, LEGS_JUMP, LEGS_JUMPB, LEGS_LAND, LEGS_LANDB, LEGS_RUN, LEGS_SWIM,
    LEGS_TURN, LEGS_WALK, LEGS_WALKCR, MAX_ANIMATIONS, MAX_TOTALANIMATIONS, TORSO_AFFIRMATIVE,
    TORSO_ATTACK, TORSO_ATTACK2, TORSO_DROP, TORSO_FOLLOWME, TORSO_GESTURE, TORSO_GETFLAG,
    TORSO_GUARDBASE, TORSO_NEGATIVE, TORSO_PATROL, TORSO_RAISE, TORSO_STAND, TORSO_STAND2, WP_BFG,
    WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE,
    WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
};
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
    byte, colorWhite, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, vec3_t, vec4_t,
    vec_t, Q_CleanStr, Q_stricmp, Q_strupr, EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::{fabs, memset, strcat, strcmp, strcpy};
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
use ui_cinematics::{UI_CinematicsMenu, UI_CinematicsMenu_Cache, UI_CinematicsMenu_f};
use ui_confirm::{ConfirmMenu_Cache, UI_ConfirmMenu, UI_ConfirmMenu_Style, UI_Message};
use ui_connect::UI_DrawConnectScreen;
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
    _tag_menuframework, lerpFrame_t, menuaction_s, menubitmap_s, menucommon_s, menuframework_s,
    menuradiobutton_s, menuslider_s, menutext_s, playerInfo_t, trap_Cmd_ExecuteText,
    trap_Cvar_Reset, trap_Cvar_SetValue, trap_Cvar_VariableStringBuffer, trap_Cvar_VariableValue,
    trap_Key_GetBindingBuf, trap_Key_KeynumToStringBuf, trap_Key_SetBinding, trap_R_RegisterModel,
    trap_R_RegisterShaderNoMip, uiStatic_t,
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
// ui_controls2.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_ControlsMenu() {
    Controls_MenuInit();
    UI_PushMenu(&mut s_controls.menu);
}
static mut s_controls: controls_t = controls_t {
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
    player: menubitmap_s {
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
    movement: menutext_s {
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
    looking: menutext_s {
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
    weapons: menutext_s {
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
    misc: menutext_s {
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
    walkforward: menuaction_s {
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
    },
    backpedal: menuaction_s {
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
    },
    stepleft: menuaction_s {
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
    },
    stepright: menuaction_s {
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
    },
    moveup: menuaction_s {
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
    },
    movedown: menuaction_s {
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
    },
    turnleft: menuaction_s {
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
    },
    turnright: menuaction_s {
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
    },
    sidestep: menuaction_s {
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
    },
    run: menuaction_s {
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
    },
    machinegun: menuaction_s {
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
    },
    chainsaw: menuaction_s {
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
    },
    shotgun: menuaction_s {
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
    },
    grenadelauncher: menuaction_s {
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
    },
    rocketlauncher: menuaction_s {
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
    },
    lightning: menuaction_s {
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
    },
    railgun: menuaction_s {
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
    },
    plasma: menuaction_s {
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
    },
    bfg: menuaction_s {
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
    },
    attack: menuaction_s {
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
    },
    prevweapon: menuaction_s {
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
    },
    nextweapon: menuaction_s {
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
    },
    lookup: menuaction_s {
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
    },
    lookdown: menuaction_s {
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
    },
    mouselook: menuaction_s {
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
    },
    freelook: menuradiobutton_s {
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
    centerview: menuaction_s {
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
    },
    zoomview: menuaction_s {
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
    },
    gesture: menuaction_s {
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
    },
    invertmouse: menuradiobutton_s {
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
    sensitivity: menuslider_s {
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
    smoothmouse: menuradiobutton_s {
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
    alwaysrun: menuradiobutton_s {
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
    showscores: menuaction_s {
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
    },
    autoswitch: menuradiobutton_s {
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
    useitem: menuaction_s {
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
    },
    playerinfo: playerInfo_t {
        legsModel: 0,
        legsSkin: 0,
        legs: lerpFrame_t {
            oldFrame: 0,
            oldFrameTime: 0,
            frame: 0,
            frameTime: 0,
            backlerp: 0.,
            yawAngle: 0.,
            yawing: qfalse,
            pitchAngle: 0.,
            pitching: qfalse,
            animationNumber: 0,
            animation: 0 as *const animation_t as *mut animation_t,
            animationTime: 0,
        },
        torsoModel: 0,
        torsoSkin: 0,
        torso: lerpFrame_t {
            oldFrame: 0,
            oldFrameTime: 0,
            frame: 0,
            frameTime: 0,
            backlerp: 0.,
            yawAngle: 0.,
            yawing: qfalse,
            pitchAngle: 0.,
            pitching: qfalse,
            animationNumber: 0,
            animation: 0 as *const animation_t as *mut animation_t,
            animationTime: 0,
        },
        headModel: 0,
        headSkin: 0,
        animations: [animation_s {
            firstFrame: 0,
            numFrames: 0,
            loopFrames: 0,
            frameLerp: 0,
            initialLerp: 0,
            reversed: 0,
            flipflop: 0,
        }; 31],
        fixedlegs: qfalse,
        fixedtorso: qfalse,
        weaponModel: 0,
        barrelModel: 0,
        flashModel: 0,
        flashDlightColor: [0.; 3],
        muzzleFlashTime: 0,
        color1: [0.; 3],
        c1RGBA: [0; 4],
        viewAngles: [0.; 3],
        moveAngles: [0.; 3],
        currentWeapon: WP_NONE,
        legsAnim: 0,
        torsoAnim: 0,
        weapon: WP_NONE,
        lastWeapon: WP_NONE,
        pendingWeapon: WP_NONE,
        weaponTimer: 0,
        pendingLegsAnim: 0,
        torsoAnimationTimer: 0,
        pendingTorsoAnim: 0,
        legsAnimationTimer: 0,
        chat: qfalse,
        newModel: qfalse,
        barrelSpinning: qfalse,
        barrelAngle: 0.,
        barrelTime: 0,
        realWeapon: 0,
    },
    changesmade: qfalse,
    chat: menuaction_s {
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
    },
    chat2: menuaction_s {
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
    },
    chat3: menuaction_s {
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
    },
    chat4: menuaction_s {
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
    },
    togglemenu: menuaction_s {
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
    },
    joyenable: menuradiobutton_s {
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
    joythreshold: menuslider_s {
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
    section: 0,
    waitingforkey: qfalse,
    playerModel: [0; 64],
    playerViewangles: [0.; 3],
    playerMoveangles: [0.; 3],
    playerLegs: 0,
    playerTorso: 0,
    playerWeapon: WP_NONE,
    playerChat: qfalse,
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
    name: menutext_s {
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
};
/*
=================
Controls_MenuInit
=================
*/
unsafe extern "C" fn Controls_MenuInit() {
    static mut playername: [libc::c_char; 32] = [0; 32];
    memset(
        &mut s_controls as *mut controls_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<controls_t>() as libc::c_ulong,
    );
    Controls_Cache();
    s_controls.menu.key = Some(Controls_MenuKey);
    s_controls.menu.wrapAround = qtrue;
    s_controls.menu.fullscreen = qtrue;
    s_controls.banner.generic.type_0 = 10i32;
    s_controls.banner.generic.flags = 0x8i32 as libc::c_uint;
    s_controls.banner.generic.x = 320i32;
    s_controls.banner.generic.y = 16i32;
    s_controls.banner.string =
        b"CONTROLS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.banner.color = color_white.as_mut_ptr();
    s_controls.banner.style = 0x1i32;
    s_controls.framel.generic.type_0 = 6i32;
    s_controls.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_controls.framel.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_controls.framel.generic.x = 0i32;
    s_controls.framel.generic.y = 78i32;
    s_controls.framel.width = 256i32;
    s_controls.framel.height = 329i32;
    s_controls.framer.generic.type_0 = 6i32;
    s_controls.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_controls.framer.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_controls.framer.generic.x = 376i32;
    s_controls.framer.generic.y = 76i32;
    s_controls.framer.width = 256i32;
    s_controls.framer.height = 334i32;
    s_controls.looking.generic.type_0 = 9i32;
    s_controls.looking.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_controls.looking.generic.id = 101i32;
    s_controls.looking.generic.callback = Some(Controls_MenuEvent);
    s_controls.looking.generic.x = 152i32;
    s_controls.looking.generic.y = 240i32 - 2i32 * 27i32;
    s_controls.looking.string =
        b"LOOK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.looking.style = 0x2i32;
    s_controls.looking.color = color_red.as_mut_ptr();
    s_controls.movement.generic.type_0 = 9i32;
    s_controls.movement.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_controls.movement.generic.id = 100i32;
    s_controls.movement.generic.callback = Some(Controls_MenuEvent);
    s_controls.movement.generic.x = 152i32;
    s_controls.movement.generic.y = 240i32 - 27i32;
    s_controls.movement.string =
        b"MOVE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.movement.style = 0x2i32;
    s_controls.movement.color = color_red.as_mut_ptr();
    s_controls.weapons.generic.type_0 = 9i32;
    s_controls.weapons.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_controls.weapons.generic.id = 102i32;
    s_controls.weapons.generic.callback = Some(Controls_MenuEvent);
    s_controls.weapons.generic.x = 152i32;
    s_controls.weapons.generic.y = 240i32;
    s_controls.weapons.string =
        b"SHOOT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.weapons.style = 0x2i32;
    s_controls.weapons.color = color_red.as_mut_ptr();
    s_controls.misc.generic.type_0 = 9i32;
    s_controls.misc.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_controls.misc.generic.id = 103i32;
    s_controls.misc.generic.callback = Some(Controls_MenuEvent);
    s_controls.misc.generic.x = 152i32;
    s_controls.misc.generic.y = 240i32 + 27i32;
    s_controls.misc.string = b"MISC\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.misc.style = 0x2i32;
    s_controls.misc.color = color_red.as_mut_ptr();
    s_controls.back.generic.type_0 = 6i32;
    s_controls.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_controls.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_controls.back.generic.x = 0i32;
    s_controls.back.generic.y = 480i32 - 64i32;
    s_controls.back.generic.id = 105i32;
    s_controls.back.generic.callback = Some(Controls_MenuEvent);
    s_controls.back.width = 128i32;
    s_controls.back.height = 64i32;
    s_controls.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_controls.player.generic.type_0 = 6i32;
    s_controls.player.generic.flags = 0x4000i32 as libc::c_uint;
    s_controls.player.generic.ownerdraw = Some(Controls_DrawPlayer);
    s_controls.player.generic.x = 400i32;
    s_controls.player.generic.y = -40i32;
    s_controls.player.width = 32i32 * 10i32;
    s_controls.player.height = 56i32 * 10i32;
    s_controls.walkforward.generic.type_0 = 2i32;
    s_controls.walkforward.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.walkforward.generic.callback = Some(Controls_ActionEvent);
    s_controls.walkforward.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.walkforward.generic.id = 3i32;
    s_controls.backpedal.generic.type_0 = 2i32;
    s_controls.backpedal.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.backpedal.generic.callback = Some(Controls_ActionEvent);
    s_controls.backpedal.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.backpedal.generic.id = 4i32;
    s_controls.stepleft.generic.type_0 = 2i32;
    s_controls.stepleft.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.stepleft.generic.callback = Some(Controls_ActionEvent);
    s_controls.stepleft.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.stepleft.generic.id = 5i32;
    s_controls.stepright.generic.type_0 = 2i32;
    s_controls.stepright.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.stepright.generic.callback = Some(Controls_ActionEvent);
    s_controls.stepright.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.stepright.generic.id = 6i32;
    s_controls.moveup.generic.type_0 = 2i32;
    s_controls.moveup.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.moveup.generic.callback = Some(Controls_ActionEvent);
    s_controls.moveup.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.moveup.generic.id = 7i32;
    s_controls.movedown.generic.type_0 = 2i32;
    s_controls.movedown.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.movedown.generic.callback = Some(Controls_ActionEvent);
    s_controls.movedown.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.movedown.generic.id = 8i32;
    s_controls.turnleft.generic.type_0 = 2i32;
    s_controls.turnleft.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.turnleft.generic.callback = Some(Controls_ActionEvent);
    s_controls.turnleft.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.turnleft.generic.id = 9i32;
    s_controls.turnright.generic.type_0 = 2i32;
    s_controls.turnright.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.turnright.generic.callback = Some(Controls_ActionEvent);
    s_controls.turnright.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.turnright.generic.id = 10i32;
    s_controls.sidestep.generic.type_0 = 2i32;
    s_controls.sidestep.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.sidestep.generic.callback = Some(Controls_ActionEvent);
    s_controls.sidestep.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.sidestep.generic.id = 11i32;
    s_controls.run.generic.type_0 = 2i32;
    s_controls.run.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.run.generic.callback = Some(Controls_ActionEvent);
    s_controls.run.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.run.generic.id = 2i32;
    s_controls.chainsaw.generic.type_0 = 2i32;
    s_controls.chainsaw.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.chainsaw.generic.callback = Some(Controls_ActionEvent);
    s_controls.chainsaw.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.chainsaw.generic.id = 17i32;
    s_controls.machinegun.generic.type_0 = 2i32;
    s_controls.machinegun.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.machinegun.generic.callback = Some(Controls_ActionEvent);
    s_controls.machinegun.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.machinegun.generic.id = 18i32;
    s_controls.shotgun.generic.type_0 = 2i32;
    s_controls.shotgun.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.shotgun.generic.callback = Some(Controls_ActionEvent);
    s_controls.shotgun.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.shotgun.generic.id = 19i32;
    s_controls.grenadelauncher.generic.type_0 = 2i32;
    s_controls.grenadelauncher.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.grenadelauncher.generic.callback = Some(Controls_ActionEvent);
    s_controls.grenadelauncher.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.grenadelauncher.generic.id = 20i32;
    s_controls.rocketlauncher.generic.type_0 = 2i32;
    s_controls.rocketlauncher.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.rocketlauncher.generic.callback = Some(Controls_ActionEvent);
    s_controls.rocketlauncher.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.rocketlauncher.generic.id = 21i32;
    s_controls.lightning.generic.type_0 = 2i32;
    s_controls.lightning.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.lightning.generic.callback = Some(Controls_ActionEvent);
    s_controls.lightning.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.lightning.generic.id = 22i32;
    s_controls.railgun.generic.type_0 = 2i32;
    s_controls.railgun.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.railgun.generic.callback = Some(Controls_ActionEvent);
    s_controls.railgun.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.railgun.generic.id = 23i32;
    s_controls.plasma.generic.type_0 = 2i32;
    s_controls.plasma.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.plasma.generic.callback = Some(Controls_ActionEvent);
    s_controls.plasma.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.plasma.generic.id = 24i32;
    s_controls.bfg.generic.type_0 = 2i32;
    s_controls.bfg.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.bfg.generic.callback = Some(Controls_ActionEvent);
    s_controls.bfg.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.bfg.generic.id = 25i32;
    s_controls.attack.generic.type_0 = 2i32;
    s_controls.attack.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.attack.generic.callback = Some(Controls_ActionEvent);
    s_controls.attack.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.attack.generic.id = 26i32;
    s_controls.prevweapon.generic.type_0 = 2i32;
    s_controls.prevweapon.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.prevweapon.generic.callback = Some(Controls_ActionEvent);
    s_controls.prevweapon.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.prevweapon.generic.id = 27i32;
    s_controls.nextweapon.generic.type_0 = 2i32;
    s_controls.nextweapon.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.nextweapon.generic.callback = Some(Controls_ActionEvent);
    s_controls.nextweapon.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.nextweapon.generic.id = 28i32;
    s_controls.lookup.generic.type_0 = 2i32;
    s_controls.lookup.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.lookup.generic.callback = Some(Controls_ActionEvent);
    s_controls.lookup.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.lookup.generic.id = 12i32;
    s_controls.lookdown.generic.type_0 = 2i32;
    s_controls.lookdown.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.lookdown.generic.callback = Some(Controls_ActionEvent);
    s_controls.lookdown.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.lookdown.generic.id = 13i32;
    s_controls.mouselook.generic.type_0 = 2i32;
    s_controls.mouselook.generic.flags = 0x4i32 as libc::c_uint
        | 0x80i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.mouselook.generic.callback = Some(Controls_ActionEvent);
    s_controls.mouselook.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.mouselook.generic.id = 14i32;
    s_controls.freelook.generic.type_0 = 5i32;
    s_controls.freelook.generic.flags = 0x2i32 as libc::c_uint;
    s_controls.freelook.generic.x = 640i32 / 2i32;
    s_controls.freelook.generic.name = b"free look\x00" as *const u8 as *const libc::c_char;
    s_controls.freelook.generic.id = 35i32;
    s_controls.freelook.generic.callback = Some(Controls_MenuEvent);
    s_controls.freelook.generic.statusbar = Some(Controls_StatusBar);
    s_controls.centerview.generic.type_0 = 2i32;
    s_controls.centerview.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.centerview.generic.callback = Some(Controls_ActionEvent);
    s_controls.centerview.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.centerview.generic.id = 15i32;
    s_controls.zoomview.generic.type_0 = 2i32;
    s_controls.zoomview.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.zoomview.generic.callback = Some(Controls_ActionEvent);
    s_controls.zoomview.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.zoomview.generic.id = 16i32;
    s_controls.useitem.generic.type_0 = 2i32;
    s_controls.useitem.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.useitem.generic.callback = Some(Controls_ActionEvent);
    s_controls.useitem.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.useitem.generic.id = 1i32;
    s_controls.showscores.generic.type_0 = 2i32;
    s_controls.showscores.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.showscores.generic.callback = Some(Controls_ActionEvent);
    s_controls.showscores.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.showscores.generic.id = 0i32;
    s_controls.invertmouse.generic.type_0 = 5i32;
    s_controls.invertmouse.generic.flags = 0x2i32 as libc::c_uint;
    s_controls.invertmouse.generic.x = 640i32 / 2i32;
    s_controls.invertmouse.generic.name = b"invert mouse\x00" as *const u8 as *const libc::c_char;
    s_controls.invertmouse.generic.id = 36i32;
    s_controls.invertmouse.generic.callback = Some(Controls_MenuEvent);
    s_controls.invertmouse.generic.statusbar = Some(Controls_StatusBar);
    s_controls.smoothmouse.generic.type_0 = 5i32;
    s_controls.smoothmouse.generic.flags = 0x2i32 as libc::c_uint;
    s_controls.smoothmouse.generic.x = 640i32 / 2i32;
    s_controls.smoothmouse.generic.name = b"smooth mouse\x00" as *const u8 as *const libc::c_char;
    s_controls.smoothmouse.generic.id = 42i32;
    s_controls.smoothmouse.generic.callback = Some(Controls_MenuEvent);
    s_controls.smoothmouse.generic.statusbar = Some(Controls_StatusBar);
    s_controls.alwaysrun.generic.type_0 = 5i32;
    s_controls.alwaysrun.generic.flags = 0x2i32 as libc::c_uint;
    s_controls.alwaysrun.generic.x = 640i32 / 2i32;
    s_controls.alwaysrun.generic.name = b"always run\x00" as *const u8 as *const libc::c_char;
    s_controls.alwaysrun.generic.id = 37i32;
    s_controls.alwaysrun.generic.callback = Some(Controls_MenuEvent);
    s_controls.alwaysrun.generic.statusbar = Some(Controls_StatusBar);
    s_controls.autoswitch.generic.type_0 = 5i32;
    s_controls.autoswitch.generic.flags = 0x2i32 as libc::c_uint;
    s_controls.autoswitch.generic.x = 640i32 / 2i32;
    s_controls.autoswitch.generic.name =
        b"autoswitch weapons\x00" as *const u8 as *const libc::c_char;
    s_controls.autoswitch.generic.id = 38i32;
    s_controls.autoswitch.generic.callback = Some(Controls_MenuEvent);
    s_controls.autoswitch.generic.statusbar = Some(Controls_StatusBar);
    s_controls.sensitivity.generic.type_0 = 1i32;
    s_controls.sensitivity.generic.x = 640i32 / 2i32;
    s_controls.sensitivity.generic.flags = 0x2i32 as libc::c_uint;
    s_controls.sensitivity.generic.name = b"mouse speed\x00" as *const u8 as *const libc::c_char;
    s_controls.sensitivity.generic.id = 39i32;
    s_controls.sensitivity.generic.callback = Some(Controls_MenuEvent);
    s_controls.sensitivity.minvalue = 2i32 as libc::c_float;
    s_controls.sensitivity.maxvalue = 30i32 as libc::c_float;
    s_controls.sensitivity.generic.statusbar = Some(Controls_StatusBar);
    s_controls.gesture.generic.type_0 = 2i32;
    s_controls.gesture.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.gesture.generic.callback = Some(Controls_ActionEvent);
    s_controls.gesture.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.gesture.generic.id = 29i32;
    s_controls.chat.generic.type_0 = 2i32;
    s_controls.chat.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.chat.generic.callback = Some(Controls_ActionEvent);
    s_controls.chat.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.chat.generic.id = 30i32;
    s_controls.chat2.generic.type_0 = 2i32;
    s_controls.chat2.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.chat2.generic.callback = Some(Controls_ActionEvent);
    s_controls.chat2.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.chat2.generic.id = 31i32;
    s_controls.chat3.generic.type_0 = 2i32;
    s_controls.chat3.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.chat3.generic.callback = Some(Controls_ActionEvent);
    s_controls.chat3.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.chat3.generic.id = 32i32;
    s_controls.chat4.generic.type_0 = 2i32;
    s_controls.chat4.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.chat4.generic.callback = Some(Controls_ActionEvent);
    s_controls.chat4.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.chat4.generic.id = 33i32;
    s_controls.togglemenu.generic.type_0 = 2i32;
    s_controls.togglemenu.generic.flags = 0x4i32 as libc::c_uint
        | 0x100i32 as libc::c_uint
        | 0x2000i32 as libc::c_uint
        | 0x1000i32 as libc::c_uint;
    s_controls.togglemenu.generic.callback = Some(Controls_ActionEvent);
    s_controls.togglemenu.generic.ownerdraw = Some(Controls_DrawKeyBinding);
    s_controls.togglemenu.generic.id = 34i32;
    s_controls.joyenable.generic.type_0 = 5i32;
    s_controls.joyenable.generic.flags = 0x2i32 as libc::c_uint;
    s_controls.joyenable.generic.x = 640i32 / 2i32;
    s_controls.joyenable.generic.name = b"joystick\x00" as *const u8 as *const libc::c_char;
    s_controls.joyenable.generic.id = 40i32;
    s_controls.joyenable.generic.callback = Some(Controls_MenuEvent);
    s_controls.joyenable.generic.statusbar = Some(Controls_StatusBar);
    s_controls.joythreshold.generic.type_0 = 1i32;
    s_controls.joythreshold.generic.x = 640i32 / 2i32;
    s_controls.joythreshold.generic.flags = 0x2i32 as libc::c_uint;
    s_controls.joythreshold.generic.name =
        b"joystick threshold\x00" as *const u8 as *const libc::c_char;
    s_controls.joythreshold.generic.id = 41i32;
    s_controls.joythreshold.generic.callback = Some(Controls_MenuEvent);
    s_controls.joythreshold.minvalue = 0.05f32;
    s_controls.joythreshold.maxvalue = 0.75f32;
    s_controls.joythreshold.generic.statusbar = Some(Controls_StatusBar);
    s_controls.name.generic.type_0 = 9i32;
    s_controls.name.generic.flags = 0x8i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_controls.name.generic.x = 320i32;
    s_controls.name.generic.y = 440i32;
    s_controls.name.string = playername.as_mut_ptr();
    s_controls.name.style = 0x1i32;
    s_controls.name.color = text_color_normal.as_mut_ptr();
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.player as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.name as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.looking as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.movement as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.weapons as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.misc as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.sensitivity as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.smoothmouse as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.invertmouse as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.lookup as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.lookdown as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.mouselook as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.freelook as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.centerview as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.zoomview as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.joyenable as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.joythreshold as *mut menuslider_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.alwaysrun as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.run as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.walkforward as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.backpedal as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.stepleft as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.stepright as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.moveup as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.movedown as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.turnleft as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.turnright as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.sidestep as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.attack as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.nextweapon as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.prevweapon as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.autoswitch as *mut menuradiobutton_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.chainsaw as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.machinegun as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.shotgun as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.grenadelauncher as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.rocketlauncher as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.lightning as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.railgun as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.plasma as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.bfg as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.showscores as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.useitem as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.gesture as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.chat as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.chat2 as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.chat3 as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.chat4 as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.togglemenu as *mut menuaction_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_controls.menu,
        &mut s_controls.back as *mut menubitmap_s as *mut libc::c_void,
    );
    trap_Cvar_VariableStringBuffer(
        b"name\x00" as *const u8 as *const libc::c_char,
        s_controls.name.string,
        16i32,
    );
    Q_CleanStr(s_controls.name.string);
    Controls_InitCvars();
    Controls_GetConfig();
    Controls_InitModel();
    Controls_InitWeapons();
    s_controls.section = 1i32;
    Controls_Update();
}
/*
=================
Controls_Update
=================
*/
unsafe extern "C" fn Controls_Update() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut controls: *mut *mut menucommon_s = 0 as *mut *mut menucommon_s;
    let mut control: *mut menucommon_s = 0 as *mut menucommon_s;
    i = 0i32;
    while i < 4i32 {
        controls = g_controls[i as usize];
        j = 0i32;
        loop {
            control = *controls.offset(j as isize);
            if control.is_null() {
                break;
            }
            (*control).flags |= 0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
            j += 1
        }
        i += 1
    }
    controls = g_controls[s_controls.section as usize];
    j = 0i32;
    loop {
        control = *controls.offset(j as isize);
        if control.is_null() {
            break;
        }
        (*control).flags &=
            !(0x2000i32 as libc::c_uint | 0x1000i32 as libc::c_uint | 0x4000i32 as libc::c_uint);
        j += 1
    }
    y = (480i32 - j * 16i32) / 2i32;
    j = 0i32;
    loop {
        control = *controls.offset(j as isize);
        if control.is_null() {
            break;
        }
        (*control).x = 320i32;
        (*control).y = y;
        (*control).left = 320i32 - 19i32 * 8i32;
        (*control).right = 320i32 + 21i32 * 8i32;
        (*control).top = y;
        (*control).bottom = y + 16i32;
        j += 1;
        y += 16i32
    }
    if 0 != s_controls.waitingforkey as u64 {
        i = 0i32;
        while i < s_controls.menu.nitems {
            (*(s_controls.menu.items[i as usize] as *mut menucommon_s)).flags |=
                0x2000i32 as libc::c_uint;
            i += 1
        }
        (*(s_controls.menu.items[s_controls.menu.cursor as usize] as *mut menucommon_s)).flags &=
            !(0x2000i32 as libc::c_uint);
        s_controls.name.generic.flags &= !(0x2000i32 as libc::c_uint);
        return;
    }
    i = 0i32;
    while i < s_controls.menu.nitems {
        (*(s_controls.menu.items[i as usize] as *mut menucommon_s)).flags &=
            !(0x2000i32 as libc::c_uint);
        i += 1
    }
    s_controls.looking.generic.flags &=
        !(0x2000i32 as libc::c_uint | 0x40i32 as libc::c_uint | 0x80i32 as libc::c_uint);
    s_controls.movement.generic.flags &=
        !(0x2000i32 as libc::c_uint | 0x40i32 as libc::c_uint | 0x80i32 as libc::c_uint);
    s_controls.weapons.generic.flags &=
        !(0x2000i32 as libc::c_uint | 0x40i32 as libc::c_uint | 0x80i32 as libc::c_uint);
    s_controls.misc.generic.flags &=
        !(0x2000i32 as libc::c_uint | 0x40i32 as libc::c_uint | 0x80i32 as libc::c_uint);
    s_controls.looking.generic.flags |= 0x100i32 as libc::c_uint;
    s_controls.movement.generic.flags |= 0x100i32 as libc::c_uint;
    s_controls.weapons.generic.flags |= 0x100i32 as libc::c_uint;
    s_controls.misc.generic.flags |= 0x100i32 as libc::c_uint;
    match s_controls.section {
        0 => {
            s_controls.movement.generic.flags &= !(0x100i32 as libc::c_uint);
            s_controls.movement.generic.flags |= 0x40i32 as libc::c_uint | 0x80i32 as libc::c_uint
        }
        1 => {
            s_controls.looking.generic.flags &= !(0x100i32 as libc::c_uint);
            s_controls.looking.generic.flags |= 0x40i32 as libc::c_uint | 0x80i32 as libc::c_uint
        }
        2 => {
            s_controls.weapons.generic.flags &= !(0x100i32 as libc::c_uint);
            s_controls.weapons.generic.flags |= 0x40i32 as libc::c_uint | 0x80i32 as libc::c_uint
        }
        3 => {
            s_controls.misc.generic.flags &= !(0x100i32 as libc::c_uint);
            s_controls.misc.generic.flags |= 0x40i32 as libc::c_uint | 0x80i32 as libc::c_uint
        }
        _ => {}
    };
}
static mut g_controls: [*mut *mut menucommon_s; 4] = unsafe {
    [
        g_movement_controls.as_ptr() as *mut _,
        g_looking_controls.as_ptr() as *mut _,
        g_weapons_controls.as_ptr() as *mut _,
        g_misc_controls.as_ptr() as *mut _,
    ]
};
// Initialized in run_static_initializers
static mut g_misc_controls: [*mut menucommon_s; 9] =
    [0 as *const menucommon_s as *mut menucommon_s; 9];
// Initialized in run_static_initializers
static mut g_weapons_controls: [*mut menucommon_s; 14] =
    [0 as *const menucommon_s as *mut menucommon_s; 14];
// Initialized in run_static_initializers
static mut g_looking_controls: [*mut menucommon_s; 12] =
    [0 as *const menucommon_s as *mut menucommon_s; 12];
// Initialized in run_static_initializers
static mut g_movement_controls: [*mut menucommon_s; 12] =
    [0 as *const menucommon_s as *mut menucommon_s; 12];
/*
=================
Controls_InitWeapons
=================
*/
unsafe extern "C" fn Controls_InitWeapons() {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    item = bg_itemlist.as_mut_ptr().offset(1isize);
    while !(*item).classname.is_null() {
        if !((*item).giType as libc::c_uint != IT_WEAPON as libc::c_int as libc::c_uint) {
            trap_R_RegisterModel((*item).world_model[0usize]);
        }
        item = item.offset(1isize)
    }
}
/*
=================
Controls_InitModel
=================
*/
unsafe extern "C" fn Controls_InitModel() {
    memset(
        &mut s_controls.playerinfo as *mut playerInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<playerInfo_t>() as libc::c_ulong,
    );
    UI_PlayerInfo_SetModel(
        &mut s_controls.playerinfo,
        UI_Cvar_VariableString(b"model\x00" as *const u8 as *const libc::c_char),
    );
    Controls_UpdateModel(0i32);
}
/*
=================
Controls_UpdateModel
=================
*/
unsafe extern "C" fn Controls_UpdateModel(mut anim: libc::c_int) {
    s_controls.playerViewangles[2usize] = 0i32 as vec_t;
    s_controls.playerViewangles[1usize] = s_controls.playerViewangles[2usize];
    s_controls.playerViewangles[0usize] = s_controls.playerViewangles[1usize];
    s_controls.playerMoveangles[2usize] = 0i32 as vec_t;
    s_controls.playerMoveangles[1usize] = s_controls.playerMoveangles[2usize];
    s_controls.playerMoveangles[0usize] = s_controls.playerMoveangles[1usize];
    s_controls.playerViewangles[1usize] = (180i32 - 30i32) as vec_t;
    s_controls.playerMoveangles[1usize] = s_controls.playerViewangles[1usize];
    s_controls.playerLegs = LEGS_IDLE as libc::c_int;
    s_controls.playerTorso = TORSO_STAND as libc::c_int;
    s_controls.playerWeapon = WP_NUM_WEAPONS;
    s_controls.playerChat = qfalse;
    match anim {
        1 => s_controls.playerLegs = LEGS_RUN as libc::c_int,
        2 => s_controls.playerLegs = LEGS_WALK as libc::c_int,
        3 => s_controls.playerLegs = LEGS_BACK as libc::c_int,
        4 => s_controls.playerLegs = LEGS_JUMP as libc::c_int,
        5 => s_controls.playerLegs = LEGS_IDLECR as libc::c_int,
        8 => s_controls.playerViewangles[1usize] += 90i32 as libc::c_float,
        9 => s_controls.playerViewangles[1usize] -= 90i32 as libc::c_float,
        6 => {
            s_controls.playerLegs = LEGS_WALK as libc::c_int;
            s_controls.playerMoveangles[1usize] =
                s_controls.playerViewangles[1usize] + 90i32 as libc::c_float
        }
        7 => {
            s_controls.playerLegs = LEGS_WALK as libc::c_int;
            s_controls.playerMoveangles[1usize] =
                s_controls.playerViewangles[1usize] - 90i32 as libc::c_float
        }
        10 => s_controls.playerViewangles[0usize] = -45i32 as vec_t,
        11 => s_controls.playerViewangles[0usize] = 45i32 as vec_t,
        12 => s_controls.playerWeapon = WP_GAUNTLET,
        13 => s_controls.playerWeapon = WP_MACHINEGUN,
        14 => s_controls.playerWeapon = WP_SHOTGUN,
        15 => s_controls.playerWeapon = WP_GRENADE_LAUNCHER,
        16 => s_controls.playerWeapon = WP_ROCKET_LAUNCHER,
        17 => s_controls.playerWeapon = WP_LIGHTNING,
        18 => s_controls.playerWeapon = WP_RAILGUN,
        19 => s_controls.playerWeapon = WP_PLASMAGUN,
        20 => s_controls.playerWeapon = WP_BFG,
        21 => s_controls.playerWeapon = WP_GRAPPLING_HOOK,
        22 => s_controls.playerTorso = TORSO_ATTACK as libc::c_int,
        23 => s_controls.playerTorso = TORSO_GESTURE as libc::c_int,
        24 => {
            s_controls.playerLegs = BOTH_DEATH1 as libc::c_int;
            s_controls.playerTorso = BOTH_DEATH1 as libc::c_int;
            s_controls.playerWeapon = WP_NONE
        }
        25 => s_controls.playerChat = qtrue,
        _ => {}
    }
    UI_PlayerInfo_SetInfo(
        &mut s_controls.playerinfo,
        s_controls.playerLegs,
        s_controls.playerTorso,
        s_controls.playerViewangles.as_mut_ptr(),
        s_controls.playerMoveangles.as_mut_ptr(),
        s_controls.playerWeapon,
        s_controls.playerChat,
    );
}
/*
=================
Controls_GetConfig
=================
*/
unsafe extern "C" fn Controls_GetConfig() {
    let mut i: libc::c_int = 0;
    let mut twokeys: [libc::c_int; 2] = [0; 2];
    let mut bindptr: *mut bind_t = 0 as *mut bind_t;
    bindptr = g_bindings.as_mut_ptr();
    i = 0i32;
    while !(*bindptr).label.is_null() {
        Controls_GetKeyAssignment((*bindptr).command, twokeys.as_mut_ptr());
        (*bindptr).bind1 = twokeys[0usize];
        (*bindptr).bind2 = twokeys[1usize];
        i += 1;
        bindptr = bindptr.offset(1isize)
    }
    s_controls.invertmouse.curvalue = (Controls_GetCvarValue(
        b"m_pitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) < 0i32 as libc::c_float) as libc::c_int;
    s_controls.smoothmouse.curvalue = UI_ClampCvar(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        Controls_GetCvarValue(
            b"m_filter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) as libc::c_int;
    s_controls.alwaysrun.curvalue = UI_ClampCvar(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        Controls_GetCvarValue(
            b"cl_run\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) as libc::c_int;
    s_controls.autoswitch.curvalue = UI_ClampCvar(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        Controls_GetCvarValue(
            b"cg_autoswitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) as libc::c_int;
    s_controls.sensitivity.curvalue = UI_ClampCvar(
        2i32 as libc::c_float,
        30i32 as libc::c_float,
        Controls_GetCvarValue(
            b"sensitivity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    s_controls.joyenable.curvalue = UI_ClampCvar(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        Controls_GetCvarValue(
            b"in_joystick\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) as libc::c_int;
    s_controls.joythreshold.curvalue = UI_ClampCvar(
        0.05f32,
        0.75f32,
        Controls_GetCvarValue(
            b"joy_threshold\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    s_controls.freelook.curvalue = UI_ClampCvar(
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        Controls_GetCvarValue(
            b"cl_freelook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    ) as libc::c_int;
}
/*
=================
Controls_GetCvarValue
=================
*/
unsafe extern "C" fn Controls_GetCvarValue(mut name: *mut libc::c_char) -> libc::c_float {
    let mut cvarptr: *mut configcvar_t = 0 as *mut configcvar_t;
    let mut i: libc::c_int = 0;
    cvarptr = g_configcvars.as_mut_ptr();
    i = 0i32;
    loop {
        if (*cvarptr).name.is_null() {
            return 0i32 as libc::c_float;
        }
        if 0 == strcmp((*cvarptr).name, name) {
            break;
        }
        i += 1;
        cvarptr = cvarptr.offset(1isize)
    }
    return (*cvarptr).value;
}
static mut g_configcvars: [configcvar_t; 9] = [
    configcvar_t {
        name: b"cl_run\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        defaultvalue: 0i32 as libc::c_float,
        value: 0i32 as libc::c_float,
    },
    configcvar_t {
        name: b"m_pitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        defaultvalue: 0i32 as libc::c_float,
        value: 0i32 as libc::c_float,
    },
    configcvar_t {
        name: b"cg_autoswitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        defaultvalue: 0i32 as libc::c_float,
        value: 0i32 as libc::c_float,
    },
    configcvar_t {
        name: b"sensitivity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        defaultvalue: 0i32 as libc::c_float,
        value: 0i32 as libc::c_float,
    },
    configcvar_t {
        name: b"in_joystick\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        defaultvalue: 0i32 as libc::c_float,
        value: 0i32 as libc::c_float,
    },
    configcvar_t {
        name: b"joy_threshold\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        defaultvalue: 0i32 as libc::c_float,
        value: 0i32 as libc::c_float,
    },
    configcvar_t {
        name: b"m_filter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        defaultvalue: 0i32 as libc::c_float,
        value: 0i32 as libc::c_float,
    },
    configcvar_t {
        name: b"cl_freelook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        defaultvalue: 0i32 as libc::c_float,
        value: 0i32 as libc::c_float,
    },
    configcvar_t {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        defaultvalue: 0i32 as libc::c_float,
        value: 0i32 as libc::c_float,
    },
];
/*
=================
Controls_GetKeyAssignment
=================
*/
unsafe extern "C" fn Controls_GetKeyAssignment(
    mut command: *mut libc::c_char,
    mut twokeys: *mut libc::c_int,
) {
    let mut count: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b: [libc::c_char; 256] = [0; 256];
    let ref mut fresh0 = *twokeys.offset(1isize);
    *fresh0 = -1i32;
    *twokeys.offset(0isize) = *fresh0;
    count = 0i32;
    j = 0i32;
    while j < 256i32 {
        trap_Key_GetBindingBuf(j, b.as_mut_ptr(), 256i32);
        if !(*b.as_mut_ptr() as libc::c_int == 0i32) {
            if 0 == Q_stricmp(b.as_mut_ptr(), command) {
                *twokeys.offset(count as isize) = j;
                count += 1;
                if count == 2i32 {
                    break;
                }
            }
        }
        j += 1
    }
}
static mut g_bindings: [bind_t; 36] = [
    bind_t {
        command: b"+scores\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"show scores\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 0i32,
        anim: 0i32,
        defaultbind1: K_TAB as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+button2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"use item\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 1i32,
        anim: 0i32,
        defaultbind1: K_ENTER as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"run / walk\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 2i32,
        anim: 1i32,
        defaultbind1: K_SHIFT as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+forward\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"walk forward\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 3i32,
        anim: 2i32,
        defaultbind1: K_UPARROW as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+back\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"backpedal\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 4i32,
        anim: 3i32,
        defaultbind1: K_DOWNARROW as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+moveleft\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"step left\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 5i32,
        anim: 6i32,
        defaultbind1: ',' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+moveright\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"step right\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 6i32,
        anim: 7i32,
        defaultbind1: '.' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+moveup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"up / jump\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 7i32,
        anim: 4i32,
        defaultbind1: K_SPACE as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+movedown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"down / crouch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 8i32,
        anim: 5i32,
        defaultbind1: 'c' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+left\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"turn left\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 9i32,
        anim: 8i32,
        defaultbind1: K_LEFTARROW as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+right\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"turn right\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 10i32,
        anim: 9i32,
        defaultbind1: K_RIGHTARROW as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+strafe\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"sidestep / turn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 11i32,
        anim: 0i32,
        defaultbind1: K_ALT as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+lookup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"look up\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 12i32,
        anim: 10i32,
        defaultbind1: K_PGDN as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+lookdown\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"look down\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 13i32,
        anim: 11i32,
        defaultbind1: K_DEL as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+mlook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"mouse look\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 14i32,
        anim: 0i32,
        defaultbind1: '/' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"centerview\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"center view\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 15i32,
        anim: 0i32,
        defaultbind1: K_END as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+zoom\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"zoom view\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 16i32,
        anim: 0i32,
        defaultbind1: -1i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapon 1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"gauntlet\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 17i32,
        anim: 12i32,
        defaultbind1: '1' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapon 2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"machinegun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 18i32,
        anim: 13i32,
        defaultbind1: '2' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapon 3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"shotgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 19i32,
        anim: 14i32,
        defaultbind1: '3' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapon 4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"grenade launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 20i32,
        anim: 15i32,
        defaultbind1: '4' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapon 5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"rocket launcher\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 21i32,
        anim: 16i32,
        defaultbind1: '5' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapon 6\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"lightning\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 22i32,
        anim: 17i32,
        defaultbind1: '6' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapon 7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"railgun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 23i32,
        anim: 18i32,
        defaultbind1: '7' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapon 8\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"plasma gun\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 24i32,
        anim: 19i32,
        defaultbind1: '8' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapon 9\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"BFG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 25i32,
        anim: 20i32,
        defaultbind1: '9' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+attack\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"attack\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 26i32,
        anim: 22i32,
        defaultbind1: K_CTRL as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapprev\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"prev weapon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 27i32,
        anim: 0i32,
        defaultbind1: '[' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"weapnext\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"next weapon\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 28i32,
        anim: 0i32,
        defaultbind1: ']' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"+button3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"gesture\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 29i32,
        anim: 23i32,
        defaultbind1: K_MOUSE3 as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"messagemode\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"chat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 30i32,
        anim: 25i32,
        defaultbind1: 't' as i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"messagemode2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"chat - team\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 31i32,
        anim: 25i32,
        defaultbind1: -1i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"messagemode3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"chat - target\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 32i32,
        anim: 25i32,
        defaultbind1: -1i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"messagemode4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"chat - attacker\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 33i32,
        anim: 25i32,
        defaultbind1: -1i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: b"togglemenu\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        label: b"toggle menu\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        id: 34i32,
        anim: 0i32,
        defaultbind1: K_ESCAPE as libc::c_int,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
    bind_t {
        command: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        label: 0 as *const libc::c_void as *mut libc::c_void as *mut libc::c_char,
        id: 0i32,
        anim: 0i32,
        defaultbind1: -1i32,
        defaultbind2: -1i32,
        bind1: -1i32,
        bind2: -1i32,
    },
];
/*
=================
Controls_InitCvars
=================
*/
unsafe extern "C" fn Controls_InitCvars() {
    let mut i: libc::c_int = 0;
    let mut cvarptr: *mut configcvar_t = 0 as *mut configcvar_t;
    cvarptr = g_configcvars.as_mut_ptr();
    i = 0i32;
    while !(*cvarptr).name.is_null() {
        (*cvarptr).value = trap_Cvar_VariableValue((*cvarptr).name);
        trap_Cvar_Reset((*cvarptr).name);
        (*cvarptr).defaultvalue = trap_Cvar_VariableValue((*cvarptr).name);
        trap_Cvar_SetValue((*cvarptr).name, (*cvarptr).value);
        i += 1;
        cvarptr = cvarptr.offset(1isize)
    }
}
/*
=================
Controls_StatusBar
=================
*/
unsafe extern "C" fn Controls_StatusBar(mut self_0: *mut libc::c_void) {
    UI_DrawString(
        (640i32 as libc::c_double * 0.50f64) as libc::c_int,
        (480i32 as libc::c_double * 0.80f64) as libc::c_int,
        b"Use Arrow Keys or CLICK to change\x00" as *const u8 as *const libc::c_char,
        0x10i32 | 0x1i32,
        colorWhite.as_mut_ptr(),
    );
}
/*
=================
Controls_MenuEvent
=================
*/
unsafe extern "C" fn Controls_MenuEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    match (*(ptr as *mut menucommon_s)).id {
        100 => {
            if event == 3i32 {
                s_controls.section = 0i32;
                Controls_Update();
            }
        }
        101 => {
            if event == 3i32 {
                s_controls.section = 1i32;
                Controls_Update();
            }
        }
        102 => {
            if event == 3i32 {
                s_controls.section = 2i32;
                Controls_Update();
            }
        }
        103 => {
            if event == 3i32 {
                s_controls.section = 3i32;
                Controls_Update();
            }
        }
        104 => {
            if event == 3i32 {
                UI_ConfirmMenu(
                    b"SET TO DEFAULTS?\x00" as *const u8 as *const libc::c_char,
                    Some(Controls_ResetDefaults_Draw),
                    Some(Controls_ResetDefaults_Action),
                );
            }
        }
        105 => {
            if event == 3i32 {
                if 0 != s_controls.changesmade as u64 {
                    Controls_SetConfig();
                }
                UI_PopMenu();
            }
        }
        106 => {
            if event == 3i32 {
                Controls_SetConfig();
                UI_PopMenu();
            }
        }
        107 => {
            if event == 3i32 {
                UI_PopMenu();
            }
        }
        35 | 39 | 36 | 42 | 37 | 38 | 40 | 41 => {
            if event == 3i32 {
                s_controls.changesmade = qtrue
            }
        }
        _ => {}
    };
}
/*
=================
Controls_SetConfig
=================
*/
unsafe extern "C" fn Controls_SetConfig() {
    let mut i: libc::c_int = 0;
    let mut bindptr: *mut bind_t = 0 as *mut bind_t;
    bindptr = g_bindings.as_mut_ptr();
    i = 0i32;
    while !(*bindptr).label.is_null() {
        if (*bindptr).bind1 != -1i32 {
            trap_Key_SetBinding((*bindptr).bind1, (*bindptr).command);
            if (*bindptr).bind2 != -1i32 {
                trap_Key_SetBinding((*bindptr).bind2, (*bindptr).command);
            }
        }
        i += 1;
        bindptr = bindptr.offset(1isize)
    }
    if 0 != s_controls.invertmouse.curvalue {
        trap_Cvar_SetValue(
            b"m_pitch\x00" as *const u8 as *const libc::c_char,
            -fabs(
                trap_Cvar_VariableValue(b"m_pitch\x00" as *const u8 as *const libc::c_char)
                    as libc::c_double,
            ) as libc::c_float,
        );
    } else {
        trap_Cvar_SetValue(
            b"m_pitch\x00" as *const u8 as *const libc::c_char,
            fabs(
                trap_Cvar_VariableValue(b"m_pitch\x00" as *const u8 as *const libc::c_char)
                    as libc::c_double,
            ) as libc::c_float,
        );
    }
    trap_Cvar_SetValue(
        b"m_filter\x00" as *const u8 as *const libc::c_char,
        s_controls.smoothmouse.curvalue as libc::c_float,
    );
    trap_Cvar_SetValue(
        b"cl_run\x00" as *const u8 as *const libc::c_char,
        s_controls.alwaysrun.curvalue as libc::c_float,
    );
    trap_Cvar_SetValue(
        b"cg_autoswitch\x00" as *const u8 as *const libc::c_char,
        s_controls.autoswitch.curvalue as libc::c_float,
    );
    trap_Cvar_SetValue(
        b"sensitivity\x00" as *const u8 as *const libc::c_char,
        s_controls.sensitivity.curvalue,
    );
    trap_Cvar_SetValue(
        b"in_joystick\x00" as *const u8 as *const libc::c_char,
        s_controls.joyenable.curvalue as libc::c_float,
    );
    trap_Cvar_SetValue(
        b"joy_threshold\x00" as *const u8 as *const libc::c_char,
        s_controls.joythreshold.curvalue,
    );
    trap_Cvar_SetValue(
        b"cl_freelook\x00" as *const u8 as *const libc::c_char,
        s_controls.freelook.curvalue as libc::c_float,
    );
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        b"in_restart\n\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
Controls_ResetDefaults_Action
=================
*/
unsafe extern "C" fn Controls_ResetDefaults_Action(mut result: qboolean) {
    if 0 == result as u64 {
        return;
    }
    s_controls.changesmade = qtrue;
    Controls_SetDefaults();
    Controls_Update();
}
/*
=================
Controls_SetDefaults
=================
*/
unsafe extern "C" fn Controls_SetDefaults() {
    let mut i: libc::c_int = 0;
    let mut bindptr: *mut bind_t = 0 as *mut bind_t;
    bindptr = g_bindings.as_mut_ptr();
    i = 0i32;
    while !(*bindptr).label.is_null() {
        (*bindptr).bind1 = (*bindptr).defaultbind1;
        (*bindptr).bind2 = (*bindptr).defaultbind2;
        i += 1;
        bindptr = bindptr.offset(1isize)
    }
    s_controls.invertmouse.curvalue = (Controls_GetCvarDefault(
        b"m_pitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) < 0i32 as libc::c_float) as libc::c_int;
    s_controls.smoothmouse.curvalue = Controls_GetCvarDefault(
        b"m_filter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    s_controls.alwaysrun.curvalue = Controls_GetCvarDefault(
        b"cl_run\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    s_controls.autoswitch.curvalue = Controls_GetCvarDefault(
        b"cg_autoswitch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    s_controls.sensitivity.curvalue = Controls_GetCvarDefault(
        b"sensitivity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s_controls.joyenable.curvalue = Controls_GetCvarDefault(
        b"in_joystick\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    s_controls.joythreshold.curvalue = Controls_GetCvarDefault(
        b"joy_threshold\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s_controls.freelook.curvalue = Controls_GetCvarDefault(
        b"cl_freelook\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
}
/*
=================
Controls_GetCvarDefault
=================
*/
unsafe extern "C" fn Controls_GetCvarDefault(mut name: *mut libc::c_char) -> libc::c_float {
    let mut cvarptr: *mut configcvar_t = 0 as *mut configcvar_t;
    let mut i: libc::c_int = 0;
    cvarptr = g_configcvars.as_mut_ptr();
    i = 0i32;
    loop {
        if (*cvarptr).name.is_null() {
            return 0i32 as libc::c_float;
        }
        if 0 == strcmp((*cvarptr).name, name) {
            break;
        }
        i += 1;
        cvarptr = cvarptr.offset(1isize)
    }
    return (*cvarptr).defaultvalue;
}
/*
=================
Controls_ResetDefaults_Draw
=================
*/
unsafe extern "C" fn Controls_ResetDefaults_Draw() {
    UI_DrawProportionalString(
        640i32 / 2i32,
        356i32 + 27i32 * 0i32,
        b"WARNING: This will reset all\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_yellow.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640i32 / 2i32,
        356i32 + 27i32 * 1i32,
        b"controls to their default values.\x00" as *const u8 as *const libc::c_char,
        0x1i32 | 0x10i32,
        color_yellow.as_mut_ptr(),
    );
}
/*
=================
Controls_DrawKeyBinding
=================
*/
unsafe extern "C" fn Controls_DrawKeyBinding(mut self_0: *mut libc::c_void) {
    let mut a: *mut menuaction_s = 0 as *mut menuaction_s;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut b1: libc::c_int = 0;
    let mut b2: libc::c_int = 0;
    let mut c: qboolean = qfalse;
    let mut name: [libc::c_char; 32] = [0; 32];
    let mut name2: [libc::c_char; 32] = [0; 32];
    a = self_0 as *mut menuaction_s;
    x = (*a).generic.x;
    y = (*a).generic.y;
    c = (Menu_ItemAtCursor((*a).generic.parent) == a as *mut libc::c_void) as libc::c_int
        as qboolean;
    b1 = g_bindings[(*a).generic.id as usize].bind1;
    if b1 == -1i32 {
        strcpy(
            name.as_mut_ptr(),
            b"???\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        trap_Key_KeynumToStringBuf(b1, name.as_mut_ptr(), 32i32);
        Q_strupr(name.as_mut_ptr());
        b2 = g_bindings[(*a).generic.id as usize].bind2;
        if b2 != -1i32 {
            trap_Key_KeynumToStringBuf(b2, name2.as_mut_ptr(), 32i32);
            Q_strupr(name2.as_mut_ptr());
            strcat(
                name.as_mut_ptr(),
                b" or \x00" as *const u8 as *const libc::c_char,
            );
            strcat(name.as_mut_ptr(), name2.as_mut_ptr());
        }
    }
    if 0 != c as u64 {
        UI_FillRect(
            (*a).generic.left as libc::c_float,
            (*a).generic.top as libc::c_float,
            ((*a).generic.right - (*a).generic.left + 1i32) as libc::c_float,
            ((*a).generic.bottom - (*a).generic.top + 1i32) as libc::c_float,
            listbar_color.as_mut_ptr(),
        );
        UI_DrawString(
            x - 8i32,
            y,
            g_bindings[(*a).generic.id as usize].label,
            0x2i32 | 0x10i32,
            text_color_highlight.as_mut_ptr(),
        );
        UI_DrawString(
            x + 8i32,
            y,
            name.as_mut_ptr(),
            0i32 | 0x10i32 | 0x4000i32,
            text_color_highlight.as_mut_ptr(),
        );
        if 0 != s_controls.waitingforkey as u64 {
            UI_DrawChar(
                x,
                y,
                '=' as i32,
                0x1i32 | 0x1000i32 | 0x10i32,
                text_color_highlight.as_mut_ptr(),
            );
            UI_DrawString(
                (640i32 as libc::c_double * 0.50f64) as libc::c_int,
                (480i32 as libc::c_double * 0.80f64) as libc::c_int,
                b"Waiting for new key ... ESCAPE to cancel\x00" as *const u8 as *const libc::c_char,
                0x10i32 | 0x1i32 | 0x4000i32,
                colorWhite.as_mut_ptr(),
            );
        } else {
            UI_DrawChar(
                x,
                y,
                13i32,
                0x1i32 | 0x1000i32 | 0x10i32,
                text_color_highlight.as_mut_ptr(),
            );
            UI_DrawString(
                (640i32 as libc::c_double * 0.50f64) as libc::c_int,
                (480i32 as libc::c_double * 0.78f64) as libc::c_int,
                b"Press ENTER or CLICK to change\x00" as *const u8 as *const libc::c_char,
                0x10i32 | 0x1i32,
                colorWhite.as_mut_ptr(),
            );
            UI_DrawString(
                (640i32 as libc::c_double * 0.50f64) as libc::c_int,
                (480i32 as libc::c_double * 0.82f64) as libc::c_int,
                b"Press BACKSPACE to clear\x00" as *const u8 as *const libc::c_char,
                0x10i32 | 0x1i32,
                colorWhite.as_mut_ptr(),
            );
        }
    } else if 0 != (*a).generic.flags & 0x2000i32 as libc::c_uint {
        UI_DrawString(
            x - 8i32,
            y,
            g_bindings[(*a).generic.id as usize].label,
            0x2i32 | 0x10i32,
            text_color_disabled.as_mut_ptr(),
        );
        UI_DrawString(
            x + 8i32,
            y,
            name.as_mut_ptr(),
            0i32 | 0x10i32,
            text_color_disabled.as_mut_ptr(),
        );
    } else {
        UI_DrawString(
            x - 8i32,
            y,
            g_bindings[(*a).generic.id as usize].label,
            0x2i32 | 0x10i32,
            controls_binding_color.as_mut_ptr(),
        );
        UI_DrawString(
            x + 8i32,
            y,
            name.as_mut_ptr(),
            0i32 | 0x10i32,
            controls_binding_color.as_mut_ptr(),
        );
    };
}
static mut controls_binding_color: vec4_t = [1.00f32, 0.43f32, 0.00f32, 1.00f32];
/*
=================
Controls_ActionEvent
=================
*/
unsafe extern "C" fn Controls_ActionEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event == 2i32 {
        Controls_UpdateModel(0i32);
    } else if event == 1i32 {
        Controls_UpdateModel(g_bindings[(*(ptr as *mut menucommon_s)).id as usize].anim);
    } else if event == 3i32 && 0 == s_controls.waitingforkey as u64 {
        s_controls.waitingforkey = qtrue;
        Controls_Update();
    };
}
/*
=================
Controls_DrawPlayer
=================
*/
unsafe extern "C" fn Controls_DrawPlayer(mut self_0: *mut libc::c_void) {
    let mut b: *mut menubitmap_s = 0 as *mut menubitmap_s;
    let mut buf: [libc::c_char; 64] = [0; 64];
    trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    if strcmp(buf.as_mut_ptr(), s_controls.playerModel.as_mut_ptr()) != 0i32 {
        UI_PlayerInfo_SetModel(&mut s_controls.playerinfo, buf.as_mut_ptr());
        strcpy(s_controls.playerModel.as_mut_ptr(), buf.as_mut_ptr());
        Controls_UpdateModel(0i32);
    }
    b = self_0 as *mut menubitmap_s;
    UI_DrawPlayer(
        (*b).generic.x as libc::c_float,
        (*b).generic.y as libc::c_float,
        (*b).width as libc::c_float,
        (*b).height as libc::c_float,
        &mut s_controls.playerinfo,
        uis.realtime / 2i32,
    );
}
/*
=================
Controls_MenuKey
=================
*/
unsafe extern "C" fn Controls_MenuKey(mut key: libc::c_int) -> sfxHandle_t {
    let mut current_block: u64;
    let mut id: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut found: qboolean = qfalse;
    let mut bindptr: *mut bind_t = 0 as *mut bind_t;
    found = qfalse;
    if 0 == s_controls.waitingforkey as u64 {
        match key {
            127 | 140 | 171 => {
                current_block = 2905725322067095232;
                match current_block {
                    14600208599415517630 => {
                        if 0 != s_controls.changesmade as u64 {
                            Controls_SetConfig();
                        }
                        current_block = 3002389269805436602;
                    }
                    _ => {
                        key = -1i32;
                        current_block = 15904375183555213903;
                    }
                }
            }
            179 | 27 => {
                current_block = 14600208599415517630;
                match current_block {
                    14600208599415517630 => {
                        if 0 != s_controls.changesmade as u64 {
                            Controls_SetConfig();
                        }
                        current_block = 3002389269805436602;
                    }
                    _ => {
                        key = -1i32;
                        current_block = 15904375183555213903;
                    }
                }
            }
            _ => {
                current_block = 3002389269805436602;
            }
        }
    } else if 0 != key & 1024i32 {
        current_block = 3002389269805436602;
    } else {
        match key {
            27 => {
                s_controls.waitingforkey = qfalse;
                Controls_Update();
                return menu_out_sound;
            }
            96 => {
                current_block = 3002389269805436602;
            }
            _ => {
                current_block = 15904375183555213903;
            }
        }
    }
    match current_block {
        15904375183555213903 => {
            s_controls.changesmade = qtrue;
            if key != -1i32 {
                bindptr = g_bindings.as_mut_ptr();
                i = 0i32;
                while !(*bindptr).label.is_null() {
                    if (*bindptr).bind2 == key {
                        (*bindptr).bind2 = -1i32
                    }
                    if (*bindptr).bind1 == key {
                        (*bindptr).bind1 = (*bindptr).bind2;
                        (*bindptr).bind2 = -1i32
                    }
                    i += 1;
                    bindptr = bindptr.offset(1isize)
                }
            }
            id =
                (*(s_controls.menu.items[s_controls.menu.cursor as usize] as *mut menucommon_s)).id;
            bindptr = g_bindings.as_mut_ptr();
            i = 0i32;
            while !(*bindptr).label.is_null() {
                if (*bindptr).id == id {
                    found = qtrue;
                    if key == -1i32 {
                        if (*bindptr).bind1 != -1i32 {
                            trap_Key_SetBinding(
                                (*bindptr).bind1,
                                b"\x00" as *const u8 as *const libc::c_char,
                            );
                            (*bindptr).bind1 = -1i32
                        }
                        if (*bindptr).bind2 != -1i32 {
                            trap_Key_SetBinding(
                                (*bindptr).bind2,
                                b"\x00" as *const u8 as *const libc::c_char,
                            );
                            (*bindptr).bind2 = -1i32
                        }
                    } else if (*bindptr).bind1 == -1i32 {
                        (*bindptr).bind1 = key
                    } else if (*bindptr).bind1 != key && (*bindptr).bind2 == -1i32 {
                        (*bindptr).bind2 = key
                    } else {
                        trap_Key_SetBinding(
                            (*bindptr).bind1,
                            b"\x00" as *const u8 as *const libc::c_char,
                        );
                        trap_Key_SetBinding(
                            (*bindptr).bind2,
                            b"\x00" as *const u8 as *const libc::c_char,
                        );
                        (*bindptr).bind1 = key;
                        (*bindptr).bind2 = -1i32
                    }
                    break;
                } else {
                    i += 1;
                    bindptr = bindptr.offset(1isize)
                }
            }
            s_controls.waitingforkey = qfalse;
            if 0 != found as u64 {
                Controls_Update();
                return menu_out_sound;
            }
        }
        _ => {}
    }
    return Menu_DefaultKey(&mut s_controls.menu, key);
}
#[no_mangle]
pub unsafe extern "C" fn Controls_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn run_static_initializers() {
    g_misc_controls = [
        &s_controls.showscores as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.useitem as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.gesture as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.chat as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.chat2 as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.chat3 as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.chat4 as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.togglemenu as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        0 as *const menucommon_s as *mut menucommon_s,
    ];
    g_weapons_controls = [
        &s_controls.attack as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.nextweapon as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.prevweapon as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.autoswitch as *const menuradiobutton_s as *mut menuradiobutton_s
            as *mut menucommon_s,
        &s_controls.chainsaw as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.machinegun as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.shotgun as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.grenadelauncher as *const menuaction_s as *mut menuaction_s
            as *mut menucommon_s,
        &s_controls.rocketlauncher as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.lightning as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.railgun as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.plasma as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.bfg as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        0 as *const menucommon_s as *mut menucommon_s,
    ];
    g_looking_controls = [
        &s_controls.sensitivity as *const menuslider_s as *mut menuslider_s as *mut menucommon_s,
        &s_controls.smoothmouse as *const menuradiobutton_s as *mut menuradiobutton_s
            as *mut menucommon_s,
        &s_controls.invertmouse as *const menuradiobutton_s as *mut menuradiobutton_s
            as *mut menucommon_s,
        &s_controls.lookup as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.lookdown as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.mouselook as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.freelook as *const menuradiobutton_s as *mut menuradiobutton_s
            as *mut menucommon_s,
        &s_controls.centerview as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.zoomview as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.joyenable as *const menuradiobutton_s as *mut menuradiobutton_s
            as *mut menucommon_s,
        &s_controls.joythreshold as *const menuslider_s as *mut menuslider_s as *mut menucommon_s,
        0 as *const menucommon_s as *mut menucommon_s,
    ];
    g_movement_controls = [
        &s_controls.alwaysrun as *const menuradiobutton_s as *mut menuradiobutton_s
            as *mut menucommon_s,
        &s_controls.run as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.walkforward as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.backpedal as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.stepleft as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.stepright as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.moveup as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.movedown as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.turnleft as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.turnright as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        &s_controls.sidestep as *const menuaction_s as *mut menuaction_s as *mut menucommon_s,
        0 as *const menucommon_s as *mut menucommon_s,
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct configcvar_t {
    pub name: *mut libc::c_char,
    pub defaultvalue: libc::c_float,
    pub value: libc::c_float,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct controls_t {
    pub menu: menuframework_s,
    pub banner: menutext_s,
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub player: menubitmap_s,
    pub movement: menutext_s,
    pub looking: menutext_s,
    pub weapons: menutext_s,
    pub misc: menutext_s,
    pub walkforward: menuaction_s,
    pub backpedal: menuaction_s,
    pub stepleft: menuaction_s,
    pub stepright: menuaction_s,
    pub moveup: menuaction_s,
    pub movedown: menuaction_s,
    pub turnleft: menuaction_s,
    pub turnright: menuaction_s,
    pub sidestep: menuaction_s,
    pub run: menuaction_s,
    pub machinegun: menuaction_s,
    pub chainsaw: menuaction_s,
    pub shotgun: menuaction_s,
    pub grenadelauncher: menuaction_s,
    pub rocketlauncher: menuaction_s,
    pub lightning: menuaction_s,
    pub railgun: menuaction_s,
    pub plasma: menuaction_s,
    pub bfg: menuaction_s,
    pub attack: menuaction_s,
    pub prevweapon: menuaction_s,
    pub nextweapon: menuaction_s,
    pub lookup: menuaction_s,
    pub lookdown: menuaction_s,
    pub mouselook: menuaction_s,
    pub freelook: menuradiobutton_s,
    pub centerview: menuaction_s,
    pub zoomview: menuaction_s,
    pub gesture: menuaction_s,
    pub invertmouse: menuradiobutton_s,
    pub sensitivity: menuslider_s,
    pub smoothmouse: menuradiobutton_s,
    pub alwaysrun: menuradiobutton_s,
    pub showscores: menuaction_s,
    pub autoswitch: menuradiobutton_s,
    pub useitem: menuaction_s,
    pub playerinfo: playerInfo_t,
    pub changesmade: qboolean,
    pub chat: menuaction_s,
    pub chat2: menuaction_s,
    pub chat3: menuaction_s,
    pub chat4: menuaction_s,
    pub togglemenu: menuaction_s,
    pub joyenable: menuradiobutton_s,
    pub joythreshold: menuslider_s,
    pub section: libc::c_int,
    pub waitingforkey: qboolean,
    pub playerModel: [libc::c_char; 64],
    pub playerViewangles: vec3_t,
    pub playerMoveangles: vec3_t,
    pub playerLegs: libc::c_int,
    pub playerTorso: libc::c_int,
    pub playerWeapon: weapon_t,
    pub playerChat: qboolean,
    pub back: menubitmap_s,
    pub name: menutext_s,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bind_t {
    pub command: *mut libc::c_char,
    pub label: *mut libc::c_char,
    pub id: libc::c_int,
    pub anim: libc::c_int,
    pub defaultbind1: libc::c_int,
    pub defaultbind2: libc::c_int,
    pub bind1: libc::c_int,
    pub bind2: libc::c_int,
}
