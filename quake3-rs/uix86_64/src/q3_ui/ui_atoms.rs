use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;

pub use crate::src::q3_ui::ui_addbots::UI_AddBots_Cache;
pub use crate::src::q3_ui::ui_cdkey::UI_CDKeyMenu_Cache;
pub use crate::src::q3_ui::ui_cdkey::UI_CDKeyMenu_f;
pub use crate::src::q3_ui::ui_cinematics::UI_CinematicsMenu_Cache;
pub use crate::src::q3_ui::ui_cinematics::UI_CinematicsMenu_f;
pub use crate::src::q3_ui::ui_confirm::ConfirmMenu_Cache;
pub use crate::src::q3_ui::ui_confirm::UI_ConfirmMenu;
pub use crate::src::q3_ui::ui_controls2::Controls_Cache;
pub use crate::src::q3_ui::ui_demo2::Demos_Cache;
pub use crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu_Cache;
pub use crate::src::q3_ui::ui_gameinfo::UI_InitGameinfo;
pub use crate::src::q3_ui::ui_gameinfo::UI_SPUnlockMedals_f;
pub use crate::src::q3_ui::ui_gameinfo::UI_SPUnlock_f;
pub use crate::src::q3_ui::ui_ingame::InGame_Cache;
pub use crate::src::q3_ui::ui_ingame::UI_InGameMenu;
pub use crate::src::q3_ui::ui_main::UI_RegisterCvars;
pub use crate::src::q3_ui::ui_main::UI_UpdateCvars;
pub use crate::src::q3_ui::ui_menu::MainMenu_Cache;
pub use crate::src::q3_ui::ui_menu::UI_MainMenu;
pub use crate::src::q3_ui::ui_mods::UI_ModsMenu_Cache;
pub use crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu_Cache;
pub use crate::src::q3_ui::ui_playermodel::PlayerModel_Cache;
pub use crate::src::q3_ui::ui_playersettings::PlayerSettings_Cache;
pub use crate::src::q3_ui::ui_preferences::Preferences_Cache;
pub use crate::src::q3_ui::ui_qmenu::menu_in_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_move_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_null_sound;
pub use crate::src::q3_ui::ui_qmenu::menu_out_sound;
pub use crate::src::q3_ui::ui_qmenu::Menu_Cache;
pub use crate::src::q3_ui::ui_qmenu::Menu_DefaultKey;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursor;
pub use crate::src::q3_ui::ui_removebots::UI_RemoveBots_Cache;
pub use crate::src::q3_ui::ui_serverinfo::ServerInfo_Cache;
pub use crate::src::q3_ui::ui_servers2::ArenaServers_Cache;
pub use crate::src::q3_ui::ui_setup::UI_SetupMenu_Cache;
pub use crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu_Cache;
pub use crate::src::q3_ui::ui_specifyserver::SpecifyServer_Cache;
pub use crate::src::q3_ui::ui_splevel::UI_SPLevelMenu_Cache;
pub use crate::src::q3_ui::ui_splevel::UI_SPLevelMenu_f;
pub use crate::src::q3_ui::ui_sppostgame::UI_SPPostgameMenu_Cache;
pub use crate::src::q3_ui::ui_sppostgame::UI_SPPostgameMenu_f;
pub use crate::src::q3_ui::ui_spskill::UI_SPSkillMenu_Cache;
pub use crate::src::q3_ui::ui_startserver::ServerOptions_Cache;
pub use crate::src::q3_ui::ui_startserver::StartServer_Cache;
pub use crate::src::q3_ui::ui_startserver::UI_BotSelectMenu_Cache;
pub use crate::src::q3_ui::ui_team::TeamMain_Cache;
pub use crate::src::q3_ui::ui_teamorders::UI_TeamOrdersMenu_f;
pub use crate::src::q3_ui::ui_video::DriverInfo_Cache;
pub use crate::src::q3_ui::ui_video::GraphicsOptions_Cache;
pub use crate::src::qcommon::q_math::colorBlack;
pub use crate::src::qcommon::q_math::colorWhite;
pub use crate::src::qcommon::q_math::g_color_table;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::Q_stricmp;
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
pub use crate::src::ui::ui_syscalls::trap_Argv;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Error;
pub use crate::src::ui::ui_syscalls::trap_GetGlconfig;
pub use crate::src::ui::ui_syscalls::trap_Key_ClearStates;
pub use crate::src::ui::ui_syscalls::trap_Key_GetCatcher;
pub use crate::src::ui::ui_syscalls::trap_Key_SetCatcher;
pub use crate::src::ui::ui_syscalls::trap_Print;
pub use crate::src::ui::ui_syscalls::trap_R_DrawStretchPic;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_R_SetColor;
pub use crate::src::ui::ui_syscalls::trap_S_StartLocalSound;
pub use crate::src::ui::ui_syscalls::trap_UpdateScreen;

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
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::uiStatic_t;
pub use crate::ui_public_h::uiMenuCommand_t;
pub use crate::ui_public_h::UIMENU_BAD_CD_KEY;
pub use crate::ui_public_h::UIMENU_INGAME;
pub use crate::ui_public_h::UIMENU_MAIN;
pub use crate::ui_public_h::UIMENU_NEED_CD;
pub use crate::ui_public_h::UIMENU_NONE;
pub use crate::ui_public_h::UIMENU_POSTGAME;
pub use crate::ui_public_h::UIMENU_TEAM;
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
/* *********************************************************************
    UI_ATOMS.C

    User interface building blocks and support functions.
**********************************************************************/
#[no_mangle]

pub static mut uis: crate::ui_local_h::uiStatic_t = crate::ui_local_h::uiStatic_t {
    frametime: 0,
    realtime: 0,
    cursorx: 0,
    cursory: 0,
    menusp: 0,
    activemenu: 0 as *mut crate::ui_local_h::menuframework_s,
    stack: [0 as *mut crate::ui_local_h::menuframework_s; 8],
    glconfig: crate::tr_types_h::glconfig_t {
        renderer_string: [0; 1024],
        vendor_string: [0; 1024],
        version_string: [0; 1024],
        extensions_string: [0; 8192],
        maxTextureSize: 0,
        numTextureUnits: 0,
        colorBits: 0,
        depthBits: 0,
        stencilBits: 0,
        driverType: crate::tr_types_h::GLDRV_ICD,
        hardwareType: crate::tr_types_h::GLHW_GENERIC,
        deviceSupportsGamma: crate::src::qcommon::q_shared::qfalse,
        textureCompression: crate::tr_types_h::TC_NONE,
        textureEnvAddAvailable: crate::src::qcommon::q_shared::qfalse,
        vidWidth: 0,
        vidHeight: 0,
        windowAspect: 0.,
        displayFrequency: 0,
        isFullscreen: crate::src::qcommon::q_shared::qfalse,
        stereoEnabled: crate::src::qcommon::q_shared::qfalse,
        smpActive: crate::src::qcommon::q_shared::qfalse,
    },
    debug: crate::src::qcommon::q_shared::qfalse,
    whiteShader: 0,
    menuBackShader: 0,
    menuBackNoLogoShader: 0,
    charset: 0,
    charsetProp: 0,
    charsetPropGlow: 0,
    charsetPropB: 0,
    cursor: 0,
    rb_on: 0,
    rb_off: 0,
    xscale: 0.,
    yscale: 0.,
    bias: 0.,
    demoversion: crate::src::qcommon::q_shared::qfalse,
    firstdraw: crate::src::qcommon::q_shared::qfalse,
};
#[no_mangle]

pub static mut m_entersound: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
// after a frame, so caching won't disrupt the sound
#[no_mangle]

pub unsafe extern "C" fn Com_Error(mut _level: i32, mut error: *const i8, mut args: ...) -> ! {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [i8; 1024] = [0; 1024];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>(),
        error,
        argptr.as_va_list(),
    );
    crate::src::ui::ui_syscalls::trap_Error(text.as_mut_ptr());
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
// q_shared.h -- included first by ALL program modules.
// A user mod should never modify this file
// Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
// When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
// You shouldn't change this unless you know what you're doing
// number of supported master servers
// standard demo extension
//Ignore __attribute__ on non-gcc platforms
/* *********************************************************************
 VM Considerations

 The VM can not use the standard system headers because we aren't really
 using the compiler they were meant for.  We use bg_lib.h which contains
 prototypes for the functions we define for our own use in bg_lib.c.

 When writing mods, please add needed headers HERE, do not start including
 stuff like <stdio.h> in the various .c files that make up each of the VMs
 since you will be including system headers files can will have issues.

 Remember, if you use a C library function that is not defined in bg_lib.c,
 you will have to add your own version for support in the VM.

**********************************************************************/
//=============================================================
// expand constants before stringifying them
// angle indexes
// up / down
// left / right
// fall over
// the game guarantees that no string from the network will ever
// exceed MAX_STRING_CHARS
// max length of a string passed to Cmd_TokenizeString
// max tokens resulting from Cmd_TokenizeString
// max length of an individual token
// used for system info key only
// max length of a quake game pathname
// max length of a client name
// parameters for command buffer stuffing
// don't return until completed, a VM should NEVER use this,
// because some commands might cause the VM to be unloaded...
// insert at current position, but don't run yet
// add to end of the command buffer (normal case)
//
// these aren't needed by any of the VMs.  put in another header?
//
// bit vector of area visibility
// print levels from renderer (FIXME: set up for game / cgame?)
// only print when "developer 1"
// parameters to the main Error routine
// exit the entire game with a popup window
// print to console and disconnect from game
// don't kill server
// client disconnected from the server
// pop up the need-cd dialog
// font rendering values used by ui and cgame
// default
// default
/*
==============================================================

MATHLIB

==============================================================
*/
// all drawing is done to a 640*480 virtual screen size
// and will be automatically scaled to the real resolution
// ^[0-9a-zA-Z]
/*
// if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
// or write a mail to the ioq3 mailing list.
#else
  #define Q_ftol(v) ((long) (v))
  #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
  #define Q_SnapVector(vec) \
    do\
    {\
        vec3_t *temp = (vec);\
        \
        Q_round((*temp)[0]);\
        Q_round((*temp)[1]);\
        Q_round((*temp)[2]);\
    } while(0)
#endif
*/
// reciprocal square root
// this isn't a real cheap function to call!
// just in case you don't want to use the macros
// fast vector normalize routine that does not check to make sure
// that length != 0, nor does it return length, uses rsqrt approximation
// returns vector length
// perpendicular vector could be replaced by this
//int	PlaneTypeForNormal (vec3_t normal);
//=============================================
//int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
//token types
// string
// literal
// number
// name
// punctuation
// data is an in/out parm, returns a parsed out token
// mode parm for FS_FOpenFile
//=============================================
// portable case insensitive compare
// buffer size safe library replacements
// strlen that discounts Quake color sequences
// removes color sequences from string
// Count the number of char tocount encountered in string
//=============================================
// 64-bit integers for global rankings interface
// implemented as a struct for qvm compatibility
//=============================================
/*
short	BigShort(short l);
short	LittleShort(short l);
int		BigLong (int l);
int		LittleLong (int l);
qint64  BigLong64 (qint64 l);
qint64  LittleLong64 (qint64 l);
float	BigFloat (const float *l);
float	LittleFloat (const float *l);

void	Swap_Init (void);
*/
//=============================================
//
// key / value info strings
//
// this is only here so the functions in q_shared.c and bg_*.c can link
#[no_mangle]

pub unsafe extern "C" fn Com_Printf(mut msg: *const i8, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [i8; 1024] = [0; 1024];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>(),
        msg,
        argptr.as_va_list(),
    );
    crate::src::ui::ui_syscalls::trap_Print(text.as_mut_ptr());
}
/*
=================
UI_ClampCvar
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ClampCvar(mut min: f32, mut max: f32, mut value: f32) -> f32 {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
/*
=================
UI_StartDemoLoop
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_StartDemoLoop() {
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
        b"d1\n\x00" as *const u8 as *const i8,
    );
}
/*
=================
UI_PushMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_PushMenu(mut menu: *mut crate::ui_local_h::menuframework_s) {
    let mut i: i32 = 0;
    let mut item: *mut crate::ui_local_h::menucommon_s = 0 as *mut crate::ui_local_h::menucommon_s;
    // avoid stacking menus invoked by hotkeys
    i = 0;
    while i < uis.menusp {
        if uis.stack[i as usize] == menu {
            uis.menusp = i;
            break;
        } else {
            i += 1
        }
    }
    if i == uis.menusp {
        if uis.menusp >= 8 {
            crate::src::ui::ui_syscalls::trap_Error(
                b"UI_PushMenu: menu stack overflow\x00" as *const u8 as *const i8,
            );
        }
        let fresh0 = uis.menusp;
        uis.menusp = uis.menusp + 1;
        uis.stack[fresh0 as usize] = menu
    }
    uis.activemenu = menu;
    // default cursor position
    (*menu).cursor = 0;
    (*menu).cursor_prev = 0;
    m_entersound = crate::src::qcommon::q_shared::qtrue;
    crate::src::ui::ui_syscalls::trap_Key_SetCatcher(0x2);
    // force first available item to have focus
    i = 0;
    while i < (*menu).nitems {
        item = (*menu).items[i as usize] as *mut crate::ui_local_h::menucommon_s;
        if (*item).flags & (0x2000 | 0x800 | 0x4000) == 0 {
            (*menu).cursor_prev = -(1);
            crate::src::q3_ui::ui_qmenu::Menu_SetCursor(menu, i);
            break;
        } else {
            i += 1
        }
    }
    uis.firstdraw = crate::src::qcommon::q_shared::qtrue;
}
/*
=================
UI_PopMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_PopMenu() {
    crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
        crate::src::q3_ui::ui_qmenu::menu_out_sound,
        crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as i32,
    );
    uis.menusp -= 1;
    if uis.menusp < 0 {
        crate::src::ui::ui_syscalls::trap_Error(
            b"UI_PopMenu: menu stack underflow\x00" as *const u8 as *const i8,
        );
    }
    if uis.menusp != 0 {
        uis.activemenu = uis.stack[(uis.menusp - 1) as usize];
        uis.firstdraw = crate::src::qcommon::q_shared::qtrue
    } else {
        UI_ForceMenuOff();
    };
}
#[no_mangle]

pub unsafe extern "C" fn UI_ForceMenuOff() {
    uis.menusp = 0;
    uis.activemenu = 0 as *mut crate::ui_local_h::menuframework_s;
    crate::src::ui::ui_syscalls::trap_Key_SetCatcher(
        crate::src::ui::ui_syscalls::trap_Key_GetCatcher() & !(0x2),
    );
    crate::src::ui::ui_syscalls::trap_Key_ClearStates();
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"cl_paused\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
    );
}
/*
=================
UI_LerpColor
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_LerpColor(
    mut a: *mut crate::src::qcommon::q_shared::vec_t,
    mut b: *mut crate::src::qcommon::q_shared::vec_t,
    mut c: *mut crate::src::qcommon::q_shared::vec_t,
    mut t: f32,
) {
    let mut i: i32 = 0;
    // lerp and clamp each component
    i = 0;
    while i < 4 {
        *c.offset(i as isize) =
            *a.offset(i as isize) + t * (*b.offset(i as isize) - *a.offset(i as isize));
        if *c.offset(i as isize) < 0f32 {
            *c.offset(i as isize) = 0f32
        } else if *c.offset(i as isize) as f64 > 1.0 {
            *c.offset(i as isize) = 1f32
        }
        i += 1
    }
}
/*
=================
UI_DrawProportionalString2
=================
*/

static mut propMap: [[i32; 3]; 128] = [
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, -(1)],
    [0, 0, 8],
    [11, 122, 7],
    [154, 181, 14],
    [55, 122, 17],
    [79, 122, 18],
    [101, 122, 23],
    [153, 122, 18],
    [9, 93, 7],
    [207, 122, 8],
    [230, 122, 9],
    [177, 122, 18],
    [30, 152, 18],
    [85, 181, 7],
    [34, 93, 11],
    [110, 181, 6],
    [130, 152, 14],
    [22, 64, 17],
    [41, 64, 12],
    [58, 64, 17],
    [78, 64, 18],
    [98, 64, 19],
    [120, 64, 18],
    [141, 64, 18],
    [204, 64, 16],
    [162, 64, 17],
    [182, 64, 18],
    [59, 181, 7],
    [35, 181, 7],
    [203, 152, 14],
    [56, 93, 14],
    [228, 152, 14],
    [177, 181, 18],
    [28, 122, 22],
    [5, 4, 18],
    [27, 4, 18],
    [48, 4, 18],
    [69, 4, 17],
    [90, 4, 13],
    [106, 4, 13],
    [121, 4, 18],
    [143, 4, 17],
    [164, 4, 8],
    [175, 4, 16],
    [195, 4, 18],
    [216, 4, 12],
    [230, 4, 23],
    [6, 34, 18],
    [27, 34, 18],
    [48, 34, 18],
    [68, 34, 18],
    [90, 34, 17],
    [110, 34, 18],
    [130, 34, 14],
    [146, 34, 18],
    [166, 34, 19],
    [185, 34, 29],
    [215, 34, 18],
    [234, 34, 18],
    [5, 64, 14],
    [60, 152, 7],
    [106, 151, 13],
    [83, 152, 7],
    [128, 122, 17],
    [4, 152, 21],
    [134, 181, 5],
    [5, 4, 18],
    [27, 4, 18],
    [48, 4, 18],
    [69, 4, 17],
    [90, 4, 13],
    [106, 4, 13],
    [121, 4, 18],
    [143, 4, 17],
    [164, 4, 8],
    [175, 4, 16],
    [195, 4, 18],
    [216, 4, 12],
    [230, 4, 23],
    [6, 34, 18],
    [27, 34, 18],
    [48, 34, 18],
    [68, 34, 18],
    [90, 34, 17],
    [110, 34, 18],
    [130, 34, 14],
    [146, 34, 18],
    [166, 34, 19],
    [185, 34, 29],
    [215, 34, 18],
    [234, 34, 18],
    [5, 64, 14],
    [153, 152, 13],
    [11, 181, 5],
    [180, 152, 13],
    [79, 93, 17],
    [0, 0, -(1)],
];

static mut propMapB: [[i32; 3]; 26] = [
    [11, 12, 33],
    [49, 12, 31],
    [85, 12, 31],
    [120, 12, 30],
    [156, 12, 21],
    [183, 12, 21],
    [207, 12, 32],
    [13, 55, 30],
    [49, 55, 13],
    [66, 55, 29],
    [101, 55, 31],
    [135, 55, 21],
    [158, 55, 40],
    [204, 55, 32],
    [12, 97, 31],
    [48, 97, 31],
    [82, 97, 30],
    [118, 97, 30],
    [153, 97, 30],
    [185, 97, 25],
    [213, 97, 30],
    [11, 139, 32],
    [42, 139, 51],
    [93, 139, 32],
    [126, 139, 31],
    [158, 139, 25],
];
/*
=================
UI_DrawBannerString
=================
*/

unsafe extern "C" fn UI_DrawBannerString2(
    mut x: i32,
    mut y: i32,
    mut str: *const i8,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut s: *const i8 = 0 as *const i8;
    let mut ch: u8 = 0;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut fwidth: f32 = 0.;
    let mut fheight: f32 = 0.;
    // draw the colored text
    crate::src::ui::ui_syscalls::trap_R_SetColor(color as *const f32);
    ax = x as f32 * uis.xscale + uis.bias;
    ay = y as f32 * uis.yscale;
    s = str;
    while *s != 0 {
        ch = (*s as i32 & 127) as u8;
        if ch as i32 == ' ' as i32 {
            ax += (12f32 + 4f32) * uis.xscale
        } else if ch as i32 >= 'A' as i32 && ch as i32 <= 'Z' as i32 {
            ch = (ch as i32 - 'A' as i32) as u8;
            fcol = propMapB[ch as usize][0] as f32 / 256.0;
            frow = propMapB[ch as usize][1] as f32 / 256.0;
            fwidth = propMapB[ch as usize][2] as f32 / 256.0;
            fheight = 36f32 / 256.0;
            aw = propMapB[ch as usize][2] as f32 * uis.xscale;
            ah = 36f32 * uis.yscale;
            crate::src::ui::ui_syscalls::trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                uis.charsetPropB,
            );
            ax += aw + 4f32 * uis.xscale
        }
        s = s.offset(1)
    }
    crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const f32);
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawBannerString(
    mut x: i32,
    mut y: i32,
    mut str: *const i8,
    mut style: i32,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut s: *const i8 = 0 as *const i8;
    let mut ch: i32 = 0;
    let mut width: i32 = 0;
    let mut drawcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    // find the width of the drawn text
    s = str;
    width = 0;
    while *s != 0 {
        ch = *s as i32;
        if ch == ' ' as i32 {
            width += 12
        } else if ch >= 'A' as i32 && ch <= 'Z' as i32 {
            width += propMapB[(ch - 'A' as i32) as usize][2] + 4
        }
        s = s.offset(1)
    }
    width -= 4;
    match style & 0x7 {
        1 => x -= width / 2,
        2 => x -= width,
        0 | _ => {}
    }
    if style & 0x800 != 0 {
        drawcolor[2] = 0f32;
        drawcolor[1] = drawcolor[2];
        drawcolor[0] = drawcolor[1];
        drawcolor[3] = *color.offset(3);
        UI_DrawBannerString2(x + 2i32, y + 2i32, str, drawcolor.as_mut_ptr());
    }
    UI_DrawBannerString2(x, y, str, color);
}
#[no_mangle]

pub unsafe extern "C" fn UI_ProportionalStringWidth(mut str: *const i8) -> i32 {
    let mut s: *const i8 = 0 as *const i8;
    let mut ch: i32 = 0;
    let mut charWidth: i32 = 0;
    let mut width: i32 = 0;
    s = str;
    width = 0;
    while *s != 0 {
        ch = *s as i32 & 127;
        charWidth = propMap[ch as usize][2];
        if charWidth != -(1) {
            width += charWidth;
            width += 3
        }
        s = s.offset(1)
    }
    width -= 3;
    return width;
}

unsafe extern "C" fn UI_DrawProportionalString2(
    mut x: i32,
    mut y: i32,
    mut str: *const i8,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
    mut sizeScale: f32,
    mut charset: crate::src::qcommon::q_shared::qhandle_t,
) {
    let mut s: *const i8 = 0 as *const i8;
    let mut ch: u8 = 0;
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0f32;
    let mut ah: f32 = 0.;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    let mut fwidth: f32 = 0.;
    let mut fheight: f32 = 0.;
    // draw the colored text
    crate::src::ui::ui_syscalls::trap_R_SetColor(color as *const f32);
    ax = x as f32 * uis.xscale + uis.bias;
    ay = y as f32 * uis.yscale;
    s = str;
    while *s != 0 {
        ch = (*s as i32 & 127) as u8;
        if ch as i32 == ' ' as i32 {
            aw = 8f32 * uis.xscale * sizeScale
        } else if propMap[ch as usize][2] != -(1) {
            fcol = propMap[ch as usize][0] as f32 / 256.0;
            frow = propMap[ch as usize][1] as f32 / 256.0;
            fwidth = propMap[ch as usize][2] as f32 / 256.0;
            fheight = 27f32 / 256.0;
            aw = propMap[ch as usize][2] as f32 * uis.xscale * sizeScale;
            ah = 27f32 * uis.yscale * sizeScale;
            crate::src::ui::ui_syscalls::trap_R_DrawStretchPic(
                ax,
                ay,
                aw,
                ah,
                fcol,
                frow,
                fcol + fwidth,
                frow + fheight,
                charset,
            );
        }
        ax += aw + 3f32 * uis.xscale * sizeScale;
        s = s.offset(1)
    }
    crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const f32);
}
/*
=================
UI_ProportionalSizeScale
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ProportionalSizeScale(mut style: i32) -> f32 {
    if style & 0x10 != 0 {
        return 0.75f32;
    }
    return 1f32;
}
/*
=================
UI_DrawProportionalString
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawProportionalString(
    mut x: i32,
    mut y: i32,
    mut str: *const i8,
    mut style: i32,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut drawcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut width: i32 = 0;
    let mut sizeScale: f32 = 0.;
    if str.is_null() {
        return;
    }
    sizeScale = UI_ProportionalSizeScale(style);
    match style & 0x7 {
        1 => {
            width = (UI_ProportionalStringWidth(str) as f32 * sizeScale) as i32;
            x -= width / 2
        }
        2 => {
            width = (UI_ProportionalStringWidth(str) as f32 * sizeScale) as i32;
            x -= width
        }
        0 | _ => {}
    }
    if style & 0x800 != 0 {
        drawcolor[2] = 0f32;
        drawcolor[1] = drawcolor[2];
        drawcolor[0] = drawcolor[1];
        drawcolor[3] = *color.offset(3);
        UI_DrawProportionalString2(
            x + 2i32,
            y + 2i32,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            uis.charsetProp,
        );
    }
    if style & 0x2000 != 0 {
        drawcolor[0] = (*color.offset(0) as f64 * 0.7) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[1] = (*color.offset(1) as f64 * 0.7) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[2] = (*color.offset(2) as f64 * 0.7) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[3] = *color.offset(3);
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            uis.charsetProp,
        );
        return;
    }
    if style & 0x4000 != 0 {
        drawcolor[0] = (*color.offset(0) as f64 * 0.7) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[1] = (*color.offset(1) as f64 * 0.7) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[2] = (*color.offset(2) as f64 * 0.7) as crate::src::qcommon::q_shared::vec_t;
        drawcolor[3] = *color.offset(3);
        UI_DrawProportionalString2(x, y, str, color, sizeScale, uis.charsetProp);
        drawcolor[0] = *color.offset(0);
        drawcolor[1] = *color.offset(1);
        drawcolor[2] = *color.offset(2);
        drawcolor[3] = (0.5 + 0.5 * crate::stdlib::sin((uis.realtime / 75) as f64))
            as crate::src::qcommon::q_shared::vec_t;
        UI_DrawProportionalString2(
            x,
            y,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            uis.charsetPropGlow,
        );
        return;
    }
    UI_DrawProportionalString2(x, y, str, color, sizeScale, uis.charsetProp);
}
/*
=================
UI_DrawProportionalString_Wrapped
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawProportionalString_AutoWrapped(
    mut x: i32,
    mut y: i32,
    mut xmax: i32,
    mut ystep: i32,
    mut str: *const i8,
    mut style: i32,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut width: i32 = 0;
    let mut s1: *mut i8 = 0 as *mut i8;
    let mut s2: *mut i8 = 0 as *mut i8;
    let mut s3: *mut i8 = 0 as *mut i8;
    let mut c_bcp: i8 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    let mut sizeScale: f32 = 0.;
    if str.is_null() || *str.offset(0) as i32 == '\u{0}' as i32 {
        return;
    }
    sizeScale = UI_ProportionalSizeScale(style);
    crate::src::qcommon::q_shared::Q_strncpyz(
        buf.as_mut_ptr(),
        str,
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    s3 = buf.as_mut_ptr();
    s2 = s3;
    s1 = s2;
    loop {
        loop {
            s3 = s3.offset(1);
            if !(*s3 as i32 != ' ' as i32 && *s3 as i32 != '\u{0}' as i32) {
                break;
            }
        }
        c_bcp = *s3;
        *s3 = '\u{0}' as i8;
        width = (UI_ProportionalStringWidth(s1) as f32 * sizeScale) as i32;
        *s3 = c_bcp;
        if width > xmax {
            if s1 == s2 {
                // fuck, don't have a clean cut, we'll overflow
                s2 = s3
            }
            *s2 = '\u{0}' as i8;
            UI_DrawProportionalString(x, y, s1, style, color);
            y += ystep;
            if c_bcp as i32 == '\u{0}' as i32 {
                // that was the last word
                // we could start a new loop, but that wouldn't be much use
                // even if the word is too long, we would overflow it (see above)
                // so just print it now if needed
                s2 = s2.offset(1);
                if *s2 as i32 != '\u{0}' as i32 {
                    // if we are printing an overflowing line we have s2 == s3
                    UI_DrawProportionalString(x, y, s2, style, color);
                }
                break;
            } else {
                s2 = s2.offset(1);
                s1 = s2;
                s3 = s2
            }
        } else {
            s2 = s3;
            if !(c_bcp as i32 == '\u{0}' as i32) {
                continue;
            }
            // we reached the end
            UI_DrawProportionalString(x, y, s1, style, color);
            break;
        }
    }
}
/*
=================
UI_DrawString2
=================
*/

unsafe extern "C" fn UI_DrawString2(
    mut x: i32,
    mut y: i32,
    mut str: *const i8,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
    mut charw: i32,
    mut charh: i32,
) {
    let mut s: *const i8 = 0 as *const i8; //APSFIXME;
    let mut ch: i8 = 0;
    let mut forceColor: i32 = crate::src::qcommon::q_shared::qfalse as i32;
    let mut tempcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut ax: f32 = 0.;
    let mut ay: f32 = 0.;
    let mut aw: f32 = 0.;
    let mut ah: f32 = 0.;
    let mut frow: f32 = 0.;
    let mut fcol: f32 = 0.;
    if y < -charh {
        // offscreen
        return;
    }
    // draw the colored text
    crate::src::ui::ui_syscalls::trap_R_SetColor(color as *const f32);
    ax = x as f32 * uis.xscale + uis.bias;
    ay = y as f32 * uis.yscale;
    aw = charw as f32 * uis.xscale;
    ah = charh as f32 * uis.yscale;
    s = str;
    while *s != 0 {
        if crate::src::qcommon::q_shared::Q_IsColorString(s) as u64 != 0 {
            if forceColor == 0 {
                crate::stdlib::memcpy(
                    tempcolor.as_mut_ptr() as *mut libc::c_void,
                    crate::src::qcommon::q_math::g_color_table
                        [(*s.offset(1) as i32 - '0' as i32 & 0x7) as usize]
                        .as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<crate::src::qcommon::q_shared::vec4_t>(),
                );
                tempcolor[3] = *color.offset(3);
                crate::src::ui::ui_syscalls::trap_R_SetColor(tempcolor.as_mut_ptr());
            }
            s = s.offset(2)
        } else {
            ch = (*s as i32 & 255) as i8;
            if ch as i32 != ' ' as i32 {
                frow = ((ch as i32 >> 4) as f64 * 0.0625) as f32;
                fcol = ((ch as i32 & 15) as f64 * 0.0625) as f32;
                crate::src::ui::ui_syscalls::trap_R_DrawStretchPic(
                    ax,
                    ay,
                    aw,
                    ah,
                    fcol,
                    frow,
                    (fcol as f64 + 0.0625f64) as f32,
                    (frow as f64 + 0.0625f64) as f32,
                    uis.charset,
                );
            }
            ax += aw;
            s = s.offset(1)
        }
    }
    crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const f32);
}
/*
=================
UI_DrawString
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawString(
    mut x: i32,
    mut y: i32,
    mut str: *const i8,
    mut style: i32,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut len: i32 = 0;
    let mut charw: i32 = 0;
    let mut charh: i32 = 0;
    let mut newcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut lowlight: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut drawcolor: *mut f32 = 0 as *mut f32;
    let mut dropcolor: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    if str.is_null() {
        return;
    }
    if style & 0x1000 != 0 && uis.realtime / 200 & 1 != 0 {
        return;
    }
    if style & 0x10 != 0 {
        charw = 8;
        charh = 16
    } else if style & 0x40 != 0 {
        charw = 32;
        charh = 48
    } else {
        charw = 16;
        charh = 16
    }
    if style & 0x4000 != 0 {
        lowlight[0] = (0.8 * *color.offset(0) as f64) as crate::src::qcommon::q_shared::vec_t;
        lowlight[1] = (0.8 * *color.offset(1) as f64) as crate::src::qcommon::q_shared::vec_t;
        lowlight[2] = (0.8 * *color.offset(2) as f64) as crate::src::qcommon::q_shared::vec_t;
        lowlight[3] = (0.8 * *color.offset(3) as f64) as crate::src::qcommon::q_shared::vec_t;
        UI_LerpColor(
            color,
            lowlight.as_mut_ptr(),
            newcolor.as_mut_ptr(),
            (0.5 + 0.5 * crate::stdlib::sin((uis.realtime / 75) as f64)) as f32,
        );
        drawcolor = newcolor.as_mut_ptr()
    } else {
        drawcolor = color
    }
    match style & 0x7 {
        1 => {
            // center justify at x
            len = crate::stdlib::strlen(str) as i32;
            x = x - len * charw / 2
        }
        2 => {
            // right justify at x
            len = crate::stdlib::strlen(str) as i32;
            x = x - len * charw
        }
        _ => {}
    }
    if style & 0x800 != 0 {
        dropcolor[2] = 0f32;
        dropcolor[1] = dropcolor[2];
        dropcolor[0] = dropcolor[1];
        dropcolor[3] = *drawcolor.offset(3);
        UI_DrawString2(
            x + 2i32,
            y + 2i32,
            str,
            dropcolor.as_mut_ptr(),
            charw,
            charh,
        );
    }
    UI_DrawString2(x, y, str, drawcolor, charw, charh);
}
/*
=================
UI_DrawChar
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawChar(
    mut x: i32,
    mut y: i32,
    mut ch: i32,
    mut style: i32,
    mut color: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut buff: [i8; 2] = [0; 2];
    buff[0] = ch as i8;
    buff[1] = '\u{0}' as i8;
    UI_DrawString(x, y, buff.as_mut_ptr(), style, color);
}
#[no_mangle]

pub unsafe extern "C" fn UI_IsFullscreen() -> crate::src::qcommon::q_shared::qboolean {
    if !uis.activemenu.is_null() && crate::src::ui::ui_syscalls::trap_Key_GetCatcher() & 0x2 != 0 {
        return (*uis.activemenu).fullscreen;
    }
    return crate::src::qcommon::q_shared::qfalse;
}

unsafe extern "C" fn NeedCDAction(mut result: crate::src::qcommon::q_shared::qboolean) {
    if result as u64 == 0 {
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            b"quit\n\x00" as *const u8 as *const i8,
        );
    };
}

unsafe extern "C" fn NeedCDKeyAction(mut result: crate::src::qcommon::q_shared::qboolean) {
    if result as u64 == 0 {
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            b"quit\n\x00" as *const u8 as *const i8,
        );
    };
}
#[no_mangle]

pub unsafe extern "C" fn UI_SetActiveMenu(mut menu: crate::ui_public_h::uiMenuCommand_t) {
    // this should be the ONLY way the menu system is brought up
    // ensure minimum menu data is cached
    crate::src::q3_ui::ui_qmenu::Menu_Cache();
    match menu {
        0 => {
            UI_ForceMenuOff();
            return;
        }
        1 => {
            crate::src::q3_ui::ui_menu::UI_MainMenu();
            return;
        }
        3 => {
            crate::src::q3_ui::ui_confirm::UI_ConfirmMenu(
                b"Insert the CD\x00" as *const u8 as *const i8,
                None,
                Some(
                    NeedCDAction
                        as unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> (),
                ),
            );
            return;
        }
        4 => {
            crate::src::q3_ui::ui_confirm::UI_ConfirmMenu(
                b"Bad CD Key\x00" as *const u8 as *const i8,
                None,
                Some(
                    NeedCDKeyAction
                        as unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> (),
                ),
            );
            return;
        }
        2 => {
            /*
            //GRank
            UI_RankingsMenu();
            return;
            */
            crate::src::ui::ui_syscalls::trap_Cvar_Set(
                b"cl_paused\x00" as *const u8 as *const i8,
                b"1\x00" as *const u8 as *const i8,
            );
            crate::src::q3_ui::ui_ingame::UI_InGameMenu();
            return;
        }
        5 | 6 | _ => {}
    };
}
/*
=================
UI_KeyEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_KeyEvent(mut key: i32, mut down: i32) {
    let mut s: crate::src::qcommon::q_shared::sfxHandle_t = 0;
    if uis.activemenu.is_null() {
        return;
    }
    if down == 0 {
        return;
    }
    if (*uis.activemenu).key.is_some() {
        s = (*uis.activemenu).key.expect("non-null function pointer")(key)
    } else {
        s = crate::src::q3_ui::ui_qmenu::Menu_DefaultKey(uis.activemenu, key)
    }
    if s > 0 && s != crate::src::q3_ui::ui_qmenu::menu_null_sound {
        crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
            s,
            crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as i32,
        );
    };
}
/*
=================
UI_MouseEvent
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_MouseEvent(mut dx: i32, mut dy: i32) {
    let mut _i: i32 = 0;
    let mut bias: i32 = 0;
    let mut m: *mut crate::ui_local_h::menucommon_s = 0 as *mut crate::ui_local_h::menucommon_s;
    if uis.activemenu.is_null() {
        return;
    }
    // convert X bias to 640 coords
    bias = (uis.bias / uis.xscale) as i32;
    // update mouse screen position
    uis.cursorx += dx;
    if uis.cursorx < -bias {
        uis.cursorx = -bias
    } else if uis.cursorx > 640 + bias {
        uis.cursorx = 640 + bias
    }
    uis.cursory += dy;
    if uis.cursory < 0 {
        uis.cursory = 0
    } else if uis.cursory > 480 {
        uis.cursory = 480
    }

    for i in 0..(*uis.activemenu).nitems {
        m = (*uis.activemenu).items[i as usize] as *mut crate::ui_local_h::menucommon_s;

        if !((*m).flags & (0x2000 | 0x4000) != 0) {
            if !(uis.cursorx < (*m).left
                || uis.cursorx > (*m).right
                || uis.cursory < (*m).top
                || uis.cursory > (*m).bottom)
            {
                // set focus to item at cursor
                if (*uis.activemenu).cursor != i {
                    crate::src::q3_ui::ui_qmenu::Menu_SetCursor(uis.activemenu, i);
                    (*((*uis.activemenu).items[(*uis.activemenu).cursor_prev as usize]
                        as *mut crate::ui_local_h::menucommon_s))
                        .flags &= !(0x200);
                    if (*((*uis.activemenu).items[(*uis.activemenu).cursor as usize]
                        as *mut crate::ui_local_h::menucommon_s))
                        .flags
                        & 0x100000
                        == 0
                    {
                        crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
                            crate::src::q3_ui::ui_qmenu::menu_move_sound,
                            crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as i32,
                        );
                    }
                }
                (*((*uis.activemenu).items[(*uis.activemenu).cursor as usize]
                    as *mut crate::ui_local_h::menucommon_s))
                    .flags |= 0x200;
                return;
            }
        }
    }
    if (*uis.activemenu).nitems > 0 {
        // out of any region
        (*((*uis.activemenu).items[(*uis.activemenu).cursor as usize]
            as *mut crate::ui_local_h::menucommon_s))
            .flags &= !(0x200)
    };
}
#[no_mangle]

pub unsafe extern "C" fn UI_Argv(mut arg: i32) -> *mut i8 {
    static mut buffer: [i8; 1024] = [0; 1024];
    crate::src::ui::ui_syscalls::trap_Argv(
        arg,
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    return buffer.as_mut_ptr();
}
#[no_mangle]

pub unsafe extern "C" fn UI_Cvar_VariableString(mut var_name: *const i8) -> *mut i8 {
    static mut buffer: [i8; 1024] = [0; 1024];
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        var_name,
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    return buffer.as_mut_ptr();
}
/*
=================
UI_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Cache_f() {
    crate::src::q3_ui::ui_menu::MainMenu_Cache();
    crate::src::q3_ui::ui_ingame::InGame_Cache();
    crate::src::q3_ui::ui_confirm::ConfirmMenu_Cache();
    crate::src::q3_ui::ui_playermodel::PlayerModel_Cache();
    crate::src::q3_ui::ui_playersettings::PlayerSettings_Cache();
    crate::src::q3_ui::ui_controls2::Controls_Cache();
    crate::src::q3_ui::ui_demo2::Demos_Cache();
    crate::src::q3_ui::ui_cinematics::UI_CinematicsMenu_Cache();
    crate::src::q3_ui::ui_preferences::Preferences_Cache();
    crate::src::q3_ui::ui_serverinfo::ServerInfo_Cache();
    crate::src::q3_ui::ui_specifyserver::SpecifyServer_Cache();
    crate::src::q3_ui::ui_servers2::ArenaServers_Cache();
    crate::src::q3_ui::ui_startserver::StartServer_Cache();
    crate::src::q3_ui::ui_startserver::ServerOptions_Cache();
    crate::src::q3_ui::ui_video::DriverInfo_Cache();
    crate::src::q3_ui::ui_video::GraphicsOptions_Cache();
    crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu_Cache();
    crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu_Cache();
    crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu_Cache();
    crate::src::q3_ui::ui_splevel::UI_SPLevelMenu_Cache();
    crate::src::q3_ui::ui_spskill::UI_SPSkillMenu_Cache();
    crate::src::q3_ui::ui_sppostgame::UI_SPPostgameMenu_Cache();
    crate::src::q3_ui::ui_team::TeamMain_Cache();
    crate::src::q3_ui::ui_addbots::UI_AddBots_Cache();
    crate::src::q3_ui::ui_removebots::UI_RemoveBots_Cache();
    crate::src::q3_ui::ui_setup::UI_SetupMenu_Cache();
    //	UI_LoadConfig_Cache();
    //	UI_SaveConfigMenu_Cache();
    crate::src::q3_ui::ui_startserver::UI_BotSelectMenu_Cache();
    crate::src::q3_ui::ui_cdkey::UI_CDKeyMenu_Cache();
    crate::src::q3_ui::ui_mods::UI_ModsMenu_Cache();
}
/*
=================
UI_ConsoleCommand
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ConsoleCommand(
    mut realTime: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut cmd: *mut i8 = 0 as *mut i8;
    uis.frametime = realTime - uis.realtime;
    uis.realtime = realTime;
    cmd = UI_Argv(0);
    // ensure minimum menu data is available
    crate::src::q3_ui::ui_qmenu::Menu_Cache();
    if crate::src::qcommon::q_shared::Q_stricmp(cmd, b"levelselect\x00" as *const u8 as *const i8)
        == 0
    {
        crate::src::q3_ui::ui_splevel::UI_SPLevelMenu_f();
        return crate::src::qcommon::q_shared::qtrue;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(cmd, b"postgame\x00" as *const u8 as *const i8) == 0
    {
        crate::src::q3_ui::ui_sppostgame::UI_SPPostgameMenu_f();
        return crate::src::qcommon::q_shared::qtrue;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(cmd, b"ui_cache\x00" as *const u8 as *const i8) == 0
    {
        UI_Cache_f();
        return crate::src::qcommon::q_shared::qtrue;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(cmd, b"ui_cinematics\x00" as *const u8 as *const i8)
        == 0
    {
        crate::src::q3_ui::ui_cinematics::UI_CinematicsMenu_f();
        return crate::src::qcommon::q_shared::qtrue;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(cmd, b"ui_teamOrders\x00" as *const u8 as *const i8)
        == 0
    {
        crate::src::q3_ui::ui_teamorders::UI_TeamOrdersMenu_f();
        return crate::src::qcommon::q_shared::qtrue;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(cmd, b"iamacheater\x00" as *const u8 as *const i8)
        == 0
    {
        crate::src::q3_ui::ui_gameinfo::UI_SPUnlock_f();
        return crate::src::qcommon::q_shared::qtrue;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(cmd, b"iamamonkey\x00" as *const u8 as *const i8)
        == 0
    {
        crate::src::q3_ui::ui_gameinfo::UI_SPUnlockMedals_f();
        return crate::src::qcommon::q_shared::qtrue;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(cmd, b"ui_cdkey\x00" as *const u8 as *const i8) == 0
    {
        crate::src::q3_ui::ui_cdkey::UI_CDKeyMenu_f();
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
=================
UI_Shutdown
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Shutdown() {}
/*
=================
UI_Init
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Init() {
    crate::src::q3_ui::ui_main::UI_RegisterCvars();
    crate::src::q3_ui::ui_gameinfo::UI_InitGameinfo();
    // cache redundant calulations
    crate::src::ui::ui_syscalls::trap_GetGlconfig(&mut uis.glconfig);
    // for 640x480 virtualized screen
    uis.xscale = (uis.glconfig.vidWidth as f64 * (1.0 / 640.0)) as f32;
    uis.yscale = (uis.glconfig.vidHeight as f64 * (1.0 / 480.0)) as f32;
    if uis.glconfig.vidWidth * 480 > uis.glconfig.vidHeight * 640 {
        // wide screen
        uis.bias = (0.5
            * (uis.glconfig.vidWidth as f64 - uis.glconfig.vidHeight as f64 * (640.0 / 480.0)))
            as f32;
        uis.xscale = uis.yscale
    } else {
        // no wide screen
        uis.bias = 0f32
    }
    // initialize the menu system
    crate::src::q3_ui::ui_qmenu::Menu_Cache();
    uis.activemenu = 0 as *mut crate::ui_local_h::menuframework_s;
    uis.menusp = 0;
}
/*
================
UI_AdjustFrom640

Adjusted for resolution and screen aspect ratio
================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_AdjustFrom640(
    mut x: *mut f32,
    mut y: *mut f32,
    mut w: *mut f32,
    mut h: *mut f32,
) {
    // expect valid pointers
    *x = *x * uis.xscale + uis.bias;
    *y *= uis.yscale;
    *w *= uis.xscale;
    *h *= uis.yscale;
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawNamedPic(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut picname: *const i8,
) {
    let mut hShader: crate::src::qcommon::q_shared::qhandle_t = 0;
    hShader = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(picname);
    UI_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    crate::src::ui::ui_syscalls::trap_R_DrawStretchPic(
        x, y, width, height, 0f32, 0f32, 1f32, 1f32, hShader,
    );
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawHandlePic(
    mut x: f32,
    mut y: f32,
    mut w: f32,
    mut h: f32,
    mut hShader: crate::src::qcommon::q_shared::qhandle_t,
) {
    let mut s0: f32 = 0.;
    let mut s1: f32 = 0.;
    let mut t0: f32 = 0.;
    let mut t1: f32 = 0.;
    if w < 0f32 {
        // flip about vertical
        w = -w;
        s0 = 1f32;
        s1 = 0f32
    } else {
        s0 = 0f32;
        s1 = 1f32
    }
    if h < 0f32 {
        // flip about horizontal
        h = -h;
        t0 = 1f32;
        t1 = 0f32
    } else {
        t0 = 0f32;
        t1 = 1f32
    }
    UI_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    crate::src::ui::ui_syscalls::trap_R_DrawStretchPic(x, y, w, h, s0, t0, s1, t1, hShader);
}
/*
================
UI_FillRect

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_FillRect(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut color: *const f32,
) {
    crate::src::ui::ui_syscalls::trap_R_SetColor(color);
    UI_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    crate::src::ui::ui_syscalls::trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0f32,
        0f32,
        0f32,
        0f32,
        uis.whiteShader,
    );
    crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const f32);
}
/*
================
UI_DrawRect

Coordinates are 640*480 virtual values
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_DrawRect(
    mut x: f32,
    mut y: f32,
    mut width: f32,
    mut height: f32,
    mut color: *const f32,
) {
    crate::src::ui::ui_syscalls::trap_R_SetColor(color);
    UI_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    crate::src::ui::ui_syscalls::trap_R_DrawStretchPic(
        x,
        y,
        width,
        1f32,
        0f32,
        0f32,
        0f32,
        0f32,
        uis.whiteShader,
    );
    crate::src::ui::ui_syscalls::trap_R_DrawStretchPic(
        x,
        y,
        1f32,
        height,
        0f32,
        0f32,
        0f32,
        0f32,
        uis.whiteShader,
    );
    crate::src::ui::ui_syscalls::trap_R_DrawStretchPic(
        x,
        y + height - 1f32,
        width,
        1f32,
        0f32,
        0f32,
        0f32,
        0f32,
        uis.whiteShader,
    );
    crate::src::ui::ui_syscalls::trap_R_DrawStretchPic(
        x + width - 1f32,
        y,
        1f32,
        height,
        0f32,
        0f32,
        0f32,
        0f32,
        uis.whiteShader,
    );
    crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const f32);
}
#[no_mangle]

pub unsafe extern "C" fn UI_SetColor(mut rgba: *const f32) {
    crate::src::ui::ui_syscalls::trap_R_SetColor(rgba);
}
#[no_mangle]

pub unsafe extern "C" fn UI_UpdateScreen() {
    crate::src::ui::ui_syscalls::trap_UpdateScreen();
}
/*
=================
UI_Refresh
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Refresh(mut realtime: i32) {
    uis.frametime = realtime - uis.realtime;
    uis.realtime = realtime;
    if crate::src::ui::ui_syscalls::trap_Key_GetCatcher() & 0x2 == 0 {
        return;
    }
    crate::src::q3_ui::ui_main::UI_UpdateCvars();
    if !uis.activemenu.is_null() {
        if (*uis.activemenu).fullscreen as u64 != 0 {
            // draw the background
            if (*uis.activemenu).showlogo as u64 != 0 {
                UI_DrawHandlePic(0f32, 0f32, 640f32, 480f32, uis.menuBackShader);
            } else {
                UI_DrawHandlePic(0f32, 0f32, 640f32, 480f32, uis.menuBackNoLogoShader);
            }
        }
        if (*uis.activemenu).draw.is_some() {
            (*uis.activemenu).draw.expect("non-null function pointer")();
        } else {
            crate::src::q3_ui::ui_qmenu::Menu_Draw(uis.activemenu);
        }
        if uis.firstdraw as u64 != 0 {
            UI_MouseEvent(0, 0);
            uis.firstdraw = crate::src::qcommon::q_shared::qfalse
        }
    }
    // draw cursor
    UI_SetColor(0 as *const f32);
    UI_DrawHandlePic(
        (uis.cursorx - 16) as f32,
        (uis.cursory - 16) as f32,
        32f32,
        32f32,
        uis.cursor,
    );
    // delay playing the enter sound until after the
    // menu has been drawn, to avoid delay while
    // caching images
    if m_entersound as u64 != 0 {
        crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
            crate::src::q3_ui::ui_qmenu::menu_in_sound,
            crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND as i32,
        );
        m_entersound = crate::src::qcommon::q_shared::qfalse
    };
}
#[no_mangle]

pub unsafe extern "C" fn UI_DrawTextBox(mut x: i32, mut y: i32, mut width: i32, mut lines: i32) {
    UI_FillRect(
        (x + 16i32 / 2) as f32,
        (y + 16i32 / 2) as f32,
        ((width + 1i32) * 16) as f32,
        ((lines + 1i32) * 16) as f32,
        crate::src::qcommon::q_math::colorBlack.as_mut_ptr(),
    );
    UI_DrawRect(
        (x + 16i32 / 2) as f32,
        (y + 16i32 / 2) as f32,
        ((width + 1i32) * 16) as f32,
        ((lines + 1i32) * 16) as f32,
        crate::src::qcommon::q_math::colorWhite.as_mut_ptr(),
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
#[no_mangle]

pub unsafe extern "C" fn UI_CursorInRect(
    mut x: i32,
    mut y: i32,
    mut width: i32,
    mut height: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    if uis.cursorx < x || uis.cursory < y || uis.cursorx > x + width || uis.cursory > y + height {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
