#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast,
           custom_attribute,
           libc,
           ptr_wrapping_offset_from)]
use ai_main::{
    bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown, BotAIShutdownClient,
    BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
};
use bg_misc::{
    bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
    BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
    BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
    BG_PlayerTouchesItem, BG_TouchJumpPad,
};
use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
use bg_public_h::{
    gitem_s, gitem_t, itemType_t, team_t, unnamed, unnamed_0, unnamed_1, ET_BEAM, ET_EVENTS,
    ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL,
    ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, EV_BULLET, EV_BULLET_HIT_FLESH,
    EV_BULLET_HIT_WALL, EV_CHANGE_WEAPON, EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE,
    EV_FALL_FAR, EV_FALL_MEDIUM, EV_FALL_SHORT, EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP,
    EV_FOOTSTEP_METAL, EV_FOOTWADE, EV_GENERAL_SOUND, EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP,
    EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND, EV_GRENADE_BOUNCE, EV_INVUL_IMPACT, EV_ITEM_PICKUP,
    EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED, EV_JUMP, EV_JUMP_PAD, EV_KAMIKAZE, EV_LIGHTNINGBOLT,
    EV_MISSILE_HIT, EV_MISSILE_MISS, EV_MISSILE_MISS_METAL, EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE,
    EV_OBELISKPAIN, EV_OBITUARY, EV_PAIN, EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT,
    EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD, EV_POWERUP_REGEN, EV_PROXIMITY_MINE_STICK,
    EV_PROXIMITY_MINE_TRIGGER, EV_RAILTRAIL, EV_SCOREPLUM, EV_SHOTGUN, EV_STEP_12, EV_STEP_16,
    EV_STEP_4, EV_STEP_8, EV_STOPLOOPINGSOUND, EV_SWIM, EV_TAUNT, EV_TAUNT_FOLLOWME,
    EV_TAUNT_GETFLAG, EV_TAUNT_GUARDBASE, EV_TAUNT_NO, EV_TAUNT_PATROL, EV_TAUNT_YES, EV_USE_ITEM0,
    EV_USE_ITEM1, EV_USE_ITEM10, EV_USE_ITEM11, EV_USE_ITEM12, EV_USE_ITEM13, EV_USE_ITEM14,
    EV_USE_ITEM15, EV_USE_ITEM2, EV_USE_ITEM3, EV_USE_ITEM4, EV_USE_ITEM5, EV_USE_ITEM6,
    EV_USE_ITEM7, EV_USE_ITEM8, EV_USE_ITEM9, EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH,
    EV_WATER_UNDER, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP,
    IT_POWERUP, IT_TEAM, IT_WEAPON, MOD_BFG, MOD_BFG_SPLASH, MOD_CRUSH, MOD_FALLING, MOD_GAUNTLET,
    MOD_GRAPPLE, MOD_GRENADE, MOD_GRENADE_SPLASH, MOD_LAVA, MOD_LIGHTNING, MOD_MACHINEGUN,
    MOD_PLASMA, MOD_PLASMA_SPLASH, MOD_RAILGUN, MOD_ROCKET, MOD_ROCKET_SPLASH, MOD_SHOTGUN,
    MOD_SLIME, MOD_SUICIDE, MOD_TARGET_LASER, MOD_TELEFRAG, MOD_TRIGGER_HURT, MOD_UNKNOWN,
    MOD_WATER, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
use g_active::{ClientEndFrame, ClientThink, G_RunClient};
use g_arenas::{
    podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f, UpdateTournamentInfo,
};
use g_bot::{
    G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
    Svcmd_BotList_f,
};
use g_client::{
    ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
    CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch, SP_info_player_intermission,
    SP_info_player_start, SelectSpawnPoint, SetClientViewAngle, SpotWouldTelefrag, TeamCount,
    TeamLeader,
};
use g_cmds::{
    BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
    DeathmatchScoreboardMessage, SetTeam, StopFollowing,
};
use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
use g_items::{
    ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem, G_SpawnItem,
    RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
};
use g_local_h::{
    clientConnected_t, clientPersistant_t, clientSession_t, gclient_s, gentity_s, gentity_t,
    level_locals_t, moverState_t, playerTeamStateState_t, playerTeamState_t, spectatorState_t,
    trap_DebugPolygonCreate, trap_EntitiesInBox, trap_GetConfigstring, trap_LinkEntity,
    trap_LocateGameData, trap_SendServerCommand, trap_SetConfigstring, trap_UnlinkEntity,
    CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED, MOVER_1TO2, MOVER_2TO1, MOVER_POS1,
    MOVER_POS2, SPECTATOR_FOLLOW, SPECTATOR_FREE, SPECTATOR_NOT, SPECTATOR_SCOREBOARD, TEAM_ACTIVE,
    TEAM_BEGIN,
};
use g_main::{
    g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
    g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn, g_friendlyFire,
    g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref, g_maxGameClients,
    g_maxclients, g_motd, g_password, g_quadfactor, g_restarted, g_smoothClients, g_speed,
    g_synchronousClients, g_teamAutoJoin, g_teamForceBalance, g_weaponRespawn, g_weaponTeamRespawn,
    level, pmove_fixed, pmove_msec, AddTournamentQueue, BeginIntermission, CalculateRanks,
    CheckTeamLeader, ExitLevel, FindIntermissionPoint, G_RunThink, MoveClientToIntermission,
    SetLeader,
};
use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
use g_misc::{
    SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model, SP_misc_portal_camera,
    SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade, SP_shooter_plasma,
    SP_shooter_rocket, TeleportPlayer,
};
use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
use g_mover::{
    G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
    SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
};
use g_public_h::entityShared_t;
use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
use g_spawn::{G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector};
use g_svcmds::{ConsoleCommand, G_FilterPacket, G_ProcessIPBans};
use g_target::{
    SP_target_delay, SP_target_give, SP_target_kill, SP_target_laser, SP_target_location,
    SP_target_position, SP_target_print, SP_target_relay, SP_target_remove_powerups,
    SP_target_score, SP_target_speaker, SP_target_teleporter,
};
use g_team::{
    OnSameTeam, SP_team_CTF_blueplayer, SP_team_CTF_bluespawn, SP_team_CTF_redplayer,
    SP_team_CTF_redspawn, Team_CheckDroppedItem,
};
use g_trigger::{
    SP_func_timer, SP_target_push, SP_trigger_always, SP_trigger_hurt, SP_trigger_multiple,
    SP_trigger_push, SP_trigger_teleport,
};
use g_variadic_h::{G_Error, G_Printf};
use g_weapon::{
    CheckGauntletAttack, FireWeapon, LogAccuracyHit, SnapVectorTowards, Weapon_HookFree,
    Weapon_HookThink,
};
use q_math::{
    vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
    DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, cplane_s, cplane_t, entityState_s, entityState_t, fileHandle_t, playerState_s,
    playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t, usercmd_s, usercmd_t,
    va, vec3_t, vec_t, Com_sprintf, Q_strcat, Q_stricmp, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stddef_h::size_t;
use stdlib::{atan2, memset, rand, strcmp, strcpy};
extern crate libc;

unsafe extern "C" fn VectorCompare(mut v1: *const vec_t, mut v2: *const vec_t) -> libc::c_int {
    if *v1.offset(0isize) != *v2.offset(0isize)
        || *v1.offset(1isize) != *v2.offset(1isize)
        || *v1.offset(2isize) != *v2.offset(2isize)
    {
        return 0i32;
    }
    return 1i32;
}
unsafe extern "C" fn CrossProduct(
    mut v1: *const vec_t,
    mut v2: *const vec_t,
    mut cross: *mut vec_t,
) {
    *cross.offset(0isize) =
        *v1.offset(1isize) * *v2.offset(2isize) - *v1.offset(2isize) * *v2.offset(1isize);
    *cross.offset(1isize) =
        *v1.offset(2isize) * *v2.offset(0isize) - *v1.offset(0isize) * *v2.offset(2isize);
    *cross.offset(2isize) =
        *v1.offset(0isize) * *v2.offset(1isize) - *v1.offset(1isize) * *v2.offset(0isize);
}
//
// g_utils.c
//
#[no_mangle]
pub unsafe extern "C" fn G_ModelIndex(mut name: *mut libc::c_char) -> libc::c_int {
    return G_FindConfigstringIndex(name, 32i32, 256i32, qtrue);
}
/*
=========================================================================

model / sound configstring indexes

=========================================================================
*/
/*
================
G_FindConfigstringIndex

================
*/
#[no_mangle]
pub unsafe extern "C" fn G_FindConfigstringIndex(
    mut name: *mut libc::c_char,
    mut start: libc::c_int,
    mut max: libc::c_int,
    mut create: qboolean,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut s: [libc::c_char; 1024] = [0; 1024];
    if name.is_null() || 0 == *name.offset(0isize) {
        return 0i32;
    }
    i = 1i32;
    while i < max {
        trap_GetConfigstring(
            start + i,
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        );
        if 0 == s[0usize] {
            break;
        }
        if 0 == strcmp(s.as_mut_ptr(), name) {
            return i;
        }
        i += 1
    }
    if 0 == create as u64 {
        return 0i32;
    }
    if i == max {
        G_Error(b"G_FindConfigstringIndex: overflow\x00" as *const u8 as *const libc::c_char);
    }
    trap_SetConfigstring(start + i, name);
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn G_SoundIndex(mut name: *mut libc::c_char) -> libc::c_int {
    return G_FindConfigstringIndex(name, 32i32 + 256i32, 256i32, qtrue);
}
#[no_mangle]
pub unsafe extern "C" fn G_TeamCommand(mut team: team_t, mut cmd: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < level.maxclients {
        if (*level.clients.offset(i as isize)).pers.connected as libc::c_uint
            == CON_CONNECTED as libc::c_int as libc::c_uint
        {
            if (*level.clients.offset(i as isize)).sess.sessionTeam as libc::c_uint
                == team as libc::c_uint
            {
                trap_SendServerCommand(
                    i,
                    va(
                        b"%s\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        cmd,
                    ),
                );
            }
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_KillBox(mut ent: *mut gentity_t) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut touch: [libc::c_int; 1024] = [0; 1024];
    let mut hit: *mut gentity_t = 0 as *mut gentity_t;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    mins[0usize] = (*(*ent).client).ps.origin[0usize] + (*ent).r.mins[0usize];
    mins[1usize] = (*(*ent).client).ps.origin[1usize] + (*ent).r.mins[1usize];
    mins[2usize] = (*(*ent).client).ps.origin[2usize] + (*ent).r.mins[2usize];
    maxs[0usize] = (*(*ent).client).ps.origin[0usize] + (*ent).r.maxs[0usize];
    maxs[1usize] = (*(*ent).client).ps.origin[1usize] + (*ent).r.maxs[1usize];
    maxs[2usize] = (*(*ent).client).ps.origin[2usize] + (*ent).r.maxs[2usize];
    num = trap_EntitiesInBox(
        mins.as_mut_ptr() as *const vec_t,
        maxs.as_mut_ptr() as *const vec_t,
        touch.as_mut_ptr(),
        1i32 << 10i32,
    );
    i = 0i32;
    while i < num {
        hit = &mut *g_entities
            .as_mut_ptr()
            .offset(*touch.as_mut_ptr().offset(i as isize) as isize)
            as *mut gentity_t;
        if !(*hit).client.is_null() {
            G_Damage(
                hit,
                ent,
                ent,
                0 as *mut vec_t,
                0 as *mut vec_t,
                100000i32,
                0x8i32,
                MOD_TELEFRAG as libc::c_int,
            );
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_Find(
    mut from: *mut gentity_t,
    mut fieldofs: libc::c_int,
    mut match_0: *const libc::c_char,
) -> *mut gentity_t {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if from.is_null() {
        from = g_entities.as_mut_ptr()
    } else {
        from = from.offset(1isize)
    }
    while from < &mut *g_entities.as_mut_ptr().offset(level.num_entities as isize) as *mut gentity_t
    {
        if !(0 == (*from).inuse as u64) {
            s = *((from as *mut byte).offset(fieldofs as isize) as *mut *mut libc::c_char);
            if !s.is_null() {
                if 0 == Q_stricmp(s, match_0) {
                    return from;
                }
            }
        }
        from = from.offset(1isize)
    }
    return 0 as *mut gentity_t;
}
#[no_mangle]
pub unsafe extern "C" fn G_PickTarget(mut targetname: *mut libc::c_char) -> *mut gentity_t {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut num_choices: libc::c_int = 0i32;
    let mut choice: [*mut gentity_t; 32] = [0 as *mut gentity_t; 32];
    if targetname.is_null() {
        G_Printf(
            b"G_PickTarget called with NULL targetname\n\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut gentity_t;
    }
    loop {
        ent = G_Find(
            ent,
            &mut (*(0 as *mut gentity_t)).targetname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            targetname,
        );
        if ent.is_null() {
            break;
        }
        let fresh0 = num_choices;
        num_choices = num_choices + 1;
        choice[fresh0 as usize] = ent;
        if num_choices == 32i32 {
            break;
        }
    }
    if 0 == num_choices {
        G_Printf(
            b"G_PickTarget: target %s not found\n\x00" as *const u8 as *const libc::c_char,
            targetname,
        );
        return 0 as *mut gentity_t;
    }
    return choice[(rand() % num_choices) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn G_UseTargets(mut ent: *mut gentity_t, mut activator: *mut gentity_t) {
    let mut t: *mut gentity_t = 0 as *mut gentity_t;
    if ent.is_null() {
        return;
    }
    if !(*ent).targetShaderName.is_null() && !(*ent).targetShaderNewName.is_null() {
        let mut f: libc::c_float = (level.time as libc::c_double * 0.001f64) as libc::c_float;
        AddRemap((*ent).targetShaderName, (*ent).targetShaderNewName, f);
        trap_SetConfigstring(24i32, BuildShaderStateConfig());
    }
    if (*ent).target.is_null() {
        return;
    }
    t = 0 as *mut gentity_t;
    loop {
        t = G_Find(
            t,
            &mut (*(0 as *mut gentity_t)).targetname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            (*ent).target,
        );
        if t.is_null() {
            break;
        }
        if t == ent {
            G_Printf(b"WARNING: Entity used itself.\n\x00" as *const u8 as *const libc::c_char);
        } else if (*t).use_0.is_some() {
            (*t).use_0.expect("non-null function pointer")(t, ent, activator);
        }
        if 0 == (*ent).inuse as u64 {
            G_Printf(
                b"entity was removed while using targets\n\x00" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn BuildShaderStateConfig() -> *const libc::c_char {
    static mut buff: [libc::c_char; 4096] = [0; 4096];
    let mut out: [libc::c_char; 133] = [0; 133];
    let mut i: libc::c_int = 0;
    memset(
        buff.as_mut_ptr() as *mut libc::c_void,
        0i32,
        1024i32 as libc::c_ulong,
    );
    i = 0i32;
    while i < remapCount {
        Com_sprintf(
            out.as_mut_ptr(),
            64i32 * 2i32 + 5i32,
            b"%s=%s:%5.2f@\x00" as *const u8 as *const libc::c_char,
            remappedShaders[i as usize].oldShader.as_mut_ptr(),
            remappedShaders[i as usize].newShader.as_mut_ptr(),
            remappedShaders[i as usize].timeOffset as libc::c_double,
        );
        Q_strcat(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong as libc::c_int,
            out.as_mut_ptr(),
        );
        i += 1
    }
    return buff.as_mut_ptr();
}
#[no_mangle]
pub static mut remappedShaders: [shaderRemap_t; 128] = [shaderRemap_t {
    oldShader: [0; 64],
    newShader: [0; 64],
    timeOffset: 0.,
}; 128];
#[no_mangle]
pub static mut remapCount: libc::c_int = 0i32;
#[no_mangle]
pub unsafe extern "C" fn AddRemap(
    mut oldShader: *const libc::c_char,
    mut newShader: *const libc::c_char,
    mut timeOffset: libc::c_float,
) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < remapCount {
        if Q_stricmp(
            oldShader,
            remappedShaders[i as usize].oldShader.as_mut_ptr(),
        ) == 0i32
        {
            strcpy(
                remappedShaders[i as usize].newShader.as_mut_ptr(),
                newShader,
            );
            remappedShaders[i as usize].timeOffset = timeOffset;
            return;
        }
        i += 1
    }
    if remapCount < 128i32 {
        strcpy(
            remappedShaders[remapCount as usize].newShader.as_mut_ptr(),
            newShader,
        );
        strcpy(
            remappedShaders[remapCount as usize].oldShader.as_mut_ptr(),
            oldShader,
        );
        remappedShaders[remapCount as usize].timeOffset = timeOffset;
        remapCount += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn G_SetMovedir(mut angles: *mut vec_t, mut movedir: *mut vec_t) {
    static mut VEC_UP: vec3_t = [0i32 as vec_t, -1i32 as vec_t, 0i32 as vec_t];
    static mut MOVEDIR_UP: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    static mut VEC_DOWN: vec3_t = [0i32 as vec_t, -2i32 as vec_t, 0i32 as vec_t];
    static mut MOVEDIR_DOWN: vec3_t = [0i32 as vec_t, 0i32 as vec_t, -1i32 as vec_t];
    if 0 != VectorCompare(angles as *const vec_t, VEC_UP.as_mut_ptr() as *const vec_t) {
        *movedir.offset(0isize) = MOVEDIR_UP[0usize];
        *movedir.offset(1isize) = MOVEDIR_UP[1usize];
        *movedir.offset(2isize) = MOVEDIR_UP[2usize]
    } else if 0
        != VectorCompare(
            angles as *const vec_t,
            VEC_DOWN.as_mut_ptr() as *const vec_t,
        )
    {
        *movedir.offset(0isize) = MOVEDIR_DOWN[0usize];
        *movedir.offset(1isize) = MOVEDIR_DOWN[1usize];
        *movedir.offset(2isize) = MOVEDIR_DOWN[2usize]
    } else {
        AngleVectors(
            angles as *const vec_t,
            movedir,
            0 as *mut vec_t,
            0 as *mut vec_t,
        );
    }
    let ref mut fresh2 = *angles.offset(1isize);
    let ref mut fresh1 = *angles.offset(2isize);
    *fresh1 = 0i32 as vec_t;
    *fresh2 = *fresh1;
    *angles.offset(0isize) = *fresh2;
}
#[no_mangle]
pub unsafe extern "C" fn G_InitGentity(mut e: *mut gentity_t) {
    (*e).inuse = qtrue;
    (*e).classname = b"noclass\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*e).s.number = e.wrapping_offset_from(g_entities.as_mut_ptr()) as libc::c_long as libc::c_int;
    (*e).r.ownerNum = (1i32 << 10i32) - 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn G_Spawn() -> *mut gentity_t {
    let mut i: libc::c_int = 0;
    let mut force: libc::c_int = 0;
    let mut e: *mut gentity_t = 0 as *mut gentity_t;
    e = 0 as *mut gentity_t;
    force = 0i32;
    while force < 2i32 {
        e = &mut *g_entities.as_mut_ptr().offset(64isize) as *mut gentity_t;
        i = 64i32;
        while i < level.num_entities {
            if !(0 != (*e).inuse as u64) {
                // the first couple seconds of server time can involve a lot of
                // freeing and allocating, so relax the replacement policy
                if !(0 == force
                    && (*e).freetime > level.startTime + 2000i32
                    && level.time - (*e).freetime < 1000i32)
                {
                    G_InitGentity(e);
                    return e;
                }
            }
            i += 1;
            e = e.offset(1isize)
        }
        if level.num_entities < (1i32 << 10i32) - 2i32 {
            break;
        }
        force += 1
    }
    if level.num_entities == (1i32 << 10i32) - 2i32 {
        i = 0i32;
        while i < 1i32 << 10i32 {
            G_Printf(
                b"%4i: %s\n\x00" as *const u8 as *const libc::c_char,
                i,
                g_entities[i as usize].classname,
            );
            i += 1
        }
        G_Error(b"G_Spawn: no free entities\x00" as *const u8 as *const libc::c_char);
    }
    level.num_entities += 1;
    trap_LocateGameData(
        level.gentities,
        level.num_entities,
        ::std::mem::size_of::<gentity_t>() as libc::c_ulong as libc::c_int,
        &mut (*level.clients.offset(0isize)).ps,
        ::std::mem::size_of::<gclient_s>() as libc::c_ulong as libc::c_int,
    );
    G_InitGentity(e);
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn G_TempEntity(
    mut origin: *mut vec_t,
    mut event: libc::c_int,
) -> *mut gentity_t {
    let mut e: *mut gentity_t = 0 as *mut gentity_t;
    let mut snapped: vec3_t = [0.; 3];
    e = G_Spawn();
    (*e).s.eType = ET_EVENTS as libc::c_int + event;
    (*e).classname = b"tempEntity\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*e).eventTime = level.time;
    (*e).freeAfterEvent = qtrue;
    snapped[0usize] = *origin.offset(0isize);
    snapped[1usize] = *origin.offset(1isize);
    snapped[2usize] = *origin.offset(2isize);
    snapped[0usize] = snapped[0usize] as libc::c_int as vec_t;
    snapped[1usize] = snapped[1usize] as libc::c_int as vec_t;
    snapped[2usize] = snapped[2usize] as libc::c_int as vec_t;
    G_SetOrigin(e, snapped.as_mut_ptr());
    trap_LinkEntity(e);
    return e;
}
#[no_mangle]
pub unsafe extern "C" fn G_SetOrigin(mut ent: *mut gentity_t, mut origin: *mut vec_t) {
    (*ent).s.pos.trBase[0usize] = *origin.offset(0isize);
    (*ent).s.pos.trBase[1usize] = *origin.offset(1isize);
    (*ent).s.pos.trBase[2usize] = *origin.offset(2isize);
    (*ent).s.pos.trType = TR_STATIONARY;
    (*ent).s.pos.trTime = 0i32;
    (*ent).s.pos.trDuration = 0i32;
    (*ent).s.pos.trDelta[2usize] = 0i32 as vec_t;
    (*ent).s.pos.trDelta[1usize] = (*ent).s.pos.trDelta[2usize];
    (*ent).s.pos.trDelta[0usize] = (*ent).s.pos.trDelta[1usize];
    (*ent).r.currentOrigin[0usize] = *origin.offset(0isize);
    (*ent).r.currentOrigin[1usize] = *origin.offset(1isize);
    (*ent).r.currentOrigin[2usize] = *origin.offset(2isize);
}
#[no_mangle]
pub unsafe extern "C" fn G_Sound(
    mut ent: *mut gentity_t,
    mut channel: libc::c_int,
    mut soundIndex: libc::c_int,
) {
    let mut te: *mut gentity_t = 0 as *mut gentity_t;
    te = G_TempEntity(
        (*ent).r.currentOrigin.as_mut_ptr(),
        EV_GENERAL_SOUND as libc::c_int,
    );
    (*te).s.eventParm = soundIndex;
}
#[no_mangle]
pub unsafe extern "C" fn G_FreeEntity(mut ed: *mut gentity_t) {
    trap_UnlinkEntity(ed);
    if 0 != (*ed).neverFree as u64 {
        return;
    }
    memset(
        ed as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<gentity_t>() as libc::c_ulong,
    );
    (*ed).classname = b"freed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*ed).freetime = level.time;
    (*ed).inuse = qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn G_EntitiesFree() -> qboolean {
    let mut i: libc::c_int = 0;
    let mut e: *mut gentity_t = 0 as *mut gentity_t;
    if level.num_entities < (1i32 << 10i32) - 2i32 {
        return qtrue;
    }
    e = &mut *g_entities.as_mut_ptr().offset(64isize) as *mut gentity_t;
    i = 64i32;
    while i < level.num_entities {
        if 0 != (*e).inuse as u64 {
            i += 1;
            e = e.offset(1isize)
        } else {
            return qtrue;
        }
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn tv(
    mut x: libc::c_float,
    mut y: libc::c_float,
    mut z: libc::c_float,
) -> *mut libc::c_float {
    static mut index: libc::c_int = 0;
    static mut vecs: [vec3_t; 8] = [[0.; 3]; 8];
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    v = vecs[index as usize].as_mut_ptr();
    index = index + 1i32 & 7i32;
    *v.offset(0isize) = x;
    *v.offset(1isize) = y;
    *v.offset(2isize) = z;
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn vtos(mut v: *const vec_t) -> *mut libc::c_char {
    static mut index: libc::c_int = 0;
    static mut str: [[libc::c_char; 32]; 8] = [[0; 32]; 8];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    s = str[index as usize].as_mut_ptr();
    index = index + 1i32 & 7i32;
    Com_sprintf(
        s,
        32i32,
        b"(%i %i %i)\x00" as *const u8 as *const libc::c_char,
        *v.offset(0isize) as libc::c_int,
        *v.offset(1isize) as libc::c_int,
        *v.offset(2isize) as libc::c_int,
    );
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn vectoyaw(mut vec: *const vec_t) -> libc::c_float {
    let mut yaw: libc::c_float = 0.;
    if *vec.offset(1isize) == 0i32 as libc::c_float && *vec.offset(0isize) == 0i32 as libc::c_float
    {
        yaw = 0i32 as libc::c_float
    } else {
        if 0. != *vec.offset(0isize) {
            yaw = (atan2(
                *vec.offset(1isize) as libc::c_double,
                *vec.offset(0isize) as libc::c_double,
            ) * 180i32 as libc::c_double
                / 3.14159265358979323846f64) as libc::c_float
        } else if *vec.offset(1isize) > 0i32 as libc::c_float {
            yaw = 90i32 as libc::c_float
        } else {
            yaw = 270i32 as libc::c_float
        }
        if yaw < 0i32 as libc::c_float {
            yaw += 360i32 as libc::c_float
        }
    }
    return yaw;
}
#[no_mangle]
pub unsafe extern "C" fn G_AddPredictableEvent(
    mut ent: *mut gentity_t,
    mut event: libc::c_int,
    mut eventParm: libc::c_int,
) {
    if (*ent).client.is_null() {
        return;
    }
    BG_AddPredictableEventToPlayerstate(event, eventParm, &mut (*(*ent).client).ps);
}
#[no_mangle]
pub unsafe extern "C" fn G_AddEvent(
    mut ent: *mut gentity_t,
    mut event: libc::c_int,
    mut eventParm: libc::c_int,
) {
    let mut bits: libc::c_int = 0;
    if 0 == event {
        G_Printf(
            b"G_AddEvent: zero event added for entity %i\n\x00" as *const u8 as *const libc::c_char,
            (*ent).s.number,
        );
        return;
    }
    if !(*ent).client.is_null() {
        bits = (*(*ent).client).ps.externalEvent & (0x100i32 | 0x200i32);
        bits = bits + 0x100i32 & (0x100i32 | 0x200i32);
        (*(*ent).client).ps.externalEvent = event | bits;
        (*(*ent).client).ps.externalEventParm = eventParm;
        (*(*ent).client).ps.externalEventTime = level.time
    } else {
        bits = (*ent).s.event & (0x100i32 | 0x200i32);
        bits = bits + 0x100i32 & (0x100i32 | 0x200i32);
        (*ent).s.event = event | bits;
        (*ent).s.eventParm = eventParm
    }
    (*ent).eventTime = level.time;
}
/*
================
DebugLine

  debug polygons only work when running a local game
  with r_debugSurface set to 2
================
*/
#[no_mangle]
pub unsafe extern "C" fn DebugLine(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut color: libc::c_int,
) -> libc::c_int {
    let mut points: [vec3_t; 4] = [[0.; 3]; 4];
    let mut dir: vec3_t = [0.; 3];
    let mut cross: vec3_t = [0.; 3];
    let mut up: vec3_t = [0i32 as vec_t, 0i32 as vec_t, 1i32 as vec_t];
    let mut dot: libc::c_float = 0.;
    points[0usize][0usize] = *start.offset(0isize);
    points[0usize][1usize] = *start.offset(1isize);
    points[0usize][2usize] = *start.offset(2isize);
    points[1usize][0usize] = *start.offset(0isize);
    points[1usize][1usize] = *start.offset(1isize);
    points[1usize][2usize] = *start.offset(2isize);
    points[2usize][0usize] = *end.offset(0isize);
    points[2usize][1usize] = *end.offset(1isize);
    points[2usize][2usize] = *end.offset(2isize);
    points[3usize][0usize] = *end.offset(0isize);
    points[3usize][1usize] = *end.offset(1isize);
    points[3usize][2usize] = *end.offset(2isize);
    dir[0usize] = *end.offset(0isize) - *start.offset(0isize);
    dir[1usize] = *end.offset(1isize) - *start.offset(1isize);
    dir[2usize] = *end.offset(2isize) - *start.offset(2isize);
    VectorNormalize(dir.as_mut_ptr());
    dot = dir[0usize] * up[0usize] + dir[1usize] * up[1usize] + dir[2usize] * up[2usize];
    if dot as libc::c_double > 0.99f64 || (dot as libc::c_double) < -0.99f64 {
        cross[0usize] = 1i32 as vec_t;
        cross[1usize] = 0i32 as vec_t;
        cross[2usize] = 0i32 as vec_t
    } else {
        CrossProduct(
            dir.as_mut_ptr() as *const vec_t,
            up.as_mut_ptr() as *const vec_t,
            cross.as_mut_ptr(),
        );
    }
    VectorNormalize(cross.as_mut_ptr());
    points[0usize][0usize] = points[0usize][0usize] + cross[0usize] * 2i32 as libc::c_float;
    points[0usize][1usize] = points[0usize][1usize] + cross[1usize] * 2i32 as libc::c_float;
    points[0usize][2usize] = points[0usize][2usize] + cross[2usize] * 2i32 as libc::c_float;
    points[1usize][0usize] = points[1usize][0usize] + cross[0usize] * -2i32 as libc::c_float;
    points[1usize][1usize] = points[1usize][1usize] + cross[1usize] * -2i32 as libc::c_float;
    points[1usize][2usize] = points[1usize][2usize] + cross[2usize] * -2i32 as libc::c_float;
    points[2usize][0usize] = points[2usize][0usize] + cross[0usize] * -2i32 as libc::c_float;
    points[2usize][1usize] = points[2usize][1usize] + cross[1usize] * -2i32 as libc::c_float;
    points[2usize][2usize] = points[2usize][2usize] + cross[2usize] * -2i32 as libc::c_float;
    points[3usize][0usize] = points[3usize][0usize] + cross[0usize] * 2i32 as libc::c_float;
    points[3usize][1usize] = points[3usize][1usize] + cross[1usize] * 2i32 as libc::c_float;
    points[3usize][2usize] = points[3usize][2usize] + cross[2usize] * 2i32 as libc::c_float;
    return trap_DebugPolygonCreate(color, 4i32, points.as_mut_ptr());
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shaderRemap_t {
    pub oldShader: [libc::c_char; 64],
    pub newShader: [libc::c_char; 64],
    pub timeOffset: libc::c_float,
}
