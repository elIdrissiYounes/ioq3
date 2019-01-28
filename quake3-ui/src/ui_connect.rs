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
    connstate_t, qboolean, qfalse, qhandle_t, qtrue, sfxHandle_t, unnamed, va, vec4_t, vec_t,
    Com_sprintf, Info_ValueForKey, CA_ACTIVE, CA_AUTHORIZING, CA_CHALLENGING, CA_CINEMATIC,
    CA_CONNECTED, CA_CONNECTING, CA_DISCONNECTED, CA_LOADING, CA_PRIMED, CA_UNINITIALIZED,
    EXEC_APPEND, EXEC_INSERT, EXEC_NOW,
};
use stdlib::strlen;
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
    _tag_menuframework, menucommon_s, menufield_s, menuframework_s, mfield_t, trap_Cmd_ExecuteText,
    trap_Cvar_VariableStringBuffer, trap_Cvar_VariableValue, trap_GetClientState,
    trap_GetConfigString, uiStatic_t,
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
use ui_public_h::uiClientState_t;
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
// ui_connect.c
//
#[no_mangle]
pub unsafe extern "C" fn UI_DrawConnectScreen(mut overlay: qboolean) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cstate: uiClientState_t = uiClientState_t {
        connState: CA_UNINITIALIZED,
        connectPacketCount: 0,
        clientNum: 0,
        servername: [0; 1024],
        updateInfoString: [0; 1024],
        messageString: [0; 1024],
    };
    let mut info: [libc::c_char; 1024] = [0; 1024];
    Menu_Cache();
    if 0 == overlay as u64 {
        UI_SetColor(color_white.as_mut_ptr());
        UI_DrawHandlePic(
            0i32 as libc::c_float,
            0i32 as libc::c_float,
            640i32 as libc::c_float,
            480i32 as libc::c_float,
            uis.menuBackShader,
        );
    }
    trap_GetClientState(&mut cstate);
    info[0usize] = '\u{0}' as i32 as libc::c_char;
    if 0 != trap_GetConfigString(
        0i32,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    ) {
        UI_DrawProportionalString(
            320i32,
            16i32,
            va(
                b"Loading %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                Info_ValueForKey(
                    info.as_mut_ptr(),
                    b"mapname\x00" as *const u8 as *const libc::c_char,
                ),
            ),
            0x20i32 | 0x1i32 | 0x800i32,
            color_white.as_mut_ptr(),
        );
    }
    UI_DrawProportionalString(
        320i32,
        64i32,
        va(
            b"Connecting to %s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            cstate.servername.as_mut_ptr(),
        ),
        0x1i32 | 0x10i32 | 0x800i32,
        menu_text_color.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        640i32 / 2i32,
        480i32 - 32i32,
        Info_ValueForKey(
            cstate.updateInfoString.as_mut_ptr(),
            b"motd\x00" as *const u8 as *const libc::c_char,
        ),
        0x1i32 | 0x10i32 | 0x800i32,
        menu_text_color.as_mut_ptr(),
    );
    if (cstate.connState as libc::c_uint) < CA_CONNECTED as libc::c_int as libc::c_uint {
        UI_DrawProportionalString_AutoWrapped(
            320i32,
            192i32,
            630i32,
            20i32,
            cstate.messageString.as_mut_ptr(),
            0x1i32 | 0x10i32 | 0x800i32,
            menu_text_color.as_mut_ptr(),
        );
    }
    if lastConnState as libc::c_uint > cstate.connState as libc::c_uint {
        lastLoadingText[0usize] = '\u{0}' as i32 as libc::c_char
    }
    lastConnState = cstate.connState;
    match cstate.connState as libc::c_uint {
        3 => {
            s = va(
                b"Awaiting challenge...%i\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cstate.connectPacketCount,
            )
        }
        4 => {
            s = va(
                b"Awaiting connection...%i\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                cstate.connectPacketCount,
            )
        }
        5 => {
            let mut downloadName: [libc::c_char; 1024] = [0; 1024];
            trap_Cvar_VariableStringBuffer(
                b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
                downloadName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
            );
            if 0 != *downloadName.as_mut_ptr() {
                UI_DisplayDownloadInfo(downloadName.as_mut_ptr());
                return;
            }
            s = b"Awaiting gamestate...\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char
        }
        6 => return,
        7 => return,
        _ => return,
    }
    UI_DrawProportionalString(
        320i32,
        128i32,
        s,
        0x1i32 | 0x10i32 | 0x800i32,
        color_white.as_mut_ptr(),
    );
}
unsafe extern "C" fn UI_DisplayDownloadInfo(mut downloadName: *const libc::c_char) {
    static mut dlText: [libc::c_char; 13] =
        [68, 111, 119, 110, 108, 111, 97, 100, 105, 110, 103, 58, 0];
    static mut etaText: [libc::c_char; 21] = [
        69, 115, 116, 105, 109, 97, 116, 101, 100, 32, 116, 105, 109, 101, 32, 108, 101, 102, 116,
        58, 0,
    ];
    static mut xferText: [libc::c_char; 15] = [
        84, 114, 97, 110, 115, 102, 101, 114, 32, 114, 97, 116, 101, 58, 0,
    ];
    let mut downloadSize: libc::c_int = 0;
    let mut downloadCount: libc::c_int = 0;
    let mut downloadTime: libc::c_int = 0;
    let mut dlSizeBuf: [libc::c_char; 64] = [0; 64];
    let mut totalSizeBuf: [libc::c_char; 64] = [0; 64];
    let mut xferRateBuf: [libc::c_char; 64] = [0; 64];
    let mut dlTimeBuf: [libc::c_char; 64] = [0; 64];
    let mut xferRate: libc::c_int = 0;
    let mut width: libc::c_int = 0;
    let mut leftWidth: libc::c_int = 0;
    let mut style: libc::c_int = 0i32 | 0x10i32 | 0x800i32;
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    downloadSize =
        trap_Cvar_VariableValue(b"cl_downloadSize\x00" as *const u8 as *const libc::c_char)
            as libc::c_int;
    downloadCount =
        trap_Cvar_VariableValue(b"cl_downloadCount\x00" as *const u8 as *const libc::c_char)
            as libc::c_int;
    downloadTime =
        trap_Cvar_VariableValue(b"cl_downloadTime\x00" as *const u8 as *const libc::c_char)
            as libc::c_int;
    leftWidth = (UI_ProportionalStringWidth(dlText.as_mut_ptr()) as libc::c_float
        * UI_ProportionalSizeScale(style)) as libc::c_int;
    width = (UI_ProportionalStringWidth(etaText.as_mut_ptr()) as libc::c_float
        * UI_ProportionalSizeScale(style)) as libc::c_int;
    if width > leftWidth {
        leftWidth = width
    }
    width = (UI_ProportionalStringWidth(xferText.as_mut_ptr()) as libc::c_float
        * UI_ProportionalSizeScale(style)) as libc::c_int;
    if width > leftWidth {
        leftWidth = width
    }
    leftWidth += 16i32;
    UI_DrawProportionalString(
        8i32,
        128i32,
        dlText.as_mut_ptr(),
        style,
        color_white.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        8i32,
        160i32,
        etaText.as_mut_ptr(),
        style,
        color_white.as_mut_ptr(),
    );
    UI_DrawProportionalString(
        8i32,
        224i32,
        xferText.as_mut_ptr(),
        style,
        color_white.as_mut_ptr(),
    );
    if downloadSize > 0i32 {
        s = va(
            b"%s (%d%%)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            downloadName,
            (downloadCount as libc::c_float * 100.0f32 / downloadSize as libc::c_float)
                as libc::c_int,
        )
    } else {
        s = downloadName
    }
    UI_DrawProportionalString(leftWidth, 128i32, s, style, color_white.as_mut_ptr());
    UI_ReadableSize(
        dlSizeBuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        downloadCount,
    );
    UI_ReadableSize(
        totalSizeBuf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        downloadSize,
    );
    if downloadCount < 4096i32 || 0 == downloadTime {
        UI_DrawProportionalString(
            leftWidth,
            160i32,
            b"estimating\x00" as *const u8 as *const libc::c_char,
            style,
            color_white.as_mut_ptr(),
        );
        UI_DrawProportionalString(
            leftWidth,
            192i32,
            va(
                b"(%s of %s copied)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dlSizeBuf.as_mut_ptr(),
                totalSizeBuf.as_mut_ptr(),
            ),
            style,
            color_white.as_mut_ptr(),
        );
    } else {
        if 0 != (uis.realtime - downloadTime) / 1000i32 {
            xferRate = downloadCount / ((uis.realtime - downloadTime) / 1000i32)
        } else {
            xferRate = 0i32
        }
        UI_ReadableSize(
            xferRateBuf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            xferRate,
        );
        if 0 != downloadSize && 0 != xferRate {
            let mut n: libc::c_int = downloadSize / xferRate;
            n = (n - downloadCount / 1024i32 * n / (downloadSize / 1024i32)) * 1000i32;
            UI_PrintTime(
                dlTimeBuf.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                n,
            );
            UI_DrawProportionalString(
                leftWidth,
                160i32,
                dlTimeBuf.as_mut_ptr(),
                style,
                color_white.as_mut_ptr(),
            );
            UI_DrawProportionalString(
                leftWidth,
                192i32,
                va(
                    b"(%s of %s copied)\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    dlSizeBuf.as_mut_ptr(),
                    totalSizeBuf.as_mut_ptr(),
                ),
                style,
                color_white.as_mut_ptr(),
            );
        } else {
            UI_DrawProportionalString(
                leftWidth,
                160i32,
                b"estimating\x00" as *const u8 as *const libc::c_char,
                style,
                color_white.as_mut_ptr(),
            );
            if 0 != downloadSize {
                UI_DrawProportionalString(
                    leftWidth,
                    192i32,
                    va(
                        b"(%s of %s copied)\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        dlSizeBuf.as_mut_ptr(),
                        totalSizeBuf.as_mut_ptr(),
                    ),
                    style,
                    color_white.as_mut_ptr(),
                );
            } else {
                UI_DrawProportionalString(
                    leftWidth,
                    192i32,
                    va(
                        b"(%s copied)\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        dlSizeBuf.as_mut_ptr(),
                    ),
                    style,
                    color_white.as_mut_ptr(),
                );
            }
        }
        if 0 != xferRate {
            UI_DrawProportionalString(
                leftWidth,
                224i32,
                va(
                    b"%s/Sec\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    xferRateBuf.as_mut_ptr(),
                ),
                style,
                color_white.as_mut_ptr(),
            );
        }
    };
}
// Assumes time is in msec
unsafe extern "C" fn UI_PrintTime(
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut time: libc::c_int,
) {
    time /= 1000i32;
    if time > 3600i32 {
        Com_sprintf(
            buf,
            bufsize,
            b"%d hr %d min\x00" as *const u8 as *const libc::c_char,
            time / 3600i32,
            time % 3600i32 / 60i32,
        );
    } else if time > 60i32 {
        Com_sprintf(
            buf,
            bufsize,
            b"%d min %d sec\x00" as *const u8 as *const libc::c_char,
            time / 60i32,
            time % 60i32,
        );
    } else {
        Com_sprintf(
            buf,
            bufsize,
            b"%d sec\x00" as *const u8 as *const libc::c_char,
            time,
        );
    };
}
unsafe extern "C" fn UI_ReadableSize(
    mut buf: *mut libc::c_char,
    mut bufsize: libc::c_int,
    mut value: libc::c_int,
) {
    if value > 1024i32 * 1024i32 * 1024i32 {
        Com_sprintf(
            buf,
            bufsize,
            b"%d\x00" as *const u8 as *const libc::c_char,
            value / (1024i32 * 1024i32 * 1024i32),
        );
        Com_sprintf(
            buf.offset(strlen(buf) as isize),
            (bufsize as libc::c_ulong).wrapping_sub(strlen(buf)) as libc::c_int,
            b".%02d GB\x00" as *const u8 as *const libc::c_char,
            value % (1024i32 * 1024i32 * 1024i32) * 100i32 / (1024i32 * 1024i32 * 1024i32),
        );
    } else if value > 1024i32 * 1024i32 {
        Com_sprintf(
            buf,
            bufsize,
            b"%d\x00" as *const u8 as *const libc::c_char,
            value / (1024i32 * 1024i32),
        );
        Com_sprintf(
            buf.offset(strlen(buf) as isize),
            (bufsize as libc::c_ulong).wrapping_sub(strlen(buf)) as libc::c_int,
            b".%02d MB\x00" as *const u8 as *const libc::c_char,
            value % (1024i32 * 1024i32) * 100i32 / (1024i32 * 1024i32),
        );
    } else if value > 1024i32 {
        Com_sprintf(
            buf,
            bufsize,
            b"%d KB\x00" as *const u8 as *const libc::c_char,
            value / 1024i32,
        );
    } else {
        Com_sprintf(
            buf,
            bufsize,
            b"%d bytes\x00" as *const u8 as *const libc::c_char,
            value,
        );
    };
}
static mut lastConnState: connstate_t = CA_UNINITIALIZED;
static mut lastLoadingText: [libc::c_char; 1024] = [0; 1024];
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
/*
===============================================================================

CONNECTION SCREEN

===============================================================================
*/
#[no_mangle]
pub static mut passwordNeeded: qboolean = qtrue;
#[no_mangle]
pub static mut passwordField: menufield_s = menufield_s {
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
};
// password required / connection rejected information goes here
/*
===================
UI_KeyConnect
===================
*/
#[no_mangle]
pub unsafe extern "C" fn UI_KeyConnect(mut key: libc::c_int) {
    if key == K_ESCAPE as libc::c_int {
        trap_Cmd_ExecuteText(
            EXEC_APPEND as libc::c_int,
            b"disconnect\n\x00" as *const u8 as *const libc::c_char,
        );
        return;
    };
}
