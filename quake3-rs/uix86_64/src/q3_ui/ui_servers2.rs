use ::libc;

pub mod stdlib_h {

    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::stddef_h::size_t;

pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
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
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_confirm::UI_ConfirmMenu_Style;
pub use crate::src::q3_ui::ui_confirm::UI_Message;
pub use crate::src::q3_ui::ui_main::ui_browserGameType;
pub use crate::src::q3_ui::ui_main::ui_browserMaster;
pub use crate::src::q3_ui::ui_main::ui_browserShowEmpty;
pub use crate::src::q3_ui::ui_main::ui_browserShowFull;
pub use crate::src::q3_ui::ui_main::ui_browserSortKey;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::menu_move_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_text_color;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor;
pub use crate::src::q3_ui::ui_qmenu::ScrollList_Key;
pub use crate::src::q3_ui::ui_servers2::stdlib_h::atoi;
pub use crate::src::q3_ui::ui_specifyserver::UI_SpecifyServerMenu;
pub use crate::src::q3_ui::ui_startserver::UI_StartServerMenu;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::Q_strupr;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Register;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_LAN_ClearPing;
pub use crate::src::ui::ui_syscalls::trap_LAN_GetPing;
pub use crate::src::ui::ui_syscalls::trap_LAN_GetPingInfo;
pub use crate::src::ui::ui_syscalls::trap_LAN_GetPingQueueCount;
pub use crate::src::ui::ui_syscalls::trap_LAN_GetServerAddressString;
pub use crate::src::ui::ui_syscalls::trap_LAN_GetServerCount;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_SetPbClStatus;
pub use crate::stdlib::__compar_fn_t;

pub use crate::stdlib::qsort;

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
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menuradiobutton_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arenaservers_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub master: crate::ui_local_h::menulist_s,
    pub gametype: crate::ui_local_h::menulist_s,
    pub sortkey: crate::ui_local_h::menulist_s,
    pub showfull: crate::ui_local_h::menuradiobutton_s,
    pub showempty: crate::ui_local_h::menuradiobutton_s,
    pub list: crate::ui_local_h::menulist_s,
    pub mappic: crate::ui_local_h::menubitmap_s,
    pub arrows: crate::ui_local_h::menubitmap_s,
    pub up: crate::ui_local_h::menubitmap_s,
    pub down: crate::ui_local_h::menubitmap_s,
    pub status: crate::ui_local_h::menutext_s,
    pub statusbar: crate::ui_local_h::menutext_s,
    pub remove: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub refresh: crate::ui_local_h::menubitmap_s,
    pub specify: crate::ui_local_h::menubitmap_s,
    pub create: crate::ui_local_h::menubitmap_s,
    pub go: crate::ui_local_h::menubitmap_s,
    pub pinglist: [pinglist_t; 32],
    pub table: [table_t; 128],
    pub items: [*mut i8; 128],
    pub numqueriedservers: i32,
    pub numservers: *mut i32,
    pub serverlist: *mut servernode_t,
    pub currentping: i32,
    pub refreshservers: crate::src::qcommon::q_shared::qboolean,
    pub nextpingtime: i32,
    pub maxservers: i32,
    pub refreshtime: i32,
    pub favoriteaddresses: [[i8; 64]; 16],
    pub numfavoriteaddresses: i32,
    pub punkbuster: crate::ui_local_h::menulist_s,
    pub pblogo: crate::ui_local_h::menubitmap_s,
}

pub type servernode_t = servernode_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct servernode_s {
    pub adrstr: [i8; 64],
    pub hostname: [i8; 25],
    pub mapname: [i8; 16],
    pub numclients: i32,
    pub maxclients: i32,
    pub pingtime: i32,
    pub gametype: i32,
    pub gamename: [i8; 12],
    pub nettype: i32,
    pub minPing: i32,
    pub maxPing: i32,
    pub bPB: crate::src::qcommon::q_shared::qboolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct table_t {
    pub buff: [i8; 68],
    pub servernode: *mut servernode_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pinglist_t {
    pub adrstr: [i8; 64],
    pub start: i32,
}

static mut master_items: [*const i8; 9] = [
    b"Local\x00" as *const u8 as *const i8,
    b"Internet\x00" as *const u8 as *const i8,
    b"Master1\x00" as *const u8 as *const i8,
    b"Master2\x00" as *const u8 as *const i8,
    b"Master3\x00" as *const u8 as *const i8,
    b"Master4\x00" as *const u8 as *const i8,
    b"Master5\x00" as *const u8 as *const i8,
    b"Favorites\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut servertype_items: [*const i8; 6] = [
    b"All\x00" as *const u8 as *const i8,
    b"Free For All\x00" as *const u8 as *const i8,
    b"Team Deathmatch\x00" as *const u8 as *const i8,
    b"Tournament\x00" as *const u8 as *const i8,
    b"Capture the Flag\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut sortkey_items: [*const i8; 6] = [
    b"Server Name\x00" as *const u8 as *const i8,
    b"Map Name\x00" as *const u8 as *const i8,
    b"Open Player Spots\x00" as *const u8 as *const i8,
    b"Game Type\x00" as *const u8 as *const i8,
    b"Ping Time\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut gamenames: [*mut i8; 14] = [
    b"DM \x00" as *const u8 as *mut i8,
    b"1v1\x00" as *const u8 as *mut i8,
    b"SP \x00" as *const u8 as *mut i8,
    b"Team DM\x00" as *const u8 as *mut i8,
    b"CTF\x00" as *const u8 as *mut i8,
    b"One Flag CTF\x00" as *const u8 as *mut i8,
    b"OverLoad\x00" as *const u8 as *mut i8,
    b"Harvester\x00" as *const u8 as *mut i8,
    b"Rocket Arena 3\x00" as *const u8 as *mut i8,
    b"Q3F\x00" as *const u8 as *mut i8,
    b"Urban Terror\x00" as *const u8 as *mut i8,
    b"OSP\x00" as *const u8 as *mut i8,
    b"???\x00" as *const u8 as *mut i8,
    0 as *mut i8,
];

static mut netnames: [*mut i8; 4] = [
    b"??? \x00" as *const u8 as *mut i8,
    b"UDP \x00" as *const u8 as *mut i8,
    b"UDP6\x00" as *const u8 as *mut i8,
    0 as *mut i8,
];

static mut quake3worldMessage: [i8; 59] = [
    86, 105, 115, 105, 116, 32, 119, 119, 119, 46, 113, 117, 97, 107, 101, 51, 119, 111, 114, 108,
    100, 46, 99, 111, 109, 32, 45, 32, 78, 101, 119, 115, 44, 32, 67, 111, 109, 109, 117, 110, 105,
    116, 121, 44, 32, 69, 118, 101, 110, 116, 115, 44, 32, 70, 105, 108, 101, 115, 0,
];
#[no_mangle]

pub static mut punkbuster_items: [*const i8; 3] = [
    b"Disabled\x00" as *const u8 as *const i8,
    b"Enabled\x00" as *const u8 as *const i8,
    0 as *const i8,
];
#[no_mangle]

pub static mut punkbuster_msg: [*const i8; 5] = [
    b"PunkBuster will be\x00" as *const u8 as *const i8,
    b"disabled the next time\x00" as *const u8 as *const i8,
    b"Quake III Arena\x00" as *const u8 as *const i8,
    b"is started.\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut g_arenaservers: arenaservers_t = arenaservers_t {
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
    banner: crate::ui_local_h::menutext_s {
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
        string: 0 as *mut i8,
        style: 0,
        color: 0 as *mut f32,
    },
    master: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *mut *const i8,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    gametype: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *mut *const i8,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    sortkey: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *mut *const i8,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    showfull: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
    },
    showempty: crate::ui_local_h::menuradiobutton_s {
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
        curvalue: 0,
    },
    list: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *mut *const i8,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    mappic: crate::ui_local_h::menubitmap_s {
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
    arrows: crate::ui_local_h::menubitmap_s {
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
    up: crate::ui_local_h::menubitmap_s {
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
    down: crate::ui_local_h::menubitmap_s {
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
    status: crate::ui_local_h::menutext_s {
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
        string: 0 as *mut i8,
        style: 0,
        color: 0 as *mut f32,
    },
    statusbar: crate::ui_local_h::menutext_s {
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
        string: 0 as *mut i8,
        style: 0,
        color: 0 as *mut f32,
    },
    remove: crate::ui_local_h::menubitmap_s {
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
    back: crate::ui_local_h::menubitmap_s {
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
    refresh: crate::ui_local_h::menubitmap_s {
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
    specify: crate::ui_local_h::menubitmap_s {
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
    create: crate::ui_local_h::menubitmap_s {
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
    go: crate::ui_local_h::menubitmap_s {
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
    pinglist: [pinglist_t {
        adrstr: [0; 64],
        start: 0,
    }; 32],
    table: [table_t {
        buff: [0; 68],
        servernode: 0 as *mut servernode_t,
    }; 128],
    items: [0 as *mut i8; 128],
    numqueriedservers: 0,
    numservers: 0 as *mut i32,
    serverlist: 0 as *mut servernode_t,
    currentping: 0,
    refreshservers: crate::src::qcommon::q_shared::qfalse,
    nextpingtime: 0,
    maxservers: 0,
    refreshtime: 0,
    favoriteaddresses: [[0; 64]; 16],
    numfavoriteaddresses: 0,
    punkbuster: crate::ui_local_h::menulist_s {
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
        oldvalue: 0,
        curvalue: 0,
        numitems: 0,
        top: 0,
        itemnames: 0 as *mut *const i8,
        width: 0,
        height: 0,
        columns: 0,
        separation: 0,
    },
    pblogo: crate::ui_local_h::menubitmap_s {
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
};

static mut g_globalserverlist: [[servernode_t; 128]; 6] = [[servernode_t {
    adrstr: [0; 64],
    hostname: [0; 25],
    mapname: [0; 16],
    numclients: 0,
    maxclients: 0,
    pingtime: 0,
    gametype: 0,
    gamename: [0; 12],
    nettype: 0,
    minPing: 0,
    maxPing: 0,
    bPB: crate::src::qcommon::q_shared::qfalse,
}; 128]; 6];

static mut g_numglobalservers: [i32; 6] = [0; 6];

static mut g_localserverlist: [servernode_t; 128] = [servernode_t {
    adrstr: [0; 64],
    hostname: [0; 25],
    mapname: [0; 16],
    numclients: 0,
    maxclients: 0,
    pingtime: 0,
    gametype: 0,
    gamename: [0; 12],
    nettype: 0,
    minPing: 0,
    maxPing: 0,
    bPB: crate::src::qcommon::q_shared::qfalse,
}; 128];

static mut g_numlocalservers: i32 = 0;

static mut g_favoriteserverlist: [servernode_t; 16] = [servernode_t {
    adrstr: [0; 64],
    hostname: [0; 25],
    mapname: [0; 16],
    numclients: 0,
    maxclients: 0,
    pingtime: 0,
    gametype: 0,
    gamename: [0; 12],
    nettype: 0,
    minPing: 0,
    maxPing: 0,
    bPB: crate::src::qcommon::q_shared::qfalse,
}; 16];

static mut g_numfavoriteservers: i32 = 0;

static mut g_servertype: i32 = 0;

static mut g_gametype: i32 = 0;

static mut g_sortkey: i32 = 0;

static mut g_emptyservers: i32 = 0;

static mut g_fullservers: i32 = 0;
/*
=================
ArenaServers_MaxPing
=================
*/

unsafe extern "C" fn ArenaServers_MaxPing() -> i32 {
    let mut maxPing: i32 = 0;
    maxPing = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"cl_maxPing\x00" as *const u8 as *const i8,
    ) as i32;
    if maxPing < 100 {
        maxPing = 100
    }
    return maxPing;
}
/*
=================
ArenaServers_Compare
=================
*/

unsafe extern "C" fn ArenaServers_Compare(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> i32 {
    let mut f1: f32 = 0.;
    let mut f2: f32 = 0.;
    let mut t1: *mut servernode_t = 0 as *mut servernode_t;
    let mut t2: *mut servernode_t = 0 as *mut servernode_t;
    t1 = arg1 as *mut servernode_t;
    t2 = arg2 as *mut servernode_t;
    match g_sortkey {
        0 => {
            return crate::src::qcommon::q_shared::Q_stricmp(
                (*t1).hostname.as_mut_ptr(),
                (*t2).hostname.as_mut_ptr(),
            )
        }
        1 => {
            return crate::src::qcommon::q_shared::Q_stricmp(
                (*t1).mapname.as_mut_ptr(),
                (*t2).mapname.as_mut_ptr(),
            )
        }
        2 => {
            f1 = ((*t1).maxclients - (*t1).numclients) as f32;
            if f1 < 0f32 {
                f1 = 0f32
            }
            f2 = ((*t2).maxclients - (*t2).numclients) as f32;
            if f2 < 0f32 {
                f2 = 0f32
            }
            if f1 < f2 {
                return 1i32;
            }
            if f1 == f2 {
                return 0i32;
            }
            return -(1i32);
        }
        3 => {
            if (*t1).gametype < (*t2).gametype {
                return -(1i32);
            }
            if (*t1).gametype == (*t2).gametype {
                return 0i32;
            }
            return 1i32;
        }
        4 => {
            if (*t1).pingtime < (*t2).pingtime {
                return -(1i32);
            }
            if (*t1).pingtime > (*t2).pingtime {
                return 1i32;
            }
            return crate::src::qcommon::q_shared::Q_stricmp(
                (*t1).hostname.as_mut_ptr(),
                (*t2).hostname.as_mut_ptr(),
            );
        }
        _ => {}
    }
    return 0;
}
/*
=================
ArenaServers_SourceForLAN

Convert ui's g_servertype to AS_* used by trap calls.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_SourceForLAN() -> i32 {
    match g_servertype {
        1 | 2 | 3 | 4 | 5 | 6 => return 2,
        7 => return 3,
        0 | _ => return 0,
    };
}
/*
=================
ArenaServers_Go
=================
*/

unsafe extern "C" fn ArenaServers_Go() {
    let mut servernode: *mut servernode_t = 0 as *mut servernode_t;
    servernode = g_arenaservers.table[g_arenaservers.list.curvalue as usize].servernode;
    if !servernode.is_null() {
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            crate::src::qcommon::q_shared::va(
                b"connect %s\n\x00" as *const u8 as *mut i8,
                (*servernode).adrstr.as_mut_ptr(),
            ),
        );
    };
}
/*
=================
ArenaServers_UpdatePicture
=================
*/

unsafe extern "C" fn ArenaServers_UpdatePicture() {
    static mut picname: [i8; 64] = [0; 64];
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    if g_arenaservers.list.numitems == 0 {
        g_arenaservers.mappic.generic.name = 0 as *const i8
    } else {
        servernodeptr = g_arenaservers.table[g_arenaservers.list.curvalue as usize].servernode;
        crate::src::qcommon::q_shared::Com_sprintf(
            picname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"levelshots/%s.tga\x00" as *const u8 as *const i8,
            (*servernodeptr).mapname.as_mut_ptr(),
        );
        g_arenaservers.mappic.generic.name = picname.as_mut_ptr()
    }
    // force shader update during draw
    g_arenaservers.mappic.shader = 0;
}
/*
=================
ArenaServers_UpdateMenu
=================
*/

unsafe extern "C" fn ArenaServers_UpdateMenu() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut count: i32 = 0;
    let mut buff: *mut i8 = 0 as *mut i8;
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    let mut tableptr: *mut table_t = 0 as *mut table_t;
    let mut pingColor: *mut i8 = 0 as *mut i8;
    if g_arenaservers.numqueriedservers > 0 {
        // servers found
        if g_arenaservers.refreshservers != 0
            && g_arenaservers.currentping <= g_arenaservers.numqueriedservers
        {
            // show progress
            crate::src::qcommon::q_shared::Com_sprintf(
                g_arenaservers.status.string,
                64,
                b"%d of %d Arena Servers.\x00" as *const u8 as *const i8,
                g_arenaservers.currentping,
                g_arenaservers.numqueriedservers,
            );
            g_arenaservers.statusbar.string = b"Press SPACE to stop\x00" as *const u8 as *mut i8;
            crate::stdlib::qsort(
                g_arenaservers.serverlist as *mut libc::c_void,
                *g_arenaservers.numservers as crate::stddef_h::size_t,
                ::std::mem::size_of::<servernode_t>(),
                Some(
                    ArenaServers_Compare
                        as unsafe extern "C" fn(
                            _: *const libc::c_void,
                            _: *const libc::c_void,
                        ) -> i32,
                ),
            );
        } else {
            // all servers pinged - enable controls
            g_arenaservers.gametype.generic.flags &= !(0x2000);
            g_arenaservers.sortkey.generic.flags &= !(0x2000);
            g_arenaservers.showempty.generic.flags &= !(0x2000);
            g_arenaservers.showfull.generic.flags &= !(0x2000);
            g_arenaservers.list.generic.flags &= !(0x2000);
            g_arenaservers.refresh.generic.flags &= !(0x2000);
            g_arenaservers.go.generic.flags &= !(0x2000);
            g_arenaservers.punkbuster.generic.flags &= !(0x2000);
            // update status bar
            if g_servertype >= 1 && g_servertype <= 6 {
                g_arenaservers.statusbar.string = quake3worldMessage.as_mut_ptr()
            } else {
                g_arenaservers.statusbar.string = b"\x00" as *const u8 as *mut i8
            }
        }
    } else {
        // no servers found
        if g_arenaservers.refreshservers as u64 != 0 {
            crate::stdlib::strcpy(
                g_arenaservers.status.string,
                b"Scanning For Servers.\x00" as *const u8 as *const i8,
            );
            g_arenaservers.statusbar.string = b"Press SPACE to stop\x00" as *const u8 as *mut i8;
            // disable controls during refresh
            g_arenaservers.gametype.generic.flags |= 0x2000;
            g_arenaservers.sortkey.generic.flags |= 0x2000;
            g_arenaservers.showempty.generic.flags |= 0x2000;
            g_arenaservers.showfull.generic.flags |= 0x2000;
            g_arenaservers.list.generic.flags |= 0x2000;
            g_arenaservers.refresh.generic.flags |= 0x2000;
            g_arenaservers.go.generic.flags |= 0x2000;
            g_arenaservers.punkbuster.generic.flags |= 0x2000
        } else {
            if g_arenaservers.numqueriedservers < 0 {
                crate::stdlib::strcpy(
                    g_arenaservers.status.string,
                    b"No Response From Master Server.\x00" as *const u8 as *const i8,
                );
            } else {
                crate::stdlib::strcpy(
                    g_arenaservers.status.string,
                    b"No Servers Found.\x00" as *const u8 as *const i8,
                );
            }
            // update status bar
            if g_servertype >= 1 && g_servertype <= 6 {
                g_arenaservers.statusbar.string = quake3worldMessage.as_mut_ptr()
            } else {
                g_arenaservers.statusbar.string = b"\x00" as *const u8 as *mut i8
            }
            // end of refresh - set control state
            g_arenaservers.master.generic.flags &= !(0x2000);
            g_arenaservers.gametype.generic.flags &= !(0x2000);
            g_arenaservers.sortkey.generic.flags &= !(0x2000);
            g_arenaservers.showempty.generic.flags &= !(0x2000);
            g_arenaservers.showfull.generic.flags &= !(0x2000);
            g_arenaservers.list.generic.flags |= 0x2000;
            g_arenaservers.refresh.generic.flags &= !(0x2000);
            g_arenaservers.go.generic.flags |= 0x2000;
            g_arenaservers.punkbuster.generic.flags &= !(0x2000)
        }
        // zero out list box
        g_arenaservers.list.numitems = 0;
        g_arenaservers.list.curvalue = 0;
        g_arenaservers.list.top = 0;
        // update picture
        ArenaServers_UpdatePicture();
        return;
    }
    // build list box strings - apply culling filters
    servernodeptr = g_arenaservers.serverlist;
    count = *g_arenaservers.numservers;
    let mut current_block_80: u64;
    i = 0;
    j = 0;
    while i < count {
        tableptr = &mut *g_arenaservers.table.as_mut_ptr().offset(j as isize) as *mut table_t;
        (*tableptr).servernode = servernodeptr;
        buff = (*tableptr).buff.as_mut_ptr();
        // can only cull valid results
        if !(g_emptyservers == 0 && (*servernodeptr).numclients == 0) {
            if !(g_fullservers == 0 && (*servernodeptr).numclients == (*servernodeptr).maxclients) {
                match g_gametype {
                    1 => {
                        if (*servernodeptr).gametype != crate::bg_public_h::GT_FFA as i32 {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    2 => {
                        if (*servernodeptr).gametype != crate::bg_public_h::GT_TEAM as i32 {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    3 => {
                        if (*servernodeptr).gametype != crate::bg_public_h::GT_TOURNAMENT as i32 {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    4 => {
                        if (*servernodeptr).gametype != crate::bg_public_h::GT_CTF as i32 {
                            current_block_80 = 13325891313334703151;
                        } else {
                            current_block_80 = 17441561948628420366;
                        }
                    }
                    0 | _ => {
                        current_block_80 = 17441561948628420366;
                    }
                }
                match current_block_80 {
                    13325891313334703151 => {}
                    _ => {
                        if (*servernodeptr).pingtime < (*servernodeptr).minPing {
                            pingColor = b"^4\x00" as *const u8 as *mut i8
                        } else if (*servernodeptr).maxPing != 0
                            && (*servernodeptr).pingtime > (*servernodeptr).maxPing
                        {
                            pingColor = b"^4\x00" as *const u8 as *mut i8
                        } else if (*servernodeptr).pingtime < 200 {
                            pingColor = b"^2\x00" as *const u8 as *mut i8
                        } else if (*servernodeptr).pingtime < 400 {
                            pingColor = b"^3\x00" as *const u8 as *mut i8
                        } else {
                            pingColor = b"^1\x00" as *const u8 as *mut i8
                        }
                        crate::src::qcommon::q_shared::Com_sprintf(
                            buff,
                            68,
                            b"%-20.20s %-12.12s %2d/%2d %-8.8s %4s%s%3d ^3%s\x00" as *const u8
                                as *const i8,
                            (*servernodeptr).hostname.as_mut_ptr(),
                            (*servernodeptr).mapname.as_mut_ptr(),
                            (*servernodeptr).numclients,
                            (*servernodeptr).maxclients,
                            (*servernodeptr).gamename.as_mut_ptr(),
                            netnames[(*servernodeptr).nettype as usize],
                            pingColor,
                            (*servernodeptr).pingtime,
                            if (*servernodeptr).bPB != 0 {
                                b"Yes\x00" as *const u8 as *const i8
                            } else {
                                b"No\x00" as *const u8 as *const i8
                            },
                        );
                        j += 1
                    }
                }
            }
        }
        i += 1;
        servernodeptr = servernodeptr.offset(1)
    }
    g_arenaservers.list.numitems = j;
    g_arenaservers.list.curvalue = 0;
    g_arenaservers.list.top = 0;
    // update picture
    ArenaServers_UpdatePicture();
}
/*
=================
ArenaServers_Remove
=================
*/

unsafe extern "C" fn ArenaServers_Remove() {
    let mut i: i32 = 0;
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    let mut tableptr: *mut table_t = 0 as *mut table_t;
    if g_arenaservers.list.numitems == 0 {
        return;
    }
    // remove selected item from display list
    // items are in scattered order due to sort and cull
    // perform delete on list box contents, resync all lists
    tableptr = &mut *g_arenaservers
        .table
        .as_mut_ptr()
        .offset(g_arenaservers.list.curvalue as isize) as *mut table_t;
    servernodeptr = (*tableptr).servernode;
    // find address in master list
    i = 0;
    while i < g_arenaservers.numfavoriteaddresses {
        if crate::src::qcommon::q_shared::Q_stricmp(
            g_arenaservers.favoriteaddresses[i as usize].as_mut_ptr(),
            (*servernodeptr).adrstr.as_mut_ptr(),
        ) == 0
        {
            // delete address from master list
            if i < g_arenaservers.numfavoriteaddresses - 1 {
                // shift items up
                crate::stdlib::memcpy(
                    &mut *g_arenaservers
                        .favoriteaddresses
                        .as_mut_ptr()
                        .offset(i as isize) as *mut [i8; 64]
                        as *mut libc::c_void,
                    &mut *g_arenaservers
                        .favoriteaddresses
                        .as_mut_ptr()
                        .offset((i + 1i32) as isize) as *mut [i8; 64]
                        as *const libc::c_void,
                    ((g_arenaservers.numfavoriteaddresses - i - 1i32) * 64i32) as usize,
                );
            }
            g_arenaservers.numfavoriteaddresses -= 1;
            crate::stdlib::memset(
                &mut *g_arenaservers
                    .favoriteaddresses
                    .as_mut_ptr()
                    .offset(g_arenaservers.numfavoriteaddresses as isize)
                    as *mut [i8; 64] as *mut libc::c_void,
                0,
                64,
            );
            break;
        } else {
            i += 1
        }
    }
    // find address in server list
    i = 0;
    while i < g_numfavoriteservers {
        if &mut *g_favoriteserverlist.as_mut_ptr().offset(i as isize) as *mut servernode_t
            == servernodeptr
        {
            // delete address from server list
            if i < g_numfavoriteservers - 1 {
                // shift items up
                crate::stdlib::memcpy(
                    &mut *g_favoriteserverlist.as_mut_ptr().offset(i as isize) as *mut servernode_t
                        as *mut libc::c_void,
                    &mut *g_favoriteserverlist
                        .as_mut_ptr()
                        .offset((i + 1i32) as isize) as *mut servernode_t
                        as *const libc::c_void,
                    ((g_numfavoriteservers - i - 1i32) as usize)
                        .wrapping_mul(::std::mem::size_of::<servernode_t>()),
                );
            }
            g_numfavoriteservers -= 1;
            crate::stdlib::memset(
                &mut *g_favoriteserverlist
                    .as_mut_ptr()
                    .offset(g_numfavoriteservers as isize) as *mut servernode_t
                    as *mut libc::c_void,
                0,
                ::std::mem::size_of::<servernode_t>(),
            );
            break;
        } else {
            i += 1
        }
    }
    g_arenaservers.numqueriedservers = g_arenaservers.numfavoriteaddresses;
    g_arenaservers.currentping = g_arenaservers.numfavoriteaddresses;
}
/*
=================
ArenaServers_Insert
=================
*/

unsafe extern "C" fn ArenaServers_Insert(
    mut adrstr: *mut i8,
    mut info: *mut i8,
    mut pingtime: i32,
) {
    let mut servernodeptr: *mut servernode_t = 0 as *mut servernode_t;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    if pingtime >= ArenaServers_MaxPing() && g_servertype != 7 {
        // slow global or local servers do not get entered
        return;
    }
    if *g_arenaservers.numservers >= g_arenaservers.maxservers {
        // list full;
        servernodeptr = g_arenaservers
            .serverlist
            .offset(*g_arenaservers.numservers as isize)
            .offset(-(1))
    } else {
        // next slot
        servernodeptr = g_arenaservers
            .serverlist
            .offset(*g_arenaservers.numservers as isize);
        *g_arenaservers.numservers += 1
    }
    crate::src::qcommon::q_shared::Q_strncpyz((*servernodeptr).adrstr.as_mut_ptr(), adrstr, 64);
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*servernodeptr).hostname.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"hostname\x00" as *const u8 as *const i8,
        ),
        22,
    );
    crate::src::qcommon::q_shared::Q_CleanStr((*servernodeptr).hostname.as_mut_ptr());
    crate::src::qcommon::q_shared::Q_strupr((*servernodeptr).hostname.as_mut_ptr());
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*servernodeptr).mapname.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"mapname\x00" as *const u8 as *const i8,
        ),
        16,
    );
    crate::src::qcommon::q_shared::Q_CleanStr((*servernodeptr).mapname.as_mut_ptr());
    crate::src::qcommon::q_shared::Q_strupr((*servernodeptr).mapname.as_mut_ptr());
    (*servernodeptr).numclients = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"clients\x00" as *const u8 as *const i8,
    ));
    (*servernodeptr).maxclients = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"sv_maxclients\x00" as *const u8 as *const i8,
    ));
    (*servernodeptr).pingtime = pingtime;
    (*servernodeptr).minPing = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"minPing\x00" as *const u8 as *const i8,
    ));
    (*servernodeptr).maxPing = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"maxPing\x00" as *const u8 as *const i8,
    ));
    (*servernodeptr).bPB = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"punkbuster\x00" as *const u8 as *const i8,
    )) as crate::src::qcommon::q_shared::qboolean;
    /*
    s = Info_ValueForKey( info, "nettype" );
    for (i=0; ;i++)
    {
        if (!netnames[i])
        {
            servernodeptr->nettype = 0;
            break;
        }
        else if (!Q_stricmp( netnames[i], s ))
        {
            servernodeptr->nettype = i;
            break;
        }
    }
    */
    (*servernodeptr).nettype = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"nettype\x00" as *const u8 as *const i8,
    )); //-1;
    if (*servernodeptr).nettype < 0
        || (*servernodeptr).nettype as usize
            >= (::std::mem::size_of::<[*mut i8; 4]>())
                .wrapping_div(::std::mem::size_of::<*mut i8>())
                .wrapping_sub(1usize)
    {
        (*servernodeptr).nettype = 0
    }
    s = crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"game\x00" as *const u8 as *const i8,
    );
    i = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"gametype\x00" as *const u8 as *const i8,
    ));
    if i < 0 {
        i = 0
    } else if i > 11 {
        i = 12
    }
    if *s != 0 {
        (*servernodeptr).gametype = i;
        crate::src::qcommon::q_shared::Q_strncpyz(
            (*servernodeptr).gamename.as_mut_ptr(),
            s,
            ::std::mem::size_of::<[i8; 12]>() as i32,
        );
    } else {
        (*servernodeptr).gametype = i;
        crate::src::qcommon::q_shared::Q_strncpyz(
            (*servernodeptr).gamename.as_mut_ptr(),
            gamenames[i as usize],
            ::std::mem::size_of::<[i8; 12]>() as i32,
        );
    };
}
/*
=================
ArenaServers_LoadFavorites

Load cvar address book entries into local lists.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_LoadFavorites() {
    let mut _i: i32 = 0;
    let mut j: i32 = 0;
    let mut numtempitems: i32 = 0;
    let mut adrstr: [i8; 64] = [0; 64];
    let mut templist: [servernode_t; 16] = [servernode_t {
        adrstr: [0; 64],
        hostname: [0; 25],
        mapname: [0; 16],
        numclients: 0,
        maxclients: 0,
        pingtime: 0,
        gametype: 0,
        gamename: [0; 12],
        nettype: 0,
        minPing: 0,
        maxPing: 0,
        bPB: crate::src::qcommon::q_shared::qfalse,
    }; 16];
    let mut found: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    found = crate::src::qcommon::q_shared::qfalse;
    // copy the old
    crate::stdlib::memcpy(
        templist.as_mut_ptr() as *mut libc::c_void,
        g_favoriteserverlist.as_mut_ptr() as *const libc::c_void,
        (::std::mem::size_of::<servernode_t>()).wrapping_mul(16usize),
    );
    numtempitems = g_numfavoriteservers;
    // clear the current for sync
    crate::stdlib::memset(
        g_favoriteserverlist.as_mut_ptr() as *mut libc::c_void,
        0,
        (::std::mem::size_of::<servernode_t>()).wrapping_mul(16usize),
    );
    g_numfavoriteservers = 0;

    for i in 0..16 {
        crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
            crate::src::qcommon::q_shared::va(b"server%d\x00" as *const u8 as *mut i8, i + 1i32),
            adrstr.as_mut_ptr(),
            64,
        );

        if !(adrstr[0] == 0) {
            // favorite server addresses must be maintained outside refresh list
            // this mimics local and global netadr's stored in client
            // these can be fetched to fill ping list
            crate::stdlib::strcpy(
                g_arenaservers.favoriteaddresses[g_numfavoriteservers as usize].as_mut_ptr(),
                adrstr.as_mut_ptr(),
            );
            // find this server in the old list
            j = 0;
            while j < numtempitems {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    templist[j as usize].adrstr.as_mut_ptr(),
                    adrstr.as_mut_ptr(),
                ) == 0
                {
                    break;
                }
                j += 1
            }
            if j < numtempitems {
                // found server - add exisiting results
                crate::stdlib::memcpy(
                    &mut *g_favoriteserverlist
                        .as_mut_ptr()
                        .offset(g_numfavoriteservers as isize)
                        as *mut servernode_t as *mut libc::c_void,
                    &mut *templist.as_mut_ptr().offset(j as isize) as *mut servernode_t
                        as *const libc::c_void,
                    ::std::mem::size_of::<servernode_t>(),
                );
                found = crate::src::qcommon::q_shared::qtrue
            } else {
                // add new server
                crate::src::qcommon::q_shared::Q_strncpyz(
                    g_favoriteserverlist[g_numfavoriteservers as usize]
                        .adrstr
                        .as_mut_ptr(),
                    adrstr.as_mut_ptr(),
                    64,
                );
                g_favoriteserverlist[g_numfavoriteservers as usize].pingtime =
                    ArenaServers_MaxPing()
            }
            g_numfavoriteservers += 1
        }
    }
    g_arenaservers.numfavoriteaddresses = g_numfavoriteservers;
    if found as u64 == 0 {
        // no results were found, reset server list
        // list will be automatically refreshed when selected
        g_numfavoriteservers = 0
    };
}
/*
=================
ArenaServers_StopRefresh
=================
*/

unsafe extern "C" fn ArenaServers_StopRefresh() {
    if g_arenaservers.refreshservers as u64 == 0 {
        // not currently refreshing
        return;
    }
    g_arenaservers.refreshservers = crate::src::qcommon::q_shared::qfalse;
    // final tally
    if g_arenaservers.numqueriedservers >= 0 {
        g_arenaservers.currentping = *g_arenaservers.numservers;
        g_arenaservers.numqueriedservers = *g_arenaservers.numservers
    }
    // sort
    crate::stdlib::qsort(
        g_arenaservers.serverlist as *mut libc::c_void,
        *g_arenaservers.numservers as crate::stddef_h::size_t,
        ::std::mem::size_of::<servernode_t>(),
        Some(
            ArenaServers_Compare
                as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
        ),
    );
    ArenaServers_UpdateMenu();
}
/*
=================
ArenaServers_DoRefresh
=================
*/

unsafe extern "C" fn ArenaServers_DoRefresh() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut time: i32 = 0;
    let mut maxPing: i32 = 0;
    let mut adrstr: [i8; 64] = [0; 64];
    let mut info: [i8; 1024] = [0; 1024];
    if crate::src::q3_ui::ui_atoms::uis.realtime < g_arenaservers.refreshtime {
        if g_servertype != 7 {
            if g_servertype == 0 {
                if crate::src::ui::ui_syscalls::trap_LAN_GetServerCount(0) == 0 {
                    return;
                }
            }
            if crate::src::ui::ui_syscalls::trap_LAN_GetServerCount(ArenaServers_SourceForLAN()) < 0
            {
                // still waiting for response
                return;
            }
        }
    } else if g_servertype == 0 {
        if crate::src::ui::ui_syscalls::trap_LAN_GetServerCount(0) == 0 {
            // no local servers found, check again
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                b"localservers\n\x00" as *const u8 as *const i8,
            );
            g_arenaservers.refreshtime = crate::src::q3_ui::ui_atoms::uis.realtime + 5000;
            return;
        }
    }
    if crate::src::q3_ui::ui_atoms::uis.realtime < g_arenaservers.nextpingtime {
        // wait for time trigger
        return;
    }
    // trigger at 10Hz intervals
    g_arenaservers.nextpingtime = crate::src::q3_ui::ui_atoms::uis.realtime + 10;
    // process ping results
    maxPing = ArenaServers_MaxPing();
    let mut current_block_41: u64;
    i = 0;
    while i < 32 {
        crate::src::ui::ui_syscalls::trap_LAN_GetPing(i, adrstr.as_mut_ptr(), 64, &mut time);
        if !(adrstr[0] == 0) {
            // find ping result in our local list
            j = 0;
            while j < 32 {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    adrstr.as_mut_ptr(),
                    g_arenaservers.pinglist[j as usize].adrstr.as_mut_ptr(),
                ) == 0
                {
                    break;
                }
                j += 1
            }
            if j < 32 {
                // found it
                if time == 0 {
                    time = crate::src::q3_ui::ui_atoms::uis.realtime
                        - g_arenaservers.pinglist[j as usize].start;
                    if time < maxPing {
                        current_block_41 = 9828876828309294594;
                    } else {
                        current_block_41 = 6417057564578538666;
                    }
                } else {
                    current_block_41 = 6417057564578538666;
                }
                match current_block_41 {
                    9828876828309294594 => {}
                    _ => {
                        if time > maxPing {
                            // stale it out
                            info[0] = '\u{0}' as i8;
                            time = maxPing;
                            // set hostname for nonresponsive favorite server
                            if g_servertype == 7 {
                                crate::src::qcommon::q_shared::Info_SetValueForKey(
                                    info.as_mut_ptr(),
                                    b"hostname\x00" as *const u8 as *const i8,
                                    adrstr.as_mut_ptr(),
                                );
                                crate::src::qcommon::q_shared::Info_SetValueForKey(
                                    info.as_mut_ptr(),
                                    b"game\x00" as *const u8 as *const i8,
                                    b"???\x00" as *const u8 as *const i8,
                                );
                            }
                        } else {
                            crate::src::ui::ui_syscalls::trap_LAN_GetPingInfo(
                                i,
                                info.as_mut_ptr(),
                                1024i32,
                            );
                        }
                        // insert ping results
                        ArenaServers_Insert(adrstr.as_mut_ptr(), info.as_mut_ptr(), time);
                        // clear this query from internal list
                        g_arenaservers.pinglist[j as usize].adrstr[0] = '\u{0}' as i8;
                        current_block_41 = 1924505913685386279;
                    }
                }
            } else {
                current_block_41 = 1924505913685386279;
            }
            match current_block_41 {
                9828876828309294594 => {}
                _ => {
                    // clear this query from external list
                    crate::src::ui::ui_syscalls::trap_LAN_ClearPing(i);
                }
            }
        }
        // still waiting
        i += 1
    }
    // get results of servers query
    // counts can increase as servers respond
    if g_servertype == 7 {
        g_arenaservers.numqueriedservers = g_arenaservers.numfavoriteaddresses
    } else {
        g_arenaservers.numqueriedservers =
            crate::src::ui::ui_syscalls::trap_LAN_GetServerCount(ArenaServers_SourceForLAN())
    }
    //	if (g_arenaservers.numqueriedservers > g_arenaservers.maxservers)
    //		g_arenaservers.numqueriedservers = g_arenaservers.maxservers;
    // send ping requests in reasonable bursts
    // iterate ping through all found servers
    i = 0;
    while i < 32 && g_arenaservers.currentping < g_arenaservers.numqueriedservers {
        if crate::src::ui::ui_syscalls::trap_LAN_GetPingQueueCount() >= 32 {
            // ping queue is full
            break;
        } else {
            // find empty slot
            j = 0;
            while j < 32 {
                if g_arenaservers.pinglist[j as usize].adrstr[0] == 0 {
                    break;
                }
                j += 1
            }
            if j >= 32 {
                break;
            }
            // get an address to ping
            if g_servertype == 7 {
                crate::stdlib::strcpy(
                    adrstr.as_mut_ptr(),
                    g_arenaservers.favoriteaddresses[g_arenaservers.currentping as usize]
                        .as_mut_ptr(),
                );
            } else {
                crate::src::ui::ui_syscalls::trap_LAN_GetServerAddressString(
                    ArenaServers_SourceForLAN(),
                    g_arenaservers.currentping,
                    adrstr.as_mut_ptr(),
                    64i32,
                );
            }
            crate::stdlib::strcpy(
                g_arenaservers.pinglist[j as usize].adrstr.as_mut_ptr(),
                adrstr.as_mut_ptr(),
            );
            g_arenaservers.pinglist[j as usize].start = crate::src::q3_ui::ui_atoms::uis.realtime;
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_NOW as i32,
                crate::src::qcommon::q_shared::va(
                    b"ping %s\n\x00" as *const u8 as *mut i8,
                    adrstr.as_mut_ptr(),
                ),
            );
            // advance to next server
            g_arenaservers.currentping += 1;
            i += 1
        }
    }
    if crate::src::ui::ui_syscalls::trap_LAN_GetPingQueueCount() == 0 {
        // all pings completed
        ArenaServers_StopRefresh();
        return;
    }
    // update the user interface with ping status
    ArenaServers_UpdateMenu();
}
/*
=================
ArenaServers_StartRefresh
=================
*/

unsafe extern "C" fn ArenaServers_StartRefresh() {
    let mut _i: i32 = 0;
    let mut myargs: [i8; 32] = [0; 32];
    let mut protocol: [i8; 32] = [0; 32];
    crate::stdlib::memset(
        g_arenaservers.serverlist as *mut libc::c_void,
        0,
        (g_arenaservers.maxservers as usize).wrapping_mul(::std::mem::size_of::<table_t>()),
    );

    for i in 0..32 {
        g_arenaservers.pinglist[i as usize].adrstr[0] = '\u{0}' as i8;

        crate::src::ui::ui_syscalls::trap_LAN_ClearPing(i);
    }
    g_arenaservers.refreshservers = crate::src::qcommon::q_shared::qtrue;
    g_arenaservers.currentping = 0;
    g_arenaservers.nextpingtime = 0;
    *g_arenaservers.numservers = 0;
    g_arenaservers.numqueriedservers = 0;
    // allow max 5 seconds for responses
    g_arenaservers.refreshtime = crate::src::q3_ui::ui_atoms::uis.realtime + 5000;
    // place menu in zeroed state
    ArenaServers_UpdateMenu();
    if g_servertype == 0 {
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            b"localservers\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    if g_servertype >= 1 && g_servertype <= 6 {
        match g_arenaservers.gametype.curvalue {
            1 => {
                crate::stdlib::strcpy(myargs.as_mut_ptr(), b" ffa\x00" as *const u8 as *const i8);
            }
            2 => {
                crate::stdlib::strcpy(myargs.as_mut_ptr(), b" team\x00" as *const u8 as *const i8);
            }
            3 => {
                crate::stdlib::strcpy(
                    myargs.as_mut_ptr(),
                    b" tourney\x00" as *const u8 as *const i8,
                );
            }
            4 => {
                crate::stdlib::strcpy(myargs.as_mut_ptr(), b" ctf\x00" as *const u8 as *const i8);
            }
            0 | _ => myargs[0] = 0,
        }
        if g_emptyservers != 0 {
            crate::stdlib::strcat(myargs.as_mut_ptr(), b" empty\x00" as *const u8 as *const i8);
        }
        if g_fullservers != 0 {
            crate::stdlib::strcat(myargs.as_mut_ptr(), b" full\x00" as *const u8 as *const i8);
        }
        protocol[0] = '\u{0}' as i8;
        crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
            b"debug_protocol\x00" as *const u8 as *const i8,
            protocol.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 32]>() as i32,
        );
        if crate::stdlib::strlen(protocol.as_mut_ptr()) != 0 {
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                crate::src::qcommon::q_shared::va(
                    b"globalservers %d %s%s\n\x00" as *const u8 as *mut i8,
                    g_servertype - 1i32,
                    protocol.as_mut_ptr(),
                    myargs.as_mut_ptr(),
                ),
            );
        } else {
            crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                crate::src::qcommon::q_shared::va(
                    b"globalservers %d %d%s\n\x00" as *const u8 as *mut i8,
                    g_servertype - 1i32,
                    crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                        b"protocol\x00" as *const u8 as *const i8,
                    ) as i32,
                    myargs.as_mut_ptr(),
                ),
            );
        }
    };
}
/*
=================
ArenaServers_SaveChanges
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_SaveChanges() {
    let mut i: i32 = 0;
    i = 0;
    while i < g_arenaservers.numfavoriteaddresses {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            crate::src::qcommon::q_shared::va(b"server%d\x00" as *const u8 as *mut i8, i + 1i32),
            g_arenaservers.favoriteaddresses[i as usize].as_mut_ptr(),
        );
        i += 1
    }
    while i < 16 {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            crate::src::qcommon::q_shared::va(b"server%d\x00" as *const u8 as *mut i8, i + 1i32),
            b"\x00" as *const u8 as *const i8,
        );
        i += 1
    }
}
/*
=================
ArenaServers_Sort
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_Sort(mut type_0: i32) {
    if g_sortkey == type_0 {
        return;
    }
    g_sortkey = type_0;
    crate::stdlib::qsort(
        g_arenaservers.serverlist as *mut libc::c_void,
        *g_arenaservers.numservers as crate::stddef_h::size_t,
        ::std::mem::size_of::<servernode_t>(),
        Some(
            ArenaServers_Compare
                as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
        ),
    );
}
/*
=================
ArenaServers_SetType
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_SetType(mut type_0: i32) -> i32 {
    ArenaServers_StopRefresh();
    if type_0 >= 2 && type_0 <= 6 {
        let mut masterstr: [i8; 2] = [0; 2];
        let mut cvarname: [i8; 11] = [0; 11];
        let mut direction: i32 = 0;
        if type_0 == g_servertype || type_0 == (g_servertype + 1) % 8 {
            direction = 1
        } else {
            direction = -(1)
        }
        while type_0 >= 2 && type_0 <= 6 {
            crate::src::qcommon::q_shared::Com_sprintf(
                cvarname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 11]>() as i32,
                b"sv_master%d\x00" as *const u8 as *const i8,
                type_0 - 1i32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
                cvarname.as_mut_ptr(),
                masterstr.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 2]>() as i32,
            );
            if *masterstr.as_mut_ptr() != 0 {
                break;
            }
            type_0 += direction
        }
    }
    g_servertype = type_0;
    match type_0 {
        1 | 2 | 3 | 4 | 5 | 6 => {
            g_arenaservers.remove.generic.flags |= 0x4000 | 0x1000;
            g_arenaservers.serverlist = g_globalserverlist[(type_0 - 1i32) as usize].as_mut_ptr();
            g_arenaservers.numservers = &mut *g_numglobalservers
                .as_mut_ptr()
                .offset((type_0 - 1i32) as isize)
                as *mut i32;
            g_arenaservers.maxservers = 128
        }
        7 => {
            g_arenaservers.remove.generic.flags &= !(0x4000 | 0x1000);
            g_arenaservers.serverlist = g_favoriteserverlist.as_mut_ptr();
            g_arenaservers.numservers = &mut g_numfavoriteservers;
            g_arenaservers.maxservers = 16
        }
        0 | _ => {
            g_arenaservers.remove.generic.flags |= 0x4000 | 0x1000;
            g_arenaservers.serverlist = g_localserverlist.as_mut_ptr();
            g_arenaservers.numservers = &mut g_numlocalservers;
            g_arenaservers.maxservers = 128
        }
    }
    if *g_arenaservers.numservers == 0 {
        ArenaServers_StartRefresh();
    } else {
        // avoid slow operation, use existing results
        g_arenaservers.currentping = *g_arenaservers.numservers;
        g_arenaservers.numqueriedservers = *g_arenaservers.numservers;
        ArenaServers_UpdateMenu();
        crate::stdlib::strcpy(
            g_arenaservers.status.string,
            b"hit refresh to update\x00" as *const u8 as *const i8,
        );
    }
    return type_0;
}
/*
=================
PunkBuster_Confirm
=================
*/

unsafe extern "C" fn Punkbuster_ConfirmEnable(mut result: crate::src::qcommon::q_shared::qboolean) {
    if result as u64 != 0 {
        crate::src::ui::ui_syscalls::trap_SetPbClStatus(1i32);
    }
    g_arenaservers.punkbuster.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        1f32,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"cl_punkbuster\x00" as *const u8 as *const i8,
        ),
    ) as i32;
}

unsafe extern "C" fn Punkbuster_ConfirmDisable(
    mut result: crate::src::qcommon::q_shared::qboolean,
) {
    if result as u64 != 0 {
        crate::src::ui::ui_syscalls::trap_SetPbClStatus(0);
        crate::src::q3_ui::ui_confirm::UI_Message(punkbuster_msg.as_mut_ptr());
    }
    g_arenaservers.punkbuster.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        1f32,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"cl_punkbuster\x00" as *const u8 as *const i8,
        ),
    ) as i32;
}
/*
=================
ArenaServers_Event
=================
*/

unsafe extern "C" fn ArenaServers_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut id: i32 = 0;
    id = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id;
    if event != 3 && id != 15 {
        return;
    }
    match id {
        10 => {
            g_arenaservers.master.curvalue = ArenaServers_SetType(g_arenaservers.master.curvalue);
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_browserMaster\x00" as *const u8 as *const i8,
                g_arenaservers.master.curvalue as f32,
            );
        }
        11 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_browserGameType\x00" as *const u8 as *const i8,
                g_arenaservers.gametype.curvalue as f32,
            );
            g_gametype = g_arenaservers.gametype.curvalue;
            ArenaServers_UpdateMenu();
        }
        12 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_browserSortKey\x00" as *const u8 as *const i8,
                g_arenaservers.sortkey.curvalue as f32,
            );
            ArenaServers_Sort(g_arenaservers.sortkey.curvalue);
            ArenaServers_UpdateMenu();
        }
        13 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_browserShowFull\x00" as *const u8 as *const i8,
                g_arenaservers.showfull.curvalue as f32,
            );
            g_fullservers = g_arenaservers.showfull.curvalue;
            ArenaServers_UpdateMenu();
        }
        14 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_browserShowEmpty\x00" as *const u8 as *const i8,
                g_arenaservers.showempty.curvalue as f32,
            );
            g_emptyservers = g_arenaservers.showempty.curvalue;
            ArenaServers_UpdateMenu();
        }
        15 => {
            if event == 1 {
                ArenaServers_UpdatePicture();
            }
        }
        16 => {
            crate::src::q3_ui::ui_qmenu::ScrollList_Key(
                &mut g_arenaservers.list,
                crate::keycodes_h::K_UPARROW as i32,
            );
        }
        17 => {
            crate::src::q3_ui::ui_qmenu::ScrollList_Key(
                &mut g_arenaservers.list,
                crate::keycodes_h::K_DOWNARROW as i32,
            );
        }
        18 => {
            ArenaServers_StopRefresh();
            ArenaServers_SaveChanges();
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        19 => {
            ArenaServers_StartRefresh();
        }
        20 => {
            crate::src::q3_ui::ui_specifyserver::UI_SpecifyServerMenu();
        }
        21 => {
            crate::src::q3_ui::ui_startserver::UI_StartServerMenu(
                crate::src::qcommon::q_shared::qtrue,
            );
        }
        22 => {
            ArenaServers_Go();
        }
        23 => {
            ArenaServers_Remove();
            ArenaServers_UpdateMenu();
        }
        24 => {
            if g_arenaservers.punkbuster.curvalue != 0 {
                crate::src::q3_ui::ui_confirm::UI_ConfirmMenu_Style(
                    b"Enable Punkbuster?\x00" as *const u8 as *const i8,
                    0x1i32 | 0x2000i32 | 0x10i32,
                    None,
                    Some(
                        Punkbuster_ConfirmEnable
                            as unsafe extern "C" fn(
                                _: crate::src::qcommon::q_shared::qboolean,
                            ) -> (),
                    ),
                );
            } else {
                crate::src::q3_ui::ui_confirm::UI_ConfirmMenu_Style(
                    b"Disable Punkbuster?\x00" as *const u8 as *const i8,
                    0x1i32 | 0x2000i32 | 0x10i32,
                    None,
                    Some(
                        Punkbuster_ConfirmDisable
                            as unsafe extern "C" fn(
                                _: crate::src::qcommon::q_shared::qboolean,
                            ) -> (),
                    ),
                );
            }
        }
        _ => {}
    };
}
/*
=================
ArenaServers_MenuDraw
=================
*/

unsafe extern "C" fn ArenaServers_MenuDraw() {
    if g_arenaservers.refreshservers as u64 != 0 {
        ArenaServers_DoRefresh();
    }
    crate::src::q3_ui::ui_qmenu::Menu_Draw(&mut g_arenaservers.menu);
}
/*
=================
ArenaServers_MenuKey
=================
*/

unsafe extern "C" fn ArenaServers_MenuKey(
    mut key: i32,
) -> crate::src::qcommon::q_shared::sfxHandle_t {
    if key == crate::keycodes_h::K_SPACE as i32 && g_arenaservers.refreshservers != 0 {
        ArenaServers_StopRefresh();
        return crate::src::q3_ui::ui_qmenu::menu_move_sound;
    }
    if (key == crate::keycodes_h::K_DEL as i32 || key == crate::keycodes_h::K_KP_DEL as i32)
        && g_servertype == 7
        && crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor(&mut g_arenaservers.menu)
            == &mut g_arenaservers.list as *mut crate::ui_local_h::menulist_s as *mut libc::c_void
    {
        ArenaServers_Remove();
        ArenaServers_UpdateMenu();
        return crate::src::q3_ui::ui_qmenu::menu_move_sound;
    }
    if key == crate::keycodes_h::K_MOUSE2 as i32 || key == crate::keycodes_h::K_ESCAPE as i32 {
        ArenaServers_StopRefresh();
        ArenaServers_SaveChanges();
    }
    return crate::src::q3_ui::ui_qmenu::Menu_DefaultKey(&mut g_arenaservers.menu, key);
}
/*
=================
ArenaServers_MenuInit
=================
*/

unsafe extern "C" fn ArenaServers_MenuInit() {
    let mut _i: i32 = 0;
    let mut y: i32 = 0;
    static mut statusbuffer: [i8; 64] = [0; 64];
    // zero set all our globals
    crate::stdlib::memset(
        &mut g_arenaservers as *mut arenaservers_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<arenaservers_t>(),
    );
    ArenaServers_Cache();
    g_arenaservers.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    g_arenaservers.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    g_arenaservers.menu.draw = Some(ArenaServers_MenuDraw as unsafe extern "C" fn() -> ());
    g_arenaservers.menu.key = Some(
        ArenaServers_MenuKey
            as unsafe extern "C" fn(_: i32) -> crate::src::qcommon::q_shared::sfxHandle_t,
    );
    g_arenaservers.banner.generic.type_0 = 10;
    g_arenaservers.banner.generic.flags = 0x8;
    g_arenaservers.banner.generic.x = 320;
    g_arenaservers.banner.generic.y = 16;
    g_arenaservers.banner.string = b"ARENA SERVERS\x00" as *const u8 as *mut i8;
    g_arenaservers.banner.style = 0x1;
    g_arenaservers.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    y = 80;
    g_arenaservers.master.generic.type_0 = 3;
    g_arenaservers.master.generic.name = b"Servers:\x00" as *const u8 as *const i8;
    g_arenaservers.master.generic.flags = 0x100 | 0x2;
    g_arenaservers.master.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.master.generic.id = 10;
    g_arenaservers.master.generic.x = 320;
    g_arenaservers.master.generic.y = y;
    g_arenaservers.master.itemnames = master_items.as_mut_ptr();
    y += 16;
    g_arenaservers.gametype.generic.type_0 = 3;
    g_arenaservers.gametype.generic.name = b"Game Type:\x00" as *const u8 as *const i8;
    g_arenaservers.gametype.generic.flags = 0x100 | 0x2;
    g_arenaservers.gametype.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.gametype.generic.id = 11;
    g_arenaservers.gametype.generic.x = 320;
    g_arenaservers.gametype.generic.y = y;
    g_arenaservers.gametype.itemnames = servertype_items.as_mut_ptr();
    y += 16;
    g_arenaservers.sortkey.generic.type_0 = 3;
    g_arenaservers.sortkey.generic.name = b"Sort By:\x00" as *const u8 as *const i8;
    g_arenaservers.sortkey.generic.flags = 0x100 | 0x2;
    g_arenaservers.sortkey.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.sortkey.generic.id = 12;
    g_arenaservers.sortkey.generic.x = 320;
    g_arenaservers.sortkey.generic.y = y;
    g_arenaservers.sortkey.itemnames = sortkey_items.as_mut_ptr();
    y += 16;
    g_arenaservers.showfull.generic.type_0 = 5;
    g_arenaservers.showfull.generic.name = b"Show Full:\x00" as *const u8 as *const i8;
    g_arenaservers.showfull.generic.flags = 0x100 | 0x2;
    g_arenaservers.showfull.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.showfull.generic.id = 13;
    g_arenaservers.showfull.generic.x = 320;
    g_arenaservers.showfull.generic.y = y;
    y += 16;
    g_arenaservers.showempty.generic.type_0 = 5;
    g_arenaservers.showempty.generic.name = b"Show Empty:\x00" as *const u8 as *const i8;
    g_arenaservers.showempty.generic.flags = 0x100 | 0x2;
    g_arenaservers.showempty.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.showempty.generic.id = 14;
    g_arenaservers.showempty.generic.x = 320;
    g_arenaservers.showempty.generic.y = y;
    y += 3 * 16;
    g_arenaservers.list.generic.type_0 = 8;
    g_arenaservers.list.generic.flags = 0x80;
    g_arenaservers.list.generic.id = 15;
    g_arenaservers.list.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.list.generic.x = 72;
    g_arenaservers.list.generic.y = y;
    g_arenaservers.list.width = 68;
    g_arenaservers.list.height = 11;
    g_arenaservers.list.itemnames = g_arenaservers.items.as_mut_ptr() as *mut *const i8;

    for i in 0..128 {
        g_arenaservers.items[i as usize] = g_arenaservers.table[i as usize].buff.as_mut_ptr();
    }
    g_arenaservers.mappic.generic.type_0 = 6;
    g_arenaservers.mappic.generic.flags = 0x4 | 0x4000;
    g_arenaservers.mappic.generic.x = 72;
    g_arenaservers.mappic.generic.y = 80;
    g_arenaservers.mappic.width = 128;
    g_arenaservers.mappic.height = 96;
    g_arenaservers.mappic.errorpic = b"menu/art/unknownmap\x00" as *const u8 as *mut i8;
    g_arenaservers.arrows.generic.type_0 = 6;
    g_arenaservers.arrows.generic.name = b"menu/art/arrows_vert_0\x00" as *const u8 as *const i8;
    g_arenaservers.arrows.generic.flags = 0x4 | 0x4000;
    g_arenaservers.arrows.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.arrows.generic.x = 512 + 48;
    g_arenaservers.arrows.generic.y = 240 - 64 + 16;
    g_arenaservers.arrows.width = 64;
    g_arenaservers.arrows.height = 128;
    g_arenaservers.up.generic.type_0 = 6;
    g_arenaservers.up.generic.flags = 0x4 | 0x100 | 0x800;
    g_arenaservers.up.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.up.generic.id = 16;
    g_arenaservers.up.generic.x = 512 + 48;
    g_arenaservers.up.generic.y = 240 - 64 + 16;
    g_arenaservers.up.width = 64;
    g_arenaservers.up.height = 64;
    g_arenaservers.up.focuspic = b"menu/art/arrows_vert_top\x00" as *const u8 as *mut i8;
    g_arenaservers.down.generic.type_0 = 6;
    g_arenaservers.down.generic.flags = 0x4 | 0x100 | 0x800;
    g_arenaservers.down.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.down.generic.id = 17;
    g_arenaservers.down.generic.x = 512 + 48;
    g_arenaservers.down.generic.y = 240 + 16;
    g_arenaservers.down.width = 64;
    g_arenaservers.down.height = 64;
    g_arenaservers.down.focuspic = b"menu/art/arrows_vert_bot\x00" as *const u8 as *mut i8;
    y = 376;
    g_arenaservers.status.generic.type_0 = 7;
    g_arenaservers.status.generic.x = 320;
    g_arenaservers.status.generic.y = y;
    g_arenaservers.status.string = statusbuffer.as_mut_ptr();
    g_arenaservers.status.style = 0x1 | 0x10;
    g_arenaservers.status.color = crate::src::q3_ui::ui_qmenu::menu_text_color.as_mut_ptr();
    y += 16;
    g_arenaservers.statusbar.generic.type_0 = 7;
    g_arenaservers.statusbar.generic.x = 320;
    g_arenaservers.statusbar.generic.y = y;
    g_arenaservers.statusbar.string = b"\x00" as *const u8 as *mut i8;
    g_arenaservers.statusbar.style = 0x1 | 0x10;
    g_arenaservers.statusbar.color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr();
    g_arenaservers.remove.generic.type_0 = 6;
    g_arenaservers.remove.generic.name = b"menu/art/delete_0\x00" as *const u8 as *const i8;
    g_arenaservers.remove.generic.flags = 0x4 | 0x100;
    g_arenaservers.remove.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.remove.generic.id = 23;
    g_arenaservers.remove.generic.x = 450;
    g_arenaservers.remove.generic.y = 86;
    g_arenaservers.remove.width = 96;
    g_arenaservers.remove.height = 48;
    g_arenaservers.remove.focuspic = b"menu/art/delete_1\x00" as *const u8 as *mut i8;
    g_arenaservers.back.generic.type_0 = 6;
    g_arenaservers.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    g_arenaservers.back.generic.flags = 0x4 | 0x100;
    g_arenaservers.back.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.back.generic.id = 18;
    g_arenaservers.back.generic.x = 0;
    g_arenaservers.back.generic.y = 480 - 64;
    g_arenaservers.back.width = 128;
    g_arenaservers.back.height = 64;
    g_arenaservers.back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    g_arenaservers.specify.generic.type_0 = 6;
    g_arenaservers.specify.generic.name = b"menu/art/specify_0\x00" as *const u8 as *const i8;
    g_arenaservers.specify.generic.flags = 0x4 | 0x100;
    g_arenaservers.specify.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.specify.generic.id = 20;
    g_arenaservers.specify.generic.x = 128;
    g_arenaservers.specify.generic.y = 480 - 64;
    g_arenaservers.specify.width = 128;
    g_arenaservers.specify.height = 64;
    g_arenaservers.specify.focuspic = b"menu/art/specify_1\x00" as *const u8 as *mut i8;
    g_arenaservers.refresh.generic.type_0 = 6;
    g_arenaservers.refresh.generic.name = b"menu/art/refresh_0\x00" as *const u8 as *const i8;
    g_arenaservers.refresh.generic.flags = 0x4 | 0x100;
    g_arenaservers.refresh.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.refresh.generic.id = 19;
    g_arenaservers.refresh.generic.x = 256;
    g_arenaservers.refresh.generic.y = 480 - 64;
    g_arenaservers.refresh.width = 128;
    g_arenaservers.refresh.height = 64;
    g_arenaservers.refresh.focuspic = b"menu/art/refresh_1\x00" as *const u8 as *mut i8;
    g_arenaservers.create.generic.type_0 = 6;
    g_arenaservers.create.generic.name = b"menu/art/create_0\x00" as *const u8 as *const i8;
    g_arenaservers.create.generic.flags = 0x4 | 0x100;
    g_arenaservers.create.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.create.generic.id = 21;
    g_arenaservers.create.generic.x = 384;
    g_arenaservers.create.generic.y = 480 - 64;
    g_arenaservers.create.width = 128;
    g_arenaservers.create.height = 64;
    g_arenaservers.create.focuspic = b"menu/art/create_1\x00" as *const u8 as *mut i8;
    g_arenaservers.go.generic.type_0 = 6;
    g_arenaservers.go.generic.name = b"menu/art/fight_0\x00" as *const u8 as *const i8;
    g_arenaservers.go.generic.flags = 0x10 | 0x100;
    g_arenaservers.go.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.go.generic.id = 22;
    g_arenaservers.go.generic.x = 640;
    g_arenaservers.go.generic.y = 480 - 64;
    g_arenaservers.go.width = 128;
    g_arenaservers.go.height = 64;
    g_arenaservers.go.focuspic = b"menu/art/fight_1\x00" as *const u8 as *mut i8;
    g_arenaservers.punkbuster.generic.type_0 = 3;
    g_arenaservers.punkbuster.generic.name = b"Punkbuster:\x00" as *const u8 as *const i8;
    g_arenaservers.punkbuster.generic.flags = 0x100 | 0x2;
    g_arenaservers.punkbuster.generic.callback =
        Some(ArenaServers_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    g_arenaservers.punkbuster.generic.id = 24;
    g_arenaservers.punkbuster.generic.x = 480 + 32;
    g_arenaservers.punkbuster.generic.y = 144;
    g_arenaservers.punkbuster.itemnames = punkbuster_items.as_mut_ptr();
    g_arenaservers.pblogo.generic.type_0 = 6;
    g_arenaservers.pblogo.generic.name = b"menu/art/pblogo\x00" as *const u8 as *const i8;
    g_arenaservers.pblogo.generic.flags = 0x4 | 0x4000;
    g_arenaservers.pblogo.generic.x = 526;
    g_arenaservers.pblogo.generic.y = 176;
    g_arenaservers.pblogo.width = 32;
    g_arenaservers.pblogo.height = 16;
    g_arenaservers.pblogo.errorpic = b"menu/art/unknownmap\x00" as *const u8 as *mut i8;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.master as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.gametype as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.sortkey as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.showfull as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.showempty as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.mappic as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.status as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.statusbar as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.arrows as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.up as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.down as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.list as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.remove as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.specify as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.refresh as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.create as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.go as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.punkbuster as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut g_arenaservers.menu,
        &mut g_arenaservers.pblogo as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    ArenaServers_LoadFavorites();
    g_servertype = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        (8i32 - 1) as f32,
        crate::src::q3_ui::ui_main::ui_browserMaster.integer as f32,
    ) as i32;
    g_arenaservers.master.curvalue = g_servertype;
    g_gametype = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        (5i32 - 1) as f32,
        crate::src::q3_ui::ui_main::ui_browserGameType.integer as f32,
    ) as i32;
    g_arenaservers.gametype.curvalue = g_gametype;
    g_sortkey = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        (5i32 - 1) as f32,
        crate::src::q3_ui::ui_main::ui_browserSortKey.integer as f32,
    ) as i32;
    g_arenaservers.sortkey.curvalue = g_sortkey;
    g_fullservers = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        1f32,
        crate::src::q3_ui::ui_main::ui_browserShowFull.integer as f32,
    ) as i32;
    g_arenaservers.showfull.curvalue = g_fullservers;
    g_emptyservers = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        1f32,
        crate::src::q3_ui::ui_main::ui_browserShowEmpty.integer as f32,
    ) as i32;
    g_arenaservers.showempty.curvalue = g_emptyservers;
    g_arenaservers.punkbuster.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        1f32,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"cl_punkbuster\x00" as *const u8 as *const i8,
        ),
    ) as i32;
    // force to initial state and refresh
    g_servertype = ArenaServers_SetType(g_servertype);
    g_arenaservers.master.curvalue = g_servertype;
    crate::src::ui::ui_syscalls::trap_Cvar_Register(
        0 as *mut crate::src::qcommon::q_shared::vmCvar_t,
        b"debug_protocol\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        0,
    );
}
/*
=================
ArenaServers_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ArenaServers_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/create_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/create_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/specify_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/specify_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/refresh_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/refresh_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fight_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fight_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_vert_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_vert_top\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/arrows_vert_bot\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/unknownmap\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/pblogo\x00" as *const u8 as *const i8,
    );
}
/*
=================
UI_ArenaServersMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ArenaServersMenu() {
    ArenaServers_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut g_arenaservers.menu);
}
