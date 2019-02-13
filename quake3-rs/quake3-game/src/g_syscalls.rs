use libc;
#[header_src = "/usr/include/stdint.h"]
pub mod stdint_h {
    pub type intptr_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_shared.h"]
pub mod q_shared_h {
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
    // q_shared.h -- included first by ALL program modules.
// A user mod should never modify this file
    // Heartbeat for dpmaster protocol. You shouldn't change this unless you know what you're doing
    // When com_gamename is LEGACY_MASTER_GAMENAME, use quake3 master protocol.
// You shouldn't change this unless you know what you're doing
    // number of supported master servers
    // standard demo extension
    //Ignore __attribute__ on non-gcc platforms
    /* *********************************************************************
  VM Considerations

  The VM can not use the standard system headers because we aren't really
  using the compiler they were meant for.  We use bg_lib.h which contains
  prototypes for the functions we define for our own use in bg_lib.c.

  When writing mods, please add needed headers HERE, do not start including
  stuff like <stdio.h> in the various .c files that make up each of the VMs
  since you will be including system headers files can will have issues.

  Remember, if you use a C library function that is not defined in bg_lib.c,
  you will have to add your own version for support in the VM.

 **********************************************************************/
    //=============================================================
    pub type byte = libc::c_uchar;
    pub type qboolean = libc::c_uint;
    pub const qtrue: qboolean = 1;
    pub const qfalse: qboolean = 0;
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union floatint_t {
        pub f: libc::c_float,
        pub i: libc::c_int,
        pub ui: libc::c_uint,
    }
    pub type fileHandle_t = libc::c_int;
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
    /*
==============================================================

VoIP

==============================================================
*/
    // if you change the count of flags be sure to also change VOIP_FLAGNUM
    // spatialized voip message
    // non-spatialized voip message
    // number of flags voip knows. You will have to bump protocol version number if you
// change this.
    /*
==============================================================

COLLISION DETECTION

==============================================================
*/
    // plane types are used to speed some tests
// 0-2 are axial planes
    /*
=================
PlaneTypeForNormal
=================
*/
    // plane_t structure
// !!! if this is changed, it must be changed in asm code too !!!
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cplane_s {
        pub normal: vec3_t,
        pub dist: libc::c_float,
        pub type_0: byte,
        pub signbits: byte,
        pub pad: [byte; 2],
    }
    //int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );
    //token types
    // string
    // literal
    // number
    // name
    // punctuation
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct pc_token_s {
        pub type_0: libc::c_int,
        pub subtype: libc::c_int,
        pub intvalue: libc::c_int,
        pub floatvalue: libc::c_float,
        pub string: [libc::c_char; 1024],
    }
    pub type pc_token_t = pc_token_s;
    // mode parm for FS_FOpenFile
    pub type fsMode_t = libc::c_uint;
    pub const FS_APPEND_SYNC: fsMode_t = 3;
    pub const FS_APPEND: fsMode_t = 2;
    pub const FS_WRITE: fsMode_t = 1;
    pub const FS_READ: fsMode_t = 0;
    pub type cvarHandle_t = libc::c_int;
    // the modules that run in the virtual machine can't access the cvar_t directly,
// so they must ask for structured updates
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vmCvar_t {
        pub handle: cvarHandle_t,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub string: [libc::c_char; 256],
    }
    pub type cplane_t = cplane_s;
    // a trace is returned when a box is swept through the world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
    //=========================================================
    // bit field limits
    // playerState_t is the information needed by both the client and server
// to predict player motion and actions
// nothing outside of pmove should modify these, or some degree of prediction error
// will occur
    // you can't add anything to this without modifying the code in msg.c
    // playerState_t is a full superset of entityState_t as it is used by players,
// so if a playerState_t is transmitted, the entityState_t can be fully derived
// from it.
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
    pub type playerState_t = playerState_s;
    //====================================================================
    //
// usercmd_t->button bits, many of which are generated by the client system,
// so they aren't game/cgame only definitions
//
    // displays talk balloon and disables actions
    // walking can't just be inferred from MOVE_RUN
    // because a key pressed late in the frame will
										// only generate a small move value for that frame
										// walking will use different animations and
										// won't generate footsteps
    // any key whatsoever
    // if forwardmove or rightmove are >= MOVE_RUN,
    // then BUTTON_WALKING should be set
    // usercmd_t is sent to the server each client frame
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct usercmd_s {
        pub serverTime: libc::c_int,
        pub angles: [libc::c_int; 3],
        pub buttons: libc::c_int,
        pub weapon: byte,
        pub forwardmove: libc::c_schar,
        pub rightmove: libc::c_schar,
        pub upmove: libc::c_schar,
    }
    pub type usercmd_t = usercmd_s;
    //===================================================================
    // if entityState->solid == SOLID_BMODEL, modelindex is an inline model number
    pub type trType_t = libc::c_uint;
    pub const TR_GRAVITY: trType_t = 5;
    // value = base + sin( time / duration ) * delta
    pub const TR_SINE: trType_t = 4;
    pub const TR_LINEAR_STOP: trType_t = 3;
    pub const TR_LINEAR: trType_t = 2;
    // non-parametric, but interpolate between snapshots
    pub const TR_INTERPOLATE: trType_t = 1;
    pub const TR_STATIONARY: trType_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct trajectory_t {
        pub trType: trType_t,
        pub trTime: libc::c_int,
        pub trDuration: libc::c_int,
        pub trBase: vec3_t,
        pub trDelta: vec3_t,
    }
    // entityState_t is the information conveyed from the server
// in an update message about entities that the client will
// need to render in some way
// Different eTypes may use the information in different ways
// The messages are delta compressed, so it doesn't really matter if
// the structure size is fairly large
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
    pub type entityState_t = entityState_s;
    // real time
//=============================================
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct qtime_s {
        pub tm_sec: libc::c_int,
        pub tm_min: libc::c_int,
        pub tm_hour: libc::c_int,
        pub tm_mday: libc::c_int,
        pub tm_mon: libc::c_int,
        pub tm_year: libc::c_int,
        pub tm_wday: libc::c_int,
        pub tm_yday: libc::c_int,
        pub tm_isdst: libc::c_int,
    }
    pub type qtime_t = qtime_s;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/bg_public.h"]
pub mod bg_public_h {
    // flip the togglebit every time an animation
// changes so a restart of the same anim can be detected
    pub type team_t = libc::c_uint;
    pub const TEAM_NUM_TEAMS: team_t = 4;
    pub const TEAM_SPECTATOR: team_t = 3;
    pub const TEAM_BLUE: team_t = 2;
    pub const TEAM_RED: team_t = 1;
    pub const TEAM_FREE: team_t = 0;
    //---------------------------------------------------------
    // gitem_t->type
    pub type itemType_t = libc::c_uint;
    pub const IT_TEAM: itemType_t = 8;
    // EFX: rotate + bob
    pub const IT_PERSISTANT_POWERUP: itemType_t = 7;
    // EFX: rotate + external ring that rotates
    // single use, holdable item
    pub const IT_HOLDABLE: itemType_t = 6;
    // instant on, timer based
    pub const IT_POWERUP: itemType_t = 5;
    // EFX: static external sphere + rotating internal
    pub const IT_HEALTH: itemType_t = 4;
    // EFX: rotate + minlight
    pub const IT_ARMOR: itemType_t = 3;
    // EFX: rotate
    pub const IT_AMMO: itemType_t = 2;
    // EFX: rotate + upscale + minlight
    pub const IT_WEAPON: itemType_t = 1;
    pub const IT_BAD: itemType_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
    pub type gitem_t = gitem_s;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/g_public.h"]
pub mod g_public_h {
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
    // g_public.h -- game module information visible to server
    // entity->svFlags
// the server does not know how to interpret most of the values
// in entityStates (level eType), so the game must explicitly flag
// special server behaviors
    // don't send entity to clients, even if it has effects
    // TTimo
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=551
    // set if the entity is a bot
    // send to all connected clients
    // merge a second pvs at origin2 into snapshots
    // entity->r.currentOrigin instead of entity->s.origin
    // for link position (missiles and movers)
    // only send to a single client (entityShared_t->singleClient)
    // don't send CS_SERVERINFO updates to this client
    // so that it can be updated for ping tools without
											// lagging clients
    // use capsule for collision detection instead of bbox
    // send entity to everyone but one client
    // (entityShared_t->singleClient)
    //===============================================================
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
    //===============================================================
    //
// system traps provided by the main engine
//
    pub type unnamed = libc::c_uint;
    pub const BOTLIB_PC_SOURCE_FILE_AND_LINE: unnamed = 581;
    pub const BOTLIB_PC_READ_TOKEN: unnamed = 580;
    pub const BOTLIB_PC_FREE_SOURCE: unnamed = 579;
    pub const BOTLIB_PC_LOAD_SOURCE: unnamed = 578;
    pub const BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX: unnamed = 577;
    pub const BOTLIB_AAS_PREDICT_ROUTE: unnamed = 576;
    pub const BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL: unnamed = 575;
    pub const BOTLIB_AI_ADD_AVOID_SPOT: unnamed = 574;
    pub const BOTLIB_AI_SET_AVOID_GOAL_TIME: unnamed = 573;
    pub const BOTLIB_AI_PREDICT_VISIBLE_POSITION: unnamed = 572;
    pub const BOTLIB_AI_REMOVE_FROM_AVOID_GOALS: unnamed = 571;
    pub const BOTLIB_AI_GET_CHAT_MESSAGE: unnamed = 570;
    pub const BOTLIB_AI_NUM_INITIAL_CHATS: unnamed = 569;
    pub const BOTLIB_AI_GET_MAP_LOCATION_GOAL: unnamed = 568;
    pub const BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL: unnamed = 567;
    pub const BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC: unnamed = 566;
    pub const BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC: unnamed = 565;
    pub const BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION: unnamed = 564;
    pub const BOTLIB_AI_RESET_WEAPON_STATE: unnamed = 563;
    pub const BOTLIB_AI_FREE_WEAPON_STATE: unnamed = 562;
    pub const BOTLIB_AI_ALLOC_WEAPON_STATE: unnamed = 561;
    pub const BOTLIB_AI_LOAD_WEAPON_WEIGHTS: unnamed = 560;
    pub const BOTLIB_AI_GET_WEAPON_INFO: unnamed = 559;
    pub const BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON: unnamed = 558;
    pub const BOTLIB_AI_INIT_MOVE_STATE: unnamed = 557;
    pub const BOTLIB_AI_FREE_MOVE_STATE: unnamed = 556;
    pub const BOTLIB_AI_ALLOC_MOVE_STATE: unnamed = 555;
    pub const BOTLIB_AI_MOVEMENT_VIEW_TARGET: unnamed = 554;
    pub const BOTLIB_AI_REACHABILITY_AREA: unnamed = 553;
    pub const BOTLIB_AI_RESET_LAST_AVOID_REACH: unnamed = 552;
    pub const BOTLIB_AI_RESET_AVOID_REACH: unnamed = 551;
    pub const BOTLIB_AI_MOVE_IN_DIRECTION: unnamed = 550;
    pub const BOTLIB_AI_MOVE_TO_GOAL: unnamed = 549;
    pub const BOTLIB_AI_RESET_MOVE_STATE: unnamed = 548;
    pub const BOTLIB_AI_FREE_GOAL_STATE: unnamed = 547;
    pub const BOTLIB_AI_ALLOC_GOAL_STATE: unnamed = 546;
    pub const BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC: unnamed = 545;
    pub const BOTLIB_AI_FREE_ITEM_WEIGHTS: unnamed = 544;
    pub const BOTLIB_AI_LOAD_ITEM_WEIGHTS: unnamed = 543;
    pub const BOTLIB_AI_UPDATE_ENTITY_ITEMS: unnamed = 542;
    pub const BOTLIB_AI_INIT_LEVEL_ITEMS: unnamed = 541;
    pub const BOTLIB_AI_AVOID_GOAL_TIME: unnamed = 540;
    pub const BOTLIB_AI_GET_LEVEL_ITEM_GOAL: unnamed = 539;
    pub const BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE: unnamed = 538;
    pub const BOTLIB_AI_TOUCHING_GOAL: unnamed = 537;
    pub const BOTLIB_AI_CHOOSE_NBG_ITEM: unnamed = 536;
    pub const BOTLIB_AI_CHOOSE_LTG_ITEM: unnamed = 535;
    pub const BOTLIB_AI_GET_SECOND_GOAL: unnamed = 534;
    pub const BOTLIB_AI_GET_TOP_GOAL: unnamed = 533;
    pub const BOTLIB_AI_GOAL_NAME: unnamed = 532;
    pub const BOTLIB_AI_DUMP_GOAL_STACK: unnamed = 531;
    pub const BOTLIB_AI_DUMP_AVOID_GOALS: unnamed = 530;
    pub const BOTLIB_AI_EMPTY_GOAL_STACK: unnamed = 529;
    pub const BOTLIB_AI_POP_GOAL: unnamed = 528;
    pub const BOTLIB_AI_PUSH_GOAL: unnamed = 527;
    pub const BOTLIB_AI_RESET_AVOID_GOALS: unnamed = 526;
    pub const BOTLIB_AI_RESET_GOAL_STATE: unnamed = 525;
    pub const BOTLIB_AI_SET_CHAT_NAME: unnamed = 524;
    pub const BOTLIB_AI_SET_CHAT_GENDER: unnamed = 523;
    pub const BOTLIB_AI_LOAD_CHAT_FILE: unnamed = 522;
    pub const BOTLIB_AI_REPLACE_SYNONYMS: unnamed = 521;
    pub const BOTLIB_AI_UNIFY_WHITE_SPACES: unnamed = 520;
    pub const BOTLIB_AI_MATCH_VARIABLE: unnamed = 519;
    pub const BOTLIB_AI_FIND_MATCH: unnamed = 518;
    pub const BOTLIB_AI_STRING_CONTAINS: unnamed = 517;
    pub const BOTLIB_AI_ENTER_CHAT: unnamed = 516;
    pub const BOTLIB_AI_CHAT_LENGTH: unnamed = 515;
    pub const BOTLIB_AI_REPLY_CHAT: unnamed = 514;
    pub const BOTLIB_AI_INITIAL_CHAT: unnamed = 513;
    pub const BOTLIB_AI_NUM_CONSOLE_MESSAGE: unnamed = 512;
    pub const BOTLIB_AI_NEXT_CONSOLE_MESSAGE: unnamed = 511;
    pub const BOTLIB_AI_REMOVE_CONSOLE_MESSAGE: unnamed = 510;
    pub const BOTLIB_AI_QUEUE_CONSOLE_MESSAGE: unnamed = 509;
    pub const BOTLIB_AI_FREE_CHAT_STATE: unnamed = 508;
    pub const BOTLIB_AI_ALLOC_CHAT_STATE: unnamed = 507;
    pub const BOTLIB_AI_CHARACTERISTIC_STRING: unnamed = 506;
    pub const BOTLIB_AI_CHARACTERISTIC_BINTEGER: unnamed = 505;
    pub const BOTLIB_AI_CHARACTERISTIC_INTEGER: unnamed = 504;
    pub const BOTLIB_AI_CHARACTERISTIC_BFLOAT: unnamed = 503;
    pub const BOTLIB_AI_CHARACTERISTIC_FLOAT: unnamed = 502;
    pub const BOTLIB_AI_FREE_CHARACTER: unnamed = 501;
    pub const BOTLIB_AI_LOAD_CHARACTER: unnamed = 500;
    pub const BOTLIB_EA_RESET_INPUT: unnamed = 423;
    pub const BOTLIB_EA_GET_INPUT: unnamed = 422;
    pub const BOTLIB_EA_END_REGULAR: unnamed = 421;
    pub const BOTLIB_EA_VIEW: unnamed = 420;
    pub const BOTLIB_EA_MOVE: unnamed = 419;
    pub const BOTLIB_EA_DELAYED_JUMP: unnamed = 418;
    pub const BOTLIB_EA_JUMP: unnamed = 417;
    pub const BOTLIB_EA_SELECT_WEAPON: unnamed = 416;
    pub const BOTLIB_EA_MOVE_RIGHT: unnamed = 415;
    pub const BOTLIB_EA_MOVE_LEFT: unnamed = 414;
    pub const BOTLIB_EA_MOVE_BACK: unnamed = 413;
    pub const BOTLIB_EA_MOVE_FORWARD: unnamed = 412;
    pub const BOTLIB_EA_MOVE_DOWN: unnamed = 411;
    pub const BOTLIB_EA_MOVE_UP: unnamed = 410;
    pub const BOTLIB_EA_CROUCH: unnamed = 409;
    pub const BOTLIB_EA_RESPAWN: unnamed = 408;
    pub const BOTLIB_EA_USE: unnamed = 407;
    pub const BOTLIB_EA_ATTACK: unnamed = 406;
    pub const BOTLIB_EA_TALK: unnamed = 405;
    pub const BOTLIB_EA_GESTURE: unnamed = 404;
    pub const BOTLIB_EA_ACTION: unnamed = 403;
    pub const BOTLIB_EA_COMMAND: unnamed = 402;
    pub const BOTLIB_EA_SAY_TEAM: unnamed = 401;
    pub const BOTLIB_EA_SAY: unnamed = 400;
    pub const BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT: unnamed = 318;
    pub const BOTLIB_AAS_SWIMMING: unnamed = 317;
    pub const BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA: unnamed = 316;
    pub const BOTLIB_AAS_AREA_REACHABILITY: unnamed = 315;
    pub const BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY: unnamed = 314;
    pub const BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY: unnamed = 313;
    pub const BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY: unnamed = 312;
    pub const BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY: unnamed = 311;
    pub const BOTLIB_AAS_NEXT_BSP_ENTITY: unnamed = 310;
    pub const BOTLIB_AAS_POINT_CONTENTS: unnamed = 309;
    pub const BOTLIB_AAS_TRACE_AREAS: unnamed = 308;
    pub const BOTLIB_AAS_POINT_AREA_NUM: unnamed = 307;
    pub const BOTLIB_AAS_TIME: unnamed = 306;
    pub const BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX: unnamed = 305;
    pub const BOTLIB_AAS_INITIALIZED: unnamed = 304;
    pub const BOTLIB_AAS_ENTITY_INFO: unnamed = 303;
    pub const BOTLIB_AAS_AREA_INFO: unnamed = 302;
    pub const BOTLIB_AAS_BBOX_AREAS: unnamed = 301;
    pub const BOTLIB_AAS_ENABLE_ROUTING_AREA: unnamed = 300;
    // ( int client, usercmd_t *ucmd );
    pub const BOTLIB_USER_COMMAND: unnamed = 211;
    // ( int client, char *message, int size );
    pub const BOTLIB_GET_CONSOLE_MESSAGE: unnamed = 210;
    // ( int client, int ent );
    pub const BOTLIB_GET_SNAPSHOT_ENTITY: unnamed = 209;
    pub const BOTLIB_TEST: unnamed = 208;
    pub const BOTLIB_UPDATENTITY: unnamed = 207;
    pub const BOTLIB_LOAD_MAP: unnamed = 206;
    pub const BOTLIB_START_FRAME: unnamed = 205;
    pub const BOTLIB_PC_ADD_GLOBAL_DEFINE: unnamed = 204;
    pub const BOTLIB_LIBVAR_GET: unnamed = 203;
    pub const BOTLIB_LIBVAR_SET: unnamed = 202;
    // ( void );
    pub const BOTLIB_SHUTDOWN: unnamed = 201;
    // ( void );
    pub const BOTLIB_SETUP: unnamed = 200;
    // 1.32
    pub const G_FS_SEEK: unnamed = 45;
    // ( const vec3_t mins, const vec3_t maxs, const gentity_t *ent );
    pub const G_ENTITY_CONTACTCAPSULE: unnamed = 44;
    // ( trace_t *results, const vec3_t start, const vec3_t mins, const vec3_t maxs, const vec3_t end, int passEntityNum, int contentmask );
    pub const G_TRACECAPSULE: unnamed = 43;
    pub const G_SNAPVECTOR: unnamed = 42;
    pub const G_REAL_TIME: unnamed = 41;
    pub const G_DEBUG_POLYGON_DELETE: unnamed = 40;
    pub const G_DEBUG_POLYGON_CREATE: unnamed = 39;
    // Retrieves the next string token from the entity spawn text, returning
	// false when all tokens have been parsed.
	// This should only be done at GAME_INIT time.
    pub const G_FS_GETFILELIST: unnamed = 38;
    // qboolean ( char *buffer, int bufferSize )
    pub const G_GET_ENTITY_TOKEN: unnamed = 37;
    // ( int clientNum, usercmd_t *cmd )
    pub const G_GET_USERCMD: unnamed = 36;
    // ( int clientNum );
    pub const G_BOT_FREE_CLIENT: unnamed = 35;
    // perform an exact check against inline brush models of non-square shape
    // access for bots to get and free a server client (FIXME?)
    // ( void );
    pub const G_BOT_ALLOCATE_CLIENT: unnamed = 34;
    // EntitiesInBox will return brush models based on their bounding box,
	// so exact determination must still be done with EntityContact
    // ( const vec3_t mins, const vec3_t maxs, const gentity_t *ent );
    pub const G_ENTITY_CONTACT: unnamed = 33;
    // call before removing an interactive entity
    // ( const vec3_t mins, const vec3_t maxs, gentity_t **list, int maxcount );
    pub const G_ENTITIES_IN_BOX: unnamed = 32;
    // an entity will never be sent to a client or used for collision
	// if it is not passed to linkentity.  If the size, position, or
	// solidity changes, it must be relinked.
    // ( gentity_t *ent );		
    pub const G_UNLINKENTITY: unnamed = 31;
    // ( gentity_t *ent );
    pub const G_LINKENTITY: unnamed = 30;
    // ( int area1, int area2 );
    pub const G_AREAS_CONNECTED: unnamed = 29;
    // ( gentity_t *ent, qboolean open );
    pub const G_ADJUST_AREA_PORTAL_STATE: unnamed = 28;
    // ( const vec3_t p1, const vec3_t p2 );
    pub const G_IN_PVS_IGNORE_PORTALS: unnamed = 27;
    // point contents against all linked entities
    // ( const vec3_t p1, const vec3_t p2 );
    pub const G_IN_PVS: unnamed = 26;
    // collision detection against all linked entities
    // ( const vec3_t point, int passEntityNum );
    pub const G_POINT_CONTENTS: unnamed = 25;
    // sets mins and maxs based on the brushmodel name
    // ( trace_t *results, const vec3_t start, const vec3_t mins, const vec3_t maxs, const vec3_t end, int passEntityNum, int contentmask );
    pub const G_TRACE: unnamed = 24;
    // the serverinfo info string has all the cvars visible to server browsers
    // ( gentity_t *ent, const char *name );
    pub const G_SET_BRUSH_MODEL: unnamed = 23;
    // ( char *buffer, int bufferSize );
    pub const G_GET_SERVERINFO: unnamed = 22;
    // userinfo strings are maintained by the server system, so they
	// are persistant across level loads, while all other game visible
	// data is completely reset
    // ( int num, const char *buffer );
    pub const G_SET_USERINFO: unnamed = 21;
    // ( int num, char *buffer, int bufferSize );
    pub const G_GET_USERINFO: unnamed = 20;
    // config strings hold all the index strings, and various other information
	// that is reliably communicated to all clients
	// All of the current configstrings are sent to clients when
	// they connect, and changes are sent to all connected clients.
	// All confgstrings are cleared at each level start.
    // ( int num, char *buffer, int bufferSize );
    pub const G_GET_CONFIGSTRING: unnamed = 19;
    // reliably sends a command string to be interpreted by the given
	// client.  If clientNum is -1, it will be sent to all clients
    // ( int num, const char *string );
    pub const G_SET_CONFIGSTRING: unnamed = 18;
    // kick a client off the server with a message
    // ( int clientNum, const char *fmt, ... );
    pub const G_SEND_SERVER_COMMAND: unnamed = 17;
    //							playerState_t *clients, int sizeofGameClient );
	// the game needs to let the server system know where and how big the gentities
	// are, so it can look at them directly without going through an interface
    // ( int clientNum, const char *reason );
    pub const G_DROP_CLIENT: unnamed = 16;
    // add commands to the console as if they were typed in
	// for map changing, etc
    //=========== server specific functionality =============
    // ( gentity_t *gEnts, int numGEntities, int sizeofGEntity_t,
    pub const G_LOCATE_GAME_DATA: unnamed = 15;
    // ( const char *text );
    pub const G_SEND_CONSOLE_COMMAND: unnamed = 14;
    // ( fileHandle_t f );
    pub const G_FS_FCLOSE_FILE: unnamed = 13;
    // ( const void *buffer, int len, fileHandle_t f );
    pub const G_FS_WRITE: unnamed = 12;
    // ( void *buffer, int len, fileHandle_t f );
    pub const G_FS_READ: unnamed = 11;
    // ( const char *qpath, fileHandle_t *file, fsMode_t mode );
    pub const G_FS_FOPEN_FILE: unnamed = 10;
    // ClientCommand and ServerCommand parameter access
    // ( int n, char *buffer, int bufferLength );
    pub const G_ARGV: unnamed = 9;
    // ( void );
    pub const G_ARGC: unnamed = 8;
    // ( const char *var_name, char *buffer, int bufsize );
    pub const G_CVAR_VARIABLE_STRING_BUFFER: unnamed = 7;
    // ( const char *var_name );
    pub const G_CVAR_VARIABLE_INTEGER_VALUE: unnamed = 6;
    // ( const char *var_name, const char *value );
    pub const G_CVAR_SET: unnamed = 5;
    // ( vmCvar_t *vmCvar );
    pub const G_CVAR_UPDATE: unnamed = 4;
    // get current time for profiling reasons
	// this should NOT be used for any game related tasks,
	// because it is not journaled
    // console variable interaction
    // ( vmCvar_t *vmCvar, const char *varName, const char *defaultValue, int flags );
    pub const G_CVAR_REGISTER: unnamed = 3;
    // abort the game
    // ( void );
    pub const G_MILLISECONDS: unnamed = 2;
    // print message on the local console
    // ( const char *string );
    pub const G_ERROR: unnamed = 1;
    //============== general Quake services ==================
    // ( const char *string );
    pub const G_PRINT: unnamed = 0;
    use super::q_shared_h::{entityState_t, qboolean, vec3_t};
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/g_local.h"]
pub mod g_local_h {
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
// g_local.h -- local definitions for game module
    //==================================================================
    // the "gameversion" client command will print this plus compile date
    // msec
    // gentity->flags
    // not the first on the team
    // spawn point not for bot use
    // spawn point just for bots
    // force gesture on client
    // movers are things like doors, plats, buttons, etc
    pub type moverState_t = libc::c_uint;
    pub const MOVER_2TO1: moverState_t = 3;
    pub const MOVER_1TO2: moverState_t = 2;
    pub const MOVER_POS2: moverState_t = 1;
    pub const MOVER_POS1: moverState_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
        pub blocked: Option<unsafe extern "C" fn(_: *mut gentity_t,
                                                 _: *mut gentity_t) -> ()>,
        pub touch: Option<unsafe extern "C" fn(_: *mut gentity_t,
                                               _: *mut gentity_t,
                                               _: *mut trace_t) -> ()>,
        pub use_0: Option<unsafe extern "C" fn(_: *mut gentity_t,
                                               _: *mut gentity_t,
                                               _: *mut gentity_t) -> ()>,
        pub pain: Option<unsafe extern "C" fn(_: *mut gentity_t,
                                              _: *mut gentity_t,
                                              _: libc::c_int) -> ()>,
        pub die: Option<unsafe extern "C" fn(_: *mut gentity_t,
                                             _: *mut gentity_t,
                                             _: *mut gentity_t,
                                             _: libc::c_int, _: libc::c_int)
                            -> ()>,
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
    //============================================================================
    pub type gentity_t = gentity_s;
    // this structure is cleared on each ClientSpawn(),
// except for 'client->pers' and 'client->sess'
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
    // client data that stays across multiple levels or tournament restarts
// this is achieved by writing all the data to cvar strings at game shutdown
// time and reading them back at connection time.  Anything added here
// MUST be dealt with in G_InitSessionData() / G_ReadSessionData() / G_WriteSessionData()
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientSession_t {
        pub sessionTeam: team_t,
        pub spectatorNum: libc::c_int,
        pub spectatorState: spectatorState_t,
        pub spectatorClient: libc::c_int,
        pub wins: libc::c_int,
        pub losses: libc::c_int,
        pub teamLeader: qboolean,
    }
    pub type spectatorState_t = libc::c_uint;
    pub const SPECTATOR_SCOREBOARD: spectatorState_t = 3;
    pub const SPECTATOR_FOLLOW: spectatorState_t = 2;
    pub const SPECTATOR_FREE: spectatorState_t = 1;
    pub const SPECTATOR_NOT: spectatorState_t = 0;
    //
    // client data that stays across multiple respawns, but is cleared
// on each level change or team change at ClientBegin()
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
    pub type playerTeamStateState_t = libc::c_uint;
    // Now actively playing
    pub const TEAM_ACTIVE: playerTeamStateState_t = 1;
    // Beginning a team game, spawn at base
    pub const TEAM_BEGIN: playerTeamStateState_t = 0;
    pub type clientConnected_t = libc::c_uint;
    pub const CON_CONNECTED: clientConnected_t = 2;
    pub const CON_CONNECTING: clientConnected_t = 1;
    pub const CON_DISCONNECTED: clientConnected_t = 0;
    use super::{libc};
    use super::q_shared_h::{entityState_t, qboolean, vec3_t, trace_t,
                            playerState_t, usercmd_t, qtime_t, fileHandle_t,
                            fsMode_t, vmCvar_t, vec_t};
    use super::g_public_h::{entityShared_t};
    use super::bg_public_h::{gitem_t, team_t};
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn exit(_: libc::c_int) -> !;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/g_syscalls.c"]
pub mod g_syscalls_c {
    use super::{libc};
    use super::stdint_h::{intptr_t};
    use super::q_shared_h::{trace_t, vec_t, qboolean, pc_token_t};
    use super::g_local_h::{gentity_t};
}
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, floatint_t,
                       fileHandle_t, vec_t, vec3_t, cplane_s, pc_token_s,
                       pc_token_t, fsMode_t, FS_APPEND_SYNC, FS_APPEND,
                       FS_WRITE, FS_READ, cvarHandle_t, vmCvar_t, cplane_t,
                       trace_t, playerState_s, playerState_t, usercmd_s,
                       usercmd_t, trType_t, TR_GRAVITY, TR_SINE,
                       TR_LINEAR_STOP, TR_LINEAR, TR_INTERPOLATE,
                       TR_STATIONARY, trajectory_t, entityState_s,
                       entityState_t, qtime_s, qtime_t};
use self::bg_public_h::{team_t, TEAM_NUM_TEAMS, TEAM_SPECTATOR, TEAM_BLUE,
                        TEAM_RED, TEAM_FREE, itemType_t, IT_TEAM,
                        IT_PERSISTANT_POWERUP, IT_HOLDABLE, IT_POWERUP,
                        IT_HEALTH, IT_ARMOR, IT_AMMO, IT_WEAPON, IT_BAD,
                        gitem_s, gitem_t};
use self::g_public_h::{entityShared_t, unnamed,
                       BOTLIB_PC_SOURCE_FILE_AND_LINE, BOTLIB_PC_READ_TOKEN,
                       BOTLIB_PC_FREE_SOURCE, BOTLIB_PC_LOAD_SOURCE,
                       BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX,
                       BOTLIB_AAS_PREDICT_ROUTE,
                       BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL,
                       BOTLIB_AI_ADD_AVOID_SPOT,
                       BOTLIB_AI_SET_AVOID_GOAL_TIME,
                       BOTLIB_AI_PREDICT_VISIBLE_POSITION,
                       BOTLIB_AI_REMOVE_FROM_AVOID_GOALS,
                       BOTLIB_AI_GET_CHAT_MESSAGE,
                       BOTLIB_AI_NUM_INITIAL_CHATS,
                       BOTLIB_AI_GET_MAP_LOCATION_GOAL,
                       BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL,
                       BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC,
                       BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC,
                       BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION,
                       BOTLIB_AI_RESET_WEAPON_STATE,
                       BOTLIB_AI_FREE_WEAPON_STATE,
                       BOTLIB_AI_ALLOC_WEAPON_STATE,
                       BOTLIB_AI_LOAD_WEAPON_WEIGHTS,
                       BOTLIB_AI_GET_WEAPON_INFO,
                       BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON,
                       BOTLIB_AI_INIT_MOVE_STATE, BOTLIB_AI_FREE_MOVE_STATE,
                       BOTLIB_AI_ALLOC_MOVE_STATE,
                       BOTLIB_AI_MOVEMENT_VIEW_TARGET,
                       BOTLIB_AI_REACHABILITY_AREA,
                       BOTLIB_AI_RESET_LAST_AVOID_REACH,
                       BOTLIB_AI_RESET_AVOID_REACH,
                       BOTLIB_AI_MOVE_IN_DIRECTION, BOTLIB_AI_MOVE_TO_GOAL,
                       BOTLIB_AI_RESET_MOVE_STATE, BOTLIB_AI_FREE_GOAL_STATE,
                       BOTLIB_AI_ALLOC_GOAL_STATE,
                       BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC,
                       BOTLIB_AI_FREE_ITEM_WEIGHTS,
                       BOTLIB_AI_LOAD_ITEM_WEIGHTS,
                       BOTLIB_AI_UPDATE_ENTITY_ITEMS,
                       BOTLIB_AI_INIT_LEVEL_ITEMS, BOTLIB_AI_AVOID_GOAL_TIME,
                       BOTLIB_AI_GET_LEVEL_ITEM_GOAL,
                       BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE,
                       BOTLIB_AI_TOUCHING_GOAL, BOTLIB_AI_CHOOSE_NBG_ITEM,
                       BOTLIB_AI_CHOOSE_LTG_ITEM, BOTLIB_AI_GET_SECOND_GOAL,
                       BOTLIB_AI_GET_TOP_GOAL, BOTLIB_AI_GOAL_NAME,
                       BOTLIB_AI_DUMP_GOAL_STACK, BOTLIB_AI_DUMP_AVOID_GOALS,
                       BOTLIB_AI_EMPTY_GOAL_STACK, BOTLIB_AI_POP_GOAL,
                       BOTLIB_AI_PUSH_GOAL, BOTLIB_AI_RESET_AVOID_GOALS,
                       BOTLIB_AI_RESET_GOAL_STATE, BOTLIB_AI_SET_CHAT_NAME,
                       BOTLIB_AI_SET_CHAT_GENDER, BOTLIB_AI_LOAD_CHAT_FILE,
                       BOTLIB_AI_REPLACE_SYNONYMS,
                       BOTLIB_AI_UNIFY_WHITE_SPACES, BOTLIB_AI_MATCH_VARIABLE,
                       BOTLIB_AI_FIND_MATCH, BOTLIB_AI_STRING_CONTAINS,
                       BOTLIB_AI_ENTER_CHAT, BOTLIB_AI_CHAT_LENGTH,
                       BOTLIB_AI_REPLY_CHAT, BOTLIB_AI_INITIAL_CHAT,
                       BOTLIB_AI_NUM_CONSOLE_MESSAGE,
                       BOTLIB_AI_NEXT_CONSOLE_MESSAGE,
                       BOTLIB_AI_REMOVE_CONSOLE_MESSAGE,
                       BOTLIB_AI_QUEUE_CONSOLE_MESSAGE,
                       BOTLIB_AI_FREE_CHAT_STATE, BOTLIB_AI_ALLOC_CHAT_STATE,
                       BOTLIB_AI_CHARACTERISTIC_STRING,
                       BOTLIB_AI_CHARACTERISTIC_BINTEGER,
                       BOTLIB_AI_CHARACTERISTIC_INTEGER,
                       BOTLIB_AI_CHARACTERISTIC_BFLOAT,
                       BOTLIB_AI_CHARACTERISTIC_FLOAT,
                       BOTLIB_AI_FREE_CHARACTER, BOTLIB_AI_LOAD_CHARACTER,
                       BOTLIB_EA_RESET_INPUT, BOTLIB_EA_GET_INPUT,
                       BOTLIB_EA_END_REGULAR, BOTLIB_EA_VIEW, BOTLIB_EA_MOVE,
                       BOTLIB_EA_DELAYED_JUMP, BOTLIB_EA_JUMP,
                       BOTLIB_EA_SELECT_WEAPON, BOTLIB_EA_MOVE_RIGHT,
                       BOTLIB_EA_MOVE_LEFT, BOTLIB_EA_MOVE_BACK,
                       BOTLIB_EA_MOVE_FORWARD, BOTLIB_EA_MOVE_DOWN,
                       BOTLIB_EA_MOVE_UP, BOTLIB_EA_CROUCH, BOTLIB_EA_RESPAWN,
                       BOTLIB_EA_USE, BOTLIB_EA_ATTACK, BOTLIB_EA_TALK,
                       BOTLIB_EA_GESTURE, BOTLIB_EA_ACTION, BOTLIB_EA_COMMAND,
                       BOTLIB_EA_SAY_TEAM, BOTLIB_EA_SAY,
                       BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT,
                       BOTLIB_AAS_SWIMMING,
                       BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA,
                       BOTLIB_AAS_AREA_REACHABILITY,
                       BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY,
                       BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY,
                       BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY,
                       BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY,
                       BOTLIB_AAS_NEXT_BSP_ENTITY, BOTLIB_AAS_POINT_CONTENTS,
                       BOTLIB_AAS_TRACE_AREAS, BOTLIB_AAS_POINT_AREA_NUM,
                       BOTLIB_AAS_TIME, BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX,
                       BOTLIB_AAS_INITIALIZED, BOTLIB_AAS_ENTITY_INFO,
                       BOTLIB_AAS_AREA_INFO, BOTLIB_AAS_BBOX_AREAS,
                       BOTLIB_AAS_ENABLE_ROUTING_AREA, BOTLIB_USER_COMMAND,
                       BOTLIB_GET_CONSOLE_MESSAGE, BOTLIB_GET_SNAPSHOT_ENTITY,
                       BOTLIB_TEST, BOTLIB_UPDATENTITY, BOTLIB_LOAD_MAP,
                       BOTLIB_START_FRAME, BOTLIB_PC_ADD_GLOBAL_DEFINE,
                       BOTLIB_LIBVAR_GET, BOTLIB_LIBVAR_SET, BOTLIB_SHUTDOWN,
                       BOTLIB_SETUP, G_FS_SEEK, G_ENTITY_CONTACTCAPSULE,
                       G_TRACECAPSULE, G_SNAPVECTOR, G_REAL_TIME,
                       G_DEBUG_POLYGON_DELETE, G_DEBUG_POLYGON_CREATE,
                       G_FS_GETFILELIST, G_GET_ENTITY_TOKEN, G_GET_USERCMD,
                       G_BOT_FREE_CLIENT, G_BOT_ALLOCATE_CLIENT,
                       G_ENTITY_CONTACT, G_ENTITIES_IN_BOX, G_UNLINKENTITY,
                       G_LINKENTITY, G_AREAS_CONNECTED,
                       G_ADJUST_AREA_PORTAL_STATE, G_IN_PVS_IGNORE_PORTALS,
                       G_IN_PVS, G_POINT_CONTENTS, G_TRACE, G_SET_BRUSH_MODEL,
                       G_GET_SERVERINFO, G_SET_USERINFO, G_GET_USERINFO,
                       G_GET_CONFIGSTRING, G_SET_CONFIGSTRING,
                       G_SEND_SERVER_COMMAND, G_DROP_CLIENT,
                       G_LOCATE_GAME_DATA, G_SEND_CONSOLE_COMMAND,
                       G_FS_FCLOSE_FILE, G_FS_WRITE, G_FS_READ,
                       G_FS_FOPEN_FILE, G_ARGV, G_ARGC,
                       G_CVAR_VARIABLE_STRING_BUFFER,
                       G_CVAR_VARIABLE_INTEGER_VALUE, G_CVAR_SET,
                       G_CVAR_UPDATE, G_CVAR_REGISTER, G_MILLISECONDS,
                       G_ERROR, G_PRINT};
use self::g_local_h::{moverState_t, MOVER_2TO1, MOVER_1TO2, MOVER_POS2,
                      MOVER_POS1, gentity_s, gentity_t, gclient_s,
                      clientSession_t, spectatorState_t, SPECTATOR_SCOREBOARD,
                      SPECTATOR_FOLLOW, SPECTATOR_FREE, SPECTATOR_NOT,
                      clientPersistant_t, playerTeamState_t,
                      playerTeamStateState_t, TEAM_ACTIVE, TEAM_BEGIN,
                      clientConnected_t, CON_CONNECTED, CON_CONNECTING,
                      CON_DISCONNECTED};
use self::stdlib_h::{exit};
#[no_mangle]
pub unsafe extern "C" fn trap_Print(mut text: *const libc::c_char) {
    syscall.expect("non-null function pointer")(G_PRINT as libc::c_int as
                                                    intptr_t, text);
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
// this file is only included when building a dll
// g_syscalls.asm is included instead when building a qvm
static mut syscall: Option<unsafe extern "C" fn(_: intptr_t, ...) -> intptr_t>
       =
    ::std::mem::transmute::<libc::intptr_t,
                            Option<unsafe extern "C" fn(_: intptr_t, ...)
                                       -> intptr_t>>(-1i32 as libc::intptr_t);
#[no_mangle]
pub unsafe extern "C" fn trap_Error(mut text: *const libc::c_char) -> ! {
    syscall.expect("non-null function pointer")(G_ERROR as libc::c_int as
                                                    intptr_t, text);
    exit(1i32);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Milliseconds() -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_MILLISECONDS as
                                                           libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_RealTime(mut qtime: *mut qtime_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_REAL_TIME as
                                                           libc::c_int as
                                                           intptr_t, qtime) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Argc() -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_ARGC as libc::c_int
                                                           as intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Argv(mut n: libc::c_int,
                                   mut buffer: *mut libc::c_char,
                                   mut bufferLength: libc::c_int) {
    syscall.expect("non-null function pointer")(G_ARGV as libc::c_int as
                                                    intptr_t, n, buffer,
                                                bufferLength);
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_FOpenFile(mut qpath: *const libc::c_char,
                                           mut f: *mut fileHandle_t,
                                           mut mode: fsMode_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_FS_FOPEN_FILE as
                                                           libc::c_int as
                                                           intptr_t, qpath, f,
                                                       mode as libc::c_uint)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_Read(mut buffer: *mut libc::c_void,
                                      mut len: libc::c_int,
                                      mut f: fileHandle_t) {
    syscall.expect("non-null function pointer")(G_FS_READ as libc::c_int as
                                                    intptr_t, buffer, len, f);
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_Write(mut buffer: *const libc::c_void,
                                       mut len: libc::c_int,
                                       mut f: fileHandle_t) {
    syscall.expect("non-null function pointer")(G_FS_WRITE as libc::c_int as
                                                    intptr_t, buffer, len, f);
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_FCloseFile(mut f: fileHandle_t) {
    syscall.expect("non-null function pointer")(G_FS_FCLOSE_FILE as
                                                    libc::c_int as intptr_t,
                                                f);
}
#[no_mangle]
pub unsafe extern "C" fn trap_FS_GetFileList(mut path: *const libc::c_char,
                                             mut extension:
                                                 *const libc::c_char,
                                             mut listbuf: *mut libc::c_char,
                                             mut bufsize: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_FS_GETFILELIST as
                                                           libc::c_int as
                                                           intptr_t, path,
                                                       extension, listbuf,
                                                       bufsize) as
               libc::c_int;
}
// fsOrigin_t
#[no_mangle]
pub unsafe extern "C" fn trap_FS_Seek(mut f: fileHandle_t,
                                      mut offset: libc::c_long,
                                      mut origin: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_FS_SEEK as
                                                           libc::c_int as
                                                           intptr_t, f,
                                                       offset, origin) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_SendConsoleCommand(mut exec_when: libc::c_int,
                                                 mut text:
                                                     *const libc::c_char) {
    syscall.expect("non-null function pointer")(G_SEND_CONSOLE_COMMAND as
                                                    libc::c_int as intptr_t,
                                                exec_when, text);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Register(mut cvar: *mut vmCvar_t,
                                            mut var_name: *const libc::c_char,
                                            mut value: *const libc::c_char,
                                            mut flags: libc::c_int) {
    syscall.expect("non-null function pointer")(G_CVAR_REGISTER as libc::c_int
                                                    as intptr_t, cvar,
                                                var_name, value, flags);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Update(mut cvar: *mut vmCvar_t) {
    syscall.expect("non-null function pointer")(G_CVAR_UPDATE as libc::c_int
                                                    as intptr_t, cvar);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_Set(mut var_name: *const libc::c_char,
                                       mut value: *const libc::c_char) {
    syscall.expect("non-null function pointer")(G_CVAR_SET as libc::c_int as
                                                    intptr_t, var_name,
                                                value);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_VariableIntegerValue(mut var_name:
                                                            *const libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_CVAR_VARIABLE_INTEGER_VALUE
                                                           as libc::c_int as
                                                           intptr_t, var_name)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Cvar_VariableStringBuffer(mut var_name:
                                                            *const libc::c_char,
                                                        mut buffer:
                                                            *mut libc::c_char,
                                                        mut bufsize:
                                                            libc::c_int) {
    syscall.expect("non-null function pointer")(G_CVAR_VARIABLE_STRING_BUFFER
                                                    as libc::c_int as
                                                    intptr_t, var_name,
                                                buffer, bufsize);
}
#[no_mangle]
pub unsafe extern "C" fn trap_LocateGameData(mut gEnts: *mut gentity_t,
                                             mut numGEntities: libc::c_int,
                                             mut sizeofGEntity_t: libc::c_int,
                                             mut clients: *mut playerState_t,
                                             mut sizeofGClient: libc::c_int) {
    syscall.expect("non-null function pointer")(G_LOCATE_GAME_DATA as
                                                    libc::c_int as intptr_t,
                                                gEnts, numGEntities,
                                                sizeofGEntity_t, clients,
                                                sizeofGClient);
}
#[no_mangle]
pub unsafe extern "C" fn trap_DropClient(mut clientNum: libc::c_int,
                                         mut reason: *const libc::c_char) {
    syscall.expect("non-null function pointer")(G_DROP_CLIENT as libc::c_int
                                                    as intptr_t, clientNum,
                                                reason);
}
#[no_mangle]
pub unsafe extern "C" fn trap_SendServerCommand(mut clientNum: libc::c_int,
                                                mut text:
                                                    *const libc::c_char) {
    syscall.expect("non-null function pointer")(G_SEND_SERVER_COMMAND as
                                                    libc::c_int as intptr_t,
                                                clientNum, text);
}
#[no_mangle]
pub unsafe extern "C" fn trap_SetConfigstring(mut num: libc::c_int,
                                              mut string:
                                                  *const libc::c_char) {
    syscall.expect("non-null function pointer")(G_SET_CONFIGSTRING as
                                                    libc::c_int as intptr_t,
                                                num, string);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetConfigstring(mut num: libc::c_int,
                                              mut buffer: *mut libc::c_char,
                                              mut bufferSize: libc::c_int) {
    syscall.expect("non-null function pointer")(G_GET_CONFIGSTRING as
                                                    libc::c_int as intptr_t,
                                                num, buffer, bufferSize);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetUserinfo(mut num: libc::c_int,
                                          mut buffer: *mut libc::c_char,
                                          mut bufferSize: libc::c_int) {
    syscall.expect("non-null function pointer")(G_GET_USERINFO as libc::c_int
                                                    as intptr_t, num, buffer,
                                                bufferSize);
}
#[no_mangle]
pub unsafe extern "C" fn trap_SetUserinfo(mut num: libc::c_int,
                                          mut buffer: *const libc::c_char) {
    syscall.expect("non-null function pointer")(G_SET_USERINFO as libc::c_int
                                                    as intptr_t, num, buffer);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetServerinfo(mut buffer: *mut libc::c_char,
                                            mut bufferSize: libc::c_int) {
    syscall.expect("non-null function pointer")(G_GET_SERVERINFO as
                                                    libc::c_int as intptr_t,
                                                buffer, bufferSize);
}
#[no_mangle]
pub unsafe extern "C" fn trap_SetBrushModel(mut ent: *mut gentity_t,
                                            mut name: *const libc::c_char) {
    syscall.expect("non-null function pointer")(G_SET_BRUSH_MODEL as
                                                    libc::c_int as intptr_t,
                                                ent, name);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Trace(mut results: *mut trace_t,
                                    mut start: *const vec_t,
                                    mut mins: *const vec_t,
                                    mut maxs: *const vec_t,
                                    mut end: *const vec_t,
                                    mut passEntityNum: libc::c_int,
                                    mut contentmask: libc::c_int) {
    syscall.expect("non-null function pointer")(G_TRACE as libc::c_int as
                                                    intptr_t, results, start,
                                                mins, maxs, end,
                                                passEntityNum, contentmask);
}
#[no_mangle]
pub unsafe extern "C" fn trap_PointContents(mut point: *const vec_t,
                                            mut passEntityNum: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_POINT_CONTENTS as
                                                           libc::c_int as
                                                           intptr_t, point,
                                                       passEntityNum) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_InPVS(mut p1: *const vec_t,
                                    mut p2: *const vec_t) -> qboolean {
    return syscall.expect("non-null function pointer")(G_IN_PVS as libc::c_int
                                                           as intptr_t, p1,
                                                       p2) as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_InPVSIgnorePortals(mut p1: *const vec_t,
                                                 mut p2: *const vec_t)
 -> qboolean {
    return syscall.expect("non-null function pointer")(G_IN_PVS_IGNORE_PORTALS
                                                           as libc::c_int as
                                                           intptr_t, p1, p2)
               as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AdjustAreaPortalState(mut ent: *mut gentity_t,
                                                    mut open: qboolean) {
    syscall.expect("non-null function pointer")(G_ADJUST_AREA_PORTAL_STATE as
                                                    libc::c_int as intptr_t,
                                                ent, open as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn trap_AreasConnected(mut area1: libc::c_int,
                                             mut area2: libc::c_int)
 -> qboolean {
    return syscall.expect("non-null function pointer")(G_AREAS_CONNECTED as
                                                           libc::c_int as
                                                           intptr_t, area1,
                                                       area2) as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_LinkEntity(mut ent: *mut gentity_t) {
    syscall.expect("non-null function pointer")(G_LINKENTITY as libc::c_int as
                                                    intptr_t, ent);
}
#[no_mangle]
pub unsafe extern "C" fn trap_UnlinkEntity(mut ent: *mut gentity_t) {
    syscall.expect("non-null function pointer")(G_UNLINKENTITY as libc::c_int
                                                    as intptr_t, ent);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EntitiesInBox(mut mins: *const vec_t,
                                            mut maxs: *const vec_t,
                                            mut list: *mut libc::c_int,
                                            mut maxcount: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_ENTITIES_IN_BOX as
                                                           libc::c_int as
                                                           intptr_t, mins,
                                                       maxs, list, maxcount)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_EntityContact(mut mins: *const vec_t,
                                            mut maxs: *const vec_t,
                                            mut ent: *const gentity_t)
 -> qboolean {
    return syscall.expect("non-null function pointer")(G_ENTITY_CONTACT as
                                                           libc::c_int as
                                                           intptr_t, mins,
                                                       maxs, ent) as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotAllocateClient() -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_BOT_ALLOCATE_CLIENT
                                                           as libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotFreeClient(mut clientNum: libc::c_int) {
    syscall.expect("non-null function pointer")(G_BOT_FREE_CLIENT as
                                                    libc::c_int as intptr_t,
                                                clientNum);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetUsercmd(mut clientNum: libc::c_int,
                                         mut cmd: *mut usercmd_t) {
    syscall.expect("non-null function pointer")(G_GET_USERCMD as libc::c_int
                                                    as intptr_t, clientNum,
                                                cmd);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GetEntityToken(mut buffer: *mut libc::c_char,
                                             mut bufferSize: libc::c_int)
 -> qboolean {
    return syscall.expect("non-null function pointer")(G_GET_ENTITY_TOKEN as
                                                           libc::c_int as
                                                           intptr_t, buffer,
                                                       bufferSize) as
               qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_DebugPolygonCreate(mut color: libc::c_int,
                                                 mut numPoints: libc::c_int,
                                                 mut points: *mut vec3_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(G_DEBUG_POLYGON_CREATE
                                                           as libc::c_int as
                                                           intptr_t, color,
                                                       numPoints, points) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_DebugPolygonDelete(mut id: libc::c_int) {
    syscall.expect("non-null function pointer")(G_DEBUG_POLYGON_DELETE as
                                                    libc::c_int as intptr_t,
                                                id);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLibSetup() -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_SETUP as
                                                           libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLibShutdown() -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_SHUTDOWN as
                                                           libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLibVarSet(mut var_name: *mut libc::c_char,
                                           mut value: *mut libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_LIBVAR_SET as
                                                           libc::c_int as
                                                           intptr_t, var_name,
                                                       value) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLibVarGet(mut var_name: *mut libc::c_char,
                                           mut value: *mut libc::c_char,
                                           mut size: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_LIBVAR_GET as
                                                           libc::c_int as
                                                           intptr_t, var_name,
                                                       value, size) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLibDefine(mut string: *mut libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_PC_ADD_GLOBAL_DEFINE
                                                           as libc::c_int as
                                                           intptr_t, string)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLibStartFrame(mut time: libc::c_float)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_START_FRAME as
                                                           libc::c_int as
                                                           intptr_t,
                                                       PASSFLOAT(time)) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn PASSFLOAT(mut x: libc::c_float) -> libc::c_int {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.f = x;
    return fi.i;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLibLoadMap(mut mapname: *const libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_LOAD_MAP as
                                                           libc::c_int as
                                                           intptr_t, mapname)
               as libc::c_int;
}
/* struct bot_updateentity_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotLibUpdateEntity(mut ent: libc::c_int,
                                                 mut bue: *mut libc::c_void)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_UPDATENTITY as
                                                           libc::c_int as
                                                           intptr_t, ent, bue)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLibTest(mut parm0: libc::c_int,
                                         mut parm1: *mut libc::c_char,
                                         mut parm2: *mut vec_t,
                                         mut parm3: *mut vec_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_TEST as
                                                           libc::c_int as
                                                           intptr_t, parm0,
                                                       parm1, parm2, parm3) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotGetSnapshotEntity(mut clientNum: libc::c_int,
                                                   mut sequence: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_GET_SNAPSHOT_ENTITY
                                                           as libc::c_int as
                                                           intptr_t,
                                                       clientNum, sequence) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotGetServerCommand(mut clientNum: libc::c_int,
                                                  mut message:
                                                      *mut libc::c_char,
                                                  mut size: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_GET_CONSOLE_MESSAGE
                                                           as libc::c_int as
                                                           intptr_t,
                                                       clientNum, message,
                                                       size) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotUserCommand(mut clientNum: libc::c_int,
                                             mut ucmd: *mut usercmd_t) {
    syscall.expect("non-null function pointer")(BOTLIB_USER_COMMAND as
                                                    libc::c_int as intptr_t,
                                                clientNum, ucmd);
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_BBoxAreas(mut absmins: *mut vec_t,
                                            mut absmaxs: *mut vec_t,
                                            mut areas: *mut libc::c_int,
                                            mut maxareas: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_BBOX_AREAS
                                                           as libc::c_int as
                                                           intptr_t, absmins,
                                                       absmaxs, areas,
                                                       maxareas) as
               libc::c_int;
}
/* struct aas_areainfo_s */
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_AreaInfo(mut areanum: libc::c_int,
                                           mut info: *mut libc::c_void)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_AREA_INFO as
                                                           libc::c_int as
                                                           intptr_t, areanum,
                                                       info) as libc::c_int;
}
/* struct aas_entityinfo_s */
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_EntityInfo(mut entnum: libc::c_int,
                                             mut info: *mut libc::c_void) {
    syscall.expect("non-null function pointer")(BOTLIB_AAS_ENTITY_INFO as
                                                    libc::c_int as intptr_t,
                                                entnum, info);
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_Initialized() -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_INITIALIZED
                                                           as libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_PresenceTypeBoundingBox(mut presencetype:
                                                              libc::c_int,
                                                          mut mins:
                                                              *mut vec_t,
                                                          mut maxs:
                                                              *mut vec_t) {
    syscall.expect("non-null function pointer")(BOTLIB_AAS_PRESENCE_TYPE_BOUNDING_BOX
                                                    as libc::c_int as
                                                    intptr_t, presencetype,
                                                mins, maxs);
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_Time() -> libc::c_float {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.i =
        syscall.expect("non-null function pointer")(BOTLIB_AAS_TIME as
                                                        libc::c_int as
                                                        intptr_t) as
            libc::c_int;
    return fi.f;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_PointAreaNum(mut point: *mut vec_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_POINT_AREA_NUM
                                                           as libc::c_int as
                                                           intptr_t, point) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_PointReachabilityAreaIndex(mut point:
                                                                 *mut vec_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_POINT_REACHABILITY_AREA_INDEX
                                                           as libc::c_int as
                                                           intptr_t, point) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_TraceAreas(mut start: *mut vec_t,
                                             mut end: *mut vec_t,
                                             mut areas: *mut libc::c_int,
                                             mut points: *mut vec3_t,
                                             mut maxareas: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_TRACE_AREAS
                                                           as libc::c_int as
                                                           intptr_t, start,
                                                       end, areas, points,
                                                       maxareas) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_PointContents(mut point: *mut vec_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_POINT_CONTENTS
                                                           as libc::c_int as
                                                           intptr_t, point) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_NextBSPEntity(mut ent: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_NEXT_BSP_ENTITY
                                                           as libc::c_int as
                                                           intptr_t, ent) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_ValueForBSPEpairKey(mut ent: libc::c_int,
                                                      mut key:
                                                          *mut libc::c_char,
                                                      mut value:
                                                          *mut libc::c_char,
                                                      mut size: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_VALUE_FOR_BSP_EPAIR_KEY
                                                           as libc::c_int as
                                                           intptr_t, ent, key,
                                                       value, size) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_VectorForBSPEpairKey(mut ent: libc::c_int,
                                                       mut key:
                                                           *mut libc::c_char,
                                                       mut v: *mut vec_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_VECTOR_FOR_BSP_EPAIR_KEY
                                                           as libc::c_int as
                                                           intptr_t, ent, key,
                                                       v) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_FloatForBSPEpairKey(mut ent: libc::c_int,
                                                      mut key:
                                                          *mut libc::c_char,
                                                      mut value:
                                                          *mut libc::c_float)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_FLOAT_FOR_BSP_EPAIR_KEY
                                                           as libc::c_int as
                                                           intptr_t, ent, key,
                                                       value) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_IntForBSPEpairKey(mut ent: libc::c_int,
                                                    mut key:
                                                        *mut libc::c_char,
                                                    mut value:
                                                        *mut libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_INT_FOR_BSP_EPAIR_KEY
                                                           as libc::c_int as
                                                           intptr_t, ent, key,
                                                       value) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_AreaReachability(mut areanum: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_AREA_REACHABILITY
                                                           as libc::c_int as
                                                           intptr_t, areanum)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_AreaTravelTimeToGoalArea(mut areanum:
                                                               libc::c_int,
                                                           mut origin:
                                                               *mut vec_t,
                                                           mut goalareanum:
                                                               libc::c_int,
                                                           mut travelflags:
                                                               libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_AREA_TRAVEL_TIME_TO_GOAL_AREA
                                                           as libc::c_int as
                                                           intptr_t, areanum,
                                                       origin, goalareanum,
                                                       travelflags) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_EnableRoutingArea(mut areanum: libc::c_int,
                                                    mut enable: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_ENABLE_ROUTING_AREA
                                                           as libc::c_int as
                                                           intptr_t, areanum,
                                                       enable) as libc::c_int;
}
/*struct aas_predictroute_s*/
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_PredictRoute(mut route: *mut libc::c_void,
                                               mut areanum: libc::c_int,
                                               mut origin: *mut vec_t,
                                               mut goalareanum: libc::c_int,
                                               mut travelflags: libc::c_int,
                                               mut maxareas: libc::c_int,
                                               mut maxtime: libc::c_int,
                                               mut stopevent: libc::c_int,
                                               mut stopcontents: libc::c_int,
                                               mut stoptfl: libc::c_int,
                                               mut stopareanum: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_PREDICT_ROUTE
                                                           as libc::c_int as
                                                           intptr_t, route,
                                                       areanum, origin,
                                                       goalareanum,
                                                       travelflags, maxareas,
                                                       maxtime, stopevent,
                                                       stopcontents, stoptfl,
                                                       stopareanum) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_AlternativeRouteGoals(mut start: *mut vec_t,
                                                        mut startareanum:
                                                            libc::c_int,
                                                        mut goal: *mut vec_t,
                                                        mut goalareanum:
                                                            libc::c_int,
                                                        mut travelflags:
                                                            libc::c_int,
                                                        mut altroutegoals:
                                                            *mut libc::c_void,
                                                        mut maxaltroutegoals:
                                                            libc::c_int,
                                                        mut type_0:
                                                            libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_ALTERNATIVE_ROUTE_GOAL
                                                           as libc::c_int as
                                                           intptr_t, start,
                                                       startareanum, goal,
                                                       goalareanum,
                                                       travelflags,
                                                       altroutegoals,
                                                       maxaltroutegoals,
                                                       type_0) as libc::c_int;
}
/*struct aas_altroutegoal_s*/
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_Swimming(mut origin: *mut vec_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_SWIMMING as
                                                           libc::c_int as
                                                           intptr_t, origin)
               as libc::c_int;
}
/* aas_clientmove_s */
#[no_mangle]
pub unsafe extern "C" fn trap_AAS_PredictClientMovement(mut move_0:
                                                            *mut libc::c_void,
                                                        mut entnum:
                                                            libc::c_int,
                                                        mut origin:
                                                            *mut vec_t,
                                                        mut presencetype:
                                                            libc::c_int,
                                                        mut onground:
                                                            libc::c_int,
                                                        mut velocity:
                                                            *mut vec_t,
                                                        mut cmdmove:
                                                            *mut vec_t,
                                                        mut cmdframes:
                                                            libc::c_int,
                                                        mut maxframes:
                                                            libc::c_int,
                                                        mut frametime:
                                                            libc::c_float,
                                                        mut stopevent:
                                                            libc::c_int,
                                                        mut stopareanum:
                                                            libc::c_int,
                                                        mut visualize:
                                                            libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AAS_PREDICT_CLIENT_MOVEMENT
                                                           as libc::c_int as
                                                           intptr_t, move_0,
                                                       entnum, origin,
                                                       presencetype, onground,
                                                       velocity, cmdmove,
                                                       cmdframes, maxframes,
                                                       PASSFLOAT(frametime),
                                                       stopevent, stopareanum,
                                                       visualize) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Say(mut client: libc::c_int,
                                     mut str: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_SAY as libc::c_int
                                                    as intptr_t, client, str);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_SayTeam(mut client: libc::c_int,
                                         mut str: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_SAY_TEAM as
                                                    libc::c_int as intptr_t,
                                                client, str);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Command(mut client: libc::c_int,
                                         mut command: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_COMMAND as
                                                    libc::c_int as intptr_t,
                                                client, command);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Action(mut client: libc::c_int,
                                        mut action: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_ACTION as
                                                    libc::c_int as intptr_t,
                                                client, action);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Gesture(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_GESTURE as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Talk(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_TALK as libc::c_int
                                                    as intptr_t, client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Attack(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_ATTACK as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Use(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_USE as libc::c_int
                                                    as intptr_t, client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Respawn(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_RESPAWN as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Crouch(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_CROUCH as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_MoveUp(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_MOVE_UP as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_MoveDown(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_MOVE_DOWN as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_MoveForward(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_MOVE_FORWARD as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_MoveBack(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_MOVE_BACK as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_MoveLeft(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_MOVE_LEFT as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_MoveRight(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_MOVE_RIGHT as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_SelectWeapon(mut client: libc::c_int,
                                              mut weapon: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_SELECT_WEAPON as
                                                    libc::c_int as intptr_t,
                                                client, weapon);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Jump(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_JUMP as libc::c_int
                                                    as intptr_t, client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_DelayedJump(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_DELAYED_JUMP as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_Move(mut client: libc::c_int,
                                      mut dir: *mut vec_t,
                                      mut speed: libc::c_float) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_MOVE as libc::c_int
                                                    as intptr_t, client, dir,
                                                PASSFLOAT(speed));
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_View(mut client: libc::c_int,
                                      mut viewangles: *mut vec_t) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_VIEW as libc::c_int
                                                    as intptr_t, client,
                                                viewangles);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_EndRegular(mut client: libc::c_int,
                                            mut thinktime: libc::c_float) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_END_REGULAR as
                                                    libc::c_int as intptr_t,
                                                client, PASSFLOAT(thinktime));
}
/* struct bot_input_s */
#[no_mangle]
pub unsafe extern "C" fn trap_EA_GetInput(mut client: libc::c_int,
                                          mut thinktime: libc::c_float,
                                          mut input: *mut libc::c_void) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_GET_INPUT as
                                                    libc::c_int as intptr_t,
                                                client, PASSFLOAT(thinktime),
                                                input);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EA_ResetInput(mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_EA_RESET_INPUT as
                                                    libc::c_int as intptr_t,
                                                client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLoadCharacter(mut charfile:
                                                   *mut libc::c_char,
                                               mut skill: libc::c_float)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_LOAD_CHARACTER
                                                           as libc::c_int as
                                                           intptr_t, charfile,
                                                       PASSFLOAT(skill)) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotFreeCharacter(mut character: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_FREE_CHARACTER as
                                                    libc::c_int as intptr_t,
                                                character);
}
#[no_mangle]
pub unsafe extern "C" fn trap_Characteristic_Float(mut character: libc::c_int,
                                                   mut index: libc::c_int)
 -> libc::c_float {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.i =
        syscall.expect("non-null function pointer")(BOTLIB_AI_CHARACTERISTIC_FLOAT
                                                        as libc::c_int as
                                                        intptr_t, character,
                                                    index) as libc::c_int;
    return fi.f;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Characteristic_BFloat(mut character:
                                                        libc::c_int,
                                                    mut index: libc::c_int,
                                                    mut min: libc::c_float,
                                                    mut max: libc::c_float)
 -> libc::c_float {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.i =
        syscall.expect("non-null function pointer")(BOTLIB_AI_CHARACTERISTIC_BFLOAT
                                                        as libc::c_int as
                                                        intptr_t, character,
                                                    index, PASSFLOAT(min),
                                                    PASSFLOAT(max)) as
            libc::c_int;
    return fi.f;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Characteristic_Integer(mut character:
                                                         libc::c_int,
                                                     mut index: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_CHARACTERISTIC_INTEGER
                                                           as libc::c_int as
                                                           intptr_t,
                                                       character, index) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Characteristic_BInteger(mut character:
                                                          libc::c_int,
                                                      mut index: libc::c_int,
                                                      mut min: libc::c_int,
                                                      mut max: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_CHARACTERISTIC_BINTEGER
                                                           as libc::c_int as
                                                           intptr_t,
                                                       character, index, min,
                                                       max) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_Characteristic_String(mut character:
                                                        libc::c_int,
                                                    mut index: libc::c_int,
                                                    mut buf:
                                                        *mut libc::c_char,
                                                    mut size: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_CHARACTERISTIC_STRING
                                                    as libc::c_int as
                                                    intptr_t, character,
                                                index, buf, size);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotAllocChatState() -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_ALLOC_CHAT_STATE
                                                           as libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotFreeChatState(mut handle: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_FREE_CHAT_STATE as
                                                    libc::c_int as intptr_t,
                                                handle);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotQueueConsoleMessage(mut chatstate:
                                                         libc::c_int,
                                                     mut type_0: libc::c_int,
                                                     mut message:
                                                         *mut libc::c_char) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_QUEUE_CONSOLE_MESSAGE
                                                    as libc::c_int as
                                                    intptr_t, chatstate,
                                                type_0, message);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotRemoveConsoleMessage(mut chatstate:
                                                          libc::c_int,
                                                      mut handle:
                                                          libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_REMOVE_CONSOLE_MESSAGE
                                                    as libc::c_int as
                                                    intptr_t, chatstate,
                                                handle);
}
/* struct bot_consolemessage_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotNextConsoleMessage(mut chatstate:
                                                        libc::c_int,
                                                    mut cm: *mut libc::c_void)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_NEXT_CONSOLE_MESSAGE
                                                           as libc::c_int as
                                                           intptr_t,
                                                       chatstate, cm) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotNumConsoleMessages(mut chatstate:
                                                        libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_NUM_CONSOLE_MESSAGE
                                                           as libc::c_int as
                                                           intptr_t,
                                                       chatstate) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotInitialChat(mut chatstate: libc::c_int,
                                             mut type_0: *mut libc::c_char,
                                             mut mcontext: libc::c_int,
                                             mut var0: *mut libc::c_char,
                                             mut var1: *mut libc::c_char,
                                             mut var2: *mut libc::c_char,
                                             mut var3: *mut libc::c_char,
                                             mut var4: *mut libc::c_char,
                                             mut var5: *mut libc::c_char,
                                             mut var6: *mut libc::c_char,
                                             mut var7: *mut libc::c_char) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_INITIAL_CHAT as
                                                    libc::c_int as intptr_t,
                                                chatstate, type_0, mcontext,
                                                var0, var1, var2, var3, var4,
                                                var5, var6, var7);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotNumInitialChats(mut chatstate: libc::c_int,
                                                 mut type_0:
                                                     *mut libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_NUM_INITIAL_CHATS
                                                           as libc::c_int as
                                                           intptr_t,
                                                       chatstate, type_0) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotReplyChat(mut chatstate: libc::c_int,
                                           mut message: *mut libc::c_char,
                                           mut mcontext: libc::c_int,
                                           mut vcontext: libc::c_int,
                                           mut var0: *mut libc::c_char,
                                           mut var1: *mut libc::c_char,
                                           mut var2: *mut libc::c_char,
                                           mut var3: *mut libc::c_char,
                                           mut var4: *mut libc::c_char,
                                           mut var5: *mut libc::c_char,
                                           mut var6: *mut libc::c_char,
                                           mut var7: *mut libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_REPLY_CHAT as
                                                           libc::c_int as
                                                           intptr_t,
                                                       chatstate, message,
                                                       mcontext, vcontext,
                                                       var0, var1, var2, var3,
                                                       var4, var5, var6, var7)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotChatLength(mut chatstate: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_CHAT_LENGTH
                                                           as libc::c_int as
                                                           intptr_t,
                                                       chatstate) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotEnterChat(mut chatstate: libc::c_int,
                                           mut client: libc::c_int,
                                           mut sendto: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_ENTER_CHAT as
                                                    libc::c_int as intptr_t,
                                                chatstate, client, sendto);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotGetChatMessage(mut chatstate: libc::c_int,
                                                mut buf: *mut libc::c_char,
                                                mut size: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_GET_CHAT_MESSAGE as
                                                    libc::c_int as intptr_t,
                                                chatstate, buf, size);
}
#[no_mangle]
pub unsafe extern "C" fn trap_StringContains(mut str1: *mut libc::c_char,
                                             mut str2: *mut libc::c_char,
                                             mut casesensitive: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_STRING_CONTAINS
                                                           as libc::c_int as
                                                           intptr_t, str1,
                                                       str2, casesensitive) as
               libc::c_int;
}
/* struct bot_match_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotFindMatch(mut str: *mut libc::c_char,
                                           mut match_0: *mut libc::c_void,
                                           mut context: libc::c_ulong)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_FIND_MATCH as
                                                           libc::c_int as
                                                           intptr_t, str,
                                                       match_0, context) as
               libc::c_int;
}
/* struct bot_match_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotMatchVariable(mut match_0: *mut libc::c_void,
                                               mut variable: libc::c_int,
                                               mut buf: *mut libc::c_char,
                                               mut size: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_MATCH_VARIABLE as
                                                    libc::c_int as intptr_t,
                                                match_0, variable, buf, size);
}
#[no_mangle]
pub unsafe extern "C" fn trap_UnifyWhiteSpaces(mut string:
                                                   *mut libc::c_char) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_UNIFY_WHITE_SPACES
                                                    as libc::c_int as
                                                    intptr_t, string);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotReplaceSynonyms(mut string:
                                                     *mut libc::c_char,
                                                 mut context: libc::c_ulong) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_REPLACE_SYNONYMS as
                                                    libc::c_int as intptr_t,
                                                string, context);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLoadChatFile(mut chatstate: libc::c_int,
                                              mut chatfile: *mut libc::c_char,
                                              mut chatname: *mut libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_LOAD_CHAT_FILE
                                                           as libc::c_int as
                                                           intptr_t,
                                                       chatstate, chatfile,
                                                       chatname) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotSetChatGender(mut chatstate: libc::c_int,
                                               mut gender: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_SET_CHAT_GENDER as
                                                    libc::c_int as intptr_t,
                                                chatstate, gender);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotSetChatName(mut chatstate: libc::c_int,
                                             mut name: *mut libc::c_char,
                                             mut client: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_SET_CHAT_NAME as
                                                    libc::c_int as intptr_t,
                                                chatstate, name, client);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotResetGoalState(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_RESET_GOAL_STATE as
                                                    libc::c_int as intptr_t,
                                                goalstate);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotRemoveFromAvoidGoals(mut goalstate:
                                                          libc::c_int,
                                                      mut number:
                                                          libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_REMOVE_FROM_AVOID_GOALS
                                                    as libc::c_int as
                                                    intptr_t, goalstate,
                                                number);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotResetAvoidGoals(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_RESET_AVOID_GOALS as
                                                    libc::c_int as intptr_t,
                                                goalstate);
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotPushGoal(mut goalstate: libc::c_int,
                                          mut goal: *mut libc::c_void) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_PUSH_GOAL as
                                                    libc::c_int as intptr_t,
                                                goalstate, goal);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotPopGoal(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_POP_GOAL as
                                                    libc::c_int as intptr_t,
                                                goalstate);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotEmptyGoalStack(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_EMPTY_GOAL_STACK as
                                                    libc::c_int as intptr_t,
                                                goalstate);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotDumpAvoidGoals(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_DUMP_AVOID_GOALS as
                                                    libc::c_int as intptr_t,
                                                goalstate);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotDumpGoalStack(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_DUMP_GOAL_STACK as
                                                    libc::c_int as intptr_t,
                                                goalstate);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotGoalName(mut number: libc::c_int,
                                          mut name: *mut libc::c_char,
                                          mut size: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_GOAL_NAME as
                                                    libc::c_int as intptr_t,
                                                number, name, size);
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotGetTopGoal(mut goalstate: libc::c_int,
                                            mut goal: *mut libc::c_void)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_GET_TOP_GOAL
                                                           as libc::c_int as
                                                           intptr_t,
                                                       goalstate, goal) as
               libc::c_int;
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotGetSecondGoal(mut goalstate: libc::c_int,
                                               mut goal: *mut libc::c_void)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_GET_SECOND_GOAL
                                                           as libc::c_int as
                                                           intptr_t,
                                                       goalstate, goal) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotChooseLTGItem(mut goalstate: libc::c_int,
                                               mut origin: *mut vec_t,
                                               mut inventory:
                                                   *mut libc::c_int,
                                               mut travelflags: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_CHOOSE_LTG_ITEM
                                                           as libc::c_int as
                                                           intptr_t,
                                                       goalstate, origin,
                                                       inventory, travelflags)
               as libc::c_int;
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotChooseNBGItem(mut goalstate: libc::c_int,
                                               mut origin: *mut vec_t,
                                               mut inventory:
                                                   *mut libc::c_int,
                                               mut travelflags: libc::c_int,
                                               mut ltg: *mut libc::c_void,
                                               mut maxtime: libc::c_float)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_CHOOSE_NBG_ITEM
                                                           as libc::c_int as
                                                           intptr_t,
                                                       goalstate, origin,
                                                       inventory, travelflags,
                                                       ltg,
                                                       PASSFLOAT(maxtime)) as
               libc::c_int;
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotTouchingGoal(mut origin: *mut vec_t,
                                              mut goal: *mut libc::c_void)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_TOUCHING_GOAL
                                                           as libc::c_int as
                                                           intptr_t, origin,
                                                       goal) as libc::c_int;
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotItemGoalInVisButNotVisible(mut viewer:
                                                                libc::c_int,
                                                            mut eye:
                                                                *mut vec_t,
                                                            mut viewangles:
                                                                *mut vec_t,
                                                            mut goal:
                                                                *mut libc::c_void)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_ITEM_GOAL_IN_VIS_BUT_NOT_VISIBLE
                                                           as libc::c_int as
                                                           intptr_t, viewer,
                                                       eye, viewangles, goal)
               as libc::c_int;
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotGetNextCampSpotGoal(mut num: libc::c_int,
                                                     mut goal:
                                                         *mut libc::c_void)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_GET_NEXT_CAMP_SPOT_GOAL
                                                           as libc::c_int as
                                                           intptr_t, num,
                                                       goal) as libc::c_int;
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotGetMapLocationGoal(mut name:
                                                        *mut libc::c_char,
                                                    mut goal:
                                                        *mut libc::c_void)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_GET_MAP_LOCATION_GOAL
                                                           as libc::c_int as
                                                           intptr_t, name,
                                                       goal) as libc::c_int;
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotGetLevelItemGoal(mut index: libc::c_int,
                                                  mut classname:
                                                      *mut libc::c_char,
                                                  mut goal: *mut libc::c_void)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_GET_LEVEL_ITEM_GOAL
                                                           as libc::c_int as
                                                           intptr_t, index,
                                                       classname, goal) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotAvoidGoalTime(mut goalstate: libc::c_int,
                                               mut number: libc::c_int)
 -> libc::c_float {
    let mut fi: floatint_t = floatint_t{f: 0.,};
    fi.i =
        syscall.expect("non-null function pointer")(BOTLIB_AI_AVOID_GOAL_TIME
                                                        as libc::c_int as
                                                        intptr_t, goalstate,
                                                    number) as libc::c_int;
    return fi.f;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotSetAvoidGoalTime(mut goalstate: libc::c_int,
                                                  mut number: libc::c_int,
                                                  mut avoidtime:
                                                      libc::c_float) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_SET_AVOID_GOAL_TIME
                                                    as libc::c_int as
                                                    intptr_t, goalstate,
                                                number, PASSFLOAT(avoidtime));
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotInitLevelItems() {
    syscall.expect("non-null function pointer")(BOTLIB_AI_INIT_LEVEL_ITEMS as
                                                    libc::c_int as intptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotUpdateEntityItems() {
    syscall.expect("non-null function pointer")(BOTLIB_AI_UPDATE_ENTITY_ITEMS
                                                    as libc::c_int as
                                                    intptr_t);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLoadItemWeights(mut goalstate: libc::c_int,
                                                 mut filename:
                                                     *mut libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_LOAD_ITEM_WEIGHTS
                                                           as libc::c_int as
                                                           intptr_t,
                                                       goalstate, filename) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotFreeItemWeights(mut goalstate: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_FREE_ITEM_WEIGHTS as
                                                    libc::c_int as intptr_t,
                                                goalstate);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotInterbreedGoalFuzzyLogic(mut parent1:
                                                              libc::c_int,
                                                          mut parent2:
                                                              libc::c_int,
                                                          mut child:
                                                              libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_INTERBREED_GOAL_FUZZY_LOGIC
                                                    as libc::c_int as
                                                    intptr_t, parent1,
                                                parent2, child);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotSaveGoalFuzzyLogic(mut goalstate:
                                                        libc::c_int,
                                                    mut filename:
                                                        *mut libc::c_char) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_SAVE_GOAL_FUZZY_LOGIC
                                                    as libc::c_int as
                                                    intptr_t, goalstate,
                                                filename);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotMutateGoalFuzzyLogic(mut goalstate:
                                                          libc::c_int,
                                                      mut range:
                                                          libc::c_float) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_MUTATE_GOAL_FUZZY_LOGIC
                                                    as libc::c_int as
                                                    intptr_t, goalstate,
                                                PASSFLOAT(range));
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotAllocGoalState(mut state: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_ALLOC_GOAL_STATE
                                                           as libc::c_int as
                                                           intptr_t, state) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotFreeGoalState(mut handle: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_FREE_GOAL_STATE as
                                                    libc::c_int as intptr_t,
                                                handle);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotResetMoveState(mut movestate: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_RESET_MOVE_STATE as
                                                    libc::c_int as intptr_t,
                                                movestate);
}
/* struct bot_moveresult_s */
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotMoveToGoal(mut result: *mut libc::c_void,
                                            mut movestate: libc::c_int,
                                            mut goal: *mut libc::c_void,
                                            mut travelflags: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_MOVE_TO_GOAL as
                                                    libc::c_int as intptr_t,
                                                result, movestate, goal,
                                                travelflags);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotMoveInDirection(mut movestate: libc::c_int,
                                                 mut dir: *mut vec_t,
                                                 mut speed: libc::c_float,
                                                 mut type_0: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_MOVE_IN_DIRECTION
                                                           as libc::c_int as
                                                           intptr_t,
                                                       movestate, dir,
                                                       PASSFLOAT(speed),
                                                       type_0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotResetAvoidReach(mut movestate: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_RESET_AVOID_REACH as
                                                    libc::c_int as intptr_t,
                                                movestate);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotResetLastAvoidReach(mut movestate:
                                                         libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_RESET_LAST_AVOID_REACH
                                                    as libc::c_int as
                                                    intptr_t, movestate);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotReachabilityArea(mut origin: *mut vec_t,
                                                  mut testground: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_REACHABILITY_AREA
                                                           as libc::c_int as
                                                           intptr_t, origin,
                                                       testground) as
               libc::c_int;
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotMovementViewTarget(mut movestate:
                                                        libc::c_int,
                                                    mut goal:
                                                        *mut libc::c_void,
                                                    mut travelflags:
                                                        libc::c_int,
                                                    mut lookahead:
                                                        libc::c_float,
                                                    mut target: *mut vec_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_MOVEMENT_VIEW_TARGET
                                                           as libc::c_int as
                                                           intptr_t,
                                                       movestate, goal,
                                                       travelflags,
                                                       PASSFLOAT(lookahead),
                                                       target) as libc::c_int;
}
/* struct bot_goal_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotPredictVisiblePosition(mut origin:
                                                            *mut vec_t,
                                                        mut areanum:
                                                            libc::c_int,
                                                        mut goal:
                                                            *mut libc::c_void,
                                                        mut travelflags:
                                                            libc::c_int,
                                                        mut target:
                                                            *mut vec_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_PREDICT_VISIBLE_POSITION
                                                           as libc::c_int as
                                                           intptr_t, origin,
                                                       areanum, goal,
                                                       travelflags, target) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotAllocMoveState() -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_ALLOC_MOVE_STATE
                                                           as libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotFreeMoveState(mut handle: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_FREE_MOVE_STATE as
                                                    libc::c_int as intptr_t,
                                                handle);
}
/* struct bot_initmove_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotInitMoveState(mut handle: libc::c_int,
                                               mut initmove:
                                                   *mut libc::c_void) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_INIT_MOVE_STATE as
                                                    libc::c_int as intptr_t,
                                                handle, initmove);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotAddAvoidSpot(mut movestate: libc::c_int,
                                              mut origin: *mut vec_t,
                                              mut radius: libc::c_float,
                                              mut type_0: libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_ADD_AVOID_SPOT as
                                                    libc::c_int as intptr_t,
                                                movestate, origin,
                                                PASSFLOAT(radius), type_0);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotChooseBestFightWeapon(mut weaponstate:
                                                           libc::c_int,
                                                       mut inventory:
                                                           *mut libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_CHOOSE_BEST_FIGHT_WEAPON
                                                           as libc::c_int as
                                                           intptr_t,
                                                       weaponstate, inventory)
               as libc::c_int;
}
/* struct weaponinfo_s */
#[no_mangle]
pub unsafe extern "C" fn trap_BotGetWeaponInfo(mut weaponstate: libc::c_int,
                                               mut weapon: libc::c_int,
                                               mut weaponinfo:
                                                   *mut libc::c_void) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_GET_WEAPON_INFO as
                                                    libc::c_int as intptr_t,
                                                weaponstate, weapon,
                                                weaponinfo);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotLoadWeaponWeights(mut weaponstate:
                                                       libc::c_int,
                                                   mut filename:
                                                       *mut libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_LOAD_WEAPON_WEIGHTS
                                                           as libc::c_int as
                                                           intptr_t,
                                                       weaponstate, filename)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotAllocWeaponState() -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_ALLOC_WEAPON_STATE
                                                           as libc::c_int as
                                                           intptr_t) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotFreeWeaponState(mut weaponstate:
                                                     libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_FREE_WEAPON_STATE as
                                                    libc::c_int as intptr_t,
                                                weaponstate);
}
#[no_mangle]
pub unsafe extern "C" fn trap_BotResetWeaponState(mut weaponstate:
                                                      libc::c_int) {
    syscall.expect("non-null function pointer")(BOTLIB_AI_RESET_WEAPON_STATE
                                                    as libc::c_int as
                                                    intptr_t, weaponstate);
}
#[no_mangle]
pub unsafe extern "C" fn trap_GeneticParentsAndChildSelection(mut numranks:
                                                                  libc::c_int,
                                                              mut ranks:
                                                                  *mut libc::c_float,
                                                              mut parent1:
                                                                  *mut libc::c_int,
                                                              mut parent2:
                                                                  *mut libc::c_int,
                                                              mut child:
                                                                  *mut libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_AI_GENETIC_PARENTS_AND_CHILD_SELECTION
                                                           as libc::c_int as
                                                           intptr_t, numranks,
                                                       ranks, parent1,
                                                       parent2, child) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_SnapVector(mut v: *mut libc::c_float) {
    syscall.expect("non-null function pointer")(G_SNAPVECTOR as libc::c_int as
                                                    intptr_t, v);
}
#[no_mangle]
pub unsafe extern "C" fn dllEntry(mut syscallptr:
                                      Option<unsafe extern "C" fn(_:
                                                                      intptr_t, ...)
                                                 -> intptr_t>) {
    syscall = syscallptr;
}
#[no_mangle]
pub unsafe extern "C" fn trap_TraceCapsule(mut results: *mut trace_t,
                                           mut start: *const vec_t,
                                           mut mins: *const vec_t,
                                           mut maxs: *const vec_t,
                                           mut end: *const vec_t,
                                           mut passEntityNum: libc::c_int,
                                           mut contentmask: libc::c_int) {
    syscall.expect("non-null function pointer")(G_TRACECAPSULE as libc::c_int
                                                    as intptr_t, results,
                                                start, mins, maxs, end,
                                                passEntityNum, contentmask);
}
#[no_mangle]
pub unsafe extern "C" fn trap_EntityContactCapsule(mut mins: *const vec_t,
                                                   mut maxs: *const vec_t,
                                                   mut ent: *const gentity_t)
 -> qboolean {
    return syscall.expect("non-null function pointer")(G_ENTITY_CONTACTCAPSULE
                                                           as libc::c_int as
                                                           intptr_t, mins,
                                                       maxs, ent) as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_LoadSource(mut filename: *const libc::c_char)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_PC_LOAD_SOURCE
                                                           as libc::c_int as
                                                           intptr_t, filename)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_FreeSource(mut handle: libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_PC_FREE_SOURCE
                                                           as libc::c_int as
                                                           intptr_t, handle)
               as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_ReadToken(mut handle: libc::c_int,
                                           mut pc_token: *mut pc_token_t)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_PC_READ_TOKEN as
                                                           libc::c_int as
                                                           intptr_t, handle,
                                                       pc_token) as
               libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trap_PC_SourceFileAndLine(mut handle: libc::c_int,
                                                   mut filename:
                                                       *mut libc::c_char,
                                                   mut line: *mut libc::c_int)
 -> libc::c_int {
    return syscall.expect("non-null function pointer")(BOTLIB_PC_SOURCE_FILE_AND_LINE
                                                           as libc::c_int as
                                                           intptr_t, handle,
                                                       filename, line) as
               libc::c_int;
}