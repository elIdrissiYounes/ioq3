use ::libc;

pub mod stdlib_float_h {
    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const i8) -> f64 {
        return crate::stdlib::strtod(__nptr, 0 as *mut *mut i8);
    }
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::bg_public_h::IT_AMMO;
pub use crate::bg_public_h::IT_ARMOR;
pub use crate::bg_public_h::IT_BAD;
pub use crate::bg_public_h::IT_HEALTH;
pub use crate::bg_public_h::IT_HOLDABLE;
pub use crate::bg_public_h::IT_PERSISTANT_POWERUP;
pub use crate::bg_public_h::IT_POWERUP;
pub use crate::bg_public_h::IT_TEAM;
pub use crate::bg_public_h::IT_WEAPON;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::g_public_h::entityShared_t;
pub use crate::src::game::g_main::Com_Printf;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trace_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::COM_Parse;
pub use crate::src::qcommon::q_shared::COM_ParseExt;
pub use crate::src::qcommon::q_shared::Com_Clamp;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::g_local_h::bot_settings_s;
pub use crate::g_local_h::bot_settings_t;
pub use crate::g_local_h::clientConnected_t;
pub use crate::g_local_h::clientPersistant_t;
pub use crate::g_local_h::clientSession_t;
pub use crate::g_local_h::gclient_s;
pub use crate::g_local_h::gclient_t;
pub use crate::g_local_h::gentity_s;
pub use crate::g_local_h::gentity_t;
pub use crate::g_local_h::level_locals_t;
pub use crate::g_local_h::moverState_t;
pub use crate::g_local_h::playerTeamStateState_t;
pub use crate::g_local_h::playerTeamState_t;
pub use crate::g_local_h::spectatorState_t;
pub use crate::g_local_h::CON_CONNECTED;
pub use crate::g_local_h::CON_CONNECTING;
pub use crate::g_local_h::CON_DISCONNECTED;
pub use crate::g_local_h::MOVER_1TO2;
pub use crate::g_local_h::MOVER_2TO1;
pub use crate::g_local_h::MOVER_POS1;
pub use crate::g_local_h::MOVER_POS2;
pub use crate::g_local_h::SPECTATOR_FOLLOW;
pub use crate::g_local_h::SPECTATOR_FREE;
pub use crate::g_local_h::SPECTATOR_NOT;
pub use crate::g_local_h::SPECTATOR_SCOREBOARD;
pub use crate::g_local_h::TEAM_ACTIVE;
pub use crate::g_local_h::TEAM_BEGIN;
pub use crate::src::game::ai_main::BotAISetupClient;
pub use crate::src::game::g_bot::stdlib_float_h::atof;
pub use crate::src::game::g_bot::stdlib_h::atoi;
pub use crate::src::game::g_client::ClientBegin;
pub use crate::src::game::g_client::ClientConnect;
pub use crate::src::game::g_client::PickTeam;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::g_gametype;
pub use crate::src::game::g_main::g_maxclients;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_mem::G_Alloc;
pub use crate::src::game::g_syscalls::trap_Argv;
pub use crate::src::game::g_syscalls::trap_BotAllocateClient;
pub use crate::src::game::g_syscalls::trap_BotFreeClient;
pub use crate::src::game::g_syscalls::trap_Cvar_Register;
pub use crate::src::game::g_syscalls::trap_Cvar_Set;
pub use crate::src::game::g_syscalls::trap_Cvar_Update;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::game::g_syscalls::trap_DropClient;
pub use crate::src::game::g_syscalls::trap_FS_FCloseFile;
pub use crate::src::game::g_syscalls::trap_FS_FOpenFile;
pub use crate::src::game::g_syscalls::trap_FS_GetFileList;
pub use crate::src::game::g_syscalls::trap_FS_Read;
pub use crate::src::game::g_syscalls::trap_GetServerinfo;
pub use crate::src::game::g_syscalls::trap_GetUserinfo;
pub use crate::src::game::g_syscalls::trap_Print;
pub use crate::src::game::g_syscalls::trap_SendConsoleCommand;
pub use crate::src::game::g_syscalls::trap_SendServerCommand;
pub use crate::src::game::g_syscalls::trap_SetUserinfo;
pub use crate::stdlib::rand;

pub use crate::stdlib::strtod;
pub use crate::stdlib::strtol;
extern "C" {
    #[no_mangle]
    pub static mut podium1: *mut crate::g_local_h::gentity_t;
    #[no_mangle]
    pub static mut podium2: *mut crate::g_local_h::gentity_t;
    #[no_mangle]
    pub static mut podium3: *mut crate::g_local_h::gentity_t;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct botSpawnQueue_t {
    pub clientNum: i32,
    pub spawnTime: i32,
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
// g_bot.c

static mut g_numBots: i32 = 0;

static mut g_botInfos: [*mut i8; 1024] = [0 as *mut i8; 1024];
#[no_mangle]

pub static mut g_numArenas: i32 = 0;

static mut g_arenaInfos: [*mut i8; 1024] = [0 as *mut i8; 1024];

static mut botSpawnQueue: [botSpawnQueue_t; 16] = [botSpawnQueue_t {
    clientNum: 0,
    spawnTime: 0,
}; 16];
#[no_mangle]

pub static mut bot_minplayers: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub unsafe extern "C" fn trap_Cvar_VariableValue(mut var_name: *const i8) -> f32 {
    let mut buf: [i8; 128] = [0; 128];
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        var_name,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 128]>() as i32,
    );
    return atof(buf.as_mut_ptr()) as f32;
}
/*
===============
G_ParseInfos
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_ParseInfos(
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
            crate::src::game::g_main::Com_Printf(
                b"Missing { in info file\n\x00" as *const u8 as *const i8,
            );
            break;
        } else if count == max {
            crate::src::game::g_main::Com_Printf(
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
                    crate::src::game::g_main::Com_Printf(
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
            *fresh0 = crate::src::game::g_mem::G_Alloc(
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
G_LoadArenasFromFile
===============
*/

unsafe extern "C" fn G_LoadArenasFromFile(mut filename: *mut i8) {
    let mut len: i32 = 0;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut buf: [i8; 8192] = [0; 8192];
    len = crate::src::game::g_syscalls::trap_FS_FOpenFile(
        filename,
        &mut f,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if f == 0 {
        crate::src::game::g_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file not found: %s\n\x00" as *const u8 as *mut i8,
            filename,
        ));
        return;
    }
    if len >= 8192 {
        crate::src::game::g_syscalls::trap_FS_FCloseFile(f);
        crate::src::game::g_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8 as *mut i8,
            filename,
            len,
            8192i32,
        ));
        return;
    }
    crate::src::game::g_syscalls::trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0;
    crate::src::game::g_syscalls::trap_FS_FCloseFile(f);
    g_numArenas += G_ParseInfos(
        buf.as_mut_ptr(),
        1024 - g_numArenas,
        &mut *g_arenaInfos.as_mut_ptr().offset(g_numArenas as isize),
    );
}
/*
===============
G_LoadArenas
===============
*/

unsafe extern "C" fn G_LoadArenas() {
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
    let mut dirlist: [i8; 1024] = [0; 1024];
    let mut dirptr: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut n: i32 = 0;
    let mut dirlen: i32 = 0;
    g_numArenas = 0;
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut arenasFile,
        b"g_arenasFile\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        0x10 | 0x40,
    );
    if *arenasFile.string.as_mut_ptr() != 0 {
        G_LoadArenasFromFile(arenasFile.string.as_mut_ptr());
    } else {
        G_LoadArenasFromFile(b"scripts/arenas.txt\x00" as *const u8 as *mut i8);
    }
    // get all arenas from .arena files
    numdirs = crate::src::game::g_syscalls::trap_FS_GetFileList(
        b"scripts\x00" as *const u8 as *const i8,
        b".arena\x00" as *const u8 as *const i8,
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
        G_LoadArenasFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1) as isize)
    }
    crate::src::game::g_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
        b"%i arenas parsed\n\x00" as *const u8 as *mut i8,
        g_numArenas,
    ));
    n = 0;
    while n < g_numArenas {
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            g_arenaInfos[n as usize],
            b"num\x00" as *const u8 as *const i8,
            crate::src::qcommon::q_shared::va(b"%i\x00" as *const u8 as *mut i8, n),
        );
        n += 1
    }
}
/*
===============
G_GetArenaInfoByNumber
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_GetArenaInfoByMap(mut map: *const i8) -> *const i8 {
    let mut _n: i32 = 0;

    for n in 0..g_numArenas {
        if crate::src::qcommon::q_shared::Q_stricmp(
            crate::src::qcommon::q_shared::Info_ValueForKey(
                g_arenaInfos[n as usize],
                b"map\x00" as *const u8 as *const i8,
            ),
            map,
        ) == 0
        {
            return g_arenaInfos[n as usize];
        }
    }
    return 0 as *const i8;
}
/*
=================
PlayerIntroSound
=================
*/

unsafe extern "C" fn PlayerIntroSound(mut modelAndSkin: *const i8) {
    let mut model: [i8; 64] = [0; 64];
    let mut skin: *mut i8 = 0 as *mut i8;
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
        skin = model.as_mut_ptr()
    }
    if crate::src::qcommon::q_shared::Q_stricmp(skin, b"default\x00" as *const u8 as *const i8) == 0
    {
        skin = model.as_mut_ptr()
    }
    crate::src::game::g_syscalls::trap_SendConsoleCommand(
        crate::src::qcommon::q_shared::EXEC_APPEND as i32,
        crate::src::qcommon::q_shared::va(
            b"play sound/player/announce/%s.wav\n\x00" as *const u8 as *mut i8,
            skin,
        ),
    );
}
/*
===============
G_CountBotPlayersByName

Check connected and connecting (delay join) bots.

Returns number of bots with name on specified team or whole server if team is -1.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CountBotPlayersByName(mut name: *const i8, mut team: i32) -> i32 {
    let mut _i: i32 = 0;
    let mut num: i32 = 0;
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    num = 0;

    for i in 0..crate::src::game::g_main::g_maxclients.integer {
        cl = crate::src::game::g_main::level.clients.offset(i as isize);

        if !((*cl).pers.connected == crate::g_local_h::CON_DISCONNECTED) {
            if !(crate::src::game::g_main::g_entities[i as usize].r.svFlags & 0x8 == 0) {
                if !(team >= 0 && (*cl).sess.sessionTeam != team as u32) {
                    if !(!name.is_null()
                        && crate::src::qcommon::q_shared::Q_stricmp(
                            name,
                            (*cl).pers.netname.as_mut_ptr(),
                        ) != 0)
                    {
                        num += 1
                    }
                }
            }
        }
    }
    return num;
}
/*
===============
G_SelectRandomBotInfo

Get random least used bot info on team or whole server if team is -1.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_SelectRandomBotInfo(mut team: i32) -> i32 {
    let mut selection: [i32; 1024] = [0; 1024];
    let mut _n: i32 = 0;
    let mut num: i32 = 0;
    let mut count: i32 = 0;
    let mut bestCount: i32 = 0;
    let mut value: *mut i8 = 0 as *mut i8;
    // don't add duplicate bots to the server if there are less bots than bot types
    if team != -(1) && G_CountBotPlayersByName(0 as *const i8, -(1)) < g_numBots {
        team = -(1)
    }
    num = 0;
    bestCount = 64;

    for n in 0..g_numBots {
        value = crate::src::qcommon::q_shared::Info_ValueForKey(
            g_botInfos[n as usize],
            b"funname\x00" as *const u8 as *const i8,
        );

        if *value.offset(0) == 0 {
            value = crate::src::qcommon::q_shared::Info_ValueForKey(
                g_botInfos[n as usize],
                b"name\x00" as *const u8 as *const i8,
            )
        }

        count = G_CountBotPlayersByName(value, team);

        if count < bestCount {
            bestCount = count;
            num = 0
        }

        if count == bestCount {
            let fresh2 = num;
            num = num + 1;
            selection[fresh2 as usize] = n;
            if num == 1024 {
                break;
            }
        }
    }
    if num > 0 {
        num = ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * (num - 1) as f32) as i32;
        return selection[num as usize];
    }
    return -(1);
}
/*
===============
G_AddRandomBot
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_AddRandomBot(mut team: i32) {
    let mut teamstr: *mut i8 = 0 as *mut i8;
    let mut skill: f32 = 0.;
    skill = trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const i8);
    if team == crate::bg_public_h::TEAM_RED as i32 {
        teamstr = b"red\x00" as *const u8 as *mut i8
    } else if team == crate::bg_public_h::TEAM_BLUE as i32 {
        teamstr = b"blue\x00" as *const u8 as *mut i8
    } else {
        teamstr = b"free\x00" as *const u8 as *mut i8
    }
    crate::src::game::g_syscalls::trap_SendConsoleCommand(
        crate::src::qcommon::q_shared::EXEC_INSERT as i32,
        crate::src::qcommon::q_shared::va(
            b"addbot random %f %s %i\n\x00" as *const u8 as *mut i8,
            skill as f64,
            teamstr,
            0i32,
        ),
    );
}
/*
===============
G_RemoveRandomBot
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_RemoveRandomBot(mut team: i32) -> i32 {
    let mut _i: i32 = 0;
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;

    for i in 0..crate::src::game::g_main::g_maxclients.integer {
        cl = crate::src::game::g_main::level.clients.offset(i as isize);

        if !((*cl).pers.connected != crate::g_local_h::CON_CONNECTED) {
            if !(crate::src::game::g_main::g_entities[i as usize].r.svFlags & 0x8 == 0) {
                if !(team >= 0 && (*cl).sess.sessionTeam != team as u32) {
                    crate::src::game::g_syscalls::trap_SendConsoleCommand(
                        crate::src::qcommon::q_shared::EXEC_INSERT as i32,
                        crate::src::qcommon::q_shared::va(
                            b"clientkick %d\n\x00" as *const u8 as *mut i8,
                            i,
                        ),
                    );
                    return crate::src::qcommon::q_shared::qtrue as i32;
                }
            }
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
===============
G_CountHumanPlayers
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CountHumanPlayers(mut team: i32) -> i32 {
    let mut _i: i32 = 0;
    let mut num: i32 = 0;
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    num = 0;

    for i in 0..crate::src::game::g_main::g_maxclients.integer {
        cl = crate::src::game::g_main::level.clients.offset(i as isize);

        if !((*cl).pers.connected != crate::g_local_h::CON_CONNECTED) {
            if !(crate::src::game::g_main::g_entities[i as usize].r.svFlags & 0x8 != 0) {
                if !(team >= 0 && (*cl).sess.sessionTeam != team as u32) {
                    num += 1
                }
            }
        }
    }
    return num;
}
/*
===============
G_CountBotPlayers

Check connected and connecting (delay join) bots.
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CountBotPlayers(mut team: i32) -> i32 {
    let mut _i: i32 = 0;
    let mut num: i32 = 0;
    let mut cl: *mut crate::g_local_h::gclient_t = 0 as *mut crate::g_local_h::gclient_t;
    num = 0;

    for i in 0..crate::src::game::g_main::g_maxclients.integer {
        cl = crate::src::game::g_main::level.clients.offset(i as isize);

        if !((*cl).pers.connected == crate::g_local_h::CON_DISCONNECTED) {
            if !(crate::src::game::g_main::g_entities[i as usize].r.svFlags & 0x8 == 0) {
                if !(team >= 0 && (*cl).sess.sessionTeam != team as u32) {
                    num += 1
                }
            }
        }
    }
    return num;
}
/*
===============
G_CheckMinimumPlayers
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CheckMinimumPlayers() {
    let mut minplayers: i32 = 0;
    let mut humanplayers: i32 = 0;
    let mut botplayers: i32 = 0;
    static mut checkminimumplayers_time: i32 = 0;
    if crate::src::game::g_main::level.intermissiontime != 0 {
        return;
    }
    //only check once each 10 seconds
    if checkminimumplayers_time > crate::src::game::g_main::level.time - 10000 {
        return;
    }
    checkminimumplayers_time = crate::src::game::g_main::level.time;
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_minplayers);
    minplayers = bot_minplayers.integer;
    if minplayers <= 0 {
        return;
    }
    if crate::src::game::g_main::g_gametype.integer >= crate::bg_public_h::GT_TEAM as i32 {
        if minplayers >= crate::src::game::g_main::g_maxclients.integer / 2 {
            minplayers = crate::src::game::g_main::g_maxclients.integer / 2 - 1
        }
        humanplayers = G_CountHumanPlayers(crate::bg_public_h::TEAM_RED as i32);
        botplayers = G_CountBotPlayers(crate::bg_public_h::TEAM_RED as i32);
        //
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(crate::bg_public_h::TEAM_RED as i32);
        } else if humanplayers + botplayers > minplayers && botplayers != 0 {
            G_RemoveRandomBot(crate::bg_public_h::TEAM_RED as i32);
        }
        //
        humanplayers = G_CountHumanPlayers(crate::bg_public_h::TEAM_BLUE as i32);
        botplayers = G_CountBotPlayers(crate::bg_public_h::TEAM_BLUE as i32);
        //
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(crate::bg_public_h::TEAM_BLUE as i32);
        } else if humanplayers + botplayers > minplayers && botplayers != 0 {
            G_RemoveRandomBot(crate::bg_public_h::TEAM_BLUE as i32);
        }
    } else if crate::src::game::g_main::g_gametype.integer
        == crate::bg_public_h::GT_TOURNAMENT as i32
    {
        if minplayers >= crate::src::game::g_main::g_maxclients.integer {
            minplayers = crate::src::game::g_main::g_maxclients.integer - 1
        }
        humanplayers = G_CountHumanPlayers(-(1));
        botplayers = G_CountBotPlayers(-(1));
        //
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(crate::bg_public_h::TEAM_FREE as i32);
        } else if humanplayers + botplayers > minplayers && botplayers != 0 {
            // try to remove spectators first
            if G_RemoveRandomBot(crate::bg_public_h::TEAM_SPECTATOR as i32) == 0 {
                // just remove the bot that is playing
                G_RemoveRandomBot(-(1i32));
            }
        }
    } else if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_FFA as i32 {
        if minplayers >= crate::src::game::g_main::g_maxclients.integer {
            minplayers = crate::src::game::g_main::g_maxclients.integer - 1
        }
        humanplayers = G_CountHumanPlayers(crate::bg_public_h::TEAM_FREE as i32);
        botplayers = G_CountBotPlayers(crate::bg_public_h::TEAM_FREE as i32);
        //
        if humanplayers + botplayers < minplayers {
            G_AddRandomBot(crate::bg_public_h::TEAM_FREE as i32);
        } else if humanplayers + botplayers > minplayers && botplayers != 0 {
            G_RemoveRandomBot(crate::bg_public_h::TEAM_FREE as i32);
        }
    };
}
/*
===============
G_CheckBotSpawn
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_CheckBotSpawn() {
    let mut n: i32 = 0;
    let mut userinfo: [i8; 1024] = [0; 1024];
    G_CheckMinimumPlayers();
    n = 0;
    while n < 16 {
        if !(botSpawnQueue[n as usize].spawnTime == 0) {
            if !(botSpawnQueue[n as usize].spawnTime > crate::src::game::g_main::level.time) {
                crate::src::game::g_client::ClientBegin(botSpawnQueue[n as usize].clientNum);
                botSpawnQueue[n as usize].spawnTime = 0;
                if crate::src::game::g_main::g_gametype.integer
                    == crate::bg_public_h::GT_SINGLE_PLAYER as i32
                {
                    crate::src::game::g_syscalls::trap_GetUserinfo(
                        botSpawnQueue[n as usize].clientNum,
                        userinfo.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 1024]>() as i32,
                    );
                    PlayerIntroSound(crate::src::qcommon::q_shared::Info_ValueForKey(
                        userinfo.as_mut_ptr(),
                        b"model\x00" as *const u8 as *const i8,
                    ));
                }
            }
        }
        n += 1
    }
}
/*
===============
AddBotToSpawnQueue
===============
*/

unsafe extern "C" fn AddBotToSpawnQueue(mut clientNum: i32, mut delay: i32) {
    let mut _n: i32 = 0;

    for n in 0..16 {
        if botSpawnQueue[n as usize].spawnTime == 0 {
            botSpawnQueue[n as usize].spawnTime = crate::src::game::g_main::level.time + delay;
            botSpawnQueue[n as usize].clientNum = clientNum;
            return;
        }
    }
    crate::src::game::g_main::G_Printf(b"^3Unable to delay spawn\n\x00" as *const u8 as *const i8);
    crate::src::game::g_client::ClientBegin(clientNum);
}
/*
===============
G_RemoveQueuedBotBegin

Called on client disconnect to make sure the delayed spawn
doesn't happen on a freed index
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_RemoveQueuedBotBegin(mut clientNum: i32) {
    let mut n: i32 = 0;
    n = 0;
    while n < 16 {
        if botSpawnQueue[n as usize].clientNum == clientNum {
            botSpawnQueue[n as usize].spawnTime = 0;
            return;
        }
        n += 1
    }
}
/*
===============
G_BotConnect
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_BotConnect(
    mut clientNum: i32,
    mut restart: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut settings: crate::g_local_h::bot_settings_t = crate::g_local_h::bot_settings_t {
        characterfile: [0; 144],
        skill: 0.,
    };
    let mut userinfo: [i8; 1024] = [0; 1024];
    crate::src::game::g_syscalls::trap_GetUserinfo(
        clientNum,
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        settings.characterfile.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            userinfo.as_mut_ptr(),
            b"characterfile\x00" as *const u8 as *const i8,
        ),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    settings.skill = atof(crate::src::qcommon::q_shared::Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"skill\x00" as *const u8 as *const i8,
    )) as f32;
    if crate::src::game::ai_main::BotAISetupClient(clientNum, &mut settings, restart) == 0 {
        crate::src::game::g_syscalls::trap_DropClient(
            clientNum,
            b"BotAISetupClient failed\x00" as *const u8 as *const i8,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
/*
===============
G_AddBot
===============
*/

unsafe extern "C" fn G_AddBot(
    mut name: *const i8,
    mut skill: f32,
    mut team: *const i8,
    mut delay: i32,
    mut altname: *mut i8,
) {
    let mut clientNum: i32 = 0;
    let mut teamNum: i32 = 0;
    let mut botinfoNum: i32 = 0;
    let mut botinfo: *mut i8 = 0 as *mut i8;
    let mut key: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut botname: *mut i8 = 0 as *mut i8;
    let mut model: *mut i8 = 0 as *mut i8;
    let mut headmodel: *mut i8 = 0 as *mut i8;
    let mut userinfo: [i8; 1024] = [0; 1024];
    // have the server allocate a client slot
    clientNum = crate::src::game::g_syscalls::trap_BotAllocateClient();
    if clientNum == -(1) {
        crate::src::game::g_main::G_Printf(
            b"^1Unable to add bot. All player slots are in use.\n\x00" as *const u8 as *const i8,
        );
        crate::src::game::g_main::G_Printf(b"^1Start server with more \'open\' slots (or check setting of sv_maxclients cvar).\n\x00"
                     as *const u8 as *const i8);
        return;
    }
    // set default team
    if team.is_null() || *team == 0 {
        if crate::src::game::g_main::g_gametype.integer >= crate::bg_public_h::GT_TEAM as i32 {
            if crate::src::game::g_client::PickTeam(clientNum) == crate::bg_public_h::TEAM_RED {
                team = b"red\x00" as *const u8 as *const i8
            } else {
                team = b"blue\x00" as *const u8 as *const i8
            }
        } else {
            team = b"free\x00" as *const u8 as *const i8
        }
    }
    // get the botinfo from bots.txt
    if crate::src::qcommon::q_shared::Q_stricmp(name, b"random\x00" as *const u8 as *const i8) == 0
    {
        if crate::src::qcommon::q_shared::Q_stricmp(team, b"red\x00" as *const u8 as *const i8) == 0
            || crate::src::qcommon::q_shared::Q_stricmp(team, b"r\x00" as *const u8 as *const i8)
                == 0
        {
            teamNum = crate::bg_public_h::TEAM_RED as i32
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            team,
            b"blue\x00" as *const u8 as *const i8,
        ) == 0
            || crate::src::qcommon::q_shared::Q_stricmp(team, b"b\x00" as *const u8 as *const i8)
                == 0
        {
            teamNum = crate::bg_public_h::TEAM_BLUE as i32
        } else if crate::src::qcommon::q_shared::Q_stricmp(
            team,
            b"spectator\x00" as *const u8 as *const i8,
        ) == 0
            || crate::src::qcommon::q_shared::Q_stricmp(team, b"s\x00" as *const u8 as *const i8)
                == 0
        {
            teamNum = crate::bg_public_h::TEAM_SPECTATOR as i32
        } else {
            teamNum = crate::bg_public_h::TEAM_FREE as i32
        }
        botinfoNum = G_SelectRandomBotInfo(teamNum);
        if botinfoNum < 0 {
            crate::src::game::g_main::G_Printf(
                b"^1Error: Cannot add random bot, no bot info available.\n\x00" as *const u8
                    as *const i8,
            );
            crate::src::game::g_syscalls::trap_BotFreeClient(clientNum);
            return;
        }
        botinfo = G_GetBotInfoByNumber(botinfoNum)
    } else {
        botinfo = G_GetBotInfoByName(name)
    }
    if botinfo.is_null() {
        crate::src::game::g_main::G_Printf(
            b"^1Error: Bot \'%s\' not defined\n\x00" as *const u8 as *const i8,
            name,
        );
        crate::src::game::g_syscalls::trap_BotFreeClient(clientNum);
        return;
    }
    // create the bot's userinfo
    userinfo[0] = '\u{0}' as i8;
    botname = crate::src::qcommon::q_shared::Info_ValueForKey(
        botinfo,
        b"funname\x00" as *const u8 as *const i8,
    );
    if *botname.offset(0) == 0 {
        botname = crate::src::qcommon::q_shared::Info_ValueForKey(
            botinfo,
            b"name\x00" as *const u8 as *const i8,
        )
    }
    // check for an alternative name
    if !altname.is_null() && *altname.offset(0) as i32 != 0 {
        botname = altname
    }
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"name\x00" as *const u8 as *const i8,
        botname,
    );
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"rate\x00" as *const u8 as *const i8,
        b"25000\x00" as *const u8 as *const i8,
    );
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"snaps\x00" as *const u8 as *const i8,
        b"20\x00" as *const u8 as *const i8,
    );
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"skill\x00" as *const u8 as *const i8,
        crate::src::qcommon::q_shared::va(b"%.2f\x00" as *const u8 as *mut i8, skill as f64),
    );
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"teampref\x00" as *const u8 as *const i8,
        team,
    );
    if skill >= 1f32 && skill < 2f32 {
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"handicap\x00" as *const u8 as *const i8,
            b"50\x00" as *const u8 as *const i8,
        );
    } else if skill >= 2f32 && skill < 3f32 {
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"handicap\x00" as *const u8 as *const i8,
            b"70\x00" as *const u8 as *const i8,
        );
    } else if skill >= 3f32 && skill < 4f32 {
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"handicap\x00" as *const u8 as *const i8,
            b"90\x00" as *const u8 as *const i8,
        );
    }
    key = b"model\x00" as *const u8 as *mut i8;
    model = crate::src::qcommon::q_shared::Info_ValueForKey(botinfo, key);
    if *model == 0 {
        model = b"visor/default\x00" as *const u8 as *mut i8
    }
    crate::src::qcommon::q_shared::Info_SetValueForKey(userinfo.as_mut_ptr(), key, model);
    key = b"team_model\x00" as *const u8 as *mut i8;
    crate::src::qcommon::q_shared::Info_SetValueForKey(userinfo.as_mut_ptr(), key, model);
    key = b"headmodel\x00" as *const u8 as *mut i8;
    headmodel = crate::src::qcommon::q_shared::Info_ValueForKey(botinfo, key);
    if *headmodel == 0 {
        headmodel = model
    }
    crate::src::qcommon::q_shared::Info_SetValueForKey(userinfo.as_mut_ptr(), key, headmodel);
    key = b"team_headmodel\x00" as *const u8 as *mut i8;
    crate::src::qcommon::q_shared::Info_SetValueForKey(userinfo.as_mut_ptr(), key, headmodel);
    key = b"gender\x00" as *const u8 as *mut i8;
    s = crate::src::qcommon::q_shared::Info_ValueForKey(botinfo, key);
    if *s == 0 {
        s = b"male\x00" as *const u8 as *mut i8
    }
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"sex\x00" as *const u8 as *const i8,
        s,
    );
    key = b"color1\x00" as *const u8 as *mut i8;
    s = crate::src::qcommon::q_shared::Info_ValueForKey(botinfo, key);
    if *s == 0 {
        s = b"4\x00" as *const u8 as *mut i8
    }
    crate::src::qcommon::q_shared::Info_SetValueForKey(userinfo.as_mut_ptr(), key, s);
    key = b"color2\x00" as *const u8 as *mut i8;
    s = crate::src::qcommon::q_shared::Info_ValueForKey(botinfo, key);
    if *s == 0 {
        s = b"5\x00" as *const u8 as *mut i8
    }
    crate::src::qcommon::q_shared::Info_SetValueForKey(userinfo.as_mut_ptr(), key, s);
    s = crate::src::qcommon::q_shared::Info_ValueForKey(
        botinfo,
        b"aifile\x00" as *const u8 as *const i8,
    );
    if *s == 0 {
        crate::src::game::g_syscalls::trap_Print(
            b"^1Error: bot has no aifile specified\n\x00" as *const u8 as *const i8,
        );
        crate::src::game::g_syscalls::trap_BotFreeClient(clientNum);
        return;
    }
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"characterfile\x00" as *const u8 as *const i8,
        s,
    );
    // don't send tinfo to bots, they don't parse it
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"teamoverlay\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
    );
    // register the userinfo
    crate::src::game::g_syscalls::trap_SetUserinfo(clientNum, userinfo.as_mut_ptr());
    // have it connect to the game as a normal client
    if !crate::src::game::g_client::ClientConnect(
        clientNum,
        crate::src::qcommon::q_shared::qtrue,
        crate::src::qcommon::q_shared::qtrue,
    )
    .is_null()
    {
        return;
    }
    if delay == 0 {
        crate::src::game::g_client::ClientBegin(clientNum);
        return;
    }
    AddBotToSpawnQueue(clientNum, delay);
}
/*
===============
Svcmd_AddBot_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Svcmd_AddBot_f() {
    let mut skill: f32 = 0.;
    let mut delay: i32 = 0;
    let mut name: [i8; 1024] = [0; 1024];
    let mut altname: [i8; 1024] = [0; 1024];
    let mut string: [i8; 1024] = [0; 1024];
    let mut team: [i8; 1024] = [0; 1024];
    // are bots enabled?
    if crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
        b"bot_enable\x00" as *const u8 as *const i8,
    ) == 0
    {
        return;
    }
    // name
    crate::src::game::g_syscalls::trap_Argv(
        1,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if name[0] == 0 {
        crate::src::game::g_syscalls::trap_Print(
            b"Usage: Addbot <botname> [skill 1-5] [team] [msec delay] [altname]\n\x00" as *const u8
                as *const i8,
        );
        return;
    }
    // skill
    crate::src::game::g_syscalls::trap_Argv(
        2,
        string.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if string[0] == 0 {
        skill = 4f32
    } else {
        skill =
            crate::src::qcommon::q_shared::Com_Clamp(1f32, 5f32, atof(string.as_mut_ptr()) as f32)
    }
    // team
    crate::src::game::g_syscalls::trap_Argv(
        3,
        team.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    // delay
    crate::src::game::g_syscalls::trap_Argv(
        4,
        string.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if string[0] == 0 {
        delay = 0
    } else {
        delay = atoi(string.as_mut_ptr())
    }
    // alternative name
    crate::src::game::g_syscalls::trap_Argv(
        5,
        altname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    G_AddBot(
        name.as_mut_ptr(),
        skill,
        team.as_mut_ptr(),
        delay,
        altname.as_mut_ptr(),
    );
    // if this was issued during gameplay and we are playing locally,
    // go ahead and load the bot's media immediately
    if crate::src::game::g_main::level.time - crate::src::game::g_main::level.startTime > 1000
        && crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
            b"cl_running\x00" as *const u8 as *const i8,
        ) != 0
    {
        crate::src::game::g_syscalls::trap_SendServerCommand(
            -(1i32),
            b"loaddefered\n\x00" as *const u8 as *const i8,
        );
        // FIXME: spelled wrong, but not changing for demo
    };
}
/*
===============
Svcmd_BotList_f
===============
*/
#[no_mangle]

pub unsafe extern "C" fn Svcmd_BotList_f() {
    let mut i: i32 = 0;
    let mut name: [i8; 1024] = [0; 1024];
    let mut funname: [i8; 1024] = [0; 1024];
    let mut model: [i8; 1024] = [0; 1024];
    let mut aifile: [i8; 1024] = [0; 1024];
    crate::src::game::g_syscalls::trap_Print(
        b"^1name             model            aifile              funname\n\x00" as *const u8
            as *const i8,
    );
    i = 0;
    while i < g_numBots {
        crate::src::qcommon::q_shared::Q_strncpyz(
            name.as_mut_ptr(),
            crate::src::qcommon::q_shared::Info_ValueForKey(
                g_botInfos[i as usize],
                b"name\x00" as *const u8 as *const i8,
            ),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        if *name.as_mut_ptr() == 0 {
            crate::stdlib::strcpy(
                name.as_mut_ptr(),
                b"UnnamedPlayer\x00" as *const u8 as *const i8,
            );
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            funname.as_mut_ptr(),
            crate::src::qcommon::q_shared::Info_ValueForKey(
                g_botInfos[i as usize],
                b"funname\x00" as *const u8 as *const i8,
            ),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        if *funname.as_mut_ptr() == 0 {
            crate::stdlib::strcpy(funname.as_mut_ptr(), b"\x00" as *const u8 as *const i8);
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            model.as_mut_ptr(),
            crate::src::qcommon::q_shared::Info_ValueForKey(
                g_botInfos[i as usize],
                b"model\x00" as *const u8 as *const i8,
            ),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        if *model.as_mut_ptr() == 0 {
            crate::stdlib::strcpy(
                model.as_mut_ptr(),
                b"visor/default\x00" as *const u8 as *const i8,
            );
        }
        crate::src::qcommon::q_shared::Q_strncpyz(
            aifile.as_mut_ptr(),
            crate::src::qcommon::q_shared::Info_ValueForKey(
                g_botInfos[i as usize],
                b"aifile\x00" as *const u8 as *const i8,
            ),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        if *aifile.as_mut_ptr() == 0 {
            crate::stdlib::strcpy(
                aifile.as_mut_ptr(),
                b"bots/default_c.c\x00" as *const u8 as *const i8,
            );
        }
        crate::src::game::g_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"%-16s %-16s %-20s %-20s\n\x00" as *const u8 as *mut i8,
            name.as_mut_ptr(),
            model.as_mut_ptr(),
            aifile.as_mut_ptr(),
            funname.as_mut_ptr(),
        ));
        i += 1
    }
}
/*
===============
G_SpawnBots
===============
*/

unsafe extern "C" fn G_SpawnBots(mut botList: *mut i8, mut baseDelay: i32) {
    let mut bot: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut skill: f32 = 0.;
    let mut delay: i32 = 0;
    let mut bots: [i8; 1024] = [0; 1024];
    podium1 = 0 as *mut crate::g_local_h::gentity_t;
    podium2 = 0 as *mut crate::g_local_h::gentity_t;
    podium3 = 0 as *mut crate::g_local_h::gentity_t;
    skill = trap_Cvar_VariableValue(b"g_spSkill\x00" as *const u8 as *const i8);
    if skill < 1f32 {
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"g_spSkill\x00" as *const u8 as *const i8,
            b"1\x00" as *const u8 as *const i8,
        );
        skill = 1f32
    } else if skill > 5f32 {
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"g_spSkill\x00" as *const u8 as *const i8,
            b"5\x00" as *const u8 as *const i8,
        );
        skill = 5f32
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        bots.as_mut_ptr(),
        botList,
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    p = &mut *bots.as_mut_ptr().offset(0) as *mut i8;
    delay = baseDelay;
    while *p != 0 {
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
            let fresh3 = p;
            p = p.offset(1);
            *fresh3 = 0i8
        }
        // we must add the bot this way, calling G_AddBot directly at this stage
        // does "Bad Things"
        crate::src::game::g_syscalls::trap_SendConsoleCommand(
            crate::src::qcommon::q_shared::EXEC_INSERT as i32,
            crate::src::qcommon::q_shared::va(
                b"addbot %s %f free %i\n\x00" as *const u8 as *mut i8,
                bot,
                skill as f64,
                delay,
            ),
        );
        delay += 1500
    }
}
/*
===============
G_LoadBotsFromFile
===============
*/

unsafe extern "C" fn G_LoadBotsFromFile(mut filename: *mut i8) {
    let mut len: i32 = 0;
    let mut f: crate::src::qcommon::q_shared::fileHandle_t = 0;
    let mut buf: [i8; 8192] = [0; 8192];
    len = crate::src::game::g_syscalls::trap_FS_FOpenFile(
        filename,
        &mut f,
        crate::src::qcommon::q_shared::FS_READ,
    );
    if f == 0 {
        crate::src::game::g_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file not found: %s\n\x00" as *const u8 as *mut i8,
            filename,
        ));
        return;
    }
    if len >= 8192 {
        crate::src::game::g_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1file too large: %s is %i, max allowed is %i\n\x00" as *const u8 as *mut i8,
            filename,
            len,
            8192i32,
        ));
        crate::src::game::g_syscalls::trap_FS_FCloseFile(f);
        return;
    }
    crate::src::game::g_syscalls::trap_FS_Read(buf.as_mut_ptr() as *mut libc::c_void, len, f);
    buf[len as usize] = 0;
    crate::src::game::g_syscalls::trap_FS_FCloseFile(f);
    g_numBots += G_ParseInfos(
        buf.as_mut_ptr(),
        1024 - g_numBots,
        &mut *g_botInfos.as_mut_ptr().offset(g_numBots as isize),
    );
}
/*
===============
G_LoadBots
===============
*/

unsafe extern "C" fn G_LoadBots() {
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
    if crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
        b"bot_enable\x00" as *const u8 as *const i8,
    ) == 0
    {
        return;
    }
    g_numBots = 0;
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut botsFile,
        b"g_botsFile\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        0x10 | 0x40,
    );
    if *botsFile.string.as_mut_ptr() != 0 {
        G_LoadBotsFromFile(botsFile.string.as_mut_ptr());
    } else {
        G_LoadBotsFromFile(b"scripts/bots.txt\x00" as *const u8 as *mut i8);
    }
    // get all bots from .bot files
    numdirs = crate::src::game::g_syscalls::trap_FS_GetFileList(
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
        G_LoadBotsFromFile(filename.as_mut_ptr());
        i += 1;
        dirptr = dirptr.offset((dirlen + 1) as isize)
    }
    crate::src::game::g_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
        b"%i bots parsed\n\x00" as *const u8 as *mut i8,
        g_numBots,
    ));
}
/*
===============
G_GetBotInfoByNumber
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_GetBotInfoByNumber(mut num: i32) -> *mut i8 {
    if num < 0 || num >= g_numBots {
        crate::src::game::g_syscalls::trap_Print(crate::src::qcommon::q_shared::va(
            b"^1Invalid bot number: %i\n\x00" as *const u8 as *mut i8,
            num,
        ));
        return 0 as *mut i8;
    }
    return g_botInfos[num as usize];
}
/*
===============
G_GetBotInfoByName
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_GetBotInfoByName(mut name: *const i8) -> *mut i8 {
    let mut _n: i32 = 0;
    let mut value: *mut i8 = 0 as *mut i8;

    for n in 0..g_numBots {
        value = crate::src::qcommon::q_shared::Info_ValueForKey(
            g_botInfos[n as usize],
            b"name\x00" as *const u8 as *const i8,
        );

        if crate::src::qcommon::q_shared::Q_stricmp(value, name) == 0 {
            return g_botInfos[n as usize];
        }
    }
    return 0 as *mut i8;
}
/*
===============
G_InitBots
===============
*/
#[no_mangle]

pub unsafe extern "C" fn G_InitBots(mut restart: crate::src::qcommon::q_shared::qboolean) {
    let mut fragLimit: i32 = 0;
    let mut timeLimit: i32 = 0;
    let mut arenainfo: *const i8 = 0 as *const i8;
    let mut strValue: *mut i8 = 0 as *mut i8;
    let mut basedelay: i32 = 0;
    let mut map: [i8; 64] = [0; 64];
    let mut serverinfo: [i8; 1024] = [0; 1024];
    G_LoadBots();
    G_LoadArenas();
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_minplayers,
        b"bot_minplayers\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x4,
    );
    if crate::src::game::g_main::g_gametype.integer == crate::bg_public_h::GT_SINGLE_PLAYER as i32 {
        crate::src::game::g_syscalls::trap_GetServerinfo(
            serverinfo.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        crate::src::qcommon::q_shared::Q_strncpyz(
            map.as_mut_ptr(),
            crate::src::qcommon::q_shared::Info_ValueForKey(
                serverinfo.as_mut_ptr(),
                b"mapname\x00" as *const u8 as *const i8,
            ),
            ::std::mem::size_of::<[i8; 64]>() as i32,
        );
        arenainfo = G_GetArenaInfoByMap(map.as_mut_ptr());
        if arenainfo.is_null() {
            return;
        }
        strValue = crate::src::qcommon::q_shared::Info_ValueForKey(
            arenainfo,
            b"fraglimit\x00" as *const u8 as *const i8,
        );
        fragLimit = atoi(strValue);
        if fragLimit != 0 {
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"fraglimit\x00" as *const u8 as *const i8,
                strValue,
            );
        } else {
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"fraglimit\x00" as *const u8 as *const i8,
                b"0\x00" as *const u8 as *const i8,
            );
        }
        strValue = crate::src::qcommon::q_shared::Info_ValueForKey(
            arenainfo,
            b"timelimit\x00" as *const u8 as *const i8,
        );
        timeLimit = atoi(strValue);
        if timeLimit != 0 {
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"timelimit\x00" as *const u8 as *const i8,
                strValue,
            );
        } else {
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"timelimit\x00" as *const u8 as *const i8,
                b"0\x00" as *const u8 as *const i8,
            );
        }
        if fragLimit == 0 && timeLimit == 0 {
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"fraglimit\x00" as *const u8 as *const i8,
                b"10\x00" as *const u8 as *const i8,
            );
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"timelimit\x00" as *const u8 as *const i8,
                b"0\x00" as *const u8 as *const i8,
            );
        }
        basedelay = 2000;
        strValue = crate::src::qcommon::q_shared::Info_ValueForKey(
            arenainfo,
            b"special\x00" as *const u8 as *const i8,
        );
        if crate::src::qcommon::q_shared::Q_stricmp(
            strValue,
            b"training\x00" as *const u8 as *const i8,
        ) == 0
        {
            basedelay += 10000
        }
        if restart as u64 == 0 {
            G_SpawnBots(
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    arenainfo,
                    b"bots\x00" as *const u8 as *const i8,
                ),
                basedelay,
            );
        }
    };
}
