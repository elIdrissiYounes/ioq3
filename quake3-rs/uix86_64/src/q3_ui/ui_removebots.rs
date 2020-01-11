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

pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_qmenu::color_orange;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_removebots::stdlib_h::atoi;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_GetConfigString;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
use crate::stdlib::memset;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menutext_s;
pub use ::libc::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct removeBotsMenuInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub background: crate::ui_local_h::menubitmap_s,
    pub arrows: crate::ui_local_h::menubitmap_s,
    pub up: crate::ui_local_h::menubitmap_s,
    pub down: crate::ui_local_h::menubitmap_s,
    pub bots: [crate::ui_local_h::menutext_s; 7],
    pub delete: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub numBots: libc::c_int,
    pub baseBotNum: libc::c_int,
    pub selectedBotNum: libc::c_int,
    pub botnames: [[libc::c_char; 32]; 7],
    pub botClientNums: [libc::c_int; 1024],
}

static mut removeBotsMenuInfo: removeBotsMenuInfo_t = removeBotsMenuInfo_t {
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
    background: crate::ui_local_h::menubitmap_s {
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
    arrows: crate::ui_local_h::menubitmap_s {
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
    up: crate::ui_local_h::menubitmap_s {
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
    down: crate::ui_local_h::menubitmap_s {
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
    bots: [crate::ui_local_h::menutext_s {
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
    }; 7],
    delete: crate::ui_local_h::menubitmap_s {
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
    numBots: 0,
    baseBotNum: 0,
    selectedBotNum: 0,
    botnames: [[0; 32]; 7],
    botClientNums: [0; 1024],
};
/*
=================
UI_RemoveBotsMenu_SetBotNames
=================
*/

unsafe extern "C" fn UI_RemoveBotsMenu_SetBotNames() {
    let mut n: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    n = 0 as libc::c_int;
    while n < 7 as libc::c_int && removeBotsMenuInfo.baseBotNum + n < removeBotsMenuInfo.numBots {
        crate::src::ui::ui_syscalls::trap_GetConfigString(
            32 as libc::c_int
                + 256 as libc::c_int
                + 256 as libc::c_int
                + removeBotsMenuInfo.botClientNums[(removeBotsMenuInfo.baseBotNum + n) as usize],
            info.as_mut_ptr(),
            1024 as libc::c_int,
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            removeBotsMenuInfo.botnames[n as usize].as_mut_ptr(),
            crate::src::qcommon::q_shared::Info_ValueForKey(
                info.as_mut_ptr(),
                b"n\x00" as *const u8 as *const libc::c_char,
            ),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
        );
        crate::src::qcommon::q_shared::Q_CleanStr(
            removeBotsMenuInfo.botnames[n as usize].as_mut_ptr(),
        );
        n += 1
    }
}
/*
=================
UI_RemoveBotsMenu_DeleteEvent
=================
*/

unsafe extern "C" fn UI_RemoveBotsMenu_DeleteEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3 as libc::c_int {
        return;
    }
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as libc::c_int,
        crate::src::qcommon::q_shared::va(
            b"clientkick %i\n\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            removeBotsMenuInfo.botClientNums
                [(removeBotsMenuInfo.baseBotNum + removeBotsMenuInfo.selectedBotNum) as usize],
        ),
    );
}
/*
=================
UI_RemoveBotsMenu_BotEvent
=================
*/

unsafe extern "C" fn UI_RemoveBotsMenu_BotEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3 as libc::c_int {
        return;
    }
    removeBotsMenuInfo.bots[removeBotsMenuInfo.selectedBotNum as usize].color =
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr();
    removeBotsMenuInfo.selectedBotNum =
        (*(ptr as *mut crate::ui_local_h::menucommon_s)).id - 20 as libc::c_int;
    removeBotsMenuInfo.bots[removeBotsMenuInfo.selectedBotNum as usize].color =
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
}
/*
=================
UI_RemoveAddBotsMenu_BackEvent
=================
*/

unsafe extern "C" fn UI_RemoveBotsMenu_BackEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3 as libc::c_int {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
}
/*
=================
UI_RemoveBotsMenu_UpEvent
=================
*/

unsafe extern "C" fn UI_RemoveBotsMenu_UpEvent(mut ptr: *mut libc::c_void, mut event: libc::c_int) {
    if event != 3 as libc::c_int {
        return;
    }
    if removeBotsMenuInfo.baseBotNum > 0 as libc::c_int {
        removeBotsMenuInfo.baseBotNum -= 1;
        UI_RemoveBotsMenu_SetBotNames();
    };
}
/*
=================
UI_RemoveBotsMenu_DownEvent
=================
*/

unsafe extern "C" fn UI_RemoveBotsMenu_DownEvent(
    mut ptr: *mut libc::c_void,
    mut event: libc::c_int,
) {
    if event != 3 as libc::c_int {
        return;
    }
    if (removeBotsMenuInfo.baseBotNum + 7 as libc::c_int) < removeBotsMenuInfo.numBots {
        removeBotsMenuInfo.baseBotNum += 1;
        UI_RemoveBotsMenu_SetBotNames();
    };
}
/*
=================
UI_RemoveBotsMenu_GetBots
=================
*/

unsafe extern "C" fn UI_RemoveBotsMenu_GetBots() {
    let mut numPlayers: libc::c_int = 0;
    let mut isBot: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut info: [libc::c_char; 1024] = [0; 1024];
    crate::src::ui::ui_syscalls::trap_GetConfigString(
        0 as libc::c_int,
        info.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    );
    numPlayers = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info.as_mut_ptr(),
        b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
    ));
    removeBotsMenuInfo.numBots = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n < numPlayers {
        crate::src::ui::ui_syscalls::trap_GetConfigString(
            32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int + n,
            info.as_mut_ptr(),
            1024 as libc::c_int,
        );
        isBot = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info.as_mut_ptr(),
            b"skill\x00" as *const u8 as *const libc::c_char,
        ));
        if !(isBot == 0) {
            removeBotsMenuInfo.botClientNums[removeBotsMenuInfo.numBots as usize] = n;
            removeBotsMenuInfo.numBots += 1
        }
        n += 1
    }
}
/*
=================
UI_RemoveBots_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_RemoveBots_Cache() {
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/delete_0\x00" as *const u8 as *const libc::c_char,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/delete_1\x00" as *const u8 as *const libc::c_char,
    );
}
/*
=================
UI_RemoveBotsMenu_Init
=================
*/

unsafe extern "C" fn UI_RemoveBotsMenu_Init() {
    let mut n: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    crate::stdlib::memset(
        &mut removeBotsMenuInfo as *mut removeBotsMenuInfo_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<removeBotsMenuInfo_t>() as libc::c_ulong,
    );
    removeBotsMenuInfo.menu.fullscreen = crate::src::qcommon::q_shared::qfalse;
    removeBotsMenuInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    UI_RemoveBots_Cache();
    UI_RemoveBotsMenu_GetBots();
    UI_RemoveBotsMenu_SetBotNames();
    count = if removeBotsMenuInfo.numBots < 7 as libc::c_int {
        removeBotsMenuInfo.numBots
    } else {
        7 as libc::c_int
    };
    removeBotsMenuInfo.banner.generic.type_0 = 10 as libc::c_int;
    removeBotsMenuInfo.banner.generic.x = 320 as libc::c_int;
    removeBotsMenuInfo.banner.generic.y = 16 as libc::c_int;
    removeBotsMenuInfo.banner.string =
        b"REMOVE BOTS\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    removeBotsMenuInfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    removeBotsMenuInfo.banner.style = 0x1 as libc::c_int;
    removeBotsMenuInfo.background.generic.type_0 = 6 as libc::c_int;
    removeBotsMenuInfo.background.generic.name =
        b"menu/art/addbotframe\x00" as *const u8 as *const libc::c_char;
    removeBotsMenuInfo.background.generic.flags = 0x4000 as libc::c_int as libc::c_uint;
    removeBotsMenuInfo.background.generic.x = 320 as libc::c_int - 233 as libc::c_int;
    removeBotsMenuInfo.background.generic.y = 240 as libc::c_int - 166 as libc::c_int;
    removeBotsMenuInfo.background.width = 466 as libc::c_int;
    removeBotsMenuInfo.background.height = 332 as libc::c_int;
    removeBotsMenuInfo.arrows.generic.type_0 = 6 as libc::c_int;
    removeBotsMenuInfo.arrows.generic.name =
        b"menu/art/arrows_vert_0\x00" as *const u8 as *const libc::c_char;
    removeBotsMenuInfo.arrows.generic.flags = 0x4000 as libc::c_int as libc::c_uint;
    removeBotsMenuInfo.arrows.generic.x = 200 as libc::c_int;
    removeBotsMenuInfo.arrows.generic.y = 128 as libc::c_int;
    removeBotsMenuInfo.arrows.width = 64 as libc::c_int;
    removeBotsMenuInfo.arrows.height = 128 as libc::c_int;
    removeBotsMenuInfo.up.generic.type_0 = 6 as libc::c_int;
    removeBotsMenuInfo.up.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    removeBotsMenuInfo.up.generic.x = 200 as libc::c_int;
    removeBotsMenuInfo.up.generic.y = 128 as libc::c_int;
    removeBotsMenuInfo.up.generic.id = 10 as libc::c_int;
    removeBotsMenuInfo.up.generic.callback = Some(
        UI_RemoveBotsMenu_UpEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    removeBotsMenuInfo.up.width = 64 as libc::c_int;
    removeBotsMenuInfo.up.height = 64 as libc::c_int;
    removeBotsMenuInfo.up.focuspic =
        b"menu/art/arrows_vert_top\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    removeBotsMenuInfo.down.generic.type_0 = 6 as libc::c_int;
    removeBotsMenuInfo.down.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    removeBotsMenuInfo.down.generic.x = 200 as libc::c_int;
    removeBotsMenuInfo.down.generic.y = 128 as libc::c_int + 64 as libc::c_int;
    removeBotsMenuInfo.down.generic.id = 11 as libc::c_int;
    removeBotsMenuInfo.down.generic.callback = Some(
        UI_RemoveBotsMenu_DownEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    removeBotsMenuInfo.down.width = 64 as libc::c_int;
    removeBotsMenuInfo.down.height = 64 as libc::c_int;
    removeBotsMenuInfo.down.focuspic =
        b"menu/art/arrows_vert_bot\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    n = 0 as libc::c_int;
    y = 120 as libc::c_int;
    while n < count {
        removeBotsMenuInfo.bots[n as usize].generic.type_0 = 9 as libc::c_int;
        removeBotsMenuInfo.bots[n as usize].generic.flags =
            0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
        removeBotsMenuInfo.bots[n as usize].generic.id = 20 as libc::c_int + n;
        removeBotsMenuInfo.bots[n as usize].generic.x = 320 as libc::c_int - 56 as libc::c_int;
        removeBotsMenuInfo.bots[n as usize].generic.y = y;
        removeBotsMenuInfo.bots[n as usize].generic.callback = Some(
            UI_RemoveBotsMenu_BotEvent
                as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
        );
        removeBotsMenuInfo.bots[n as usize].string =
            removeBotsMenuInfo.botnames[n as usize].as_mut_ptr();
        removeBotsMenuInfo.bots[n as usize].color =
            crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr();
        removeBotsMenuInfo.bots[n as usize].style = 0 as libc::c_int | 0x10 as libc::c_int;
        n += 1;
        y += 20 as libc::c_int
    }
    removeBotsMenuInfo.delete.generic.type_0 = 6 as libc::c_int;
    removeBotsMenuInfo.delete.generic.name =
        b"menu/art/delete_0\x00" as *const u8 as *const libc::c_char;
    removeBotsMenuInfo.delete.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    removeBotsMenuInfo.delete.generic.id = 12 as libc::c_int;
    removeBotsMenuInfo.delete.generic.callback = Some(
        UI_RemoveBotsMenu_DeleteEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    removeBotsMenuInfo.delete.generic.x =
        320 as libc::c_int + 128 as libc::c_int - 128 as libc::c_int;
    removeBotsMenuInfo.delete.generic.y =
        256 as libc::c_int + 128 as libc::c_int - 64 as libc::c_int;
    removeBotsMenuInfo.delete.width = 128 as libc::c_int;
    removeBotsMenuInfo.delete.height = 64 as libc::c_int;
    removeBotsMenuInfo.delete.focuspic =
        b"menu/art/delete_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    removeBotsMenuInfo.back.generic.type_0 = 6 as libc::c_int;
    removeBotsMenuInfo.back.generic.name =
        b"menu/art/back_0\x00" as *const u8 as *const libc::c_char;
    removeBotsMenuInfo.back.generic.flags =
        0x4 as libc::c_int as libc::c_uint | 0x100 as libc::c_int as libc::c_uint;
    removeBotsMenuInfo.back.generic.id = 13 as libc::c_int;
    removeBotsMenuInfo.back.generic.callback = Some(
        UI_RemoveBotsMenu_BackEvent
            as unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_int) -> (),
    );
    removeBotsMenuInfo.back.generic.x = 320 as libc::c_int - 128 as libc::c_int;
    removeBotsMenuInfo.back.generic.y = 256 as libc::c_int + 128 as libc::c_int - 64 as libc::c_int;
    removeBotsMenuInfo.back.width = 128 as libc::c_int;
    removeBotsMenuInfo.back.height = 64 as libc::c_int;
    removeBotsMenuInfo.back.focuspic =
        b"menu/art/back_1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut removeBotsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut removeBotsMenuInfo.background as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut removeBotsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut removeBotsMenuInfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut removeBotsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut removeBotsMenuInfo.arrows as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut removeBotsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut removeBotsMenuInfo.up as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut removeBotsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut removeBotsMenuInfo.down as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    n = 0 as libc::c_int;
    while n < count {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut removeBotsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
            &mut *removeBotsMenuInfo.bots.as_mut_ptr().offset(n as isize)
                as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
        );
        n += 1
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut removeBotsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut removeBotsMenuInfo.delete as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut removeBotsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
        &mut removeBotsMenuInfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    removeBotsMenuInfo.baseBotNum = 0 as libc::c_int;
    removeBotsMenuInfo.selectedBotNum = 0 as libc::c_int;
    removeBotsMenuInfo.bots[0 as libc::c_int as usize].color =
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
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
/*
=================
UI_RemoveBotsMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_RemoveBotsMenu() {
    UI_RemoveBotsMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(
        &mut removeBotsMenuInfo.menu as *mut _ as *mut crate::ui_local_h::_tag_menuframework,
    );
}
