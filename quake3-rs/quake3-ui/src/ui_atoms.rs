#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, libc)]
use bg_misc::bg_itemlist;
use q_math::{
    colorBlack, colorMdGrey, colorRed, colorWhite, colorYellow, g_color_table, vec3_origin,
    vectoangles, AngleMod, AngleNormalize180, AngleSubtract, AngleVectors, AnglesSubtract,
    AnglesToAxis, AxisClear, MatrixMultiply, Q_fabs,
};
use q_shared_h::{
    qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, unnamed_0, va, vec4_t, vec_t,
    Com_Printf, Q_IsColorString, Q_stricmp, Q_strncpyz, CHAN_ANNOUNCER, CHAN_AUTO, CHAN_BODY,
    CHAN_ITEM, CHAN_LOCAL, CHAN_LOCAL_SOUND, CHAN_VOICE, CHAN_WEAPON, EXEC_APPEND, EXEC_INSERT,
    EXEC_NOW,
};
use stdlib::{memcpy, sin, strlen};
use tr_types_h::{
    glDriverType_t, glHardwareType_t, glconfig_t, textureCompression_t, GLDRV_ICD,
    GLDRV_STANDALONE, GLDRV_VOODOO, GLHW_3DFX_2D3D, GLHW_GENERIC, GLHW_PERMEDIA2, GLHW_RAGEPRO,
    GLHW_RIVA128, TC_NONE, TC_S3TC, TC_S3TC_ARB,
};
use ui_addbots::{UI_AddBotsMenu, UI_AddBots_Cache};
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
    _tag_menuframework, menucommon_s, menuframework_s, trap_Argv, trap_Cmd_ExecuteText,
    trap_Cvar_Set, trap_Cvar_VariableStringBuffer, trap_Error, trap_GetGlconfig,
    trap_Key_ClearStates, trap_Key_GetCatcher, trap_Key_SetCatcher, trap_R_DrawStretchPic,
    trap_R_RegisterShaderNoMip, trap_R_SetColor, trap_S_StartLocalSound, trap_UpdateScreen,
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
use ui_playermodel::{PlayerModel_Cache, UI_PlayerModelMenu};
use ui_players::{UI_DrawPlayer, UI_PlayerInfo_SetInfo, UI_PlayerInfo_SetModel};
use ui_playersettings::{PlayerSettings_Cache, UI_PlayerSettingsMenu};
use ui_preferences::{Preferences_Cache, UI_PreferencesMenu};
use ui_public_h::{
    uiMenuCommand_t, UIMENU_BAD_CD_KEY, UIMENU_INGAME, UIMENU_MAIN, UIMENU_NEED_CD, UIMENU_NONE,
    UIMENU_POSTGAME, UIMENU_TEAM,
};
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

#[no_mangle]
pub unsafe extern "C" fn UI_Init() {
    UI_RegisterCvars();
    UI_InitGameinfo();
    trap_GetGlconfig(&mut uis.glconfig);
    uis.xscale = (uis.glconfig.vidWidth as libc::c_double * (1.0f64 / 640.0f64)) as libc::c_float;
    uis.yscale = (uis.glconfig.vidHeight as libc::c_double * (1.0f64 / 480.0f64)) as libc::c_float;
    if uis.glconfig.vidWidth * 480i32 > uis.glconfig.vidHeight * 640i32 {
        uis.bias = (0.5f64
            * (uis.glconfig.vidWidth as libc::c_double
                - uis.glconfig.vidHeight as libc::c_double * (640.0f64 / 480.0f64)))
            as libc::c_float;
        uis.xscale = uis.yscale
    } else {
        uis.bias = 0i32 as libc::c_float
    }
    Menu_Cache();
    uis.activemenu = 0 as *mut menuframework_s;
    uis.menusp = 0i32;
}
#[no_mangle]
pub static mut uis: uiStatic_t = uiStatic_t {
    frametime: 0,
    realtime: 0,
    cursorx: 0,
    cursory: 0,
    menusp: 0,
    activemenu: 0 as *const menuframework_s as *mut menuframework_s,
    stack: [0 as *const menuframework_s as *mut menuframework_s; 8],
    glconfig: glconfig_t {
        renderer_string: [0; 1024],
        vendor_string: [0; 1024],
        version_string: [0; 1024],
        extensions_string: [0; 8192],
        maxTextureSize: 0,
        numTextureUnits: 0,
        colorBits: 0,
        depthBits: 0,
        stencilBits: 0,
        driverType: GLDRV_ICD,
        hardwareType: GLHW_GENERIC,
        deviceSupportsGamma: qfalse,
        textureCompression: TC_NONE,
        textureEnvAddAvailable: qfalse,
        vidWidth: 0,
        vidHeight: 0,
        windowAspect: 0.,
        displayFrequency: 0,
        isFullscreen: qfalse,
        stereoEnabled: qfalse,
        smpActive: qfalse,
    },
    debug: qfalse,
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
    demoversion: qfalse,
    firstdraw: qfalse,
};
#[no_mangle]
pub unsafe extern "C" fn UI_Shutdown() {}
#[no_mangle]
pub unsafe extern "C" fn UI_KeyEvent(mut key: libc::c_int, mut down: libc::c_int) {
    let mut s: sfxHandle_t = 0;
    if uis.activemenu.is_null() {
        return;
    }
    if 0 == down {
        return;
    }
    if (*uis.activemenu).key.is_some() {
        s = (*uis.activemenu).key.expect("non-null function pointer")(key)
    } else {
        s = Menu_DefaultKey(uis.activemenu, key)
    }
    if s > 0i32 && s != menu_null_sound {
        trap_S_StartLocalSound(s, CHAN_LOCAL_SOUND as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_MouseEvent(mut dx: libc::c_int, mut dy: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut bias: libc::c_int = 0;
    let mut m: *mut menucommon_s = 0 as *mut menucommon_s;
    if uis.activemenu.is_null() {
        return;
    }
    bias = (uis.bias / uis.xscale) as libc::c_int;
    uis.cursorx += dx;
    if uis.cursorx < -bias {
        uis.cursorx = -bias
    } else if uis.cursorx > 640i32 + bias {
        uis.cursorx = 640i32 + bias
    }
    uis.cursory += dy;
    if uis.cursory < 0i32 {
        uis.cursory = 0i32
    } else if uis.cursory > 480i32 {
        uis.cursory = 480i32
    }
    i = 0i32;
    while i < (*uis.activemenu).nitems {
        m = (*uis.activemenu).items[i as usize] as *mut menucommon_s;
        if !(0 != (*m).flags & (0x2000i32 as libc::c_uint | 0x4000i32 as libc::c_uint)) {
            if !(uis.cursorx < (*m).left
                || uis.cursorx > (*m).right
                || uis.cursory < (*m).top
                || uis.cursory > (*m).bottom)
            {
                // cursor out of item bounds
                if (*uis.activemenu).cursor != i {
                    Menu_SetCursor(uis.activemenu, i);
                    (*((*uis.activemenu).items[(*uis.activemenu).cursor_prev as usize]
                        as *mut menucommon_s))
                        .flags &= !(0x200i32 as libc::c_uint);
                    if 0 == (*((*uis.activemenu).items[(*uis.activemenu).cursor as usize]
                        as *mut menucommon_s))
                        .flags
                        & 0x100000i32 as libc::c_uint
                    {
                        trap_S_StartLocalSound(menu_move_sound, CHAN_LOCAL_SOUND as libc::c_int);
                    }
                }
                (*((*uis.activemenu).items[(*uis.activemenu).cursor as usize]
                    as *mut menucommon_s))
                    .flags |= 0x200i32 as libc::c_uint;
                return;
            }
        }
        i += 1
    }
    if (*uis.activemenu).nitems > 0i32 {
        (*((*uis.activemenu).items[(*uis.activemenu).cursor as usize] as *mut menucommon_s))
            .flags &= !(0x200i32 as libc::c_uint)
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_Refresh(mut realtime: libc::c_int) {
    uis.frametime = realtime - uis.realtime;
    uis.realtime = realtime;
    if 0 == trap_Key_GetCatcher() & 0x2i32 {
        return;
    }
    UI_UpdateCvars();
    if !uis.activemenu.is_null() {
        if 0 != (*uis.activemenu).fullscreen as u64 {
            if 0 != (*uis.activemenu).showlogo as u64 {
                UI_DrawHandlePic(
                    0i32 as libc::c_float,
                    0i32 as libc::c_float,
                    640i32 as libc::c_float,
                    480i32 as libc::c_float,
                    uis.menuBackShader,
                );
            } else {
                UI_DrawHandlePic(
                    0i32 as libc::c_float,
                    0i32 as libc::c_float,
                    640i32 as libc::c_float,
                    480i32 as libc::c_float,
                    uis.menuBackNoLogoShader,
                );
            }
        }
        if (*uis.activemenu).draw.is_some() {
            (*uis.activemenu).draw.expect("non-null function pointer")();
        } else {
            Menu_Draw(uis.activemenu);
        }
        if 0 != uis.firstdraw as u64 {
            UI_MouseEvent(0i32, 0i32);
            uis.firstdraw = qfalse
        }
    }
    UI_SetColor(0 as *const libc::c_float);
    UI_DrawHandlePic(
        (uis.cursorx - 16i32) as libc::c_float,
        (uis.cursory - 16i32) as libc::c_float,
        32i32 as libc::c_float,
        32i32 as libc::c_float,
        uis.cursor,
    );
    if 0 != uis.debug as u64 {
        UI_DrawString(
            0i32,
            0i32,
            va(
                b"(%d,%d)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                uis.cursorx,
                uis.cursory,
            ),
            0i32 | 0x10i32,
            colorRed.as_mut_ptr(),
        );
    }
    if 0 != m_entersound as u64 {
        trap_S_StartLocalSound(menu_in_sound, CHAN_LOCAL_SOUND as libc::c_int);
        m_entersound = qfalse
    };
}
#[no_mangle]
pub static mut m_entersound: qboolean = qfalse;
#[no_mangle]
pub unsafe extern "C" fn UI_DrawString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut style: libc::c_int,
    mut color: *mut vec_t,
) {
    let mut len: libc::c_int = 0;
    let mut charw: libc::c_int = 0;
    let mut charh: libc::c_int = 0;
    let mut newcolor: vec4_t = [0.; 4];
    let mut lowlight: vec4_t = [0.; 4];
    let mut drawcolor: *mut libc::c_float = 0 as *mut libc::c_float;
    let mut dropcolor: vec4_t = [0.; 4];
    if str.is_null() {
        return;
    }
    if 0 != style & 0x1000i32 && 0 != uis.realtime / 200i32 & 1i32 {
        return;
    }
    if 0 != style & 0x10i32 {
        charw = 8i32;
        charh = 16i32
    } else if 0 != style & 0x40i32 {
        charw = 32i32;
        charh = 48i32
    } else {
        charw = 16i32;
        charh = 16i32
    }
    if 0 != style & 0x4000i32 {
        lowlight[0usize] = (0.8f64 * *color.offset(0isize) as libc::c_double) as vec_t;
        lowlight[1usize] = (0.8f64 * *color.offset(1isize) as libc::c_double) as vec_t;
        lowlight[2usize] = (0.8f64 * *color.offset(2isize) as libc::c_double) as vec_t;
        lowlight[3usize] = (0.8f64 * *color.offset(3isize) as libc::c_double) as vec_t;
        UI_LerpColor(
            color,
            lowlight.as_mut_ptr(),
            newcolor.as_mut_ptr(),
            (0.5f64 + 0.5f64 * sin((uis.realtime / 75i32) as libc::c_double)) as libc::c_float,
        );
        drawcolor = newcolor.as_mut_ptr()
    } else {
        drawcolor = color
    }
    match style & 0x7i32 {
        1 => {
            len = strlen(str) as libc::c_int;
            x = x - len * charw / 2i32
        }
        2 => {
            len = strlen(str) as libc::c_int;
            x = x - len * charw
        }
        _ => {}
    }
    if 0 != style & 0x800i32 {
        dropcolor[2usize] = 0i32 as vec_t;
        dropcolor[1usize] = dropcolor[2usize];
        dropcolor[0usize] = dropcolor[1usize];
        dropcolor[3usize] = *drawcolor.offset(3isize);
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
UI_DrawString2
=================
*/
unsafe extern "C" fn UI_DrawString2(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut color: *mut vec_t,
    mut charw: libc::c_int,
    mut charh: libc::c_int,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_char = 0;
    //APSFIXME;
    let mut forceColor: libc::c_int = qfalse as libc::c_int;
    let mut tempcolor: vec4_t = [0.; 4];
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0.;
    let mut ah: libc::c_float = 0.;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    if y < -charh {
        return;
    }
    trap_R_SetColor(color as *const libc::c_float);
    ax = x as libc::c_float * uis.xscale + uis.bias;
    ay = y as libc::c_float * uis.yscale;
    aw = charw as libc::c_float * uis.xscale;
    ah = charh as libc::c_float * uis.yscale;
    s = str;
    while 0 != *s {
        if 0 != Q_IsColorString(s) as u64 {
            if 0 == forceColor {
                memcpy(
                    tempcolor.as_mut_ptr() as *mut libc::c_void,
                    g_color_table[(*s.offset(1isize) as libc::c_int - '0' as i32 & 0x7i32) as usize]
                        .as_mut_ptr() as *const libc::c_void,
                    ::std::mem::size_of::<vec4_t>() as libc::c_ulong,
                );
                tempcolor[3usize] = *color.offset(3isize);
                trap_R_SetColor(tempcolor.as_mut_ptr());
            }
            s = s.offset(2isize)
        } else {
            ch = (*s as libc::c_int & 255i32) as libc::c_char;
            if ch as libc::c_int != ' ' as i32 {
                frow = ((ch as libc::c_int >> 4i32) as libc::c_double * 0.0625f64) as libc::c_float;
                fcol = ((ch as libc::c_int & 15i32) as libc::c_double * 0.0625f64) as libc::c_float;
                trap_R_DrawStretchPic(
                    ax,
                    ay,
                    aw,
                    ah,
                    fcol,
                    frow,
                    (fcol as libc::c_double + 0.0625f64) as libc::c_float,
                    (frow as libc::c_double + 0.0625f64) as libc::c_float,
                    uis.charset,
                );
            }
            ax += aw;
            s = s.offset(1isize)
        }
    }
    trap_R_SetColor(0 as *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn UI_LerpColor(
    mut a: *mut vec_t,
    mut b: *mut vec_t,
    mut c: *mut vec_t,
    mut t: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 4i32 {
        *c.offset(i as isize) =
            *a.offset(i as isize) + t * (*b.offset(i as isize) - *a.offset(i as isize));
        if *c.offset(i as isize) < 0i32 as libc::c_float {
            *c.offset(i as isize) = 0i32 as vec_t
        } else if *c.offset(i as isize) as libc::c_double > 1.0f64 {
            *c.offset(i as isize) = 1.0f64 as vec_t
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn UI_DrawHandlePic(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut w: libc::c_float,
    mut h: libc::c_float,
    mut hShader: qhandle_t,
) {
    let mut s0: libc::c_float = 0.;
    let mut s1: libc::c_float = 0.;
    let mut t0: libc::c_float = 0.;
    let mut t1: libc::c_float = 0.;
    if w < 0i32 as libc::c_float {
        w = -w;
        s0 = 1i32 as libc::c_float;
        s1 = 0i32 as libc::c_float
    } else {
        s0 = 0i32 as libc::c_float;
        s1 = 1i32 as libc::c_float
    }
    if h < 0i32 as libc::c_float {
        h = -h;
        t0 = 1i32 as libc::c_float;
        t1 = 0i32 as libc::c_float
    } else {
        t0 = 0i32 as libc::c_float;
        t1 = 1i32 as libc::c_float
    }
    UI_AdjustFrom640(&mut x, &mut y, &mut w, &mut h);
    trap_R_DrawStretchPic(x, y, w, h, s0, t0, s1, t1, hShader);
}
#[no_mangle]
pub unsafe extern "C" fn UI_AdjustFrom640(
    mut x: *mut libc::c_float,
    mut y: *mut libc::c_float,
    mut w: *mut libc::c_float,
    mut h: *mut libc::c_float,
) {
    *x = *x * uis.xscale + uis.bias;
    *y *= uis.yscale;
    *w *= uis.xscale;
    *h *= uis.yscale;
}
#[no_mangle]
pub unsafe extern "C" fn UI_SetColor(mut rgba: *const libc::c_float) {
    trap_R_SetColor(rgba);
}
#[no_mangle]
pub unsafe extern "C" fn UI_ConsoleCommand(mut realTime: libc::c_int) -> qboolean {
    let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
    uis.frametime = realTime - uis.realtime;
    uis.realtime = realTime;
    cmd = UI_Argv(0i32);
    Menu_Cache();
    if Q_stricmp(cmd, b"levelselect\x00" as *const u8 as *const libc::c_char) == 0i32 {
        UI_SPLevelMenu_f();
        return qtrue;
    }
    if Q_stricmp(cmd, b"postgame\x00" as *const u8 as *const libc::c_char) == 0i32 {
        UI_SPPostgameMenu_f();
        return qtrue;
    }
    if Q_stricmp(cmd, b"ui_cache\x00" as *const u8 as *const libc::c_char) == 0i32 {
        UI_Cache_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd,
        b"ui_cinematics\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        UI_CinematicsMenu_f();
        return qtrue;
    }
    if Q_stricmp(
        cmd,
        b"ui_teamOrders\x00" as *const u8 as *const libc::c_char,
    ) == 0i32
    {
        UI_TeamOrdersMenu_f();
        return qtrue;
    }
    if Q_stricmp(cmd, b"iamacheater\x00" as *const u8 as *const libc::c_char) == 0i32 {
        UI_SPUnlock_f();
        return qtrue;
    }
    if Q_stricmp(cmd, b"iamamonkey\x00" as *const u8 as *const libc::c_char) == 0i32 {
        UI_SPUnlockMedals_f();
        return qtrue;
    }
    if Q_stricmp(cmd, b"ui_cdkey\x00" as *const u8 as *const libc::c_char) == 0i32 {
        UI_CDKeyMenu_f();
        return qtrue;
    }
    return qfalse;
}
/*
=================
UI_Cache
=================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_Cache_f() {
    MainMenu_Cache();
    InGame_Cache();
    ConfirmMenu_Cache();
    PlayerModel_Cache();
    PlayerSettings_Cache();
    Controls_Cache();
    Demos_Cache();
    UI_CinematicsMenu_Cache();
    Preferences_Cache();
    ServerInfo_Cache();
    SpecifyServer_Cache();
    ArenaServers_Cache();
    StartServer_Cache();
    ServerOptions_Cache();
    DriverInfo_Cache();
    GraphicsOptions_Cache();
    UI_DisplayOptionsMenu_Cache();
    UI_SoundOptionsMenu_Cache();
    UI_NetworkOptionsMenu_Cache();
    UI_SPLevelMenu_Cache();
    UI_SPSkillMenu_Cache();
    UI_SPPostgameMenu_Cache();
    TeamMain_Cache();
    UI_AddBots_Cache();
    UI_RemoveBots_Cache();
    UI_SetupMenu_Cache();
    UI_BotSelectMenu_Cache();
    UI_CDKeyMenu_Cache();
    UI_ModsMenu_Cache();
}
#[no_mangle]
pub unsafe extern "C" fn UI_Argv(mut arg: libc::c_int) -> *mut libc::c_char {
    static mut buffer: [libc::c_char; 1024] = [0; 1024];
    trap_Argv(
        arg,
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    return buffer.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn UI_ClampCvar(
    mut min: libc::c_float,
    mut max: libc::c_float,
    mut value: libc::c_float,
) -> libc::c_float {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn UI_DrawNamedPic(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut picname: *const libc::c_char,
) {
    let mut hShader: qhandle_t = 0;
    hShader = trap_R_RegisterShaderNoMip(picname);
    UI_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        1i32 as libc::c_float,
        1i32 as libc::c_float,
        hShader,
    );
}
#[no_mangle]
pub unsafe extern "C" fn UI_FillRect(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut color: *const libc::c_float,
) {
    trap_R_SetColor(color);
    UI_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    trap_R_DrawStretchPic(
        x,
        y,
        width,
        height,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        uis.whiteShader,
    );
    trap_R_SetColor(0 as *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn UI_DrawRect(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut width: libc::c_float,
    mut height: libc::c_float,
    mut color: *const libc::c_float,
) {
    trap_R_SetColor(color);
    UI_AdjustFrom640(&mut x, &mut y, &mut width, &mut height);
    trap_R_DrawStretchPic(
        x,
        y,
        width,
        1i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        uis.whiteShader,
    );
    trap_R_DrawStretchPic(
        x,
        y,
        1i32 as libc::c_float,
        height,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        uis.whiteShader,
    );
    trap_R_DrawStretchPic(
        x,
        y + height - 1i32 as libc::c_float,
        width,
        1i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        uis.whiteShader,
    );
    trap_R_DrawStretchPic(
        x + width - 1i32 as libc::c_float,
        y,
        1i32 as libc::c_float,
        height,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
        uis.whiteShader,
    );
    trap_R_SetColor(0 as *const libc::c_float);
}
#[no_mangle]
pub unsafe extern "C" fn UI_UpdateScreen() {
    trap_UpdateScreen();
}
#[no_mangle]
pub unsafe extern "C" fn UI_DrawBannerString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut style: libc::c_int,
    mut color: *mut vec_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut drawcolor: vec4_t = [0.; 4];
    s = str;
    width = 0i32;
    while 0 != *s {
        ch = *s as libc::c_int;
        if ch == ' ' as i32 {
            width += 12i32
        } else if ch >= 'A' as i32 && ch <= 'Z' as i32 {
            width += propMapB[(ch - 'A' as i32) as usize][2usize] + 4i32
        }
        s = s.offset(1isize)
    }
    width -= 4i32;
    match style & 0x7i32 {
        1 => x -= width / 2i32,
        2 => x -= width,
        0 | _ => {}
    }
    if 0 != style & 0x800i32 {
        drawcolor[2usize] = 0i32 as vec_t;
        drawcolor[1usize] = drawcolor[2usize];
        drawcolor[0usize] = drawcolor[1usize];
        drawcolor[3usize] = *color.offset(3isize);
        UI_DrawBannerString2(x + 2i32, y + 2i32, str, drawcolor.as_mut_ptr());
    }
    UI_DrawBannerString2(x, y, str, color);
}
/*
=================
UI_DrawBannerString
=================
*/
unsafe extern "C" fn UI_DrawBannerString2(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut color: *mut vec_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_uchar = 0;
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0.;
    let mut ah: libc::c_float = 0.;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut fwidth: libc::c_float = 0.;
    let mut fheight: libc::c_float = 0.;
    trap_R_SetColor(color as *const libc::c_float);
    ax = x as libc::c_float * uis.xscale + uis.bias;
    ay = y as libc::c_float * uis.yscale;
    s = str;
    while 0 != *s {
        ch = (*s as libc::c_int & 127i32) as libc::c_uchar;
        if ch as libc::c_int == ' ' as i32 {
            ax += (12i32 as libc::c_float + 4i32 as libc::c_float) * uis.xscale
        } else if ch as libc::c_int >= 'A' as i32 && ch as libc::c_int <= 'Z' as i32 {
            ch = (ch as libc::c_int - 'A' as i32) as libc::c_uchar;
            fcol = propMapB[ch as usize][0usize] as libc::c_float / 256.0f32;
            frow = propMapB[ch as usize][1usize] as libc::c_float / 256.0f32;
            fwidth = propMapB[ch as usize][2usize] as libc::c_float / 256.0f32;
            fheight = 36i32 as libc::c_float / 256.0f32;
            aw = propMapB[ch as usize][2usize] as libc::c_float * uis.xscale;
            ah = 36i32 as libc::c_float * uis.yscale;
            trap_R_DrawStretchPic(
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
            ax += aw + 4i32 as libc::c_float * uis.xscale
        }
        s = s.offset(1isize)
    }
    trap_R_SetColor(0 as *const libc::c_float);
}
// SPACE
// !
// "
// #
// $
// %
// &
// '
// (
// )
// *
// +
// ,
// -
// .
// /
// 0
// 1
// 2
// 3
// 4
// 5
// 6
// 7
// 8
// 9
// :
// ;
// <
// =
// >
// ?
// @
// A
// B
// C
// D
// E
// F
// G
// H
// I
// J
// K
// L
// M
// N
// O
// P
// Q
// R
// S
// T
// U
// V
// W
// X
// Y
// Z
// [
// '\'
// ]
// ^
// _
// '
// A
// B
// C
// D
// E
// F
// G
// H
// I
// J
// K
// L
// M
// N
// O
// P
// Q
// R
// S
// T
// U
// V
// W
// X
// Y
// Z
// {
// |
// }
// ~
// DEL
static mut propMapB: [[libc::c_int; 3]; 26] = [
    [11i32, 12i32, 33i32],
    [49i32, 12i32, 31i32],
    [85i32, 12i32, 31i32],
    [120i32, 12i32, 30i32],
    [156i32, 12i32, 21i32],
    [183i32, 12i32, 21i32],
    [207i32, 12i32, 32i32],
    [13i32, 55i32, 30i32],
    [49i32, 55i32, 13i32],
    [66i32, 55i32, 29i32],
    [101i32, 55i32, 31i32],
    [135i32, 55i32, 21i32],
    [158i32, 55i32, 40i32],
    [204i32, 55i32, 32i32],
    [12i32, 97i32, 31i32],
    [48i32, 97i32, 31i32],
    [82i32, 97i32, 30i32],
    [118i32, 97i32, 30i32],
    [153i32, 97i32, 30i32],
    [185i32, 97i32, 25i32],
    [213i32, 97i32, 30i32],
    [11i32, 139i32, 32i32],
    [42i32, 139i32, 51i32],
    [93i32, 139i32, 32i32],
    [126i32, 139i32, 31i32],
    [158i32, 139i32, 25i32],
];
#[no_mangle]
pub unsafe extern "C" fn UI_ProportionalSizeScale(mut style: libc::c_int) -> libc::c_float {
    if 0 != style & 0x10i32 {
        return 0.75f64 as libc::c_float;
    }
    return 1.00f64 as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn UI_DrawProportionalString(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut style: libc::c_int,
    mut color: *mut vec_t,
) {
    let mut drawcolor: vec4_t = [0.; 4];
    let mut width: libc::c_int = 0;
    let mut sizeScale: libc::c_float = 0.;
    if str.is_null() {
        return;
    }
    sizeScale = UI_ProportionalSizeScale(style);
    match style & 0x7i32 {
        1 => {
            width = (UI_ProportionalStringWidth(str) as libc::c_float * sizeScale) as libc::c_int;
            x -= width / 2i32
        }
        2 => {
            width = (UI_ProportionalStringWidth(str) as libc::c_float * sizeScale) as libc::c_int;
            x -= width
        }
        0 | _ => {}
    }
    if 0 != style & 0x800i32 {
        drawcolor[2usize] = 0i32 as vec_t;
        drawcolor[1usize] = drawcolor[2usize];
        drawcolor[0usize] = drawcolor[1usize];
        drawcolor[3usize] = *color.offset(3isize);
        UI_DrawProportionalString2(
            x + 2i32,
            y + 2i32,
            str,
            drawcolor.as_mut_ptr(),
            sizeScale,
            uis.charsetProp,
        );
    }
    if 0 != style & 0x2000i32 {
        drawcolor[0usize] = (*color.offset(0isize) as libc::c_double * 0.7f64) as vec_t;
        drawcolor[1usize] = (*color.offset(1isize) as libc::c_double * 0.7f64) as vec_t;
        drawcolor[2usize] = (*color.offset(2isize) as libc::c_double * 0.7f64) as vec_t;
        drawcolor[3usize] = *color.offset(3isize);
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
    if 0 != style & 0x4000i32 {
        drawcolor[0usize] = (*color.offset(0isize) as libc::c_double * 0.7f64) as vec_t;
        drawcolor[1usize] = (*color.offset(1isize) as libc::c_double * 0.7f64) as vec_t;
        drawcolor[2usize] = (*color.offset(2isize) as libc::c_double * 0.7f64) as vec_t;
        drawcolor[3usize] = *color.offset(3isize);
        UI_DrawProportionalString2(x, y, str, color, sizeScale, uis.charsetProp);
        drawcolor[0usize] = *color.offset(0isize);
        drawcolor[1usize] = *color.offset(1isize);
        drawcolor[2usize] = *color.offset(2isize);
        drawcolor[3usize] =
            (0.5f64 + 0.5f64 * sin((uis.realtime / 75i32) as libc::c_double)) as vec_t;
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
unsafe extern "C" fn UI_DrawProportionalString2(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut str: *const libc::c_char,
    mut color: *mut vec_t,
    mut sizeScale: libc::c_float,
    mut charset: qhandle_t,
) {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_uchar = 0;
    let mut ax: libc::c_float = 0.;
    let mut ay: libc::c_float = 0.;
    let mut aw: libc::c_float = 0i32 as libc::c_float;
    let mut ah: libc::c_float = 0.;
    let mut frow: libc::c_float = 0.;
    let mut fcol: libc::c_float = 0.;
    let mut fwidth: libc::c_float = 0.;
    let mut fheight: libc::c_float = 0.;
    trap_R_SetColor(color as *const libc::c_float);
    ax = x as libc::c_float * uis.xscale + uis.bias;
    ay = y as libc::c_float * uis.yscale;
    s = str;
    while 0 != *s {
        ch = (*s as libc::c_int & 127i32) as libc::c_uchar;
        if ch as libc::c_int == ' ' as i32 {
            aw = 8i32 as libc::c_float * uis.xscale * sizeScale
        } else if propMap[ch as usize][2usize] != -1i32 {
            fcol = propMap[ch as usize][0usize] as libc::c_float / 256.0f32;
            frow = propMap[ch as usize][1usize] as libc::c_float / 256.0f32;
            fwidth = propMap[ch as usize][2usize] as libc::c_float / 256.0f32;
            fheight = 27i32 as libc::c_float / 256.0f32;
            aw = propMap[ch as usize][2usize] as libc::c_float * uis.xscale * sizeScale;
            ah = 27i32 as libc::c_float * uis.yscale * sizeScale;
            trap_R_DrawStretchPic(
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
        ax += aw + 3i32 as libc::c_float * uis.xscale * sizeScale;
        s = s.offset(1isize)
    }
    trap_R_SetColor(0 as *const libc::c_float);
}
/*
=================
UI_DrawProportionalString2
=================
*/
static mut propMap: [[libc::c_int; 3]; 128] = [
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, -1i32],
    [0i32, 0i32, 8i32],
    [11i32, 122i32, 7i32],
    [154i32, 181i32, 14i32],
    [55i32, 122i32, 17i32],
    [79i32, 122i32, 18i32],
    [101i32, 122i32, 23i32],
    [153i32, 122i32, 18i32],
    [9i32, 93i32, 7i32],
    [207i32, 122i32, 8i32],
    [230i32, 122i32, 9i32],
    [177i32, 122i32, 18i32],
    [30i32, 152i32, 18i32],
    [85i32, 181i32, 7i32],
    [34i32, 93i32, 11i32],
    [110i32, 181i32, 6i32],
    [130i32, 152i32, 14i32],
    [22i32, 64i32, 17i32],
    [41i32, 64i32, 12i32],
    [58i32, 64i32, 17i32],
    [78i32, 64i32, 18i32],
    [98i32, 64i32, 19i32],
    [120i32, 64i32, 18i32],
    [141i32, 64i32, 18i32],
    [204i32, 64i32, 16i32],
    [162i32, 64i32, 17i32],
    [182i32, 64i32, 18i32],
    [59i32, 181i32, 7i32],
    [35i32, 181i32, 7i32],
    [203i32, 152i32, 14i32],
    [56i32, 93i32, 14i32],
    [228i32, 152i32, 14i32],
    [177i32, 181i32, 18i32],
    [28i32, 122i32, 22i32],
    [5i32, 4i32, 18i32],
    [27i32, 4i32, 18i32],
    [48i32, 4i32, 18i32],
    [69i32, 4i32, 17i32],
    [90i32, 4i32, 13i32],
    [106i32, 4i32, 13i32],
    [121i32, 4i32, 18i32],
    [143i32, 4i32, 17i32],
    [164i32, 4i32, 8i32],
    [175i32, 4i32, 16i32],
    [195i32, 4i32, 18i32],
    [216i32, 4i32, 12i32],
    [230i32, 4i32, 23i32],
    [6i32, 34i32, 18i32],
    [27i32, 34i32, 18i32],
    [48i32, 34i32, 18i32],
    [68i32, 34i32, 18i32],
    [90i32, 34i32, 17i32],
    [110i32, 34i32, 18i32],
    [130i32, 34i32, 14i32],
    [146i32, 34i32, 18i32],
    [166i32, 34i32, 19i32],
    [185i32, 34i32, 29i32],
    [215i32, 34i32, 18i32],
    [234i32, 34i32, 18i32],
    [5i32, 64i32, 14i32],
    [60i32, 152i32, 7i32],
    [106i32, 151i32, 13i32],
    [83i32, 152i32, 7i32],
    [128i32, 122i32, 17i32],
    [4i32, 152i32, 21i32],
    [134i32, 181i32, 5i32],
    [5i32, 4i32, 18i32],
    [27i32, 4i32, 18i32],
    [48i32, 4i32, 18i32],
    [69i32, 4i32, 17i32],
    [90i32, 4i32, 13i32],
    [106i32, 4i32, 13i32],
    [121i32, 4i32, 18i32],
    [143i32, 4i32, 17i32],
    [164i32, 4i32, 8i32],
    [175i32, 4i32, 16i32],
    [195i32, 4i32, 18i32],
    [216i32, 4i32, 12i32],
    [230i32, 4i32, 23i32],
    [6i32, 34i32, 18i32],
    [27i32, 34i32, 18i32],
    [48i32, 34i32, 18i32],
    [68i32, 34i32, 18i32],
    [90i32, 34i32, 17i32],
    [110i32, 34i32, 18i32],
    [130i32, 34i32, 14i32],
    [146i32, 34i32, 18i32],
    [166i32, 34i32, 19i32],
    [185i32, 34i32, 29i32],
    [215i32, 34i32, 18i32],
    [234i32, 34i32, 18i32],
    [5i32, 64i32, 14i32],
    [153i32, 152i32, 13i32],
    [11i32, 181i32, 5i32],
    [180i32, 152i32, 13i32],
    [79i32, 93i32, 17i32],
    [0i32, 0i32, -1i32],
];
#[no_mangle]
pub unsafe extern "C" fn UI_ProportionalStringWidth(mut str: *const libc::c_char) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ch: libc::c_int = 0;
    let mut charWidth: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    s = str;
    width = 0i32;
    while 0 != *s {
        ch = *s as libc::c_int & 127i32;
        charWidth = propMap[ch as usize][2usize];
        if charWidth != -1i32 {
            width += charWidth;
            width += 3i32
        }
        s = s.offset(1isize)
    }
    width -= 3i32;
    return width;
}
#[no_mangle]
pub unsafe extern "C" fn UI_DrawProportionalString_AutoWrapped(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut xmax: libc::c_int,
    mut ystep: libc::c_int,
    mut str: *const libc::c_char,
    mut style: libc::c_int,
    mut color: *mut vec_t,
) {
    let mut width: libc::c_int = 0;
    let mut s1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s3: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c_bcp: libc::c_char = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut sizeScale: libc::c_float = 0.;
    if str.is_null() || *str.offset(0isize) as libc::c_int == '\u{0}' as i32 {
        return;
    }
    sizeScale = UI_ProportionalSizeScale(style);
    Q_strncpyz(
        buf.as_mut_ptr(),
        str,
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    s3 = buf.as_mut_ptr();
    s2 = s3;
    s1 = s2;
    loop {
        loop {
            s3 = s3.offset(1isize);
            if !(*s3 as libc::c_int != ' ' as i32 && *s3 as libc::c_int != '\u{0}' as i32) {
                break;
            }
        }
        c_bcp = *s3;
        *s3 = '\u{0}' as i32 as libc::c_char;
        width = (UI_ProportionalStringWidth(s1) as libc::c_float * sizeScale) as libc::c_int;
        *s3 = c_bcp;
        if width > xmax {
            if s1 == s2 {
                s2 = s3
            }
            *s2 = '\u{0}' as i32 as libc::c_char;
            UI_DrawProportionalString(x, y, s1, style, color);
            y += ystep;
            if c_bcp as libc::c_int == '\u{0}' as i32 {
                s2 = s2.offset(1isize);
                if *s2 as libc::c_int != '\u{0}' as i32 {
                    UI_DrawProportionalString(x, y, s2, style, color);
                }
                break;
            } else {
                s2 = s2.offset(1isize);
                s1 = s2;
                s3 = s2
            }
        } else {
            s2 = s3;
            // we reached the end
            if !(c_bcp as libc::c_int == '\u{0}' as i32) {
                continue;
            }
            UI_DrawProportionalString(x, y, s1, style, color);
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn UI_DrawChar(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut ch: libc::c_int,
    mut style: libc::c_int,
    mut color: *mut vec_t,
) {
    let mut buff: [libc::c_char; 2] = [0; 2];
    buff[0usize] = ch as libc::c_char;
    buff[1usize] = '\u{0}' as i32 as libc::c_char;
    UI_DrawString(x, y, buff.as_mut_ptr(), style, color);
}
#[no_mangle]
pub unsafe extern "C" fn UI_CursorInRect(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> qboolean {
    if uis.cursorx < x || uis.cursory < y || uis.cursorx > x + width || uis.cursory > y + height {
        return qfalse;
    }
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn UI_DrawTextBox(
    mut x: libc::c_int,
    mut y: libc::c_int,
    mut width: libc::c_int,
    mut lines: libc::c_int,
) {
    UI_FillRect(
        (x + 16i32 / 2i32) as libc::c_float,
        (y + 16i32 / 2i32) as libc::c_float,
        ((width + 1i32) * 16i32) as libc::c_float,
        ((lines + 1i32) * 16i32) as libc::c_float,
        colorBlack.as_mut_ptr(),
    );
    UI_DrawRect(
        (x + 16i32 / 2i32) as libc::c_float,
        (y + 16i32 / 2i32) as libc::c_float,
        ((width + 1i32) * 16i32) as libc::c_float,
        ((lines + 1i32) * 16i32) as libc::c_float,
        colorWhite.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn UI_IsFullscreen() -> qboolean {
    if !uis.activemenu.is_null() && 0 != trap_Key_GetCatcher() & 0x2i32 {
        return (*uis.activemenu).fullscreen;
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn UI_SetActiveMenu(mut menu: uiMenuCommand_t) {
    Menu_Cache();
    match menu as libc::c_uint {
        0 => {
            UI_ForceMenuOff();
            return;
        }
        1 => {
            UI_MainMenu();
            return;
        }
        3 => {
            UI_ConfirmMenu(
                b"Insert the CD\x00" as *const u8 as *const libc::c_char,
                None,
                Some(NeedCDAction),
            );
            return;
        }
        4 => {
            UI_ConfirmMenu(
                b"Bad CD Key\x00" as *const u8 as *const libc::c_char,
                None,
                Some(NeedCDKeyAction),
            );
            return;
        }
        2 => {
            trap_Cvar_Set(
                b"cl_paused\x00" as *const u8 as *const libc::c_char,
                b"1\x00" as *const u8 as *const libc::c_char,
            );
            UI_InGameMenu();
            return;
        }
        5 | 6 | _ => {
            Com_Printf(
                b"UI_SetActiveMenu: bad enum %d\n\x00" as *const u8 as *const libc::c_char,
                menu as libc::c_uint,
            );
        }
    };
}
unsafe extern "C" fn NeedCDKeyAction(mut result: qboolean) {
    if 0 == result as u64 {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            b"quit\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}
unsafe extern "C" fn NeedCDAction(mut result: qboolean) {
    if 0 == result as u64 {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            b"quit\n\x00" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_ForceMenuOff() {
    uis.menusp = 0i32;
    uis.activemenu = 0 as *mut menuframework_s;
    trap_Key_SetCatcher(trap_Key_GetCatcher() & !0x2i32);
    trap_Key_ClearStates();
    trap_Cvar_Set(
        b"cl_paused\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn UI_PushMenu(mut menu: *mut menuframework_s) {
    let mut i: libc::c_int = 0;
    let mut item: *mut menucommon_s = 0 as *mut menucommon_s;
    i = 0i32;
    while i < uis.menusp {
        if uis.stack[i as usize] == menu {
            uis.menusp = i;
            break;
        } else {
            i += 1
        }
    }
    if i == uis.menusp {
        if uis.menusp >= 8i32 {
            trap_Error(b"UI_PushMenu: menu stack overflow\x00" as *const u8 as *const libc::c_char);
        }
        let fresh0 = uis.menusp;
        uis.menusp = uis.menusp + 1;
        uis.stack[fresh0 as usize] = menu
    }
    uis.activemenu = menu;
    (*menu).cursor = 0i32;
    (*menu).cursor_prev = 0i32;
    m_entersound = qtrue;
    trap_Key_SetCatcher(0x2i32);
    i = 0i32;
    while i < (*menu).nitems {
        item = (*menu).items[i as usize] as *mut menucommon_s;
        if 0 == (*item).flags
            & (0x2000i32 as libc::c_uint | 0x800i32 as libc::c_uint | 0x4000i32 as libc::c_uint)
        {
            (*menu).cursor_prev = -1i32;
            Menu_SetCursor(menu, i);
            break;
        } else {
            i += 1
        }
    }
    uis.firstdraw = qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn UI_PopMenu() {
    trap_S_StartLocalSound(menu_out_sound, CHAN_LOCAL_SOUND as libc::c_int);
    uis.menusp -= 1;
    if uis.menusp < 0i32 {
        trap_Error(b"UI_PopMenu: menu stack underflow\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != uis.menusp {
        uis.activemenu = uis.stack[(uis.menusp - 1i32) as usize];
        uis.firstdraw = qtrue
    } else {
        UI_ForceMenuOff();
    };
}
#[no_mangle]
pub unsafe extern "C" fn UI_Cvar_VariableString(
    mut var_name: *const libc::c_char,
) -> *mut libc::c_char {
    static mut buffer: [libc::c_char; 1024] = [0; 1024];
    trap_Cvar_VariableStringBuffer(
        var_name,
        buffer.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    return buffer.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn UI_StartDemoLoop() {
    trap_Cmd_ExecuteText(
        EXEC_APPEND as libc::c_int,
        b"d1\n\x00" as *const u8 as *const libc::c_char,
    );
}
