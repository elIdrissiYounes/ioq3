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
    gitem_s, gitem_t, itemType_t, pmove_t, powerup_t, team_t, unnamed, unnamed_0, unnamed_1,
    unnamed_2, unnamed_3, unnamed_4, ET_BEAM, ET_EVENTS, ET_GENERAL, ET_GRAPPLE, ET_INVISIBLE,
    ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER, ET_TEAM,
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
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, IT_AMMO, IT_ARMOR, IT_BAD,
    IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, MOD_BFG,
    MOD_BFG_SPLASH, MOD_CRUSH, MOD_FALLING, MOD_GAUNTLET, MOD_GRAPPLE, MOD_GRENADE,
    MOD_GRENADE_SPLASH, MOD_LAVA, MOD_LIGHTNING, MOD_MACHINEGUN, MOD_PLASMA, MOD_PLASMA_SPLASH,
    MOD_RAILGUN, MOD_ROCKET, MOD_ROCKET_SPLASH, MOD_SHOTGUN, MOD_SLIME, MOD_SUICIDE,
    MOD_TARGET_LASER, MOD_TELEFRAG, MOD_TRIGGER_HURT, MOD_UNKNOWN, MOD_WATER, PM_DEAD, PM_FREEZE,
    PM_INTERMISSION, PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, PW_AMMOREGEN,
    PW_BATTLESUIT, PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS,
    PW_INVULNERABILITY, PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN,
    PW_SCOUT, STAT_ARMOR, STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM,
    STAT_MAX_HEALTH, STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
    WP_BFG, WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN,
    WP_NONE, WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
};
use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
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
    clientConnected_t, clientPersistant_t, clientSession_t, gclient_s, gclient_t, gentity_s,
    gentity_t, level_locals_t, moverState_t, playerTeamStateState_t, playerTeamState_t,
    spectatorState_t, trap_Cvar_Set, trap_Cvar_Update, trap_DropClient, trap_EntitiesInBox,
    trap_EntityContact, trap_GetUsercmd, trap_LinkEntity, trap_PointContents,
    trap_SendServerCommand, trap_Trace, trap_UnlinkEntity, CON_CONNECTED, CON_CONNECTING,
    CON_DISCONNECTED, MOVER_1TO2, MOVER_2TO1, MOVER_POS1, MOVER_POS2, SPECTATOR_FOLLOW,
    SPECTATOR_FREE, SPECTATOR_NOT, SPECTATOR_SCOREBOARD, TEAM_ACTIVE, TEAM_BEGIN,
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
use g_utils::{
    tv, vectoyaw, vtos, G_AddEvent, G_AddPredictableEvent, G_Find, G_FreeEntity, G_InitGentity,
    G_KillBox, G_ModelIndex, G_PickTarget, G_SetMovedir, G_SetOrigin, G_Sound, G_SoundIndex,
    G_Spawn, G_TeamCommand, G_TempEntity, G_UseTargets,
};
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
use stdlib::memset;

#[no_mangle]
pub unsafe extern "C" fn G_TouchTriggers(mut ent: *mut gentity_t) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut touch: [libc::c_int; 1024] = [0; 1024];
    let mut hit: *mut gentity_t = 0 as *mut gentity_t;
    let mut trace: trace_t = trace_t {
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
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    static mut range: vec3_t = [40i32 as vec_t, 40i32 as vec_t, 52i32 as vec_t];
    if (*ent).client.is_null() {
        return;
    }
    if (*(*ent).client).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        return;
    }
    mins[0usize] = (*(*ent).client).ps.origin[0usize] - range[0usize];
    mins[1usize] = (*(*ent).client).ps.origin[1usize] - range[1usize];
    mins[2usize] = (*(*ent).client).ps.origin[2usize] - range[2usize];
    maxs[0usize] = (*(*ent).client).ps.origin[0usize] + range[0usize];
    maxs[1usize] = (*(*ent).client).ps.origin[1usize] + range[1usize];
    maxs[2usize] = (*(*ent).client).ps.origin[2usize] + range[2usize];
    num = trap_EntitiesInBox(
        mins.as_mut_ptr() as *const vec_t,
        maxs.as_mut_ptr() as *const vec_t,
        touch.as_mut_ptr(),
        1i32 << 10i32,
    );
    mins[0usize] = (*(*ent).client).ps.origin[0usize] + (*ent).r.mins[0usize];
    mins[1usize] = (*(*ent).client).ps.origin[1usize] + (*ent).r.mins[1usize];
    mins[2usize] = (*(*ent).client).ps.origin[2usize] + (*ent).r.mins[2usize];
    maxs[0usize] = (*(*ent).client).ps.origin[0usize] + (*ent).r.maxs[0usize];
    maxs[1usize] = (*(*ent).client).ps.origin[1usize] + (*ent).r.maxs[1usize];
    maxs[2usize] = (*(*ent).client).ps.origin[2usize] + (*ent).r.maxs[2usize];
    let mut current_block_19: u64;
    i = 0i32;
    while i < num {
        hit = &mut g_entities[touch[i as usize] as usize] as *mut gentity_t;
        if !((*hit).touch.is_none() && (*ent).touch.is_none()) {
            if !(0 == (*hit).r.contents & 0x40000000i32) {
                // ignore most entities if a spectator
                if (*(*ent).client).sess.sessionTeam as libc::c_uint
                    == TEAM_SPECTATOR as libc::c_int as libc::c_uint
                {
                    if (*hit).s.eType != ET_TELEPORT_TRIGGER as libc::c_int
                        && (*hit).touch != Some(Touch_DoorTrigger)
                    {
                        // this is ugly but adding a new ET_? type will
                        // most likely cause network incompatibilities
                        current_block_19 = 13586036798005543211;
                    } else {
                        current_block_19 = 2668756484064249700;
                    }
                } else {
                    current_block_19 = 2668756484064249700;
                }
                match current_block_19 {
                    13586036798005543211 => {}
                    _ => {
                        // use separate code for determining if an item is picked up
                        // so you don't have to actually contact its bounding box
                        if (*hit).s.eType == ET_ITEM as libc::c_int {
                            if 0 == BG_PlayerTouchesItem(
                                &mut (*(*ent).client).ps,
                                &mut (*hit).s,
                                level.time,
                            ) as u64
                            {
                                current_block_19 = 13586036798005543211;
                            } else {
                                current_block_19 = 15345278821338558188;
                            }
                        } else if 0
                            == trap_EntityContact(
                                mins.as_mut_ptr() as *const vec_t,
                                maxs.as_mut_ptr() as *const vec_t,
                                hit,
                            ) as u64
                        {
                            current_block_19 = 13586036798005543211;
                        } else {
                            current_block_19 = 15345278821338558188;
                        }
                        match current_block_19 {
                            13586036798005543211 => {}
                            _ => {
                                memset(
                                    &mut trace as *mut trace_t as *mut libc::c_void,
                                    0i32,
                                    ::std::mem::size_of::<trace_t>() as libc::c_ulong,
                                );
                                if (*hit).touch.is_some() {
                                    (*hit).touch.expect("non-null function pointer")(
                                        hit, ent, &mut trace,
                                    );
                                }
                                if 0 != (*ent).r.svFlags & 0x8i32 && (*ent).touch.is_some() {
                                    (*ent).touch.expect("non-null function pointer")(
                                        ent, hit, &mut trace,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
    if (*(*ent).client).ps.jumppad_frame != (*(*ent).client).ps.pmove_framecount {
        (*(*ent).client).ps.jumppad_frame = 0i32;
        (*(*ent).client).ps.jumppad_ent = 0i32
    };
}
//
// g_active.c
//
#[no_mangle]
pub unsafe extern "C" fn ClientThink(mut clientNum: libc::c_int) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    ent = g_entities.as_mut_ptr().offset(clientNum as isize);
    trap_GetUsercmd(clientNum, &mut (*(*ent).client).pers.cmd);
    (*(*ent).client).lastCmdTime = level.time;
    if 0 == (*ent).r.svFlags & 0x8i32 && 0 == g_synchronousClients.integer {
        ClientThink_real(ent);
    };
}
/*
==============
ClientThink

This will be called once for each client frame, which will
usually be a couple times for each server frame on fast clients.

If "g_synchronousClients 1" is set, this will be called exactly
once for each server frame, which makes for smooth demo recording.
==============
*/
#[no_mangle]
pub unsafe extern "C" fn ClientThink_real(mut ent: *mut gentity_t) {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut pm1: pmove_t = pmove_t {
        ps: 0 as *mut playerState_t,
        cmd: usercmd_s {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        },
        tracemask: 0,
        debugLevel: 0,
        noFootsteps: qfalse,
        gauntletHit: qfalse,
        framecount: 0,
        numtouch: 0,
        touchents: [0; 32],
        mins: [0.; 3],
        maxs: [0.; 3],
        watertype: 0,
        waterlevel: 0,
        xyspeed: 0.,
        pmove_fixed: 0,
        pmove_msec: 0,
        trace: None,
        pointcontents: None,
    };
    let mut oldEventSequence: libc::c_int = 0;
    let mut msec: libc::c_int = 0;
    let mut ucmd: *mut usercmd_t = 0 as *mut usercmd_t;
    client = (*ent).client;
    if (*client).pers.connected as libc::c_uint != CON_CONNECTED as libc::c_int as libc::c_uint {
        return;
    }
    ucmd = &mut (*(*ent).client).pers.cmd;
    if (*ucmd).serverTime > level.time + 200i32 {
        (*ucmd).serverTime = level.time + 200i32
    }
    if (*ucmd).serverTime < level.time - 1000i32 {
        (*ucmd).serverTime = level.time - 1000i32
    }
    msec = (*ucmd).serverTime - (*client).ps.commandTime;
    if msec < 1i32
        && (*client).sess.spectatorState as libc::c_uint
            != SPECTATOR_FOLLOW as libc::c_int as libc::c_uint
    {
        return;
    }
    if msec > 200i32 {
        msec = 200i32
    }
    if pmove_msec.integer < 8i32 {
        trap_Cvar_Set(
            b"pmove_msec\x00" as *const u8 as *const libc::c_char,
            b"8\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Update(&mut pmove_msec);
    } else if pmove_msec.integer > 33i32 {
        trap_Cvar_Set(
            b"pmove_msec\x00" as *const u8 as *const libc::c_char,
            b"33\x00" as *const u8 as *const libc::c_char,
        );
        trap_Cvar_Update(&mut pmove_msec);
    }
    if 0 != pmove_fixed.integer || 0 != (*client).pers.pmoveFixed as libc::c_uint {
        (*ucmd).serverTime = ((*ucmd).serverTime + pmove_msec.integer - 1i32) / pmove_msec.integer
            * pmove_msec.integer
    }
    if 0 != level.intermissiontime {
        ClientIntermissionThink(client);
        return;
    }
    if (*client).sess.sessionTeam as libc::c_uint == TEAM_SPECTATOR as libc::c_int as libc::c_uint {
        if (*client).sess.spectatorState as libc::c_uint
            == SPECTATOR_SCOREBOARD as libc::c_int as libc::c_uint
        {
            return;
        }
        SpectatorThink(ent, ucmd);
        return;
    }
    if 0 == ClientInactivityTimer(client) as u64 {
        return;
    }
    if level.time > (*client).rewardTime {
        (*client).ps.eFlags &= !(0x8000i32 | 0x8i32 | 0x40i32 | 0x20000i32 | 0x10000i32 | 0x800i32)
    }
    if 0 != (*client).noclip as u64 {
        (*client).ps.pm_type = PM_NOCLIP as libc::c_int
    } else if (*client).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        (*client).ps.pm_type = PM_DEAD as libc::c_int
    } else {
        (*client).ps.pm_type = PM_NORMAL as libc::c_int
    }
    (*client).ps.gravity = g_gravity.value as libc::c_int;
    (*client).ps.speed = g_speed.value as libc::c_int;
    if 0 != (*client).ps.powerups[PW_HASTE as libc::c_int as usize] {
        (*client).ps.speed = ((*client).ps.speed as libc::c_double * 1.3f64) as libc::c_int
    }
    if (*client).ps.weapon == WP_GRAPPLING_HOOK as libc::c_int
        && !(*client).hook.is_null()
        && 0 == (*ucmd).buttons & 1i32
    {
        Weapon_HookFree((*client).hook);
    }
    oldEventSequence = (*client).ps.eventSequence;
    memset(
        &mut pm1 as *mut pmove_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<pmove_t>() as libc::c_ulong,
    );
    if (*client).ps.weapon == WP_GAUNTLET as libc::c_int
        && 0 == (*ucmd).buttons & 2i32
        && 0 != (*ucmd).buttons & 1i32
        && (*client).ps.weaponTime <= 0i32
    {
        pm1.gauntletHit = CheckGauntletAttack(ent)
    }
    if 0 != (*ent).flags & 0x8000i32 {
        (*ent).flags &= !0x8000i32;
        (*(*ent).client).pers.cmd.buttons |= 8i32
    }
    pm1.ps = &mut (*client).ps;
    pm1.cmd = *ucmd;
    if (*pm1.ps).pm_type == PM_DEAD as libc::c_int {
        pm1.tracemask = (1i32 | 0x10000i32 | 0x2000000i32) & !0x2000000i32
    } else if 0 != (*ent).r.svFlags & 0x8i32 {
        pm1.tracemask = 1i32 | 0x10000i32 | 0x2000000i32 | 0x400000i32
    } else {
        pm1.tracemask = 1i32 | 0x10000i32 | 0x2000000i32
    }
    pm1.trace = Some(trap_Trace);
    pm1.pointcontents = Some(trap_PointContents);
    pm1.debugLevel = g_debugMove.integer;
    pm1.noFootsteps = (g_dmflags.integer & 32i32 > 0i32) as libc::c_int as qboolean;
    pm1.pmove_fixed = (pmove_fixed.integer as libc::c_uint
        | (*client).pers.pmoveFixed as libc::c_uint) as libc::c_int;
    pm1.pmove_msec = pmove_msec.integer;
    (*client).oldOrigin[0usize] = (*client).ps.origin[0usize];
    (*client).oldOrigin[1usize] = (*client).ps.origin[1usize];
    (*client).oldOrigin[2usize] = (*client).ps.origin[2usize];
    Pmove(&mut pm1);
    if (*(*ent).client).ps.eventSequence != oldEventSequence {
        (*ent).eventTime = level.time
    }
    if 0 != g_smoothClients.integer {
        BG_PlayerStateToEntityStateExtraPolate(
            &mut (*(*ent).client).ps,
            &mut (*ent).s,
            (*(*ent).client).ps.commandTime,
            qtrue,
        );
    } else {
        BG_PlayerStateToEntityState(&mut (*(*ent).client).ps, &mut (*ent).s, qtrue);
    }
    SendPendingPredictableEvents(&mut (*(*ent).client).ps);
    if 0 == (*(*ent).client).ps.eFlags & 0x100i32 {
        (*client).fireHeld = qfalse
    }
    (*ent).r.currentOrigin[0usize] = (*ent).s.pos.trBase[0usize];
    (*ent).r.currentOrigin[1usize] = (*ent).s.pos.trBase[1usize];
    (*ent).r.currentOrigin[2usize] = (*ent).s.pos.trBase[2usize];
    (*ent).r.mins[0usize] = pm1.mins[0usize];
    (*ent).r.mins[1usize] = pm1.mins[1usize];
    (*ent).r.mins[2usize] = pm1.mins[2usize];
    (*ent).r.maxs[0usize] = pm1.maxs[0usize];
    (*ent).r.maxs[1usize] = pm1.maxs[1usize];
    (*ent).r.maxs[2usize] = pm1.maxs[2usize];
    (*ent).waterlevel = pm1.waterlevel;
    (*ent).watertype = pm1.watertype;
    ClientEvents(ent, oldEventSequence);
    trap_LinkEntity(ent);
    if 0 == (*(*ent).client).noclip as u64 {
        G_TouchTriggers(ent);
    }
    (*ent).r.currentOrigin[0usize] = (*(*ent).client).ps.origin[0usize];
    (*ent).r.currentOrigin[1usize] = (*(*ent).client).ps.origin[1usize];
    (*ent).r.currentOrigin[2usize] = (*(*ent).client).ps.origin[2usize];
    BotTestAAS((*ent).r.currentOrigin.as_mut_ptr());
    ClientImpacts(ent, &mut pm1);
    if (*(*ent).client).ps.eventSequence != oldEventSequence {
        (*ent).eventTime = level.time
    }
    (*client).oldbuttons = (*client).buttons;
    (*client).buttons = (*ucmd).buttons;
    (*client).latched_buttons |= (*client).buttons & !(*client).oldbuttons;
    if (*client).ps.stats[STAT_HEALTH as libc::c_int as usize] <= 0i32 {
        if level.time > (*client).respawnTime {
            if g_forcerespawn.integer > 0i32
                && level.time - (*client).respawnTime > g_forcerespawn.integer * 1000i32
            {
                ClientRespawn(ent);
                return;
            }
            if 0 != (*ucmd).buttons & (1i32 | 4i32) {
                ClientRespawn(ent);
            }
        }
        return;
    }
    ClientTimerActions(ent, msec);
}
/*
==================
ClientTimerActions

Actions that happen once a second
==================
*/
#[no_mangle]
pub unsafe extern "C" fn ClientTimerActions(mut ent: *mut gentity_t, mut msec: libc::c_int) {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    client = (*ent).client;
    (*client).timeResidual += msec;
    while (*client).timeResidual >= 1000i32 {
        (*client).timeResidual -= 1000i32;
        if 0 != (*client).ps.powerups[PW_REGEN as libc::c_int as usize] {
            if (*ent).health < (*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] {
                (*ent).health += 15i32;
                if (*ent).health as libc::c_double
                    > (*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] as libc::c_double
                        * 1.1f64
                {
                    (*ent).health = ((*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize]
                        as libc::c_double
                        * 1.1f64) as libc::c_int
                }
                G_AddEvent(ent, EV_POWERUP_REGEN as libc::c_int, 0i32);
            } else if (*ent).health
                < (*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] * 2i32
            {
                (*ent).health += 5i32;
                if (*ent).health
                    > (*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] * 2i32
                {
                    (*ent).health =
                        (*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] * 2i32
                }
                G_AddEvent(ent, EV_POWERUP_REGEN as libc::c_int, 0i32);
            }
        } else if (*ent).health > (*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] {
            (*ent).health -= 1
        }
        if (*client).ps.stats[STAT_ARMOR as libc::c_int as usize]
            > (*client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize]
        {
            (*client).ps.stats[STAT_ARMOR as libc::c_int as usize] -= 1
        }
    }
}
//==============================================================
/*
==============
ClientImpacts
==============
*/
#[no_mangle]
pub unsafe extern "C" fn ClientImpacts(mut ent: *mut gentity_t, mut pm1: *mut pmove_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut trace: trace_t = trace_t {
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
    let mut other: *mut gentity_t = 0 as *mut gentity_t;
    memset(
        &mut trace as *mut trace_t as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<trace_t>() as libc::c_ulong,
    );
    i = 0i32;
    while i < (*pm1).numtouch {
        j = 0i32;
        while j < i {
            if (*pm1).touchents[j as usize] == (*pm1).touchents[i as usize] {
                break;
            }
            j += 1
        }
        if !(j != i) {
            // duplicated
            other = &mut g_entities[(*pm1).touchents[i as usize] as usize] as *mut gentity_t;
            if 0 != (*ent).r.svFlags & 0x8i32 && (*ent).touch.is_some() {
                (*ent).touch.expect("non-null function pointer")(ent, other, &mut trace);
            }
            if !(*other).touch.is_none() {
                (*other).touch.expect("non-null function pointer")(other, ent, &mut trace);
            }
        }
        i += 1
    }
}
/*
================
ClientEvents

Events will be passed on to the clients for presentation,
but any server game effects are handled here
================
*/
#[no_mangle]
pub unsafe extern "C" fn ClientEvents(mut ent: *mut gentity_t, mut oldEventSequence: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut event: libc::c_int = 0;
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut damage: libc::c_int = 0;
    let mut origin: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    //	qboolean	fired;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut drop_0: *mut gentity_t = 0 as *mut gentity_t;
    client = (*ent).client;
    if oldEventSequence < (*client).ps.eventSequence - 2i32 {
        oldEventSequence = (*client).ps.eventSequence - 2i32
    }
    i = oldEventSequence;
    while i < (*client).ps.eventSequence {
        event = (*client).ps.events[(i & 2i32 - 1i32) as usize];
        match event {
            11 | 12 => {
                if !((*ent).s.eType != ET_PLAYER as libc::c_int) {
                    // not in the player model
                    if !(0 != g_dmflags.integer & 8i32) {
                        if event == EV_FALL_FAR as libc::c_int {
                            damage = 10i32
                        } else {
                            damage = 5i32
                        }
                        (*ent).pain_debounce_time = level.time + 200i32;
                        G_Damage(
                            ent,
                            0 as *mut gentity_t,
                            0 as *mut gentity_t,
                            0 as *mut vec_t,
                            0 as *mut vec_t,
                            damage,
                            0i32,
                            MOD_FALLING as libc::c_int,
                        );
                    }
                }
            }
            23 => {
                FireWeapon(ent);
            }
            25 => {
                item = 0 as *mut gitem_t;
                j = 0i32;
                if 0 != (*(*ent).client).ps.powerups[PW_REDFLAG as libc::c_int as usize] {
                    item = BG_FindItemForPowerup(PW_REDFLAG);
                    j = PW_REDFLAG as libc::c_int
                } else if 0 != (*(*ent).client).ps.powerups[PW_BLUEFLAG as libc::c_int as usize] {
                    item = BG_FindItemForPowerup(PW_BLUEFLAG);
                    j = PW_BLUEFLAG as libc::c_int
                } else if 0 != (*(*ent).client).ps.powerups[PW_NEUTRALFLAG as libc::c_int as usize]
                {
                    item = BG_FindItemForPowerup(PW_NEUTRALFLAG);
                    j = PW_NEUTRALFLAG as libc::c_int
                }
                if !item.is_null() {
                    drop_0 = Drop_Item(ent, item, 0i32 as libc::c_float);
                    (*drop_0).count =
                        ((*(*ent).client).ps.powerups[j as usize] - level.time) / 1000i32;
                    if (*drop_0).count < 1i32 {
                        (*drop_0).count = 1i32
                    }
                    (*(*ent).client).ps.powerups[j as usize] = 0i32
                }
                SelectSpawnPoint(
                    (*(*ent).client).ps.origin.as_mut_ptr(),
                    origin.as_mut_ptr(),
                    angles.as_mut_ptr(),
                    qfalse,
                );
                TeleportPlayer(ent, origin.as_mut_ptr(), angles.as_mut_ptr());
            }
            26 => {
                (*ent).health =
                    (*(*ent).client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize] + 25i32
            }
            _ => {}
        }
        i += 1
    }
}
/*
==============
SendPendingPredictableEvents
==============
*/
#[no_mangle]
pub unsafe extern "C" fn SendPendingPredictableEvents(mut ps: *mut playerState_t) {
    let mut t: *mut gentity_t = 0 as *mut gentity_t;
    let mut event: libc::c_int = 0;
    let mut seq: libc::c_int = 0;
    let mut extEvent: libc::c_int = 0;
    let mut number: libc::c_int = 0;
    if (*ps).entityEventSequence < (*ps).eventSequence {
        seq = (*ps).entityEventSequence & 2i32 - 1i32;
        event = (*ps).events[seq as usize] | ((*ps).entityEventSequence & 3i32) << 8i32;
        extEvent = (*ps).externalEvent;
        (*ps).externalEvent = 0i32;
        t = G_TempEntity((*ps).origin.as_mut_ptr(), event);
        number = (*t).s.number;
        BG_PlayerStateToEntityState(ps, &mut (*t).s, qtrue);
        (*t).s.number = number;
        (*t).s.eType = ET_EVENTS as libc::c_int + event;
        (*t).s.eFlags |= 0x10i32;
        (*t).s.otherEntityNum = (*ps).clientNum;
        (*t).r.svFlags |= 0x800i32;
        (*t).r.singleClient = (*ps).clientNum;
        (*ps).externalEvent = extEvent
    };
}
/*
=================
ClientInactivityTimer

Returns qfalse if the client is dropped
=================
*/
#[no_mangle]
pub unsafe extern "C" fn ClientInactivityTimer(mut client: *mut gclient_t) -> qboolean {
    if 0 == g_inactivity.integer {
        (*client).inactivityTime = level.time + 60i32 * 1000i32;
        (*client).inactivityWarning = qfalse
    } else if 0 != (*client).pers.cmd.forwardmove as libc::c_int
        || 0 != (*client).pers.cmd.rightmove as libc::c_int
        || 0 != (*client).pers.cmd.upmove as libc::c_int
        || 0 != (*client).pers.cmd.buttons & 1i32
    {
        (*client).inactivityTime = level.time + g_inactivity.integer * 1000i32;
        (*client).inactivityWarning = qfalse
    } else if 0 == (*client).pers.localClient as u64 {
        if level.time > (*client).inactivityTime {
            trap_DropClient(
                client.wrapping_offset_from(level.clients) as libc::c_long as libc::c_int,
                b"Dropped due to inactivity\x00" as *const u8 as *const libc::c_char,
            );
            return qfalse;
        }
        if level.time > (*client).inactivityTime - 10000i32
            && 0 == (*client).inactivityWarning as u64
        {
            (*client).inactivityWarning = qtrue;
            trap_SendServerCommand(
                client.wrapping_offset_from(level.clients) as libc::c_long as libc::c_int,
                b"cp \"Ten seconds until inactivity drop!\n\"\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    return qtrue;
}
/*
=================
SpectatorThink
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SpectatorThink(mut ent: *mut gentity_t, mut ucmd: *mut usercmd_t) {
    let mut pm1: pmove_t = pmove_t {
        ps: 0 as *mut playerState_t,
        cmd: usercmd_s {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        },
        tracemask: 0,
        debugLevel: 0,
        noFootsteps: qfalse,
        gauntletHit: qfalse,
        framecount: 0,
        numtouch: 0,
        touchents: [0; 32],
        mins: [0.; 3],
        maxs: [0.; 3],
        watertype: 0,
        waterlevel: 0,
        xyspeed: 0.,
        pmove_fixed: 0,
        pmove_msec: 0,
        trace: None,
        pointcontents: None,
    };
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    client = (*ent).client;
    if (*client).sess.spectatorState as libc::c_uint
        != SPECTATOR_FOLLOW as libc::c_int as libc::c_uint
        || 0 == (*client).ps.pm_flags & 4096i32
    {
        if (*client).sess.spectatorState as libc::c_uint
            == SPECTATOR_FREE as libc::c_int as libc::c_uint
        {
            if 0 != (*client).noclip as u64 {
                (*client).ps.pm_type = PM_NOCLIP as libc::c_int
            } else {
                (*client).ps.pm_type = PM_SPECTATOR as libc::c_int
            }
        } else {
            (*client).ps.pm_type = PM_FREEZE as libc::c_int
        }
        (*client).ps.speed = 400i32;
        memset(
            &mut pm1 as *mut pmove_t as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<pmove_t>() as libc::c_ulong,
        );
        pm1.ps = &mut (*client).ps;
        pm1.cmd = *ucmd;
        pm1.tracemask = (1i32 | 0x10000i32 | 0x2000000i32) & !0x2000000i32;
        pm1.trace = Some(trap_Trace);
        pm1.pointcontents = Some(trap_PointContents);
        Pmove(&mut pm1);
        (*ent).s.origin[0usize] = (*client).ps.origin[0usize];
        (*ent).s.origin[1usize] = (*client).ps.origin[1usize];
        (*ent).s.origin[2usize] = (*client).ps.origin[2usize];
        G_TouchTriggers(ent);
        trap_UnlinkEntity(ent);
    }
    (*client).oldbuttons = (*client).buttons;
    (*client).buttons = (*ucmd).buttons;
    if 0 != (*client).buttons & 1i32 && 0 == (*client).oldbuttons & 1i32 {
        Cmd_FollowCycle_f(ent, 1i32);
    };
}
/*
====================
ClientIntermissionThink
====================
*/
#[no_mangle]
pub unsafe extern "C" fn ClientIntermissionThink(mut client: *mut gclient_t) {
    (*client).ps.eFlags &= !0x1000i32;
    (*client).ps.eFlags &= !0x100i32;
    (*client).oldbuttons = (*client).buttons;
    (*client).buttons = (*client).pers.cmd.buttons;
    if 0 != (*client).buttons & (1i32 | 4i32) & ((*client).oldbuttons ^ (*client).buttons) {
        (*client).readyToExit = qtrue
    };
}
#[no_mangle]
pub unsafe extern "C" fn ClientEndFrame(mut ent: *mut gentity_t) {
    let mut i: libc::c_int = 0;
    if (*(*ent).client).sess.sessionTeam as libc::c_uint
        == TEAM_SPECTATOR as libc::c_int as libc::c_uint
    {
        SpectatorClientEndFrame(ent);
        return;
    }
    i = 0i32;
    while i < 16i32 {
        if (*(*ent).client).ps.powerups[i as usize] < level.time {
            (*(*ent).client).ps.powerups[i as usize] = 0i32
        }
        i += 1
    }
    if 0 != level.intermissiontime {
        return;
    }
    P_WorldEffects(ent);
    P_DamageFeedback(ent);
    if level.time - (*(*ent).client).lastCmdTime > 1000i32 {
        (*(*ent).client).ps.eFlags |= 0x2000i32
    } else {
        (*(*ent).client).ps.eFlags &= !0x2000i32
    }
    (*(*ent).client).ps.stats[STAT_HEALTH as libc::c_int as usize] = (*ent).health;
    G_SetClientSound(ent);
    if 0 != g_smoothClients.integer {
        BG_PlayerStateToEntityStateExtraPolate(
            &mut (*(*ent).client).ps,
            &mut (*ent).s,
            (*(*ent).client).ps.commandTime,
            qtrue,
        );
    } else {
        BG_PlayerStateToEntityState(&mut (*(*ent).client).ps, &mut (*ent).s, qtrue);
    }
    SendPendingPredictableEvents(&mut (*(*ent).client).ps);
}
/*
===============
G_SetClientSound
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_SetClientSound(mut ent: *mut gentity_t) {
    if 0 != (*ent).waterlevel && 0 != (*ent).watertype & (8i32 | 16i32) {
        (*(*ent).client).ps.loopSound = level.snd_fry
    } else {
        (*(*ent).client).ps.loopSound = 0i32
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
===============
G_DamageFeedback

Called just before a snapshot is sent to the given player.
Totals up all damage and generates both the player_state_t
damage values to that client for pain blends and kicks, and
global pain sound events for all clients.
===============
*/
#[no_mangle]
pub unsafe extern "C" fn P_DamageFeedback(mut player: *mut gentity_t) {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut count: libc::c_float = 0.;
    let mut angles: vec3_t = [0.; 3];
    client = (*player).client;
    if (*client).ps.pm_type == PM_DEAD as libc::c_int {
        return;
    }
    count = ((*client).damage_blood + (*client).damage_armor) as libc::c_float;
    if count == 0i32 as libc::c_float {
        return;
    }
    if count > 255i32 as libc::c_float {
        count = 255i32 as libc::c_float
    }
    if 0 != (*client).damage_fromWorld as u64 {
        (*client).ps.damagePitch = 255i32;
        (*client).ps.damageYaw = 255i32;
        (*client).damage_fromWorld = qfalse
    } else {
        vectoangles(
            (*client).damage_from.as_mut_ptr() as *const vec_t,
            angles.as_mut_ptr(),
        );
        (*client).ps.damagePitch =
            (angles[0usize] as libc::c_double / 360.0f64 * 256i32 as libc::c_double) as libc::c_int;
        (*client).ps.damageYaw =
            (angles[1usize] as libc::c_double / 360.0f64 * 256i32 as libc::c_double) as libc::c_int
    }
    if level.time > (*player).pain_debounce_time && 0 == (*player).flags & 0x10i32 {
        (*player).pain_debounce_time = level.time + 700i32;
        G_AddEvent(player, EV_PAIN as libc::c_int, (*player).health);
        (*client).ps.damageEvent += 1
    }
    (*client).ps.damageCount = count as libc::c_int;
    (*client).damage_blood = 0i32;
    (*client).damage_armor = 0i32;
    (*client).damage_knockback = 0i32;
}
/*
=============
P_WorldEffects

Check for lava / slime contents and drowning
=============
*/
#[no_mangle]
pub unsafe extern "C" fn P_WorldEffects(mut ent: *mut gentity_t) {
    let mut envirosuit: qboolean = qfalse;
    let mut waterlevel: libc::c_int = 0;
    if 0 != (*(*ent).client).noclip as u64 {
        (*(*ent).client).airOutTime = level.time + 12000i32;
        return;
    }
    waterlevel = (*ent).waterlevel;
    envirosuit = ((*(*ent).client).ps.powerups[PW_BATTLESUIT as libc::c_int as usize] > level.time)
        as libc::c_int as qboolean;
    if waterlevel == 3i32 {
        if 0 != envirosuit as u64 {
            (*(*ent).client).airOutTime = level.time + 10000i32
        }
        if (*(*ent).client).airOutTime < level.time {
            (*(*ent).client).airOutTime += 1000i32;
            if (*ent).health > 0i32 {
                (*ent).damage += 2i32;
                if (*ent).damage > 15i32 {
                    (*ent).damage = 15i32
                }
                (*ent).pain_debounce_time = level.time + 200i32;
                G_Damage(
                    ent,
                    0 as *mut gentity_t,
                    0 as *mut gentity_t,
                    0 as *mut vec_t,
                    0 as *mut vec_t,
                    (*ent).damage,
                    0x2i32,
                    MOD_WATER as libc::c_int,
                );
            }
        }
    } else {
        (*(*ent).client).airOutTime = level.time + 12000i32;
        (*ent).damage = 2i32
    }
    if 0 != waterlevel && 0 != (*ent).watertype & (8i32 | 16i32) {
        if (*ent).health > 0i32 && (*ent).pain_debounce_time <= level.time {
            if 0 != envirosuit as u64 {
                G_AddEvent(ent, EV_POWERUP_BATTLESUIT as libc::c_int, 0i32);
            } else {
                if 0 != (*ent).watertype & 8i32 {
                    G_Damage(
                        ent,
                        0 as *mut gentity_t,
                        0 as *mut gentity_t,
                        0 as *mut vec_t,
                        0 as *mut vec_t,
                        30i32 * waterlevel,
                        0i32,
                        MOD_LAVA as libc::c_int,
                    );
                }
                if 0 != (*ent).watertype & 16i32 {
                    G_Damage(
                        ent,
                        0 as *mut gentity_t,
                        0 as *mut gentity_t,
                        0 as *mut vec_t,
                        0 as *mut vec_t,
                        10i32 * waterlevel,
                        0i32,
                        MOD_SLIME as libc::c_int,
                    );
                }
            }
        }
    };
}
/*
==================
SpectatorClientEndFrame

==================
*/
#[no_mangle]
pub unsafe extern "C" fn SpectatorClientEndFrame(mut ent: *mut gentity_t) {
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    if (*(*ent).client).sess.spectatorState as libc::c_uint
        == SPECTATOR_FOLLOW as libc::c_int as libc::c_uint
    {
        let mut clientNum: libc::c_int = 0;
        let mut flags: libc::c_int = 0;
        clientNum = (*(*ent).client).sess.spectatorClient;
        if clientNum == -1i32 {
            clientNum = level.follow1
        } else if clientNum == -2i32 {
            clientNum = level.follow2
        }
        if clientNum >= 0i32 {
            cl = &mut *level.clients.offset(clientNum as isize) as *mut gclient_s;
            if (*cl).pers.connected as libc::c_uint == CON_CONNECTED as libc::c_int as libc::c_uint
                && (*cl).sess.sessionTeam as libc::c_uint
                    != TEAM_SPECTATOR as libc::c_int as libc::c_uint
            {
                flags = (*cl).ps.eFlags & !(0x4000i32 | 0x80000i32)
                    | (*(*ent).client).ps.eFlags & (0x4000i32 | 0x80000i32);
                (*(*ent).client).ps = (*cl).ps;
                (*(*ent).client).ps.pm_flags |= 4096i32;
                (*(*ent).client).ps.eFlags = flags;
                return;
            }
        }
        if 0 != (*(*ent).client).ps.pm_flags & 4096i32 {
            if (*(*ent).client).sess.spectatorClient >= 0i32 {
                (*(*ent).client).sess.spectatorState = SPECTATOR_FREE
            }
            ClientBegin(
                (*ent).client.wrapping_offset_from(level.clients) as libc::c_long as libc::c_int,
            );
        }
    }
    if (*(*ent).client).sess.spectatorState as libc::c_uint
        == SPECTATOR_SCOREBOARD as libc::c_int as libc::c_uint
    {
        (*(*ent).client).ps.pm_flags |= 8192i32
    } else {
        (*(*ent).client).ps.pm_flags &= !8192i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn G_RunClient(mut ent: *mut gentity_t) {
    if 0 == (*ent).r.svFlags & 0x8i32 && 0 == g_synchronousClients.integer {
        return;
    }
    (*(*ent).client).pers.cmd.serverTime = level.time;
    ClientThink_real(ent);
}
