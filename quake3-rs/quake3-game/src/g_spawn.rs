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
    gitem_s, gitem_t, itemType_t, team_t, unnamed, unnamed_0, ET_BEAM, ET_EVENTS, ET_GENERAL,
    ET_GRAPPLE, ET_INVISIBLE, ET_ITEM, ET_MISSILE, ET_MOVER, ET_PLAYER, ET_PORTAL, ET_PUSH_TRIGGER,
    ET_SPEAKER, ET_TEAM, ET_TELEPORT_TRIGGER, GT_1FCTF, GT_CTF, GT_FFA, GT_HARVESTER,
    GT_MAX_GAME_TYPE, GT_OBELISK, GT_SINGLE_PLAYER, GT_TEAM, GT_TOURNAMENT, IT_AMMO, IT_ARMOR,
    IT_BAD, IT_HEALTH, IT_HOLDABLE, IT_PERSISTANT_POWERUP, IT_POWERUP, IT_TEAM, IT_WEAPON,
    TEAM_BLUE, TEAM_FREE, TEAM_NUM_TEAMS, TEAM_RED, TEAM_SPECTATOR,
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
    trap_AdjustAreaPortalState, trap_Cvar_Set, trap_GetEntityToken, trap_LinkEntity,
    trap_SetConfigstring, CON_CONNECTED, CON_CONNECTING, CON_DISCONNECTED, MOVER_1TO2, MOVER_2TO1,
    MOVER_POS1, MOVER_POS2, SPECTATOR_FOLLOW, SPECTATOR_FREE, SPECTATOR_NOT, SPECTATOR_SCOREBOARD,
    TEAM_ACTIVE, TEAM_BEGIN,
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
use g_variadic_h::{G_Error, G_LogPrintf, G_Printf};
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
    usercmd_s, usercmd_t, va, vec3_t, vec_t, vmCvar_t, Q_stricmp, TR_GRAVITY, TR_INTERPOLATE,
    TR_LINEAR, TR_LINEAR_STOP, TR_SINE, TR_STATIONARY,
};
use stddef_h::size_t;
use stdlib::{atof, atoi, memcpy, sscanf, strcmp, strlen, strstr};

//
// g_spawn.c
//
#[no_mangle]
pub unsafe extern "C" fn G_SpawnString(
    mut key: *const libc::c_char,
    mut defaultString: *const libc::c_char,
    mut out: *mut *mut libc::c_char,
) -> qboolean {
    let mut i: libc::c_int = 0;
    if 0 == level.spawning as u64 {
        *out = defaultString as *mut libc::c_char
    }
    i = 0i32;
    while i < level.numSpawnVars {
        if 0 == Q_stricmp(key, level.spawnVars[i as usize][0usize]) {
            *out = level.spawnVars[i as usize][1usize];
            return qtrue;
        }
        i += 1
    }
    *out = defaultString as *mut libc::c_char;
    return qfalse;
}
// spawn string returns a temporary reference, you must CopyString() if you want to keep it
#[no_mangle]
pub unsafe extern "C" fn G_SpawnFloat(
    mut key: *const libc::c_char,
    mut defaultString: *const libc::c_char,
    mut out: *mut libc::c_float,
) -> qboolean {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut present: qboolean = qfalse;
    present = G_SpawnString(key, defaultString, &mut s);
    *out = atof(s) as libc::c_float;
    return present;
}
#[no_mangle]
pub unsafe extern "C" fn G_SpawnInt(
    mut key: *const libc::c_char,
    mut defaultString: *const libc::c_char,
    mut out: *mut libc::c_int,
) -> qboolean {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut present: qboolean = qfalse;
    present = G_SpawnString(key, defaultString, &mut s);
    *out = atoi(s);
    return present;
}
#[no_mangle]
pub unsafe extern "C" fn G_SpawnVector(
    mut key: *const libc::c_char,
    mut defaultString: *const libc::c_char,
    mut out: *mut libc::c_float,
) -> qboolean {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut present: qboolean = qfalse;
    present = G_SpawnString(key, defaultString, &mut s);
    sscanf(
        s,
        b"%f %f %f\x00" as *const u8 as *const libc::c_char,
        &mut *out.offset(0isize) as *mut libc::c_float,
        &mut *out.offset(1isize) as *mut libc::c_float,
        &mut *out.offset(2isize) as *mut libc::c_float,
    );
    return present;
}
#[no_mangle]
pub unsafe extern "C" fn G_SpawnEntitiesFromString() {
    level.spawning = qtrue;
    level.numSpawnVars = 0i32;
    if 0 == G_ParseSpawnVars() as u64 {
        G_Error(b"SpawnEntities: no entities\x00" as *const u8 as *const libc::c_char);
    }
    SP_worldspawn();
    while 0 != G_ParseSpawnVars() as u64 {
        G_SpawnGEntityFromSpawnVars();
    }
    level.spawning = qfalse;
}
/*
===================
G_SpawnGEntityFromSpawnVars

Spawn an entity and fill in all of the level fields from
level.spawnVars[], then call the class specific spawn function
===================
*/
#[no_mangle]
pub unsafe extern "C" fn G_SpawnGEntityFromSpawnVars() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut gentity_t = 0 as *mut gentity_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gametypeName: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut gametypeNames: [*mut libc::c_char; 8] = [
        b"ffa\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"tournament\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"single\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"team\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"ctf\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"oneflag\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"obelisk\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"harvester\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    ent = G_Spawn();
    i = 0i32;
    while i < level.numSpawnVars {
        G_ParseField(
            level.spawnVars[i as usize][0usize],
            level.spawnVars[i as usize][1usize],
            ent,
        );
        i += 1
    }
    if g_gametype.integer == GT_SINGLE_PLAYER as libc::c_int {
        G_SpawnInt(
            b"notsingle\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            &mut i,
        );
        if 0 != i {
            if (*ent).s.eType == ET_MOVER as libc::c_int {
                trap_LinkEntity(ent);
                trap_AdjustAreaPortalState(ent, qtrue);
            }
            G_FreeEntity(ent);
            return;
        }
    }
    if g_gametype.integer >= GT_TEAM as libc::c_int {
        G_SpawnInt(
            b"notteam\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            &mut i,
        );
        if 0 != i {
            if (*ent).s.eType == ET_MOVER as libc::c_int {
                trap_LinkEntity(ent);
                trap_AdjustAreaPortalState(ent, qtrue);
            }
            G_FreeEntity(ent);
            return;
        }
    } else {
        G_SpawnInt(
            b"notfree\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
            &mut i,
        );
        if 0 != i {
            if (*ent).s.eType == ET_MOVER as libc::c_int {
                trap_LinkEntity(ent);
                trap_AdjustAreaPortalState(ent, qtrue);
            }
            G_FreeEntity(ent);
            return;
        }
    }
    G_SpawnInt(
        b"notq3a\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut i,
    );
    if 0 != i {
        if (*ent).s.eType == ET_MOVER as libc::c_int {
            trap_LinkEntity(ent);
            trap_AdjustAreaPortalState(ent, qtrue);
        }
        G_FreeEntity(ent);
        return;
    }
    if 0 != G_SpawnString(
        b"gametype\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
        &mut value,
    ) as u64
    {
        if g_gametype.integer >= GT_FFA as libc::c_int
            && g_gametype.integer < GT_MAX_GAME_TYPE as libc::c_int
        {
            gametypeName = gametypeNames[g_gametype.integer as usize];
            s = strstr(value, gametypeName);
            if s.is_null() {
                if (*ent).s.eType == ET_MOVER as libc::c_int {
                    trap_LinkEntity(ent);
                    trap_AdjustAreaPortalState(ent, qtrue);
                }
                G_FreeEntity(ent);
                return;
            }
        }
    }
    (*ent).s.pos.trBase[0usize] = (*ent).s.origin[0usize];
    (*ent).s.pos.trBase[1usize] = (*ent).s.origin[1usize];
    (*ent).s.pos.trBase[2usize] = (*ent).s.origin[2usize];
    (*ent).r.currentOrigin[0usize] = (*ent).s.origin[0usize];
    (*ent).r.currentOrigin[1usize] = (*ent).s.origin[1usize];
    (*ent).r.currentOrigin[2usize] = (*ent).s.origin[2usize];
    if 0 == G_CallSpawn(ent) as u64 {
        G_FreeEntity(ent);
    };
}
// info entities don't do anything at all, but provide positional
// information for things controlled by other processes
// use target_position instead
// rename trigger_timer?
// Triggers are brush objects that cause an effect when contacted
// by a living player, usually involving firing targets.
// While almost everything could be done with
// a single trigger class and different targets, triggered effects
// could not be client side predicted (push and teleport).
// targets perform no action by themselves, but must be triggered
// by another entity
/*
===============
G_CallSpawn

Finds the spawn function for the entity and calls it,
returning qfalse if not found
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_CallSpawn(mut ent: *mut gentity_t) -> qboolean {
    let mut s: *mut spawn_t = 0 as *mut spawn_t;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    if (*ent).classname.is_null() {
        G_Printf(b"G_CallSpawn: NULL classname\n\x00" as *const u8 as *const libc::c_char);
        return qfalse;
    }
    item = bg_itemlist.as_mut_ptr().offset(1isize);
    while !(*item).classname.is_null() {
        if 0 == strcmp((*item).classname, (*ent).classname) {
            G_SpawnItem(ent, item);
            return qtrue;
        }
        item = item.offset(1isize)
    }
    s = spawns.as_mut_ptr();
    while !(*s).name.is_null() {
        if 0 == strcmp((*s).name, (*ent).classname) {
            (*s).spawn.expect("non-null function pointer")(ent);
            return qtrue;
        }
        s = s.offset(1isize)
    }
    G_Printf(
        b"%s doesn\'t have a spawn function\n\x00" as *const u8 as *const libc::c_char,
        (*ent).classname,
    );
    return qfalse;
}
#[no_mangle]
pub static mut spawns: [spawn_t; 49] = [
    spawn_t {
        name: b"info_player_start\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_info_player_start),
    },
    spawn_t {
        name: b"info_player_deathmatch\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spawn: Some(SP_info_player_deathmatch),
    },
    spawn_t {
        name: b"info_player_intermission\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spawn: Some(SP_info_player_intermission),
    },
    spawn_t {
        name: b"info_null\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_info_null),
    },
    spawn_t {
        name: b"info_notnull\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_info_notnull),
    },
    spawn_t {
        name: b"info_camp\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_info_camp),
    },
    spawn_t {
        name: b"func_plat\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_func_plat),
    },
    spawn_t {
        name: b"func_button\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_func_button),
    },
    spawn_t {
        name: b"func_door\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_func_door),
    },
    spawn_t {
        name: b"func_static\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_func_static),
    },
    spawn_t {
        name: b"func_rotating\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_func_rotating),
    },
    spawn_t {
        name: b"func_bobbing\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_func_bobbing),
    },
    spawn_t {
        name: b"func_pendulum\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_func_pendulum),
    },
    spawn_t {
        name: b"func_train\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_func_train),
    },
    spawn_t {
        name: b"func_group\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_info_null),
    },
    spawn_t {
        name: b"func_timer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_func_timer),
    },
    spawn_t {
        name: b"trigger_always\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_trigger_always),
    },
    spawn_t {
        name: b"trigger_multiple\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_trigger_multiple),
    },
    spawn_t {
        name: b"trigger_push\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_trigger_push),
    },
    spawn_t {
        name: b"trigger_teleport\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_trigger_teleport),
    },
    spawn_t {
        name: b"trigger_hurt\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_trigger_hurt),
    },
    spawn_t {
        name: b"target_give\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_give),
    },
    spawn_t {
        name: b"target_remove_powerups\x00" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        spawn: Some(SP_target_remove_powerups),
    },
    spawn_t {
        name: b"target_delay\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_delay),
    },
    spawn_t {
        name: b"target_speaker\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_speaker),
    },
    spawn_t {
        name: b"target_print\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_print),
    },
    spawn_t {
        name: b"target_laser\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_laser),
    },
    spawn_t {
        name: b"target_score\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_score),
    },
    spawn_t {
        name: b"target_teleporter\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_teleporter),
    },
    spawn_t {
        name: b"target_relay\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_relay),
    },
    spawn_t {
        name: b"target_kill\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_kill),
    },
    spawn_t {
        name: b"target_position\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_position),
    },
    spawn_t {
        name: b"target_location\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_location),
    },
    spawn_t {
        name: b"target_push\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_target_push),
    },
    spawn_t {
        name: b"light\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_light),
    },
    spawn_t {
        name: b"path_corner\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_path_corner),
    },
    spawn_t {
        name: b"misc_teleporter_dest\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_misc_teleporter_dest),
    },
    spawn_t {
        name: b"misc_model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_misc_model),
    },
    spawn_t {
        name: b"misc_portal_surface\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_misc_portal_surface),
    },
    spawn_t {
        name: b"misc_portal_camera\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_misc_portal_camera),
    },
    spawn_t {
        name: b"shooter_rocket\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_shooter_rocket),
    },
    spawn_t {
        name: b"shooter_grenade\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_shooter_grenade),
    },
    spawn_t {
        name: b"shooter_plasma\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_shooter_plasma),
    },
    spawn_t {
        name: b"team_CTF_redplayer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_team_CTF_redplayer),
    },
    spawn_t {
        name: b"team_CTF_blueplayer\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_team_CTF_blueplayer),
    },
    spawn_t {
        name: b"team_CTF_redspawn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_team_CTF_redspawn),
    },
    spawn_t {
        name: b"team_CTF_bluespawn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_team_CTF_bluespawn),
    },
    spawn_t {
        name: b"item_botroam\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        spawn: Some(SP_item_botroam),
    },
    spawn_t {
        name: 0 as *const libc::c_char as *mut libc::c_char,
        spawn: None,
    },
];
#[no_mangle]
pub unsafe extern "C" fn SP_item_botroam(mut ent: *mut gentity_t) {}
/*
===============
G_ParseField

Takes a key/value pair and sets the binary values
in a gentity
===============
*/
#[no_mangle]
pub unsafe extern "C" fn G_ParseField(
    mut key: *const libc::c_char,
    mut value: *const libc::c_char,
    mut ent: *mut gentity_t,
) {
    let mut f: *mut field_t = 0 as *mut field_t;
    let mut b: *mut byte = 0 as *mut byte;
    let mut v: libc::c_float = 0.;
    let mut vec: vec3_t = [0.; 3];
    f = fields.as_mut_ptr();
    while !(*f).name.is_null() {
        if 0 == Q_stricmp((*f).name, key) {
            b = ent as *mut byte;
            match (*f).type_0 as libc::c_uint {
                2 => {
                    let ref mut fresh0 = *(b.offset((*f).ofs as isize) as *mut *mut libc::c_char);
                    *fresh0 = G_NewString(value)
                }
                3 => {
                    sscanf(
                        value,
                        b"%f %f %f\x00" as *const u8 as *const libc::c_char,
                        &mut vec[0usize] as *mut vec_t,
                        &mut vec[1usize] as *mut vec_t,
                        &mut vec[2usize] as *mut vec_t,
                    );
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float).offset(0isize) =
                        vec[0usize];
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float).offset(1isize) =
                        vec[1usize];
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float).offset(2isize) =
                        vec[2usize]
                }
                0 => *(b.offset((*f).ofs as isize) as *mut libc::c_int) = atoi(value),
                1 => {
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float) =
                        atof(value) as libc::c_float
                }
                4 => {
                    v = atof(value) as libc::c_float;
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float).offset(0isize) =
                        0i32 as libc::c_float;
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float).offset(1isize) = v;
                    *(b.offset((*f).ofs as isize) as *mut libc::c_float).offset(2isize) =
                        0i32 as libc::c_float
                }
                _ => {}
            }
            return;
        }
        f = f.offset(1isize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_NewString(mut string: *const libc::c_char) -> *mut libc::c_char {
    let mut newb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut new_p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    l = strlen(string).wrapping_add(1i32 as libc::c_ulong) as libc::c_int;
    newb = G_Alloc(l) as *mut libc::c_char;
    new_p = newb;
    i = 0i32;
    while i < l {
        if *string.offset(i as isize) as libc::c_int == '\\' as i32 && i < l - 1i32 {
            i += 1;
            if *string.offset(i as isize) as libc::c_int == 'n' as i32 {
                let fresh1 = new_p;
                new_p = new_p.offset(1);
                *fresh1 = '\n' as i32 as libc::c_char
            } else {
                let fresh2 = new_p;
                new_p = new_p.offset(1);
                *fresh2 = '\\' as i32 as libc::c_char
            }
        } else {
            let fresh3 = new_p;
            new_p = new_p.offset(1);
            *fresh3 = *string.offset(i as isize)
        }
        i += 1
    }
    return newb;
}
// Initialized in run_static_initializers
#[no_mangle]
pub static mut fields: [field_t; 20] = [field_t {
    name: 0 as *const libc::c_char as *mut libc::c_char,
    ofs: 0,
    type_0: F_INT,
}; 20];
/*
====================
G_ParseSpawnVars

Parses a brace bounded set of key / value pairs out of the
level's entity strings into level.spawnVars[]

This does not actually spawn an entity.
====================
*/
#[no_mangle]
pub unsafe extern "C" fn G_ParseSpawnVars() -> qboolean {
    let mut keyname: [libc::c_char; 1024] = [0; 1024];
    let mut com_token: [libc::c_char; 1024] = [0; 1024];
    level.numSpawnVars = 0i32;
    level.numSpawnVarChars = 0i32;
    if 0 == trap_GetEntityToken(
        com_token.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
    ) as u64
    {
        return qfalse;
    }
    if com_token[0usize] as libc::c_int != '{' as i32 {
        G_Error(
            b"G_ParseSpawnVars: found %s when expecting {\x00" as *const u8 as *const libc::c_char,
            com_token.as_mut_ptr(),
        );
    }
    loop {
        if 0 == trap_GetEntityToken(
            keyname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        ) as u64
        {
            G_Error(
                b"G_ParseSpawnVars: EOF without closing brace\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        if keyname[0usize] as libc::c_int == '}' as i32 {
            break;
        }
        if 0 == trap_GetEntityToken(
            com_token.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        ) as u64
        {
            G_Error(
                b"G_ParseSpawnVars: EOF without closing brace\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        if com_token[0usize] as libc::c_int == '}' as i32 {
            G_Error(
                b"G_ParseSpawnVars: closing brace without data\x00" as *const u8
                    as *const libc::c_char,
            );
        }
        if level.numSpawnVars == 64i32 {
            G_Error(b"G_ParseSpawnVars: MAX_SPAWN_VARS\x00" as *const u8 as *const libc::c_char);
        }
        level.spawnVars[level.numSpawnVars as usize][0usize] =
            G_AddSpawnVarToken(keyname.as_mut_ptr());
        level.spawnVars[level.numSpawnVars as usize][1usize] =
            G_AddSpawnVarToken(com_token.as_mut_ptr());
        level.numSpawnVars += 1
    }
    return qtrue;
}
/*
====================
G_AddSpawnVarToken
====================
*/
#[no_mangle]
pub unsafe extern "C" fn G_AddSpawnVarToken(mut string: *const libc::c_char) -> *mut libc::c_char {
    let mut l: libc::c_int = 0;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    l = strlen(string) as libc::c_int;
    if level.numSpawnVarChars + l + 1i32 > 4096i32 {
        G_Error(
            b"G_AddSpawnVarToken: MAX_SPAWN_VARS_CHARS\x00" as *const u8 as *const libc::c_char,
        );
    }
    dest = level
        .spawnVarChars
        .as_mut_ptr()
        .offset(level.numSpawnVarChars as isize);
    memcpy(
        dest as *mut libc::c_void,
        string as *const libc::c_void,
        (l + 1i32) as libc::c_ulong,
    );
    level.numSpawnVarChars += l + 1i32;
    return dest;
}
/*QUAKED worldspawn (0 0 0) ?

Every map should have exactly one worldspawn.
"music"		music wav file
"gravity"	800 is default gravity
"message"	Text to print during connection process
*/
#[no_mangle]
pub unsafe extern "C" fn SP_worldspawn() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    G_SpawnString(
        b"classname\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        &mut s,
    );
    if 0 != Q_stricmp(s, b"worldspawn\x00" as *const u8 as *const libc::c_char) {
        G_Error(
            b"SP_worldspawn: The first entity isn\'t \'worldspawn\'\x00" as *const u8
                as *const libc::c_char,
        );
    }
    trap_SetConfigstring(20i32, b"baseq3-1\x00" as *const u8 as *const libc::c_char);
    trap_SetConfigstring(
        21i32,
        va(
            b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            level.startTime,
        ),
    );
    G_SpawnString(
        b"music\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        &mut s,
    );
    trap_SetConfigstring(2i32, s);
    G_SpawnString(
        b"message\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char,
        &mut s,
    );
    trap_SetConfigstring(3i32, s);
    trap_SetConfigstring(4i32, g_motd.string.as_mut_ptr());
    G_SpawnString(
        b"gravity\x00" as *const u8 as *const libc::c_char,
        // The default is "800", but changed just for fun - MGS
        b"100\x00" as *const u8 as *const libc::c_char,
        &mut s,
    );
    trap_Cvar_Set(b"g_gravity\x00" as *const u8 as *const libc::c_char, s);
    G_SpawnString(
        b"enableDust\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut s,
    );
    trap_Cvar_Set(b"g_enableDust\x00" as *const u8 as *const libc::c_char, s);
    G_SpawnString(
        b"enableBreath\x00" as *const u8 as *const libc::c_char,
        b"0\x00" as *const u8 as *const libc::c_char,
        &mut s,
    );
    trap_Cvar_Set(b"g_enableBreath\x00" as *const u8 as *const libc::c_char, s);
    g_entities[((1i32 << 10i32) - 2i32) as usize].s.number = (1i32 << 10i32) - 2i32;
    g_entities[((1i32 << 10i32) - 2i32) as usize].r.ownerNum = (1i32 << 10i32) - 1i32;
    g_entities[((1i32 << 10i32) - 2i32) as usize].classname =
        b"worldspawn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    g_entities[((1i32 << 10i32) - 1i32) as usize].s.number = (1i32 << 10i32) - 1i32;
    g_entities[((1i32 << 10i32) - 1i32) as usize].r.ownerNum = (1i32 << 10i32) - 1i32;
    g_entities[((1i32 << 10i32) - 1i32) as usize].classname =
        b"nothing\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
    trap_SetConfigstring(5i32, b"\x00" as *const u8 as *const libc::c_char);
    if 0 != g_restarted.integer {
        trap_Cvar_Set(
            b"g_restarted\x00" as *const u8 as *const libc::c_char,
            b"0\x00" as *const u8 as *const libc::c_char,
        );
        level.warmupTime = 0i32
    } else if 0 != g_doWarmup.integer {
        level.warmupTime = -1i32;
        trap_SetConfigstring(
            5i32,
            va(
                b"%i\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                level.warmupTime,
            ),
        );
        G_LogPrintf(b"Warmup:\n\x00" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn run_static_initializers() {
    fields = [
        field_t {
            name: b"classname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).classname as *mut *mut libc::c_char as size_t,
            type_0: F_STRING,
        },
        field_t {
            name: b"origin\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).s.origin as *mut vec3_t as size_t,
            type_0: F_VECTOR,
        },
        field_t {
            name: b"model\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).model as *mut *mut libc::c_char as size_t,
            type_0: F_STRING,
        },
        field_t {
            name: b"model2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).model2 as *mut *mut libc::c_char as size_t,
            type_0: F_STRING,
        },
        field_t {
            name: b"spawnflags\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).spawnflags as *mut libc::c_int as size_t,
            type_0: F_INT,
        },
        field_t {
            name: b"speed\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).speed as *mut libc::c_float as size_t,
            type_0: F_FLOAT,
        },
        field_t {
            name: b"target\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).target as *mut *mut libc::c_char as size_t,
            type_0: F_STRING,
        },
        field_t {
            name: b"targetname\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).targetname as *mut *mut libc::c_char as size_t,
            type_0: F_STRING,
        },
        field_t {
            name: b"message\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).message as *mut *mut libc::c_char as size_t,
            type_0: F_STRING,
        },
        field_t {
            name: b"team\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).team as *mut *mut libc::c_char as size_t,
            type_0: F_STRING,
        },
        field_t {
            name: b"wait\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).wait as *mut libc::c_float as size_t,
            type_0: F_FLOAT,
        },
        field_t {
            name: b"random\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).random as *mut libc::c_float as size_t,
            type_0: F_FLOAT,
        },
        field_t {
            name: b"count\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).count as *mut libc::c_int as size_t,
            type_0: F_INT,
        },
        field_t {
            name: b"health\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).health as *mut libc::c_int as size_t,
            type_0: F_INT,
        },
        field_t {
            name: b"dmg\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).damage as *mut libc::c_int as size_t,
            type_0: F_INT,
        },
        field_t {
            name: b"angles\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).s.angles as *mut vec3_t as size_t,
            type_0: F_VECTOR,
        },
        field_t {
            name: b"angle\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).s.angles as *mut vec3_t as size_t,
            type_0: F_ANGLEHACK,
        },
        field_t {
            name: b"targetShaderName\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).targetShaderName as *mut *mut libc::c_char as size_t,
            type_0: F_STRING,
        },
        field_t {
            name: b"targetShaderNewName\x00" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ofs: &mut (*(0 as *mut gentity_t)).targetShaderNewName as *mut *mut libc::c_char
                as size_t,
            type_0: F_STRING,
        },
        field_t {
            name: 0 as *mut libc::c_char,
            ofs: 0,
            type_0: F_INT,
        },
    ]
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
pub const F_INT: fieldtype_t = 0;
pub const F_ANGLEHACK: fieldtype_t = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct field_t {
    pub name: *mut libc::c_char,
    pub ofs: size_t,
    pub type_0: fieldtype_t,
}
pub const F_FLOAT: fieldtype_t = 1;
pub const F_STRING: fieldtype_t = 2;
pub const F_VECTOR: fieldtype_t = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spawn_t {
    pub name: *mut libc::c_char,
    pub spawn: Option<unsafe extern "C" fn(_: *mut gentity_t) -> ()>,
}
pub type fieldtype_t = libc::c_uint;
