use ::libc;

pub mod q_shared_h {

    #[inline]

    pub unsafe extern "C" fn VectorCompare(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
    ) -> i32 {
        if *v1.offset(0) != *v2.offset(0)
            || *v1.offset(1) != *v2.offset(1)
            || *v1.offset(2) != *v2.offset(2)
        {
            return 0i32;
        }
        return 1;
    }
    #[inline]

    pub unsafe extern "C" fn VectorLength(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return crate::stdlib::sqrt(
            (*v.offset(0) * *v.offset(0)
                + *v.offset(1) * *v.offset(1)
                + *v.offset(2) * *v.offset(2)) as f64,
        ) as crate::src::qcommon::q_shared::vec_t;
    }
    #[inline]

    pub unsafe extern "C" fn VectorLengthSquared(
        mut v: *const crate::src::qcommon::q_shared::vec_t,
    ) -> crate::src::qcommon::q_shared::vec_t {
        return *v.offset(0) * *v.offset(0)
            + *v.offset(1) * *v.offset(1)
            + *v.offset(2) * *v.offset(2);
    }
    #[inline]

    pub unsafe extern "C" fn CrossProduct(
        mut v1: *const crate::src::qcommon::q_shared::vec_t,
        mut v2: *const crate::src::qcommon::q_shared::vec_t,
        mut cross: *mut crate::src::qcommon::q_shared::vec_t,
    ) {
        *cross.offset(0) = *v1.offset(1) * *v2.offset(2) - *v1.offset(2) * *v2.offset(1);
        *cross.offset(1) = *v1.offset(2) * *v2.offset(0) - *v1.offset(0) * *v2.offset(2);
        *cross.offset(2) = *v1.offset(0) * *v2.offset(1) - *v1.offset(1) * *v2.offset(0);
    }
    use crate::stdlib::sqrt;

    // __Q_SHARED_H
}

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub mod ctype_h {
    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
        return if __c >= -(128) && __c < 256 {
            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}

pub use crate::stdlib::__int32_t;

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
pub use crate::bg_public_h::EV_BULLET;
pub use crate::bg_public_h::EV_BULLET_HIT_FLESH;
pub use crate::bg_public_h::EV_BULLET_HIT_WALL;
pub use crate::bg_public_h::EV_CHANGE_WEAPON;
pub use crate::bg_public_h::EV_DEATH1;
pub use crate::bg_public_h::EV_DEATH2;
pub use crate::bg_public_h::EV_DEATH3;
pub use crate::bg_public_h::EV_DEBUG_LINE;
pub use crate::bg_public_h::EV_FALL_FAR;
pub use crate::bg_public_h::EV_FALL_MEDIUM;
pub use crate::bg_public_h::EV_FALL_SHORT;
pub use crate::bg_public_h::EV_FIRE_WEAPON;
pub use crate::bg_public_h::EV_FOOTSPLASH;
pub use crate::bg_public_h::EV_FOOTSTEP;
pub use crate::bg_public_h::EV_FOOTSTEP_METAL;
pub use crate::bg_public_h::EV_FOOTWADE;
pub use crate::bg_public_h::EV_GENERAL_SOUND;
pub use crate::bg_public_h::EV_GIB_PLAYER;
pub use crate::bg_public_h::EV_GLOBAL_ITEM_PICKUP;
pub use crate::bg_public_h::EV_GLOBAL_SOUND;
pub use crate::bg_public_h::EV_GLOBAL_TEAM_SOUND;
pub use crate::bg_public_h::EV_GRENADE_BOUNCE;
pub use crate::bg_public_h::EV_INVUL_IMPACT;
pub use crate::bg_public_h::EV_ITEM_PICKUP;
pub use crate::bg_public_h::EV_ITEM_POP;
pub use crate::bg_public_h::EV_ITEM_RESPAWN;
pub use crate::bg_public_h::EV_JUICED;
pub use crate::bg_public_h::EV_JUMP;
pub use crate::bg_public_h::EV_JUMP_PAD;
pub use crate::bg_public_h::EV_KAMIKAZE;
pub use crate::bg_public_h::EV_LIGHTNINGBOLT;
pub use crate::bg_public_h::EV_MISSILE_HIT;
pub use crate::bg_public_h::EV_MISSILE_MISS;
pub use crate::bg_public_h::EV_MISSILE_MISS_METAL;
pub use crate::bg_public_h::EV_NOAMMO;
pub use crate::bg_public_h::EV_NONE;
pub use crate::bg_public_h::EV_OBELISKEXPLODE;
pub use crate::bg_public_h::EV_OBELISKPAIN;
pub use crate::bg_public_h::EV_OBITUARY;
pub use crate::bg_public_h::EV_PAIN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_IN;
pub use crate::bg_public_h::EV_PLAYER_TELEPORT_OUT;
pub use crate::bg_public_h::EV_POWERUP_BATTLESUIT;
pub use crate::bg_public_h::EV_POWERUP_QUAD;
pub use crate::bg_public_h::EV_POWERUP_REGEN;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_STICK;
pub use crate::bg_public_h::EV_PROXIMITY_MINE_TRIGGER;
pub use crate::bg_public_h::EV_RAILTRAIL;
pub use crate::bg_public_h::EV_SCOREPLUM;
pub use crate::bg_public_h::EV_SHOTGUN;
pub use crate::bg_public_h::EV_STEP_12;
pub use crate::bg_public_h::EV_STEP_16;
pub use crate::bg_public_h::EV_STEP_4;
pub use crate::bg_public_h::EV_STEP_8;
pub use crate::bg_public_h::EV_STOPLOOPINGSOUND;
pub use crate::bg_public_h::EV_SWIM;
pub use crate::bg_public_h::EV_TAUNT;
pub use crate::bg_public_h::EV_TAUNT_FOLLOWME;
pub use crate::bg_public_h::EV_TAUNT_GETFLAG;
pub use crate::bg_public_h::EV_TAUNT_GUARDBASE;
pub use crate::bg_public_h::EV_TAUNT_NO;
pub use crate::bg_public_h::EV_TAUNT_PATROL;
pub use crate::bg_public_h::EV_TAUNT_YES;
pub use crate::bg_public_h::EV_USE_ITEM0;
pub use crate::bg_public_h::EV_USE_ITEM1;
pub use crate::bg_public_h::EV_USE_ITEM10;
pub use crate::bg_public_h::EV_USE_ITEM11;
pub use crate::bg_public_h::EV_USE_ITEM12;
pub use crate::bg_public_h::EV_USE_ITEM13;
pub use crate::bg_public_h::EV_USE_ITEM14;
pub use crate::bg_public_h::EV_USE_ITEM15;
pub use crate::bg_public_h::EV_USE_ITEM2;
pub use crate::bg_public_h::EV_USE_ITEM3;
pub use crate::bg_public_h::EV_USE_ITEM4;
pub use crate::bg_public_h::EV_USE_ITEM5;
pub use crate::bg_public_h::EV_USE_ITEM6;
pub use crate::bg_public_h::EV_USE_ITEM7;
pub use crate::bg_public_h::EV_USE_ITEM8;
pub use crate::bg_public_h::EV_USE_ITEM9;
pub use crate::bg_public_h::EV_WATER_CLEAR;
pub use crate::bg_public_h::EV_WATER_LEAVE;
pub use crate::bg_public_h::EV_WATER_TOUCH;
pub use crate::bg_public_h::EV_WATER_UNDER;
pub use crate::bg_public_h::GTS_BLUEOBELISK_ATTACKED;
pub use crate::bg_public_h::GTS_BLUETEAM_SCORED;
pub use crate::bg_public_h::GTS_BLUETEAM_TOOK_LEAD;
pub use crate::bg_public_h::GTS_BLUE_CAPTURE;
pub use crate::bg_public_h::GTS_BLUE_RETURN;
pub use crate::bg_public_h::GTS_BLUE_TAKEN;
pub use crate::bg_public_h::GTS_KAMIKAZE;
pub use crate::bg_public_h::GTS_REDOBELISK_ATTACKED;
pub use crate::bg_public_h::GTS_REDTEAM_SCORED;
pub use crate::bg_public_h::GTS_REDTEAM_TOOK_LEAD;
pub use crate::bg_public_h::GTS_RED_CAPTURE;
pub use crate::bg_public_h::GTS_RED_RETURN;
pub use crate::bg_public_h::GTS_RED_TAKEN;
pub use crate::bg_public_h::GTS_TEAMS_ARE_TIED;
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
pub use crate::bg_public_h::PERS_ASSIST_COUNT;
pub use crate::bg_public_h::PERS_ATTACKEE_ARMOR;
pub use crate::bg_public_h::PERS_ATTACKER;
pub use crate::bg_public_h::PERS_CAPTURES;
pub use crate::bg_public_h::PERS_DEFEND_COUNT;
pub use crate::bg_public_h::PERS_EXCELLENT_COUNT;
pub use crate::bg_public_h::PERS_GAUNTLET_FRAG_COUNT;
pub use crate::bg_public_h::PERS_HITS;
pub use crate::bg_public_h::PERS_IMPRESSIVE_COUNT;
pub use crate::bg_public_h::PERS_KILLED;
pub use crate::bg_public_h::PERS_PLAYEREVENTS;
pub use crate::bg_public_h::PERS_RANK;
pub use crate::bg_public_h::PERS_SCORE;
pub use crate::bg_public_h::PERS_SPAWN_COUNT;
pub use crate::bg_public_h::PERS_TEAM;
pub use crate::bg_public_h::PM_DEAD;
pub use crate::bg_public_h::PM_FREEZE;
pub use crate::bg_public_h::PM_INTERMISSION;
pub use crate::bg_public_h::PM_NOCLIP;
pub use crate::bg_public_h::PM_NORMAL;
pub use crate::bg_public_h::PM_SPECTATOR;
pub use crate::bg_public_h::PM_SPINTERMISSION;
pub use crate::bg_public_h::PW_AMMOREGEN;
pub use crate::bg_public_h::PW_BATTLESUIT;
pub use crate::bg_public_h::PW_BLUEFLAG;
pub use crate::bg_public_h::PW_DOUBLER;
pub use crate::bg_public_h::PW_FLIGHT;
pub use crate::bg_public_h::PW_GUARD;
pub use crate::bg_public_h::PW_HASTE;
pub use crate::bg_public_h::PW_INVIS;
pub use crate::bg_public_h::PW_INVULNERABILITY;
pub use crate::bg_public_h::PW_NEUTRALFLAG;
pub use crate::bg_public_h::PW_NONE;
pub use crate::bg_public_h::PW_NUM_POWERUPS;
pub use crate::bg_public_h::PW_QUAD;
pub use crate::bg_public_h::PW_REDFLAG;
pub use crate::bg_public_h::PW_REGEN;
pub use crate::bg_public_h::PW_SCOUT;
pub use crate::bg_public_h::STAT_ARMOR;
pub use crate::bg_public_h::STAT_CLIENTS_READY;
pub use crate::bg_public_h::STAT_DEAD_YAW;
pub use crate::bg_public_h::STAT_HEALTH;
pub use crate::bg_public_h::STAT_HOLDABLE_ITEM;
pub use crate::bg_public_h::STAT_MAX_HEALTH;
pub use crate::bg_public_h::STAT_WEAPONS;
pub use crate::bg_public_h::TEAMTASK_CAMP;
pub use crate::bg_public_h::TEAMTASK_DEFENSE;
pub use crate::bg_public_h::TEAMTASK_ESCORT;
pub use crate::bg_public_h::TEAMTASK_FOLLOW;
pub use crate::bg_public_h::TEAMTASK_NONE;
pub use crate::bg_public_h::TEAMTASK_OFFENSE;
pub use crate::bg_public_h::TEAMTASK_PATROL;
pub use crate::bg_public_h::TEAMTASK_RETRIEVE;
pub use crate::bg_public_h::TEAM_BLUE;
pub use crate::bg_public_h::TEAM_FREE;
pub use crate::bg_public_h::TEAM_NUM_TEAMS;
pub use crate::bg_public_h::TEAM_RED;
pub use crate::bg_public_h::TEAM_SPECTATOR;
pub use crate::bg_public_h::WEAPON_DROPPING;
pub use crate::bg_public_h::WEAPON_FIRING;
pub use crate::bg_public_h::WEAPON_RAISING;
pub use crate::bg_public_h::WEAPON_READY;
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
pub use crate::src::game::ai_dmq3::q_shared_h::CrossProduct;
pub use crate::src::game::ai_dmq3::q_shared_h::VectorCompare;
pub use crate::src::game::ai_dmq3::q_shared_h::VectorLength;
pub use crate::src::game::ai_dmq3::q_shared_h::VectorLengthSquared;
pub use crate::src::qcommon::q_math::vec3_origin;
pub use crate::src::qcommon::q_math::vectoangles;
pub use crate::src::qcommon::q_math::AngleMod;
pub use crate::src::qcommon::q_math::AngleVectors;
pub use crate::src::qcommon::q_math::VectorNormalize;
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
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_CleanStr;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;

pub use crate::be_aas_h::aas_altroutegoal_s;
pub use crate::be_aas_h::aas_altroutegoal_t;
pub use crate::be_aas_h::aas_areainfo_s;
pub use crate::be_aas_h::aas_areainfo_t;
pub use crate::be_aas_h::aas_clientmove_s;
pub use crate::be_aas_h::aas_clientmove_t;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_aas_h::aas_predictroute_s;
pub use crate::be_aas_h::aas_predictroute_t;
pub use crate::be_aas_h::aas_trace_s;
pub use crate::be_aas_h::aas_trace_t;
pub use crate::be_ai_chat_h::bot_consolemessage_s;
pub use crate::be_ai_chat_h::bot_consolemessage_t;
pub use crate::be_ai_chat_h::bot_match_s;
pub use crate::be_ai_chat_h::bot_match_t;
pub use crate::be_ai_chat_h::bot_matchvariable_s;
pub use crate::be_ai_chat_h::bot_matchvariable_t;
pub use crate::be_ai_goal_h::bot_goal_s;
pub use crate::be_ai_goal_h::bot_goal_t;
pub use crate::be_ai_move_h::bot_initmove_s;
pub use crate::be_ai_move_h::bot_initmove_t;
pub use crate::be_ai_move_h::bot_moveresult_s;
pub use crate::be_ai_move_h::bot_moveresult_t;
pub use crate::be_ai_weap_h::projectileinfo_s;
pub use crate::be_ai_weap_h::projectileinfo_t;
pub use crate::be_ai_weap_h::weaponinfo_s;
pub use crate::be_ai_weap_h::weaponinfo_t;
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
use crate::src::game::ai_chat::BotChatTime;
use crate::src::game::ai_chat::BotChat_EnterGame;
use crate::src::game::ai_chat::BotValidChatPosition;
use crate::src::game::ai_cmd::BotMatchMessage;
pub use crate::src::game::ai_dmq3::ctype_h::toupper;
pub use crate::src::game::ai_dmq3::stdlib_h::atoi;
pub use crate::src::game::ai_main::bot_activategoal_s;
pub use crate::src::game::ai_main::bot_activategoal_t;
pub use crate::src::game::ai_main::bot_state_s;
pub use crate::src::game::ai_main::bot_state_t;
pub use crate::src::game::ai_main::bot_waypoint_s;
pub use crate::src::game::ai_main::bot_waypoint_t;
pub use crate::src::game::ai_main::floattime;
pub use crate::src::game::ai_main::BotAI_GetClientState;
pub use crate::src::game::ai_main::BotAI_GetEntityState;
pub use crate::src::game::ai_main::BotAI_GetSnapshotEntity;
pub use crate::src::game::ai_main::BotAI_Print;
pub use crate::src::game::ai_main::BotAI_Trace;
pub use crate::src::game::ai_main::BotEntityInfo;
pub use crate::src::game::ai_main::BotTeamLeader;
pub use crate::src::game::ai_main::NumBots;
pub use crate::src::game::g_client::ClientUserinfoChanged;
pub use crate::src::game::g_main::g_entities;
pub use crate::src::game::g_main::level;
pub use crate::src::game::g_syscalls::trap_AAS_AreaInfo;
pub use crate::src::game::g_syscalls::trap_AAS_AreaReachability;
pub use crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea;
pub use crate::src::game::g_syscalls::trap_AAS_BBoxAreas;
pub use crate::src::game::g_syscalls::trap_AAS_EnableRoutingArea;
pub use crate::src::game::g_syscalls::trap_AAS_FloatForBSPEpairKey;
pub use crate::src::game::g_syscalls::trap_AAS_IntForBSPEpairKey;
pub use crate::src::game::g_syscalls::trap_AAS_NextBSPEntity;
pub use crate::src::game::g_syscalls::trap_AAS_PointAreaNum;
pub use crate::src::game::g_syscalls::trap_AAS_PointContents;
pub use crate::src::game::g_syscalls::trap_AAS_PredictClientMovement;
pub use crate::src::game::g_syscalls::trap_AAS_PredictRoute;
pub use crate::src::game::g_syscalls::trap_AAS_PresenceTypeBoundingBox;
pub use crate::src::game::g_syscalls::trap_AAS_TraceAreas;
pub use crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey;
pub use crate::src::game::g_syscalls::trap_AAS_VectorForBSPEpairKey;
pub use crate::src::game::g_syscalls::trap_BotAddAvoidSpot;
pub use crate::src::game::g_syscalls::trap_BotChooseBestFightWeapon;
pub use crate::src::game::g_syscalls::trap_BotDumpAvoidGoals;
pub use crate::src::game::g_syscalls::trap_BotDumpGoalStack;
pub use crate::src::game::g_syscalls::trap_BotFindMatch;
pub use crate::src::game::g_syscalls::trap_BotGetLevelItemGoal;
pub use crate::src::game::g_syscalls::trap_BotGetNextCampSpotGoal;
pub use crate::src::game::g_syscalls::trap_BotGetWeaponInfo;
pub use crate::src::game::g_syscalls::trap_BotInitMoveState;
pub use crate::src::game::g_syscalls::trap_BotLibVarSet;
pub use crate::src::game::g_syscalls::trap_BotMatchVariable;
pub use crate::src::game::g_syscalls::trap_BotMoveInDirection;
pub use crate::src::game::g_syscalls::trap_BotMoveToGoal;
pub use crate::src::game::g_syscalls::trap_BotNextConsoleMessage;
pub use crate::src::game::g_syscalls::trap_BotNumConsoleMessages;
pub use crate::src::game::g_syscalls::trap_BotPredictVisiblePosition;
pub use crate::src::game::g_syscalls::trap_BotRemoveConsoleMessage;
pub use crate::src::game::g_syscalls::trap_BotRemoveFromAvoidGoals;
pub use crate::src::game::g_syscalls::trap_BotReplaceSynonyms;
pub use crate::src::game::g_syscalls::trap_BotReplyChat;
pub use crate::src::game::g_syscalls::trap_BotSetChatGender;
pub use crate::src::game::g_syscalls::trap_BotSetChatName;
pub use crate::src::game::g_syscalls::trap_Characteristic_BFloat;
pub use crate::src::game::g_syscalls::trap_Characteristic_String;
pub use crate::src::game::g_syscalls::trap_Cvar_Register;
pub use crate::src::game::g_syscalls::trap_Cvar_Update;
pub use crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue;
pub use crate::src::game::g_syscalls::trap_EA_Action;
pub use crate::src::game::g_syscalls::trap_EA_Attack;
pub use crate::src::game::g_syscalls::trap_EA_Say;
pub use crate::src::game::g_syscalls::trap_EA_SelectWeapon;
pub use crate::src::game::g_syscalls::trap_EA_Use;
pub use crate::src::game::g_syscalls::trap_EA_View;
pub use crate::src::game::g_syscalls::trap_GetConfigstring;
pub use crate::src::game::g_syscalls::trap_GetServerinfo;
pub use crate::src::game::g_syscalls::trap_GetUserinfo;
pub use crate::src::game::g_syscalls::trap_PointContents;
pub use crate::src::game::g_syscalls::trap_SetUserinfo;
pub use crate::src::game::g_syscalls::trap_UnifyWhiteSpaces;
pub use crate::src::game::g_utils::G_ModelIndex;
pub use crate::stdlib::__ctype_toupper_loc;
use crate::stdlib::fabs;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;
use crate::stdlib::memset;
pub use crate::stdlib::rand;
use crate::stdlib::sqrt;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
use crate::stdlib::strncpy;
use crate::stdlib::strstr;
pub use crate::stdlib::strtol;

use crate::src::game::ai_dmnet::AIEnter_Seek_ActivateEntity;
use crate::src::game::ai_dmnet::AIEnter_Seek_LTG;
use crate::src::game::ai_dmnet::AIEnter_Stand;
use crate::src::game::ai_dmnet::AINode_Seek_LTG;
use crate::src::game::ai_dmnet::AINode_Seek_NBG;
use crate::src::game::ai_dmnet::AINode_Stand;
use crate::src::game::ai_dmnet::BotDumpNodeSwitches;
use crate::src::game::ai_dmnet::BotResetNodeSwitches;
use crate::src::game::ai_team::BotTeamAI;
use crate::src::game::ai_team::BotVoiceChat;
extern "C" {
    #[no_mangle]
    pub static mut bot_developer: crate::src::qcommon::q_shared::vmCvar_t;
}
//
#[no_mangle]

pub static mut botai_waypoints: [crate::src::game::ai_main::bot_waypoint_t; 128] =
    [crate::src::game::ai_main::bot_waypoint_t {
        inuse: 0,
        name: [0; 32],
        goal: crate::be_ai_goal_h::bot_goal_t {
            origin: [0.; 3],
            areanum: 0,
            mins: [0.; 3],
            maxs: [0.; 3],
            entitynum: 0,
            number: 0,
            flags: 0,
            iteminfo: 0,
        },
        next: 0 as *mut crate::src::game::ai_main::bot_waypoint_s,
        prev: 0 as *mut crate::src::game::ai_main::bot_waypoint_s,
    }; 128];
#[no_mangle]

pub static mut botai_freewaypoints: *mut crate::src::game::ai_main::bot_waypoint_t =
    0 as *mut crate::src::game::ai_main::bot_waypoint_t;
//NOTE: not using a cvars which can be updated because the game should be reloaded anyway
#[no_mangle]

pub static mut gametype: i32 = 0;
//game type
#[no_mangle]

pub static mut bot_grapple: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_rocketjump: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_fastchat: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_nochat: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_testrchat: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_challenge: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut bot_predictobstacles: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut g_spSkill: crate::src::qcommon::q_shared::vmCvar_t =
    crate::src::qcommon::q_shared::vmCvar_t {
        handle: 0,
        modificationCount: 0,
        value: 0.,
        integer: 0,
        string: [0; 256],
    };
#[no_mangle]

pub static mut lastteleport_origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
//last teleport event origin
#[no_mangle]

pub static mut lastteleport_time: f32 = 0.;
//last teleport event time
#[no_mangle]

pub static mut max_bspmodelindex: i32 = 0;
//maximum BSP model index
//CTF flag goals
#[no_mangle]

pub static mut ctf_redflag: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
    origin: [0.; 3],
    areanum: 0,
    mins: [0.; 3],
    maxs: [0.; 3],
    entitynum: 0,
    number: 0,
    flags: 0,
    iteminfo: 0,
};
#[no_mangle]

pub static mut ctf_blueflag: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
    origin: [0.; 3],
    areanum: 0,
    mins: [0.; 3],
    maxs: [0.; 3],
    entitynum: 0,
    number: 0,
    flags: 0,
    iteminfo: 0,
};
#[no_mangle]

pub static mut altroutegoals_setup: i32 = 0;
#[no_mangle]

pub static mut red_altroutegoals: [crate::be_aas_h::aas_altroutegoal_t; 32] =
    [crate::be_aas_h::aas_altroutegoal_t {
        origin: [0.; 3],
        areanum: 0,
        starttraveltime: 0,
        goaltraveltime: 0,
        extratraveltime: 0,
    }; 32];
#[no_mangle]

pub static mut red_numaltroutegoals: i32 = 0;
#[no_mangle]

pub static mut blue_altroutegoals: [crate::be_aas_h::aas_altroutegoal_t; 32] =
    [crate::be_aas_h::aas_altroutegoal_t {
        origin: [0.; 3],
        areanum: 0,
        starttraveltime: 0,
        goaltraveltime: 0,
        extratraveltime: 0,
    }; 32];
#[no_mangle]

pub static mut blue_numaltroutegoals: i32 = 0;
// set a user info key/value pair
/*
==================
BotSetUserInfo
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetUserInfo(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut key: *mut i8,
    mut value: *mut i8,
) {
    let mut userinfo: [i8; 1024] = [0; 1024];
    crate::src::game::g_syscalls::trap_GetUserinfo(
        (*bs).client,
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::src::qcommon::q_shared::Info_SetValueForKey(userinfo.as_mut_ptr(), key, value);
    crate::src::game::g_syscalls::trap_SetUserinfo((*bs).client, userinfo.as_mut_ptr());
    crate::src::game::g_client::ClientUserinfoChanged((*bs).client);
}
//returns the flag the bot is carrying (CTFFLAG_?)
/*
==================
BotCTFCarryingFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFCarryingFlag(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    if gametype != crate::bg_public_h::GT_CTF as i32 {
        return 0i32;
    }
    if (*bs).inventory[45] > 0 {
        return 1i32;
    } else {
        if (*bs).inventory[46] > 0 {
            return 2i32;
        }
    }
    return 0;
}
//returns the team the bot is in
/*
==================
BotTeam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeam(mut bs: *mut crate::src::game::ai_main::bot_state_t) -> i32 {
    if (*bs).client < 0 || (*bs).client >= 64 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if (*crate::src::game::g_main::level
        .clients
        .offset((*bs).client as isize))
    .sess
    .sessionTeam
        == crate::bg_public_h::TEAM_RED
    {
        return crate::bg_public_h::TEAM_RED as i32;
    } else {
        if (*crate::src::game::g_main::level
            .clients
            .offset((*bs).client as isize))
        .sess
        .sessionTeam
            == crate::bg_public_h::TEAM_BLUE
        {
            return crate::bg_public_h::TEAM_BLUE as i32;
        }
    }
    return crate::bg_public_h::TEAM_FREE as i32;
}
//returns the opposite team of the bot
/*
==================
BotOppositeTeam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotOppositeTeam(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    match BotTeam(bs) {
        1 => return crate::bg_public_h::TEAM_BLUE as i32,
        2 => return crate::bg_public_h::TEAM_RED as i32,
        _ => return crate::bg_public_h::TEAM_FREE as i32,
    };
}
/*
==================
BotEnemyFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotEnemyFlag(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> *mut crate::be_ai_goal_h::bot_goal_t {
    if BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
        return &mut ctf_blueflag;
    } else {
        return &mut ctf_redflag;
    };
}
/*
==================
BotTeamFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamFlag(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> *mut crate::be_ai_goal_h::bot_goal_t {
    if BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
        return &mut ctf_redflag;
    } else {
        return &mut ctf_blueflag;
    };
}
//returns true if the entity is dead
/*
==================
EntityIsDead
==================
*/
#[no_mangle]

pub unsafe extern "C" fn EntityIsDead(
    mut entinfo: *mut crate::be_aas_h::aas_entityinfo_t,
) -> crate::src::qcommon::q_shared::qboolean {
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
    if (*entinfo).number >= 0 && (*entinfo).number < 64 {
        //retrieve the current client state
        if crate::src::game::ai_main::BotAI_GetClientState((*entinfo).number, &mut ps) == 0 {
            return crate::src::qcommon::q_shared::qfalse;
        }
        if ps.pm_type != crate::bg_public_h::PM_NORMAL as i32 {
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
==================
EntityCarriesFlag
==================
*/
#[no_mangle]

pub unsafe extern "C" fn EntityCarriesFlag(
    mut entinfo: *mut crate::be_aas_h::aas_entityinfo_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*entinfo).powerups & (1) << crate::bg_public_h::PW_REDFLAG as i32 != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    if (*entinfo).powerups & (1) << crate::bg_public_h::PW_BLUEFLAG as i32 != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
//returns true if the entity is invisible
/*
==================
EntityIsInvisible
==================
*/
#[no_mangle]

pub unsafe extern "C" fn EntityIsInvisible(
    mut entinfo: *mut crate::be_aas_h::aas_entityinfo_t,
) -> crate::src::qcommon::q_shared::qboolean {
    // the flag is always visible
    if EntityCarriesFlag(entinfo) as u64 != 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if (*entinfo).powerups & (1) << crate::bg_public_h::PW_INVIS as i32 != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
//returns true if the entity is shooting
/*
==================
EntityIsShooting
==================
*/
#[no_mangle]

pub unsafe extern "C" fn EntityIsShooting(
    mut entinfo: *mut crate::be_aas_h::aas_entityinfo_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*entinfo).flags & 0x100 != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
==================
EntityIsChatting
==================
*/
#[no_mangle]

pub unsafe extern "C" fn EntityIsChatting(
    mut entinfo: *mut crate::be_aas_h::aas_entityinfo_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*entinfo).flags & 0x1000 != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
==================
EntityHasQuad
==================
*/
#[no_mangle]

pub unsafe extern "C" fn EntityHasQuad(
    mut entinfo: *mut crate::be_aas_h::aas_entityinfo_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*entinfo).powerups & (1) << crate::bg_public_h::PW_QUAD as i32 != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
//remember the last ordered task
/*
==================
BotRememberLastOrderedTask
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotRememberLastOrderedTask(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    if (*bs).ordered == 0 {
        return;
    }
    (*bs).lastgoal_decisionmaker = (*bs).decisionmaker;
    (*bs).lastgoal_ltgtype = (*bs).ltgtype;
    crate::stdlib::memcpy(
        &mut (*bs).lastgoal_teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
        ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>(),
    );
    (*bs).lastgoal_teammate = (*bs).teammate;
}
// set the team status (offense, defense etc.)
/*
==================
BotSetTeamStatus
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetTeamStatus(mut bs: *mut crate::src::game::ai_main::bot_state_t) {}
// set last ordered task
/*
==================
BotSetLastOrderedTask
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetLastOrderedTask(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    if gametype == crate::bg_public_h::GT_CTF as i32 {
        // don't go back to returning the flag if it's at the base
        if (*bs).lastgoal_ltgtype == 6 {
            if BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
                if (*bs).redflagstatus == 0 {
                    (*bs).lastgoal_ltgtype = 0
                }
            } else if (*bs).blueflagstatus == 0 {
                (*bs).lastgoal_ltgtype = 0
            }
        }
    }
    if (*bs).lastgoal_ltgtype != 0 {
        (*bs).decisionmaker = (*bs).lastgoal_decisionmaker;
        (*bs).ordered = crate::src::qcommon::q_shared::qtrue as i32;
        (*bs).ltgtype = (*bs).lastgoal_ltgtype;
        crate::stdlib::memcpy(
            &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
            &mut (*bs).lastgoal_teamgoal as *mut crate::be_ai_goal_h::bot_goal_t
                as *const libc::c_void,
            ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>(),
        );
        (*bs).teammate = (*bs).lastgoal_teammate;
        (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 300f32;
        BotSetTeamStatus(bs);
        //
        if gametype == crate::bg_public_h::GT_CTF as i32 {
            if (*bs).ltgtype == 4 {
                let mut tb: *mut crate::be_ai_goal_h::bot_goal_t =
                    0 as *mut crate::be_ai_goal_h::bot_goal_t;
                let mut eb: *mut crate::be_ai_goal_h::bot_goal_t =
                    0 as *mut crate::be_ai_goal_h::bot_goal_t;
                let mut tt: i32 = 0;
                let mut et: i32 = 0;
                tb = BotTeamFlag(bs);
                eb = BotEnemyFlag(bs);
                tt = crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
                    (*bs).areanum,
                    (*bs).origin.as_mut_ptr(),
                    (*tb).areanum,
                    0x2 | 0x4
                        | 0x8
                        | 0x10
                        | 0x20
                        | 0x80
                        | 0x100
                        | 0x200
                        | 0x400
                        | 0x800
                        | 0x80000
                        | 0x100000
                        | 0x40000
                        | 0x1000000,
                );
                et = crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
                    (*bs).areanum,
                    (*bs).origin.as_mut_ptr(),
                    (*eb).areanum,
                    0x2 | 0x4
                        | 0x8
                        | 0x10
                        | 0x20
                        | 0x80
                        | 0x100
                        | 0x200
                        | 0x400
                        | 0x800
                        | 0x80000
                        | 0x100000
                        | 0x40000
                        | 0x1000000,
                );
                // if the travel time towards the enemy base is larger than towards our base
                if et > tt {
                    //get an alternative route goal towards the enemy base
                    BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
                }
            }
        }
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
==================
BotRefuseOrder
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotRefuseOrder(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    if (*bs).ordered == 0 {
        return;
    }
    // if the bot was ordered to do something
    if (*bs).order_time != 0. && (*bs).order_time > crate::src::game::ai_main::floattime - 10f32 {
        crate::src::game::g_syscalls::trap_EA_Action((*bs).client, 0x200000);
        crate::src::game::ai_team::BotVoiceChat(
            bs,
            (*bs).decisionmaker,
            b"no\x00" as *const u8 as *mut i8,
        );
        (*bs).order_time = 0f32
    };
}
//set ctf goals (defend base, get enemy flag) during seek
/*
==================
BotCTFSeekGoals
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFSeekGoals(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut rnd: f32 = 0.;
    let mut l1: f32 = 0.;
    let mut l2: f32 = 0.;
    let mut flagstatus: i32 = 0;
    let mut c: i32 = 0;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    //when carrying a flag in ctf the bot should rush to the base
    if BotCTFCarryingFlag(bs) != 0 {
        //if not already rushing to the base
        if (*bs).ltgtype != 5 {
            BotRefuseOrder(bs);
            (*bs).ltgtype = 5;
            (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 120f32;
            (*bs).rushbaseaway_time = 0f32;
            (*bs).decisionmaker = (*bs).client;
            (*bs).ordered = crate::src::qcommon::q_shared::qfalse as i32;
            //
            match BotTeam(bs) {
                1 => {
                    dir[0] = (*bs).origin[0] - ctf_blueflag.origin[0];
                    dir[1] = (*bs).origin[1] - ctf_blueflag.origin[1];
                    dir[2] = (*bs).origin[2] - ctf_blueflag.origin[2]
                }
                2 => {
                    dir[0] = (*bs).origin[0] - ctf_redflag.origin[0];
                    dir[1] = (*bs).origin[1] - ctf_redflag.origin[1];
                    dir[2] = (*bs).origin[2] - ctf_redflag.origin[2]
                }
                _ => {
                    dir[0] = 999f32;
                    dir[1] = 999f32;
                    dir[2] = 999f32
                }
            }
            // if the bot picked up the flag very close to the enemy base
            if VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
                < 128f32
            {
                // get an alternative route goal through the enemy base
                BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
            } else {
                // don't use any alt route goal, just get the hell out of the base
                (*bs).altroutegoal.areanum = 0
            }
            BotSetUserInfo(
                bs,
                b"teamtask\x00" as *const u8 as *mut i8,
                crate::src::qcommon::q_shared::va(
                    b"%d\x00" as *const u8 as *mut i8,
                    crate::bg_public_h::TEAMTASK_OFFENSE as i32,
                ),
            );
            crate::src::game::ai_team::BotVoiceChat(
                bs,
                -(1i32),
                b"ihaveflag\x00" as *const u8 as *mut i8,
            );
        } else if (*bs).rushbaseaway_time > crate::src::game::ai_main::floattime {
            if BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
                flagstatus = (*bs).redflagstatus
            } else {
                flagstatus = (*bs).blueflagstatus
            }
            //if the flag is back
            if flagstatus == 0 {
                (*bs).rushbaseaway_time = 0f32
            }
        }
        return;
    }
    // if the bot decided to follow someone
    if (*bs).ltgtype == 2 && (*bs).ordered == 0 {
        // if the team mate being accompanied no longer carries the flag
        crate::src::game::ai_main::BotEntityInfo((*bs).teammate, &mut entinfo);
        if EntityCarriesFlag(&mut entinfo) as u64 == 0 {
            (*bs).ltgtype = 0
        }
    }
    //
    if BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
        flagstatus = (*bs).redflagstatus * 2 + (*bs).blueflagstatus
    } else {
        flagstatus = (*bs).blueflagstatus * 2 + (*bs).redflagstatus
    }
    //if our team has the enemy flag and our flag is at the base
    if flagstatus == 1 {
        //
        if ((*bs).owndecision_time as f32) < crate::src::game::ai_main::floattime {
            //if Not defending the base already
            if !((*bs).ltgtype == 3
                && ((*bs).teamgoal.number == ctf_redflag.number
                    || (*bs).teamgoal.number == ctf_blueflag.number))
            {
                //if there is a visible team mate flag carrier
                c = BotTeamFlagCarrierVisible(bs);
                if c >= 0 && ((*bs).ltgtype != 2 || (*bs).teammate != c) {
                    //
                    BotRefuseOrder(bs);
                    //follow the flag carrier
                    (*bs).decisionmaker = (*bs).client;
                    (*bs).ordered = crate::src::qcommon::q_shared::qfalse as i32;
                    //the team mate
                    (*bs).teammate = c;
                    //last time the team mate was visible
                    (*bs).teammatevisible_time = crate::src::game::ai_main::floattime;
                    //no message
                    (*bs).teammessage_time = 0f32;
                    //no arrive message
                    (*bs).arrive_time = 1f32;
                    //
                    crate::src::game::ai_team::BotVoiceChat(
                        bs,
                        (*bs).teammate,
                        b"onfollow\x00" as *const u8 as *mut i8,
                    );
                    //get the team goal time
                    (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32; //3.5 meter
                    (*bs).ltgtype = 2;
                    (*bs).formation_dist = (3.5f64 * 32f64) as f32;
                    BotSetTeamStatus(bs);
                    (*bs).owndecision_time = (crate::src::game::ai_main::floattime + 5f32) as i32
                }
            }
        }
        return;
    } else {
        //if the enemy has our flag
        if flagstatus == 2 {
            //
            if ((*bs).owndecision_time as f32) < crate::src::game::ai_main::floattime {
                //if enemy flag carrier is visible
                c = BotEnemyFlagCarrierVisible(bs);
                (c) >= 0;
                //if not already doing something important
                if (*bs).ltgtype != 4
                    && (*bs).ltgtype != 6
                    && (*bs).ltgtype != 1
                    && (*bs).ltgtype != 2
                    && (*bs).ltgtype != 8
                    && (*bs).ltgtype != 9
                    && (*bs).ltgtype != 10
                {
                    BotRefuseOrder(bs);
                    (*bs).decisionmaker = (*bs).client;
                    (*bs).ordered = crate::src::qcommon::q_shared::qfalse as i32;
                    //
                    if (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64) < 0.5 {
                        //go for the enemy flag
                        (*bs).ltgtype = 4
                    } else {
                        (*bs).ltgtype = 6
                    }
                    //no team message
                    (*bs).teammessage_time = 0f32;
                    //set the time the bot will stop getting the flag
                    (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32;
                    //get an alternative route goal towards the enemy base
                    BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
                    //
                    BotSetTeamStatus(bs);
                    (*bs).owndecision_time = (crate::src::game::ai_main::floattime + 5f32) as i32
                }
            }
            return;
        } else {
            //if both flags Not at their bases
            if flagstatus == 3 {
                //
                if ((*bs).owndecision_time as f32) < crate::src::game::ai_main::floattime {
                    // if not trying to return the flag and not following the team flag carrier
                    if (*bs).ltgtype != 6 && (*bs).ltgtype != 2 {
                        //
                        c = BotTeamFlagCarrierVisible(bs);
                        // if there is a visible team mate flag carrier
                        if c >= 0 {
                            BotRefuseOrder(bs);
                            //follow the flag carrier
                            (*bs).decisionmaker = (*bs).client;
                            (*bs).ordered = crate::src::qcommon::q_shared::qfalse as i32;
                            //the team mate
                            (*bs).teammate = c;
                            //last time the team mate was visible
                            (*bs).teammatevisible_time = crate::src::game::ai_main::floattime;
                            //no message
                            (*bs).teammessage_time = 0f32;
                            //no arrive message
                            (*bs).arrive_time = 1f32;
                            //
                            crate::src::game::ai_team::BotVoiceChat(
                                bs,
                                (*bs).teammate,
                                b"onfollow\x00" as *const u8 as *mut i8,
                            );
                            //get the team goal time
                            (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32; //3.5 meter
                            (*bs).ltgtype = 2;
                            (*bs).formation_dist = (3.5f64 * 32f64) as f32;
                            //
                            BotSetTeamStatus(bs);
                            (*bs).owndecision_time =
                                (crate::src::game::ai_main::floattime + 5f32) as i32
                        } else {
                            BotRefuseOrder(bs);
                            (*bs).decisionmaker = (*bs).client;
                            (*bs).ordered = crate::src::qcommon::q_shared::qfalse as i32;
                            //get the enemy flag
                            (*bs).teammessage_time = crate::src::game::ai_main::floattime
                                + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
                            //get the flag
                            (*bs).ltgtype = 6;
                            //set the time the bot will stop getting the flag
                            (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 180f32;
                            //get an alternative route goal towards the enemy base
                            BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
                            //
                            BotSetTeamStatus(bs);
                            (*bs).owndecision_time =
                                (crate::src::game::ai_main::floattime + 5f32) as i32
                        }
                    }
                }
                return;
            }
        }
    }
    // don't just do something wait for the bot team leader to give orders
    if crate::src::game::ai_main::BotTeamLeader(bs) != 0 {
        return;
    }
    // if the bot is ordered to do something
    if (*bs).lastgoal_ltgtype != 0 {
        (*bs).teamgoal_time += 60f32
    }
    // if the bot decided to do something on its own and has a last ordered goal
    if (*bs).ordered == 0 && (*bs).lastgoal_ltgtype != 0 {
        (*bs).ltgtype = 0
    }
    //if already a CTF or team goal
    if (*bs).ltgtype == 1
        || (*bs).ltgtype == 2
        || (*bs).ltgtype == 3
        || (*bs).ltgtype == 4
        || (*bs).ltgtype == 5
        || (*bs).ltgtype == 6
        || (*bs).ltgtype == 8
        || (*bs).ltgtype == 9
        || (*bs).ltgtype == 10
        || (*bs).ltgtype == 14
        || (*bs).ltgtype == 15
    {
        return;
    }
    //
    if BotSetLastOrderedTask(bs) != 0 {
        return;
    }
    //
    if (*bs).owndecision_time as f32 > crate::src::game::ai_main::floattime {
        return;
    }
    //if the bot is roaming
    if (*bs).ctfroam_time > crate::src::game::ai_main::floattime {
        return;
    }
    //if the bot has enough aggression to decide what to do
    if BotAggression(bs) < 50f32 {
        return;
    }
    //set the time to send a message to the team mates
    (*bs).teammessage_time = crate::src::game::ai_main::floattime
        + 2f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
    //
    if (*bs).teamtaskpreference & (2 | 1) != 0 {
        if (*bs).teamtaskpreference & 2 != 0 {
            l1 = 0.7
        } else {
            l1 = 0.2
        }
        l2 = 0.9
    } else {
        l1 = 0.4;
        l2 = 0.7
    }
    //get the flag or defend the base
    rnd = (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32;
    if rnd < l1 && ctf_redflag.areanum != 0 && ctf_blueflag.areanum != 0 {
        (*bs).decisionmaker = (*bs).client;
        (*bs).ordered = crate::src::qcommon::q_shared::qfalse as i32;
        (*bs).ltgtype = 4;
        //set the time the bot will stop getting the flag
        (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32;
        //get an alternative route goal towards the enemy base
        BotGetAlternateRouteGoal(bs, BotOppositeTeam(bs));
        BotSetTeamStatus(bs);
    } else if rnd < l2 && ctf_redflag.areanum != 0 && ctf_blueflag.areanum != 0 {
        (*bs).decisionmaker = (*bs).client;
        (*bs).ordered = crate::src::qcommon::q_shared::qfalse as i32;
        //
        if BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
            crate::stdlib::memcpy(
                &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
                &mut ctf_redflag as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
                ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>(),
            );
        } else {
            crate::stdlib::memcpy(
                &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
                &mut ctf_blueflag as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
                ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>(),
            );
        }
        //set the ltg type
        (*bs).ltgtype = 3;
        //set the time the bot stops defending the base
        (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 600f32;
        (*bs).defendaway_time = 0f32;
        BotSetTeamStatus(bs);
    } else {
        (*bs).ltgtype = 0;
        //set the time the bot will stop roaming
        (*bs).ctfroam_time = crate::src::game::ai_main::floattime + 60f32;
        BotSetTeamStatus(bs);
    }
    (*bs).owndecision_time = (crate::src::game::ai_main::floattime + 5f32) as i32;
    //DEBUG
}
//set ctf goals (defend base, get enemy flag) during retreat
/*
==================
BotCTFRetreatGoals
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCTFRetreatGoals(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    //when carrying a flag in ctf the bot should rush to the base
    if BotCTFCarryingFlag(bs) != 0 {
        //if not already rushing to the base
        if (*bs).ltgtype != 5 {
            BotRefuseOrder(bs);
            (*bs).ltgtype = 5;
            (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 120f32;
            (*bs).rushbaseaway_time = 0f32;
            (*bs).decisionmaker = (*bs).client;
            (*bs).ordered = crate::src::qcommon::q_shared::qfalse as i32;
            BotSetTeamStatus(bs);
        }
    };
}
// selection of goals for teamplay
/*
==================
BotTeamGoals
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamGoals(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut retreat: i32,
) {
    if retreat != 0 {
        if gametype == crate::bg_public_h::GT_CTF as i32 {
            BotCTFRetreatGoals(bs);
        }
    } else if gametype == crate::bg_public_h::GT_CTF as i32 {
        //decide what to do in CTF mode
        BotCTFSeekGoals(bs);
    }
    // reset the order time which is used to see if
    // we decided to refuse an order
    (*bs).order_time = 0f32;
}
//
/*
==================
BotPointAreaNum
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotPointAreaNum(
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut areanum: i32 = 0;
    let mut numareas: i32 = 0;
    let mut areas: [i32; 10] = [0; 10];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    areanum = crate::src::game::g_syscalls::trap_AAS_PointAreaNum(origin);
    if areanum != 0 {
        return areanum;
    }
    end[0] = *origin.offset(0);
    end[1] = *origin.offset(1);
    end[2] = *origin.offset(2);
    end[2] += 10f32;
    numareas = crate::src::game::g_syscalls::trap_AAS_TraceAreas(
        origin,
        end.as_mut_ptr(),
        areas.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec3_t,
        10,
    );
    if numareas > 0 {
        return areas[0usize];
    }
    return 0;
}
//returns the name of the client
/*
==================
ClientName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientName(mut client: i32, mut name: *mut i8, mut size: i32) -> *mut i8 {
    let mut buf: [i8; 1024] = [0; 1024];
    if client < 0 || client >= 64 {
        crate::src::game::ai_main::BotAI_Print(
            3,
            b"ClientName: client out of range\n\x00" as *const u8 as *mut i8,
        );
        return b"[client out of range]\x00" as *const u8 as *mut i8;
    }
    crate::src::game::g_syscalls::trap_GetConfigstring(
        32 + 256 + 256 + client,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::stdlib::strncpy(
        name,
        crate::src::qcommon::q_shared::Info_ValueForKey(
            buf.as_mut_ptr(),
            b"n\x00" as *const u8 as *const i8,
        ),
        (size - 1i32) as usize,
    );
    *name.offset((size - 1i32) as isize) = '\u{0}' as i8;
    crate::src::qcommon::q_shared::Q_CleanStr(name);
    return name;
}
//returns the skin used by the client
/*
==================
ClientSkin
==================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientSkin(mut client: i32, mut skin: *mut i8, mut size: i32) -> *mut i8 {
    let mut buf: [i8; 1024] = [0; 1024];
    if client < 0 || client >= 64 {
        crate::src::game::ai_main::BotAI_Print(
            3,
            b"ClientSkin: client out of range\n\x00" as *const u8 as *mut i8,
        );
        return b"[client out of range]\x00" as *const u8 as *mut i8;
    }
    crate::src::game::g_syscalls::trap_GetConfigstring(
        32 + 256 + 256 + client,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::stdlib::strncpy(
        skin,
        crate::src::qcommon::q_shared::Info_ValueForKey(
            buf.as_mut_ptr(),
            b"model\x00" as *const u8 as *const i8,
        ),
        (size - 1i32) as usize,
    );
    *skin.offset((size - 1i32) as isize) = '\u{0}' as i8;
    return skin;
}
//returns the number of the client with the given name
/*
==================
ClientFromName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientFromName(mut name: *mut i8) -> i32 {
    let mut i: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];

    for i in 0..crate::src::game::g_main::level.maxclients {
        crate::src::game::g_syscalls::trap_GetConfigstring(
            32 + 256 + 256 + i,
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );

        crate::src::qcommon::q_shared::Q_CleanStr(buf.as_mut_ptr());

        if crate::src::qcommon::q_shared::Q_stricmp(
            crate::src::qcommon::q_shared::Info_ValueForKey(
                buf.as_mut_ptr(),
                b"n\x00" as *const u8 as *const i8,
            ),
            name,
        ) == 0
        {
            return i;
        }
    }
    return -(1);
}
/*
==================
ClientOnSameTeamFromName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn ClientOnSameTeamFromName(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut name: *mut i8,
) -> i32 {
    let mut i: i32 = 0;
    let mut buf: [i8; 1024] = [0; 1024];

    for i in 0..crate::src::game::g_main::level.maxclients {
        if !(BotSameTeam(bs, i) == 0) {
            crate::src::game::g_syscalls::trap_GetConfigstring(
                32 + 256 + 256 + i,
                buf.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 1024]>() as i32,
            );
            crate::src::qcommon::q_shared::Q_CleanStr(buf.as_mut_ptr());
            if crate::src::qcommon::q_shared::Q_stricmp(
                crate::src::qcommon::q_shared::Info_ValueForKey(
                    buf.as_mut_ptr(),
                    b"n\x00" as *const u8 as *const i8,
                ),
                name,
            ) == 0
            {
                return i;
            }
        }
    }
    return -(1);
}
//strstr but case insensitive
/*
==================
stristr
==================
*/
#[no_mangle]

pub unsafe extern "C" fn stristr(mut str: *mut i8, mut charset: *mut i8) -> *mut i8 {
    let mut i: i32 = 0;
    while *str != 0 {
        i = 0;
        while *charset.offset(i as isize) as i32 != 0 && *str.offset(i as isize) as i32 != 0 {
            if ({
                let mut __res: i32 = 0;
                if ::std::mem::size_of::<i8>() > 1 {
                    if 0 != 0 {
                        let mut __c: i32 = *charset.offset(i as isize) as i32;
                        __res = (if __c < -(128) || __c > 255 {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        })
                    } else {
                        __res = toupper(*charset.offset(i as isize) as i32)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(*charset.offset(i as isize) as i32 as isize)
                }
                __res
            }) != ({
                let mut __res: i32 = 0;
                if ::std::mem::size_of::<i8>() > 1 {
                    if 0 != 0 {
                        let mut __c: i32 = *str.offset(i as isize) as i32;
                        __res = (if __c < -(128) || __c > 255 {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                        })
                    } else {
                        __res = toupper(*str.offset(i as isize) as i32)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_toupper_loc())
                        .offset(*str.offset(i as isize) as i32 as isize)
                }
                __res
            }) {
                break;
            }
            i += 1
        }
        if *charset.offset(i as isize) == 0 {
            return str;
        }
        str = str.offset(1)
    }
    return 0 as *mut i8;
}
//returns a simplified client name
/*
==================
EasyClientName
==================
*/
#[no_mangle]

pub unsafe extern "C" fn EasyClientName(
    mut client: i32,
    mut buf: *mut i8,
    mut size: i32,
) -> *mut i8 {
    let mut i: i32 = 0;
    let mut str1: *mut i8 = 0 as *mut i8;
    let mut str2: *mut i8 = 0 as *mut i8;
    let mut ptr: *mut i8 = 0 as *mut i8;
    let mut c: i8 = 0;
    let mut name: [i8; 128] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ];
    ClientName(
        client,
        name.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 128]>() as i32,
    );
    i = 0;
    while name[i as usize] != 0 {
        name[i as usize] = (name[i as usize] as i32 & 127) as i8;
        i += 1
    }
    //remove all spaces
    ptr = crate::stdlib::strstr(name.as_mut_ptr(), b" \x00" as *const u8 as *const i8);
    while !ptr.is_null() {
        crate::stdlib::memmove(
            ptr as *mut libc::c_void,
            ptr.offset(1) as *const libc::c_void,
            crate::stdlib::strlen(ptr.offset(1)).wrapping_add(1usize),
        );
        ptr = crate::stdlib::strstr(name.as_mut_ptr(), b" \x00" as *const u8 as *const i8)
    }
    //check for [x] and ]x[ clan names
    str1 = crate::stdlib::strstr(name.as_mut_ptr(), b"[\x00" as *const u8 as *const i8);
    str2 = crate::stdlib::strstr(name.as_mut_ptr(), b"]\x00" as *const u8 as *const i8);
    if !str1.is_null() && !str2.is_null() {
        if str2 > str1 {
            crate::stdlib::memmove(
                str1 as *mut libc::c_void,
                str2.offset(1isize) as *const libc::c_void,
                crate::stdlib::strlen(str2.offset(1isize)).wrapping_add(1usize),
            );
        } else {
            crate::stdlib::memmove(
                str2 as *mut libc::c_void,
                str1.offset(1isize) as *const libc::c_void,
                crate::stdlib::strlen(str1.offset(1isize)).wrapping_add(1usize),
            );
        }
    }
    //remove Mr prefix
    if (name[0] as i32 == 'm' as i32 || name[0] as i32 == 'M' as i32)
        && (name[1] as i32 == 'r' as i32 || name[1] as i32 == 'R' as i32)
    {
        crate::stdlib::memmove(
            name.as_mut_ptr() as *mut libc::c_void,
            name.as_mut_ptr().offset(2isize) as *const libc::c_void,
            crate::stdlib::strlen(name.as_mut_ptr().offset(2isize)).wrapping_add(1usize),
        );
    }
    //only allow lower case alphabet characters
    ptr = name.as_mut_ptr();
    while *ptr != 0 {
        c = *ptr;
        if c as i32 >= 'a' as i32 && c as i32 <= 'z' as i32
            || c as i32 >= '0' as i32 && c as i32 <= '9' as i32
            || c as i32 == '_' as i32
        {
            ptr = ptr.offset(1)
        } else if c as i32 >= 'A' as i32 && c as i32 <= 'Z' as i32 {
            *ptr = (*ptr as i32 + ('a' as i32 - 'A' as i32)) as i8;
            ptr = ptr.offset(1)
        } else {
            crate::stdlib::memmove(
                ptr as *mut libc::c_void,
                ptr.offset(1isize) as *const libc::c_void,
                crate::stdlib::strlen(ptr.offset(1isize)).wrapping_add(1usize),
            );
        }
    }
    crate::stdlib::strncpy(buf, name.as_mut_ptr(), (size - 1i32) as usize);
    *buf.offset((size - 1i32) as isize) = '\u{0}' as i8;
    return buf;
}
// returns the appropriate synonym context for the current game type and situation
/*
==================
BotSynonymContext
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSynonymContext(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut context: i32 = 0;
    context = 1 | 2 | 1024;
    //
    if gametype == crate::bg_public_h::GT_CTF as i32 {
        if BotTeam(bs) == crate::bg_public_h::TEAM_RED as i32 {
            context |= 4
        } else {
            context |= 8
        }
    }
    return context;
}
//choose a weapon
/*
==================
BotChooseWeapon
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotChooseWeapon(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut newweaponnum: i32 = 0;
    if (*bs).cur_ps.weaponstate == crate::bg_public_h::WEAPON_RAISING as i32
        || (*bs).cur_ps.weaponstate == crate::bg_public_h::WEAPON_DROPPING as i32
    {
        crate::src::game::g_syscalls::trap_EA_SelectWeapon((*bs).client, (*bs).weaponnum);
    } else {
        newweaponnum = crate::src::game::g_syscalls::trap_BotChooseBestFightWeapon(
            (*bs).ws,
            (*bs).inventory.as_mut_ptr(),
        );
        if (*bs).weaponnum != newweaponnum {
            (*bs).weaponchange_time = crate::src::game::ai_main::floattime
        }
        (*bs).weaponnum = newweaponnum;
        //BotAI_Print(PRT_MESSAGE, "bs->weaponnum = %d\n", bs->weaponnum);
        crate::src::game::g_syscalls::trap_EA_SelectWeapon((*bs).client, (*bs).weaponnum);
    };
}
//setup movement stuff
/*
==================
BotSetupForMovement
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetupForMovement(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut initmove: crate::be_ai_move_h::bot_initmove_t = crate::be_ai_move_h::bot_initmove_t {
        origin: [0.; 3],
        velocity: [0.; 3],
        viewoffset: [0.; 3],
        entitynum: 0,
        client: 0,
        thinktime: 0.,
        presencetype: 0,
        viewangles: [0.; 3],
        or_moveflags: 0,
    };
    crate::stdlib::memset(
        &mut initmove as *mut crate::be_ai_move_h::bot_initmove_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::be_ai_move_h::bot_initmove_t>(),
    );
    initmove.origin[0] = (*bs).cur_ps.origin[0];
    initmove.origin[1] = (*bs).cur_ps.origin[1];
    initmove.origin[2] = (*bs).cur_ps.origin[2];
    initmove.velocity[0] = (*bs).cur_ps.velocity[0];
    initmove.velocity[1] = (*bs).cur_ps.velocity[1];
    initmove.velocity[2] = (*bs).cur_ps.velocity[2];
    initmove.viewoffset[2] = 0f32;
    initmove.viewoffset[1] = initmove.viewoffset[2];
    initmove.viewoffset[0] = initmove.viewoffset[1];
    initmove.viewoffset[2] += (*bs).cur_ps.viewheight as f32;
    initmove.entitynum = (*bs).entitynum;
    initmove.client = (*bs).client;
    initmove.thinktime = (*bs).thinktime;
    //set the onground flag
    if (*bs).cur_ps.groundEntityNum != ((1) << 10) - 1 {
        initmove.or_moveflags |= 2
    }
    //set the teleported flag
    if (*bs).cur_ps.pm_flags & 64 != 0 && (*bs).cur_ps.pm_time > 0 {
        initmove.or_moveflags |= 32
    }
    //set the waterjump flag
    if (*bs).cur_ps.pm_flags & 256 != 0 && (*bs).cur_ps.pm_time > 0 {
        initmove.or_moveflags |= 16
    }
    //set presence type
    if (*bs).cur_ps.pm_flags & 1 != 0 {
        initmove.presencetype = 4
    } else {
        initmove.presencetype = 2
    }
    //
    if (*bs).walker as f64 > 0.5 {
        initmove.or_moveflags |= 512
    }
    //
    initmove.viewangles[0] = (*bs).viewangles[0];
    initmove.viewangles[1] = (*bs).viewangles[1];
    initmove.viewangles[2] = (*bs).viewangles[2];
    //
    crate::src::game::g_syscalls::trap_BotInitMoveState(
        (*bs).ms,
        &mut initmove as *mut crate::be_ai_move_h::bot_initmove_t as *mut libc::c_void,
    );
}
/*
==================
BotCheckItemPickup
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCheckItemPickup(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut oldinventory: *mut i32,
) {
}
//update the inventory
/*
==================
BotUpdateInventory
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotUpdateInventory(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut oldinventory: [i32; 256] = [0; 256];
    crate::stdlib::memcpy(
        oldinventory.as_mut_ptr() as *mut libc::c_void,
        (*bs).inventory.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[i32; 256]>(),
    );
    //armor
    (*bs).inventory[1] = (*bs).cur_ps.stats[crate::bg_public_h::STAT_ARMOR as usize];
    //weapons
    (*bs).inventory[4] = ((*bs).cur_ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << crate::bg_public_h::WP_GAUNTLET as i32
        != 0) as i32;
    (*bs).inventory[5] = ((*bs).cur_ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << crate::bg_public_h::WP_SHOTGUN as i32
        != 0) as i32;
    (*bs).inventory[6] = ((*bs).cur_ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << crate::bg_public_h::WP_MACHINEGUN as i32
        != 0) as i32;
    (*bs).inventory[7] = ((*bs).cur_ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << crate::bg_public_h::WP_GRENADE_LAUNCHER as i32
        != 0) as i32;
    (*bs).inventory[8] = ((*bs).cur_ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << crate::bg_public_h::WP_ROCKET_LAUNCHER as i32
        != 0) as i32;
    (*bs).inventory[9] = ((*bs).cur_ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << crate::bg_public_h::WP_LIGHTNING as i32
        != 0) as i32;
    (*bs).inventory[10] = ((*bs).cur_ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << crate::bg_public_h::WP_RAILGUN as i32
        != 0) as i32;
    (*bs).inventory[11] = ((*bs).cur_ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << crate::bg_public_h::WP_PLASMAGUN as i32
        != 0) as i32;
    (*bs).inventory[13] = ((*bs).cur_ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << crate::bg_public_h::WP_BFG as i32
        != 0) as i32;
    (*bs).inventory[14] = ((*bs).cur_ps.stats[crate::bg_public_h::STAT_WEAPONS as usize]
        & (1) << crate::bg_public_h::WP_GRAPPLING_HOOK as i32
        != 0) as i32;
    //ammo
    (*bs).inventory[18] = (*bs).cur_ps.ammo[crate::bg_public_h::WP_SHOTGUN as usize];
    (*bs).inventory[19] = (*bs).cur_ps.ammo[crate::bg_public_h::WP_MACHINEGUN as usize];
    (*bs).inventory[20] = (*bs).cur_ps.ammo[crate::bg_public_h::WP_GRENADE_LAUNCHER as usize];
    (*bs).inventory[21] = (*bs).cur_ps.ammo[crate::bg_public_h::WP_PLASMAGUN as usize];
    (*bs).inventory[22] = (*bs).cur_ps.ammo[crate::bg_public_h::WP_LIGHTNING as usize];
    (*bs).inventory[23] = (*bs).cur_ps.ammo[crate::bg_public_h::WP_ROCKET_LAUNCHER as usize];
    (*bs).inventory[24] = (*bs).cur_ps.ammo[crate::bg_public_h::WP_RAILGUN as usize];
    (*bs).inventory[25] = (*bs).cur_ps.ammo[crate::bg_public_h::WP_BFG as usize];
    //powerups
    (*bs).inventory[29] = (*bs).cur_ps.stats[crate::bg_public_h::STAT_HEALTH as usize];
    (*bs).inventory[30] =
        ((*bs).cur_ps.stats[crate::bg_public_h::STAT_HOLDABLE_ITEM as usize] == 26) as i32;
    (*bs).inventory[31] =
        ((*bs).cur_ps.stats[crate::bg_public_h::STAT_HOLDABLE_ITEM as usize] == 27) as i32;
    (*bs).inventory[35] = ((*bs).cur_ps.powerups[crate::bg_public_h::PW_QUAD as usize] != 0) as i32;
    (*bs).inventory[36] =
        ((*bs).cur_ps.powerups[crate::bg_public_h::PW_BATTLESUIT as usize] != 0) as i32;
    (*bs).inventory[37] =
        ((*bs).cur_ps.powerups[crate::bg_public_h::PW_HASTE as usize] != 0) as i32;
    (*bs).inventory[38] =
        ((*bs).cur_ps.powerups[crate::bg_public_h::PW_INVIS as usize] != 0) as i32;
    (*bs).inventory[39] =
        ((*bs).cur_ps.powerups[crate::bg_public_h::PW_REGEN as usize] != 0) as i32;
    (*bs).inventory[40] =
        ((*bs).cur_ps.powerups[crate::bg_public_h::PW_FLIGHT as usize] != 0) as i32;
    (*bs).inventory[45] =
        ((*bs).cur_ps.powerups[crate::bg_public_h::PW_REDFLAG as usize] != 0) as i32;
    (*bs).inventory[46] =
        ((*bs).cur_ps.powerups[crate::bg_public_h::PW_BLUEFLAG as usize] != 0) as i32;
    BotCheckItemPickup(bs, oldinventory.as_mut_ptr());
}
//update the inventory during battle
/*
==================
BotUpdateBattleInventory
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotUpdateBattleInventory(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut enemy: i32,
) {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    crate::src::game::ai_main::BotEntityInfo(enemy, &mut entinfo);
    dir[0] = entinfo.origin[0] - (*bs).origin[0];
    dir[1] = entinfo.origin[1] - (*bs).origin[1];
    dir[2] = entinfo.origin[2] - (*bs).origin[2];
    (*bs).inventory[201] = dir[2] as i32;
    dir[2] = 0f32;
    (*bs).inventory[200] =
        VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t) as i32;
    //FIXME: add num visible enemies and num visible team mates to the inventory
}
//use holdable items during battle
/*
==================
BotBattleUseItems
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotBattleUseItems(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    if (*bs).inventory[29] < 40 {
        if (*bs).inventory[30] > 0 {
            if BotCTFCarryingFlag(bs) == 0 {
                crate::src::game::g_syscalls::trap_EA_Use((*bs).client);
            }
        }
    }
    if (*bs).inventory[29] < 60 {
        if (*bs).inventory[31] > 0 {
            crate::src::game::g_syscalls::trap_EA_Use((*bs).client);
        }
    };
}
/*
==================
BotSetTeleportTime
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetTeleportTime(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    if ((*bs).cur_ps.eFlags ^ (*bs).last_eFlags) & 0x4 != 0 {
        (*bs).teleport_time = crate::src::game::ai_main::floattime
    }
    (*bs).last_eFlags = (*bs).cur_ps.eFlags;
}
//return true if the bot is dead
/*
==================
BotIsDead
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotIsDead(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> crate::src::qcommon::q_shared::qboolean {
    return ((*bs).cur_ps.pm_type == crate::bg_public_h::PM_DEAD as i32)
        as crate::src::qcommon::q_shared::qboolean;
}
//returns true if the bot is in observer mode
/*
==================
BotIsObserver
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotIsObserver(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut buf: [i8; 1024] = [0; 1024];
    if (*bs).cur_ps.pm_type == crate::bg_public_h::PM_SPECTATOR as i32 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    crate::src::game::g_syscalls::trap_GetConfigstring(
        32 + 256 + 256 + (*bs).client,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        buf.as_mut_ptr(),
        b"t\x00" as *const u8 as *const i8,
    )) == crate::bg_public_h::TEAM_SPECTATOR as i32
    {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return crate::src::qcommon::q_shared::qfalse;
}
//returns true if the bot is in the intermission
/*
==================
BotIntermission
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotIntermission(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> crate::src::qcommon::q_shared::qboolean {
    //NOTE: we shouldn't be looking at the game code...
    if crate::src::game::g_main::level.intermissiontime != 0 {
        return crate::src::qcommon::q_shared::qtrue;
    }
    return ((*bs).cur_ps.pm_type == crate::bg_public_h::PM_FREEZE as i32
        || (*bs).cur_ps.pm_type == crate::bg_public_h::PM_INTERMISSION as i32)
        as crate::src::qcommon::q_shared::qboolean;
}
//returns true if the bot is in lava or slime
/*
==================
BotInLavaOrSlime
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotInLavaOrSlime(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut feet: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    feet[0] = (*bs).origin[0];
    feet[1] = (*bs).origin[1];
    feet[2] = (*bs).origin[2];
    feet[2] -= 23f32;
    return (crate::src::game::g_syscalls::trap_AAS_PointContents(feet.as_mut_ptr()) & (8 | 16))
        as crate::src::qcommon::q_shared::qboolean;
}
//create a new waypoint
/*
==================
BotCreateWayPoint
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCreateWayPoint(
    mut name: *mut i8,
    mut origin: *mut crate::src::qcommon::q_shared::vec_t,
    mut areanum: i32,
) -> *mut crate::src::game::ai_main::bot_waypoint_t {
    let mut wp: *mut crate::src::game::ai_main::bot_waypoint_t =
        0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    let mut waypointmins: crate::src::qcommon::q_shared::vec3_t = [-8f32, -8f32, -8f32];
    let mut waypointmaxs: crate::src::qcommon::q_shared::vec3_t = [8f32, 8f32, 8f32];
    wp = botai_freewaypoints;
    if wp.is_null() {
        crate::src::game::ai_main::BotAI_Print(
            2,
            b"BotCreateWayPoint: Out of waypoints\n\x00" as *const u8 as *mut i8,
        );
        return 0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    }
    botai_freewaypoints = (*botai_freewaypoints).next;
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*wp).name.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[i8; 32]>() as i32,
    );
    (*wp).goal.origin[0] = *origin.offset(0);
    (*wp).goal.origin[1] = *origin.offset(1);
    (*wp).goal.origin[2] = *origin.offset(2);
    (*wp).goal.mins[0] = waypointmins[0];
    (*wp).goal.mins[1] = waypointmins[1];
    (*wp).goal.mins[2] = waypointmins[2];
    (*wp).goal.maxs[0] = waypointmaxs[0];
    (*wp).goal.maxs[1] = waypointmaxs[1];
    (*wp).goal.maxs[2] = waypointmaxs[2];
    (*wp).goal.areanum = areanum;
    (*wp).next = 0 as *mut crate::src::game::ai_main::bot_waypoint_s;
    (*wp).prev = 0 as *mut crate::src::game::ai_main::bot_waypoint_s;
    return wp;
}
//find a waypoint with the given name
/*
==================
BotFindWayPoint
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotFindWayPoint(
    mut waypoints: *mut crate::src::game::ai_main::bot_waypoint_t,
    mut name: *mut i8,
) -> *mut crate::src::game::ai_main::bot_waypoint_t {
    let mut wp: *mut crate::src::game::ai_main::bot_waypoint_t =
        0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    wp = waypoints;
    while !wp.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp((*wp).name.as_mut_ptr(), name) == 0 {
            return wp;
        }
        wp = (*wp).next
    }
    return 0 as *mut crate::src::game::ai_main::bot_waypoint_t;
}
//free waypoints
/*
==================
BotFreeWaypoints
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotFreeWaypoints(mut wp: *mut crate::src::game::ai_main::bot_waypoint_t) {
    let mut nextwp: *mut crate::src::game::ai_main::bot_waypoint_t =
        0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    while !wp.is_null() {
        nextwp = (*wp).next;
        (*wp).next = botai_freewaypoints;
        botai_freewaypoints = wp;
        wp = nextwp
    }
}
/*
==================
BotInitWaypoints
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotInitWaypoints() {
    let mut i: i32 = 0;
    botai_freewaypoints = 0 as *mut crate::src::game::ai_main::bot_waypoint_t;
    i = 0;
    while i < 128 {
        botai_waypoints[i as usize].next = botai_freewaypoints;
        botai_freewaypoints = &mut *botai_waypoints.as_mut_ptr().offset(i as isize)
            as *mut crate::src::game::ai_main::bot_waypoint_t;
        i += 1
    }
}
//returns true if teamplay is on
/*
==================
TeamPlayIsOn
==================
*/
#[no_mangle]

pub unsafe extern "C" fn TeamPlayIsOn() -> i32 {
    return (gametype >= crate::bg_public_h::GT_TEAM as i32) as i32;
}
//returns the aggression of the bot in the range [0, 100]
/*
==================
BotAggression
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAggression(mut bs: *mut crate::src::game::ai_main::bot_state_t) -> f32 {
    //if the bot has quad
    if (*bs).inventory[35] != 0 {
        //if the bot is not holding the gauntlet or the enemy is really nearby
        if (*bs).weaponnum != crate::bg_public_h::WP_GAUNTLET as i32 || (*bs).inventory[200] < 80 {
            return 70f32;
        }
    }
    //if the enemy is located way higher than the bot
    if (*bs).inventory[201] > 200 {
        return 0f32;
    }
    //if the bot is very low on health
    if (*bs).inventory[29] < 60 {
        return 0f32;
    }
    //if the bot is low on health
    if (*bs).inventory[29] < 80 {
        //if the bot has insufficient armor
        if (*bs).inventory[1] < 40 {
            return 0f32;
        }
    }
    //if the bot can use the bfg
    if (*bs).inventory[13] > 0 && (*bs).inventory[25] > 7 {
        return 100f32;
    }
    //if the bot can use the railgun
    if (*bs).inventory[10] > 0 && (*bs).inventory[24] > 5 {
        return 95f32;
    }
    //if the bot can use the lightning gun
    if (*bs).inventory[9] > 0 && (*bs).inventory[22] > 50 {
        return 90f32;
    }
    //if the bot can use the rocketlauncher
    if (*bs).inventory[8] > 0 && (*bs).inventory[23] > 5 {
        return 90f32;
    }
    //if the bot can use the plasmagun
    if (*bs).inventory[11] > 0 && (*bs).inventory[21] > 40 {
        return 85f32;
    }
    //if the bot can use the grenade launcher
    if (*bs).inventory[7] > 0 && (*bs).inventory[20] > 10 {
        return 80f32;
    }
    //if the bot can use the shotgun
    if (*bs).inventory[5] > 0 && (*bs).inventory[18] > 10 {
        return 50f32;
    }
    //otherwise the bot is not feeling too good
    return 0f32;
}
//returns how bad the bot feels
/*
==================
BotFeelingBad
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotFeelingBad(mut bs: *mut crate::src::game::ai_main::bot_state_t) -> f32 {
    if (*bs).weaponnum == crate::bg_public_h::WP_GAUNTLET as i32 {
        return 100f32;
    }
    if (*bs).inventory[29] < 40 {
        return 100f32;
    }
    if (*bs).weaponnum == crate::bg_public_h::WP_MACHINEGUN as i32 {
        return 90f32;
    }
    if (*bs).inventory[29] < 60 {
        return 80f32;
    }
    return 0f32;
}
//returns true if the bot wants to retreat
/*
==================
BotWantsToRetreat
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotWantsToRetreat(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    if gametype == crate::bg_public_h::GT_CTF as i32 {
        //always retreat when carrying a CTF flag
        if BotCTFCarryingFlag(bs) != 0 {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    //
    if (*bs).enemy >= 0 {
        crate::src::game::ai_main::BotEntityInfo((*bs).enemy, &mut entinfo);
        // if the enemy is carrying a flag
        if EntityCarriesFlag(&mut entinfo) as u64 != 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //if the bot is getting the flag
    if (*bs).ltgtype == 4 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //
    if BotAggression(bs) < 50f32 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//returns true if the bot wants to chase
/*
==================
BotWantsToChase
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotWantsToChase(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    if gametype == crate::bg_public_h::GT_CTF as i32 {
        //never chase when carrying a CTF flag
        if BotCTFCarryingFlag(bs) != 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //always chase if the enemy is carrying a flag
        crate::src::game::ai_main::BotEntityInfo((*bs).enemy, &mut entinfo);
        if EntityCarriesFlag(&mut entinfo) as u64 != 0 {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    //if the bot is getting the flag
    if (*bs).ltgtype == 4 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    if BotAggression(bs) > 50f32 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//returns true if the bot wants to help
/*
==================
BotWantsToHelp
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotWantsToHelp(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//returns true if the bot can and wants to rocketjump
/*
==================
BotCanAndWantsToRocketJump
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCanAndWantsToRocketJump(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut rocketjumper: f32 = 0.;
    //if rocket jumping is disabled
    if bot_rocketjump.integer == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if no rocket launcher
    if (*bs).inventory[8] <= 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if low on rockets
    if (*bs).inventory[23] < 3 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //never rocket jump with the Quad
    if (*bs).inventory[35] != 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if low on health
    if (*bs).inventory[29] < 60 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if not full health
    if (*bs).inventory[29] < 90 {
        //if the bot has insufficient armor
        if (*bs).inventory[1] < 40 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    rocketjumper =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 38, 0f32, 1f32);
    if (rocketjumper as f64) < 0.5 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
// returns true if the bot has a persistant powerup and a weapon
/*
==================
BotHasPersistantPowerupAndWeapon
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotHasPersistantPowerupAndWeapon(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    //if the bot is very low on health
    if (*bs).inventory[29] < 60 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the bot is low on health
    if (*bs).inventory[29] < 80 {
        //if the bot has insufficient armor
        if (*bs).inventory[1] < 40 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //if the bot can use the bfg
    if (*bs).inventory[13] > 0 && (*bs).inventory[25] > 7 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //if the bot can use the railgun
    if (*bs).inventory[10] > 0 && (*bs).inventory[24] > 5 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //if the bot can use the lightning gun
    if (*bs).inventory[9] > 0 && (*bs).inventory[22] > 50 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //if the bot can use the rocketlauncher
    if (*bs).inventory[8] > 0 && (*bs).inventory[23] > 5 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //
    if (*bs).inventory[15] > 0 && (*bs).inventory[26] > 5 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //
    if (*bs).inventory[16] > 0 && (*bs).inventory[27] > 5 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //
    if (*bs).inventory[17] > 0 && (*bs).inventory[28] > 40 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //if the bot can use the plasmagun
    if (*bs).inventory[11] > 0 && (*bs).inventory[21] > 20 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
==================
BotGoCamp
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGoCamp(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) {
    let mut camper: f32 = 0.;
    (*bs).decisionmaker = (*bs).client;
    //set message time to zero so bot will NOT show any message
    (*bs).teammessage_time = 0f32;
    //set the ltg type
    (*bs).ltgtype = 7;
    //set the team goal
    crate::stdlib::memcpy(
        &mut (*bs).teamgoal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        goal as *const libc::c_void,
        ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>(),
    );
    //get the team goal time
    camper =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 44, 0f32, 1f32);
    if camper as f64 > 0.99 {
        (*bs).teamgoal_time = crate::src::game::ai_main::floattime + 99999f32
    } else {
        (*bs).teamgoal_time = crate::src::game::ai_main::floattime
            + 120f32
            + 180f32 * camper
            + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 15f32
    }
    //set the last time the bot started camping
    (*bs).camp_time = crate::src::game::ai_main::floattime;
    //the teammate that requested the camping
    (*bs).teammate = 0;
    //do NOT type arrive message
    (*bs).arrive_time = 1f32;
}
//returns true if the bot wants to and goes camping
/*
==================
BotWantsToCamp
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotWantsToCamp(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut camper: f32 = 0.;
    let mut cs: i32 = 0;
    let mut traveltime: i32 = 0;
    let mut besttraveltime: i32 = 0;
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
    let mut bestgoal: crate::be_ai_goal_h::bot_goal_t = crate::be_ai_goal_h::bot_goal_t {
        origin: [0.; 3],
        areanum: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        entitynum: 0,
        number: 0,
        flags: 0,
        iteminfo: 0,
    };
    camper =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 44, 0f32, 1f32);
    if (camper as f64) < 0.1 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the bot has a team goal
    if (*bs).ltgtype == 1
        || (*bs).ltgtype == 2
        || (*bs).ltgtype == 3
        || (*bs).ltgtype == 4
        || (*bs).ltgtype == 5
        || (*bs).ltgtype == 7
        || (*bs).ltgtype == 8
        || (*bs).ltgtype == 9
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if camped recently
    if (*bs).camp_time > crate::src::game::ai_main::floattime - 60f32 + 300f32 * (1f32 - camper) {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    if (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 > camper {
        (*bs).camp_time = crate::src::game::ai_main::floattime;
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the bot isn't healthy enough
    if BotAggression(bs) < 50f32 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //the bot should have at least have the rocket launcher, the railgun or the bfg10k with some ammo
    if ((*bs).inventory[8] <= 0 || (*bs).inventory[23] < 10)
        && ((*bs).inventory[10] <= 0 || (*bs).inventory[24] < 10)
        && ((*bs).inventory[13] <= 0 || (*bs).inventory[25] < 10)
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //find the closest camp spot
    besttraveltime = 99999;
    cs = crate::src::game::g_syscalls::trap_BotGetNextCampSpotGoal(
        0,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
    );
    while cs != 0 {
        traveltime = crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
            (*bs).areanum,
            (*bs).origin.as_mut_ptr(),
            goal.areanum,
            0x2 | 0x4
                | 0x8
                | 0x10
                | 0x20
                | 0x80
                | 0x100
                | 0x200
                | 0x400
                | 0x800
                | 0x80000
                | 0x100000
                | 0x40000
                | 0x1000000,
        );
        if traveltime != 0 && traveltime < besttraveltime {
            besttraveltime = traveltime;
            crate::stdlib::memcpy(
                &mut bestgoal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
                &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
                ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>(),
            );
        }
        cs = crate::src::game::g_syscalls::trap_BotGetNextCampSpotGoal(
            cs,
            &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        )
    }
    if besttraveltime > 150 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //ok found a camp spot, go camp there
    BotGoCamp(bs, &mut bestgoal);
    (*bs).ordered = crate::src::qcommon::q_shared::qfalse as i32;
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotDontAvoid
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotDontAvoid(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut itemname: *mut i8,
) {
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
    let mut num: i32 = 0;
    num = crate::src::game::g_syscalls::trap_BotGetLevelItemGoal(
        -(1),
        itemname,
        &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
    );
    while num >= 0 {
        crate::src::game::g_syscalls::trap_BotRemoveFromAvoidGoals((*bs).gs, goal.number);
        num = crate::src::game::g_syscalls::trap_BotGetLevelItemGoal(
            num,
            itemname,
            &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        )
    }
}
/*
==================
BotGoForPowerups
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGoForPowerups(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    //don't avoid any of the powerups anymore
    BotDontAvoid(bs, b"Quad Damage\x00" as *const u8 as *mut i8);
    BotDontAvoid(bs, b"Regeneration\x00" as *const u8 as *mut i8);
    BotDontAvoid(bs, b"Battle Suit\x00" as *const u8 as *mut i8);
    BotDontAvoid(bs, b"Speed\x00" as *const u8 as *mut i8);
    BotDontAvoid(bs, b"Invisibility\x00" as *const u8 as *mut i8);
    //BotDontAvoid(bs, "Flight");
    //reset the long term goal time so the bot will go for the powerup
    //NOTE: the long term goal type doesn't change
    (*bs).ltg_time = 0f32;
}
//returns a roam goal
/*
==================
BotRoamGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotRoamGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut goal: *mut crate::src::qcommon::q_shared::vec_t,
) {
    let mut pc: i32 = 0;
    let mut i: i32 = 0;
    let mut len: f32 = 0.;
    let mut rnd: f32 = 0.;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bestorg: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut belowbestorg: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut trace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };

    for i in 0..10 {
        //start at the bot origin
        bestorg[0] = (*bs).origin[0];

        bestorg[1] = (*bs).origin[1];

        bestorg[2] = (*bs).origin[2];

        rnd = (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32;

        if rnd as f64 > 0.25 {
            //add a random value to the x-coordinate
            if (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64) < 0.5 {
                bestorg[0] -= 800f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) + 100f32
            } else {
                bestorg[0] += 800f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) + 100f32
            }
        }

        if (rnd as f64) < 0.75 {
            //add a random value to the y-coordinate
            if (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64) < 0.5 {
                bestorg[1] -= 800f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) + 100f32
            } else {
                bestorg[1] += 800f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) + 100f32
            }
        }
        //add a random value to the z-coordinate (NOTE: 48 = maxjump?)
        bestorg[2] = (bestorg[2] as f64
            + (2i32 * 48) as f64
                * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)))
            as crate::src::qcommon::q_shared::vec_t;
        //trace a line from the origin to the roam target
        crate::src::game::ai_main::BotAI_Trace(
            &mut trace,
            (*bs).origin.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            bestorg.as_mut_ptr(),
            (*bs).entitynum,
            1,
        );
        //direction and length towards the roam target
        dir[0] = trace.endpos[0] - (*bs).origin[0];

        dir[1] = trace.endpos[1] - (*bs).origin[1];

        dir[2] = trace.endpos[2] - (*bs).origin[2];

        len = crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        //if the roam target is far away enough
        if len > 200f32 {
            //the roam target is in the given direction before walls
            dir[0] = dir[0] * (len * trace.fraction - 40f32);
            dir[1] = dir[1] * (len * trace.fraction - 40f32);
            dir[2] = dir[2] * (len * trace.fraction - 40f32);
            bestorg[0] = (*bs).origin[0] + dir[0];
            bestorg[1] = (*bs).origin[1] + dir[1];
            bestorg[2] = (*bs).origin[2] + dir[2];
            //get the coordinates of the floor below the roam target
            belowbestorg[0] = bestorg[0];
            belowbestorg[1] = bestorg[1];
            belowbestorg[2] = bestorg[2] - 800f32;
            crate::src::game::ai_main::BotAI_Trace(
                &mut trace,
                bestorg.as_mut_ptr(),
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                belowbestorg.as_mut_ptr(),
                (*bs).entitynum,
                1,
            );
            //
            if trace.startsolid as u64 == 0 {
                trace.endpos[2] += 1.;
                pc = crate::src::game::g_syscalls::trap_PointContents(
                    trace.endpos.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                    (*bs).entitynum,
                );
                if pc & (8 | 16) == 0 {
                    *goal.offset(0) = bestorg[0];
                    *goal.offset(1) = bestorg[1];
                    *goal.offset(2) = bestorg[2];
                    return;
                }
            }
        }
    }
    *goal.offset(0) = bestorg[0];
    *goal.offset(1) = bestorg[1];
    *goal.offset(2) = bestorg[2];
}
//the bot will perform attack movements
/*
==================
BotAttackMove
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAttackMove(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut tfl: i32,
) -> crate::be_ai_move_h::bot_moveresult_t {
    let mut movetype: i32 = 0;
    let mut i: i32 = 0;
    let mut attackentity: i32 = 0;
    let mut attack_skill: f32 = 0.;
    let mut jumper: f32 = 0.;
    let mut croucher: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut strafechange_time: f32 = 0.;
    let mut attack_dist: f32 = 0.;
    let mut attack_range: f32 = 0.;
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut backward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut sideward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 1f32];
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    let mut moveresult: crate::be_ai_move_h::bot_moveresult_t =
        crate::be_ai_move_h::bot_moveresult_t {
            failure: 0,
            type_0: 0,
            blocked: 0,
            blockentity: 0,
            traveltype: 0,
            flags: 0,
            weapon: 0,
            movedir: [0.; 3],
            ideal_viewangles: [0.; 3],
        };
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
    attackentity = (*bs).enemy;
    //
    if (*bs).attackchase_time > crate::src::game::ai_main::floattime {
        //create the chase goal
        goal.entitynum = attackentity;
        goal.areanum = (*bs).lastenemyareanum;
        goal.origin[0] = (*bs).lastenemyorigin[0];
        goal.origin[1] = (*bs).lastenemyorigin[1];
        goal.origin[2] = (*bs).lastenemyorigin[2];
        goal.mins[0] = -8f32;
        goal.mins[1] = -8f32;
        goal.mins[2] = -8f32;
        goal.maxs[0] = 8f32;
        goal.maxs[1] = 8f32;
        goal.maxs[2] = 8f32;
        //initialize the movement state
        BotSetupForMovement(bs);
        //move towards the goal
        crate::src::game::g_syscalls::trap_BotMoveToGoal(
            &mut moveresult as *mut crate::be_ai_move_h::bot_moveresult_t as *mut libc::c_void,
            (*bs).ms,
            &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
            tfl,
        );
        return moveresult;
    }
    //
    crate::stdlib::memset(
        &mut moveresult as *mut crate::be_ai_move_h::bot_moveresult_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::be_ai_move_h::bot_moveresult_t>(),
    );
    //
    attack_skill =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 2, 0f32, 1f32);
    jumper =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 37, 0f32, 1f32);
    croucher =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 36, 0f32, 1f32);
    //if the bot is really stupid
    if (attack_skill as f64) < 0.2 {
        return moveresult;
    }
    //initialize the movement state
    BotSetupForMovement(bs);
    //get the enemy entity info
    crate::src::game::ai_main::BotEntityInfo(attackentity, &mut entinfo);
    //direction towards the enemy
    forward[0] = entinfo.origin[0] - (*bs).origin[0];
    forward[1] = entinfo.origin[1] - (*bs).origin[1];
    forward[2] = entinfo.origin[2] - (*bs).origin[2];
    //the distance towards the enemy
    dist = crate::src::qcommon::q_math::VectorNormalize(forward.as_mut_ptr());
    backward[0] = -forward[0];
    backward[1] = -forward[1];
    backward[2] = -forward[2];
    //walk, crouch or jump
    movetype = 1;
    //
    if (*bs).attackcrouch_time < crate::src::game::ai_main::floattime - 1f32 {
        if ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) < jumper {
            movetype = 4
        } else if (*bs).attackcrouch_time < crate::src::game::ai_main::floattime - 1f32
            && ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) < croucher
        {
            (*bs).attackcrouch_time = crate::src::game::ai_main::floattime + croucher * 5f32
        }
    }
    if (*bs).attackcrouch_time > crate::src::game::ai_main::floattime {
        movetype = 2
    }
    //wait at least one second before crouching again
    //if the bot should jump
    if movetype == 4 {
        //if jumped last frame
        if (*bs).attackjump_time > crate::src::game::ai_main::floattime {
            movetype = 1
        } else {
            (*bs).attackjump_time = crate::src::game::ai_main::floattime + 1f32
        }
    }
    if (*bs).cur_ps.weapon == crate::bg_public_h::WP_GAUNTLET as i32 {
        attack_dist = 0f32;
        attack_range = 0f32
    } else {
        attack_dist = 140f32;
        attack_range = 40f32
    }
    //if the bot is stupid
    if attack_skill as f64 <= 0.4 {
        //just walk to or away from the enemy
        if dist > attack_dist + attack_range {
            if crate::src::game::g_syscalls::trap_BotMoveInDirection(
                (*bs).ms,
                forward.as_mut_ptr(),
                400f32,
                movetype,
            ) != 0
            {
                return moveresult;
            }
        }
        if dist < attack_dist - attack_range {
            if crate::src::game::g_syscalls::trap_BotMoveInDirection(
                (*bs).ms,
                backward.as_mut_ptr(),
                400f32,
                movetype,
            ) != 0
            {
                return moveresult;
            }
        }
        return moveresult;
    }
    //increase the strafe time
    (*bs).attackstrafe_time += (*bs).thinktime;
    //get the strafe change time
    strafechange_time = (0.4 + (1f32 - attack_skill) as f64 * 0.2) as f32;
    if attack_skill as f64 > 0.7 {
        strafechange_time = (strafechange_time as f64
            + 2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5) * 0.2)
            as f32
    }
    //if the strafe direction should be changed
    if (*bs).attackstrafe_time > strafechange_time {
        //some magic number :)
        if ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 > 0.935 {
            //flip the strafe direction
            (*bs).flags ^= 1;
            (*bs).attackstrafe_time = 0f32
        }
    }

    for i in 0..2 {
        hordir[0] = forward[0];

        hordir[1] = forward[1];

        hordir[2] = 0f32;

        crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr());
        //get the sideward vector
        CrossProduct(
            hordir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            sideward.as_mut_ptr(),
        );
        //reverse the vector depending on the strafe direction
        if (*bs).flags & 1 != 0 {
            sideward[0] = -sideward[0];
            sideward[1] = -sideward[1];
            sideward[2] = -sideward[2]
        }
        //randomly go back a little
        if ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 > 0.9 {
            sideward[0] = sideward[0] + backward[0];
            sideward[1] = sideward[1] + backward[1];
            sideward[2] = sideward[2] + backward[2]
        } else if dist > attack_dist + attack_range {
            sideward[0] = sideward[0] + forward[0];
            sideward[1] = sideward[1] + forward[1];
            sideward[2] = sideward[2] + forward[2]
        } else if dist < attack_dist - attack_range {
            sideward[0] = sideward[0] + backward[0];
            sideward[1] = sideward[1] + backward[1];
            sideward[2] = sideward[2] + backward[2]
        }
        //walk forward or backward to get at the ideal attack distance
        //perform the movement
        if crate::src::game::g_syscalls::trap_BotMoveInDirection(
            (*bs).ms,
            sideward.as_mut_ptr(),
            400f32,
            movetype,
        ) != 0
        {
            return moveresult;
        }
        //movement failed, flip the strafe direction
        (*bs).flags ^= 1;

        (*bs).attackstrafe_time = 0f32;
    }
    //bot couldn't do any useful movement
    //	bs->attackchase_time = AAS_Time() + 6;
    return moveresult;
}
//returns true if the bot and the entity are in the same team
/*
==================
BotSameTeam
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSameTeam(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut entnum: i32,
) -> i32 {
    if (*bs).client < 0 || (*bs).client >= 64 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if entnum < 0 || entnum >= 64 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if gametype >= crate::bg_public_h::GT_TEAM as i32 {
        if (*crate::src::game::g_main::level
            .clients
            .offset((*bs).client as isize))
        .sess
        .sessionTeam
            == (*crate::src::game::g_main::level
                .clients
                .offset(entnum as isize))
            .sess
            .sessionTeam
        {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//returns true if within the field of vision for the given angles
/*
==================
InFieldOfVision
==================
*/
#[no_mangle]

pub unsafe extern "C" fn InFieldOfVision(
    mut viewangles: *mut crate::src::qcommon::q_shared::vec_t,
    mut fov: f32,
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    let mut diff: f32 = 0.;
    let mut angle: f32 = 0.;

    for i in 0..2 {
        angle = crate::src::qcommon::q_math::AngleMod(*viewangles.offset(i as isize));

        *angles.offset(i as isize) =
            crate::src::qcommon::q_math::AngleMod(*angles.offset(i as isize));

        diff = *angles.offset(i as isize) - angle;

        if *angles.offset(i as isize) > angle {
            if diff as f64 > 180.0 {
                diff = (diff as f64 - 360.0) as f32
            }
        } else if (diff as f64) < -180.0 {
            diff = (diff as f64 + 360.0) as f32
        }

        if diff > 0f32 {
            if diff as f64 > fov as f64 * 0.5 {
                return crate::src::qcommon::q_shared::qfalse;
            }
        } else if (diff as f64) < -fov as f64 * 0.5 {
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    return crate::src::qcommon::q_shared::qtrue;
}
//returns entity visibility in the range [0, 1]
/*
==================
BotEntityVisible

returns visibility in the range [0, 1] taking fog and water surfaces into account
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotEntityVisible(
    mut viewer: i32,
    mut eye: *mut crate::src::qcommon::q_shared::vec_t,
    mut viewangles: *mut crate::src::qcommon::q_shared::vec_t,
    mut fov: f32,
    mut ent: i32,
) -> f32 {
    let mut i: i32 = 0;
    let mut contents_mask: i32 = 0;
    let mut passent: i32 = 0;
    let mut hitent: i32 = 0;
    let mut infog: i32 = 0;
    let mut inwater: i32 = 0;
    let mut otherinfog: i32 = 0;
    let mut pc: i32 = 0;
    let mut squaredfogdist: f32 = 0.;
    let mut waterfactor: f32 = 0.;
    let mut vis: f32 = 0.;
    let mut bestvis: f32 = 0.;
    let mut trace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut entangles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut middle: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::src::game::ai_main::BotEntityInfo(ent, &mut entinfo);
    if entinfo.valid == 0 {
        return 0f32;
    }
    //calculate middle of bounding box
    middle[0] = entinfo.mins[0] + entinfo.maxs[0];
    middle[1] = entinfo.mins[1] + entinfo.maxs[1];
    middle[2] = entinfo.mins[2] + entinfo.maxs[2];
    middle[0] = (middle[0] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    middle[1] = (middle[1] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    middle[2] = (middle[2] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    middle[0] = entinfo.origin[0] + middle[0];
    middle[1] = entinfo.origin[1] + middle[1];
    middle[2] = entinfo.origin[2] + middle[2];
    //check if entity is within field of vision
    dir[0] = middle[0] - *eye.offset(0);
    dir[1] = middle[1] - *eye.offset(1);
    dir[2] = middle[2] - *eye.offset(2);
    crate::src::qcommon::q_math::vectoangles(
        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        entangles.as_mut_ptr(),
    );
    if InFieldOfVision(viewangles, fov, entangles.as_mut_ptr()) as u64 == 0 {
        return 0f32;
    }
    //
    pc = crate::src::game::g_syscalls::trap_AAS_PointContents(eye);
    infog = pc & 64;
    inwater = pc & (8 | 16 | 32);
    //
    bestvis = 0f32;

    for i in 0..3 {
        //if the point is not in potential visible sight
        //if (!AAS_inPVS(eye, middle)) continue;
        //
        contents_mask = 1 | 0x10000;

        passent = viewer;

        hitent = ent;

        start[0] = *eye.offset(0);

        start[1] = *eye.offset(1);

        start[2] = *eye.offset(2);

        end[0] = middle[0];

        end[1] = middle[1];

        end[2] = middle[2];
        //if the entity is in water, lava or slime
        if crate::src::game::g_syscalls::trap_AAS_PointContents(middle.as_mut_ptr()) & (8 | 16 | 32)
            != 0
        {
            contents_mask |= 8 | 16 | 32
        }
        //if eye is in water, lava or slime
        if inwater != 0 {
            if contents_mask & (8 | 16 | 32) == 0 {
                passent = ent;
                hitent = viewer;
                start[0] = middle[0];
                start[1] = middle[1];
                start[2] = middle[2];
                end[0] = *eye.offset(0);
                end[1] = *eye.offset(1);
                end[2] = *eye.offset(2)
            }
            contents_mask ^= 8 | 16 | 32
        }
        //trace from start to end
        crate::src::game::ai_main::BotAI_Trace(
            &mut trace,
            start.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            end.as_mut_ptr(),
            passent,
            contents_mask,
        );
        //if water was hit
        waterfactor = 1f32;
        //note: trace.contents is always 0, see BotAI_Trace
        if trace.contents & (8 | 16 | 32) != 0 {
            //if the water surface is translucent
            //trace through the water
            contents_mask &= !(8 | 16 | 32);
            crate::src::game::ai_main::BotAI_Trace(
                &mut trace,
                trace.endpos.as_mut_ptr(),
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                0 as *mut crate::src::qcommon::q_shared::vec_t,
                end.as_mut_ptr(),
                passent,
                contents_mask,
            );
            waterfactor = 0.5
        }
        //if a full trace or the hitent was hit
        if trace.fraction >= 1f32 || trace.ent == hitent {
            //check for fog, assuming there's only one fog brush where
            //either the viewer or the entity is in or both are in
            otherinfog =
                crate::src::game::g_syscalls::trap_AAS_PointContents(middle.as_mut_ptr()) & 64;
            if infog != 0 && otherinfog != 0 {
                dir[0] = trace.endpos[0] - *eye.offset(0);
                dir[1] = trace.endpos[1] - *eye.offset(1);
                dir[2] = trace.endpos[2] - *eye.offset(2);
                squaredfogdist = VectorLengthSquared(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                )
            } else if infog != 0 {
                start[0] = trace.endpos[0];
                start[1] = trace.endpos[1];
                start[2] = trace.endpos[2];
                crate::src::game::ai_main::BotAI_Trace(
                    &mut trace,
                    start.as_mut_ptr(),
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    eye,
                    viewer,
                    64,
                );
                dir[0] = *eye.offset(0) - trace.endpos[0];
                dir[1] = *eye.offset(1) - trace.endpos[1];
                dir[2] = *eye.offset(2) - trace.endpos[2];
                squaredfogdist = VectorLengthSquared(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                )
            } else if otherinfog != 0 {
                end[0] = trace.endpos[0];
                end[1] = trace.endpos[1];
                end[2] = trace.endpos[2];
                crate::src::game::ai_main::BotAI_Trace(
                    &mut trace,
                    eye,
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    end.as_mut_ptr(),
                    viewer,
                    64,
                );
                dir[0] = end[0] - trace.endpos[0];
                dir[1] = end[1] - trace.endpos[1];
                dir[2] = end[2] - trace.endpos[2];
                squaredfogdist = VectorLengthSquared(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                )
            } else {
                //if the entity and the viewer are not in fog assume there's no fog in between
                squaredfogdist = 0f32
            }
            //decrease visibility with the view distance through fog
            vis = (1f64
                / (if squaredfogdist as f64 * 0.001 < 1f64 {
                    1f64
                } else {
                    (squaredfogdist as f64) * 0.001
                })) as f32;
            //if entering water visibility is reduced
            vis *= waterfactor;
            //
            if vis > bestvis {
                bestvis = vis
            }
            //if pretty much no fog
            if bestvis as f64 >= 0.95 {
                return bestvis;
            }
        }
        //check bottom and top of bounding box as well
        if i == 0 {
            middle[2] += entinfo.mins[2]
        } else if i == 1 {
            middle[2] += entinfo.maxs[2] - entinfo.mins[2]
        }
    }
    return bestvis;
}
//returns true and sets the .enemy field when an enemy is found
/*
==================
BotFindEnemy
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotFindEnemy(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut curenemy: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut healthdecrease: i32 = 0;
    let mut f: f32 = 0.;
    let mut alertness: f32 = 0.;
    let mut easyfragger: f32 = 0.;
    let mut vis: f32 = 0.;
    let mut squaredist: f32 = 0.;
    let mut cursquaredist: f32 = 0.;
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    let mut curenemyinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    alertness =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 46, 0f32, 1f32);
    easyfragger =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 45, 0f32, 1f32);
    //check if the health decreased
    healthdecrease = ((*bs).lasthealth > (*bs).inventory[29]) as i32;
    //remember the current health value
    (*bs).lasthealth = (*bs).inventory[29];
    //
    if curenemy >= 0 {
        crate::src::game::ai_main::BotEntityInfo(curenemy, &mut curenemyinfo);
        if EntityCarriesFlag(&mut curenemyinfo) as u64 != 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        dir[0] = curenemyinfo.origin[0] - (*bs).origin[0];
        dir[1] = curenemyinfo.origin[1] - (*bs).origin[1];
        dir[2] = curenemyinfo.origin[2] - (*bs).origin[2];
        cursquaredist =
            VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
    } else {
        cursquaredist = 0f32
    }
    let mut current_block_32: u64;

    for i in 0..crate::src::game::g_main::level.maxclients {
        if !(i == (*bs).client) {
            //if it's the current enemy
            if !(i == curenemy) {
                //if the enemy has targeting disabled
                if !(crate::src::game::g_main::g_entities[i as usize].flags & 0x20 != 0) {
                    //
                    crate::src::game::ai_main::BotEntityInfo(i, &mut entinfo);
                    //
                    if !(entinfo.valid == 0) {
                        //if the enemy isn't dead and the enemy isn't the bot self
                        if !(EntityIsDead(&mut entinfo) != 0 || entinfo.number == (*bs).entitynum) {
                            //if the enemy is invisible and not shooting
                            if !(EntityIsInvisible(&mut entinfo) != 0
                                && EntityIsShooting(&mut entinfo) as u64 == 0)
                            {
                                //if not an easy fragger don't shoot at chatting players
                                if !((easyfragger as f64) < 0.5
                                    && EntityIsChatting(&mut entinfo) != 0)
                                {
                                    //
                                    if lastteleport_time
                                        > crate::src::game::ai_main::floattime - 3f32
                                    {
                                        dir[0] = entinfo.origin[0] - lastteleport_origin[0];
                                        dir[1] = entinfo.origin[1] - lastteleport_origin[1];
                                        dir[2] = entinfo.origin[2] - lastteleport_origin[2];
                                        if VectorLengthSquared(dir.as_mut_ptr()
                                            as *const crate::src::qcommon::q_shared::vec_t)
                                            < (70i32 * 70) as f32
                                        {
                                            current_block_32 = 8236137900636309791;
                                        } else {
                                            current_block_32 = 5689316957504528238;
                                        }
                                    } else {
                                        current_block_32 = 5689316957504528238;
                                    }
                                    match current_block_32 {
                                        8236137900636309791 => {}
                                        _ => {
                                            //calculate the distance towards the enemy
                                            dir[0] = entinfo.origin[0] - (*bs).origin[0];
                                            dir[1] = entinfo.origin[1] - (*bs).origin[1];
                                            dir[2] = entinfo.origin[2] - (*bs).origin[2];
                                            squaredist = VectorLengthSquared(dir.as_mut_ptr()
                                                as *const crate::src::qcommon::q_shared::vec_t);
                                            //if this entity is not carrying a flag
                                            if EntityCarriesFlag(&mut entinfo) as u64 == 0 {
                                                //end if
                                                //if this enemy is further away than the current one
                                                if curenemy >= 0 && squaredist > cursquaredist {
                                                    current_block_32 = 8236137900636309791;
                                                } else {
                                                    current_block_32 = 18153031941552419006;
                                                }
                                            } else {
                                                current_block_32 = 18153031941552419006;
                                            }
                                            match current_block_32 {
                                                8236137900636309791 => {}
                                                _ =>
                                                //if the bot has no
                                                {
                                                    if !(squaredist as f64
                                                        > (900.0 + alertness as f64 * 4000.0)
                                                            * (900.0 + alertness as f64 * 4000.0))
                                                    {
                                                        //if on the same team
                                                        if !(BotSameTeam(bs, i) != 0) {
                                                            //if the bot's health decreased or the enemy is shooting
                                                            if curenemy < 0
                                                                && (healthdecrease != 0
                                                                    || EntityIsShooting(
                                                                        &mut entinfo,
                                                                    ) != 0)
                                                            {
                                                                f = 360f32
                                                            } else {
                                                                f = (90i32 + 90) as f32
                                                                    - (90f32
                                                                        - (if squaredist
                                                                            > (810i32 * 810) as f32
                                                                        {
                                                                            (810i32 * 810) as f32
                                                                        } else {
                                                                            squaredist
                                                                        }) / (810i32 * 9) as f32)
                                                            }
                                                            //check if the enemy is visible
                                                            vis = BotEntityVisible(
                                                                (*bs).entitynum,
                                                                (*bs).eye.as_mut_ptr(),
                                                                (*bs).viewangles.as_mut_ptr(),
                                                                f,
                                                                i,
                                                            );
                                                            if !(vis <= 0f32) {
                                                                //if the enemy is quite far away, not shooting and the bot is not damaged
                                                                if curenemy < 0
                                                                    && squaredist
                                                                        > (100i32 * 100) as f32
                                                                    && healthdecrease == 0
                                                                    && EntityIsShooting(
                                                                        &mut entinfo,
                                                                    )
                                                                        as u64
                                                                        == 0
                                                                {
                                                                    //check if we can avoid this enemy
                                                                    dir[0] = (*bs).origin[0]
                                                                        - entinfo.origin[0];
                                                                    dir[1] = (*bs).origin[1]
                                                                        - entinfo.origin[1];
                                                                    dir[2] = (*bs).origin[2]
                                                                        - entinfo.origin[2];
                                                                    crate::src::qcommon::q_math::vectoangles(dir.as_mut_ptr()
                                                                                    as
                                                                                    *const crate::src::qcommon::q_shared::vec_t,
                                                                                angles.as_mut_ptr());
                                                                    //if the bot isn't in the fov of the enemy
                                                                    if InFieldOfVision(
                                                                        entinfo.angles.as_mut_ptr(),
                                                                        90f32,
                                                                        angles.as_mut_ptr(),
                                                                    )
                                                                        as u64
                                                                        == 0
                                                                    {
                                                                        //update some stuff for this enemy
                                                                        BotUpdateBattleInventory(
                                                                            bs, i,
                                                                        );
                                                                        //if the bot doesn't really want to fight
                                                                        if BotWantsToRetreat(bs)
                                                                            != 0
                                                                        {
                                                                            current_block_32 =
                                                                                8236137900636309791;
                                                                        } else {
                                                                            current_block_32
                                                                                =
                                                                                12497913735442871383;
                                                                        }
                                                                    } else {
                                                                        current_block_32 =
                                                                            12497913735442871383;
                                                                    }
                                                                } else {
                                                                    current_block_32 =
                                                                        12497913735442871383;
                                                                }
                                                                match current_block_32 {
                                                                    8236137900636309791 => {}
                                                                    _ => {
                                                                        //found an enemy
                                                                        (*bs).enemy =
                                                                            entinfo.number;
                                                                        if curenemy >= 0 {
                                                                            (*bs).enemysight_time
                                                                                =
                                                                                crate::src::game::ai_main::floattime
                                                                                    -
                                                                                    2f32
                                                                        } else {
                                                                            (*bs).enemysight_time
                                                                                =
                                                                                crate::src::game::ai_main::floattime
                                                                        }
                                                                        (*bs).enemysuicide
                                                                            =
                                                                            crate::src::qcommon::q_shared::qfalse
                                                                                as
                                                                                i32;
                                                                        (*bs).enemydeath_time =
                                                                            0f32;
                                                                        (*bs).enemyvisible_time
                                                                            =
                                                                            crate::src::game::ai_main::floattime;
                                                                        return crate::src::qcommon::q_shared::qtrue
                                                                                   as
                                                                                   i32;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//returns visible team mate flag carrier if available
/*
==================
BotTeamFlagCarrierVisible
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamFlagCarrierVisible(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut vis: f32 = 0.;
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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

    for i in 0..crate::src::game::g_main::level.maxclients {
        if !(i == (*bs).client) {
            //
            crate::src::game::ai_main::BotEntityInfo(i, &mut entinfo);
            //if this player is active
            if !(entinfo.valid == 0) {
                //if this player is carrying a flag
                if !(EntityCarriesFlag(&mut entinfo) as u64 == 0) {
                    //if the flag carrier is not on the same team
                    if !(BotSameTeam(bs, i) == 0) {
                        //if the flag carrier is not visible
                        vis = BotEntityVisible(
                            (*bs).entitynum,
                            (*bs).eye.as_mut_ptr(),
                            (*bs).viewangles.as_mut_ptr(),
                            360f32,
                            i,
                        );
                        if !(vis <= 0f32) {
                            //
                            return i;
                        }
                    }
                }
            }
        }
    }
    return -(1);
}
// returns the client number of the team mate flag carrier (-1 if none)
/*
==================
BotTeamFlagCarrier
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTeamFlagCarrier(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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

    for i in 0..crate::src::game::g_main::level.maxclients {
        if !(i == (*bs).client) {
            //
            crate::src::game::ai_main::BotEntityInfo(i, &mut entinfo);
            //if this player is active
            if !(entinfo.valid == 0) {
                //if this player is carrying a flag
                if !(EntityCarriesFlag(&mut entinfo) as u64 == 0) {
                    //if the flag carrier is not on the same team
                    if !(BotSameTeam(bs, i) == 0) {
                        //
                        return i;
                    }
                }
            }
        }
    }
    return -(1);
}
//returns visible enemy flag carrier if available
/*
==================
BotEnemyFlagCarrierVisible
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotEnemyFlagCarrierVisible(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut vis: f32 = 0.;
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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

    for i in 0..crate::src::game::g_main::level.maxclients {
        if !(i == (*bs).client) {
            //
            crate::src::game::ai_main::BotEntityInfo(i, &mut entinfo);
            //if this player is active
            if !(entinfo.valid == 0) {
                //if this player is carrying a flag
                if !(EntityCarriesFlag(&mut entinfo) as u64 == 0) {
                    //if the flag carrier is on the same team
                    if !(BotSameTeam(bs, i) != 0) {
                        //if the flag carrier is not visible
                        vis = BotEntityVisible(
                            (*bs).entitynum,
                            (*bs).eye.as_mut_ptr(),
                            (*bs).viewangles.as_mut_ptr(),
                            360f32,
                            i,
                        );
                        if !(vis <= 0f32) {
                            //
                            return i;
                        }
                    }
                }
            }
        }
    }
    return -(1);
}
//get the number of visible teammates and enemies
/*
==================
BotVisibleTeamMatesAndEnemies
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotVisibleTeamMatesAndEnemies(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut teammates: *mut i32,
    mut enemies: *mut i32,
    mut range: f32,
) {
    let mut i: i32 = 0;
    let mut vis: f32 = 0.;
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    if !teammates.is_null() {
        *teammates = 0
    }
    if !enemies.is_null() {
        *enemies = 0
    }
    i = 0;
    while i < crate::src::game::g_main::level.maxclients {
        if !(i == (*bs).client) {
            //
            crate::src::game::ai_main::BotEntityInfo(i, &mut entinfo);
            //if this player is active
            if !(entinfo.valid == 0) {
                //if this player is carrying a flag
                if !(EntityCarriesFlag(&mut entinfo) as u64 == 0) {
                    //if not within range
                    dir[0] = entinfo.origin[0] - (*bs).origin[0];
                    dir[1] = entinfo.origin[1] - (*bs).origin[1];
                    dir[2] = entinfo.origin[2] - (*bs).origin[2];
                    if !(VectorLengthSquared(
                        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                    ) > range * range)
                    {
                        //if the flag carrier is not visible
                        vis = BotEntityVisible(
                            (*bs).entitynum,
                            (*bs).eye.as_mut_ptr(),
                            (*bs).viewangles.as_mut_ptr(),
                            360f32,
                            i,
                        );
                        if !(vis <= 0f32) {
                            //if the flag carrier is on the same team
                            if BotSameTeam(bs, i) != 0 {
                                if !teammates.is_null() {
                                    *teammates += 1
                                }
                            } else if !enemies.is_null() {
                                *enemies += 1
                            }
                        }
                    }
                }
            }
        }
        i += 1
    }
}
//the bot will aim at the current enemy
/*
==================
BotAimAtEnemy
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAimAtEnemy(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut i: i32 = 0;
    let mut enemyvisible: i32 = 0;
    let mut dist: f32 = 0.;
    let mut f: f32 = 0.;
    let mut aim_skill: f32 = 0.;
    let mut aim_accuracy: f32 = 0.;
    let mut speed: f32 = 0.;
    let mut reactiontime: f32 = 0.;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bestorigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut groundtarget: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut cmdmove: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut enemyvelocity: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [-4f32, -4f32, -4f32];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [4f32, 4f32, 4f32];
    let mut wi: crate::be_ai_weap_h::weaponinfo_t = crate::be_ai_weap_h::weaponinfo_t {
        valid: 0,
        number: 0,
        name: [0; 80],
        model: [0; 80],
        level: 0,
        weaponindex: 0,
        flags: 0,
        projectile: [0; 80],
        numprojectiles: 0,
        hspread: 0.,
        vspread: 0.,
        speed: 0.,
        acceleration: 0.,
        recoil: [0.; 3],
        offset: [0.; 3],
        angleoffset: [0.; 3],
        extrazvelocity: 0.,
        ammoamount: 0,
        ammoindex: 0,
        activate: 0.,
        reload: 0.,
        spinup: 0.,
        spindown: 0.,
        proj: crate::be_ai_weap_h::projectileinfo_t {
            name: [0; 80],
            model: [0; 80],
            flags: 0,
            gravity: 0.,
            damage: 0,
            radius: 0.,
            visdamage: 0,
            damagetype: 0,
            healthinc: 0,
            push: 0.,
            detonation: 0.,
            bounce: 0.,
            bouncefric: 0.,
            bouncestop: 0.,
        },
    };
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    let mut trace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    let mut target: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //if the bot has no enemy
    if (*bs).enemy < 0 {
        return;
    }
    //get the enemy entity information
    crate::src::game::ai_main::BotEntityInfo((*bs).enemy, &mut entinfo);
    //if this is not a player (should be an obelisk)
    if (*bs).enemy >= 64 {
        //if the obelisk is visible
        target[0] = entinfo.origin[0];
        target[1] = entinfo.origin[1];
        target[2] = entinfo.origin[2];
        //aim at the obelisk
        dir[0] = target[0] - (*bs).eye[0];
        dir[1] = target[1] - (*bs).eye[1];
        dir[2] = target[2] - (*bs).eye[2];
        crate::src::qcommon::q_math::vectoangles(
            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            (*bs).ideal_viewangles.as_mut_ptr(),
        );
        //set the aim target before trying to attack
        (*bs).aimtarget[0] = target[0];
        (*bs).aimtarget[1] = target[1];
        (*bs).aimtarget[2] = target[2];
        return;
    }
    //
    //BotAI_Print(PRT_MESSAGE, "client %d: aiming at client %d\n", bs->entitynum, bs->enemy);
    //
    aim_skill =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 16, 0f32, 1f32);
    aim_accuracy =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 7, 0f32, 1f32);
    //
    if aim_skill as f64 > 0.95 {
        //don't aim too early
        reactiontime = (0.5
            * crate::src::game::g_syscalls::trap_Characteristic_BFloat(
                (*bs).character,
                6,
                0f32,
                1f32,
            ) as f64) as f32;
        if (*bs).enemysight_time > crate::src::game::ai_main::floattime - reactiontime {
            return;
        }
        if (*bs).teleport_time > crate::src::game::ai_main::floattime - reactiontime {
            return;
        }
    }
    //get the weapon information
    crate::src::game::g_syscalls::trap_BotGetWeaponInfo(
        (*bs).ws,
        (*bs).weaponnum,
        &mut wi as *mut crate::be_ai_weap_h::weaponinfo_t as *mut libc::c_void,
    );
    //get the weapon specific aim accuracy and or aim skill
    if wi.number == crate::bg_public_h::WP_MACHINEGUN as i32 {
        aim_accuracy =
            crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 8, 0f32, 1f32)
    } else if wi.number == crate::bg_public_h::WP_SHOTGUN as i32 {
        aim_accuracy =
            crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 9, 0f32, 1f32)
    } else if wi.number == crate::bg_public_h::WP_GRENADE_LAUNCHER as i32 {
        aim_accuracy = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            11,
            0f32,
            1f32,
        );
        aim_skill = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            18,
            0f32,
            1f32,
        )
    } else if wi.number == crate::bg_public_h::WP_ROCKET_LAUNCHER as i32 {
        aim_accuracy = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            10,
            0f32,
            1f32,
        );
        aim_skill = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            17,
            0f32,
            1f32,
        )
    } else if wi.number == crate::bg_public_h::WP_LIGHTNING as i32 {
        aim_accuracy = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            12,
            0f32,
            1f32,
        )
    } else if wi.number == crate::bg_public_h::WP_RAILGUN as i32 {
        aim_accuracy = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            14,
            0f32,
            1f32,
        )
    } else if wi.number == crate::bg_public_h::WP_PLASMAGUN as i32 {
        aim_accuracy = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            13,
            0f32,
            1f32,
        );
        aim_skill = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            19,
            0f32,
            1f32,
        )
    } else if wi.number == crate::bg_public_h::WP_BFG as i32 {
        aim_accuracy = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            15,
            0f32,
            1f32,
        );
        aim_skill = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
            (*bs).character,
            20,
            0f32,
            1f32,
        )
    }
    //
    if aim_accuracy <= 0f32 {
        aim_accuracy = 0.0001
    }
    //get the enemy entity information
    crate::src::game::ai_main::BotEntityInfo((*bs).enemy, &mut entinfo);
    //if the enemy is invisible then shoot crappy most of the time
    if EntityIsInvisible(&mut entinfo) as u64 != 0 {
        if ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 > 0.1 {
            aim_accuracy *= 0.4
        }
    }
    //
    enemyvelocity[0] = entinfo.origin[0] - entinfo.lastvisorigin[0];
    enemyvelocity[1] = entinfo.origin[1] - entinfo.lastvisorigin[1];
    enemyvelocity[2] = entinfo.origin[2] - entinfo.lastvisorigin[2];
    enemyvelocity[0] = enemyvelocity[0] * (1f32 / entinfo.update_time);
    enemyvelocity[1] = enemyvelocity[1] * (1f32 / entinfo.update_time);
    enemyvelocity[2] = enemyvelocity[2] * (1f32 / entinfo.update_time);
    //enemy origin and velocity is remembered every 0.5 seconds
    if (*bs).enemyposition_time < crate::src::game::ai_main::floattime {
        //
        (*bs).enemyposition_time = (crate::src::game::ai_main::floattime as f64 + 0.5) as f32;
        (*bs).enemyvelocity[0] = enemyvelocity[0];
        (*bs).enemyvelocity[1] = enemyvelocity[1];
        (*bs).enemyvelocity[2] = enemyvelocity[2];
        (*bs).enemyorigin[0] = entinfo.origin[0];
        (*bs).enemyorigin[1] = entinfo.origin[1];
        (*bs).enemyorigin[2] = entinfo.origin[2]
    }
    //if not extremely skilled
    if (aim_skill as f64) < 0.9 {
        dir[0] = entinfo.origin[0] - (*bs).enemyorigin[0];
        dir[1] = entinfo.origin[1] - (*bs).enemyorigin[1];
        dir[2] = entinfo.origin[2] - (*bs).enemyorigin[2];
        //if the enemy moved a bit
        if VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
            > (48i32 * 48) as f32
        {
            //if the enemy changed direction
            if (*bs).enemyvelocity[0] * enemyvelocity[0]
                + (*bs).enemyvelocity[1] * enemyvelocity[1]
                + (*bs).enemyvelocity[2] * enemyvelocity[2]
                < 0f32
            {
                //aim accuracy should be worse now
                aim_accuracy *= 0.7
            }
        }
    }
    //check visibility of enemy
    enemyvisible = BotEntityVisible(
        (*bs).entitynum,
        (*bs).eye.as_mut_ptr(),
        (*bs).viewangles.as_mut_ptr(),
        360f32,
        (*bs).enemy,
    ) as i32;
    //if the enemy is visible
    if enemyvisible != 0 {
        //
        bestorigin[0] = entinfo.origin[0];
        bestorigin[1] = entinfo.origin[1];
        bestorigin[2] = entinfo.origin[2];
        bestorigin[2] += 8f32;
        //get the start point shooting from
        //NOTE: the x and y projectile start offsets are ignored
        start[0] = (*bs).origin[0];
        start[1] = (*bs).origin[1];
        start[2] = (*bs).origin[2];
        start[2] += (*bs).cur_ps.viewheight as f32;
        start[2] += wi.offset[2];
        //
        crate::src::game::ai_main::BotAI_Trace(
            &mut trace,
            start.as_mut_ptr(),
            mins.as_mut_ptr(),
            maxs.as_mut_ptr(),
            bestorigin.as_mut_ptr(),
            (*bs).entitynum,
            1 | 0x2000000 | 0x4000000,
        );
        //if the enemy is NOT hit
        if trace.fraction <= 1f32 && trace.ent != entinfo.number {
            bestorigin[2] += 16f32
        }
        //if it is not an instant hit weapon the bot might want to predict the enemy
        if wi.speed != 0. {
            //
            dir[0] = bestorigin[0] - (*bs).origin[0];
            dir[1] = bestorigin[1] - (*bs).origin[1];
            dir[2] = bestorigin[2] - (*bs).origin[2];
            dist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
            dir[0] = entinfo.origin[0] - (*bs).enemyorigin[0];
            dir[1] = entinfo.origin[1] - (*bs).enemyorigin[1];
            dir[2] = entinfo.origin[2] - (*bs).enemyorigin[2];
            //if the enemy is NOT pretty far away and strafing just small steps left and right
            if !(dist > 100f32
                && VectorLengthSquared(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                ) < (32i32 * 32) as f32)
            {
                //if skilled enough do exact prediction
                if aim_skill as f64 > 0.8
                    && (*bs).cur_ps.weaponstate == crate::bg_public_h::WEAPON_READY as i32
                {
                    let mut move_0: crate::be_aas_h::aas_clientmove_t =
                        crate::be_aas_h::aas_clientmove_t {
                            endpos: [0.; 3],
                            endarea: 0,
                            velocity: [0.; 3],
                            trace: crate::be_aas_h::aas_trace_t {
                                startsolid: crate::src::qcommon::q_shared::qfalse,
                                fraction: 0.,
                                endpos: [0.; 3],
                                ent: 0,
                                lastarea: 0,
                                area: 0,
                                planenum: 0,
                            },
                            presencetype: 0,
                            stopevent: 0,
                            endcontents: 0,
                            time: 0.,
                            frames: 0,
                        };
                    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
                    dir[0] = entinfo.origin[0] - (*bs).origin[0];
                    dir[1] = entinfo.origin[1] - (*bs).origin[1];
                    dir[2] = entinfo.origin[2] - (*bs).origin[2];
                    //BotAI_Print(PRT_MESSAGE, "%1.1f predicted speed = %f, frames = %f\n", FloatTime(), VectorLength(dir), dist * 10 / wi.speed);
                    dist = VectorLength(
                        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                    );
                    dir[0] = entinfo.origin[0] - entinfo.lastvisorigin[0];
                    dir[1] = entinfo.origin[1] - entinfo.lastvisorigin[1];
                    dir[2] = entinfo.origin[2] - entinfo.lastvisorigin[2];
                    dir[0] = dir[0] * (1f32 / entinfo.update_time);
                    dir[1] = dir[1] * (1f32 / entinfo.update_time);
                    dir[2] = dir[2] * (1f32 / entinfo.update_time);
                    origin[0] = entinfo.origin[0];
                    origin[1] = entinfo.origin[1];
                    origin[2] = entinfo.origin[2];
                    origin[2] += 1f32;
                    cmdmove[2] = 0f32;
                    cmdmove[1] = cmdmove[2];
                    cmdmove[0] = cmdmove[1];
                    crate::src::game::g_syscalls::trap_AAS_PredictClientMovement(
                        &mut move_0 as *mut crate::be_aas_h::aas_clientmove_t as *mut libc::c_void,
                        (*bs).enemy,
                        origin.as_mut_ptr(),
                        4,
                        crate::src::qcommon::q_shared::qfalse as i32,
                        dir.as_mut_ptr(),
                        cmdmove.as_mut_ptr(),
                        0,
                        (dist * 10f32 / wi.speed) as i32,
                        0.1,
                        0,
                        0,
                        crate::src::qcommon::q_shared::qfalse as i32,
                    );
                    bestorigin[0] = move_0.endpos[0];
                    bestorigin[1] = move_0.endpos[1];
                    bestorigin[2] = move_0.endpos[2]
                } else if aim_skill as f64 > 0.4 {
                    dir[0] = entinfo.origin[0] - (*bs).origin[0];
                    dir[1] = entinfo.origin[1] - (*bs).origin[1];
                    dir[2] = entinfo.origin[2] - (*bs).origin[2];
                    //distance towards the enemy
                    //direction the enemy is moving in
                    //
                    //
                    //
                    //AAS_ClearShownDebugLines();
                    //if not that skilled do linear prediction
                    //distance towards the enemy
                    dist = VectorLength(
                        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                    );
                    //direction the enemy is moving in
                    dir[0] = entinfo.origin[0] - entinfo.lastvisorigin[0];
                    dir[1] = entinfo.origin[1] - entinfo.lastvisorigin[1];
                    dir[2] = entinfo.origin[2] - entinfo.lastvisorigin[2];
                    dir[2] = 0f32;
                    //
                    speed = crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr())
                        / entinfo.update_time;
                    //botimport.Print(PRT_MESSAGE, "speed = %f, wi->speed = %f\n", speed, wi->speed);
                    //best spot to aim at
                    bestorigin[0] = entinfo.origin[0] + dir[0] * (dist / wi.speed * speed);
                    bestorigin[1] = entinfo.origin[1] + dir[1] * (dist / wi.speed * speed);
                    bestorigin[2] = entinfo.origin[2] + dir[2] * (dist / wi.speed * speed)
                }
            }
        }
        //if the projectile does radial damage
        if aim_skill as f64 > 0.6 && wi.proj.damagetype & 2 != 0 {
            //if the enemy isn't standing significantly higher than the bot
            if entinfo.origin[2] < (*bs).origin[2] + 16f32 {
                //try to aim at the ground in front of the enemy
                end[0] = entinfo.origin[0];
                end[1] = entinfo.origin[1];
                end[2] = entinfo.origin[2];
                end[2] -= 64f32;
                crate::src::game::ai_main::BotAI_Trace(
                    &mut trace,
                    entinfo.origin.as_mut_ptr(),
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    end.as_mut_ptr(),
                    entinfo.number,
                    1 | 0x2000000 | 0x4000000,
                );
                //
                groundtarget[0] = bestorigin[0];
                groundtarget[1] = bestorigin[1];
                groundtarget[2] = bestorigin[2];
                if trace.startsolid as u64 != 0 {
                    groundtarget[2] = entinfo.origin[2] - 16f32
                } else {
                    groundtarget[2] = trace.endpos[2] - 8f32
                }
                //trace a line from projectile start to ground target
                crate::src::game::ai_main::BotAI_Trace(
                    &mut trace,
                    start.as_mut_ptr(),
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    groundtarget.as_mut_ptr(),
                    (*bs).entitynum,
                    1 | 0x2000000 | 0x4000000,
                );
                //if hitpoint is not vertically too far from the ground target
                if crate::stdlib::fabs((trace.endpos[2] - groundtarget[2]) as f64) < 50f64 {
                    dir[0] = trace.endpos[0] - groundtarget[0];
                    dir[1] = trace.endpos[1] - groundtarget[1];
                    dir[2] = trace.endpos[2] - groundtarget[2];
                    //if the hitpoint is near enough the ground target
                    if VectorLengthSquared(
                        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                    ) < (60i32 * 60) as f32
                    {
                        dir[0] = trace.endpos[0] - start[0];
                        dir[1] = trace.endpos[1] - start[1];
                        dir[2] = trace.endpos[2] - start[2];
                        //if the hitpoint is far enough from the bot
                        if VectorLengthSquared(
                            dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                        ) > (100i32 * 100) as f32
                        {
                            //check if the bot is visible from the ground target
                            trace.endpos[2] += 1f32;
                            crate::src::game::ai_main::BotAI_Trace(
                                &mut trace,
                                trace.endpos.as_mut_ptr(),
                                0 as *mut crate::src::qcommon::q_shared::vec_t,
                                0 as *mut crate::src::qcommon::q_shared::vec_t,
                                entinfo.origin.as_mut_ptr(),
                                entinfo.number,
                                1 | 0x2000000 | 0x4000000,
                            );
                            if trace.fraction >= 1f32 {
                                //botimport.Print(PRT_MESSAGE, "%1.1f aiming at ground\n", AAS_Time());
                                bestorigin[0] = groundtarget[0];
                                bestorigin[1] = groundtarget[1];
                                bestorigin[2] = groundtarget[2]
                            }
                        }
                    }
                }
            }
        }
        bestorigin[0] = (bestorigin[0] as f64
            + 20f64
                * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5))
                * (1f32 - aim_accuracy) as f64)
            as crate::src::qcommon::q_shared::vec_t;
        bestorigin[1] = (bestorigin[1] as f64
            + 20f64
                * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5))
                * (1f32 - aim_accuracy) as f64)
            as crate::src::qcommon::q_shared::vec_t;
        bestorigin[2] = (bestorigin[2] as f64
            + 10f64
                * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5))
                * (1f32 - aim_accuracy) as f64)
            as crate::src::qcommon::q_shared::vec_t
    } else {
        //
        bestorigin[0] = (*bs).lastenemyorigin[0];
        bestorigin[1] = (*bs).lastenemyorigin[1];
        bestorigin[2] = (*bs).lastenemyorigin[2];
        bestorigin[2] += 8f32;
        //if the bot is skilled enough
        if aim_skill as f64 > 0.5 {
            //do prediction shots around corners
            if wi.number == crate::bg_public_h::WP_BFG as i32
                || wi.number == crate::bg_public_h::WP_ROCKET_LAUNCHER as i32
                || wi.number == crate::bg_public_h::WP_GRENADE_LAUNCHER as i32
            {
                //create the chase goal
                goal.entitynum = (*bs).client;
                goal.areanum = (*bs).areanum;
                goal.origin[0] = (*bs).eye[0];
                goal.origin[1] = (*bs).eye[1];
                goal.origin[2] = (*bs).eye[2];
                goal.mins[0] = -8f32;
                goal.mins[1] = -8f32;
                goal.mins[2] = -8f32;
                goal.maxs[0] = 8f32;
                goal.maxs[1] = 8f32;
                goal.maxs[2] = 8f32;
                //
                if crate::src::game::g_syscalls::trap_BotPredictVisiblePosition(
                    (*bs).lastenemyorigin.as_mut_ptr(),
                    (*bs).lastenemyareanum,
                    &mut goal as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
                    0x2 | 0x4
                        | 0x8
                        | 0x10
                        | 0x20
                        | 0x80
                        | 0x100
                        | 0x200
                        | 0x400
                        | 0x800
                        | 0x80000
                        | 0x100000
                        | 0x40000
                        | 0x1000000,
                    target.as_mut_ptr(),
                ) != 0
                {
                    dir[0] = target[0] - (*bs).eye[0];
                    dir[1] = target[1] - (*bs).eye[1];
                    dir[2] = target[2] - (*bs).eye[2];
                    if VectorLengthSquared(
                        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                    ) > (80i32 * 80) as f32
                    {
                        bestorigin[0] = target[0];
                        bestorigin[1] = target[1];
                        bestorigin[2] = target[2];
                        bestorigin[2] -= 20f32
                    }
                }
                aim_accuracy = 1f32
            }
        }
    }
    //
    if enemyvisible != 0 {
        crate::src::game::ai_main::BotAI_Trace(
            &mut trace,
            (*bs).eye.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            bestorigin.as_mut_ptr(),
            (*bs).entitynum,
            1 | 0x2000000 | 0x4000000,
        );
        (*bs).aimtarget[0] = trace.endpos[0];
        (*bs).aimtarget[1] = trace.endpos[1];
        (*bs).aimtarget[2] = trace.endpos[2]
    } else {
        (*bs).aimtarget[0] = bestorigin[0];
        (*bs).aimtarget[1] = bestorigin[1];
        (*bs).aimtarget[2] = bestorigin[2]
    }
    //get aim direction
    dir[0] = bestorigin[0] - (*bs).eye[0];
    dir[1] = bestorigin[1] - (*bs).eye[1];
    dir[2] = bestorigin[2] - (*bs).eye[2];
    //
    if wi.number == crate::bg_public_h::WP_MACHINEGUN as i32
        || wi.number == crate::bg_public_h::WP_SHOTGUN as i32
        || wi.number == crate::bg_public_h::WP_LIGHTNING as i32
        || wi.number == crate::bg_public_h::WP_RAILGUN as i32
    {
        //distance towards the enemy
        dist = VectorLength(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t);
        if dist > 150f32 {
            dist = 150f32
        }
        f = (0.6 + (dist / 150f32) as f64 * 0.4) as f32;
        aim_accuracy *= f
    }
    //add some random stuff to the aim direction depending on the aim accuracy
    if (aim_accuracy as f64) < 0.8 {
        crate::src::qcommon::q_math::VectorNormalize(dir.as_mut_ptr());
        i = 0;
        while i < 3 {
            dir[i as usize] = (dir[i as usize] as f64
                + 0.3
                    * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5))
                    * (1f32 - aim_accuracy) as f64)
                as crate::src::qcommon::q_shared::vec_t;
            i += 1
        }
    }
    //set the ideal view angles
    crate::src::qcommon::q_math::vectoangles(
        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        (*bs).ideal_viewangles.as_mut_ptr(),
    );
    //take the weapon spread into account for lower skilled bots
    (*bs).ideal_viewangles[0] = ((*bs).ideal_viewangles[0] as f64
        + (6f32 * wi.vspread) as f64
            * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5))
            * (1f32 - aim_accuracy) as f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*bs).ideal_viewangles[0] = crate::src::qcommon::q_math::AngleMod((*bs).ideal_viewangles[0]);
    (*bs).ideal_viewangles[1] = ((*bs).ideal_viewangles[1] as f64
        + (6f32 * wi.hspread) as f64
            * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5))
            * (1f32 - aim_accuracy) as f64)
        as crate::src::qcommon::q_shared::vec_t;
    (*bs).ideal_viewangles[1] = crate::src::qcommon::q_math::AngleMod((*bs).ideal_viewangles[1]);
    //if the bots should be really challenging
    if bot_challenge.integer != 0 {
        //if the bot is really accurate and has the enemy in view for some time
        if aim_accuracy as f64 > 0.9
            && (*bs).enemysight_time < crate::src::game::ai_main::floattime - 1f32
        {
            //set the view angles directly
            if (*bs).ideal_viewangles[0] > 180f32 {
                (*bs).ideal_viewangles[0] -= 360f32
            }
            (*bs).viewangles[0] = (*bs).ideal_viewangles[0];
            (*bs).viewangles[1] = (*bs).ideal_viewangles[1];
            (*bs).viewangles[2] = (*bs).ideal_viewangles[2];
            crate::src::game::g_syscalls::trap_EA_View((*bs).client, (*bs).viewangles.as_mut_ptr());
        }
    };
}
//check if the bot should attack
/*
==================
BotCheckAttack
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCheckAttack(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut points: f32 = 0.;
    let mut reactiontime: f32 = 0.;
    let mut fov: f32 = 0.;
    let mut firethrottle: f32 = 0.;
    let mut attackentity: i32 = 0;
    let mut bsptrace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    //float selfpreservation;
    let mut forward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut right: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut wi: crate::be_ai_weap_h::weaponinfo_t = crate::be_ai_weap_h::weaponinfo_t {
        valid: 0,
        number: 0,
        name: [0; 80],
        model: [0; 80],
        level: 0,
        weaponindex: 0,
        flags: 0,
        projectile: [0; 80],
        numprojectiles: 0,
        hspread: 0.,
        vspread: 0.,
        speed: 0.,
        acceleration: 0.,
        recoil: [0.; 3],
        offset: [0.; 3],
        angleoffset: [0.; 3],
        extrazvelocity: 0.,
        ammoamount: 0,
        ammoindex: 0,
        activate: 0.,
        reload: 0.,
        spinup: 0.,
        spindown: 0.,
        proj: crate::be_ai_weap_h::projectileinfo_t {
            name: [0; 80],
            model: [0; 80],
            flags: 0,
            gravity: 0.,
            damage: 0,
            radius: 0.,
            visdamage: 0,
            damagetype: 0,
            healthinc: 0,
            push: 0.,
            detonation: 0.,
            bounce: 0.,
            bouncefric: 0.,
            bouncestop: 0.,
        },
    };
    let mut trace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [-8f32, -8f32, -8f32];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [8f32, 8f32, 8f32];
    attackentity = (*bs).enemy;
    //
    crate::src::game::ai_main::BotEntityInfo(attackentity, &mut entinfo);
    // if not attacking a player
    (attackentity) >= 64;
    //
    reactiontime =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 6, 0f32, 1f32);
    if (*bs).enemysight_time > crate::src::game::ai_main::floattime - reactiontime {
        return;
    }
    if (*bs).teleport_time > crate::src::game::ai_main::floattime - reactiontime {
        return;
    }
    //if changing weapons
    if (*bs).weaponchange_time as f64 > crate::src::game::ai_main::floattime as f64 - 0.1 {
        return;
    }
    //check fire throttle characteristic
    if (*bs).firethrottlewait_time > crate::src::game::ai_main::floattime {
        return;
    }
    firethrottle =
        crate::src::game::g_syscalls::trap_Characteristic_BFloat((*bs).character, 47, 0f32, 1f32);
    if (*bs).firethrottleshoot_time < crate::src::game::ai_main::floattime {
        if (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 > firethrottle {
            (*bs).firethrottlewait_time = crate::src::game::ai_main::floattime + firethrottle;
            (*bs).firethrottleshoot_time = 0f32
        } else {
            (*bs).firethrottleshoot_time =
                crate::src::game::ai_main::floattime + 1f32 - firethrottle;
            (*bs).firethrottlewait_time = 0f32
        }
    }
    //
    //
    dir[0] = (*bs).aimtarget[0] - (*bs).eye[0];
    dir[1] = (*bs).aimtarget[1] - (*bs).eye[1];
    dir[2] = (*bs).aimtarget[2] - (*bs).eye[2];
    //
    if (*bs).weaponnum == crate::bg_public_h::WP_GAUNTLET as i32 {
        if VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
            > (60i32 * 60) as f32
        {
            return;
        }
    }
    if VectorLengthSquared(dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t)
        < (100i32 * 100) as f32
    {
        fov = 120f32
    } else {
        fov = 50f32
    }
    //
    crate::src::qcommon::q_math::vectoangles(
        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        angles.as_mut_ptr(),
    );
    if InFieldOfVision((*bs).viewangles.as_mut_ptr(), fov, angles.as_mut_ptr()) as u64 == 0 {
        return;
    }
    crate::src::game::ai_main::BotAI_Trace(
        &mut bsptrace,
        (*bs).eye.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        (*bs).aimtarget.as_mut_ptr(),
        (*bs).client,
        1 | 0x10000,
    );
    if bsptrace.fraction < 1f32 && bsptrace.ent != attackentity {
        return;
    }
    //get the weapon info
    crate::src::game::g_syscalls::trap_BotGetWeaponInfo(
        (*bs).ws,
        (*bs).weaponnum,
        &mut wi as *mut crate::be_ai_weap_h::weaponinfo_t as *mut libc::c_void,
    );
    //get the start point shooting from
    start[0] = (*bs).origin[0];
    start[1] = (*bs).origin[1];
    start[2] = (*bs).origin[2];
    start[2] += (*bs).cur_ps.viewheight as f32;
    crate::src::qcommon::q_math::AngleVectors(
        (*bs).viewangles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    start[0] += forward[0] * wi.offset[0] + right[0] * wi.offset[1];
    start[1] += forward[1] * wi.offset[0] + right[1] * wi.offset[1];
    start[2] += forward[2] * wi.offset[0] + right[2] * wi.offset[1] + wi.offset[2];
    //end point aiming at
    end[0] = start[0] + forward[0] * 1000f32;
    end[1] = start[1] + forward[1] * 1000f32;
    end[2] = start[2] + forward[2] * 1000f32;
    //a little back to make sure not inside a very close enemy
    start[0] = start[0] + forward[0] * -12f32;
    start[1] = start[1] + forward[1] * -12f32;
    start[2] = start[2] + forward[2] * -12f32;
    crate::src::game::ai_main::BotAI_Trace(
        &mut trace,
        start.as_mut_ptr(),
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
        end.as_mut_ptr(),
        (*bs).entitynum,
        1 | 0x2000000 | 0x4000000,
    );
    //if the entity is a client
    if trace.ent >= 0 && trace.ent < 64 {
        if trace.ent != attackentity {
            //if a teammate is hit
            if BotSameTeam(bs, trace.ent) != 0 {
                return;
            }
        }
    }
    //if won't hit the enemy or not attacking a player (obelisk)
    if trace.ent != attackentity || attackentity >= 64 {
        //if the projectile does radial damage
        if wi.proj.damagetype & 2 != 0 {
            if (trace.fraction * 1000f32) < wi.proj.radius {
                points =
                    ((wi.proj.damage as f64 - 0.5 * trace.fraction as f64 * 1000f64) * 0.5) as f32;
                if points > 0f32 {
                    return;
                }
            }
            //FIXME: check if a teammate gets radial damage
        }
    }
    //if fire has to be release to activate weapon
    if wi.flags & 1 != 0 {
        if (*bs).flags & 2 != 0 {
            crate::src::game::g_syscalls::trap_EA_Attack((*bs).client);
        }
    } else {
        crate::src::game::g_syscalls::trap_EA_Attack((*bs).client);
    }
    (*bs).flags ^= 2;
}
//
/*
==================
BotMapScripts
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotMapScripts(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut info: [i8; 1024] = [0; 1024];
    let mut mapname: [i8; 128] = [0; 128];
    let mut i: i32 = 0;
    let mut shootbutton: i32 = 0;
    let mut aim_accuracy: f32 = 0.;
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::src::game::g_syscalls::trap_GetServerinfo(
        info.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    crate::stdlib::strncpy(
        mapname.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            info.as_mut_ptr(),
            b"mapname\x00" as *const u8 as *const i8,
        ),
        (::std::mem::size_of::<[i8; 128]>()).wrapping_sub(1usize),
    );
    mapname[(::std::mem::size_of::<[i8; 128]>()).wrapping_sub(1usize)] = '\u{0}' as i8;
    if crate::src::qcommon::q_shared::Q_stricmp(
        mapname.as_mut_ptr(),
        b"q3tourney6\x00" as *const u8 as *const i8,
    ) == 0
        || crate::src::qcommon::q_shared::Q_stricmp(
            mapname.as_mut_ptr(),
            b"q3tourney6_ctf\x00" as *const u8 as *const i8,
        ) == 0
        || crate::src::qcommon::q_shared::Q_stricmp(
            mapname.as_mut_ptr(),
            b"mpq3tourney6\x00" as *const u8 as *const i8,
        ) == 0
    {
        let mut mins: crate::src::qcommon::q_shared::vec3_t = [694f32, 200f32, 480f32];
        let mut maxs: crate::src::qcommon::q_shared::vec3_t = [968f32, 472f32, 680f32];
        let mut buttonorg: crate::src::qcommon::q_shared::vec3_t = [304f32, 352f32, 920f32];
        //NOTE: NEVER use the func_bobbing in q3tourney6
        (*bs).tfl &= !(0x1000000);
        //crush area is higher in mpq3tourney6
        if crate::src::qcommon::q_shared::Q_stricmp(
            mapname.as_mut_ptr(),
            b"mpq3tourney6\x00" as *const u8 as *const i8,
        ) == 0
        {
            mins[2] += 64f32;
            maxs[2] += 64f32
        }
        //if the bot is in the bounding box of the crush area
        if (*bs).origin[0] > mins[0] && (*bs).origin[0] < maxs[0] {
            if (*bs).origin[1] > mins[1] && (*bs).origin[1] < maxs[1] {
                if (*bs).origin[2] > mins[2] && (*bs).origin[2] < maxs[2] {
                    return;
                }
            }
        }
        shootbutton = crate::src::qcommon::q_shared::qfalse as i32;

        for i in 0..crate::src::game::g_main::level.maxclients {
            if !(i == (*bs).client) {
                //
                crate::src::game::ai_main::BotEntityInfo(i, &mut entinfo);
                //
                if !(entinfo.valid == 0) {
                    //if the enemy isn't dead and the enemy isn't the bot self
                    if !(EntityIsDead(&mut entinfo) != 0 || entinfo.number == (*bs).entitynum) {
                        //
                        if entinfo.origin[0] > mins[0] && entinfo.origin[0] < maxs[0] {
                            if entinfo.origin[1] > mins[1] && entinfo.origin[1] < maxs[1] {
                                if entinfo.origin[2] > mins[2] && entinfo.origin[2] < maxs[2] {
                                    //if there's a team mate below the crusher
                                    if BotSameTeam(bs, i) != 0 {
                                        shootbutton = crate::src::qcommon::q_shared::qfalse as i32;
                                        break;
                                    } else if (*bs).enemy == i {
                                        shootbutton = crate::src::qcommon::q_shared::qtrue as i32
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if shootbutton != 0 {
            (*bs).flags |= 32;
            dir[0] = buttonorg[0] - (*bs).eye[0];
            dir[1] = buttonorg[1] - (*bs).eye[1];
            dir[2] = buttonorg[2] - (*bs).eye[2];
            crate::src::qcommon::q_math::vectoangles(
                dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
                (*bs).ideal_viewangles.as_mut_ptr(),
            );
            aim_accuracy = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
                (*bs).character,
                7,
                0f32,
                1f32,
            );
            (*bs).ideal_viewangles[0] = ((*bs).ideal_viewangles[0] as f64
                + 8f64
                    * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5))
                    * (1f32 - aim_accuracy) as f64)
                as crate::src::qcommon::q_shared::vec_t;
            (*bs).ideal_viewangles[0] =
                crate::src::qcommon::q_math::AngleMod((*bs).ideal_viewangles[0]);
            (*bs).ideal_viewangles[1] = ((*bs).ideal_viewangles[1] as f64
                + 8f64
                    * (2.0 * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5))
                    * (1f32 - aim_accuracy) as f64)
                as crate::src::qcommon::q_shared::vec_t;
            (*bs).ideal_viewangles[1] =
                crate::src::qcommon::q_math::AngleMod((*bs).ideal_viewangles[1]);
            //
            if InFieldOfVision(
                (*bs).viewangles.as_mut_ptr(),
                20f32,
                (*bs).ideal_viewangles.as_mut_ptr(),
            ) as u64
                != 0
            {
                crate::src::game::g_syscalls::trap_EA_Attack((*bs).client);
            }
        }
    };
}
/*
==================
BotSetMovedir
==================
*/

static mut VEC_UP: crate::src::qcommon::q_shared::vec3_t = [0f32, -1f32, 0f32];

static mut MOVEDIR_UP: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 1f32];

static mut VEC_DOWN: crate::src::qcommon::q_shared::vec3_t = [0f32, -2f32, 0f32];

static mut MOVEDIR_DOWN: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, -1f32];
#[no_mangle]

pub unsafe extern "C" fn BotSetMovedir(
    mut angles: *mut crate::src::qcommon::q_shared::vec_t,
    mut movedir: *mut crate::src::qcommon::q_shared::vec_t,
) {
    if VectorCompare(
        angles as *const crate::src::qcommon::q_shared::vec_t,
        VEC_UP.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    ) != 0
    {
        *movedir.offset(0) = MOVEDIR_UP[0];
        *movedir.offset(1) = MOVEDIR_UP[1];
        *movedir.offset(2) = MOVEDIR_UP[2]
    } else if VectorCompare(
        angles as *const crate::src::qcommon::q_shared::vec_t,
        VEC_DOWN.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
    ) != 0
    {
        *movedir.offset(0) = MOVEDIR_DOWN[0];
        *movedir.offset(1) = MOVEDIR_DOWN[1];
        *movedir.offset(2) = MOVEDIR_DOWN[2]
    } else {
        crate::src::qcommon::q_math::AngleVectors(
            angles as *const crate::src::qcommon::q_shared::vec_t,
            movedir,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
        );
    };
}
/*
==================
BotModelMinsMaxs

this is ugly
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotModelMinsMaxs(
    mut modelindex: i32,
    mut eType: i32,
    mut contents: i32,
    mut mins: *mut crate::src::qcommon::q_shared::vec_t,
    mut maxs: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut i: i32 = 0;
    ent = &mut *crate::src::game::g_main::g_entities.as_mut_ptr().offset(0)
        as *mut crate::g_local_h::gentity_t;
    i = 0;
    while i < crate::src::game::g_main::level.num_entities {
        if !((*ent).inuse as u64 == 0) {
            if !(eType != 0 && (*ent).s.eType != eType) {
                if !(contents != 0 && (*ent).r.contents != contents) {
                    if (*ent).s.modelindex == modelindex {
                        if !mins.is_null() {
                            *mins.offset(0) = (*ent).r.currentOrigin[0] + (*ent).r.mins[0];
                            *mins.offset(1) = (*ent).r.currentOrigin[1] + (*ent).r.mins[1];
                            *mins.offset(2) = (*ent).r.currentOrigin[2] + (*ent).r.mins[2]
                        }
                        if !maxs.is_null() {
                            *maxs.offset(0) = (*ent).r.currentOrigin[0] + (*ent).r.maxs[0];
                            *maxs.offset(1) = (*ent).r.currentOrigin[1] + (*ent).r.maxs[1];
                            *maxs.offset(2) = (*ent).r.currentOrigin[2] + (*ent).r.maxs[2]
                        }
                        return i;
                    }
                }
            }
        }
        i += 1;
        ent = ent.offset(1)
    }
    if !mins.is_null() {
        let ref mut fresh0 = *mins.offset(2);
        *fresh0 = 0f32;
        let ref mut fresh1 = *mins.offset(1);
        *fresh1 = *fresh0;
        *mins.offset(0) = *fresh1
    }
    if !maxs.is_null() {
        let ref mut fresh2 = *maxs.offset(2);
        *fresh2 = 0f32;
        let ref mut fresh3 = *maxs.offset(1);
        *fresh3 = *fresh2;
        *maxs.offset(0) = *fresh3
    }
    return 0;
}
/*
==================
BotFuncButtonGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotFuncButtonActivateGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut bspent: i32,
    mut activategoal: *mut crate::src::game::ai_main::bot_activategoal_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut areas: [i32; 10] = [0; 10];
    let mut numareas: i32 = 0;
    let mut modelindex: i32 = 0;
    let mut entitynum: i32 = 0;
    let mut model: [i8; 128] = [0; 128];
    let mut lip: f32 = 0.;
    let mut dist: f32 = 0.;
    let mut health: f32 = 0.;
    let mut angle: f32 = 0.;
    let mut size: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut points: [crate::src::qcommon::q_shared::vec3_t; 10] = [[0.; 3]; 10];
    let mut movedir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut goalorigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bboxmins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut bboxmaxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut extramins: crate::src::qcommon::q_shared::vec3_t = [1f32, 1f32, 1f32];
    let mut extramaxs: crate::src::qcommon::q_shared::vec3_t = [-1f32, -1f32, -1f32];
    let mut bsptrace: crate::botlib_h::bsp_trace_t = crate::botlib_h::bsp_trace_t {
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
        exp_dist: 0.,
        sidenum: 0,
        surface: crate::botlib_h::bsp_surface_t {
            name: [0; 16],
            flags: 0,
            value: 0,
        },
        contents: 0,
        ent: 0,
    };
    (*activategoal).shoot = crate::src::qcommon::q_shared::qfalse as i32;
    (*activategoal).target[2] = 0f32;
    (*activategoal).target[1] = (*activategoal).target[2];
    (*activategoal).target[0] = (*activategoal).target[1];
    //create a bot goal towards the button
    crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
        bspent,
        b"model\x00" as *const u8 as *mut i8,
        model.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 128]>() as i32,
    );
    if *model.as_mut_ptr() == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    modelindex = atoi(model.as_mut_ptr().offset(1));
    if modelindex == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    entitynum = BotModelMinsMaxs(
        modelindex,
        crate::bg_public_h::ET_MOVER as i32,
        0,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    );
    //get the lip of the button
    crate::src::game::g_syscalls::trap_AAS_FloatForBSPEpairKey(
        bspent,
        b"lip\x00" as *const u8 as *mut i8,
        &mut lip,
    );
    if lip == 0. {
        lip = 4f32
    }
    //get the move direction from the angle
    crate::src::game::g_syscalls::trap_AAS_FloatForBSPEpairKey(
        bspent,
        b"angle\x00" as *const u8 as *mut i8,
        &mut angle,
    );
    angles[0] = 0f32;
    angles[1] = angle;
    angles[2] = 0f32;
    BotSetMovedir(angles.as_mut_ptr(), movedir.as_mut_ptr());
    //button size
    size[0] = maxs[0] - mins[0];
    size[1] = maxs[1] - mins[1];
    size[2] = maxs[2] - mins[2];
    //button origin
    origin[0] = mins[0] + maxs[0];
    origin[1] = mins[1] + maxs[1];
    origin[2] = mins[2] + maxs[2];
    origin[0] = (origin[0] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    origin[1] = (origin[1] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    origin[2] = (origin[2] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    //touch distance of the button
    dist = (crate::stdlib::fabs(movedir[0] as f64) * size[0] as f64
        + crate::stdlib::fabs(movedir[1] as f64) * size[1] as f64
        + crate::stdlib::fabs(movedir[2] as f64) * size[2] as f64) as f32;
    dist = (dist as f64 * 0.5) as f32;
    //
    crate::src::game::g_syscalls::trap_AAS_FloatForBSPEpairKey(
        bspent,
        b"health\x00" as *const u8 as *mut i8,
        &mut health,
    );
    //if the button is shootable
    if health != 0. {
        //calculate the shoot target
        goalorigin[0] = origin[0] + movedir[0] * -dist;
        goalorigin[1] = origin[1] + movedir[1] * -dist;
        goalorigin[2] = origin[2] + movedir[2] * -dist;
        //
        (*activategoal).target[0] = goalorigin[0];
        (*activategoal).target[1] = goalorigin[1];
        (*activategoal).target[2] = goalorigin[2];
        (*activategoal).shoot = crate::src::qcommon::q_shared::qtrue as i32;
        //
        crate::src::game::ai_main::BotAI_Trace(
            &mut bsptrace,
            (*bs).eye.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            goalorigin.as_mut_ptr(),
            (*bs).entitynum,
            1 | 0x2000000 | 0x4000000,
        );
        // if the button is visible from the current position
        if bsptrace.fraction as f64 >= 1.0 || bsptrace.ent == entitynum {
            //
            (*activategoal).goal.entitynum = entitynum; //NOTE: this is the entity number of the shootable button
            (*activategoal).goal.number = 0;
            (*activategoal).goal.flags = 0;
            (*activategoal).goal.origin[0] = (*bs).origin[0];
            (*activategoal).goal.origin[1] = (*bs).origin[1];
            (*activategoal).goal.origin[2] = (*bs).origin[2];
            (*activategoal).goal.areanum = (*bs).areanum;
            (*activategoal).goal.mins[0] = -8f32;
            (*activategoal).goal.mins[1] = -8f32;
            (*activategoal).goal.mins[2] = -8f32;
            (*activategoal).goal.maxs[0] = 8f32;
            (*activategoal).goal.maxs[1] = 8f32;
            (*activategoal).goal.maxs[2] = 8f32;
            //
            return crate::src::qcommon::q_shared::qtrue as i32;
        } else {
            //create a goal from where the button is visible and shoot at the button from there
            //add bounding box size to the dist
            crate::src::game::g_syscalls::trap_AAS_PresenceTypeBoundingBox(
                4,
                bboxmins.as_mut_ptr(),
                bboxmaxs.as_mut_ptr(),
            );
            i = 0;
            while i < 3 {
                if movedir[i as usize] < 0f32 {
                    dist = (dist as f64
                        + crate::stdlib::fabs(movedir[i as usize] as f64)
                            * crate::stdlib::fabs(bboxmaxs[i as usize] as f64))
                        as f32
                } else {
                    dist = (dist as f64
                        + crate::stdlib::fabs(movedir[i as usize] as f64)
                            * crate::stdlib::fabs(bboxmins[i as usize] as f64))
                        as f32
                }
                i += 1
            }
            //calculate the goal origin
            goalorigin[0] = origin[0] + movedir[0] * -dist;
            goalorigin[1] = origin[1] + movedir[1] * -dist;
            goalorigin[2] = origin[2] + movedir[2] * -dist;
            //
            start[0] = goalorigin[0];
            start[1] = goalorigin[1];
            start[2] = goalorigin[2];
            start[2] += 24f32;
            end[0] = start[0];
            end[1] = start[1];
            end[2] = start[2];
            end[2] -= 512f32;
            numareas = crate::src::game::g_syscalls::trap_AAS_TraceAreas(
                start.as_mut_ptr(),
                end.as_mut_ptr(),
                areas.as_mut_ptr(),
                points.as_mut_ptr(),
                10,
            );
            //
            i = numareas - 1;
            while i >= 0 {
                if crate::src::game::g_syscalls::trap_AAS_AreaReachability(areas[i as usize]) != 0 {
                    break;
                }
                i -= 1
            }
            (i) < 0;
            if i >= 0 {
                //
                (*activategoal).goal.origin[0] = points[i as usize][0];
                (*activategoal).goal.origin[1] = points[i as usize][1];
                (*activategoal).goal.origin[2] = points[i as usize][2];
                (*activategoal).goal.areanum = areas[i as usize];
                (*activategoal).goal.mins[0] = 8f32;
                (*activategoal).goal.mins[1] = 8f32;
                (*activategoal).goal.mins[2] = 8f32;
                (*activategoal).goal.maxs[0] = -8f32;
                (*activategoal).goal.maxs[1] = -8f32;
                (*activategoal).goal.maxs[2] = -8f32;
                //
                i = 0; //end for
                while i < 3 {
                    if movedir[i as usize] < 0f32 {
                        (*activategoal).goal.maxs[i as usize] =
                            ((*activategoal).goal.maxs[i as usize] as f64
                                + crate::stdlib::fabs(movedir[i as usize] as f64)
                                    * crate::stdlib::fabs(extramaxs[i as usize] as f64))
                                as crate::src::qcommon::q_shared::vec_t
                    } else {
                        (*activategoal).goal.mins[i as usize] =
                            ((*activategoal).goal.mins[i as usize] as f64
                                + crate::stdlib::fabs(movedir[i as usize] as f64)
                                    * crate::stdlib::fabs(extramins[i as usize] as f64))
                                as crate::src::qcommon::q_shared::vec_t
                    }
                    i += 1
                }
                //
                (*activategoal).goal.entitynum = entitynum;
                (*activategoal).goal.number = 0;
                (*activategoal).goal.flags = 0;
                return crate::src::qcommon::q_shared::qtrue as i32;
            }
        }
        return crate::src::qcommon::q_shared::qfalse as i32;
    } else {
        //add bounding box size to the dist
        crate::src::game::g_syscalls::trap_AAS_PresenceTypeBoundingBox(
            4,
            bboxmins.as_mut_ptr(),
            bboxmaxs.as_mut_ptr(),
        );
        i = 0;
        while i < 3 {
            if movedir[i as usize] < 0f32 {
                dist = (dist as f64
                    + crate::stdlib::fabs(movedir[i as usize] as f64)
                        * crate::stdlib::fabs(bboxmaxs[i as usize] as f64))
                    as f32
            } else {
                dist = (dist as f64
                    + crate::stdlib::fabs(movedir[i as usize] as f64)
                        * crate::stdlib::fabs(bboxmins[i as usize] as f64))
                    as f32
            }
            i += 1
        }
        //calculate the goal origin
        goalorigin[0] = origin[0] + movedir[0] * -dist;
        goalorigin[1] = origin[1] + movedir[1] * -dist;
        goalorigin[2] = origin[2] + movedir[2] * -dist;
        //
        start[0] = goalorigin[0];
        start[1] = goalorigin[1];
        start[2] = goalorigin[2];
        start[2] += 24f32;
        end[0] = start[0];
        end[1] = start[1];
        end[2] = start[2];
        end[2] -= 100f32;
        numareas = crate::src::game::g_syscalls::trap_AAS_TraceAreas(
            start.as_mut_ptr(),
            end.as_mut_ptr(),
            areas.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec3_t,
            10,
        );
        //
        i = 0;
        while i < numareas {
            if crate::src::game::g_syscalls::trap_AAS_AreaReachability(areas[i as usize]) != 0 {
                break;
            }
            i += 1
        }
        if i < numareas {
            //
            (*activategoal).goal.origin[0] = origin[0];
            (*activategoal).goal.origin[1] = origin[1];
            (*activategoal).goal.origin[2] = origin[2];
            (*activategoal).goal.areanum = areas[i as usize];
            (*activategoal).goal.mins[0] = mins[0] - origin[0];
            (*activategoal).goal.mins[1] = mins[1] - origin[1];
            (*activategoal).goal.mins[2] = mins[2] - origin[2];
            (*activategoal).goal.maxs[0] = maxs[0] - origin[0];
            (*activategoal).goal.maxs[1] = maxs[1] - origin[1];
            (*activategoal).goal.maxs[2] = maxs[2] - origin[2];
            //
            i = 0; //end for
            while i < 3 {
                if movedir[i as usize] < 0f32 {
                    (*activategoal).goal.maxs[i as usize] = ((*activategoal).goal.maxs[i as usize]
                        as f64
                        + crate::stdlib::fabs(movedir[i as usize] as f64)
                            * crate::stdlib::fabs(extramaxs[i as usize] as f64))
                        as crate::src::qcommon::q_shared::vec_t
                } else {
                    (*activategoal).goal.mins[i as usize] = ((*activategoal).goal.mins[i as usize]
                        as f64
                        + crate::stdlib::fabs(movedir[i as usize] as f64)
                            * crate::stdlib::fabs(extramins[i as usize] as f64))
                        as crate::src::qcommon::q_shared::vec_t
                }
                i += 1
            }
            //
            (*activategoal).goal.entitynum = entitynum;
            (*activategoal).goal.number = 0;
            (*activategoal).goal.flags = 0;
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
==================
BotFuncDoorGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotFuncDoorActivateGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut bspent: i32,
    mut activategoal: *mut crate::src::game::ai_main::bot_activategoal_t,
) -> i32 {
    let mut modelindex: i32 = 0;
    let mut entitynum: i32 = 0;
    let mut model: [i8; 1024] = [0; 1024];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    //shoot at the shootable door
    crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
        bspent,
        b"model\x00" as *const u8 as *mut i8,
        model.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if *model.as_mut_ptr() == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    modelindex = atoi(model.as_mut_ptr().offset(1));
    if modelindex == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    entitynum = BotModelMinsMaxs(
        modelindex,
        crate::bg_public_h::ET_MOVER as i32,
        0,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    );
    //door origin
    origin[0] = mins[0] + maxs[0];
    origin[1] = mins[1] + maxs[1];
    origin[2] = mins[2] + maxs[2];
    origin[0] = (origin[0] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    origin[1] = (origin[1] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    origin[2] = (origin[2] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    (*activategoal).target[0] = origin[0];
    (*activategoal).target[1] = origin[1];
    (*activategoal).target[2] = origin[2];
    (*activategoal).shoot = crate::src::qcommon::q_shared::qtrue as i32;
    //
    (*activategoal).goal.entitynum = entitynum; //NOTE: this is the entity number of the shootable door
    (*activategoal).goal.number = 0;
    (*activategoal).goal.flags = 0;
    (*activategoal).goal.origin[0] = (*bs).origin[0];
    (*activategoal).goal.origin[1] = (*bs).origin[1];
    (*activategoal).goal.origin[2] = (*bs).origin[2];
    (*activategoal).goal.areanum = (*bs).areanum;
    (*activategoal).goal.mins[0] = -8f32;
    (*activategoal).goal.mins[1] = -8f32;
    (*activategoal).goal.mins[2] = -8f32;
    (*activategoal).goal.maxs[0] = 8f32;
    (*activategoal).goal.maxs[1] = 8f32;
    (*activategoal).goal.maxs[2] = 8f32;
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotTriggerMultipleGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotTriggerMultipleActivateGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut bspent: i32,
    mut activategoal: *mut crate::src::game::ai_main::bot_activategoal_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut areas: [i32; 10] = [0; 10];
    let mut numareas: i32 = 0;
    let mut modelindex: i32 = 0;
    let mut entitynum: i32 = 0;
    let mut model: [i8; 128] = [0; 128];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut mins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut maxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut goalorigin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    (*activategoal).shoot = crate::src::qcommon::q_shared::qfalse as i32;
    (*activategoal).target[2] = 0f32;
    (*activategoal).target[1] = (*activategoal).target[2];
    (*activategoal).target[0] = (*activategoal).target[1];
    //create a bot goal towards the trigger
    crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
        bspent,
        b"model\x00" as *const u8 as *mut i8,
        model.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 128]>() as i32,
    );
    if *model.as_mut_ptr() == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    modelindex = atoi(model.as_mut_ptr().offset(1));
    if modelindex == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    entitynum = BotModelMinsMaxs(
        modelindex,
        0,
        0x40000000,
        mins.as_mut_ptr(),
        maxs.as_mut_ptr(),
    );
    //trigger origin
    origin[0] = mins[0] + maxs[0];
    origin[1] = mins[1] + maxs[1];
    origin[2] = mins[2] + maxs[2];
    origin[0] = (origin[0] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    origin[1] = (origin[1] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    origin[2] = (origin[2] as f64 * 0.5) as crate::src::qcommon::q_shared::vec_t;
    goalorigin[0] = origin[0];
    goalorigin[1] = origin[1];
    goalorigin[2] = origin[2];
    //
    start[0] = goalorigin[0];
    start[1] = goalorigin[1];
    start[2] = goalorigin[2];
    start[2] += 24f32;
    end[0] = start[0];
    end[1] = start[1];
    end[2] = start[2];
    end[2] -= 100f32;
    numareas = crate::src::game::g_syscalls::trap_AAS_TraceAreas(
        start.as_mut_ptr(),
        end.as_mut_ptr(),
        areas.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec3_t,
        10,
    );
    //
    i = 0;
    while i < numareas {
        if crate::src::game::g_syscalls::trap_AAS_AreaReachability(areas[i as usize]) != 0 {
            break;
        }
        i += 1
    }
    if i < numareas {
        (*activategoal).goal.origin[0] = origin[0];
        (*activategoal).goal.origin[1] = origin[1];
        (*activategoal).goal.origin[2] = origin[2];
        (*activategoal).goal.areanum = areas[i as usize];
        (*activategoal).goal.mins[0] = mins[0] - origin[0];
        (*activategoal).goal.mins[1] = mins[1] - origin[1];
        (*activategoal).goal.mins[2] = mins[2] - origin[2];
        (*activategoal).goal.maxs[0] = maxs[0] - origin[0];
        (*activategoal).goal.maxs[1] = maxs[1] - origin[1];
        (*activategoal).goal.maxs[2] = maxs[2] - origin[2];
        //
        (*activategoal).goal.entitynum = entitynum;
        (*activategoal).goal.number = 0;
        (*activategoal).goal.flags = 0;
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//pop an activate goal from the stack
/*
==================
BotPopFromActivateGoalStack
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotPopFromActivateGoalStack(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) -> i32 {
    if (*bs).activatestack.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    BotEnableActivateGoalAreas(
        (*bs).activatestack,
        crate::src::qcommon::q_shared::qtrue as i32,
    );
    (*(*bs).activatestack).inuse = crate::src::qcommon::q_shared::qfalse as i32;
    (*(*bs).activatestack).justused_time = crate::src::game::ai_main::floattime;
    (*bs).activatestack = (*(*bs).activatestack).next;
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotPushOntoActivateGoalStack
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotPushOntoActivateGoalStack(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut activategoal: *mut crate::src::game::ai_main::bot_activategoal_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut best: i32 = 0;
    let mut besttime: f32 = 0.;
    best = -(1);
    besttime = crate::src::game::ai_main::floattime + 9999f32;

    for i in 0..8 {
        if (*bs).activategoalheap[i as usize].inuse == 0 {
            if (*bs).activategoalheap[i as usize].justused_time < besttime {
                besttime = (*bs).activategoalheap[i as usize].justused_time;
                best = i
            }
        }
    }
    if best != -(1) {
        crate::stdlib::memcpy(
            &mut *(*bs).activategoalheap.as_mut_ptr().offset(best as isize)
                as *mut crate::src::game::ai_main::bot_activategoal_t
                as *mut libc::c_void,
            activategoal as *const libc::c_void,
            ::std::mem::size_of::<crate::src::game::ai_main::bot_activategoal_t>(),
        );
        (*bs).activategoalheap[best as usize].inuse = crate::src::qcommon::q_shared::qtrue as i32;
        (*bs).activategoalheap[best as usize].next = (*bs).activatestack;
        (*bs).activatestack = &mut *(*bs).activategoalheap.as_mut_ptr().offset(best as isize)
            as *mut crate::src::game::ai_main::bot_activategoal_t;
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//clear the activate goal stack
/*
==================
BotClearActivateGoalStack
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotClearActivateGoalStack(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    while !(*bs).activatestack.is_null() {
        BotPopFromActivateGoalStack(bs);
    }
}
//enable or disable the areas the blocking entity is in
/*
==================
BotEnableActivateGoalAreas
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotEnableActivateGoalAreas(
    mut activategoal: *mut crate::src::game::ai_main::bot_activategoal_t,
    mut enable: i32,
) {
    let mut i: i32 = 0;
    if (*activategoal).areasdisabled == (enable == 0) as i32 {
        return;
    }

    for i in 0..(*activategoal).numareas {
        crate::src::game::g_syscalls::trap_AAS_EnableRoutingArea(
            (*activategoal).areas[i as usize],
            enable,
        );
    }
    (*activategoal).areasdisabled = (enable == 0) as i32;
}
/*
==================
BotIsGoingToActivateEntity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotIsGoingToActivateEntity(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut entitynum: i32,
) -> i32 {
    let mut a: *mut crate::src::game::ai_main::bot_activategoal_t =
        0 as *mut crate::src::game::ai_main::bot_activategoal_t;
    let mut i: i32 = 0;
    a = (*bs).activatestack;
    while !a.is_null() {
        if !((*a).time < crate::src::game::ai_main::floattime) {
            if (*a).goal.entitynum == entitynum {
                return crate::src::qcommon::q_shared::qtrue as i32;
            }
        }
        a = (*a).next
    }

    for i in 0..8 {
        if !((*bs).activategoalheap[i as usize].inuse != 0) {
            //
            if (*bs).activategoalheap[i as usize].goal.entitynum == entitynum {
                // if the bot went for this goal less than 2 seconds ago
                if (*bs).activategoalheap[i as usize].justused_time
                    > crate::src::game::ai_main::floattime - 2f32
                {
                    return crate::src::qcommon::q_shared::qtrue as i32;
                }
            }
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
==================
BotGetActivateGoal

  returns the number of the bsp entity to activate
  goal->entitynum will be set to the game entity to activate
==================
*/
//#define OBSTACLEDEBUG
#[no_mangle]

pub unsafe extern "C" fn BotGetActivateGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut entitynum: i32,
    mut activategoal: *mut crate::src::game::ai_main::bot_activategoal_t,
) -> i32 {
    let mut i: i32 = 0;
    let mut ent: i32 = 0;
    let mut cur_entities: [i32; 10] = [0; 10];
    let mut spawnflags: i32 = 0;
    let mut modelindex: i32 = 0;
    let mut areas: [i32; 64] = [0; 64];
    let mut numareas: i32 = 0;
    let mut t: i32 = 0;
    let mut model: [i8; 1024] = [0; 1024];
    let mut tmpmodel: [i8; 128] = [0; 128];
    let mut target: [i8; 128] = [0; 128];
    let mut classname: [i8; 128] = [0; 128];
    let mut health: f32 = 0.;
    let mut targetname: [[i8; 128]; 10] = [[0; 128]; 10];
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    let mut areainfo: crate::be_aas_h::aas_areainfo_t = crate::be_aas_h::aas_areainfo_t {
        contents: 0,
        flags: 0,
        presencetype: 0,
        cluster: 0,
        mins: [0.; 3],
        maxs: [0.; 3],
        center: [0.; 3],
    };
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut absmins: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut absmaxs: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    crate::stdlib::memset(
        activategoal as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::game::ai_main::bot_activategoal_t>(),
    );
    crate::src::game::ai_main::BotEntityInfo(entitynum, &mut entinfo);
    crate::src::qcommon::q_shared::Com_sprintf(
        model.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
        b"*%d\x00" as *const u8 as *const i8,
        entinfo.modelindex,
    );
    ent = crate::src::game::g_syscalls::trap_AAS_NextBSPEntity(0);
    while ent != 0 {
        if !(crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
            ent,
            b"model\x00" as *const u8 as *mut i8,
            tmpmodel.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 128]>() as i32,
        ) == 0)
        {
            if crate::stdlib::strcmp(model.as_mut_ptr(), tmpmodel.as_mut_ptr()) == 0 {
                break;
            }
        }
        ent = crate::src::game::g_syscalls::trap_AAS_NextBSPEntity(ent)
    }
    if ent == 0 {
        crate::src::game::ai_main::BotAI_Print(
            3,
            b"BotGetActivateGoal: no entity found with model %s\n\x00" as *const u8 as *mut i8,
            model.as_mut_ptr(),
        );
        return 0i32;
    }
    crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
        ent,
        b"classname\x00" as *const u8 as *mut i8,
        classname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 128]>() as i32,
    );
    if *classname.as_mut_ptr() == 0 {
        crate::src::game::ai_main::BotAI_Print(
            3,
            b"BotGetActivateGoal: entity with model %s has no classname\n\x00" as *const u8
                as *mut i8,
            model.as_mut_ptr(),
        );
        return 0i32;
    }
    //if it is a door
    if crate::stdlib::strcmp(
        classname.as_mut_ptr(),
        b"func_door\x00" as *const u8 as *const i8,
    ) == 0
    {
        if crate::src::game::g_syscalls::trap_AAS_FloatForBSPEpairKey(
            ent,
            b"health\x00" as *const u8 as *mut i8,
            &mut health,
        ) != 0
        {
            //if the door has health then the door must be shot to open
            if health != 0. {
                BotFuncDoorActivateGoal(bs, ent, activategoal);
                return ent;
            }
        }
        //
        crate::src::game::g_syscalls::trap_AAS_IntForBSPEpairKey(
            ent,
            b"spawnflags\x00" as *const u8 as *mut i8,
            &mut spawnflags,
        );
        // if the door starts open then just wait for the door to return
        if spawnflags & 1 != 0 {
            return 0i32;
        }
        //get the door origin
        if crate::src::game::g_syscalls::trap_AAS_VectorForBSPEpairKey(
            ent,
            b"origin\x00" as *const u8 as *mut i8,
            origin.as_mut_ptr(),
        ) == 0
        {
            origin[2] = 0f32;
            origin[1] = origin[2];
            origin[0] = origin[1]
        }
        //if the door is open or opening already
        if VectorCompare(
            origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            entinfo.origin.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        ) == 0
        {
            return 0i32;
        }
        // store all the areas the door is in
        crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
            ent,
            b"model\x00" as *const u8 as *mut i8,
            model.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        if *model.as_mut_ptr() != 0 {
            modelindex = atoi(model.as_mut_ptr().offset(1));
            if modelindex != 0 {
                BotModelMinsMaxs(
                    modelindex,
                    crate::bg_public_h::ET_MOVER as i32,
                    0,
                    absmins.as_mut_ptr(),
                    absmaxs.as_mut_ptr(),
                );
                //
                numareas = crate::src::game::g_syscalls::trap_AAS_BBoxAreas(
                    absmins.as_mut_ptr(),
                    absmaxs.as_mut_ptr(),
                    areas.as_mut_ptr(),
                    32 * 2,
                );
                // store the areas with reachabilities first
                i = 0;
                while i < numareas {
                    if (*activategoal).numareas >= 32 {
                        break;
                    }
                    if !(crate::src::game::g_syscalls::trap_AAS_AreaReachability(areas[i as usize])
                        == 0)
                    {
                        crate::src::game::g_syscalls::trap_AAS_AreaInfo(
                            areas[i as usize],
                            &mut areainfo as *mut crate::be_aas_h::aas_areainfo_t
                                as *mut libc::c_void,
                        );
                        if areainfo.contents & 1024 != 0 {
                            let fresh4 = (*activategoal).numareas;
                            (*activategoal).numareas = (*activategoal).numareas + 1;
                            (*activategoal).areas[fresh4 as usize] = areas[i as usize]
                        }
                    }
                    i += 1
                }
                // store any remaining areas
                i = 0;
                while i < numareas {
                    if (*activategoal).numareas >= 32 {
                        break;
                    }
                    if !(crate::src::game::g_syscalls::trap_AAS_AreaReachability(areas[i as usize])
                        != 0)
                    {
                        crate::src::game::g_syscalls::trap_AAS_AreaInfo(
                            areas[i as usize],
                            &mut areainfo as *mut crate::be_aas_h::aas_areainfo_t
                                as *mut libc::c_void,
                        );
                        if areainfo.contents & 1024 != 0 {
                            let fresh5 = (*activategoal).numareas;
                            (*activategoal).numareas = (*activategoal).numareas + 1;
                            (*activategoal).areas[fresh5 as usize] = areas[i as usize]
                        }
                    }
                    i += 1
                }
            }
        }
    }
    // if the bot is blocked by or standing on top of a button
    if crate::stdlib::strcmp(
        classname.as_mut_ptr(),
        b"func_button\x00" as *const u8 as *const i8,
    ) == 0
    {
        return 0i32;
    }
    // get the targetname so we can find an entity with a matching target
    if crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
        ent,
        b"targetname\x00" as *const u8 as *mut i8,
        targetname[0].as_mut_ptr(),
        ::std::mem::size_of::<[i8; 128]>() as i32,
    ) == 0
    {
        if bot_developer.integer != 0 {
            crate::src::game::ai_main::BotAI_Print(
                3i32,
                b"BotGetActivateGoal: entity with model \"%s\" has no targetname\n\x00" as *const u8
                    as *mut i8,
                model.as_mut_ptr(),
            );
        }
        return 0i32;
    }
    // allow tree-like activation
    cur_entities[0] = crate::src::game::g_syscalls::trap_AAS_NextBSPEntity(0);
    i = 0;
    // just skip the func_timer
    while i >= 0 && i < 10 {
        ent = cur_entities[i as usize];
        while ent != 0 {
            if !(crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
                ent,
                b"target\x00" as *const u8 as *mut i8,
                target.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 128]>() as i32,
            ) == 0)
            {
                if crate::stdlib::strcmp(targetname[i as usize].as_mut_ptr(), target.as_mut_ptr())
                    == 0
                {
                    cur_entities[i as usize] =
                        crate::src::game::g_syscalls::trap_AAS_NextBSPEntity(ent);
                    break;
                }
            }
            ent = crate::src::game::g_syscalls::trap_AAS_NextBSPEntity(ent)
        }
        if ent == 0 {
            if bot_developer.integer != 0 {
                crate::src::game::ai_main::BotAI_Print(
                    3i32,
                    b"BotGetActivateGoal: no entity with target \"%s\"\n\x00" as *const u8
                        as *mut i8,
                    targetname[i as usize].as_mut_ptr(),
                );
            }
            i -= 1
        } else if crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *mut i8,
            classname.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 128]>() as i32,
        ) == 0
        {
            if bot_developer.integer != 0 {
                crate::src::game::ai_main::BotAI_Print(
                    3i32,
                    b"BotGetActivateGoal: entity with target \"%s\" has no classname\n\x00"
                        as *const u8 as *mut i8,
                    targetname[i as usize].as_mut_ptr(),
                );
            }
        } else if crate::stdlib::strcmp(
            classname.as_mut_ptr(),
            b"func_button\x00" as *const u8 as *const i8,
        ) == 0
        {
            // BSP button model
            //
            if BotFuncButtonActivateGoal(bs, ent, activategoal) == 0 {
                continue;
            }
            // if the bot tries to activate this button already
            if !(*bs).activatestack.is_null()
                && (*(*bs).activatestack).inuse != 0
                && (*(*bs).activatestack).goal.entitynum == (*activategoal).goal.entitynum
                && (*(*bs).activatestack).time > crate::src::game::ai_main::floattime
                && (*(*bs).activatestack).start_time < crate::src::game::ai_main::floattime - 2f32
            {
                continue;
            }
            // if the bot is in a reachability area
            if crate::src::game::g_syscalls::trap_AAS_AreaReachability((*bs).areanum) != 0 {
                // disable all areas the blocking entity is in
                BotEnableActivateGoalAreas(
                    activategoal,
                    crate::src::qcommon::q_shared::qfalse as i32,
                );
                //
                t = crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
                    (*bs).areanum,
                    (*bs).origin.as_mut_ptr(),
                    (*activategoal).goal.areanum,
                    (*bs).tfl,
                );
                // if the button is not reachable
                if t == 0 {
                    continue;
                }
                (*activategoal).time =
                    (crate::src::game::ai_main::floattime as f64 + t as f64 * 0.01 + 5f64) as f32
            }
            return ent;
        } else if crate::stdlib::strcmp(
            classname.as_mut_ptr(),
            b"trigger_multiple\x00" as *const u8 as *const i8,
        ) == 0
        {
            // invisible trigger multiple box
            //
            if BotTriggerMultipleActivateGoal(bs, ent, activategoal) == 0 {
                continue;
            }
            // if the bot tries to activate this trigger already
            if !(*bs).activatestack.is_null()
                && (*(*bs).activatestack).inuse != 0
                && (*(*bs).activatestack).goal.entitynum == (*activategoal).goal.entitynum
                && (*(*bs).activatestack).time > crate::src::game::ai_main::floattime
                && (*(*bs).activatestack).start_time < crate::src::game::ai_main::floattime - 2f32
            {
                continue;
            }
            // if the bot is in a reachability area
            if crate::src::game::g_syscalls::trap_AAS_AreaReachability((*bs).areanum) != 0 {
                // disable all areas the blocking entity is in
                BotEnableActivateGoalAreas(
                    activategoal,
                    crate::src::qcommon::q_shared::qfalse as i32,
                );
                //
                t = crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
                    (*bs).areanum,
                    (*bs).origin.as_mut_ptr(),
                    (*activategoal).goal.areanum,
                    (*bs).tfl,
                );
                // if the trigger is not reachable
                if t == 0 {
                    continue;
                }
                (*activategoal).time =
                    (crate::src::game::ai_main::floattime as f64 + t as f64 * 0.01 + 5f64) as f32
            }
            return ent;
        } else {
            if crate::stdlib::strcmp(
                classname.as_mut_ptr(),
                b"func_timer\x00" as *const u8 as *const i8,
            ) == 0
            {
                continue;
            }
            // the actual button or trigger might be linked through a target_relay or target_delay
            if crate::stdlib::strcmp(
                classname.as_mut_ptr(),
                b"target_relay\x00" as *const u8 as *const i8,
            ) == 0
                || crate::stdlib::strcmp(
                    classname.as_mut_ptr(),
                    b"target_delay\x00" as *const u8 as *const i8,
                ) == 0
            {
                if crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
                    ent,
                    b"targetname\x00" as *const u8 as *mut i8,
                    targetname[(i + 1) as usize].as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 128]>() as i32,
                ) != 0
                {
                    i += 1;
                    cur_entities[i as usize] =
                        crate::src::game::g_syscalls::trap_AAS_NextBSPEntity(0)
                }
            }
        }
    }
    return 0;
}
/*
==================
BotGoForActivateGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGoForActivateGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut activategoal: *mut crate::src::game::ai_main::bot_activategoal_t,
) -> i32 {
    let mut activateinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    (*activategoal).inuse = crate::src::qcommon::q_shared::qtrue as i32;
    if (*activategoal).time == 0. {
        (*activategoal).time = crate::src::game::ai_main::floattime + 10f32
    }
    (*activategoal).start_time = crate::src::game::ai_main::floattime;
    crate::src::game::ai_main::BotEntityInfo((*activategoal).goal.entitynum, &mut activateinfo);
    (*activategoal).origin[0] = activateinfo.origin[0];
    (*activategoal).origin[1] = activateinfo.origin[1];
    (*activategoal).origin[2] = activateinfo.origin[2];
    //
    if BotPushOntoActivateGoalStack(bs, activategoal) != 0 {
        // enter the activate entity AI node
        crate::src::game::ai_dmnet::AIEnter_Seek_ActivateEntity(
            bs,
            b"BotGoForActivateGoal\x00" as *const u8 as *mut i8,
        );
        return crate::src::qcommon::q_shared::qtrue as i32;
    } else {
        // enable any routing areas that were disabled
        BotEnableActivateGoalAreas(activategoal, crate::src::qcommon::q_shared::qtrue as i32);
        return crate::src::qcommon::q_shared::qfalse as i32;
    };
}
/*
==================
BotPrintActivateGoalInfo
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotPrintActivateGoalInfo(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut activategoal: *mut crate::src::game::ai_main::bot_activategoal_t,
    mut bspent: i32,
) {
    let mut netname: [i8; 36] = [0; 36];
    let mut classname: [i8; 128] = [0; 128];
    let mut buf: [i8; 128] = [0; 128];
    ClientName(
        (*bs).client,
        netname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 36]>() as i32,
    );
    crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
        bspent,
        b"classname\x00" as *const u8 as *mut i8,
        classname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 128]>() as i32,
    );
    if (*activategoal).shoot != 0 {
        crate::src::qcommon::q_shared::Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 128]>() as i32,
            b"%s: I have to shoot at a %s from %1.1f %1.1f %1.1f in area %d\n\x00" as *const u8
                as *const i8,
            netname.as_mut_ptr(),
            classname.as_mut_ptr(),
            (*activategoal).goal.origin[0usize] as f64,
            (*activategoal).goal.origin[1usize] as f64,
            (*activategoal).goal.origin[2usize] as f64,
            (*activategoal).goal.areanum,
        );
    } else {
        crate::src::qcommon::q_shared::Com_sprintf(
            buf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 128]>() as i32,
            b"%s: I have to activate a %s at %1.1f %1.1f %1.1f in area %d\n\x00" as *const u8
                as *const i8,
            netname.as_mut_ptr(),
            classname.as_mut_ptr(),
            (*activategoal).goal.origin[0usize] as f64,
            (*activategoal).goal.origin[1usize] as f64,
            (*activategoal).goal.origin[2usize] as f64,
            (*activategoal).goal.areanum,
        );
    }
    crate::src::game::g_syscalls::trap_EA_Say((*bs).client, buf.as_mut_ptr());
}
/*
==================
BotRandomMove
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotRandomMove(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut moveresult: *mut crate::be_ai_move_h::bot_moveresult_t,
) {
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    angles[0] = 0f32;
    angles[1] = (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * 360f32;
    angles[2] = 0f32;
    crate::src::qcommon::q_math::AngleVectors(
        angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        dir.as_mut_ptr(),
        0 as *mut crate::src::qcommon::q_shared::vec_t,
        0 as *mut crate::src::qcommon::q_shared::vec_t,
    );
    crate::src::game::g_syscalls::trap_BotMoveInDirection((*bs).ms, dir.as_mut_ptr(), 400f32, 1);
    (*moveresult).failure = crate::src::qcommon::q_shared::qfalse as i32;
    (*moveresult).movedir[0] = dir[0];
    (*moveresult).movedir[1] = dir[1];
    (*moveresult).movedir[2] = dir[2];
}
//AI when the bot is blocked
/*
==================
BotAIBlocked

Very basic handling of bots being blocked by other entities.
Check what kind of entity is blocking the bot and try to activate
it. If that's not an option then try to walk around or over the entity.
Before the bot ends in this part of the AI it should predict which doors to
open, which buttons to activate etc.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAIBlocked(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut moveresult: *mut crate::be_ai_move_h::bot_moveresult_t,
    mut activate: i32,
) {
    let mut movetype: i32 = 0;
    let mut bspent: i32 = 0;
    let mut hordir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut sideward: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut angles: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut up: crate::src::qcommon::q_shared::vec3_t = [0f32, 0f32, 1f32];
    //vec3_t start, end, mins, maxs;
    let mut entinfo: crate::be_aas_h::aas_entityinfo_t = crate::be_aas_h::aas_entityinfo_t {
        valid: 0,
        type_0: 0,
        flags: 0,
        ltime: 0.,
        update_time: 0.,
        number: 0,
        origin: [0.; 3],
        angles: [0.; 3],
        old_origin: [0.; 3],
        lastvisorigin: [0.; 3],
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
    let mut activategoal: crate::src::game::ai_main::bot_activategoal_t =
        crate::src::game::ai_main::bot_activategoal_t {
            inuse: 0,
            goal: crate::be_ai_goal_h::bot_goal_t {
                origin: [0.; 3],
                areanum: 0,
                mins: [0.; 3],
                maxs: [0.; 3],
                entitynum: 0,
                number: 0,
                flags: 0,
                iteminfo: 0,
            },
            time: 0.,
            start_time: 0.,
            justused_time: 0.,
            shoot: 0,
            weapon: 0,
            target: [0.; 3],
            origin: [0.; 3],
            areas: [0; 32],
            numareas: 0,
            areasdisabled: 0,
            next: 0 as *mut crate::src::game::ai_main::bot_activategoal_s,
        };
    // if the bot is not blocked by anything
    if (*moveresult).blocked == 0 {
        (*bs).notblocked_time = crate::src::game::ai_main::floattime;
        return;
    }
    // if stuck in a solid area
    if (*moveresult).type_0 == 8 {
        // move in a random direction in the hope to get out
        BotRandomMove(bs, moveresult);
        //
        return;
    }
    // get info for the entity that is blocking the bot
    crate::src::game::ai_main::BotEntityInfo((*moveresult).blockentity, &mut entinfo);
    // OBSTACLEDEBUG
    // if blocked by a bsp model and the bot wants to activate it
    if activate != 0 && entinfo.modelindex > 0 && entinfo.modelindex <= max_bspmodelindex {
        // find the bsp entity which should be activated in order to get the blocking entity out of the way
        bspent = BotGetActivateGoal(bs, entinfo.number, &mut activategoal);
        if bspent != 0 {
            //
            if !(*bs).activatestack.is_null() && (*(*bs).activatestack).inuse == 0 {
                (*bs).activatestack = 0 as *mut crate::src::game::ai_main::bot_activategoal_t
            }
            // if not already trying to activate this entity
            if BotIsGoingToActivateEntity(bs, activategoal.goal.entitynum) == 0 {
                //
                BotGoForActivateGoal(bs, &mut activategoal);
            }
            // if ontop of an obstacle or
            // if the bot is not in a reachability area it'll still
            // need some dynamic obstacle avoidance, otherwise return
            if (*moveresult).flags & 32 == 0
                && crate::src::game::g_syscalls::trap_AAS_AreaReachability((*bs).areanum) != 0
            {
                return;
            }
        } else {
            // enable any routing areas that were disabled
            BotEnableActivateGoalAreas(
                &mut activategoal,
                crate::src::qcommon::q_shared::qtrue as i32,
            );
        }
    }
    // just some basic dynamic obstacle avoidance code
    hordir[0] = (*moveresult).movedir[0];
    hordir[1] = (*moveresult).movedir[1];
    hordir[2] = 0f32;
    // if no direction just take a random direction
    if (crate::src::qcommon::q_math::VectorNormalize(hordir.as_mut_ptr()) as f64) < 0.1 {
        angles[0] = 0f32;
        angles[1] = 360f32 * ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32);
        angles[2] = 0f32;
        crate::src::qcommon::q_math::AngleVectors(
            angles.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
            hordir.as_mut_ptr(),
            0 as *mut crate::src::qcommon::q_shared::vec_t,
            0 as *mut crate::src::qcommon::q_shared::vec_t,
        );
    }
    //
    //if (moveresult->flags & MOVERESULT_ONTOPOFOBSTACLE) movetype = MOVE_JUMP;
    //else
    movetype = 1;
    // if there's an obstacle at the bot's feet and head then
    // the bot might be able to crouch through
    //VectorCopy(bs->origin, start);
    //start[2] += 18;
    //VectorMA(start, 5, hordir, end);
    //VectorSet(mins, -16, -16, -24);
    //VectorSet(maxs, 16, 16, 4);
    //
    //bsptrace = AAS_Trace(start, mins, maxs, end, bs->entitynum, MASK_PLAYERSOLID);
    //if (bsptrace.fraction >= 1) movetype = MOVE_CROUCH;
    // get the sideward vector
    CrossProduct(
        hordir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        up.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t,
        sideward.as_mut_ptr(),
    );
    //
    if (*bs).flags & 16 != 0 {
        sideward[0] = -sideward[0];
        sideward[1] = -sideward[1];
        sideward[2] = -sideward[2]
    }
    // try to crouch straight forward?
    if movetype != 2
        || crate::src::game::g_syscalls::trap_BotMoveInDirection(
            (*bs).ms,
            hordir.as_mut_ptr(),
            400f32,
            movetype,
        ) == 0
    {
        // perform the movement
        if crate::src::game::g_syscalls::trap_BotMoveInDirection(
            (*bs).ms,
            sideward.as_mut_ptr(),
            400f32,
            movetype,
        ) == 0
        {
            // flip the avoid direction flag
            (*bs).flags ^= 16;
            // flip the direction
            // VectorNegate(sideward, sideward);
            sideward[0] = sideward[0] + hordir[0] * -1f32;
            sideward[1] = sideward[1] + hordir[1] * -1f32;
            sideward[2] = sideward[2] + hordir[2] * -1f32;
            // move in the other direction
            crate::src::game::g_syscalls::trap_BotMoveInDirection(
                (*bs).ms,
                sideward.as_mut_ptr(),
                400f32,
                movetype,
            );
        }
    }
    //
    if ((*bs).notblocked_time as f64) < crate::src::game::ai_main::floattime as f64 - 0.4 {
        // just reset goals and hope the bot will go into another direction?
        // is this still needed??
        if (*bs).ainode
            == Some(
                crate::src::game::ai_dmnet::AINode_Seek_NBG
                    as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
            )
        {
            (*bs).nbg_time = 0f32
        } else if (*bs).ainode
            == Some(
                crate::src::game::ai_dmnet::AINode_Seek_LTG
                    as unsafe extern "C" fn(_: *mut crate::src::game::ai_main::bot_state_t) -> i32,
            )
        {
            (*bs).ltg_time = 0f32
        }
    };
}
//AI to predict obstacles
/*
==================
BotAIPredictObstacles

Predict the route towards the goal and check if the bot
will be blocked by certain obstacles. When the bot has obstacles
on its path the bot should figure out if they can be removed
by activating certain entities.
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAIPredictObstacles(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> i32 {
    let mut modelnum: i32 = 0;
    let mut entitynum: i32 = 0;
    let mut bspent: i32 = 0;
    let mut activategoal: crate::src::game::ai_main::bot_activategoal_t =
        crate::src::game::ai_main::bot_activategoal_t {
            inuse: 0,
            goal: crate::be_ai_goal_h::bot_goal_t {
                origin: [0.; 3],
                areanum: 0,
                mins: [0.; 3],
                maxs: [0.; 3],
                entitynum: 0,
                number: 0,
                flags: 0,
                iteminfo: 0,
            },
            time: 0.,
            start_time: 0.,
            justused_time: 0.,
            shoot: 0,
            weapon: 0,
            target: [0.; 3],
            origin: [0.; 3],
            areas: [0; 32],
            numareas: 0,
            areasdisabled: 0,
            next: 0 as *mut crate::src::game::ai_main::bot_activategoal_s,
        };
    let mut route: crate::be_aas_h::aas_predictroute_t = crate::be_aas_h::aas_predictroute_t {
        endpos: [0.; 3],
        endarea: 0,
        stopevent: 0,
        endcontents: 0,
        endtravelflags: 0,
        numareas: 0,
        time: 0,
    };
    if bot_predictobstacles.integer == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    // always predict when the goal change or at regular intervals
    if (*bs).predictobstacles_goalareanum == (*goal).areanum
        && (*bs).predictobstacles_time > crate::src::game::ai_main::floattime - 6f32
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    (*bs).predictobstacles_goalareanum = (*goal).areanum;
    (*bs).predictobstacles_time = crate::src::game::ai_main::floattime;
    // predict at most 100 areas or 1 second ahead
    crate::src::game::g_syscalls::trap_AAS_PredictRoute(
        &mut route as *mut crate::be_aas_h::aas_predictroute_t as *mut libc::c_void,
        (*bs).areanum,
        (*bs).origin.as_mut_ptr(),
        (*goal).areanum,
        (*bs).tfl,
        100,
        1000,
        2 | 4,
        1024,
        0x4000000,
        0,
    );
    // if bot has to travel through an area with a mover
    if route.stopevent & 4 != 0 {
        // if the bot will run into a mover
        if route.endcontents & 1024 != 0 {
            //NOTE: this only works with bspc 2.1 or higher
            modelnum = (route.endcontents & (0xff) << 24) >> 24;
            if modelnum != 0 {
                //
                entitynum = BotModelMinsMaxs(
                    modelnum,
                    crate::bg_public_h::ET_MOVER as i32,
                    0,
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                    0 as *mut crate::src::qcommon::q_shared::vec_t,
                );
                if entitynum != 0 {
                    //NOTE: BotGetActivateGoal already checks if the door is open or not
                    bspent = BotGetActivateGoal(bs, entitynum, &mut activategoal);
                    if bspent != 0 {
                        //
                        if !(*bs).activatestack.is_null() && (*(*bs).activatestack).inuse == 0 {
                            (*bs).activatestack =
                                0 as *mut crate::src::game::ai_main::bot_activategoal_t
                        }
                        // if not already trying to activate this entity
                        if BotIsGoingToActivateEntity(bs, activategoal.goal.entitynum) == 0 {
                            //
                            //BotAI_Print(PRT_MESSAGE, "blocked by mover model %d, entity %d ?\n", modelnum, entitynum);
                            //
                            BotGoForActivateGoal(bs, &mut activategoal);
                            return crate::src::qcommon::q_shared::qtrue as i32;
                        } else {
                            // enable any routing areas that were disabled
                            BotEnableActivateGoalAreas(
                                &mut activategoal,
                                crate::src::qcommon::q_shared::qtrue as i32,
                            );
                        }
                    }
                }
            }
        }
    } else if route.stopevent & 2 != 0 {
        (route.endtravelflags & 0x4000000i32) != 0;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
/*
==================
BotCheckConsoleMessages
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCheckConsoleMessages(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
) {
    let mut botname: [i8; 36] = [0; 36];
    let mut message: [i8; 256] = [0; 256];
    let mut netname: [i8; 36] = [0; 36];
    let mut ptr: *mut i8 = 0 as *mut i8;
    let mut chat_reply: f32 = 0.;
    let mut context: i32 = 0;
    let mut handle: i32 = 0;
    let mut m: crate::be_ai_chat_h::bot_consolemessage_t =
        crate::be_ai_chat_h::bot_consolemessage_t {
            handle: 0,
            time: 0.,
            type_0: 0,
            message: [0; 256],
            prev: 0 as *mut crate::be_ai_chat_h::bot_consolemessage_s,
            next: 0 as *mut crate::be_ai_chat_h::bot_consolemessage_s,
        };
    let mut match_0: crate::be_ai_chat_h::bot_match_t = crate::be_ai_chat_h::bot_match_t {
        string: [0; 256],
        type_0: 0,
        subtype: 0,
        variables: [crate::be_ai_chat_h::bot_matchvariable_t {
            offset: 0,
            length: 0,
        }; 8],
    };
    //the name of this bot
    ClientName(
        (*bs).client,
        botname.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 36]>() as i32,
    );
    loop
    //
    {
        handle = crate::src::game::g_syscalls::trap_BotNextConsoleMessage(
            (*bs).cs,
            &mut m as *mut crate::be_ai_chat_h::bot_consolemessage_t as *mut libc::c_void,
        );
        if !(handle != 0) {
            break;
        }
        //if the chat state is flooded with messages the bot will read them quickly
        if crate::src::game::g_syscalls::trap_BotNumConsoleMessages((*bs).cs) < 10 {
            //if it is a chat message the bot needs some time to read it
            if m.type_0 == 1
                && m.time
                    > crate::src::game::ai_main::floattime
                        - (1f32 + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32)
            {
                break;
            }
        }
        //
        ptr = m.message.as_mut_ptr();
        //if it is a chat message then don't unify white spaces and don't
        //replace synonyms in the netname
        if m.type_0 == 1 {
            //
            if crate::src::game::g_syscalls::trap_BotFindMatch(
                m.message.as_mut_ptr(),
                &mut match_0 as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
                128,
            ) != 0
            {
                ptr = m
                    .message
                    .as_mut_ptr()
                    .offset(match_0.variables[2].offset as i32 as isize)
            }
        }
        //unify the white spaces in the message
        crate::src::game::g_syscalls::trap_UnifyWhiteSpaces(ptr);
        //replace synonyms in the right context
        context = BotSynonymContext(bs);
        crate::src::game::g_syscalls::trap_BotReplaceSynonyms(ptr, context as usize);
        //if there's no match
        if crate::src::game::ai_cmd::BotMatchMessage(bs, m.message.as_mut_ptr()) == 0 {
            //if it is a chat message
            if m.type_0 == 1 && bot_nochat.integer == 0 {
                //
                if crate::src::game::g_syscalls::trap_BotFindMatch(
                    m.message.as_mut_ptr(),
                    &mut match_0 as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
                    128,
                ) == 0
                {
                    crate::src::game::g_syscalls::trap_BotRemoveConsoleMessage((*bs).cs, handle);
                    continue;
                } else if match_0.subtype & 32768 != 0 {
                    crate::src::game::g_syscalls::trap_BotRemoveConsoleMessage((*bs).cs, handle);
                    continue;
                } else {
                    //don't use eliza chats with team messages
                    //
                    crate::src::game::g_syscalls::trap_BotMatchVariable(
                        &mut match_0 as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
                        0,
                        netname.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 36]>() as i32,
                    );
                    crate::src::game::g_syscalls::trap_BotMatchVariable(
                        &mut match_0 as *mut crate::be_ai_chat_h::bot_match_t as *mut libc::c_void,
                        2,
                        message.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 256]>() as i32,
                    );
                    //if this is a message from the bot self
                    if (*bs).client == ClientFromName(netname.as_mut_ptr()) {
                        crate::src::game::g_syscalls::trap_BotRemoveConsoleMessage(
                            (*bs).cs,
                            handle,
                        );
                        continue;
                    } else {
                        //unify the message
                        crate::src::game::g_syscalls::trap_UnifyWhiteSpaces(message.as_mut_ptr());
                        //
                        crate::src::game::g_syscalls::trap_Cvar_Update(&mut bot_testrchat);
                        if bot_testrchat.integer != 0 {
                            //
                            crate::src::game::g_syscalls::trap_BotLibVarSet(
                                b"bot_testrchat\x00" as *const u8 as *mut i8,
                                b"1\x00" as *const u8 as *mut i8,
                            );
                            //if bot replies with a chat message
                            if crate::src::game::g_syscalls::trap_BotReplyChat(
                                (*bs).cs,
                                message.as_mut_ptr(),
                                context,
                                16,
                                0 as *mut i8,
                                0 as *mut i8,
                                0 as *mut i8,
                                0 as *mut i8,
                                0 as *mut i8,
                                0 as *mut i8,
                                botname.as_mut_ptr(),
                                netname.as_mut_ptr(),
                            ) != 0
                            {
                                crate::src::game::ai_main::BotAI_Print(
                                    1i32,
                                    b"------------------------\n\x00" as *const u8 as *mut i8,
                                );
                            } else {
                                crate::src::game::ai_main::BotAI_Print(
                                    1i32,
                                    b"**** no valid reply ****\n\x00" as *const u8 as *mut i8,
                                );
                            }
                        } else if (*bs).ainode
                            != Some(
                                crate::src::game::ai_dmnet::AINode_Stand
                                    as unsafe extern "C" fn(
                                        _: *mut crate::src::game::ai_main::bot_state_t,
                                    )
                                        -> i32,
                            )
                            && crate::src::game::ai_chat::BotValidChatPosition(bs) != 0
                            && TeamPlayIsOn() == 0
                        {
                            chat_reply = crate::src::game::g_syscalls::trap_Characteristic_BFloat(
                                (*bs).character,
                                35,
                                0f32,
                                1f32,
                            );
                            if (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64)
                                < 1.5 / (crate::src::game::ai_main::NumBots() + 1) as f64
                                && ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) < chat_reply
                            {
                                //if at a valid chat position and not chatting already and not in teamplay
                                //if bot replies with a chat message
                                if crate::src::game::g_syscalls::trap_BotReplyChat(
                                    (*bs).cs,
                                    message.as_mut_ptr(),
                                    context,
                                    16,
                                    0 as *mut i8,
                                    0 as *mut i8,
                                    0 as *mut i8,
                                    0 as *mut i8,
                                    0 as *mut i8,
                                    0 as *mut i8,
                                    botname.as_mut_ptr(),
                                    netname.as_mut_ptr(),
                                ) != 0
                                {
                                    //remove the console message
                                    crate::src::game::g_syscalls::trap_BotRemoveConsoleMessage(
                                        (*bs).cs,
                                        handle,
                                    );
                                    (*bs).stand_time = crate::src::game::ai_main::floattime
                                        + crate::src::game::ai_chat::BotChatTime(bs);
                                    crate::src::game::ai_dmnet::AIEnter_Stand(
                                        bs,
                                        b"BotCheckConsoleMessages: reply chat\x00" as *const u8
                                            as *mut i8,
                                    );
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        //remove the console message
        crate::src::game::g_syscalls::trap_BotRemoveConsoleMessage((*bs).cs, handle);
    }
}
/*
==================
BotCheckEvents
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCheckForGrenades(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut state: *mut crate::src::qcommon::q_shared::entityState_t,
) {
    // if this is not a grenade
    if (*state).eType != crate::bg_public_h::ET_MISSILE as i32
        || (*state).weapon != crate::bg_public_h::WP_GRENADE_LAUNCHER as i32
    {
        return;
    }
    // try to avoid the grenade
    crate::src::game::g_syscalls::trap_BotAddAvoidSpot(
        (*bs).ms,
        (*state).pos.trBase.as_mut_ptr(),
        160f32,
        1,
    );
}
/*
==================
BotCheckEvents
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCheckEvents(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut state: *mut crate::src::qcommon::q_shared::entityState_t,
) {
    let mut event: i32 = 0;
    let mut buf: [i8; 128] = [0; 128];
    //NOTE: this sucks, we're accessing the gentity_t directly
    //but there's no other fast way to do it right now
    if (*bs).entityeventTime[(*state).number as usize]
        == crate::src::game::g_main::g_entities[(*state).number as usize].eventTime
    {
        return;
    }
    (*bs).entityeventTime[(*state).number as usize] =
        crate::src::game::g_main::g_entities[(*state).number as usize].eventTime;
    //if it's an event only entity
    if (*state).eType > crate::bg_public_h::ET_EVENTS as i32 {
        event = (*state).eType - crate::bg_public_h::ET_EVENTS as i32 & !(0x100 | 0x200)
    } else {
        event = (*state).event & !(0x100 | 0x200)
    }
    //
    match event {
        60 => {
            //client obituary event
            let mut target: i32 = 0;
            let mut attacker: i32 = 0;
            let mut mod_0: i32 = 0;
            target = (*state).otherEntityNum;
            attacker = (*state).otherEntityNum2;
            mod_0 = (*state).eventParm;
            //
            if target == (*bs).client {
                (*bs).botdeathtype = mod_0;
                (*bs).lastkilledby = attacker;
                //
                if target == attacker || target == ((1) << 10) - 1 || target == ((1) << 10) - 2 {
                    (*bs).botsuicide = crate::src::qcommon::q_shared::qtrue as i32
                } else {
                    (*bs).botsuicide = crate::src::qcommon::q_shared::qfalse as i32
                }
                //
                (*bs).num_deaths += 1
            } else if attacker == (*bs).client {
                (*bs).enemydeathtype = mod_0;
                (*bs).lastkilledplayer = target;
                (*bs).killedenemy_time = crate::src::game::ai_main::floattime;
                //else if this client was killed by the bot
                //
                (*bs).num_kills += 1
            } else if attacker == (*bs).enemy && target == attacker {
                (*bs).enemysuicide = crate::src::qcommon::q_shared::qtrue as i32
            }
        }
        46 => {
            if (*state).eventParm < 0 || (*state).eventParm >= 256 {
                crate::src::game::ai_main::BotAI_Print(
                    3i32,
                    b"EV_GLOBAL_SOUND: eventParm (%d) out of range\n\x00" as *const u8 as *mut i8,
                    (*state).eventParm,
                );
            } else {
                crate::src::game::g_syscalls::trap_GetConfigstring(
                    32 + 256 + (*state).eventParm,
                    buf.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 128]>() as i32,
                );
                /*
                if (!strcmp(buf, "sound/teamplay/flagret_red.wav")) {
                    //red flag is returned
                    bs->redflagstatus = 0;
                    bs->flagstatuschanged = qtrue;
                }
                else if (!strcmp(buf, "sound/teamplay/flagret_blu.wav")) {
                    //blue flag is returned
                    bs->blueflagstatus = 0;
                    bs->flagstatuschanged = qtrue;
                }
                else*/
                if crate::stdlib::strcmp(
                    buf.as_mut_ptr(),
                    b"sound/items/poweruprespawn.wav\x00" as *const u8 as *const i8,
                ) == 0
                {
                    //powerup respawned... go get it
                    BotGoForPowerups(bs);
                }
            }
        }
        47 => {
            if gametype == crate::bg_public_h::GT_CTF as i32 {
                match (*state).eventParm {
                    0 => {
                        (*bs).blueflagstatus = 0;
                        (*bs).redflagstatus = 0;
                        (*bs).flagstatuschanged = crate::src::qcommon::q_shared::qtrue as i32
                        //see BotMatch_CTF
                    }
                    1 => {
                        (*bs).blueflagstatus = 0;
                        (*bs).redflagstatus = 0;
                        (*bs).flagstatuschanged = crate::src::qcommon::q_shared::qtrue as i32
                    }
                    2 => {
                        //blue flag is returned
                        (*bs).blueflagstatus = 0;
                        (*bs).flagstatuschanged = crate::src::qcommon::q_shared::qtrue as i32
                    }
                    3 => {
                        //red flag is returned
                        (*bs).redflagstatus = 0;
                        (*bs).flagstatuschanged = crate::src::qcommon::q_shared::qtrue as i32
                    }
                    4 => {
                        //blue flag is taken
                        (*bs).blueflagstatus = 1;
                        (*bs).flagstatuschanged = crate::src::qcommon::q_shared::qtrue as i32
                    }
                    5 => {
                        //red flag is taken
                        (*bs).redflagstatus = 1;
                        (*bs).flagstatuschanged = crate::src::qcommon::q_shared::qtrue as i32
                    }
                    _ => {}
                }
            }
        }
        42 => {
            lastteleport_origin[0] = (*state).origin[0];
            lastteleport_origin[1] = (*state).origin[1];
            lastteleport_origin[2] = (*state).origin[2];
            lastteleport_time = crate::src::game::ai_main::floattime
        }
        45 => {
            //if this sound is played on the bot
            if (*state).number == (*bs).client {
                if (*state).eventParm < 0 || (*state).eventParm >= 256 {
                    crate::src::game::ai_main::BotAI_Print(
                        3i32,
                        b"EV_GENERAL_SOUND: eventParm (%d) out of range\n\x00" as *const u8
                            as *mut i8,
                        (*state).eventParm,
                    );
                } else {
                    //check out the sound
                    crate::src::game::g_syscalls::trap_GetConfigstring(
                        32 + 256 + (*state).eventParm,
                        buf.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 128]>() as i32,
                    );
                    //if falling into a death pit
                    if crate::stdlib::strcmp(
                        buf.as_mut_ptr(),
                        b"*falling1.wav\x00" as *const u8 as *const i8,
                    ) == 0
                    {
                        //if the bot has a personal teleporter
                        if (*bs).inventory[30] > 0 {
                            //use the holdable item
                            crate::src::game::g_syscalls::trap_EA_Use((*bs).client);
                        }
                    }
                }
            }
        }
        1 | 2 | 3 | 4 | 5 | 10 | 11 | 12 | 6 | 7 | 8 | 9 | 13 | 14 | 76 | 15 | 16 | 17 | 18
        | 19 | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35
        | 36 | 37 | 38 | 39 | _ => {}
    };
}
/*
==================
BotCheckSnapshot
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCheckSnapshot(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    let mut ent: i32 = 0;
    let mut state: crate::src::qcommon::q_shared::entityState_t =
        crate::src::qcommon::q_shared::entityState_t {
            number: 0,
            eType: 0,
            eFlags: 0,
            pos: crate::src::qcommon::q_shared::trajectory_t {
                trType: crate::src::qcommon::q_shared::TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            apos: crate::src::qcommon::q_shared::trajectory_t {
                trType: crate::src::qcommon::q_shared::TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            time: 0,
            time2: 0,
            origin: [0.; 3],
            origin2: [0.; 3],
            angles: [0.; 3],
            angles2: [0.; 3],
            otherEntityNum: 0,
            otherEntityNum2: 0,
            groundEntityNum: 0,
            constantLight: 0,
            loopSound: 0,
            modelindex: 0,
            modelindex2: 0,
            clientNum: 0,
            frame: 0,
            solid: 0,
            event: 0,
            eventParm: 0,
            powerups: 0,
            weapon: 0,
            legsAnim: 0,
            torsoAnim: 0,
            generic1: 0,
        };
    //remove all avoid spots
    crate::src::game::g_syscalls::trap_BotAddAvoidSpot(
        (*bs).ms,
        crate::src::qcommon::q_math::vec3_origin.as_mut_ptr(),
        0f32,
        0,
    );
    //reset kamikaze body
    (*bs).kamikazebody = 0;
    //reset number of proxmines
    (*bs).numproxmines = 0;
    //
    ent = 0;
    loop {
        ent = crate::src::game::ai_main::BotAI_GetSnapshotEntity((*bs).client, ent, &mut state);
        if !(ent != -(1)) {
            break;
        }
        //check the entity state for events
        BotCheckEvents(bs, &mut state);
        //
        BotCheckForGrenades(bs, &mut state);
    }
    //check for grenades the bot should avoid
    //check the player state for events
    crate::src::game::ai_main::BotAI_GetEntityState((*bs).client, &mut state);
    //copy the player state events to the entity state
    state.event = (*bs).cur_ps.externalEvent;
    state.eventParm = (*bs).cur_ps.externalEventParm;
    //
    BotCheckEvents(bs, &mut state);
}
/*
==================
BotCheckAir
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotCheckAir(mut bs: *mut crate::src::game::ai_main::bot_state_t) {
    if (*bs).inventory[36] <= 0 {
        if crate::src::game::g_syscalls::trap_AAS_PointContents((*bs).eye.as_mut_ptr())
            & (32 | 16 | 8)
            != 0
        {
            return;
        }
    }
    (*bs).lastair_time = crate::src::game::ai_main::floattime;
}
//returns either the alternate route goal or the given goal
/*
==================
BotAlternateRoute
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotAlternateRoute(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> *mut crate::be_ai_goal_h::bot_goal_t {
    let mut t: i32 = 0;
    // if the bot has an alternative route goal
    if (*bs).altroutegoal.areanum != 0 {
        //
        if (*bs).reachedaltroutegoal_time != 0. {
            return goal;
        }
        // travel time towards alternative route goal
        t = crate::src::game::g_syscalls::trap_AAS_AreaTravelTimeToGoalArea(
            (*bs).areanum,
            (*bs).origin.as_mut_ptr(),
            (*bs).altroutegoal.areanum,
            (*bs).tfl,
        );
        if t != 0 && t < 20 {
            //BotAI_Print(PRT_MESSAGE, "reached alternate route goal\n");
            (*bs).reachedaltroutegoal_time = crate::src::game::ai_main::floattime
        }
        crate::stdlib::memcpy(
            goal as *mut libc::c_void,
            &mut (*bs).altroutegoal as *mut crate::be_ai_goal_h::bot_goal_t as *const libc::c_void,
            ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>(),
        );
        return &mut (*bs).altroutegoal;
    }
    return goal;
}
//
//get a random alternate route goal towards the given base
/*
==================
BotGetAlternateRouteGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGetAlternateRouteGoal(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut base: i32,
) -> i32 {
    let mut altroutegoals: *mut crate::be_aas_h::aas_altroutegoal_t =
        0 as *mut crate::be_aas_h::aas_altroutegoal_t;
    let mut goal: *mut crate::be_ai_goal_h::bot_goal_t = 0 as *mut crate::be_ai_goal_h::bot_goal_t;
    let mut numaltroutegoals: i32 = 0;
    let mut rnd: i32 = 0;
    if base == crate::bg_public_h::TEAM_RED as i32 {
        altroutegoals = red_altroutegoals.as_mut_ptr();
        numaltroutegoals = red_numaltroutegoals
    } else {
        altroutegoals = blue_altroutegoals.as_mut_ptr();
        numaltroutegoals = blue_numaltroutegoals
    }
    if numaltroutegoals == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    rnd = ((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32 * numaltroutegoals as f32) as i32;
    if rnd >= numaltroutegoals {
        rnd = numaltroutegoals - 1
    }
    goal = &mut (*bs).altroutegoal;
    (*goal).areanum = (*altroutegoals.offset(rnd as isize)).areanum;
    (*goal).origin[0] = (*altroutegoals.offset(rnd as isize)).origin[0];
    (*goal).origin[1] = (*altroutegoals.offset(rnd as isize)).origin[1];
    (*goal).origin[2] = (*altroutegoals.offset(rnd as isize)).origin[2];
    (*goal).mins[0] = -8f32;
    (*goal).mins[1] = -8f32;
    (*goal).mins[2] = -8f32;
    (*goal).maxs[0] = 8f32;
    (*goal).maxs[1] = 8f32;
    (*goal).maxs[2] = 8f32;
    (*goal).entitynum = 0;
    (*goal).iteminfo = 0;
    (*goal).number = 0;
    (*goal).flags = 0;
    //
    (*bs).reachedaltroutegoal_time = 0f32;
    return crate::src::qcommon::q_shared::qtrue as i32;
}
/*
==================
BotSetupAlternateRouteGoals
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetupAlternativeRouteGoals() {
    if altroutegoals_setup != 0 {
        return;
    }
    altroutegoals_setup = crate::src::qcommon::q_shared::qtrue as i32;
}
//let the bot live within its deathmatch AI net
/*
==================
BotDeathmatchAI
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotDeathmatchAI(
    mut bs: *mut crate::src::game::ai_main::bot_state_t,
    mut thinktime: f32,
) {
    let mut gender: [i8; 144] = [0; 144];
    let mut name: [i8; 144] = [0; 144];
    let mut userinfo: [i8; 1024] = [0; 1024];
    let mut i: i32 = 0;
    //if the bot has just been setup
    if (*bs).setupcount > 0 {
        (*bs).setupcount -= 1;
        if (*bs).setupcount > 0 {
            return;
        }
        //get the gender characteristic
        crate::src::game::g_syscalls::trap_Characteristic_String(
            (*bs).character,
            1,
            gender.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 144]>() as i32,
        );
        //set the bot gender
        crate::src::game::g_syscalls::trap_GetUserinfo(
            (*bs).client,
            userinfo.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        );
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            userinfo.as_mut_ptr(),
            b"sex\x00" as *const u8 as *const i8,
            gender.as_mut_ptr(),
        );
        crate::src::game::g_syscalls::trap_SetUserinfo((*bs).client, userinfo.as_mut_ptr());
        //set the chat gender
        if gender[0] as i32 == 'm' as i32 {
            crate::src::game::g_syscalls::trap_BotSetChatGender((*bs).cs, 2i32);
        } else if gender[0] as i32 == 'f' as i32 {
            crate::src::game::g_syscalls::trap_BotSetChatGender((*bs).cs, 1i32);
        } else {
            crate::src::game::g_syscalls::trap_BotSetChatGender((*bs).cs, 0i32);
        }
        //set the chat name
        ClientName(
            (*bs).client,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 144]>() as i32,
        );
        crate::src::game::g_syscalls::trap_BotSetChatName(
            (*bs).cs,
            name.as_mut_ptr(),
            (*bs).client,
        );
        //
        (*bs).lastframe_health = (*bs).inventory[29];
        (*bs).lasthitcount = (*bs).cur_ps.persistant[crate::bg_public_h::PERS_HITS as usize];
        //
        (*bs).setupcount = 0;
        //
        BotSetupAlternativeRouteGoals();
    }
    //no ideal view set
    (*bs).flags &= !(32);
    //
    if BotIntermission(bs) as u64 == 0 {
        //set the teleport time
        BotSetTeleportTime(bs);
        //update some inventory values
        BotUpdateInventory(bs);
        //check out the snapshot
        BotCheckSnapshot(bs);
        //check for air
        BotCheckAir(bs);
    }
    //check the console messages
    BotCheckConsoleMessages(bs);
    //if not in the intermission and not in observer mode
    if BotIntermission(bs) as u64 == 0 && BotIsObserver(bs) as u64 == 0 {
        //do team AI
        crate::src::game::ai_team::BotTeamAI(bs);
    }
    //if the bot has no ai node
    if (*bs).ainode.is_none() {
        crate::src::game::ai_dmnet::AIEnter_Seek_LTG(
            bs,
            b"BotDeathmatchAI: no ai node\x00" as *const u8 as *mut i8,
        );
    }
    //if the bot entered the game less than 8 seconds ago
    if (*bs).entergamechat == 0
        && (*bs).entergame_time > crate::src::game::ai_main::floattime - 8f32
    {
        if crate::src::game::ai_chat::BotChat_EnterGame(bs) != 0 {
            (*bs).stand_time =
                crate::src::game::ai_main::floattime + crate::src::game::ai_chat::BotChatTime(bs);
            crate::src::game::ai_dmnet::AIEnter_Stand(
                bs,
                b"BotDeathmatchAI: chat enter game\x00" as *const u8 as *mut i8,
            );
        }
        (*bs).entergamechat = crate::src::qcommon::q_shared::qtrue as i32
    }
    //reset the node switches from the previous frame
    crate::src::game::ai_dmnet::BotResetNodeSwitches();
    //execute AI nodes
    i = 0;
    while i < 50 {
        if (*bs).ainode.expect("non-null function pointer")(bs) != 0 {
            break;
        }
        i += 1
    }
    //if the bot removed itself :)
    if (*bs).inuse == 0 {
        return;
    }
    //if the bot executed too many AI nodes
    if i >= 50 {
        crate::src::game::g_syscalls::trap_BotDumpGoalStack((*bs).gs);
        crate::src::game::g_syscalls::trap_BotDumpAvoidGoals((*bs).gs);
        crate::src::game::ai_dmnet::BotDumpNodeSwitches(bs);
        ClientName(
            (*bs).client,
            name.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 144]>() as i32,
        );
        crate::src::game::ai_main::BotAI_Print(
            3i32,
            b"%s at %1.1f switched more than %d AI nodes\n\x00" as *const u8 as *mut i8,
            name.as_mut_ptr(),
            crate::src::game::ai_main::floattime as f64,
            50i32,
        );
    }
    //
    (*bs).lastframe_health = (*bs).inventory[29];
    (*bs).lasthitcount = (*bs).cur_ps.persistant[crate::bg_public_h::PERS_HITS as usize];
}
/*
==================
BotSetEntityNumForGoalWithModel
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetEntityNumForGoalWithModel(
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
    mut eType: i32,
    mut modelname: *mut i8,
) {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut i: i32 = 0;
    let mut modelindex: i32 = 0;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    modelindex = crate::src::game::g_utils::G_ModelIndex(modelname);
    ent = &mut *crate::src::game::g_main::g_entities.as_mut_ptr().offset(0)
        as *mut crate::g_local_h::gentity_t;
    i = 0;
    while i < crate::src::game::g_main::level.num_entities {
        if !((*ent).inuse as u64 == 0) {
            if !(eType != 0 && (*ent).s.eType != eType) {
                if !((*ent).s.modelindex != modelindex) {
                    dir[0] = (*goal).origin[0] - (*ent).s.origin[0];
                    dir[1] = (*goal).origin[1] - (*ent).s.origin[1];
                    dir[2] = (*goal).origin[2] - (*ent).s.origin[2];
                    if VectorLengthSquared(
                        dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                    ) < (10i32 * 10) as f32
                    {
                        (*goal).entitynum = i;
                        return;
                    }
                }
            }
        }
        i += 1;
        ent = ent.offset(1)
    }
}
/*
==================
BotSetEntityNumForGoal
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetEntityNumForGoal(
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
    mut classname: *mut i8,
) {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut i: i32 = 0;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    ent = &mut *crate::src::game::g_main::g_entities.as_mut_ptr().offset(0)
        as *mut crate::g_local_h::gentity_t;
    i = 0;
    while i < crate::src::game::g_main::level.num_entities {
        if !((*ent).inuse as u64 == 0) {
            if !(crate::src::qcommon::q_shared::Q_stricmp((*ent).classname, classname) != 0) {
                dir[0] = (*goal).origin[0] - (*ent).s.origin[0];
                dir[1] = (*goal).origin[1] - (*ent).s.origin[1];
                dir[2] = (*goal).origin[2] - (*ent).s.origin[2];
                if VectorLengthSquared(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                ) < (10i32 * 10) as f32
                {
                    (*goal).entitynum = i;
                    return;
                }
            }
        }
        i += 1;
        ent = ent.offset(1)
    }
}
/*
==================
BotSetEntityNumForGoalWithActivator
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetEntityNumForGoalWithActivator(
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
    mut classname: *mut i8,
) {
    let mut ent: *mut crate::g_local_h::gentity_t = 0 as *mut crate::g_local_h::gentity_t;
    let mut i: i32 = 0;
    let mut dir: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    ent = &mut *crate::src::game::g_main::g_entities.as_mut_ptr().offset(0)
        as *mut crate::g_local_h::gentity_t;
    i = 0;
    while i < crate::src::game::g_main::level.num_entities {
        if !((*ent).inuse as u64 == 0 || (*ent).activator.is_null()) {
            if !(crate::src::qcommon::q_shared::Q_stricmp((*(*ent).activator).classname, classname)
                != 0)
            {
                dir[0] = (*goal).origin[0] - (*ent).s.origin[0];
                dir[1] = (*goal).origin[1] - (*ent).s.origin[1];
                dir[2] = (*goal).origin[2] - (*ent).s.origin[2];
                if VectorLengthSquared(
                    dir.as_mut_ptr() as *const crate::src::qcommon::q_shared::vec_t
                ) < (10i32 * 10) as f32
                {
                    (*goal).entitynum = i;
                    return;
                }
            }
        }
        i += 1;
        ent = ent.offset(1)
    }
}
/*
==================
BotGoalForBSPEntity
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotGoalForBSPEntity(
    mut classname: *mut i8,
    mut goal: *mut crate::be_ai_goal_h::bot_goal_t,
) -> i32 {
    let mut value: [i8; 1024] = [0; 1024];
    let mut origin: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut start: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut end: crate::src::qcommon::q_shared::vec3_t = [0.; 3];
    let mut ent: i32 = 0;
    let mut numareas: i32 = 0;
    let mut areas: [i32; 10] = [0; 10];
    crate::stdlib::memset(
        goal as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::be_ai_goal_h::bot_goal_t>(),
    );
    ent = crate::src::game::g_syscalls::trap_AAS_NextBSPEntity(0);
    while ent != 0 {
        if !(crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
            ent,
            b"classname\x00" as *const u8 as *mut i8,
            value.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 1024]>() as i32,
        ) == 0)
        {
            if crate::stdlib::strcmp(value.as_mut_ptr(), classname) == 0 {
                if crate::src::game::g_syscalls::trap_AAS_VectorForBSPEpairKey(
                    ent,
                    b"origin\x00" as *const u8 as *mut i8,
                    origin.as_mut_ptr(),
                ) == 0
                {
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
                (*goal).origin[0] = origin[0];
                (*goal).origin[1] = origin[1];
                (*goal).origin[2] = origin[2];
                start[0] = origin[0];
                start[1] = origin[1];
                start[2] = origin[2];
                start[2] -= 32f32;
                end[0] = origin[0];
                end[1] = origin[1];
                end[2] = origin[2];
                end[2] += 32f32;
                numareas = crate::src::game::g_syscalls::trap_AAS_TraceAreas(
                    start.as_mut_ptr(),
                    end.as_mut_ptr(),
                    areas.as_mut_ptr(),
                    0 as *mut crate::src::qcommon::q_shared::vec3_t,
                    10,
                );
                if numareas == 0 {
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
                (*goal).areanum = areas[0];
                return crate::src::qcommon::q_shared::qtrue as i32;
            }
        }
        ent = crate::src::game::g_syscalls::trap_AAS_NextBSPEntity(ent)
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
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
 * name:		ai_dmq3.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_chat.c $
 *
 *****************************************************************************/
//setup the deathmatch AI
/*
==================
BotSetupDeathmatchAI
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotSetupDeathmatchAI() {
    let mut ent: i32 = 0;
    let mut modelnum: i32 = 0;
    let mut model: [i8; 128] = [0; 128];
    gametype = crate::src::game::g_syscalls::trap_Cvar_VariableIntegerValue(
        b"g_gametype\x00" as *const u8 as *const i8,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_rocketjump,
        b"bot_rocketjump\x00" as *const u8 as *const i8,
        b"1\x00" as *const u8 as *const i8,
        0,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_grapple,
        b"bot_grapple\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_fastchat,
        b"bot_fastchat\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_nochat,
        b"bot_nochat\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_testrchat,
        b"bot_testrchat\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_challenge,
        b"bot_challenge\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut bot_predictobstacles,
        b"bot_predictobstacles\x00" as *const u8 as *const i8,
        b"1\x00" as *const u8 as *const i8,
        0,
    );
    crate::src::game::g_syscalls::trap_Cvar_Register(
        &mut g_spSkill,
        b"g_spSkill\x00" as *const u8 as *const i8,
        b"2\x00" as *const u8 as *const i8,
        0,
    );
    //
    if gametype == crate::bg_public_h::GT_CTF as i32 {
        if crate::src::game::g_syscalls::trap_BotGetLevelItemGoal(
            -(1),
            b"Red Flag\x00" as *const u8 as *mut i8,
            &mut ctf_redflag as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        ) < 0
        {
            crate::src::game::ai_main::BotAI_Print(
                2i32,
                b"CTF without Red Flag\n\x00" as *const u8 as *mut i8,
            );
        }
        if crate::src::game::g_syscalls::trap_BotGetLevelItemGoal(
            -(1),
            b"Blue Flag\x00" as *const u8 as *mut i8,
            &mut ctf_blueflag as *mut crate::be_ai_goal_h::bot_goal_t as *mut libc::c_void,
        ) < 0
        {
            crate::src::game::ai_main::BotAI_Print(
                2i32,
                b"CTF without Blue Flag\n\x00" as *const u8 as *mut i8,
            );
        }
    }
    max_bspmodelindex = 0;
    ent = crate::src::game::g_syscalls::trap_AAS_NextBSPEntity(0);
    while ent != 0 {
        if !(crate::src::game::g_syscalls::trap_AAS_ValueForBSPEpairKey(
            ent,
            b"model\x00" as *const u8 as *mut i8,
            model.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 128]>() as i32,
        ) == 0)
        {
            if model[0] as i32 == '*' as i32 {
                modelnum = atoi(model.as_mut_ptr().offset(1));
                if modelnum > max_bspmodelindex {
                    max_bspmodelindex = modelnum
                }
            }
        }
        ent = crate::src::game::g_syscalls::trap_AAS_NextBSPEntity(ent)
    }
    //initialize the waypoint heap
    BotInitWaypoints();
}
//shutdown the deathmatch AI
/*
==================
BotShutdownDeathmatchAI
==================
*/
#[no_mangle]

pub unsafe extern "C" fn BotShutdownDeathmatchAI() {
    altroutegoals_setup = crate::src::qcommon::q_shared::qfalse as i32;
}
