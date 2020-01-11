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
pub use crate::src::q3_ui::ui_addbots::stdlib_h::atoi;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetNumBots;
pub use crate::src::q3_ui::ui_qmenu::color_orange;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_GetConfigString;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::stdlib::__compar_fn_t;
use crate::stdlib::memset;
pub use crate::stdlib::qsort;
pub use crate::stdlib::strtol;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menutext_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addBotsMenuInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub background: crate::ui_local_h::menubitmap_s,
    pub arrows: crate::ui_local_h::menubitmap_s,
    pub up: crate::ui_local_h::menubitmap_s,
    pub down: crate::ui_local_h::menubitmap_s,
    pub bots: [crate::ui_local_h::menutext_s; 7],
    pub skill: crate::ui_local_h::menulist_s,
    pub team: crate::ui_local_h::menulist_s,
    pub go: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub numBots: i32,
    pub delay: i32,
    pub baseBotNum: i32,
    pub selectedBotNum: i32,
    pub sortedBotNums: [i32; 1024],
    pub botnames: [[i8; 32]; 7],
}

static mut addBotsMenuInfo: addBotsMenuInfo_t = addBotsMenuInfo_t {
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
    background: crate::ui_local_h::menubitmap_s {
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
    bots: [crate::ui_local_h::menutext_s {
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
    }; 7],
    skill: crate::ui_local_h::menulist_s {
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
    team: crate::ui_local_h::menulist_s {
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
    numBots: 0,
    delay: 0,
    baseBotNum: 0,
    selectedBotNum: 0,
    sortedBotNums: [0; 1024],
    botnames: [[0; 32]; 7],
};
/*
=================
UI_AddBotsMenu_FightEvent
=================
*/

unsafe extern "C" fn UI_AddBotsMenu_FightEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut team: *const i8 = 0 as *const i8;
    let mut skill: i32 = 0;
    if event != 3 {
        return;
    }
    team = *addBotsMenuInfo
        .team
        .itemnames
        .offset(addBotsMenuInfo.team.curvalue as isize);
    skill = addBotsMenuInfo.skill.curvalue + 1;
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
        crate::src::qcommon::q_shared::va(
            b"addbot %s %i %s %i\n\x00" as *const u8 as *mut i8,
            addBotsMenuInfo.botnames[addBotsMenuInfo.selectedBotNum as usize].as_mut_ptr(),
            skill,
            team,
            addBotsMenuInfo.delay,
        ),
    );
    addBotsMenuInfo.delay += 1500;
}
/*
=================
UI_AddBotsMenu_BotEvent
=================
*/

unsafe extern "C" fn UI_AddBotsMenu_BotEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    addBotsMenuInfo.bots[addBotsMenuInfo.selectedBotNum as usize].color =
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr();
    addBotsMenuInfo.selectedBotNum = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id - 20;
    addBotsMenuInfo.bots[addBotsMenuInfo.selectedBotNum as usize].color =
        crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
}
/*
=================
UI_AddBotsMenu_BackEvent
=================
*/

unsafe extern "C" fn UI_AddBotsMenu_BackEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
}
/*
=================
UI_AddBotsMenu_SetBotNames
=================
*/

unsafe extern "C" fn UI_AddBotsMenu_SetBotNames() {
    let mut n: i32 = 0;
    let mut info: *const i8 = 0 as *const i8;
    n = 0;
    while n < 7 {
        info = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber(
            addBotsMenuInfo.sortedBotNums[(addBotsMenuInfo.baseBotNum + n) as usize],
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            addBotsMenuInfo.botnames[n as usize].as_mut_ptr(),
            crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"name\x00" as *const u8 as *const i8,
            ),
            ::std::mem::size_of::<[i8; 32]>() as i32,
        );
        n += 1
    }
}
/*
=================
UI_AddBotsMenu_UpEvent
=================
*/

unsafe extern "C" fn UI_AddBotsMenu_UpEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    if addBotsMenuInfo.baseBotNum > 0 {
        addBotsMenuInfo.baseBotNum -= 1;
        UI_AddBotsMenu_SetBotNames();
    };
}
/*
=================
UI_AddBotsMenu_DownEvent
=================
*/

unsafe extern "C" fn UI_AddBotsMenu_DownEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    if (addBotsMenuInfo.baseBotNum + 7) < addBotsMenuInfo.numBots {
        addBotsMenuInfo.baseBotNum += 1;
        UI_AddBotsMenu_SetBotNames();
    };
}
/*
=================
UI_AddBotsMenu_GetSortedBotNums
=================
*/

unsafe extern "C" fn UI_AddBotsMenu_SortCompare(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> i32 {
    let mut num1: i32 = 0;
    let mut num2: i32 = 0;
    let mut info1: *const i8 = 0 as *const i8;
    let mut info2: *const i8 = 0 as *const i8;
    let mut name1: *const i8 = 0 as *const i8;
    let mut name2: *const i8 = 0 as *const i8;
    num1 = *(arg1 as *mut i32);
    num2 = *(arg2 as *mut i32);
    info1 = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber(num1);
    info2 = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber(num2);
    name1 = crate::src::qcommon::q_shared::Info_ValueForKey(
        info1,
        b"name\x00" as *const u8 as *const i8,
    );
    name2 = crate::src::qcommon::q_shared::Info_ValueForKey(
        info2,
        b"name\x00" as *const u8 as *const i8,
    );
    return crate::src::qcommon::q_shared::Q_stricmp(name1, name2);
}

unsafe extern "C" fn UI_AddBotsMenu_GetSortedBotNums() {
    let mut n: i32 = 0;

    for n in 0..addBotsMenuInfo.numBots {
        addBotsMenuInfo.sortedBotNums[n as usize] = n;
    }
    crate::stdlib::qsort(
        addBotsMenuInfo.sortedBotNums.as_mut_ptr() as *mut libc::c_void,
        addBotsMenuInfo.numBots as crate::stddef_h::size_t,
        ::std::mem::size_of::<i32>(),
        Some(
            UI_AddBotsMenu_SortCompare
                as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
        ),
    );
}
/*
=================
UI_AddBotsMenu_Init
=================
*/

static mut skillNames: [*const i8; 6] = [
    b"I Can Win\x00" as *const u8 as *const i8,
    b"Bring It On\x00" as *const u8 as *const i8,
    b"Hurt Me Plenty\x00" as *const u8 as *const i8,
    b"Hardcore\x00" as *const u8 as *const i8,
    b"Nightmare!\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut teamNames1: [*const i8; 2] = [b"Free\x00" as *const u8 as *const i8, 0 as *const i8];

static mut teamNames2: [*const i8; 3] = [
    b"Red\x00" as *const u8 as *const i8,
    b"Blue\x00" as *const u8 as *const i8,
    0 as *const i8,
];

unsafe extern "C" fn UI_AddBotsMenu_Init() {
    let mut n: i32 = 0;
    let mut y: i32 = 0;
    let mut gametype: i32 = 0;
    let mut count: i32 = 0;
    let mut info: [i8; 1024] = [0; 1024];
    crate::src::ui::ui_syscalls::trap_GetConfigString(0, info.as_mut_ptr(), 1024);
    gametype = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info.as_mut_ptr(),
        b"g_gametype\x00" as *const u8 as *const i8,
    ));
    crate::stdlib::memset(
        &mut addBotsMenuInfo as *mut addBotsMenuInfo_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<addBotsMenuInfo_t>(),
    );
    addBotsMenuInfo.menu.fullscreen = crate::src::qcommon::q_shared::qfalse;
    addBotsMenuInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    addBotsMenuInfo.delay = 1000;
    UI_AddBots_Cache();
    addBotsMenuInfo.numBots = crate::src::q3_ui::ui_gameinfo::UI_GetNumBots();
    count = if addBotsMenuInfo.numBots < 7 {
        addBotsMenuInfo.numBots
    } else {
        7
    };
    addBotsMenuInfo.banner.generic.type_0 = 10;
    addBotsMenuInfo.banner.generic.x = 320;
    addBotsMenuInfo.banner.generic.y = 16;
    addBotsMenuInfo.banner.string = b"ADD BOTS\x00" as *const u8 as *mut i8;
    addBotsMenuInfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    addBotsMenuInfo.banner.style = 0x1;
    addBotsMenuInfo.background.generic.type_0 = 6;
    addBotsMenuInfo.background.generic.name = b"menu/art/addbotframe\x00" as *const u8 as *const i8;
    addBotsMenuInfo.background.generic.flags = 0x4000;
    addBotsMenuInfo.background.generic.x = 320 - 233;
    addBotsMenuInfo.background.generic.y = 240 - 166;
    addBotsMenuInfo.background.width = 466;
    addBotsMenuInfo.background.height = 332;
    addBotsMenuInfo.arrows.generic.type_0 = 6;
    addBotsMenuInfo.arrows.generic.name = b"menu/art/arrows_vert_0\x00" as *const u8 as *const i8;
    addBotsMenuInfo.arrows.generic.flags = 0x4000;
    addBotsMenuInfo.arrows.generic.x = 200;
    addBotsMenuInfo.arrows.generic.y = 128;
    addBotsMenuInfo.arrows.width = 64;
    addBotsMenuInfo.arrows.height = 128;
    addBotsMenuInfo.up.generic.type_0 = 6;
    addBotsMenuInfo.up.generic.flags = 0x4 | 0x100;
    addBotsMenuInfo.up.generic.x = 200;
    addBotsMenuInfo.up.generic.y = 128;
    addBotsMenuInfo.up.generic.id = 13;
    addBotsMenuInfo.up.generic.callback =
        Some(UI_AddBotsMenu_UpEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    addBotsMenuInfo.up.width = 64;
    addBotsMenuInfo.up.height = 64;
    addBotsMenuInfo.up.focuspic = b"menu/art/arrows_vert_top\x00" as *const u8 as *mut i8;
    addBotsMenuInfo.down.generic.type_0 = 6;
    addBotsMenuInfo.down.generic.flags = 0x4 | 0x100;
    addBotsMenuInfo.down.generic.x = 200;
    addBotsMenuInfo.down.generic.y = 128 + 64;
    addBotsMenuInfo.down.generic.id = 14;
    addBotsMenuInfo.down.generic.callback =
        Some(UI_AddBotsMenu_DownEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    addBotsMenuInfo.down.width = 64;
    addBotsMenuInfo.down.height = 64;
    addBotsMenuInfo.down.focuspic = b"menu/art/arrows_vert_bot\x00" as *const u8 as *mut i8;
    n = 0;
    y = 120;
    while n < count {
        addBotsMenuInfo.bots[n as usize].generic.type_0 = 9;
        addBotsMenuInfo.bots[n as usize].generic.flags = 0x4 | 0x100;
        addBotsMenuInfo.bots[n as usize].generic.id = 20 + n;
        addBotsMenuInfo.bots[n as usize].generic.x = 320 - 56;
        addBotsMenuInfo.bots[n as usize].generic.y = y;
        addBotsMenuInfo.bots[n as usize].generic.callback = Some(
            UI_AddBotsMenu_BotEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
        );
        addBotsMenuInfo.bots[n as usize].string = addBotsMenuInfo.botnames[n as usize].as_mut_ptr();
        addBotsMenuInfo.bots[n as usize].color =
            crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr();
        addBotsMenuInfo.bots[n as usize].style = 0 | 0x10;
        n += 1;
        y += 20
    }
    y += 12;
    addBotsMenuInfo.skill.generic.type_0 = 3;
    addBotsMenuInfo.skill.generic.flags = 0x100 | 0x2;
    addBotsMenuInfo.skill.generic.x = 320;
    addBotsMenuInfo.skill.generic.y = y;
    addBotsMenuInfo.skill.generic.name = b"Skill:\x00" as *const u8 as *const i8;
    addBotsMenuInfo.skill.generic.id = 15;
    addBotsMenuInfo.skill.itemnames = skillNames.as_mut_ptr();
    addBotsMenuInfo.skill.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        4f32,
        (crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"g_spSkill\x00" as *const u8 as *const i8,
        ) as i32
            - 1) as f32,
    ) as i32;
    y += 16;
    addBotsMenuInfo.team.generic.type_0 = 3;
    addBotsMenuInfo.team.generic.flags = 0x100 | 0x2;
    addBotsMenuInfo.team.generic.x = 320;
    addBotsMenuInfo.team.generic.y = y;
    addBotsMenuInfo.team.generic.name = b"Team: \x00" as *const u8 as *const i8;
    addBotsMenuInfo.team.generic.id = 16;
    if gametype >= crate::bg_public_h::GT_TEAM as i32 {
        addBotsMenuInfo.team.itemnames = teamNames2.as_mut_ptr()
    } else {
        addBotsMenuInfo.team.itemnames = teamNames1.as_mut_ptr();
        addBotsMenuInfo.team.generic.flags = 0x2000
    }
    addBotsMenuInfo.go.generic.type_0 = 6;
    addBotsMenuInfo.go.generic.name = b"menu/art/accept_0\x00" as *const u8 as *const i8;
    addBotsMenuInfo.go.generic.flags = 0x4 | 0x100;
    addBotsMenuInfo.go.generic.id = 11;
    addBotsMenuInfo.go.generic.callback =
        Some(UI_AddBotsMenu_FightEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    addBotsMenuInfo.go.generic.x = 320 + 128 - 128;
    addBotsMenuInfo.go.generic.y = 256 + 128 - 64;
    addBotsMenuInfo.go.width = 128;
    addBotsMenuInfo.go.height = 64;
    addBotsMenuInfo.go.focuspic = b"menu/art/accept_1\x00" as *const u8 as *mut i8;
    addBotsMenuInfo.back.generic.type_0 = 6;
    addBotsMenuInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    addBotsMenuInfo.back.generic.flags = 0x4 | 0x100;
    addBotsMenuInfo.back.generic.id = 10;
    addBotsMenuInfo.back.generic.callback =
        Some(UI_AddBotsMenu_BackEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    addBotsMenuInfo.back.generic.x = 320 - 128;
    addBotsMenuInfo.back.generic.y = 256 + 128 - 64;
    addBotsMenuInfo.back.width = 128;
    addBotsMenuInfo.back.height = 64;
    addBotsMenuInfo.back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    addBotsMenuInfo.baseBotNum = 0;
    addBotsMenuInfo.selectedBotNum = 0;
    addBotsMenuInfo.bots[0].color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    UI_AddBotsMenu_GetSortedBotNums();
    UI_AddBotsMenu_SetBotNames();
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.background as *mut crate::ui_local_h::menubitmap_s
            as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.arrows as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.up as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.down as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    n = 0;
    while n < count {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut addBotsMenuInfo.menu,
            &mut *addBotsMenuInfo.bots.as_mut_ptr().offset(n as isize)
                as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
        );
        n += 1
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.skill as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.team as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.go as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut addBotsMenuInfo.menu,
        &mut addBotsMenuInfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
}
/*
=================
UI_AddBots_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_AddBots_Cache() {
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
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/addbotframe\x00" as *const u8 as *const i8,
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
/*
=================
UI_AddBotsMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_AddBotsMenu() {
    UI_AddBotsMenu_Init();
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut addBotsMenuInfo.menu);
}
