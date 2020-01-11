pub type C2RustUnnamed_0 = libc::c_uint;
pub const GT_FFA: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const GT_TOURNAMENT: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const GT_SINGLE_PLAYER: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const GT_TEAM: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const GT_CTF: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const GT_1FCTF: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const GT_OBELISK: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const GT_HARVESTER: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const GT_MAX_GAME_TYPE: crate::bg_public_h::C2RustUnnamed_0 = 8;
pub const PM_NORMAL: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const PM_NOCLIP: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const PM_SPECTATOR: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const PM_DEAD: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const PM_FREEZE: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const PM_INTERMISSION: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const PM_SPINTERMISSION: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const WEAPON_READY: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const WEAPON_RAISING: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const WEAPON_DROPPING: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const WEAPON_FIRING: crate::bg_public_h::C2RustUnnamed_0 = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmove_t {
    pub ps: *mut crate::src::qcommon::q_shared::playerState_t,
    pub cmd: crate::src::qcommon::q_shared::usercmd_t,
    pub tracemask: libc::c_int,
    pub debugLevel: libc::c_int,
    pub noFootsteps: crate::src::qcommon::q_shared::qboolean,
    pub gauntletHit: crate::src::qcommon::q_shared::qboolean,
    pub framecount: libc::c_int,
    pub numtouch: libc::c_int,
    pub touchents: [libc::c_int; 32],
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub watertype: libc::c_int,
    pub waterlevel: libc::c_int,
    pub xyspeed: libc::c_float,
    pub pmove_fixed: libc::c_int,
    pub pmove_msec: libc::c_int,
    pub trace: Option<
        unsafe extern "C" fn(
            _: *mut crate::src::qcommon::q_shared::trace_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: libc::c_int,
            _: libc::c_int,
        ) -> (),
    >,
    pub pointcontents: Option<
        unsafe extern "C" fn(
            _: *const crate::src::qcommon::q_shared::vec_t,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
}
pub const STAT_HEALTH: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const STAT_HOLDABLE_ITEM: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const STAT_WEAPONS: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const STAT_ARMOR: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const STAT_DEAD_YAW: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const STAT_CLIENTS_READY: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const STAT_MAX_HEALTH: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const PERS_SCORE: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const PERS_HITS: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const PERS_RANK: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const PERS_TEAM: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const PERS_SPAWN_COUNT: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const PERS_PLAYEREVENTS: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const PERS_ATTACKER: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const PERS_ATTACKEE_ARMOR: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const PERS_KILLED: crate::bg_public_h::C2RustUnnamed_0 = 8;
pub const PERS_IMPRESSIVE_COUNT: crate::bg_public_h::C2RustUnnamed_0 = 9;
pub const PERS_EXCELLENT_COUNT: crate::bg_public_h::C2RustUnnamed_0 = 10;
pub const PERS_DEFEND_COUNT: crate::bg_public_h::C2RustUnnamed_0 = 11;
pub const PERS_ASSIST_COUNT: crate::bg_public_h::C2RustUnnamed_0 = 12;
pub const PERS_GAUNTLET_FRAG_COUNT: crate::bg_public_h::C2RustUnnamed_0 = 13;
pub const PERS_CAPTURES: crate::bg_public_h::C2RustUnnamed_0 = 14;
pub type powerup_t = libc::c_uint;
pub const PW_NONE: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const PW_QUAD: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const PW_BATTLESUIT: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const PW_HASTE: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const PW_INVIS: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const PW_REGEN: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const PW_FLIGHT: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const PW_REDFLAG: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const PW_BLUEFLAG: crate::bg_public_h::C2RustUnnamed_0 = 8;
pub const PW_NEUTRALFLAG: crate::bg_public_h::C2RustUnnamed_0 = 9;
pub const PW_SCOUT: crate::bg_public_h::C2RustUnnamed_0 = 10;
pub const PW_GUARD: crate::bg_public_h::C2RustUnnamed_0 = 11;
pub const PW_DOUBLER: crate::bg_public_h::C2RustUnnamed_0 = 12;
pub const PW_AMMOREGEN: crate::bg_public_h::C2RustUnnamed_0 = 13;
pub const PW_INVULNERABILITY: crate::bg_public_h::C2RustUnnamed_0 = 14;
pub const PW_NUM_POWERUPS: crate::bg_public_h::C2RustUnnamed_0 = 15;
pub type holdable_t = libc::c_uint;
pub const HI_NONE: crate::bg_public_h::holdable_t = 0;
pub const HI_TELEPORTER: crate::bg_public_h::holdable_t = 1;
pub const HI_MEDKIT: crate::bg_public_h::holdable_t = 2;
pub const HI_KAMIKAZE: crate::bg_public_h::holdable_t = 3;
pub const HI_PORTAL: crate::bg_public_h::holdable_t = 4;
pub const HI_INVULNERABILITY: crate::bg_public_h::holdable_t = 5;
pub const HI_NUM_HOLDABLE: crate::bg_public_h::holdable_t = 6;
pub type weapon_t = libc::c_uint;
pub const WP_NONE: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const WP_GAUNTLET: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const WP_MACHINEGUN: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const WP_SHOTGUN: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const WP_GRENADE_LAUNCHER: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const WP_ROCKET_LAUNCHER: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const WP_LIGHTNING: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const WP_RAILGUN: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const WP_PLASMAGUN: crate::bg_public_h::C2RustUnnamed_0 = 8;
pub const WP_BFG: crate::bg_public_h::C2RustUnnamed_0 = 9;
pub const WP_GRAPPLING_HOOK: crate::bg_public_h::C2RustUnnamed_0 = 10;
pub const WP_NUM_WEAPONS: crate::bg_public_h::C2RustUnnamed_0 = 11;
pub const EV_NONE: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const EV_FOOTSTEP: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const EV_FOOTSTEP_METAL: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const EV_FOOTSPLASH: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const EV_FOOTWADE: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const EV_SWIM: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const EV_STEP_4: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const EV_STEP_8: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const EV_STEP_12: crate::bg_public_h::C2RustUnnamed_0 = 8;
pub const EV_STEP_16: crate::bg_public_h::C2RustUnnamed_0 = 9;
pub const EV_FALL_SHORT: crate::bg_public_h::C2RustUnnamed_0 = 10;
pub const EV_FALL_MEDIUM: crate::bg_public_h::C2RustUnnamed_0 = 11;
pub const EV_FALL_FAR: crate::bg_public_h::C2RustUnnamed_0 = 12;
pub const EV_JUMP_PAD: crate::bg_public_h::C2RustUnnamed_0 = 13;
pub const EV_JUMP: crate::bg_public_h::C2RustUnnamed_0 = 14;
pub const EV_WATER_TOUCH: crate::bg_public_h::C2RustUnnamed_0 = 15;
pub const EV_WATER_LEAVE: crate::bg_public_h::C2RustUnnamed_0 = 16;
pub const EV_WATER_UNDER: crate::bg_public_h::C2RustUnnamed_0 = 17;
pub const EV_WATER_CLEAR: crate::bg_public_h::C2RustUnnamed_0 = 18;
pub const EV_ITEM_PICKUP: crate::bg_public_h::C2RustUnnamed_0 = 19;
pub const EV_GLOBAL_ITEM_PICKUP: crate::bg_public_h::C2RustUnnamed_0 = 20;
pub const EV_NOAMMO: crate::bg_public_h::C2RustUnnamed_0 = 21;
pub const EV_CHANGE_WEAPON: crate::bg_public_h::C2RustUnnamed_0 = 22;
pub const EV_FIRE_WEAPON: crate::bg_public_h::C2RustUnnamed_0 = 23;
pub const EV_USE_ITEM0: crate::bg_public_h::C2RustUnnamed_0 = 24;
pub const EV_USE_ITEM1: crate::bg_public_h::C2RustUnnamed_0 = 25;
pub const EV_USE_ITEM2: crate::bg_public_h::C2RustUnnamed_0 = 26;
pub const EV_USE_ITEM3: crate::bg_public_h::C2RustUnnamed_0 = 27;
pub const EV_USE_ITEM4: crate::bg_public_h::C2RustUnnamed_0 = 28;
pub const EV_USE_ITEM5: crate::bg_public_h::C2RustUnnamed_0 = 29;
pub const EV_USE_ITEM6: crate::bg_public_h::C2RustUnnamed_0 = 30;
pub const EV_USE_ITEM7: crate::bg_public_h::C2RustUnnamed_0 = 31;
pub const EV_USE_ITEM8: crate::bg_public_h::C2RustUnnamed_0 = 32;
pub const EV_USE_ITEM9: crate::bg_public_h::C2RustUnnamed_0 = 33;
pub const EV_USE_ITEM10: crate::bg_public_h::C2RustUnnamed_0 = 34;
pub const EV_USE_ITEM11: crate::bg_public_h::C2RustUnnamed_0 = 35;
pub const EV_USE_ITEM12: crate::bg_public_h::C2RustUnnamed_0 = 36;
pub const EV_USE_ITEM13: crate::bg_public_h::C2RustUnnamed_0 = 37;
pub const EV_USE_ITEM14: crate::bg_public_h::C2RustUnnamed_0 = 38;
pub const EV_USE_ITEM15: crate::bg_public_h::C2RustUnnamed_0 = 39;
pub const EV_ITEM_RESPAWN: crate::bg_public_h::C2RustUnnamed_0 = 40;
pub const EV_ITEM_POP: crate::bg_public_h::C2RustUnnamed_0 = 41;
pub const EV_PLAYER_TELEPORT_IN: crate::bg_public_h::C2RustUnnamed_0 = 42;
pub const EV_PLAYER_TELEPORT_OUT: crate::bg_public_h::C2RustUnnamed_0 = 43;
pub const EV_GRENADE_BOUNCE: crate::bg_public_h::C2RustUnnamed_0 = 44;
pub const EV_GENERAL_SOUND: crate::bg_public_h::C2RustUnnamed_0 = 45;
pub const EV_GLOBAL_SOUND: crate::bg_public_h::C2RustUnnamed_0 = 46;
pub const EV_GLOBAL_TEAM_SOUND: crate::bg_public_h::C2RustUnnamed_0 = 47;
pub const EV_BULLET_HIT_FLESH: crate::bg_public_h::C2RustUnnamed_0 = 48;
pub const EV_BULLET_HIT_WALL: crate::bg_public_h::C2RustUnnamed_0 = 49;
pub const EV_MISSILE_HIT: crate::bg_public_h::C2RustUnnamed_0 = 50;
pub const EV_MISSILE_MISS: crate::bg_public_h::C2RustUnnamed_0 = 51;
pub const EV_MISSILE_MISS_METAL: crate::bg_public_h::C2RustUnnamed_0 = 52;
pub const EV_RAILTRAIL: crate::bg_public_h::C2RustUnnamed_0 = 53;
pub const EV_SHOTGUN: crate::bg_public_h::C2RustUnnamed_0 = 54;
pub const EV_BULLET: crate::bg_public_h::C2RustUnnamed_0 = 55;
pub const EV_PAIN: crate::bg_public_h::C2RustUnnamed_0 = 56;
pub const EV_DEATH1: crate::bg_public_h::C2RustUnnamed_0 = 57;
pub const EV_DEATH2: crate::bg_public_h::C2RustUnnamed_0 = 58;
pub const EV_DEATH3: crate::bg_public_h::C2RustUnnamed_0 = 59;
pub const EV_OBITUARY: crate::bg_public_h::C2RustUnnamed_0 = 60;
pub const EV_POWERUP_QUAD: crate::bg_public_h::C2RustUnnamed_0 = 61;
pub const EV_POWERUP_BATTLESUIT: crate::bg_public_h::C2RustUnnamed_0 = 62;
pub const EV_POWERUP_REGEN: crate::bg_public_h::C2RustUnnamed_0 = 63;
pub const EV_GIB_PLAYER: crate::bg_public_h::C2RustUnnamed_0 = 64;
pub const EV_SCOREPLUM: crate::bg_public_h::C2RustUnnamed_0 = 65;
pub const EV_PROXIMITY_MINE_STICK: crate::bg_public_h::C2RustUnnamed_0 = 66;
pub const EV_PROXIMITY_MINE_TRIGGER: crate::bg_public_h::C2RustUnnamed_0 = 67;
pub const EV_KAMIKAZE: crate::bg_public_h::C2RustUnnamed_0 = 68;
pub const EV_OBELISKEXPLODE: crate::bg_public_h::C2RustUnnamed_0 = 69;
pub const EV_OBELISKPAIN: crate::bg_public_h::C2RustUnnamed_0 = 70;
pub const EV_INVUL_IMPACT: crate::bg_public_h::C2RustUnnamed_0 = 71;
pub const EV_JUICED: crate::bg_public_h::C2RustUnnamed_0 = 72;
pub const EV_LIGHTNINGBOLT: crate::bg_public_h::C2RustUnnamed_0 = 73;
pub const EV_DEBUG_LINE: crate::bg_public_h::C2RustUnnamed_0 = 74;
pub const EV_STOPLOOPINGSOUND: crate::bg_public_h::C2RustUnnamed_0 = 75;
pub const EV_TAUNT: crate::bg_public_h::C2RustUnnamed_0 = 76;
pub const EV_TAUNT_YES: crate::bg_public_h::C2RustUnnamed_0 = 77;
pub const EV_TAUNT_NO: crate::bg_public_h::C2RustUnnamed_0 = 78;
pub const EV_TAUNT_FOLLOWME: crate::bg_public_h::C2RustUnnamed_0 = 79;
pub const EV_TAUNT_GETFLAG: crate::bg_public_h::C2RustUnnamed_0 = 80;
pub const EV_TAUNT_GUARDBASE: crate::bg_public_h::C2RustUnnamed_0 = 81;
pub const EV_TAUNT_PATROL: crate::bg_public_h::C2RustUnnamed_0 = 82;
pub const GTS_RED_CAPTURE: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const GTS_BLUE_CAPTURE: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const GTS_RED_RETURN: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const GTS_BLUE_RETURN: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const GTS_RED_TAKEN: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const GTS_BLUE_TAKEN: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const GTS_REDOBELISK_ATTACKED: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const GTS_BLUEOBELISK_ATTACKED: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const GTS_REDTEAM_SCORED: crate::bg_public_h::C2RustUnnamed_0 = 8;
pub const GTS_BLUETEAM_SCORED: crate::bg_public_h::C2RustUnnamed_0 = 9;
pub const GTS_REDTEAM_TOOK_LEAD: crate::bg_public_h::C2RustUnnamed_0 = 10;
pub const GTS_BLUETEAM_TOOK_LEAD: crate::bg_public_h::C2RustUnnamed_0 = 11;
pub const GTS_TEAMS_ARE_TIED: crate::bg_public_h::C2RustUnnamed_0 = 12;
pub const GTS_KAMIKAZE: crate::bg_public_h::C2RustUnnamed_0 = 13;
pub const BOTH_DEATH1: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const BOTH_DEAD1: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const BOTH_DEATH2: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const BOTH_DEAD2: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const BOTH_DEATH3: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const BOTH_DEAD3: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const TORSO_GESTURE: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const TORSO_ATTACK: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const TORSO_ATTACK2: crate::bg_public_h::C2RustUnnamed_0 = 8;
pub const TORSO_DROP: crate::bg_public_h::C2RustUnnamed_0 = 9;
pub const TORSO_RAISE: crate::bg_public_h::C2RustUnnamed_0 = 10;
pub const TORSO_STAND: crate::bg_public_h::C2RustUnnamed_0 = 11;
pub const TORSO_STAND2: crate::bg_public_h::C2RustUnnamed_0 = 12;
pub const LEGS_WALKCR: crate::bg_public_h::C2RustUnnamed_0 = 13;
pub const LEGS_WALK: crate::bg_public_h::C2RustUnnamed_0 = 14;
pub const LEGS_RUN: crate::bg_public_h::C2RustUnnamed_0 = 15;
pub const LEGS_BACK: crate::bg_public_h::C2RustUnnamed_0 = 16;
pub const LEGS_SWIM: crate::bg_public_h::C2RustUnnamed_0 = 17;
pub const LEGS_JUMP: crate::bg_public_h::C2RustUnnamed_0 = 18;
pub const LEGS_LAND: crate::bg_public_h::C2RustUnnamed_0 = 19;
pub const LEGS_JUMPB: crate::bg_public_h::C2RustUnnamed_0 = 20;
pub const LEGS_LANDB: crate::bg_public_h::C2RustUnnamed_0 = 21;
pub const LEGS_IDLE: crate::bg_public_h::C2RustUnnamed_0 = 22;
pub const LEGS_IDLECR: crate::bg_public_h::C2RustUnnamed_0 = 23;
pub const LEGS_TURN: crate::bg_public_h::C2RustUnnamed_0 = 24;
pub const TORSO_GETFLAG: crate::bg_public_h::C2RustUnnamed_0 = 25;
pub const TORSO_GUARDBASE: crate::bg_public_h::C2RustUnnamed_0 = 26;
pub const TORSO_PATROL: crate::bg_public_h::C2RustUnnamed_0 = 27;
pub const TORSO_FOLLOWME: crate::bg_public_h::C2RustUnnamed_0 = 28;
pub const TORSO_AFFIRMATIVE: crate::bg_public_h::C2RustUnnamed_0 = 29;
pub const TORSO_NEGATIVE: crate::bg_public_h::C2RustUnnamed_0 = 30;
pub const MAX_ANIMATIONS: crate::bg_public_h::C2RustUnnamed_0 = 31;
pub const LEGS_BACKCR: crate::bg_public_h::C2RustUnnamed_0 = 32;
pub const LEGS_BACKWALK: crate::bg_public_h::C2RustUnnamed_0 = 33;
pub const FLAG_RUN: crate::bg_public_h::C2RustUnnamed_0 = 34;
pub const FLAG_STAND: crate::bg_public_h::C2RustUnnamed_0 = 35;
pub const FLAG_STAND2RUN: crate::bg_public_h::C2RustUnnamed_0 = 36;
pub const MAX_TOTALANIMATIONS: crate::bg_public_h::C2RustUnnamed_0 = 37;
pub type team_t = libc::c_uint;
pub const TEAM_FREE: crate::bg_public_h::team_t = 0;
pub const TEAM_RED: crate::bg_public_h::team_t = 1;
pub const TEAM_BLUE: crate::bg_public_h::team_t = 2;
pub const TEAM_SPECTATOR: crate::bg_public_h::team_t = 3;
pub const TEAM_NUM_TEAMS: crate::bg_public_h::team_t = 4;
pub const TEAMTASK_NONE: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const TEAMTASK_OFFENSE: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const TEAMTASK_DEFENSE: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const TEAMTASK_PATROL: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const TEAMTASK_FOLLOW: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const TEAMTASK_RETRIEVE: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const TEAMTASK_ESCORT: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const TEAMTASK_CAMP: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const MOD_UNKNOWN: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const MOD_SHOTGUN: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const MOD_GAUNTLET: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const MOD_MACHINEGUN: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const MOD_GRENADE: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const MOD_GRENADE_SPLASH: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const MOD_ROCKET: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const MOD_ROCKET_SPLASH: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const MOD_PLASMA: crate::bg_public_h::C2RustUnnamed_0 = 8;
pub const MOD_PLASMA_SPLASH: crate::bg_public_h::C2RustUnnamed_0 = 9;
pub const MOD_RAILGUN: crate::bg_public_h::C2RustUnnamed_0 = 10;
pub const MOD_LIGHTNING: crate::bg_public_h::C2RustUnnamed_0 = 11;
pub const MOD_BFG: crate::bg_public_h::C2RustUnnamed_0 = 12;
pub const MOD_BFG_SPLASH: crate::bg_public_h::C2RustUnnamed_0 = 13;
pub const MOD_WATER: crate::bg_public_h::C2RustUnnamed_0 = 14;
pub const MOD_SLIME: crate::bg_public_h::C2RustUnnamed_0 = 15;
pub const MOD_LAVA: crate::bg_public_h::C2RustUnnamed_0 = 16;
pub const MOD_CRUSH: crate::bg_public_h::C2RustUnnamed_0 = 17;
pub const MOD_TELEFRAG: crate::bg_public_h::C2RustUnnamed_0 = 18;
pub const MOD_FALLING: crate::bg_public_h::C2RustUnnamed_0 = 19;
pub const MOD_SUICIDE: crate::bg_public_h::C2RustUnnamed_0 = 20;
pub const MOD_TARGET_LASER: crate::bg_public_h::C2RustUnnamed_0 = 21;
pub const MOD_TRIGGER_HURT: crate::bg_public_h::C2RustUnnamed_0 = 22;
pub const MOD_GRAPPLE: crate::bg_public_h::C2RustUnnamed_0 = 23;
pub type itemType_t = libc::c_uint;
pub const IT_BAD: crate::bg_public_h::itemType_t = 0;
pub const IT_WEAPON: crate::bg_public_h::itemType_t = 1;
pub const IT_AMMO: crate::bg_public_h::itemType_t = 2;
pub const IT_ARMOR: crate::bg_public_h::itemType_t = 3;
pub const IT_HEALTH: crate::bg_public_h::itemType_t = 4;
pub const IT_POWERUP: crate::bg_public_h::itemType_t = 5;
pub const IT_HOLDABLE: crate::bg_public_h::itemType_t = 6;
pub const IT_PERSISTANT_POWERUP: crate::bg_public_h::itemType_t = 7;
pub const IT_TEAM: crate::bg_public_h::itemType_t = 8;
pub type gitem_t = crate::bg_public_h::gitem_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gitem_s {
    pub classname: *mut libc::c_char,
    pub pickup_sound: *mut libc::c_char,
    pub world_model: [*mut libc::c_char; 4],
    pub icon: *mut libc::c_char,
    pub pickup_name: *mut libc::c_char,
    pub quantity: libc::c_int,
    pub giType: crate::bg_public_h::itemType_t,
    pub giTag: libc::c_int,
    pub precaches: *mut libc::c_char,
    pub sounds: *mut libc::c_char,
}
pub const ET_GENERAL: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const ET_PLAYER: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const ET_ITEM: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const ET_MISSILE: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const ET_MOVER: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const ET_BEAM: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const ET_PORTAL: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const ET_SPEAKER: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const ET_PUSH_TRIGGER: crate::bg_public_h::C2RustUnnamed_0 = 8;
pub const ET_TELEPORT_TRIGGER: crate::bg_public_h::C2RustUnnamed_0 = 9;
pub const ET_INVISIBLE: crate::bg_public_h::C2RustUnnamed_0 = 10;
pub const ET_GRAPPLE: crate::bg_public_h::C2RustUnnamed_0 = 11;
// any of the EV_* events can be added freestanding

// by setting eType to ET_EVENTS + eventNum

// this avoids having to set eFlags and eventNum

// grapple hooked on wall
pub const ET_TEAM: crate::bg_public_h::C2RustUnnamed_0 = 12;
pub const ET_EVENTS: crate::bg_public_h::C2RustUnnamed_0 = 13;
