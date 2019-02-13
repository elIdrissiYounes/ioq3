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
    gitem_s, gitem_t, itemType_t, powerup_t, team_t, unnamed, unnamed_0, unnamed_1, unnamed_2,
    unnamed_3, unnamed_4, unnamed_5, unnamed_6, unnamed_7, weapon_t, BOTH_DEAD1, BOTH_DEAD2,
    BOTH_DEAD3, BOTH_DEATH1, BOTH_DEATH2, BOTH_DEATH3, ET_BEAM, ET_EVENTS, ET_GENERAL, ET_GRAPPLE,
    ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER, ET_SPEAKER,
    ET_TEAM, ET_TELEPORT_TRIGGER, EV_BULLET, EV_BULLET_HIT_FLESH, EV_BULLET_HIT_WALL,
    EV_CHANGE_WEAPON, EV_DEATH1, EV_DEATH2, EV_DEATH3, EV_DEBUG_LINE, EV_FALL_FAR, EV_FALL_MEDIUM,
    EV_FALL_SHORT, EV_FIRE_WEAPON, EV_FOOTSPLASH, EV_FOOTSTEP, EV_FOOTSTEP_METAL, EV_FOOTWADE,
    EV_GENERAL_SOUND, EV_GIB_PLAYER, EV_GLOBAL_ITEM_PICKUP, EV_GLOBAL_SOUND, EV_GLOBAL_TEAM_SOUND,
    EV_GRENADE_BOUNCE, EV_INVUL_IMPACT, EV_ITEM_PICKUP, EV_ITEM_POP, EV_ITEM_RESPAWN, EV_JUICED,
    EV_JUMP, EV_JUMP_PAD, EV_KAMIKAZE, EV_LIGHTNINGBOLT, EV_MISSILE_HIT, EV_MISSILE_MISS,
    EV_MISSILE_MISS_METAL, EV_NOAMMO, EV_NONE, EV_OBELISKEXPLODE, EV_OBELISKPAIN, EV_OBITUARY,
    EV_PAIN, EV_PLAYER_TELEPORT_IN, EV_PLAYER_TELEPORT_OUT, EV_POWERUP_BATTLESUIT, EV_POWERUP_QUAD,
    EV_POWERUP_REGEN, EV_PROXIMITY_MINE_STICK, EV_PROXIMITY_MINE_TRIGGER, EV_RAILTRAIL,
    EV_SCOREPLUM, EV_SHOTGUN, EV_STEP_12, EV_STEP_16, EV_STEP_4, EV_STEP_8, EV_STOPLOOPINGSOUND,
    EV_SWIM, EV_TAUNT, EV_TAUNT_FOLLOWME, EV_TAUNT_GETFLAG, EV_TAUNT_GUARDBASE, EV_TAUNT_NO,
    EV_TAUNT_PATROL, EV_TAUNT_YES, EV_USE_ITEM0, EV_USE_ITEM1, EV_USE_ITEM10, EV_USE_ITEM11,
    EV_USE_ITEM12, EV_USE_ITEM13, EV_USE_ITEM14, EV_USE_ITEM15, EV_USE_ITEM2, EV_USE_ITEM3,
    EV_USE_ITEM4, EV_USE_ITEM5, EV_USE_ITEM6, EV_USE_ITEM7, EV_USE_ITEM8, EV_USE_ITEM9,
    EV_WATER_CLEAR, EV_WATER_LEAVE, EV_WATER_TOUCH, EV_WATER_UNDER, FLAG_RUN, FLAG_STAND,
    FLAG_STAND2RUN, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER, GT_MAX_GAME_TYPE, GT_OBELISK,
    GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR, IT_BAD, IT_HEALTH, IT_HOLDABLE,
    IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON, LEGS_BACK, LEGS_BACKCR, LEGS_BACKWALK,
    LEGS_IDLE, LEGS_IDLECR, LEGS_JUMP, LEGS_JUMPB, LEGS_LAND, LEGS_LANDB, LEGS_RUN, LEGS_SWIM,
    LEGS_TURN, LEGS_WALK, LEGS_WALKCR, MAX_ANIMATIONS, MAX_TOTALANIMATIONS, MOD_BFG,
    MOD_BFG_SPLASH, MOD_CRUSH, MOD_FALLING, MOD_GAUNTLET, MOD_GRAPPLE, MOD_GRENADE,
    MOD_GRENADE_SPLASH, MOD_LAVA, MOD_LIGHTNING, MOD_MACHINEGUN, MOD_PLASMA, MOD_PLASMA_SPLASH,
    MOD_RAILGUN, MOD_ROCKET, MOD_ROCKET_SPLASH, MOD_SHOTGUN, MOD_SLIME, MOD_SUICIDE,
    MOD_TARGET_LASER, MOD_TELEFRAG, MOD_TRIGGER_HURT, MOD_UNKNOWN, MOD_WATER, PERS_ASSIST_COUNT,
    PERS_ATTACKEE_ARMOR, PERS_ATTACKER, PERS_CAPTURES, PERS_DEFEND_COUNT, PERS_EXCELLENT_COUNT,
    PERS_GAUNTLET_FRAG_COUNT, PERS_HITS, PERS_IMPRESSIVE_COUNT, PERS_KILLED, PERS_PLAYEREVENTS,
    PERS_RANK, PERS_SCORE, PERS_SPAWN_COUNT, PERS_TEAM, PM_DEAD, PM_FREEZE, PM_INTERMISSION,
    PM_NOCLIP, PM_NORMAL, PM_SPECTATOR, PM_SPINTERMISSION, PW_AMMOREGEN, PW_BATTLESUIT,
    PW_BLUEFLAG, PW_DOUBLER, PW_FLIGHT, PW_GUARD, PW_HASTE, PW_INVIS, PW_INVULNERABILITY,
    PW_NEUTRALFLAG, PW_NONE, PW_NUM_POWERUPS, PW_QUAD, PW_REDFLAG, PW_REGEN, PW_SCOUT, STAT_ARMOR,
    STAT_CLIENTS_READY, STAT_DEAD_YAW, STAT_HEALTH, STAT_HOLDABLE_ITEM, STAT_MAX_HEALTH,
    STAT_WEAPONS, TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
    TORSO_AFFIRMATIVE, TORSO_ATTACK, TORSO_ATTACK2, TORSO_DROP, TORSO_FOLLOWME, TORSO_GESTURE,
    TORSO_GETFLAG, TORSO_GUARDBASE, TORSO_NEGATIVE, TORSO_PATROL, TORSO_RAISE, TORSO_STAND,
    TORSO_STAND2, WEAPON_DROPPING, WEAPON_FIRING, WEAPON_RAISING, WEAPON_READY, WP_BFG,
    WP_GAUNTLET, WP_GRAPPLING_HOOK, WP_GRENADE_LAUNCHER, WP_LIGHTNING, WP_MACHINEGUN, WP_NONE,
    WP_NUM_WEAPONS, WP_PLASMAGUN, WP_RAILGUN, WP_ROCKET_LAUNCHER, WP_SHOTGUN,
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
use g_items::{
    ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem, G_SpawnItem,
    RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
};
use g_local_h::{
    clientConnected_t, clientPersistant_t, clientSession_t, gclient_s, gclient_t, gentity_s,
    gentity_t, level_locals_t, moverState_t, playerTeamStateState_t, playerTeamState_t,
    spectatorState_t, trap_EntitiesInBox, trap_LinkEntity, trap_PointContents, trap_Trace,
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
    SP_team_CTF_redspawn, Team_CheckDroppedItem, Team_CheckHurtCarrier, Team_FragBonuses,
    Team_ReturnFlag,
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
use g_variadic_h::{G_LogPrintf, G_Printf};
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
use stdlib::{ceil, memset, sqrt, strcmp};

unsafe extern "C" fn VectorLength(mut v: *const vec_t) -> vec_t {
    return sqrt(
        (*v.offset(0isize) * *v.offset(0isize)
            + *v.offset(1isize) * *v.offset(1isize)
            + *v.offset(2isize) * *v.offset(2isize)) as libc::c_double,
    ) as vec_t;
}
//
// g_combat.c
//
#[no_mangle]
pub unsafe extern "C" fn CanDamage(mut targ: *mut gentity_t, mut origin: *mut vec_t) -> qboolean {
    let mut dest: vec3_t = [0.; 3];
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
    let mut midpoint: vec3_t = [0.; 3];
    let mut offsetmins: vec3_t = [-15i32 as vec_t, -15i32 as vec_t, -15i32 as vec_t];
    let mut offsetmaxs: vec3_t = [15i32 as vec_t, 15i32 as vec_t, 15i32 as vec_t];
    midpoint[0usize] = (*targ).r.absmin[0usize] + (*targ).r.absmax[0usize];
    midpoint[1usize] = (*targ).r.absmin[1usize] + (*targ).r.absmax[1usize];
    midpoint[2usize] = (*targ).r.absmin[2usize] + (*targ).r.absmax[2usize];
    midpoint[0usize] = (midpoint[0usize] as libc::c_double * 0.5f64) as vec_t;
    midpoint[1usize] = (midpoint[1usize] as libc::c_double * 0.5f64) as vec_t;
    midpoint[2usize] = (midpoint[2usize] as libc::c_double * 0.5f64) as vec_t;
    dest[0usize] = midpoint[0usize];
    dest[1usize] = midpoint[1usize];
    dest[2usize] = midpoint[2usize];
    trap_Trace(
        &mut tr,
        origin as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        dest.as_mut_ptr() as *const vec_t,
        (1i32 << 10i32) - 1i32,
        1i32,
    );
    if tr.fraction as libc::c_double == 1.0f64 || tr.entityNum == (*targ).s.number {
        return qtrue;
    }
    dest[0usize] = midpoint[0usize];
    dest[1usize] = midpoint[1usize];
    dest[2usize] = midpoint[2usize];
    dest[0usize] += offsetmaxs[0usize];
    dest[1usize] += offsetmaxs[1usize];
    dest[2usize] += offsetmaxs[2usize];
    trap_Trace(
        &mut tr,
        origin as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        dest.as_mut_ptr() as *const vec_t,
        (1i32 << 10i32) - 1i32,
        1i32,
    );
    if tr.fraction as libc::c_double == 1.0f64 {
        return qtrue;
    }
    dest[0usize] = midpoint[0usize];
    dest[1usize] = midpoint[1usize];
    dest[2usize] = midpoint[2usize];
    dest[0usize] += offsetmaxs[0usize];
    dest[1usize] += offsetmins[1usize];
    dest[2usize] += offsetmaxs[2usize];
    trap_Trace(
        &mut tr,
        origin as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        dest.as_mut_ptr() as *const vec_t,
        (1i32 << 10i32) - 1i32,
        1i32,
    );
    if tr.fraction as libc::c_double == 1.0f64 {
        return qtrue;
    }
    dest[0usize] = midpoint[0usize];
    dest[1usize] = midpoint[1usize];
    dest[2usize] = midpoint[2usize];
    dest[0usize] += offsetmins[0usize];
    dest[1usize] += offsetmaxs[1usize];
    dest[2usize] += offsetmaxs[2usize];
    trap_Trace(
        &mut tr,
        origin as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        dest.as_mut_ptr() as *const vec_t,
        (1i32 << 10i32) - 1i32,
        1i32,
    );
    if tr.fraction as libc::c_double == 1.0f64 {
        return qtrue;
    }
    dest[0usize] = midpoint[0usize];
    dest[1usize] = midpoint[1usize];
    dest[2usize] = midpoint[2usize];
    dest[0usize] += offsetmins[0usize];
    dest[1usize] += offsetmins[1usize];
    dest[2usize] += offsetmaxs[2usize];
    trap_Trace(
        &mut tr,
        origin as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        dest.as_mut_ptr() as *const vec_t,
        (1i32 << 10i32) - 1i32,
        1i32,
    );
    if tr.fraction as libc::c_double == 1.0f64 {
        return qtrue;
    }
    dest[0usize] = midpoint[0usize];
    dest[1usize] = midpoint[1usize];
    dest[2usize] = midpoint[2usize];
    dest[0usize] += offsetmaxs[0usize];
    dest[1usize] += offsetmaxs[1usize];
    dest[2usize] += offsetmins[2usize];
    trap_Trace(
        &mut tr,
        origin as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        dest.as_mut_ptr() as *const vec_t,
        (1i32 << 10i32) - 1i32,
        1i32,
    );
    if tr.fraction as libc::c_double == 1.0f64 {
        return qtrue;
    }
    dest[0usize] = midpoint[0usize];
    dest[1usize] = midpoint[1usize];
    dest[2usize] = midpoint[2usize];
    dest[0usize] += offsetmaxs[0usize];
    dest[1usize] += offsetmins[1usize];
    dest[2usize] += offsetmins[2usize];
    trap_Trace(
        &mut tr,
        origin as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        dest.as_mut_ptr() as *const vec_t,
        (1i32 << 10i32) - 1i32,
        1i32,
    );
    if tr.fraction as libc::c_double == 1.0f64 {
        return qtrue;
    }
    dest[0usize] = midpoint[0usize];
    dest[1usize] = midpoint[1usize];
    dest[2usize] = midpoint[2usize];
    dest[0usize] += offsetmins[0usize];
    dest[1usize] += offsetmaxs[1usize];
    dest[2usize] += offsetmins[2usize];
    trap_Trace(
        &mut tr,
        origin as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        dest.as_mut_ptr() as *const vec_t,
        (1i32 << 10i32) - 1i32,
        1i32,
    );
    if tr.fraction as libc::c_double == 1.0f64 {
        return qtrue;
    }
    dest[0usize] = midpoint[0usize];
    dest[1usize] = midpoint[1usize];
    dest[2usize] = midpoint[2usize];
    dest[0usize] += offsetmins[0usize];
    dest[1usize] += offsetmins[1usize];
    dest[2usize] += offsetmins[2usize];
    trap_Trace(
        &mut tr,
        origin as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        vec3_origin.as_mut_ptr() as *const vec_t,
        dest.as_mut_ptr() as *const vec_t,
        (1i32 << 10i32) - 1i32,
        1i32,
    );
    if tr.fraction as libc::c_double == 1.0f64 {
        return qtrue;
    }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn G_Damage(
    mut targ: *mut gentity_t,
    mut inflictor: *mut gentity_t,
    mut attacker: *mut gentity_t,
    mut dir: *mut vec_t,
    mut point: *mut vec_t,
    mut damage: libc::c_int,
    mut dflags: libc::c_int,
    mut mod_0: libc::c_int,
) {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut take: libc::c_int = 0;
    let mut asave: libc::c_int = 0;
    let mut knockback: libc::c_int = 0;
    let mut max: libc::c_int = 0;
    if 0 == (*targ).takedamage as u64 {
        return;
    }
    if 0 != level.intermissionQueued {
        return;
    }
    if inflictor.is_null() {
        inflictor = &mut g_entities[((1i32 << 10i32) - 2i32) as usize] as *mut gentity_t
    }
    if attacker.is_null() {
        attacker = &mut g_entities[((1i32 << 10i32) - 2i32) as usize] as *mut gentity_t
    }
    if (*targ).s.eType == ET_MOVER as libc::c_int {
        if (*targ).use_0.is_some()
            && (*targ).moverState as libc::c_uint == MOVER_POS1 as libc::c_int as libc::c_uint
        {
            (*targ).use_0.expect("non-null function pointer")(targ, inflictor, attacker);
        }
        return;
    }
    if !(*attacker).client.is_null() && attacker != targ {
        max = (*(*attacker).client).ps.stats[STAT_MAX_HEALTH as libc::c_int as usize];
        damage = damage * max / 100i32
    }
    client = (*targ).client;
    if !client.is_null() {
        if 0 != (*client).noclip as u64 {
            return;
        }
    }
    if dir.is_null() {
        dflags |= 0x4i32
    } else {
        VectorNormalize(dir);
    }
    knockback = damage;
    if knockback > 200i32 {
        knockback = 200i32
    }
    if 0 != (*targ).flags & 0x800i32 {
        knockback = 0i32
    }
    if 0 != dflags & 0x4i32 {
        knockback = 0i32
    }
    if 0 != knockback && !(*targ).client.is_null() {
        let mut kvel: vec3_t = [0.; 3];
        let mut mass: libc::c_float = 0.;
        mass = 200i32 as libc::c_float;
        kvel[0usize] =
            *dir.offset(0isize) * (g_knockback.value * knockback as libc::c_float / mass);
        kvel[1usize] =
            *dir.offset(1isize) * (g_knockback.value * knockback as libc::c_float / mass);
        kvel[2usize] =
            *dir.offset(2isize) * (g_knockback.value * knockback as libc::c_float / mass);
        (*(*targ).client).ps.velocity[0usize] =
            (*(*targ).client).ps.velocity[0usize] + kvel[0usize];
        (*(*targ).client).ps.velocity[1usize] =
            (*(*targ).client).ps.velocity[1usize] + kvel[1usize];
        (*(*targ).client).ps.velocity[2usize] =
            (*(*targ).client).ps.velocity[2usize] + kvel[2usize];
        if 0 == (*(*targ).client).ps.pm_time {
            let mut t: libc::c_int = 0;
            t = knockback * 2i32;
            if t < 50i32 {
                t = 50i32
            }
            if t > 200i32 {
                t = 200i32
            }
            (*(*targ).client).ps.pm_time = t;
            (*(*targ).client).ps.pm_flags |= 64i32
        }
    }
    if 0 == dflags & 0x8i32 {
        if targ != attacker && 0 != OnSameTeam(targ, attacker) as libc::c_uint {
            if 0 == g_friendlyFire.integer {
                return;
            }
        }
        if 0 != (*targ).flags & 0x10i32 {
            return;
        }
    }
    if !client.is_null() && 0 != (*client).ps.powerups[PW_BATTLESUIT as libc::c_int as usize] {
        G_AddEvent(targ, EV_POWERUP_BATTLESUIT as libc::c_int, 0i32);
        if 0 != dflags & 0x1i32 || mod_0 == MOD_FALLING as libc::c_int {
            return;
        }
        damage = (damage as libc::c_double * 0.5f64) as libc::c_int
    }
    if !(*attacker).client.is_null()
        && !client.is_null()
        && targ != attacker
        && (*targ).health > 0i32
        && (*targ).s.eType != ET_MISSILE as libc::c_int
        && (*targ).s.eType != ET_GENERAL as libc::c_int
    {
        if 0 != OnSameTeam(targ, attacker) as u64 {
            (*(*attacker).client).ps.persistant[PERS_HITS as libc::c_int as usize] -= 1
        } else {
            (*(*attacker).client).ps.persistant[PERS_HITS as libc::c_int as usize] += 1
        }
        (*(*attacker).client).ps.persistant[PERS_ATTACKEE_ARMOR as libc::c_int as usize] =
            (*targ).health << 8i32 | (*client).ps.stats[STAT_ARMOR as libc::c_int as usize]
    }
    if targ == attacker {
        damage = (damage as libc::c_double * 0.5f64) as libc::c_int
    }
    if damage < 1i32 {
        damage = 1i32
    }
    take = damage;
    asave = CheckArmor(targ, take, dflags);
    take -= asave;
    if 0 != g_debugDamage.integer {
        G_Printf(
            b"%i: client:%i health:%i damage:%i armor:%i\n\x00" as *const u8 as *const libc::c_char,
            level.time,
            (*targ).s.number,
            (*targ).health,
            take,
            asave,
        );
    }
    if !client.is_null() {
        if !attacker.is_null() {
            (*client).ps.persistant[PERS_ATTACKER as libc::c_int as usize] = (*attacker).s.number
        } else {
            (*client).ps.persistant[PERS_ATTACKER as libc::c_int as usize] = (1i32 << 10i32) - 2i32
        }
        (*client).damage_armor += asave;
        (*client).damage_blood += take;
        (*client).damage_knockback += knockback;
        if !dir.is_null() {
            (*client).damage_from[0usize] = *dir.offset(0isize);
            (*client).damage_from[1usize] = *dir.offset(1isize);
            (*client).damage_from[2usize] = *dir.offset(2isize);
            (*client).damage_fromWorld = qfalse
        } else {
            (*client).damage_from[0usize] = (*targ).r.currentOrigin[0usize];
            (*client).damage_from[1usize] = (*targ).r.currentOrigin[1usize];
            (*client).damage_from[2usize] = (*targ).r.currentOrigin[2usize];
            (*client).damage_fromWorld = qtrue
        }
    }
    if g_gametype.integer == GT_CTF as libc::c_int {
        Team_CheckHurtCarrier(targ, attacker);
    }
    if !(*targ).client.is_null() {
        (*(*targ).client).lasthurt_client = (*attacker).s.number;
        (*(*targ).client).lasthurt_mod = mod_0
    }
    if 0 != take {
        (*targ).health = (*targ).health - take;
        if !(*targ).client.is_null() {
            (*(*targ).client).ps.stats[STAT_HEALTH as libc::c_int as usize] = (*targ).health
        }
        if (*targ).health <= 0i32 {
            if !client.is_null() {
                (*targ).flags |= 0x800i32
            }
            if (*targ).health < -999i32 {
                (*targ).health = -999i32
            }
            (*targ).enemy = attacker;
            (*targ).die.expect("non-null function pointer")(targ, inflictor, attacker, take, mod_0);
            return;
        } else {
            if (*targ).pain.is_some() {
                (*targ).pain.expect("non-null function pointer")(targ, attacker, take);
            }
        }
    };
}
/*
================
CheckArmor
================
*/
#[no_mangle]
pub unsafe extern "C" fn CheckArmor(
    mut ent: *mut gentity_t,
    mut damage: libc::c_int,
    mut dflags: libc::c_int,
) -> libc::c_int {
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut save: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    if 0 == damage {
        return 0i32;
    }
    client = (*ent).client;
    if client.is_null() {
        return 0i32;
    }
    if 0 != dflags & 0x2i32 {
        return 0i32;
    }
    count = (*client).ps.stats[STAT_ARMOR as libc::c_int as usize];
    save = ceil(damage as libc::c_double * 0.66f64) as libc::c_int;
    if save >= count {
        save = count
    }
    if 0 == save {
        return 0i32;
    }
    (*client).ps.stats[STAT_ARMOR as libc::c_int as usize] -= save;
    return save;
}
#[no_mangle]
pub unsafe extern "C" fn G_RadiusDamage(
    mut origin: *mut vec_t,
    mut attacker: *mut gentity_t,
    mut damage: libc::c_float,
    mut radius: libc::c_float,
    mut ignore: *mut gentity_t,
    mut mod_0: libc::c_int,
) -> qboolean {
    let mut points: libc::c_float = 0.;
    let mut dist: libc::c_float = 0.;
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut entityList: [libc::c_int; 1024] = [0; 1024];
    let mut numListedEntities: libc::c_int = 0;
    let mut mins: vec3_t = [0.; 3];
    let mut maxs: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut hitClient: qboolean = qfalse;
    if radius < 1i32 as libc::c_float {
        radius = 1i32 as libc::c_float
    }
    i = 0i32;
    while i < 3i32 {
        mins[i as usize] = *origin.offset(i as isize) - radius;
        maxs[i as usize] = *origin.offset(i as isize) + radius;
        i += 1
    }
    numListedEntities = trap_EntitiesInBox(
        mins.as_mut_ptr() as *const vec_t,
        maxs.as_mut_ptr() as *const vec_t,
        entityList.as_mut_ptr(),
        1i32 << 10i32,
    );
    e = 0i32;
    while e < numListedEntities {
        ent = &mut g_entities[entityList[e as usize] as usize] as *mut gentity_t;
        if !(ent == ignore) {
            if !(0 == (*ent).takedamage as u64) {
                i = 0i32;
                while i < 3i32 {
                    if *origin.offset(i as isize) < (*ent).r.absmin[i as usize] {
                        v[i as usize] = (*ent).r.absmin[i as usize] - *origin.offset(i as isize)
                    } else if *origin.offset(i as isize) > (*ent).r.absmax[i as usize] {
                        v[i as usize] = *origin.offset(i as isize) - (*ent).r.absmax[i as usize]
                    } else {
                        v[i as usize] = 0i32 as vec_t
                    }
                    i += 1
                }
                dist = VectorLength(v.as_mut_ptr() as *const vec_t);
                if !(dist >= radius) {
                    points = (damage as libc::c_double
                        * (1.0f64 - (dist / radius) as libc::c_double))
                        as libc::c_float;
                    if 0 != CanDamage(ent, origin) as u64 {
                        if 0 != LogAccuracyHit(ent, attacker) as u64 {
                            hitClient = qtrue
                        }
                        dir[0usize] = (*ent).r.currentOrigin[0usize] - *origin.offset(0isize);
                        dir[1usize] = (*ent).r.currentOrigin[1usize] - *origin.offset(1isize);
                        dir[2usize] = (*ent).r.currentOrigin[2usize] - *origin.offset(2isize);
                        dir[2usize] += 24i32 as libc::c_float;
                        G_Damage(
                            ent,
                            0 as *mut gentity_t,
                            attacker,
                            dir.as_mut_ptr(),
                            origin,
                            points as libc::c_int,
                            0x1i32,
                            mod_0,
                        );
                    }
                }
            }
        }
        e += 1
    }
    return hitClient;
}
#[no_mangle]
pub unsafe extern "C" fn body_die(
    mut self_0: *mut gentity_t,
    mut inflictor: *mut gentity_t,
    mut attacker: *mut gentity_t,
    mut damage: libc::c_int,
    mut meansOfDeath: libc::c_int,
) {
    if (*self_0).health > -40i32 {
        return;
    }
    if 0 == g_blood.integer {
        (*self_0).health = -40i32 + 1i32;
        return;
    }
    GibEntity(self_0, 0i32);
}
/*
==================
GibEntity
==================
*/
#[no_mangle]
pub unsafe extern "C" fn GibEntity(mut self_0: *mut gentity_t, mut killer: libc::c_int) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut i: libc::c_int = 0;
    if 0 != (*self_0).s.eFlags & 0x200i32 {
        i = 0i32;
        while i < level.num_entities {
            ent = &mut g_entities[i as usize] as *mut gentity_t;
            if !(0 == (*ent).inuse as u64) {
                if !((*ent).activator != self_0) {
                    if !(0
                        != strcmp(
                            (*ent).classname,
                            b"kamikaze timer\x00" as *const u8 as *const libc::c_char,
                        ))
                    {
                        G_FreeEntity(ent);
                        break;
                    }
                }
            }
            i += 1
        }
    }
    G_AddEvent(self_0, EV_GIB_PLAYER as libc::c_int, killer);
    (*self_0).takedamage = qfalse;
    (*self_0).s.eType = ET_INVISIBLE as libc::c_int;
    (*self_0).r.contents = 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn TossClientItems(mut self_0: *mut gentity_t) {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut weapon: libc::c_int = 0;
    let mut angle: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut drop_0: *mut gentity_t = 0 as *mut gentity_t;
    weapon = (*self_0).s.weapon;
    if weapon == WP_MACHINEGUN as libc::c_int || weapon == WP_GRAPPLING_HOOK as libc::c_int {
        if (*(*self_0).client).ps.weaponstate == WEAPON_DROPPING as libc::c_int {
            weapon = (*(*self_0).client).pers.cmd.weapon as libc::c_int
        }
        if 0 == (*(*self_0).client).ps.stats[STAT_WEAPONS as libc::c_int as usize] & 1i32 << weapon
        {
            weapon = WP_NONE as libc::c_int
        }
    }
    if weapon > WP_MACHINEGUN as libc::c_int
        && weapon != WP_GRAPPLING_HOOK as libc::c_int
        && 0 != (*(*self_0).client).ps.ammo[weapon as usize]
    {
        item = BG_FindItemForWeapon(weapon as weapon_t);
        Drop_Item(self_0, item, 0i32 as libc::c_float);
    }
    if g_gametype.integer != GT_TEAM as libc::c_int {
        angle = 45i32 as libc::c_float;
        i = 1i32;
        while i < PW_NUM_POWERUPS as libc::c_int {
            if (*(*self_0).client).ps.powerups[i as usize] > level.time {
                item = BG_FindItemForPowerup(i as powerup_t);
                if !item.is_null() {
                    drop_0 = Drop_Item(self_0, item, angle);
                    (*drop_0).count =
                        ((*(*self_0).client).ps.powerups[i as usize] - level.time) / 1000i32;
                    if (*drop_0).count < 1i32 {
                        (*drop_0).count = 1i32
                    }
                    angle += 45i32 as libc::c_float
                }
            }
            i += 1
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn player_die(
    mut self_0: *mut gentity_t,
    mut inflictor: *mut gentity_t,
    mut attacker: *mut gentity_t,
    mut damage: libc::c_int,
    mut meansOfDeath: libc::c_int,
) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut anim: libc::c_int = 0;
    let mut contents: libc::c_int = 0;
    let mut killer: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut killerName: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut obit: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*self_0).client).ps.pm_type == PM_DEAD as libc::c_int {
        return;
    }
    if 0 != level.intermissiontime {
        return;
    }
    CheckAlmostCapture(self_0, attacker);
    CheckAlmostScored(self_0, attacker);
    if !(*self_0).client.is_null() && !(*(*self_0).client).hook.is_null() {
        Weapon_HookFree((*(*self_0).client).hook);
    }
    (*(*self_0).client).ps.pm_type = PM_DEAD as libc::c_int;
    if !attacker.is_null() {
        killer = (*attacker).s.number;
        if !(*attacker).client.is_null() {
            killerName = (*(*attacker).client).pers.netname.as_mut_ptr()
        } else {
            killerName =
                b"<non-client>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
    } else {
        killer = (1i32 << 10i32) - 2i32;
        killerName = b"<world>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if killer < 0i32 || killer >= 64i32 {
        killer = (1i32 << 10i32) - 2i32;
        killerName = b"<world>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    }
    if meansOfDeath < 0i32
        || meansOfDeath as libc::c_ulong
            >= (::std::mem::size_of::<[*mut libc::c_char; 24]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        obit = b"<bad obituary>\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
    } else {
        obit = modNames[meansOfDeath as usize]
    }
    G_LogPrintf(
        b"Kill: %i %i %i: %s killed %s by %s\n\x00" as *const u8 as *const libc::c_char,
        killer,
        (*self_0).s.number,
        meansOfDeath,
        killerName,
        (*(*self_0).client).pers.netname.as_mut_ptr(),
        obit,
    );
    ent = G_TempEntity(
        (*self_0).r.currentOrigin.as_mut_ptr(),
        EV_OBITUARY as libc::c_int,
    );
    (*ent).s.eventParm = meansOfDeath;
    (*ent).s.otherEntityNum = (*self_0).s.number;
    (*ent).s.otherEntityNum2 = killer;
    (*ent).r.svFlags = 0x20i32;
    (*self_0).enemy = attacker;
    (*(*self_0).client).ps.persistant[PERS_KILLED as libc::c_int as usize] += 1;
    if !attacker.is_null() && !(*attacker).client.is_null() {
        (*(*attacker).client).lastkilled_client = (*self_0).s.number;
        if attacker == self_0 || 0 != OnSameTeam(self_0, attacker) as libc::c_uint {
            AddScore(attacker, (*self_0).r.currentOrigin.as_mut_ptr(), -1i32);
        } else {
            AddScore(attacker, (*self_0).r.currentOrigin.as_mut_ptr(), 1i32);
            if meansOfDeath == MOD_GAUNTLET as libc::c_int {
                (*(*attacker).client).ps.persistant
                    [PERS_GAUNTLET_FRAG_COUNT as libc::c_int as usize] += 1;
                (*(*attacker).client).ps.eFlags &=
                    !(0x8000i32 | 0x8i32 | 0x40i32 | 0x20000i32 | 0x10000i32 | 0x800i32);
                (*(*attacker).client).ps.eFlags |= 0x40i32;
                (*(*attacker).client).rewardTime = level.time + 2000i32;
                (*(*self_0).client).ps.persistant[PERS_PLAYEREVENTS as libc::c_int as usize] ^=
                    0x2i32
            }
            if level.time - (*(*attacker).client).lastKillTime < 3000i32 {
                (*(*attacker).client).ps.persistant
                    [PERS_EXCELLENT_COUNT as libc::c_int as usize] += 1;
                (*(*attacker).client).ps.eFlags &=
                    !(0x8000i32 | 0x8i32 | 0x40i32 | 0x20000i32 | 0x10000i32 | 0x800i32);
                (*(*attacker).client).ps.eFlags |= 0x8i32;
                (*(*attacker).client).rewardTime = level.time + 2000i32
            }
            (*(*attacker).client).lastKillTime = level.time
        }
    } else {
        AddScore(self_0, (*self_0).r.currentOrigin.as_mut_ptr(), -1i32);
    }
    Team_FragBonuses(self_0, inflictor, attacker);
    if meansOfDeath == MOD_SUICIDE as libc::c_int {
        if 0 != (*(*self_0).client).ps.powerups[PW_NEUTRALFLAG as libc::c_int as usize] {
            Team_ReturnFlag(TEAM_FREE as libc::c_int);
            (*(*self_0).client).ps.powerups[PW_NEUTRALFLAG as libc::c_int as usize] = 0i32
        } else if 0 != (*(*self_0).client).ps.powerups[PW_REDFLAG as libc::c_int as usize] {
            Team_ReturnFlag(TEAM_RED as libc::c_int);
            (*(*self_0).client).ps.powerups[PW_REDFLAG as libc::c_int as usize] = 0i32
        } else if 0 != (*(*self_0).client).ps.powerups[PW_BLUEFLAG as libc::c_int as usize] {
            Team_ReturnFlag(TEAM_BLUE as libc::c_int);
            (*(*self_0).client).ps.powerups[PW_BLUEFLAG as libc::c_int as usize] = 0i32
        }
    }
    TossClientItems(self_0);
    Cmd_Score_f(self_0);
    i = 0i32;
    while i < level.maxclients {
        let mut client: *mut gclient_t = 0 as *mut gclient_t;
        client = &mut *level.clients.offset(i as isize) as *mut gclient_s;
        if !((*client).pers.connected as libc::c_uint
            != CON_CONNECTED as libc::c_int as libc::c_uint)
        {
            if !((*client).sess.sessionTeam as libc::c_uint
                != TEAM_SPECTATOR as libc::c_int as libc::c_uint)
            {
                if (*client).sess.spectatorClient == (*self_0).s.number {
                    Cmd_Score_f(g_entities.as_mut_ptr().offset(i as isize));
                }
            }
        }
        i += 1
    }
    (*self_0).takedamage = qtrue;
    (*self_0).s.weapon = WP_NONE as libc::c_int;
    (*self_0).s.powerups = 0i32;
    (*self_0).r.contents = 0x4000000i32;
    (*self_0).s.angles[0usize] = 0i32 as vec_t;
    (*self_0).s.angles[2usize] = 0i32 as vec_t;
    LookAtKiller(self_0, inflictor, attacker);
    (*(*self_0).client).ps.viewangles[0usize] = (*self_0).s.angles[0usize];
    (*(*self_0).client).ps.viewangles[1usize] = (*self_0).s.angles[1usize];
    (*(*self_0).client).ps.viewangles[2usize] = (*self_0).s.angles[2usize];
    (*self_0).s.loopSound = 0i32;
    (*self_0).r.maxs[2usize] = -8i32 as vec_t;
    (*(*self_0).client).respawnTime = level.time + 1700i32;
    memset(
        (*(*self_0).client).ps.powerups.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
    );
    contents = trap_PointContents(
        (*self_0).r.currentOrigin.as_mut_ptr() as *const vec_t,
        -1i32,
    );
    if (*self_0).health <= -40i32
        && 0 == contents as libc::c_uint & 0x80000000u32
        && 0 != g_blood.integer
        || meansOfDeath == MOD_SUICIDE as libc::c_int
    {
        GibEntity(self_0, killer);
    } else {
        static mut i_0: libc::c_int = 0;
        match i_0 {
            0 => anim = BOTH_DEATH1 as libc::c_int,
            1 => anim = BOTH_DEATH2 as libc::c_int,
            2 | _ => anim = BOTH_DEATH3 as libc::c_int,
        }
        if (*self_0).health <= -40i32 {
            (*self_0).health = -40i32 + 1i32
        }
        (*(*self_0).client).ps.legsAnim = (*(*self_0).client).ps.legsAnim & 128i32 ^ 128i32 | anim;
        (*(*self_0).client).ps.torsoAnim =
            (*(*self_0).client).ps.torsoAnim & 128i32 ^ 128i32 | anim;
        G_AddEvent(self_0, EV_DEATH1 as libc::c_int + i_0, killer);
        (*self_0).die = Some(body_die);
        i_0 = (i_0 + 1i32) % 3i32
    }
    trap_LinkEntity(self_0);
}
/*
==================
LookAtKiller
==================
*/
#[no_mangle]
pub unsafe extern "C" fn LookAtKiller(
    mut self_0: *mut gentity_t,
    mut inflictor: *mut gentity_t,
    mut attacker: *mut gentity_t,
) {
    let mut dir: vec3_t = [0.; 3];
    if !attacker.is_null() && attacker != self_0 {
        dir[0usize] = (*attacker).s.pos.trBase[0usize] - (*self_0).s.pos.trBase[0usize];
        dir[1usize] = (*attacker).s.pos.trBase[1usize] - (*self_0).s.pos.trBase[1usize];
        dir[2usize] = (*attacker).s.pos.trBase[2usize] - (*self_0).s.pos.trBase[2usize]
    } else if !inflictor.is_null() && inflictor != self_0 {
        dir[0usize] = (*inflictor).s.pos.trBase[0usize] - (*self_0).s.pos.trBase[0usize];
        dir[1usize] = (*inflictor).s.pos.trBase[1usize] - (*self_0).s.pos.trBase[1usize];
        dir[2usize] = (*inflictor).s.pos.trBase[2usize] - (*self_0).s.pos.trBase[2usize]
    } else {
        (*(*self_0).client).ps.stats[STAT_DEAD_YAW as libc::c_int as usize] =
            (*self_0).s.angles[1usize] as libc::c_int;
        return;
    }
    (*(*self_0).client).ps.stats[STAT_DEAD_YAW as libc::c_int as usize] =
        vectoyaw(dir.as_mut_ptr() as *const vec_t) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn AddScore(
    mut ent: *mut gentity_t,
    mut origin: *mut vec_t,
    mut score: libc::c_int,
) {
    if (*ent).client.is_null() {
        return;
    }
    if 0 != level.warmupTime {
        return;
    }
    ScorePlum(ent, origin, score);
    (*(*ent).client).ps.persistant[PERS_SCORE as libc::c_int as usize] += score;
    if g_gametype.integer == GT_TEAM as libc::c_int {
        level.teamScores
            [(*(*ent).client).ps.persistant[PERS_TEAM as libc::c_int as usize] as usize] += score
    }
    CalculateRanks();
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
// g_combat.c
/*
============
ScorePlum
============
*/
#[no_mangle]
pub unsafe extern "C" fn ScorePlum(
    mut ent: *mut gentity_t,
    mut origin: *mut vec_t,
    mut score: libc::c_int,
) {
    let mut plum: *mut gentity_t = 0 as *mut gentity_t;
    plum = G_TempEntity(origin, EV_SCOREPLUM as libc::c_int);
    (*plum).r.svFlags |= 0x100i32;
    (*plum).r.singleClient = (*ent).s.number;
    (*plum).s.otherEntityNum = (*ent).s.number;
    (*plum).s.time = score;
}
// these are just for logging, the client prints its own messages
#[no_mangle]
pub static mut modNames: [*mut libc::c_char; 24] = [
    b"MOD_UNKNOWN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_SHOTGUN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_GAUNTLET\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_MACHINEGUN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_GRENADE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_GRENADE_SPLASH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_ROCKET\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_ROCKET_SPLASH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_PLASMA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_PLASMA_SPLASH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_RAILGUN\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_LIGHTNING\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_BFG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_BFG_SPLASH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_WATER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_SLIME\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_LAVA\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_CRUSH\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_TELEFRAG\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_FALLING\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_SUICIDE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_TARGET_LASER\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_TRIGGER_HURT\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"MOD_GRAPPLE\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
/*
==================
CheckAlmostScored
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CheckAlmostScored(
    mut self_0: *mut gentity_t,
    mut attacker: *mut gentity_t,
) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut dir: vec3_t = [0.; 3];
    let mut classname: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 != (*(*self_0).client).ps.generic1 {
        if (*(*self_0).client).sess.sessionTeam as libc::c_uint
            == TEAM_BLUE as libc::c_int as libc::c_uint
        {
            classname =
                b"team_redobelisk\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        } else {
            classname =
                b"team_blueobelisk\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        ent = G_Find(
            0 as *mut gentity_t,
            &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                as libc::c_int,
            classname,
        );
        if !ent.is_null() {
            dir[0usize] = (*(*self_0).client).ps.origin[0usize] - (*ent).s.origin[0usize];
            dir[1usize] = (*(*self_0).client).ps.origin[1usize] - (*ent).s.origin[1usize];
            dir[2usize] = (*(*self_0).client).ps.origin[2usize] - (*ent).s.origin[2usize];
            if VectorLength(dir.as_mut_ptr() as *const vec_t) < 200i32 as libc::c_float {
                (*(*self_0).client).ps.persistant[PERS_PLAYEREVENTS as libc::c_int as usize] ^=
                    0x4i32;
                if !(*attacker).client.is_null() {
                    (*(*attacker).client).ps.persistant
                        [PERS_PLAYEREVENTS as libc::c_int as usize] ^= 0x4i32
                }
            }
        }
    };
}
/*
==================
CheckAlmostCapture
==================
*/
#[no_mangle]
pub unsafe extern "C" fn CheckAlmostCapture(
    mut self_0: *mut gentity_t,
    mut attacker: *mut gentity_t,
) {
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut dir: vec3_t = [0.; 3];
    let mut classname: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 != (*(*self_0).client).ps.powerups[PW_REDFLAG as libc::c_int as usize]
        || 0 != (*(*self_0).client).ps.powerups[PW_BLUEFLAG as libc::c_int as usize]
        || 0 != (*(*self_0).client).ps.powerups[PW_NEUTRALFLAG as libc::c_int as usize]
    {
        if g_gametype.integer == GT_CTF as libc::c_int {
            if (*(*self_0).client).sess.sessionTeam as libc::c_uint
                == TEAM_BLUE as libc::c_int as libc::c_uint
            {
                classname = b"team_CTF_blueflag\x00" as *const u8 as *const libc::c_char
                    as *mut libc::c_char
            } else {
                classname =
                    b"team_CTF_redflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
            }
        } else if (*(*self_0).client).sess.sessionTeam as libc::c_uint
            == TEAM_BLUE as libc::c_int as libc::c_uint
        {
            classname =
                b"team_CTF_redflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        } else {
            classname =
                b"team_CTF_blueflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char
        }
        ent = 0 as *mut gentity_t;
        loop {
            ent = G_Find(
                ent,
                &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t
                    as libc::c_int,
                classname,
            );
            if !(!ent.is_null() && 0 != (*ent).flags & 0x1000i32) {
                break;
            }
        }
        if !ent.is_null() && 0 == (*ent).r.svFlags & 0x1i32 {
            dir[0usize] = (*(*self_0).client).ps.origin[0usize] - (*ent).s.origin[0usize];
            dir[1usize] = (*(*self_0).client).ps.origin[1usize] - (*ent).s.origin[1usize];
            dir[2usize] = (*(*self_0).client).ps.origin[2usize] - (*ent).s.origin[2usize];
            if VectorLength(dir.as_mut_ptr() as *const vec_t) < 200i32 as libc::c_float {
                (*(*self_0).client).ps.persistant[PERS_PLAYEREVENTS as libc::c_int as usize] ^=
                    0x4i32;
                if !(*attacker).client.is_null() {
                    (*(*attacker).client).ps.persistant
                        [PERS_PLAYEREVENTS as libc::c_int as usize] ^= 0x4i32
                }
            }
        }
    };
}
/*
================
RaySphereIntersections
================
*/
#[no_mangle]
pub unsafe extern "C" fn RaySphereIntersections(
    mut origin: *mut vec_t,
    mut radius: libc::c_float,
    mut point: *mut vec_t,
    mut dir: *mut vec_t,
    mut intersections: *mut vec3_t,
) -> libc::c_int {
    let mut b: libc::c_float = 0.;
    let mut c: libc::c_float = 0.;
    let mut d: libc::c_float = 0.;
    let mut t: libc::c_float = 0.;
    VectorNormalize(dir);
    b = 2i32 as libc::c_float
        * (*dir.offset(0isize) * (*point.offset(0isize) - *origin.offset(0isize))
            + *dir.offset(1isize) * (*point.offset(1isize) - *origin.offset(1isize))
            + *dir.offset(2isize) * (*point.offset(2isize) - *origin.offset(2isize)));
    c = (*point.offset(0isize) - *origin.offset(0isize))
        * (*point.offset(0isize) - *origin.offset(0isize))
        + (*point.offset(1isize) - *origin.offset(1isize))
            * (*point.offset(1isize) - *origin.offset(1isize))
        + (*point.offset(2isize) - *origin.offset(2isize))
            * (*point.offset(2isize) - *origin.offset(2isize))
        - radius * radius;
    d = b * b - 4i32 as libc::c_float * c;
    if d > 0i32 as libc::c_float {
        t = ((-b as libc::c_double + sqrt(d as libc::c_double)) / 2i32 as libc::c_double)
            as libc::c_float;
        (*intersections.offset(0isize))[0usize] = *point.offset(0isize) + *dir.offset(0isize) * t;
        (*intersections.offset(0isize))[1usize] = *point.offset(1isize) + *dir.offset(1isize) * t;
        (*intersections.offset(0isize))[2usize] = *point.offset(2isize) + *dir.offset(2isize) * t;
        t = ((-b as libc::c_double - sqrt(d as libc::c_double)) / 2i32 as libc::c_double)
            as libc::c_float;
        (*intersections.offset(1isize))[0usize] = *point.offset(0isize) + *dir.offset(0isize) * t;
        (*intersections.offset(1isize))[1usize] = *point.offset(1isize) + *dir.offset(1isize) * t;
        (*intersections.offset(1isize))[2usize] = *point.offset(2isize) + *dir.offset(2isize) * t;
        return 2i32;
    } else {
        if d == 0i32 as libc::c_float {
            t = -b / 2i32 as libc::c_float;
            (*intersections.offset(0isize))[0usize] =
                *point.offset(0isize) + *dir.offset(0isize) * t;
            (*intersections.offset(0isize))[1usize] =
                *point.offset(1isize) + *dir.offset(1isize) * t;
            (*intersections.offset(0isize))[2usize] =
                *point.offset(2isize) + *dir.offset(2isize) * t;
            return 1i32;
        }
    }
    return 0i32;
}
