use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::keycodes_h::K_ALT;
pub use crate::keycodes_h::K_AUX1;
pub use crate::keycodes_h::K_AUX10;
pub use crate::keycodes_h::K_AUX11;
pub use crate::keycodes_h::K_AUX12;
pub use crate::keycodes_h::K_AUX13;
pub use crate::keycodes_h::K_AUX14;
pub use crate::keycodes_h::K_AUX15;
pub use crate::keycodes_h::K_AUX16;
pub use crate::keycodes_h::K_AUX2;
pub use crate::keycodes_h::K_AUX3;
pub use crate::keycodes_h::K_AUX4;
pub use crate::keycodes_h::K_AUX5;
pub use crate::keycodes_h::K_AUX6;
pub use crate::keycodes_h::K_AUX7;
pub use crate::keycodes_h::K_AUX8;
pub use crate::keycodes_h::K_AUX9;
pub use crate::keycodes_h::K_BACKSPACE;
pub use crate::keycodes_h::K_BREAK;
pub use crate::keycodes_h::K_CAPSLOCK;
pub use crate::keycodes_h::K_COMMAND;
pub use crate::keycodes_h::K_COMPOSE;
pub use crate::keycodes_h::K_CONSOLE;
pub use crate::keycodes_h::K_CTRL;
pub use crate::keycodes_h::K_DEL;
pub use crate::keycodes_h::K_DOWNARROW;
pub use crate::keycodes_h::K_END;
pub use crate::keycodes_h::K_ENTER;
pub use crate::keycodes_h::K_ESCAPE;
pub use crate::keycodes_h::K_EURO;
pub use crate::keycodes_h::K_F1;
pub use crate::keycodes_h::K_F10;
pub use crate::keycodes_h::K_F11;
pub use crate::keycodes_h::K_F12;
pub use crate::keycodes_h::K_F13;
pub use crate::keycodes_h::K_F14;
pub use crate::keycodes_h::K_F15;
pub use crate::keycodes_h::K_F2;
pub use crate::keycodes_h::K_F3;
pub use crate::keycodes_h::K_F4;
pub use crate::keycodes_h::K_F5;
pub use crate::keycodes_h::K_F6;
pub use crate::keycodes_h::K_F7;
pub use crate::keycodes_h::K_F8;
pub use crate::keycodes_h::K_F9;
pub use crate::keycodes_h::K_HELP;
pub use crate::keycodes_h::K_HOME;
pub use crate::keycodes_h::K_INS;
pub use crate::keycodes_h::K_JOY1;
pub use crate::keycodes_h::K_JOY10;
pub use crate::keycodes_h::K_JOY11;
pub use crate::keycodes_h::K_JOY12;
pub use crate::keycodes_h::K_JOY13;
pub use crate::keycodes_h::K_JOY14;
pub use crate::keycodes_h::K_JOY15;
pub use crate::keycodes_h::K_JOY16;
pub use crate::keycodes_h::K_JOY17;
pub use crate::keycodes_h::K_JOY18;
pub use crate::keycodes_h::K_JOY19;
pub use crate::keycodes_h::K_JOY2;
pub use crate::keycodes_h::K_JOY20;
pub use crate::keycodes_h::K_JOY21;
pub use crate::keycodes_h::K_JOY22;
pub use crate::keycodes_h::K_JOY23;
pub use crate::keycodes_h::K_JOY24;
pub use crate::keycodes_h::K_JOY25;
pub use crate::keycodes_h::K_JOY26;
pub use crate::keycodes_h::K_JOY27;
pub use crate::keycodes_h::K_JOY28;
pub use crate::keycodes_h::K_JOY29;
pub use crate::keycodes_h::K_JOY3;
pub use crate::keycodes_h::K_JOY30;
pub use crate::keycodes_h::K_JOY31;
pub use crate::keycodes_h::K_JOY32;
pub use crate::keycodes_h::K_JOY4;
pub use crate::keycodes_h::K_JOY5;
pub use crate::keycodes_h::K_JOY6;
pub use crate::keycodes_h::K_JOY7;
pub use crate::keycodes_h::K_JOY8;
pub use crate::keycodes_h::K_JOY9;
pub use crate::keycodes_h::K_KP_5;
pub use crate::keycodes_h::K_KP_DEL;
pub use crate::keycodes_h::K_KP_DOWNARROW;
pub use crate::keycodes_h::K_KP_END;
pub use crate::keycodes_h::K_KP_ENTER;
pub use crate::keycodes_h::K_KP_EQUALS;
pub use crate::keycodes_h::K_KP_HOME;
pub use crate::keycodes_h::K_KP_INS;
pub use crate::keycodes_h::K_KP_LEFTARROW;
pub use crate::keycodes_h::K_KP_MINUS;
pub use crate::keycodes_h::K_KP_NUMLOCK;
pub use crate::keycodes_h::K_KP_PGDN;
pub use crate::keycodes_h::K_KP_PGUP;
pub use crate::keycodes_h::K_KP_PLUS;
pub use crate::keycodes_h::K_KP_RIGHTARROW;
pub use crate::keycodes_h::K_KP_SLASH;
pub use crate::keycodes_h::K_KP_STAR;
pub use crate::keycodes_h::K_KP_UPARROW;
pub use crate::keycodes_h::K_LEFTARROW;
pub use crate::keycodes_h::K_MENU;
pub use crate::keycodes_h::K_MODE;
pub use crate::keycodes_h::K_MOUSE1;
pub use crate::keycodes_h::K_MOUSE2;
pub use crate::keycodes_h::K_MOUSE3;
pub use crate::keycodes_h::K_MOUSE4;
pub use crate::keycodes_h::K_MOUSE5;
pub use crate::keycodes_h::K_MWHEELDOWN;
pub use crate::keycodes_h::K_MWHEELUP;
pub use crate::keycodes_h::K_PAD0_A;
pub use crate::keycodes_h::K_PAD0_B;
pub use crate::keycodes_h::K_PAD0_BACK;
pub use crate::keycodes_h::K_PAD0_DPAD_DOWN;
pub use crate::keycodes_h::K_PAD0_DPAD_LEFT;
pub use crate::keycodes_h::K_PAD0_DPAD_RIGHT;
pub use crate::keycodes_h::K_PAD0_DPAD_UP;
pub use crate::keycodes_h::K_PAD0_GUIDE;
pub use crate::keycodes_h::K_PAD0_LEFTSHOULDER;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_LEFTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_LEFTTRIGGER;
pub use crate::keycodes_h::K_PAD0_RIGHTSHOULDER;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_CLICK;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_DOWN;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_LEFT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_RIGHT;
pub use crate::keycodes_h::K_PAD0_RIGHTSTICK_UP;
pub use crate::keycodes_h::K_PAD0_RIGHTTRIGGER;
pub use crate::keycodes_h::K_PAD0_START;
pub use crate::keycodes_h::K_PAD0_X;
pub use crate::keycodes_h::K_PAD0_Y;
pub use crate::keycodes_h::K_PAUSE;
pub use crate::keycodes_h::K_PGDN;
pub use crate::keycodes_h::K_PGUP;
pub use crate::keycodes_h::K_POWER;
pub use crate::keycodes_h::K_PRINT;
pub use crate::keycodes_h::K_RIGHTARROW;
pub use crate::keycodes_h::K_SCROLLOCK;
pub use crate::keycodes_h::K_SHIFT;
pub use crate::keycodes_h::K_SPACE;
pub use crate::keycodes_h::K_SUPER;
pub use crate::keycodes_h::K_SYSREQ;
pub use crate::keycodes_h::K_TAB;
pub use crate::keycodes_h::K_UNDO;
pub use crate::keycodes_h::K_UPARROW;
pub use crate::keycodes_h::K_WORLD_0;
pub use crate::keycodes_h::K_WORLD_1;
pub use crate::keycodes_h::K_WORLD_10;
pub use crate::keycodes_h::K_WORLD_11;
pub use crate::keycodes_h::K_WORLD_12;
pub use crate::keycodes_h::K_WORLD_13;
pub use crate::keycodes_h::K_WORLD_14;
pub use crate::keycodes_h::K_WORLD_15;
pub use crate::keycodes_h::K_WORLD_16;
pub use crate::keycodes_h::K_WORLD_17;
pub use crate::keycodes_h::K_WORLD_18;
pub use crate::keycodes_h::K_WORLD_19;
pub use crate::keycodes_h::K_WORLD_2;
pub use crate::keycodes_h::K_WORLD_20;
pub use crate::keycodes_h::K_WORLD_21;
pub use crate::keycodes_h::K_WORLD_22;
pub use crate::keycodes_h::K_WORLD_23;
pub use crate::keycodes_h::K_WORLD_24;
pub use crate::keycodes_h::K_WORLD_25;
pub use crate::keycodes_h::K_WORLD_26;
pub use crate::keycodes_h::K_WORLD_27;
pub use crate::keycodes_h::K_WORLD_28;
pub use crate::keycodes_h::K_WORLD_29;
pub use crate::keycodes_h::K_WORLD_3;
pub use crate::keycodes_h::K_WORLD_30;
pub use crate::keycodes_h::K_WORLD_31;
pub use crate::keycodes_h::K_WORLD_32;
pub use crate::keycodes_h::K_WORLD_33;
pub use crate::keycodes_h::K_WORLD_34;
pub use crate::keycodes_h::K_WORLD_35;
pub use crate::keycodes_h::K_WORLD_36;
pub use crate::keycodes_h::K_WORLD_37;
pub use crate::keycodes_h::K_WORLD_38;
pub use crate::keycodes_h::K_WORLD_39;
pub use crate::keycodes_h::K_WORLD_4;
pub use crate::keycodes_h::K_WORLD_40;
pub use crate::keycodes_h::K_WORLD_41;
pub use crate::keycodes_h::K_WORLD_42;
pub use crate::keycodes_h::K_WORLD_43;
pub use crate::keycodes_h::K_WORLD_44;
pub use crate::keycodes_h::K_WORLD_45;
pub use crate::keycodes_h::K_WORLD_46;
pub use crate::keycodes_h::K_WORLD_47;
pub use crate::keycodes_h::K_WORLD_48;
pub use crate::keycodes_h::K_WORLD_49;
pub use crate::keycodes_h::K_WORLD_5;
pub use crate::keycodes_h::K_WORLD_50;
pub use crate::keycodes_h::K_WORLD_51;
pub use crate::keycodes_h::K_WORLD_52;
pub use crate::keycodes_h::K_WORLD_53;
pub use crate::keycodes_h::K_WORLD_54;
pub use crate::keycodes_h::K_WORLD_55;
pub use crate::keycodes_h::K_WORLD_56;
pub use crate::keycodes_h::K_WORLD_57;
pub use crate::keycodes_h::K_WORLD_58;
pub use crate::keycodes_h::K_WORLD_59;
pub use crate::keycodes_h::K_WORLD_6;
pub use crate::keycodes_h::K_WORLD_60;
pub use crate::keycodes_h::K_WORLD_61;
pub use crate::keycodes_h::K_WORLD_62;
pub use crate::keycodes_h::K_WORLD_63;
pub use crate::keycodes_h::K_WORLD_64;
pub use crate::keycodes_h::K_WORLD_65;
pub use crate::keycodes_h::K_WORLD_66;
pub use crate::keycodes_h::K_WORLD_67;
pub use crate::keycodes_h::K_WORLD_68;
pub use crate::keycodes_h::K_WORLD_69;
pub use crate::keycodes_h::K_WORLD_7;
pub use crate::keycodes_h::K_WORLD_70;
pub use crate::keycodes_h::K_WORLD_71;
pub use crate::keycodes_h::K_WORLD_72;
pub use crate::keycodes_h::K_WORLD_73;
pub use crate::keycodes_h::K_WORLD_74;
pub use crate::keycodes_h::K_WORLD_75;
pub use crate::keycodes_h::K_WORLD_76;
pub use crate::keycodes_h::K_WORLD_77;
pub use crate::keycodes_h::K_WORLD_78;
pub use crate::keycodes_h::K_WORLD_79;
pub use crate::keycodes_h::K_WORLD_8;
pub use crate::keycodes_h::K_WORLD_80;
pub use crate::keycodes_h::K_WORLD_81;
pub use crate::keycodes_h::K_WORLD_82;
pub use crate::keycodes_h::K_WORLD_83;
pub use crate::keycodes_h::K_WORLD_84;
pub use crate::keycodes_h::K_WORLD_85;
pub use crate::keycodes_h::K_WORLD_86;
pub use crate::keycodes_h::K_WORLD_87;
pub use crate::keycodes_h::K_WORLD_88;
pub use crate::keycodes_h::K_WORLD_89;
pub use crate::keycodes_h::K_WORLD_9;
pub use crate::keycodes_h::K_WORLD_90;
pub use crate::keycodes_h::K_WORLD_91;
pub use crate::keycodes_h::K_WORLD_92;
pub use crate::keycodes_h::K_WORLD_93;
pub use crate::keycodes_h::K_WORLD_94;
pub use crate::keycodes_h::K_WORLD_95;
pub use crate::keycodes_h::MAX_KEYS;
pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_Argv;
pub use crate::src::q3_ui::ui_atoms::UI_DrawNamedPic;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_ProportionalStringWidth;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByMap;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetAwardLevel;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetCurrentGame;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetNumSPTiers;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo;
pub use crate::src::q3_ui::ui_gameinfo::UI_LogAwardData;
pub use crate::src::q3_ui::ui_gameinfo::UI_SetBestScore;
pub use crate::src::q3_ui::ui_gameinfo::UI_ShowTierVideo;
pub use crate::src::q3_ui::ui_gameinfo::UI_TierCompleted;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::color_yellow;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::q3_ui::ui_sparena::UI_SPArena_Start;
pub use crate::src::q3_ui::ui_sppostgame::stdlib_h::atoi;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_GetConfigString;
pub use crate::src::ui::ui_syscalls::trap_Key_SetCatcher;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_S_RegisterSound;
pub use crate::src::ui::ui_syscalls::trap_S_StartLocalSound;

pub use crate::stdlib::strtol;
pub use crate::tr_types_h::glDriverType_t;
pub use crate::tr_types_h::glHardwareType_t;
pub use crate::tr_types_h::glconfig_t;
pub use crate::tr_types_h::textureCompression_t;
pub use crate::tr_types_h::GLDRV_ICD;
pub use crate::tr_types_h::GLDRV_STANDALONE;
pub use crate::tr_types_h::GLDRV_VOODOO;
pub use crate::tr_types_h::GLHW_3DFX_2D3D;
pub use crate::tr_types_h::GLHW_GENERIC;
pub use crate::tr_types_h::GLHW_PERMEDIA2;
pub use crate::tr_types_h::GLHW_RAGEPRO;
pub use crate::tr_types_h::GLHW_RIVA128;
pub use crate::tr_types_h::TC_NONE;
pub use crate::tr_types_h::TC_S3TC;
pub use crate::tr_types_h::TC_S3TC_ARB;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::uiStatic_t;
pub use crate::ui_local_h::AWARD_ACCURACY;
pub use crate::ui_local_h::AWARD_EXCELLENT;
pub use crate::ui_local_h::AWARD_FRAGS;
pub use crate::ui_local_h::AWARD_GAUNTLET;
pub use crate::ui_local_h::AWARD_IMPRESSIVE;
pub use crate::ui_local_h::AWARD_PERFECT;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct postgameMenuInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub item_again: crate::ui_local_h::menubitmap_s,
    pub item_next: crate::ui_local_h::menubitmap_s,
    pub item_menu: crate::ui_local_h::menubitmap_s,
    pub phase: i32,
    pub ignoreKeysTime: i32,
    pub starttime: i32,
    pub scoreboardtime: i32,
    pub serverId: i32,
    pub clientNums: [i32; 8],
    pub ranks: [i32; 8],
    pub scores: [i32; 8],
    pub placeNames: [[i8; 64]; 3],
    pub level: i32,
    pub numClients: i32,
    pub won: i32,
    pub numAwards: i32,
    pub awardsEarned: [i32; 6],
    pub awardsLevels: [i32; 6],
    pub playedSound: [crate::src::qcommon::q_shared::qboolean; 6],
    pub lastTier: i32,
    pub winnerSound: crate::src::qcommon::q_shared::sfxHandle_t,
}

static mut postgameMenuInfo: postgameMenuInfo_t = postgameMenuInfo_t {
    menu: crate::ui_local_h::menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [0 as *mut libc::c_void; 64],
        draw: None,
        key: None,
        wrapAround: crate::src::qcommon::q_shared::qfalse,
        fullscreen: crate::src::qcommon::q_shared::qfalse,
        showlogo: crate::src::qcommon::q_shared::qfalse,
    },
    item_again: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *mut i8,
        errorpic: 0 as *mut i8,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *mut f32,
    },
    item_next: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *mut i8,
        errorpic: 0 as *mut i8,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *mut f32,
    },
    item_menu: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const i8,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *mut crate::ui_local_h::menuframework_s,
            menuPosition: 0,
            flags: 0,
            callback: None,
            statusbar: None,
            ownerdraw: None,
        },
        focuspic: 0 as *mut i8,
        errorpic: 0 as *mut i8,
        shader: 0,
        focusshader: 0,
        width: 0,
        height: 0,
        focuscolor: 0 as *mut f32,
    },
    phase: 0,
    ignoreKeysTime: 0,
    starttime: 0,
    scoreboardtime: 0,
    serverId: 0,
    clientNums: [0; 8],
    ranks: [0; 8],
    scores: [0; 8],
    placeNames: [[0; 64]; 3],
    level: 0,
    numClients: 0,
    won: 0,
    numAwards: 0,
    awardsEarned: [0; 6],
    awardsLevels: [0; 6],
    playedSound: [crate::src::qcommon::q_shared::qfalse; 6],
    lastTier: 0,
    winnerSound: 0,
};

static mut arenainfo: [i8; 1024] = [0; 1024];
#[no_mangle]

pub static mut ui_medalNames: [*mut i8; 6] = [
    b"Accuracy\x00" as *const u8 as *mut i8,
    b"Impressive\x00" as *const u8 as *mut i8,
    b"Excellent\x00" as *const u8 as *mut i8,
    b"Gauntlet\x00" as *const u8 as *mut i8,
    b"Frags\x00" as *const u8 as *mut i8,
    b"Perfect\x00" as *const u8 as *mut i8,
];
#[no_mangle]

pub static mut ui_medalPicNames: [*mut i8; 6] = [
    b"menu/medals/medal_accuracy\x00" as *const u8 as *mut i8,
    b"menu/medals/medal_impressive\x00" as *const u8 as *mut i8,
    b"menu/medals/medal_excellent\x00" as *const u8 as *mut i8,
    b"menu/medals/medal_gauntlet\x00" as *const u8 as *mut i8,
    b"menu/medals/medal_frags\x00" as *const u8 as *mut i8,
    b"menu/medals/medal_victory\x00" as *const u8 as *mut i8,
];
#[no_mangle]

pub static mut ui_medalSounds: [*mut i8; 6] = [
    b"sound/feedback/accuracy.wav\x00" as *const u8 as *mut i8,
    b"sound/feedback/impressive_a.wav\x00" as *const u8 as *mut i8,
    b"sound/feedback/excellent_a.wav\x00" as *const u8 as *mut i8,
    b"sound/feedback/gauntlet.wav\x00" as *const u8 as *mut i8,
    b"sound/feedback/frags.wav\x00" as *const u8 as *mut i8,
    b"sound/feedback/perfect.wav\x00" as *const u8 as *mut i8,
];
/*
=================
UI_SPPostgameMenu_AgainEvent
=================
*/

unsafe extern "C" fn UI_SPPostgameMenu_AgainEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
        b"map_restart 0\n\x00" as *const u8 as *const i8,
    );
}
/*
=================
UI_SPPostgameMenu_NextEvent
=================
*/

unsafe extern "C" fn UI_SPPostgameMenu_NextEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    let mut currentSet: i32 = 0;
    let mut levelSet: i32 = 0;
    let mut level: i32 = 0;
    let mut currentLevel: i32 = 0;
    let mut arenaInfo: *const i8 = 0 as *const i8;
    if event != 3 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
    // handle specially if we just won the training map
    if postgameMenuInfo.won == 0 {
        level = 0
    } else {
        level = postgameMenuInfo.level + 1
    }
    levelSet = level / 4;
    currentLevel = crate::src::q3_ui::ui_gameinfo::UI_GetCurrentGame();
    if currentLevel == -(1) {
        currentLevel = postgameMenuInfo.level
    }
    currentSet = currentLevel / 4;
    if levelSet > currentSet || levelSet == crate::src::q3_ui::ui_gameinfo::UI_GetNumSPTiers() {
        level = currentLevel
    }
    arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(level);
    if arenaInfo.is_null() {
        return;
    }
    crate::src::q3_ui::ui_sparena::UI_SPArena_Start(arenaInfo);
}
/*
=================
UI_SPPostgameMenu_MenuEvent
=================
*/

unsafe extern "C" fn UI_SPPostgameMenu_MenuEvent(mut _ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
        b"disconnect; levelselect\n\x00" as *const u8 as *const i8,
    );
}
/*
=================
UI_SPPostgameMenu_MenuKey
=================
*/

unsafe extern "C" fn UI_SPPostgameMenu_MenuKey(
    mut key: i32,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    if crate::src::q3_ui::ui_atoms::uis.realtime < postgameMenuInfo.ignoreKeysTime {
        return 0i32;
    }
    if postgameMenuInfo.phase == 1 {
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            b"abort_podium\n\x00" as *const u8 as *const i8,
        );
        postgameMenuInfo.phase = 2;
        postgameMenuInfo.starttime = crate::src::q3_ui::ui_atoms::uis.realtime;
        postgameMenuInfo.ignoreKeysTime = crate::src::q3_ui::ui_atoms::uis.realtime + 250;
        return 0i32;
    }
    if postgameMenuInfo.phase == 2 {
        postgameMenuInfo.phase = 3;
        postgameMenuInfo.starttime = crate::src::q3_ui::ui_atoms::uis.realtime;
        postgameMenuInfo.ignoreKeysTime = crate::src::q3_ui::ui_atoms::uis.realtime + 250;
        return 0i32;
    }
    if key == crate::keycodes_h::K_ESCAPE as i32 || key == crate::keycodes_h::K_MOUSE2 as i32 {
        return 0i32;
    }
    return crate::src::q3_ui::ui_qmenu::Menu_DefaultKey(&mut postgameMenuInfo.menu, key);
}

static mut medalLocations: [i32; 6] = [144, 448, 88, 504, 32, 560];

unsafe extern "C" fn UI_SPPostgameMenu_DrawAwardsMedals(mut max: i32) {
    let mut n: i32 = 0;
    let mut medal: i32 = 0;
    let mut amount: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut buf: [i8; 16] = [0; 16];
    let mut current_block_9: u64;
    n = 0;
    while n < max {
        x = medalLocations[n as usize];
        y = 64;
        medal = postgameMenuInfo.awardsEarned[n as usize];
        amount = postgameMenuInfo.awardsLevels[n as usize];
        crate::src::q3_ui::ui_atoms::UI_DrawNamedPic(
            x as f32,
            y as f32,
            48f32,
            48f32,
            ui_medalPicNames[medal as usize],
        );
        if medal == crate::ui_local_h::AWARD_ACCURACY as i32 {
            crate::src::qcommon::q_shared::Com_sprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 16]>() as i32,
                b"%i%%\x00" as *const u8 as *const i8,
                amount,
            );
            current_block_9 = 8236137900636309791;
        } else if amount == 1 {
            current_block_9 = 4644295000439058019;
        } else {
            crate::src::qcommon::q_shared::Com_sprintf(
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 16]>() as i32,
                b"%i\x00" as *const u8 as *const i8,
                amount,
            );
            current_block_9 = 8236137900636309791;
        }
        match current_block_9 {
            8236137900636309791 => {
                crate::src::q3_ui::ui_atoms::UI_DrawString(
                    x + 24i32,
                    y + 52i32,
                    buf.as_mut_ptr(),
                    0x1i32,
                    crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
                );
            }
            _ => {}
        }
        n += 1
    }
}

unsafe extern "C" fn UI_SPPostgameMenu_DrawAwardsPresentation(mut timer: i32) {
    let mut awardNum: i32 = 0;
    let mut atimer: i32 = 0;
    let mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    awardNum = timer / 2000;
    atimer = timer % 2000;
    color[2] = 1.0;
    color[1] = color[2];
    color[0] = color[1];
    color[3] = (2000 - atimer) as f32 / 2000f32;
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        320,
        64,
        ui_medalNames[postgameMenuInfo.awardsEarned[awardNum as usize] as usize],
        0x1,
        color.as_mut_ptr(),
    );
    UI_SPPostgameMenu_DrawAwardsMedals(awardNum + 1);
    if postgameMenuInfo.playedSound[awardNum as usize] as u64 == 0 {
        postgameMenuInfo.playedSound[awardNum as usize] = crate::src::qcommon::q_shared::qtrue;
        crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
            crate::src::ui::ui_syscalls::trap_S_RegisterSound(
                ui_medalSounds[postgameMenuInfo.awardsEarned[awardNum as usize] as usize],
                crate::src::qcommon::q_shared::qfalse,
            ),
            crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32,
        );
    };
}
/*
=================
UI_SPPostgameMenu_MenuDrawScoreLine
=================
*/

unsafe extern "C" fn UI_SPPostgameMenu_MenuDrawScoreLine(mut n: i32, mut y: i32) {
    let mut rank: i32 = 0;
    let mut name: [i8; 64] = [0; 64];
    let mut info: [i8; 1024] = [0; 1024];
    if n > postgameMenuInfo.numClients + 1 {
        n -= postgameMenuInfo.numClients + 2
    }
    if n >= postgameMenuInfo.numClients {
        return;
    }
    rank = postgameMenuInfo.ranks[n as usize];
    if rank & 0x4000 != 0 {
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            640 - 31 * 8,
            y,
            b"(tie)\x00" as *const u8 as *const i8,
            0 | 0x10,
            crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
        );
        rank &= !(0x4000)
    }
    crate::src::ui::ui_syscalls::trap_GetConfigString(
        32 + 256 + 256 + postgameMenuInfo.clientNums[n as usize],
        info.as_mut_ptr(),
        1024,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        name.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            info.as_mut_ptr(),
            b"n\x00" as *const u8 as *const i8,
        ),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    crate::src::qcommon::q_shared::Q_CleanStr(name.as_mut_ptr());
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        640 - 25 * 8,
        y,
        crate::src::qcommon::q_shared::va(
            b"#%i: %-16s %2i\x00" as *const u8 as *mut i8,
            rank + 1i32,
            name.as_mut_ptr(),
            postgameMenuInfo.scores[n as usize],
        ),
        0 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
    );
}
/*
=================
UI_SPPostgameMenu_MenuDraw
=================
*/

unsafe extern "C" fn UI_SPPostgameMenu_MenuDraw() {
    let mut timer: i32 = 0;
    let mut serverId: i32 = 0;
    let mut n: i32 = 0;
    let mut info: [i8; 1024] = [0; 1024];
    crate::src::ui::ui_syscalls::trap_GetConfigString(
        1,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    serverId = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info.as_mut_ptr(),
        b"sv_serverid\x00" as *const u8 as *const i8,
    ));
    if serverId != postgameMenuInfo.serverId {
        crate::src::q3_ui::ui_atoms::UI_PopMenu();
        return;
    }
    // phase 1
    if postgameMenuInfo.numClients > 2 {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            510i32,
            480i32 - 64i32 - 27i32,
            postgameMenuInfo.placeNames[2usize].as_mut_ptr(),
            0x1i32,
            crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
        );
    }
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        130,
        480 - 64 - 27,
        postgameMenuInfo.placeNames[1].as_mut_ptr(),
        0x1,
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        320,
        480 - 64 - 2 * 27,
        postgameMenuInfo.placeNames[0].as_mut_ptr(),
        0x1,
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr(),
    );
    if postgameMenuInfo.phase == 1 {
        timer = crate::src::q3_ui::ui_atoms::uis.realtime - postgameMenuInfo.starttime;
        if timer >= 1000 && postgameMenuInfo.winnerSound != 0 {
            crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
                postgameMenuInfo.winnerSound,
                crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32,
            );
            postgameMenuInfo.winnerSound = 0
        }
        if timer < 5000 {
            return;
        }
        postgameMenuInfo.phase = 2;
        postgameMenuInfo.starttime = crate::src::q3_ui::ui_atoms::uis.realtime
    }
    // phase 2
    if postgameMenuInfo.phase == 2 {
        timer = crate::src::q3_ui::ui_atoms::uis.realtime - postgameMenuInfo.starttime;
        if timer >= postgameMenuInfo.numAwards * 2000 {
            if timer < 5000 {
                return;
            }
            postgameMenuInfo.phase = 3;
            postgameMenuInfo.starttime = crate::src::q3_ui::ui_atoms::uis.realtime
        } else {
            UI_SPPostgameMenu_DrawAwardsPresentation(timer);
        }
    }
    // phase 3
    if postgameMenuInfo.phase == 3 {
        if crate::src::q3_ui::ui_atoms::uis.demoversion as u64 != 0 {
            if postgameMenuInfo.won == 1 && crate::src::q3_ui::ui_gameinfo::UI_ShowTierVideo(8) != 0
            {
                crate::src::ui::ui_syscalls::trap_Cvar_Set(
                    b"nextmap\x00" as *const u8 as *const i8,
                    b"\x00" as *const u8 as *const i8,
                );
                crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                    crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                    b"disconnect; cinematic demoEnd.RoQ\n\x00" as *const u8 as *const i8,
                );
                return;
            }
        } else if postgameMenuInfo.won > -(1)
            && crate::src::q3_ui::ui_gameinfo::UI_ShowTierVideo(postgameMenuInfo.won + 1) != 0
        {
            if postgameMenuInfo.won == postgameMenuInfo.lastTier {
                crate::src::ui::ui_syscalls::trap_Cvar_Set(
                    b"nextmap\x00" as *const u8 as *const i8,
                    b"\x00" as *const u8 as *const i8,
                );
                crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                    crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                    b"disconnect; cinematic end.RoQ\n\x00" as *const u8 as *const i8,
                );
                return;
            }
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_spSelection\x00" as *const u8 as *const i8,
                (postgameMenuInfo.won * 4) as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_Set(
                b"nextmap\x00" as *const u8 as *const i8,
                b"levelselect\x00" as *const u8 as *const i8,
            );
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                crate::src::qcommon::q_shared::va(
                    b"disconnect; cinematic tier%i.RoQ\n\x00" as *const u8 as *mut i8,
                    postgameMenuInfo.won + 1i32,
                ),
            );
            return;
        }
        postgameMenuInfo.item_again.generic.flags &= !(0x4000);
        postgameMenuInfo.item_next.generic.flags &= !(0x4000);
        postgameMenuInfo.item_menu.generic.flags &= !(0x4000);
        UI_SPPostgameMenu_DrawAwardsMedals(postgameMenuInfo.numAwards);
        crate::src::q3_ui::ui_qmenu::Menu_Draw(&mut postgameMenuInfo.menu);
    }
    // draw the scoreboard
    if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"ui_spScoreboard\x00" as *const u8 as *const i8,
    ) == 0.
    {
        return;
    }
    timer = crate::src::q3_ui::ui_atoms::uis.realtime - postgameMenuInfo.scoreboardtime;
    if postgameMenuInfo.numClients <= 3 {
        n = 0
    } else {
        n = timer / 1500 % (postgameMenuInfo.numClients + 2)
    }
    UI_SPPostgameMenu_MenuDrawScoreLine(n, 0);
    UI_SPPostgameMenu_MenuDrawScoreLine(n + 1, 0 + 16);
    UI_SPPostgameMenu_MenuDrawScoreLine(n + 2, 0 + 2 * 16);
}
/*
=================
UI_SPPostgameMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPPostgameMenu_Cache() {
    let mut _n: i32 = 0;
    let mut buildscript: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    buildscript = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"com_buildscript\x00" as *const u8 as *const i8,
    ) as crate::src::qcommon::q_shared::qboolean;
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/menu_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/menu_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/replay_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/replay_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/next_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/next_1\x00" as *const u8 as *const i8,
    );

    for n in 0..6 {
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(ui_medalPicNames[n as usize]);

        crate::src::ui::ui_syscalls::trap_S_RegisterSound(
            ui_medalSounds[n as usize],
            crate::src::qcommon::q_shared::qfalse,
        );
    }
    if buildscript as u64 != 0 {
        crate::src::ui::ui_syscalls::trap_S_RegisterSound(
            b"music/loss.wav\x00" as *const u8 as *const i8,
            crate::src::qcommon::q_shared::qfalse,
        );
        crate::src::ui::ui_syscalls::trap_S_RegisterSound(
            b"music/win.wav\x00" as *const u8 as *const i8,
            crate::src::qcommon::q_shared::qfalse,
        );
        crate::src::ui::ui_syscalls::trap_S_RegisterSound(
            b"sound/player/announce/youwin.wav\x00" as *const u8 as *const i8,
            crate::src::qcommon::q_shared::qfalse,
        );
    };
}
/*
=================
UI_SPPostgameMenu_Init
=================
*/

unsafe extern "C" fn UI_SPPostgameMenu_Init() {
    postgameMenuInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    postgameMenuInfo.menu.key = Some(
        UI_SPPostgameMenu_MenuKey
            as unsafe extern "C" fn(_: i32) -> crate::src::qcommon::q_shared::sfxHandle_t,
    );
    postgameMenuInfo.menu.draw = Some(UI_SPPostgameMenu_MenuDraw as unsafe extern "C" fn() -> ());
    postgameMenuInfo.ignoreKeysTime = crate::src::q3_ui::ui_atoms::uis.realtime + 1500;
    UI_SPPostgameMenu_Cache();
    postgameMenuInfo.item_menu.generic.type_0 = 6;
    postgameMenuInfo.item_menu.generic.name = b"menu/art/menu_0\x00" as *const u8 as *const i8;
    postgameMenuInfo.item_menu.generic.flags = 0x4 | 0x100 | 0x4000;
    postgameMenuInfo.item_menu.generic.x = 0;
    postgameMenuInfo.item_menu.generic.y = 480 - 64;
    postgameMenuInfo.item_menu.generic.callback = Some(
        UI_SPPostgameMenu_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    postgameMenuInfo.item_menu.generic.id = 12;
    postgameMenuInfo.item_menu.width = 128;
    postgameMenuInfo.item_menu.height = 64;
    postgameMenuInfo.item_menu.focuspic = b"menu/art/menu_1\x00" as *const u8 as *mut i8;
    postgameMenuInfo.item_again.generic.type_0 = 6;
    postgameMenuInfo.item_again.generic.name = b"menu/art/replay_0\x00" as *const u8 as *const i8;
    postgameMenuInfo.item_again.generic.flags = 0x8 | 0x100 | 0x4000;
    postgameMenuInfo.item_again.generic.x = 320;
    postgameMenuInfo.item_again.generic.y = 480 - 64;
    postgameMenuInfo.item_again.generic.callback = Some(
        UI_SPPostgameMenu_AgainEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    postgameMenuInfo.item_again.generic.id = 10;
    postgameMenuInfo.item_again.width = 128;
    postgameMenuInfo.item_again.height = 64;
    postgameMenuInfo.item_again.focuspic = b"menu/art/replay_1\x00" as *const u8 as *mut i8;
    postgameMenuInfo.item_next.generic.type_0 = 6;
    postgameMenuInfo.item_next.generic.name = b"menu/art/next_0\x00" as *const u8 as *const i8;
    postgameMenuInfo.item_next.generic.flags = 0x10 | 0x100 | 0x4000;
    postgameMenuInfo.item_next.generic.x = 640;
    postgameMenuInfo.item_next.generic.y = 480 - 64;
    postgameMenuInfo.item_next.generic.callback = Some(
        UI_SPPostgameMenu_NextEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    postgameMenuInfo.item_next.generic.id = 11;
    postgameMenuInfo.item_next.width = 128;
    postgameMenuInfo.item_next.height = 64;
    postgameMenuInfo.item_next.focuspic = b"menu/art/next_1\x00" as *const u8 as *mut i8;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut postgameMenuInfo.menu,
        &mut postgameMenuInfo.item_menu as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut postgameMenuInfo.menu,
        &mut postgameMenuInfo.item_again as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut postgameMenuInfo.menu,
        &mut postgameMenuInfo.item_next as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
}

unsafe extern "C" fn Prepname(mut index: i32) {
    let mut len: i32 = 0;
    let mut name: [i8; 64] = [0; 64];
    let mut info: [i8; 1024] = [0; 1024];
    crate::src::ui::ui_syscalls::trap_GetConfigString(
        32 + 256 + 256 + postgameMenuInfo.clientNums[index as usize],
        info.as_mut_ptr(),
        1024,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        name.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            info.as_mut_ptr(),
            b"n\x00" as *const u8 as *const i8,
        ),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    crate::src::qcommon::q_shared::Q_CleanStr(name.as_mut_ptr());
    len = crate::stdlib::strlen(name.as_mut_ptr()) as i32;
    while len != 0
        && crate::src::q3_ui::ui_atoms::UI_ProportionalStringWidth(name.as_mut_ptr()) > 256
    {
        len -= 1;
        name[len as usize] = 0
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        postgameMenuInfo.placeNames[index as usize].as_mut_ptr(),
        name.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
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
//NOTE: include the ui_public.h from the new UI
//redefine to old API version
//
// ui_qmenu.c
//
// edit field is only numbers
// steady focus
// pulse if focus
// only mouse input allowed
// skips drawing
// grays and disables
// disables any input
// skip default initialization
// edit field is all lower case
// edit field is all upper case
// callback notifications
//
// ui_mfield.c
//
//
// ui_menu.c
//
//
// ui_credits.c
//
//
// ui_ingame.c
//
//
// ui_confirm.c
//
//
// ui_setup.c
//
//
// ui_team.c
//
//
// ui_connect.c
//
//
// ui_controls2.c
//
//
// ui_demo2.c
//
//
// ui_cinematics.c
//
//
// ui_mods.c
//
//
// ui_cdkey.c
//
//
// ui_playermodel.c
//
//
// ui_playersettings.c
//
//
// ui_preferences.c
//
//
// ui_specifyleague.c
//
//
// ui_specifyserver.c
//
//
// ui_servers2.c
//
//
// ui_startserver.c
//
//
// ui_serverinfo.c
//
//
// ui_video.c
//
//
// ui_players.c
//
//FIXME ripped from cg_local.h
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// model info
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// currently in use drawing parms
// animation vars
//
// ui_atoms.c
//
//
// ui_spLevel.c
//
//
// ui_spArena.c
//
//
// ui_spPostgame.c
//
/*
=================
UI_SPPostgameMenu_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPPostgameMenu_f() {
    let mut playerGameRank: i32 = 0; // in case they ended game as a spectator
    let mut playerClientNum: i32 = 0;
    let mut _n: i32 = 0;
    let mut oldFrags: i32 = 0;
    let mut newFrags: i32 = 0;
    let mut arena: *const i8 = 0 as *const i8;
    let mut awardValues: [i32; 6] = [0; 6];
    let mut map: [i8; 64] = [0; 64];
    let mut info: [i8; 1024] = [0; 1024];
    crate::stdlib::memset(
        &mut postgameMenuInfo as *mut postgameMenuInfo_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<postgameMenuInfo_t>(),
    );
    crate::src::ui::ui_syscalls::trap_GetConfigString(
        1,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    postgameMenuInfo.serverId = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info.as_mut_ptr(),
        b"sv_serverid\x00" as *const u8 as *const i8,
    ));
    crate::src::ui::ui_syscalls::trap_GetConfigString(
        0,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        map.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            info.as_mut_ptr(),
            b"mapname\x00" as *const u8 as *const i8,
        ),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    arena = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByMap(map.as_mut_ptr());
    if arena.is_null() {
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        arenainfo.as_mut_ptr(),
        arena,
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    postgameMenuInfo.level = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        arenainfo.as_mut_ptr(),
        b"num\x00" as *const u8 as *const i8,
    ));
    postgameMenuInfo.numClients = atoi(crate::src::q3_ui::ui_atoms::UI_Argv(1));
    playerClientNum = atoi(crate::src::q3_ui::ui_atoms::UI_Argv(2));
    playerGameRank = 8;
    if postgameMenuInfo.numClients > 8 {
        postgameMenuInfo.numClients = 8
    }

    for n in 0..postgameMenuInfo.numClients {
        postgameMenuInfo.clientNums[n as usize] =
            atoi(crate::src::q3_ui::ui_atoms::UI_Argv(8 + n * 3 + 1));

        postgameMenuInfo.ranks[n as usize] =
            atoi(crate::src::q3_ui::ui_atoms::UI_Argv(8 + n * 3 + 2));

        postgameMenuInfo.scores[n as usize] =
            atoi(crate::src::q3_ui::ui_atoms::UI_Argv(8 + n * 3 + 3));

        if postgameMenuInfo.clientNums[n as usize] == playerClientNum {
            playerGameRank = (postgameMenuInfo.ranks[n as usize] & !(0x4000)) + 1
        }
    }
    crate::src::q3_ui::ui_gameinfo::UI_SetBestScore(postgameMenuInfo.level, playerGameRank);
    // process award stats and prepare presentation data
    awardValues[crate::ui_local_h::AWARD_ACCURACY as usize] =
        atoi(crate::src::q3_ui::ui_atoms::UI_Argv(3));
    awardValues[crate::ui_local_h::AWARD_IMPRESSIVE as usize] =
        atoi(crate::src::q3_ui::ui_atoms::UI_Argv(4));
    awardValues[crate::ui_local_h::AWARD_EXCELLENT as usize] =
        atoi(crate::src::q3_ui::ui_atoms::UI_Argv(5));
    awardValues[crate::ui_local_h::AWARD_GAUNTLET as usize] =
        atoi(crate::src::q3_ui::ui_atoms::UI_Argv(6));
    awardValues[crate::ui_local_h::AWARD_FRAGS as usize] =
        atoi(crate::src::q3_ui::ui_atoms::UI_Argv(7));
    awardValues[crate::ui_local_h::AWARD_PERFECT as usize] =
        atoi(crate::src::q3_ui::ui_atoms::UI_Argv(8));
    postgameMenuInfo.numAwards = 0;
    if awardValues[crate::ui_local_h::AWARD_ACCURACY as usize] >= 50 {
        crate::src::q3_ui::ui_gameinfo::UI_LogAwardData(
            crate::ui_local_h::AWARD_ACCURACY as i32,
            1,
        );
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            crate::ui_local_h::AWARD_ACCURACY as i32;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] =
            awardValues[crate::ui_local_h::AWARD_ACCURACY as usize];
        postgameMenuInfo.numAwards += 1
    }
    if awardValues[crate::ui_local_h::AWARD_IMPRESSIVE as usize] != 0 {
        crate::src::q3_ui::ui_gameinfo::UI_LogAwardData(
            crate::ui_local_h::AWARD_IMPRESSIVE as i32,
            awardValues[crate::ui_local_h::AWARD_IMPRESSIVE as usize],
        );
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            crate::ui_local_h::AWARD_IMPRESSIVE as i32;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] =
            awardValues[crate::ui_local_h::AWARD_IMPRESSIVE as usize];
        postgameMenuInfo.numAwards += 1
    }
    if awardValues[crate::ui_local_h::AWARD_EXCELLENT as usize] != 0 {
        crate::src::q3_ui::ui_gameinfo::UI_LogAwardData(
            crate::ui_local_h::AWARD_EXCELLENT as i32,
            awardValues[crate::ui_local_h::AWARD_EXCELLENT as usize],
        );
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            crate::ui_local_h::AWARD_EXCELLENT as i32;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] =
            awardValues[crate::ui_local_h::AWARD_EXCELLENT as usize];
        postgameMenuInfo.numAwards += 1
    }
    if awardValues[crate::ui_local_h::AWARD_GAUNTLET as usize] != 0 {
        crate::src::q3_ui::ui_gameinfo::UI_LogAwardData(
            crate::ui_local_h::AWARD_GAUNTLET as i32,
            awardValues[crate::ui_local_h::AWARD_GAUNTLET as usize],
        );
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            crate::ui_local_h::AWARD_GAUNTLET as i32;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] =
            awardValues[crate::ui_local_h::AWARD_GAUNTLET as usize];
        postgameMenuInfo.numAwards += 1
    }
    oldFrags =
        crate::src::q3_ui::ui_gameinfo::UI_GetAwardLevel(crate::ui_local_h::AWARD_FRAGS as i32)
            / 100;
    crate::src::q3_ui::ui_gameinfo::UI_LogAwardData(
        crate::ui_local_h::AWARD_FRAGS as i32,
        awardValues[crate::ui_local_h::AWARD_FRAGS as usize],
    );
    newFrags =
        crate::src::q3_ui::ui_gameinfo::UI_GetAwardLevel(crate::ui_local_h::AWARD_FRAGS as i32)
            / 100;
    if newFrags > oldFrags {
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            crate::ui_local_h::AWARD_FRAGS as i32;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] = newFrags * 100;
        postgameMenuInfo.numAwards += 1
    }
    if awardValues[crate::ui_local_h::AWARD_PERFECT as usize] != 0 {
        crate::src::q3_ui::ui_gameinfo::UI_LogAwardData(crate::ui_local_h::AWARD_PERFECT as i32, 1);
        postgameMenuInfo.awardsEarned[postgameMenuInfo.numAwards as usize] =
            crate::ui_local_h::AWARD_PERFECT as i32;
        postgameMenuInfo.awardsLevels[postgameMenuInfo.numAwards as usize] = 1;
        postgameMenuInfo.numAwards += 1
    }
    if playerGameRank == 1 {
        postgameMenuInfo.won =
            crate::src::q3_ui::ui_gameinfo::UI_TierCompleted(postgameMenuInfo.level)
    } else {
        postgameMenuInfo.won = -(1)
    }
    postgameMenuInfo.starttime = crate::src::q3_ui::ui_atoms::uis.realtime;
    postgameMenuInfo.scoreboardtime = crate::src::q3_ui::ui_atoms::uis.realtime;
    crate::src::ui::ui_syscalls::trap_Key_SetCatcher(0x2);
    crate::src::q3_ui::ui_atoms::uis.menusp = 0;
    UI_SPPostgameMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut postgameMenuInfo.menu);
    if playerGameRank == 1 {
        crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
            &mut postgameMenuInfo.menu,
            &mut postgameMenuInfo.item_next as *mut crate::ui_local_h::menubitmap_s
                as *mut libc::c_void,
        );
    } else {
        crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
            &mut postgameMenuInfo.menu,
            &mut postgameMenuInfo.item_again as *mut crate::ui_local_h::menubitmap_s
                as *mut libc::c_void,
        );
    }
    Prepname(0);
    Prepname(1);
    Prepname(2);
    if playerGameRank != 1 {
        postgameMenuInfo.winnerSound = crate::src::ui::ui_syscalls::trap_S_RegisterSound(
            crate::src::qcommon::q_shared::va(
                b"sound/player/announce/%s_wins.wav\x00" as *const u8 as *mut i8,
                postgameMenuInfo.placeNames[0usize].as_mut_ptr(),
            ),
            crate::src::qcommon::q_shared::qfalse,
        );
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            b"music music/loss\n\x00" as *const u8 as *const i8,
        );
    } else {
        postgameMenuInfo.winnerSound = crate::src::ui::ui_syscalls::trap_S_RegisterSound(
            b"sound/player/announce/youwin.wav\x00" as *const u8 as *const i8,
            crate::src::qcommon::q_shared::qfalse,
        );
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            b"music music/win\n\x00" as *const u8 as *const i8,
        );
    }
    postgameMenuInfo.phase = 1;
    postgameMenuInfo.lastTier = crate::src::q3_ui::ui_gameinfo::UI_GetNumSPTiers();
    if !crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo(
        b"final\x00" as *const u8 as *const i8,
    )
    .is_null()
    {
        postgameMenuInfo.lastTier += 1
    };
}
