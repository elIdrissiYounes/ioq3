use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
        return ::libc::strtol(
            __nptr,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
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
use ::libc::strcpy;
use ::libc::strrchr;
pub use ::libc::strtol;

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
    pub selectedArenaInfo: *const libc::c_char,
    pub numMaps: libc::c_int,
    pub levelPicNames: [[libc::c_char; 64]; 4],
    pub levelNames: [[libc::c_char; 16]; 4],
    pub levelScores: [libc::c_int; 4],
    pub levelScoresSkill: [libc::c_int; 4],
    pub levelSelectedPic: crate::src::qcommon::q_shared::qhandle_t,
    pub levelFocusPic: crate::src::qcommon::q_shared::qhandle_t,
    pub levelCompletePic: [crate::src::qcommon::q_shared::qhandle_t; 5],
    pub playerModel: [libc::c_char; 64],
    pub playerPicName: [libc::c_char; 64],
    pub awardLevels: [libc::c_int; 6],
    pub awardSounds: [crate::src::qcommon::q_shared::sfxHandle_t; 6],
    pub numBots: libc::c_int,
    pub botPics: [crate::src::qcommon::q_shared::qhandle_t; 7],
    pub botNames: [[libc::c_char; 10]; 7],
}

static mut levelMenuInfo: levelMenuInfo_t = levelMenuInfo_t {
    menu: crate::ui_local_h::menuframework_s {
        cursor: 0,
        cursor_prev: 0,
        nitems: 0,
        items: [0 as *const libc::c_void as *mut libc::c_void; 64],
        draw: None,
        key: None,
        wrapAround: crate::src::qcommon::q_shared::qfalse,
        fullscreen: crate::src::qcommon::q_shared::qfalse,
        showlogo: crate::src::qcommon::q_shared::qfalse,
    },
    item_banner: crate::ui_local_h::menutext_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    item_leftarrow: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    item_maps: [crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    }; 4],
    item_rightarrow: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    item_player: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    item_awards: [crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    }; 6],
    item_back: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    item_reset: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    item_custom: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    item_next: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    item_null: crate::ui_local_h::menubitmap_s {
        generic: crate::ui_local_h::menucommon_s {
            type_0: 0,
            name: 0 as *const libc::c_char,
            id: 0,
            x: 0,
            y: 0,
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            parent: 0 as *const crate::ui_local_h::menuframework_s
                as *mut crate::ui_local_h::menuframework_s,
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
    reinit: crate::src::qcommon::q_shared::qfalse,
    selectedArenaInfo: 0 as *const libc::c_char,
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

static mut selectedArenaSet: libc::c_int = 0;

static mut selectedArena: libc::c_int = 0;

static mut currentSet: libc::c_int = 0;

static mut currentGame: libc::c_int = 0;

static mut trainingTier: libc::c_int = 0;

static mut finalTier: libc::c_int = 0;

static mut minTier: libc::c_int = 0;

static mut maxTier: libc::c_int = 0;
/*
=================
PlayerIcon
=================
*/

unsafe extern "C" fn PlayerIcon(
    mut modelAndSkin: *const libc::c_char,
    mut iconName: *mut libc::c_char,
    mut iconNameMaxSize: libc::c_int,
) {
    let mut skin: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut model: [libc::c_char; 64] = [0; 64];
    crate::src::qcommon::q_shared::Q_strncpyz(
        model.as_mut_ptr(),
        modelAndSkin,
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    skin = ::libc::strrchr(model.as_mut_ptr(), '/' as i32);
    if !skin.is_null() {
        let fresh0 = skin;
        skin = skin.offset(1);
        *fresh0 = '\u{0}' as i32 as libc::c_char
    } else {
        skin = b"default\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        iconName,
        iconNameMaxSize,
        b"models/players/%s/icon_%s.tga\x00" as *const u8 as *const libc::c_char,
        model.as_mut_ptr(),
        skin,
    );
    if crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(iconName) == 0
        && crate::src::qcommon::q_shared::Q_stricmp(
            skin,
            b"default\x00" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        crate::src::qcommon::q_shared::Com_sprintf(
            iconName,
            iconNameMaxSize,
            b"models/players/%s/icon_default.tga\x00" as *const u8 as *const libc::c_char,
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
    mut modelAndSkin: *const libc::c_char,
) -> crate::src::qcommon::q_shared::qhandle_t {
    let mut iconName: [libc::c_char; 64] = [0; 64];
    PlayerIcon(
        modelAndSkin,
        iconName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    return crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(iconName.as_mut_ptr());
}
/*
=================
UI_SPLevelMenu_SetBots
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_SetBots() {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut botInfo: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bots: [libc::c_char; 1024] = [0; 1024];
    levelMenuInfo.numBots = 0 as libc::c_int;
    if selectedArenaSet > currentSet {
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        bots.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"bots\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    p = &mut *bots.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_char;
    while *p as libc::c_int != 0 && levelMenuInfo.numBots < 7 as libc::c_int {
        //skip spaces
        while *p as libc::c_int != 0 && *p as libc::c_int == ' ' as i32 {
            p = p.offset(1)
        }
        if *p == 0 {
            break;
        }
        // mark start of bot name
        bot = p;
        // skip until space of null
        while *p as libc::c_int != 0 && *p as libc::c_int != ' ' as i32 {
            p = p.offset(1)
        }
        if *p != 0 {
            let fresh1 = p;
            p = p.offset(1);
            *fresh1 = 0 as libc::c_int as libc::c_char
        }
        botInfo = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByName(bot);
        if botInfo.is_null() {
            botInfo = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber(levelMenuInfo.numBots)
        }
        if !botInfo.is_null() {
            levelMenuInfo.botPics[levelMenuInfo.numBots as usize] =
                PlayerIconHandle(crate::src::qcommon::q_shared::Info_ValueForKey(
                    botInfo,
                    b"model\x00" as *const u8 as *const libc::c_char,
                ));
            crate::src::qcommon::q_shared::Q_strncpyz(
                levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr(),
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    botInfo,
                    b"name\x00" as *const u8 as *const libc::c_char,
                ),
                10 as libc::c_int,
            );
        } else {
            levelMenuInfo.botPics[levelMenuInfo.numBots as usize] = 0 as libc::c_int;
            crate::src::qcommon::q_shared::Q_strncpyz(
                levelMenuInfo.botNames[levelMenuInfo.numBots as usize].as_mut_ptr(),
                bot,
                10 as libc::c_int,
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
    mut n: libc::c_int,
    mut level: libc::c_int,
    mut arenaInfo: *const libc::c_char,
) {
    let mut map: [libc::c_char; 64] = [0; 64];
    crate::src::qcommon::q_shared::Q_strncpyz(
        map.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            arenaInfo,
            b"map\x00" as *const u8 as *const libc::c_char,
        ),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        levelMenuInfo.levelNames[n as usize].as_mut_ptr(),
        map.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as libc::c_int,
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
    if levelMenuInfo.levelScores[n as usize] > 8 as libc::c_int {
        levelMenuInfo.levelScores[n as usize] = 8 as libc::c_int
    }
    crate::src::qcommon::q_shared::Com_sprintf(
        levelMenuInfo.levelPicNames[n as usize].as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"levelshots/%s.tga\x00" as *const u8 as *const libc::c_char,
        map.as_mut_ptr(),
    );
    if crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        levelMenuInfo.levelPicNames[n as usize].as_mut_ptr(),
    ) == 0
    {
        ::libc::strcpy(
            levelMenuInfo.levelPicNames[n as usize].as_mut_ptr(),
            b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char,
        );
    }
    levelMenuInfo.item_maps[n as usize].shader = 0 as libc::c_int;
    if selectedArenaSet > currentSet {
        levelMenuInfo.item_maps[n as usize].generic.flags |= 0x2000 as libc::c_int as libc::c_uint
    } else {
        levelMenuInfo.item_maps[n as usize].generic.flags &=
            !(0x2000 as libc::c_int as libc::c_uint)
    }
    levelMenuInfo.item_maps[n as usize].generic.flags &= !(0x4000 as libc::c_int as libc::c_uint);
}

unsafe extern "C" fn UI_SPLevelMenu_SetMenuItems() {
    let mut n: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut arenaInfo: *const libc::c_char = 0 as *const libc::c_char;
    if selectedArenaSet > currentSet {
        selectedArena = -(1 as libc::c_int)
    } else if selectedArena == -(1 as libc::c_int) {
        selectedArena = 0 as libc::c_int
    }
    if selectedArenaSet == trainingTier || selectedArenaSet == finalTier {
        selectedArena = 0 as libc::c_int
    }
    if selectedArena != -(1 as libc::c_int) {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
            (selectedArenaSet * 4 as libc::c_int + selectedArena) as libc::c_float,
        );
    }
    if selectedArenaSet == trainingTier {
        arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo(
            b"training\x00" as *const u8 as *const libc::c_char,
        );
        level = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const libc::c_char,
        ));
        UI_SPLevelMenu_SetMenuArena(0 as libc::c_int, level, arenaInfo);
        levelMenuInfo.selectedArenaInfo = arenaInfo;
        levelMenuInfo.item_maps[0 as libc::c_int as usize].generic.x = 256 as libc::c_int;
        crate::src::q3_ui::ui_qmenu::Bitmap_Init(
            &mut *levelMenuInfo
                .item_maps
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut _
                as *mut crate::ui_local_h::menubitmap_s,
        );
        levelMenuInfo.item_maps[0 as libc::c_int as usize]
            .generic
            .bottom += 32 as libc::c_int;
        levelMenuInfo.numMaps = 1 as libc::c_int;
        levelMenuInfo.item_maps[1 as libc::c_int as usize]
            .generic
            .flags |= 0x4000 as libc::c_int as libc::c_uint;
        levelMenuInfo.item_maps[2 as libc::c_int as usize]
            .generic
            .flags |= 0x4000 as libc::c_int as libc::c_uint;
        levelMenuInfo.item_maps[3 as libc::c_int as usize]
            .generic
            .flags |= 0x4000 as libc::c_int as libc::c_uint;
        levelMenuInfo.levelPicNames[1 as libc::c_int as usize][0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        levelMenuInfo.levelPicNames[2 as libc::c_int as usize][0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        levelMenuInfo.levelPicNames[3 as libc::c_int as usize][0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        levelMenuInfo.item_maps[1 as libc::c_int as usize].shader = 0 as libc::c_int;
        levelMenuInfo.item_maps[2 as libc::c_int as usize].shader = 0 as libc::c_int;
        levelMenuInfo.item_maps[3 as libc::c_int as usize].shader = 0 as libc::c_int
    } else if selectedArenaSet == finalTier {
        arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo(
            b"final\x00" as *const u8 as *const libc::c_char,
        );
        level = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const libc::c_char,
        ));
        UI_SPLevelMenu_SetMenuArena(0 as libc::c_int, level, arenaInfo);
        levelMenuInfo.selectedArenaInfo = arenaInfo;
        levelMenuInfo.item_maps[0 as libc::c_int as usize].generic.x = 256 as libc::c_int;
        crate::src::q3_ui::ui_qmenu::Bitmap_Init(
            &mut *levelMenuInfo
                .item_maps
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut _
                as *mut crate::ui_local_h::menubitmap_s,
        );
        levelMenuInfo.item_maps[0 as libc::c_int as usize]
            .generic
            .bottom += 32 as libc::c_int;
        levelMenuInfo.numMaps = 1 as libc::c_int;
        levelMenuInfo.item_maps[1 as libc::c_int as usize]
            .generic
            .flags |= 0x4000 as libc::c_int as libc::c_uint;
        levelMenuInfo.item_maps[2 as libc::c_int as usize]
            .generic
            .flags |= 0x4000 as libc::c_int as libc::c_uint;
        levelMenuInfo.item_maps[3 as libc::c_int as usize]
            .generic
            .flags |= 0x4000 as libc::c_int as libc::c_uint;
        levelMenuInfo.levelPicNames[1 as libc::c_int as usize][0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        levelMenuInfo.levelPicNames[2 as libc::c_int as usize][0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        levelMenuInfo.levelPicNames[3 as libc::c_int as usize][0 as libc::c_int as usize] =
            0 as libc::c_int as libc::c_char;
        levelMenuInfo.item_maps[1 as libc::c_int as usize].shader = 0 as libc::c_int;
        levelMenuInfo.item_maps[2 as libc::c_int as usize].shader = 0 as libc::c_int;
        levelMenuInfo.item_maps[3 as libc::c_int as usize].shader = 0 as libc::c_int
    } else {
        levelMenuInfo.item_maps[0 as libc::c_int as usize].generic.x = 46 as libc::c_int;
        crate::src::q3_ui::ui_qmenu::Bitmap_Init(
            &mut *levelMenuInfo
                .item_maps
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut _
                as *mut crate::ui_local_h::menubitmap_s,
        );
        levelMenuInfo.item_maps[0 as libc::c_int as usize]
            .generic
            .bottom += 18 as libc::c_int;
        levelMenuInfo.numMaps = 4 as libc::c_int;
        n = 0 as libc::c_int;
        while n < 4 as libc::c_int {
            level = selectedArenaSet * 4 as libc::c_int + n;
            arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(level);
            UI_SPLevelMenu_SetMenuArena(n, level, arenaInfo);
            n += 1
        }
        if selectedArena != -(1 as libc::c_int) {
            levelMenuInfo.selectedArenaInfo =
                crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(
                    selectedArenaSet * 4 as libc::c_int + selectedArena,
                )
        }
    }
    // enable/disable arrows when they are valid/invalid
    if selectedArenaSet == minTier {
        levelMenuInfo.item_leftarrow.generic.flags |=
            0x4000 as libc::c_int as libc::c_uint | 0x1000 as libc::c_int as libc::c_uint
    } else {
        levelMenuInfo.item_leftarrow.generic.flags &=
            !(0x4000 as libc::c_int as libc::c_uint | 0x1000 as libc::c_int as libc::c_uint)
    }
    if selectedArenaSet == maxTier {
        levelMenuInfo.item_rightarrow.generic.flags |=
            0x4000 as libc::c_int as libc::c_uint | 0x1000 as libc::c_int as libc::c_uint
    } else {
        levelMenuInfo.item_rightarrow.generic.flags &=
            !(0x4000 as libc::c_int as libc::c_uint | 0x1000 as libc::c_int as libc::c_uint)
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
        640 as libc::c_int / 2 as libc::c_int,
        356 as libc::c_int + 27 as libc::c_int * 0 as libc::c_int,
        b"WARNING: This resets all of the\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x10 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        640 as libc::c_int / 2 as libc::c_int,
        356 as libc::c_int + 27 as libc::c_int * 1 as libc::c_int,
        b"single player game variables.\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x10 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        640 as libc::c_int / 2 as libc::c_int,
        356 as libc::c_int + 27 as libc::c_int * 2 as libc::c_int,
        b"Do this only if you want to\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x10 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        640 as libc::c_int / 2 as libc::c_int,
        356 as libc::c_int + 27 as libc::c_int * 3 as libc::c_int,
        b"start over from the beginning.\x00" as *const u8 as *const libc::c_char,
        0x1 as libc::c_int | 0x10 as libc::c_int,
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
        b"training\x00" as *const u8 as *const libc::c_char,
    )
    .is_null()
    {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
            -(4 as libc::c_int) as libc::c_float,
        );
    } else {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
            0 as libc::c_int as libc::c_float,
        );
    }
    // make the level select menu re-initialize
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
    UI_SPLevelMenu();
}

unsafe extern "C" fn UI_SPLevelMenu_ResetEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3 as libc::c_int {
        return;
    }
    crate::src::q3_ui::ui_confirm::UI_ConfirmMenu(
        b"RESET GAME?\x00" as *const u8 as *const libc::c_char,
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

unsafe extern "C" fn UI_SPLevelMenu_LevelEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3 as libc::c_int {
        return;
    }
    if selectedArenaSet == trainingTier || selectedArenaSet == finalTier {
        return;
    }
    selectedArena = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id - 11 as libc::c_int;
    levelMenuInfo.selectedArenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(
        selectedArenaSet * 4 as libc::c_int + selectedArena,
    );
    UI_SPLevelMenu_SetBots();
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
        (selectedArenaSet * 4 as libc::c_int + selectedArena) as libc::c_float,
    );
}
/*
=================
UI_SPLevelMenu_LeftArrowEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_LeftArrowEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3 as libc::c_int {
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
    mut notification: libc::c_int,
) {
    if notification != 3 as libc::c_int {
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

unsafe extern "C" fn UI_SPLevelMenu_PlayerEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3 as libc::c_int {
        return;
    }
    crate::src::q3_ui::ui_playersettings::UI_PlayerSettingsMenu();
}
/*
=================
UI_SPLevelMenu_AwardEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_AwardEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    if notification != 3 as libc::c_int {
        return;
    }
    n = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id - 17 as libc::c_int;
    crate::src::ui::ui_syscalls::trap_S_StartLocalSound(
        levelMenuInfo.awardSounds[n as usize],
        crate::src::qcommon::q_shared::CHAN_ANNOUNCER as libc::c_int,
    );
}
/*
=================
UI_SPLevelMenu_NextEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_NextEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3 as libc::c_int {
        return;
    }
    if selectedArenaSet > currentSet {
        return;
    }
    if selectedArena == -(1 as libc::c_int) {
        selectedArena = 0 as libc::c_int
    }
    crate::src::q3_ui::ui_spskill::UI_SPSkillMenu(levelMenuInfo.selectedArenaInfo);
}
/*
=================
UI_SPLevelMenu_BackEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_BackEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3 as libc::c_int {
        return;
    }
    if selectedArena == -(1 as libc::c_int) {
        selectedArena = 0 as libc::c_int
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
}
/*
=================
UI_SPLevelMenu_CustomEvent
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_CustomEvent(
    mut ptr: *mut libc::c_void,
    mut notification: libc::c_int,
) {
    if notification != 3 as libc::c_int {
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
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut color: crate::src::qcommon::q_shared::vec4_t = [0.; 4];
    let mut level: libc::c_int = 0;
    //	int				fraglimit;
    let mut pad: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 64] = [0; 64];
    if levelMenuInfo.reinit as u64 != 0 {
        crate::src::q3_ui::ui_atoms::UI_PopMenu();
        UI_SPLevelMenu();
        return;
    }
    // draw player name
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"name\x00" as *const u8 as *const libc::c_char,
        string.as_mut_ptr(),
        32 as libc::c_int,
    );
    crate::src::qcommon::q_shared::Q_CleanStr(string.as_mut_ptr());
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        320 as libc::c_int,
        314 as libc::c_int,
        string.as_mut_ptr(),
        0x1 as libc::c_int | 0x10 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
    );
    // check for model changes
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(
        buf.as_mut_ptr(),
        levelMenuInfo.playerModel.as_mut_ptr(),
    ) != 0 as libc::c_int
    {
        crate::src::qcommon::q_shared::Q_strncpyz(
            levelMenuInfo.playerModel.as_mut_ptr(),
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        PlayerIcon(
            levelMenuInfo.playerModel.as_mut_ptr(),
            levelMenuInfo.playerPicName.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        );
        levelMenuInfo.item_player.shader = 0 as libc::c_int
    }
    // standard menu drawing
    crate::src::q3_ui::ui_qmenu::Menu_Draw(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
    // draw player award levels
    y = 314 as libc::c_int + 26 as libc::c_int;
    i = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n < 6 as libc::c_int {
        level = levelMenuInfo.awardLevels[n as usize];
        if level > 0 as libc::c_int {
            if i & 1 as libc::c_int != 0 {
                x = 224 as libc::c_int
                    - (i - 1 as libc::c_int) / 2 as libc::c_int
                        * (48 as libc::c_int + 16 as libc::c_int)
            } else {
                x = 368 as libc::c_int
                    + i / 2 as libc::c_int * (48 as libc::c_int + 16 as libc::c_int)
            }
            i += 1;
            if !(level == 1 as libc::c_int) {
                if level >= 1000000 as libc::c_int {
                    crate::src::qcommon::q_shared::Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                        b"%im\x00" as *const u8 as *const libc::c_char,
                        level / 1000000 as libc::c_int,
                    );
                } else if level >= 1000 as libc::c_int {
                    crate::src::qcommon::q_shared::Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                        b"%ik\x00" as *const u8 as *const libc::c_char,
                        level / 1000 as libc::c_int,
                    );
                } else {
                    crate::src::qcommon::q_shared::Com_sprintf(
                        string.as_mut_ptr(),
                        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
                        b"%i\x00" as *const u8 as *const libc::c_char,
                        level,
                    );
                }
                crate::src::q3_ui::ui_atoms::UI_DrawString(
                    x + 24 as libc::c_int,
                    y + 48 as libc::c_int,
                    string.as_mut_ptr(),
                    0x1 as libc::c_int,
                    crate::src::q3_ui::ui_qmenu::color_yellow.as_mut_ptr(),
                );
            }
        }
        n += 1
    }
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        18 as libc::c_int,
        38 as libc::c_int,
        crate::src::qcommon::q_shared::va(
            b"Tier %i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            selectedArenaSet + 1 as libc::c_int,
        ),
        0 as libc::c_int | 0x10 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
    );
    n = 0 as libc::c_int;
    while n < levelMenuInfo.numMaps {
        x = levelMenuInfo.item_maps[n as usize].generic.x;
        y = levelMenuInfo.item_maps[n as usize].generic.y;
        crate::src::q3_ui::ui_atoms::UI_FillRect(
            x as libc::c_float,
            (y + 96 as libc::c_int) as libc::c_float,
            128 as libc::c_int as libc::c_float,
            18 as libc::c_int as libc::c_float,
            crate::src::q3_ui::ui_qmenu::color_black.as_mut_ptr(),
        );
        n += 1
    }
    if selectedArenaSet > currentSet {
        crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
            320 as libc::c_int,
            216 as libc::c_int,
            b"ACCESS DENIED\x00" as *const u8 as *const libc::c_char,
            0x1 as libc::c_int | 0x20 as libc::c_int,
            crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
        );
        return;
    }
    // show levelshots for levels of current tier
    color[0 as libc::c_int as usize] =
        crate::src::q3_ui::ui_qmenu::color_white[0 as libc::c_int as usize];
    color[1 as libc::c_int as usize] =
        crate::src::q3_ui::ui_qmenu::color_white[1 as libc::c_int as usize];
    color[2 as libc::c_int as usize] =
        crate::src::q3_ui::ui_qmenu::color_white[2 as libc::c_int as usize];
    color[3 as libc::c_int as usize] =
        crate::src::q3_ui::ui_qmenu::color_white[3 as libc::c_int as usize];
    color[3 as libc::c_int as usize] = (0.5f64
        + 0.5f64
            * crate::stdlib::sin(
                (crate::src::q3_ui::ui_atoms::uis.realtime / 75 as libc::c_int) as libc::c_double,
            )) as crate::src::qcommon::q_shared::vec_t;
    n = 0 as libc::c_int;
    while n < levelMenuInfo.numMaps {
        x = levelMenuInfo.item_maps[n as usize].generic.x;
        y = levelMenuInfo.item_maps[n as usize].generic.y;
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            x + 64 as libc::c_int,
            y + 96 as libc::c_int,
            levelMenuInfo.levelNames[n as usize].as_mut_ptr(),
            0x1 as libc::c_int | 0x10 as libc::c_int,
            crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
        );
        if levelMenuInfo.levelScores[n as usize] == 1 as libc::c_int {
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
                x as libc::c_float,
                y as libc::c_float,
                128 as libc::c_int as libc::c_float,
                96 as libc::c_int as libc::c_float,
                levelMenuInfo.levelCompletePic
                    [(levelMenuInfo.levelScoresSkill[n as usize] - 1 as libc::c_int) as usize],
            );
        }
        if n == selectedArena {
            if crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor(
                &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
            ) == &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(n as isize)
                as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void
            {
                crate::src::ui::ui_syscalls::trap_R_SetColor(color.as_mut_ptr());
            }
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
                (x - 1 as libc::c_int) as libc::c_float,
                (y - 1 as libc::c_int) as libc::c_float,
                130 as libc::c_int as libc::c_float,
                (130 as libc::c_int - 14 as libc::c_int) as libc::c_float,
                levelMenuInfo.levelSelectedPic,
            );
            crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const libc::c_float);
        } else if crate::src::q3_ui::ui_qmenu::Menu_ItemAtCursor(
            &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        ) == &mut *levelMenuInfo.item_maps.as_mut_ptr().offset(n as isize)
            as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void
        {
            crate::src::ui::ui_syscalls::trap_R_SetColor(color.as_mut_ptr());
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
                (x - 31 as libc::c_int) as libc::c_float,
                (y - 30 as libc::c_int) as libc::c_float,
                256 as libc::c_int as libc::c_float,
                (256 as libc::c_int - 27 as libc::c_int) as libc::c_float,
                levelMenuInfo.levelFocusPic,
            );
            crate::src::ui::ui_syscalls::trap_R_SetColor(0 as *const libc::c_float);
        }
        n += 1
    }
    // show map name and long name of selected level
    y = 192 as libc::c_int;
    crate::src::qcommon::q_shared::Q_strncpyz(
        buf.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"map\x00" as *const u8 as *const libc::c_char,
        ),
        20 as libc::c_int,
    );
    crate::src::qcommon::q_shared::Q_strupr(buf.as_mut_ptr());
    crate::src::qcommon::q_shared::Com_sprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"%s: %s\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            levelMenuInfo.selectedArenaInfo,
            b"longname\x00" as *const u8 as *const libc::c_char,
        ),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
        320 as libc::c_int,
        y,
        string.as_mut_ptr(),
        0x1 as libc::c_int | 0x10 as libc::c_int,
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
    );
    //	fraglimit = atoi( Info_ValueForKey( levelMenuInfo.selectedArenaInfo, "fraglimit" ) );
    //	UI_DrawString( 18, 212, va("Frags %i", fraglimit) , UI_LEFT|UI_SMALLFONT, color_orange );
    // draw bot opponents
    y += 24 as libc::c_int;
    pad = (7 as libc::c_int - levelMenuInfo.numBots) * (64 as libc::c_int + 26 as libc::c_int)
        / 2 as libc::c_int;
    n = 0 as libc::c_int;
    while n < levelMenuInfo.numBots {
        x = 18 as libc::c_int + pad + (64 as libc::c_int + 26 as libc::c_int) * n;
        if levelMenuInfo.botPics[n as usize] != 0 {
            crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
                x as libc::c_float,
                y as libc::c_float,
                64 as libc::c_int as libc::c_float,
                64 as libc::c_int as libc::c_float,
                levelMenuInfo.botPics[n as usize],
            );
        } else {
            crate::src::q3_ui::ui_atoms::UI_FillRect(
                x as libc::c_float,
                y as libc::c_float,
                64 as libc::c_int as libc::c_float,
                64 as libc::c_int as libc::c_float,
                crate::src::q3_ui::ui_qmenu::color_black.as_mut_ptr(),
            );
            crate::src::q3_ui::ui_atoms::UI_DrawProportionalString(
                x + 22 as libc::c_int,
                y + 18 as libc::c_int,
                b"?\x00" as *const u8 as *const libc::c_char,
                0x20 as libc::c_int,
                crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
            );
        }
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            x,
            y + 64 as libc::c_int,
            levelMenuInfo.botNames[n as usize].as_mut_ptr(),
            0x10 as libc::c_int | 0 as libc::c_int,
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
    let mut n: libc::c_int = 0;
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/maps_selected\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/narrow_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/narrow_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/unknownmap\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete2\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete3\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete4\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/level_complete5\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/reset_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/reset_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/skirmish_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/skirmish_1\x00" as *const u8 as *const libc::c_char,
    );
    n = 0 as libc::c_int;
    while n < 6 as libc::c_int {
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
        b"menu/art/maps_selected\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelFocusPic = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/maps_select\x00" as *const u8 as *const libc::c_char,
    );
    levelMenuInfo.levelCompletePic[0 as libc::c_int as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/level_complete1\x00" as *const u8 as *const libc::c_char,
        );
    levelMenuInfo.levelCompletePic[1 as libc::c_int as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/level_complete2\x00" as *const u8 as *const libc::c_char,
        );
    levelMenuInfo.levelCompletePic[2 as libc::c_int as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/level_complete3\x00" as *const u8 as *const libc::c_char,
        );
    levelMenuInfo.levelCompletePic[3 as libc::c_int as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/level_complete4\x00" as *const u8 as *const libc::c_char,
        );
    levelMenuInfo.levelCompletePic[4 as libc::c_int as usize] =
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
            b"menu/art/level_complete5\x00" as *const u8 as *const libc::c_char,
        );
}
/*
=================
UI_SPLevelMenu_Init
=================
*/

unsafe extern "C" fn UI_SPLevelMenu_Init() {
    let mut skill: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    skill = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"g_spSkill\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if skill < 1 as libc::c_int || skill > 5 as libc::c_int {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            b"g_spSkill\x00" as *const u8 as *const libc::c_char,
            b"2\x00" as *const u8 as *const libc::c_char,
        );
    }
    crate::stdlib::memset(
        &mut levelMenuInfo as *mut levelMenuInfo_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<levelMenuInfo_t>() as libc::c_ulong,
    );
    levelMenuInfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    levelMenuInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    levelMenuInfo.menu.draw = Some(UI_SPLevelMenu_MenuDraw as unsafe extern "C" fn() -> ());
    UI_SPLevelMenu_Cache();
    levelMenuInfo.item_banner.generic.type_0 = 10 as libc::c_int;
    levelMenuInfo.item_banner.generic.x = 320 as libc::c_int;
    levelMenuInfo.item_banner.generic.y = 16 as libc::c_int;
    levelMenuInfo.item_banner.string =
        b"CHOOSE LEVEL\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_banner.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    levelMenuInfo.item_banner.style = 0x1 as libc::c_int;
    levelMenuInfo.item_leftarrow.generic.type_0 = 6 as libc::c_int;
    levelMenuInfo.item_leftarrow.generic.name =
        b"menu/art/narrow_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_leftarrow.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_leftarrow.generic.x = 18 as libc::c_int;
    levelMenuInfo.item_leftarrow.generic.y = 64 as libc::c_int;
    levelMenuInfo.item_leftarrow.generic.callback = Some(
        UI_SPLevelMenu_LeftArrowEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_leftarrow.generic.id = 10 as libc::c_int;
    levelMenuInfo.item_leftarrow.width = 16 as libc::c_int;
    levelMenuInfo.item_leftarrow.height = 114 as libc::c_int;
    levelMenuInfo.item_leftarrow.focuspic =
        b"menu/art/narrow_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_maps[0 as libc::c_int as usize]
        .generic
        .type_0 = 6 as libc::c_int;
    levelMenuInfo.item_maps[0 as libc::c_int as usize]
        .generic
        .name = levelMenuInfo.levelPicNames[0 as libc::c_int as usize].as_mut_ptr();
    levelMenuInfo.item_maps[0 as libc::c_int as usize]
        .generic
        .flags = 0x4 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_maps[0 as libc::c_int as usize].generic.x = 46 as libc::c_int;
    levelMenuInfo.item_maps[0 as libc::c_int as usize].generic.y = 64 as libc::c_int;
    levelMenuInfo.item_maps[0 as libc::c_int as usize]
        .generic
        .id = 11 as libc::c_int;
    levelMenuInfo.item_maps[0 as libc::c_int as usize]
        .generic
        .callback = Some(
        UI_SPLevelMenu_LevelEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_maps[0 as libc::c_int as usize].width = 128 as libc::c_int;
    levelMenuInfo.item_maps[0 as libc::c_int as usize].height = 96 as libc::c_int;
    levelMenuInfo.item_maps[1 as libc::c_int as usize]
        .generic
        .type_0 = 6 as libc::c_int;
    levelMenuInfo.item_maps[1 as libc::c_int as usize]
        .generic
        .name = levelMenuInfo.levelPicNames[1 as libc::c_int as usize].as_mut_ptr();
    levelMenuInfo.item_maps[1 as libc::c_int as usize]
        .generic
        .flags = 0x4 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_maps[1 as libc::c_int as usize].generic.x = 186 as libc::c_int;
    levelMenuInfo.item_maps[1 as libc::c_int as usize].generic.y = 64 as libc::c_int;
    levelMenuInfo.item_maps[1 as libc::c_int as usize]
        .generic
        .id = 12 as libc::c_int;
    levelMenuInfo.item_maps[1 as libc::c_int as usize]
        .generic
        .callback = Some(
        UI_SPLevelMenu_LevelEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_maps[1 as libc::c_int as usize].width = 128 as libc::c_int;
    levelMenuInfo.item_maps[1 as libc::c_int as usize].height = 96 as libc::c_int;
    levelMenuInfo.item_maps[2 as libc::c_int as usize]
        .generic
        .type_0 = 6 as libc::c_int;
    levelMenuInfo.item_maps[2 as libc::c_int as usize]
        .generic
        .name = levelMenuInfo.levelPicNames[2 as libc::c_int as usize].as_mut_ptr();
    levelMenuInfo.item_maps[2 as libc::c_int as usize]
        .generic
        .flags = 0x4 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_maps[2 as libc::c_int as usize].generic.x = 326 as libc::c_int;
    levelMenuInfo.item_maps[2 as libc::c_int as usize].generic.y = 64 as libc::c_int;
    levelMenuInfo.item_maps[2 as libc::c_int as usize]
        .generic
        .id = 13 as libc::c_int;
    levelMenuInfo.item_maps[2 as libc::c_int as usize]
        .generic
        .callback = Some(
        UI_SPLevelMenu_LevelEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_maps[2 as libc::c_int as usize].width = 128 as libc::c_int;
    levelMenuInfo.item_maps[2 as libc::c_int as usize].height = 96 as libc::c_int;
    levelMenuInfo.item_maps[3 as libc::c_int as usize]
        .generic
        .type_0 = 6 as libc::c_int;
    levelMenuInfo.item_maps[3 as libc::c_int as usize]
        .generic
        .name = levelMenuInfo.levelPicNames[3 as libc::c_int as usize].as_mut_ptr();
    levelMenuInfo.item_maps[3 as libc::c_int as usize]
        .generic
        .flags = 0x4 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_maps[3 as libc::c_int as usize].generic.x = 466 as libc::c_int;
    levelMenuInfo.item_maps[3 as libc::c_int as usize].generic.y = 64 as libc::c_int;
    levelMenuInfo.item_maps[3 as libc::c_int as usize]
        .generic
        .id = 14 as libc::c_int;
    levelMenuInfo.item_maps[3 as libc::c_int as usize]
        .generic
        .callback = Some(
        UI_SPLevelMenu_LevelEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_maps[3 as libc::c_int as usize].width = 128 as libc::c_int;
    levelMenuInfo.item_maps[3 as libc::c_int as usize].height = 96 as libc::c_int;
    levelMenuInfo.item_rightarrow.generic.type_0 = 6 as libc::c_int;
    levelMenuInfo.item_rightarrow.generic.name =
        b"menu/art/narrow_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_rightarrow.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_rightarrow.generic.x = 606 as libc::c_int;
    levelMenuInfo.item_rightarrow.generic.y = 64 as libc::c_int;
    levelMenuInfo.item_rightarrow.generic.callback = Some(
        UI_SPLevelMenu_RightArrowEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_rightarrow.generic.id = 15 as libc::c_int;
    levelMenuInfo.item_rightarrow.width = -(16 as libc::c_int);
    levelMenuInfo.item_rightarrow.height = 114 as libc::c_int;
    levelMenuInfo.item_rightarrow.focuspic =
        b"menu/art/narrow_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"model\x00" as *const u8 as *const libc::c_char,
        levelMenuInfo.playerModel.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    PlayerIcon(
        levelMenuInfo.playerModel.as_mut_ptr(),
        levelMenuInfo.playerPicName.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    levelMenuInfo.item_player.generic.type_0 = 6 as libc::c_int;
    levelMenuInfo.item_player.generic.name = levelMenuInfo.playerPicName.as_mut_ptr();
    levelMenuInfo.item_player.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x800 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_player.generic.x = 288 as libc::c_int;
    levelMenuInfo.item_player.generic.y = 314 as libc::c_int + 26 as libc::c_int;
    levelMenuInfo.item_player.generic.id = 16 as libc::c_int;
    levelMenuInfo.item_player.generic.callback = Some(
        UI_SPLevelMenu_PlayerEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_player.width = 64 as libc::c_int;
    levelMenuInfo.item_player.height = 64 as libc::c_int;
    n = 0 as libc::c_int;
    while n < 6 as libc::c_int {
        levelMenuInfo.awardLevels[n as usize] = crate::src::q3_ui::ui_gameinfo::UI_GetAwardLevel(n);
        n += 1
    }
    levelMenuInfo.awardLevels[crate::ui_local_h::AWARD_FRAGS as libc::c_int as usize] = 100
        as libc::c_int
        * (levelMenuInfo.awardLevels[crate::ui_local_h::AWARD_FRAGS as libc::c_int as usize]
            / 100 as libc::c_int);
    y = 314 as libc::c_int + 26 as libc::c_int;
    count = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n < 6 as libc::c_int {
        if levelMenuInfo.awardLevels[n as usize] != 0 {
            if count & 1 as libc::c_int != 0 {
                x = 224 as libc::c_int
                    - (count - 1 as libc::c_int) / 2 as libc::c_int
                        * (48 as libc::c_int + 16 as libc::c_int)
            } else {
                x = 368 as libc::c_int
                    + count / 2 as libc::c_int * (48 as libc::c_int + 16 as libc::c_int)
            }
            levelMenuInfo.item_awards[count as usize].generic.type_0 = 6 as libc::c_int;
            levelMenuInfo.item_awards[count as usize].generic.name =
                *crate::src::q3_ui::ui_sppostgame::ui_medalPicNames
                    .as_mut_ptr()
                    .offset(n as isize);
            levelMenuInfo.item_awards[count as usize].generic.flags = 0x4 as libc::c_int
                as libc::c_uint
                | 0x100000 as libc::c_int as libc::c_uint
                | 0x800 as libc::c_int as libc::c_uint;
            levelMenuInfo.item_awards[count as usize].generic.x = x;
            levelMenuInfo.item_awards[count as usize].generic.y = y;
            levelMenuInfo.item_awards[count as usize].generic.id = 17 as libc::c_int + n;
            levelMenuInfo.item_awards[count as usize].generic.callback = Some(
                UI_SPLevelMenu_AwardEvent
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
            );
            levelMenuInfo.item_awards[count as usize].width = 48 as libc::c_int;
            levelMenuInfo.item_awards[count as usize].height = 48 as libc::c_int;
            count += 1
        }
        n += 1
    }
    levelMenuInfo.item_back.generic.type_0 = 6 as libc::c_int;
    levelMenuInfo.item_back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_back.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_back.generic.x = 0 as libc::c_int;
    levelMenuInfo.item_back.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    levelMenuInfo.item_back.generic.callback = Some(
        UI_SPLevelMenu_BackEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_back.generic.id = 23 as libc::c_int;
    levelMenuInfo.item_back.width = 128 as libc::c_int;
    levelMenuInfo.item_back.height = 64 as libc::c_int;
    levelMenuInfo.item_back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_reset.generic.type_0 = 6 as libc::c_int;
    levelMenuInfo.item_reset.generic.name =
        b"menu/art/reset_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_reset.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_reset.generic.x = 170 as libc::c_int;
    levelMenuInfo.item_reset.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    levelMenuInfo.item_reset.generic.callback = Some(
        UI_SPLevelMenu_ResetEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_reset.generic.id = 24 as libc::c_int;
    levelMenuInfo.item_reset.width = 128 as libc::c_int;
    levelMenuInfo.item_reset.height = 64 as libc::c_int;
    levelMenuInfo.item_reset.focuspic =
        b"menu/art/reset_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_custom.generic.type_0 = 6 as libc::c_int;
    levelMenuInfo.item_custom.generic.name =
        b"menu/art/skirmish_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_custom.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_custom.generic.x = 342 as libc::c_int;
    levelMenuInfo.item_custom.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    levelMenuInfo.item_custom.generic.callback = Some(
        UI_SPLevelMenu_CustomEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_custom.generic.id = 25 as libc::c_int;
    levelMenuInfo.item_custom.width = 128 as libc::c_int;
    levelMenuInfo.item_custom.height = 64 as libc::c_int;
    levelMenuInfo.item_custom.focuspic =
        b"menu/art/skirmish_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_next.generic.type_0 = 6 as libc::c_int;
    levelMenuInfo.item_next.generic.name =
        b"menu/art/fight_0\x00" as *const u8 as *const libc::c_char;
    levelMenuInfo.item_next.generic.flags =
        0x10 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_next.generic.x = 640 as libc::c_int;
    levelMenuInfo.item_next.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    levelMenuInfo.item_next.generic.callback = Some(
        UI_SPLevelMenu_NextEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    levelMenuInfo.item_next.generic.id = 26 as libc::c_int;
    levelMenuInfo.item_next.width = 128 as libc::c_int;
    levelMenuInfo.item_next.height = 64 as libc::c_int;
    levelMenuInfo.item_next.focuspic =
        b"menu/art/fight_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    levelMenuInfo.item_null.generic.type_0 = 6 as libc::c_int;
    levelMenuInfo.item_null.generic.flags = 0x4 as libc::c_int as libc::c_uint
        | 0x800 as libc::c_int as libc::c_uint
        | 0x100000 as libc::c_int as libc::c_uint;
    levelMenuInfo.item_null.generic.x = 0 as libc::c_int;
    levelMenuInfo.item_null.generic.y = 0 as libc::c_int;
    levelMenuInfo.item_null.width = 640 as libc::c_int;
    levelMenuInfo.item_null.height = 480 as libc::c_int;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut levelMenuInfo.item_banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut levelMenuInfo.item_leftarrow as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut *levelMenuInfo
            .item_maps
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize) as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut *levelMenuInfo
            .item_maps
            .as_mut_ptr()
            .offset(1 as libc::c_int as isize) as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut *levelMenuInfo
            .item_maps
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize) as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut *levelMenuInfo
            .item_maps
            .as_mut_ptr()
            .offset(3 as libc::c_int as isize) as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    levelMenuInfo.item_maps[0 as libc::c_int as usize]
        .generic
        .bottom += 18 as libc::c_int;
    levelMenuInfo.item_maps[1 as libc::c_int as usize]
        .generic
        .bottom += 18 as libc::c_int;
    levelMenuInfo.item_maps[2 as libc::c_int as usize]
        .generic
        .bottom += 18 as libc::c_int;
    levelMenuInfo.item_maps[3 as libc::c_int as usize]
        .generic
        .bottom += 18 as libc::c_int;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut levelMenuInfo.item_rightarrow as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut levelMenuInfo.item_player as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    n = 0 as libc::c_int;
    while n < count {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
            &mut *levelMenuInfo.item_awards.as_mut_ptr().offset(n as isize)
                as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
        );
        n += 1
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut levelMenuInfo.item_back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut levelMenuInfo.item_reset as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut levelMenuInfo.item_custom as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut levelMenuInfo.item_next as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut levelMenuInfo.item_null as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"ui_spSelection\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
    );
    if *buf.as_mut_ptr() != 0 {
        n = atoi(buf.as_mut_ptr());
        selectedArenaSet = n / 4 as libc::c_int;
        selectedArena = n % 4 as libc::c_int
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
    let mut level: libc::c_int = 0;
    let mut trainingLevel: libc::c_int = 0;
    let mut arenaInfo: *const libc::c_char = 0 as *const libc::c_char;
    trainingTier = -(1 as libc::c_int);
    arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo(
        b"training\x00" as *const u8 as *const libc::c_char,
    );
    if !arenaInfo.is_null() {
        minTier = trainingTier;
        trainingLevel = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            arenaInfo,
            b"num\x00" as *const u8 as *const libc::c_char,
        ))
    } else {
        minTier = 0 as libc::c_int;
        trainingLevel = -(2 as libc::c_int)
    }
    finalTier = crate::src::q3_ui::ui_gameinfo::UI_GetNumSPTiers();
    arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetSpecialArenaInfo(
        b"final\x00" as *const u8 as *const libc::c_char,
    );
    if !arenaInfo.is_null() {
        maxTier = finalTier
    } else {
        maxTier = finalTier - 1 as libc::c_int;
        if maxTier < minTier {
            maxTier = minTier
        }
    }
    level = crate::src::q3_ui::ui_gameinfo::UI_GetCurrentGame();
    if level == -(1 as libc::c_int) {
        level = crate::src::q3_ui::ui_gameinfo::UI_GetNumSPArenas() - 1 as libc::c_int;
        if maxTier == finalTier {
            level += 1
        }
    }
    if level == trainingLevel {
        currentSet = -(1 as libc::c_int);
        currentGame = 0 as libc::c_int
    } else {
        currentSet = level / 4 as libc::c_int;
        currentGame = level % 4 as libc::c_int
    }
    UI_SPLevelMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
    crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
        &mut levelMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
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
    crate::src::ui::ui_syscalls::trap_Key_SetCatcher(0x2 as libc::c_int);
    crate::src::q3_ui::ui_atoms::uis.menusp = 0 as libc::c_int;
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
