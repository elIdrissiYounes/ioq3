#[repr(C)]
#[derive(Copy, Clone)]
pub union floatint_t {
    pub f: f32,
    pub i: i32,
    pub ui: u32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pc_token_s {
    pub type_0: i32,
    pub subtype: i32,
    pub intvalue: i32,
    pub floatvalue: f32,
    pub string: [i8; 1024],
}
pub type pc_token_t = crate::src::qcommon::q_shared::pc_token_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtime_s {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
}
pub type qtime_t = crate::src::qcommon::q_shared::qtime_s;
pub const CHAN_ANNOUNCER: crate::bg_public_h::C2RustUnnamed_0 = 7;
pub const CHAN_LOCAL_SOUND: crate::bg_public_h::C2RustUnnamed_0 = 6;
pub const CHAN_BODY: crate::bg_public_h::C2RustUnnamed_0 = 5;
pub const CHAN_ITEM: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const CHAN_VOICE: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const CHAN_WEAPON: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const CHAN_LOCAL: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const CHAN_AUTO: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub type _flag_status = u32;
pub type flagStatus_t = crate::src::qcommon::q_shared::_flag_status;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qint64 {
    pub b0: crate::src::qcommon::q_shared::byte,
    pub b1: crate::src::qcommon::q_shared::byte,
    pub b2: crate::src::qcommon::q_shared::byte,
    pub b3: crate::src::qcommon::q_shared::byte,
    pub b4: crate::src::qcommon::q_shared::byte,
    pub b5: crate::src::qcommon::q_shared::byte,
    pub b6: crate::src::qcommon::q_shared::byte,
    pub b7: crate::src::qcommon::q_shared::byte,
}
pub const FLAG_DROPPED: crate::src::qcommon::q_shared::_flag_status = 4;
pub const FLAG_TAKEN_BLUE: crate::src::qcommon::q_shared::_flag_status = 3;
pub const FLAG_TAKEN_RED: crate::src::qcommon::q_shared::_flag_status = 2;
pub const FLAG_TAKEN: crate::src::qcommon::q_shared::_flag_status = 1;
pub const FLAG_ATBASE: crate::src::qcommon::q_shared::_flag_status = 0;
pub type fsMode_t = u32;
pub const ERR_NEED_CD: crate::bg_public_h::C2RustUnnamed_0 = 4;
pub const ERR_DISCONNECT: crate::bg_public_h::C2RustUnnamed_0 = 3;
pub const ERR_SERVERDISCONNECT: crate::bg_public_h::C2RustUnnamed_0 = 2;
pub const ERR_DROP: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const ERR_FATAL: crate::bg_public_h::C2RustUnnamed_0 = 0;
pub const FS_APPEND_SYNC: crate::src::qcommon::q_shared::fsMode_t = 3;
pub const FS_APPEND: crate::src::qcommon::q_shared::fsMode_t = 2;
pub const FS_WRITE: crate::src::qcommon::q_shared::fsMode_t = 1;
pub const FS_READ: crate::src::qcommon::q_shared::fsMode_t = 0;
pub type vec4_t = [crate::src::qcommon::q_shared::vec_t; 4];
pub type byte = u8;
pub type qboolean = u32;
pub type fileHandle_t = i32;
pub type vec_t = f32;
pub type vec3_t = [crate::src::qcommon::q_shared::vec_t; 3];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cplane_s {
    pub normal: crate::src::qcommon::q_shared::vec3_t,
    pub dist: f32,
    pub type_0: crate::src::qcommon::q_shared::byte,
    pub signbits: crate::src::qcommon::q_shared::byte,
    pub pad: [crate::src::qcommon::q_shared::byte; 2],
}
pub type cvarHandle_t = i32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmCvar_t {
    pub handle: crate::src::qcommon::q_shared::cvarHandle_t,
    pub modificationCount: i32,
    pub value: f32,
    pub integer: i32,
    pub string: [i8; 256],
}
pub type cplane_t = crate::src::qcommon::q_shared::cplane_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_t {
    pub allsolid: crate::src::qcommon::q_shared::qboolean,
    pub startsolid: crate::src::qcommon::q_shared::qboolean,
    pub fraction: f32,
    pub endpos: crate::src::qcommon::q_shared::vec3_t,
    pub plane: crate::src::qcommon::q_shared::cplane_t,
    pub surfaceFlags: i32,
    pub contents: i32,
    pub entityNum: i32,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct playerState_s {
    pub commandTime: i32,
    pub pm_type: i32,
    pub bobCycle: i32,
    pub pm_flags: i32,
    pub pm_time: i32,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub velocity: crate::src::qcommon::q_shared::vec3_t,
    pub weaponTime: i32,
    pub gravity: i32,
    pub speed: i32,
    pub delta_angles: [i32; 3],
    pub groundEntityNum: i32,
    pub legsTimer: i32,
    pub legsAnim: i32,
    pub torsoTimer: i32,
    pub torsoAnim: i32,
    pub movementDir: i32,
    pub grapplePoint: crate::src::qcommon::q_shared::vec3_t,
    pub eFlags: i32,
    pub eventSequence: i32,
    pub events: [i32; 2],
    pub eventParms: [i32; 2],
    pub externalEvent: i32,
    pub externalEventParm: i32,
    pub externalEventTime: i32,
    pub clientNum: i32,
    pub weapon: i32,
    pub weaponstate: i32,
    pub viewangles: crate::src::qcommon::q_shared::vec3_t,
    pub viewheight: i32,
    pub damageEvent: i32,
    pub damageYaw: i32,
    pub damagePitch: i32,
    pub damageCount: i32,
    pub stats: [i32; 16],
    pub persistant: [i32; 16],
    pub powerups: [i32; 16],
    pub ammo: [i32; 16],
    pub generic1: i32,
    pub loopSound: i32,
    pub jumppad_ent: i32,
    pub ping: i32,
    pub pmove_framecount: i32,
    pub jumppad_frame: i32,
    pub entityEventSequence: i32,
}
pub type playerState_t = crate::src::qcommon::q_shared::playerState_s;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usercmd_s {
    pub serverTime: i32,
    pub angles: [i32; 3],
    pub buttons: i32,
    pub weapon: crate::src::qcommon::q_shared::byte,
    pub forwardmove: i8,
    pub rightmove: i8,
    pub upmove: i8,
}
pub type usercmd_t = crate::src::qcommon::q_shared::usercmd_s;
pub type trType_t = u32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trajectory_t {
    pub trType: crate::src::qcommon::q_shared::trType_t,
    pub trTime: i32,
    pub trDuration: i32,
    pub trBase: crate::src::qcommon::q_shared::vec3_t,
    pub trDelta: crate::src::qcommon::q_shared::vec3_t,
}
// these are sent over the net as 8 bits

// so they cannot be blindly increased

// these are the only configstrings that the system reserves, all the

// other ones are strictly for servergame to clientgame communication

// an info string with all the serverinfo cvars

// an info string for server system to client system configuration (timescale, etc)

// game can't modify below this, only the system can

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

// cmd->serverTime of last executed command

// for view bobbing and footstep generation

// ducked, jump_held, etc

// add to command angles to get view direction

// changed by spawns, rotating objects, and teleporters

// ENTITYNUM_NONE = in air

// don't change low priority animations until this runs out

// mask off ANIM_TOGGLEBIT

// don't change low priority animations until this runs out

// mask off ANIM_TOGGLEBIT

// a number 0 to 7 that represents the relative angle

// of movement to the view angle (axial and diagonals)

// when at rest, the value will remain unchanged

// used to twist the legs during strafing

// location of grapple to pull towards if PMF_GRAPPLE_PULL

// copied to entityState_t->eFlags

// pmove generated events

// events set on player from another source

// ranges from 0 to MAX_CLIENTS-1

// copied to entityState_t->weapon

// for fixed views

// damage feedback

// when it changes, latch the other parms

// stats that aren't cleared on death

// level.time that the powerup runs out

// jumppad entity hit this frame

// not communicated over the net at all

// server to game info for scoreboard

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

// weapon

//===================================================================

// if entityState->solid == SOLID_BMODEL, modelindex is an inline model number

// non-parametric, but interpolate between snapshots

// value = base + sin( time / duration ) * delta

// if non 0, trTime + trDuration = stop time

// velocity, etc

// entityState_t is the information conveyed from the server

// in an update message about entities that the client will

// need to render in some way

// Different eTypes may use the information in different ways

// The messages are delta compressed, so it doesn't really matter if

// the structure size is fairly large
#[repr(C)]
#[derive(Copy, Clone)]
pub struct entityState_s {
    pub number: i32,
    pub eType: i32,
    pub eFlags: i32,
    pub pos: crate::src::qcommon::q_shared::trajectory_t,
    pub apos: crate::src::qcommon::q_shared::trajectory_t,
    pub time: i32,
    pub time2: i32,
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub origin2: crate::src::qcommon::q_shared::vec3_t,
    pub angles: crate::src::qcommon::q_shared::vec3_t,
    pub angles2: crate::src::qcommon::q_shared::vec3_t,
    pub otherEntityNum: i32,
    pub otherEntityNum2: i32,
    pub groundEntityNum: i32,
    pub constantLight: i32,
    pub loopSound: i32,
    pub modelindex: i32,
    pub modelindex2: i32,
    pub clientNum: i32,
    pub frame: i32,
    pub solid: i32,
    pub event: i32,
    pub eventParm: i32,
    pub powerups: i32,
    pub weapon: i32,
    pub legsAnim: i32,
    pub torsoAnim: i32,
    pub generic1: i32,
}
pub type entityState_t = crate::src::qcommon::q_shared::entityState_s;
pub const qtrue: crate::src::qcommon::q_shared::qboolean = 1;
pub const qfalse: crate::src::qcommon::q_shared::qboolean = 0;
pub const TR_GRAVITY: crate::src::qcommon::q_shared::trType_t = 5;
pub const TR_SINE: crate::src::qcommon::q_shared::trType_t = 4;
pub const TR_LINEAR_STOP: crate::src::qcommon::q_shared::trType_t = 3;
pub const TR_LINEAR: crate::src::qcommon::q_shared::trType_t = 2;
pub const TR_INTERPOLATE: crate::src::qcommon::q_shared::trType_t = 1;
pub const TR_STATIONARY: crate::src::qcommon::q_shared::trType_t = 0;
// add to end of the command buffer (normal case)

// insert at current position, but don't run yet
pub const EXEC_APPEND: crate::bg_public_h::C2RustUnnamed_0 = 2;
// don't return until completed, a VM should NEVER use this,

// because some commands might cause the VM to be unloaded...
pub const EXEC_INSERT: crate::bg_public_h::C2RustUnnamed_0 = 1;
pub const EXEC_NOW: crate::bg_public_h::C2RustUnnamed_0 = 0;
// check think function

// if it is in a nodrop volume, remove it

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

//===================================================================

// if entityState->solid == SOLID_BMODEL, modelindex is an inline model number

// value = base + sin( time / duration ) * delta

// non-parametric, but interpolate between snapshots

// entityState_t is the information conveyed from the server

// in an update message about entities that the client will

// need to render in some way

// Different eTypes may use the information in different ways

// The messages are delta compressed, so it doesn't really matter if

// the structure size is fairly large

// __Q_SHARED_H

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

// bg_public.h -- definitions shared by both the server game and client game modules

// because games can change separately from the main system version, we need a

// second version that must match between game and cgame

// item sizes are needed for client side pickup detection

// for the CS_SCORES[12] when only one player is present

// 30 seconds before vote times out

//

// config strings are a general means of communicating variable length strings

// from the server to all connected clients.

//

// CS_SERVERINFO and CS_SYSTEMINFO are defined in q_shared.h

// from the map worldspawn's message field

// g_motd string for server message of the day

// server time when the match will be restarted

// so the timer only shows the current level

// when 1, fraglimit/timelimit has been hit and intermission will start in a second or two

// string indicating flag status in CTF

// string of 0's and 1's that tell which items are present

// capture the flag

// team deathmatch

// single player ffa

//-- team games go after this --

// one on one tournament

// free for all

// flip the togglebit every time an animation

// changes so a restart of the same anim can be detected

//---------------------------------------------------------

// gitem_t->type

// single use, holdable item

// EFX: rotate + bob

// instant on, timer based

// EFX: rotate + external ring that rotates

// EFX: static external sphere + rotating internal

// EFX: rotate + minlight

// EFX: rotate

// EFX: rotate + upscale + minlight

// string of all sounds this item will use

// g_dmflags->integer flags

// content masks

//

// entityState_t->eType

//

// any of the EV_* events can be added freestanding

// by setting eType to ET_EVENTS + eventNum

// this avoids having to set eFlags and eventNum

// grapple hooked on wall

// maximum radius of the models during the effect

// radius of the models without scaling

// 2nd shockwave times

// explosion/implosion times

// 1st shockwave times

// Kamikaze

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

//

// functions exported by the game subsystem

//

// ( int time );

// ( void );

// ConsoleCommand will be called when a command has been issued

// that is not recognized as a builtin function.

// The game can issue trap_argc() / trap_argv() commands to get the command

// and parameters.  Return qfalse if the game doesn't recognize it as a command.

// ( int levelTime );

// ( int clientNum );

// ( int clientNum );

// ( int clientNum );

// ( int clientNum );

// ( int clientNum );

// ( int clientNum, qboolean firstTime, qboolean isBot );

// return NULL if the client is allowed to connect, otherwise return

// a text string with the reason for denial

// (void);

// ( int levelTime, int randomSeed, int restart );

// init and shutdown will be called every single level

// The game should call G_GET_ENTITY_TOKEN to parse through all the

// entity configuration text and spawn gentities.

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

// Initialized in run_static_initializers

/*
================
vmMain

This is the only way control passes into the module.
This must be the very first function compiled into the .q3vm file
================
*/

/*
================
G_FindTeams

Chain together all entities with a matching team field.
Entity teams are used for item groups and multi-entity mover groups.

All but the first will have the FL_TEAMSLAVE flag set and teammaster field set
All but the last will have the teamchain field set to the next one
================
*/

// make sure that targets only point at the master

/*
=================
G_RegisterCvars
=================
*/

// check some things

/*
=================
G_UpdateCvars
=================
*/

/*
============
G_InitGame

============
*/

// set some level globals
use ::libc;
pub mod ctype_h {

    #[inline]

    pub unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
        return if __c >= -(128) && __c < 256 {
            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
        } else {
            __c
        };
    }

    #[inline]

    pub unsafe extern "C" fn toupper(mut __c: i32) -> i32 {
        return if __c >= -(128) && __c < 256 {
            *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
        } else {
            __c
        };
    }
}
pub mod stdlib_float_h {

    #[inline]

    pub unsafe extern "C" fn atof(mut __nptr: *const i8) -> f64 {
        return crate::stdlib::strtod(__nptr, 0 as *mut *mut i8);
    }

    use crate::stdlib::strtod;
}
pub use crate::bg_public_h::C2RustUnnamed_0;
pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::src::game::g_main::Com_Error;
pub use crate::src::game::g_main::Com_Printf;
pub use crate::src::qcommon::q_shared::ctype_h::tolower;
pub use crate::src::qcommon::q_shared::ctype_h::toupper;
pub use crate::src::qcommon::q_shared::stdlib_float_h::atof;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::_ISalnum;
pub use crate::stdlib::_ISalpha;
pub use crate::stdlib::_ISblank;
pub use crate::stdlib::_IScntrl;
pub use crate::stdlib::_ISdigit;
pub use crate::stdlib::_ISgraph;
pub use crate::stdlib::_ISlower;
pub use crate::stdlib::_ISprint;
pub use crate::stdlib::_ISpunct;
pub use crate::stdlib::_ISspace;
pub use crate::stdlib::_ISupper;
pub use crate::stdlib::_ISxdigit;
pub use crate::stdlib::__ctype_b_loc;
pub use crate::stdlib::__ctype_tolower_loc;
pub use crate::stdlib::__ctype_toupper_loc;
pub use crate::stdlib::__int32_t;
use crate::stdlib::memmove;
use crate::stdlib::strcat;
use crate::stdlib::strchr;
use crate::stdlib::strcmp;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
use crate::stdlib::strncpy;
use crate::stdlib::strrchr;
use crate::stdlib::strtod;
use crate::stdlib::vsnprintf;
#[no_mangle]
pub unsafe extern "C" fn Q_IsColorString(
    mut p: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    if p.is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if *p.offset(0) as i32 != '^' as i32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if *p.offset(1) as i32 == 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    // isalnum expects a signed integer in the range -1 (EOF) to 255, or it might assert on undefined behaviour
    // a dereferenced char pointer has the range -128 to 127, so we just need to rangecheck the negative part
    if (*p.offset(1) as i32) < 0 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if *(*crate::stdlib::__ctype_b_loc()).offset(*p.offset(1) as i32 as isize) as i32
        & crate::stdlib::_ISalnum as u16 as i32
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn Com_Clamp(mut min: f32, mut max: f32, mut value: f32) -> f32 {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn COM_SkipPath(mut pathname: *mut i8) -> *mut i8 {
    let mut last: *mut i8 = 0 as *mut i8;
    last = pathname;
    while *pathname != 0 {
        if *pathname as i32 == '/' as i32 {
            last = pathname.offset(1)
        }
        pathname = pathname.offset(1)
    }
    return last;
}
#[no_mangle]
pub unsafe extern "C" fn COM_GetExtension(mut name: *const i8) -> *const i8 {
    let mut dot: *const i8 = crate::stdlib::strrchr(name, '.' as i32);
    let mut slash: *const i8 = 0 as *const i8;
    if !dot.is_null() && {
        slash = crate::stdlib::strrchr(name, '/' as i32);
        (slash.is_null()) || slash < dot
    } {
        return dot.offset(1isize);
    } else {
        return b"\x00" as *const u8 as *const i8;
    };
}
#[no_mangle]
pub unsafe extern "C" fn COM_StripExtension(
    mut in_0: *const i8,
    mut out: *mut i8,
    mut destsize: i32,
) {
    let mut dot: *const i8 = crate::stdlib::strrchr(in_0, '.' as i32);
    let mut slash: *const i8 = 0 as *const i8;
    if !dot.is_null() && {
        slash = crate::stdlib::strrchr(in_0, '/' as i32);
        (slash.is_null()) || slash < dot
    } {
        destsize = if (destsize as isize) < dot.wrapping_offset_from(in_0) + 1 {
            destsize as isize
        } else {
            (dot.wrapping_offset_from(in_0)) + 1
        } as i32
    }
    if in_0 == out as *const i8 && destsize > 1 {
        *out.offset((destsize - 1i32) as isize) = '\u{0}' as i8
    } else {
        Q_strncpyz(out, in_0, destsize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn COM_CompareExtension(
    mut in_0: *const i8,
    mut ext: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut inlen: i32 = 0;
    let mut extlen: i32 = 0;
    inlen = crate::stdlib::strlen(in_0) as i32;
    extlen = crate::stdlib::strlen(ext) as i32;
    if extlen <= inlen {
        in_0 = in_0.offset((inlen - extlen) as isize);
        if Q_stricmp(in_0, ext) == 0 {
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn COM_DefaultExtension(
    mut path: *mut i8,
    mut maxSize: i32,
    mut extension: *const i8,
) {
    let mut dot: *const i8 = crate::stdlib::strrchr(path, '.' as i32);
    let mut slash: *const i8 = 0 as *const i8;
    if !dot.is_null() && {
        slash = crate::stdlib::strrchr(path, '/' as i32);
        (slash.is_null()) || slash < dot
    } {
        return;
    } else {
        Q_strcat(path, maxSize, extension);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CopyShortSwap(mut dest: *mut libc::c_void, mut src: *mut libc::c_void) {
    let mut to: *mut crate::src::qcommon::q_shared::byte =
        dest as *mut crate::src::qcommon::q_shared::byte;
    let mut from: *mut crate::src::qcommon::q_shared::byte =
        src as *mut crate::src::qcommon::q_shared::byte;
    *to.offset(0) = *from.offset(1);
    *to.offset(1) = *from.offset(0);
}
#[no_mangle]
pub unsafe extern "C" fn CopyLongSwap(mut dest: *mut libc::c_void, mut src: *mut libc::c_void) {
    let mut to: *mut crate::src::qcommon::q_shared::byte =
        dest as *mut crate::src::qcommon::q_shared::byte;
    let mut from: *mut crate::src::qcommon::q_shared::byte =
        src as *mut crate::src::qcommon::q_shared::byte;
    *to.offset(0) = *from.offset(3);
    *to.offset(1) = *from.offset(2);
    *to.offset(2) = *from.offset(1);
    *to.offset(3) = *from.offset(0);
}
#[no_mangle]
pub unsafe extern "C" fn ShortSwap(mut l: i16) -> i16 {
    let mut b1: crate::src::qcommon::q_shared::byte = 0;
    let mut b2: crate::src::qcommon::q_shared::byte = 0;
    b1 = (l as i32 & 255) as crate::src::qcommon::q_shared::byte;
    b2 = (l as i32 >> 8 & 255) as crate::src::qcommon::q_shared::byte;
    return (((b1 as i32) << 8) + b2 as i32) as i16;
}
#[no_mangle]
pub unsafe extern "C" fn ShortNoSwap(mut l: i16) -> i16 {
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn LongSwap(mut l: i32) -> i32 {
    let mut b1: crate::src::qcommon::q_shared::byte = 0;
    let mut b2: crate::src::qcommon::q_shared::byte = 0;
    let mut b3: crate::src::qcommon::q_shared::byte = 0;
    let mut b4: crate::src::qcommon::q_shared::byte = 0;
    b1 = (l & 255i32) as crate::src::qcommon::q_shared::byte;
    b2 = (l >> 8 & 255i32) as crate::src::qcommon::q_shared::byte;
    b3 = (l >> 16 & 255i32) as crate::src::qcommon::q_shared::byte;
    b4 = (l >> 24 & 255i32) as crate::src::qcommon::q_shared::byte;
    return ((b1 as i32) << 24) + ((b2 as i32) << 16) + ((b3 as i32) << 8) + b4 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn LongNoSwap(mut l: i32) -> i32 {
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn Long64Swap(
    mut ll: crate::src::qcommon::q_shared::qint64,
) -> crate::src::qcommon::q_shared::qint64 {
    let mut result: crate::src::qcommon::q_shared::qint64 = crate::src::qcommon::q_shared::qint64 {
        b0: 0,
        b1: 0,
        b2: 0,
        b3: 0,
        b4: 0,
        b5: 0,
        b6: 0,
        b7: 0,
    };
    result.b0 = ll.b7;
    result.b1 = ll.b6;
    result.b2 = ll.b5;
    result.b3 = ll.b4;
    result.b4 = ll.b3;
    result.b5 = ll.b2;
    result.b6 = ll.b1;
    result.b7 = ll.b0;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn Long64NoSwap(
    mut ll: crate::src::qcommon::q_shared::qint64,
) -> crate::src::qcommon::q_shared::qint64 {
    return ll;
}
#[no_mangle]
pub unsafe extern "C" fn FloatSwap(mut f: *const f32) -> f32 {
    let mut out: crate::src::qcommon::q_shared::floatint_t =
        crate::src::qcommon::q_shared::floatint_t { f: 0. };
    out.f = *f;
    out.ui = LongSwap(out.ui as i32) as u32;
    return out.f;
}
#[no_mangle]
pub unsafe extern "C" fn FloatNoSwap(mut f: *const f32) -> f32 {
    return *f;
}
static mut com_token: [i8; 1024] = [0; 1024];
static mut com_parsename: [i8; 1024] = [0; 1024];
static mut com_lines: i32 = 0;
static mut com_tokenline: i32 = 0;
#[no_mangle]
pub unsafe extern "C" fn COM_BeginParseSession(mut name: *const i8) {
    com_lines = 1;
    com_tokenline = 0;
    Com_sprintf(
        com_parsename.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
        b"%s\x00" as *const u8 as *const i8,
        name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn COM_GetCurrentParseLine() -> i32 {
    if com_tokenline != 0 {
        return com_tokenline;
    }
    return com_lines;
}
#[no_mangle]
pub unsafe extern "C" fn COM_Parse(mut data_p: *mut *mut i8) -> *mut i8 {
    return COM_ParseExt(data_p, crate::src::qcommon::q_shared::qtrue);
}
#[no_mangle]
pub unsafe extern "C" fn COM_ParseError(mut format: *mut i8, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [i8; 4096] = [0; 4096];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 4096]>(),
        format,
        argptr.as_va_list(),
    );
    crate::src::game::g_main::Com_Printf(
        b"ERROR: %s, line %d: %s\n\x00" as *const u8 as *const i8,
        com_parsename.as_mut_ptr(),
        COM_GetCurrentParseLine(),
        string.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn COM_ParseWarning(mut format: *mut i8, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [i8; 4096] = [0; 4096];
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        string.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 4096]>(),
        format,
        argptr.as_va_list(),
    );
    crate::src::game::g_main::Com_Printf(
        b"WARNING: %s, line %d: %s\n\x00" as *const u8 as *const i8,
        com_parsename.as_mut_ptr(),
        COM_GetCurrentParseLine(),
        string.as_mut_ptr(),
    );
}
unsafe extern "C" fn SkipWhitespace(
    mut data: *mut i8,
    mut hasNewLines: *mut crate::src::qcommon::q_shared::qboolean,
) -> *mut i8 {
    let mut c: i32 = 0;
    loop {
        c = *data as i32;
        if !(c <= ' ' as i32) {
            break;
        }
        if c == 0 {
            return 0 as *mut i8;
        }
        if c == '\n' as i32 {
            com_lines += 1;
            *hasNewLines = crate::src::qcommon::q_shared::qtrue
        }
        data = data.offset(1)
    }
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn COM_Compress(mut data_p: *mut i8) -> i32 {
    let mut in_0: *mut i8 = 0 as *mut i8;
    let mut out: *mut i8 = 0 as *mut i8;
    let mut c: i32 = 0;
    let mut newline: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut whitespace: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    out = data_p;
    in_0 = out;
    if !in_0.is_null() {
        loop {
            c = *in_0 as i32;
            if !(c != 0) {
                break;
            }
            // skip double slash comments
            if c == '/' as i32 && *in_0.offset(1) as i32 == '/' as i32 {
                while *in_0 as i32 != 0 && *in_0 as i32 != '\n' as i32 {
                    in_0 = in_0.offset(1)
                }
            // skip /* */ comments
            } else if c == '/' as i32 && *in_0.offset(1) as i32 == '*' as i32 {
                while *in_0 as i32 != 0
                    && (*in_0 as i32 != '*' as i32 || *in_0.offset(1) as i32 != '/' as i32)
                {
                    in_0 = in_0.offset(1)
                }
                if *in_0 != 0 {
                    in_0 = in_0.offset(2)
                }
            // record when we hit a newline
            } else if c == '\n' as i32 || c == '\r' as i32 {
                newline = crate::src::qcommon::q_shared::qtrue;
                in_0 = in_0.offset(1)
            // record when we hit whitespace
            } else if c == ' ' as i32 || c == '\t' as i32 {
                whitespace = crate::src::qcommon::q_shared::qtrue;
                in_0 = in_0.offset(1)
            // an actual token
            } else {
                // if we have a pending newline, emit it (and it counts as whitespace)
                if newline as u64 != 0 {
                    let fresh0 = out;
                    out = out.offset(1);
                    *fresh0 = '\n' as i8;
                    newline = crate::src::qcommon::q_shared::qfalse;
                    whitespace = crate::src::qcommon::q_shared::qfalse
                }
                if whitespace as u64 != 0 {
                    let fresh1 = out;
                    out = out.offset(1);
                    *fresh1 = ' ' as i8;
                    whitespace = crate::src::qcommon::q_shared::qfalse
                }
                // copy quoted strings unmolested
                if c == '\"' as i32 {
                    let fresh2 = out;
                    out = out.offset(1);
                    *fresh2 = c as i8;
                    in_0 = in_0.offset(1);
                    loop {
                        c = *in_0 as i32;
                        if !(c != 0 && c != '\"' as i32) {
                            break;
                        }
                        let fresh3 = out;
                        out = out.offset(1);
                        *fresh3 = c as i8;
                        in_0 = in_0.offset(1)
                    }
                    if c == '\"' as i32 {
                        let fresh4 = out;
                        out = out.offset(1);
                        *fresh4 = c as i8;
                        in_0 = in_0.offset(1)
                    }
                } else {
                    *out = c as i8;
                    out = out.offset(1);
                    in_0 = in_0.offset(1)
                }
            }
        }
        *out = 0
    }
    return out.wrapping_offset_from(data_p) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn COM_ParseExt(
    mut data_p: *mut *mut i8,
    mut allowLineBreaks: crate::src::qcommon::q_shared::qboolean,
) -> *mut i8 {
    let mut c: i32 = 0;
    let mut len: i32 = 0;
    let mut hasNewLines: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut data: *mut i8 = 0 as *mut i8;
    data = *data_p;
    len = 0;
    com_token[0] = 0;
    com_tokenline = 0;
    // make sure incoming data is valid
    if data.is_null() {
        *data_p = 0 as *mut i8;
        return com_token.as_mut_ptr();
    }
    loop {
        // skip whitespace
        data = SkipWhitespace(data, &mut hasNewLines);
        if data.is_null() {
            *data_p = 0 as *mut i8;
            return com_token.as_mut_ptr();
        }
        if hasNewLines != 0 && allowLineBreaks as u64 == 0 {
            *data_p = data;
            return com_token.as_mut_ptr();
        }
        c = *data as i32;
        // skip double slash comments
        if c == '/' as i32 && *data.offset(1) as i32 == '/' as i32 {
            data = data.offset(2);
            while *data as i32 != 0 && *data as i32 != '\n' as i32 {
                data = data.offset(1)
            }
        } else {
            // skip /* */ comments
            if !(c == '/' as i32 && *data.offset(1) as i32 == '*' as i32) {
                break;
            }
            data = data.offset(2);
            while *data as i32 != 0
                && (*data as i32 != '*' as i32 || *data.offset(1) as i32 != '/' as i32)
            {
                if *data as i32 == '\n' as i32 {
                    com_lines += 1
                }
                data = data.offset(1)
            }
            if *data != 0 {
                data = data.offset(2)
            }
        }
    }
    // token starts on this line
    com_tokenline = com_lines;
    // handle quoted strings
    if c == '\"' as i32 {
        data = data.offset(1);
        loop {
            let fresh5 = data;
            data = data.offset(1);
            c = *fresh5 as i32;
            if c == '\"' as i32 || c == 0 {
                com_token[len as usize] = 0;
                *data_p = data;
                return com_token.as_mut_ptr();
            }
            if c == '\n' as i32 {
                com_lines += 1
            }
            if len < 1024 - 1 {
                com_token[len as usize] = c as i8;
                len += 1
            }
        }
    }
    loop
    // parse a regular word
    {
        if len < 1024 - 1 {
            com_token[len as usize] = c as i8;
            len += 1
        }
        data = data.offset(1);
        c = *data as i32;
        if !(c > 32) {
            break;
        }
    }
    com_token[len as usize] = 0;
    *data_p = data;
    return com_token.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn COM_MatchToken(mut buf_p: *mut *mut i8, mut match_0: *mut i8) {
    let mut token: *mut i8 = 0 as *mut i8;
    token = COM_Parse(buf_p);
    if crate::stdlib::strcmp(token, match_0) != 0 {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"MatchToken: %s != %s\x00" as *const u8 as *const i8,
            token,
            match_0,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn SkipBracedSection(
    mut program: *mut *mut i8,
    mut depth: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut token: *mut i8 = 0 as *mut i8;
    loop {
        token = COM_ParseExt(program, crate::src::qcommon::q_shared::qtrue);
        if *token.offset(1) as i32 == 0 {
            if *token.offset(0) as i32 == '{' as i32 {
                depth += 1
            } else if *token.offset(0) as i32 == '}' as i32 {
                depth -= 1
            }
        }
        if !(depth != 0 && !(*program).is_null()) {
            break;
        }
    }
    return (depth == 0) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn SkipRestOfLine(mut data: *mut *mut i8) {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut c: i32 = 0;
    p = *data;
    if *p == 0 {
        return;
    }
    loop {
        let fresh6 = p;
        p = p.offset(1);
        c = *fresh6 as i32;
        if !(c != 0) {
            break;
        }
        if !(c == '\n' as i32) {
            continue;
        }
        com_lines += 1;
        break;
    }
    *data = p;
}
#[no_mangle]
pub unsafe extern "C" fn Parse1DMatrix(mut buf_p: *mut *mut i8, mut x: i32, mut m: *mut f32) {
    let mut token: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    COM_MatchToken(buf_p, b"(\x00" as *const u8 as *mut i8);
    i = 0;
    while i < x {
        token = COM_Parse(buf_p);
        *m.offset(i as isize) = atof(token) as f32;
        i += 1
    }
    COM_MatchToken(buf_p, b")\x00" as *const u8 as *mut i8);
}
#[no_mangle]
pub unsafe extern "C" fn Parse2DMatrix(
    mut buf_p: *mut *mut i8,
    mut y: i32,
    mut x: i32,
    mut m: *mut f32,
) {
    let mut i: i32 = 0;
    COM_MatchToken(buf_p, b"(\x00" as *const u8 as *mut i8);
    i = 0;
    while i < y {
        Parse1DMatrix(buf_p, x, m.offset((i * x) as isize));
        i += 1
    }
    COM_MatchToken(buf_p, b")\x00" as *const u8 as *mut i8);
}
#[no_mangle]
pub unsafe extern "C" fn Parse3DMatrix(
    mut buf_p: *mut *mut i8,
    mut z: i32,
    mut y: i32,
    mut x: i32,
    mut m: *mut f32,
) {
    let mut i: i32 = 0;
    COM_MatchToken(buf_p, b"(\x00" as *const u8 as *mut i8);
    i = 0;
    while i < z {
        Parse2DMatrix(buf_p, y, x, m.offset((i * x * y) as isize));
        i += 1
    }
    COM_MatchToken(buf_p, b")\x00" as *const u8 as *mut i8);
}
#[no_mangle]
pub unsafe extern "C" fn Com_HexStrToInt(mut str: *const i8) -> i32 {
    if str.is_null() {
        return -(1i32);
    }
    // check for hex code
    if *str.offset(0) as i32 == '0' as i32
        && *str.offset(1) as i32 == 'x' as i32
        && *str.offset(2) as i32 != '\u{0}' as i32
    {
        let mut i: i32 = 0;
        let mut n: i32 = 0;
        let mut len: i32 = crate::stdlib::strlen(str) as i32;
        i = 2;
        while i < len {
            let mut digit: i8 = 0;
            n *= 16;
            digit = ({
                let mut __res: i32 = 0;
                if ::std::mem::size_of::<i8>() > 1 {
                    if 0 != 0 {
                        let mut __c: i32 = *str.offset(i as isize) as i32;
                        __res = if __c < -(128) || __c > 255 {
                            __c
                        } else {
                            *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                        }
                    } else {
                        __res = tolower(*str.offset(i as isize) as i32)
                    }
                } else {
                    __res = *(*crate::stdlib::__ctype_tolower_loc())
                        .offset(*str.offset(i as isize) as i32 as isize)
                }
                __res
            }) as i8;
            if digit as i32 >= '0' as i32 && digit as i32 <= '9' as i32 {
                digit = (digit as i32 - '0' as i32) as i8
            } else if digit as i32 >= 'a' as i32 && digit as i32 <= 'f' as i32 {
                digit = (digit as i32 - 'a' as i32 + 10) as i8
            } else {
                return -(1i32);
            }
            n += digit as i32;
            i += 1
        }
        return n;
    }
    return -(1);
}
#[no_mangle]
pub unsafe extern "C" fn Q_isprint(mut c: i32) -> i32 {
    if c >= 0x20 && c <= 0x7e {
        return 1i32;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn Q_islower(mut c: i32) -> i32 {
    if c >= 'a' as i32 && c <= 'z' as i32 {
        return 1i32;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isupper(mut c: i32) -> i32 {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        return 1i32;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isalpha(mut c: i32) -> i32 {
    if c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32 {
        return 1i32;
    }
    return 0;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isanumber(mut s: *const i8) -> crate::src::qcommon::q_shared::qboolean {
    let mut p: *mut i8 = 0 as *mut i8;
    let mut d: f64 = 0.;
    if *s as i32 == '\u{0}' as i32 {
        return crate::src::qcommon::q_shared::qfalse;
    }
    d = crate::stdlib::strtod(s, &mut p);
    return (*p as i32 == '\u{0}' as i32) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn Q_isintegral(mut f: f32) -> crate::src::qcommon::q_shared::qboolean {
    return (f as i32 as f32 == f) as crate::src::qcommon::q_shared::qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strncpyz(mut dest: *mut i8, mut src: *const i8, mut destsize: i32) {
    if dest.is_null() {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Q_strncpyz: NULL dest\x00" as *const u8 as *const i8,
        );
    }
    if src.is_null() {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Q_strncpyz: NULL src\x00" as *const u8 as *const i8,
        );
    }
    if destsize < 1 {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Q_strncpyz: destsize < 1\x00" as *const u8 as *const i8,
        );
    }
    crate::stdlib::strncpy(dest, src, (destsize - 1i32) as usize);
    *dest.offset((destsize - 1i32) as isize) = 0i8;
}
#[no_mangle]
pub unsafe extern "C" fn Q_stricmpn(mut s1: *const i8, mut s2: *const i8, mut n: i32) -> i32 {
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    if s1.is_null() {
        if s2.is_null() {
            return 0i32;
        } else {
            return -(1i32);
        }
    } else {
        if s2.is_null() {
            return 1i32;
        }
    }
    loop {
        let fresh7 = s1;
        s1 = s1.offset(1);
        c1 = *fresh7 as i32;
        let fresh8 = s2;
        s2 = s2.offset(1);
        c2 = *fresh8 as i32;
        let fresh9 = n;
        n = n - 1;
        if fresh9 == 0 {
            return 0i32;
            // strings are equal until end point
        }
        if c1 != c2 {
            if c1 >= 'a' as i32 && c1 <= 'z' as i32 {
                c1 -= 'a' as i32 - 'A' as i32
            }
            if c2 >= 'a' as i32 && c2 <= 'z' as i32 {
                c2 -= 'a' as i32 - 'A' as i32
            }
            if c1 != c2 {
                return if c1 < c2 { -(1i32) } else { 1i32 };
            }
        }
        if !(c1 != 0) {
            break;
        }
    }
    return 0;
    // strings are equal
}
#[no_mangle]
pub unsafe extern "C" fn Q_strncmp(mut s1: *const i8, mut s2: *const i8, mut n: i32) -> i32 {
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    loop {
        let fresh10 = s1;
        s1 = s1.offset(1);
        c1 = *fresh10 as i32;
        let fresh11 = s2;
        s2 = s2.offset(1);
        c2 = *fresh11 as i32;
        let fresh12 = n;
        n = n - 1;
        if fresh12 == 0 {
            return 0i32;
            // strings are equal until end point
        }
        if c1 != c2 {
            return if c1 < c2 { -(1i32) } else { 1i32 };
        }
        if !(c1 != 0) {
            break;
        }
    }
    return 0;
    // strings are equal
}
#[no_mangle]
pub unsafe extern "C" fn Q_stricmp(mut s1: *const i8, mut s2: *const i8) -> i32 {
    return if !s1.is_null() && !s2.is_null() {
        Q_stricmpn(s1, s2, 99999)
    } else {
        -(1)
    };
}
#[no_mangle]
pub unsafe extern "C" fn Q_strlwr(mut s1: *mut i8) -> *mut i8 {
    let mut s: *mut i8 = 0 as *mut i8;
    s = s1;
    while *s != 0 {
        *s = ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<i8>() > 1 {
                if 0 != 0 {
                    let mut __c: i32 = *s as i32;
                    __res = if __c < -(128) || __c > 255 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_tolower_loc()).offset(__c as isize)
                    }
                } else {
                    __res = tolower(*s as i32)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_tolower_loc()).offset(*s as i32 as isize)
            }
            __res
        }) as i8;
        s = s.offset(1)
    }
    return s1;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strupr(mut s1: *mut i8) -> *mut i8 {
    let mut s: *mut i8 = 0 as *mut i8;
    s = s1;
    while *s != 0 {
        *s = ({
            let mut __res: i32 = 0;
            if ::std::mem::size_of::<i8>() > 1 {
                if 0 != 0 {
                    let mut __c: i32 = *s as i32;
                    __res = if __c < -(128) || __c > 255 {
                        __c
                    } else {
                        *(*crate::stdlib::__ctype_toupper_loc()).offset(__c as isize)
                    }
                } else {
                    __res = toupper(*s as i32)
                }
            } else {
                __res = *(*crate::stdlib::__ctype_toupper_loc()).offset(*s as i32 as isize)
            }
            __res
        }) as i8;
        s = s.offset(1)
    }
    return s1;
}
#[no_mangle]
pub unsafe extern "C" fn Q_strcat(mut dest: *mut i8, mut size: i32, mut src: *const i8) {
    let mut l1: i32 = 0;
    l1 = crate::stdlib::strlen(dest) as i32;
    if l1 >= size {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Q_strcat: already overflowed\x00" as *const u8 as *const i8,
        );
    }
    Q_strncpyz(dest.offset(l1 as isize), src, size - l1);
}
#[no_mangle]
pub unsafe extern "C" fn Q_stristr(mut s: *const i8, mut find: *const i8) -> *const i8 {
    let mut c: i8 = 0;
    let mut sc: i8 = 0;
    let mut len: crate::stddef_h::size_t = 0;
    let fresh13 = find;
    find = find.offset(1);
    c = *fresh13;
    if c as i32 != 0 {
        if c as i32 >= 'a' as i32 && c as i32 <= 'z' as i32 {
            c = (c as i32 - ('a' as i32 - 'A' as i32)) as i8
        }
        len = crate::stdlib::strlen(find);
        loop {
            loop {
                let fresh14 = s;
                s = s.offset(1);
                sc = *fresh14;
                if sc as i32 == 0 {
                    return 0 as *const i8;
                }
                if sc as i32 >= 'a' as i32 && sc as i32 <= 'z' as i32 {
                    sc = (sc as i32 - ('a' as i32 - 'A' as i32)) as i8
                }
                if !(sc as i32 != c as i32) {
                    break;
                }
            }
            if !(Q_stricmpn(s, find, len as i32) != 0) {
                break;
            }
        }
        s = s.offset(-1)
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn Q_PrintStrlen(mut string: *const i8) -> i32 {
    let mut len: i32 = 0;
    let mut p: *const i8 = 0 as *const i8;
    if string.is_null() {
        return 0i32;
    }
    len = 0;
    p = string;
    while *p != 0 {
        if Q_IsColorString(p) as u64 != 0 {
            p = p.offset(2)
        } else {
            p = p.offset(1);
            len += 1
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn Q_CleanStr(mut string: *mut i8) -> *mut i8 {
    let mut d: *mut i8 = 0 as *mut i8;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut c: i32 = 0;
    s = string;
    d = string;
    loop {
        c = *s as i32;
        if !(c != 0) {
            break;
        }
        if Q_IsColorString(s) as u64 != 0 {
            s = s.offset(1)
        } else if c >= 0x20 && c <= 0x7e {
            let fresh15 = d;
            d = d.offset(1);
            *fresh15 = c as i8
        }
        s = s.offset(1)
    }
    *d = '\u{0}' as i8;
    return string;
}
#[no_mangle]
pub unsafe extern "C" fn Q_CountChar(mut string: *const i8, mut tocount: i8) -> i32 {
    let mut count: i32 = 0;
    count = 0;
    while *string != 0 {
        if *string as i32 == tocount as i32 {
            count += 1
        }
        string = string.offset(1)
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn Com_sprintf(
    mut dest: *mut i8,
    mut size: i32,
    mut fmt: *const i8,
    mut args: ...
) -> i32 {
    let mut len: i32 = 0;
    let mut argptr: ::std::ffi::VaListImpl;
    argptr = args.clone();
    len = crate::stdlib::vsnprintf(dest, size as usize, fmt, argptr.as_va_list());
    if len >= size {
        crate::src::game::g_main::Com_Printf(
            b"Com_sprintf: Output length %d too short, require %d bytes.\n\x00" as *const u8
                as *const i8,
            size,
            len + 1i32,
        );
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn va(mut format: *mut i8, mut args: ...) -> *mut i8 {
    let mut argptr: ::std::ffi::VaListImpl; // in case va is called by nested functions
    static mut string: [[i8; 32000]; 2] = [[0; 32000]; 2];
    static mut index: i32 = 0;
    let mut buf: *mut i8 = 0 as *mut i8;
    buf = string[(index & 1) as usize].as_mut_ptr();
    index += 1;
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        buf,
        ::std::mem::size_of::<[i8; 32000]>(),
        format,
        argptr.as_va_list(),
    );
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn Com_TruncateLongString(mut buffer: *mut i8, mut s: *const i8) {
    let mut length: i32 = crate::stdlib::strlen(s) as i32;
    if length <= 64 {
        Q_strncpyz(buffer, s, 64i32);
    } else {
        Q_strncpyz(buffer, s, 64 / 2 - 3);
        Q_strcat(buffer, 64, b" ... \x00" as *const u8 as *const i8);
        Q_strcat(
            buffer,
            64i32,
            s.offset(length as isize)
                .offset(-((64i32 / 2i32) as isize))
                .offset(3isize),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Info_ValueForKey(mut s: *const i8, mut key: *const i8) -> *mut i8 {
    let mut pkey: [i8; 8192] = [0; 8192]; // use two buffers so compares
    static mut value: [[i8; 8192]; 2] = [[0; 8192]; 2];
    // work without stomping on each other
    static mut valueindex: i32 = 0;
    let mut o: *mut i8 = 0 as *mut i8;
    if s.is_null() || key.is_null() {
        return b"\x00" as *const u8 as *mut i8;
    }
    if crate::stdlib::strlen(s) >= 8192 {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_ValueForKey: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    valueindex ^= 1;
    if *s as i32 == '\\' as i32 {
        s = s.offset(1)
    }
    loop {
        o = pkey.as_mut_ptr();
        while *s as i32 != '\\' as i32 {
            if *s == 0 {
                return b"\x00" as *const u8 as *mut i8;
            }
            let fresh16 = s;
            s = s.offset(1);
            let fresh17 = o;
            o = o.offset(1);
            *fresh17 = *fresh16
        }
        *o = 0;
        s = s.offset(1);
        o = value[valueindex as usize].as_mut_ptr();
        while *s as i32 != '\\' as i32 && *s as i32 != 0 {
            let fresh18 = s;
            s = s.offset(1);
            let fresh19 = o;
            o = o.offset(1);
            *fresh19 = *fresh18
        }
        *o = 0;
        if Q_stricmp(key, pkey.as_mut_ptr()) == 0 {
            return value[valueindex as usize].as_mut_ptr();
        }
        if *s == 0 {
            break;
        }
        s = s.offset(1)
    }
    return b"\x00" as *const u8 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn Info_NextPair(
    mut head: *mut *const i8,
    mut key: *mut i8,
    mut value: *mut i8,
) {
    let mut o: *mut i8 = 0 as *mut i8;
    let mut s: *const i8 = 0 as *const i8;
    s = *head;
    if *s as i32 == '\\' as i32 {
        s = s.offset(1)
    }
    *key.offset(0) = 0i8;
    *value.offset(0) = 0i8;
    o = key;
    while *s as i32 != '\\' as i32 {
        if *s == 0 {
            *o = 0;
            *head = s;
            return;
        }
        let fresh20 = s;
        s = s.offset(1);
        let fresh21 = o;
        o = o.offset(1);
        *fresh21 = *fresh20
    }
    *o = 0;
    s = s.offset(1);
    o = value;
    while *s as i32 != '\\' as i32 && *s as i32 != 0 {
        let fresh22 = s;
        s = s.offset(1);
        let fresh23 = o;
        o = o.offset(1);
        *fresh23 = *fresh22
    }
    *o = 0;
    *head = s;
}
#[no_mangle]
pub unsafe extern "C" fn Info_RemoveKey(mut s: *mut i8, mut key: *const i8) {
    let mut start: *mut i8 = 0 as *mut i8; // remove this part
    let mut pkey: [i8; 1024] = [0; 1024];
    let mut value: [i8; 1024] = [0; 1024];
    let mut o: *mut i8 = 0 as *mut i8;
    if crate::stdlib::strlen(s) >= 1024 {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_RemoveKey: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    if !crate::stdlib::strchr(key, '\\' as i32).is_null() {
        return;
    }
    loop {
        start = s;
        if *s as i32 == '\\' as i32 {
            s = s.offset(1)
        }
        o = pkey.as_mut_ptr();
        while *s as i32 != '\\' as i32 {
            if *s == 0 {
                return;
            }
            let fresh24 = s;
            s = s.offset(1);
            let fresh25 = o;
            o = o.offset(1);
            *fresh25 = *fresh24
        }
        *o = 0;
        s = s.offset(1);
        o = value.as_mut_ptr();
        while *s as i32 != '\\' as i32 && *s as i32 != 0 {
            if *s == 0 {
                return;
            }
            let fresh26 = s;
            s = s.offset(1);
            let fresh27 = o;
            o = o.offset(1);
            *fresh27 = *fresh26
        }
        *o = 0;
        if crate::stdlib::strcmp(key, pkey.as_mut_ptr()) == 0 {
            crate::stdlib::memmove(
                start as *mut libc::c_void,
                s as *const libc::c_void,
                crate::stdlib::strlen(s).wrapping_add(1usize),
            );
            return;
        }
        if *s == 0 {
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Info_RemoveKey_Big(mut s: *mut i8, mut key: *const i8) {
    let mut start: *mut i8 = 0 as *mut i8; // remove this part
    let mut pkey: [i8; 8192] = [0; 8192];
    let mut value: [i8; 8192] = [0; 8192];
    let mut o: *mut i8 = 0 as *mut i8;
    if crate::stdlib::strlen(s) >= 8192 {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_RemoveKey_Big: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    if !crate::stdlib::strchr(key, '\\' as i32).is_null() {
        return;
    }
    loop {
        start = s;
        if *s as i32 == '\\' as i32 {
            s = s.offset(1)
        }
        o = pkey.as_mut_ptr();
        while *s as i32 != '\\' as i32 {
            if *s == 0 {
                return;
            }
            let fresh28 = s;
            s = s.offset(1);
            let fresh29 = o;
            o = o.offset(1);
            *fresh29 = *fresh28
        }
        *o = 0;
        s = s.offset(1);
        o = value.as_mut_ptr();
        while *s as i32 != '\\' as i32 && *s as i32 != 0 {
            if *s == 0 {
                return;
            }
            let fresh30 = s;
            s = s.offset(1);
            let fresh31 = o;
            o = o.offset(1);
            *fresh31 = *fresh30
        }
        *o = 0;
        if crate::stdlib::strcmp(key, pkey.as_mut_ptr()) == 0 {
            crate::stdlib::memmove(
                start as *mut libc::c_void,
                s as *const libc::c_void,
                crate::stdlib::strlen(s).wrapping_add(1usize),
            );
            return;
        }
        if *s == 0 {
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Info_Validate(
    mut s: *const i8,
) -> crate::src::qcommon::q_shared::qboolean {
    if !crate::stdlib::strchr(s, '\"' as i32).is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    if !crate::stdlib::strchr(s, ';' as i32).is_null() {
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn Info_SetValueForKey(
    mut s: *mut i8,
    mut key: *const i8,
    mut value: *const i8,
) {
    let mut newi: [i8; 1024] = [0; 1024];
    let mut blacklist: *const i8 = b"\\;\"\x00" as *const u8 as *const i8;
    if crate::stdlib::strlen(s) >= 1024 {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_SetValueForKey: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    while *blacklist != 0 {
        if !crate::stdlib::strchr(key, *blacklist as i32).is_null()
            || !crate::stdlib::strchr(value, *blacklist as i32).is_null()
        {
            crate::src::game::g_main::Com_Printf(
                b"^3Can\'t use keys or values with a \'%c\': %s = %s\n\x00" as *const u8
                    as *const i8,
                *blacklist as i32,
                key,
                value,
            );
            return;
        }
        blacklist = blacklist.offset(1)
    }
    Info_RemoveKey(s, key);
    if value.is_null() || crate::stdlib::strlen(value) == 0 {
        return;
    }
    Com_sprintf(
        newi.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
        b"\\%s\\%s\x00" as *const u8 as *const i8,
        key,
        value,
    );
    if crate::stdlib::strlen(newi.as_mut_ptr()).wrapping_add(crate::stdlib::strlen(s)) >= 1024usize
    {
        crate::src::game::g_main::Com_Printf(
            b"Info string length exceeded\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    crate::stdlib::strcat(newi.as_mut_ptr(), s);
    crate::stdlib::strcpy(s, newi.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Info_SetValueForKey_Big(
    mut s: *mut i8,
    mut key: *const i8,
    mut value: *const i8,
) {
    let mut newi: [i8; 8192] = [0; 8192];
    let mut blacklist: *const i8 = b"\\;\"\x00" as *const u8 as *const i8;
    if crate::stdlib::strlen(s) >= 8192 {
        crate::src::game::g_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_SetValueForKey: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    while *blacklist != 0 {
        if !crate::stdlib::strchr(key, *blacklist as i32).is_null()
            || !crate::stdlib::strchr(value, *blacklist as i32).is_null()
        {
            crate::src::game::g_main::Com_Printf(
                b"^3Can\'t use keys or values with a \'%c\': %s = %s\n\x00" as *const u8
                    as *const i8,
                *blacklist as i32,
                key,
                value,
            );
            return;
        }
        blacklist = blacklist.offset(1)
    }
    Info_RemoveKey_Big(s, key);
    if value.is_null() {
        return;
    }
    Com_sprintf(
        newi.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 8192]>() as i32,
        b"\\%s\\%s\x00" as *const u8 as *const i8,
        key,
        value,
    );
    if crate::stdlib::strlen(newi.as_mut_ptr()).wrapping_add(crate::stdlib::strlen(s)) >= 8192usize
    {
        crate::src::game::g_main::Com_Printf(
            b"BIG Info string length exceeded\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    crate::stdlib::strcat(s, newi.as_mut_ptr());
}
unsafe extern "C" fn Com_CharIsOneOfCharset(
    mut c: i8,
    mut set: *mut i8,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    i = 0;
    while (i as usize) < crate::stdlib::strlen(set) {
        if *set.offset(i as isize) as i32 == c as i32 {
            return crate::src::qcommon::q_shared::qtrue;
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn Com_SkipCharset(mut s: *mut i8, mut sep: *mut i8) -> *mut i8 {
    let mut p: *mut i8 = s;
    while !p.is_null() {
        if !(Com_CharIsOneOfCharset(*p, sep) as u64 != 0) {
            break;
        }
        p = p.offset(1)
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn Com_SkipTokens(
    mut s: *mut i8,
    mut numTokens: i32,
    mut sep: *mut i8,
) -> *mut i8 {
    let mut sepCount: i32 = 0;
    let mut p: *mut i8 = s;
    while sepCount < numTokens {
        let fresh32 = p;
        p = p.offset(1);
        if Com_CharIsOneOfCharset(*fresh32, sep) as u64 != 0 {
            sepCount += 1;
            while Com_CharIsOneOfCharset(*p, sep) as u64 != 0 {
                p = p.offset(1)
            }
        } else if *p as i32 == '\u{0}' as i32 {
            break;
        }
    }
    if sepCount == numTokens {
        return p;
    } else {
        return s;
    };
}
