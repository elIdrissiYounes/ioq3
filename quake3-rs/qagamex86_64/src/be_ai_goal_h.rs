#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_goal_s {
    pub origin: crate::src::qcommon::q_shared::vec3_t,
    pub areanum: libc::c_int,
    pub mins: crate::src::qcommon::q_shared::vec3_t,
    pub maxs: crate::src::qcommon::q_shared::vec3_t,
    pub entitynum: libc::c_int,
    pub number: libc::c_int,
    pub flags: libc::c_int,
    pub iteminfo: libc::c_int,
}
// true if updated this frame

// entity type

// entity flags

// local time

// time between last and current update

// number of the entity

// origin of the entity

// angles of the model

// for lerping

// last visible origin

// bounding box minimums

// bounding box maximums

// ground entity

// solid type

// model used

// weapons, CTF flags, etc

// model frame number

// impulse events -- muzzle flashes, footsteps, etc

// even parameter

// bit flags

// determines weapon and flash model, etc

// mask off ANIM_TOGGLEBIT

// mask off ANIM_TOGGLEBIT

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
 * name:		be_ai_goal.h
 *
 * desc:		goal AI
 *
 * $Archive: /source/code/botlib/be_ai_goal.h $
 *
 *****************************************************************************/

//a bot goal
pub type bot_goal_t = crate::be_ai_goal_h::bot_goal_s;
