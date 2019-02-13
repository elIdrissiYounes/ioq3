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
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
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
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/g_local.h"]
pub mod g_local_h {
    // ai_main.c
    //bot settings
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_settings_s {
        pub characterfile: [libc::c_char; 144],
        pub skill: libc::c_float,
    }
    pub type bot_settings_t = bot_settings_s;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn trap_BotInitialChat(chatstate: libc::c_int,
                                   type_0: *mut libc::c_char,
                                   mcontext: libc::c_int,
                                   var0: *mut libc::c_char,
                                   var1: *mut libc::c_char,
                                   var2: *mut libc::c_char,
                                   var3: *mut libc::c_char,
                                   var4: *mut libc::c_char,
                                   var5: *mut libc::c_char,
                                   var6: *mut libc::c_char,
                                   var7: *mut libc::c_char);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_goal.h"]
pub mod be_ai_goal_h {
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
/*****************************************************************************
 * name:		be_ai_goal.h
 *
 * desc:		goal AI
 *
 * $Archive: /source/code/botlib/be_ai_goal.h $
 *
 *****************************************************************************/
    //a bot goal
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
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
    use super::q_shared_h::{vec3_t};
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/ai_main.h"]
pub mod ai_main_h {
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
 * name:		ai_main.h
 *
 * desc:		Quake3 bot AI
 *
 * $Archive: /source/code/botai/ai_chat.c $
 *
 *****************************************************************************/
    //#define DEBUG
    //bot flags
    //strafe to the right
    //bot has attacked last ai frame
    //bot jumped during attack last frame
    //bot aimed at the enemy this frame
    //avoid obstacles by going to the right
    //bot has ideal view angles set
    //bot is in a suicidal fight
    //long term goal types
    //help a team mate
    //accompany a team mate
    //defend a key area
    //get the enemy flag
    //rush to the base
    //return the flag
    //camp somewhere
    //ordered to camp somewhere
    //patrol
    //get an item
    //kill someone
    //harvest skulls
    //attack the enemy base
    //some goal dedication times
    //1 minute teamplay help time
    //10 minutes teamplay accompany time
    //10 minutes ctf defend base time
    //10 minutes camping time
    //10 minutes patrolling time
    //10 minutes taking the lead
    //1 minute
    //3 minute to kill someone
    //10 minutes
    //2 minutes
    //10 minutes ctf get flag time
    //2 minutes ctf rush base time
    //3 minutes to return the flag
    //1 minute ctf roam time
    //patrol flags
    //teamplay task preference
    //CTF strategy
    //copied from the aas file header
    //
    //check points
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_waypoint_s {
        pub inuse: libc::c_int,
        pub name: [libc::c_char; 32],
        pub goal: bot_goal_t,
        pub next: *mut bot_waypoint_s,
        pub prev: *mut bot_waypoint_s,
    }
    pub type bot_waypoint_t = bot_waypoint_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_activategoal_s {
        pub inuse: libc::c_int,
        pub goal: bot_goal_t,
        pub time: libc::c_float,
        pub start_time: libc::c_float,
        pub justused_time: libc::c_float,
        pub shoot: libc::c_int,
        pub weapon: libc::c_int,
        pub target: vec3_t,
        pub origin: vec3_t,
        pub areas: [libc::c_int; 32],
        pub numareas: libc::c_int,
        pub areasdisabled: libc::c_int,
        pub next: *mut bot_activategoal_s,
    }
    pub type bot_activategoal_t = bot_activategoal_s;
    //bot state
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_state_s {
        pub inuse: libc::c_int,
        pub botthink_residual: libc::c_int,
        pub client: libc::c_int,
        pub entitynum: libc::c_int,
        pub cur_ps: playerState_t,
        pub last_eFlags: libc::c_int,
        pub lastucmd: usercmd_t,
        pub entityeventTime: [libc::c_int; 1024],
        pub settings: bot_settings_t,
        pub ainode: Option<unsafe extern "C" fn(_: *mut bot_state_s)
                               -> libc::c_int>,
        pub thinktime: libc::c_float,
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub presencetype: libc::c_int,
        pub eye: vec3_t,
        pub areanum: libc::c_int,
        pub inventory: [libc::c_int; 256],
        pub tfl: libc::c_int,
        pub flags: libc::c_int,
        pub respawn_wait: libc::c_int,
        pub lasthealth: libc::c_int,
        pub lastkilledplayer: libc::c_int,
        pub lastkilledby: libc::c_int,
        pub botdeathtype: libc::c_int,
        pub enemydeathtype: libc::c_int,
        pub botsuicide: libc::c_int,
        pub enemysuicide: libc::c_int,
        pub setupcount: libc::c_int,
        pub map_restart: libc::c_int,
        pub entergamechat: libc::c_int,
        pub num_deaths: libc::c_int,
        pub num_kills: libc::c_int,
        pub revenge_enemy: libc::c_int,
        pub revenge_kills: libc::c_int,
        pub lastframe_health: libc::c_int,
        pub lasthitcount: libc::c_int,
        pub chatto: libc::c_int,
        pub walker: libc::c_float,
        pub ltime: libc::c_float,
        pub entergame_time: libc::c_float,
        pub ltg_time: libc::c_float,
        pub nbg_time: libc::c_float,
        pub respawn_time: libc::c_float,
        pub respawnchat_time: libc::c_float,
        pub chase_time: libc::c_float,
        pub enemyvisible_time: libc::c_float,
        pub check_time: libc::c_float,
        pub stand_time: libc::c_float,
        pub lastchat_time: libc::c_float,
        pub kamikaze_time: libc::c_float,
        pub invulnerability_time: libc::c_float,
        pub standfindenemy_time: libc::c_float,
        pub attackstrafe_time: libc::c_float,
        pub attackcrouch_time: libc::c_float,
        pub attackchase_time: libc::c_float,
        pub attackjump_time: libc::c_float,
        pub enemysight_time: libc::c_float,
        pub enemydeath_time: libc::c_float,
        pub enemyposition_time: libc::c_float,
        pub defendaway_time: libc::c_float,
        pub defendaway_range: libc::c_float,
        pub rushbaseaway_time: libc::c_float,
        pub attackaway_time: libc::c_float,
        pub harvestaway_time: libc::c_float,
        pub ctfroam_time: libc::c_float,
        pub killedenemy_time: libc::c_float,
        pub arrive_time: libc::c_float,
        pub lastair_time: libc::c_float,
        pub teleport_time: libc::c_float,
        pub camp_time: libc::c_float,
        pub weaponchange_time: libc::c_float,
        pub firethrottlewait_time: libc::c_float,
        pub firethrottleshoot_time: libc::c_float,
        pub notblocked_time: libc::c_float,
        pub blockedbyavoidspot_time: libc::c_float,
        pub predictobstacles_time: libc::c_float,
        pub predictobstacles_goalareanum: libc::c_int,
        pub aimtarget: vec3_t,
        pub enemyvelocity: vec3_t,
        pub enemyorigin: vec3_t,
        pub kamikazebody: libc::c_int,
        pub proxmines: [libc::c_int; 64],
        pub numproxmines: libc::c_int,
        pub character: libc::c_int,
        pub ms: libc::c_int,
        pub gs: libc::c_int,
        pub cs: libc::c_int,
        pub ws: libc::c_int,
        pub enemy: libc::c_int,
        pub lastenemyareanum: libc::c_int,
        pub lastenemyorigin: vec3_t,
        pub weaponnum: libc::c_int,
        pub viewangles: vec3_t,
        pub ideal_viewangles: vec3_t,
        pub viewanglespeed: vec3_t,
        pub ltgtype: libc::c_int,
        pub teammate: libc::c_int,
        pub decisionmaker: libc::c_int,
        pub ordered: libc::c_int,
        pub order_time: libc::c_float,
        pub owndecision_time: libc::c_int,
        pub teamgoal: bot_goal_t,
        pub altroutegoal: bot_goal_t,
        pub reachedaltroutegoal_time: libc::c_float,
        pub teammessage_time: libc::c_float,
        pub teamgoal_time: libc::c_float,
        pub teammatevisible_time: libc::c_float,
        pub teamtaskpreference: libc::c_int,
        pub lastgoal_decisionmaker: libc::c_int,
        pub lastgoal_ltgtype: libc::c_int,
        pub lastgoal_teammate: libc::c_int,
        pub lastgoal_teamgoal: bot_goal_t,
        pub lead_teammate: libc::c_int,
        pub lead_teamgoal: bot_goal_t,
        pub lead_time: libc::c_float,
        pub leadvisible_time: libc::c_float,
        pub leadmessage_time: libc::c_float,
        pub leadbackup_time: libc::c_float,
        pub teamleader: [libc::c_char; 36],
        pub askteamleader_time: libc::c_float,
        pub becometeamleader_time: libc::c_float,
        pub teamgiveorders_time: libc::c_float,
        pub lastflagcapture_time: libc::c_float,
        pub numteammates: libc::c_int,
        pub redflagstatus: libc::c_int,
        pub blueflagstatus: libc::c_int,
        pub neutralflagstatus: libc::c_int,
        pub flagstatuschanged: libc::c_int,
        pub forceorders: libc::c_int,
        pub flagcarrier: libc::c_int,
        pub ctfstrategy: libc::c_int,
        pub subteam: [libc::c_char; 32],
        pub formation_dist: libc::c_float,
        pub activatestack: *mut bot_activategoal_t,
        pub activategoalheap: [bot_activategoal_t; 8],
        pub checkpoints: *mut bot_waypoint_t,
        pub patrolpoints: *mut bot_waypoint_t,
        pub curpatrolpoint: *mut bot_waypoint_t,
        pub patrolflags: libc::c_int,
    }
    pub type bot_state_t = bot_state_s;
    use super::{libc};
    use super::be_ai_goal_h::{bot_goal_t};
    use super::q_shared_h::{vec3_t, playerState_t, usercmd_t};
    use super::g_local_h::{bot_settings_t};
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
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/g_variadic.h"]
pub mod g_variadic_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn G_Printf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn G_Error(fmt: *const libc::c_char, ...) -> !;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/ai_dmq3.h"]
pub mod ai_dmq3_h {
    use super::{libc};
    use super::ai_main_h::{bot_state_t};
    extern "C" {
        // returns the appropriate synonym context for the current game type and situation
        #[no_mangle]
        pub fn BotSynonymContext(bs: *mut bot_state_t) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/ai_variadic.h"]
pub mod ai_variadic_h {
    use super::{libc};
    use super::ai_main_h::{bot_state_t};
}
use self::vararg::{__builtin_va_list, __va_list_tag};
use self::stdarg_h::{va_list};
use self::q_shared_h::{byte, vec_t, vec3_t, playerState_s, playerState_t,
                       usercmd_s, usercmd_t};
use self::g_local_h::{bot_settings_s, bot_settings_t, trap_BotInitialChat};
use self::be_ai_goal_h::{bot_goal_s, bot_goal_t};
use self::ai_main_h::{bot_waypoint_s, bot_waypoint_t, bot_activategoal_s,
                      bot_activategoal_t, bot_state_s, bot_state_t};
use self::stdio_h::{vsnprintf};
use self::string_h::{memset};
use self::g_variadic_h::{G_Printf, G_Error};
use self::ai_dmq3_h::{BotSynonymContext};
#[no_mangle]
pub unsafe extern "C" fn BotAI_Print(mut type_0: libc::c_int,
                                     mut fmt: *mut libc::c_char, ...) {
    let mut str: [libc::c_char; 2048] = [0; 2048];
    vsnprintf(str.as_mut_ptr(),
              ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong,
              fmt, ap);
    match type_0 {
        1 => {
            G_Printf(b"%s\x00" as *const u8 as *const libc::c_char,
                     str.as_mut_ptr());
        }
        2 => {
            G_Printf(b"^3Warning: %s\x00" as *const u8 as *const libc::c_char,
                     str.as_mut_ptr());
        }
        3 => {
            G_Printf(b"^1Error: %s\x00" as *const u8 as *const libc::c_char,
                     str.as_mut_ptr());
        }
        4 => {
            G_Printf(b"^1Fatal: %s\x00" as *const u8 as *const libc::c_char,
                     str.as_mut_ptr());
        }
        5 => {
            G_Error(b"^1Exit: %s\x00" as *const u8 as *const libc::c_char,
                    str.as_mut_ptr());
        }
        _ => {
            G_Printf(b"unknown print type\n\x00" as *const u8 as
                         *const libc::c_char);
        }
    };
}