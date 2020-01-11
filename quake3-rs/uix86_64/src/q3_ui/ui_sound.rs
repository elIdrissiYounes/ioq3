use ::libc;

pub use crate::src::q3_ui::ui_atoms::UI_ForceMenuOff;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu;
pub use crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::q3_ui::ui_video::UI_GraphicsOptionsMenu;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
use crate::stdlib::memset;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menuslider_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soundOptionsInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub graphics: crate::ui_local_h::menutext_s,
    pub display: crate::ui_local_h::menutext_s,
    pub sound: crate::ui_local_h::menutext_s,
    pub network: crate::ui_local_h::menutext_s,
    pub sfxvolume: crate::ui_local_h::menuslider_s,
    pub musicvolume: crate::ui_local_h::menuslider_s,
    pub soundSystem: crate::ui_local_h::menulist_s,
    pub quality: crate::ui_local_h::menulist_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub apply: crate::ui_local_h::menubitmap_s,
    pub sfxvolume_original: libc::c_float,
    pub musicvolume_original: libc::c_float,
    pub soundSystem_original: libc::c_int,
    pub quality_original: libc::c_int,
}

static mut quality_items: [*const libc::c_char; 4] = [
    b"Low\x00" as *const u8 as *const libc::c_char,
    b"Medium\x00" as *const u8 as *const libc::c_char,
    b"High\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

static mut soundSystem_items: [*const libc::c_char; 3] = [
    b"SDL\x00" as *const u8 as *const libc::c_char,
    b"OpenAL\x00" as *const u8 as *const libc::c_char,
    0 as *const libc::c_char,
];

static mut soundOptionsInfo: soundOptionsInfo_t = soundOptionsInfo_t {
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
    banner: crate::ui_local_h::menutext_s {
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
    framel: crate::ui_local_h::menubitmap_s {
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
    framer: crate::ui_local_h::menubitmap_s {
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
    graphics: crate::ui_local_h::menutext_s {
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
    display: crate::ui_local_h::menutext_s {
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
    sound: crate::ui_local_h::menutext_s {
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
    network: crate::ui_local_h::menutext_s {
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
    sfxvolume: crate::ui_local_h::menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    musicvolume: crate::ui_local_h::menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    soundSystem: crate::ui_local_h::menulist_s {
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
    quality: crate::ui_local_h::menulist_s {
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
    back: crate::ui_local_h::menubitmap_s {
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
    apply: crate::ui_local_h::menubitmap_s {
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
    sfxvolume_original: 0.,
    musicvolume_original: 0.,
    soundSystem_original: 0,
    quality_original: 0,
};
/*
=================
UI_SoundOptionsMenu_Event
=================
*/

unsafe extern "C" fn UI_SoundOptionsMenu_Event(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3 as libc::c_int {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        10 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_video::UI_GraphicsOptionsMenu();
        }
        11 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu();
        }
        13 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu();
        }
        19 => {
            /*
                case ID_A3D:
                    if( soundOptionsInfo.a3d.curvalue ) {
                        trap_Cmd_ExecuteText( EXEC_NOW, "s_enable_a3d\n" );
                    }
                    else {
                        trap_Cmd_ExecuteText( EXEC_NOW, "s_disable_a3d\n" );
                    }
                    soundOptionsInfo.a3d.curvalue = (int)trap_Cvar_VariableValue( "s_usingA3D" );
                    break;
            */
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        20 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"s_volume\x00" as *const u8 as *const libc::c_char,
                soundOptionsInfo.sfxvolume.curvalue / 10 as libc::c_int as libc::c_float,
            );
            soundOptionsInfo.sfxvolume_original = soundOptionsInfo.sfxvolume.curvalue;
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"s_musicvolume\x00" as *const u8 as *const libc::c_char,
                soundOptionsInfo.musicvolume.curvalue / 10 as libc::c_int as libc::c_float,
            );
            soundOptionsInfo.musicvolume_original = soundOptionsInfo.musicvolume.curvalue;
            // Check if something changed that requires the sound system to be restarted.
            if soundOptionsInfo.quality_original != soundOptionsInfo.quality.curvalue
                || soundOptionsInfo.soundSystem_original != soundOptionsInfo.soundSystem.curvalue
            {
                let mut speed: libc::c_int = 0;
                match soundOptionsInfo.quality.curvalue {
                    1 => speed = 22050 as libc::c_int,
                    2 => speed = 44100 as libc::c_int,
                    0 | _ => speed = 11025 as libc::c_int,
                }
                if speed == 22050 as libc::c_int {
                    speed = 0 as libc::c_int
                }
                crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                    b"s_sdlSpeed\x00" as *const u8 as *const libc::c_char,
                    speed as libc::c_float,
                );
                soundOptionsInfo.quality_original = soundOptionsInfo.quality.curvalue;
                crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                    b"s_useOpenAL\x00" as *const u8 as *const libc::c_char,
                    (soundOptionsInfo.soundSystem.curvalue == 1 as libc::c_int) as libc::c_int
                        as libc::c_float,
                );
                soundOptionsInfo.soundSystem_original = soundOptionsInfo.soundSystem.curvalue;
                crate::src::q3_ui::ui_atoms::UI_ForceMenuOff();
                crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                    crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
                    b"snd_restart\n\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        12 | _ => {}
    };
}
/*
=================
SoundOptions_UpdateMenuItems
=================
*/

unsafe extern "C" fn SoundOptions_UpdateMenuItems() {
    if soundOptionsInfo.soundSystem.curvalue == 0 as libc::c_int {
        soundOptionsInfo.quality.generic.flags &= !(0x2000 as libc::c_int as libc::c_uint)
    } else {
        soundOptionsInfo.quality.generic.flags |= 0x2000 as libc::c_int as libc::c_uint
    }
    soundOptionsInfo.apply.generic.flags |=
        0x1000 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint;
    if soundOptionsInfo.sfxvolume_original != soundOptionsInfo.sfxvolume.curvalue {
        soundOptionsInfo.apply.generic.flags &=
            !(0x1000 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint)
    }
    if soundOptionsInfo.musicvolume_original != soundOptionsInfo.musicvolume.curvalue {
        soundOptionsInfo.apply.generic.flags &=
            !(0x1000 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint)
    }
    if soundOptionsInfo.soundSystem_original != soundOptionsInfo.soundSystem.curvalue {
        soundOptionsInfo.apply.generic.flags &=
            !(0x1000 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint)
    }
    if soundOptionsInfo.quality_original != soundOptionsInfo.quality.curvalue {
        soundOptionsInfo.apply.generic.flags &=
            !(0x1000 as libc::c_int as libc::c_uint | 0x4000 as libc::c_int as libc::c_uint)
    };
}
/*
================
SoundOptions_MenuDraw
================
*/
#[no_mangle]

pub unsafe extern "C" fn SoundOptions_MenuDraw() {
    //APSFIX - rework this
    SoundOptions_UpdateMenuItems();
    crate::src::q3_ui::ui_qmenu::Menu_Draw(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}
/*
===============
UI_SoundOptionsMenu_Init
===============
*/

unsafe extern "C" fn UI_SoundOptionsMenu_Init() {
    let mut y: libc::c_int = 0;
    let mut speed: libc::c_int = 0;
    crate::stdlib::memset(
        &mut soundOptionsInfo as *mut soundOptionsInfo_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<soundOptionsInfo_t>() as libc::c_ulong,
    );
    UI_SoundOptionsMenu_Cache();
    soundOptionsInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    soundOptionsInfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    soundOptionsInfo.menu.draw = Some(SoundOptions_MenuDraw as unsafe extern "C" fn() -> ());
    soundOptionsInfo.banner.generic.type_0 = 10 as libc::c_int;
    soundOptionsInfo.banner.generic.flags = 0x8 as libc::c_int as libc::c_uint;
    soundOptionsInfo.banner.generic.x = 320 as libc::c_int;
    soundOptionsInfo.banner.generic.y = 16 as libc::c_int;
    soundOptionsInfo.banner.string =
        b"SYSTEM SETUP\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    soundOptionsInfo.banner.style = 0x1 as libc::c_int;
    soundOptionsInfo.framel.generic.type_0 = 6 as libc::c_int;
    soundOptionsInfo.framel.generic.name =
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.framel.generic.flags = 0x4000 as libc::c_int as libc::c_uint;
    soundOptionsInfo.framel.generic.x = 0 as libc::c_int;
    soundOptionsInfo.framel.generic.y = 78 as libc::c_int;
    soundOptionsInfo.framel.width = 256 as libc::c_int;
    soundOptionsInfo.framel.height = 329 as libc::c_int;
    soundOptionsInfo.framer.generic.type_0 = 6 as libc::c_int;
    soundOptionsInfo.framer.generic.name =
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.framer.generic.flags = 0x4000 as libc::c_int as libc::c_uint;
    soundOptionsInfo.framer.generic.x = 376 as libc::c_int;
    soundOptionsInfo.framer.generic.y = 76 as libc::c_int;
    soundOptionsInfo.framer.width = 256 as libc::c_int;
    soundOptionsInfo.framer.height = 334 as libc::c_int;
    soundOptionsInfo.graphics.generic.type_0 = 9 as libc::c_int;
    soundOptionsInfo.graphics.generic.flags =
        0x10 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    soundOptionsInfo.graphics.generic.id = 10 as libc::c_int;
    soundOptionsInfo.graphics.generic.callback = Some(
        UI_SoundOptionsMenu_Event
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    soundOptionsInfo.graphics.generic.x = 216 as libc::c_int;
    soundOptionsInfo.graphics.generic.y = 240 as libc::c_int - 2 as libc::c_int * 27 as libc::c_int;
    soundOptionsInfo.graphics.string =
        b"GRAPHICS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.graphics.style = 0x2 as libc::c_int;
    soundOptionsInfo.graphics.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    soundOptionsInfo.display.generic.type_0 = 9 as libc::c_int;
    soundOptionsInfo.display.generic.flags =
        0x10 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    soundOptionsInfo.display.generic.id = 11 as libc::c_int;
    soundOptionsInfo.display.generic.callback = Some(
        UI_SoundOptionsMenu_Event
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    soundOptionsInfo.display.generic.x = 216 as libc::c_int;
    soundOptionsInfo.display.generic.y = 240 as libc::c_int - 27 as libc::c_int;
    soundOptionsInfo.display.string =
        b"DISPLAY\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.display.style = 0x2 as libc::c_int;
    soundOptionsInfo.display.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    soundOptionsInfo.sound.generic.type_0 = 9 as libc::c_int;
    soundOptionsInfo.sound.generic.flags = 0x10 as libc::c_int as libc::c_uint;
    soundOptionsInfo.sound.generic.id = 12 as libc::c_int;
    soundOptionsInfo.sound.generic.callback = Some(
        UI_SoundOptionsMenu_Event
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    soundOptionsInfo.sound.generic.x = 216 as libc::c_int;
    soundOptionsInfo.sound.generic.y = 240 as libc::c_int;
    soundOptionsInfo.sound.string =
        b"SOUND\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.sound.style = 0x2 as libc::c_int;
    soundOptionsInfo.sound.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    soundOptionsInfo.network.generic.type_0 = 9 as libc::c_int;
    soundOptionsInfo.network.generic.flags =
        0x10 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    soundOptionsInfo.network.generic.id = 13 as libc::c_int;
    soundOptionsInfo.network.generic.callback = Some(
        UI_SoundOptionsMenu_Event
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    soundOptionsInfo.network.generic.x = 216 as libc::c_int;
    soundOptionsInfo.network.generic.y = 240 as libc::c_int + 27 as libc::c_int;
    soundOptionsInfo.network.string =
        b"NETWORK\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.network.style = 0x2 as libc::c_int;
    soundOptionsInfo.network.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    y = 240 as libc::c_int - 2 as libc::c_int * (16 as libc::c_int + 2 as libc::c_int);
    soundOptionsInfo.sfxvolume.generic.type_0 = 1 as libc::c_int;
    soundOptionsInfo.sfxvolume.generic.name =
        b"Effects Volume:\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.sfxvolume.generic.flags =
        0x100 as libc::c_int as libc::c_uint | 0x2 as libc::c_int as libc::c_uint;
    soundOptionsInfo.sfxvolume.generic.callback = Some(
        UI_SoundOptionsMenu_Event
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    soundOptionsInfo.sfxvolume.generic.id = 14 as libc::c_int;
    soundOptionsInfo.sfxvolume.generic.x = 400 as libc::c_int;
    soundOptionsInfo.sfxvolume.generic.y = y;
    soundOptionsInfo.sfxvolume.minvalue = 0 as libc::c_int as libc::c_float;
    soundOptionsInfo.sfxvolume.maxvalue = 10 as libc::c_int as libc::c_float;
    y += 16 as libc::c_int + 2 as libc::c_int;
    soundOptionsInfo.musicvolume.generic.type_0 = 1 as libc::c_int;
    soundOptionsInfo.musicvolume.generic.name =
        b"Music Volume:\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.musicvolume.generic.flags =
        0x100 as libc::c_int as libc::c_uint | 0x2 as libc::c_int as libc::c_uint;
    soundOptionsInfo.musicvolume.generic.callback = Some(
        UI_SoundOptionsMenu_Event
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    soundOptionsInfo.musicvolume.generic.id = 15 as libc::c_int;
    soundOptionsInfo.musicvolume.generic.x = 400 as libc::c_int;
    soundOptionsInfo.musicvolume.generic.y = y;
    soundOptionsInfo.musicvolume.minvalue = 0 as libc::c_int as libc::c_float;
    soundOptionsInfo.musicvolume.maxvalue = 10 as libc::c_int as libc::c_float;
    y += 16 as libc::c_int + 2 as libc::c_int;
    soundOptionsInfo.soundSystem.generic.type_0 = 3 as libc::c_int;
    soundOptionsInfo.soundSystem.generic.name =
        b"Sound System:\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.soundSystem.generic.flags =
        0x100 as libc::c_int as libc::c_uint | 0x2 as libc::c_int as libc::c_uint;
    soundOptionsInfo.soundSystem.generic.callback = Some(
        UI_SoundOptionsMenu_Event
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    soundOptionsInfo.soundSystem.generic.id = 17 as libc::c_int;
    soundOptionsInfo.soundSystem.generic.x = 400 as libc::c_int;
    soundOptionsInfo.soundSystem.generic.y = y;
    soundOptionsInfo.soundSystem.itemnames = soundSystem_items.as_mut_ptr();
    y += 16 as libc::c_int + 2 as libc::c_int;
    soundOptionsInfo.quality.generic.type_0 = 3 as libc::c_int;
    soundOptionsInfo.quality.generic.name =
        b"SDL Sound Quality:\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.quality.generic.flags =
        0x100 as libc::c_int as libc::c_uint | 0x2 as libc::c_int as libc::c_uint;
    soundOptionsInfo.quality.generic.callback = Some(
        UI_SoundOptionsMenu_Event
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    soundOptionsInfo.quality.generic.id = 16 as libc::c_int;
    soundOptionsInfo.quality.generic.x = 400 as libc::c_int;
    soundOptionsInfo.quality.generic.y = y;
    soundOptionsInfo.quality.itemnames = quality_items.as_mut_ptr();
    /*
        y += BIGCHAR_HEIGHT+2;
        soundOptionsInfo.a3d.generic.type			= MTYPE_RADIOBUTTON;
        soundOptionsInfo.a3d.generic.name			= "A3D:";
        soundOptionsInfo.a3d.generic.flags			= QMF_PULSEIFFOCUS|QMF_SMALLFONT;
        soundOptionsInfo.a3d.generic.callback		= UI_SoundOptionsMenu_Event;
        soundOptionsInfo.a3d.generic.id				= ID_A3D;
        soundOptionsInfo.a3d.generic.x				= 400;
        soundOptionsInfo.a3d.generic.y				= y;
    */
    soundOptionsInfo.back.generic.type_0 = 6 as libc::c_int;
    soundOptionsInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.back.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    soundOptionsInfo.back.generic.callback = Some(
        UI_SoundOptionsMenu_Event
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    soundOptionsInfo.back.generic.id = 19 as libc::c_int;
    soundOptionsInfo.back.generic.x = 0 as libc::c_int;
    soundOptionsInfo.back.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    soundOptionsInfo.back.width = 128 as libc::c_int;
    soundOptionsInfo.back.height = 64 as libc::c_int;
    soundOptionsInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    soundOptionsInfo.apply.generic.type_0 = 6 as libc::c_int;
    soundOptionsInfo.apply.generic.name =
        b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char;
    soundOptionsInfo.apply.generic.flags = 0x10 as libc::c_int as libc::c_uint
        | 0x100 as libc::c_int as libc::c_uint
        | 0x1000 as libc::c_int as libc::c_uint
        | 0x4000 as libc::c_int as libc::c_uint;
    soundOptionsInfo.apply.generic.callback = Some(
        UI_SoundOptionsMenu_Event
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    soundOptionsInfo.apply.generic.id = 20 as libc::c_int;
    soundOptionsInfo.apply.generic.x = 640 as libc::c_int;
    soundOptionsInfo.apply.generic.y = 480 as libc::c_int - 64 as libc::c_int;
    soundOptionsInfo.apply.width = 128 as libc::c_int;
    soundOptionsInfo.apply.height = 64 as libc::c_int;
    soundOptionsInfo.apply.focuspic =
        b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.graphics as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.display as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.sound as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.network as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.sfxvolume as *mut crate::ui_local_h::menuslider_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.musicvolume as *mut crate::ui_local_h::menuslider_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.soundSystem as *mut crate::ui_local_h::menulist_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.quality as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    //	Menu_AddItem( &soundOptionsInfo.menu, ( void * ) &soundOptionsInfo.a3d );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.apply as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    soundOptionsInfo.sfxvolume_original = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"s_volume\x00" as *const u8 as *const libc::c_char,
    ) * 10 as libc::c_int as libc::c_float;
    soundOptionsInfo.sfxvolume.curvalue = soundOptionsInfo.sfxvolume_original;
    soundOptionsInfo.musicvolume_original = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"s_musicvolume\x00" as *const u8 as *const libc::c_char,
    ) * 10 as libc::c_int as libc::c_float;
    soundOptionsInfo.musicvolume.curvalue = soundOptionsInfo.musicvolume_original;
    if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"s_useOpenAL\x00" as *const u8 as *const libc::c_char,
    ) != 0.
    {
        soundOptionsInfo.soundSystem_original = 1 as libc::c_int
    } else {
        soundOptionsInfo.soundSystem_original = 0 as libc::c_int
    }
    soundOptionsInfo.soundSystem.curvalue = soundOptionsInfo.soundSystem_original;
    speed = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"s_sdlSpeed\x00" as *const u8 as *const libc::c_char,
    ) as libc::c_int;
    if speed == 0 {
        // Check for default
        speed = 22050 as libc::c_int
    }
    if speed <= 11025 as libc::c_int {
        soundOptionsInfo.quality_original = 0 as libc::c_int
    } else if speed <= 22050 as libc::c_int {
        soundOptionsInfo.quality_original = 1 as libc::c_int
    } else {
        // 44100
        soundOptionsInfo.quality_original = 2 as libc::c_int
    }
    soundOptionsInfo.quality.curvalue = soundOptionsInfo.quality_original;
    //	soundOptionsInfo.a3d.curvalue = (int)trap_Cvar_VariableValue( "s_usingA3D" );
}
/*
===============
UI_SoundOptionsMenu_Cache
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SoundOptionsMenu_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame2_l\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame1_r\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/accept_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/accept_1\x00" as *const u8 as *const libc::c_char,
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
//
// ui_spSkill.c
//
//
// ui_syscalls.c
//
// don't use EXEC_NOW!
// fsOrigin_t
//
// ui_addbots.c
//
//
// ui_removebots.c
//
//
// ui_teamorders.c
//
//
// ui_loadconfig.c
//
//
// ui_saveconfig.c
//
//
// ui_display.c
//
//
// ui_sound.c
//
/*
===============
UI_SoundOptionsMenu
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SoundOptionsMenu() {
    UI_SoundOptionsMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
    crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
        &mut soundOptionsInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut soundOptionsInfo.sound as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
}
