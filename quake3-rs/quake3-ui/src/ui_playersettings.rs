#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use bg_misc::bg_itemlist;
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
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, colorYellow, g_color_table, vec3_origin,
    vectoangles, AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract,
    AnglesToAxis, AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    byte, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, va, vec3_t, vec4_t, vec_t, Com_Clamp,
    Q_CleanStr, Q_IsColorString, Q_strncpyz,
};
use stdlib::{memset, strcmp, strcpy};
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
    _tag_menuframework, lerpFrame_t, menubitmap_s, menucommon_s, menufield_s, menuframework_s,
    menulist_s, menutext_s, mfield_t, playerInfo_t, trap_Cvar_Set, trap_Cvar_SetValue,
    trap_Cvar_VariableStringBuffer, trap_Cvar_VariableValue, trap_Key_GetOverstrikeMode,
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
use ui_players::{playersettings_t, UI_DrawPlayer, UI_PlayerInfo_SetInfo, UI_PlayerInfo_SetModel};
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
// ui_playersettings.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_PlayerSettingsMenu() {
    PlayerSettings_MenuInit();
    UI_PushMenu(&mut s_playersettings.menu);
}
static mut s_playersettings: playersettings_t = playersettings_t {
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
    name: menufield_s {
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
    handicap: menulist_s {
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
    effects: menulist_s {
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
    model: menubitmap_s {
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
    fxBasePic: 0,
    fxPic: [0; 7],
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
    current_fx: 0,
    playerModel: [0; 64],
};
/*
=================
PlayerSettings_MenuInit
=================
*/
unsafe extern "C" fn PlayerSettings_MenuInit() {
    let mut y: libc::c_int = 0;
    memset(
        &mut s_playersettings as *mut playersettings_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<playersettings_t>() as libc::c_ulong,
    );
    PlayerSettings_Cache();
    s_playersettings.menu.key = Some(PlayerSettings_MenuKey);
    s_playersettings.menu.wrapAround = qtrue;
    s_playersettings.menu.fullscreen = qtrue;
    s_playersettings.banner.generic.type_0 = 10i32;
    s_playersettings.banner.generic.x = 320i32;
    s_playersettings.banner.generic.y = 16i32;
    s_playersettings.banner.string =
        b"PLAYER SETTINGS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playersettings.banner.color = color_white.as_mut_ptr();
    s_playersettings.banner.style = 0x1i32;
    s_playersettings.framel.generic.type_0 = 6i32;
    s_playersettings.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    s_playersettings.framel.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_playersettings.framel.generic.x = 0i32;
    s_playersettings.framel.generic.y = 78i32;
    s_playersettings.framel.width = 256i32;
    s_playersettings.framel.height = 329i32;
    s_playersettings.framer.generic.type_0 = 6i32;
    s_playersettings.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    s_playersettings.framer.generic.flags = 0x4i32 as libc::c_uint | 0x4000i32 as libc::c_uint;
    s_playersettings.framer.generic.x = 376i32;
    s_playersettings.framer.generic.y = 76i32;
    s_playersettings.framer.width = 256i32;
    s_playersettings.framer.height = 334i32;
    y = 144i32;
    s_playersettings.name.generic.type_0 = 4i32;
    s_playersettings.name.generic.flags = 0x8000i32 as libc::c_uint;
    s_playersettings.name.generic.ownerdraw = Some(PlayerSettings_DrawName);
    s_playersettings.name.field.widthInChars = 20i32;
    s_playersettings.name.field.maxchars = 20i32;
    s_playersettings.name.generic.x = 192i32;
    s_playersettings.name.generic.y = y;
    s_playersettings.name.generic.left = 192i32 - 8i32;
    s_playersettings.name.generic.top = y - 8i32;
    s_playersettings.name.generic.right = 192i32 + 200i32;
    s_playersettings.name.generic.bottom = y + 2i32 * 27i32;
    y += 3i32 * 27i32;
    s_playersettings.handicap.generic.type_0 = 3i32;
    s_playersettings.handicap.generic.flags = 0x8000i32 as libc::c_uint;
    s_playersettings.handicap.generic.id = 11i32;
    s_playersettings.handicap.generic.ownerdraw = Some(PlayerSettings_DrawHandicap);
    s_playersettings.handicap.generic.x = 192i32;
    s_playersettings.handicap.generic.y = y;
    s_playersettings.handicap.generic.left = 192i32 - 8i32;
    s_playersettings.handicap.generic.top = y - 8i32;
    s_playersettings.handicap.generic.right = 192i32 + 200i32;
    s_playersettings.handicap.generic.bottom = y + 2i32 * 27i32;
    s_playersettings.handicap.numitems = 20i32;
    y += 3i32 * 27i32;
    s_playersettings.effects.generic.type_0 = 3i32;
    s_playersettings.effects.generic.flags = 0x8000i32 as libc::c_uint;
    s_playersettings.effects.generic.id = 12i32;
    s_playersettings.effects.generic.ownerdraw = Some(PlayerSettings_DrawEffects);
    s_playersettings.effects.generic.x = 192i32;
    s_playersettings.effects.generic.y = y;
    s_playersettings.effects.generic.left = 192i32 - 8i32;
    s_playersettings.effects.generic.top = y - 8i32;
    s_playersettings.effects.generic.right = 192i32 + 200i32;
    s_playersettings.effects.generic.bottom = y + 2i32 * 27i32;
    s_playersettings.effects.numitems = 7i32;
    s_playersettings.model.generic.type_0 = 6i32;
    s_playersettings.model.generic.name =
        b"menu/art/model_0\x00" as *const u8 as *const libc::c_char;
    s_playersettings.model.generic.flags = 0x10i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_playersettings.model.generic.id = 14i32;
    s_playersettings.model.generic.callback = Some(PlayerSettings_MenuEvent);
    s_playersettings.model.generic.x = 640i32;
    s_playersettings.model.generic.y = 480i32 - 64i32;
    s_playersettings.model.width = 128i32;
    s_playersettings.model.height = 64i32;
    s_playersettings.model.focuspic =
        b"menu/art/model_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playersettings.player.generic.type_0 = 6i32;
    s_playersettings.player.generic.flags = 0x4000i32 as libc::c_uint;
    s_playersettings.player.generic.ownerdraw = Some(PlayerSettings_DrawPlayer);
    s_playersettings.player.generic.x = 400i32;
    s_playersettings.player.generic.y = -40i32;
    s_playersettings.player.width = 32i32 * 10i32;
    s_playersettings.player.height = 56i32 * 10i32;
    s_playersettings.back.generic.type_0 = 6i32;
    s_playersettings.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    s_playersettings.back.generic.flags = 0x4i32 as libc::c_uint | 0x100i32 as libc::c_uint;
    s_playersettings.back.generic.id = 13i32;
    s_playersettings.back.generic.callback = Some(PlayerSettings_MenuEvent);
    s_playersettings.back.generic.x = 0i32;
    s_playersettings.back.generic.y = 480i32 - 64i32;
    s_playersettings.back.width = 128i32;
    s_playersettings.back.height = 64i32;
    s_playersettings.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    s_playersettings.item_null.generic.type_0 = 6i32;
    s_playersettings.item_null.generic.flags =
        0x4i32 as libc::c_uint | 0x800i32 as libc::c_uint | 0x100000i32 as libc::c_uint;
    s_playersettings.item_null.generic.x = 0i32;
    s_playersettings.item_null.generic.y = 0i32;
    s_playersettings.item_null.width = 640i32;
    s_playersettings.item_null.height = 480i32;
    Menu_AddItem(
        &mut s_playersettings.menu,
        &mut s_playersettings.banner as *mut menutext_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playersettings.menu,
        &mut s_playersettings.framel as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playersettings.menu,
        &mut s_playersettings.framer as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playersettings.menu,
        &mut s_playersettings.name as *mut menufield_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playersettings.menu,
        &mut s_playersettings.handicap as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playersettings.menu,
        &mut s_playersettings.effects as *mut menulist_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playersettings.menu,
        &mut s_playersettings.model as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playersettings.menu,
        &mut s_playersettings.back as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playersettings.menu,
        &mut s_playersettings.player as *mut menubitmap_s as *mut libc::c_void,
    );
    Menu_AddItem(
        &mut s_playersettings.menu,
        &mut s_playersettings.item_null as *mut menubitmap_s as *mut libc::c_void,
    );
    PlayerSettings_SetMenuItems();
}
/*
=================
PlayerSettings_SetMenuItems
=================
*/
unsafe extern "C" fn PlayerSettings_SetMenuItems() {
    let mut viewangles: vec3_t = [0.; 3];
    let mut c: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    Q_strncpyz(
        s_playersettings.name.field.buffer.as_mut_ptr(),
        UI_Cvar_VariableString(b"name\x00" as *const u8 as *const libc::c_char),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
    );
    c = (trap_Cvar_VariableValue(b"color1\x00" as *const u8 as *const libc::c_char)
        - 1i32 as libc::c_float) as libc::c_int;
    if c < 0i32 || c > 6i32 {
        c = 6i32
    }
    s_playersettings.effects.curvalue = gamecodetoui[c as usize];
    memset(
        &mut s_playersettings.playerinfo as *mut playerInfo_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<playerInfo_t>() as libc::c_ulong,
    );
    viewangles[1usize] = (180i32 - 30i32) as vec_t;
    viewangles[0usize] = 0i32 as vec_t;
    viewangles[2usize] = 0i32 as vec_t;
    UI_PlayerInfo_SetModel(
        &mut s_playersettings.playerinfo,
        UI_Cvar_VariableString(b"model\x00" as *const u8 as *const libc::c_char),
    );
    UI_PlayerInfo_SetInfo(
        &mut s_playersettings.playerinfo,
        LEGS_IDLE as libc::c_int,
        TORSO_STAND as libc::c_int,
        viewangles.as_mut_ptr(),
        vec3_origin.as_mut_ptr(),
        WP_MACHINEGUN,
        qfalse,
    );
    h = Com_Clamp(
        5i32 as libc::c_float,
        100i32 as libc::c_float,
        trap_Cvar_VariableValue(b"handicap\x00" as *const u8 as *const libc::c_char),
    ) as libc::c_int;
    s_playersettings.handicap.curvalue = 20i32 - h / 5i32;
}
static mut gamecodetoui: [libc::c_int; 7] = [4i32, 2i32, 3i32, 0i32, 5i32, 1i32, 6i32];
/*
=================
PlayerSettings_MenuEvent
=================
*/
unsafe extern "C" fn PlayerSettings_MenuEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3i32 {
        return;
    }
    match (*(ptr as *mut menucommon_s)).id {
        11 => {
            trap_Cvar_Set(
                b"handicap\x00" as *const u8 as *const libc::c_char,
                va(
                    b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    100i32 - 25i32 * s_playersettings.handicap.curvalue,
                ),
            );
        }
        14 => {
            PlayerSettings_SaveChanges();
            UI_PlayerModelMenu();
        }
        13 => {
            PlayerSettings_SaveChanges();
            UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
PlayerSettings_SaveChanges
=================
*/
unsafe extern "C" fn PlayerSettings_SaveChanges() {
    trap_Cvar_Set(
        b"name\x00" as *const u8 as *const libc::c_char,
        s_playersettings.name.field.buffer.as_mut_ptr(),
    );
    trap_Cvar_SetValue(
        b"handicap\x00" as *const u8 as *const libc::c_char,
        (100i32 - s_playersettings.handicap.curvalue * 5i32) as libc::c_float,
    );
    trap_Cvar_SetValue(
        b"color1\x00" as *const u8 as *const libc::c_char,
        uitogamecode[s_playersettings.effects.curvalue as usize] as libc::c_float,
    );
}
static mut uitogamecode: [libc::c_int; 7] = [4i32, 6i32, 2i32, 3i32, 1i32, 5i32, 7i32];
/*
=================
PlayerSettings_DrawPlayer
=================
*/
unsafe extern "C" fn PlayerSettings_DrawPlayer(mut self_0: *mut libc::c_void) {
    let mut b: *mut menubitmap_s = 0 as *mut menubitmap_s;
    let mut viewangles: vec3_t = [0.; 3];
    let mut buf: [libc::c_char; 64] = [0; 64];
    trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    if strcmp(buf.as_mut_ptr(), s_playersettings.playerModel.as_mut_ptr()) != 0i32 {
        UI_PlayerInfo_SetModel(&mut s_playersettings.playerinfo, buf.as_mut_ptr());
        strcpy(s_playersettings.playerModel.as_mut_ptr(), buf.as_mut_ptr());
        viewangles[1usize] = (180i32 - 30i32) as vec_t;
        viewangles[0usize] = 0i32 as vec_t;
        viewangles[2usize] = 0i32 as vec_t;
        UI_PlayerInfo_SetInfo(
            &mut s_playersettings.playerinfo,
            LEGS_IDLE as libc::c_int,
            TORSO_STAND as libc::c_int,
            viewangles.as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            WP_MACHINEGUN,
            qfalse,
        );
    }
    b = self_0 as *mut menubitmap_s;
    UI_DrawPlayer(
        (*b).generic.x as libc::c_float,
        (*b).generic.y as libc::c_float,
        (*b).width as libc::c_float,
        (*b).height as libc::c_float,
        &mut s_playersettings.playerinfo,
        uis.realtime / 2i32,
    );
}
/*
=================
PlayerSettings_DrawEffects
=================
*/
unsafe extern "C" fn PlayerSettings_DrawEffects(mut self_0: *mut libc::c_void) {
    let mut item: *mut menulist_s = 0 as *mut menulist_s;
    let mut focus: qboolean = qfalse;
    let mut style: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    item = self_0 as *mut menulist_s;
    focus = ((*(*item).generic.parent).cursor == (*item).generic.menuPosition) as libc::c_int
        as qboolean;
    style = 0i32 | 0x10i32;
    color = text_color_normal.as_mut_ptr();
    if 0 != focus as u64 {
        style |= 0x4000i32;
        color = text_color_highlight.as_mut_ptr()
    }
    UI_DrawProportionalString(
        (*item).generic.x,
        (*item).generic.y,
        b"Effects\x00" as *const u8 as *const libc::c_char,
        style,
        color,
    );
    UI_DrawHandlePic(
        ((*item).generic.x + 64i32) as libc::c_float,
        ((*item).generic.y + 27i32 + 8i32) as libc::c_float,
        128i32 as libc::c_float,
        8i32 as libc::c_float,
        s_playersettings.fxBasePic,
    );
    UI_DrawHandlePic(
        ((*item).generic.x + 64i32 + (*item).curvalue * 16i32 + 8i32) as libc::c_float,
        ((*item).generic.y + 27i32 + 6i32) as libc::c_float,
        16i32 as libc::c_float,
        12i32 as libc::c_float,
        s_playersettings.fxPic[(*item).curvalue as usize],
    );
}
/*
=================
PlayerSettings_DrawHandicap
=================
*/
unsafe extern "C" fn PlayerSettings_DrawHandicap(mut self_0: *mut libc::c_void) {
    let mut item: *mut menulist_s = 0 as *mut menulist_s;
    let mut focus: qboolean = qfalse;
    let mut style: libc::c_int = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    item = self_0 as *mut menulist_s;
    focus = ((*(*item).generic.parent).cursor == (*item).generic.menuPosition) as libc::c_int
        as qboolean;
    style = 0i32 | 0x10i32;
    color = text_color_normal.as_mut_ptr();
    if 0 != focus as u64 {
        style |= 0x4000i32;
        color = text_color_highlight.as_mut_ptr()
    }
    UI_DrawProportionalString(
        (*item).generic.x,
        (*item).generic.y,
        b"Handicap\x00" as *const u8 as *const libc::c_char,
        style,
        color,
    );
    UI_DrawProportionalString(
        (*item).generic.x + 64i32,
        (*item).generic.y + 27i32,
        handicap_items[(*item).curvalue as usize],
        style,
        color,
    );
}
static mut handicap_items: [*const libc::c_char; 21] = [
    b"None\x00" as *const u8 as *const libc::c_char,
    b"95\x00" as *const u8 as *const libc::c_char,
    b"90\x00" as *const u8 as *const libc::c_char,
    b"85\x00" as *const u8 as *const libc::c_char,
    b"80\x00" as *const u8 as *const libc::c_char,
    b"75\x00" as *const u8 as *const libc::c_char,
    b"70\x00" as *const u8 as *const libc::c_char,
    b"65\x00" as *const u8 as *const libc::c_char,
    b"60\x00" as *const u8 as *const libc::c_char,
    b"55\x00" as *const u8 as *const libc::c_char,
    b"50\x00" as *const u8 as *const libc::c_char,
    b"45\x00" as *const u8 as *const libc::c_char,
    b"40\x00" as *const u8 as *const libc::c_char,
    b"35\x00" as *const u8 as *const libc::c_char,
    b"30\x00" as *const u8 as *const libc::c_char,
    b"25\x00" as *const u8 as *const libc::c_char,
    b"20\x00" as *const u8 as *const libc::c_char,
    b"15\x00" as *const u8 as *const libc::c_char,
    b"10\x00" as *const u8 as *const libc::c_char,
    b"5\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];
/*
=================
PlayerSettings_DrawName
=================
*/
unsafe extern "C" fn PlayerSettings_DrawName(mut self_0: *mut libc::c_void) {
    let mut f: *mut menufield_s = 0 as *mut menufield_s;
    let mut focus: qboolean = qfalse;
    let mut style: libc::c_int = 0;
    let mut txt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut color: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut n: libc::c_int = 0;
    let mut basex: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut name: [libc::c_char; 32] = [0; 32];
    f = self_0 as *mut menufield_s;
    basex = (*f).generic.x;
    y = (*f).generic.y;
    focus = ((*(*f).generic.parent).cursor == (*f).generic.menuPosition) as libc::c_int as qboolean;
    style = 0i32 | 0x10i32;
    color = text_color_normal.as_mut_ptr();
    if 0 != focus as u64 {
        style |= 0x4000i32;
        color = text_color_highlight.as_mut_ptr()
    }
    UI_DrawProportionalString(
        basex,
        y,
        b"Name\x00" as *const u8 as *const libc::c_char,
        style,
        color,
    );
    basex += 64i32;
    y += 27i32;
    txt = (*f).field.buffer.as_mut_ptr();
    color = g_color_table[('7' as i32 - '0' as i32 & 0x7i32) as usize].as_mut_ptr();
    x = basex;
    loop {
        c = *txt;
        if !(c as libc::c_int != 0i32) {
            break;
        }
        if 0 == focus as u64 && 0 != Q_IsColorString(txt) as libc::c_uint {
            n = *txt.offset(1isize) as libc::c_int - '0' as i32 & 0x7i32;
            if n == 0i32 {
                n = 7i32
            }
            color = g_color_table[n as usize].as_mut_ptr();
            txt = txt.offset(2isize)
        } else {
            UI_DrawChar(x, y, c as libc::c_int, style, color);
            txt = txt.offset(1isize);
            x += 8i32
        }
    }
    if 0 != focus as u64 {
        if 0 != trap_Key_GetOverstrikeMode() as u64 {
            c = 11i32 as libc::c_char
        } else {
            c = 10i32 as libc::c_char
        }
        style &= !0x4000i32;
        style |= 0x1000i32;
        UI_DrawChar(
            basex + (*f).field.cursor * 8i32,
            y,
            c as libc::c_int,
            style,
            color_white.as_mut_ptr(),
        );
    }
    Q_strncpyz(
        name.as_mut_ptr(),
        (*f).field.buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
    );
    Q_CleanStr(name.as_mut_ptr());
    UI_DrawProportionalString(
        320i32,
        440i32,
        name.as_mut_ptr(),
        0x1i32 | 0x20i32,
        text_color_normal.as_mut_ptr(),
    );
}
/*
=================
PlayerSettings_MenuKey
=================
*/
unsafe extern "C" fn PlayerSettings_MenuKey(mut key: libc::c_int) -> sfxHandle_t {
    if key == K_MOUSE2 as libc::c_int || key == K_ESCAPE as libc::c_int {
        PlayerSettings_SaveChanges();
    }
    return Menu_DefaultKey(&mut s_playersettings.menu, key);
}
#[no_mangle]
pub unsafe extern "C" fn PlayerSettings_Cache() {
    trap_R_RegisterShaderNoMip(b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/model_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/model_1\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_0\x00" as *const u8 as *const libc::c_char);
    trap_R_RegisterShaderNoMip(b"menu/art/back_1\x00" as *const u8 as *const libc::c_char);
    s_playersettings.fxBasePic =
        trap_R_RegisterShaderNoMip(b"menu/art/fx_base\x00" as *const u8 as *const libc::c_char);
    s_playersettings.fxPic[0usize] =
        trap_R_RegisterShaderNoMip(b"menu/art/fx_red\x00" as *const u8 as *const libc::c_char);
    s_playersettings.fxPic[1usize] =
        trap_R_RegisterShaderNoMip(b"menu/art/fx_yel\x00" as *const u8 as *const libc::c_char);
    s_playersettings.fxPic[2usize] =
        trap_R_RegisterShaderNoMip(b"menu/art/fx_grn\x00" as *const u8 as *const libc::c_char);
    s_playersettings.fxPic[3usize] =
        trap_R_RegisterShaderNoMip(b"menu/art/fx_teal\x00" as *const u8 as *const libc::c_char);
    s_playersettings.fxPic[4usize] =
        trap_R_RegisterShaderNoMip(b"menu/art/fx_blue\x00" as *const u8 as *const libc::c_char);
    s_playersettings.fxPic[5usize] =
        trap_R_RegisterShaderNoMip(b"menu/art/fx_cyan\x00" as *const u8 as *const libc::c_char);
    s_playersettings.fxPic[6usize] =
        trap_R_RegisterShaderNoMip(b"menu/art/fx_white\x00" as *const u8 as *const libc::c_char);
}
