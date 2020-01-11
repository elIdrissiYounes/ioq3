// =============== BEGIN ai_main_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_waypoint_s {
    pub inuse: i32,
    pub name: [i8; 32],
    pub goal: crate::be_ai_goal_h::bot_goal_t,
    pub next: *mut crate::src::game::ai_main::bot_waypoint_s,
    pub prev: *mut crate::src::game::ai_main::bot_waypoint_s,
}
//
//check points

pub type bot_waypoint_t = crate::src::game::ai_main::bot_waypoint_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_activategoal_s {
    pub inuse: i32,
    pub goal: crate::be_ai_goal_h::bot_goal_t,
    pub time: f32,
    pub start_time: f32,
    pub justused_time: f32,
    pub shoot: i32,
    pub weapon: i32,
    pub target: crate::src::qcommon::q_shared::vec3_t,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub areas: [i32; 32],
    pub numareas: i32,
    pub areasdisabled: i32,
    pub next: *mut crate::src::game::ai_main::bot_activategoal_s,
}

pub type bot_activategoal_t = crate::src::game::ai_main::bot_activategoal_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_state_s {
    pub inuse: i32,
    pub botthink_residual: i32,
    pub client: i32,
    pub entitynum: i32,
    pub cur_ps: crate::src::qcommon::q_shared::playerState_t,
    pub last_eFlags: i32,
    pub lastucmd: crate::src::qcommon::q_shared::usercmd_t,
    pub entityeventTime: [i32; 1024],
    pub settings: crate::g_local_h::bot_settings_t,
    pub ainode: Option<unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_s) -> i32>,
    pub thinktime: f32,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub presencetype: i32,
    pub eye: crate::src::qcommon::q_shared::vec3_t,
    pub areanum: i32,
    pub inventory: [i32; 256],
    pub tfl: i32,
    pub flags: i32,
    pub respawn_wait: i32,
    pub lasthealth: i32,
    pub lastkilledplayer: i32,
    pub lastkilledby: i32,
    pub botdeathtype: i32,
    pub enemydeathtype: i32,
    pub botsuicide: i32,
    pub enemysuicide: i32,
    pub setupcount: i32,
    pub map_restart: i32,
    pub entergamechat: i32,
    pub num_deaths: i32,
    pub num_kills: i32,
    pub revenge_enemy: i32,
    pub revenge_kills: i32,
    pub lastframe_health: i32,
    pub lasthitcount: i32,
    pub chatto: i32,
    pub walker: f32,
    pub ltime: f32,
    pub entergame_time: f32,
    pub ltg_time: f32,
    pub nbg_time: f32,
    pub respawn_time: f32,
    pub respawnchat_time: f32,
    pub chase_time: f32,
    pub enemyvisible_time: f32,
    pub check_time: f32,
    pub stand_time: f32,
    pub lastchat_time: f32,
    pub kamikaze_time: f32,
    pub invulnerability_time: f32,
    pub standfindenemy_time: f32,
    pub attackstrafe_time: f32,
    pub attackcrouch_time: f32,
    pub attackchase_time: f32,
    pub attackjump_time: f32,
    pub enemysight_time: f32,
    pub enemydeath_time: f32,
    pub enemyposition_time: f32,
    pub defendaway_time: f32,
    pub defendaway_range: f32,
    pub rushbaseaway_time: f32,
    pub attackaway_time: f32,
    pub harvestaway_time: f32,
    pub ctfroam_time: f32,
    pub killedenemy_time: f32,
    pub arrive_time: f32,
    pub lastair_time: f32,
    pub teleport_time: f32,
    pub camp_time: f32,
    pub weaponchange_time: f32,
    pub firethrottlewait_time: f32,
    pub firethrottleshoot_time: f32,
    pub notblocked_time: f32,
    pub blockedbyavoidspot_time: f32,
    pub predictobstacles_time: f32,
    pub predictobstacles_goalareanum: i32,
    pub aimtarget: crate::src::qcommon::q_shared::vec3_t,
    pub enemyvelocity: crate::src::qcommon::q_shared::vec3_t,
    pub enemyorigin: crate::src::qcommon::q_shared::vec3_t,
    pub kamikazebody: i32,
    pub proxmines: [i32; 64],
    pub numproxmines: i32,
    pub character: i32,
    pub ms: i32,
    pub gs: i32,
    pub cs: i32,
    pub ws: i32,
    pub enemy: i32,
    pub lastenemyareanum: i32,
    pub lastenemyorigin: crate::src::qcommon::q_shared::vec3_t,
    pub weaponnum: i32,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub ideal_viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub viewanglespeed: crate::src::qcommon::q_shared::vec3_t,
    pub ltgtype: i32,
    pub teammate: i32,
    pub decisionmaker: i32,
    pub ordered: i32,
    pub order_time: f32,
    pub owndecision_time: i32,
    pub teamgoal: crate::be_ai_goal_h::bot_goal_t,
    pub altroutegoal: crate::be_ai_goal_h::bot_goal_t,
    pub reachedaltroutegoal_time: f32,
    pub teammessage_time: f32,
    pub teamgoal_time: f32,
    pub teammatevisible_time: f32,
    pub teamtaskpreference: i32,
    pub lastgoal_decisionmaker: i32,
    pub lastgoal_ltgtype: i32,
    pub lastgoal_teammate: i32,
    pub lastgoal_teamgoal: crate::be_ai_goal_h::bot_goal_t,
    pub lead_teammate: i32,
    pub lead_teamgoal: crate::be_ai_goal_h::bot_goal_t,
    pub lead_time: f32,
    pub leadvisible_time: f32,
    pub leadmessage_time: f32,
    pub leadbackup_time: f32,
    pub teamleader: [i8; 36],
    pub askteamleader_time: f32,
    pub becometeamleader_time: f32,
    pub teamgiveorders_time: f32,
    pub lastflagcapture_time: f32,
    pub numteammates: i32,
    pub redflagstatus: i32,
    pub blueflagstatus: i32,
    pub neutralflagstatus: i32,
    pub flagstatuschanged: i32,
    pub forceorders: i32,
    pub flagcarrier: i32,
    pub ctfstrategy: i32,
    pub subteam: [i8; 32],
    pub formation_dist: f32,
    pub activatestack: *mut crate::src::game::ai_main::bot_activategoal_t,
    pub activategoalheap: [crate::src::game::ai_main::bot_activategoal_t; 8],
    pub checkpoints: *mut crate::src::game::ai_main::bot_waypoint_t,
    pub patrolpoints: *mut crate::src::game::ai_main::bot_waypoint_t,
    pub curpatrolpoint: *mut crate::src::game::ai_main::bot_waypoint_t,
    pub patrolflags: i32,
}
//goal to activate (buttons etc.)
//time to activate something
//time starting to activate something
//time the goal was used
//true if bot has to shoot to activate
//weapon to be used for activation
//target to shoot at to activate something
//origin of the blocking entity to activate
//routing areas disabled by blocking entity
//number of disabled routing areas
//true if the areas are disabled for the routing
//next activate goal on stack
//bot state

pub type bot_state_t = crate::src::game::ai_main::bot_state_s;
use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;

pub use crate::bg_public_h::gitem_s;
pub use crate::bg_public_h::gitem_t;
pub use crate::bg_public_h::itemType_t;
pub use crate::bg_public_h::team_t;
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::bg_public_h::ET_BEAM;
pub use crate::bg_public_h::ET_EVENTS;
pub use crate::bg_public_h::ET_GENERAL;
pub use crate::bg_public_h::ET_GRAPPLE;
pub use crate::bg_public_h::ET_INVISIBLE;
pub use crate::bg_public_h::ET_ITEM;
pub use crate::bg_public_h::ET_MISSILE;
pub use crate::bg_public_h::ET_MOVER;
pub use crate::bg_public_h::ET_PLAYER;
pub use crate::bg_public_h::ET_PORTAL;
pub use crate::bg_public_h::ET_PUSH_TRIGGER;
pub use crate::bg_public_h::ET_SPEAKER;
pub use crate::bg_public_h::ET_TEAM;
pub use crate::bg_public_h::ET_TELEPORT_TRIGGER;
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
pub use crate::bg_public_h::WP_BFG;
pub use crate::bg_public_h::WP_GAUNTLET;
pub use crate::bg_public_h::WP_GRAPPLING_HOOK;
pub use crate::bg_public_h::WP_GRENADE_LAUNCHER;
pub use crate::bg_public_h::WP_LIGHTNING;
pub use crate::bg_public_h::WP_MACHINEGUN;
pub use crate::bg_public_h::WP_NONE;
pub use crate::bg_public_h::WP_NUM_WEAPONS;
pub use crate::bg_public_h::WP_PLASMAGUN;
pub use crate::bg_public_h::WP_RAILGUN;
pub use crate::bg_public_h::WP_ROCKET_LAUNCHER;
pub use crate::bg_public_h::WP_SHOTGUN;
pub use crate::g_public_h::entityShared_t;
pub use crate::src::qcommon::q_math::AngleMod;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
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
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_IsColorString;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::EXEC_APPEND;
pub use crate::src::qcommon::q_shared::EXEC_INSERT;
pub use crate::src::qcommon::q_shared::EXEC_NOW;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::be_aas_h::aas_areainfo_s;
pub use crate::be_aas_h::aas_areainfo_t;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_aas_h::SOLID_BBOX;
pub use crate::be_aas_h::SOLID_BSP;
pub use crate::be_aas_h::SOLID_NOT;
pub use crate::be_aas_h::SOLID_TRIGGER;
pub use crate::be_ai_goal_h::bot_goal_s;
pub use crate::be_ai_goal_h::bot_goal_t;
pub use crate::botlib_h::bot_entitystate_s;
pub use crate::botlib_h::bot_entitystate_t;
pub use crate::botlib_h::bot_input_s;
pub use crate::botlib_h::bot_input_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::g_local_h::bot_settings_s;
pub use crate::g_local_h::bot_settings_t;
pub use crate::g_local_h::clientConnected_t;
pub use crate::g_local_h::clientPersistant_t;
pub use crate::g_local_h::clientSession_t;
pub use crate::g_local_h::gclient_s;
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
pub use crate::src::game::g_bot::G_CheckBotSpawn;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_main::G_Error;
pub use crate::src::game::g_main::G_Printf;
pub use crate::src::game::g_mem::G_Alloc;
pub use crate::src::game::g_syscalls::trap_AAS_AreaInfo;
pub use crate::src::game::g_syscalls::trap_AAS_EntityInfo;
pub use crate::src::game::g_syscalls::trap_AAS_Initialized;
pub use crate::src::game::g_syscalls::trap_AAS_Time;
pub use crate::src::game::g_syscalls::trap_BotAllocChatState;
pub use crate::src::game::g_syscalls::trap_BotAllocGoalState;
pub use crate::src::game::g_syscalls::trap_BotAllocMoveState;
pub use crate::src::game::g_syscalls::trap_BotAllocWeaponState;
pub use crate::src::game::g_syscalls::trap_BotEnterChat;
pub use crate::src::game::g_syscalls::trap_BotFreeCharacter;
pub use crate::src::game::g_syscalls::trap_BotFreeChatState;
pub use crate::src::game::g_syscalls::trap_BotFreeGoalState;
pub use crate::src::game::g_syscalls::trap_BotFreeMoveState;
pub use crate::src::game::g_syscalls::trap_BotFreeWeaponState;
pub use crate::src::game::g_syscalls::trap_BotGetServerCommand;
pub use crate::src::game::g_syscalls::trap_BotGetSnapshotEntity;
pub use crate::src::game::g_syscalls::trap_BotGetTopGoal;
pub use crate::src::game::g_syscalls::trap_BotGoalName;
pub use crate::src::game::g_syscalls::trap_BotInitialChat;
pub use crate::src::game::g_syscalls::trap_BotInterbreedGoalFuzzyLogic;
pub use crate::src::game::g_syscalls::trap_BotLibLoadMap;
pub use crate::src::game::g_syscalls::trap_BotLibSetup;
pub use crate::src::game::g_syscalls::trap_BotLibShutdown;
pub use crate::src::game::g_syscalls::trap_BotLibStartFrame;
pub use crate::src::game::g_syscalls::trap_BotLibUpdateEntity;
pub use crate::src::game::g_syscalls::trap_BotLibVarSet;
pub use crate::src::game::g_syscalls::trap_BotLoadCharacter;
pub use crate::src::game::g_syscalls::trap_BotLoadChatFile;
pub use crate::src::game::g_syscalls::trap_BotLoadItemWeights;
pub use crate::src::game::g_syscalls::trap_BotLoadWeaponWeights;
pub use crate::src::game::g_syscalls::trap_BotMutateGoalFuzzyLogic;
pub use crate::src::game::g_syscalls::trap_BotQueueConsoleMessage;
pub use crate::src::game::g_syscalls::trap_BotResetAvoidGoals;
pub use crate::src::game::g_syscalls::trap_BotResetAvoidReach;
pub use crate::src::game::g_syscalls::trap_BotResetGoalState;
pub use crate::src::game::g_syscalls::trap_BotResetMoveState;
pub use crate::src::game::g_syscalls::trap_BotResetWeaponState;
pub use crate::src::game::g_syscalls::trap_BotSaveGoalFuzzyLogic;
pub use crate::src::game::g_syscalls::trap_BotSetChatGender;
pub use crate::src::game::g_syscalls::trap_BotUpdateEntityItems;
pub use crate::src::game::g_syscalls::trap_BotUserCommand;
pub use crate::src::game::g_syscalls::trap_Characteristic_BFloat;
pub use crate::src::game::g_syscalls::trap_Characteristic_String;
pub use crate::src::game::g_syscalls::trap_Cvar_Register;
pub use crate::src::game::g_syscalls::trap_Cvar_Set;
pub use crate::src::game::g_syscalls::trap_Cvar_Update;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::game::g_syscalls::trap_EA_GetInput;
pub use crate::src::game::g_syscalls::trap_EA_ResetInput;
pub use crate::src::game::g_syscalls::trap_EA_SelectWeapon;
pub use crate::src::game::g_syscalls::trap_EA_View;
pub use crate::src::game::g_syscalls::trap_GeneticParentsAndChildSelection;
pub use crate::src::game::g_syscalls::trap_GetConfigstring;
pub use crate::src::game::g_syscalls::trap_SendConsoleCommand;
pub use crate::src::game::g_syscalls::trap_SetConfigstring;
pub use crate::src::game::g_syscalls::trap_Trace;
use crate::stdlib::fabs;

use crate::src::game::ai_chat::BotChatTest;
use crate::src::game::ai_chat::BotChat_ExitGame;
use crate::src::game::ai_dmq3::bot_challenge;
use crate::src::game::ai_dmq3::bot_fastchat;
use crate::src::game::ai_dmq3::bot_grapple;
use crate::src::game::ai_dmq3::bot_nochat;
use crate::src::game::ai_dmq3::bot_rocketjump;
use crate::src::game::ai_dmq3::bot_testrchat;
use crate::src::game::ai_dmq3::gametype;
use crate::src::game::ai_dmq3::BotCTFCarryingFlag;
use crate::src::game::ai_dmq3::BotClearActivateGoalStack;
use crate::src::game::ai_dmq3::BotDeathmatchAI;
use crate::src::game::ai_dmq3::BotFreeWaypoints;
use crate::src::game::ai_dmq3::BotPointAreaNum;
use crate::src::game::ai_dmq3::BotSetupDeathmatchAI;
use crate::src::game::ai_dmq3::BotSynonymContext;
use crate::src::game::ai_dmq3::BotTeam;
use crate::src::game::ai_dmq3::ClientFromName;
use crate::src::game::ai_dmq3::ClientName;
use crate::src::game::ai_dmq3::EasyClientName;
pub use crate::src::game::ai_main::stdlib_h::atoi;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;
use crate::stdlib::memset;
use crate::stdlib::sscanf;
use crate::stdlib::strchr;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
pub use crate::stdlib::strtol;
use crate::stdlib::vsnprintf;
extern "C" {
    #[no_mangle]
    pub fn ExitLevel();
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
/* ****************************************************************************
 * name:		ai_main.c
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /MissionPack/code/game/ai_main.c $
 *
 *****************************************************************************/
//
//
//bot states
#[no_mangle]

pub static mut botstates: [*mut crate::src::game::ai_main::bot_state_t; 64] =
    [0 as *mut crate::src::game::ai_main::bot_state_t; 64];
//number of bots
#[no_mangle]

pub static mut numbots: i32 = 0;
//floating point time
#[no_mangle]

pub static mut floattime: f32 = 0.;
//time to do a regular update
#[no_mangle]

pub static mut regularupdate_time: f32 = 0.;
//
#[no_mangle]

pub static mut bot_interbreed: i32 = 0;
#[no_mangle]

pub static mut bot_interbreedmatchcount: i32 = 0;
//
#[no_mangle]

pub static mut bot_thinktime: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_memorydump: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_saveroutingcache: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_pause: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_report: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_testsolid: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_testclusters: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_developer: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_interbreedchar: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_interbreedbots: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_interbreedcycle: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_interbreedwrite: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
// from the game source
/*
==================
BotAI_Print
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_Print(mut type_0: i32, mut fmt: *mut i8, mut args: ...) {
    let mut str: [i8; 2048] = [0; 2048];
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    crate::stdlib::vsnprintf(
        str.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 2048]>(),
        fmt,
        ap.as_va_list(),
    );
    match type_0 {
        1 => {
            crate::src::game::g_main::G_Printf(
                b"%s\x00" as *const u8 as *const i8,
                str.as_mut_ptr(),
            );
        }
        2 => {
            crate::src::game::g_main::G_Printf(
                b"^3Warning: %s\x00" as *const u8 as *const i8,
                str.as_mut_ptr(),
            );
        }
        3 => {
            crate::src::game::g_main::G_Printf(
                b"^1Error: %s\x00" as *const u8 as *const i8,
                str.as_mut_ptr(),
            );
        }
        4 => {
            crate::src::game::g_main::G_Printf(
                b"^1Fatal: %s\x00" as *const u8 as *const i8,
                str.as_mut_ptr(),
            );
        }
        5 => {
            crate::src::game::g_main::G_Error(
                b"^1Exit: %s\x00" as *const u8 as *const i8,
                str.as_mut_ptr(),
            );
        }
        _ => {
            crate::src::game::g_main::G_Printf(
                b"unknown print type\n\x00" as *const u8 as *const i8,
            );
        }
    };
}
/*
==================
BotAI_Trace
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_Trace(
    mut bsptrace: *mut crate::botlib_h::bsp_trace_t,
    mut start: *mut crate::src::qcommon::q_shared::vec_t,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
    mut end: *mut crate::src::qcommon::q_shared::vec_t,
    mut passent: i32,
    mut contentmask: i32,
) {
    let mut trace: crate::src::qcommon::q_shared::trace_t =
        crate::src::qcommon::q_shared::trace_t {
            allsolid: crate::src::qcommon::q_shared::qfalse,
            startsolid: crate::src::qcommon::q_shared::qfalse,
            fraction: 0.,
            endpos: [0.; 3],
            plane: crate::src::qcommon::q_shared::cplane_t {
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
    crate::src::game::g_syscalls::trap_Trace(
        &mut trace,
        start as *const crate::src::qcommon::q_shared::vec_t,
        mins as *const crate::src::qcommon::q_shared::vec_t,
        maxs as *const crate::src::qcommon::q_shared::vec_t,
        end as *const crate::src::qcommon::q_shared::vec_t,
        passent,
        contentmask,
    );
    //copy the trace information
    (*bsptrace).allsolid = trace.allsolid;
    (*bsptrace).startsolid = trace.startsolid;
    (*bsptrace).fraction = trace.fraction;
    (*bsptrace).endpos[0] = trace.endpos[0];
    (*bsptrace).endpos[1] = trace.endpos[1];
    (*bsptrace).endpos[2] = trace.endpos[2];
    (*bsptrace).plane.dist = trace.plane.dist;
    (*bsptrace).plane.normal[0] = trace.plane.normal[0];
    (*bsptrace).plane.normal[1] = trace.plane.normal[1];
    (*bsptrace).plane.normal[2] = trace.plane.normal[2];
    (*bsptrace).plane.signbits = trace.plane.signbits;
    (*bsptrace).plane.type_0 = trace.plane.type_0;
    (*bsptrace).surface.value = 0;
    (*bsptrace).surface.flags = trace.surfaceFlags;
    (*bsptrace).ent = trace.entityNum;
    (*bsptrace).exp_dist = 0f32;
    (*bsptrace).sidenum = 0;
    (*bsptrace).contents = 0;
}
/*
==================
BotAI_GetClientState
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_GetClientState(
    mut clientNum: i32,
    mut state: *mut crate::src::qcommon::q_shared::playerState_t,
) -> i32 {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    ent = &mut *crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset(clientNum as isize) as *mut crate::g_local_h::gentity_t;
    if (*ent).inuse as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if (*ent).client.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    crate::stdlib::memcpy(
        state as *mut libc::c_void,
        &mut (*(*ent).client).ps as *mut crate::src::qcommon::q_shared::playerState_t
            as *const libc::c_void,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::playerState_t>(),
    );
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotAI_GetEntityState
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_GetEntityState(
    mut entityNum: i32,
    mut state: *mut crate::src::qcommon::q_shared::entityState_t,
) -> i32 {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    ent = &mut *crate::src::game::g_main::g_entities
        .as_mut_ptr()
        .offset(entityNum as isize) as *mut crate::g_local_h::gentity_t;
    crate::stdlib::memset(
        state as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>(),
    );
    if (*ent).inuse as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if (*ent).r.linked as u64 == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if (*ent).r.svFlags & 0x1 != 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    crate::stdlib::memcpy(
        state as *mut libc::c_void,
        &mut (*ent).s as *mut crate::src::qcommon::q_shared::entityState_t as *const libc::c_void,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>(),
    );
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotAI_GetSnapshotEntity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_GetSnapshotEntity(
    mut clientNum: i32,
    mut sequence: i32,
    mut state: *mut crate::src::qcommon::q_shared::entityState_t,
) -> i32 {
    let mut entNum: i32 = 0;
    entNum = crate::src::game::g_syscalls::trap_BotGetSnapshotEntity(clientNum, sequence);
    if entNum == -(1) {
        crate::stdlib::memset(
            state as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>(),
        );
        return -(1i32);
    }
    BotAI_GetEntityState(entNum, state);
    return sequence + 1;
}
/*
==================
BotAI_BotInitialChat
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI_BotInitialChat(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut type_0: *mut i8,
    mut args: ...
) {
    let mut i: i32 = 0;
    let mut mcontext: i32 = 0;
    let mut ap: ::std::ffi::VaListImpl;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut vars: [*mut i8; 8] = [0 as *mut i8; 8];
    crate::stdlib::memset(
        vars.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[*mut i8; 8]>(),
    );
    ap = args.clone();
    p = ap.as_va_list().arg::<*mut i8>();

    for i in 0..8 {
        if p.is_null() {
            break;
        }

        vars[i as usize] = p;

        p = ap.as_va_list().arg::<*mut i8>();
    }
    mcontext = crate::src::game::ai_dmq3::BotSynonymContext(bs);
    crate::src::game::g_syscalls::trap_BotInitialChat(
        (*bs).cs,
        type_0,
        mcontext,
        vars[0],
        vars[1],
        vars[2],
        vars[3],
        vars[4],
        vars[5],
        vars[6],
        vars[7],
    );
}
/*
==================
BotTestAAS
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTestAAS(mut origin: *mut crate::src::qcommon::q_shared::vec_t) {
    let mut areanum: i32 = 0;
    let mut info: crate::be_aas_h::aas_areainfo_t = crate::be_aas_h::aas_areainfo_t {
        contents: 0,
        flags: 0,
        presencetype: 0,
        cluster: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        center: [0.; 3],
    };
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_testsolid);
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_testclusters);
    if bot_testsolid.integer != 0 {
        if crate::src::game::g_syscalls::trap_AAS_Initialized() == 0 {
            return;
        }
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(origin);
        if areanum != 0 {
            BotAI_Print(1i32, b"\rempty area\x00" as *const u8 as *mut i8);
        } else {
            BotAI_Print(1i32, b"\r^1SOLID area\x00" as *const u8 as *mut i8);
        }
    } else if bot_testclusters.integer != 0 {
        if crate::src::game::g_syscalls::trap_AAS_Initialized() == 0 {
            return;
        }
        areanum = crate::src::game::ai_dmq3::BotPointAreaNum(origin);
        if areanum == 0 {
            BotAI_Print(
                1i32,
                b"\r^1Solid!                              \x00" as *const u8 as *mut i8,
            );
        } else {
            crate::src::game::g_syscalls::trap_AAS_AreaInfo(
                areanum,
                &mut info as *mut crate::be_aas_h::aas_areainfo_t as *mut libc::c_void,
            );
            BotAI_Print(
                1i32,
                b"\rarea %d, cluster %d       \x00" as *const u8 as *mut i8,
                areanum,
                info.cluster,
            );
        }
    };
}
/*
==================
BotReportStatus
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotReportStatus(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut goalname: [i8; 256] = [0; 256];
    let mut netname: [i8; 256] = [0; 256];
    let mut leader: *mut i8 = 0 as *mut i8;
    let mut flagstatus: [i8; 32] = [0; 32];
    //
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr())
        == 0
    {
        leader = b"L\x00" as *const u8 as *mut i8
    } else {
        leader = b" \x00" as *const u8 as *mut i8
    }
    crate::stdlib::strcpy(flagstatus.as_mut_ptr(), b"  \x00" as *const u8 as *const i8);
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        if crate::src::game::ai_dmq3::BotCTFCarryingFlag(bs) != 0 {
            if crate::src::game::ai_dmq3::BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
                crate::stdlib::strcpy(
                    flagstatus.as_mut_ptr(),
                    b"^1F \x00" as *const u8 as *const i8,
                );
            } else {
                crate::stdlib::strcpy(
                    flagstatus.as_mut_ptr(),
                    b"^4F \x00" as *const u8 as *const i8,
                );
            }
        }
    }
    match (*bs).ltgtype {
        1 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            BotAI_Print(
                1i32,
                b"%-20s%s%s: helping %s\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        2 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            BotAI_Print(
                1i32,
                b"%-20s%s%s: accompanying %s\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        3 => {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            BotAI_Print(
                1i32,
                b"%-20s%s%s: defending %s\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        10 => {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            BotAI_Print(
                1i32,
                b"%-20s%s%s: getting item %s\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        11 => {
            crate::src::game::ai_dmq3::ClientName(
                (*bs).teamgoal.entitynum,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            BotAI_Print(
                1i32,
                b"%-20s%s%s: killing %s\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
                goalname.as_mut_ptr(),
            );
        }
        7 | 8 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: camping\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        9 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: patrolling\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        4 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: capturing flag\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        5 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: rushing base\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        6 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: returning flag\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        13 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: attacking the enemy base\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        12 => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: harvesting\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
        _ => {
            BotAI_Print(
                1i32,
                b"%-20s%s%s: roaming\n\x00" as *const u8 as *mut i8,
                netname.as_mut_ptr(),
                leader,
                flagstatus.as_mut_ptr(),
            );
        }
    };
}
/*
==================
BotTeamplayReport
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamplayReport() {
    let mut i: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    BotAI_Print(1, b"^1RED\n\x00" as *const u8 as *mut i8);
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        //
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            //
            crate::src::game::g_syscalls::trap_GetConfigstring(
                32 + 256 + 256 + i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 1024]>() as i32,
            );
            //if no config string or no name
            if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
                || crate::stdlib::strlen(crate::src::qcommon::q_shared::Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"n\x00" as *const u8 as *const i8,
                )) == 0)
            {
                //skip spectators
                if atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"t\x00" as *const u8 as *const i8,
                )) == crate::bg_public_h::TEAM_RED as i32
                {
                    BotReportStatus(botstates[i as usize]);
                }
            }
        }
        i += 1
    }
    BotAI_Print(1, b"^4BLUE\n\x00" as *const u8 as *mut i8);
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        //
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            //
            crate::src::game::g_syscalls::trap_GetConfigstring(
                32 + 256 + 256 + i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 1024]>() as i32,
            );
            //if no config string or no name
            if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
                || crate::stdlib::strlen(crate::src::qcommon::q_shared::Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"n\x00" as *const u8 as *const i8,
                )) == 0)
            {
                //skip spectators
                if atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"t\x00" as *const u8 as *const i8,
                )) == crate::bg_public_h::TEAM_BLUE as i32
                {
                    BotReportStatus(botstates[i as usize]);
                }
            }
        }
        i += 1
    }
}
/*
==================
BotSetInfoConfigString
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetInfoConfigString(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut goalname: [i8; 256] = [0; 256];
    let mut netname: [i8; 256] = [0; 256];
    let mut action: [i8; 256] = [0; 256];
    let mut leader: *mut i8 = 0 as *mut i8;
    let mut carrying: [i8; 32] = [0; 32];
    let mut cs: *mut i8 = 0 as *mut i8;
    let mut goal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    //
    crate::src::game::ai_dmq3::ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 256]>() as i32,
    );
    if crate::src::qcommon::q_shared::Q_stricmp(netname.as_mut_ptr(), (*bs).teamleader.as_mut_ptr())
        == 0
    {
        leader = b"L\x00" as *const u8 as *mut i8
    } else {
        leader = b" \x00" as *const u8 as *mut i8
    }
    crate::stdlib::strcpy(carrying.as_mut_ptr(), b"  \x00" as *const u8 as *const i8);
    if crate::src::game::ai_dmq3::gametype == crate::bg_public_h::GT_CTF as i32 {
        if crate::src::game::ai_dmq3::BotCTFCarryingFlag(bs) != 0 {
            crate::stdlib::strcpy(carrying.as_mut_ptr(), b"F \x00" as *const u8 as *const i8);
        }
    }
    match (*bs).ltgtype {
        1 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"helping %s\x00" as *const u8 as *const i8,
                goalname.as_mut_ptr(),
            );
        }
        2 => {
            crate::src::game::ai_dmq3::EasyClientName(
                (*bs).teammate,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"accompanying %s\x00" as *const u8 as *const i8,
                goalname.as_mut_ptr(),
            );
        }
        3 => {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"defending %s\x00" as *const u8 as *const i8,
                goalname.as_mut_ptr(),
            );
        }
        10 => {
            crate::src::game::g_syscalls::trap_BotGoalName(
                (*bs).teamgoal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"getting item %s\x00" as *const u8 as *const i8,
                goalname.as_mut_ptr(),
            );
        }
        11 => {
            crate::src::game::ai_dmq3::ClientName(
                (*bs).teamgoal.entitynum,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"killing %s\x00" as *const u8 as *const i8,
                goalname.as_mut_ptr(),
            );
        }
        7 | 8 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"camping\x00" as *const u8 as *const i8,
            );
        }
        9 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"patrolling\x00" as *const u8 as *const i8,
            );
        }
        4 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"capturing flag\x00" as *const u8 as *const i8,
            );
        }
        5 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"rushing base\x00" as *const u8 as *const i8,
            );
        }
        6 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"returning flag\x00" as *const u8 as *const i8,
            );
        }
        13 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"attacking the enemy base\x00" as *const u8 as *const i8,
            );
        }
        12 => {
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"harvesting\x00" as *const u8 as *const i8,
            );
        }
        _ => {
            crate::src::game::g_syscalls::trap_BotGetTopGoal(
                (*bs).gs,
                &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
            );
            crate::src::game::g_syscalls::trap_BotGoalName(
                goal.number,
                goalname.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
            );
            crate::src::qcommon::q_shared::Com_sprintf(
                action.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 256]>() as i32,
                b"roaming %s\x00" as *const u8 as *const i8,
                goalname.as_mut_ptr(),
            );
        }
    }
    cs = crate::src::qcommon::q_shared::va(
        b"l\\%s\\c\\%s\\a\\%s\x00" as *const u8 as *mut i8,
        leader,
        carrying.as_mut_ptr(),
        action.as_mut_ptr(),
    );
    crate::src::game::g_syscalls::trap_SetConfigstring(25 + (*bs).client, cs);
}
/*
==============
BotUpdateInfoConfigStrings
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotUpdateInfoConfigStrings() {
    let mut i: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        //
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            //
            crate::src::game::g_syscalls::trap_GetConfigstring(
                32 + 256 + 256 + i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 1024]>() as i32,
            );
            //if no config string or no name
            if !(crate::stdlib::strlen(buf.as_mut_ptr()) == 0
                || crate::stdlib::strlen(crate::src::qcommon::q_shared::Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"n\x00" as *const u8 as *const i8,
                )) == 0)
            {
                BotSetInfoConfigString(botstates[i as usize]);
            }
        }
        i += 1
    }
}
/*
==============
BotInterbreedBots
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotInterbreedBots() {
    let mut ranks: [f32; 64] = [0.; 64];
    let mut parent1: i32 = 0;
    let mut parent2: i32 = 0;
    let mut child: i32 = 0;
    let mut i: i32 = 0;
    // get rankings for all the bots
    i = 0;
    while i < 64 {
        if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
            ranks[i as usize] = ((*botstates[i as usize]).num_kills * 2
                - (*botstates[i as usize]).num_deaths) as f32
        } else {
            ranks[i as usize] = -1f32
        }
        i += 1
    }
    if crate::src::game::g_syscalls::trap_GeneticParentsAndChildSelection(
        64,
        ranks.as_mut_ptr(),
        &mut parent1,
        &mut parent2,
        &mut child,
    ) != 0
    {
        crate::src::game::g_syscalls::trap_BotInterbreedGoalFuzzyLogic(
            (*botstates[parent1 as usize]).gs,
            (*botstates[parent2 as usize]).gs,
            (*botstates[child as usize]).gs,
        );
        crate::src::game::g_syscalls::trap_BotMutateGoalFuzzyLogic(
            (*botstates[child as usize]).gs,
            1f32,
        );
    }
    // reset the kills and deaths
    i = 0;
    while i < 64 {
        if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
            (*botstates[i as usize]).num_kills = 0;
            (*botstates[i as usize]).num_deaths = 0
        }
        i += 1
    }
}
/*
==============
BotWriteInterbreeded
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotWriteInterbreeded(mut filename: *mut i8) {
    let mut rank: f32 = 0.;
    let mut bestrank: f32 = 0.;
    let mut i: i32 = 0;
    let mut bestbot: i32 = 0;
    bestrank = 0f32;
    bestbot = -(1);

    for i in 0..64 {
        if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
            rank = ((*botstates[i as usize]).num_kills * 2 - (*botstates[i as usize]).num_deaths)
                as f32
        } else {
            rank = -1f32
        }

        if rank > bestrank {
            bestrank = rank;
            bestbot = i
        }
    }
    if bestbot >= 0 {
        //write out the new goal fuzzy logic
        crate::src::game::g_syscalls::trap_BotSaveGoalFuzzyLogic(
            (*botstates[bestbot as usize]).gs,
            filename,
        );
    };
}
/*
==============
BotInterbreedEndMatch

add link back into ExitLevel?
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotInterbreedEndMatch() {
    if bot_interbreed == 0 {
        return;
    }
    bot_interbreedmatchcount += 1;
    if bot_interbreedmatchcount >= bot_interbreedcycle.integer {
        bot_interbreedmatchcount = 0;
        //
        crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_interbreedwrite);
        if crate::stdlib::strlen(bot_interbreedwrite.string.as_mut_ptr()) != 0 {
            BotWriteInterbreeded(bot_interbreedwrite.string.as_mut_ptr());
            crate::src::game::g_syscalls::trap_Cvar_Set(
                b"bot_interbreedwrite\x00" as *const u8 as *const i8,
                b"\x00" as *const u8 as *const i8,
            );
        }
        BotInterbreedBots();
    };
}
/*
==============
BotInterbreeding
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotInterbreeding() {
    let mut i: i32 = 0;
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_interbreedchar);
    if crate::stdlib::strlen(bot_interbreedchar.string.as_mut_ptr()) == 0 {
        return;
    }
    //make sure we are in tournament mode
    if crate::src::game::ai_dmq3::gametype != crate::bg_public_h::GT_TOURNAMENT as i32 {
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"g_gametype\x00" as *const u8 as *const i8,
            crate::src::qcommon::q_shared::va(
                b"%d\x00" as *const u8 as *mut i8,
                crate::bg_public_h::GT_TOURNAMENT as i32,
            ),
        );
        ExitLevel();
        return;
    }
    //shutdown all the bots
    i = 0;
    while i < 64 {
        if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
            BotAIShutdownClient(
                (*botstates[i as usize]).client,
                crate::src::qcommon::q_shared::qfalse,
            );
        }
        i += 1
    }
    //make sure all item weight configs are reloaded and Not shared
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"bot_reloadcharacters\x00" as *const u8 as *mut i8,
        b"1\x00" as *const u8 as *mut i8,
    );
    //add a number of bots using the desired bot character
    i = 0;
    while i < bot_interbreedbots.integer {
        crate::src::game::g_syscalls::trap_SendConsoleCommand(
            crate::src::qcommon::q_shared::EXEC_INSERT as i32,
            crate::src::qcommon::q_shared::va(
                b"addbot %s 4 free %i %s%d\n\x00" as *const u8 as *mut i8,
                bot_interbreedchar.string.as_mut_ptr(),
                i * 50i32,
                bot_interbreedchar.string.as_mut_ptr(),
                i,
            ),
        );
        i += 1
    }
    //
    crate::src::game::g_syscalls::trap_Cvar_Set(
        b"bot_interbreedchar\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
    );
    bot_interbreed = crate::src::qcommon::q_shared::qtrue as i32;
}
//returns info about the entity
/*
==============
BotEntityInfo
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotEntityInfo(
    mut entnum: i32,
    mut info: *mut crate::be_aas_h::aas_entityinfo_t,
) {
    crate::src::game::g_syscalls::trap_AAS_EntityInfo(entnum, info as *mut libc::c_void);
}
//returns the number of bots in the game
/*
==============
NumBots
==============
*/
#[no_mangle]

pub unsafe extern "C" fn NumBots() -> i32 {
    return numbots;
}
/*
==============
BotTeamLeader
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamLeader(mut bs: *mut crate::src::game::ai_main::bot_state_t) -> i32 {
    let mut leader: i32 = 0;
    leader = crate::src::game::ai_dmq3::ClientFromName((*bs).teamleader.as_mut_ptr());
    if leader < 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if botstates[leader as usize].is_null() || (*botstates[leader as usize]).inuse == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==============
AngleDifference
==============
*/
#[no_mangle]

pub unsafe extern "C" fn AngleDifference(mut ang1: f32, mut ang2: f32) -> f32 {
    let mut diff: f32 = 0.;
    diff = ang1 - ang2;
    if ang1 > ang2 {
        if diff as f64 > 180.0 {
            diff = (diff as f64 - 360.0) as f32
        }
    } else if (diff as f64) < -180.0 {
        diff = (diff as f64 + 360.0) as f32
    }
    return diff;
}
/*
==============
BotChangeViewAngle
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotChangeViewAngle(
    mut angle: f32,
    mut ideal_angle: f32,
    mut speed: f32,
) -> f32 {
    let mut move_0: f32 = 0.;
    angle = crate::src::qcommon::q_math::AngleMod(angle);
    ideal_angle = crate::src::qcommon::q_math::AngleMod(ideal_angle);
    if angle == ideal_angle {
        return angle;
    }
    move_0 = ideal_angle - angle;
    if ideal_angle > angle {
        if move_0 as f64 > 180.0 {
            move_0 = (move_0 as f64 - 360.0) as f32
        }
    } else if (move_0 as f64) < -180.0 {
        move_0 = (move_0 as f64 + 360.0) as f32
    }
    if move_0 > 0f32 {
        if move_0 > speed {
            move_0 = speed
        }
    } else if move_0 < -speed {
        move_0 = -speed
    }
    return crate::src::qcommon::q_math::AngleMod(angle + move_0);
}
/*
==============
BotChangeViewAngles
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotChangeViewAngles(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut thinktime: f32,
) {
    let mut diff: f32 = 0.;
    let mut factor: f32 = 0.;
    let mut maxchange: f32 = 0.;
    let mut anglespeed: f32 = 0.;
    let mut disired_speed: f32 = 0.;
    let mut i: i32 = 0;
    if (*bs).ideal_viewangles[0] > 180f32 {
        (*bs).ideal_viewangles[0] -= 360f32
    }
    //
    if (*bs).enemy >= 0 {
        factor = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            4,
            0.01,
            1f32,
        );
        maxchange = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            5,
            1f32,
            1800f32,
        )
    } else {
        factor = 0.05;
        maxchange = 360f32
    }
    if maxchange < 240f32 {
        maxchange = 240f32
    }
    maxchange *= thinktime;

    for i in 0..2 {
        //
        if crate::src::game::ai_dmq3::bot_challenge.integer != 0 {
            //smooth slowdown view model
            diff = crate::stdlib::fabs(AngleDifference(
                (*bs).viewangles[i as usize],
                (*bs).ideal_viewangles[i as usize],
            ) as f64) as f32;
            anglespeed = diff * factor;
            if anglespeed > maxchange {
                anglespeed = maxchange
            }
            (*bs).viewangles[i as usize] = BotChangeViewAngle(
                (*bs).viewangles[i as usize],
                (*bs).ideal_viewangles[i as usize],
                anglespeed,
            )
        } else {
            //over reaction view model
            (*bs).viewangles[i as usize] =
                crate::src::qcommon::q_math::AngleMod((*bs).viewangles[i as usize]);
            (*bs).ideal_viewangles[i as usize] =
                crate::src::qcommon::q_math::AngleMod((*bs).ideal_viewangles[i as usize]);
            diff = AngleDifference(
                (*bs).viewangles[i as usize],
                (*bs).ideal_viewangles[i as usize],
            );
            disired_speed = diff * factor;
            (*bs).viewanglespeed[i as usize] += (*bs).viewanglespeed[i as usize] - disired_speed;
            if (*bs).viewanglespeed[i as usize] > 180f32 {
                (*bs).viewanglespeed[i as usize] = maxchange
            }
            if (*bs).viewanglespeed[i as usize] < -180f32 {
                (*bs).viewanglespeed[i as usize] = -maxchange
            }
            anglespeed = (*bs).viewanglespeed[i as usize];
            if anglespeed > maxchange {
                anglespeed = maxchange
            }
            if anglespeed < -maxchange {
                anglespeed = -maxchange
            }
            (*bs).viewangles[i as usize] += anglespeed;
            (*bs).viewangles[i as usize] =
                crate::src::qcommon::q_math::AngleMod((*bs).viewangles[i as usize]);
            //demping
            (*bs).viewanglespeed[i as usize] = ((*bs).viewanglespeed[i as usize] as f64
                * (0.45 * (1f32 - factor) as f64))
                as crate::src::qcommon::q_shared::vec_t
        }
    }
    //bs->viewangles[PITCH] = 0;
    if (*bs).viewangles[0] > 180f32 {
        (*bs).viewangles[0] -= 360f32
    }
    //elementary action: view
    crate::src::game::g_syscalls::trap_EA_View((*bs).client, (*bs).viewangles.as_mut_ptr());
}
/*
==============
BotInputToUserCommand
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotInputToUserCommand(
    mut bi: *mut crate::botlib_h::bot_input_t,
    mut ucmd: *mut crate::src::qcommon::q_shared::usercmd_t,
    mut delta_angles: *mut i32,
    mut time: i32,
) {
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut temp: i16 = 0;
    let mut j: i32 = 0;
    let mut f: f32 = 0.;
    let mut r: f32 = 0.;
    let mut u: f32 = 0.;
    let mut m: f32 = 0.;
    //clear the whole structure
    crate::stdlib::memset(
        ucmd as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::usercmd_t>(),
    );
    //the duration for the user command in milli seconds
    (*ucmd).serverTime = time;
    //
    if (*bi).actionflags & 0x8000 != 0 {
        (*bi).actionflags |= 0x10;
        (*bi).actionflags &= !(0x8000)
    }
    //set the buttons
    if (*bi).actionflags & 0x8 != 0 {
        (*ucmd).buttons = 1
    }
    if (*bi).actionflags & 0x1 != 0 {
        (*ucmd).buttons |= 1
    }
    if (*bi).actionflags & 0x10000 != 0 {
        (*ucmd).buttons |= 2
    }
    if (*bi).actionflags & 0x20000 != 0 {
        (*ucmd).buttons |= 8
    }
    if (*bi).actionflags & 0x2 != 0 {
        (*ucmd).buttons |= 4
    }
    if (*bi).actionflags & 0x80000 != 0 {
        (*ucmd).buttons |= 16
    }
    if (*bi).actionflags & 0x100000 != 0 {
        (*ucmd).buttons |= 32
    }
    if (*bi).actionflags & 0x200000 != 0 {
        (*ucmd).buttons |= 64
    }
    if (*bi).actionflags & 0x800000 != 0 {
        (*ucmd).buttons |= 128
    }
    if (*bi).actionflags & 0x1000000 != 0 {
        (*ucmd).buttons |= 256
    }
    if (*bi).actionflags & 0x2000000 != 0 {
        (*ucmd).buttons |= 512
    }
    if (*bi).actionflags & 0x8000000 != 0 {
        (*ucmd).buttons |= 1024
    }
    //
    (*ucmd).weapon = (*bi).weapon as crate::src::qcommon::q_shared::byte;
    //set the view angles
    //NOTE: the ucmd->angles are the angles WITHOUT the delta angles
    (*ucmd).angles[0] = ((*bi).viewangles[0] * 65536f32 / 360f32) as i32 & 65535;
    (*ucmd).angles[1] = ((*bi).viewangles[1] * 65536f32 / 360f32) as i32 & 65535;
    (*ucmd).angles[2] = ((*bi).viewangles[2] * 65536f32 / 360f32) as i32 & 65535;

    for j in 0..3 {
        temp = ((*ucmd).angles[j as usize] - *delta_angles.offset(j as isize)) as i16;

        (*ucmd).angles[j as usize] = temp as i32;
    }
    //NOTE: movement is relative to the REAL view angles
    //get the horizontal forward and right vector
    //get the pitch in the range [-180, 180]
    if (*bi).dir[2] != 0. {
        angles[0] = (*bi).viewangles[0]
    } else {
        angles[0] = 0f32
    }
    angles[1] = (*bi).viewangles[1];
    angles[2] = 0f32;
    crate::src::qcommon::q_math::AngleVectors(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    //bot input speed is in the range [0, 400]
    (*bi).speed = (*bi).speed * 127f32 / 400f32;
    //set the view independent movement
    f = forward[0] * (*bi).dir[0] + forward[1] * (*bi).dir[1] + forward[2] * (*bi).dir[2];
    r = right[0] * (*bi).dir[0] + right[1] * (*bi).dir[1] + right[2] * (*bi).dir[2];
    u = (crate::stdlib::fabs(forward[2] as f64) * (*bi).dir[2] as f64) as f32;
    m = crate::stdlib::fabs(f as f64) as f32;
    if crate::stdlib::fabs(r as f64) > m as f64 {
        m = crate::stdlib::fabs(r as f64) as f32
    }
    if crate::stdlib::fabs(u as f64) > m as f64 {
        m = crate::stdlib::fabs(u as f64) as f32
    }
    if m > 0f32 {
        f *= (*bi).speed / m;
        r *= (*bi).speed / m;
        u *= (*bi).speed / m
    }
    (*ucmd).forwardmove = f as i8;
    (*ucmd).rightmove = r as i8;
    (*ucmd).upmove = u as i8;
    if (*bi).actionflags & 0x200 != 0 {
        (*ucmd).forwardmove = 127i8
    }
    if (*bi).actionflags & 0x800 != 0 {
        (*ucmd).forwardmove = -127i8
    }
    if (*bi).actionflags & 0x1000 != 0 {
        (*ucmd).rightmove = -127i8
    }
    if (*bi).actionflags & 0x2000 != 0 {
        (*ucmd).rightmove = 127i8
    }
    //jump/moveup
    if (*bi).actionflags & 0x10 != 0 {
        (*ucmd).upmove = 127i8
    }
    //crouch/movedown
    if (*bi).actionflags & 0x80 != 0 {
        (*ucmd).upmove = -127i8
    };
}
/*
==============
BotUpdateInput
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotUpdateInput(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut time: i32,
    mut elapsed_time: i32,
) {
    let mut bi: crate::botlib_h::bot_input_t = crate::botlib_h::bot_input_t {
        thinktime: 0.,
        dir: [0.; 3],
        speed: 0.,
        viewangles: [0.; 3],
        actionflags: 0,
        weapon: 0,
    };
    let mut j: i32 = 0;
    //add the delta angles to the bot's current view angles
    j = 0;
    while j < 3 {
        (*bs).viewangles[j as usize] = crate::src::qcommon::q_math::AngleMod(
            ((*bs).viewangles[j as usize] as f64
                + (*bs).cur_ps.delta_angles[j as usize] as f64 * (360.0 / 65536f64))
                as f32,
        );
        j += 1
    }
    //change the bot view angles
    BotChangeViewAngles(bs, elapsed_time as f32 / 1000f32);
    //retrieve the bot input
    crate::src::game::g_syscalls::trap_EA_GetInput(
        (*bs).client,
        time as f32 / 1000f32,
        &mut bi as *mut crate::botlib_h::bot_input_t as *mut libc::c_void,
    );
    //respawn hack
    if bi.actionflags & 0x8 != 0 {
        if (*bs).lastucmd.buttons & 1 != 0 {
            bi.actionflags &= !(0x8 | 0x1)
        }
    }
    //convert the bot input to a usercmd
    BotInputToUserCommand(
        &mut bi,
        &mut (*bs).lastucmd,
        (*bs).cur_ps.delta_angles.as_mut_ptr(),
        time,
    );
    //subtract the delta angles
    j = 0;
    while j < 3 {
        (*bs).viewangles[j as usize] = crate::src::qcommon::q_math::AngleMod(
            ((*bs).viewangles[j as usize] as f64
                - (*bs).cur_ps.delta_angles[j as usize] as f64 * (360.0 / 65536f64))
                as f32,
        );
        j += 1
    }
}
/*
==============
BotAIRegularUpdate
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAIRegularUpdate() {
    if regularupdate_time < floattime {
        crate::src::game::g_syscalls::trap_BotUpdateEntityItems();
        regularupdate_time = (floattime as f64 + 0.3) as f32
    };
}
/*
==============
RemoveColorEscapeSequences
==============
*/
#[no_mangle]

pub unsafe extern "C" fn RemoveColorEscapeSequences(mut text: *mut i8) {
    let mut i: i32 = 0;
    let mut l: i32 = 0;
    l = 0;
    i = 0;
    while *text.offset(i as isize) != 0 {
        if crate::src::qcommon::q_shared::Q_IsColorString(&mut *text.offset(i as isize)) as u64 != 0
        {
            i += 1
        } else if !(*text.offset(i as isize) as i32 > 0x7e) {
            let fresh0 = l;
            l = l + 1;
            *text.offset(fresh0 as isize) = *text.offset(i as isize)
        }
        i += 1
    }
    *text.offset(l as isize) = '\u{0}' as i8;
}
/*
==============
BotAI
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAI(mut client: i32, mut thinktime: f32) -> i32 {
    let mut bs: *mut crate::src::game::ai_main::bot_state_t =
        0 as *mut crate::src::game::ai_main::bot_state_t;
    let mut buf: [i8; 1024] = [0; 1024];
    let mut args: *mut i8 = 0 as *mut i8;
    let mut j: i32 = 0;
    crate::src::game::g_syscalls::trap_EA_ResetInput(client);
    //
    bs = botstates[client as usize];
    if bs.is_null() || (*bs).inuse == 0 {
        BotAI_Print(
            4,
            b"BotAI: client %d is not setup\n\x00" as *const u8 as *mut i8,
            client,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //retrieve the current client state
    if BotAI_GetClientState(client, &mut (*bs).cur_ps) == 0 {
        BotAI_Print(
            4,
            b"BotAI: failed to get player state for player %d\n\x00" as *const u8 as *mut i8,
            client,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //retrieve any waiting server commands
    while crate::src::game::g_syscalls::trap_BotGetServerCommand(
        client,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    ) != 0
    {
        //have buf point to the command and args to the command arguments
        args = crate::stdlib::strchr(buf.as_mut_ptr(), ' ' as i32);
        if args.is_null() {
            continue;
        }
        let fresh1 = args;
        args = args.offset(1);
        *fresh1 = '\u{0}' as i8;
        //remove color espace sequences from the arguments
        RemoveColorEscapeSequences(args);
        if !(crate::src::qcommon::q_shared::Q_stricmp(
            buf.as_mut_ptr(),
            b"cp \x00" as *const u8 as *const i8,
        ) == 0)
        {
            if !(crate::src::qcommon::q_shared::Q_stricmp(
                buf.as_mut_ptr(),
                b"cs\x00" as *const u8 as *const i8,
            ) == 0)
            {
                if crate::src::qcommon::q_shared::Q_stricmp(
                    buf.as_mut_ptr(),
                    b"print\x00" as *const u8 as *const i8,
                ) == 0
                {
                    //remove first and last quote from the chat message
                    crate::stdlib::memmove(
                        args as *mut libc::c_void,
                        args.offset(1) as *const libc::c_void,
                        crate::stdlib::strlen(args),
                    );
                    *args.offset(crate::stdlib::strlen(args).wrapping_sub(1usize) as isize) =
                        '\u{0}' as i8;
                    crate::src::game::g_syscalls::trap_BotQueueConsoleMessage((*bs).cs, 0i32, args);
                } else if crate::src::qcommon::q_shared::Q_stricmp(
                    buf.as_mut_ptr(),
                    b"chat\x00" as *const u8 as *const i8,
                ) == 0
                {
                    //remove first and last quote from the chat message
                    crate::stdlib::memmove(
                        args as *mut libc::c_void,
                        args.offset(1) as *const libc::c_void,
                        crate::stdlib::strlen(args),
                    );
                    *args.offset(crate::stdlib::strlen(args).wrapping_sub(1usize) as isize) =
                        '\u{0}' as i8;
                    crate::src::game::g_syscalls::trap_BotQueueConsoleMessage((*bs).cs, 1i32, args);
                } else if crate::src::qcommon::q_shared::Q_stricmp(
                    buf.as_mut_ptr(),
                    b"tchat\x00" as *const u8 as *const i8,
                ) == 0
                {
                    //remove first and last quote from the chat message
                    crate::stdlib::memmove(
                        args as *mut libc::c_void,
                        args.offset(1) as *const libc::c_void,
                        crate::stdlib::strlen(args),
                    );
                    *args.offset(crate::stdlib::strlen(args).wrapping_sub(1usize) as isize) =
                        '\u{0}' as i8;
                    crate::src::game::g_syscalls::trap_BotQueueConsoleMessage((*bs).cs, 1i32, args);
                } else if !(crate::src::qcommon::q_shared::Q_stricmp(
                    buf.as_mut_ptr(),
                    b"scores\x00" as *const u8 as *const i8,
                ) == 0)
                {
                    (crate::src::qcommon::q_shared::Q_stricmp(
                        buf.as_mut_ptr(),
                        b"clientLevelShot\x00" as *const u8 as *const i8,
                    )) == 0;
                }
            }
        }
    }
    //add the delta angles to the bot's current view angles
    j = 0;
    while j < 3 {
        (*bs).viewangles[j as usize] = crate::src::qcommon::q_math::AngleMod(
            ((*bs).viewangles[j as usize] as f64
                + (*bs).cur_ps.delta_angles[j as usize] as f64 * (360.0 / 65536f64))
                as f32,
        );
        j += 1
    }
    //increase the local time of the bot
    (*bs).ltime += thinktime;
    //
    (*bs).thinktime = thinktime;
    //origin of the bot
    (*bs).origin[0] = (*bs).cur_ps.origin[0];
    (*bs).origin[1] = (*bs).cur_ps.origin[1];
    (*bs).origin[2] = (*bs).cur_ps.origin[2];
    //eye coordinates of the bot
    (*bs).eye[0] = (*bs).cur_ps.origin[0];
    (*bs).eye[1] = (*bs).cur_ps.origin[1];
    (*bs).eye[2] = (*bs).cur_ps.origin[2];
    (*bs).eye[2] += (*bs).cur_ps.viewheight as f32;
    //get the area the bot is in
    (*bs).areanum = crate::src::game::ai_dmq3::BotPointAreaNum((*bs).origin.as_mut_ptr());
    //the real AI
    crate::src::game::ai_dmq3::BotDeathmatchAI(bs, thinktime);
    //set the weapon selection every AI frame
    crate::src::game::g_syscalls::trap_EA_SelectWeapon((*bs).client, (*bs).weaponnum);
    //subtract the delta angles
    j = 0;
    while j < 3 {
        (*bs).viewangles[j as usize] = crate::src::qcommon::q_math::AngleMod(
            ((*bs).viewangles[j as usize] as f64
                - (*bs).cur_ps.delta_angles[j as usize] as f64 * (360.0 / 65536f64))
                as f32,
        );
        j += 1
    }
    //everything was ok
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotScheduleBotThink
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotScheduleBotThink() {
    let mut i: i32 = 0;
    let mut botnum: i32 = 0;
    botnum = 0;
    i = 0;
    while i < 64 {
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            //initialize the bot think residual time
            (*botstates[i as usize]).botthink_residual = bot_thinktime.integer * botnum / numbots;
            botnum += 1
        }
        i += 1
    }
}
/*
==============
BotWriteSessionData
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotWriteSessionData(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut s: *const i8 = 0 as *const i8;
    let mut var: *const i8 = 0 as *const i8;
    s = crate::src::qcommon::q_shared::va(
        b"%i %i %i %i %i %i %i %i %f %f %f %f %f %f %f %f %f %f\x00" as *const u8 as *mut i8,
        (*bs).lastgoal_decisionmaker,
        (*bs).lastgoal_ltgtype,
        (*bs).lastgoal_teammate,
        (*bs).lastgoal_teamgoal.areanum,
        (*bs).lastgoal_teamgoal.entitynum,
        (*bs).lastgoal_teamgoal.flags,
        (*bs).lastgoal_teamgoal.iteminfo,
        (*bs).lastgoal_teamgoal.number,
        (*bs).lastgoal_teamgoal.origin[0usize] as f64,
        (*bs).lastgoal_teamgoal.origin[1usize] as f64,
        (*bs).lastgoal_teamgoal.origin[2usize] as f64,
        (*bs).lastgoal_teamgoal.mins[0usize] as f64,
        (*bs).lastgoal_teamgoal.mins[1usize] as f64,
        (*bs).lastgoal_teamgoal.mins[2usize] as f64,
        (*bs).lastgoal_teamgoal.maxs[0usize] as f64,
        (*bs).lastgoal_teamgoal.maxs[1usize] as f64,
        (*bs).lastgoal_teamgoal.maxs[2usize] as f64,
        (*bs).formation_dist as f64,
    );
    var = crate::src::qcommon::q_shared::va(
        b"botsession%i\x00" as *const u8 as *mut i8,
        (*bs).client,
    );
    crate::src::game::g_syscalls::trap_Cvar_Set(var, s);
}
/*
==============
BotReadSessionData
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotReadSessionData(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut s: [i8; 1024] = [0; 1024];
    let mut var: *const i8 = 0 as *const i8;
    var = crate::src::qcommon::q_shared::va(
        b"botsession%i\x00" as *const u8 as *mut i8,
        (*bs).client,
    );
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        var,
        s.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::stdlib::sscanf(
        s.as_mut_ptr(),
        b"%i %i %i %i %i %i %i %i %f %f %f %f %f %f %f %f %f %f\x00" as *const u8 as *const i8,
        &mut (*bs).lastgoal_decisionmaker as *mut i32,
        &mut (*bs).lastgoal_ltgtype as *mut i32,
        &mut (*bs).lastgoal_teammate as *mut i32,
        &mut (*bs).lastgoal_teamgoal.areanum as *mut i32,
        &mut (*bs).lastgoal_teamgoal.entitynum as *mut i32,
        &mut (*bs).lastgoal_teamgoal.flags as *mut i32,
        &mut (*bs).lastgoal_teamgoal.iteminfo as *mut i32,
        &mut (*bs).lastgoal_teamgoal.number as *mut i32,
        &mut *(*bs).lastgoal_teamgoal.origin.as_mut_ptr().offset(0isize)
            as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs).lastgoal_teamgoal.origin.as_mut_ptr().offset(1isize)
            as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs).lastgoal_teamgoal.origin.as_mut_ptr().offset(2isize)
            as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs).lastgoal_teamgoal.mins.as_mut_ptr().offset(0isize)
            as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs).lastgoal_teamgoal.mins.as_mut_ptr().offset(1isize)
            as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs).lastgoal_teamgoal.mins.as_mut_ptr().offset(2isize)
            as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs).lastgoal_teamgoal.maxs.as_mut_ptr().offset(0isize)
            as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs).lastgoal_teamgoal.maxs.as_mut_ptr().offset(1isize)
            as *mut crate::src::qcommon::q_shared::vec_t,
        &mut *(*bs).lastgoal_teamgoal.maxs.as_mut_ptr().offset(2isize)
            as *mut crate::src::qcommon::q_shared::vec_t,
        &mut (*bs).formation_dist as *mut f32,
    );
}
/*
==============
BotAISetupClient
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAISetupClient(
    mut client: i32,
    mut settings: *mut crate::g_local_h::bot_settings_s,
    mut restart: crate::src::qcommon::q_shared::qboolean,
) -> i32 {
    let mut filename: [i8; 144] = [0; 144];
    let mut name: [i8; 144] = [0; 144];
    let mut gender: [i8; 144] = [0; 144];
    let mut bs: *mut crate::src::game::ai_main::bot_state_t =
        0 as *mut crate::src::game::ai_main::bot_state_t;
    let mut errnum: i32 = 0;
    if botstates[client as usize].is_null() {
        botstates[client as usize] = crate::src::game::g_mem::G_Alloc(::std::mem::size_of::<
            crate::src::game::ai_main::bot_state_t,
        >() as i32)
            as *mut crate::src::game::ai_main::bot_state_t
    }
    bs = botstates[client as usize];
    if bs.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if !bs.is_null() && (*bs).inuse != 0 {
        BotAI_Print(
            4,
            b"BotAISetupClient: client %d already setup\n\x00" as *const u8 as *mut i8,
            client,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if crate::src::game::g_syscalls::trap_AAS_Initialized() == 0 {
        BotAI_Print(4, b"AAS not initialized\n\x00" as *const u8 as *mut i8);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //load the bot character
    (*bs).character = crate::src::game::g_syscalls::trap_BotLoadCharacter(
        (*settings).characterfile.as_mut_ptr(),
        (*settings).skill,
    );
    if (*bs).character == 0 {
        BotAI_Print(
            4,
            b"couldn\'t load skill %f from %s\n\x00" as *const u8 as *mut i8,
            (*settings).skill as f64,
            (*settings).characterfile.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //copy the settings
    crate::stdlib::memcpy(
        &mut (*bs).settings as *mut crate::g_local_h::bot_settings_t as *mut libc::c_void,
        settings as *const libc::c_void,
        ::std::mem::size_of::<crate::g_local_h::bot_settings_t>(),
    );
    //allocate a goal state
    (*bs).gs = crate::src::game::g_syscalls::trap_BotAllocGoalState(client);
    //load the item weights
    crate::src::game::g_syscalls::trap_Characteristic_String(
        (*bs).character,
        40,
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    errnum = crate::src::game::g_syscalls::trap_BotLoadItemWeights((*bs).gs, filename.as_mut_ptr());
    if errnum != 0 {
        crate::src::game::g_syscalls::trap_BotFreeGoalState((*bs).gs);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //allocate a weapon state
    (*bs).ws = crate::src::game::g_syscalls::trap_BotAllocWeaponState();
    //load the weapon weights
    crate::src::game::g_syscalls::trap_Characteristic_String(
        (*bs).character,
        3,
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    errnum =
        crate::src::game::g_syscalls::trap_BotLoadWeaponWeights((*bs).ws, filename.as_mut_ptr());
    if errnum != 0 {
        crate::src::game::g_syscalls::trap_BotFreeGoalState((*bs).gs);
        crate::src::game::g_syscalls::trap_BotFreeWeaponState((*bs).ws);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //allocate a chat state
    (*bs).cs = crate::src::game::g_syscalls::trap_BotAllocChatState();
    //load the chat file
    crate::src::game::g_syscalls::trap_Characteristic_String(
        (*bs).character,
        21,
        filename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    crate::src::game::g_syscalls::trap_Characteristic_String(
        (*bs).character,
        22,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    errnum = crate::src::game::g_syscalls::trap_BotLoadChatFile(
        (*bs).cs,
        filename.as_mut_ptr(),
        name.as_mut_ptr(),
    );
    if errnum != 0 {
        crate::src::game::g_syscalls::trap_BotFreeChatState((*bs).cs);
        crate::src::game::g_syscalls::trap_BotFreeGoalState((*bs).gs);
        crate::src::game::g_syscalls::trap_BotFreeWeaponState((*bs).ws);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //get the gender characteristic
    crate::src::game::g_syscalls::trap_Characteristic_String(
        (*bs).character,
        1,
        gender.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    //set the chat gender
    if *gender.as_mut_ptr() as i32 == 'f' as i32 || *gender.as_mut_ptr() as i32 == 'F' as i32 {
        crate::src::game::g_syscalls::trap_BotSetChatGender((*bs).cs, 1i32);
    } else if *gender.as_mut_ptr() as i32 == 'm' as i32 || *gender.as_mut_ptr() as i32 == 'M' as i32
    {
        crate::src::game::g_syscalls::trap_BotSetChatGender((*bs).cs, 2i32);
    } else {
        crate::src::game::g_syscalls::trap_BotSetChatGender((*bs).cs, 0i32);
    }
    (*bs).inuse = crate::src::qcommon::q_shared::qtrue as i32;
    (*bs).client = client;
    (*bs).entitynum = client;
    (*bs).setupcount = 4;
    (*bs).entergame_time = floattime;
    (*bs).ms = crate::src::game::g_syscalls::trap_BotAllocMoveState();
    (*bs).walker =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 48, 0f32, 1f32);
    numbots += 1;
    if crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
        b"bot_testichat\x00" as *const u8 as *const i8,
    ) != 0
    {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"bot_testichat\x00" as *const u8 as *mut i8,
            b"1\x00" as *const u8 as *mut i8,
        );
        crate::src::game::ai_chat::BotChatTest(bs);
    }
    //NOTE: reschedule the bot thinking
    BotScheduleBotThink();
    //if interbreeding start with a mutation
    if bot_interbreed != 0 {
        crate::src::game::g_syscalls::trap_BotMutateGoalFuzzyLogic((*bs).gs, 1f32);
    }
    // if we kept the bot client
    if restart as u64 != 0 {
        BotReadSessionData(bs);
    }
    //bot has been setup successfully
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==============
BotAIShutdownClient
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAIShutdownClient(
    mut client: i32,
    mut restart: crate::src::qcommon::q_shared::qboolean,
) -> i32 {
    let mut bs: *mut crate::src::game::ai_main::bot_state_t =
        0 as *mut crate::src::game::ai_main::bot_state_t;
    bs = botstates[client as usize];
    if bs.is_null() || (*bs).inuse == 0 {
        //BotAI_Print(PRT_ERROR, "BotAIShutdownClient: client %d already shutdown\n", client);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if restart as u64 != 0 {
        BotWriteSessionData(bs);
    }
    if crate::src::game::ai_chat::BotChat_ExitGame(bs) != 0 {
        crate::src::game::g_syscalls::trap_BotEnterChat((*bs).cs, (*bs).client, 0i32);
    }
    crate::src::game::g_syscalls::trap_BotFreeMoveState((*bs).ms);
    //free the goal state
    crate::src::game::g_syscalls::trap_BotFreeGoalState((*bs).gs);
    //free the chat file
    crate::src::game::g_syscalls::trap_BotFreeChatState((*bs).cs);
    //free the weapon weights
    crate::src::game::g_syscalls::trap_BotFreeWeaponState((*bs).ws);
    //free the bot character
    crate::src::game::g_syscalls::trap_BotFreeCharacter((*bs).character);
    //
    crate::src::game::ai_dmq3::BotFreeWaypoints((*bs).checkpoints);
    crate::src::game::ai_dmq3::BotFreeWaypoints((*bs).patrolpoints);
    //clear activate goal stack
    crate::src::game::ai_dmq3::BotClearActivateGoalStack(bs);
    //clear the bot state
    crate::stdlib::memset(
        bs as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::game::ai_main::bot_state_t>(),
    );
    //set the inuse flag to qfalse
    (*bs).inuse = crate::src::qcommon::q_shared::qfalse as i32;
    //there's one bot less
    numbots -= 1;
    //everything went ok
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//resets the whole bot state
/*
==============
BotResetState

called when a bot enters the intermission or observer mode and
when the level is changed
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotResetState(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut client: i32 = 0; //current player state
    let mut entitynum: i32 = 0;
    let mut inuse: i32 = 0;
    let mut movestate: i32 = 0;
    let mut goalstate: i32 = 0;
    let mut chatstate: i32 = 0;
    let mut weaponstate: i32 = 0;
    let mut settings: crate::g_local_h::bot_settings_t = crate::g_local_h::bot_settings_t {
        characterfile: [0; 144],
        skill: 0.,
    };
    let mut character: i32 = 0;
    let mut ps: crate::src::qcommon::q_shared::playerState_t =
        crate::src::qcommon::q_shared::playerState_t {
            commandTime: 0,
            pm_type: 0,
            bobCycle: 0,
            pm_flags: 0,
            pm_time: 0,
            origin: [0.; 3],
            velocity: [0.; 3],
            weaponTime: 0,
            gravity: 0,
            speed: 0,
            delta_angles: [0; 3],
            groundEntityNum: 0,
            legsTimer: 0,
            legsAnim: 0,
            torsoTimer: 0,
            torsoAnim: 0,
            movementDir: 0,
            grapplePoint: [0.; 3],
            eFlags: 0,
            eventSequence: 0,
            events: [0; 2],
            eventParms: [0; 2],
            externalEvent: 0,
            externalEventParm: 0,
            externalEventTime: 0,
            clientNum: 0,
            weapon: 0,
            weaponstate: 0,
            viewangles: [0.; 3],
            viewheight: 0,
            damageEvent: 0,
            damageYaw: 0,
            damagePitch: 0,
            damageCount: 0,
            stats: [0; 16],
            persistant: [0; 16],
            powerups: [0; 16],
            ammo: [0; 16],
            generic1: 0,
            loopSound: 0,
            jumppad_ent: 0,
            ping: 0,
            pmove_framecount: 0,
            jumppad_frame: 0,
            entityEventSequence: 0,
        };
    let mut entergame_time: f32 = 0.;
    //save some things that should not be reset here
    crate::stdlib::memcpy(
        &mut settings as *mut crate::g_local_h::bot_settings_t as *mut libc::c_void,
        &mut (*bs).settings as *mut crate::g_local_h::bot_settings_t as *const libc::c_void,
        ::std::mem::size_of::<crate::g_local_h::bot_settings_t>(),
    );
    crate::stdlib::memcpy(
        &mut ps as *mut crate::src::qcommon::q_shared::playerState_t as *mut libc::c_void,
        &mut (*bs).cur_ps as *mut crate::src::qcommon::q_shared::playerState_t
            as *const libc::c_void,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::playerState_t>(),
    );
    inuse = (*bs).inuse;
    client = (*bs).client;
    entitynum = (*bs).entitynum;
    character = (*bs).character;
    movestate = (*bs).ms;
    goalstate = (*bs).gs;
    chatstate = (*bs).cs;
    weaponstate = (*bs).ws;
    entergame_time = (*bs).entergame_time;
    //free checkpoints and patrol points
    crate::src::game::ai_dmq3::BotFreeWaypoints((*bs).checkpoints);
    crate::src::game::ai_dmq3::BotFreeWaypoints((*bs).patrolpoints);
    //reset the whole state
    crate::stdlib::memset(
        bs as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::game::ai_main::bot_state_t>(),
    );
    //copy back some state stuff that should not be reset
    (*bs).ms = movestate;
    (*bs).gs = goalstate;
    (*bs).cs = chatstate;
    (*bs).ws = weaponstate;
    crate::stdlib::memcpy(
        &mut (*bs).cur_ps as *mut crate::src::qcommon::q_shared::playerState_t as *mut libc::c_void,
        &mut ps as *mut crate::src::qcommon::q_shared::playerState_t as *const libc::c_void,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::playerState_t>(),
    );
    crate::stdlib::memcpy(
        &mut (*bs).settings as *mut crate::g_local_h::bot_settings_t as *mut libc::c_void,
        &mut settings as *mut crate::g_local_h::bot_settings_t as *const libc::c_void,
        ::std::mem::size_of::<crate::g_local_h::bot_settings_t>(),
    );
    (*bs).inuse = inuse;
    (*bs).client = client;
    (*bs).entitynum = entitynum;
    (*bs).character = character;
    (*bs).entergame_time = entergame_time;
    //reset several states
    if (*bs).ms != 0 {
        crate::src::game::g_syscalls::trap_BotResetMoveState((*bs).ms);
    }
    if (*bs).gs != 0 {
        crate::src::game::g_syscalls::trap_BotResetGoalState((*bs).gs);
    }
    if (*bs).ws != 0 {
        crate::src::game::g_syscalls::trap_BotResetWeaponState((*bs).ws);
    }
    if (*bs).gs != 0 {
        crate::src::game::g_syscalls::trap_BotResetAvoidGoals((*bs).gs);
    }
    if (*bs).ms != 0 {
        crate::src::game::g_syscalls::trap_BotResetAvoidReach((*bs).ms);
    };
}
/*
==============
BotAILoadMap
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAILoadMap(mut restart: i32) -> i32 {
    let mut i: i32 = 0;
    let mut mapname: crate::src::qcommon::q_shared::vmCvar_t =
        crate::src::qcommon::q_shared::vmCvar_t {
            handle: 0,
            modificationCount: 0,
            value: 0.,
            integer: 0,
            string: [0; 256],
        };
    if restart == 0 {
        crate::src::game::g_syscalls::trap_Cvar_Register(
            &mut mapname,
            b"mapname\x00" as *const u8 as *const i8,
            b"\x00" as *const u8 as *const i8,
            0x4 | 0x40,
        );
        crate::src::game::g_syscalls::trap_BotLibLoadMap(mapname.string.as_mut_ptr());
    }

    for i in 0..64 {
        if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
            BotResetState(botstates[i as usize]);
            (*botstates[i as usize]).setupcount = 4
        }
    }
    crate::src::game::ai_dmq3::BotSetupDeathmatchAI();
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotAIStartFrame
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAIStartFrame(mut time: i32) -> i32 {
    let mut i: i32 = 0;
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut state: crate::botlib_h::bot_entitystate_t = crate::botlib_h::bot_entitystate_t {
        type_0: 0,
        flags: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        mins: [0.; 3],
        maxs: [0.; 3],
        groundent: 0,
        solid: 0,
        modelindex: 0,
        modelindex2: 0,
        frame: 0,
        event: 0,
        eventParm: 0,
        powerups: 0,
        weapon: 0,
        legsAnim: 0,
        torsoAnim: 0,
    };
    let mut elapsed_time: i32 = 0;
    let mut thinktime: i32 = 0;
    static mut local_time: i32 = 0;
    static mut botlib_residual: i32 = 0;
    static mut lastbotthink_time: i32 = 0;
    crate::src::game::g_bot::G_CheckBotSpawn();
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut crate::src::game::ai_dmq3::bot_rocketjump);
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut crate::src::game::ai_dmq3::bot_grapple);
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut crate::src::game::ai_dmq3::bot_fastchat);
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut crate::src::game::ai_dmq3::bot_nochat);
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut crate::src::game::ai_dmq3::bot_testrchat);
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_thinktime);
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_memorydump);
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_saveroutingcache);
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_pause);
    crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_report);
    if bot_report.integer != 0 {
        //		BotTeamplayReport();
        //		trap_Cvar_Set("bot_report", "0");
        BotUpdateInfoConfigStrings();
    }
    if bot_pause.integer != 0 {
        // execute bot user commands every frame
        i = 0;
        while i < 64 {
            if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
                if !((*crate::src::game::g_main::g_entities[i as usize].client)
                    .pers
                    .connected
                    != crate::g_local_h::CON_CONNECTED)
                {
                    (*botstates[i as usize]).lastucmd.forwardmove = 0;
                    (*botstates[i as usize]).lastucmd.rightmove = 0;
                    (*botstates[i as usize]).lastucmd.upmove = 0;
                    (*botstates[i as usize]).lastucmd.buttons = 0;
                    (*botstates[i as usize]).lastucmd.serverTime = time;
                    crate::src::game::g_syscalls::trap_BotUserCommand(
                        (*botstates[i as usize]).client,
                        &mut (**botstates.as_mut_ptr().offset(i as isize)).lastucmd,
                    );
                }
            }
            i += 1
        }
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    if bot_memorydump.integer != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"memorydump\x00" as *const u8 as *mut i8,
            b"1\x00" as *const u8 as *mut i8,
        );
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"bot_memorydump\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
    }
    if bot_saveroutingcache.integer != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"saveroutingcache\x00" as *const u8 as *mut i8,
            b"1\x00" as *const u8 as *mut i8,
        );
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"bot_saveroutingcache\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
    }
    //check if bot interbreeding is activated
    BotInterbreeding();
    //cap the bot think time
    if bot_thinktime.integer > 200 {
        crate::src::game::g_syscalls::trap_Cvar_Set(
            b"bot_thinktime\x00" as *const u8 as *const i8,
            b"200\x00" as *const u8 as *const i8,
        );
    }
    //if the bot think time changed we should reschedule the bots
    if bot_thinktime.integer != lastbotthink_time {
        lastbotthink_time = bot_thinktime.integer;
        BotScheduleBotThink();
    }
    elapsed_time = time - local_time;
    local_time = time;
    botlib_residual += elapsed_time;
    if elapsed_time > bot_thinktime.integer {
        thinktime = elapsed_time
    } else {
        thinktime = bot_thinktime.integer
    }
    // update the bot library
    if botlib_residual >= thinktime {
        botlib_residual -= thinktime;
        crate::src::game::g_syscalls::trap_BotLibStartFrame(time as f32 / 1000f32);
        if crate::src::game::g_syscalls::trap_AAS_Initialized() == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //update entities in the botlib
        i = 0;
        while i < (1) << 10 {
            ent = &mut *crate::src::game::g_main::g_entities
                .as_mut_ptr()
                .offset(i as isize) as *mut crate::g_local_h::gentity_t;
            if (*ent).inuse as u64 == 0 {
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if (*ent).r.linked as u64 == 0 {
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if (*ent).r.svFlags & 0x1 != 0 {
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if (*ent).s.eType == crate::bg_public_h::ET_MISSILE as i32
                && (*ent).s.weapon != crate::bg_public_h::WP_GRAPPLING_HOOK as i32
            {
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else if (*ent).s.eType > crate::bg_public_h::ET_EVENTS as i32 {
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(i, 0 as *mut libc::c_void);
            } else {
                // do not update missiles
                // do not update event only entities
                //
                crate::stdlib::memset(
                    &mut state as *mut crate::botlib_h::bot_entitystate_t as *mut libc::c_void,
                    0,
                    ::std::mem::size_of::<crate::botlib_h::bot_entitystate_t>(),
                );
                //
                state.origin[0] = (*ent).r.currentOrigin[0];
                state.origin[1] = (*ent).r.currentOrigin[1];
                state.origin[2] = (*ent).r.currentOrigin[2];
                if i < 64 {
                    state.angles[0] = (*ent).s.apos.trBase[0];
                    state.angles[1] = (*ent).s.apos.trBase[1];
                    state.angles[2] = (*ent).s.apos.trBase[2]
                } else {
                    state.angles[0] = (*ent).r.currentAngles[0];
                    state.angles[1] = (*ent).r.currentAngles[1];
                    state.angles[2] = (*ent).r.currentAngles[2]
                }
                state.old_origin[0] = (*ent).s.origin2[0];
                state.old_origin[1] = (*ent).s.origin2[1];
                state.old_origin[2] = (*ent).s.origin2[2];
                state.mins[0] = (*ent).r.mins[0];
                state.mins[1] = (*ent).r.mins[1];
                state.mins[2] = (*ent).r.mins[2];
                state.maxs[0] = (*ent).r.maxs[0];
                state.maxs[1] = (*ent).r.maxs[1];
                state.maxs[2] = (*ent).r.maxs[2];
                state.type_0 = (*ent).s.eType;
                state.flags = (*ent).s.eFlags;
                if (*ent).r.bmodel as u64 != 0 {
                    state.solid = crate::be_aas_h::SOLID_BSP as i32
                } else {
                    state.solid = crate::be_aas_h::SOLID_BBOX as i32
                }
                state.groundent = (*ent).s.groundEntityNum;
                state.modelindex = (*ent).s.modelindex;
                state.modelindex2 = (*ent).s.modelindex2;
                state.frame = (*ent).s.frame;
                state.event = (*ent).s.event;
                state.eventParm = (*ent).s.eventParm;
                state.powerups = (*ent).s.powerups;
                state.legsAnim = (*ent).s.legsAnim;
                state.torsoAnim = (*ent).s.torsoAnim;
                state.weapon = (*ent).s.weapon;
                //
                crate::src::game::g_syscalls::trap_BotLibUpdateEntity(
                    i,
                    &mut state as *mut crate::botlib_h::bot_entitystate_t as *mut libc::c_void,
                );
            }
            i += 1
        }
        BotAIRegularUpdate();
    }
    floattime = crate::src::game::g_syscalls::trap_AAS_Time();
    // execute scheduled bot AI
    i = 0;
    while i < 64 {
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            //
            (*botstates[i as usize]).botthink_residual += elapsed_time;
            //
            if (*botstates[i as usize]).botthink_residual >= thinktime {
                (*botstates[i as usize]).botthink_residual -= thinktime;
                if crate::src::game::g_syscalls::trap_AAS_Initialized() == 0 {
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
                if (*crate::src::game::g_main::g_entities[i as usize].client)
                    .pers
                    .connected
                    == crate::g_local_h::CON_CONNECTED
                {
                    BotAI(i, thinktime as f32 / 1000f32);
                }
            }
        }
        i += 1
    }
    // execute bot user commands every frame
    i = 0;
    while i < 64 {
        if !(botstates[i as usize].is_null() || (*botstates[i as usize]).inuse == 0) {
            if !((*crate::src::game::g_main::g_entities[i as usize].client)
                .pers
                .connected
                != crate::g_local_h::CON_CONNECTED)
            {
                BotUpdateInput(botstates[i as usize], time, elapsed_time);
                crate::src::game::g_syscalls::trap_BotUserCommand(
                    (*botstates[i as usize]).client,
                    &mut (**botstates.as_mut_ptr().offset(i as isize)).lastucmd,
                );
            }
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==============
BotInitLibrary
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotInitLibrary() -> i32 {
    let mut buf: [i8; 144] = [0; 144];
    //set the maxclients and maxentities library variables before calling BotSetupLibrary
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
        b"%d\x00" as *const u8 as *const i8,
        crate::src::game::g_main::level.maxclients,
    );
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"maxclients\x00" as *const u8 as *mut i8,
        buf.as_mut_ptr(),
    );
    crate::src::qcommon::q_shared::Com_sprintf(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
        b"%d\x00" as *const u8 as *const i8,
        (1i32) << 10i32,
    );
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"maxentities\x00" as *const u8 as *mut i8,
        buf.as_mut_ptr(),
    );
    //bsp checksum
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"sv_mapChecksum\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"sv_mapChecksum\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //maximum number of aas links
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"max_aaslinks\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"max_aaslinks\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //maximum number of items in a level
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"max_levelitems\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"max_levelitems\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //game type
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"g_gametype\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) == 0 {
        crate::stdlib::strcpy(buf.as_mut_ptr(), b"0\x00" as *const u8 as *const i8);
    }
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"g_gametype\x00" as *const u8 as *mut i8,
        buf.as_mut_ptr(),
    );
    //bot developer mode and log file
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"bot_developer\x00" as *const u8 as *mut i8,
        bot_developer.string.as_mut_ptr(),
    );
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"logfile\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"log\x00" as *const u8 as *mut i8,
        buf.as_mut_ptr(),
    );
    //no chatting
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_nochat\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"nochat\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //visualize jump pads
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_visualizejumppads\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"bot_visualizejumppads\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //forced clustering calculations
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_forceclustering\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"forceclustering\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //forced reachability calculations
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_forcereachability\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"forcereachability\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //force writing of AAS to file
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_forcewrite\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"forcewrite\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //no AAS optimization
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_aasoptimize\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"aasoptimize\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_saveroutingcache\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"saveroutingcache\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //reload instead of cache bot character files
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"bot_reloadcharacters\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) == 0 {
        crate::stdlib::strcpy(buf.as_mut_ptr(), b"0\x00" as *const u8 as *const i8);
    }
    crate::src::game::g_syscalls::trap_BotLibVarSet(
        b"bot_reloadcharacters\x00" as *const u8 as *mut i8,
        buf.as_mut_ptr(),
    );
    //base directory
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"fs_basepath\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"basedir\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //game directory
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"fs_game\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"gamedir\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //home directory
    crate::src::game::g_syscalls::trap_Cvar_VariableStringBuffer(
        b"fs_homepath\x00" as *const u8 as *const i8,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 144]>() as i32,
    );
    if crate::stdlib::strlen(buf.as_mut_ptr()) != 0 {
        crate::src::game::g_syscalls::trap_BotLibVarSet(
            b"homedir\x00" as *const u8 as *mut i8,
            buf.as_mut_ptr(),
        );
    }
    //
    //setup the bot library
    return crate::src::game::g_syscalls::trap_BotLibSetup();
}
/*
==============
BotAISetup
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAISetup(mut restart: i32) -> i32 {
    let mut errnum: i32 = 0;
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_thinktime,
        b"bot_thinktime\x00" as *const u8 as *const i8,
        b"100\x00" as *const u8 as *const i8,
        0x200,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_memorydump,
        b"bot_memorydump\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x200,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_saveroutingcache,
        b"bot_saveroutingcache\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x200,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_pause,
        b"bot_pause\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x200,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_report,
        b"bot_report\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x200,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_testsolid,
        b"bot_testsolid\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x200,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_testclusters,
        b"bot_testclusters\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x200,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_developer,
        b"bot_developer\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x200,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_interbreedchar,
        b"bot_interbreedchar\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        0,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_interbreedbots,
        b"bot_interbreedbots\x00" as *const u8 as *const i8,
        b"10\x00" as *const u8 as *const i8,
        0,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_interbreedcycle,
        b"bot_interbreedcycle\x00" as *const u8 as *const i8,
        b"20\x00" as *const u8 as *const i8,
        0,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_interbreedwrite,
        b"bot_interbreedwrite\x00" as *const u8 as *const i8,
        b"\x00" as *const u8 as *const i8,
        0,
    );
    //if the game is restarted for a tournament
    if restart != 0 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //initialize the bot states
    crate::stdlib::memset(
        botstates.as_mut_ptr() as *mut libc::c_void,
        0,
        ::std::mem::size_of::<[*mut crate::src::game::ai_main::bot_state_t; 64]>(),
    );
    errnum = BotInitLibrary();
    if errnum != 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==============
BotAIShutdown
==============
*/
#[no_mangle]

pub unsafe extern "C" fn BotAIShutdown(mut restart: i32) -> i32 {
    let mut i: i32 = 0;
    //if the game is restarted for a tournament
    if restart != 0 {
        //shutdown all the bots in the botlib
        i = 0;
        while i < 64 {
            if !botstates[i as usize].is_null() && (*botstates[i as usize]).inuse != 0 {
                BotAIShutdownClient(
                    (*botstates[i as usize]).client,
                    restart as crate::src::qcommon::q_shared::qboolean,
                );
            }
            i += 1
        }
    //don't shutdown the bot library
    } else {
        crate::src::game::g_syscalls::trap_BotLibShutdown();
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
