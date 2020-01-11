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
pub use crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString;
pub use crate::src::q3_ui::ui_atoms::UI_DrawChar;
pub use crate::src::q3_ui::ui_atoms::UI_DrawHandlePic;
pub use crate::src::q3_ui::ui_atoms::UI_DrawString;
pub use crate::src::q3_ui::ui_atoms::UI_FillRect;
pub use crate::src::q3_ui::ui_atoms::UI_PopMenu;
pub use crate::src::q3_ui::ui_atoms::UI_PushMenu;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByMap;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByName;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetNumArenas;
pub use crate::src::q3_ui::ui_gameinfo::UI_GetNumBots;
pub use crate::src::q3_ui::ui_qmenu::color_orange;
pub use crate::src::q3_ui::ui_qmenu::color_red;
pub use crate::src::q3_ui::ui_qmenu::color_white;
pub use crate::src::q3_ui::ui_qmenu::listbar_color;
pub use crate::src::q3_ui::ui_qmenu::text_color_disabled;
pub use crate::src::q3_ui::ui_qmenu::text_color_highlight;
pub use crate::src::q3_ui::ui_qmenu::text_color_normal;
pub use crate::src::q3_ui::ui_qmenu::Bitmap_Draw;
pub use crate::src::q3_ui::ui_qmenu::Menu_AddItem;
pub use crate::src::q3_ui::ui_startserver::stdlib_h::atoi;
pub use crate::src::qcommon::q_math::colorBlack;
pub use crate::src::qcommon::q_math::colorRed;
pub use crate::src::qcommon::q_math::colorWhite;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec4_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::COM_ParseExt;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::Q_strupr;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_SetValue;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip;
pub use crate::stdlib::__compar_fn_t;
use crate::stdlib::memset;
pub use crate::stdlib::qsort;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
use crate::stdlib::strrchr;
pub use crate::stdlib::strtol;
pub use crate::ui_local_h::_tag_menuframework;
pub use crate::ui_local_h::menubitmap_s;
pub use crate::ui_local_h::menucommon_s;
pub use crate::ui_local_h::menufield_s;
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::menulist_s;
pub use crate::ui_local_h::menuradiobutton_s;
pub use crate::ui_local_h::menutext_s;
pub use crate::ui_local_h::mfield_t;
extern "C" {
    // use ui_servers2.c definition
    #[no_mangle]
    pub static mut punkbuster_items: [*const i8; 0];
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct startserver_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub framel: crate::ui_local_h::menubitmap_s,
    pub framer: crate::ui_local_h::menubitmap_s,
    pub gametype: crate::ui_local_h::menulist_s,
    pub mappics: [crate::ui_local_h::menubitmap_s; 4],
    pub mapbuttons: [crate::ui_local_h::menubitmap_s; 4],
    pub arrows: crate::ui_local_h::menubitmap_s,
    pub prevpage: crate::ui_local_h::menubitmap_s,
    pub nextpage: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub next: crate::ui_local_h::menubitmap_s,
    pub mapname: crate::ui_local_h::menutext_s,
    pub item_null: crate::ui_local_h::menubitmap_s,
    pub multiplayer: crate::src::qcommon::q_shared::qboolean,
    pub currentmap: i32,
    pub nummaps: i32,
    pub page: i32,
    pub maxpages: i32,
    pub maplist: [i32; 1024],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serveroptions_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub mappic: crate::ui_local_h::menubitmap_s,
    pub picframe: crate::ui_local_h::menubitmap_s,
    pub dedicated: crate::ui_local_h::menulist_s,
    pub timelimit: crate::ui_local_h::menufield_s,
    pub fraglimit: crate::ui_local_h::menufield_s,
    pub flaglimit: crate::ui_local_h::menufield_s,
    pub friendlyfire: crate::ui_local_h::menuradiobutton_s,
    pub hostname: crate::ui_local_h::menufield_s,
    pub pure_0: crate::ui_local_h::menuradiobutton_s,
    pub botSkill: crate::ui_local_h::menulist_s,
    pub player0: crate::ui_local_h::menutext_s,
    pub playerType: [crate::ui_local_h::menulist_s; 12],
    pub playerName: [crate::ui_local_h::menutext_s; 12],
    pub playerTeam: [crate::ui_local_h::menulist_s; 12],
    pub go: crate::ui_local_h::menubitmap_s,
    pub next: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub multiplayer: crate::src::qcommon::q_shared::qboolean,
    pub gametype: i32,
    pub mapnamebuffer: [i8; 32],
    pub playerNameBuffers: [[i8; 16]; 12],
    pub newBot: crate::src::qcommon::q_shared::qboolean,
    pub newBotIndex: i32,
    pub newBotName: [i8; 16],
    pub punkbuster: crate::ui_local_h::menulist_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct botSelectInfo_t {
    pub menu: crate::ui_local_h::menuframework_s,
    pub banner: crate::ui_local_h::menutext_s,
    pub pics: [crate::ui_local_h::menubitmap_s; 16],
    pub picbuttons: [crate::ui_local_h::menubitmap_s; 16],
    pub picnames: [crate::ui_local_h::menutext_s; 16],
    pub arrows: crate::ui_local_h::menubitmap_s,
    pub left: crate::ui_local_h::menubitmap_s,
    pub right: crate::ui_local_h::menubitmap_s,
    pub go: crate::ui_local_h::menubitmap_s,
    pub back: crate::ui_local_h::menubitmap_s,
    pub numBots: i32,
    pub modelpage: i32,
    pub numpages: i32,
    pub selectedmodel: i32,
    pub sortedBotNums: [i32; 1024],
    pub boticons: [[i8; 64]; 16],
    pub botnames: [[i8; 16]; 16],
}

static mut s_startserver: startserver_t = startserver_t {
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
    mappics: [crate::ui_local_h::menubitmap_s {
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
    mapbuttons: [crate::ui_local_h::menubitmap_s {
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
    prevpage: crate::ui_local_h::menubitmap_s {
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
    nextpage: crate::ui_local_h::menubitmap_s {
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
    next: crate::ui_local_h::menubitmap_s {
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
    mapname: crate::ui_local_h::menutext_s {
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
    multiplayer: crate::src::qcommon::q_shared::qfalse,
    currentmap: 0,
    nummaps: 0,
    page: 0,
    maxpages: 0,
    maplist: [0; 1024],
};

static mut gametype_items: [*const i8; 5] = [
    b"Free For All\x00" as *const u8 as *const i8,
    b"Team Deathmatch\x00" as *const u8 as *const i8,
    b"Tournament\x00" as *const u8 as *const i8,
    b"Capture the Flag\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut gametype_remap: [i32; 4] = [
    crate::bg_public_h::GT_FFA as i32,
    crate::bg_public_h::GT_TEAM as i32,
    crate::bg_public_h::GT_TOURNAMENT as i32,
    crate::bg_public_h::GT_CTF as i32,
];

static mut gametype_remap2: [i32; 5] = [0, 2, 0, 1, 3];
/*
=================
GametypeBits
=================
*/

unsafe extern "C" fn GametypeBits(mut string: *mut i8) -> i32 {
    let mut bits: i32 = 0;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut token: *mut i8 = 0 as *mut i8;
    bits = 0;
    p = string;
    loop {
        token = crate::src::qcommon::q_shared::COM_ParseExt(
            &mut p,
            crate::src::qcommon::q_shared::qfalse,
        );
        if *token.offset(0) == 0 {
            break;
        }
        if crate::src::qcommon::q_shared::Q_stricmp(token, b"ffa\x00" as *const u8 as *const i8)
            == 0
        {
            bits |= (1) << crate::bg_public_h::GT_FFA as i32
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"tourney\x00" as *const u8 as *const i8,
        ) == 0
        {
            bits |= (1) << crate::bg_public_h::GT_TOURNAMENT as i32
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"single\x00" as *const u8 as *const i8,
        ) == 0
        {
            bits |= (1) << crate::bg_public_h::GT_SINGLE_PLAYER as i32
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            token,
            b"team\x00" as *const u8 as *const i8,
        ) == 0
        {
            bits |= (1) << crate::bg_public_h::GT_TEAM as i32
        } else {
            if !(crate::src::qcommon::q_shared::Q_stricmp(
                token,
                b"ctf\x00" as *const u8 as *const i8,
            ) == 0)
            {
                continue;
            }
            bits |= (1) << crate::bg_public_h::GT_CTF as i32
        }
    }
    return bits;
}
/*
=================
StartServer_Update
=================
*/

unsafe extern "C" fn StartServer_Update() {
    let mut i: i32 = 0;
    let mut top: i32 = 0;
    static mut picname: [[i8; 64]; 4] = [[0; 64]; 4];
    let mut info: *const i8 = 0 as *const i8;
    let mut mapname: [i8; 16] = [0; 16];
    top = s_startserver.page * 4;
    i = 0;
    while i < 4 {
        if top + i >= s_startserver.nummaps {
            break;
        }
        info = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(
            s_startserver.maplist[(top + i) as usize],
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            mapname.as_mut_ptr(),
            crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"map\x00" as *const u8 as *const i8,
            ),
            16,
        );
        crate::src::qcommon::q_shared::Q_strupr(mapname.as_mut_ptr());
        crate::src::qcommon::q_shared::Com_sprintf(
            picname[i as usize].as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"levelshots/%s\x00" as *const u8 as *const i8,
            mapname.as_mut_ptr(),
        );
        s_startserver.mappics[i as usize].generic.flags &= !(0x40);
        s_startserver.mappics[i as usize].generic.name = picname[i as usize].as_mut_ptr();
        s_startserver.mappics[i as usize].shader = 0;
        // reset
        s_startserver.mapbuttons[i as usize].generic.flags |= 0x100;
        s_startserver.mapbuttons[i as usize].generic.flags &= !(0x4000);
        i += 1
    }
    while i < 4 {
        s_startserver.mappics[i as usize].generic.flags &= !(0x40);
        s_startserver.mappics[i as usize].generic.name = 0 as *const i8;
        s_startserver.mappics[i as usize].shader = 0;
        // disable
        s_startserver.mapbuttons[i as usize].generic.flags &= !(0x100);
        s_startserver.mapbuttons[i as usize].generic.flags |= 0x4000;
        i += 1
    }
    // no servers to start
    if s_startserver.nummaps == 0 {
        s_startserver.next.generic.flags |= 0x4000;
        // set the map name
        crate::stdlib::strcpy(
            s_startserver.mapname.string,
            b"NO MAPS FOUND\x00" as *const u8 as *const i8,
        );
    } else {
        // set the highlight
        s_startserver.next.generic.flags &= !(0x4000);
        i = s_startserver.currentmap - top;
        if i >= 0 && i < 4 {
            s_startserver.mappics[i as usize].generic.flags |= 0x40;
            s_startserver.mapbuttons[i as usize].generic.flags &= !(0x100)
        }
        // set the map name
        info = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(
            s_startserver.maplist[s_startserver.currentmap as usize],
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            s_startserver.mapname.string,
            crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"map\x00" as *const u8 as *const i8,
            ),
            16i32,
        );
    }
    crate::src::qcommon::q_shared::Q_strupr(s_startserver.mapname.string);
}
/*
=================
StartServer_MapEvent
=================
*/

unsafe extern "C" fn StartServer_MapEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    s_startserver.currentmap =
        s_startserver.page * 4 + ((*(ptr as *mut crate::ui_local_h::menucommon_s)).id - 11);
    StartServer_Update();
}
/*
=================
StartServer_GametypeEvent
=================
*/

unsafe extern "C" fn StartServer_GametypeEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut i: i32 = 0;
    let mut count: i32 = 0;
    let mut gamebits: i32 = 0;
    let mut matchbits: i32 = 0;
    let mut info: *const i8 = 0 as *const i8;
    if event != 3 {
        return;
    }
    count = crate::src::q3_ui::ui_gameinfo::UI_GetNumArenas();
    s_startserver.nummaps = 0;
    matchbits = (1) << gametype_remap[s_startserver.gametype.curvalue as usize];
    if gametype_remap[s_startserver.gametype.curvalue as usize] == crate::bg_public_h::GT_FFA as i32
    {
        matchbits |= (1) << crate::bg_public_h::GT_SINGLE_PLAYER as i32
    }

    for i in 0..count {
        info = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(i);

        gamebits = GametypeBits(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"type\x00" as *const u8 as *const i8,
        ));

        if !(gamebits & matchbits == 0) {
            s_startserver.maplist[s_startserver.nummaps as usize] = i;
            s_startserver.nummaps += 1
        }
    }
    s_startserver.maxpages = (s_startserver.nummaps + 4 - 1) / 4;
    s_startserver.page = 0;
    s_startserver.currentmap = 0;
    StartServer_Update();
}
/*
=================
StartServer_MenuEvent
=================
*/

unsafe extern "C" fn StartServer_MenuEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        15 => {
            if s_startserver.page > 0 {
                s_startserver.page -= 1;
                StartServer_Update();
            }
        }
        16 => {
            if s_startserver.page < s_startserver.maxpages - 1 {
                s_startserver.page += 1;
                StartServer_Update();
            }
        }
        18 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"g_gameType\x00" as *const u8 as *const i8,
                gametype_remap[s_startserver.gametype.curvalue as usize] as f32,
            );
            UI_ServerOptionsMenu(s_startserver.multiplayer);
        }
        17 => {
            crate::src::q3_ui::ui_atoms::UI_PopMenu();
        }
        _ => {}
    };
}
/*
===============
StartServer_LevelshotDraw
===============
*/

unsafe extern "C" fn StartServer_LevelshotDraw(mut self_0: *mut libc::c_void) {
    let mut b: *mut crate::ui_local_h::menubitmap_s = 0 as *mut crate::ui_local_h::menubitmap_s;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    let mut n: i32 = 0;
    let mut info: *const i8 = 0 as *const i8;
    let mut mapname: [i8; 16] = [0; 16];
    b = self_0 as *mut crate::ui_local_h::menubitmap_s;
    if (*b).generic.name.is_null() {
        return;
    }
    if !(*b).generic.name.is_null() && (*b).shader == 0 {
        (*b).shader = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip((*b).generic.name);
        if (*b).shader == 0 && !(*b).errorpic.is_null() {
            (*b).shader = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip((*b).errorpic)
        }
    }
    if !(*b).focuspic.is_null() && (*b).focusshader == 0 {
        (*b).focusshader = crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip((*b).focuspic)
    }
    x = (*b).generic.x;
    y = (*b).generic.y;
    w = (*b).width;
    h = (*b).height;
    if (*b).shader != 0 {
        crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
            x as f32,
            y as f32,
            w as f32,
            h as f32,
            (*b).shader,
        );
    }
    x = (*b).generic.x;
    y = (*b).generic.y + (*b).height;
    crate::src::q3_ui::ui_atoms::UI_FillRect(
        x as f32,
        y as f32,
        (*b).width as f32,
        28f32,
        crate::src::qcommon::q_math::colorBlack.as_mut_ptr(),
    );
    x += (*b).width / 2;
    y += 4;
    n = s_startserver.page * 4 + (*b).generic.id - 11;
    info =
        crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(s_startserver.maplist[n as usize]);
    crate::src::qcommon::q_shared::Q_strncpyz(
        mapname.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(info, b"map\x00" as *const u8 as *const i8),
        16,
    );
    crate::src::qcommon::q_shared::Q_strupr(mapname.as_mut_ptr());
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        x,
        y,
        mapname.as_mut_ptr(),
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
    );
    x = (*b).generic.x;
    y = (*b).generic.y;
    w = (*b).width;
    h = (*b).height + 28;
    if (*b).generic.flags & 0x40 != 0 {
        crate::src::q3_ui::ui_atoms::UI_DrawHandlePic(
            x as f32,
            y as f32,
            w as f32,
            h as f32,
            (*b).focusshader,
        );
    };
}
/*
=================
StartServer_MenuInit
=================
*/

unsafe extern "C" fn StartServer_MenuInit() {
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    static mut mapnamebuffer: [i8; 64] = [0; 64];
    // zero set all our globals
    crate::stdlib::memset(
        &mut s_startserver as *mut startserver_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<startserver_t>(),
    );
    StartServer_Cache();
    s_startserver.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_startserver.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_startserver.banner.generic.type_0 = 10;
    s_startserver.banner.generic.x = 320;
    s_startserver.banner.generic.y = 16;
    s_startserver.banner.string = b"GAME SERVER\x00" as *const u8 as *mut i8;
    s_startserver.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_startserver.banner.style = 0x1;
    s_startserver.framel.generic.type_0 = 6;
    s_startserver.framel.generic.name = b"menu/art/frame2_l\x00" as *const u8 as *const i8;
    s_startserver.framel.generic.flags = 0x4000;
    s_startserver.framel.generic.x = 0;
    s_startserver.framel.generic.y = 78;
    s_startserver.framel.width = 256;
    s_startserver.framel.height = 329;
    s_startserver.framer.generic.type_0 = 6;
    s_startserver.framer.generic.name = b"menu/art/frame1_r\x00" as *const u8 as *const i8;
    s_startserver.framer.generic.flags = 0x4000;
    s_startserver.framer.generic.x = 376;
    s_startserver.framer.generic.y = 76;
    s_startserver.framer.width = 256;
    s_startserver.framer.height = 334;
    s_startserver.gametype.generic.type_0 = 3;
    s_startserver.gametype.generic.name = b"Game Type:\x00" as *const u8 as *const i8;
    s_startserver.gametype.generic.flags = 0x100 | 0x2;
    s_startserver.gametype.generic.callback =
        Some(StartServer_GametypeEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_startserver.gametype.generic.id = 10;
    s_startserver.gametype.generic.x = 320 - 24;
    s_startserver.gametype.generic.y = 368;
    s_startserver.gametype.itemnames = gametype_items.as_mut_ptr();
    i = 0;
    while i < 4 {
        x = i % 2 * (128 + 8) + 188;
        y = i / 2 * (128 + 8) + 96;
        s_startserver.mappics[i as usize].generic.type_0 = 6;
        s_startserver.mappics[i as usize].generic.flags = 0x4 | 0x4000;
        s_startserver.mappics[i as usize].generic.x = x;
        s_startserver.mappics[i as usize].generic.y = y;
        s_startserver.mappics[i as usize].generic.id = 11 + i;
        s_startserver.mappics[i as usize].width = 128;
        s_startserver.mappics[i as usize].height = 96;
        s_startserver.mappics[i as usize].focuspic =
            b"menu/art/maps_selected\x00" as *const u8 as *mut i8;
        s_startserver.mappics[i as usize].errorpic =
            b"menu/art/unknownmap\x00" as *const u8 as *mut i8;
        s_startserver.mappics[i as usize].generic.ownerdraw =
            Some(StartServer_LevelshotDraw as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        s_startserver.mapbuttons[i as usize].generic.type_0 = 6;
        s_startserver.mapbuttons[i as usize].generic.flags = 0x4 | 0x100 | 0x8000;
        s_startserver.mapbuttons[i as usize].generic.id = 11 + i;
        s_startserver.mapbuttons[i as usize].generic.callback =
            Some(StartServer_MapEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
        s_startserver.mapbuttons[i as usize].generic.x = x - 30;
        s_startserver.mapbuttons[i as usize].generic.y = y - 32;
        s_startserver.mapbuttons[i as usize].width = 256;
        s_startserver.mapbuttons[i as usize].height = 248;
        s_startserver.mapbuttons[i as usize].generic.left = x;
        s_startserver.mapbuttons[i as usize].generic.top = y;
        s_startserver.mapbuttons[i as usize].generic.right = x + 128;
        s_startserver.mapbuttons[i as usize].generic.bottom = y + 128;
        s_startserver.mapbuttons[i as usize].focuspic =
            b"menu/art/maps_select\x00" as *const u8 as *mut i8;
        i += 1
    }
    s_startserver.arrows.generic.type_0 = 6;
    s_startserver.arrows.generic.name = b"menu/art/gs_arrows_0\x00" as *const u8 as *const i8;
    s_startserver.arrows.generic.flags = 0x4000;
    s_startserver.arrows.generic.x = 260;
    s_startserver.arrows.generic.y = 400;
    s_startserver.arrows.width = 128;
    s_startserver.arrows.height = 32;
    s_startserver.prevpage.generic.type_0 = 6;
    s_startserver.prevpage.generic.flags = 0x4 | 0x100;
    s_startserver.prevpage.generic.callback =
        Some(StartServer_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_startserver.prevpage.generic.id = 15;
    s_startserver.prevpage.generic.x = 260;
    s_startserver.prevpage.generic.y = 400;
    s_startserver.prevpage.width = 64;
    s_startserver.prevpage.height = 32;
    s_startserver.prevpage.focuspic = b"menu/art/gs_arrows_l\x00" as *const u8 as *mut i8;
    s_startserver.nextpage.generic.type_0 = 6;
    s_startserver.nextpage.generic.flags = 0x4 | 0x100;
    s_startserver.nextpage.generic.callback =
        Some(StartServer_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_startserver.nextpage.generic.id = 16;
    s_startserver.nextpage.generic.x = 321;
    s_startserver.nextpage.generic.y = 400;
    s_startserver.nextpage.width = 64;
    s_startserver.nextpage.height = 32;
    s_startserver.nextpage.focuspic = b"menu/art/gs_arrows_r\x00" as *const u8 as *mut i8;
    s_startserver.mapname.generic.type_0 = 9;
    s_startserver.mapname.generic.flags = 0x8 | 0x4000;
    s_startserver.mapname.generic.x = 320;
    s_startserver.mapname.generic.y = 440;
    s_startserver.mapname.string = mapnamebuffer.as_mut_ptr();
    s_startserver.mapname.style = 0x1 | 0x20;
    s_startserver.mapname.color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr();
    s_startserver.back.generic.type_0 = 6;
    s_startserver.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    s_startserver.back.generic.flags = 0x4 | 0x100;
    s_startserver.back.generic.callback =
        Some(StartServer_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_startserver.back.generic.id = 17;
    s_startserver.back.generic.x = 0;
    s_startserver.back.generic.y = 480 - 64;
    s_startserver.back.width = 128;
    s_startserver.back.height = 64;
    s_startserver.back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    s_startserver.next.generic.type_0 = 6;
    s_startserver.next.generic.name = b"menu/art/next_0\x00" as *const u8 as *const i8;
    s_startserver.next.generic.flags = 0x10 | 0x100;
    s_startserver.next.generic.callback =
        Some(StartServer_MenuEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_startserver.next.generic.id = 18;
    s_startserver.next.generic.x = 640;
    s_startserver.next.generic.y = 480 - 64;
    s_startserver.next.width = 128;
    s_startserver.next.height = 64;
    s_startserver.next.focuspic = b"menu/art/next_1\x00" as *const u8 as *mut i8;
    s_startserver.item_null.generic.type_0 = 6;
    s_startserver.item_null.generic.flags = 0x4 | 0x800 | 0x100000;
    s_startserver.item_null.generic.x = 0;
    s_startserver.item_null.generic.y = 0;
    s_startserver.item_null.width = 640;
    s_startserver.item_null.height = 480;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.framel as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.framer as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.gametype as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    i = 0;
    while i < 4 {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_startserver.menu,
            &mut *s_startserver.mappics.as_mut_ptr().offset(i as isize)
                as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
        );
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_startserver.menu,
            &mut *s_startserver.mapbuttons.as_mut_ptr().offset(i as isize)
                as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
        );
        i += 1
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.arrows as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.prevpage as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.nextpage as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.next as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.mapname as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_startserver.menu,
        &mut s_startserver.item_null as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    StartServer_GametypeEvent(0 as *mut libc::c_void, 3);
}
/*
=================
StartServer_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn StartServer_Cache() {
    let mut i: i32 = 0;
    let mut info: *const i8 = 0 as *const i8;
    let mut precache: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut picname: [i8; 64] = [0; 64];
    let mut mapname: [i8; 16] = [0; 16];
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/back_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/next_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/next_1\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame2_l\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/frame1_r\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/maps_select\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/maps_selected\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/unknownmap\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/gs_arrows_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/gs_arrows_l\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/gs_arrows_r\x00" as *const u8 as *const i8,
    );
    precache = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"com_buildscript\x00" as *const u8 as *const i8,
    ) as crate::src::qcommon::q_shared::qboolean;
    if precache as u64 != 0 {
        i = 0;
        while i < crate::src::q3_ui::ui_gameinfo::UI_GetNumArenas() {
            info = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(i);
            crate::src::qcommon::q_shared::Q_strncpyz(
                mapname.as_mut_ptr(),
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    info,
                    b"map\x00" as *const u8 as *const i8,
                ),
                16,
            );
            crate::src::qcommon::q_shared::Q_strupr(mapname.as_mut_ptr());
            crate::src::qcommon::q_shared::Com_sprintf(
                picname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 64]>() as i32,
                b"levelshots/%s\x00" as *const u8 as *const i8,
                mapname.as_mut_ptr(),
            );
            crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(picname.as_mut_ptr());
            i += 1
        }
    };
}
/*
=================
UI_StartServerMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_StartServerMenu(
    mut multiplayer: crate::src::qcommon::q_shared::qboolean,
) {
    StartServer_MenuInit();
    s_startserver.multiplayer = multiplayer;
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut s_startserver.menu);
}

static mut s_serveroptions: serveroptions_t = serveroptions_t {
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
    picframe: crate::ui_local_h::menubitmap_s {
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
    dedicated: crate::ui_local_h::menulist_s {
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
    timelimit: crate::ui_local_h::menufield_s {
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
        field: crate::ui_local_h::mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    fraglimit: crate::ui_local_h::menufield_s {
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
        field: crate::ui_local_h::mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    flaglimit: crate::ui_local_h::menufield_s {
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
        field: crate::ui_local_h::mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    friendlyfire: crate::ui_local_h::menuradiobutton_s {
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
    hostname: crate::ui_local_h::menufield_s {
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
        field: crate::ui_local_h::mfield_t {
            cursor: 0,
            scroll: 0,
            widthInChars: 0,
            buffer: [0; 256],
            maxchars: 0,
        },
    },
    pure_0: crate::ui_local_h::menuradiobutton_s {
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
    botSkill: crate::ui_local_h::menulist_s {
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
    player0: crate::ui_local_h::menutext_s {
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
    playerType: [crate::ui_local_h::menulist_s {
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
    }; 12],
    playerName: [crate::ui_local_h::menutext_s {
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
    }; 12],
    playerTeam: [crate::ui_local_h::menulist_s {
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
    }; 12],
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
    next: crate::ui_local_h::menubitmap_s {
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
    multiplayer: crate::src::qcommon::q_shared::qfalse,
    gametype: 0,
    mapnamebuffer: [0; 32],
    playerNameBuffers: [[0; 16]; 12],
    newBot: crate::src::qcommon::q_shared::qfalse,
    newBotIndex: 0,
    newBotName: [0; 16],
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
};

static mut dedicated_list: [*const i8; 4] = [
    b"No\x00" as *const u8 as *const i8,
    b"LAN\x00" as *const u8 as *const i8,
    b"Internet\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut playerType_list: [*const i8; 4] = [
    b"Open\x00" as *const u8 as *const i8,
    b"Bot\x00" as *const u8 as *const i8,
    b"----\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut playerTeam_list: [*const i8; 3] = [
    b"Blue\x00" as *const u8 as *const i8,
    b"Red\x00" as *const u8 as *const i8,
    0 as *const i8,
];

static mut botSkill_list: [*const i8; 6] = [
    b"I Can Win\x00" as *const u8 as *const i8,
    b"Bring It On\x00" as *const u8 as *const i8,
    b"Hurt Me Plenty\x00" as *const u8 as *const i8,
    b"Hardcore\x00" as *const u8 as *const i8,
    b"Nightmare!\x00" as *const u8 as *const i8,
    0 as *const i8,
];
/*
=================
BotAlreadySelected
=================
*/

unsafe extern "C" fn BotAlreadySelected(
    mut checkName: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut n: i32 = 0;

    for n in 1..12 {
        if !(s_serveroptions.playerType[n as usize].curvalue != 1) {
            if !(s_serveroptions.gametype >= crate::bg_public_h::GT_TEAM as i32
                && s_serveroptions.playerTeam[n as usize].curvalue
                    != s_serveroptions.playerTeam[s_serveroptions.newBotIndex as usize].curvalue)
            {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    checkName,
                    s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
                ) == 0
                {
                    return crate::src::qcommon::q_shared::qtrue;
                }
            }
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
=================
ServerOptions_Start
=================
*/

unsafe extern "C" fn ServerOptions_Start() {
    let mut timelimit: i32 = 0;
    let mut fraglimit: i32 = 0;
    let mut maxclients: i32 = 0;
    let mut dedicated: i32 = 0;
    let mut friendlyfire: i32 = 0;
    let mut flaglimit: i32 = 0;
    let mut pure_0: i32 = 0;
    let mut skill: i32 = 0;
    let mut n: i32 = 0;
    let mut buf: [i8; 64] = [0; 64];
    let mut info: *const i8 = 0 as *const i8;
    timelimit = atoi(s_serveroptions.timelimit.field.buffer.as_mut_ptr());
    fraglimit = atoi(s_serveroptions.fraglimit.field.buffer.as_mut_ptr());
    flaglimit = atoi(s_serveroptions.flaglimit.field.buffer.as_mut_ptr());
    dedicated = s_serveroptions.dedicated.curvalue;
    friendlyfire = s_serveroptions.friendlyfire.curvalue;
    pure_0 = s_serveroptions.pure_0.curvalue;
    skill = s_serveroptions.botSkill.curvalue + 1;
    //set maxclients
    n = 0;
    maxclients = 0;
    while n < 12 {
        if !(s_serveroptions.playerType[n as usize].curvalue == 2) {
            if !(s_serveroptions.playerType[n as usize].curvalue == 1
                && s_serveroptions.playerNameBuffers[n as usize][0] as i32 == 0)
            {
                maxclients += 1
            }
        }
        n += 1
    }
    match s_serveroptions.gametype {
        1 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_tourney_fraglimit\x00" as *const u8 as *const i8,
                fraglimit as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_tourney_timelimit\x00" as *const u8 as *const i8,
                timelimit as f32,
            );
        }
        3 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_team_fraglimit\x00" as *const u8 as *const i8,
                fraglimit as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_team_timelimit\x00" as *const u8 as *const i8,
                timelimit as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_team_friendly\x00" as *const u8 as *const i8,
                friendlyfire as f32,
            );
        }
        4 => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_ctf_capturelimit\x00" as *const u8 as *const i8,
                flaglimit as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_ctf_timelimit\x00" as *const u8 as *const i8,
                timelimit as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_ctf_friendly\x00" as *const u8 as *const i8,
                friendlyfire as f32,
            );
        }
        0 | _ => {
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_ffa_fraglimit\x00" as *const u8 as *const i8,
                fraglimit as f32,
            );
            crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
                b"ui_ffa_timelimit\x00" as *const u8 as *const i8,
                timelimit as f32,
            );
        }
    }
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"sv_maxclients\x00" as *const u8 as *const i8,
        crate::src::qcommon::q_shared::Com_Clamp(0f32, 12f32, maxclients as f32),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"dedicated\x00" as *const u8 as *const i8,
        crate::src::qcommon::q_shared::Com_Clamp(0f32, 2f32, dedicated as f32),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"timelimit\x00" as *const u8 as *const i8,
        crate::src::qcommon::q_shared::Com_Clamp(0f32, timelimit as f32, timelimit as f32),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"fraglimit\x00" as *const u8 as *const i8,
        crate::src::qcommon::q_shared::Com_Clamp(0f32, fraglimit as f32, fraglimit as f32),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"capturelimit\x00" as *const u8 as *const i8,
        crate::src::qcommon::q_shared::Com_Clamp(0f32, flaglimit as f32, flaglimit as f32),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"g_friendlyfire\x00" as *const u8 as *const i8,
        friendlyfire as f32,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"sv_pure\x00" as *const u8 as *const i8,
        pure_0 as f32,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"sv_hostname\x00" as *const u8 as *const i8,
        s_serveroptions.hostname.field.buffer.as_mut_ptr(),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_SetValue(
        b"sv_punkbuster\x00" as *const u8 as *const i8,
        s_serveroptions.punkbuster.curvalue as f32,
    );
    // the wait commands will allow the dedicated to take effect
    info = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(
        s_startserver.maplist[s_startserver.currentmap as usize],
    );
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
        crate::src::qcommon::q_shared::va(
            b"wait ; wait ; map %s\n\x00" as *const u8 as *mut i8,
            crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"map\x00" as *const u8 as *const i8,
            ),
        ),
    );
    // add bots
    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
        b"wait 3\n\x00" as *const u8 as *const i8,
    );
    n = 1;
    while n < 12 {
        if !(s_serveroptions.playerType[n as usize].curvalue != 1) {
            if !(s_serveroptions.playerNameBuffers[n as usize][0] as i32 == 0) {
                if !(s_serveroptions.playerNameBuffers[n as usize][0] as i32 == '-' as i32) {
                    if s_serveroptions.gametype >= crate::bg_public_h::GT_TEAM as i32 {
                        crate::src::qcommon::q_shared::Com_sprintf(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[i8; 64]>() as i32,
                            b"addbot %s %i %s\n\x00" as *const u8 as *const i8,
                            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
                            skill,
                            playerTeam_list
                                [s_serveroptions.playerTeam[n as usize].curvalue as usize],
                        );
                    } else {
                        crate::src::qcommon::q_shared::Com_sprintf(
                            buf.as_mut_ptr(),
                            ::std::mem::size_of::<[i8; 64]>() as i32,
                            b"addbot %s %i\n\x00" as *const u8 as *const i8,
                            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
                            skill,
                        );
                    }
                    crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
                        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
                        buf.as_mut_ptr(),
                    );
                }
            }
        }
        n += 1
    }
    // set player's team
    if dedicated == 0 && s_serveroptions.gametype >= crate::bg_public_h::GT_TEAM as i32 {
        // send team command for vanilla q3 game qvm
        crate::src::ui::ui_syscalls::trap_Cmd_ExecuteText(
            crate::src::qcommon::q_shared::EXEC_APPEND as i32,
            crate::src::qcommon::q_shared::va(
                b"wait 5; team %s\n\x00" as *const u8 as *mut i8,
                playerTeam_list[s_serveroptions.playerTeam[0usize].curvalue as usize],
            ),
        );
        // set g_localTeamPref for ioq3 game qvm
        crate::src::ui::ui_syscalls::trap_Cvar_Set(
            b"g_localTeamPref\x00" as *const u8 as *const i8,
            playerTeam_list[s_serveroptions.playerTeam[0usize].curvalue as usize],
        );
    };
}
/*
=================
ServerOptions_InitPlayerItems
=================
*/

unsafe extern "C" fn ServerOptions_InitPlayerItems() {
    let mut n: i32 = 0;
    let mut v: i32 = 0;
    // init types
    if s_serveroptions.multiplayer as u64 != 0 {
        v = 0
    // open
    } else {
        v = 1
        // bot
    }
    n = 0;
    while n < 12 {
        s_serveroptions.playerType[n as usize].curvalue = v;
        n += 1
    }
    if s_serveroptions.multiplayer != 0
        && s_serveroptions.gametype < crate::bg_public_h::GT_TEAM as i32
    {
        n = 8;
        while n < 12 {
            s_serveroptions.playerType[n as usize].curvalue = 2;
            n += 1
        }
    }
    // if not a dedicated server, first slot is reserved for the human on the server
    if s_serveroptions.dedicated.curvalue == 0 {
        // human
        s_serveroptions.playerType[0].generic.flags |= 0x4000;
        s_serveroptions.playerType[0].curvalue = 0;
        crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
            b"name\x00" as *const u8 as *const i8,
            s_serveroptions.playerNameBuffers[0].as_mut_ptr(),
            ::std::mem::size_of::<[i8; 16]>() as i32,
        );
        crate::src::qcommon::q_shared::Q_CleanStr(
            s_serveroptions.playerNameBuffers[0usize].as_mut_ptr(),
        );
    }
    // init teams
    if s_serveroptions.gametype >= crate::bg_public_h::GT_TEAM as i32 {
        n = 0;
        while n < 12 / 2 {
            s_serveroptions.playerTeam[n as usize].curvalue = 0;
            n += 1
        }
        while n < 12 {
            s_serveroptions.playerTeam[n as usize].curvalue = 1;
            n += 1
        }
    } else {
        n = 0;
        while n < 12 {
            s_serveroptions.playerTeam[n as usize].generic.flags |= 0x4000 | 0x1000;
            n += 1
        }
    };
}
/*
=================
ServerOptions_SetPlayerItems
=================
*/

unsafe extern "C" fn ServerOptions_SetPlayerItems() {
    let mut start: i32 = 0;
    let mut n: i32 = 0;
    // types
    //	for( n = 0; n < PLAYER_SLOTS; n++ ) {
    //		if( (!s_serveroptions.multiplayer) && (n > 0) && (s_serveroptions.playerType[n].curvalue == 0) ) {
    //			s_serveroptions.playerType[n].curvalue = 1;
    //		}
    //	}
    // names
    if s_serveroptions.dedicated.curvalue == 0 {
        s_serveroptions.player0.string = b"Human\x00" as *const u8 as *mut i8;
        s_serveroptions.playerName[0].generic.flags &= !(0x1000);
        start = 1
    } else {
        s_serveroptions.player0.string = b"Open\x00" as *const u8 as *mut i8;
        start = 0
    }
    n = start;
    while n < 12 {
        if s_serveroptions.playerType[n as usize].curvalue == 1 {
            s_serveroptions.playerName[n as usize].generic.flags &= !(0x4000 | 0x1000)
        } else {
            s_serveroptions.playerName[n as usize].generic.flags |= 0x4000 | 0x1000
        }
        n += 1
    }
    // teams
    if s_serveroptions.gametype < crate::bg_public_h::GT_TEAM as i32 {
        return;
    }
    n = start;
    while n < 12 {
        if s_serveroptions.playerType[n as usize].curvalue == 2 {
            s_serveroptions.playerTeam[n as usize].generic.flags |= 0x4000 | 0x1000
        } else {
            s_serveroptions.playerTeam[n as usize].generic.flags &= !(0x4000 | 0x1000)
        }
        n += 1
    }
}
/*
=================
ServerOptions_Event
=================
*/

unsafe extern "C" fn ServerOptions_Event(mut ptr: *mut libc::c_void, mut event: i32) {
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        20 => {
            //if( event != QM_ACTIVATED && event != QM_LOSTFOCUS) {
            //	return;
            //}
            if !(event != 3) {
                ServerOptions_SetPlayerItems();
            }
        }
        21 | 22 => {
            ServerOptions_SetPlayerItems();
        }
        23 => {
            if !(event != 3) {
                ServerOptions_Start();
            }
        }
        18 => {
            (event) != 3i32;
        }
        24 => {
            if !(event != 3) {
                crate::src::q3_ui::ui_atoms::UI_PopMenu();
            }
        }
        _ => {}
    };
}

unsafe extern "C" fn ServerOptions_PlayerNameEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut n: i32 = 0;
    if event != 3 {
        return;
    }
    n = (*(ptr as *mut crate::ui_local_h::menutext_s)).generic.id;
    s_serveroptions.newBotIndex = n;
    UI_BotSelectMenu(s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr());
}
/*
=================
ServerOptions_StatusBar
=================
*/

unsafe extern "C" fn ServerOptions_StatusBar(mut ptr: *mut libc::c_void) {
    match (*(ptr as *mut crate::ui_local_h::menucommon_s)).id {
        _ => {}
    }
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        320,
        440,
        b"0 = NO LIMIT\x00" as *const u8 as *const i8,
        0x1 | 0x10,
        crate::src::qcommon::q_math::colorWhite.as_mut_ptr(),
    );
}
/*
===============
ServerOptions_LevelshotDraw
===============
*/

unsafe extern "C" fn ServerOptions_LevelshotDraw(mut self_0: *mut libc::c_void) {
    let mut b: *mut crate::ui_local_h::menubitmap_s = 0 as *mut crate::ui_local_h::menubitmap_s;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    // strange place for this, but it works
    if s_serveroptions.newBot as u64 != 0 {
        crate::src::qcommon::q_shared::Q_strncpyz(
            s_serveroptions.playerNameBuffers[s_serveroptions.newBotIndex as usize].as_mut_ptr(),
            s_serveroptions.newBotName.as_mut_ptr(),
            16,
        ); // skip the first slot, reserved for a human
        s_serveroptions.newBot = crate::src::qcommon::q_shared::qfalse
    }
    b = self_0 as *mut crate::ui_local_h::menubitmap_s;
    crate::src::q3_ui::ui_qmenu::Bitmap_Draw(b);
    x = (*b).generic.x;
    y = (*b).generic.y + (*b).height;
    crate::src::q3_ui::ui_atoms::UI_FillRect(
        x as f32,
        y as f32,
        (*b).width as f32,
        40f32,
        crate::src::qcommon::q_math::colorBlack.as_mut_ptr(),
    );
    x += (*b).width / 2;
    y += 4;
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        x,
        y,
        s_serveroptions.mapnamebuffer.as_mut_ptr(),
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
    );
    y += 16;
    crate::src::q3_ui::ui_atoms::UI_DrawString(
        x,
        y,
        gametype_items[gametype_remap2[s_serveroptions.gametype as usize] as usize],
        0x1 | 0x10,
        crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr(),
    );
}

unsafe extern "C" fn ServerOptions_InitBotNames() {
    let mut count: i32 = 0;
    let mut n: i32 = 0;
    let mut arenaInfo: *const i8 = 0 as *const i8;
    let mut botInfo: *const i8 = 0 as *const i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut bot: *mut i8 = 0 as *mut i8;
    let mut bots: [i8; 1024] = [0; 1024];
    if s_serveroptions.gametype >= crate::bg_public_h::GT_TEAM as i32 {
        crate::src::qcommon::q_shared::Q_strncpyz(
            s_serveroptions.playerNameBuffers[1].as_mut_ptr(),
            b"grunt\x00" as *const u8 as *const i8,
            16,
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            s_serveroptions.playerNameBuffers[2].as_mut_ptr(),
            b"major\x00" as *const u8 as *const i8,
            16,
        );
        if s_serveroptions.gametype == crate::bg_public_h::GT_TEAM as i32 {
            crate::src::qcommon::q_shared::Q_strncpyz(
                s_serveroptions.playerNameBuffers[3usize].as_mut_ptr(),
                b"visor\x00" as *const u8 as *const i8,
                16i32,
            );
        } else {
            s_serveroptions.playerType[3].curvalue = 2
        }
        s_serveroptions.playerType[4].curvalue = 2;
        s_serveroptions.playerType[5].curvalue = 2;
        crate::src::qcommon::q_shared::Q_strncpyz(
            s_serveroptions.playerNameBuffers[6].as_mut_ptr(),
            b"sarge\x00" as *const u8 as *const i8,
            16,
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            s_serveroptions.playerNameBuffers[7].as_mut_ptr(),
            b"grunt\x00" as *const u8 as *const i8,
            16,
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            s_serveroptions.playerNameBuffers[8].as_mut_ptr(),
            b"major\x00" as *const u8 as *const i8,
            16,
        );
        if s_serveroptions.gametype == crate::bg_public_h::GT_TEAM as i32 {
            crate::src::qcommon::q_shared::Q_strncpyz(
                s_serveroptions.playerNameBuffers[9usize].as_mut_ptr(),
                b"visor\x00" as *const u8 as *const i8,
                16i32,
            );
        } else {
            s_serveroptions.playerType[9].curvalue = 2
        }
        s_serveroptions.playerType[10].curvalue = 2;
        s_serveroptions.playerType[11].curvalue = 2;
        return;
    }
    count = 1;
    // get info for this map
    arenaInfo = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByMap(
        s_serveroptions.mapnamebuffer.as_mut_ptr(),
    );
    // get the bot info - we'll seed with them if any are listed
    crate::src::qcommon::q_shared::Q_strncpyz(
        bots.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            arenaInfo,
            b"bots\x00" as *const u8 as *const i8,
        ),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    p = &mut *bots.as_mut_ptr().offset(0) as *mut i8;
    while *p as i32 != 0 && count < 12 {
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
            let fresh0 = p;
            p = p.offset(1);
            *fresh0 = 0i8
        }
        botInfo = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByName(bot);
        if botInfo.is_null() {
            botInfo = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber(count)
        }
        bot = crate::src::qcommon::q_shared::Info_ValueForKey(
            botInfo,
            b"name\x00" as *const u8 as *const i8,
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            s_serveroptions.playerNameBuffers[count as usize].as_mut_ptr(),
            bot,
            ::std::mem::size_of::<[i8; 16]>() as i32,
        );
        count += 1
    }

    for n in count..12 {
        crate::stdlib::strcpy(
            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr(),
            b"--------\x00" as *const u8 as *const i8,
        );
    }
    // pad up to #8 as open slots
    while count < 8 {
        s_serveroptions.playerType[count as usize].curvalue = 0;
        count += 1
    }
    // close off the rest by default
    while count < 12 {
        if s_serveroptions.playerType[count as usize].curvalue == 1 {
            s_serveroptions.playerType[count as usize].curvalue = 2
        }
        count += 1
    }
}
/*
=================
ServerOptions_SetMenuItems
=================
*/

unsafe extern "C" fn ServerOptions_SetMenuItems() {
    static mut picname: [i8; 64] = [0; 64];
    let mut mapname: [i8; 16] = [0; 16];
    let mut info: *const i8 = 0 as *const i8;
    match s_serveroptions.gametype {
        1 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                s_serveroptions.fraglimit.field.buffer.as_mut_ptr(),
                4,
                b"%i\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::Com_Clamp(
                    0f32,
                    999f32,
                    crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                        b"ui_tourney_fraglimit\x00" as *const u8 as *const i8,
                    ),
                ) as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4i32,
                b"%i\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::Com_Clamp(
                    0f32,
                    999f32,
                    crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                        b"ui_tourney_timelimit\x00" as *const u8 as *const i8,
                    ),
                ) as i32,
            );
        }
        3 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                s_serveroptions.fraglimit.field.buffer.as_mut_ptr(),
                4,
                b"%i\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::Com_Clamp(
                    0f32,
                    999f32,
                    crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                        b"ui_team_fraglimit\x00" as *const u8 as *const i8,
                    ),
                ) as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4,
                b"%i\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::Com_Clamp(
                    0f32,
                    999f32,
                    crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                        b"ui_team_timelimit\x00" as *const u8 as *const i8,
                    ),
                ) as i32,
            );
            s_serveroptions.friendlyfire.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
                0f32,
                1f32,
                crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                    b"ui_team_friendly\x00" as *const u8 as *const i8,
                ),
            ) as i32
        }
        4 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                s_serveroptions.flaglimit.field.buffer.as_mut_ptr(),
                4,
                b"%i\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::Com_Clamp(
                    0f32,
                    100f32,
                    crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                        b"ui_ctf_capturelimit\x00" as *const u8 as *const i8,
                    ),
                ) as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4,
                b"%i\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::Com_Clamp(
                    0f32,
                    999f32,
                    crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                        b"ui_ctf_timelimit\x00" as *const u8 as *const i8,
                    ),
                ) as i32,
            );
            s_serveroptions.friendlyfire.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
                0f32,
                1f32,
                crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                    b"ui_ctf_friendly\x00" as *const u8 as *const i8,
                ),
            ) as i32
        }
        0 | _ => {
            crate::src::qcommon::q_shared::Com_sprintf(
                s_serveroptions.fraglimit.field.buffer.as_mut_ptr(),
                4,
                b"%i\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::Com_Clamp(
                    0f32,
                    999f32,
                    crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                        b"ui_ffa_fraglimit\x00" as *const u8 as *const i8,
                    ),
                ) as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                s_serveroptions.timelimit.field.buffer.as_mut_ptr(),
                4i32,
                b"%i\x00" as *const u8 as *const i8,
                crate::src::qcommon::q_shared::Com_Clamp(
                    0f32,
                    999f32,
                    crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
                        b"ui_ffa_timelimit\x00" as *const u8 as *const i8,
                    ),
                ) as i32,
            );
        }
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        s_serveroptions.hostname.field.buffer.as_mut_ptr(),
        crate::src::q3_ui::ui_atoms::UI_Cvar_VariableString(
            b"sv_hostname\x00" as *const u8 as *const i8,
        ),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    s_serveroptions.pure_0.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        1f32,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"sv_pure\x00" as *const u8 as *const i8,
        ),
    ) as i32;
    // set the map pic
    info = crate::src::q3_ui::ui_gameinfo::UI_GetArenaInfoByNumber(
        s_startserver.maplist[s_startserver.currentmap as usize],
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        mapname.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(info, b"map\x00" as *const u8 as *const i8),
        16,
    );
    crate::src::qcommon::q_shared::Q_strupr(mapname.as_mut_ptr());
    crate::src::qcommon::q_shared::Com_sprintf(
        picname.as_mut_ptr(),
        64,
        b"levelshots/%s\x00" as *const u8 as *const i8,
        mapname.as_mut_ptr(),
    );
    s_serveroptions.mappic.generic.name = picname.as_mut_ptr();
    // set the map name
    crate::stdlib::strcpy(
        s_serveroptions.mapnamebuffer.as_mut_ptr(),
        s_startserver.mapname.string,
    );
    crate::src::qcommon::q_shared::Q_strupr(s_serveroptions.mapnamebuffer.as_mut_ptr());
    // get the player selections initialized
    ServerOptions_InitPlayerItems();
    ServerOptions_SetPlayerItems();
    // seed bot names
    ServerOptions_InitBotNames();
    ServerOptions_SetPlayerItems();
}
/*
=================
PlayerName_Draw
=================
*/

unsafe extern "C" fn PlayerName_Draw(mut item: *mut libc::c_void) {
    let mut s: *mut crate::ui_local_h::menutext_s = 0 as *mut crate::ui_local_h::menutext_s;
    let mut color: *mut f32 = 0 as *mut f32;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut style: i32 = 0;
    let mut focus: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    s = item as *mut crate::ui_local_h::menutext_s;
    x = (*s).generic.x;
    y = (*s).generic.y;
    style = 0x10;
    focus = ((*(*s).generic.parent).cursor == (*s).generic.menuPosition)
        as crate::src::qcommon::q_shared::qboolean;
    if (*s).generic.flags & 0x2000 != 0 {
        color = crate::src::q3_ui::ui_qmenu::text_color_disabled.as_mut_ptr()
    } else if focus as u64 != 0 {
        color = crate::src::q3_ui::ui_qmenu::text_color_highlight.as_mut_ptr();
        style |= 0x4000
    } else if (*s).generic.flags & 0x1 != 0 {
        color = crate::src::q3_ui::ui_qmenu::text_color_highlight.as_mut_ptr();
        style |= 0x1000
    } else {
        color = crate::src::q3_ui::ui_qmenu::text_color_normal.as_mut_ptr()
    }
    if focus as u64 != 0 {
        // draw cursor
        crate::src::q3_ui::ui_atoms::UI_FillRect(
            (*s).generic.left as f32,
            (*s).generic.top as f32,
            ((*s).generic.right - (*s).generic.left + 1) as f32,
            ((*s).generic.bottom - (*s).generic.top + 1) as f32,
            crate::src::q3_ui::ui_qmenu::listbar_color.as_mut_ptr(),
        );
        crate::src::q3_ui::ui_atoms::UI_DrawChar(x, y, 13i32, 0x1i32 | 0x1000i32 | 0x10i32, color);
    }
    crate::src::q3_ui::ui_atoms::UI_DrawString(x - 8, y, (*s).generic.name, style | 0x2, color);
    crate::src::q3_ui::ui_atoms::UI_DrawString(x + 8, y, (*s).string, style | 0, color);
}

unsafe extern "C" fn ServerOptions_MenuInit(
    mut multiplayer: crate::src::qcommon::q_shared::qboolean,
) {
    let mut y: i32 = 0;
    let mut n: i32 = 0;
    crate::stdlib::memset(
        &mut s_serveroptions as *mut serveroptions_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<serveroptions_t>(),
    );
    s_serveroptions.multiplayer = multiplayer;
    s_serveroptions.gametype = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        (::std::mem::size_of::<[i32; 5]>())
            .wrapping_div(::std::mem::size_of::<i32>())
            .wrapping_sub(1usize) as f32,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"g_gametype\x00" as *const u8 as *const i8,
        ),
    ) as i32;
    s_serveroptions.punkbuster.curvalue = crate::src::qcommon::q_shared::Com_Clamp(
        0f32,
        1f32,
        crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
            b"sv_punkbuster\x00" as *const u8 as *const i8,
        ),
    ) as i32;
    ServerOptions_Cache();
    s_serveroptions.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    s_serveroptions.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    s_serveroptions.banner.generic.type_0 = 10;
    s_serveroptions.banner.generic.x = 320;
    s_serveroptions.banner.generic.y = 16;
    s_serveroptions.banner.string = b"GAME SERVER\x00" as *const u8 as *mut i8;
    s_serveroptions.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    s_serveroptions.banner.style = 0x1;
    s_serveroptions.mappic.generic.type_0 = 6;
    s_serveroptions.mappic.generic.flags = 0x4 | 0x4000;
    s_serveroptions.mappic.generic.x = 352;
    s_serveroptions.mappic.generic.y = 80;
    s_serveroptions.mappic.width = 160;
    s_serveroptions.mappic.height = 120;
    s_serveroptions.mappic.errorpic = b"menu/art/unknownmap\x00" as *const u8 as *mut i8;
    s_serveroptions.mappic.generic.ownerdraw =
        Some(ServerOptions_LevelshotDraw as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_serveroptions.picframe.generic.type_0 = 6;
    s_serveroptions.picframe.generic.flags = 0x4 | 0x4000 | 0x40;
    s_serveroptions.picframe.generic.x = 352 - 38;
    s_serveroptions.picframe.generic.y = 80 - 40;
    s_serveroptions.picframe.width = 320;
    s_serveroptions.picframe.height = 320;
    s_serveroptions.picframe.focuspic = b"menu/art/maps_select\x00" as *const u8 as *mut i8;
    y = 272;
    if s_serveroptions.gametype != crate::bg_public_h::GT_CTF as i32 {
        s_serveroptions.fraglimit.generic.type_0 = 4;
        s_serveroptions.fraglimit.generic.name = b"Frag Limit:\x00" as *const u8 as *const i8;
        s_serveroptions.fraglimit.generic.flags = 0x20 | 0x100 | 0x2;
        s_serveroptions.fraglimit.generic.x = 456;
        s_serveroptions.fraglimit.generic.y = y;
        s_serveroptions.fraglimit.generic.statusbar =
            Some(ServerOptions_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        s_serveroptions.fraglimit.field.widthInChars = 3;
        s_serveroptions.fraglimit.field.maxchars = 3
    } else {
        s_serveroptions.flaglimit.generic.type_0 = 4;
        s_serveroptions.flaglimit.generic.name = b"Capture Limit:\x00" as *const u8 as *const i8;
        s_serveroptions.flaglimit.generic.flags = 0x20 | 0x100 | 0x2;
        s_serveroptions.flaglimit.generic.x = 456;
        s_serveroptions.flaglimit.generic.y = y;
        s_serveroptions.flaglimit.generic.statusbar =
            Some(ServerOptions_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        s_serveroptions.flaglimit.field.widthInChars = 3;
        s_serveroptions.flaglimit.field.maxchars = 3
    }
    y += 16 + 2;
    s_serveroptions.timelimit.generic.type_0 = 4;
    s_serveroptions.timelimit.generic.name = b"Time Limit:\x00" as *const u8 as *const i8;
    s_serveroptions.timelimit.generic.flags = 0x20 | 0x100 | 0x2;
    s_serveroptions.timelimit.generic.x = 456;
    s_serveroptions.timelimit.generic.y = y;
    s_serveroptions.timelimit.generic.statusbar =
        Some(ServerOptions_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_serveroptions.timelimit.field.widthInChars = 3;
    s_serveroptions.timelimit.field.maxchars = 3;
    if s_serveroptions.gametype >= crate::bg_public_h::GT_TEAM as i32 {
        y += 16 + 2;
        s_serveroptions.friendlyfire.generic.type_0 = 5;
        s_serveroptions.friendlyfire.generic.flags = 0x100 | 0x2;
        s_serveroptions.friendlyfire.generic.x = 456;
        s_serveroptions.friendlyfire.generic.y = y;
        s_serveroptions.friendlyfire.generic.name = b"Friendly Fire:\x00" as *const u8 as *const i8
    }
    y += 16 + 2;
    s_serveroptions.pure_0.generic.type_0 = 5;
    s_serveroptions.pure_0.generic.flags = 0x100 | 0x2;
    s_serveroptions.pure_0.generic.x = 456;
    s_serveroptions.pure_0.generic.y = y;
    s_serveroptions.pure_0.generic.name = b"Pure Server:\x00" as *const u8 as *const i8;
    if s_serveroptions.multiplayer as u64 != 0 {
        y += 16 + 2;
        s_serveroptions.dedicated.generic.type_0 = 3;
        s_serveroptions.dedicated.generic.id = 22;
        s_serveroptions.dedicated.generic.flags = 0x100 | 0x2;
        s_serveroptions.dedicated.generic.callback =
            Some(ServerOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
        s_serveroptions.dedicated.generic.x = 456;
        s_serveroptions.dedicated.generic.y = y;
        s_serveroptions.dedicated.generic.name = b"Dedicated:\x00" as *const u8 as *const i8;
        s_serveroptions.dedicated.itemnames = dedicated_list.as_mut_ptr()
    }
    if s_serveroptions.multiplayer as u64 != 0 {
        y += 16 + 2;
        s_serveroptions.hostname.generic.type_0 = 4;
        s_serveroptions.hostname.generic.name = b"Hostname:\x00" as *const u8 as *const i8;
        s_serveroptions.hostname.generic.flags = 0x2;
        s_serveroptions.hostname.generic.x = 456;
        s_serveroptions.hostname.generic.y = y;
        s_serveroptions.hostname.field.widthInChars = 18;
        s_serveroptions.hostname.field.maxchars = 64
    }
    y += 16 + 2;
    s_serveroptions.punkbuster.generic.type_0 = 3;
    s_serveroptions.punkbuster.generic.name = b"Punkbuster:\x00" as *const u8 as *const i8;
    s_serveroptions.punkbuster.generic.flags = 0x100 | 0x2;
    s_serveroptions.punkbuster.generic.id = 0;
    s_serveroptions.punkbuster.generic.x = 456;
    s_serveroptions.punkbuster.generic.y = y;
    s_serveroptions.punkbuster.itemnames = punkbuster_items.as_mut_ptr();
    y = 80;
    s_serveroptions.botSkill.generic.type_0 = 3;
    s_serveroptions.botSkill.generic.flags = 0x100 | 0x2;
    s_serveroptions.botSkill.generic.name = b"Bot Skill:\x00" as *const u8 as *const i8;
    s_serveroptions.botSkill.generic.x = (32usize).wrapping_add(
        crate::stdlib::strlen(s_serveroptions.botSkill.generic.name)
            .wrapping_add(2usize)
            .wrapping_mul(8usize),
    ) as i32;
    s_serveroptions.botSkill.generic.y = y;
    s_serveroptions.botSkill.itemnames = botSkill_list.as_mut_ptr();
    s_serveroptions.botSkill.curvalue = 1;
    y += 2 * 16;
    s_serveroptions.player0.generic.type_0 = 7;
    s_serveroptions.player0.generic.flags = 0x2;
    s_serveroptions.player0.generic.x = 32 + 8;
    s_serveroptions.player0.generic.y = y;
    s_serveroptions.player0.color = crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr();
    s_serveroptions.player0.style = 0 | 0x10;
    n = 0;
    while n < 12 {
        s_serveroptions.playerType[n as usize].generic.type_0 = 3;
        s_serveroptions.playerType[n as usize].generic.flags = 0x2;
        s_serveroptions.playerType[n as usize].generic.id = 20;
        s_serveroptions.playerType[n as usize].generic.callback =
            Some(ServerOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
        s_serveroptions.playerType[n as usize].generic.x = 32;
        s_serveroptions.playerType[n as usize].generic.y = y;
        s_serveroptions.playerType[n as usize].itemnames = playerType_list.as_mut_ptr();
        s_serveroptions.playerName[n as usize].generic.type_0 = 7;
        s_serveroptions.playerName[n as usize].generic.flags = 0x2;
        s_serveroptions.playerName[n as usize].generic.x = 96;
        s_serveroptions.playerName[n as usize].generic.y = y;
        s_serveroptions.playerName[n as usize].generic.callback = Some(
            ServerOptions_PlayerNameEvent
                as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
        );
        s_serveroptions.playerName[n as usize].generic.id = n;
        s_serveroptions.playerName[n as usize].generic.ownerdraw =
            Some(PlayerName_Draw as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
        s_serveroptions.playerName[n as usize].color =
            crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr();
        s_serveroptions.playerName[n as usize].style = 0x10;
        s_serveroptions.playerName[n as usize].string =
            s_serveroptions.playerNameBuffers[n as usize].as_mut_ptr();
        s_serveroptions.playerName[n as usize].generic.top =
            s_serveroptions.playerName[n as usize].generic.y;
        s_serveroptions.playerName[n as usize].generic.bottom =
            s_serveroptions.playerName[n as usize].generic.y + 16;
        s_serveroptions.playerName[n as usize].generic.left =
            s_serveroptions.playerName[n as usize].generic.x - 16 / 2;
        s_serveroptions.playerName[n as usize].generic.right =
            s_serveroptions.playerName[n as usize].generic.x + 16 * 8;
        s_serveroptions.playerTeam[n as usize].generic.type_0 = 3;
        s_serveroptions.playerTeam[n as usize].generic.flags = 0x2;
        s_serveroptions.playerTeam[n as usize].generic.x = 240;
        s_serveroptions.playerTeam[n as usize].generic.y = y;
        s_serveroptions.playerTeam[n as usize].itemnames = playerTeam_list.as_mut_ptr();
        y += 16 + 4;
        n += 1
    }
    s_serveroptions.back.generic.type_0 = 6;
    s_serveroptions.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    s_serveroptions.back.generic.flags = 0x4 | 0x100;
    s_serveroptions.back.generic.callback =
        Some(ServerOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_serveroptions.back.generic.id = 24;
    s_serveroptions.back.generic.x = 0;
    s_serveroptions.back.generic.y = 480 - 64;
    s_serveroptions.back.width = 128;
    s_serveroptions.back.height = 64;
    s_serveroptions.back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    s_serveroptions.next.generic.type_0 = 6;
    s_serveroptions.next.generic.name = b"menu/art/next_0\x00" as *const u8 as *const i8;
    s_serveroptions.next.generic.flags = 0x10 | 0x100 | 0x4000 | 0x2000 | 0x1000;
    s_serveroptions.next.generic.callback =
        Some(ServerOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_serveroptions.next.generic.id = 18;
    s_serveroptions.next.generic.x = 640;
    s_serveroptions.next.generic.y = 480 - 64 - 72;
    s_serveroptions.next.generic.statusbar =
        Some(ServerOptions_StatusBar as unsafe extern "C" fn(_: *mut libc::c_void) -> ());
    s_serveroptions.next.width = 128;
    s_serveroptions.next.height = 64;
    s_serveroptions.next.focuspic = b"menu/art/next_1\x00" as *const u8 as *mut i8;
    s_serveroptions.go.generic.type_0 = 6;
    s_serveroptions.go.generic.name = b"menu/art/fight_0\x00" as *const u8 as *const i8;
    s_serveroptions.go.generic.flags = 0x10 | 0x100;
    s_serveroptions.go.generic.callback =
        Some(ServerOptions_Event as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> ());
    s_serveroptions.go.generic.id = 23;
    s_serveroptions.go.generic.x = 640;
    s_serveroptions.go.generic.y = 480 - 64;
    s_serveroptions.go.width = 128;
    s_serveroptions.go.height = 64;
    s_serveroptions.go.focuspic = b"menu/art/fight_1\x00" as *const u8 as *mut i8;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.mappic as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.picframe as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.botSkill as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.player0 as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    n = 0;
    while n < 12 {
        if n != 0 {
            crate::src::q3_ui::ui_qmenu::Menu_AddItem(
                &mut s_serveroptions.menu,
                &mut *s_serveroptions.playerType.as_mut_ptr().offset(n as isize)
                    as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
            );
        }
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut *s_serveroptions.playerName.as_mut_ptr().offset(n as isize)
                as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
        );
        if s_serveroptions.gametype >= crate::bg_public_h::GT_TEAM as i32 {
            crate::src::q3_ui::ui_qmenu::Menu_AddItem(
                &mut s_serveroptions.menu,
                &mut *s_serveroptions.playerTeam.as_mut_ptr().offset(n as isize)
                    as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
            );
        }
        n += 1
    }
    if s_serveroptions.gametype != crate::bg_public_h::GT_CTF as i32 {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.fraglimit as *mut crate::ui_local_h::menufield_s
                as *mut libc::c_void,
        );
    } else {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.flaglimit as *mut crate::ui_local_h::menufield_s
                as *mut libc::c_void,
        );
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.timelimit as *mut crate::ui_local_h::menufield_s as *mut libc::c_void,
    );
    if s_serveroptions.gametype >= crate::bg_public_h::GT_TEAM as i32 {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.friendlyfire as *mut crate::ui_local_h::menuradiobutton_s
                as *mut libc::c_void,
        );
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.pure_0 as *mut crate::ui_local_h::menuradiobutton_s
            as *mut libc::c_void,
    );
    if s_serveroptions.multiplayer as u64 != 0 {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.dedicated as *mut crate::ui_local_h::menulist_s
                as *mut libc::c_void,
        );
    }
    if s_serveroptions.multiplayer as u64 != 0 {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut s_serveroptions.menu,
            &mut s_serveroptions.hostname as *mut crate::ui_local_h::menufield_s
                as *mut libc::c_void,
        );
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.next as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.go as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut s_serveroptions.menu,
        &mut s_serveroptions.punkbuster as *mut crate::ui_local_h::menulist_s as *mut libc::c_void,
    );
    ServerOptions_SetMenuItems();
}
/*
=================
ServerOptions_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ServerOptions_Cache() {
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
        b"menu/art/maps_select\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/unknownmap\x00" as *const u8 as *const i8,
    );
}
/*
=================
UI_ServerOptionsMenu
=================
*/

unsafe extern "C" fn UI_ServerOptionsMenu(
    mut multiplayer: crate::src::qcommon::q_shared::qboolean,
) {
    ServerOptions_MenuInit(multiplayer);
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut s_serveroptions.menu);
}

static mut botSelectInfo: botSelectInfo_t = botSelectInfo_t {
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
    pics: [crate::ui_local_h::menubitmap_s {
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
    }; 16],
    picbuttons: [crate::ui_local_h::menubitmap_s {
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
    }; 16],
    picnames: [crate::ui_local_h::menutext_s {
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
    }; 16],
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
    left: crate::ui_local_h::menubitmap_s {
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
    right: crate::ui_local_h::menubitmap_s {
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
    modelpage: 0,
    numpages: 0,
    selectedmodel: 0,
    sortedBotNums: [0; 1024],
    boticons: [[0; 64]; 16],
    botnames: [[0; 16]; 16],
};
/*
=================
UI_BotSelectMenu_SortCompare
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_SortCompare(
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
/*
=================
UI_BotSelectMenu_BuildList
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_BuildList() {
    let mut n: i32 = 0;
    botSelectInfo.modelpage = 0;
    botSelectInfo.numBots = crate::src::q3_ui::ui_gameinfo::UI_GetNumBots();
    botSelectInfo.numpages = botSelectInfo.numBots / (4 * 4);
    if botSelectInfo.numBots % (4 * 4) != 0 {
        botSelectInfo.numpages += 1
    }

    for n in 0..botSelectInfo.numBots {
        botSelectInfo.sortedBotNums[n as usize] = n;
    }
    // now sort it
    crate::stdlib::qsort(
        botSelectInfo.sortedBotNums.as_mut_ptr() as *mut libc::c_void,
        botSelectInfo.numBots as crate::stddef_h::size_t,
        ::std::mem::size_of::<i32>(),
        Some(
            UI_BotSelectMenu_SortCompare
                as unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> i32,
        ),
    );
}
/*
=================
ServerPlayerIcon
=================
*/

unsafe extern "C" fn ServerPlayerIcon(
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
        let fresh1 = skin;
        skin = skin.offset(1);
        *fresh1 = '\u{0}' as i8
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
UI_BotSelectMenu_UpdateGrid
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_UpdateGrid() {
    let mut info: *const i8 = 0 as *const i8;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    j = botSelectInfo.modelpage * (4 * 4);
    i = 0;
    while i < 4 * 4 {
        if j < botSelectInfo.numBots {
            info = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber(
                botSelectInfo.sortedBotNums[j as usize],
            );
            ServerPlayerIcon(
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    info,
                    b"model\x00" as *const u8 as *const i8,
                ),
                botSelectInfo.boticons[i as usize].as_mut_ptr(),
                64,
            );
            crate::src::qcommon::q_shared::Q_strncpyz(
                botSelectInfo.botnames[i as usize].as_mut_ptr(),
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    info,
                    b"name\x00" as *const u8 as *const i8,
                ),
                16,
            );
            crate::src::qcommon::q_shared::Q_CleanStr(
                botSelectInfo.botnames[i as usize].as_mut_ptr(),
            );
            botSelectInfo.pics[i as usize].generic.name =
                botSelectInfo.boticons[i as usize].as_mut_ptr();
            if BotAlreadySelected(botSelectInfo.botnames[i as usize].as_mut_ptr()) as u64 != 0 {
                botSelectInfo.picnames[i as usize].color =
                    crate::src::q3_ui::ui_qmenu::color_red.as_mut_ptr()
            } else {
                botSelectInfo.picnames[i as usize].color =
                    crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr()
            }
            botSelectInfo.picbuttons[i as usize].generic.flags &= !(0x4000)
        } else {
            // dead slot
            botSelectInfo.pics[i as usize].generic.name = 0 as *const i8;
            botSelectInfo.picbuttons[i as usize].generic.flags |= 0x4000;
            botSelectInfo.botnames[i as usize][0] = 0
        }
        botSelectInfo.pics[i as usize].generic.flags &= !(0x40);
        botSelectInfo.pics[i as usize].shader = 0;
        botSelectInfo.picbuttons[i as usize].generic.flags |= 0x100;
        i += 1;
        j += 1
    }
    // set selected model
    i = botSelectInfo.selectedmodel % (4 * 4);
    botSelectInfo.pics[i as usize].generic.flags |= 0x40;
    botSelectInfo.picbuttons[i as usize].generic.flags &= !(0x100);
    if botSelectInfo.numpages > 1 {
        if botSelectInfo.modelpage > 0 {
            botSelectInfo.left.generic.flags &= !(0x4000)
        } else {
            botSelectInfo.left.generic.flags |= 0x4000
        }
        if botSelectInfo.modelpage < botSelectInfo.numpages - 1 {
            botSelectInfo.right.generic.flags &= !(0x4000)
        } else {
            botSelectInfo.right.generic.flags |= 0x4000
        }
    } else {
        // hide left/right markers
        botSelectInfo.left.generic.flags |= 0x4000;
        botSelectInfo.right.generic.flags |= 0x4000
    };
}
/*
=================
UI_BotSelectMenu_Default
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_Default(mut bot: *mut i8) {
    let mut botInfo: *const i8 = 0 as *const i8;
    let mut test: *const i8 = 0 as *const i8;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    n = 0;
    while n < botSelectInfo.numBots {
        botInfo = crate::src::q3_ui::ui_gameinfo::UI_GetBotInfoByNumber(n);
        test = crate::src::qcommon::q_shared::Info_ValueForKey(
            botInfo,
            b"name\x00" as *const u8 as *const i8,
        );
        if crate::src::qcommon::q_shared::Q_stricmp(bot, test) == 0 {
            break;
        }
        n += 1
    }
    if n == botSelectInfo.numBots {
        botSelectInfo.selectedmodel = 0;
        return;
    }
    i = 0;
    while i < botSelectInfo.numBots {
        if botSelectInfo.sortedBotNums[i as usize] == n {
            break;
        }
        i += 1
    }
    if i == botSelectInfo.numBots {
        botSelectInfo.selectedmodel = 0;
        return;
    }
    botSelectInfo.selectedmodel = i;
}
/*
=================
UI_BotSelectMenu_LeftEvent
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_LeftEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    if botSelectInfo.modelpage > 0 {
        botSelectInfo.modelpage -= 1;
        botSelectInfo.selectedmodel = botSelectInfo.modelpage * (4 * 4);
        UI_BotSelectMenu_UpdateGrid();
    };
}
/*
=================
UI_BotSelectMenu_RightEvent
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_RightEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    if botSelectInfo.modelpage < botSelectInfo.numpages - 1 {
        botSelectInfo.modelpage += 1;
        botSelectInfo.selectedmodel = botSelectInfo.modelpage * (4 * 4);
        UI_BotSelectMenu_UpdateGrid();
    };
}
/*
=================
UI_BotSelectMenu_BotEvent
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_BotEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    let mut i: i32 = 0;
    if event != 3 {
        return;
    }
    i = 0;
    while i < 4 * 4 {
        botSelectInfo.pics[i as usize].generic.flags &= !(0x40);
        botSelectInfo.picbuttons[i as usize].generic.flags |= 0x100;
        i += 1
    }
    // set selected
    i = (*(ptr as *mut crate::ui_local_h::menucommon_s)).id;
    botSelectInfo.pics[i as usize].generic.flags |= 0x40;
    botSelectInfo.picbuttons[i as usize].generic.flags &= !(0x100);
    botSelectInfo.selectedmodel = botSelectInfo.modelpage * (4 * 4) + i;
}
/*
=================
UI_BotSelectMenu_BackEvent
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_BackEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
}
/*
=================
UI_BotSelectMenu_SelectEvent
=================
*/

unsafe extern "C" fn UI_BotSelectMenu_SelectEvent(mut ptr: *mut libc::c_void, mut event: i32) {
    if event != 3 {
        return;
    }
    crate::src::q3_ui::ui_atoms::UI_PopMenu();
    s_serveroptions.newBot = crate::src::qcommon::q_shared::qtrue;
    crate::src::qcommon::q_shared::Q_strncpyz(
        s_serveroptions.newBotName.as_mut_ptr(),
        botSelectInfo.botnames[(botSelectInfo.selectedmodel % (4 * 4)) as usize].as_mut_ptr(),
        16,
    );
}
/*
=================
UI_BotSelectMenu_Cache
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_BotSelectMenu_Cache() {
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
        b"menu/art/opponents_select\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/opponents_selected\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/gs_arrows_0\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/gs_arrows_l\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_R_RegisterShaderNoMip(
        b"menu/art/gs_arrows_r\x00" as *const u8 as *const i8,
    );
}

unsafe extern "C" fn UI_BotSelectMenu_Init(mut bot: *mut i8) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    crate::stdlib::memset(
        &mut botSelectInfo as *mut botSelectInfo_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<botSelectInfo_t>(),
    );
    botSelectInfo.menu.wrapAround = crate::src::qcommon::q_shared::qtrue;
    botSelectInfo.menu.fullscreen = crate::src::qcommon::q_shared::qtrue;
    UI_BotSelectMenu_Cache();
    botSelectInfo.banner.generic.type_0 = 10;
    botSelectInfo.banner.generic.x = 320;
    botSelectInfo.banner.generic.y = 16;
    botSelectInfo.banner.string = b"SELECT BOT\x00" as *const u8 as *mut i8;
    botSelectInfo.banner.color = crate::src::q3_ui::ui_qmenu::color_white.as_mut_ptr();
    botSelectInfo.banner.style = 0x1;
    y = 80;
    i = 0;
    k = 0;
    while i < 4 {
        x = 180;
        j = 0;
        while j < 4 {
            botSelectInfo.pics[k as usize].generic.type_0 = 6;
            botSelectInfo.pics[k as usize].generic.flags = 0x4 | 0x4000;
            botSelectInfo.pics[k as usize].generic.x = x;
            botSelectInfo.pics[k as usize].generic.y = y;
            botSelectInfo.pics[k as usize].generic.name =
                botSelectInfo.boticons[k as usize].as_mut_ptr();
            botSelectInfo.pics[k as usize].width = 64;
            botSelectInfo.pics[k as usize].height = 64;
            botSelectInfo.pics[k as usize].focuspic =
                b"menu/art/opponents_selected\x00" as *const u8 as *mut i8;
            botSelectInfo.pics[k as usize].focuscolor =
                crate::src::qcommon::q_math::colorRed.as_mut_ptr();
            botSelectInfo.picbuttons[k as usize].generic.type_0 = 6;
            botSelectInfo.picbuttons[k as usize].generic.flags = 0x4 | 0x8000 | 0x100;
            botSelectInfo.picbuttons[k as usize].generic.callback = Some(
                UI_BotSelectMenu_BotEvent
                    as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
            );
            botSelectInfo.picbuttons[k as usize].generic.id = k;
            botSelectInfo.picbuttons[k as usize].generic.x = x - 16;
            botSelectInfo.picbuttons[k as usize].generic.y = y - 16;
            botSelectInfo.picbuttons[k as usize].generic.left = x;
            botSelectInfo.picbuttons[k as usize].generic.top = y;
            botSelectInfo.picbuttons[k as usize].generic.right = x + 64;
            botSelectInfo.picbuttons[k as usize].generic.bottom = y + 64;
            botSelectInfo.picbuttons[k as usize].width = 128;
            botSelectInfo.picbuttons[k as usize].height = 128;
            botSelectInfo.picbuttons[k as usize].focuspic =
                b"menu/art/opponents_select\x00" as *const u8 as *mut i8;
            botSelectInfo.picbuttons[k as usize].focuscolor =
                crate::src::qcommon::q_math::colorRed.as_mut_ptr();
            botSelectInfo.picnames[k as usize].generic.type_0 = 7;
            botSelectInfo.picnames[k as usize].generic.flags = 0x2;
            botSelectInfo.picnames[k as usize].generic.x = x + 32;
            botSelectInfo.picnames[k as usize].generic.y = y + 64;
            botSelectInfo.picnames[k as usize].string =
                botSelectInfo.botnames[k as usize].as_mut_ptr();
            botSelectInfo.picnames[k as usize].color =
                crate::src::q3_ui::ui_qmenu::color_orange.as_mut_ptr();
            botSelectInfo.picnames[k as usize].style = 0x1 | 0x10;
            x += 64 + 6;
            j += 1;
            k += 1
        }
        y += 64 + 16 + 6;
        i += 1
    }
    botSelectInfo.arrows.generic.type_0 = 6;
    botSelectInfo.arrows.generic.name = b"menu/art/gs_arrows_0\x00" as *const u8 as *const i8;
    botSelectInfo.arrows.generic.flags = 0x4000;
    botSelectInfo.arrows.generic.x = 260;
    botSelectInfo.arrows.generic.y = 440;
    botSelectInfo.arrows.width = 128;
    botSelectInfo.arrows.height = 32;
    botSelectInfo.left.generic.type_0 = 6;
    botSelectInfo.left.generic.flags = 0x4 | 0x100;
    botSelectInfo.left.generic.callback = Some(
        UI_BotSelectMenu_LeftEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    botSelectInfo.left.generic.x = 260;
    botSelectInfo.left.generic.y = 440;
    botSelectInfo.left.width = 64;
    botSelectInfo.left.height = 32;
    botSelectInfo.left.focuspic = b"menu/art/gs_arrows_l\x00" as *const u8 as *mut i8;
    botSelectInfo.right.generic.type_0 = 6;
    botSelectInfo.right.generic.flags = 0x4 | 0x100;
    botSelectInfo.right.generic.callback = Some(
        UI_BotSelectMenu_RightEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    botSelectInfo.right.generic.x = 321;
    botSelectInfo.right.generic.y = 440;
    botSelectInfo.right.width = 64;
    botSelectInfo.right.height = 32;
    botSelectInfo.right.focuspic = b"menu/art/gs_arrows_r\x00" as *const u8 as *mut i8;
    botSelectInfo.back.generic.type_0 = 6;
    botSelectInfo.back.generic.name = b"menu/art/back_0\x00" as *const u8 as *const i8;
    botSelectInfo.back.generic.flags = 0x4 | 0x100;
    botSelectInfo.back.generic.callback = Some(
        UI_BotSelectMenu_BackEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    botSelectInfo.back.generic.x = 0;
    botSelectInfo.back.generic.y = 480 - 64;
    botSelectInfo.back.width = 128;
    botSelectInfo.back.height = 64;
    botSelectInfo.back.focuspic = b"menu/art/back_1\x00" as *const u8 as *mut i8;
    botSelectInfo.go.generic.type_0 = 6;
    botSelectInfo.go.generic.name = b"menu/art/accept_0\x00" as *const u8 as *const i8;
    botSelectInfo.go.generic.flags = 0x10 | 0x100;
    botSelectInfo.go.generic.callback = Some(
        UI_BotSelectMenu_SelectEvent as unsafe extern "C" fn(_: *mut libc::c_void, _: i32) -> (),
    );
    botSelectInfo.go.generic.x = 640;
    botSelectInfo.go.generic.y = 480 - 64;
    botSelectInfo.go.width = 128;
    botSelectInfo.go.height = 64;
    botSelectInfo.go.focuspic = b"menu/art/accept_1\x00" as *const u8 as *mut i8;
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.banner as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
    );
    i = 0;
    while i < 4 * 4 {
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut botSelectInfo.menu,
            &mut *botSelectInfo.pics.as_mut_ptr().offset(i as isize)
                as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
        );
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut botSelectInfo.menu,
            &mut *botSelectInfo.picbuttons.as_mut_ptr().offset(i as isize)
                as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
        );
        crate::src::q3_ui::ui_qmenu::Menu_AddItem(
            &mut botSelectInfo.menu,
            &mut *botSelectInfo.picnames.as_mut_ptr().offset(i as isize)
                as *mut crate::ui_local_h::menutext_s as *mut libc::c_void,
        );
        i += 1
    }
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.arrows as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.left as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.right as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.back as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    crate::src::q3_ui::ui_qmenu::Menu_AddItem(
        &mut botSelectInfo.menu,
        &mut botSelectInfo.go as *mut crate::ui_local_h::menubitmap_s as *mut libc::c_void,
    );
    UI_BotSelectMenu_BuildList();
    UI_BotSelectMenu_Default(bot);
    botSelectInfo.modelpage = botSelectInfo.selectedmodel / (4 * 4);
    UI_BotSelectMenu_UpdateGrid();
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
/*
=================
UI_BotSelectMenu
=================
*/
#[no_mangle]

pub unsafe extern "C" fn UI_BotSelectMenu(mut bot: *mut i8) {
    UI_BotSelectMenu_Init(bot);
    crate::src::q3_ui::ui_atoms::UI_PushMenu(&mut botSelectInfo.menu);
}
