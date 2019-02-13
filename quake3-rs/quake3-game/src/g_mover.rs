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
    gitem_s, gitem_t, itemType_t, team_t, unnamed, unnamed_0, unnamed_1, unnamed_2, ET_BEAM,
    ET_EVENTS, ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER,
    ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, EV_BULLET,
    EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL, EV_CHANGE_WEAPON, EV_DEATH1, EV_DEATH2, EV_DEATH3,
    EV_DEBUG_LINE, EV_FALL_FAR, EV_FALL_MEDIUM, EV_FALL_SHORT, EV_FIRE_WEAPON, EV_FOOTSPLASH,
    EV_FOOTSTEP, EV_FOOTSTEP_METAL, EV_FOOTWADE, EV_GENERAL_SOUND, EV_GIB_PLAYER,
    EV_GLOBAL_ITEM_PICKUP, EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND, EV_GRENADE_BOUNCE,
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
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, IT_AMMO, IT_ARMOR, IT_BAD,
    IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, MOD_BFG,
    MOD_BFG_SPLASH, MOD_CRUSH, MOD_FALLING, MOD_GAUNTLET, MOD_GRAPPLE, MOD_GRENADE,
    MOD_GRENADE_SPLASH, MOD_LAVA, MOD_LIGHTNING, MOD_MACHINEGUN, MOD_PLASMA, MOD_PLASMA_SPLASH,
    MOD_RAILGUN, MOD_ROCKET, MOD_ROCKET_SPLASH, MOD_SHOTGUN, MOD_SLIME, MOD_SUICIDE,
    MOD_TARGET_LASER, MOD_TELEFRAG, MOD_TRIGGER_HURT, MOD_UNKNOWN, MOD_WATER, STAT_ARMOR,
    STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH,
    STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
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
    trap_AdjustAreaPortalState, trap_EntitiesInBox, trap_LinkEntity, trap_SetBrushModel,
    trap_Trace, trap_UnlinkEntity, CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED, MOVER_1TO2,
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
    SP_team_CTF_redspawn, Team_CheckDroppedItem, Team_DroppedFlagThink,
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
use g_variadic_h::{G_Error, G_Printf};
use g_weapon::{
    CheckGauntletAttack, FireWeapon, LogAccuracyHit, SnapVectorTowards, Weapon_HookFree,
    Weapon_HookThink,
};
use libc;
use q_math::{
    vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
    DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize, VectorNormalize2,
};
use q_shared_h::{
    byte, cplane_s, cplane_t, cvarHandle_t, entityState_s, entityState_t, fileHandle_t,
    playerState_s, playerState_t, qboolean, qfalse, qtrue, trType_t, trace_t, trajectory_t,
    usercmd_s, usercmd_t, vec3_t, vec_t, vmCvar_t, TR_GRAVITY, TR_INTERPOLATE, TR_LINEAR,
    TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stddef_h::size_t;
use stdlib::{fabs, sqrt, strcmp};

unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
}
unsafe extern "C" fn VectorInverse(mut v: *mut vec_t) {
    *v.offset(0isize) = -*v.offset(0isize);
    *v.offset(1isize) = -*v.offset(1isize);
    *v.offset(2isize) = -*v.offset(2isize);
}
//
// g_mover.c
//
#[no_mangle]
pub unsafe extern "C" fn G_RunMover(mut ent: *mut gentity_t) {
    if 0 != (*ent).flags & 0x400i32 {
        return;
    }
    if (*ent).s.pos.trType as libc::c_uint != TR_STATIONARY as libc::c_int as libc::c_uint
        || (*ent).s.apos.trType as libc::c_uint != TR_STATIONARY as libc::c_int as libc::c_uint
    {
        G_MoverTeam(ent);
    }
    G_RunThink(ent);
}
/*
=================
G_MoverTeam
=================
*/
#[no_mangle]
pub unsafe extern "C" fn G_MoverTeam(mut ent: *mut gentity_t) {
    let mut move_0: vec3_t = [0.; 3];
    let mut amove: vec3_t = [0.; 3];
    let mut part: *mut gentity_t = 0 as *mut gentity_t;
    let mut obstacle: *mut gentity_t = 0 as *mut gentity_t;
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    obstacle = 0 as *mut gentity_t;
    pushed_p = pushed.as_mut_ptr();
    part = ent;
    while !part.is_null() {
        BG_EvaluateTrajectory(&mut (*part).s.pos, level.time, origin.as_mut_ptr());
        BG_EvaluateTrajectory(&mut (*part).s.apos, level.time, angles.as_mut_ptr());
        move_0[0usize] = origin[0usize] - (*part).r.currentOrigin[0usize];
        move_0[1usize] = origin[1usize] - (*part).r.currentOrigin[1usize];
        move_0[2usize] = origin[2usize] - (*part).r.currentOrigin[2usize];
        amove[0usize] = angles[0usize] - (*part).r.currentAngles[0usize];
        amove[1usize] = angles[1usize] - (*part).r.currentAngles[1usize];
        amove[2usize] = angles[2usize] - (*part).r.currentAngles[2usize];
        if 0 == G_MoverPush(part, move_0.as_mut_ptr(), amove.as_mut_ptr(), &mut obstacle) as u64 {
            // move was blocked
            break;
        } else {
            part = (*part).teamchain
        }
    }
    if !part.is_null() {
        part = ent;
        while !part.is_null() {
            (*part).s.pos.trTime += level.time - level.previousTime;
            (*part).s.apos.trTime += level.time - level.previousTime;
            BG_EvaluateTrajectory(
                &mut (*part).s.pos,
                level.time,
                (*part).r.currentOrigin.as_mut_ptr(),
            );
            BG_EvaluateTrajectory(
                &mut (*part).s.apos,
                level.time,
                (*part).r.currentAngles.as_mut_ptr(),
            );
            trap_LinkEntity(part);
            part = (*part).teamchain
        }
        if (*ent).blocked.is_some() {
            (*ent).blocked.expect("non-null function pointer")(ent, obstacle);
        }
        return;
    }
    part = ent;
    while !part.is_null() {
        if (*part).s.pos.trType as libc::c_uint == TR_LINEAR_STOP as libc::c_int as libc::c_uint {
            if level.time >= (*part).s.pos.trTime + (*part).s.pos.trDuration {
                if (*part).reached.is_some() {
                    (*part).reached.expect("non-null function pointer")(part);
                }
            }
        }
        part = (*part).teamchain
    }
}
/*
============
G_MoverPush

Objects need to be moved back on a failed push,
otherwise riders would continue to slide.
If qfalse is returned, *obstacle will be the blocking entity
============
*/
#[no_mangle]
pub unsafe extern "C" fn G_MoverPush(
    mut pusher: *mut gentity_t,
    mut move_0: *mut vec_t,
    mut amove: *mut vec_t,
    mut obstacle: *mut *mut gentity_t,
) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut check: *mut gentity_t = 0 as *mut gentity_t;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut p: *mut pushed_t = 0 as *mut pushed_t;
    let mut entityList: [libc::c_int; 1024] = [0; 1024];
    let mut listedEntities: libc::c_int = 0;
    let mut totalMins: vec3_t = [0.; 3];
    let mut totalMaxs: vec3_t = [0.; 3];
    *obstacle = 0 as *mut gentity_t;
    if 0. != (*pusher).r.currentAngles[0usize]
        || 0. != (*pusher).r.currentAngles[1usize]
        || 0. != (*pusher).r.currentAngles[2usize]
        || 0. != *amove.offset(0isize)
        || 0. != *amove.offset(1isize)
        || 0. != *amove.offset(2isize)
    {
        let mut radius: libc::c_float = 0.;
        radius = RadiusFromBounds(
            (*pusher).r.mins.as_mut_ptr() as *const vec_t,
            (*pusher).r.maxs.as_mut_ptr() as *const vec_t,
        );
        i = 0i32;
        while i < 3i32 {
            mins[i as usize] =
                (*pusher).r.currentOrigin[i as usize] + *move_0.offset(i as isize) - radius;
            maxs[i as usize] =
                (*pusher).r.currentOrigin[i as usize] + *move_0.offset(i as isize) + radius;
            totalMins[i as usize] = mins[i as usize] - *move_0.offset(i as isize);
            totalMaxs[i as usize] = maxs[i as usize] - *move_0.offset(i as isize);
            i += 1
        }
    } else {
        i = 0i32;
        while i < 3i32 {
            mins[i as usize] = (*pusher).r.absmin[i as usize] + *move_0.offset(i as isize);
            maxs[i as usize] = (*pusher).r.absmax[i as usize] + *move_0.offset(i as isize);
            i += 1
        }
        totalMins[0usize] = (*pusher).r.absmin[0usize];
        totalMins[1usize] = (*pusher).r.absmin[1usize];
        totalMins[2usize] = (*pusher).r.absmin[2usize];
        totalMaxs[0usize] = (*pusher).r.absmax[0usize];
        totalMaxs[1usize] = (*pusher).r.absmax[1usize];
        totalMaxs[2usize] = (*pusher).r.absmax[2usize];
        i = 0i32;
        while i < 3i32 {
            if *move_0.offset(i as isize) > 0i32 as libc::c_float {
                totalMaxs[i as usize] += *move_0.offset(i as isize)
            } else {
                totalMins[i as usize] += *move_0.offset(i as isize)
            }
            i += 1
        }
    }
    trap_UnlinkEntity(pusher);
    listedEntities = trap_EntitiesInBox(
        totalMins.as_mut_ptr() as *const vec_t,
        totalMaxs.as_mut_ptr() as *const vec_t,
        entityList.as_mut_ptr(),
        1i32 << 10i32,
    );
    (*pusher).r.currentOrigin[0usize] = (*pusher).r.currentOrigin[0usize] + *move_0.offset(0isize);
    (*pusher).r.currentOrigin[1usize] = (*pusher).r.currentOrigin[1usize] + *move_0.offset(1isize);
    (*pusher).r.currentOrigin[2usize] = (*pusher).r.currentOrigin[2usize] + *move_0.offset(2isize);
    (*pusher).r.currentAngles[0usize] = (*pusher).r.currentAngles[0usize] + *amove.offset(0isize);
    (*pusher).r.currentAngles[1usize] = (*pusher).r.currentAngles[1usize] + *amove.offset(1isize);
    (*pusher).r.currentAngles[2usize] = (*pusher).r.currentAngles[2usize] + *amove.offset(2isize);
    trap_LinkEntity(pusher);
    let mut current_block_46: u64;
    e = 0i32;
    while e < listedEntities {
        check = &mut g_entities[entityList[e as usize] as usize] as *mut gentity_t;
        // only push items and players
        if !((*check).s.eType != ET_ITEM as libc::c_int
            && (*check).s.eType != ET_PLAYER as libc::c_int
            && 0 == (*check).physicsObject as u64)
        {
            // if the entity is standing on the pusher, it will definitely be moved
            if (*check).s.groundEntityNum != (*pusher).s.number {
                // see if the ent needs to be tested
                if (*check).r.absmin[0usize] >= maxs[0usize]
                    || (*check).r.absmin[1usize] >= maxs[1usize]
                    || (*check).r.absmin[2usize] >= maxs[2usize]
                    || (*check).r.absmax[0usize] <= mins[0usize]
                    || (*check).r.absmax[1usize] <= mins[1usize]
                    || (*check).r.absmax[2usize] <= mins[2usize]
                {
                    current_block_46 = 9520865839495247062;
                } else if G_TestEntityPosition(check).is_null() {
                    current_block_46 = 9520865839495247062;
                } else {
                    current_block_46 = 790185930182612747;
                }
            } else {
                current_block_46 = 790185930182612747;
            }
            match current_block_46 {
                9520865839495247062 => {}
                _ => {
                    // the entity needs to be pushed
                    if !(0 != G_TryPushingEntity(check, pusher, move_0, amove) as u64) {
                        // the move was blocked an entity
                        // bobbing entities are instant-kill and never get blocked
                        if (*pusher).s.pos.trType as libc::c_uint
                            == TR_SINE as libc::c_int as libc::c_uint
                            || (*pusher).s.apos.trType as libc::c_uint
                                == TR_SINE as libc::c_int as libc::c_uint
                        {
                            G_Damage(
                                check,
                                pusher,
                                pusher,
                                0 as *mut vec_t,
                                0 as *mut vec_t,
                                99999i32,
                                0i32,
                                MOD_CRUSH as libc::c_int,
                            );
                        } else {
                            *obstacle = check;
                            p = pushed_p.offset(-1isize);
                            while p >= pushed.as_mut_ptr() {
                                (*(*p).ent).s.pos.trBase[0usize] = (*p).origin[0usize];
                                (*(*p).ent).s.pos.trBase[1usize] = (*p).origin[1usize];
                                (*(*p).ent).s.pos.trBase[2usize] = (*p).origin[2usize];
                                (*(*p).ent).s.apos.trBase[0usize] = (*p).angles[0usize];
                                (*(*p).ent).s.apos.trBase[1usize] = (*p).angles[1usize];
                                (*(*p).ent).s.apos.trBase[2usize] = (*p).angles[2usize];
                                if !(*(*p).ent).client.is_null() {
                                    (*(*(*p).ent).client).ps.delta_angles[1usize] =
                                        (*p).deltayaw as libc::c_int;
                                    (*(*(*p).ent).client).ps.origin[0usize] = (*p).origin[0usize];
                                    (*(*(*p).ent).client).ps.origin[1usize] = (*p).origin[1usize];
                                    (*(*(*p).ent).client).ps.origin[2usize] = (*p).origin[2usize]
                                }
                                trap_LinkEntity((*p).ent);
                                p = p.offset(-1isize)
                            }
                            return qfalse;
                        }
                    }
                }
            }
        }
        e += 1
    }
    return qtrue;
}
#[no_mangle]
pub static mut pushed: [pushed_t; 1024] = [pushed_t {
    ent: 0 as *const gentity_t as *mut gentity_t,
    origin: [0.; 3],
    angles: [0.; 3],
    deltayaw: 0.,
}; 1024];
#[no_mangle]
pub static mut pushed_p: *mut pushed_t = 0 as *const pushed_t as *mut pushed_t;
/*
==================
G_TryPushingEntity

Returns qfalse if the move is blocked
==================
*/
#[no_mangle]
pub unsafe extern "C" fn G_TryPushingEntity(
    mut check: *mut gentity_t,
    mut pusher: *mut gentity_t,
    mut move_0: *mut vec_t,
    mut amove: *mut vec_t,
) -> qboolean {
    let mut matrix: [vec3_t; 3] = [[0.; 3]; 3];
    let mut transpose: [vec3_t; 3] = [[0.; 3]; 3];
    let mut org: vec3_t = [0.; 3];
    let mut org2: vec3_t = [0.; 3];
    let mut move2: vec3_t = [0.; 3];
    let mut block: *mut gentity_t = 0 as *mut gentity_t;
    if 0 != (*pusher).s.eFlags & 0x400i32 && (*check).s.groundEntityNum != (*pusher).s.number {
        return qfalse;
    }
    if pushed_p > &mut pushed[((1i32 << 10i32) as usize) - 1] as *mut pushed_t {
        G_Error(b"pushed_p > &pushed[MAX_GENTITIES]\x00" as *const u8 as *const libc::c_char);
    }
    (*pushed_p).ent = check;
    (*pushed_p).origin[0usize] = (*check).s.pos.trBase[0usize];
    (*pushed_p).origin[1usize] = (*check).s.pos.trBase[1usize];
    (*pushed_p).origin[2usize] = (*check).s.pos.trBase[2usize];
    (*pushed_p).angles[0usize] = (*check).s.apos.trBase[0usize];
    (*pushed_p).angles[1usize] = (*check).s.apos.trBase[1usize];
    (*pushed_p).angles[2usize] = (*check).s.apos.trBase[2usize];
    if !(*check).client.is_null() {
        (*pushed_p).deltayaw = (*(*check).client).ps.delta_angles[1usize] as libc::c_float;
        (*pushed_p).origin[0usize] = (*(*check).client).ps.origin[0usize];
        (*pushed_p).origin[1usize] = (*(*check).client).ps.origin[1usize];
        (*pushed_p).origin[2usize] = (*(*check).client).ps.origin[2usize]
    }
    pushed_p = pushed_p.offset(1isize);
    G_CreateRotationMatrix(amove, transpose.as_mut_ptr());
    G_TransposeMatrix(transpose.as_mut_ptr(), matrix.as_mut_ptr());
    if !(*check).client.is_null() {
        org[0usize] = (*(*check).client).ps.origin[0usize] - (*pusher).r.currentOrigin[0usize];
        org[1usize] = (*(*check).client).ps.origin[1usize] - (*pusher).r.currentOrigin[1usize];
        org[2usize] = (*(*check).client).ps.origin[2usize] - (*pusher).r.currentOrigin[2usize]
    } else {
        org[0usize] = (*check).s.pos.trBase[0usize] - (*pusher).r.currentOrigin[0usize];
        org[1usize] = (*check).s.pos.trBase[1usize] - (*pusher).r.currentOrigin[1usize];
        org[2usize] = (*check).s.pos.trBase[2usize] - (*pusher).r.currentOrigin[2usize]
    }
    org2[0usize] = org[0usize];
    org2[1usize] = org[1usize];
    org2[2usize] = org[2usize];
    G_RotatePoint(org2.as_mut_ptr(), matrix.as_mut_ptr());
    move2[0usize] = org2[0usize] - org[0usize];
    move2[1usize] = org2[1usize] - org[1usize];
    move2[2usize] = org2[2usize] - org[2usize];
    (*check).s.pos.trBase[0usize] = (*check).s.pos.trBase[0usize] + *move_0.offset(0isize);
    (*check).s.pos.trBase[1usize] = (*check).s.pos.trBase[1usize] + *move_0.offset(1isize);
    (*check).s.pos.trBase[2usize] = (*check).s.pos.trBase[2usize] + *move_0.offset(2isize);
    (*check).s.pos.trBase[0usize] = (*check).s.pos.trBase[0usize] + move2[0usize];
    (*check).s.pos.trBase[1usize] = (*check).s.pos.trBase[1usize] + move2[1usize];
    (*check).s.pos.trBase[2usize] = (*check).s.pos.trBase[2usize] + move2[2usize];
    if !(*check).client.is_null() {
        (*(*check).client).ps.origin[0usize] =
            (*(*check).client).ps.origin[0usize] + *move_0.offset(0isize);
        (*(*check).client).ps.origin[1usize] =
            (*(*check).client).ps.origin[1usize] + *move_0.offset(1isize);
        (*(*check).client).ps.origin[2usize] =
            (*(*check).client).ps.origin[2usize] + *move_0.offset(2isize);
        (*(*check).client).ps.origin[0usize] = (*(*check).client).ps.origin[0usize] + move2[0usize];
        (*(*check).client).ps.origin[1usize] = (*(*check).client).ps.origin[1usize] + move2[1usize];
        (*(*check).client).ps.origin[2usize] = (*(*check).client).ps.origin[2usize] + move2[2usize];
        (*(*check).client).ps.delta_angles[1usize] +=
            (*amove.offset(1isize) * 65536i32 as libc::c_float / 360i32 as libc::c_float)
                as libc::c_int
                & 65535i32
    }
    if (*check).s.groundEntityNum != (*pusher).s.number {
        (*check).s.groundEntityNum = (1i32 << 10i32) - 1i32
    }
    block = G_TestEntityPosition(check);
    if block.is_null() {
        if !(*check).client.is_null() {
            (*check).r.currentOrigin[0usize] = (*(*check).client).ps.origin[0usize];
            (*check).r.currentOrigin[1usize] = (*(*check).client).ps.origin[1usize];
            (*check).r.currentOrigin[2usize] = (*(*check).client).ps.origin[2usize]
        } else {
            (*check).r.currentOrigin[0usize] = (*check).s.pos.trBase[0usize];
            (*check).r.currentOrigin[1usize] = (*check).s.pos.trBase[1usize];
            (*check).r.currentOrigin[2usize] = (*check).s.pos.trBase[2usize]
        }
        trap_LinkEntity(check);
        return qtrue;
    }
    (*check).s.pos.trBase[0usize] = (*pushed_p.offset(-1isize)).origin[0usize];
    (*check).s.pos.trBase[1usize] = (*pushed_p.offset(-1isize)).origin[1usize];
    (*check).s.pos.trBase[2usize] = (*pushed_p.offset(-1isize)).origin[2usize];
    if !(*check).client.is_null() {
        (*(*check).client).ps.origin[0usize] = (*pushed_p.offset(-1isize)).origin[0usize];
        (*(*check).client).ps.origin[1usize] = (*pushed_p.offset(-1isize)).origin[1usize];
        (*(*check).client).ps.origin[2usize] = (*pushed_p.offset(-1isize)).origin[2usize]
    }
    (*check).s.apos.trBase[0usize] = (*pushed_p.offset(-1isize)).angles[0usize];
    (*check).s.apos.trBase[1usize] = (*pushed_p.offset(-1isize)).angles[1usize];
    (*check).s.apos.trBase[2usize] = (*pushed_p.offset(-1isize)).angles[2usize];
    block = G_TestEntityPosition(check);
    if block.is_null() {
        (*check).s.groundEntityNum = (1i32 << 10i32) - 1i32;
        pushed_p = pushed_p.offset(-1isize);
        return qtrue;
    }
    return qfalse;
}
/*
============
G_TestEntityPosition

============
*/
#[no_mangle]
pub unsafe extern "C" fn G_TestEntityPosition(mut ent: *mut gentity_t) -> *mut gentity_t {
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
    let mut mask: libc::c_int = 0;
    if 0 != (*ent).clipmask {
        mask = (*ent).clipmask
    } else {
        mask = 1i32
    }
    if !(*ent).client.is_null() {
        trap_Trace(
            &mut tr,
            (*(*ent).client).ps.origin.as_mut_ptr() as *const vec_t,
            (*ent).r.mins.as_mut_ptr() as *const vec_t,
            (*ent).r.maxs.as_mut_ptr() as *const vec_t,
            (*(*ent).client).ps.origin.as_mut_ptr() as *const vec_t,
            (*ent).s.number,
            mask,
        );
    } else {
        trap_Trace(
            &mut tr,
            (*ent).s.pos.trBase.as_mut_ptr() as *const vec_t,
            (*ent).r.mins.as_mut_ptr() as *const vec_t,
            (*ent).r.maxs.as_mut_ptr() as *const vec_t,
            (*ent).s.pos.trBase.as_mut_ptr() as *const vec_t,
            (*ent).s.number,
            mask,
        );
    }
    if 0 != tr.startsolid as u64 {
        return &mut g_entities[tr.entityNum as usize] as *mut gentity_t;
    }
    return 0 as *mut gentity_t;
}
/*
================
G_RotatePoint
================
*/
#[no_mangle]
pub unsafe extern "C" fn G_RotatePoint(mut point: *mut vec_t, mut matrix: *mut vec3_t) {
    let mut tvec: vec3_t = [0.; 3];
    tvec[0usize] = *point.offset(0isize);
    tvec[1usize] = *point.offset(1isize);
    tvec[2usize] = *point.offset(2isize);
    *point.offset(0isize) = (*matrix.offset(0isize))[0usize] * tvec[0usize]
        + (*matrix.offset(0isize))[1usize] * tvec[1usize]
        + (*matrix.offset(0isize))[2usize] * tvec[2usize];
    *point.offset(1isize) = (*matrix.offset(1isize))[0usize] * tvec[0usize]
        + (*matrix.offset(1isize))[1usize] * tvec[1usize]
        + (*matrix.offset(1isize))[2usize] * tvec[2usize];
    *point.offset(2isize) = (*matrix.offset(2isize))[0usize] * tvec[0usize]
        + (*matrix.offset(2isize))[1usize] * tvec[1usize]
        + (*matrix.offset(2isize))[2usize] * tvec[2usize];
}
/*
================
G_TransposeMatrix
================
*/
#[no_mangle]
pub unsafe extern "C" fn G_TransposeMatrix(mut matrix: *mut vec3_t, mut transpose: *mut vec3_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    i = 0i32;
    while i < 3i32 {
        j = 0i32;
        while j < 3i32 {
            (*transpose.offset(i as isize))[j as usize] = (*matrix.offset(j as isize))[i as usize];
            j += 1
        }
        i += 1
    }
}
/*
================
G_CreateRotationMatrix
================
*/
#[no_mangle]
pub unsafe extern "C" fn G_CreateRotationMatrix(mut angles: *mut vec_t, mut matrix: *mut vec3_t) {
    AngleVectors(
        angles as *const vec_t,
        (*matrix.offset(0isize)).as_mut_ptr(),
        (*matrix.offset(1isize)).as_mut_ptr(),
        (*matrix.offset(2isize)).as_mut_ptr(),
    );
    VectorInverse((*matrix.offset(1isize)).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Touch_DoorTrigger(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut trace: *mut trace_t,
) {
    if !(*other).client.is_null()
        && (*(*other).client).sess.sessionTeam as libc::c_uint
            == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        if (*(*ent).parent).moverState as libc::c_uint != MOVER_1TO2 as libc::c_int as libc::c_uint
            && (*(*ent).parent).moverState as libc::c_uint
                != MOVER_POS2 as libc::c_int as libc::c_uint
        {
            Touch_DoorTriggerSpectator(ent, other, trace);
        }
    } else if (*(*ent).parent).moverState as libc::c_uint
        != MOVER_1TO2 as libc::c_int as libc::c_uint
    {
        Use_BinaryMover((*ent).parent, ent, other);
    };
}
/*
================
Use_BinaryMover
================
*/
#[no_mangle]
pub unsafe extern "C" fn Use_BinaryMover(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut activator: *mut gentity_t,
) {
    let mut total: libc::c_int = 0;
    let mut partial: libc::c_int = 0;
    if 0 != (*ent).flags & 0x400i32 {
        Use_BinaryMover((*ent).teammaster, other, activator);
        return;
    }
    (*ent).activator = activator;
    if (*ent).moverState as libc::c_uint == MOVER_POS1 as libc::c_int as libc::c_uint {
        MatchTeam(ent, MOVER_1TO2 as libc::c_int, level.time + 50i32);
        if 0 != (*ent).sound1to2 {
            G_AddEvent(ent, EV_GENERAL_SOUND as libc::c_int, (*ent).sound1to2);
        }
        (*ent).s.loopSound = (*ent).soundLoop;
        if (*ent).teammaster == ent || (*ent).teammaster.is_null() {
            trap_AdjustAreaPortalState(ent, qtrue);
        }
        return;
    }
    if (*ent).moverState as libc::c_uint == MOVER_POS2 as libc::c_int as libc::c_uint {
        (*ent).nextthink = (level.time as libc::c_float + (*ent).wait) as libc::c_int;
        return;
    }
    if (*ent).moverState as libc::c_uint == MOVER_2TO1 as libc::c_int as libc::c_uint {
        total = (*ent).s.pos.trDuration;
        partial = level.time - (*ent).s.pos.trTime;
        if partial > total {
            partial = total
        }
        MatchTeam(
            ent,
            MOVER_1TO2 as libc::c_int,
            level.time - (total - partial),
        );
        if 0 != (*ent).sound1to2 {
            G_AddEvent(ent, EV_GENERAL_SOUND as libc::c_int, (*ent).sound1to2);
        }
        return;
    }
    if (*ent).moverState as libc::c_uint == MOVER_1TO2 as libc::c_int as libc::c_uint {
        total = (*ent).s.pos.trDuration;
        partial = level.time - (*ent).s.pos.trTime;
        if partial > total {
            partial = total
        }
        MatchTeam(
            ent,
            MOVER_2TO1 as libc::c_int,
            level.time - (total - partial),
        );
        if 0 != (*ent).sound2to1 {
            G_AddEvent(ent, EV_GENERAL_SOUND as libc::c_int, (*ent).sound2to1);
        }
        return;
    };
}
/*
================
MatchTeam

All entities in a mover team will move from pos1 to pos2
in the same amount of time
================
*/
#[no_mangle]
pub unsafe extern "C" fn MatchTeam(
    mut teamLeader: *mut gentity_t,
    mut moverState: libc::c_int,
    mut time: libc::c_int,
) {
    let mut slave: *mut gentity_t = 0 as *mut gentity_t;
    slave = teamLeader;
    while !slave.is_null() {
        SetMoverState(slave, moverState as moverState_t, time);
        slave = (*slave).teamchain
    }
}
/*
============================================================================

GENERAL MOVERS

Doors, plats, and buttons are all binary (two position) movers
Pos1 is "at rest", pos2 is "activated"
============================================================================
*/
/*
===============
SetMoverState
===============
*/
#[no_mangle]
pub unsafe extern "C" fn SetMoverState(
    mut ent: *mut gentity_t,
    mut moverState: moverState_t,
    mut time: libc::c_int,
) {
    let mut delta: vec3_t = [0.; 3];
    let mut f: libc::c_float = 0.;
    (*ent).moverState = moverState;
    (*ent).s.pos.trTime = time;
    match moverState as libc::c_uint {
        0 => {
            (*ent).s.pos.trBase[0usize] = (*ent).pos1[0usize];
            (*ent).s.pos.trBase[1usize] = (*ent).pos1[1usize];
            (*ent).s.pos.trBase[2usize] = (*ent).pos1[2usize];
            (*ent).s.pos.trType = TR_STATIONARY
        }
        1 => {
            (*ent).s.pos.trBase[0usize] = (*ent).pos2[0usize];
            (*ent).s.pos.trBase[1usize] = (*ent).pos2[1usize];
            (*ent).s.pos.trBase[2usize] = (*ent).pos2[2usize];
            (*ent).s.pos.trType = TR_STATIONARY
        }
        2 => {
            (*ent).s.pos.trBase[0usize] = (*ent).pos1[0usize];
            (*ent).s.pos.trBase[1usize] = (*ent).pos1[1usize];
            (*ent).s.pos.trBase[2usize] = (*ent).pos1[2usize];
            delta[0usize] = (*ent).pos2[0usize] - (*ent).pos1[0usize];
            delta[1usize] = (*ent).pos2[1usize] - (*ent).pos1[1usize];
            delta[2usize] = (*ent).pos2[2usize] - (*ent).pos1[2usize];
            f = (1000.0f64 / (*ent).s.pos.trDuration as libc::c_double) as libc::c_float;
            (*ent).s.pos.trDelta[0usize] = delta[0usize] * f;
            (*ent).s.pos.trDelta[1usize] = delta[1usize] * f;
            (*ent).s.pos.trDelta[2usize] = delta[2usize] * f;
            (*ent).s.pos.trType = TR_LINEAR_STOP
        }
        3 => {
            (*ent).s.pos.trBase[0usize] = (*ent).pos2[0usize];
            (*ent).s.pos.trBase[1usize] = (*ent).pos2[1usize];
            (*ent).s.pos.trBase[2usize] = (*ent).pos2[2usize];
            delta[0usize] = (*ent).pos1[0usize] - (*ent).pos2[0usize];
            delta[1usize] = (*ent).pos1[1usize] - (*ent).pos2[1usize];
            delta[2usize] = (*ent).pos1[2usize] - (*ent).pos2[2usize];
            f = (1000.0f64 / (*ent).s.pos.trDuration as libc::c_double) as libc::c_float;
            (*ent).s.pos.trDelta[0usize] = delta[0usize] * f;
            (*ent).s.pos.trDelta[1usize] = delta[1usize] * f;
            (*ent).s.pos.trDelta[2usize] = delta[2usize] * f;
            (*ent).s.pos.trType = TR_LINEAR_STOP
        }
        _ => {}
    }
    BG_EvaluateTrajectory(
        &mut (*ent).s.pos,
        level.time,
        (*ent).r.currentOrigin.as_mut_ptr(),
    );
    trap_LinkEntity(ent);
}
/*
================
Touch_DoorTriggerSpectator
================
*/
unsafe extern "C" fn Touch_DoorTriggerSpectator(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut trace: *mut trace_t,
) {
    let mut axis: libc::c_int = 0;
    let mut doorMin: libc::c_float = 0.;
    let mut doorMax: libc::c_float = 0.;
    let mut origin: vec3_t = [0.; 3];
    axis = (*ent).count;
    doorMin = (*ent).r.absmin[axis as usize] + 100i32 as libc::c_float;
    doorMax = (*ent).r.absmax[axis as usize] - 100i32 as libc::c_float;
    origin[0usize] = (*(*other).client).ps.origin[0usize];
    origin[1usize] = (*(*other).client).ps.origin[1usize];
    origin[2usize] = (*(*other).client).ps.origin[2usize];
    if origin[axis as usize] < doorMin || origin[axis as usize] > doorMax {
        return;
    }
    if fabs((origin[axis as usize] - doorMax) as libc::c_double)
        < fabs((origin[axis as usize] - doorMin) as libc::c_double)
    {
        origin[axis as usize] = doorMin - 10i32 as libc::c_float
    } else {
        origin[axis as usize] = doorMax + 10i32 as libc::c_float
    }
    TeleportPlayer(
        other,
        origin.as_mut_ptr(),
        tv(
            10000000.0f64 as libc::c_float,
            0i32 as libc::c_float,
            0i32 as libc::c_float,
        ),
    );
}
/*
==================
G_CheckProxMinePosition
==================
*/
#[no_mangle]
pub unsafe extern "C" fn G_CheckProxMinePosition(mut check: *mut gentity_t) -> qboolean {
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
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
    start[0usize] = ((*check).s.pos.trBase[0usize] as libc::c_double
        + (*check).movedir[0usize] as libc::c_double * 0.125f64) as vec_t;
    start[1usize] = ((*check).s.pos.trBase[1usize] as libc::c_double
        + (*check).movedir[1usize] as libc::c_double * 0.125f64) as vec_t;
    start[2usize] = ((*check).s.pos.trBase[2usize] as libc::c_double
        + (*check).movedir[2usize] as libc::c_double * 0.125f64) as vec_t;
    end[0usize] = (*check).s.pos.trBase[0usize] + (*check).movedir[0usize] * 2i32 as libc::c_float;
    end[1usize] = (*check).s.pos.trBase[1usize] + (*check).movedir[1usize] * 2i32 as libc::c_float;
    end[2usize] = (*check).s.pos.trBase[2usize] + (*check).movedir[2usize] * 2i32 as libc::c_float;
    trap_Trace(
        &mut tr,
        start.as_mut_ptr() as *const vec_t,
        0 as *const vec_t,
        0 as *const vec_t,
        end.as_mut_ptr() as *const vec_t,
        (*check).s.number,
        1i32,
    );
    if 0 != tr.startsolid as libc::c_uint || tr.fraction < 1i32 as libc::c_float {
        return qfalse;
    }
    return qtrue;
}
/*
==================
G_TryPushingProxMine
==================
*/
#[no_mangle]
pub unsafe extern "C" fn G_TryPushingProxMine(
    mut check: *mut gentity_t,
    mut pusher: *mut gentity_t,
    mut move_0: *mut vec_t,
    mut amove: *mut vec_t,
) -> qboolean {
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut org: vec3_t = [0.; 3];
    let mut org2: vec3_t = [0.; 3];
    let mut move2: vec3_t = [0.; 3];
    let mut ret: libc::c_int = 0;
    org[0usize] = vec3_origin[0usize] - *amove.offset(0isize);
    org[1usize] = vec3_origin[1usize] - *amove.offset(1isize);
    org[2usize] = vec3_origin[2usize] - *amove.offset(2isize);
    AngleVectors(
        org.as_mut_ptr() as *const vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        up.as_mut_ptr(),
    );
    (*check).s.pos.trBase[0usize] = (*check).s.pos.trBase[0usize] + *move_0.offset(0isize);
    (*check).s.pos.trBase[1usize] = (*check).s.pos.trBase[1usize] + *move_0.offset(1isize);
    (*check).s.pos.trBase[2usize] = (*check).s.pos.trBase[2usize] + *move_0.offset(2isize);
    org[0usize] = (*check).s.pos.trBase[0usize] - (*pusher).r.currentOrigin[0usize];
    org[1usize] = (*check).s.pos.trBase[1usize] - (*pusher).r.currentOrigin[1usize];
    org[2usize] = (*check).s.pos.trBase[2usize] - (*pusher).r.currentOrigin[2usize];
    org2[0usize] = org[0usize] * forward[0usize]
        + org[1usize] * forward[1usize]
        + org[2usize] * forward[2usize];
    org2[1usize] =
        -(org[0usize] * right[0usize] + org[1usize] * right[1usize] + org[2usize] * right[2usize]);
    org2[2usize] = org[0usize] * up[0usize] + org[1usize] * up[1usize] + org[2usize] * up[2usize];
    move2[0usize] = org2[0usize] - org[0usize];
    move2[1usize] = org2[1usize] - org[1usize];
    move2[2usize] = org2[2usize] - org[2usize];
    (*check).s.pos.trBase[0usize] = (*check).s.pos.trBase[0usize] + move2[0usize];
    (*check).s.pos.trBase[1usize] = (*check).s.pos.trBase[1usize] + move2[1usize];
    (*check).s.pos.trBase[2usize] = (*check).s.pos.trBase[2usize] + move2[2usize];
    ret = G_CheckProxMinePosition(check) as libc::c_int;
    if 0 != ret {
        (*check).r.currentOrigin[0usize] = (*check).s.pos.trBase[0usize];
        (*check).r.currentOrigin[1usize] = (*check).s.pos.trBase[1usize];
        (*check).r.currentOrigin[2usize] = (*check).s.pos.trBase[2usize];
        trap_LinkEntity(check);
    }
    return ret as qboolean;
}
/*
================
ReturnToPos1
================
*/
#[no_mangle]
pub unsafe extern "C" fn ReturnToPos1(mut ent: *mut gentity_t) {
    MatchTeam(ent, MOVER_2TO1 as libc::c_int, level.time);
    (*ent).s.loopSound = (*ent).soundLoop;
    if 0 != (*ent).sound2to1 {
        G_AddEvent(ent, EV_GENERAL_SOUND as libc::c_int, (*ent).sound2to1);
    };
}
/*
================
Reached_BinaryMover
================
*/
#[no_mangle]
pub unsafe extern "C" fn Reached_BinaryMover(mut ent: *mut gentity_t) {
    (*ent).s.loopSound = (*ent).soundLoop;
    if (*ent).moverState as libc::c_uint == MOVER_1TO2 as libc::c_int as libc::c_uint {
        SetMoverState(ent, MOVER_POS2, level.time);
        if 0 != (*ent).soundPos2 {
            G_AddEvent(ent, EV_GENERAL_SOUND as libc::c_int, (*ent).soundPos2);
        }
        (*ent).think = Some(ReturnToPos1);
        (*ent).nextthink = (level.time as libc::c_float + (*ent).wait) as libc::c_int;
        if (*ent).activator.is_null() {
            (*ent).activator = ent
        }
        G_UseTargets(ent, (*ent).activator);
    } else if (*ent).moverState as libc::c_uint == MOVER_2TO1 as libc::c_int as libc::c_uint {
        SetMoverState(ent, MOVER_POS1, level.time);
        if 0 != (*ent).soundPos1 {
            G_AddEvent(ent, EV_GENERAL_SOUND as libc::c_int, (*ent).soundPos1);
        }
        if (*ent).teammaster == ent || (*ent).teammaster.is_null() {
            trap_AdjustAreaPortalState(ent, qfalse);
        }
    } else {
        G_Error(b"Reached_BinaryMover: bad moverState\x00" as *const u8 as *const libc::c_char);
    };
}
/*
================
InitMover

"pos1", "pos2", and "speed" should be set before calling,
so the movement delta can be calculated
================
*/
#[no_mangle]
pub unsafe extern "C" fn InitMover(mut ent: *mut gentity_t) {
    let mut move_0: vec3_t = [0.; 3];
    let mut distance: libc::c_float = 0.;
    let mut light: libc::c_float = 0.;
    let mut color: vec3_t = [0.; 3];
    let mut lightSet: qboolean = qfalse;
    let mut colorSet: qboolean = qfalse;
    let mut sound: *mut libc::c_char = 0 as *mut libc::c_char;
    if !(*ent).model2.is_null() {
        (*ent).s.modelindex2 = G_ModelIndex((*ent).model2)
    }
    if 0 != G_SpawnString(
        b"noise\x00" as *const u8 as *const libc::c_char,
        b"100\x00" as *const u8 as *const libc::c_char,
        &mut sound,
    ) as u64
    {
        (*ent).s.loopSound = G_SoundIndex(sound)
    }
    lightSet = G_SpawnFloat(
        b"light\x00" as *const u8 as *const libc::c_char,
        b"100\x00" as *const u8 as *const libc::c_char,
        &mut light,
    );
    colorSet = G_SpawnVector(
        b"color\x00" as *const u8 as *const libc::c_char,
        b"1 1 1\x00" as *const u8 as *const libc::c_char,
        color.as_mut_ptr(),
    );
    if 0 != lightSet as libc::c_uint || 0 != colorSet as libc::c_uint {
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut i: libc::c_int = 0;
        r = (color[0usize] * 255i32 as libc::c_float) as libc::c_int;
        if r > 255i32 {
            r = 255i32
        }
        g = (color[1usize] * 255i32 as libc::c_float) as libc::c_int;
        if g > 255i32 {
            g = 255i32
        }
        b = (color[2usize] * 255i32 as libc::c_float) as libc::c_int;
        if b > 255i32 {
            b = 255i32
        }
        i = (light / 4i32 as libc::c_float) as libc::c_int;
        if i > 255i32 {
            i = 255i32
        }
        (*ent).s.constantLight = r | g << 8i32 | b << 16i32 | i << 24i32
    }
    (*ent).use_0 = Some(Use_BinaryMover);
    (*ent).reached = Some(Reached_BinaryMover);
    (*ent).moverState = MOVER_POS1;
    (*ent).r.svFlags = 0x80i32;
    (*ent).s.eType = ET_MOVER as libc::c_int;
    (*ent).r.currentOrigin[0usize] = (*ent).pos1[0usize];
    (*ent).r.currentOrigin[1usize] = (*ent).pos1[1usize];
    (*ent).r.currentOrigin[2usize] = (*ent).pos1[2usize];
    trap_LinkEntity(ent);
    (*ent).s.pos.trType = TR_STATIONARY;
    (*ent).s.pos.trBase[0usize] = (*ent).pos1[0usize];
    (*ent).s.pos.trBase[1usize] = (*ent).pos1[1usize];
    (*ent).s.pos.trBase[2usize] = (*ent).pos1[2usize];
    move_0[0usize] = (*ent).pos2[0usize] - (*ent).pos1[0usize];
    move_0[1usize] = (*ent).pos2[1usize] - (*ent).pos1[1usize];
    move_0[2usize] = (*ent).pos2[2usize] - (*ent).pos1[2usize];
    distance = VectorLength(move_0.as_mut_ptr() as *const vec_t);
    if 0. == (*ent).speed {
        (*ent).speed = 100i32 as libc::c_float
    }
    (*ent).s.pos.trDelta[0usize] = move_0[0usize] * (*ent).speed;
    (*ent).s.pos.trDelta[1usize] = move_0[1usize] * (*ent).speed;
    (*ent).s.pos.trDelta[2usize] = move_0[2usize] * (*ent).speed;
    (*ent).s.pos.trDuration = (distance * 1000i32 as libc::c_float / (*ent).speed) as libc::c_int;
    if (*ent).s.pos.trDuration <= 0i32 {
        (*ent).s.pos.trDuration = 1i32
    };
}
/*
===============================================================================

DOOR

A use can be triggered either by a touch function, by being shot, or by being
targeted by another entity.

===============================================================================
*/
/*
================
Blocked_Door
================
*/
#[no_mangle]
pub unsafe extern "C" fn Blocked_Door(mut ent: *mut gentity_t, mut other: *mut gentity_t) {
    if (*other).client.is_null() {
        if (*other).s.eType == ET_ITEM as libc::c_int
            && (*(*other).item).giType as libc::c_uint == IT_TEAM as libc::c_int as libc::c_uint
        {
            Team_DroppedFlagThink(other);
            return;
        }
        G_TempEntity((*other).s.origin.as_mut_ptr(), EV_ITEM_POP as libc::c_int);
        G_FreeEntity(other);
        return;
    }
    if 0 != (*ent).damage {
        G_Damage(
            other,
            ent,
            ent,
            0 as *mut vec_t,
            0 as *mut vec_t,
            (*ent).damage,
            0i32,
            MOD_CRUSH as libc::c_int,
        );
    }
    if 0 != (*ent).spawnflags & 4i32 {
        return;
    }
    Use_BinaryMover(ent, ent, other);
}
/*
======================
Think_SpawnNewDoorTrigger

All of the parts of a door have been spawned, so create
a trigger that encloses all of them
======================
*/
#[no_mangle]
pub unsafe extern "C" fn Think_SpawnNewDoorTrigger(mut ent: *mut gentity_t) {
    let mut other: *mut gentity_t = 0 as *mut gentity_t;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut best: libc::c_int = 0;
    if ent.is_null() {
        return;
    }
    other = ent;
    while !other.is_null() {
        (*other).takedamage = qtrue;
        other = (*other).teamchain
    }
    mins[0usize] = (*ent).r.absmin[0usize];
    mins[1usize] = (*ent).r.absmin[1usize];
    mins[2usize] = (*ent).r.absmin[2usize];
    maxs[0usize] = (*ent).r.absmax[0usize];
    maxs[1usize] = (*ent).r.absmax[1usize];
    maxs[2usize] = (*ent).r.absmax[2usize];
    other = (*ent).teamchain;
    while !other.is_null() {
        AddPointToBounds(
            (*other).r.absmin.as_mut_ptr() as *const vec_t,
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        AddPointToBounds(
            (*other).r.absmax.as_mut_ptr() as *const vec_t,
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
        );
        other = (*other).teamchain
    }
    best = 0i32;
    i = 1i32;
    while i < 3i32 {
        if maxs[i as usize] - mins[i as usize] < maxs[best as usize] - mins[best as usize] {
            best = i
        }
        i += 1
    }
    maxs[best as usize] += 120i32 as libc::c_float;
    mins[best as usize] -= 120i32 as libc::c_float;
    other = G_Spawn();
    (*other).classname =
        b"door_trigger\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*other).r.mins[0usize] = mins[0usize];
    (*other).r.mins[1usize] = mins[1usize];
    (*other).r.mins[2usize] = mins[2usize];
    (*other).r.maxs[0usize] = maxs[0usize];
    (*other).r.maxs[1usize] = maxs[1usize];
    (*other).r.maxs[2usize] = maxs[2usize];
    (*other).parent = ent;
    (*other).r.contents = 0x40000000i32;
    (*other).touch = Some(Touch_DoorTrigger);
    (*other).count = best;
    trap_LinkEntity(other);
    MatchTeam(ent, (*ent).moverState as libc::c_int, level.time);
}
#[no_mangle]
pub unsafe extern "C" fn Think_MatchTeam(mut ent: *mut gentity_t) {
    MatchTeam(ent, (*ent).moverState as libc::c_int, level.time);
}
/*QUAKED func_door (0 .5 .8) ? START_OPEN x CRUSHER
TOGGLE		wait in both the start and end states for a trigger event.
START_OPEN	the door to moves to its destination when spawned, and operate in reverse.  It is used to temporarily or permanently close off an area when triggered (not useful for touch or takedamage doors).
NOMONSTER	monsters will not trigger this door

"model2"	.md3 model to also draw
"angle"		determines the opening direction
"targetname" if set, no touch field will be spawned and a remote button or trigger field activates the door.
"speed"		movement speed (100 default)
"wait"		wait before returning (3 default, -1 = never return)
"lip"		lip remaining at end of move (8 default)
"dmg"		damage to inflict when blocked (2 default)
"color"		constantLight color
"light"		constantLight radius
"health"	if set, the door must be shot open
*/
#[no_mangle]
pub unsafe extern "C" fn SP_func_door(mut ent: *mut gentity_t) {
    let mut abs_movedir: vec3_t = [0.; 3];
    let mut distance: libc::c_float = 0.;
    let mut size: vec3_t = [0.; 3];
    let mut lip: libc::c_float = 0.;
    (*ent).sound2to1 = G_SoundIndex(
        b"sound/movers/doors/dr1_strt.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).sound1to2 = (*ent).sound2to1;
    (*ent).soundPos2 = G_SoundIndex(
        b"sound/movers/doors/dr1_end.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).soundPos1 = (*ent).soundPos2;
    (*ent).blocked = Some(Blocked_Door);
    if 0. == (*ent).speed {
        (*ent).speed = 400i32 as libc::c_float
    }
    if 0. == (*ent).wait {
        (*ent).wait = 2i32 as libc::c_float
    }
    (*ent).wait *= 1000i32 as libc::c_float;
    G_SpawnFloat(
        b"lip\x00" as *const u8 as *const libc::c_char,
        b"8\x00" as *const u8 as *const libc::c_char,
        &mut lip,
    );
    G_SpawnInt(
        b"dmg\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).damage,
    );
    (*ent).pos1[0usize] = (*ent).s.origin[0usize];
    (*ent).pos1[1usize] = (*ent).s.origin[1usize];
    (*ent).pos1[2usize] = (*ent).s.origin[2usize];
    trap_SetBrushModel(ent, (*ent).model);
    G_SetMovedir((*ent).s.angles.as_mut_ptr(), (*ent).movedir.as_mut_ptr());
    abs_movedir[0usize] = fabs((*ent).movedir[0usize] as libc::c_double) as vec_t;
    abs_movedir[1usize] = fabs((*ent).movedir[1usize] as libc::c_double) as vec_t;
    abs_movedir[2usize] = fabs((*ent).movedir[2usize] as libc::c_double) as vec_t;
    size[0usize] = (*ent).r.maxs[0usize] - (*ent).r.mins[0usize];
    size[1usize] = (*ent).r.maxs[1usize] - (*ent).r.mins[1usize];
    size[2usize] = (*ent).r.maxs[2usize] - (*ent).r.mins[2usize];
    distance = abs_movedir[0usize] * size[0usize]
        + abs_movedir[1usize] * size[1usize]
        + abs_movedir[2usize] * size[2usize]
        - lip;
    (*ent).pos2[0usize] = (*ent).pos1[0usize] + (*ent).movedir[0usize] * distance;
    (*ent).pos2[1usize] = (*ent).pos1[1usize] + (*ent).movedir[1usize] * distance;
    (*ent).pos2[2usize] = (*ent).pos1[2usize] + (*ent).movedir[2usize] * distance;
    if 0 != (*ent).spawnflags & 1i32 {
        let mut temp: vec3_t = [0.; 3];
        temp[0usize] = (*ent).pos2[0usize];
        temp[1usize] = (*ent).pos2[1usize];
        temp[2usize] = (*ent).pos2[2usize];
        (*ent).pos2[0usize] = (*ent).s.origin[0usize];
        (*ent).pos2[1usize] = (*ent).s.origin[1usize];
        (*ent).pos2[2usize] = (*ent).s.origin[2usize];
        (*ent).pos1[0usize] = temp[0usize];
        (*ent).pos1[1usize] = temp[1usize];
        (*ent).pos1[2usize] = temp[2usize]
    }
    InitMover(ent);
    (*ent).nextthink = level.time + 100i32;
    if 0 == (*ent).flags & 0x400i32 {
        let mut health: libc::c_int = 0;
        G_SpawnInt(
            b"health\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            &mut health,
        );
        if 0 != health {
            (*ent).takedamage = qtrue
        }
        if !(*ent).targetname.is_null() || 0 != health {
            (*ent).think = Some(Think_MatchTeam)
        } else {
            (*ent).think = Some(Think_SpawnNewDoorTrigger)
        }
    };
}
/*
===============================================================================

PLAT

===============================================================================
*/
/*
==============
Touch_Plat

Don't allow decent if a living player is on it
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Touch_Plat(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut trace: *mut trace_t,
) {
    if (*other).client.is_null()
        || (*(*other).client).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32
    {
        return;
    }
    if (*ent).moverState as libc::c_uint == MOVER_POS2 as libc::c_int as libc::c_uint {
        (*ent).nextthink = level.time + 1000i32
    };
}
/*
==============
Touch_PlatCenterTrigger

If the plat is at the bottom position, start it going up
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Touch_PlatCenterTrigger(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut trace: *mut trace_t,
) {
    if (*other).client.is_null() {
        return;
    }
    if (*(*ent).parent).moverState as libc::c_uint == MOVER_POS1 as libc::c_int as libc::c_uint {
        Use_BinaryMover((*ent).parent, ent, other);
    };
}
/*
================
SpawnPlatTrigger

Spawn a trigger in the middle of the plat's low position
Elevator cars require that the trigger extend through the entire low position,
not just sit on top of it.
================
*/
#[no_mangle]
pub unsafe extern "C" fn SpawnPlatTrigger(mut ent: *mut gentity_t) {
    let mut trigger: *mut gentity_t = 0 as *mut gentity_t;
    let mut tmin: vec3_t = [0.; 3];
    let mut tmax: vec3_t = [0.; 3];
    trigger = G_Spawn();
    (*trigger).classname =
        b"plat_trigger\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    (*trigger).touch = Some(Touch_PlatCenterTrigger);
    (*trigger).r.contents = 0x40000000i32;
    (*trigger).parent = ent;
    tmin[0usize] = (*ent).pos1[0usize] + (*ent).r.mins[0usize] + 33i32 as libc::c_float;
    tmin[1usize] = (*ent).pos1[1usize] + (*ent).r.mins[1usize] + 33i32 as libc::c_float;
    tmin[2usize] = (*ent).pos1[2usize] + (*ent).r.mins[2usize];
    tmax[0usize] = (*ent).pos1[0usize] + (*ent).r.maxs[0usize] - 33i32 as libc::c_float;
    tmax[1usize] = (*ent).pos1[1usize] + (*ent).r.maxs[1usize] - 33i32 as libc::c_float;
    tmax[2usize] = (*ent).pos1[2usize] + (*ent).r.maxs[2usize] + 8i32 as libc::c_float;
    if tmax[0usize] <= tmin[0usize] {
        tmin[0usize] = ((*ent).pos1[0usize] as libc::c_double
            + ((*ent).r.mins[0usize] + (*ent).r.maxs[0usize]) as libc::c_double * 0.5f64)
            as vec_t;
        tmax[0usize] = tmin[0usize] + 1i32 as libc::c_float
    }
    if tmax[1usize] <= tmin[1usize] {
        tmin[1usize] = ((*ent).pos1[1usize] as libc::c_double
            + ((*ent).r.mins[1usize] + (*ent).r.maxs[1usize]) as libc::c_double * 0.5f64)
            as vec_t;
        tmax[1usize] = tmin[1usize] + 1i32 as libc::c_float
    }
    (*trigger).r.mins[0usize] = tmin[0usize];
    (*trigger).r.mins[1usize] = tmin[1usize];
    (*trigger).r.mins[2usize] = tmin[2usize];
    (*trigger).r.maxs[0usize] = tmax[0usize];
    (*trigger).r.maxs[1usize] = tmax[1usize];
    (*trigger).r.maxs[2usize] = tmax[2usize];
    trap_LinkEntity(trigger);
}
/*QUAKED func_plat (0 .5 .8) ?
Plats are always drawn in the extended position so they will light correctly.

"lip"		default 8, protrusion above rest position
"height"	total height of movement, defaults to model height
"speed"		overrides default 200.
"dmg"		overrides default 2
"model2"	.md3 model to also draw
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]
pub unsafe extern "C" fn SP_func_plat(mut ent: *mut gentity_t) {
    let mut lip: libc::c_float = 0.;
    let mut height: libc::c_float = 0.;
    (*ent).sound2to1 = G_SoundIndex(
        b"sound/movers/plats/pt1_strt.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).sound1to2 = (*ent).sound2to1;
    (*ent).soundPos2 = G_SoundIndex(
        b"sound/movers/plats/pt1_end.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (*ent).soundPos1 = (*ent).soundPos2;
    (*ent).s.angles[2usize] = 0i32 as vec_t;
    (*ent).s.angles[1usize] = (*ent).s.angles[2usize];
    (*ent).s.angles[0usize] = (*ent).s.angles[1usize];
    G_SpawnFloat(
        b"speed\x00" as *const u8 as *const libc::c_char,
        b"200\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).speed,
    );
    G_SpawnInt(
        b"dmg\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).damage,
    );
    G_SpawnFloat(
        b"wait\x00" as *const u8 as *const libc::c_char,
        b"1\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).wait,
    );
    G_SpawnFloat(
        b"lip\x00" as *const u8 as *const libc::c_char,
        b"8\x00" as *const u8 as *const libc::c_char,
        &mut lip,
    );
    (*ent).wait = 1000i32 as libc::c_float;
    trap_SetBrushModel(ent, (*ent).model);
    if 0 == G_SpawnFloat(
        b"height\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut height,
    ) as u64
    {
        height = (*ent).r.maxs[2usize] - (*ent).r.mins[2usize] - lip
    }
    (*ent).pos2[0usize] = (*ent).s.origin[0usize];
    (*ent).pos2[1usize] = (*ent).s.origin[1usize];
    (*ent).pos2[2usize] = (*ent).s.origin[2usize];
    (*ent).pos1[0usize] = (*ent).pos2[0usize];
    (*ent).pos1[1usize] = (*ent).pos2[1usize];
    (*ent).pos1[2usize] = (*ent).pos2[2usize];
    (*ent).pos1[2usize] -= height;
    InitMover(ent);
    (*ent).touch = Some(Touch_Plat);
    (*ent).blocked = Some(Blocked_Door);
    (*ent).parent = ent;
    if (*ent).targetname.is_null() {
        SpawnPlatTrigger(ent);
    };
}
/*
===============================================================================

BUTTON

===============================================================================
*/
/*
==============
Touch_Button

===============
*/
#[no_mangle]
pub unsafe extern "C" fn Touch_Button(
    mut ent: *mut gentity_t,
    mut other: *mut gentity_t,
    mut trace: *mut trace_t,
) {
    if (*other).client.is_null() {
        return;
    }
    if (*ent).moverState as libc::c_uint == MOVER_POS1 as libc::c_int as libc::c_uint {
        Use_BinaryMover(ent, other, other);
    };
}
/*QUAKED func_button (0 .5 .8) ?
When a button is touched, it moves some distance in the direction of its angle, triggers all of its targets, waits some time, then returns to its original position where it can be triggered again.

"model2"	.md3 model to also draw
"angle"		determines the opening direction
"target"	all entities with a matching targetname will be used
"speed"		override the default 40 speed
"wait"		override the default 1 second wait (-1 = never return)
"lip"		override the default 4 pixel lip remaining at end of move
"health"	if set, the button must be killed instead of touched
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]
pub unsafe extern "C" fn SP_func_button(mut ent: *mut gentity_t) {
    let mut abs_movedir: vec3_t = [0.; 3];
    let mut distance: libc::c_float = 0.;
    let mut size: vec3_t = [0.; 3];
    let mut lip: libc::c_float = 0.;
    (*ent).sound1to2 = G_SoundIndex(
        b"sound/movers/switches/butn2.wav\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if 0. == (*ent).speed {
        (*ent).speed = 40i32 as libc::c_float
    }
    if 0. == (*ent).wait {
        (*ent).wait = 1i32 as libc::c_float
    }
    (*ent).wait *= 1000i32 as libc::c_float;
    (*ent).pos1[0usize] = (*ent).s.origin[0usize];
    (*ent).pos1[1usize] = (*ent).s.origin[1usize];
    (*ent).pos1[2usize] = (*ent).s.origin[2usize];
    trap_SetBrushModel(ent, (*ent).model);
    G_SpawnFloat(
        b"lip\x00" as *const u8 as *const libc::c_char,
        b"4\x00" as *const u8 as *const libc::c_char,
        &mut lip,
    );
    G_SetMovedir((*ent).s.angles.as_mut_ptr(), (*ent).movedir.as_mut_ptr());
    abs_movedir[0usize] = fabs((*ent).movedir[0usize] as libc::c_double) as vec_t;
    abs_movedir[1usize] = fabs((*ent).movedir[1usize] as libc::c_double) as vec_t;
    abs_movedir[2usize] = fabs((*ent).movedir[2usize] as libc::c_double) as vec_t;
    size[0usize] = (*ent).r.maxs[0usize] - (*ent).r.mins[0usize];
    size[1usize] = (*ent).r.maxs[1usize] - (*ent).r.mins[1usize];
    size[2usize] = (*ent).r.maxs[2usize] - (*ent).r.mins[2usize];
    distance = abs_movedir[0usize] * size[0usize]
        + abs_movedir[1usize] * size[1usize]
        + abs_movedir[2usize] * size[2usize]
        - lip;
    (*ent).pos2[0usize] = (*ent).pos1[0usize] + (*ent).movedir[0usize] * distance;
    (*ent).pos2[1usize] = (*ent).pos1[1usize] + (*ent).movedir[1usize] * distance;
    (*ent).pos2[2usize] = (*ent).pos1[2usize] + (*ent).movedir[2usize] * distance;
    if 0 != (*ent).health {
        (*ent).takedamage = qtrue
    } else {
        (*ent).touch = Some(Touch_Button)
    }
    InitMover(ent);
}
/*
===============================================================================

TRAIN

===============================================================================
*/
/*
===============
Think_BeginMoving

The wait time at a corner has completed, so start moving again
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Think_BeginMoving(mut ent: *mut gentity_t) {
    (*ent).s.pos.trTime = level.time;
    (*ent).s.pos.trType = TR_LINEAR_STOP;
}
/*
===============
Reached_Train
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Reached_Train(mut ent: *mut gentity_t) {
    let mut next: *mut gentity_t = 0 as *mut gentity_t;
    let mut speed: libc::c_float = 0.;
    let mut move_0: vec3_t = [0.; 3];
    let mut length: libc::c_float = 0.;
    next = (*ent).nextTrain;
    if next.is_null() || (*next).nextTrain.is_null() {
        return;
    }
    G_UseTargets(next, 0 as *mut gentity_t);
    (*ent).nextTrain = (*next).nextTrain;
    (*ent).pos1[0usize] = (*next).s.origin[0usize];
    (*ent).pos1[1usize] = (*next).s.origin[1usize];
    (*ent).pos1[2usize] = (*next).s.origin[2usize];
    (*ent).pos2[0usize] = (*(*next).nextTrain).s.origin[0usize];
    (*ent).pos2[1usize] = (*(*next).nextTrain).s.origin[1usize];
    (*ent).pos2[2usize] = (*(*next).nextTrain).s.origin[2usize];
    if 0. != (*next).speed {
        speed = (*next).speed
    } else {
        speed = (*ent).speed
    }
    if speed < 1i32 as libc::c_float {
        speed = 1i32 as libc::c_float
    }
    move_0[0usize] = (*ent).pos2[0usize] - (*ent).pos1[0usize];
    move_0[1usize] = (*ent).pos2[1usize] - (*ent).pos1[1usize];
    move_0[2usize] = (*ent).pos2[2usize] - (*ent).pos1[2usize];
    length = VectorLength(move_0.as_mut_ptr() as *const vec_t);
    (*ent).s.pos.trDuration = (length * 1000i32 as libc::c_float / speed) as libc::c_int;
    (*ent).r.svFlags &= !0x1i32;
    if (*ent).s.pos.trDuration < 1i32 {
        (*ent).s.pos.trDuration = 1i32;
        (*ent).r.svFlags |= 0x1i32
    }
    (*ent).s.loopSound = (*next).soundLoop;
    SetMoverState(ent, MOVER_1TO2, level.time);
    if 0. != (*next).wait {
        (*ent).nextthink =
            (level.time as libc::c_float + (*next).wait * 1000i32 as libc::c_float) as libc::c_int;
        (*ent).think = Some(Think_BeginMoving);
        (*ent).s.pos.trType = TR_STATIONARY
    };
}
/*
===============
Think_SetupTrainTargets

Link all the corners together
===============
*/
#[no_mangle]
pub unsafe extern "C" fn Think_SetupTrainTargets(mut ent: *mut gentity_t) {
    let mut path: *mut gentity_t = 0 as *mut gentity_t;
    let mut next: *mut gentity_t = 0 as *mut gentity_t;
    let mut start: *mut gentity_t = 0 as *mut gentity_t;
    (*ent).nextTrain = G_Find(
        0 as *mut gentity_t,
        &mut (*(0 as *mut gentity_t)).targetname as *mut *mut libc::c_char as size_t as libc::c_int,
        (*ent).target,
    );
    if (*ent).nextTrain.is_null() {
        G_Printf(
            b"func_train at %s with an unfound target\n\x00" as *const u8 as *const libc::c_char,
            vtos((*ent).r.absmin.as_mut_ptr() as *const vec_t),
        );
        return;
    }
    start = 0 as *mut gentity_t;
    path = (*ent).nextTrain;
    while path != start {
        if start.is_null() {
            start = path
        }
        if (*path).target.is_null() {
            G_Printf(
                b"Train corner at %s without a target\n\x00" as *const u8 as *const libc::c_char,
                vtos((*path).s.origin.as_mut_ptr() as *const vec_t),
            );
            return;
        }
        next = 0 as *mut gentity_t;
        loop {
            next = G_Find(
                next,
                &mut (*(0 as *mut gentity_t)).targetname as *mut *mut libc::c_char as size_t
                    as libc::c_int,
                (*path).target,
            );
            if next.is_null() {
                G_Printf(
                    b"Train corner at %s without a target path_corner\n\x00" as *const u8
                        as *const libc::c_char,
                    vtos((*path).s.origin.as_mut_ptr() as *const vec_t),
                );
                return;
            }
            if !(0
                != strcmp(
                    (*next).classname,
                    b"path_corner\x00" as *const u8 as *const libc::c_char,
                ))
            {
                break;
            }
        }
        (*path).nextTrain = next;
        path = next
    }
    Reached_Train(ent);
}
/*QUAKED path_corner (.5 .3 0) (-8 -8 -8) (8 8 8)
Train path corners.
Target: next path corner and other targets to fire
"speed" speed to move to the next corner
"wait" seconds to wait before behining move to next corner
*/
#[no_mangle]
pub unsafe extern "C" fn SP_path_corner(mut self_0: *mut gentity_t) {
    if (*self_0).targetname.is_null() {
        G_Printf(
            b"path_corner with no targetname at %s\n\x00" as *const u8 as *const libc::c_char,
            vtos((*self_0).s.origin.as_mut_ptr() as *const vec_t),
        );
        G_FreeEntity(self_0);
        return;
    };
}
// path corners don't need to be linked in
/*QUAKED func_train (0 .5 .8) ? START_ON TOGGLE BLOCK_STOPS
A train is a mover that moves between path_corner target points.
Trains MUST HAVE AN ORIGIN BRUSH.
The train spawns at the first target it is pointing at.
"model2"	.md3 model to also draw
"speed"		default 100
"dmg"		default	2
"noise"		looping sound to play when the train is in motion
"target"	next path corner
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]
pub unsafe extern "C" fn SP_func_train(mut self_0: *mut gentity_t) {
    (*self_0).s.angles[2usize] = 0i32 as vec_t;
    (*self_0).s.angles[1usize] = (*self_0).s.angles[2usize];
    (*self_0).s.angles[0usize] = (*self_0).s.angles[1usize];
    if 0 != (*self_0).spawnflags & 4i32 {
        (*self_0).damage = 0i32
    } else if 0 == (*self_0).damage {
        (*self_0).damage = 2i32
    }
    if 0. == (*self_0).speed {
        (*self_0).speed = 100i32 as libc::c_float
    }
    if (*self_0).target.is_null() {
        G_Printf(
            b"func_train without a target at %s\n\x00" as *const u8 as *const libc::c_char,
            vtos((*self_0).r.absmin.as_mut_ptr() as *const vec_t),
        );
        G_FreeEntity(self_0);
        return;
    }
    trap_SetBrushModel(self_0, (*self_0).model);
    InitMover(self_0);
    (*self_0).reached = Some(Reached_Train);
    (*self_0).nextthink = level.time + 100i32;
    (*self_0).think = Some(Think_SetupTrainTargets);
}
/*
===============================================================================

STATIC

===============================================================================
*/
/*QUAKED func_static (0 .5 .8) ?
A bmodel that just sits there, doing nothing.  Can be used for conditional walls and models.
"model2"	.md3 model to also draw
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]
pub unsafe extern "C" fn SP_func_static(mut ent: *mut gentity_t) {
    trap_SetBrushModel(ent, (*ent).model);
    InitMover(ent);
    (*ent).s.pos.trBase[0usize] = (*ent).s.origin[0usize];
    (*ent).s.pos.trBase[1usize] = (*ent).s.origin[1usize];
    (*ent).s.pos.trBase[2usize] = (*ent).s.origin[2usize];
    (*ent).r.currentOrigin[0usize] = (*ent).s.origin[0usize];
    (*ent).r.currentOrigin[1usize] = (*ent).s.origin[1usize];
    (*ent).r.currentOrigin[2usize] = (*ent).s.origin[2usize];
}
/*
===============================================================================

ROTATING

===============================================================================
*/
/*QUAKED func_rotating (0 .5 .8) ? START_ON - X_AXIS Y_AXIS
You need to have an origin brush as part of this entity.  The center of that brush will be
the point around which it is rotated. It will rotate around the Z axis by default.  You can
check either the X_AXIS or Y_AXIS box to change that.

"model2"	.md3 model to also draw
"speed"		determines how fast it moves; default value is 100.
"dmg"		damage to inflict when blocked (2 default)
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]
pub unsafe extern "C" fn SP_func_rotating(mut ent: *mut gentity_t) {
    if 0. == (*ent).speed {
        (*ent).speed = 100i32 as libc::c_float
    }
    (*ent).s.apos.trType = TR_LINEAR;
    if 0 != (*ent).spawnflags & 4i32 {
        (*ent).s.apos.trDelta[2usize] = (*ent).speed
    } else if 0 != (*ent).spawnflags & 8i32 {
        (*ent).s.apos.trDelta[0usize] = (*ent).speed
    } else {
        (*ent).s.apos.trDelta[1usize] = (*ent).speed
    }
    if 0 == (*ent).damage {
        (*ent).damage = 2i32
    }
    trap_SetBrushModel(ent, (*ent).model);
    InitMover(ent);
    (*ent).s.pos.trBase[0usize] = (*ent).s.origin[0usize];
    (*ent).s.pos.trBase[1usize] = (*ent).s.origin[1usize];
    (*ent).s.pos.trBase[2usize] = (*ent).s.origin[2usize];
    (*ent).r.currentOrigin[0usize] = (*ent).s.pos.trBase[0usize];
    (*ent).r.currentOrigin[1usize] = (*ent).s.pos.trBase[1usize];
    (*ent).r.currentOrigin[2usize] = (*ent).s.pos.trBase[2usize];
    (*ent).r.currentAngles[0usize] = (*ent).s.apos.trBase[0usize];
    (*ent).r.currentAngles[1usize] = (*ent).s.apos.trBase[1usize];
    (*ent).r.currentAngles[2usize] = (*ent).s.apos.trBase[2usize];
    trap_LinkEntity(ent);
}
/*
===============================================================================

BOBBING

===============================================================================
*/
/*QUAKED func_bobbing (0 .5 .8) ? X_AXIS Y_AXIS
Normally bobs on the Z axis
"model2"	.md3 model to also draw
"height"	amplitude of bob (32 default)
"speed"		seconds to complete a bob cycle (4 default)
"phase"		the 0.0 to 1.0 offset in the cycle to start at
"dmg"		damage to inflict when blocked (2 default)
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]
pub unsafe extern "C" fn SP_func_bobbing(mut ent: *mut gentity_t) {
    let mut height: libc::c_float = 0.;
    let mut phase: libc::c_float = 0.;
    G_SpawnFloat(
        b"speed\x00" as *const u8 as *const libc::c_char,
        b"4\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).speed,
    );
    G_SpawnFloat(
        b"height\x00" as *const u8 as *const libc::c_char,
        b"32\x00" as *const u8 as *const libc::c_char,
        &mut height,
    );
    G_SpawnInt(
        b"dmg\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).damage,
    );
    G_SpawnFloat(
        b"phase\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut phase,
    );
    trap_SetBrushModel(ent, (*ent).model);
    InitMover(ent);
    (*ent).s.pos.trBase[0usize] = (*ent).s.origin[0usize];
    (*ent).s.pos.trBase[1usize] = (*ent).s.origin[1usize];
    (*ent).s.pos.trBase[2usize] = (*ent).s.origin[2usize];
    (*ent).r.currentOrigin[0usize] = (*ent).s.origin[0usize];
    (*ent).r.currentOrigin[1usize] = (*ent).s.origin[1usize];
    (*ent).r.currentOrigin[2usize] = (*ent).s.origin[2usize];
    (*ent).s.pos.trDuration = ((*ent).speed * 1000i32 as libc::c_float) as libc::c_int;
    (*ent).s.pos.trTime = ((*ent).s.pos.trDuration as libc::c_float * phase) as libc::c_int;
    (*ent).s.pos.trType = TR_SINE;
    if 0 != (*ent).spawnflags & 1i32 {
        (*ent).s.pos.trDelta[0usize] = height
    } else if 0 != (*ent).spawnflags & 2i32 {
        (*ent).s.pos.trDelta[1usize] = height
    } else {
        (*ent).s.pos.trDelta[2usize] = height
    };
}
/*
===============================================================================

PENDULUM

===============================================================================
*/
/*QUAKED func_pendulum (0 .5 .8) ?
You need to have an origin brush as part of this entity.
Pendulums always swing north / south on unrotated models.  Add an angles field to the model to allow rotation in other directions.
Pendulum frequency is a physical constant based on the length of the beam and gravity.
"model2"	.md3 model to also draw
"speed"		the number of degrees each way the pendulum swings, (30 default)
"phase"		the 0.0 to 1.0 offset in the cycle to start at
"dmg"		damage to inflict when blocked (2 default)
"color"		constantLight color
"light"		constantLight radius
*/
#[no_mangle]
pub unsafe extern "C" fn SP_func_pendulum(mut ent: *mut gentity_t) {
    let mut freq: libc::c_float = 0.;
    let mut length: libc::c_float = 0.;
    let mut phase: libc::c_float = 0.;
    let mut speed: libc::c_float = 0.;
    G_SpawnFloat(
        b"speed\x00" as *const u8 as *const libc::c_char,
        b"30\x00" as *const u8 as *const libc::c_char,
        &mut speed,
    );
    G_SpawnInt(
        b"dmg\x00" as *const u8 as *const libc::c_char,
        b"2\x00" as *const u8 as *const libc::c_char,
        &mut (*ent).damage,
    );
    G_SpawnFloat(
        b"phase\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut phase,
    );
    trap_SetBrushModel(ent, (*ent).model);
    length = fabs((*ent).r.mins[2usize] as libc::c_double) as libc::c_float;
    if length < 8i32 as libc::c_float {
        length = 8i32 as libc::c_float
    }
    freq = (1i32 as libc::c_double / (3.14159265358979323846f64 * 2i32 as libc::c_double)
        * sqrt((g_gravity.value / (3i32 as libc::c_float * length)) as libc::c_double))
        as libc::c_float;
    (*ent).s.pos.trDuration = (1000i32 as libc::c_float / freq) as libc::c_int;
    InitMover(ent);
    (*ent).s.pos.trBase[0usize] = (*ent).s.origin[0usize];
    (*ent).s.pos.trBase[1usize] = (*ent).s.origin[1usize];
    (*ent).s.pos.trBase[2usize] = (*ent).s.origin[2usize];
    (*ent).r.currentOrigin[0usize] = (*ent).s.origin[0usize];
    (*ent).r.currentOrigin[1usize] = (*ent).s.origin[1usize];
    (*ent).r.currentOrigin[2usize] = (*ent).s.origin[2usize];
    (*ent).s.apos.trBase[0usize] = (*ent).s.angles[0usize];
    (*ent).s.apos.trBase[1usize] = (*ent).s.angles[1usize];
    (*ent).s.apos.trBase[2usize] = (*ent).s.angles[2usize];
    (*ent).s.apos.trDuration = (1000i32 as libc::c_float / freq) as libc::c_int;
    (*ent).s.apos.trTime = ((*ent).s.apos.trDuration as libc::c_float * phase) as libc::c_int;
    (*ent).s.apos.trType = TR_SINE;
    (*ent).s.apos.trDelta[2usize] = speed;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pushed_t {
    pub ent: *mut gentity_t,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub deltayaw: libc::c_float,
}
