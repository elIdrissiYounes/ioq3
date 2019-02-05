use bg_public_h::{
    animation_s, animation_t, unnamed_0, weapon_t, BOTH_DEAD1, BOTH_DEAD2, BOTH_DEAD3, BOTH_DEATH1,
    BOTH_DEATH2, BOTH_DEATH3, FLAG_RUN, FLAG_STAND, FLAG_STAND2RUN, LEGS_BACK, LEGS_BACKCR,
    LEGS_BACKWALK, LEGS_IDLE, LEGS_IDLECR, LEGS_JUMP, LEGS_JUMPB, LEGS_LAND, LEGS_LANDB, LEGS_RUN,
    LEGS_SWIM, LEGS_TURN, LEGS_WALK, LEGS_WALKCR, MAX_ANIMATIONS, MAX_TOTALANIMATIONS,
    TORSO_AFFIRMATIVE, TORSO_ATTACK, TORSO_ATTACK2, TORSO_DROP, TORSO_FOLLOWME, TORSO_GESTURE,
    TORSO_GETFLAG, TORSO_GUARDBASE, TORSO_NEGATIVE, TORSO_PATROL, TORSO_RAISE, TORSO_STAND,
    TORSO_STAND2, WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING,
    WP_MACHINEGUN, WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER,
    WP_SHOTGUN,
};
use keycodes_h::{
    unnamed, K_ALT, K_AUX1, K_AUX10, K_AUX11, K_AUX12, K_AUX13, K_AUX14, K_AUX15, K_AUX16, K_AUX2,
    K_AUX3, K_AUX4, K_AUX5, K_AUX6, K_AUX7, K_AUX8, K_AUX9, K_BACKSPACE, K_BREAK, K_CAPSLOCK,
    K_COMMAND, K_COMPOSE, K_CONSOLE, K_CTRL, K_DEL, K_DOWNARROW, K_END, K_ENTER, K_ESCAPE, K_EURO,
    K_F1, K_F10, K_F11, K_F12, K_F13, K_F14, K_F15, K_F2, K_F3, K_F4, K_F5, K_F6, K_F7, K_F8, K_F9,
    K_HELP, K_HOME, K_INS, K_JOY1, K_JOY10, K_JOY11, K_JOY12, K_JOY13, K_JOY14, K_JOY15, K_JOY16,
    K_JOY17, K_JOY18, K_JOY19, K_JOY2, K_JOY20, K_JOY21, K_JOY22, K_JOY23, K_JOY24, K_JOY25,
    K_JOY26, K_JOY27, K_JOY28, K_JOY29, K_JOY3, K_JOY30, K_JOY31, K_JOY32, K_JOY4, K_JOY5, K_JOY6,
    K_JOY7, K_JOY8, K_JOY9, K_KP_5, K_KP_DEL, K_KP_DOWNARROW, K_KP_END, K_KP_ENTER, K_KP_EQUALS,
    K_KP_HOME, K_KP_INS, K_KP_LEFTARROW, K_KP_MINUS, K_KP_NUMLOCK, K_KP_PGDN, K_KP_PGUP, K_KP_PLUS,
    K_KP_RIGHTARROW, K_KP_SLASH, K_KP_STAR, K_KP_UPARROW, K_LEFTARROW, K_MENU, K_MODE, K_MOUSE1,
    K_MOUSE2, K_MOUSE3, K_MOUSE4, K_MOUSE5, K_MWHEELDOWN, K_MWHEELUP, K_PAD0_A, K_PAD0_B,
    K_PAD0_BACK, K_PAD0_DPAD_DOWN, K_PAD0_DPAD_LEFT, K_PAD0_DPAD_RIGHT, K_PAD0_DPAD_UP,
    K_PAD0_GUIDE, K_PAD0_LEFTSHOULDER, K_PAD0_LEFTSTICK_CLICK, K_PAD0_LEFTSTICK_DOWN,
    K_PAD0_LEFTSTICK_LEFT, K_PAD0_LEFTSTICK_RIGHT, K_PAD0_LEFTSTICK_UP, K_PAD0_LEFTTRIGGER,
    K_PAD0_RIGHTSHOULDER, K_PAD0_RIGHTSTICK_CLICK, K_PAD0_RIGHTSTICK_DOWN, K_PAD0_RIGHTSTICK_LEFT,
    K_PAD0_RIGHTSTICK_RIGHT, K_PAD0_RIGHTSTICK_UP, K_PAD0_RIGHTTRIGGER, K_PAD0_START, K_PAD0_X,
    K_PAD0_Y, K_PAUSE, K_PGDN, K_PGUP, K_POWER, K_PRINT, K_RIGHTARROW, K_SCROLLOCK, K_SHIFT,
    K_SPACE, K_SUPER, K_SYSREQ, K_TAB, K_UNDO, K_UPARROW, K_WORLD_0, K_WORLD_1, K_WORLD_10,
    K_WORLD_11, K_WORLD_12, K_WORLD_13, K_WORLD_14, K_WORLD_15, K_WORLD_16, K_WORLD_17, K_WORLD_18,
    K_WORLD_19, K_WORLD_2, K_WORLD_20, K_WORLD_21, K_WORLD_22, K_WORLD_23, K_WORLD_24, K_WORLD_25,
    K_WORLD_26, K_WORLD_27, K_WORLD_28, K_WORLD_29, K_WORLD_3, K_WORLD_30, K_WORLD_31, K_WORLD_32,
    K_WORLD_33, K_WORLD_34, K_WORLD_35, K_WORLD_36, K_WORLD_37, K_WORLD_38, K_WORLD_39, K_WORLD_4,
    K_WORLD_40, K_WORLD_41, K_WORLD_42, K_WORLD_43, K_WORLD_44, K_WORLD_45, K_WORLD_46, K_WORLD_47,
    K_WORLD_48, K_WORLD_49, K_WORLD_5, K_WORLD_50, K_WORLD_51, K_WORLD_52, K_WORLD_53, K_WORLD_54,
    K_WORLD_55, K_WORLD_56, K_WORLD_57, K_WORLD_58, K_WORLD_59, K_WORLD_6, K_WORLD_60, K_WORLD_61,
    K_WORLD_62, K_WORLD_63, K_WORLD_64, K_WORLD_65, K_WORLD_66, K_WORLD_67, K_WORLD_68, K_WORLD_69,
    K_WORLD_7, K_WORLD_70, K_WORLD_71, K_WORLD_72, K_WORLD_73, K_WORLD_74, K_WORLD_75, K_WORLD_76,
    K_WORLD_77, K_WORLD_78, K_WORLD_79, K_WORLD_8, K_WORLD_80, K_WORLD_81, K_WORLD_82, K_WORLD_83,
    K_WORLD_84, K_WORLD_85, K_WORLD_86, K_WORLD_87, K_WORLD_88, K_WORLD_89, K_WORLD_9, K_WORLD_90,
    K_WORLD_91, K_WORLD_92, K_WORLD_93, K_WORLD_94, K_WORLD_95, MAX_KEYS,
};
use libc;
use q_shared_h::{
    byte, colorRed, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, va, vec3_t, vec4_t, vec_t,
    COM_StripExtension, Com_sprintf, Q_CleanStr, Q_strcat, Q_stricmp, Q_stricmpn, Q_strncpyz,
    Q_strupr,
};
use stdlib::{memset, strcat, strchr, strcmp, strlen, strstr};
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
    _tag_menuframework, lerpFrame_t, menubitmap_s, menucommon_s, menuframework_s, menutext_s,
    playerInfo_t, trap_Cvar_Set, trap_Cvar_VariableStringBuffer, trap_Cvar_VariableValue,
    trap_FS_GetFileList, trap_MemoryRemaining, trap_R_RegisterShaderNoMip, trap_S_RegisterSound,
    uiStatic_t,
};
use ui_main::{
    ui_browserGameType, ui_browserMaster, ui_browserShowEmpty, ui_browserShowFull,
    ui_browserSortKey, ui_cdkeychecked, UI_RegisterCvars, UI_UpdateCvars,
};
use ui_menu::{MainMenu_Cache, UI_MainMenu};
use ui_mfield::{MField_Draw, MenuField_Draw, MenuField_Init, MenuField_Key};
use ui_mods::{UI_ModsMenu, UI_ModsMenu_Cache};
use ui_network::{UI_NetworkOptionsMenu, UI_NetworkOptionsMenu_Cache};
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
// ui_playermodel.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_PlayerModelMenu() {
    PlayerModel_MenuInit();
    UI_PushMenu(&mut s_playermodel.menu);
    Menu_SetCursorToItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.pics[(s_playermodel.selectedmodel % (4i32 * 4i32)) as usize]
            as *mut menubitmap_s as *mut libc::c_void,
    );
}
static mut s_playermodel: playermodel_t = playermodel_t {
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
    ports: menubitmap_s {
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
    modelname: menutext_s {
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
    skinname: menutext_s {
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
    playername: menutext_s {
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
    nummodels: 0,
    modelnames: [[0; 128]; 256],
    modelpage: 0,
    numpages: 0,
    modelskin: [0; 64],
    selectedmodel: 0,
};
/*
=================
PlayerModel_MenuInit
=================
*/
unsafe extern "C" fn PlayerModel_MenuInit() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    static mut playername: [libc::c_char; 32] = [0; 32];
    static mut modelname: [libc::c_char; 32] = [0; 32];
    static mut skinname: [libc::c_char; 32] = [0; 32];
    memset(
        &mut s_playermodel as *mut playermodel_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<playermodel_t>() as libc::c_ulong,
    );
    PlayerModel_Cache();
    s_playermodel.menu.key = Some(PlayerModel_MenuKey);
    s_playermodel.menu.wrapAround = qtrue;
    s_playermodel.menu.fullscreen = qtrue;
    s_playermodel.banner.generic.type_0 = 10i32;
    s_playermodel.banner.generic.x = 320i32;
    s_playermodel.banner.generic.y = 16i32;
    s_playermodel.banner.string =
        b"PLAYER MODEL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playermodel.banner.color = color_white.as_mut_ptr();
    s_playermodel.banner.style = 0x1i32;
    s_playermodel.framel.generic.type_0 = 6i32;
    s_playermodel.framel.generic.name =
        b"menu/art/frame1_l\x00" as *const u8 as *const libc::c_char;
    s_playermodel.framel.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_playermodel.framel.generic.x = 0i32;
    s_playermodel.framel.generic.y = 78i32;
    s_playermodel.framel.width = 256i32;
    s_playermodel.framel.height = 329i32;
    s_playermodel.framer.generic.type_0 = 6i32;
    s_playermodel.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_playermodel.framer.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_playermodel.framer.generic.x = 376i32;
    s_playermodel.framer.generic.y = 76i32;
    s_playermodel.framer.width = 256i32;
    s_playermodel.framer.height = 334i32;
    s_playermodel.ports.generic.type_0 = 6i32;
    s_playermodel.ports.generic.name =
        b"menu/art/player_models_ports\x00" as *const u8 as *const libc::c_char;
    s_playermodel.ports.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_playermodel.ports.generic.x = 50i32;
    s_playermodel.ports.generic.y = 59i32;
    s_playermodel.ports.width = 274i32;
    s_playermodel.ports.height = 274i32;
    y = 59i32;
    i = 0i32;
    k = 0i32;
    while i < 4i32 {
        x = 50i32;
        j = 0i32;
        while j < 4i32 {
            s_playermodel.pics[k as usize].generic.type_0 = 6i32;
            s_playermodel.pics[k as usize].generic.flags =
                0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
            s_playermodel.pics[k as usize].generic.x = x;
            s_playermodel.pics[k as usize].generic.y = y;
            s_playermodel.pics[k as usize].width = 64i32;
            s_playermodel.pics[k as usize].height = 64i32;
            s_playermodel.pics[k as usize].focuspic =
                b"menu/art/opponents_selected\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            s_playermodel.pics[k as usize].focuscolor = colorRed.as_mut_ptr();
            s_playermodel.picbuttons[k as usize].generic.type_0 = 6i32;
            s_playermodel.picbuttons[k as usize].generic.flags =
                0x4i32 as libc::c_uint | 0x8000i32 as libc::c_uint | 0x100i32 as libc::c_uint;
            s_playermodel.picbuttons[k as usize].generic.id = 0i32 + k;
            s_playermodel.picbuttons[k as usize].generic.callback = Some(PlayerModel_PicEvent);
            s_playermodel.picbuttons[k as usize].generic.x = x - 16i32;
            s_playermodel.picbuttons[k as usize].generic.y = y - 16i32;
            s_playermodel.picbuttons[k as usize].generic.left = x;
            s_playermodel.picbuttons[k as usize].generic.top = y;
            s_playermodel.picbuttons[k as usize].generic.right = x + 64i32;
            s_playermodel.picbuttons[k as usize].generic.bottom = y + 64i32;
            s_playermodel.picbuttons[k as usize].width = 128i32;
            s_playermodel.picbuttons[k as usize].height = 128i32;
            s_playermodel.picbuttons[k as usize].focuspic =
                b"menu/art/opponents_select\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            s_playermodel.picbuttons[k as usize].focuscolor = colorRed.as_mut_ptr();
            x += 64i32 + 6i32;
            j += 1;
            k += 1
        }
        y += 64i32 + 6i32;
        i += 1
    }
    s_playermodel.playername.generic.type_0 = 9i32;
    s_playermodel.playername.generic.flags = 0x8i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_playermodel.playername.generic.x = 320i32;
    s_playermodel.playername.generic.y = 440i32;
    s_playermodel.playername.string = playername.as_mut_ptr();
    s_playermodel.playername.style = 0x1i32;
    s_playermodel.playername.color = text_color_normal.as_mut_ptr();
    s_playermodel.modelname.generic.type_0 = 9i32;
    s_playermodel.modelname.generic.flags = 0x8i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_playermodel.modelname.generic.x = 497i32;
    s_playermodel.modelname.generic.y = 54i32;
    s_playermodel.modelname.string = modelname.as_mut_ptr();
    s_playermodel.modelname.style = 0x1i32;
    s_playermodel.modelname.color = text_color_normal.as_mut_ptr();
    s_playermodel.skinname.generic.type_0 = 9i32;
    s_playermodel.skinname.generic.flags = 0x8i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_playermodel.skinname.generic.x = 497i32;
    s_playermodel.skinname.generic.y = 394i32;
    s_playermodel.skinname.string = skinname.as_mut_ptr();
    s_playermodel.skinname.style = 0x1i32;
    s_playermodel.skinname.color = text_color_normal.as_mut_ptr();
    s_playermodel.player.generic.type_0 = 6i32;
    s_playermodel.player.generic.flags = 0x4000i32 as libc::c_uint;
    s_playermodel.player.generic.ownerdraw = Some(PlayerModel_DrawPlayer);
    s_playermodel.player.generic.x = 400i32;
    s_playermodel.player.generic.y = -40i32;
    s_playermodel.player.width = 32i32 * 10i32;
    s_playermodel.player.height = 56i32 * 10i32;
    s_playermodel.arrows.generic.type_0 = 6i32;
    s_playermodel.arrows.generic.name =
        b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char;
    s_playermodel.arrows.generic.flags = 0x4000i32 as libc::c_uint;
    s_playermodel.arrows.generic.x = 125i32;
    s_playermodel.arrows.generic.y = 340i32;
    s_playermodel.arrows.width = 128i32;
    s_playermodel.arrows.height = 32i32;
    s_playermodel.left.generic.type_0 = 6i32;
    s_playermodel.left.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_playermodel.left.generic.callback = Some(PlayerModel_MenuEvent);
    s_playermodel.left.generic.id = 100i32;
    s_playermodel.left.generic.x = 125i32;
    s_playermodel.left.generic.y = 340i32;
    s_playermodel.left.width = 64i32;
    s_playermodel.left.height = 32i32;
    s_playermodel.left.focuspic =
        b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playermodel.right.generic.type_0 = 6i32;
    s_playermodel.right.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_playermodel.right.generic.callback = Some(PlayerModel_MenuEvent);
    s_playermodel.right.generic.id = 101i32;
    s_playermodel.right.generic.x = 125i32 + 61i32;
    s_playermodel.right.generic.y = 340i32;
    s_playermodel.right.width = 64i32;
    s_playermodel.right.height = 32i32;
    s_playermodel.right.focuspic =
        b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playermodel.back.generic.type_0 = 6i32;
    s_playermodel.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_playermodel.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_playermodel.back.generic.callback = Some(PlayerModel_MenuEvent);
    s_playermodel.back.generic.id = 102i32;
    s_playermodel.back.generic.x = 0i32;
    s_playermodel.back.generic.y = 480i32 - 64i32;
    s_playermodel.back.width = 128i32;
    s_playermodel.back.height = 64i32;
    s_playermodel.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.ports as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.playername as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.modelname as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.skinname as *mut menutext_s as *mut libc::c_void,
    );
    i = 0i32;
    while i < 4i32 * 4i32 {
        Menu_AddItem(
            &mut s_playermodel.menu,
            &mut s_playermodel.pics[i as usize] as *mut menubitmap_s as *mut libc::c_void,
        );
        Menu_AddItem(
            &mut s_playermodel.menu,
            &mut s_playermodel.picbuttons[i as usize] as *mut menubitmap_s as *mut libc::c_void,
        );
        i += 1
    }
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.player as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.arrows as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.left as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.right as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playermodel.menu,
        &mut s_playermodel.back as *mut menubitmap_s as *mut libc::c_void,
    );
    PlayerModel_SetMenuItems();
    PlayerModel_UpdateGrid();
    PlayerModel_UpdateModel();
}
/*
=================
PlayerModel_UpdateModel
=================
*/
unsafe extern "C" fn PlayerModel_UpdateModel() {
    let mut viewangles: vec3_t = [0.; 3];
    let mut moveangles: vec3_t = [0.; 3];
    memset(
        &mut s_playermodel.playerinfo as *mut playerInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<playerInfo_t>() as libc::c_ulong,
    );
    viewangles[1usize] = (180i32 - 30i32) as vec_t;
    viewangles[0usize] = 0i32 as vec_t;
    viewangles[2usize] = 0i32 as vec_t;
    moveangles[2usize] = 0i32 as vec_t;
    moveangles[1usize] = moveangles[2usize];
    moveangles[0usize] = moveangles[1usize];
    UI_PlayerInfo_SetModel(
        &mut s_playermodel.playerinfo,
        s_playermodel.modelskin.as_mut_ptr(),
    );
    UI_PlayerInfo_SetInfo(
        &mut s_playermodel.playerinfo,
        LEGS_IDLE as libc::c_int,
        TORSO_STAND as libc::c_int,
        viewangles.as_mut_ptr(),
        moveangles.as_mut_ptr(),
        WP_MACHINEGUN,
        qfalse,
    );
}
/*
=================
PlayerModel_UpdateGrid
=================
*/
unsafe extern "C" fn PlayerModel_UpdateGrid() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = s_playermodel.modelpage * (4i32 * 4i32);
    i = 0i32;
    while i < 4i32 * 4i32 {
        if j < s_playermodel.nummodels {
            s_playermodel.pics[i as usize].generic.name =
                s_playermodel.modelnames[j as usize].as_mut_ptr();
            s_playermodel.picbuttons[i as usize].generic.flags &= !(0x4000i32 as libc::c_uint)
        } else {
            s_playermodel.pics[i as usize].generic.name = 0 as *const libc::c_char;
            s_playermodel.picbuttons[i as usize].generic.flags |= 0x4000i32 as libc::c_uint
        }
        s_playermodel.pics[i as usize].generic.flags &= !(0x40i32 as libc::c_uint);
        s_playermodel.pics[i as usize].shader = 0i32;
        s_playermodel.picbuttons[i as usize].generic.flags |= 0x100i32 as libc::c_uint;
        i += 1;
        j += 1
    }
    if s_playermodel.selectedmodel / (4i32 * 4i32) == s_playermodel.modelpage {
        i = s_playermodel.selectedmodel % (4i32 * 4i32);
        s_playermodel.pics[i as usize].generic.flags |= 0x40i32 as libc::c_uint;
        s_playermodel.picbuttons[i as usize].generic.flags &= !(0x100i32 as libc::c_uint)
    }
    if s_playermodel.numpages > 1i32 {
        if s_playermodel.modelpage > 0i32 {
            s_playermodel.left.generic.flags &= !(0x4000i32 as libc::c_uint)
        } else {
            s_playermodel.left.generic.flags |= 0x4000i32 as libc::c_uint
        }
        if s_playermodel.modelpage < s_playermodel.numpages - 1i32 {
            s_playermodel.right.generic.flags &= !(0x4000i32 as libc::c_uint)
        } else {
            s_playermodel.right.generic.flags |= 0x4000i32 as libc::c_uint
        }
    } else {
        s_playermodel.left.generic.flags |= 0x4000i32 as libc::c_uint;
        s_playermodel.right.generic.flags |= 0x4000i32 as libc::c_uint
    };
}
/*
=================
PlayerModel_SetMenuItems
=================
*/
unsafe extern "C" fn PlayerModel_SetMenuItems() {
    let mut i: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0;
    let mut modelskin: [libc::c_char; 64] = [0; 64];
    let mut buffptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pdest: *mut libc::c_char = 0 as *mut libc::c_char;
    trap_Cvar_VariableStringBuffer(
        b"name\x00" as *const u8 as *const libc::c_char,
        s_playermodel.playername.string,
        16i32,
    );
    Q_CleanStr(s_playermodel.playername.string);
    trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        s_playermodel.modelskin.as_mut_ptr(),
        64i32,
    );
    if strchr(s_playermodel.modelskin.as_mut_ptr(), '/' as i32).is_null() {
        Q_strcat(
            s_playermodel.modelskin.as_mut_ptr(),
            64i32,
            b"/default\x00" as *const u8 as *const libc::c_char,
        );
    }
    i = 0i32;
    while i < s_playermodel.nummodels {
        buffptr = s_playermodel.modelnames[i as usize]
            .as_mut_ptr()
            .offset(strlen(b"models/players/\x00" as *const u8 as *const libc::c_char) as isize);
        pdest = strstr(buffptr, b"icon_\x00" as *const u8 as *const libc::c_char);
        if !pdest.is_null() {
            Q_strncpyz(
                modelskin.as_mut_ptr(),
                buffptr,
                (pdest.wrapping_offset_from(buffptr) as libc::c_long + 1i32 as libc::c_long)
                    as libc::c_int,
            );
            strcat(modelskin.as_mut_ptr(), pdest.offset(5isize));
            if 0 == Q_stricmp(s_playermodel.modelskin.as_mut_ptr(), modelskin.as_mut_ptr()) {
                s_playermodel.selectedmodel = i;
                s_playermodel.modelpage = i / (4i32 * 4i32);
                maxlen = pdest.wrapping_offset_from(buffptr) as libc::c_long as libc::c_int;
                if maxlen > 16i32 {
                    maxlen = 16i32
                }
                Q_strncpyz(s_playermodel.modelname.string, buffptr, maxlen);
                Q_strupr(s_playermodel.modelname.string);
                maxlen =
                    strlen(pdest.offset(5isize)).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
                if maxlen > 16i32 {
                    maxlen = 16i32
                }
                Q_strncpyz(s_playermodel.skinname.string, pdest.offset(5isize), maxlen);
                Q_strupr(s_playermodel.skinname.string);
                break;
            }
        }
        i += 1
    }
}
/*
=================
PlayerModel_MenuEvent
=================
*/
unsafe extern "C" fn PlayerModel_MenuEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        100 => {
            if s_playermodel.modelpage > 0i32 {
                s_playermodel.modelpage -= 1;
                PlayerModel_UpdateGrid();
            }
        }
        101 => {
            if s_playermodel.modelpage < s_playermodel.numpages - 1i32 {
                s_playermodel.modelpage += 1;
                PlayerModel_UpdateGrid();
            }
        }
        102 => {
            PlayerModel_SaveChanges();
            UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
PlayerModel_SaveChanges
=================
*/
unsafe extern "C" fn PlayerModel_SaveChanges() {
    trap_Cvar_Set(
        b"model\x00" as *const u8 as *const libc::c_char,
        s_playermodel.modelskin.as_mut_ptr(),
    );
    trap_Cvar_Set(
        b"headmodel\x00" as *const u8 as *const libc::c_char,
        s_playermodel.modelskin.as_mut_ptr(),
    );
    trap_Cvar_Set(
        b"team_model\x00" as *const u8 as *const libc::c_char,
        s_playermodel.modelskin.as_mut_ptr(),
    );
    trap_Cvar_Set(
        b"team_headmodel\x00" as *const u8 as *const libc::c_char,
        s_playermodel.modelskin.as_mut_ptr(),
    );
}
/*
=================
PlayerModel_DrawPlayer
=================
*/
unsafe extern "C" fn PlayerModel_DrawPlayer(mut self_0: *mut libc::c_void) {
    let mut b: *mut menubitmap_s = 0 as *mut menubitmap_s;
    b = self_0 as *mut menubitmap_s;
    if trap_MemoryRemaining() <= 5i32 * 1024i32 * 1024i32 {
        UI_DrawProportionalString(
            (*b).generic.x,
            (*b).generic.y + (*b).height / 2i32,
            b"LOW MEMORY\x00" as *const u8 as *const libc::c_char,
            0i32,
            color_red.as_mut_ptr(),
        );
        return;
    }
    UI_DrawPlayer(
        (*b).generic.x as libc::c_float,
        (*b).generic.y as libc::c_float,
        (*b).width as libc::c_float,
        (*b).height as libc::c_float,
        &mut s_playermodel.playerinfo,
        uis.realtime / 2i32,
    );
}
/*
=================
PlayerModel_PicEvent
=================
*/
unsafe extern "C" fn PlayerModel_PicEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    let mut modelnum: libc::c_int = 0;
    let mut maxlen: libc::c_int = 0;
    let mut buffptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pdest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    if event != 3i32 {
        return;
    }
    i = 0i32;
    while i < 4i32 * 4i32 {
        s_playermodel.pics[i as usize].generic.flags &= !(0x40i32 as libc::c_uint);
        s_playermodel.picbuttons[i as usize].generic.flags |= 0x100i32 as libc::c_uint;
        i += 1
    }
    i = (*(ptr as *mut menucommon_s)).id - 0i32;
    s_playermodel.pics[i as usize].generic.flags |= 0x40i32 as libc::c_uint;
    s_playermodel.picbuttons[i as usize].generic.flags &= !(0x100i32 as libc::c_uint);
    modelnum = s_playermodel.modelpage * (4i32 * 4i32) + i;
    buffptr = s_playermodel.modelnames[modelnum as usize]
        .as_mut_ptr()
        .offset(strlen(b"models/players/\x00" as *const u8 as *const libc::c_char) as isize);
    pdest = strstr(buffptr, b"icon_\x00" as *const u8 as *const libc::c_char);
    if !pdest.is_null() {
        Q_strncpyz(
            s_playermodel.modelskin.as_mut_ptr(),
            buffptr,
            (pdest.wrapping_offset_from(buffptr) as libc::c_long + 1i32 as libc::c_long)
                as libc::c_int,
        );
        strcat(s_playermodel.modelskin.as_mut_ptr(), pdest.offset(5isize));
        maxlen = pdest.wrapping_offset_from(buffptr) as libc::c_long as libc::c_int;
        if maxlen > 16i32 {
            maxlen = 16i32
        }
        Q_strncpyz(s_playermodel.modelname.string, buffptr, maxlen);
        Q_strupr(s_playermodel.modelname.string);
        maxlen = strlen(pdest.offset(5isize)).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
        if maxlen > 16i32 {
            maxlen = 16i32
        }
        Q_strncpyz(s_playermodel.skinname.string, pdest.offset(5isize), maxlen);
        Q_strupr(s_playermodel.skinname.string);
        s_playermodel.selectedmodel = modelnum;
        if trap_MemoryRemaining() > 5i32 * 1024i32 * 1024i32 {
            PlayerModel_UpdateModel();
        }
    };
}
/*
=================
PlayerModel_MenuKey
=================
*/
unsafe extern "C" fn PlayerModel_MenuKey(mut key: libc::c_int) -> sfxHandle_t {
    let mut m: *mut menucommon_s = 0 as *mut menucommon_s;
    let mut picnum: libc::c_int = 0;
    match key {
        163 | 134 => {
            m = Menu_ItemAtCursor(&mut s_playermodel.menu) as *mut menucommon_s;
            picnum = (*m).id - 0i32;
            if picnum >= 0i32 && picnum <= 15i32 {
                if picnum > 0i32 {
                    Menu_SetCursor(&mut s_playermodel.menu, s_playermodel.menu.cursor - 1i32);
                    return menu_move_sound;
                } else if s_playermodel.modelpage > 0i32 {
                    s_playermodel.modelpage -= 1;
                    Menu_SetCursor(&mut s_playermodel.menu, s_playermodel.menu.cursor + 15i32);
                    PlayerModel_UpdateGrid();
                    return menu_move_sound;
                } else {
                    return menu_buzz_sound;
                }
            }
        }
        165 | 135 => {
            m = Menu_ItemAtCursor(&mut s_playermodel.menu) as *mut menucommon_s;
            picnum = (*m).id - 0i32;
            if picnum >= 0i32 && picnum <= 15i32 {
                if picnum < 15i32
                    && s_playermodel.modelpage * (4i32 * 4i32) + picnum + 1i32
                        < s_playermodel.nummodels
                {
                    Menu_SetCursor(&mut s_playermodel.menu, s_playermodel.menu.cursor + 1i32);
                    return menu_move_sound;
                } else if picnum == 15i32 && s_playermodel.modelpage < s_playermodel.numpages - 1i32
                {
                    s_playermodel.modelpage += 1;
                    Menu_SetCursor(&mut s_playermodel.menu, s_playermodel.menu.cursor - 15i32);
                    PlayerModel_UpdateGrid();
                    return menu_move_sound;
                } else {
                    return menu_buzz_sound;
                }
            }
        }
        179 | 27 => {
            PlayerModel_SaveChanges();
        }
        _ => {}
    }
    return Menu_DefaultKey(&mut s_playermodel.menu, key);
}
#[no_mangle]
pub unsafe extern "C" fn PlayerModel_Cache() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while !playermodel_artlist[i as usize].is_null() {
        trap_R_RegisterShaderNoMip(playermodel_artlist[i as usize]);
        i += 1
    }
    PlayerModel_BuildList();
    i = 0i32;
    while i < s_playermodel.nummodels {
        trap_R_RegisterShaderNoMip(s_playermodel.modelnames[i as usize].as_mut_ptr());
        i += 1
    }
}
/*
=================
PlayerModel_BuildList
=================
*/
unsafe extern "C" fn PlayerModel_BuildList() {
    let mut numdirs: libc::c_int = 0;
    let mut numfiles: libc::c_int = 0;
    let mut dirlist: [libc::c_char; 2048] = [0; 2048];
    let mut filelist: [libc::c_char; 2048] = [0; 2048];
    let mut skinname: [libc::c_char; 64] = [0; 64];
    let mut dirptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fileptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut dirlen: libc::c_int = 0;
    let mut filelen: libc::c_int = 0;
    let mut precache: qboolean = qfalse;
    precache = trap_Cvar_VariableValue(b"com_buildscript\x00" as *const u8 as *const libc::c_char)
        as qboolean;
    s_playermodel.modelpage = 0i32;
    s_playermodel.nummodels = 0i32;
    numdirs = trap_FS_GetFileList(
        b"models/players\x00" as *const u8 as *const libc::c_char,
        b"/\x00" as *const u8 as *const libc::c_char,
        dirlist.as_mut_ptr(),
        2048i32,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0i32;
    while i < numdirs && s_playermodel.nummodels < 256i32 {
        dirlen = strlen(dirptr) as libc::c_int;
        if 0 != dirlen && *dirptr.offset((dirlen - 1i32) as isize) as libc::c_int == '/' as i32 {
            *dirptr.offset((dirlen - 1i32) as isize) = '\u{0}' as i32 as libc::c_char
        }
        if !(0 == strcmp(dirptr, b".\x00" as *const u8 as *const libc::c_char)
            || 0 == strcmp(dirptr, b"..\x00" as *const u8 as *const libc::c_char))
        {
            numfiles = trap_FS_GetFileList(
                va(
                    b"models/players/%s\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    dirptr,
                ),
                b"tga\x00" as *const u8 as *const libc::c_char,
                filelist.as_mut_ptr(),
                2048i32,
            );
            fileptr = filelist.as_mut_ptr();
            j = 0i32;
            while j < numfiles && s_playermodel.nummodels < 256i32 {
                filelen = strlen(fileptr) as libc::c_int;
                COM_StripExtension(
                    fileptr,
                    skinname.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                );
                if 0 == Q_stricmpn(
                    skinname.as_mut_ptr(),
                    b"icon_\x00" as *const u8 as *const libc::c_char,
                    5i32,
                ) {
                    let fresh0 = s_playermodel.nummodels;
                    s_playermodel.nummodels = s_playermodel.nummodels + 1;
                    Com_sprintf(
                        s_playermodel.modelnames[fresh0 as usize].as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                            as libc::c_int,
                        b"models/players/%s/%s\x00" as *const u8 as *const libc::c_char,
                        dirptr,
                        skinname.as_mut_ptr(),
                    );
                }
                if 0 != precache as u64 {
                    trap_S_RegisterSound(
                        va(
                            b"sound/player/announce/%s_wins.wav\x00" as *const u8
                                as *const libc::c_char
                                as *mut libc::c_char,
                            skinname.as_mut_ptr(),
                        ),
                        qfalse,
                    );
                }
                j += 1;
                fileptr = fileptr.offset((filelen + 1i32) as isize)
            }
        }
        i += 1;
        dirptr = dirptr.offset((dirlen + 1i32) as isize)
    }
    s_playermodel.numpages = s_playermodel.nummodels / (4i32 * 4i32);
    if 0 != s_playermodel.nummodels % (4i32 * 4i32) {
        s_playermodel.numpages += 1
    };
}
/*
===========================================================================
Copyright (C) 1999-2005 Id Software, Inc.

This file is part of Quake III Arena source code.

Quake III Arena source code is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License as
published by the Free Software Foundation; either version 2 of the License,
or (at your option) any later version.

Quake III Arena source code is distributed in the hope that it will be
useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Quake III Arena source code; if not, write to the Free Software
Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
===========================================================================
*/
//
static mut playermodel_artlist: [*mut libc::c_char; 11] = [
    b"menu/art/back_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/opponents_select\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/opponents_selected\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/frame1_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/player_models_ports\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/gs_arrows_0\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/gs_arrows_l\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"menu/art/gs_arrows_r\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct playermodel_t {
    pub menu: menuframework_s,
    pub pics: [menubitmap_s; 16],
    pub picbuttons: [menubitmap_s; 16],
    pub framel: menubitmap_s,
    pub framer: menubitmap_s,
    pub ports: menubitmap_s,
    pub banner: menutext_s,
    pub back: menubitmap_s,
    pub player: menubitmap_s,
    pub arrows: menubitmap_s,
    pub left: menubitmap_s,
    pub right: menubitmap_s,
    pub modelname: menutext_s,
    pub skinname: menutext_s,
    pub playername: menutext_s,
    pub playerinfo: playerInfo_t,
    pub nummodels: libc::c_int,
    pub modelnames: [[libc::c_char; 128]; 256],
    pub modelpage: libc::c_int,
    pub numpages: libc::c_int,
    pub modelskin: [libc::c_char; 64],
    pub selectedmodel: libc::c_int,
}
