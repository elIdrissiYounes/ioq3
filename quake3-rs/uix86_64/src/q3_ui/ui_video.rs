use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu;
pub use crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_qmenu::Menu_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem;
pub use crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu;
pub use crate::src::q3_ui::ui_video::stdlib_h::atoi;
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
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Reset;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
use crate::stdlib::memset;
use crate::stdlib::strchr;
use crate::stdlib::strlen;
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
pub use crate::ui_local_h::menuslider_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::uiStatic_t;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphicsoptions_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub graphics: crate::ui_local_h::menutext_s,
    pub display: crate::ui_local_h::menutext_s,
    pub sound: crate::ui_local_h::menutext_s,
    pub network: crate::ui_local_h::menutext_s,
    pub list: crate::ui_local_h::menulist_s,
    pub ratio: crate::ui_local_h::menulist_s,
    pub mode: crate::ui_local_h::menulist_s,
    pub driver: crate::ui_local_h::menulist_s,
    pub tq: crate::ui_local_h::menuslider_s,
    pub fs: crate::ui_local_h::menulist_s,
    pub lighting: crate::ui_local_h::menulist_s,
    pub allow_extensions: crate::ui_local_h::menulist_s,
    pub texturebits: crate::ui_local_h::menulist_s,
    pub colordepth: crate::ui_local_h::menulist_s,
    pub geometry: crate::ui_local_h::menulist_s,
    pub filter: crate::ui_local_h::menulist_s,
    pub driverinfo: crate::ui_local_h::menutext_s,
    pub apply: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InitialVideoOptions_s {
    pub mode: i32,
    pub fullscreen: crate::src::qcommon::q_shared::qboolean,
    pub tq: i32,
    pub lighting: i32,
    pub colordepth: i32,
    pub texturebits: i32,
    pub geometry: i32,
    pub filter: i32,
    pub driver: i32,
    pub extensions: crate::src::qcommon::q_shared::qboolean,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct driverinfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub stringbuff: [i8; 1024],
    pub strings: [*mut i8; 64],
    pub numstrings: i32,
}

static mut driverinfo_artlist: [*mut i8; 5] = [
    b"menu/art/frame2_l\x00" as *const u8 as *mut i8,
    b"menu/art/frame1_r\x00" as *const u8 as *mut i8,
    b"menu/art/back_0\x00" as *const u8 as *mut i8,
    b"menu/art/back_1\x00" as *const u8 as *mut i8,
    0 as *mut i8,
];

static mut s_driverinfo: driverinfo_t = driverinfo_t {
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
    framel: crate::ui_local_h::menubitmap_s {
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
    framer: crate::ui_local_h::menubitmap_s {
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
    stringbuff: [0; 1024],
    strings: [0 as *mut i8; 64],
    numstrings: 0,
};
/*
=================
DriverInfo_Event
=================
*/

unsafe extern "C" fn DriverInfo_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        100 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        _ => {}
    };
}
/*
=================
DriverInfo_MenuDraw
=================
*/

unsafe extern "C" fn DriverInfo_MenuDraw() {
    let mut i: i32 = 0;
    let mut y: i32 = 0;
    crate::src::q3_ui::ui_qmenu::Menu_Draw(&mut s_driverinfo.menu);
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320,
        80,
        b"VENDOR\x00" as *const u8 as *const i8,
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320,
        152,
        b"PIXELFORMAT\x00" as *const u8 as *const i8,
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320,
        192,
        b"EXTENSIONS\x00" as *const u8 as *const i8,
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320,
        80 + 16,
        crate::src::q3_ui::ui_atoms::uis
            .glconfig
            .vendor_string
            .as_mut_ptr(),
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320,
        96 + 16,
        crate::src::q3_ui::ui_atoms::uis
            .glconfig
            .version_string
            .as_mut_ptr(),
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320,
        112 + 16,
        crate::src::q3_ui::ui_atoms::uis
            .glconfig
            .renderer_string
            .as_mut_ptr(),
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
    );
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320,
        152 + 16,
        crate::src::qcommon::q_shared::va(
            b"color(%d-bits) Z(%d-bits) stencil(%d-bits)\x00" as *const u8 as *mut i8,
            crate::src::q3_ui::ui_atoms::uis.glconfig.colorBits,
            crate::src::q3_ui::ui_atoms::uis.glconfig.depthBits,
            crate::src::q3_ui::ui_atoms::uis.glconfig.stencilBits,
        ),
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
    );
    // double column
    y = 192 + 16;

    for i in 0..s_driverinfo.numstrings / 2 {
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            320 - 4,
            y,
            s_driverinfo.strings[(i * 2) as usize],
            0x2 | 0x10,
            crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
        );

        crate::src::q3_ui::ui_atoms::UI_DrawString(
            320 + 4,
            y,
            s_driverinfo.strings[(i * 2 + 1) as usize],
            0 | 0x10,
            crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
        );

        y += 16;
    }
    if s_driverinfo.numstrings & 1 != 0 {
        crate::src::q3_ui::ui_atoms::UI_DrawString(
            320i32,
            y,
            s_driverinfo.strings[(s_driverinfo.numstrings - 1i32) as usize],
            0x1i32 | 0x10i32,
            crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr(),
        );
    };
}
/*
=================
DriverInfo_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn DriverInfo_Cache() {
    let mut i: i32 = 0;
    // touch all our pics
    i = 0;
    while !driverinfo_artlist[i as usize].is_null() {
        crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(driverinfo_artlist[i as usize]);
        i += 1
    }
}
/*
=================
UI_DriverInfo_Menu
=================
*/

unsafe extern "C" fn UI_DriverInfo_Menu() {
    let mut eptr: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_driverinfo as *mut driverinfo_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<driverinfo_t>(),
    );
    DriverInfo_Cache();
    s_driverinfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_driverinfo.menu.draw = Some(DriverInfo_MenuDraw as unsafe extern "C" fn() -> ());
    s_driverinfo.banner.generic.type_0 = 10;
    s_driverinfo.banner.generic.x = 320;
    s_driverinfo.banner.generic.y = 16;
    s_driverinfo.banner.string = b"DRIVER INFO\x00" as *const u8 as *mut i8;
    s_driverinfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_driverinfo.banner.style = 0x1;
    s_driverinfo.framel.generic.type_0 = 6;
    s_driverinfo.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const i8;
    s_driverinfo.framel.generic.flags = 0x4000;
    s_driverinfo.framel.generic.x = 0;
    s_driverinfo.framel.generic.y = 78;
    s_driverinfo.framel.width = 256;
    s_driverinfo.framel.height = 329;
    s_driverinfo.framer.generic.type_0 = 6;
    s_driverinfo.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const i8;
    s_driverinfo.framer.generic.flags = 0x4000;
    s_driverinfo.framer.generic.x = 376;
    s_driverinfo.framer.generic.y = 76;
    s_driverinfo.framer.width = 256;
    s_driverinfo.framer.height = 334;
    s_driverinfo.back.generic.type_0 = 6;
    s_driverinfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    s_driverinfo.back.generic.flags = 0x4 | 0x100;
    s_driverinfo.back.generic.callback =
        Some(DriverInfo_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_driverinfo.back.generic.id = 100;
    s_driverinfo.back.generic.x = 0;
    s_driverinfo.back.generic.y = 480 - 64;
    s_driverinfo.back.width = 128;
    s_driverinfo.back.height = 64;
    s_driverinfo.back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    // TTimo: overflow with particularly long GL extensions (such as the gf3)
    // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=399
    // NOTE: could have pushed the size of stringbuff, but the list is already out of the screen
    // (no matter what your resolution)
    crate::src::qcommon::q_shared::Q_strncpyz(
        s_driverinfo.stringbuff.as_mut_ptr(),
        crate::src::q3_ui::ui_atoms::uis
            .glconfig
            .extensions_string
            .as_mut_ptr(),
        1024,
    );
    // build null terminated extension strings
    eptr = s_driverinfo.stringbuff.as_mut_ptr();
    while s_driverinfo.numstrings < 40 && *eptr as i32 != 0 {
        while *eptr as i32 != 0 && *eptr as i32 == ' ' as i32 {
            let fresh0 = eptr;
            eptr = eptr.offset(1);
            *fresh0 = '\u{0}' as i8
        }
        // track start of valid string
        if *eptr as i32 != 0 && *eptr as i32 != ' ' as i32 {
            let fresh1 = s_driverinfo.numstrings;
            s_driverinfo.numstrings = s_driverinfo.numstrings + 1;
            s_driverinfo.strings[fresh1 as usize] = eptr
        }
        while *eptr as i32 != 0 && *eptr as i32 != ' ' as i32 {
            eptr = eptr.offset(1)
        }
    }

    for i in 0..s_driverinfo.numstrings {
        len = crate::stdlib::strlen(s_driverinfo.strings[i as usize]) as i32;

        if len > 32 {
            *s_driverinfo.strings[i as usize].offset((len - 1) as isize) = '>' as i8;
            *s_driverinfo.strings[i as usize].offset(len as isize) = '\u{0}' as i8
        }
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_driverinfo.menu,
        &mut s_driverinfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_driverinfo.menu,
        &mut s_driverinfo.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_driverinfo.menu,
        &mut s_driverinfo.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_driverinfo.menu,
        &mut s_driverinfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut s_driverinfo.menu);
}

static mut s_ivo: InitialVideoOptions_s = InitialVideoOptions_s {
    mode: 0,
    fullscreen: crate::src::qcommon::q_shared::qfalse,
    tq: 0,
    lighting: 0,
    colordepth: 0,
    texturebits: 0,
    geometry: 0,
    filter: 0,
    driver: 0,
    extensions: crate::src::qcommon::q_shared::qfalse,
};

static mut s_graphicsoptions: graphicsoptions_t = graphicsoptions_t {
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
    framel: crate::ui_local_h::menubitmap_s {
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
    framer: crate::ui_local_h::menubitmap_s {
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
    graphics: crate::ui_local_h::menutext_s {
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
    display: crate::ui_local_h::menutext_s {
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
    sound: crate::ui_local_h::menutext_s {
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
    network: crate::ui_local_h::menutext_s {
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
    ratio: crate::ui_local_h::menulist_s {
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
    mode: crate::ui_local_h::menulist_s {
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
    driver: crate::ui_local_h::menulist_s {
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
    tq: crate::ui_local_h::menuslider_s {
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
        minvalue: 0.,
        maxvalue: 0.,
        curvalue: 0.,
        range: 0.,
    },
    fs: crate::ui_local_h::menulist_s {
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
    lighting: crate::ui_local_h::menulist_s {
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
    allow_extensions: crate::ui_local_h::menulist_s {
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
    texturebits: crate::ui_local_h::menulist_s {
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
    colordepth: crate::ui_local_h::menulist_s {
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
    geometry: crate::ui_local_h::menulist_s {
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
    filter: crate::ui_local_h::menulist_s {
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
    driverinfo: crate::ui_local_h::menutext_s {
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
    apply: crate::ui_local_h::menubitmap_s {
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
};

static mut s_ivo_templates: [InitialVideoOptions_s; 6] = [
    {
        let mut init = InitialVideoOptions_s {
            mode: 6,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 3,
            lighting: 0,
            colordepth: 2,
            texturebits: 2,
            geometry: 2,
            filter: 1,
            driver: 0,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
    {
        let mut init = InitialVideoOptions_s {
            mode: 4,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 2,
            lighting: 0,
            colordepth: 2,
            texturebits: 2,
            geometry: 1,
            filter: 1,
            driver: 0,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
    {
        let mut init = InitialVideoOptions_s {
            mode: 3,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 2,
            lighting: 0,
            colordepth: 0,
            texturebits: 0,
            geometry: 1,
            filter: 0,
            driver: 0,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
    {
        let mut init = InitialVideoOptions_s {
            mode: 2,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 1,
            lighting: 0,
            colordepth: 1,
            texturebits: 0,
            geometry: 0,
            filter: 0,
            driver: 0,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
    {
        let mut init = InitialVideoOptions_s {
            mode: 2,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 1,
            lighting: 1,
            colordepth: 1,
            texturebits: 0,
            geometry: 0,
            filter: 0,
            driver: 0,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
    {
        let mut init = InitialVideoOptions_s {
            mode: 3,
            fullscreen: crate::src::qcommon::q_shared::qtrue,
            tq: 1,
            lighting: 0,
            colordepth: 0,
            texturebits: 0,
            geometry: 1,
            filter: 0,
            driver: 0,
            extensions: crate::src::qcommon::q_shared::qtrue,
        };
        init
    },
];

static mut builtinResolutions: [*const i8; 13] = [
    b"320x240\x00" as *const u8 as *const i8,
    b"400x300\x00" as *const u8 as *const i8,
    b"512x384\x00" as *const u8 as *const i8,
    b"640x480\x00" as *const u8 as *const i8,
    b"800x600\x00" as *const u8 as *const i8,
    b"960x720\x00" as *const u8 as *const i8,
    b"1024x768\x00" as *const u8 as *const i8,
    b"1152x864\x00" as *const u8 as *const i8,
    b"1280x1024\x00" as *const u8 as *const i8,
    b"1600x1200\x00" as *const u8 as *const i8,
    b"2048x1536\x00" as *const u8 as *const i8,
    b"856x480\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut knownRatios: [[*const i8; 2]; 8] = [
    [
        b"1.25:1\x00" as *const u8 as *const i8,
        b"5:4\x00" as *const u8 as *const i8,
    ],
    [
        b"1.33:1\x00" as *const u8 as *const i8,
        b"4:3\x00" as *const u8 as *const i8,
    ],
    [
        b"1.50:1\x00" as *const u8 as *const i8,
        b"3:2\x00" as *const u8 as *const i8,
    ],
    [
        b"1.56:1\x00" as *const u8 as *const i8,
        b"14:9\x00" as *const u8 as *const i8,
    ],
    [
        b"1.60:1\x00" as *const u8 as *const i8,
        b"16:10\x00" as *const u8 as *const i8,
    ],
    [
        b"1.67:1\x00" as *const u8 as *const i8,
        b"5:3\x00" as *const u8 as *const i8,
    ],
    [
        b"1.78:1\x00" as *const u8 as *const i8,
        b"16:9\x00" as *const u8 as *const i8,
    ],
    [0 as *const i8, 0 as *const i8],
];

static mut ratios: [*const i8; 32] = [0 as *const i8; 32];

static mut ratioBuf: [[i8; 8]; 32] = [[0; 8]; 32];

static mut ratioToRes: [i32; 32] = [0; 32];

static mut resToRatio: [i32; 32] = [0; 32];

static mut resbuf: [i8; 1024] = [0; 1024];

static mut detectedResolutions: [*const i8; 32] = [0 as *const i8; 32];

static mut resolutions: *mut *const i8 = unsafe { builtinResolutions.as_ptr() as *mut _ };

static mut resolutionsDetected: crate::src::qcommon::q_shared::qboolean =
    crate::src::qcommon::q_shared::qfalse;
/*
=================
GraphicsOptions_FindBuiltinResolution
=================
*/

unsafe extern "C" fn GraphicsOptions_FindBuiltinResolution(mut mode: i32) -> i32 {
    let mut i: i32 = 0;
    if resolutionsDetected as u64 == 0 {
        return mode;
    }
    if mode < 0 {
        return -(1i32);
    }
    i = 0;
    while !builtinResolutions[i as usize].is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(
            builtinResolutions[i as usize],
            detectedResolutions[mode as usize],
        ) == 0
        {
            return i;
        }
        i += 1
    }
    return -(1);
}
/*
=================
GraphicsOptions_FindDetectedResolution
=================
*/

unsafe extern "C" fn GraphicsOptions_FindDetectedResolution(mut mode: i32) -> i32 {
    let mut i: i32 = 0;
    if resolutionsDetected as u64 == 0 {
        return mode;
    }
    if mode < 0 {
        return -(1i32);
    }
    i = 0;
    while !detectedResolutions[i as usize].is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(
            builtinResolutions[mode as usize],
            detectedResolutions[i as usize],
        ) == 0
        {
            return i;
        }
        i += 1
    }
    return -(1);
}
/*
=================
GraphicsOptions_GetAspectRatios
=================
*/

unsafe extern "C" fn GraphicsOptions_GetAspectRatios() {
    let mut i: i32 = 0;
    let mut r: i32 = 0;
    // build ratio list from resolutions
    r = 0;
    while !(*resolutions.offset(r as isize)).is_null() {
        let mut w: i32 = 0;
        let mut h: i32 = 0;
        let mut x: *mut i8 = 0 as *mut i8;
        let mut str: [i8; 8] = [0; 8];
        // calculate resolution's aspect ratio
        x = crate::stdlib::strchr(*resolutions.offset(r as isize), 'x' as i32).offset(1);
        crate::src::qcommon::q_shared::Q_strncpyz(
            str.as_mut_ptr(),
            *resolutions.offset(r as isize),
            x.wrapping_offset_from(*resolutions.offset(r as isize)) as i32,
        );
        w = atoi(str.as_mut_ptr());
        h = atoi(x);
        crate::src::qcommon::q_shared::Com_sprintf(
            str.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 8]>() as i32,
            b"%.2f:1\x00" as *const u8 as *const i8,
            (w as f32 / h as f32) as f64,
        );
        // rename common ratios ("1.33:1" -> "4:3")
        i = 0;
        while !knownRatios[i as usize][0].is_null() {
            if crate::src::qcommon::q_shared::Q_stricmp(
                str.as_mut_ptr(),
                knownRatios[i as usize][0],
            ) == 0
            {
                crate::src::qcommon::q_shared::Q_strncpyz(
                    str.as_mut_ptr(),
                    knownRatios[i as usize][1],
                    ::std::mem::size_of::<[i8; 8]>() as i32,
                );
                break;
            } else {
                i += 1
            }
        }
        // add ratio to list if it is new
        // establish res/ratio relationship
        i = 0;
        while ratioBuf[i as usize][0] != 0 {
            if crate::src::qcommon::q_shared::Q_stricmp(
                str.as_mut_ptr(),
                ratioBuf[i as usize].as_mut_ptr(),
            ) == 0
            {
                break;
            }
            i += 1
        }
        if ratioBuf[i as usize][0] == 0 {
            crate::src::qcommon::q_shared::Q_strncpyz(
                ratioBuf[i as usize].as_mut_ptr(),
                str.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 8]>() as i32,
            );
            ratioToRes[i as usize] = r
        }
        ratios[r as usize] = ratioBuf[r as usize].as_mut_ptr();
        resToRatio[r as usize] = i;
        r += 1
    }
    ratios[r as usize] = 0 as *const i8;
}
/*
=================
GraphicsOptions_GetInitialVideo
=================
*/

unsafe extern "C" fn GraphicsOptions_GetInitialVideo() {
    s_ivo.colordepth = s_graphicsoptions.colordepth.curvalue;
    s_ivo.driver = s_graphicsoptions.driver.curvalue;
    s_ivo.mode = s_graphicsoptions.mode.curvalue;
    s_ivo.fullscreen = s_graphicsoptions.fs.curvalue as crate::src::qcommon::q_shared::qboolean;
    s_ivo.extensions =
        s_graphicsoptions.allow_extensions.curvalue as crate::src::qcommon::q_shared::qboolean;
    s_ivo.tq = s_graphicsoptions.tq.curvalue as i32;
    s_ivo.lighting = s_graphicsoptions.lighting.curvalue;
    s_ivo.geometry = s_graphicsoptions.geometry.curvalue;
    s_ivo.filter = s_graphicsoptions.filter.curvalue;
    s_ivo.texturebits = s_graphicsoptions.texturebits.curvalue;
}
/*
=================
GraphicsOptions_GetResolutions
=================
*/

unsafe extern "C" fn GraphicsOptions_GetResolutions() {
    crate::src::qcommon::q_shared::Q_strncpyz(
        resbuf.as_mut_ptr(),
        crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString(
            b"r_availableModes\x00" as *const u8 as *const i8,
        ),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if *resbuf.as_mut_ptr() != 0 {
        let mut s: *mut i8 = resbuf.as_mut_ptr();
        let mut i: u32 = 0;
        while !s.is_null()
            && (i as usize)
                < (::std::mem::size_of::<[*const i8; 32]>())
                    .wrapping_div(::std::mem::size_of::<*const i8>())
                    .wrapping_sub(1usize)
        {
            let fresh2 = i;
            i = i.wrapping_add(1);
            detectedResolutions[fresh2 as usize] = s;
            s = crate::stdlib::strchr(s, ' ' as i32);
            if !s.is_null() {
                let fresh3 = s;
                s = s.offset(1);
                *fresh3 = '\u{0}' as i8
            }
        }
        detectedResolutions[i as usize] = 0 as *const i8;
        if i > 0 {
            resolutions = detectedResolutions.as_mut_ptr();
            resolutionsDetected = crate::src::qcommon::q_shared::qtrue
        }
    };
}
/*
=================
GraphicsOptions_CheckConfig
=================
*/

unsafe extern "C" fn GraphicsOptions_CheckConfig() {
    let mut i: i32 = 0;
    i = 0;
    while (i as usize)
        < (::std::mem::size_of::<[InitialVideoOptions_s; 6]>())
            .wrapping_div(::std::mem::size_of::<InitialVideoOptions_s>())
            .wrapping_sub(1usize)
    {
        if !(s_ivo_templates[i as usize].colordepth != s_graphicsoptions.colordepth.curvalue) {
            if !(s_ivo_templates[i as usize].driver != s_graphicsoptions.driver.curvalue) {
                if !(GraphicsOptions_FindDetectedResolution(s_ivo_templates[i as usize].mode)
                    != s_graphicsoptions.mode.curvalue)
                {
                    if !(s_ivo_templates[i as usize].fullscreen
                        != s_graphicsoptions.fs.curvalue as u32)
                    {
                        if !(s_ivo_templates[i as usize].tq as f32 != s_graphicsoptions.tq.curvalue)
                        {
                            if !(s_ivo_templates[i as usize].lighting
                                != s_graphicsoptions.lighting.curvalue)
                            {
                                if !(s_ivo_templates[i as usize].geometry
                                    != s_graphicsoptions.geometry.curvalue)
                                {
                                    if !(s_ivo_templates[i as usize].filter
                                        != s_graphicsoptions.filter.curvalue)
                                    {
                                        //		if ( s_ivo_templates[i].texturebits != s_graphicsoptions.texturebits.curvalue )
                                        //			continue;
                                        s_graphicsoptions.list.curvalue = i;
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    // return 'Custom' ivo template
    s_graphicsoptions.list.curvalue = (::std::mem::size_of::<[InitialVideoOptions_s; 6]>())
        .wrapping_div(::std::mem::size_of::<InitialVideoOptions_s>())
        .wrapping_sub(1usize) as i32;
}
/*
=================
GraphicsOptions_UpdateMenuItems
=================
*/

unsafe extern "C" fn GraphicsOptions_UpdateMenuItems() {
    if s_graphicsoptions.driver.curvalue == 1 {
        s_graphicsoptions.fs.curvalue = 1;
        s_graphicsoptions.fs.generic.flags |= 0x2000;
        s_graphicsoptions.colordepth.curvalue = 1
    } else {
        s_graphicsoptions.fs.generic.flags &= !(0x2000)
    }
    if s_graphicsoptions.fs.curvalue == 0 || s_graphicsoptions.driver.curvalue == 1 {
        s_graphicsoptions.colordepth.curvalue = 0;
        s_graphicsoptions.colordepth.generic.flags |= 0x2000
    } else {
        s_graphicsoptions.colordepth.generic.flags &= !(0x2000)
    }
    if s_graphicsoptions.allow_extensions.curvalue == 0 {
        if s_graphicsoptions.texturebits.curvalue == 0 {
            s_graphicsoptions.texturebits.curvalue = 1
        }
    }
    s_graphicsoptions.apply.generic.flags |= 0x1000 | 0x4000;
    if s_ivo.mode != s_graphicsoptions.mode.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 | 0x4000)
    }
    if s_ivo.fullscreen != s_graphicsoptions.fs.curvalue as u32 {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 | 0x4000)
    }
    if s_ivo.extensions != s_graphicsoptions.allow_extensions.curvalue as u32 {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 | 0x4000)
    }
    if s_ivo.tq as f32 != s_graphicsoptions.tq.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 | 0x4000)
    }
    if s_ivo.lighting != s_graphicsoptions.lighting.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 | 0x4000)
    }
    if s_ivo.colordepth != s_graphicsoptions.colordepth.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 | 0x4000)
    }
    if s_ivo.driver != s_graphicsoptions.driver.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 | 0x4000)
    }
    if s_ivo.texturebits != s_graphicsoptions.texturebits.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 | 0x4000)
    }
    if s_ivo.geometry != s_graphicsoptions.geometry.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 | 0x4000)
    }
    if s_ivo.filter != s_graphicsoptions.filter.curvalue {
        s_graphicsoptions.apply.generic.flags &= !(0x1000 | 0x4000)
    }
    GraphicsOptions_CheckConfig();
}
/*
=================
GraphicsOptions_ApplyChanges
=================
*/

unsafe extern "C" fn GraphicsOptions_ApplyChanges(
    mut unused: *mut libc::c_void,
    mut notification: i32,
) {
    if notification != 3 {
        return;
    }
    match s_graphicsoptions.texturebits.curvalue {
        0 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_texturebits\x00" as *const u8 as *const i8,
                0f32,
            );
        }
        1 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_texturebits\x00" as *const u8 as *const i8,
                16f32,
            );
        }
        2 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_texturebits\x00" as *const u8 as *const i8,
                32f32,
            );
        }
        _ => {}
    }
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"r_picmip\x00" as *const u8 as *const i8,
        3f32 - s_graphicsoptions.tq.curvalue,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"r_allowExtensions\x00" as *const u8 as *const i8,
        s_graphicsoptions.allow_extensions.curvalue as f32,
    );
    if resolutionsDetected as u64 != 0 {
        // search for builtin mode that matches the detected mode
        let mut mode: i32 = 0;
        if s_graphicsoptions.mode.curvalue == -(1)
            || s_graphicsoptions.mode.curvalue as usize
                >= (::std::mem::size_of::<[*const i8; 32]>())
                    .wrapping_div(::std::mem::size_of::<*const i8>())
        {
            s_graphicsoptions.mode.curvalue = 0
        }
        mode = GraphicsOptions_FindBuiltinResolution(s_graphicsoptions.mode.curvalue);
        if mode == -(1) {
            let mut w: [i8; 16] = [0; 16];
            let mut h: [i8; 16] = [0; 16];
            crate::src::qcommon::q_shared::Q_strncpyz(
                w.as_mut_ptr(),
                detectedResolutions[s_graphicsoptions.mode.curvalue as usize],
                ::std::mem::size_of::<[i8; 16]>() as i32,
            );
            *crate::stdlib::strchr(w.as_mut_ptr(), 'x' as i32) = 0;
            crate::src::qcommon::q_shared::Q_strncpyz(
                h.as_mut_ptr(),
                crate::stdlib::strchr(
                    detectedResolutions[s_graphicsoptions.mode.curvalue as usize],
                    'x' as i32,
                )
                .offset(1),
                ::std::mem::size_of::<[i8; 16]>() as i32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_Set(
                b"r_customwidth\x00" as *const u8 as *const i8,
                w.as_mut_ptr(),
            );
            crate::src::ui::ui_syscalls::trap_Cvar_Set(
                b"r_customheight\x00" as *const u8 as *const i8,
                h.as_mut_ptr(),
            );
        }
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_mode\x00" as *const u8 as *const i8,
            mode as f32,
        );
    } else {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_mode\x00" as *const u8 as *const i8,
            s_graphicsoptions.mode.curvalue as f32,
        );
    }
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"r_fullscreen\x00" as *const u8 as *const i8,
        s_graphicsoptions.fs.curvalue as f32,
    );
    match s_graphicsoptions.colordepth.curvalue {
        0 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_colorbits\x00" as *const u8 as *const i8,
                0f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_depthbits\x00" as *const u8 as *const i8,
                0f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_Reset(
                b"r_stencilbits\x00" as *const u8 as *const i8,
            );
        }
        1 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_colorbits\x00" as *const u8 as *const i8,
                16f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_depthbits\x00" as *const u8 as *const i8,
                16f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_stencilbits\x00" as *const u8 as *const i8,
                0f32,
            );
        }
        2 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_colorbits\x00" as *const u8 as *const i8,
                32f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_depthbits\x00" as *const u8 as *const i8,
                24f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"r_stencilbits\x00" as *const u8 as *const i8,
                8f32,
            );
        }
        _ => {}
    }
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"r_vertexLight\x00" as *const u8 as *const i8,
        s_graphicsoptions.lighting.curvalue as f32,
    );
    if s_graphicsoptions.geometry.curvalue == 2 {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_lodBias\x00" as *const u8 as *const i8,
            0f32,
        );
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_subdivisions\x00" as *const u8 as *const i8,
            4f32,
        );
    } else if s_graphicsoptions.geometry.curvalue == 1 {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_lodBias\x00" as *const u8 as *const i8,
            1f32,
        );
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_subdivisions\x00" as *const u8 as *const i8,
            12f32,
        );
    } else {
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_lodBias\x00" as *const u8 as *const i8,
            1f32,
        );
        crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
            b"r_subdivisions\x00" as *const u8 as *const i8,
            20f32,
        );
    }
    if s_graphicsoptions.filter.curvalue != 0 {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            b"r_textureMode\x00" as *const u8 as *const i8,
            b"GL_LINEAR_MIPMAP_LINEAR\x00" as *const u8 as *const i8,
        );
    } else {
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            b"r_textureMode\x00" as *const u8 as *const i8,
            b"GL_LINEAR_MIPMAP_NEAREST\x00" as *const u8 as *const i8,
        );
    }
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
        b"vid_restart\n\x00" as *const u8 as *const i8,
    );
}
/*
=================
GraphicsOptions_Event
=================
*/

unsafe extern "C" fn GraphicsOptions_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut ivo: *mut InitialVideoOptions_s = 0 as *mut InitialVideoOptions_s;
    if event != 3 {
        return;
    }
    let mut current_block_28: u64;
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        110 => {
            s_graphicsoptions.mode.curvalue = ratioToRes[s_graphicsoptions.ratio.curvalue as usize];
            current_block_28 = 6581794399938936269;
        }
        104 => {
            current_block_28 = 6581794399938936269;
        }
        103 => {
            ivo = &mut *s_ivo_templates
                .as_mut_ptr()
                .offset(s_graphicsoptions.list.curvalue as isize)
                as *mut InitialVideoOptions_s;
            s_graphicsoptions.mode.curvalue = GraphicsOptions_FindDetectedResolution((*ivo).mode);
            s_graphicsoptions.ratio.curvalue = resToRatio[s_graphicsoptions.mode.curvalue as usize];
            s_graphicsoptions.tq.curvalue = (*ivo).tq as f32;
            s_graphicsoptions.lighting.curvalue = (*ivo).lighting;
            s_graphicsoptions.colordepth.curvalue = (*ivo).colordepth;
            s_graphicsoptions.texturebits.curvalue = (*ivo).texturebits;
            s_graphicsoptions.geometry.curvalue = (*ivo).geometry;
            s_graphicsoptions.filter.curvalue = (*ivo).filter;
            s_graphicsoptions.fs.curvalue = (*ivo).fullscreen as i32;
            current_block_28 = 652864300344834934;
        }
        105 => {
            UI_DriverInfo_Menu();
            current_block_28 = 652864300344834934;
        }
        101 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            current_block_28 = 652864300344834934;
        }
        107 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_display::UI_DisplayOptionsMenu();
            current_block_28 = 652864300344834934;
        }
        108 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_sound::UI_SoundOptionsMenu();
            current_block_28 = 652864300344834934;
        }
        109 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
            crate::src::q3_ui::ui_network::UI_NetworkOptionsMenu();
            current_block_28 = 652864300344834934;
        }
        106 | _ => {
            current_block_28 = 652864300344834934;
        }
    }
    match current_block_28 {
        6581794399938936269 =>
        // fall through to apply mode constraints
        // clamp 3dfx video modes
        {
            if s_graphicsoptions.driver.curvalue == 1 {
                if s_graphicsoptions.mode.curvalue < 2 {
                    s_graphicsoptions.mode.curvalue = 2
                } else if s_graphicsoptions.mode.curvalue > 6 {
                    s_graphicsoptions.mode.curvalue = 6
                }
            }
            s_graphicsoptions.ratio.curvalue = resToRatio[s_graphicsoptions.mode.curvalue as usize]
        }
        _ => {}
    };
}
/*
================
GraphicsOptions_TQEvent
================
*/

unsafe extern "C" fn GraphicsOptions_TQEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    s_graphicsoptions.tq.curvalue = (s_graphicsoptions.tq.curvalue as f64 + 0.5) as i32 as f32;
}
/*
================
GraphicsOptions_MenuDraw
================
*/
#[no_mangle]

pub unsafe extern "C" fn GraphicsOptions_MenuDraw() {
    //APSFIX - rework this
    GraphicsOptions_UpdateMenuItems();
    crate::src::q3_ui::ui_qmenu::Menu_Draw(&mut s_graphicsoptions.menu);
}
/*
=================
GraphicsOptions_SetMenuItems
=================
*/

unsafe extern "C" fn GraphicsOptions_SetMenuItems() {
    s_graphicsoptions.mode.curvalue = GraphicsOptions_FindDetectedResolution(
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"r_mode\x00" as *const u8 as *const i8,
        ) as i32,
    );
    if s_graphicsoptions.mode.curvalue < 0 {
        if resolutionsDetected as u64 != 0 {
            let mut i: i32 = 0;
            let mut buf: [i8; 1024] = [0; 1024];
            crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
                b"r_customwidth\x00" as *const u8 as *const i8,
                buf.as_mut_ptr(),
                (::std::mem::size_of::<[i8; 1024]>()).wrapping_sub(2usize) as i32,
            );
            buf[crate::stdlib::strlen(buf.as_mut_ptr()).wrapping_add(1usize)] = 0;
            buf[crate::stdlib::strlen(buf.as_mut_ptr())] = 'x' as i8;
            crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
                b"r_customheight\x00" as *const u8 as *const i8,
                buf.as_mut_ptr()
                    .offset(crate::stdlib::strlen(buf.as_mut_ptr()) as isize),
                (::std::mem::size_of::<[i8; 1024]>())
                    .wrapping_sub(crate::stdlib::strlen(buf.as_mut_ptr())) as i32,
            );
            i = 0;
            while !detectedResolutions[i as usize].is_null() {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    buf.as_mut_ptr(),
                    detectedResolutions[i as usize],
                ) == 0
                {
                    s_graphicsoptions.mode.curvalue = i;
                    break;
                } else {
                    i += 1
                }
            }
            if s_graphicsoptions.mode.curvalue < 0 {
                s_graphicsoptions.mode.curvalue = 0
            }
        } else {
            s_graphicsoptions.mode.curvalue = 3
        }
    }
    s_graphicsoptions.ratio.curvalue = resToRatio[s_graphicsoptions.mode.curvalue as usize];
    s_graphicsoptions.fs.curvalue = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_fullscreen\x00" as *const u8 as *const i8,
    ) as i32;
    s_graphicsoptions.allow_extensions.curvalue =
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"r_allowExtensions\x00" as *const u8 as *const i8,
        ) as i32;
    s_graphicsoptions.tq.curvalue = 3f32
        - crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"r_picmip\x00" as *const u8 as *const i8,
        );
    if s_graphicsoptions.tq.curvalue < 0f32 {
        s_graphicsoptions.tq.curvalue = 0f32
    } else if s_graphicsoptions.tq.curvalue > 3f32 {
        s_graphicsoptions.tq.curvalue = 3f32
    }
    s_graphicsoptions.lighting.curvalue = (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_vertexLight\x00" as *const u8 as *const i8,
    ) != 0f32) as i32;
    match crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_texturebits\x00" as *const u8 as *const i8,
    ) as i32
    {
        16 => s_graphicsoptions.texturebits.curvalue = 1,
        32 => s_graphicsoptions.texturebits.curvalue = 2,
        0 | _ => s_graphicsoptions.texturebits.curvalue = 0,
    }
    if crate::src::qcommon::q_shared::Q_stricmp(
        crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString(
            b"r_textureMode\x00" as *const u8 as *const i8,
        ),
        b"GL_LINEAR_MIPMAP_NEAREST\x00" as *const u8 as *const i8,
    ) == 0
    {
        s_graphicsoptions.filter.curvalue = 0
    } else {
        s_graphicsoptions.filter.curvalue = 1
    }
    if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_lodBias\x00" as *const u8 as *const i8,
    ) > 0f32
    {
        if crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"r_subdivisions\x00" as *const u8 as *const i8,
        ) >= 20f32
        {
            s_graphicsoptions.geometry.curvalue = 0
        } else {
            s_graphicsoptions.geometry.curvalue = 1
        }
    } else {
        s_graphicsoptions.geometry.curvalue = 2
    }
    match crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"r_colorbits\x00" as *const u8 as *const i8,
    ) as i32
    {
        16 => s_graphicsoptions.colordepth.curvalue = 1,
        32 => s_graphicsoptions.colordepth.curvalue = 2,
        0 | _ => s_graphicsoptions.colordepth.curvalue = 0,
    }
    if s_graphicsoptions.fs.curvalue == 0 {
        s_graphicsoptions.colordepth.curvalue = 0
    }
    if s_graphicsoptions.driver.curvalue == 1 {
        s_graphicsoptions.colordepth.curvalue = 1
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
/*
================
GraphicsOptions_MenuInit
================
*/
#[no_mangle]

pub unsafe extern "C" fn GraphicsOptions_MenuInit() {
    static mut s_driver_names: [*const i8; 3] = [
        b"Default\x00" as *const u8 as *const i8,
        b"Voodoo\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    static mut tq_names: [*const i8; 4] = [
        b"Default\x00" as *const u8 as *const i8,
        b"16 bit\x00" as *const u8 as *const i8,
        b"32 bit\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    static mut s_graphics_options_names: [*const i8; 7] = [
        b"Very High Quality\x00" as *const u8 as *const i8,
        b"High Quality\x00" as *const u8 as *const i8,
        b"Normal\x00" as *const u8 as *const i8,
        b"Fast\x00" as *const u8 as *const i8,
        b"Fastest\x00" as *const u8 as *const i8,
        b"Custom\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    static mut lighting_names: [*const i8; 3] = [
        b"Lightmap\x00" as *const u8 as *const i8,
        b"Vertex\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    static mut colordepth_names: [*const i8; 4] = [
        b"Default\x00" as *const u8 as *const i8,
        b"16 bit\x00" as *const u8 as *const i8,
        b"32 bit\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    static mut filter_names: [*const i8; 3] = [
        b"Bilinear\x00" as *const u8 as *const i8,
        b"Trilinear\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    static mut quality_names: [*const i8; 4] = [
        b"Low\x00" as *const u8 as *const i8,
        b"Medium\x00" as *const u8 as *const i8,
        b"High\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    static mut enabled_names: [*const i8; 3] = [
        b"Off\x00" as *const u8 as *const i8,
        b"On\x00" as *const u8 as *const i8,
        0 as *const i8,
    ];
    let mut y: i32 = 0;
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_graphicsoptions as *mut graphicsoptions_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<graphicsoptions_t>(),
    );
    GraphicsOptions_GetResolutions();
    GraphicsOptions_GetAspectRatios();
    GraphicsOptions_Cache();
    s_graphicsoptions.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_graphicsoptions.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_graphicsoptions.menu.draw = Some(GraphicsOptions_MenuDraw as unsafe extern "C" fn() -> ());
    s_graphicsoptions.banner.generic.type_0 = 10;
    s_graphicsoptions.banner.generic.x = 320;
    s_graphicsoptions.banner.generic.y = 16;
    s_graphicsoptions.banner.string = b"SYSTEM SETUP\x00" as *const u8 as *mut i8;
    s_graphicsoptions.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_graphicsoptions.banner.style = 0x1;
    s_graphicsoptions.framel.generic.type_0 = 6;
    s_graphicsoptions.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const i8;
    s_graphicsoptions.framel.generic.flags = 0x4000;
    s_graphicsoptions.framel.generic.x = 0;
    s_graphicsoptions.framel.generic.y = 78;
    s_graphicsoptions.framel.width = 256;
    s_graphicsoptions.framel.height = 329;
    s_graphicsoptions.framer.generic.type_0 = 6;
    s_graphicsoptions.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const i8;
    s_graphicsoptions.framer.generic.flags = 0x4000;
    s_graphicsoptions.framer.generic.x = 376;
    s_graphicsoptions.framer.generic.y = 76;
    s_graphicsoptions.framer.width = 256;
    s_graphicsoptions.framer.height = 334;
    s_graphicsoptions.graphics.generic.type_0 = 9;
    s_graphicsoptions.graphics.generic.flags = 0x10;
    s_graphicsoptions.graphics.generic.id = 106;
    s_graphicsoptions.graphics.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.graphics.generic.x = 216;
    s_graphicsoptions.graphics.generic.y = 240 - 2 * 27;
    s_graphicsoptions.graphics.string = b"GRAPHICS\x00" as *const u8 as *mut i8;
    s_graphicsoptions.graphics.style = 0x2;
    s_graphicsoptions.graphics.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_graphicsoptions.display.generic.type_0 = 9;
    s_graphicsoptions.display.generic.flags = 0x10 | 0x100;
    s_graphicsoptions.display.generic.id = 107;
    s_graphicsoptions.display.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.display.generic.x = 216;
    s_graphicsoptions.display.generic.y = 240 - 27;
    s_graphicsoptions.display.string = b"DISPLAY\x00" as *const u8 as *mut i8;
    s_graphicsoptions.display.style = 0x2;
    s_graphicsoptions.display.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_graphicsoptions.sound.generic.type_0 = 9;
    s_graphicsoptions.sound.generic.flags = 0x10 | 0x100;
    s_graphicsoptions.sound.generic.id = 108;
    s_graphicsoptions.sound.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.sound.generic.x = 216;
    s_graphicsoptions.sound.generic.y = 240;
    s_graphicsoptions.sound.string = b"SOUND\x00" as *const u8 as *mut i8;
    s_graphicsoptions.sound.style = 0x2;
    s_graphicsoptions.sound.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_graphicsoptions.network.generic.type_0 = 9;
    s_graphicsoptions.network.generic.flags = 0x10 | 0x100;
    s_graphicsoptions.network.generic.id = 109;
    s_graphicsoptions.network.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.network.generic.x = 216;
    s_graphicsoptions.network.generic.y = 240 + 27;
    s_graphicsoptions.network.string = b"NETWORK\x00" as *const u8 as *mut i8;
    s_graphicsoptions.network.style = 0x2;
    s_graphicsoptions.network.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    y = 240 - 7 * (16 + 2);
    s_graphicsoptions.list.generic.type_0 = 3;
    s_graphicsoptions.list.generic.name = b"Graphics Settings:\x00" as *const u8 as *const i8;
    s_graphicsoptions.list.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.list.generic.x = 400;
    s_graphicsoptions.list.generic.y = y;
    s_graphicsoptions.list.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.list.generic.id = 103;
    s_graphicsoptions.list.itemnames = s_graphics_options_names.as_mut_ptr();
    y += 2 * (16 + 2);
    s_graphicsoptions.driver.generic.type_0 = 3;
    s_graphicsoptions.driver.generic.name = b"GL Driver:\x00" as *const u8 as *const i8;
    s_graphicsoptions.driver.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.driver.generic.x = 400;
    s_graphicsoptions.driver.generic.y = y;
    s_graphicsoptions.driver.itemnames = s_driver_names.as_mut_ptr();
    s_graphicsoptions.driver.curvalue = (crate::src::q3_ui::ui_atoms::uis.glconfig.driverType
        == crate::tr_types_h::GLDRV_VOODOO) as i32;
    y += 16 + 2;
    // references/modifies "r_allowExtensions"
    s_graphicsoptions.allow_extensions.generic.type_0 = 3;
    s_graphicsoptions.allow_extensions.generic.name =
        b"GL Extensions:\x00" as *const u8 as *const i8;
    s_graphicsoptions.allow_extensions.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.allow_extensions.generic.x = 400;
    s_graphicsoptions.allow_extensions.generic.y = y;
    s_graphicsoptions.allow_extensions.itemnames = enabled_names.as_mut_ptr();
    y += 16 + 2;
    s_graphicsoptions.ratio.generic.type_0 = 3;
    s_graphicsoptions.ratio.generic.name = b"Aspect Ratio:\x00" as *const u8 as *const i8;
    s_graphicsoptions.ratio.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.ratio.generic.x = 400;
    s_graphicsoptions.ratio.generic.y = y;
    s_graphicsoptions.ratio.itemnames = ratios.as_mut_ptr();
    s_graphicsoptions.ratio.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.ratio.generic.id = 110;
    y += 16 + 2;
    // references/modifies "r_mode"
    s_graphicsoptions.mode.generic.type_0 = 3;
    s_graphicsoptions.mode.generic.name = b"Resolution:\x00" as *const u8 as *const i8;
    s_graphicsoptions.mode.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.mode.generic.x = 400;
    s_graphicsoptions.mode.generic.y = y;
    s_graphicsoptions.mode.itemnames = resolutions;
    s_graphicsoptions.mode.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.mode.generic.id = 104;
    y += 16 + 2;
    // references "r_colorbits"
    s_graphicsoptions.colordepth.generic.type_0 = 3;
    s_graphicsoptions.colordepth.generic.name = b"Color Depth:\x00" as *const u8 as *const i8;
    s_graphicsoptions.colordepth.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.colordepth.generic.x = 400;
    s_graphicsoptions.colordepth.generic.y = y;
    s_graphicsoptions.colordepth.itemnames = colordepth_names.as_mut_ptr();
    y += 16 + 2;
    // references/modifies "r_fullscreen"
    s_graphicsoptions.fs.generic.type_0 = 3;
    s_graphicsoptions.fs.generic.name = b"Fullscreen:\x00" as *const u8 as *const i8;
    s_graphicsoptions.fs.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.fs.generic.x = 400;
    s_graphicsoptions.fs.generic.y = y;
    s_graphicsoptions.fs.itemnames = enabled_names.as_mut_ptr();
    y += 16 + 2;
    // references/modifies "r_vertexLight"
    s_graphicsoptions.lighting.generic.type_0 = 3;
    s_graphicsoptions.lighting.generic.name = b"Lighting:\x00" as *const u8 as *const i8;
    s_graphicsoptions.lighting.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.lighting.generic.x = 400;
    s_graphicsoptions.lighting.generic.y = y;
    s_graphicsoptions.lighting.itemnames = lighting_names.as_mut_ptr();
    y += 16 + 2;
    // references/modifies "r_lodBias" & "subdivisions"
    s_graphicsoptions.geometry.generic.type_0 = 3;
    s_graphicsoptions.geometry.generic.name = b"Geometric Detail:\x00" as *const u8 as *const i8;
    s_graphicsoptions.geometry.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.geometry.generic.x = 400;
    s_graphicsoptions.geometry.generic.y = y;
    s_graphicsoptions.geometry.itemnames = quality_names.as_mut_ptr();
    y += 16 + 2;
    // references/modifies "r_picmip"
    s_graphicsoptions.tq.generic.type_0 = 1;
    s_graphicsoptions.tq.generic.name = b"Texture Detail:\x00" as *const u8 as *const i8;
    s_graphicsoptions.tq.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.tq.generic.x = 400;
    s_graphicsoptions.tq.generic.y = y;
    s_graphicsoptions.tq.minvalue = 0f32;
    s_graphicsoptions.tq.maxvalue = 3f32;
    s_graphicsoptions.tq.generic.callback =
        Some(GraphicsOptions_TQEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    y += 16 + 2;
    // references/modifies "r_textureBits"
    s_graphicsoptions.texturebits.generic.type_0 = 3;
    s_graphicsoptions.texturebits.generic.name = b"Texture Quality:\x00" as *const u8 as *const i8;
    s_graphicsoptions.texturebits.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.texturebits.generic.x = 400;
    s_graphicsoptions.texturebits.generic.y = y;
    s_graphicsoptions.texturebits.itemnames = tq_names.as_mut_ptr();
    y += 16 + 2;
    // references/modifies "r_textureMode"
    s_graphicsoptions.filter.generic.type_0 = 3;
    s_graphicsoptions.filter.generic.name = b"Texture Filter:\x00" as *const u8 as *const i8;
    s_graphicsoptions.filter.generic.flags = 0x100 | 0x2;
    s_graphicsoptions.filter.generic.x = 400;
    s_graphicsoptions.filter.generic.y = y;
    s_graphicsoptions.filter.itemnames = filter_names.as_mut_ptr();
    y += 2 * 16;
    s_graphicsoptions.driverinfo.generic.type_0 = 9;
    s_graphicsoptions.driverinfo.generic.flags = 0x8 | 0x100;
    s_graphicsoptions.driverinfo.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.driverinfo.generic.id = 105;
    s_graphicsoptions.driverinfo.generic.x = 320;
    s_graphicsoptions.driverinfo.generic.y = y;
    s_graphicsoptions.driverinfo.string = b"Driver Info\x00" as *const u8 as *mut i8;
    s_graphicsoptions.driverinfo.style = 0x1 | 0x10;
    s_graphicsoptions.driverinfo.color = crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr();
    s_graphicsoptions.back.generic.type_0 = 6;
    s_graphicsoptions.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    s_graphicsoptions.back.generic.flags = 0x4 | 0x100;
    s_graphicsoptions.back.generic.callback =
        Some(GraphicsOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_graphicsoptions.back.generic.id = 101;
    s_graphicsoptions.back.generic.x = 0;
    s_graphicsoptions.back.generic.y = 480 - 64;
    s_graphicsoptions.back.width = 128;
    s_graphicsoptions.back.height = 64;
    s_graphicsoptions.back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    s_graphicsoptions.apply.generic.type_0 = 6;
    s_graphicsoptions.apply.generic.name = b"menu/art/accept_0\x00" as *const u8 as *const i8;
    s_graphicsoptions.apply.generic.flags = 0x10 | 0x100 | 0x1000 | 0x4000;
    s_graphicsoptions.apply.generic.callback = Some(
        GraphicsOptions_ApplyChanges as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    s_graphicsoptions.apply.generic.x = 640;
    s_graphicsoptions.apply.generic.y = 480 - 64;
    s_graphicsoptions.apply.width = 128;
    s_graphicsoptions.apply.height = 64;
    s_graphicsoptions.apply.focuspic = b"menu/art/accept_1\x00" as *const u8 as *mut i8;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.graphics as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.display as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.sound as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.network as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.list as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.driver as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.allow_extensions as *mut crate::ui_local_h::menulist_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.ratio as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.mode as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.colordepth as *mut crate::ui_local_h::menulist_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.fs as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.lighting as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.geometry as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.tq as *mut crate::ui_local_h::menuslider_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.texturebits as *mut crate::ui_local_h::menulist_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.filter as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.driverinfo as *mut crate::ui_local_h::menutext_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.apply as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    GraphicsOptions_SetMenuItems();
    GraphicsOptions_GetInitialVideo();
    if crate::src::q3_ui::ui_atoms::uis.glconfig.driverType == crate::tr_types_h::GLDRV_ICD
        && crate::src::q3_ui::ui_atoms::uis.glconfig.hardwareType
            == crate::tr_types_h::GLHW_3DFX_2D3D
    {
        s_graphicsoptions.driver.generic.flags |= 0x1000 | 0x4000
    };
}
/*
=================
GraphicsOptions_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn GraphicsOptions_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame2_l\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame1_r\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/accept_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/accept_1\x00" as *const u8 as *const i8,
    );
}
//
// ui_video.c
//
/*
=================
UI_GraphicsOptionsMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GraphicsOptionsMenu() {
    GraphicsOptions_MenuInit();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut s_graphicsoptions.menu);
    crate::src::q3_ui::ui_qmenu::Menu_SetCursorToItem(
        &mut s_graphicsoptions.menu,
        &mut s_graphicsoptions.graphics as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
}
