pub type fileHandle_t = i32;
pub type fsMode_t = u32;
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
pub const FS_APPEND_SYNC: crate::src::qcommon::q_shared::fsMode_t = 3;
pub const FS_APPEND: crate::src::qcommon::q_shared::fsMode_t = 2;
pub const FS_WRITE: crate::src::qcommon::q_shared::fsMode_t = 1;
pub const FS_READ: crate::src::qcommon::q_shared::fsMode_t = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct orientation_t {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub axis: [crate::src::qcommon::q_shared::vec3_t; 3],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct markFragment_t {
    pub firstPoint: i32,
    pub numPoints: i32,
}
pub const ERR_NEED_CD: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const ERR_DISCONNECT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
pub const ERR_SERVERDISCONNECT: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const ERR_DROP: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const ERR_FATAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
//=====================================================================

// in order from highest priority to lowest

// if none of the catchers are active, bound key strings will be executed

// sound channels

// channel 0 never willingly overrides

// other channels will allways override a playing sound on that channel
pub type C2RustUnnamed_0 = u32;
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
pub type byte = u8;
pub type qboolean = u32;
pub type qhandle_t = i32;
/*
==============================================================

MATHLIB

==============================================================
*/
pub type vec_t = f32;
pub type vec3_t = [crate::src::qcommon::q_shared::vec_t; 3];
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
//===================================================================

// if entityState->solid == SOLID_BMODEL, modelindex is an inline model number
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
pub type sfxHandle_t = i32;
pub type clipHandle_t = i32;
pub type vec4_t = [crate::src::qcommon::q_shared::vec_t; 4];
/*
// if your system does not have lrintf() and round() you can try this block. Please also open a bug report at bugzilla.icculus.org
// or write a mail to the ioq3 mailing list.
#else
  #define Q_ftol(v) ((long) (v))
  #define Q_round(v) do { if((v) < 0) (v) -= 0.5f; else (v) += 0.5f; (v) = Q_ftol((v)); } while(0)
  #define Q_SnapVector(vec) \
    do\
    {\
        vec3_t *temp = (vec);\
        \
        Q_round((*temp)[0]);\
        Q_round((*temp)[1]);\
        Q_round((*temp)[2]);\
    } while(0)
#endif
*/

// reciprocal square root

// this isn't a real cheap function to call!

// just in case you don't want to use the macros

// fast vector normalize routine that does not check to make sure

// that length != 0, nor does it return length, uses rsqrt approximation

// returns vector length

// perpendicular vector could be replaced by this

//int	PlaneTypeForNormal (vec3_t normal);

//=============================================

//int		COM_ParseInfos( char *buf, int max, char infos[][MAX_INFO_STRING] );

//token types

// string

// literal

// number

// name

// punctuation

// data is an in/out parm, returns a parsed out token

// mode parm for FS_FOpenFile

//=============================================

// portable case insensitive compare

// buffer size safe library replacements

// strlen that discounts Quake color sequences

// removes color sequences from string

// Count the number of char tocount encountered in string

//=============================================

// 64-bit integers for global rankings interface

// implemented as a struct for qvm compatibility

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

// this is only here so the functions in q_shared.c and bg_*.c can link

/*
==========================================================

CVARS (console variables)

Many variables can be used for cheating purposes, so when
cheats is zero, force all unspecified variables to their
default values.
==========================================================
*/

// set to cause it to be saved to vars.rc

// used for system variables, not for player

// specific configurations

// sent to server on connect or change

// sent in response to front end requests

// these cvars will be duplicated on all clients

// don't allow change from console at all,

// but can be set from the command line

// will only change when C code next does

// a Cvar_Get(), so it can't be changed

// without proper initialization.  modified

// will be set, even though the value hasn't

// changed yet

// display only, cannot be set by user at all

// created by a set command

// can be set even when cheats are disabled, but is not archived

// can not be changed if cheats are disabled

// do not clear when a cvar_restart is issued

// cvar was created by a server the client connected to.

// cvar was created exclusively in one of the VMs.

// prevent modifying this var from VMs or the server

// These flags are only returned by the Cvar_Flags() function

// Cvar was modified

// Cvar doesn't exist.

// nothing outside the Cvar_*() functions should modify these fields!

// cvar_restart will reset to this value

// for CVAR_LATCH vars

// set each time the cvar is changed

// incremented each time the cvar is changed

// atof( string )

// atoi( string )

// the modules that run in the virtual machine can't access the cvar_t directly,

// so they must ask for structured updates

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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cplane_s {
    pub normal: crate::src::qcommon::q_shared::vec3_t,
    pub dist: f32,
    pub type_0: crate::src::qcommon::q_shared::byte,
    pub signbits: crate::src::qcommon::q_shared::byte,
    pub pad: [crate::src::qcommon::q_shared::byte; 2],
}
pub type cplane_t = crate::src::qcommon::q_shared::cplane_s;
// a trace is returned when a box is swept through the world
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
/*
========================================================================

  ELEMENTS COMMUNICATED ACROSS THE NET

========================================================================
*/

// snapshot used during connection and for zombies

// toggled every map_restart so transitions can be detected

//

// per-level limits

//

// absolute limit

// don't need to send any more

// entitynums are communicated with GENTITY_BITS, so any reserved

// values that are going to be communcated over the net need to

// also be in this range

// these are sent over the net as 8 bits

// so they cannot be blindly increased

// these are the only configstrings that the system reserves, all the

// other ones are strictly for servergame to clientgame communication

// an info string with all the serverinfo cvars

// an info string for server system to client system configuration (timescale, etc)

// game can't modify below this, only the system can
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gameState_t {
    pub stringOffsets: [i32; 1024],
    pub stringData: [i8; 16000],
    pub dataCount: i32,
}
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
pub const qtrue: crate::src::qcommon::q_shared::qboolean = 1;
pub const qfalse: crate::src::qcommon::q_shared::qboolean = 0;
// value = base + sin( time / duration ) * delta
pub const TR_GRAVITY: crate::src::qcommon::q_shared::trType_t = 5;
pub const TR_SINE: crate::src::qcommon::q_shared::trType_t = 4;
pub const TR_LINEAR_STOP: crate::src::qcommon::q_shared::trType_t = 3;
// non-parametric, but interpolate between snapshots
pub const TR_LINEAR: crate::src::qcommon::q_shared::trType_t = 2;
pub const TR_INTERPOLATE: crate::src::qcommon::q_shared::trType_t = 1;
pub const TR_STATIONARY: crate::src::qcommon::q_shared::trType_t = 0;
// announcer voices, etc

// chat messages, etc
pub const CHAN_ANNOUNCER: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 7;
pub const CHAN_LOCAL_SOUND: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 6;
pub const CHAN_BODY: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 5;
pub const CHAN_ITEM: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 4;
pub const CHAN_VOICE: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 3;
// menu sounds, etc
pub const CHAN_WEAPON: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 2;
pub const CHAN_LOCAL: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 1;
pub const CHAN_AUTO: crate::src::qcommon::q_shared::C2RustUnnamed_0 = 0;
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
pub struct glyphInfo_t {
    pub height: i32,
    pub top: i32,
    pub bottom: i32,
    pub pitch: i32,
    pub xSkip: i32,
    pub imageWidth: i32,
    pub imageHeight: i32,
    pub s: f32,
    pub t: f32,
    pub s2: f32,
    pub t2: f32,
    pub glyph: crate::src::qcommon::q_shared::qhandle_t,
    pub shaderName: [i8; 32],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fontInfo_t {
    pub glyphs: [crate::src::qcommon::q_shared::glyphInfo_t; 256],
    pub glyphScale: f32,
    pub name: [i8; 64],
}
// entityState_t is the information conveyed from the server

// in an update message about entities that the client will

// need to render in some way

// Different eTypes may use the information in different ways

// The messages are delta compressed, so it doesn't really matter if

// the structure size is fairly large

// entity index

// entityType_t

// for calculating position

// for calculating angles

// shotgun sources, etc

// ENTITYNUM_NONE = in air

// r + (g<<8) + (b<<16) + (intensity<<24)

// constantly loop this sound

// 0 to (MAX_CLIENTS - 1), for players and corpses

// for client side prediction, trap_linkentity sets this properly

// impulse events -- muzzle flashes, footsteps, etc

// for players

// bit flags

// determines weapon and flash model, etc

// mask off ANIM_TOGGLEBIT

// mask off ANIM_TOGGLEBIT

// not talking to a server

// not used any more, was checking cd key

// sending request packets to the server

// sending challenge packets to the server

// netchan_t established, getting gamestate

// only during cgame initialization, never during main loop

// got gamestate, waiting for first frame

// game views should be displayed

// playing a cinematic or a static pic, not connected to a server

// font support

// number of scan lines

// top of glyph in buffer

// bottom of glyph in buffer

// width for copying

// x adjustment

// width of actual image

// height of actual image

// x offset in image where glyph starts

// y offset in image where glyph starts

// handle to the shader with the glyph

// real time

//=============================================
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
pub type e_status = u32;
pub const FMV_ID_WAIT: crate::src::qcommon::q_shared::e_status = 6;
pub const FMV_LOOPED: crate::src::qcommon::q_shared::e_status = 5;
pub const FMV_ID_IDLE: crate::src::qcommon::q_shared::e_status = 4;
pub const FMV_ID_BLT: crate::src::qcommon::q_shared::e_status = 3;
pub const FMV_EOF: crate::src::qcommon::q_shared::e_status = 2;
pub const FMV_PLAY: crate::src::qcommon::q_shared::e_status = 1;
pub const FMV_IDLE: crate::src::qcommon::q_shared::e_status = 0;
// scale the velocity

// accelerate

// move

//============================================================================

/*
================
PM_FootstepForSurface

Returns an event number appropriate for the groundsurface
================
*/

/*
=================
PM_CrashLand

Check for hard landings that generate sound events
=================
*/

// decide which landing animation to use

// calculate the exact velocity on landing

// ducking while falling doubles damage

// never take falling damage if completely underwater

// reduce falling damage if there is standing water

// create a local entity event to play the sound

// SURF_NODAMAGE is used for bounce pads where you don't ever

// want to take damage or play a crunch sound

// this is a pain grunt, so don't play it if dead

// start footstep cycle over

/*
=============
PM_CheckStuck
=============
*/

/*
void PM_CheckStuck(void) {
    trace_t trace;

    pm->trace (&trace, pm->ps->origin, pm->mins, pm->maxs, pm->ps->origin, pm->ps->clientNum, pm->tracemask);
    if (trace.allsolid) {
        //int shit = qtrue;
    }
}
*/

/*
=============
PM_CorrectAllSolid
=============
*/

// jitter around

/*
=============
PM_GroundTraceMissed

The ground trace didn't hit a surface, so we are in freefall
=============
*/

// we just transitioned into freefall

// if they aren't in a jumping animation and the ground is a ways away, force into it

// if we didn't do the trace, the player would be backflipping down staircases

/*
=============
PM_GroundTrace
=============
*/

// do something corrective if the trace starts in a solid...

// if the trace didn't hit anything, we are in free fall

// check if getting thrown off the ground

// go into jump animation

// slopes that are too steep will not be considered onground

// FIXME: if they can't slide down the slope, let them

// walk (sharp crevices)

// hitting solid ground will end a waterjump

// just hit the ground

// don't do landing time if we were just going down a slope

// don't allow another jump for a little while

// don't reset the z velocity for slopes

//	pm->ps->velocity[2] = 0;

/*
=============
PM_SetWaterLevel	FIXME: avoid this twice?  certainly if not moving
=============
*/

//

// get waterlevel, accounting for ducking

//

/*
==============
PM_CheckDuck

Sets mins, maxs, and pm->ps->viewheight
==============
*/

// invulnerability sphere has a 42 units radius

// duck

// stand up if possible

// try to stand up

//===================================================================

/*
===============
PM_Footsteps
===============
*/

//

// calculate speed and cycle to be used for

// all cyclic walking effects

//

// airborne leaves position in cycle intact, but doesn't advance

// if not trying to move
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
pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::src::cgame::cg_main::Com_Error;
pub use crate::src::cgame::cg_main::Com_Printf;
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
    crate::src::cgame::cg_main::Com_Printf(
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
    crate::src::cgame::cg_main::Com_Printf(
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
        crate::src::cgame::cg_main::Com_Error(
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

    for i in 0..x {
        token = COM_Parse(buf_p);

        *m.offset(i as isize) = atof(token) as f32;
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

    for i in 0..y {
        Parse1DMatrix(buf_p, x, m.offset((i * x) as isize));
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

    for i in 0..z {
        Parse2DMatrix(buf_p, y, x, m.offset((i * x * y) as isize));
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

        for i in 2..len {
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
        crate::src::cgame::cg_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Q_strncpyz: NULL dest\x00" as *const u8 as *const i8,
        );
    }
    if src.is_null() {
        crate::src::cgame::cg_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"Q_strncpyz: NULL src\x00" as *const u8 as *const i8,
        );
    }
    if destsize < 1 {
        crate::src::cgame::cg_main::Com_Error(
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
        crate::src::cgame::cg_main::Com_Error(
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
        crate::src::cgame::cg_main::Com_Printf(
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
        crate::src::cgame::cg_main::Com_Error(
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
        crate::src::cgame::cg_main::Com_Error(
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
        crate::src::cgame::cg_main::Com_Error(
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
        crate::src::cgame::cg_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_SetValueForKey: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    while *blacklist != 0 {
        if !crate::stdlib::strchr(key, *blacklist as i32).is_null()
            || !crate::stdlib::strchr(value, *blacklist as i32).is_null()
        {
            crate::src::cgame::cg_main::Com_Printf(
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
        crate::src::cgame::cg_main::Com_Printf(
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
        crate::src::cgame::cg_main::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Info_SetValueForKey: oversize infostring\x00" as *const u8 as *const i8,
        );
    }
    while *blacklist != 0 {
        if !crate::stdlib::strchr(key, *blacklist as i32).is_null()
            || !crate::stdlib::strchr(value, *blacklist as i32).is_null()
        {
            crate::src::cgame::cg_main::Com_Printf(
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
        crate::src::cgame::cg_main::Com_Printf(
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
