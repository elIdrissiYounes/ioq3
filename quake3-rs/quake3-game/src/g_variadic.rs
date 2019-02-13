use libc;
#[header_src = "vararg"]
pub mod vararg {
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: libc::c_uint,
        pub fp_offset: libc::c_uint,
        pub overflow_arg_area: *mut libc::c_void,
        pub reg_save_area: *mut libc::c_void,
    }
    use super::{libc};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stdarg.h"]
pub mod stdarg_h {
    pub type va_list = __builtin_va_list;
    use super::vararg::{__builtin_va_list};
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
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
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
    }
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
    //
// this structure is cleared as each map is entered
//
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
    use super::{libc};
    use super::q_shared_h::{entityState_t, qboolean, vec3_t, trace_t,
                            playerState_t, usercmd_t, fileHandle_t, vmCvar_t};
    use super::g_public_h::{entityShared_t};
    use super::bg_public_h::{gitem_t, team_t};
    extern "C" {
        #[no_mangle]
        pub fn trap_Error(text: *const libc::c_char) -> !;
        #[no_mangle]
        pub fn trap_Print(text: *const libc::c_char);
        #[no_mangle]
        pub static mut level: level_locals_t;
        #[no_mangle]
        pub static mut g_entities: [gentity_t; 1024];
        #[no_mangle]
        pub static mut g_dedicated: vmCvar_t;
        #[no_mangle]
        pub fn trap_FS_Write(buffer: *const libc::c_void, len: libc::c_int,
                             f: fileHandle_t);
        #[no_mangle]
        pub fn trap_SendServerCommand(clientNum: libc::c_int,
                                      text: *const libc::c_char);
    }
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    use super::{libc};
    use super::vararg::{__va_list_tag};
    extern "C" {
        #[no_mangle]
        pub fn vsnprintf(_: *mut libc::c_char, _: libc::c_ulong,
                         _: *const libc::c_char, _: *mut __va_list_tag)
         -> libc::c_int;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/g_variadic.h"]
pub mod g_variadic_h {
    use super::{libc};
    use super::g_local_h::{gentity_t};
}
use self::vararg::{__builtin_va_list, __va_list_tag};
use self::stdarg_h::{va_list};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, cvarHandle_t, vmCvar_t, cplane_t,
                       trace_t, playerState_s, playerState_t, usercmd_s,
                       usercmd_t, trType_t, TR_GRAVITY, TR_SINE,
                       TR_LINEAR_STOP, TR_LINEAR, TR_INTERPOLATE,
                       TR_STATIONARY, trajectory_t, entityState_s,
                       entityState_t, Com_sprintf, va};
use self::bg_public_h::{team_t, TEAM_NUM_TEAMS, TEAM_SPECTATOR, TEAM_BLUE,
                        TEAM_RED, TEAM_FREE, itemType_t, IT_TEAM,
                        IT_PERSISTANT_POWERUP, IT_HOLDABLE, IT_POWERUP,
                        IT_HEALTH, IT_ARMOR, IT_AMMO, IT_WEAPON, IT_BAD,
                        gitem_s, gitem_t};
use self::g_public_h::{entityShared_t};
use self::g_local_h::{moverState_t, MOVER_2TO1, MOVER_1TO2, MOVER_POS2,
                      MOVER_POS1, gentity_s, gentity_t, gclient_s,
                      clientSession_t, spectatorState_t, SPECTATOR_SCOREBOARD,
                      SPECTATOR_FOLLOW, SPECTATOR_FREE, SPECTATOR_NOT,
                      clientPersistant_t, playerTeamState_t,
                      playerTeamStateState_t, TEAM_ACTIVE, TEAM_BEGIN,
                      clientConnected_t, CON_CONNECTED, CON_CONNECTING,
                      CON_DISCONNECTED, level_locals_t, trap_Error,
                      trap_Print, level, g_entities, g_dedicated,
                      trap_FS_Write, trap_SendServerCommand};
use self::stdio_h::{vsnprintf};
use self::string_h::{strchr, strlen};
// this is only here so the functions in q_shared.c and bg_*.c can link
#[no_mangle]
pub unsafe extern "C" fn Com_Error(mut level_0: libc::c_int,
                                   mut error: *const libc::c_char, ...) -> ! {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(text.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
              error, argptr);
    trap_Error(text.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Com_Printf(mut msg: *const libc::c_char, ...) {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(text.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
              msg, argptr_0);
    trap_Print(text.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn G_LogPrintf(mut fmt: *const libc::c_char, ...) {
    let mut string: [libc::c_char; 1024] = [0; 1024];
    let mut min: libc::c_int = 0;
    let mut tens: libc::c_int = 0;
    let mut sec: libc::c_int = 0;
    sec = (level.time - level.startTime) / 1000i32;
    min = sec / 60i32;
    sec -= min * 60i32;
    tens = sec / 10i32;
    sec -= tens * 10i32;
    Com_sprintf(string.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"%3i:%i%i \x00" as *const u8 as *const libc::c_char, min,
                tens, sec);
    vsnprintf(string.as_mut_ptr().offset(7isize),
              (::std::mem::size_of::<[libc::c_char; 1024]>() as
                   libc::c_ulong).wrapping_sub(7i32 as libc::c_ulong), fmt,
              argptr_1);
    if 0 != g_dedicated.integer {
        G_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                 string.as_mut_ptr().offset(7isize));
    }
    if 0 == level.logFile { return }
    trap_FS_Write(string.as_mut_ptr() as *const libc::c_void,
                  strlen(string.as_mut_ptr()) as libc::c_int, level.logFile);
}
#[no_mangle]
pub unsafe extern "C" fn G_Printf(mut fmt: *const libc::c_char, ...) {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(text.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
              fmt, argptr_2);
    trap_Print(text.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn G_Error(mut fmt: *const libc::c_char, ...) -> ! {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    vsnprintf(text.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
              fmt, argptr_3);
    trap_Error(text.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn PrintMsg(mut ent: *mut gentity_t,
                                  mut fmt: *const libc::c_char, ...) {
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    if vsnprintf(msg.as_mut_ptr(),
                 ::std::mem::size_of::<[libc::c_char; 1024]>() as
                     libc::c_ulong, fmt, argptr_4) as libc::c_ulong >=
           ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong {
        G_Error(b"PrintMsg overrun\x00" as *const u8 as *const libc::c_char);
    }
    loop  {
        p = strchr(msg.as_mut_ptr(), '\"' as i32);
        if p.is_null() { break ; }
        *p = '\'' as i32 as libc::c_char
    }
    trap_SendServerCommand((if ent.is_null() {
                                -1i32 as libc::c_long
                            } else {
                                ent.wrapping_offset_from(g_entities.as_mut_ptr())
                                    as libc::c_long
                            }) as libc::c_int,
                           va(b"print \"%s\"\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              msg.as_mut_ptr()));
}