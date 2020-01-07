use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_DrawHandlePic;
pub use crate::src::q3_ui::ui_atoms::UI_DrawProportionalString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_FillRect;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_confirm::UI_ConfirmMenu;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetAwardLevel;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBestScore;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByName;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetCurrentGame;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetNumSPArenas;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetNumSPTiers;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo;
pub use crate::src::q3_ui::ui_gameinfo::UI_NewGame;
pub use crate::src::q3_ui::ui_playersettings::UI_PlayerSettingsMenu;
pub use crate::src::q3_ui::ui_qmenu::color_black;
pub use crate::src::q3_ui::ui_qmenu::color_orange;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::color_yellow;
pub use crate::src::q3_ui::ui_qmenu::Bitmap_Init;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::q3_ui::ui_sppostgame::ui_medalPicNames;
pub use crate::src::q3_ui::ui_sppostgame::ui_medalSounds;
pub use crate::src::q3_ui::ui_spskill::UI_SPSkillMenu;
pub use crate::src::q3_ui::ui_startserver::UI_StartServerMenu;
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
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::Q_strupr;
pub use crate::src::qcommon::q_shared::CHAN_ANNOUNCER;
pub use crate::src::qcommon::q_shared::CHAN_AUTO;
pub use crate::src::qcommon::q_shared::CHAN_BODY;
pub use crate::src::qcommon::q_shared::CHAN_ITEM;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL;
pub use crate::src::qcommon::q_shared::CHAN_LOCAL_SOUND;
pub use crate::src::qcommon::q_shared::CHAN_VOICE;
pub use crate::src::qcommon::q_shared::CHAN_WEAPON;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_Key_SetCatcher;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::src::ui::ui_syscalls::trap_R_SetColor;
pub use crate::src::ui::ui_syscalls::trap_S_RegisterSound;
pub use crate::src::ui::ui_syscalls::trap_S_StartLocalSound;
use crate::stdlib::sin;
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
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;
pub use crate::ui_local_h::AWARD_ACCURACY;
pub use crate::ui_local_h::AWARD_EXCELLENT;
pub use crate::ui_local_h::AWARD_FRAGS;
pub use crate::ui_local_h::AWARD_GAUNTLET;
pub use crate::ui_local_h::AWARD_IMPRESSIVE;
pub use crate::ui_local_h::AWARD_PERFECT;

pub use crate::src::q3_ui::ui_splevel::stdlib_h::atoi;
use crate::stdlib::memset;
use crate::stdlib::strcpy;
use crate::stdlib::strrchr;
pub use crate::stdlib::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct levelMenuInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub item_banner: crate::ui_local_h::menutext_s,
    pub item_leftarrow: crate::ui_local_h::menubitmap_s,
    pub item_maps: [crate::ui_local_h::menubitmap_s; 4],
    pub item_rightarrow: crate::ui_local_h::menubitmap_s,
    pub item_player: crate::ui_local_h::menubitmap_s,
    pub item_awards: [crate::ui_local_h::menubitmap_s; 6],
    pub item_back: crate::ui_local_h::menubitmap_s,
    pub item_reset: crate::ui_local_h::menubitmap_s,
    pub item_custom: crate::ui_local_h::menubitmap_s,
    pub item_next: crate::ui_local_h::menubitmap_s,
    pub item_null: crate::ui_local_h::menubitmap_s,
    pub reinit: crate::src::qcommon::q_shared::qboolean,
    pub selectedArenaInfo: *const i8,
    pub numMaps: i32,
    pub levelPicNames: [[i8; 64]; 4],
    pub levelNames: [[i8; 16]; 4],
    pub levelScores: [i32; 4],
    pub levelScoresSkill: [i32; 4],
    pub levelSelectedPic: crate::src::qcommon::q_shared::qhandle_t,
    pub levelFocusPic: crate::src::qcommon::q_shared::qhandle_t,
    pub levelCompletePic: [crate::src::qcommon::q_shared::qhandle_t; 5],
    pub playerModel: [i8; 64],
    pub playerPicName: [i8; 64],
    pub awardLevels: [i32; 6],
    pub awardSounds: [crate::src::qcommon::q_shared::sfxHandle_t; 6],
    pub numBots: i32,
    pub botPics: [crate::src::qcommon::q_shared::qhandle_t; 7],
    pub botNames: [[i8; 10]; 7],
}

static mut levelMenuInfo: levelMenuInfo_t = levelMenuInfo_t {
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
    item_banner: crate::ui_local_h::menutext_s {
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
    item_leftarrow: crate::ui_local_h::menubitmap_s {
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
    item_maps: [crate::ui_local_h::menubitmap_s {
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
    }; 4],
    item_rightarrow: crate::ui_local_h::menubitmap_s {
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
    item_player: crate::ui_local_h::menubitmap_s {
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
    item_awards: [crate::ui_local_h::menubitmap_s {
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
    }; 6],
    item_back: crate::ui_local_h::menubitmap_s {
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
    item_reset: crate::ui_local_h::menubitmap_s {
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
    item_custom: crate::ui_local_h::menubitmap_s {
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
    item_null: crate::ui_local_h::menubitmap_s {
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
    reinit: crate::src::qcommon::q_shared::qfalse,
    selectedArenaInfo: 0 as *const i8,
    numMaps: 0,
    levelPicNames: [[0; 64]; 4],
    levelNames: [[0; 16]; 4],
    levelScores: [0; 4],
    levelScoresSkill: [0; 4],
    levelSelectedPic: 0,
    levelFocusPic: 0,
    levelCompletePic: [0; 5],
    playerModel: [0; 64],
    playerPicName: [0; 64],
    awardLevels: [0; 6],
    awardSounds: [0; 6],
    numBots: 0,
    botPics: [0; 7],
    botNames: [[0; 10]; 7],
};

static mut selectedArenaSet: i32 = 0;

static mut selectedArena: i32 = 0;

static mut currentSet: i32 = 0;

static mut currentGame: i32 = 0;

static mut trainingTier: i32 = 0;

static mut finalTier: i32 = 0;

static mut minTier: i32 = 0;

static mut maxTier: i32 = 0;
/*
=================
PlayerIcon
=================
*/

unsafe extern "C" fn PlayerIcon(
    mut modelAndSkin: *const i8,
    mut iconName: *mut i8,
    mut iconNameMaxSize: i32,
) {
    let mut skin: *mut i8 = 0 as *mut i8;
    let mut model: [i8; 64] = [0; 64];
    crate::src::qcommon::q_shared::Q_strncpyz(
        model.as_mut_ptr(),
        modelAndSkin,
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    skin = crate::stdlib::strrchr(model.as_mut_ptr(), '/' as i32);
    if !skin.is_null() {
        let fresh0 = skin;
        skin = skin.offset(1);
        *fresh0 = '\u{0}' as i8
    } else {
        skin = b"default\x00" as *const u8 as *mut i8
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        iconName,
        iconNameMaxSize,
        b"models/players/%s/icon_%s.tga\x00" as *const u8 as *const i8,
        model.as_mut_ptr(),
        skin,
    );
    if crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(iconName) == 0
        && crate::src::qcommon::q_shared::Q_stricmp(skin, b"default\x00" as *const u8 as *const i8)
            != 0
    {
        crate::src::qcommon::q_shared::Com_sprintf(
            iconName,
            iconNameMaxSize,
            b"models/players/%s/icon_default.tga\x00" as *const u8 as *const i8,
            model.as_mut_ptr(),
        );
    };
}
/*
=================
PlayerIconhandle
=================
*/

unsafe extern "C" fn PlayerIconHandle(
    mut modelAndSkin: *const i8,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut iconName: [i8; 64] = [0; 64];
    PlayerIcon(
        modelAndSkin,
        iconName.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    return crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(iconName.as_mut_ptr());
}
/*
=================
UI_SPLevelMenu_SetBots
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_SetBots() {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut bot: *mut i8 = 0 as *mut i8;
    let mut botInfo: *mut i8 = 0 as *mut i8;
    let mut bots: [i8; 1024] = [0; 1024];
    levelMenuInfo.numBots = 0;
    if selectedArenaSet > currentSet {
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        bots.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"bots\x00" as *const u8 as *const i8,
        ),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    p = &mut *bots.as_mut_ptr().offset(0) as *mut i8;
    while *p as i32 != 0 && levelMenuInfo.numBots < 7 {
        //skip spaces
        while *p as i32 != 0 && *p as i32 == ' ' as i32 {
            p = p.offset(1)
        }
        if *p == 0 {
            break;
        }
        // mark start of bot name
        bot = p;
        // skip until space of null
        while *p as i32 != 0 && *p as i32 != ' ' as i32 {
            p = p.offset(1)
        }
        if *p != 0 {
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = 0i8
        }
        botInfo = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByName(bot);
        if botInfo.is_null() {
            botInfo = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber(levelMenuInfo.numBots)
        }
        if !botInfo.is_null() {
            levelMenuInfo.botPics[levelMenuInfo.numBots as usize] =
                PlayerIconHandle(crate::src::qcommon::q_shared::Info_ValueForKey(
                    botInfo,
                    b"model\x00" as *const u8 as *const i8,
                ));
            crate::src::qcommon::q_shared::Q_strncpyz(
                levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr(),
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    botInfo,
                    b"name\x00" as *const u8 as *const i8,
                ),
                10i32,
            );
        } else {
            levelMenuInfo.botPics[levelMenuInfo.numBots as usize] = 0;
            crate::src::qcommon::q_shared::Q_strncpyz(
                levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr(),
                bot,
                10i32,
            );
        }
        crate::src::qcommon::q_shared::Q_CleanStr(
            levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr(),
        );
        levelMenuInfo.numBots += 1
    }
}
/*
=================
UI_SPLevelMenu_SetMenuItems
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_SetMenuArena(
    mut n: i32,
    mut level: i32,
    mut arenaInfo: *const i8,
) {
    let mut map: [i8; 64] = [0; 64];
    crate::src::qcommon::q_shared::Q_strncpyz(
        map.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            arenaInfo,
            b"map\x00" as *const u8 as *const i8,
        ),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        levelMenuInfo.levelNames[n as usize].as_mut_ptr(),
        map.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 16]>() as i32,
    );
    crate::src::qcommon::q_shared::Q_strupr(levelMenuInfo.levelNames[n as usize].as_mut_ptr());
    crate::src::q3_ui::ui_gameinfo::UI_GetBestScore(
        level,
        &mut *levelMenuInfo.levelScores.as_mut_ptr().offset(n as isize),
        &mut *levelMenuInfo
            .levelScoresSkill
            .as_mut_ptr()
            .offset(n as isize),
    );
    if levelMenuInfo.levelScores[n as usize] > 8 {
        levelMenuInfo.levelScores[n as usize] = 8
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        levelMenuInfo.levelPicNames[n as usize].as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"levelshots/%s.tga\x00" as *const u8 as *const i8,
        map.as_mut_ptr(),
    );
    if crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        levelMenuInfo.levelPicNames[n as usize].as_mut_ptr(),
    ) == 0
    {
        crate::stdlib::strcpy(
            levelMenuInfo.levelPicNames[n as usize].as_mut_ptr(),
            b"menu/art/unknownmap\x00" as *const u8 as *const i8,
        );
    }
    levelMenuInfo.item_maps[n as usize].shader = 0;
    if selectedArenaSet > currentSet {
        levelMenuInfo.item_maps[n as usize].generic.flags |= 0x2000
    } else {
        levelMenuInfo.item_maps[n as usize].generic.flags &= !(0x2000)
    }
    levelMenuInfo.item_maps[n as usize].generic.flags &= !(0x4000);
}

unsafe extern "C" fn UI_SPLevelMenu_SetMenuItems() {
    let mut n: i32 = 0;
    let mut level: i32 = 0;
    let mut arenaInfo: *const i8 = 0 as *const i8;
    if selectedArenaSet > currentSet {
        selectedArena = -(1)
    } else if selectedArena == -(1) {
        selectedArena = 0
    }
    if selectedArenaSet == trainingTier || selectedArenaSet == finalTier {
        selectedArena = 0
    }
    if selectedArena != -(1) {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const i8,
            (selectedArenaSet * 4i32 + selectedArena) as f32,
        );
    }
    if selectedArenaSet == trainingTier {
        arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo(
            b"training\x00" as *const u8 as *const i8,
        );
        level = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const i8,
        ));
        UI_SPLevelMenu_SetMenuArena(0, level, arenaInfo);
        levelMenuInfo.selectedArenaInfo = arenaInfo;
        levelMenuInfo.item_maps[0].generic.x = 256;
        crate::src::q3_ui::ui_qmenu::Bitmap_Init(
            &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(0),
        );
        levelMenuInfo.item_maps[0].generic.bottom += 32;
        levelMenuInfo.numMaps = 1;
        levelMenuInfo.item_maps[1].generic.flags |= 0x4000;
        levelMenuInfo.item_maps[2].generic.flags |= 0x4000;
        levelMenuInfo.item_maps[3].generic.flags |= 0x4000;
        levelMenuInfo.levelPicNames[1][0] = 0;
        levelMenuInfo.levelPicNames[2][0] = 0;
        levelMenuInfo.levelPicNames[3][0] = 0;
        levelMenuInfo.item_maps[1].shader = 0;
        levelMenuInfo.item_maps[2].shader = 0;
        levelMenuInfo.item_maps[3].shader = 0
    } else if selectedArenaSet == finalTier {
        arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo(
            b"final\x00" as *const u8 as *const i8,
        );
        level = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const i8,
        ));
        UI_SPLevelMenu_SetMenuArena(0, level, arenaInfo);
        levelMenuInfo.selectedArenaInfo = arenaInfo;
        levelMenuInfo.item_maps[0].generic.x = 256;
        crate::src::q3_ui::ui_qmenu::Bitmap_Init(
            &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(0),
        );
        levelMenuInfo.item_maps[0].generic.bottom += 32;
        levelMenuInfo.numMaps = 1;
        levelMenuInfo.item_maps[1].generic.flags |= 0x4000;
        levelMenuInfo.item_maps[2].generic.flags |= 0x4000;
        levelMenuInfo.item_maps[3].generic.flags |= 0x4000;
        levelMenuInfo.levelPicNames[1][0] = 0;
        levelMenuInfo.levelPicNames[2][0] = 0;
        levelMenuInfo.levelPicNames[3][0] = 0;
        levelMenuInfo.item_maps[1].shader = 0;
        levelMenuInfo.item_maps[2].shader = 0;
        levelMenuInfo.item_maps[3].shader = 0
    } else {
        levelMenuInfo.item_maps[0].generic.x = 46;
        crate::src::q3_ui::ui_qmenu::Bitmap_Init(
            &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(0),
        );
        levelMenuInfo.item_maps[0].generic.bottom += 18;
        levelMenuInfo.numMaps = 4;
        n = 0;
        while n < 4 {
            level = selectedArenaSet * 4 + n;
            arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(level);
            UI_SPLevelMenu_SetMenuArena(n, level, arenaInfo);
            n += 1
        }
        if selectedArena != -(1) {
            levelMenuInfo.selectedArenaInfo =
                crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(
                    selectedArenaSet * 4 + selectedArena,
                )
        }
    }
    // enable/disable arrows when they are valid/invalid
    if selectedArenaSet == minTier {
        levelMenuInfo.item_leftarrow.generic.flags |= 0x4000 | 0x1000
    } else {
        levelMenuInfo.item_leftarrow.generic.flags &= !(0x4000 | 0x1000)
    }
    if selectedArenaSet == maxTier {
        levelMenuInfo.item_rightarrow.generic.flags |= 0x4000 | 0x1000
    } else {
        levelMenuInfo.item_rightarrow.generic.flags &= !(0x4000 | 0x1000)
    }
    UI_SPLevelMenu_SetBots();
}
/*
=================
UI_SPLevelMenu_ResetEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_ResetDraw() {
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        640 / 2,
        356 + 27 * 0,
        b"WARNING: This resets all of the\x00" as *const u8 as *const i8,
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        640 / 2,
        356 + 27 * 1,
        b"single player game variables.\x00" as *const u8 as *const i8,
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        640 / 2,
        356 + 27 * 2,
        b"Do this only if you want to\x00" as *const u8 as *const i8,
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        640 / 2,
        356 + 27 * 3,
        b"start over from the beginning.\x00" as *const u8 as *const i8,
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
    );
}

unsafe extern "C" fn UI_SPLevelMenu_ResetAction(
    mut result: crate::src::qcommon::q_shared::qboolean,
) {
    if result as u64 == 0 {
        return;
    }
    // clear game variables
    crate::src::q3_ui::ui_gameinfo::UI_NewGame();
    if !crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo(
        b"training\x00" as *const u8 as *const i8,
    )
    .is_null()
    {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const i8,
            -4f32,
        );
    } else {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const i8,
            0f32,
        );
    }
    // make the level select menu re-initialize
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
    UI_SPLevelMenu();
}

unsafe extern "C" fn UI_SPLevelMenu_ResetEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    crate::src::q3_ui::ui_confirm::UI_ConfirmMenu(
        b"RESET GAME?\x00" as *const u8 as *const i8,
        Some(UI_SPLevelMenu_ResetDraw as unsafe extern "C" fn() -> ()),
        Some(
            UI_SPLevelMenu_ResetAction
                as unsafe extern "C" fn(_: crate::src::qcommon::q_shared::qboolean) -> (),
        ),
    );
}
/*
=================
UI_SPLevelMenu_LevelEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_LevelEvent(mut ptr: *mut libc::c_void, mut notification: i32) {
    if notification != 3 {
        return;
    }
    if selectedArenaSet == trainingTier || selectedArenaSet == finalTier {
        return;
    }
    selectedArena = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id - 11;
    levelMenuInfo.selectedArenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(
        selectedArenaSet * 4 + selectedArena,
    );
    UI_SPLevelMenu_SetBots();
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"ui_spSelection\x00" as *const u8 as *const i8,
        (selectedArenaSet * 4 + selectedArena) as f32,
    );
}
/*
=================
UI_SPLevelMenu_LeftArrowEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_LeftArrowEvent(
    mut ptr: *mut libc::c_void,
    mut notification: i32,
) {
    if notification != 3 {
        return;
    }
    if selectedArenaSet == minTier {
        return;
    }
    selectedArenaSet -= 1;
    UI_SPLevelMenu_SetMenuItems();
}
/*
=================
UI_SPLevelMenu_RightArrowEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_RightArrowEvent(
    mut ptr: *mut libc::c_void,
    mut notification: i32,
) {
    if notification != 3 {
        return;
    }
    if selectedArenaSet == maxTier {
        return;
    }
    selectedArenaSet += 1;
    UI_SPLevelMenu_SetMenuItems();
}
/*
=================
UI_SPLevelMenu_PlayerEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_PlayerEvent(mut ptr: *mut libc::c_void, mut notification: i32) {
    if notification != 3 {
        return;
    }
    crate::src::q3_ui::ui_playersettings::UI_PlayerSettingsMenu();
}
/*
=================
UI_SPLevelMenu_AwardEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_AwardEvent(mut ptr: *mut libc::c_void, mut notification: i32) {
    let mut n: i32 = 0;
    if notification != 3 {
        return;
    }
    n = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id - 17;
    crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
        levelMenuInfo.awardSounds[n as usize],
        crate::src::qcommon::q_shared::CHAN_ANNOUNCER as i32,
    );
}
/*
=================
UI_SPLevelMenu_NextEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_NextEvent(mut ptr: *mut libc::c_void, mut notification: i32) {
    if notification != 3 {
        return;
    }
    if selectedArenaSet > currentSet {
        return;
    }
    if selectedArena == -(1) {
        selectedArena = 0
    }
    crate::src::q3_ui::ui_spskill::UI_SPSkillMenu(levelMenuInfo.selectedArenaInfo);
}
/*
=================
UI_SPLevelMenu_BackEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_BackEvent(mut ptr: *mut libc::c_void, mut notification: i32) {
    if notification != 3 {
        return;
    }
    if selectedArena == -(1) {
        selectedArena = 0
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
}
/*
=================
UI_SPLevelMenu_CustomEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_CustomEvent(mut ptr: *mut libc::c_void, mut notification: i32) {
    if notification != 3 {
        return;
    }
    crate::src::q3_ui::ui_startserver::UI_StartServerMenu(crate::src::qcommon::q_shared::qfalse);
}
/*
=================
UI_SPLevelMenu_MenuDraw
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_MenuDraw() {
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut level: i32 = 0;
    //	int				fraglimit;
    let mut pad: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    let mut string: [i8; 64] = [0; 64];
    if levelMenuInfo.reinit as u64 != 0 {
        crate::src::q3_ui::ui_atoms::UI_PopMenu();
        UI_SPLevelMenu();
        return;
    }
    // draw player name
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"name\x00" as *const u8 as *const i8,
        string.as_mut_ptr(),
        32,
    );
    crate::src::qcommon::q_shared::Q_CleanStr(string.as_mut_ptr());
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        320,
        314,
        string.as_mut_ptr(),
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
    );
    // check for model changes
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(
        buf.as_mut_ptr(),
        levelMenuInfo.playerModel.as_mut_ptr(),
    ) != 0
    {
        crate::src::qcommon::q_shared::Q_strncpyz(
            levelMenuInfo.playerModel.as_mut_ptr(),
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
        );
        PlayerIcon(
            levelMenuInfo.playerModel.as_mut_ptr(),
            levelMenuInfo.playerPicName.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
        );
        levelMenuInfo.item_player.shader = 0
    }
    // standard menu drawing
    crate::src::q3_ui::ui_qmenu::Menu_Draw(&mut levelMenuInfo.menu);
    // draw player award levels
    y = 314 + 26;
    i = 0;
    n = 0;
    while n < 6 {
        level = levelMenuInfo.awardLevels[n as usize];
        if level > 0 {
            if i & 1 != 0 {
                x = 224 - (i - 1) / 2 * (48 + 16)
            } else {
                x = 368 + i / 2 * (48 + 16)
            }
            i += 1;
            if !(level == 1) {
                if level >= 1000000 {
                    crate::src::qcommon::q_shared::Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 64]>() as i32,
                        b"%im\x00" as *const u8 as *const i8,
                        level / 1000000i32,
                    );
                } else if level >= 1000 {
                    crate::src::qcommon::q_shared::Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 64]>() as i32,
                        b"%ik\x00" as *const u8 as *const i8,
                        level / 1000i32,
                    );
                } else {
                    crate::src::qcommon::q_shared::Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 64]>() as i32,
                        b"%i\x00" as *const u8 as *const i8,
                        level,
                    );
                }
                crate::src::q3_ui::ui_atoms::UI_DrawString(
                    x + 24i32,
                    y + 48i32,
                    string.as_mut_ptr(),
                    0x1i32,
                    crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
                );
            }
        }
        n += 1
    }
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        18,
        38,
        crate::src::qcommon::q_shared::va(
            b"Tier %i\x00" as *const u8 as *mut i8,
            selectedArenaSet + 1i32,
        ),
        0 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
    );
    n = 0;
    while n < levelMenuInfo.numMaps {
        x = levelMenuInfo.item_maps[n as usize].generic.x;
        y = levelMenuInfo.item_maps[n as usize].generic.y;
        crate::src::q3_ui::ui_atoms::UI_FillRect(
            x as f32,
            (y + 96) as f32,
            128f32,
            18f32,
            crate::src::q3_ui::ui_qmenu::color_black.as_mut_ptr(),
        );
        n += 1
    }
    if selectedArenaSet > currentSet {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            320,
            216,
            b"ACCESS DENIED\x00" as *const u8 as *const i8,
            0x1 | 0x20,
            crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
        );
        return;
    }
    // show levelshots for levels of current tier
    color[0] = crate::src::q3_ui::ui_qmenu::color_white[0];
    color[1] = crate::src::q3_ui::ui_qmenu::color_white[1];
    color[2] = crate::src::q3_ui::ui_qmenu::color_white[2];
    color[3] = crate::src::q3_ui::ui_qmenu::color_white[3];
    color[3] = (0.5
        + 0.5 * crate::stdlib::sin((crate::src::q3_ui::ui_atoms::uis.realtime / 75) as f64))
        as crate::src::qcommon::q_shared::vec_t;
    n = 0;
    while n < levelMenuInfo.numMaps {
        x = levelMenuInfo.item_maps[n as usize].generic.x;
        y = levelMenuInfo.item_maps[n as usize].generic.y;
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            x + 64,
            y + 96,
            levelMenuInfo.levelNames[n as usize].as_mut_ptr(),
            0x1 | 0x10,
            crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
        );
        if levelMenuInfo.levelScores[n as usize] == 1 {
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
                x as f32,
                y as f32,
                128f32,
                96f32,
                levelMenuInfo.levelCompletePic
                    [(levelMenuInfo.levelScoresSkill[n as usize] - 1i32) as usize],
            );
        }
        if n == selectedArena {
            if crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor(&mut levelMenuInfo.menu)
                == &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(n as isize)
                    as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void
            {
                crate::src::ui::ui_syscalls::trap_R_SetColor(color.as_mut_ptr());
            }
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
                (x - 1) as f32,
                (y - 1) as f32,
                130f32,
                (130i32 - 14) as f32,
                levelMenuInfo.levelSelectedPic,
            );
            crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const f32);
        } else if crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor(&mut levelMenuInfo.menu)
            == &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(n as isize)
                as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void
        {
            crate::src::ui::ui_syscalls::trap_R_SetColor(color.as_mut_ptr());
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
                (x - 31) as f32,
                (y - 30) as f32,
                256f32,
                (256i32 - 27) as f32,
                levelMenuInfo.levelFocusPic,
            );
            crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const f32);
        }
        n += 1
    }
    // show map name and long name of selected level
    y = 192;
    crate::src::qcommon::q_shared::Q_strncpyz(
        buf.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"map\x00" as *const u8 as *const i8,
        ),
        20,
    );
    crate::src::qcommon::q_shared::Q_strupr(buf.as_mut_ptr());
    crate::src::qcommon::q_shared::Com_sprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
        b"%s: %s\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"longname\x00" as *const u8 as *const i8,
        ),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        320,
        y,
        string.as_mut_ptr(),
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
    );
    //	fraglimit = atoi( Info_ValueForKey( levelMenuInfo.selectedArenaInfo, "fraglimit" ) );
    //	UI_DrawString( 18, 212, va("Frags %i", fraglimit) , UI_LEFT|UI_SMALLFONT, color_orange );
    // draw bot opponents
    y += 24;
    pad = (7 - levelMenuInfo.numBots) * (64 + 26) / 2;
    n = 0;
    while n < levelMenuInfo.numBots {
        x = 18 + pad + (64 + 26) * n;
        if levelMenuInfo.botPics[n as usize] != 0 {
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
                x as f32,
                y as f32,
                64f32,
                64f32,
                levelMenuInfo.botPics[n as usize],
            );
        } else {
            crate::src::q3_ui::ui_atoms::UI_FillRect(
                x as f32,
                y as f32,
                64f32,
                64f32,
                crate::src::q3_ui::ui_qmenu::color_black.as_mut_ptr(),
            );
            crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
                x + 22i32,
                y + 18i32,
                b"?\x00" as *const u8 as *const i8,
                0x20i32,
                crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
            );
        }
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            x,
            y + 64,
            levelMenuInfo.botNames[n as usize].as_mut_ptr(),
            0x10 | 0,
            crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
        );
        n += 1
    }
}
/*
=================
UI_SPLevelMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPLevelMenu_Cache() {
    let mut n: i32 = 0;
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/maps_select\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/maps_selected\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/narrow_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/narrow_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/unknownmap\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete2\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete3\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete4\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete5\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fight_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fight_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/reset_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/reset_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/skirmish_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/skirmish_1\x00" as *const u8 as *const i8,
    );
    n = 0;
    while n < 6 {
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            *crate::src::q3_ui::ui_sppostgame::ui_medalPicNames
                .as_mut_ptr()
                .offset(n as isize),
        );
        levelMenuInfo.awardSounds[n as usize] = crate::src::ui::ui_syscalls::trap_S_RegisterSound(
            *crate::src::q3_ui::ui_sppostgame::ui_medalSounds
                .as_mut_ptr()
                .offset(n as isize),
            crate::src::qcommon::q_shared::qfalse,
        );
        n += 1
    }
    levelMenuInfo.levelSelectedPic = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/maps_selected\x00" as *const u8 as *const i8,
    );
    levelMenuInfo.levelFocusPic = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/maps_select\x00" as *const u8 as *const i8,
    );
    levelMenuInfo.levelCompletePic[0] = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete1\x00" as *const u8 as *const i8,
    );
    levelMenuInfo.levelCompletePic[1] = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete2\x00" as *const u8 as *const i8,
    );
    levelMenuInfo.levelCompletePic[2] = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete3\x00" as *const u8 as *const i8,
    );
    levelMenuInfo.levelCompletePic[3] = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete4\x00" as *const u8 as *const i8,
    );
    levelMenuInfo.levelCompletePic[4] = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete5\x00" as *const u8 as *const i8,
    );
}
/*
=================
UI_SPLevelMenu_Init
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_Init() {
    let mut skill: i32 = 0;
    let mut n: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut count: i32 = 0;
    let mut buf: [i8; 64] = [0; 64];
    skill = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"g_spSkill\x00" as *const u8 as *const i8,
    ) as i32;
    if skill < 1 || skill > 5 {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            b"g_spSkill\x00" as *const u8 as *const i8,
            b"2\x00" as *const u8 as *const i8,
        );
    }
    crate::stdlib::memset(
        &mut levelMenuInfo as *mut levelMenuInfo_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<levelMenuInfo_t>(),
    );
    levelMenuInfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    levelMenuInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    levelMenuInfo.menu.draw = Some(UI_SPLevelMenu_MenuDraw as unsafe extern "C" fn() -> ());
    UI_SPLevelMenu_Cache();
    levelMenuInfo.item_banner.generic.type_0 = 10;
    levelMenuInfo.item_banner.generic.x = 320;
    levelMenuInfo.item_banner.generic.y = 16;
    levelMenuInfo.item_banner.string = b"CHOOSE LEVEL\x00" as *const u8 as *mut i8;
    levelMenuInfo.item_banner.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    levelMenuInfo.item_banner.style = 0x1;
    levelMenuInfo.item_leftarrow.generic.type_0 = 6;
    levelMenuInfo.item_leftarrow.generic.name = b"menu/art/narrow_0\x00" as *const u8 as *const i8;
    levelMenuInfo.item_leftarrow.generic.flags = 0x4 | 0x100;
    levelMenuInfo.item_leftarrow.generic.x = 18;
    levelMenuInfo.item_leftarrow.generic.y = 64;
    levelMenuInfo.item_leftarrow.generic.callback = Some(
        UI_SPLevelMenu_LeftArrowEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    levelMenuInfo.item_leftarrow.generic.id = 10;
    levelMenuInfo.item_leftarrow.width = 16;
    levelMenuInfo.item_leftarrow.height = 114;
    levelMenuInfo.item_leftarrow.focuspic = b"menu/art/narrow_1\x00" as *const u8 as *mut i8;
    levelMenuInfo.item_maps[0].generic.type_0 = 6;
    levelMenuInfo.item_maps[0].generic.name = levelMenuInfo.levelPicNames[0].as_mut_ptr();
    levelMenuInfo.item_maps[0].generic.flags = 0x4;
    levelMenuInfo.item_maps[0].generic.x = 46;
    levelMenuInfo.item_maps[0].generic.y = 64;
    levelMenuInfo.item_maps[0].generic.id = 11;
    levelMenuInfo.item_maps[0].generic.callback =
        Some(UI_SPLevelMenu_LevelEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_maps[0].width = 128;
    levelMenuInfo.item_maps[0].height = 96;
    levelMenuInfo.item_maps[1].generic.type_0 = 6;
    levelMenuInfo.item_maps[1].generic.name = levelMenuInfo.levelPicNames[1].as_mut_ptr();
    levelMenuInfo.item_maps[1].generic.flags = 0x4;
    levelMenuInfo.item_maps[1].generic.x = 186;
    levelMenuInfo.item_maps[1].generic.y = 64;
    levelMenuInfo.item_maps[1].generic.id = 12;
    levelMenuInfo.item_maps[1].generic.callback =
        Some(UI_SPLevelMenu_LevelEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_maps[1].width = 128;
    levelMenuInfo.item_maps[1].height = 96;
    levelMenuInfo.item_maps[2].generic.type_0 = 6;
    levelMenuInfo.item_maps[2].generic.name = levelMenuInfo.levelPicNames[2].as_mut_ptr();
    levelMenuInfo.item_maps[2].generic.flags = 0x4;
    levelMenuInfo.item_maps[2].generic.x = 326;
    levelMenuInfo.item_maps[2].generic.y = 64;
    levelMenuInfo.item_maps[2].generic.id = 13;
    levelMenuInfo.item_maps[2].generic.callback =
        Some(UI_SPLevelMenu_LevelEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_maps[2].width = 128;
    levelMenuInfo.item_maps[2].height = 96;
    levelMenuInfo.item_maps[3].generic.type_0 = 6;
    levelMenuInfo.item_maps[3].generic.name = levelMenuInfo.levelPicNames[3].as_mut_ptr();
    levelMenuInfo.item_maps[3].generic.flags = 0x4;
    levelMenuInfo.item_maps[3].generic.x = 466;
    levelMenuInfo.item_maps[3].generic.y = 64;
    levelMenuInfo.item_maps[3].generic.id = 14;
    levelMenuInfo.item_maps[3].generic.callback =
        Some(UI_SPLevelMenu_LevelEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_maps[3].width = 128;
    levelMenuInfo.item_maps[3].height = 96;
    levelMenuInfo.item_rightarrow.generic.type_0 = 6;
    levelMenuInfo.item_rightarrow.generic.name = b"menu/art/narrow_0\x00" as *const u8 as *const i8;
    levelMenuInfo.item_rightarrow.generic.flags = 0x4 | 0x100;
    levelMenuInfo.item_rightarrow.generic.x = 606;
    levelMenuInfo.item_rightarrow.generic.y = 64;
    levelMenuInfo.item_rightarrow.generic.callback = Some(
        UI_SPLevelMenu_RightArrowEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    levelMenuInfo.item_rightarrow.generic.id = 15;
    levelMenuInfo.item_rightarrow.width = -(16);
    levelMenuInfo.item_rightarrow.height = 114;
    levelMenuInfo.item_rightarrow.focuspic = b"menu/art/narrow_1\x00" as *const u8 as *mut i8;
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const i8,
        levelMenuInfo.playerModel.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    PlayerIcon(
        levelMenuInfo.playerModel.as_mut_ptr(),
        levelMenuInfo.playerPicName.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    levelMenuInfo.item_player.generic.type_0 = 6;
    levelMenuInfo.item_player.generic.name = levelMenuInfo.playerPicName.as_mut_ptr();
    levelMenuInfo.item_player.generic.flags = 0x4 | 0x800;
    levelMenuInfo.item_player.generic.x = 288;
    levelMenuInfo.item_player.generic.y = 314 + 26;
    levelMenuInfo.item_player.generic.id = 16;
    levelMenuInfo.item_player.generic.callback = Some(
        UI_SPLevelMenu_PlayerEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    levelMenuInfo.item_player.width = 64;
    levelMenuInfo.item_player.height = 64;
    n = 0;
    while n < 6 {
        levelMenuInfo.awardLevels[n as usize] = crate::src::q3_ui::ui_gameinfo::UI_GetAwardLevel(n);
        n += 1
    }
    levelMenuInfo.awardLevels[crate::ui_local_h::AWARD_FRAGS as usize] =
        100 * (levelMenuInfo.awardLevels[crate::ui_local_h::AWARD_FRAGS as usize] / 100);
    y = 314 + 26;
    count = 0;
    n = 0;
    while n < 6 {
        if levelMenuInfo.awardLevels[n as usize] != 0 {
            if count & 1 != 0 {
                x = 224 - (count - 1) / 2 * (48 + 16)
            } else {
                x = 368 + count / 2 * (48 + 16)
            }
            levelMenuInfo.item_awards[count as usize].generic.type_0 = 6;
            levelMenuInfo.item_awards[count as usize].generic.name =
                *crate::src::q3_ui::ui_sppostgame::ui_medalPicNames
                    .as_mut_ptr()
                    .offset(n as isize);
            levelMenuInfo.item_awards[count as usize].generic.flags = 0x4 | 0x100000 | 0x800;
            levelMenuInfo.item_awards[count as usize].generic.x = x;
            levelMenuInfo.item_awards[count as usize].generic.y = y;
            levelMenuInfo.item_awards[count as usize].generic.id = 17 + n;
            levelMenuInfo.item_awards[count as usize].generic.callback = Some(
                UI_SPLevelMenu_AwardEvent
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
            );
            levelMenuInfo.item_awards[count as usize].width = 48;
            levelMenuInfo.item_awards[count as usize].height = 48;
            count += 1
        }
        n += 1
    }
    levelMenuInfo.item_back.generic.type_0 = 6;
    levelMenuInfo.item_back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    levelMenuInfo.item_back.generic.flags = 0x4 | 0x100;
    levelMenuInfo.item_back.generic.x = 0;
    levelMenuInfo.item_back.generic.y = 480 - 64;
    levelMenuInfo.item_back.generic.callback =
        Some(UI_SPLevelMenu_BackEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_back.generic.id = 23;
    levelMenuInfo.item_back.width = 128;
    levelMenuInfo.item_back.height = 64;
    levelMenuInfo.item_back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    levelMenuInfo.item_reset.generic.type_0 = 6;
    levelMenuInfo.item_reset.generic.name = b"menu/art/reset_0\x00" as *const u8 as *const i8;
    levelMenuInfo.item_reset.generic.flags = 0x4 | 0x100;
    levelMenuInfo.item_reset.generic.x = 170;
    levelMenuInfo.item_reset.generic.y = 480 - 64;
    levelMenuInfo.item_reset.generic.callback =
        Some(UI_SPLevelMenu_ResetEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_reset.generic.id = 24;
    levelMenuInfo.item_reset.width = 128;
    levelMenuInfo.item_reset.height = 64;
    levelMenuInfo.item_reset.focuspic = b"menu/art/reset_1\x00" as *const u8 as *mut i8;
    levelMenuInfo.item_custom.generic.type_0 = 6;
    levelMenuInfo.item_custom.generic.name = b"menu/art/skirmish_0\x00" as *const u8 as *const i8;
    levelMenuInfo.item_custom.generic.flags = 0x4 | 0x100;
    levelMenuInfo.item_custom.generic.x = 342;
    levelMenuInfo.item_custom.generic.y = 480 - 64;
    levelMenuInfo.item_custom.generic.callback = Some(
        UI_SPLevelMenu_CustomEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    levelMenuInfo.item_custom.generic.id = 25;
    levelMenuInfo.item_custom.width = 128;
    levelMenuInfo.item_custom.height = 64;
    levelMenuInfo.item_custom.focuspic = b"menu/art/skirmish_1\x00" as *const u8 as *mut i8;
    levelMenuInfo.item_next.generic.type_0 = 6;
    levelMenuInfo.item_next.generic.name = b"menu/art/fight_0\x00" as *const u8 as *const i8;
    levelMenuInfo.item_next.generic.flags = 0x10 | 0x100;
    levelMenuInfo.item_next.generic.x = 640;
    levelMenuInfo.item_next.generic.y = 480 - 64;
    levelMenuInfo.item_next.generic.callback =
        Some(UI_SPLevelMenu_NextEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    levelMenuInfo.item_next.generic.id = 26;
    levelMenuInfo.item_next.width = 128;
    levelMenuInfo.item_next.height = 64;
    levelMenuInfo.item_next.focuspic = b"menu/art/fight_1\x00" as *const u8 as *mut i8;
    levelMenuInfo.item_null.generic.type_0 = 6;
    levelMenuInfo.item_null.generic.flags = 0x4 | 0x800 | 0x100000;
    levelMenuInfo.item_null.generic.x = 0;
    levelMenuInfo.item_null.generic.y = 0;
    levelMenuInfo.item_null.width = 640;
    levelMenuInfo.item_null.height = 480;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_leftarrow as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(0) as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(1) as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(2) as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(3) as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    levelMenuInfo.item_maps[0].generic.bottom += 18;
    levelMenuInfo.item_maps[1].generic.bottom += 18;
    levelMenuInfo.item_maps[2].generic.bottom += 18;
    levelMenuInfo.item_maps[3].generic.bottom += 18;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_rightarrow as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_player as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    n = 0;
    while n < count {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut levelMenuInfo.menu,
            &mut *levelMenuInfo.item_awards.as_mut_ptr().offset(n as isize)
                as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
        );
        n += 1
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_reset as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_custom as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_next as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_null as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"ui_spSelection\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    if *buf.as_mut_ptr() != 0 {
        n = atoi(buf.as_mut_ptr());
        selectedArenaSet = n / 4;
        selectedArena = n % 4
    } else {
        selectedArenaSet = currentSet;
        selectedArena = currentGame
    }
    UI_SPLevelMenu_SetMenuItems();
}
/*
=================
UI_SPLevelMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPLevelMenu() {
    let mut level: i32 = 0;
    let mut trainingLevel: i32 = 0;
    let mut arenaInfo: *const i8 = 0 as *const i8;
    trainingTier = -(1);
    arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo(
        b"training\x00" as *const u8 as *const i8,
    );
    if !arenaInfo.is_null() {
        minTier = trainingTier;
        trainingLevel = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const i8,
        ))
    } else {
        minTier = 0;
        trainingLevel = -(2)
    }
    finalTier = crate::src::q3_ui::ui_gameinfo::UI_GetNumSPTiers();
    arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo(
        b"final\x00" as *const u8 as *const i8,
    );
    if !arenaInfo.is_null() {
        maxTier = finalTier
    } else {
        maxTier = finalTier - 1;
        if maxTier < minTier {
            maxTier = minTier
        }
    }
    level = crate::src::q3_ui::ui_gameinfo::UI_GetCurrentGame();
    if level == -(1) {
        level = crate::src::q3_ui::ui_gameinfo::UI_GetNumSPArenas() - 1;
        if maxTier == finalTier {
            level += 1
        }
    }
    if level == trainingLevel {
        currentSet = -(1);
        currentGame = 0
    } else {
        currentSet = level / 4;
        currentGame = level % 4
    }
    UI_SPLevelMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut levelMenuInfo.menu);
    crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
        &mut levelMenuInfo.menu,
        &mut levelMenuInfo.item_next as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_SPLevelMenu_f
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPLevelMenu_f() {
    crate::src::ui::ui_syscalls::trap_Key_SetCatcher(0x2);
    crate::src::q3_ui::ui_atoms::uis.menusp = 0;
    UI_SPLevelMenu();
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
/*
=================
UI_SPLevelMenu_ReInit
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPLevelMenu_ReInit() {
    levelMenuInfo.reinit = crate::src::qcommon::q_shared::qtrue;
}
