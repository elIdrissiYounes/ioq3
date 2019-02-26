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
    gitem_s, gitem_t, itemType_t, team_t, unnamed, unnamed_0, unnamed_1, unnamed_2, unnamed_3,
    unnamed_4, weapon_t, ET_BEAM, ET_EVENTS, ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM,
    ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM,
    ET_TELEPORT_TRIGGER, EV_BULLET, EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL, EV_CHANGE_WEAPON,
    EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE, EV_FALL_FAR, EV_FALL_MEDIUM, EV_FALL_SHORT,
    EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP, EV_FOOTSTEP_METAL, EV_FOOTWADE, EV_GENERAL_SOUND,
    EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP, EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND, EV_GRENADE_BOUNCE,
    EV_INVUL_IMPACT, EV_ITEM_PICKUP, EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED, EV_JUMP, EV_JUMP_PAD,
    EV_KAMIKAZE, EV_LIGHTNINGBOLT, EV_MISSILE_HIT, EV_MISSILE_MISS, EV_MISSILE_MISS_METAL,
    EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE, EV_OBELISKPAIN, EV_OBITUARY, EV_PAIN,
    EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT, EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD,
    EV_POWERUP_REGEN, EV_PROXIMITY_MINE_STICK, EV_PROXIMITY_MINE_TRIGGER, EV_RAILTRAIL,
    EV_SCOREPLUM, EV_SHOTGUN, EV_STEP_12, EV_STEP_16, EV_STEP_4, EV_STEP_8, EV_STOPLOOPINGSOUND,
    EV_SWIM, EV_TAUNT, EV_TAUNT_FOLLOWME, EV_TAUNT_GETFLAG, EV_TAUNT_GUARDBASE, EV_TAUNT_NO,
    EV_TAUNT_PATROL, EV_TAUNT_YES, EV_USE_ITEM0, EV_USE_ITEM1, EV_USE_ITEM10, EV_USE_ITEM11,
    EV_USE_ITEM12, EV_USE_ITEM13, EV_USE_ITEM14, EV_USE_ITEM15, EV_USE_ITEM2, EV_USE_ITEM3,
    EV_USE_ITEM4, EV_USE_ITEM5, EV_USE_ITEM6, EV_USE_ITEM7, EV_USE_ITEM8, EV_USE_ITEM9,
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, GT_1FCTF, GT_CTF, GT_FFA,
    GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT,
    HI_INVULNERABILITY, HI_KAMIKAZE, HI_MEDKIT, HI_NONE, HI_NUM_HOLDABLE, HI_PORTAL, HI_TELEPORTER,
    IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM,
    IT_WEAPON, PERS_ASSIST_COUNT, PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES,
    PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT, PERS_GAUNTLET_FRAG_COUNT, PERS_HITS,
    PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS, PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT,
    PERS_TEAM, STAT_ARMOR, STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM,
    STAT_MAX_HEALTH, STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
    WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN,
    WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
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
use g_local_h::{
    clientConnected_t, clientPersistant_t, clientSession_t, gclient_s, gclient_t, gentity_s,
    gentity_t, level_locals_t, moverState_t, playerTeamStateState_t, playerTeamState_t,
    spectatorState_t, trap_Cvar_VariableIntegerValue, trap_LinkEntity, trap_PointContents,
    trap_SetConfigstring, trap_Trace, CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED, MOVER_1TO2,
    MOVER_2TO1, MOVER_POS1, MOVER_POS2, SPECTATOR_FOLLOW, SPECTATOR_FREE, SPECTATOR_NOT,
    SPECTATOR_SCOREBOARD, TEAM_ACTIVE, TEAM_BEGIN,
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
    OnSameTeam, Pickup_Team, SP_team_CTF_blueplayer, SP_team_CTF_bluespawn, SP_team_CTF_redplayer,
    SP_team_CTF_redspawn, Team_CheckDroppedItem, Team_DroppedFlagThink, Team_FreeEntity,
    Team_InitGame,
};
use g_trigger::{
    SP_func_timer, SP_target_push, SP_trigger_always, SP_trigger_hurt, SP_trigger_multiple,
    SP_trigger_push, SP_trigger_teleport,
};
use g_utils::{
    tv, vectoyaw, vtos, G_AddEvent, G_AddPredictableEvent, G_Find, G_FreeEntity, G_InitGentity,
    G_KillBox, G_ModelIndex, G_PickTarget, G_SetMovedir, G_SetOrigin, G_Sound, G_SoundIndex,
    G_Spawn, G_TeamCommand, G_TempEntity, G_UseTargets,
};
use g_variadic_h::{G_Error, G_LogPrintf, G_Printf};
use g_weapon::{
    CheckGauntletAttack, FireWeapon, LogAccuracyHit, SnapVectorTowards, Weapon_HookFree,
    Weapon_HookThink,
};
use q_math::{
    vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
    DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, fileHandle_t,
    playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t,
    usercmd_s, usercmd_t, vec3_t, vec_t, vmCvar_t, Com_sprintf, TR_GRAVITY, TR_INTERPOLATE,
    TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stdlib::{memset, rand};
extern crate libc;

//
// g_items.c
//
#[no_mangle]
pub unsafe extern "C" fn G_CheckTeamItems() {
    Team_InitGame();
    if g_gametype.integer == GT_CTF as libc::c_int {
        let mut item: *mut gitem_t = 0 as *mut gitem_t;
        item = BG_FindItem(b"Red Flag\x00" as *const u8 as *const libc::c_char);
        if item.is_null()
            || 0 == itemRegistered
                [item.wrapping_offset_from(bg_itemlist.as_mut_ptr()) as libc::c_long as usize]
                as u64
        {
            G_Printf(
                b"^3WARNING: No team_CTF_redflag in map\n\x00" as *const u8 as *const libc::c_char,
            );
        }
        item = BG_FindItem(b"Blue Flag\x00" as *const u8 as *const libc::c_char);
        if item.is_null()
            || 0 == itemRegistered
                [item.wrapping_offset_from(bg_itemlist.as_mut_ptr()) as libc::c_long as usize]
                as u64
        {
            G_Printf(
                b"^3WARNING: No team_CTF_blueflag in map\n\x00" as *const u8 as *const libc::c_char,
            );
        }
    };
}
#[no_mangle]
pub static mut itemRegistered: [qboolean; 256] = [qfalse; 256];
#[no_mangle]
pub unsafe extern "C" fn G_RunItem(mut ent: *mut gentity_t) {
    let mut origin: vec3_t = [0.; 3];
    let mut tr: trace_t = trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_s {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surfaceFlags: 0,
        contents: 0,
        entityNum: 0,
    };
    let mut contents: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    if (*ent).s.groundEntityNum == (1i32 << 10i32) - 1i32 {
        if (*ent).s.pos.trType as libc::c_uint != TR_GRAVITY as libc::c_int as libc::c_uint {
            (*ent).s.pos.trType = TR_GRAVITY;
            (*ent).s.pos.trTime = level.time
        }
    }
    if (*ent).s.pos.trType as libc::c_uint == TR_STATIONARY as libc::c_int as libc::c_uint {
        G_RunThink(ent);
        return;
    }
    BG_EvaluateTrajectory(&mut (*ent).s.pos, level.time, origin.as_mut_ptr());
    if 0 != (*ent).clipmask {
        mask = (*ent).clipmask
    } else {
        mask = (1i32 | 0x10000i32 | 0x2000000i32) & !0x2000000i32
    }
    trap_Trace(
        &mut tr,
        (*ent).r.currentOrigin.as_mut_ptr() as *const vec_t,
        (*ent).r.mins.as_mut_ptr() as *const vec_t,
        (*ent).r.maxs.as_mut_ptr() as *const vec_t,
        origin.as_mut_ptr() as *const vec_t,
        (*ent).r.ownerNum,
        mask,
    );
    (*ent).r.currentOrigin[0usize] = tr.endpos[0usize];
    (*ent).r.currentOrigin[1usize] = tr.endpos[1usize];
    (*ent).r.currentOrigin[2usize] = tr.endpos[2usize];
    if 0 != tr.startsolid as u64 {
        tr.fraction = 0i32 as libc::c_float
    }
    trap_LinkEntity(ent);
    G_RunThink(ent);
    if tr.fraction == 1i32 as libc::c_float {
        return;
    }
    contents = trap_PointContents((*ent).r.currentOrigin.as_mut_ptr() as *const vec_t, -1i32);
    if 0 != contents as libc::c_uint & 0x80000000u32 {
        if !(*ent).item.is_null()
            && (*(*ent).item).giType as libc::c_uint == IT_TEAM as libc::c_int as libc::c_uint
        {
            Team_FreeEntity(ent);
        } else {
            G_FreeEntity(ent);
        }
        return;
    }
    G_BounceItem(ent, &mut tr);
}
/*
================
G_BounceItem

================
*/
#[no_mangle]
pub unsafe extern "C" fn G_BounceItem(mut ent: *mut gentity_t, mut trace: *mut trace_t) {
    let mut velocity: vec3_t = [0.; 3];
    let mut dot: libc::c_float = 0.;
    let mut hitTime: libc::c_int = 0;
    hitTime = (level.previousTime as libc::c_float
        + (level.time - level.previousTime) as libc::c_float * (*trace).fraction)
        as libc::c_int;
    BG_EvaluateTrajectoryDelta(&mut (*ent).s.pos, hitTime, velocity.as_mut_ptr());
    dot = velocity[0usize] * (*trace).plane.normal[0usize]
        + velocity[1usize] * (*trace).plane.normal[1usize]
        + velocity[2usize] * (*trace).plane.normal[2usize];
    (*ent).s.pos.trDelta[0usize] =
        velocity[0usize] + (*trace).plane.normal[0usize] * (-2i32 as libc::c_float * dot);
    (*ent).s.pos.trDelta[1usize] =
        velocity[1usize] + (*trace).plane.normal[1usize] * (-2i32 as libc::c_float * dot);
    (*ent).s.pos.trDelta[2usize] =
        velocity[2usize] + (*trace).plane.normal[2usize] * (-2i32 as libc::c_float * dot);
    (*ent).s.pos.trDelta[0usize] = (*ent).s.pos.trDelta[0usize] * (*ent).physicsBounce;
    (*ent).s.pos.trDelta[1usize] = (*ent).s.pos.trDelta[1usize] * (*ent).physicsBounce;
    (*ent).s.pos.trDelta[2usize] = (*ent).s.pos.trDelta[2usize] * (*ent).physicsBounce;
    if (*trace).plane.normal[2usize] > 0i32 as libc::c_float
        && (*ent).s.pos.trDelta[2usize] < 40i32 as libc::c_float
    {
        (*trace).endpos[2usize] = ((*trace).endpos[2usize] as libc::c_double + 1.0f64) as vec_t;
        (*trace).endpos[0usize] = (*trace).endpos[0usize] as libc::c_int as vec_t;
        (*trace).endpos[1usize] = (*trace).endpos[1usize] as libc::c_int as vec_t;
        (*trace).endpos[2usize] = (*trace).endpos[2usize] as libc::c_int as vec_t;
        G_SetOrigin(ent, (*trace).endpos.as_mut_ptr());
        (*ent).s.groundEntityNum = (*trace).entityNum;
        return;
    }
    (*ent).r.currentOrigin[0usize] = (*ent).r.currentOrigin[0usize] + (*trace).plane.normal[0usize];
    (*ent).r.currentOrigin[1usize] = (*ent).r.currentOrigin[1usize] + (*trace).plane.normal[1usize];
    (*ent).r.currentOrigin[2usize] = (*ent).r.currentOrigin[2usize] + (*trace).plane.normal[2usize];
    (*ent).s.pos.trBase[0usize] = (*ent).r.currentOrigin[0usize];
    (*ent).s.pos.trBase[1usize] = (*ent).r.currentOrigin[1usize];
    (*ent).s.pos.trBase[2usize] = (*ent).r.currentOrigin[2usize];
    (*ent).s.pos.trTime = level.time;
}
#[no_mangle]
pub unsafe extern "C" fn RespawnItem(mut ent: *mut gentity_t) {
    if ent.is_null() {
        return;
    }
    if !(*ent).team.is_null() {
        let mut master: *mut gentity_t = 0 as *mut gentity_t;
        let mut count: libc::c_int = 0;
        let mut choice: libc::c_int = 0;
        if (*ent).teammaster.is_null() {
            G_Error(b"RespawnItem: bad teammaster\x00" as *const u8 as *const libc::c_char);
        }
        master = (*ent).teammaster;
        count = 0i32;
        ent = master;
        while !ent.is_null() {
            ent = (*ent).teamchain;
            count += 1
        }
        choice = rand() % count;
        count = 0i32;
        ent = master;
        while !ent.is_null() && count < choice {
            ent = (*ent).teamchain;
            count += 1
        }
    }
    if ent.is_null() {
        return;
    }
    (*ent).r.contents = 0x40000000i32;
    (*ent).s.eFlags &= !0x80i32;
    (*ent).r.svFlags &= !0x1i32;
    trap_LinkEntity(ent);
    if (*(*ent).item).giType as libc::c_uint == IT_POWERUP as libc::c_int as libc::c_uint {
        let mut te: *mut gentity_t = 0 as *mut gentity_t;
        if 0. != (*ent).speed {
            te = G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                EV_GENERAL_SOUND as libc::c_int,
            )
        } else {
            te = G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                EV_GLOBAL_SOUND as libc::c_int,
            )
        }
        (*te).s.eventParm = G_SoundIndex(
            b"sound/items/poweruprespawn.wav\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*te).r.svFlags |= 0x20i32
    }
    if (*(*ent).item).giType as libc::c_uint == IT_HOLDABLE as libc::c_int as libc::c_uint
        && (*(*ent).item).giTag == HI_KAMIKAZE as libc::c_int
    {
        let mut te_0: *mut gentity_t = 0 as *mut gentity_t;
        if 0. != (*ent).speed {
            te_0 = G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                EV_GENERAL_SOUND as libc::c_int,
            )
        } else {
            te_0 = G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                EV_GLOBAL_SOUND as libc::c_int,
            )
        }
        (*te_0).s.eventParm = G_SoundIndex(
            b"sound/items/kamikazerespawn.wav\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*te_0).r.svFlags |= 0x20i32
    }
    G_AddEvent(ent, EV_ITEM_RESPAWN as libc::c_int, 0i32);
    (*ent).nextthink = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn Drop_Item(
    mut ent: *mut gentity_t,
    mut item: *mut gitem_t,
    mut angle: libc::c_float,
) -> *mut gentity_t {
    let mut velocity: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    angles[0usize] = (*ent).s.apos.trBase[0usize];
    angles[1usize] = (*ent).s.apos.trBase[1usize];
    angles[2usize] = (*ent).s.apos.trBase[2usize];
    angles[1usize] += angle;
    angles[0usize] = 0i32 as vec_t;
    AngleVectors(
        angles.as_mut_ptr() as *const vec_t,
        velocity.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    velocity[0usize] = velocity[0usize] * 150i32 as libc::c_float;
    velocity[1usize] = velocity[1usize] * 150i32 as libc::c_float;
    velocity[2usize] = velocity[2usize] * 150i32 as libc::c_float;
    velocity[2usize] = (velocity[2usize] as libc::c_double
        + (200i32 as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 50i32 as libc::c_double)) as vec_t;
    return LaunchItem(
        item,
        (*ent).s.pos.trBase.as_mut_ptr(),
        velocity.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn LaunchItem(
    mut item: *mut gitem_t,
    mut origin: *mut vec_t,
    mut velocity: *mut vec_t,
) -> *mut gentity_t {
    let mut dropped: *mut gentity_t = 0 as *mut gentity_t;
    dropped = G_Spawn();
    (*dropped).s.eType = ET_ITEM as libc::c_int;
    (*dropped).s.modelindex =
        item.wrapping_offset_from(bg_itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    (*dropped).s.modelindex2 = 1i32;
    (*dropped).classname = (*item).classname;
    (*dropped).item = item;
    (*dropped).r.mins[0usize] = -15i32 as vec_t;
    (*dropped).r.mins[1usize] = -15i32 as vec_t;
    (*dropped).r.mins[2usize] = -15i32 as vec_t;
    (*dropped).r.maxs[0usize] = 15i32 as vec_t;
    (*dropped).r.maxs[1usize] = 15i32 as vec_t;
    (*dropped).r.maxs[2usize] = 15i32 as vec_t;
    (*dropped).r.contents = 0x40000000i32;
    (*dropped).touch = Some(Touch_Item);
    G_SetOrigin(dropped, origin);
    (*dropped).s.pos.trType = TR_GRAVITY;
    (*dropped).s.pos.trTime = level.time;
    (*dropped).s.pos.trDelta[0usize] = *velocity.offset(0isize);
    (*dropped).s.pos.trDelta[1usize] = *velocity.offset(1isize);
    (*dropped).s.pos.trDelta[2usize] = *velocity.offset(2isize);
    (*dropped).s.eFlags |= 0x20i32;
    if g_gametype.integer == GT_CTF as libc::c_int
        && (*item).giType as libc::c_uint == IT_TEAM as libc::c_int as libc::c_uint
    {
        (*dropped).think = Some(Team_DroppedFlagThink);
        (*dropped).nextthink = level.time + 30000i32;
        Team_CheckDroppedItem(dropped);
    } else {
        (*dropped).think = Some(G_FreeEntity);
        (*dropped).nextthink = level.time + 30000i32
    }
    (*dropped).flags = 0x1000i32;
    trap_LinkEntity(dropped);
    return dropped;
}
#[no_mangle]
pub unsafe extern "C" fn Touch_Item(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut trace: *mut trace_t,
) {
    let mut respawn: libc::c_int = 0;
    let mut predict: qboolean = qfalse;
    if (*other).client.is_null() {
        return;
    }
    if (*other).health < 1i32 {
        return;
    }
    if 0 == BG_CanItemBeGrabbed(
        g_gametype.integer,
        &mut (*ent).s,
        &mut (*(*other).client).ps,
    ) as u64
    {
        return;
    }
    G_LogPrintf(
        b"Item: %i %s\n\x00" as *const u8 as *const libc::c_char,
        (*other).s.number,
        (*(*ent).item).classname,
    );
    predict = (*(*other).client).pers.predictItemPickup;
    match (*(*ent).item).giType as libc::c_uint {
        1 => respawn = Pickup_Weapon(ent, other),
        2 => {
            //		predict = qfalse;
            respawn = Pickup_Ammo(ent, other)
        }
        3 => {
            //		predict = qfalse;
            respawn = Pickup_Armor(ent, other)
        }
        4 => respawn = Pickup_Health(ent, other),
        5 => {
            respawn = Pickup_Powerup(ent, other);
            predict = qfalse
        }
        8 => respawn = Pickup_Team(ent, other),
        6 => respawn = Pickup_Holdable(ent, other),
        _ => return,
    }
    if 0 == respawn {
        return;
    }
    if 0 != predict as u64 {
        G_AddPredictableEvent(other, EV_ITEM_PICKUP as libc::c_int, (*ent).s.modelindex);
    } else {
        G_AddEvent(other, EV_ITEM_PICKUP as libc::c_int, (*ent).s.modelindex);
    }
    if (*(*ent).item).giType as libc::c_uint == IT_POWERUP as libc::c_int as libc::c_uint
        || (*(*ent).item).giType as libc::c_uint == IT_TEAM as libc::c_int as libc::c_uint
    {
        if 0. == (*ent).speed {
            let mut te: *mut gentity_t = 0 as *mut gentity_t;
            te = G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                EV_GLOBAL_ITEM_PICKUP as libc::c_int,
            );
            (*te).s.eventParm = (*ent).s.modelindex;
            (*te).r.svFlags |= 0x20i32
        } else {
            let mut te_0: *mut gentity_t = 0 as *mut gentity_t;
            te_0 = G_TempEntity(
                (*ent).s.pos.trBase.as_mut_ptr(),
                EV_GLOBAL_ITEM_PICKUP as libc::c_int,
            );
            (*te_0).s.eventParm = (*ent).s.modelindex;
            (*te_0).r.svFlags |= 0x100i32;
            (*te_0).r.singleClient = (*other).s.number
        }
    }
    G_UseTargets(ent, other);
    if (*ent).wait == -1i32 as libc::c_float {
        (*ent).r.svFlags |= 0x1i32;
        (*ent).s.eFlags |= 0x80i32;
        (*ent).r.contents = 0i32;
        (*ent).unlinkAfterEvent = qtrue;
        return;
    }
    if 0. != (*ent).wait {
        respawn = (*ent).wait as libc::c_int
    }
    if 0. != (*ent).random {
        respawn = (respawn as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * (*ent).random as libc::c_double) as libc::c_int;
        if respawn < 1i32 {
            respawn = 1i32
        }
    }
    if 0 != (*ent).flags & 0x1000i32 {
        (*ent).freeAfterEvent = qtrue
    }
    (*ent).r.svFlags |= 0x1i32;
    (*ent).s.eFlags |= 0x80i32;
    (*ent).r.contents = 0i32;
    if respawn <= 0i32 {
        (*ent).nextthink = 0i32;
        (*ent).think = None
    } else {
        (*ent).nextthink = level.time + respawn * 1000i32;
        (*ent).think = Some(RespawnItem)
    }
    trap_LinkEntity(ent);
}
//======================================================================
#[no_mangle]
pub unsafe extern "C" fn Pickup_Holdable(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
) -> libc::c_int {
    (*(*other).client).ps.stats[STAT_HOLDABLE_ITEM as libc::c_int as usize] =
        (*ent).item.wrapping_offset_from(bg_itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    if (*(*ent).item).giTag == HI_KAMIKAZE as libc::c_int {
        (*(*other).client).ps.eFlags |= 0x200i32
    }
    return 60i32;
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

  Items are any object that a player can touch to gain some effect.

  Pickup will return the number of seconds until they should respawn.

  all items should pop when dropped in lava or slime

  Respawnable items don't actually go away when picked up, they are
  just made invisible and untouchable.  This allows them to ride
  movers and respawn appropriately.
*/
//120
//======================================================================
#[no_mangle]
pub unsafe extern "C" fn Pickup_Powerup(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
) -> libc::c_int {
    let mut quantity: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    if 0 == (*(*other).client).ps.powerups[(*(*ent).item).giTag as usize] {
        (*(*other).client).ps.powerups[(*(*ent).item).giTag as usize] =
            level.time - level.time % 1000i32
    }
    if 0 != (*ent).count {
        quantity = (*ent).count
    } else {
        quantity = (*(*ent).item).quantity
    }
    (*(*other).client).ps.powerups[(*(*ent).item).giTag as usize] += quantity * 1000i32;
    i = 0i32;
    while i < level.maxclients {
        let mut delta: vec3_t = [0.; 3];
        let mut len: libc::c_float = 0.;
        let mut forward: vec3_t = [0.; 3];
        let mut tr: trace_t = trace_t {
            allsolid: qfalse,
            startsolid: qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            plane: cplane_s {
                normal: [0.; 3],
                dist: 0.,
                type_0: 0,
                signbits: 0,
                pad: [0; 2],
            },
            surfaceFlags: 0,
            contents: 0,
            entityNum: 0,
        };
        client = &mut *level.clients.offset(i as isize) as *mut gclient_s;
        if !(client == (*other).client) {
            if !((*client).pers.connected as libc::c_uint
                == CON_DISCONNECTED as libc::c_int as libc::c_uint)
            {
                if !((*client).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32) {
                    // if same team in team game, no sound
                    // cannot use OnSameTeam as it expects to g_entities, not clients
                    if !(g_gametype.integer >= GT_TEAM as libc::c_int
                        && (*(*other).client).sess.sessionTeam as libc::c_uint
                            == (*client).sess.sessionTeam as libc::c_uint)
                    {
                        delta[0usize] = (*ent).s.pos.trBase[0usize] - (*client).ps.origin[0usize];
                        delta[1usize] = (*ent).s.pos.trBase[1usize] - (*client).ps.origin[1usize];
                        delta[2usize] = (*ent).s.pos.trBase[2usize] - (*client).ps.origin[2usize];
                        len = VectorNormalize(delta.as_mut_ptr());
                        if !(len > 192i32 as libc::c_float) {
                            AngleVectors(
                                (*client).ps.viewangles.as_mut_ptr() as *const vec_t,
                                forward.as_mut_ptr(),
                                0 as *mut vec_t,
                                0 as *mut vec_t,
                            );
                            if !(((delta[0usize] * forward[0usize]
                                + delta[1usize] * forward[1usize]
                                + delta[2usize] * forward[2usize])
                                as libc::c_double)
                                < 0.4f64)
                            {
                                trap_Trace(
                                    &mut tr,
                                    (*client).ps.origin.as_mut_ptr() as *const vec_t,
                                    0 as *const vec_t,
                                    0 as *const vec_t,
                                    (*ent).s.pos.trBase.as_mut_ptr() as *const vec_t,
                                    (1i32 << 10i32) - 1i32,
                                    1i32,
                                );
                                if !(tr.fraction as libc::c_double != 1.0f64) {
                                    (*client).ps.persistant
                                        [PERS_PLAYEREVENTS as libc::c_int as usize] ^= 0x1i32
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    return 120i32;
}
//======================================================================
#[no_mangle]
pub unsafe extern "C" fn Pickup_Health(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
) -> libc::c_int {
    let mut max: libc::c_int = 0;
    let mut quantity: libc::c_int = 0;
    if (*(*ent).item).quantity != 5i32 && (*(*ent).item).quantity != 100i32 {
        max = (*(*other).client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize]
    } else {
        max = (*(*other).client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] * 2i32
    }
    if 0 != (*ent).count {
        quantity = (*ent).count
    } else {
        quantity = (*(*ent).item).quantity
    }
    (*other).health += quantity;
    if (*other).health > max {
        (*other).health = max
    }
    (*(*other).client).ps.stats[STAT_HEALTH as libc::c_int as usize] = (*other).health;
    if (*(*ent).item).quantity == 100i32 {
        return 35i32;
    }
    return 35i32;
}
//======================================================================
#[no_mangle]
pub unsafe extern "C" fn Pickup_Armor(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
) -> libc::c_int {
    (*(*other).client).ps.stats[STAT_ARMOR as libc::c_int as usize] += (*(*ent).item).quantity;
    if (*(*other).client).ps.stats[STAT_ARMOR as libc::c_int as usize]
        > (*(*other).client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] * 2i32
    {
        (*(*other).client).ps.stats[STAT_ARMOR as libc::c_int as usize] =
            (*(*other).client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] * 2i32
    }
    return 25i32;
}
#[no_mangle]
pub unsafe extern "C" fn Pickup_Ammo(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
) -> libc::c_int {
    let mut quantity: libc::c_int = 0;
    if 0 != (*ent).count {
        quantity = (*ent).count
    } else {
        quantity = (*(*ent).item).quantity
    }
    Add_Ammo(other, (*(*ent).item).giTag, quantity);
    return 40i32;
}
#[no_mangle]
pub unsafe extern "C" fn Add_Ammo(
    mut ent: *mut gentity_t,
    mut weapon: libc::c_int,
    mut count: libc::c_int,
) {
    (*(*ent).client).ps.ammo[weapon as usize] += count;
    if (*(*ent).client).ps.ammo[weapon as usize] > 200i32 {
        (*(*ent).client).ps.ammo[weapon as usize] = 200i32
    };
}
//======================================================================
#[no_mangle]
pub unsafe extern "C" fn Pickup_Weapon(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
) -> libc::c_int {
    let mut quantity: libc::c_int = 0;
    if (*ent).count < 0i32 {
        quantity = 0i32
    } else {
        if 0 != (*ent).count {
            quantity = (*ent).count
        } else {
            quantity = (*(*ent).item).quantity
        }
        if 0 == (*ent).flags & 0x1000i32 && g_gametype.integer != GT_TEAM as libc::c_int {
            if (*(*other).client).ps.ammo[(*(*ent).item).giTag as usize] < quantity {
                quantity = quantity - (*(*other).client).ps.ammo[(*(*ent).item).giTag as usize]
            } else {
                quantity = 1i32
            }
        }
    }
    (*(*other).client).ps.stats[STAT_WEAPONS as libc::c_int as usize] |=
        1i32 << (*(*ent).item).giTag;
    Add_Ammo(other, (*(*ent).item).giTag, quantity);
    if (*(*ent).item).giTag == WP_GRAPPLING_HOOK as libc::c_int {
        (*(*other).client).ps.ammo[(*(*ent).item).giTag as usize] = -1i32
    }
    if g_gametype.integer == GT_TEAM as libc::c_int {
        return g_weaponTeamRespawn.integer;
    }
    return g_weaponRespawn.integer;
}
#[no_mangle]
pub unsafe extern "C" fn G_SpawnItem(mut ent: *mut gentity_t, mut item: *mut gitem_t) {
    G_SpawnFloat(
        b"random\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).random,
    );
    G_SpawnFloat(
        b"wait\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).wait,
    );
    RegisterItem(item);
    if 0 != G_ItemDisabled(item) {
        return;
    }
    (*ent).item = item;
    (*ent).nextthink = level.time + 100i32 * 2i32;
    (*ent).think = Some(FinishSpawningItem);
    (*ent).physicsBounce = 0.50f64 as libc::c_float;
    if (*item).giType as libc::c_uint == IT_POWERUP as libc::c_int as libc::c_uint {
        G_SoundIndex(
            b"sound/items/poweruprespawn.wav\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        G_SpawnFloat(
            b"noglobalsound\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            &mut (*ent).speed,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn FinishSpawningItem(mut ent: *mut gentity_t) {
    let mut tr: trace_t = trace_t {
        allsolid: qfalse,
        startsolid: qfalse,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_s {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surfaceFlags: 0,
        contents: 0,
        entityNum: 0,
    };
    let mut dest: vec3_t = [0.; 3];
    (*ent).r.mins[0usize] = -15i32 as vec_t;
    (*ent).r.mins[1usize] = -15i32 as vec_t;
    (*ent).r.mins[2usize] = -15i32 as vec_t;
    (*ent).r.maxs[0usize] = 15i32 as vec_t;
    (*ent).r.maxs[1usize] = 15i32 as vec_t;
    (*ent).r.maxs[2usize] = 15i32 as vec_t;
    (*ent).s.eType = ET_ITEM as libc::c_int;
    (*ent).s.modelindex =
        (*ent).item.wrapping_offset_from(bg_itemlist.as_mut_ptr()) as libc::c_long as libc::c_int;
    (*ent).s.modelindex2 = 0i32;
    (*ent).r.contents = 0x40000000i32;
    (*ent).touch = Some(Touch_Item);
    (*ent).use_0 = Some(Use_Item);
    if 0 != (*ent).spawnflags & 1i32 {
        G_SetOrigin(ent, (*ent).s.origin.as_mut_ptr());
    } else {
        dest[0usize] = (*ent).s.origin[0usize];
        dest[1usize] = (*ent).s.origin[1usize];
        dest[2usize] = (*ent).s.origin[2usize] - 4096i32 as libc::c_float;
        trap_Trace(
            &mut tr,
            (*ent).s.origin.as_mut_ptr() as *const vec_t,
            (*ent).r.mins.as_mut_ptr() as *const vec_t,
            (*ent).r.maxs.as_mut_ptr() as *const vec_t,
            dest.as_mut_ptr() as *const vec_t,
            (*ent).s.number,
            1i32,
        );
        if 0 != tr.startsolid as u64 {
            G_Printf(
                b"FinishSpawningItem: %s startsolid at %s\n\x00" as *const u8
                    as *const libc::c_char,
                (*ent).classname,
                vtos((*ent).s.origin.as_mut_ptr() as *const vec_t),
            );
            G_FreeEntity(ent);
            return;
        }
        (*ent).s.groundEntityNum = tr.entityNum;
        G_SetOrigin(ent, tr.endpos.as_mut_ptr());
    }
    if 0 != (*ent).flags & 0x400i32 || !(*ent).targetname.is_null() {
        (*ent).s.eFlags |= 0x80i32;
        (*ent).r.contents = 0i32;
        return;
    }
    if (*(*ent).item).giType as libc::c_uint == IT_POWERUP as libc::c_int as libc::c_uint {
        let mut respawn: libc::c_float = 0.;
        respawn = (45i32 as libc::c_double
            + 2.0f64
                * (((rand() & 0x7fffi32) as libc::c_float / 0x7fffi32 as libc::c_float)
                    as libc::c_double
                    - 0.5f64)
                * 15i32 as libc::c_double) as libc::c_float;
        (*ent).s.eFlags |= 0x80i32;
        (*ent).r.contents = 0i32;
        (*ent).nextthink =
            (level.time as libc::c_float + respawn * 1000i32 as libc::c_float) as libc::c_int;
        (*ent).think = Some(RespawnItem);
        return;
    }
    trap_LinkEntity(ent);
}
/*
================
Use_Item

Respawn the item
================
*/
#[no_mangle]
pub unsafe extern "C" fn Use_Item(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    RespawnItem(ent);
}
/*
============
G_ItemDisabled
============
*/
#[no_mangle]
pub unsafe extern "C" fn G_ItemDisabled(mut item: *mut gitem_t) -> libc::c_int {
    let mut name: [libc::c_char; 128] = [0; 128];
    Com_sprintf(
        name.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"disable_%s\x00" as *const u8 as *const libc::c_char,
        (*item).classname,
    );
    return trap_Cvar_VariableIntegerValue(name.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn RegisterItem(mut item: *mut gitem_t) {
    if item.is_null() {
        G_Error(b"RegisterItem: NULL\x00" as *const u8 as *const libc::c_char);
    }
    itemRegistered[item.wrapping_offset_from(bg_itemlist.as_mut_ptr()) as libc::c_long as usize] =
        qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn ClearRegisteredItems() {
    memset(
        itemRegistered.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[qboolean; 256]>() as libc::c_ulong,
    );
    RegisterItem(BG_FindItemForWeapon(WP_MACHINEGUN));
    RegisterItem(BG_FindItemForWeapon(WP_GAUNTLET));
}
#[no_mangle]
pub unsafe extern "C" fn SaveRegisteredItems() {
    let mut string: [libc::c_char; 257] = [0; 257];
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    count = 0i32;
    i = 0i32;
    while i < bg_numItems {
        if 0 != itemRegistered[i as usize] as u64 {
            count += 1;
            string[i as usize] = '1' as i32 as libc::c_char
        } else {
            string[i as usize] = '0' as i32 as libc::c_char
        }
        i += 1
    }
    string[bg_numItems as usize] = 0i32 as libc::c_char;
    G_Printf(
        b"%i items registered\n\x00" as *const u8 as *const libc::c_char,
        count,
    );
    trap_SetConfigstring(27i32, string.as_mut_ptr());
}
