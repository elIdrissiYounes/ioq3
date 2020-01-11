use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::src::q3_ui::ui_atoms::uis;
pub use crate::src::q3_ui::ui_atoms::Com_Printf;
pub use crate::src::q3_ui::ui_gameinfo::stdlib_h::atoi;
pub use crate::src::q3_ui::ui_splevel::UI_SPLevelMenu_ReInit;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::sfxHandle_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::C2RustUnnamed_0;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::COM_ParseExt;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Register;
pub use crate::src::ui::ui_syscalls::trap_Cvar_Set;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::ui::ui_syscalls::trap_Cvar_VariableValue;
pub use crate::src::ui::ui_syscalls::trap_FS_FCloseFile;
pub use crate::src::ui::ui_syscalls::trap_FS_FOpenFile;
pub use crate::src::ui::ui_syscalls::trap_FS_GetFileList;
pub use crate::src::ui::ui_syscalls::trap_FS_Read;
pub use crate::src::ui::ui_syscalls::trap_Print;
use crate::stdlib::strcat;
use crate::stdlib::strcmp;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
use crate::stdlib::strstr;
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
pub use crate::ui_local_h::menuframework_s;
pub use crate::ui_local_h::uiStatic_t;
pub use crate::ui_local_h::AWARD_ACCURACY;
pub use crate::ui_local_h::AWARD_EXCELLENT;
pub use crate::ui_local_h::AWARD_FRAGS;
pub use crate::ui_local_h::AWARD_GAUNTLET;
pub use crate::ui_local_h::AWARD_IMPRESSIVE;
pub use crate::ui_local_h::AWARD_PERFECT;
#[no_mangle]

pub static mut ui_numBots: i32 = 0;

static mut ui_botInfos: [*mut i8; 1024] = [0 as *mut i8; 1024];

static mut ui_numArenas: i32 = 0;

static mut ui_arenaInfos: [*mut i8; 1024] = [0 as *mut i8; 1024];

static mut ui_numSinglePlayerArenas: i32 = 0;

static mut ui_numSpecialSinglePlayerArenas: i32 = 0;

static mut memoryPool: [i8; 131072] = [0; 131072];

static mut allocPoint: i32 = 0;

static mut outOfMemory: i32 = 0;
/*
===============
UI_Alloc
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_Alloc(mut size: i32) -> *mut libc::c_void {
    let mut p: *mut i8 = 0 as *mut i8;
    if allocPoint + size > 128 * 1024 {
        outOfMemory = crate::src::qcommon::q_shared::qtrue as i32;
        return 0 as *mut libc::c_void;
    }
    p = &mut *memoryPool.as_mut_ptr().offset(allocPoint as isize) as *mut i8;
    allocPoint += size + 31 & !(31);
    return p as *mut libc::c_void;
}
/*
===============
UI_InitMemory
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_InitMemory() {
    allocPoint = 0;
    outOfMemory = crate::src::qcommon::q_shared::qfalse as i32;
}
/*
===============
UI_ParseInfos
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ParseInfos(
    mut buf: *mut i8,
    mut max: i32,
    mut infos: *mut *mut i8,
) -> i32 {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut count: i32 = 0;
    let mut key: [i8; 1024] = [0; 1024];
    let mut info: [i8; 1024] = [0; 1024];
    count = 0;
    loop {
        token = crate::src::qcommon::q_shared::COM_Parse(&mut buf);
        if *token.offset(0) == 0 {
            break;
        }
        if crate::stdlib::strcmp(token, b"{\x00" as *const u8 as *const i8) != 0 {
            crate::src::q3_ui::ui_atoms::Com_Printf(
                b"Missing { in info file\n\x00" as *const u8 as *const i8,
            );
            break;
        } else if count == max {
            crate::src::q3_ui::ui_atoms::Com_Printf(
                b"Max infos exceeded\n\x00" as *const u8 as *const i8,
            );
            break;
        } else {
            info[0] = '\u{0}' as i8;
            loop {
                token = crate::src::qcommon::q_shared::COM_ParseExt(
                    &mut buf,
                    crate::src::qcommon::q_shared::qtrue,
                );
                if *token.offset(0) == 0 {
                    crate::src::q3_ui::ui_atoms::Com_Printf(
                        b"Unexpected end of info file\n\x00" as *const u8 as *const i8,
                    );
                    break;
                } else {
                    if crate::stdlib::strcmp(token, b"}\x00" as *const u8 as *const i8) == 0 {
                        break;
                    }
                    crate::src::qcommon::q_shared::Q_strncpyz(
                        key.as_mut_ptr(),
                        token,
                        ::std::mem::size_of::<[i8; 1024]>() as i32,
                    );
                    token = crate::src::qcommon::q_shared::COM_ParseExt(
                        &mut buf,
                        crate::src::qcommon::q_shared::qfalse,
                    );
                    if *token.offset(0) == 0 {
                        crate::stdlib::strcpy(token, b"<NULL>\x00" as *const u8 as *const i8);
                    }
                    crate::src::qcommon::q_shared::Info_SetValueForKey(
                        info.as_mut_ptr(),
                        key.as_mut_ptr(),
                        token,
                    );
                }
            }
            //NOTE: extra space for arena number
            let ref mut fresh0 = *infos.offset(count as isize);
            *fresh0 = UI_Alloc(
                crate::stdlib::strlen(info.as_mut_ptr())
                    .wrapping_add(crate::stdlib::strlen(
                        b"\\num\\\x00" as *const u8 as *const i8,
                    ))
                    .wrapping_add(crate::stdlib::strlen(crate::src::qcommon::q_shared::va(
                        b"%d\x00" as *const u8 as *mut i8,
                        1024i32,
                    )))
                    .wrapping_add(1usize) as i32,
            ) as *mut i8;
            if !(*infos.offset(count as isize)).is_null() {
                crate::stdlib::strcpy(*infos.offset(count as isize), info.as_mut_ptr());
                count += 1
            }
        }
    }
    return count;
}
/*
===============
UI_LoadArenasFromFile
===============
*/

unsafe extern "C" fn UI_LoadArenasFromFile(mut filename: *mut i8) {
    let mut len: i32 = 0;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut buf: [i8; 8192] = [0; 8192];
    len = crate::src::ui::ui_syscalls::trap_FS_FOpenFile(
        filename,
        &mut f,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if f == 0 {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file not found: %s\n\x00" as *const u8 as *mut i8,
            filename,
        ));
        return;
    }
    if len >= 8192 {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8 as *mut i8,
            filename,
            len,
            8192i32,
        ));
        crate::src::ui::ui_syscalls::trap_FS_FCloseFile(f);
        return;
    }
    crate::src::ui::ui_syscalls::trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0;
    crate::src::ui::ui_syscalls::trap_FS_FCloseFile(f);
    ui_numArenas += UI_ParseInfos(
        buf.as_mut_ptr(),
        1024 - ui_numArenas,
        &mut *ui_arenaInfos.as_mut_ptr().offset(ui_numArenas as isize),
    );
}
/*
===============
UI_LoadArenas
===============
*/

unsafe extern "C" fn UI_LoadArenas() {
    let mut numdirs: i32 = 0;
    let mut arenasFile: crate::src::qcommon::q_shared::vmCvar_t =
        crate::src::qcommon::q_shared::vmCvar_t {
            handle: 0,
            modificationCount: 0,
            value: 0.,
            integer: 0,
            string: [0; 256],
        };
    let mut filename: [i8; 128] = [0; 128];
    let mut dirlist: [i8; 4096] = [0; 4096];
    let mut dirptr: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut dirlen: i32 = 0;
    let mut type_0: *mut i8 = 0 as *mut i8;
    let mut tag: *mut i8 = 0 as *mut i8;
    let mut singlePlayerNum: i32 = 0;
    let mut specialNum: i32 = 0;
    let mut otherNum: i32 = 0;
    ui_numArenas = 0;
    crate::src::ui::ui_syscalls::trap_Cvar_Register(
        &mut arenasFile,
        b"g_arenasFile\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        0x10 | 0x40,
    );
    if *arenasFile.string.as_mut_ptr() != 0 {
        UI_LoadArenasFromFile(arenasFile.string.as_mut_ptr());
    } else {
        UI_LoadArenasFromFile(b"scripts/arenas.txt\x00" as *const u8 as *mut i8);
    }
    // get all arenas from .arena files
    numdirs = crate::src::ui::ui_syscalls::trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const i8,
        b".arena\x00" as *const u8 as *const i8,
        dirlist.as_mut_ptr(),
        4096,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0;
    while i < numdirs {
        dirlen = crate::stdlib::strlen(dirptr) as i32;
        crate::stdlib::strcpy(
            filename.as_mut_ptr(),
            b"scripts/\x00" as *const u8 as *const i8,
        );
        crate::stdlib::strcat(filename.as_mut_ptr(), dirptr);
        UI_LoadArenasFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1) as isize)
    }
    crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
        b"%i arenas parsed\n\x00" as *const u8 as *mut i8,
        ui_numArenas,
    ));
    if outOfMemory != 0 {
        crate::src::ui::ui_syscalls::trap_Print(
            b"^3WARNING: not enough memory in pool to load all arenas\n\x00" as *const u8
                as *const i8,
        );
    }
    // set initial numbers
    n = 0;
    while n < ui_numArenas {
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            ui_arenaInfos[n as usize],
            b"num\x00" as *const u8 as *const i8,
            crate::src::qcommon::q_shared::va(b"%i\x00" as *const u8 as *mut i8, n),
        );
        n += 1
    }
    // go through and count single players levels
    ui_numSinglePlayerArenas = 0;
    ui_numSpecialSinglePlayerArenas = 0;
    n = 0;
    while n < ui_numArenas {
        // determine type
        type_0 = crate::src::qcommon::q_shared::Info_ValueForKey(
            ui_arenaInfos[n as usize],
            b"type\x00" as *const u8 as *const i8,
        );
        // if no type specified, it will be treated as "ffa"
        if !(*type_0 == 0) {
            if !crate::stdlib::strstr(type_0, b"single\x00" as *const u8 as *const i8).is_null() {
                // check for special single player arenas (training, final)
                tag = crate::src::qcommon::q_shared::Info_ValueForKey(
                    ui_arenaInfos[n as usize],
                    b"special\x00" as *const u8 as *const i8,
                );
                if *tag != 0 {
                    ui_numSpecialSinglePlayerArenas += 1
                } else {
                    ui_numSinglePlayerArenas += 1
                }
            }
        }
        n += 1
    }
    n = ui_numSinglePlayerArenas % 4;
    if n != 0 {
        ui_numSinglePlayerArenas -= n;
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"%i arenas ignored to make count divisible by %i\n\x00" as *const u8 as *mut i8,
            n,
            4i32,
        ));
    }
    // go through once more and assign number to the levels
    singlePlayerNum = 0;
    specialNum = singlePlayerNum + ui_numSinglePlayerArenas;
    otherNum = specialNum + ui_numSpecialSinglePlayerArenas;
    let mut current_block_44: u64;
    n = 0;
    while n < ui_numArenas {
        // determine type
        type_0 = crate::src::qcommon::q_shared::Info_ValueForKey(
            ui_arenaInfos[n as usize],
            b"type\x00" as *const u8 as *const i8,
        );
        // if no type specified, it will be treated as "ffa"
        if *type_0 != 0 {
            if !crate::stdlib::strstr(type_0, b"single\x00" as *const u8 as *const i8).is_null() {
                // check for special single player arenas (training, final)
                tag = crate::src::qcommon::q_shared::Info_ValueForKey(
                    ui_arenaInfos[n as usize],
                    b"special\x00" as *const u8 as *const i8,
                );
                if *tag != 0 {
                    let fresh1 = specialNum;
                    specialNum = specialNum + 1;
                    crate::src::qcommon::q_shared::Info_SetValueForKey(
                        ui_arenaInfos[n as usize],
                        b"num\x00" as *const u8 as *const i8,
                        crate::src::qcommon::q_shared::va(
                            b"%i\x00" as *const u8 as *mut i8,
                            fresh1,
                        ),
                    );
                } else {
                    let fresh2 = singlePlayerNum;
                    singlePlayerNum = singlePlayerNum + 1;
                    crate::src::qcommon::q_shared::Info_SetValueForKey(
                        ui_arenaInfos[n as usize],
                        b"num\x00" as *const u8 as *const i8,
                        crate::src::qcommon::q_shared::va(
                            b"%i\x00" as *const u8 as *mut i8,
                            fresh2,
                        ),
                    );
                }
                current_block_44 = 14775119014532381840;
            } else {
                current_block_44 = 16415152177862271243;
            }
        } else {
            current_block_44 = 16415152177862271243;
        }
        match current_block_44 {
            16415152177862271243 => {
                let fresh3 = otherNum;
                otherNum = otherNum + 1;
                crate::src::qcommon::q_shared::Info_SetValueForKey(
                    ui_arenaInfos[n as usize],
                    b"num\x00" as *const u8 as *const i8,
                    crate::src::qcommon::q_shared::va(b"%i\x00" as *const u8 as *mut i8, fresh3),
                );
            }
            _ => {}
        }
        n += 1
    }
}
/*
===============
UI_GetArenaInfoByNumber
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetArenaInfoByNumber(mut num: i32) -> *const i8 {
    let mut n: i32 = 0;
    let mut value: *mut i8 = 0 as *mut i8;
    if num < 0 || num >= ui_numArenas {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1Invalid arena number: %i\n\x00" as *const u8 as *mut i8,
            num,
        ));
        return 0 as *const i8;
    }

    for n in 0..ui_numArenas {
        value = crate::src::qcommon::q_shared::Info_ValueForKey(
            ui_arenaInfos[n as usize],
            b"num\x00" as *const u8 as *const i8,
        );

        if *value as i32 != 0 && atoi(value) == num {
            return ui_arenaInfos[n as usize];
        }
    }
    return 0 as *const i8;
}
/*
===============
UI_GetArenaInfoByNumber
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetArenaInfoByMap(mut map: *const i8) -> *const i8 {
    let mut n: i32 = 0;

    for n in 0..ui_numArenas {
        if crate::src::qcommon::q_shared::Q_stricmp(
            crate::src::qcommon::q_shared::Info_ValueForKey(
                ui_arenaInfos[n as usize],
                b"map\x00" as *const u8 as *const i8,
            ),
            map,
        ) == 0
        {
            return ui_arenaInfos[n as usize];
        }
    }
    return 0 as *const i8;
}
/*
===============
UI_GetSpecialArenaInfo
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetSpecialArenaInfo(mut tag: *const i8) -> *const i8 {
    let mut n: i32 = 0;

    for n in 0..ui_numArenas {
        if crate::src::qcommon::q_shared::Q_stricmp(
            crate::src::qcommon::q_shared::Info_ValueForKey(
                ui_arenaInfos[n as usize],
                b"special\x00" as *const u8 as *const i8,
            ),
            tag,
        ) == 0
        {
            return ui_arenaInfos[n as usize];
        }
    }
    return 0 as *const i8;
}
/*
===============
UI_LoadBotsFromFile
===============
*/

unsafe extern "C" fn UI_LoadBotsFromFile(mut filename: *mut i8) {
    let mut len: i32 = 0;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut buf: [i8; 8192] = [0; 8192];
    len = crate::src::ui::ui_syscalls::trap_FS_FOpenFile(
        filename,
        &mut f,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if f == 0 {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file not found: %s\n\x00" as *const u8 as *mut i8,
            filename,
        ));
        return;
    }
    if len >= 8192 {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8 as *mut i8,
            filename,
            len,
            8192i32,
        ));
        crate::src::ui::ui_syscalls::trap_FS_FCloseFile(f);
        return;
    }
    crate::src::ui::ui_syscalls::trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0;
    crate::src::ui::ui_syscalls::trap_FS_FCloseFile(f);
    ui_numBots += UI_ParseInfos(
        buf.as_mut_ptr(),
        1024 - ui_numBots,
        &mut *ui_botInfos.as_mut_ptr().offset(ui_numBots as isize),
    );
    if outOfMemory != 0 {
        crate::src::ui::ui_syscalls::trap_Print(
            b"^3WARNING: not enough memory in pool to load all bots\n\x00" as *const u8
                as *const i8,
        );
    };
}
/*
===============
UI_LoadBots
===============
*/

unsafe extern "C" fn UI_LoadBots() {
    let mut botsFile: crate::src::qcommon::q_shared::vmCvar_t =
        crate::src::qcommon::q_shared::vmCvar_t {
            handle: 0,
            modificationCount: 0,
            value: 0.,
            integer: 0,
            string: [0; 256],
        };
    let mut numdirs: i32 = 0;
    let mut filename: [i8; 128] = [0; 128];
    let mut dirlist: [i8; 1024] = [0; 1024];
    let mut dirptr: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut dirlen: i32 = 0;
    ui_numBots = 0;
    crate::src::ui::ui_syscalls::trap_Cvar_Register(
        &mut botsFile,
        b"g_botsFile\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        0x10 | 0x40,
    );
    if *botsFile.string.as_mut_ptr() != 0 {
        UI_LoadBotsFromFile(botsFile.string.as_mut_ptr());
    } else {
        UI_LoadBotsFromFile(b"scripts/bots.txt\x00" as *const u8 as *mut i8);
    }
    // get all bots from .bot files
    numdirs = crate::src::ui::ui_syscalls::trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const i8,
        b".bot\x00" as *const u8 as *const i8,
        dirlist.as_mut_ptr(),
        1024,
    );
    dirptr = dirlist.as_mut_ptr();
    i = 0;
    while i < numdirs {
        dirlen = crate::stdlib::strlen(dirptr) as i32;
        crate::stdlib::strcpy(
            filename.as_mut_ptr(),
            b"scripts/\x00" as *const u8 as *const i8,
        );
        crate::stdlib::strcat(filename.as_mut_ptr(), dirptr);
        UI_LoadBotsFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1) as isize)
    }
    crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
        b"%i bots parsed\n\x00" as *const u8 as *mut i8,
        ui_numBots,
    ));
}
/*
===============
UI_GetBotInfoByNumber
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetBotInfoByNumber(mut num: i32) -> *mut i8 {
    if num < 0 || num >= ui_numBots {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1Invalid bot number: %i\n\x00" as *const u8 as *mut i8,
            num,
        ));
        return 0 as *mut i8;
    }
    return ui_botInfos[num as usize];
}
/*
===============
UI_GetBotInfoByName
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetBotInfoByName(mut name: *const i8) -> *mut i8 {
    let mut n: i32 = 0;
    let mut value: *mut i8 = 0 as *mut i8;

    for n in 0..ui_numBots {
        value = crate::src::qcommon::q_shared::Info_ValueForKey(
            ui_botInfos[n as usize],
            b"name\x00" as *const u8 as *const i8,
        );

        if crate::src::qcommon::q_shared::Q_stricmp(value, name) == 0 {
            return ui_botInfos[n as usize];
        }
    }
    return 0 as *mut i8;
}
//
// single player game info
//
/*
===============
UI_GetBestScore

Returns the player's best finish on a given level, 0 if the have not played the level
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetBestScore(mut level: i32, mut score: *mut i32, mut skill: *mut i32) {
    let mut n: i32 = 0;
    let mut skillScore: i32 = 0;
    let mut bestScore: i32 = 0;
    let mut bestScoreSkill: i32 = 0;
    let mut arenaKey: [i8; 16] = [0; 16];
    let mut scores: [i8; 1024] = [0; 1024];
    if score.is_null() || skill.is_null() {
        return;
    }
    if level < 0 || level > ui_numArenas {
        return;
    }
    bestScore = 0;
    bestScoreSkill = 0;

    for n in 1..=5 {
        crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
            crate::src::qcommon::q_shared::va(b"g_spScores%i\x00" as *const u8 as *mut i8, n),
            scores.as_mut_ptr(),
            1024,
        );

        crate::src::qcommon::q_shared::Com_sprintf(
            arenaKey.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 16]>() as i32,
            b"l%i\x00" as *const u8 as *const i8,
            level,
        );

        skillScore = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            scores.as_mut_ptr(),
            arenaKey.as_mut_ptr(),
        ));

        if !(skillScore < 1 || skillScore > 8) {
            if bestScore == 0 || skillScore <= bestScore {
                bestScore = skillScore;
                bestScoreSkill = n
            }
        }
    }
    *score = bestScore;
    *skill = bestScoreSkill;
}
/*
===============
UI_SetBestScore

Set the player's best finish for a level
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SetBestScore(mut level: i32, mut score: i32) {
    let mut skill: i32 = 0;
    let mut oldScore: i32 = 0;
    let mut arenaKey: [i8; 16] = [0; 16];
    let mut scores: [i8; 1024] = [0; 1024];
    // validate score
    if score < 1 || score > 8 {
        return;
    }
    // validate skill
    skill = crate::src::ui::ui_syscalls::trap_Cvar_VariableValue(
        b"g_spSkill\x00" as *const u8 as *const i8,
    ) as i32;
    if skill < 1 || skill > 5 {
        return;
    }
    // get scores
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        crate::src::qcommon::q_shared::va(b"g_spScores%i\x00" as *const u8 as *mut i8, skill),
        scores.as_mut_ptr(),
        1024,
    );
    // see if this is better
    crate::src::qcommon::q_shared::Com_sprintf(
        arenaKey.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 16]>() as i32,
        b"l%i\x00" as *const u8 as *const i8,
        level,
    );
    oldScore = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        scores.as_mut_ptr(),
        arenaKey.as_mut_ptr(),
    ));
    if oldScore != 0 && oldScore <= score {
        return;
    }
    // update scores
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        scores.as_mut_ptr(),
        arenaKey.as_mut_ptr(),
        crate::src::qcommon::q_shared::va(b"%i\x00" as *const u8 as *mut i8, score),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        crate::src::qcommon::q_shared::va(b"g_spScores%i\x00" as *const u8 as *mut i8, skill),
        scores.as_mut_ptr(),
    );
}
/*
===============
UI_LogAwardData
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_LogAwardData(mut award: i32, mut data: i32) {
    let mut key: [i8; 16] = [0; 16];
    let mut awardData: [i8; 1024] = [0; 1024];
    let mut oldValue: i32 = 0;
    if data == 0 {
        return;
    }
    if award > crate::ui_local_h::AWARD_PERFECT as i32 {
        crate::src::ui::ui_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1Bad award %i in UI_LogAwardData\n\x00" as *const u8 as *mut i8,
            award,
        ));
        return;
    }
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spAwards\x00" as *const u8 as *const i8,
        awardData.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 16]>() as i32,
        b"a%i\x00" as *const u8 as *const i8,
        award,
    );
    oldValue = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        awardData.as_mut_ptr(),
        key.as_mut_ptr(),
    ));
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        awardData.as_mut_ptr(),
        key.as_mut_ptr(),
        crate::src::qcommon::q_shared::va(b"%i\x00" as *const u8 as *mut i8, oldValue + data),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spAwards\x00" as *const u8 as *const i8,
        awardData.as_mut_ptr(),
    );
}
/*
===============
UI_GetAwardLevel
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetAwardLevel(mut award: i32) -> i32 {
    let mut key: [i8; 16] = [0; 16];
    let mut awardData: [i8; 1024] = [0; 1024];
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spAwards\x00" as *const u8 as *const i8,
        awardData.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 16]>() as i32,
        b"a%i\x00" as *const u8 as *const i8,
        award,
    );
    return atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        awardData.as_mut_ptr(),
        key.as_mut_ptr(),
    ));
}
/*
===============
UI_TierCompleted
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_TierCompleted(mut levelWon: i32) -> i32 {
    let mut level: i32 = 0;
    let mut n: i32 = 0;
    let mut tier: i32 = 0;
    let mut score: i32 = 0;
    let mut skill: i32 = 0;
    let mut info: *const i8 = 0 as *const i8;
    tier = levelWon / 4;
    level = tier * 4;
    if tier == UI_GetNumSPTiers() {
        info = UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const i8);
        if levelWon
            == atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                info,
                b"num\x00" as *const u8 as *const i8,
            ))
        {
            return 0i32;
        }
        info = UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const i8);
        if info.is_null()
            || levelWon
                == atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                    info,
                    b"num\x00" as *const u8 as *const i8,
                ))
        {
            return tier + 1i32;
        }
        return -(1i32);
    }
    n = 0;
    while n < 4 {
        UI_GetBestScore(level, &mut score, &mut skill);
        if score != 1 {
            return -(1i32);
        }
        n += 1;
        level += 1
    }
    return tier + 1;
}
/*
===============
UI_ShowTierVideo
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_ShowTierVideo(
    mut tier: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut key: [i8; 16] = [0; 16];
    let mut videos: [i8; 1024] = [0; 1024];
    if tier <= 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spVideos\x00" as *const u8 as *const i8,
        videos.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 16]>() as i32,
        b"tier%i\x00" as *const u8 as *const i8,
        tier,
    );
    if atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        videos.as_mut_ptr(),
        key.as_mut_ptr(),
    )) != 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        videos.as_mut_ptr(),
        key.as_mut_ptr(),
        crate::src::qcommon::q_shared::va(b"%i\x00" as *const u8 as *mut i8, 1i32),
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spVideos\x00" as *const u8 as *const i8,
        videos.as_mut_ptr(),
    );
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
UI_CanShowTierVideo
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_CanShowTierVideo(
    mut tier: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut key: [i8; 16] = [0; 16];
    let mut videos: [i8; 1024] = [0; 1024];
    if tier == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if crate::src::q3_ui::ui_atoms::uis.demoversion != 0 && tier != 8 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spVideos\x00" as *const u8 as *const i8,
        videos.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        key.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 16]>() as i32,
        b"tier%i\x00" as *const u8 as *const i8,
        tier,
    );
    if atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        videos.as_mut_ptr(),
        key.as_mut_ptr(),
    )) != 0
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
===============
UI_GetCurrentGame

Returns the next level the player has not won
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetCurrentGame() -> i32 {
    let mut level: i32 = 0;
    let mut rank: i32 = 0;
    let mut skill: i32 = 0;
    let mut info: *const i8 = 0 as *const i8;
    info = UI_GetSpecialArenaInfo(b"training\x00" as *const u8 as *const i8);
    if !info.is_null() {
        level = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
            info,
            b"num\x00" as *const u8 as *const i8,
        ));
        UI_GetBestScore(level, &mut rank, &mut skill);
        if rank == 0 || rank > 1 {
            return level;
        }
    }
    level = 0;
    while level < ui_numSinglePlayerArenas {
        UI_GetBestScore(level, &mut rank, &mut skill);
        if rank == 0 || rank > 1 {
            return level;
        }
        level += 1
    }
    info = UI_GetSpecialArenaInfo(b"final\x00" as *const u8 as *const i8);
    if info.is_null() {
        return -(1i32);
    }
    return atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        info,
        b"num\x00" as *const u8 as *const i8,
    ));
}
/*
===============
UI_NewGame

Clears the scores and sets the difficutly level
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_NewGame() {
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores1\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores2\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores3\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores4\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores5\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spAwards\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
    );
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spVideos\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
    );
}
/*
===============
UI_GetNumArenas
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetNumArenas() -> i32 {
    return ui_numArenas;
}
/*
===============
UI_GetNumSPArenas
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetNumSPArenas() -> i32 {
    return ui_numSinglePlayerArenas;
}
/*
===============
UI_GetNumSPTiers
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetNumSPTiers() -> i32 {
    return ui_numSinglePlayerArenas / 4;
}
/*
===============
UI_GetNumBots
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_GetNumBots() -> i32 {
    return ui_numBots;
}
/*
===============
UI_SPUnlock_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPUnlock_f() {
    let mut arenaKey: [i8; 16] = [0; 16];
    let mut scores: [i8; 1024] = [0; 1024];
    let mut level: i32 = 0;
    let mut tier: i32 = 0;
    // get scores for skill 1
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spScores1\x00" as *const u8 as *const i8,
        scores.as_mut_ptr(),
        1024,
    );

    for level in 0..ui_numSinglePlayerArenas + ui_numSpecialSinglePlayerArenas {
        crate::src::qcommon::q_shared::Com_sprintf(
            arenaKey.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 16]>() as i32,
            b"l%i\x00" as *const u8 as *const i8,
            level,
        );

        crate::src::qcommon::q_shared::Info_SetValueForKey(
            scores.as_mut_ptr(),
            arenaKey.as_mut_ptr(),
            b"1\x00" as *const u8 as *const i8,
        );
    }
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spScores1\x00" as *const u8 as *const i8,
        scores.as_mut_ptr(),
    );

    for tier in 1..=8 {
        UI_ShowTierVideo(tier);
    }
    crate::src::ui::ui_syscalls::trap_Print(
        b"All levels unlocked at skill level 1\n\x00" as *const u8 as *const i8,
    );
    crate::src::q3_ui::ui_splevel::UI_SPLevelMenu_ReInit();
}
/*
===============
UI_SPUnlockMedals_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_SPUnlockMedals_f() {
    let mut n: i32 = 0;
    let mut key: [i8; 16] = [0; 16];
    let mut awardData: [i8; 1024] = [0; 1024];
    crate::src::ui::ui_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_spAwards\x00" as *const u8 as *const i8,
        awardData.as_mut_ptr(),
        1024,
    );

    for n in 0..6 {
        crate::src::qcommon::q_shared::Com_sprintf(
            key.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 16]>() as i32,
            b"a%i\x00" as *const u8 as *const i8,
            n,
        );

        crate::src::qcommon::q_shared::Info_SetValueForKey(
            awardData.as_mut_ptr(),
            key.as_mut_ptr(),
            b"100\x00" as *const u8 as *const i8,
        );
    }
    crate::src::ui::ui_syscalls::trap_Cvar_Set(
        b"g_spAwards\x00" as *const u8 as *const i8,
        awardData.as_mut_ptr(),
    );
    crate::src::ui::ui_syscalls::trap_Print(
        b"All awards unlocked at 100\n\x00" as *const u8 as *const i8,
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
//
// ui_network.c
//
//
// ui_gameinfo.c
//
/*
===============
UI_InitGameinfo
===============
*/
#[no_mangle]

pub unsafe extern "C" fn UI_InitGameinfo() {
    UI_InitMemory();
    UI_LoadArenas();
    UI_LoadBots();
    crate::src::q3_ui::ui_atoms::uis.demoversion = crate::src::qcommon::q_shared::qfalse;
}
