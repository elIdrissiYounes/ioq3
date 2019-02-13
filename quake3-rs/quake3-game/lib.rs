#![feature(libc)]
#![feature(extern_types)]
#![feature(asm)]
#![feature(ptr_wrapping_offset_from)]
#![feature(label_break_value)]
#![feature(const_raw_ptr_to_usize_cast)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]
#![feature(custom_attribute)]

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
use q_math::{
    vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
    DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize, VectorNormalize2,
};
extern crate libc;

#[path = "src/ai_chat.rs"]
pub mod ai_chat;
#[path = "src/ai_cmd.rs"]
pub mod ai_cmd;
#[path = "src/ai_dmnet.rs"]
pub mod ai_dmnet;
#[path = "src/ai_dmq3.rs"]
pub mod ai_dmq3;
#[path = "src/ai_main.rs"]
pub mod ai_main;
#[path = "src/ai_team.rs"]
pub mod ai_team;
#[path = "src/ai_vcmd.rs"]
pub mod ai_vcmd;
#[path = "src/bg_lib.rs"]
pub mod bg_lib;
#[path = "src/bg_misc.rs"]
pub mod bg_misc;
#[path = "src/bg_pmove.rs"]
pub mod bg_pmove;
#[path = "src/bg_slidemove.rs"]
pub mod bg_slidemove;
#[path = "src/g_active.rs"]
pub mod g_active;
#[path = "src/g_arenas.rs"]
pub mod g_arenas;
#[path = "src/g_bot.rs"]
pub mod g_bot;
#[path = "src/g_client.rs"]
pub mod g_client;
#[path = "src/g_cmds.rs"]
pub mod g_cmds;
#[path = "src/g_combat.rs"]
pub mod g_combat;
#[path = "src/g_items.rs"]
pub mod g_items;
#[path = "src/g_main.rs"]
pub mod g_main;
#[path = "src/g_mem.rs"]
pub mod g_mem;
#[path = "src/g_misc.rs"]
pub mod g_misc;
#[path = "src/g_missile.rs"]
pub mod g_missile;
#[path = "src/g_mover.rs"]
pub mod g_mover;
#[path = "src/g_session.rs"]
pub mod g_session;
#[path = "src/g_spawn.rs"]
pub mod g_spawn;
#[path = "src/g_svcmds.rs"]
pub mod g_svcmds;
#[path = "src/g_target.rs"]
pub mod g_target;
#[path = "src/g_team.rs"]
pub mod g_team;
#[path = "src/g_trigger.rs"]
pub mod g_trigger;
#[path = "src/g_utils.rs"]
pub mod g_utils;
#[path = "src/g_weapon.rs"]
pub mod g_weapon;
#[path = "src/q_math.rs"]
pub mod q_math;
mod g_public_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    use q_shared_h::{entityState_t, qboolean, vec3_t};
    pub const GAME_CLIENT_USERINFO_CHANGED: unnamed_5 = 4;
    pub const BOTAI_START_FRAME: unnamed_5 = 10;
    pub const GAME_CONSOLE_COMMAND: unnamed_5 = 9;
    pub const GAME_CLIENT_DISCONNECT: unnamed_5 = 5;
    pub const GAME_CLIENT_BEGIN: unnamed_5 = 3;
    pub const GAME_CLIENT_CONNECT: unnamed_5 = 2;
    pub const GAME_SHUTDOWN: unnamed_5 = 1;
    pub const GAME_CLIENT_THINK: unnamed_5 = 7;
    pub const GAME_RUN_FRAME: unnamed_5 = 8;
    pub const GAME_CLIENT_COMMAND: unnamed_5 = 6;
    pub const GAME_INIT: unnamed_5 = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct entityShared_t {
        pub unused: entityState_t,
        pub linked: qboolean,
        pub linkcount: libc::c_int,
        pub svFlags: libc::c_int,
        pub singleClient: libc::c_int,
        pub bmodel: qboolean,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub contents: libc::c_int,
        pub absmin: vec3_t,
        pub absmax: vec3_t,
        pub currentOrigin: vec3_t,
        pub currentAngles: vec3_t,
        pub ownerNum: libc::c_int,
    }
    pub type unnamed_5 = libc::c_uint;
    use libc;
}
mod stdlib {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    use stddef_h::size_t;
    extern "C" {
        #[no_mangle]
        pub fn __assert_fail(
            __assertion: *const libc::c_char,
            __file: *const libc::c_char,
            __line: libc::c_uint,
            __function: *const libc::c_char,
        ) -> !;
    }
    pub const _ISdigit: unnamed = 2048;
    pub const _ISalnum: unnamed = 8;
    pub type intptr_t = libc::c_long;
    pub const _ISlower: unnamed = 512;
    extern "C" {
        #[no_mangle]
        pub fn __ctype_b_loc() -> *mut *const libc::c_ushort;
        #[no_mangle]
        pub fn tolower(_: libc::c_int) -> libc::c_int;
    }
    pub const _ISalpha: unnamed = 1024;
    extern "C" {

        #[no_mangle]
        pub fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

    }
    extern "C" {
        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;

    }
    pub const _ISupper: unnamed = 256;
    pub const _ISgraph: unnamed = 32768;
    pub const _ISxdigit: unnamed = 4096;
    pub const _ISblank: unnamed = 1;
    pub const _ISspace: unnamed = 8192;
    extern "C" {
        #[no_mangle]
        pub fn toupper(_: libc::c_int) -> libc::c_int;
    }
    pub const _IScntrl: unnamed = 2;
    extern "C" {
        #[no_mangle]
        pub fn acos(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn atan2(_: libc::c_double, _: libc::c_double) -> libc::c_double;

    }
    pub const _ISprint: unnamed = 16384;
    pub const _ISpunct: unnamed = 4;
    extern "C" {

        #[no_mangle]
        pub fn srand(__seed: libc::c_uint);

    }
    extern "C" {

        #[no_mangle]
        pub fn fabs(_: libc::c_double) -> libc::c_double;
    }
    pub type unnamed = libc::c_uint;
    extern "C" {

        #[no_mangle]
        pub fn qsort(
            __base: *mut libc::c_void,
            __nmemb: size_t,
            __size: size_t,
            __compar: __compar_fn_t,
        );
    }
    extern "C" {
        #[no_mangle]
        pub fn cos(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn sin(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn ceil(_: libc::c_double) -> libc::c_double;
        #[no_mangle]
        pub fn floor(_: libc::c_double) -> libc::c_double;
    }
    extern "C" {

        #[no_mangle]
        pub fn memmove(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;

        #[no_mangle]
        pub fn strncpy(
            _: *mut libc::c_char,
            _: *const libc::c_char,
            _: libc::c_ulong,
        ) -> *mut libc::c_char;

        #[no_mangle]
        pub fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;

    }
    extern "C" {
        #[no_mangle]
        pub fn abs(_: libc::c_int) -> libc::c_int;
    }
    pub type __compar_fn_t =
        Option<unsafe extern "C" fn(_: *const libc::c_void, _: *const libc::c_void) -> libc::c_int>;
    extern "C" {
        #[no_mangle]
        pub fn sscanf(_: *const libc::c_char, _: *const libc::c_char, ...) -> libc::c_int;
    }
    extern "C" {
        #[no_mangle]
        pub fn atof(__nptr: *const libc::c_char) -> libc::c_double;
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn rand() -> libc::c_int;
    }
    extern "C" {

        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    }
    extern "C" {
        #[no_mangle]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
    extern "C" {
        #[no_mangle]
        pub fn sqrt(_: libc::c_double) -> libc::c_double;
    }
    use libc;
}
mod ai_variadic_h {
    use ai_main::{
        bot_developer, bot_state_t, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    extern "C" {
        #[no_mangle]
        pub fn BotAI_BotInitialChat(bs: *mut bot_state_t, type_0: *mut libc::c_char, ...);
        #[no_mangle]
        pub fn BotAI_Print(type_0: libc::c_int, fmt: *mut libc::c_char, ...);
    }
    use libc;
}
mod g_local_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_public_h::{gitem_t, team_t};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_public_h::entityShared_t;
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    use q_shared_h::{
        entityState_t, fileHandle_t, fsMode_t, playerState_t, qboolean, trace_t, usercmd_t, vec3_t,
        vec_t, vmCvar_t,
    };
    extern "C" {

        //
        // g_mem.c
        //

        #[no_mangle]
        pub fn trap_GetEntityToken(buffer: *mut libc::c_char, bufferSize: libc::c_int) -> qboolean;

    }
    extern "C" {

        #[no_mangle]
        pub fn trap_DebugPolygonCreate(
            color: libc::c_int,
            numPoints: libc::c_int,
            points: *mut vec3_t,
        ) -> libc::c_int;
    }
    extern "C" {

        //
        // g_items.c
        //

        //
        // g_utils.c
        //

        // damage flags
        // damage was indirect
        // armour does not protect from this damage
        // do not affect velocity, just view angles
        // armor, shields, invulnerability, and godmode have no effect
        //
        // g_missile.c
        //

        //
        // g_mover.c
        //

        //
        // g_client.c
        //

        //
        // g_cmds.c
        //

        //
        // g_arenas.c
        //

        //
        // g_svcmds.c
        //

        //
        // g_client.c
        //

        //
        // g_active.c
        //

        //
        // g_bot.c
        //

        #[no_mangle]
        pub fn trap_LocateGameData(
            gEnts: *mut gentity_t,
            numGEntities: libc::c_int,
            sizeofGEntity_t: libc::c_int,
            gameClients: *mut playerState_t,
            sizeofGameClient: libc::c_int,
        );

    }
    extern "C" {

        #[no_mangle]
        pub fn trap_EntityContact(
            mins: *const vec_t,
            maxs: *const vec_t,
            ent: *const gentity_t,
        ) -> qboolean;

    //
    // g_misc.c
    //

    //
    // g_weapon.c
    //

    }
    extern "C" {

        #[no_mangle]
        pub fn trap_EA_SayTeam(client: libc::c_int, str: *mut libc::c_char);
        #[no_mangle]
        pub fn trap_EA_Command(client: libc::c_int, command: *mut libc::c_char);

    }
    extern "C" {

        //
        // g_client.c
        //

        //
        // g_mem.c
        //

        #[no_mangle]
        pub fn trap_Print(text: *const libc::c_char);
        #[no_mangle]
        pub fn trap_FS_FCloseFile(f: fileHandle_t);
        #[no_mangle]
        pub fn trap_FS_Read(buffer: *mut libc::c_void, len: libc::c_int, f: fileHandle_t);
        #[no_mangle]
        pub fn trap_FS_FOpenFile(
            qpath: *const libc::c_char,
            f: *mut fileHandle_t,
            mode: fsMode_t,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_FS_GetFileList(
            path: *const libc::c_char,
            extension: *const libc::c_char,
            listbuf: *mut libc::c_char,
            bufsize: libc::c_int,
        ) -> libc::c_int;

        // allow this many total, including spectators

        #[no_mangle]
        pub fn trap_BotFreeClient(clientNum: libc::c_int);
        #[no_mangle]
        pub fn trap_BotAllocateClient() -> libc::c_int;

    }
    extern "C" {

        #[no_mangle]
        pub fn trap_InPVS(p1: *const vec_t, p2: *const vec_t) -> qboolean;

    // allow this many total, including spectators

    }
    extern "C" {

        // allow this many active

        //
        // g_client.c
        //

        //
        // g_misc.c
        //

        #[no_mangle]
        pub fn trap_Argv(n: libc::c_int, buffer: *mut libc::c_char, bufferLength: libc::c_int);
        #[no_mangle]
        pub fn trap_Argc() -> libc::c_int;

    //
    // g_team.c
    //

    }
    extern "C" {
        //
        // g_mem.c
        //

        #[no_mangle]
        pub fn trap_BotMutateGoalFuzzyLogic(goalstate: libc::c_int, range: libc::c_float);
        #[no_mangle]
        pub fn trap_BotInterbreedGoalFuzzyLogic(
            parent1: libc::c_int,
            parent2: libc::c_int,
            child: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_GeneticParentsAndChildSelection(
            numranks: libc::c_int,
            ranks: *mut libc::c_float,
            parent1: *mut libc::c_int,
            parent2: *mut libc::c_int,
            child: *mut libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn trap_BotSaveGoalFuzzyLogic(goalstate: libc::c_int, filename: *mut libc::c_char);

        #[no_mangle]
        pub fn trap_BotLibSetup() -> libc::c_int;

        #[no_mangle]
        pub fn trap_BotLibShutdown() -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotFreeCharacter(character: libc::c_int);
        #[no_mangle]
        pub fn trap_BotFreeWeaponState(weaponstate: libc::c_int);
        #[no_mangle]
        pub fn trap_BotFreeChatState(handle: libc::c_int);
        #[no_mangle]
        pub fn trap_BotFreeGoalState(handle: libc::c_int);
        #[no_mangle]
        pub fn trap_BotFreeMoveState(handle: libc::c_int);

        #[no_mangle]
        pub fn trap_BotResetWeaponState(weaponstate: libc::c_int);

        #[no_mangle]
        pub fn trap_BotLibLoadMap(mapname: *const libc::c_char) -> libc::c_int;

        #[no_mangle]
        pub fn trap_BotAllocMoveState() -> libc::c_int;

        #[no_mangle]
        pub fn trap_BotLoadChatFile(
            chatstate: libc::c_int,
            chatfile: *mut libc::c_char,
            chatname: *mut libc::c_char,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotAllocChatState() -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotLoadWeaponWeights(
            weaponstate: libc::c_int,
            filename: *mut libc::c_char,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotAllocWeaponState() -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotLoadItemWeights(
            goalstate: libc::c_int,
            filename: *mut libc::c_char,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotAllocGoalState(state: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotLoadCharacter(
            charfile: *mut libc::c_char,
            skill: libc::c_float,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_Initialized() -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotUserCommand(client: libc::c_int, ucmd: *mut usercmd_t);
        #[no_mangle]
        pub fn trap_EA_GetInput(
            client: libc::c_int,
            thinktime: libc::c_float,
            input: *mut libc::c_void,
        );

        #[no_mangle]
        pub fn trap_BotGetServerCommand(
            clientNum: libc::c_int,
            message: *mut libc::c_char,
            size: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_EA_ResetInput(client: libc::c_int);
        #[no_mangle]
        pub fn trap_AAS_Time() -> libc::c_float;
        #[no_mangle]
        pub fn trap_BotUpdateEntityItems();
        #[no_mangle]
        pub fn trap_BotLibUpdateEntity(ent: libc::c_int, bue: *mut libc::c_void) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotLibStartFrame(time: libc::c_float) -> libc::c_int;

        #[no_mangle]
        pub fn trap_BotGetSnapshotEntity(
            clientNum: libc::c_int,
            sequence: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_EntityInfo(entnum: libc::c_int, info: *mut libc::c_void);
    }
    extern "C" {

        //
        // g_active.c
        //

        //
        // g_main.c
        //

        #[no_mangle]
        pub fn trap_GetUsercmd(clientNum: libc::c_int, cmd: *mut usercmd_t);

        #[no_mangle]
        pub fn trap_SendServerCommand(clientNum: libc::c_int, text: *const libc::c_char);

        #[no_mangle]
        pub fn trap_DropClient(clientNum: libc::c_int, reason: *const libc::c_char);
        //
        // g_session.c
        //

        #[no_mangle]
        pub fn trap_SendConsoleCommand(exec_when: libc::c_int, text: *const libc::c_char);

    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bot_settings_s {
        pub characterfile: [libc::c_char; 144],
        pub skill: libc::c_float,
    }
    extern "C" {
        //
        // g_utils.c
        //

        #[no_mangle]
        pub fn trap_Cvar_Register(
            cvar: *mut vmCvar_t,
            var_name: *const libc::c_char,
            value: *const libc::c_char,
            flags: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_Cvar_Update(cvar: *mut vmCvar_t);

        #[no_mangle]
        pub fn trap_GetUserinfo(
            num: libc::c_int,
            buffer: *mut libc::c_char,
            bufferSize: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_SetUserinfo(num: libc::c_int, buffer: *const libc::c_char);

        #[no_mangle]
        pub fn trap_BotLibVarSet(
            var_name: *mut libc::c_char,
            value: *mut libc::c_char,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_BBoxAreas(
            absmins: *mut vec_t,
            absmaxs: *mut vec_t,
            areas: *mut libc::c_int,
            maxareas: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_AreaInfo(areanum: libc::c_int, info: *mut libc::c_void) -> libc::c_int;

        #[no_mangle]
        pub fn trap_AAS_PointAreaNum(point: *mut vec_t) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_TraceAreas(
            start: *mut vec_t,
            end: *mut vec_t,
            areas: *mut libc::c_int,
            points: *mut vec3_t,
            maxareas: libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn trap_AAS_NextBSPEntity(ent: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_ValueForBSPEpairKey(
            ent: libc::c_int,
            key: *mut libc::c_char,
            value: *mut libc::c_char,
            size: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_VectorForBSPEpairKey(
            ent: libc::c_int,
            key: *mut libc::c_char,
            v: *mut vec_t,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_FloatForBSPEpairKey(
            ent: libc::c_int,
            key: *mut libc::c_char,
            value: *mut libc::c_float,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_IntForBSPEpairKey(
            ent: libc::c_int,
            key: *mut libc::c_char,
            value: *mut libc::c_int,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn trap_AAS_EnableRoutingArea(areanum: libc::c_int, enable: libc::c_int)
            -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_PredictRoute(
            route: *mut libc::c_void,
            areanum: libc::c_int,
            origin: *mut vec_t,
            goalareanum: libc::c_int,
            travelflags: libc::c_int,
            maxareas: libc::c_int,
            maxtime: libc::c_int,
            stopevent: libc::c_int,
            stopcontents: libc::c_int,
            stoptfl: libc::c_int,
            stopareanum: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_PredictClientMovement(
            move_0: *mut libc::c_void,
            entnum: libc::c_int,
            origin: *mut vec_t,
            presencetype: libc::c_int,
            onground: libc::c_int,
            velocity: *mut vec_t,
            cmdmove: *mut vec_t,
            cmdframes: libc::c_int,
            maxframes: libc::c_int,
            frametime: libc::c_float,
            stopevent: libc::c_int,
            stopareanum: libc::c_int,
            visualize: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_EA_Say(client: libc::c_int, str: *mut libc::c_char);

        #[no_mangle]
        pub fn trap_EA_Use(client: libc::c_int);
        #[no_mangle]
        pub fn trap_EA_SelectWeapon(client: libc::c_int, weapon: libc::c_int);
        #[no_mangle]
        pub fn trap_EA_View(client: libc::c_int, viewangles: *mut vec_t);

        #[no_mangle]
        pub fn trap_Characteristic_String(
            character: libc::c_int,
            index: libc::c_int,
            buf: *mut libc::c_char,
            size: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_BotRemoveConsoleMessage(chatstate: libc::c_int, handle: libc::c_int);
        #[no_mangle]
        pub fn trap_BotNextConsoleMessage(
            chatstate: libc::c_int,
            cm: *mut libc::c_void,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotNumConsoleMessages(chatstate: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotReplyChat(
            chatstate: libc::c_int,
            message: *mut libc::c_char,
            mcontext: libc::c_int,
            vcontext: libc::c_int,
            var0: *mut libc::c_char,
            var1: *mut libc::c_char,
            var2: *mut libc::c_char,
            var3: *mut libc::c_char,
            var4: *mut libc::c_char,
            var5: *mut libc::c_char,
            var6: *mut libc::c_char,
            var7: *mut libc::c_char,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotFindMatch(
            str: *mut libc::c_char,
            match_0: *mut libc::c_void,
            context: libc::c_ulong,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotMatchVariable(
            match_0: *mut libc::c_void,
            variable: libc::c_int,
            buf: *mut libc::c_char,
            size: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_UnifyWhiteSpaces(string: *mut libc::c_char);
        #[no_mangle]
        pub fn trap_BotReplaceSynonyms(string: *mut libc::c_char, context: libc::c_ulong);
        #[no_mangle]
        pub fn trap_BotSetChatGender(chatstate: libc::c_int, gender: libc::c_int);
        #[no_mangle]
        pub fn trap_BotSetChatName(
            chatstate: libc::c_int,
            name: *mut libc::c_char,
            client: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_BotRemoveFromAvoidGoals(goalstate: libc::c_int, number: libc::c_int);
        #[no_mangle]
        pub fn trap_BotDumpAvoidGoals(goalstate: libc::c_int);
        #[no_mangle]
        pub fn trap_BotDumpGoalStack(goalstate: libc::c_int);
        #[no_mangle]
        pub fn trap_BotGetNextCampSpotGoal(
            num: libc::c_int,
            goal: *mut libc::c_void,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotGetLevelItemGoal(
            index: libc::c_int,
            classname: *mut libc::c_char,
            goal: *mut libc::c_void,
        ) -> libc::c_int;

        #[no_mangle]
        pub fn trap_BotPredictVisiblePosition(
            origin: *mut vec_t,
            areanum: libc::c_int,
            goal: *mut libc::c_void,
            travelflags: libc::c_int,
            target: *mut vec_t,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotInitMoveState(handle: libc::c_int, initmove: *mut libc::c_void);
        #[no_mangle]
        pub fn trap_BotAddAvoidSpot(
            movestate: libc::c_int,
            origin: *mut vec_t,
            radius: libc::c_float,
            type_0: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_BotChooseBestFightWeapon(
            weaponstate: libc::c_int,
            inventory: *mut libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotGetWeaponInfo(
            weaponstate: libc::c_int,
            weapon: libc::c_int,
            weaponinfo: *mut libc::c_void,
        );
    }
    extern "C" {
        // spawn string returns a temporary reference, you must CopyString() if you want to keep it

        #[no_mangle]
        pub fn trap_Cvar_VariableIntegerValue(var_name: *const libc::c_char) -> libc::c_int;
        #[no_mangle]
        pub fn trap_SetConfigstring(num: libc::c_int, string: *const libc::c_char);
    }
    extern "C" {

        #[no_mangle]
        pub fn trap_GetServerinfo(buffer: *mut libc::c_char, bufferSize: libc::c_int);

        #[no_mangle]
        pub fn trap_AAS_PresenceTypeBoundingBox(
            presencetype: libc::c_int,
            mins: *mut vec_t,
            maxs: *mut vec_t,
        );

        #[no_mangle]
        pub fn trap_BotNumInitialChats(
            chatstate: libc::c_int,
            type_0: *mut libc::c_char,
        ) -> libc::c_int;

    }
    pub type bot_settings_t = bot_settings_s;
    pub const MOVER_POS1: moverState_t = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clientPersistant_t {
        pub connected: clientConnected_t,
        pub cmd: usercmd_t,
        pub localClient: qboolean,
        pub initialSpawn: qboolean,
        pub predictItemPickup: qboolean,
        pub pmoveFixed: qboolean,
        pub netname: [libc::c_char; 36],
        pub maxHealth: libc::c_int,
        pub enterTime: libc::c_int,
        pub teamState: playerTeamState_t,
        pub voteCount: libc::c_int,
        pub teamVoteCount: libc::c_int,
        pub teamInfo: qboolean,
    }
    pub const CON_CONNECTING: clientConnected_t = 1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct gclient_s {
        pub ps: playerState_t,
        pub pers: clientPersistant_t,
        pub sess: clientSession_t,
        pub readyToExit: qboolean,
        pub noclip: qboolean,
        pub lastCmdTime: libc::c_int,
        pub buttons: libc::c_int,
        pub oldbuttons: libc::c_int,
        pub latched_buttons: libc::c_int,
        pub oldOrigin: vec3_t,
        pub damage_armor: libc::c_int,
        pub damage_blood: libc::c_int,
        pub damage_knockback: libc::c_int,
        pub damage_from: vec3_t,
        pub damage_fromWorld: qboolean,
        pub accurateCount: libc::c_int,
        pub accuracy_shots: libc::c_int,
        pub accuracy_hits: libc::c_int,
        pub lastkilled_client: libc::c_int,
        pub lasthurt_client: libc::c_int,
        pub lasthurt_mod: libc::c_int,
        pub respawnTime: libc::c_int,
        pub inactivityTime: libc::c_int,
        pub inactivityWarning: qboolean,
        pub rewardTime: libc::c_int,
        pub airOutTime: libc::c_int,
        pub lastKillTime: libc::c_int,
        pub fireHeld: qboolean,
        pub hook: *mut gentity_t,
        pub switchTeamTime: libc::c_int,
        pub timeResidual: libc::c_int,
        pub areabits: *mut libc::c_char,
    }
    pub const SPECTATOR_FREE: spectatorState_t = 1;
    pub const MOVER_POS2: moverState_t = 1;
    extern "C" {
        //
        // g_spawn.c
        //

        // spawn string returns a temporary reference, you must CopyString() if you want to keep it

        //
        // g_utils.c
        //

        #[no_mangle]
        pub fn trap_LinkEntity(ent: *mut gentity_t);

        #[no_mangle]
        pub fn trap_Trace(
            results: *mut trace_t,
            start: *const vec_t,
            mins: *const vec_t,
            maxs: *const vec_t,
            end: *const vec_t,
            passEntityNum: libc::c_int,
            contentmask: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_EntitiesInBox(
            mins: *const vec_t,
            maxs: *const vec_t,
            entityList: *mut libc::c_int,
            maxcount: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_UnlinkEntity(ent: *mut gentity_t);
        #[no_mangle]
        pub fn trap_AdjustAreaPortalState(ent: *mut gentity_t, open: qboolean);
        //
        // g_misc.c
        //

        #[no_mangle]
        pub fn trap_SetBrushModel(ent: *mut gentity_t, name: *const libc::c_char);
    }
    pub const MOVER_2TO1: moverState_t = 3;
    pub const CON_CONNECTED: clientConnected_t = 2;
    pub type gclient_t = gclient_s;
    extern "C" {
        #[no_mangle]
        pub fn trap_PointContents(point: *const vec_t, passEntityNum: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_PointContents(point: *mut vec_t) -> libc::c_int;
        #[no_mangle]
        pub fn trap_AAS_AreaReachability(areanum: libc::c_int) -> libc::c_int;

        #[no_mangle]
        pub fn trap_AAS_Swimming(origin: *mut vec_t) -> libc::c_int;
        #[no_mangle]
        pub fn trap_EA_Action(client: libc::c_int, action: libc::c_int);
        #[no_mangle]
        pub fn trap_EA_Gesture(client: libc::c_int);
        #[no_mangle]
        pub fn trap_EA_Talk(client: libc::c_int);
        #[no_mangle]
        pub fn trap_EA_Attack(client: libc::c_int);
        #[no_mangle]
        pub fn trap_EA_Respawn(client: libc::c_int);
        #[no_mangle]
        pub fn trap_EA_Crouch(client: libc::c_int);
        #[no_mangle]
        pub fn trap_Characteristic_BFloat(
            character: libc::c_int,
            index: libc::c_int,
            min: libc::c_float,
            max: libc::c_float,
        ) -> libc::c_float;

        #[no_mangle]
        pub fn trap_BotResetGoalState(goalstate: libc::c_int);
        #[no_mangle]
        pub fn trap_BotResetAvoidGoals(goalstate: libc::c_int);
        #[no_mangle]
        pub fn trap_BotPushGoal(goalstate: libc::c_int, goal: *mut libc::c_void);
        #[no_mangle]
        pub fn trap_BotPopGoal(goalstate: libc::c_int);
        #[no_mangle]
        pub fn trap_BotEmptyGoalStack(goalstate: libc::c_int);
        #[no_mangle]
        pub fn trap_BotGoalName(number: libc::c_int, name: *mut libc::c_char, size: libc::c_int);
        #[no_mangle]
        pub fn trap_BotGetTopGoal(goalstate: libc::c_int, goal: *mut libc::c_void) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotGetSecondGoal(
            goalstate: libc::c_int,
            goal: *mut libc::c_void,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotChooseLTGItem(
            goalstate: libc::c_int,
            origin: *mut vec_t,
            inventory: *mut libc::c_int,
            travelflags: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotChooseNBGItem(
            goalstate: libc::c_int,
            origin: *mut vec_t,
            inventory: *mut libc::c_int,
            travelflags: libc::c_int,
            ltg: *mut libc::c_void,
            maxtime: libc::c_float,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotTouchingGoal(origin: *mut vec_t, goal: *mut libc::c_void) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotItemGoalInVisButNotVisible(
            viewer: libc::c_int,
            eye: *mut vec_t,
            viewangles: *mut vec_t,
            goal: *mut libc::c_void,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotSetAvoidGoalTime(
            goalstate: libc::c_int,
            number: libc::c_int,
            avoidtime: libc::c_float,
        );
        #[no_mangle]
        pub fn trap_BotResetMoveState(movestate: libc::c_int);
        #[no_mangle]
        pub fn trap_BotMoveToGoal(
            result: *mut libc::c_void,
            movestate: libc::c_int,
            goal: *mut libc::c_void,
            travelflags: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_BotMoveInDirection(
            movestate: libc::c_int,
            dir: *mut vec_t,
            speed: libc::c_float,
            type_0: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotResetAvoidReach(movestate: libc::c_int);
        #[no_mangle]
        pub fn trap_BotResetLastAvoidReach(movestate: libc::c_int);
        #[no_mangle]
        pub fn trap_BotMovementViewTarget(
            movestate: libc::c_int,
            goal: *mut libc::c_void,
            travelflags: libc::c_int,
            lookahead: libc::c_float,
            target: *mut vec_t,
        ) -> libc::c_int;
    }
    extern "C" {

        #[no_mangle]
        pub fn trap_GetConfigstring(
            num: libc::c_int,
            buffer: *mut libc::c_char,
            bufferSize: libc::c_int,
        );
        #[no_mangle]
        pub fn trap_AAS_AreaTravelTimeToGoalArea(
            areanum: libc::c_int,
            origin: *mut vec_t,
            goalareanum: libc::c_int,
            travelflags: libc::c_int,
        ) -> libc::c_int;
        #[no_mangle]
        pub fn trap_BotQueueConsoleMessage(
            chatstate: libc::c_int,
            type_0: libc::c_int,
            message: *mut libc::c_char,
        );
        #[no_mangle]
        pub fn trap_BotEnterChat(chatstate: libc::c_int, client: libc::c_int, sendto: libc::c_int);
        #[no_mangle]
        pub fn trap_BotGetChatMessage(
            chatstate: libc::c_int,
            buf: *mut libc::c_char,
            size: libc::c_int,
        );
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct playerTeamState_t {
        pub state: playerTeamStateState_t,
        pub location: libc::c_int,
        pub captures: libc::c_int,
        pub basedefense: libc::c_int,
        pub carrierdefense: libc::c_int,
        pub flagrecovery: libc::c_int,
        pub fragcarrier: libc::c_int,
        pub assists: libc::c_int,
        pub lasthurtcarrier: libc::c_float,
        pub lastreturnedflag: libc::c_float,
        pub flagsince: libc::c_float,
        pub lastfraggedcarrier: libc::c_float,
    }
    extern "C" {

        #[no_mangle]
        pub fn trap_Cvar_VariableStringBuffer(
            var_name: *const libc::c_char,
            buffer: *mut libc::c_char,
            bufsize: libc::c_int,
        );

        #[no_mangle]
        pub fn trap_Cvar_Set(var_name: *const libc::c_char, value: *const libc::c_char);
    // allow this many active

    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct gentity_s {
        pub s: entityState_t,
        pub r: entityShared_t,
        pub client: *mut gclient_s,
        pub inuse: qboolean,
        pub classname: *mut libc::c_char,
        pub spawnflags: libc::c_int,
        pub neverFree: qboolean,
        pub flags: libc::c_int,
        pub model: *mut libc::c_char,
        pub model2: *mut libc::c_char,
        pub freetime: libc::c_int,
        pub eventTime: libc::c_int,
        pub freeAfterEvent: qboolean,
        pub unlinkAfterEvent: qboolean,
        pub physicsObject: qboolean,
        pub physicsBounce: libc::c_float,
        pub clipmask: libc::c_int,
        pub moverState: moverState_t,
        pub soundPos1: libc::c_int,
        pub sound1to2: libc::c_int,
        pub sound2to1: libc::c_int,
        pub soundPos2: libc::c_int,
        pub soundLoop: libc::c_int,
        pub parent: *mut gentity_t,
        pub nextTrain: *mut gentity_t,
        pub prevTrain: *mut gentity_t,
        pub pos1: vec3_t,
        pub pos2: vec3_t,
        pub message: *mut libc::c_char,
        pub timestamp: libc::c_int,
        pub target: *mut libc::c_char,
        pub targetname: *mut libc::c_char,
        pub team: *mut libc::c_char,
        pub targetShaderName: *mut libc::c_char,
        pub targetShaderNewName: *mut libc::c_char,
        pub target_ent: *mut gentity_t,
        pub speed: libc::c_float,
        pub movedir: vec3_t,
        pub nextthink: libc::c_int,
        pub think: Option<unsafe extern "C" fn(_: *mut gentity_t) -> ()>,
        pub reached: Option<unsafe extern "C" fn(_: *mut gentity_t) -> ()>,
        pub blocked: Option<unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t) -> ()>,
        pub touch: Option<
            unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut trace_t) -> (),
        >,
        pub use_0: Option<
            unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: *mut gentity_t) -> (),
        >,
        pub pain: Option<
            unsafe extern "C" fn(_: *mut gentity_t, _: *mut gentity_t, _: libc::c_int) -> (),
        >,
        pub die: Option<
            unsafe extern "C" fn(
                _: *mut gentity_t,
                _: *mut gentity_t,
                _: *mut gentity_t,
                _: libc::c_int,
                _: libc::c_int,
            ) -> (),
        >,
        pub pain_debounce_time: libc::c_int,
        pub fly_sound_debounce_time: libc::c_int,
        pub last_move_time: libc::c_int,
        pub health: libc::c_int,
        pub takedamage: qboolean,
        pub damage: libc::c_int,
        pub splashDamage: libc::c_int,
        pub splashRadius: libc::c_int,
        pub methodOfDeath: libc::c_int,
        pub splashMethodOfDeath: libc::c_int,
        pub count: libc::c_int,
        pub chain: *mut gentity_t,
        pub enemy: *mut gentity_t,
        pub activator: *mut gentity_t,
        pub teamchain: *mut gentity_t,
        pub teammaster: *mut gentity_t,
        pub watertype: libc::c_int,
        pub waterlevel: libc::c_int,
        pub noise_index: libc::c_int,
        pub wait: libc::c_float,
        pub random: libc::c_float,
        pub item: *mut gitem_t,
    }
    pub const MOVER_1TO2: moverState_t = 2;
    pub const SPECTATOR_SCOREBOARD: spectatorState_t = 3;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct clientSession_t {
        pub sessionTeam: team_t,
        pub spectatorNum: libc::c_int,
        pub spectatorState: spectatorState_t,
        pub spectatorClient: libc::c_int,
        pub wins: libc::c_int,
        pub losses: libc::c_int,
        pub teamLeader: qboolean,
    }
    pub const SPECTATOR_NOT: spectatorState_t = 0;
    pub const TEAM_BEGIN: playerTeamStateState_t = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct level_locals_t {
        pub clients: *mut gclient_s,
        pub gentities: *mut gentity_s,
        pub gentitySize: libc::c_int,
        pub num_entities: libc::c_int,
        pub warmupTime: libc::c_int,
        pub logFile: fileHandle_t,
        pub maxclients: libc::c_int,
        pub framenum: libc::c_int,
        pub time: libc::c_int,
        pub previousTime: libc::c_int,
        pub startTime: libc::c_int,
        pub teamScores: [libc::c_int; 4],
        pub lastTeamLocationTime: libc::c_int,
        pub newSession: qboolean,
        pub restarted: qboolean,
        pub numConnectedClients: libc::c_int,
        pub numNonSpectatorClients: libc::c_int,
        pub numPlayingClients: libc::c_int,
        pub sortedClients: [libc::c_int; 64],
        pub follow1: libc::c_int,
        pub follow2: libc::c_int,
        pub snd_fry: libc::c_int,
        pub warmupModificationCount: libc::c_int,
        pub voteString: [libc::c_char; 1024],
        pub voteDisplayString: [libc::c_char; 1024],
        pub voteTime: libc::c_int,
        pub voteExecuteTime: libc::c_int,
        pub voteYes: libc::c_int,
        pub voteNo: libc::c_int,
        pub numVotingClients: libc::c_int,
        pub teamVoteString: [[libc::c_char; 1024]; 2],
        pub teamVoteTime: [libc::c_int; 2],
        pub teamVoteYes: [libc::c_int; 2],
        pub teamVoteNo: [libc::c_int; 2],
        pub numteamVotingClients: [libc::c_int; 2],
        pub spawning: qboolean,
        pub numSpawnVars: libc::c_int,
        pub spawnVars: [[*mut libc::c_char; 2]; 64],
        pub numSpawnVarChars: libc::c_int,
        pub spawnVarChars: [libc::c_char; 4096],
        pub intermissionQueued: libc::c_int,
        pub intermissiontime: libc::c_int,
        pub changemap: *mut libc::c_char,
        pub readyToExit: qboolean,
        pub exitTime: libc::c_int,
        pub intermission_origin: vec3_t,
        pub intermission_angle: vec3_t,
        pub locationLinked: qboolean,
        pub locationHead: *mut gentity_t,
        pub bodyQueIndex: libc::c_int,
        pub bodyQue: [*mut gentity_t; 8],
    }
    pub type playerTeamStateState_t = libc::c_uint;
    pub type gentity_t = gentity_s;
    pub const TEAM_ACTIVE: playerTeamStateState_t = 1;
    pub type spectatorState_t = libc::c_uint;
    pub const SPECTATOR_FOLLOW: spectatorState_t = 2;
    pub type moverState_t = libc::c_uint;
    pub type clientConnected_t = libc::c_uint;
    pub const CON_DISCONNECTED: clientConnected_t = 0;
    use libc;
}
mod stddef_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    pub type size_t = libc::c_ulong;
    use libc;
}
mod g_variadic_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_local_h::gentity_t;
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    extern "C" {

        #[no_mangle]
        pub fn PrintMsg(ent: *mut gentity_t, fmt: *const libc::c_char, ...);
    }
    extern "C" {
        #[no_mangle]
        pub fn G_LogPrintf(fmt: *const libc::c_char, ...);

    }
    extern "C" {
        #[no_mangle]
        pub fn G_Error(fmt: *const libc::c_char, ...) -> !;
        #[no_mangle]
        pub fn G_Printf(fmt: *const libc::c_char, ...);
    }
    use libc;
}
mod be_ai_move_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    use q_shared_h::vec3_t;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bot_initmove_s {
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub viewoffset: vec3_t,
        pub entitynum: libc::c_int,
        pub client: libc::c_int,
        pub thinktime: libc::c_float,
        pub presencetype: libc::c_int,
        pub viewangles: vec3_t,
        pub or_moveflags: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bot_moveresult_s {
        pub failure: libc::c_int,
        pub type_0: libc::c_int,
        pub blocked: libc::c_int,
        pub blockentity: libc::c_int,
        pub traveltype: libc::c_int,
        pub flags: libc::c_int,
        pub weapon: libc::c_int,
        pub movedir: vec3_t,
        pub ideal_viewangles: vec3_t,
    }
    pub type bot_moveresult_t = bot_moveresult_s;
    pub type bot_initmove_t = bot_initmove_s;
    use libc;
}
mod be_aas_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    use q_shared_h::{qboolean, vec3_t};
    pub type aas_trace_t = aas_trace_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_clientmove_s {
        pub endpos: vec3_t,
        pub endarea: libc::c_int,
        pub velocity: vec3_t,
        pub trace: aas_trace_t,
        pub presencetype: libc::c_int,
        pub stopevent: libc::c_int,
        pub endcontents: libc::c_int,
        pub time: libc::c_float,
        pub frames: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_predictroute_s {
        pub endpos: vec3_t,
        pub endarea: libc::c_int,
        pub stopevent: libc::c_int,
        pub endcontents: libc::c_int,
        pub endtravelflags: libc::c_int,
        pub numareas: libc::c_int,
        pub time: libc::c_int,
    }
    pub const SOLID_BBOX: unnamed_3 = 2;
    pub type aas_altroutegoal_t = aas_altroutegoal_s;
    pub type aas_areainfo_t = aas_areainfo_s;
    pub type aas_clientmove_t = aas_clientmove_s;
    pub type unnamed_3 = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_altroutegoal_s {
        pub origin: vec3_t,
        pub areanum: libc::c_int,
        pub starttraveltime: libc::c_ushort,
        pub goaltraveltime: libc::c_ushort,
        pub extratraveltime: libc::c_ushort,
    }
    pub const SOLID_NOT: unnamed_3 = 0;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_areainfo_s {
        pub contents: libc::c_int,
        pub flags: libc::c_int,
        pub presencetype: libc::c_int,
        pub cluster: libc::c_int,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub center: vec3_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_entityinfo_s {
        pub valid: libc::c_int,
        pub type_0: libc::c_int,
        pub flags: libc::c_int,
        pub ltime: libc::c_float,
        pub update_time: libc::c_float,
        pub number: libc::c_int,
        pub origin: vec3_t,
        pub angles: vec3_t,
        pub old_origin: vec3_t,
        pub lastvisorigin: vec3_t,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub groundent: libc::c_int,
        pub solid: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub frame: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
    }
    pub type aas_predictroute_t = aas_predictroute_s;
    pub const SOLID_BSP: unnamed_3 = 3;
    pub const SOLID_TRIGGER: unnamed_3 = 1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct aas_trace_s {
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub ent: libc::c_int,
        pub lastarea: libc::c_int,
        pub area: libc::c_int,
        pub planenum: libc::c_int,
    }
    pub type aas_entityinfo_t = aas_entityinfo_s;
    use libc;
}
mod botlib_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    use q_shared_h::{cplane_t, qboolean, vec3_t};

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bot_input_s {
        pub thinktime: libc::c_float,
        pub dir: vec3_t,
        pub speed: libc::c_float,
        pub viewangles: vec3_t,
        pub actionflags: libc::c_int,
        pub weapon: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bot_entitystate_s {
        pub type_0: libc::c_int,
        pub flags: libc::c_int,
        pub origin: vec3_t,
        pub angles: vec3_t,
        pub old_origin: vec3_t,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub groundent: libc::c_int,
        pub solid: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub frame: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bsp_trace_s {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub exp_dist: libc::c_float,
        pub sidenum: libc::c_int,
        pub surface: bsp_surface_t,
        pub contents: libc::c_int,
        pub ent: libc::c_int,
    }
    pub type bsp_surface_t = bsp_surface_s;
    pub type bsp_trace_t = bsp_trace_s;
    pub type bot_entitystate_t = bot_entitystate_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bsp_surface_s {
        pub name: [libc::c_char; 16],
        pub flags: libc::c_int,
        pub value: libc::c_int,
    }
    pub type bot_input_t = bot_input_s;
    use libc;
}
mod bg_public_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    use q_shared_h::{
        entityState_t, playerState_t, qboolean, trace_t, trajectory_t, usercmd_t, vec3_t, vec_t,
    };
    pub const TEAMTASK_ESCORT: unnamed_8 = 6;
    pub type holdable_t = libc::c_uint;
    pub const TEAMTASK_RETRIEVE: unnamed_8 = 5;
    pub type unnamed_8 = libc::c_uint;
    pub const GTS_TEAMS_ARE_TIED: unnamed_7 = 12;
    pub const TEAMTASK_DEFENSE: unnamed_8 = 2;
    pub const GTS_BLUE_CAPTURE: unnamed_7 = 1;
    pub const GTS_REDOBELISK_ATTACKED: unnamed_7 = 6;
    pub const GTS_RED_CAPTURE: unnamed_7 = 0;
    pub const TORSO_STAND: unnamed_6 = 11;
    pub type unnamed_9 = libc::c_uint;
    pub const TEAMTASK_OFFENSE: unnamed_8 = 1;
    pub const GTS_KAMIKAZE: unnamed_4 = 13;
    pub const LEGS_BACK: unnamed_6 = 16;
    pub type powerup_t = libc::c_uint;
    pub const TEAMTASK_NONE: unnamed_8 = 0;
    pub const HI_PORTAL: unnamed_2 = 4;
    pub const MOD_GAUNTLET: unnamed_6 = 2;
    pub const TEAMTASK_PATROL: unnamed_8 = 3;
    pub const LEGS_TURN: unnamed_6 = 24;
    pub const BOTH_DEATH1: unnamed_6 = 0;
    pub const TORSO_GETFLAG: unnamed_7 = 25;
    pub const LEGS_LANDB: unnamed_6 = 21;
    pub const LEGS_IDLECR: unnamed_7 = 23;
    pub const TORSO_ATTACK: unnamed_4 = 7;
    pub const GTS_BLUE_TAKEN: unnamed_4 = 5;
    pub const PW_DOUBLER: unnamed_2 = 12;
    pub const GTS_BLUETEAM_TOOK_LEAD: unnamed_4 = 11;
    pub const PM_INTERMISSION: unnamed_0 = 5;
    pub const TEAMTASK_CAMP: unnamed_8 = 7;
    pub const GTS_REDTEAM_SCORED: unnamed_7 = 8;
    pub const HI_KAMIKAZE: holdable_t = 3;
    pub const TORSO_DROP: unnamed_5 = 9;
    pub const TEAMTASK_FOLLOW: unnamed_8 = 4;
    pub const WEAPON_FIRING: unnamed_1 = 3;
    pub const TORSO_NEGATIVE: unnamed_7 = 30;
    pub const BOTH_DEATH2: unnamed_5 = 2;
    pub const MOD_MACHINEGUN: unnamed_1 = 3;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct pmove_t {
        pub ps: *mut playerState_t,
        pub cmd: usercmd_t,
        pub tracemask: libc::c_int,
        pub debugLevel: libc::c_int,
        pub noFootsteps: qboolean,
        pub gauntletHit: qboolean,
        pub framecount: libc::c_int,
        pub numtouch: libc::c_int,
        pub touchents: [libc::c_int; 32],
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub watertype: libc::c_int,
        pub waterlevel: libc::c_int,
        pub xyspeed: libc::c_float,
        pub pmove_fixed: libc::c_int,
        pub pmove_msec: libc::c_int,
        pub trace: Option<
            unsafe extern "C" fn(
                _: *mut trace_t,
                _: *const vec_t,
                _: *const vec_t,
                _: *const vec_t,
                _: *const vec_t,
                _: libc::c_int,
                _: libc::c_int,
            ) -> (),
        >,
        pub pointcontents:
            Option<unsafe extern "C" fn(_: *const vec_t, _: libc::c_int) -> libc::c_int>,
    }
    pub const GTS_RED_TAKEN: unnamed_7 = 4;
    pub type weapon_t = libc::c_uint;
    pub const GTS_RED_RETURN: unnamed_4 = 2;
    pub const MOD_ROCKET_SPLASH: unnamed_2 = 7;
    pub const EV_GIB_PLAYER: unnamed_0 = 64;
    pub const TORSO_ATTACK2: unnamed_5 = 8;
    pub const MAX_TOTALANIMATIONS: unnamed_6 = 37;
    pub const PW_BLUEFLAG: powerup_t = 8;
    pub const PW_NUM_POWERUPS: powerup_t = 15;
    pub const HI_NONE: holdable_t = 0;
    pub const STAT_MAX_HEALTH: unnamed = 6;
    pub const PM_SPINTERMISSION: unnamed_1 = 6;
    pub const MOD_TARGET_LASER: unnamed_1 = 21;
    pub const HI_INVULNERABILITY: holdable_t = 5;
    pub const PW_AMMOREGEN: unnamed_1 = 13;
    pub type unnamed_6 = libc::c_uint;
    pub const PERS_ATTACKEE_ARMOR: unnamed_0 = 7;
    pub const TORSO_GESTURE: unnamed_5 = 6;
    pub type unnamed_7 = libc::c_uint;
    pub const HI_TELEPORTER: holdable_t = 1;
    pub const FLAG_STAND2RUN: unnamed_6 = 36;
    pub const PM_FREEZE: unnamed_1 = 4;
    pub const PM_DEAD: unnamed_1 = 3;
    pub const TORSO_GUARDBASE: unnamed_5 = 26;
    pub const EV_PROXIMITY_MINE_TRIGGER: unnamed = 67;
    pub const GTS_REDTEAM_TOOK_LEAD: unnamed_7 = 10;
    pub type unnamed_3 = libc::c_uint;
    pub const WEAPON_READY: unnamed_1 = 0;
    pub const EV_MISSILE_MISS_METAL: unnamed = 52;
    pub const STAT_DEAD_YAW: unnamed_2 = 4;
    pub const IT_POWERUP: itemType_t = 5;
    pub type unnamed = libc::c_uint;
    pub const BOTH_DEAD1: unnamed_6 = 1;
    pub const PW_HASTE: powerup_t = 3;
    pub type unnamed_0 = libc::c_uint;
    pub const WP_NONE: weapon_t = 0;
    pub const FLAG_STAND: unnamed_5 = 35;
    pub const STAT_ARMOR: unnamed_2 = 3;
    pub const GTS_BLUEOBELISK_ATTACKED: unnamed_7 = 7;
    pub const EV_PROXIMITY_MINE_STICK: unnamed = 66;
    pub const EV_POWERUP_BATTLESUIT: unnamed_0 = 62;
    pub const LEGS_JUMP: unnamed_6 = 18;
    pub const PW_SCOUT: powerup_t = 10;
    pub const GTS_BLUETEAM_SCORED: unnamed_7 = 9;
    pub const EV_WATER_UNDER: unnamed = 17;
    pub const PW_GUARD: unnamed = 11;
    pub const PW_NONE: unnamed_4 = 0;
    pub const PW_QUAD: unnamed_2 = 1;
    pub const EV_PLAYER_TELEPORT_OUT: unnamed_0 = 43;
    pub const PM_SPECTATOR: unnamed_1 = 2;
    pub const PERS_HITS: unnamed_1 = 1;
    pub const PERS_SCORE: unnamed_1 = 0;
    pub const EV_FIRE_WEAPON: unnamed_0 = 23;
    pub const EV_BULLET: unnamed_0 = 55;
    pub const PW_INVIS: unnamed = 4;
    pub const LEGS_BACKCR: unnamed_6 = 32;
    pub const LEGS_LAND: unnamed_4 = 19;
    pub const WP_GRENADE_LAUNCHER: unnamed_2 = 4;
    pub type unnamed_2 = libc::c_uint;
    pub const PERS_TEAM: unnamed_1 = 3;
    pub const WP_NUM_WEAPONS: weapon_t = 11;
    pub const PM_NORMAL: unnamed_0 = 0;
    pub const TORSO_FOLLOWME: unnamed_4 = 28;
    pub type unnamed_1 = libc::c_uint;
    pub const MAX_ANIMATIONS: unnamed_7 = 31;
    pub const EV_USE_ITEM12: unnamed_4 = 36;
    pub const LEGS_JUMPB: unnamed_5 = 20;
    pub const STAT_WEAPONS: unnamed_0 = 2;
    pub const EV_MISSILE_HIT: unnamed_6 = 50;
    pub const EV_USE_ITEM13: unnamed = 37;
    pub const LEGS_IDLE: unnamed_6 = 22;
    pub const TEAM_BLUE: team_t = 2;
    pub const EV_TAUNT: unnamed_5 = 76;
    pub const PW_INVULNERABILITY: unnamed_2 = 14;
    pub const EV_NOAMMO: unnamed_4 = 21;
    pub const ET_PLAYER: unnamed_4 = 1;
    pub const MOD_BFG: unnamed_6 = 12;
    pub const ET_TEAM: unnamed_6 = 12;
    pub const MOD_UNKNOWN: unnamed_1 = 0;
    pub const ET_PUSH_TRIGGER: unnamed_6 = 8;
    pub const EV_WATER_TOUCH: unnamed_6 = 15;
    pub const PERS_SPAWN_COUNT: unnamed_1 = 4;
    pub const EV_CHANGE_WEAPON: unnamed_4 = 22;
    pub const TORSO_AFFIRMATIVE: unnamed_7 = 29;
    pub const TORSO_RAISE: unnamed_7 = 10;
    pub const EV_NONE: unnamed = 0;
    pub const EV_RAILTRAIL: unnamed_2 = 53;
    pub const GT_HARVESTER: unnamed = 7;
    pub const EV_USE_ITEM3: unnamed_0 = 27;
    pub const EV_GLOBAL_ITEM_PICKUP: unnamed_2 = 20;
    pub const EV_FOOTSPLASH: unnamed_0 = 3;
    pub const EV_OBELISKPAIN: unnamed_3 = 70;
    pub const LEGS_WALK: unnamed_5 = 14;
    pub const PERS_ASSIST_COUNT: unnamed_1 = 12;
    pub const EV_DEATH1: unnamed_3 = 57;
    pub const ET_GRAPPLE: unnamed_7 = 11;
    pub const IT_PERSISTANT_POWERUP: itemType_t = 7;
    pub const WEAPON_RAISING: unnamed_1 = 1;
    pub const TORSO_STAND2: unnamed_4 = 12;
    pub const GT_OBELISK: unnamed_0 = 6;
    pub const ET_TELEPORT_TRIGGER: unnamed_4 = 9;
    pub const PW_FLIGHT: unnamed_4 = 6;
    pub const EV_TAUNT_GETFLAG: unnamed_4 = 80;
    pub const IT_WEAPON: itemType_t = 1;
    pub const IT_ARMOR: itemType_t = 3;
    pub const TEAM_SPECTATOR: unnamed_1 = 3;
    pub const GT_SINGLE_PLAYER: unnamed = 2;
    pub const WP_ROCKET_LAUNCHER: unnamed_1 = 5;
    pub const PERS_RANK: unnamed_1 = 2;
    pub const MOD_PLASMA: unnamed_5 = 8;
    pub const EV_FALL_FAR: unnamed = 12;
    pub const PERS_CAPTURES: unnamed_3 = 14;
    pub const PW_REGEN: unnamed_2 = 5;
    pub const PERS_GAUNTLET_FRAG_COUNT: unnamed_3 = 13;
    pub const PW_NEUTRALFLAG: unnamed_4 = 9;
    pub const STAT_HOLDABLE_ITEM: unnamed_2 = 1;
    pub const EV_PAIN: unnamed = 56;
    pub const BOTH_DEAD3: unnamed_5 = 5;
    pub const ET_SPEAKER: unnamed_6 = 7;
    pub const MOD_FALLING: unnamed_3 = 19;
    pub const EV_BULLET_HIT_WALL: unnamed_2 = 49;
    pub const EV_USE_ITEM9: unnamed_6 = 33;
    pub const MOD_SUICIDE: unnamed_6 = 20;
    pub const MOD_GRENADE_SPLASH: unnamed_0 = 5;
    pub const EV_TAUNT_GUARDBASE: unnamed_3 = 81;
    pub const EV_ITEM_PICKUP: unnamed = 19;
    pub const WEAPON_DROPPING: unnamed_1 = 2;
    pub const TEAM_FREE: team_t = 0;
    pub const EV_GENERAL_SOUND: unnamed_4 = 45;
    pub const HI_MEDKIT: unnamed_2 = 2;
    pub const EV_FALL_SHORT: unnamed_5 = 10;
    pub const MOD_GRAPPLE: unnamed_1 = 23;
    pub type unnamed_4 = libc::c_uint;
    pub const MOD_LIGHTNING: unnamed_5 = 11;
    pub const WP_BFG: weapon_t = 9;
    pub const ET_MISSILE: unnamed_0 = 3;
    pub const EV_LIGHTNINGBOLT: unnamed_0 = 73;
    pub const EV_STEP_4: unnamed_3 = 6;
    pub const STAT_CLIENTS_READY: unnamed_0 = 5;
    pub const EV_USE_ITEM10: unnamed_6 = 34;
    pub const TORSO_PATROL: unnamed_5 = 27;
    pub const MOD_PLASMA_SPLASH: unnamed_3 = 9;
    pub const GT_MAX_GAME_TYPE: unnamed = 8;
    pub const BOTH_DEAD2: unnamed_5 = 3;
    pub const PW_BATTLESUIT: unnamed_2 = 2;
    pub const EV_FOOTSTEP_METAL: unnamed_4 = 2;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct gitem_s {
        pub classname: *mut libc::c_char,
        pub pickup_sound: *mut libc::c_char,
        pub world_model: [*mut libc::c_char; 4],
        pub icon: *mut libc::c_char,
        pub pickup_name: *mut libc::c_char,
        pub quantity: libc::c_int,
        pub giType: itemType_t,
        pub giTag: libc::c_int,
        pub precaches: *mut libc::c_char,
        pub sounds: *mut libc::c_char,
    }
    pub const IT_BAD: itemType_t = 0;
    pub const EV_OBELISKEXPLODE: unnamed_3 = 69;
    pub const EV_USE_ITEM11: unnamed_4 = 35;
    pub const EV_SHOTGUN: unnamed_3 = 54;
    pub const EV_USE_ITEM6: unnamed_5 = 30;
    pub const EV_JUMP: unnamed_0 = 14;
    pub const PERS_KILLED: unnamed_3 = 8;
    pub const MOD_ROCKET: unnamed_3 = 6;
    pub const MOD_RAILGUN: unnamed_1 = 10;
    pub const EV_USE_ITEM1: unnamed_3 = 25;
    pub const STAT_HEALTH: unnamed_0 = 0;
    pub const EV_TAUNT_PATROL: unnamed = 82;
    pub const EV_SWIM: unnamed = 5;
    pub const ET_ITEM: unnamed_1 = 2;
    pub const WP_PLASMAGUN: unnamed_2 = 8;
    pub const EV_PLAYER_TELEPORT_IN: unnamed_0 = 42;
    pub const EV_TAUNT_FOLLOWME: unnamed_0 = 79;
    pub const LEGS_SWIM: unnamed_4 = 17;
    pub const MOD_SLIME: unnamed_1 = 15;
    pub const EV_KAMIKAZE: unnamed_0 = 68;
    pub const EV_DEBUG_LINE: unnamed_0 = 74;
    pub const EV_USE_ITEM15: unnamed_4 = 39;
    pub const EV_JUICED: unnamed = 72;
    pub const EV_STEP_16: unnamed_3 = 9;
    pub const IT_HOLDABLE: itemType_t = 6;
    pub const EV_GLOBAL_SOUND: unnamed_0 = 46;
    pub const WP_GAUNTLET: weapon_t = 1;
    pub const LEGS_RUN: unnamed_5 = 15;
    pub const EV_STOPLOOPINGSOUND: unnamed_3 = 75;
    pub const BOTH_DEATH3: unnamed_5 = 4;
    pub const EV_DEATH3: unnamed_4 = 59;
    pub const EV_USE_ITEM5: unnamed = 29;
    pub const EV_WATER_CLEAR: unnamed_6 = 18;
    pub const EV_POWERUP_REGEN: unnamed_6 = 63;
    pub const PERS_DEFEND_COUNT: unnamed_0 = 11;
    pub const EV_FALL_MEDIUM: unnamed_4 = 11;
    pub const EV_JUMP_PAD: unnamed = 13;
    pub const PERS_PLAYEREVENTS: unnamed_0 = 5;
    pub const IT_HEALTH: itemType_t = 4;
    pub const ET_MOVER: unnamed_0 = 4;
    pub const TEAM_NUM_TEAMS: team_t = 4;
    pub const WP_MACHINEGUN: weapon_t = 2;
    pub const GT_TEAM: unnamed_0 = 3;
    pub type itemType_t = libc::c_uint;
    pub const ET_GENERAL: unnamed_4 = 0;
    pub const EV_BULLET_HIT_FLESH: unnamed_0 = 48;
    pub type unnamed_5 = libc::c_uint;
    pub const EV_DEATH2: unnamed = 58;
    pub const IT_AMMO: itemType_t = 2;
    pub const EV_POWERUP_QUAD: unnamed_3 = 61;
    pub const PM_NOCLIP: unnamed_1 = 1;
    pub const WP_LIGHTNING: unnamed_1 = 6;
    pub const MOD_TELEFRAG: unnamed_5 = 18;
    pub const EV_WATER_LEAVE: unnamed_0 = 16;
    pub const MOD_CRUSH: unnamed_1 = 17;
    pub const PW_REDFLAG: unnamed_2 = 7;
    pub const GT_CTF: unnamed = 4;
    pub const GTS_BLUE_RETURN: unnamed_7 = 3;
    pub const EV_STEP_12: unnamed = 8;
    pub const EV_MISSILE_MISS: unnamed_0 = 51;
    pub const ET_INVISIBLE: unnamed_2 = 10;
    pub const EV_USE_ITEM8: unnamed_6 = 32;
    pub const EV_STEP_8: unnamed = 7;
    pub const MOD_WATER: unnamed_0 = 14;
    pub const EV_FOOTWADE: unnamed_4 = 4;
    pub const LEGS_WALKCR: unnamed_5 = 13;
    pub const PERS_ATTACKER: unnamed_0 = 6;
    pub const MOD_GRENADE: unnamed_1 = 4;
    pub const EV_TAUNT_NO: unnamed_4 = 78;
    pub const EV_FOOTSTEP: unnamed = 1;
    pub const WP_SHOTGUN: unnamed_4 = 3;
    pub const EV_GRENADE_BOUNCE: unnamed = 44;
    pub const TEAM_RED: team_t = 1;
    pub const ET_EVENTS: unnamed_2 = 13;
    pub const MOD_SHOTGUN: unnamed_6 = 1;
    pub const MOD_TRIGGER_HURT: unnamed_5 = 22;
    pub const EV_SCOREPLUM: unnamed_0 = 65;
    pub const ET_PORTAL: unnamed_3 = 6;
    pub const EV_ITEM_POP: unnamed = 41;
    pub const EV_USE_ITEM0: unnamed_0 = 24;
    pub const EV_GLOBAL_TEAM_SOUND: unnamed_0 = 47;
    pub type team_t = libc::c_uint;
    pub const PERS_EXCELLENT_COUNT: unnamed_3 = 10;
    pub const EV_USE_ITEM4: unnamed_5 = 28;
    pub const EV_OBITUARY: unnamed_3 = 60;
    pub const EV_ITEM_RESPAWN: unnamed = 40;
    pub const GT_1FCTF: unnamed = 5;
    pub const HI_NUM_HOLDABLE: unnamed_2 = 6;
    pub const MOD_LAVA: unnamed_5 = 16;
    pub const EV_TAUNT_YES: unnamed_3 = 77;
    pub const MOD_BFG_SPLASH: unnamed_0 = 13;
    pub const WP_RAILGUN: unnamed_1 = 7;
    pub const ET_BEAM: unnamed_2 = 5;
    pub const GT_TOURNAMENT: unnamed = 1;
    pub const PERS_IMPRESSIVE_COUNT: unnamed_3 = 9;
    pub type gitem_t = gitem_s;
    pub const IT_TEAM: itemType_t = 8;
    pub const GT_FFA: unnamed_1 = 0;
    pub const EV_INVUL_IMPACT: unnamed = 71;
    pub const LEGS_BACKWALK: unnamed_5 = 33;
    pub const FLAG_RUN: unnamed_4 = 34;
    pub const EV_USE_ITEM7: unnamed_5 = 31;
    pub const EV_USE_ITEM2: unnamed_4 = 26;
    pub const WP_GRAPPLING_HOOK: unnamed_3 = 10;
    pub const EV_USE_ITEM14: unnamed_0 = 38;
    use libc;
}
mod q_shared_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    pub const CHAN_AUTO: unnamed = 0;
    pub type _flag_status = libc::c_uint;
    pub const CHAN_ANNOUNCER: unnamed = 7;
    pub const ERR_NEED_CD: unnamed = 4;
    extern "C" {

        // portable case insensitive compare

        // buffer size safe library replacements

        //=============================================
        /*
        short	BigShort(short l);
        short	LittleShort(short l);
        int		BigLong (int l);
        int		LittleLong (int l);
        qint64  BigLong64 (qint64 l);
        qint64  LittleLong64 (qint64 l);
        float	BigFloat (const float *l);
        float	LittleFloat (const float *l);

        void	Swap_Init (void);
        */

        //=============================================
        //
        // key / value info strings
        //

        #[no_mangle]
        pub fn Info_Validate(s: *const libc::c_char) -> qboolean;
    }
    pub const CHAN_LOCAL_SOUND: unnamed = 6;
    extern "C" {
        #[no_mangle]
        pub fn Q_IsColorString(p: *const libc::c_char) -> qboolean;

    // portable case insensitive compare

    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    //=============================================
    //
    // key / value info strings
    //

    }
    pub type flagStatus_t = _flag_status;
    extern "C" {

        // portable case insensitive compare

        #[no_mangle]
        pub fn Q_stricmpn(
            s1: *const libc::c_char,
            s2: *const libc::c_char,
            n: libc::c_int,
        ) -> libc::c_int;
    // buffer size safe library replacements

    // removes color sequences from string

    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    pub const FLAG_TAKEN: _flag_status = 1;
    pub const FLAG_TAKEN_RED: _flag_status = 2;
    extern "C" {

        // portable case insensitive compare

        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...) -> !;

    }
    pub const FS_APPEND: fsMode_t = 2;
    pub const ERR_SERVERDISCONNECT: unnamed = 2;
    pub const FS_WRITE: fsMode_t = 1;
    pub const CHAN_WEAPON: unnamed = 2;
    extern "C" {

        // portable case insensitive compare

        #[no_mangle]
        pub fn Q_strncmp(
            s1: *const libc::c_char,
            s2: *const libc::c_char,
            n: libc::c_int,
        ) -> libc::c_int;
    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    pub type vec4_t = [vec_t; 4];
    pub const FLAG_ATBASE: _flag_status = 0;
    pub const CHAN_ITEM: unnamed = 4;
    pub const CHAN_BODY: unnamed = 5;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union floatint_t {
        pub f: libc::c_float,
        pub i: libc::c_int,
        pub ui: libc::c_uint,
    }
    pub const FLAG_DROPPED: _flag_status = 4;
    pub type fsMode_t = libc::c_uint;
    pub const CHAN_LOCAL: unnamed = 1;
    extern "C" {
        // portable case insensitive compare

        // buffer size safe library replacements

        #[no_mangle]
        pub fn Q_strcat(dest: *mut libc::c_char, size: libc::c_int, src: *const libc::c_char);
    //=============================================
    /*
    short	BigShort(short l);
    short	LittleShort(short l);
    int		BigLong (int l);
    int		LittleLong (int l);
    qint64  BigLong64 (qint64 l);
    qint64  LittleLong64 (qint64 l);
    float	BigFloat (const float *l);
    float	LittleFloat (const float *l);

    void	Swap_Init (void);
    */

    }
    pub const FLAG_TAKEN_BLUE: _flag_status = 3;
    pub const ERR_FATAL: unnamed = 0;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const ERR_DROP: unnamed = 1;
    pub const CHAN_VOICE: unnamed = 3;
    pub const EXEC_NOW: unnamed = 0;
    pub const ERR_DISCONNECT: unnamed = 3;
    pub type unnamed_0 = libc::c_uint;
    pub const EXEC_APPEND: unnamed = 2;
    extern "C" {
        //=============================================
        #[no_mangle]
        pub fn Com_Clamp(
            min: libc::c_float,
            max: libc::c_float,
            value: libc::c_float,
        ) -> libc::c_float;
        #[no_mangle]
        pub fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
        #[no_mangle]
        pub fn COM_ParseExt(
            data_p: *mut *mut libc::c_char,
            allowLineBreak: qboolean,
        ) -> *mut libc::c_char;
        // portable case insensitive compare

        // buffer size safe library replacements

        //=============================================
        /*
        short	BigShort(short l);
        short	LittleShort(short l);
        int		BigLong (int l);
        int		LittleLong (int l);
        qint64  BigLong64 (qint64 l);
        qint64  LittleLong64 (qint64 l);
        float	BigFloat (const float *l);
        float	LittleFloat (const float *l);

        void	Swap_Init (void);
        */

        //=============================================
        //
        // key / value info strings
        //

        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
    pub type entityState_t = entityState_s;
    extern "C" {

        #[no_mangle]
        pub fn Com_sprintf(
            dest: *mut libc::c_char,
            size: libc::c_int,
            fmt: *const libc::c_char,
            ...
        ) -> libc::c_int;
        // portable case insensitive compare
        #[no_mangle]
        pub fn Q_stricmp(s1: *const libc::c_char, s2: *const libc::c_char) -> libc::c_int;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char, destsize: libc::c_int);
        // removes color sequences from string
        #[no_mangle]
        pub fn Q_CleanStr(string: *mut libc::c_char) -> *mut libc::c_char;
        //=============================================
        /*
        short	BigShort(short l);
        short	LittleShort(short l);
        int		BigLong (int l);
        int		LittleLong (int l);
        qint64  BigLong64 (qint64 l);
        qint64  LittleLong64 (qint64 l);
        float	BigFloat (const float *l);
        float	LittleFloat (const float *l);

        void	Swap_Init (void);
        */
        #[no_mangle]
        pub fn va(format: *mut libc::c_char, ...) -> *mut libc::c_char;
        //=============================================
        //
        // key / value info strings
        //
        #[no_mangle]
        pub fn Info_ValueForKey(
            s: *const libc::c_char,
            key: *const libc::c_char,
        ) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Info_SetValueForKey(
            s: *mut libc::c_char,
            key: *const libc::c_char,
            value: *const libc::c_char,
        );
    }
    pub const EXEC_INSERT: unnamed = 1;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct usercmd_s {
        pub serverTime: libc::c_int,
        pub angles: [libc::c_int; 3],
        pub buttons: libc::c_int,
        pub weapon: byte,
        pub forwardmove: libc::c_schar,
        pub rightmove: libc::c_schar,
        pub upmove: libc::c_schar,
    }
    pub const FS_READ: fsMode_t = 0;
    pub type cplane_t = cplane_s;
    pub type playerState_t = playerState_s;
    pub type byte = libc::c_uchar;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct trajectory_t {
        pub trType: trType_t,
        pub trTime: libc::c_int,
        pub trDuration: libc::c_int,
        pub trBase: vec3_t,
        pub trDelta: vec3_t,
    }
    pub type qboolean = libc::c_uint;
    pub const TR_GRAVITY: trType_t = 5;
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct entityState_s {
        pub number: libc::c_int,
        pub eType: libc::c_int,
        pub eFlags: libc::c_int,
        pub pos: trajectory_t,
        pub apos: trajectory_t,
        pub time: libc::c_int,
        pub time2: libc::c_int,
        pub origin: vec3_t,
        pub origin2: vec3_t,
        pub angles: vec3_t,
        pub angles2: vec3_t,
        pub otherEntityNum: libc::c_int,
        pub otherEntityNum2: libc::c_int,
        pub groundEntityNum: libc::c_int,
        pub constantLight: libc::c_int,
        pub loopSound: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub clientNum: libc::c_int,
        pub frame: libc::c_int,
        pub solid: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub generic1: libc::c_int,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct playerState_s {
        pub commandTime: libc::c_int,
        pub pm_type: libc::c_int,
        pub bobCycle: libc::c_int,
        pub pm_flags: libc::c_int,
        pub pm_time: libc::c_int,
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub weaponTime: libc::c_int,
        pub gravity: libc::c_int,
        pub speed: libc::c_int,
        pub delta_angles: [libc::c_int; 3],
        pub groundEntityNum: libc::c_int,
        pub legsTimer: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoTimer: libc::c_int,
        pub torsoAnim: libc::c_int,
        pub movementDir: libc::c_int,
        pub grapplePoint: vec3_t,
        pub eFlags: libc::c_int,
        pub eventSequence: libc::c_int,
        pub events: [libc::c_int; 2],
        pub eventParms: [libc::c_int; 2],
        pub externalEvent: libc::c_int,
        pub externalEventParm: libc::c_int,
        pub externalEventTime: libc::c_int,
        pub clientNum: libc::c_int,
        pub weapon: libc::c_int,
        pub weaponstate: libc::c_int,
        pub viewangles: vec3_t,
        pub viewheight: libc::c_int,
        pub damageEvent: libc::c_int,
        pub damageYaw: libc::c_int,
        pub damagePitch: libc::c_int,
        pub damageCount: libc::c_int,
        pub stats: [libc::c_int; 16],
        pub persistant: [libc::c_int; 16],
        pub powerups: [libc::c_int; 16],
        pub ammo: [libc::c_int; 16],
        pub generic1: libc::c_int,
        pub loopSound: libc::c_int,
        pub jumppad_ent: libc::c_int,
        pub ping: libc::c_int,
        pub pmove_framecount: libc::c_int,
        pub jumppad_frame: libc::c_int,
        pub entityEventSequence: libc::c_int,
    }
    pub const TR_STATIONARY: trType_t = 0;
    pub type cvarHandle_t = libc::c_int;
    pub type usercmd_t = usercmd_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct trace_t {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub surfaceFlags: libc::c_int,
        pub contents: libc::c_int,
        pub entityNum: libc::c_int,
    }
    pub const TR_SINE: trType_t = 4;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct cplane_s {
        pub normal: vec3_t,
        pub dist: libc::c_float,
        pub type_0: byte,
        pub signbits: byte,
        pub pad: [byte; 2],
    }
    pub const qtrue: qboolean = 1;
    pub type fileHandle_t = libc::c_int;
    pub type trType_t = libc::c_uint;
    pub const TR_LINEAR_STOP: trType_t = 3;
    pub const qfalse: qboolean = 0;
    pub const TR_INTERPOLATE: trType_t = 1;
    pub type unnamed = libc::c_uint;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct vmCvar_t {
        pub handle: cvarHandle_t,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub string: [libc::c_char; 256],
    }
    pub const TR_LINEAR: trType_t = 2;
    use libc;
}
mod be_ai_weap_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    use q_shared_h::vec3_t;
    pub type projectileinfo_t = projectileinfo_s;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct weaponinfo_s {
        pub valid: libc::c_int,
        pub number: libc::c_int,
        pub name: [libc::c_char; 80],
        pub model: [libc::c_char; 80],
        pub level: libc::c_int,
        pub weaponindex: libc::c_int,
        pub flags: libc::c_int,
        pub projectile: [libc::c_char; 80],
        pub numprojectiles: libc::c_int,
        pub hspread: libc::c_float,
        pub vspread: libc::c_float,
        pub speed: libc::c_float,
        pub acceleration: libc::c_float,
        pub recoil: vec3_t,
        pub offset: vec3_t,
        pub angleoffset: vec3_t,
        pub extrazvelocity: libc::c_float,
        pub ammoamount: libc::c_int,
        pub ammoindex: libc::c_int,
        pub activate: libc::c_float,
        pub reload: libc::c_float,
        pub spinup: libc::c_float,
        pub spindown: libc::c_float,
        pub proj: projectileinfo_t,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct projectileinfo_s {
        pub name: [libc::c_char; 80],
        pub model: [libc::c_char; 80],
        pub flags: libc::c_int,
        pub gravity: libc::c_float,
        pub damage: libc::c_int,
        pub radius: libc::c_float,
        pub visdamage: libc::c_int,
        pub damagetype: libc::c_int,
        pub healthinc: libc::c_int,
        pub push: libc::c_float,
        pub detonation: libc::c_float,
        pub bounce: libc::c_float,
        pub bouncefric: libc::c_float,
        pub bouncestop: libc::c_float,
    }
    pub type weaponinfo_t = weaponinfo_s;
    use libc;
}
mod bg_local_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_public_h::pmove_t;
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    use q_shared_h::{qboolean, trace_t, vec3_t, vec_t};

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct pml_t {
        pub forward: vec3_t,
        pub right: vec3_t,
        pub up: vec3_t,
        pub frametime: libc::c_float,
        pub msec: libc::c_int,
        pub walking: qboolean,
        pub groundPlane: qboolean,
        pub groundTrace: trace_t,
        pub impactSpeed: libc::c_float,
        pub previous_origin: vec3_t,
        pub previous_velocity: vec3_t,
        pub previous_waterlevel: libc::c_int,
    }
    use libc;
}
mod be_ai_goal_h {
    use ai_main::{
        bot_developer, BotAILoadMap, BotAISetup, BotAISetupClient, BotAIShutdown,
        BotAIShutdownClient, BotAIStartFrame, BotInterbreedEndMatch, BotTestAAS,
    };
    use bg_misc::{
        bg_itemlist, bg_numItems, BG_AddPredictableEventToPlayerstate, BG_CanItemBeGrabbed,
        BG_EvaluateTrajectory, BG_EvaluateTrajectoryDelta, BG_FindItem, BG_FindItemForPowerup,
        BG_FindItemForWeapon, BG_PlayerStateToEntityState, BG_PlayerStateToEntityStateExtraPolate,
        BG_PlayerTouchesItem, BG_TouchJumpPad,
    };
    use bg_pmove::{c_pmove, pm, pml, PM_AddEvent, PM_AddTouchEnt, PM_ClipVelocity, Pmove};
    use bg_slidemove::{PM_SlideMove, PM_StepSlideMove};
    use g_active::{ClientEndFrame, ClientThink, G_RunClient};
    use g_arenas::{
        podium1, podium2, podium3, SpawnModelsOnVictoryPads, Svcmd_AbortPodium_f,
        UpdateTournamentInfo,
    };
    use g_bot::{
        G_BotConnect, G_CheckBotSpawn, G_InitBots, G_RemoveQueuedBotBegin, Svcmd_AddBot_f,
        Svcmd_BotList_f,
    };
    use g_client::{
        ClientBegin, ClientConnect, ClientDisconnect, ClientRespawn, ClientUserinfoChanged,
        CopyToBodyQue, InitBodyQue, PickTeam, SP_info_player_deathmatch,
        SP_info_player_intermission, SP_info_player_start, SelectSpawnPoint, SetClientViewAngle,
        SpotWouldTelefrag, TeamCount, TeamLeader,
    };
    use g_cmds::{
        BroadcastTeamChange, ClientCommand, Cmd_FollowCycle_f, Cmd_Score_f, ConcatArgs,
        DeathmatchScoreboardMessage, SetTeam, StopFollowing,
    };
    use g_combat::{body_die, player_die, AddScore, G_Damage, G_RadiusDamage, TossClientItems};
    use g_items::{
        ClearRegisteredItems, Drop_Item, FinishSpawningItem, G_CheckTeamItems, G_RunItem,
        G_SpawnItem, RegisterItem, RespawnItem, SaveRegisteredItems, Touch_Item,
    };
    use g_main::{
        g_allowVote, g_banIPs, g_blood, g_cheats, g_debugAlloc, g_debugDamage, g_debugMove,
        g_dedicated, g_dmflags, g_doWarmup, g_entities, g_filterBan, g_forcerespawn,
        g_friendlyFire, g_gametype, g_gravity, g_inactivity, g_knockback, g_localTeamPref,
        g_maxGameClients, g_maxclients, g_motd, g_password, g_quadfactor, g_restarted,
        g_smoothClients, g_speed, g_synchronousClients, g_teamAutoJoin, g_teamForceBalance,
        g_weaponRespawn, g_weaponTeamRespawn, level, pmove_fixed, pmove_msec, AddTournamentQueue,
        BeginIntermission, CalculateRanks, CheckTeamLeader, ExitLevel, FindIntermissionPoint,
        G_RunThink, MoveClientToIntermission, SetLeader,
    };
    use g_mem::{G_Alloc, G_InitMemory, Svcmd_GameMem_f};
    use g_misc::{
        SP_info_camp, SP_info_notnull, SP_info_null, SP_light, SP_misc_model,
        SP_misc_portal_camera, SP_misc_portal_surface, SP_misc_teleporter_dest, SP_shooter_grenade,
        SP_shooter_plasma, SP_shooter_rocket, TeleportPlayer,
    };
    use g_missile::{fire_bfg, fire_grapple, fire_grenade, fire_plasma, fire_rocket, G_RunMissile};
    use g_mover::{
        G_RunMover, SP_func_bobbing, SP_func_button, SP_func_door, SP_func_pendulum, SP_func_plat,
        SP_func_rotating, SP_func_static, SP_func_train, SP_path_corner, Touch_DoorTrigger,
    };
    use g_session::{G_InitSessionData, G_InitWorldSession, G_ReadSessionData, G_WriteSessionData};
    use g_spawn::{
        G_SpawnEntitiesFromString, G_SpawnFloat, G_SpawnInt, G_SpawnString, G_SpawnVector,
    };
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
    use q_math::{
        vec3_origin, vectoangles, AddPointToBounds, AngleMod, AngleNormalize180, AngleVectors,
        DirToByte, PerpendicularVector, Q_crandom, RadiusFromBounds, VectorNormalize,
        VectorNormalize2,
    };
    use q_shared_h::vec3_t;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct bot_goal_s {
        pub origin: vec3_t,
        pub areanum: libc::c_int,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub entitynum: libc::c_int,
        pub number: libc::c_int,
        pub flags: libc::c_int,
        pub iteminfo: libc::c_int,
    }
    pub type bot_goal_t = bot_goal_s;
    use libc;
}
