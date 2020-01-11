pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::bg_public_h::animation_s;
pub use crate::bg_public_h::animation_t;
pub use crate::cg_public_h::snapshot_t;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvarHandle_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qhandle_t;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::vmCvar_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::tr_types_h::refEntityType_t;
pub use crate::tr_types_h::refEntity_t;
pub use crate::tr_types_h::refdef_t;
pub use crate::tr_types_h::RT_BEAM;
pub use crate::tr_types_h::RT_LIGHTNING;
pub use crate::tr_types_h::RT_MAX_REF_ENTITY_TYPE;
pub use crate::tr_types_h::RT_MODEL;
pub use crate::tr_types_h::RT_POLY;
pub use crate::tr_types_h::RT_PORTALSURFACE;
pub use crate::tr_types_h::RT_RAIL_CORE;
pub use crate::tr_types_h::RT_RAIL_RINGS;
pub use crate::tr_types_h::RT_SPRITE;

pub use crate::cg_local_h::centity_s;
pub use crate::cg_local_h::centity_t;
pub use crate::cg_local_h::cg_t;
pub use crate::cg_local_h::lerpFrame_t;
pub use crate::cg_local_h::playerEntity_t;
pub use crate::cg_local_h::score_t;
pub use crate::src::cgame::cg_consolecmds::stdlib_h::atoi;
pub use crate::src::cgame::cg_main::cg;
pub use crate::src::cgame::cg_main::cg_cameraOrbit;
pub use crate::src::cgame::cg_main::cg_viewsize;
pub use crate::src::cgame::cg_main::CG_Argv;
pub use crate::src::cgame::cg_main::CG_CrosshairPlayer;
pub use crate::src::cgame::cg_main::CG_LastAttacker;
pub use crate::src::cgame::cg_main::CG_Printf;
pub use crate::src::cgame::cg_players::CG_LoadDeferredPlayers;
pub use crate::src::cgame::cg_syscalls::trap_AddCommand;
pub use crate::src::cgame::cg_syscalls::trap_Args;
pub use crate::src::cgame::cg_syscalls::trap_Argv;
pub use crate::src::cgame::cg_syscalls::trap_Cvar_Set;
pub use crate::src::cgame::cg_syscalls::trap_Cvar_VariableStringBuffer;
pub use crate::src::cgame::cg_syscalls::trap_SendClientCommand;
pub use crate::src::cgame::cg_view::CG_TestGun_f;
pub use crate::src::cgame::cg_view::CG_TestModelNextFrame_f;
pub use crate::src::cgame::cg_view::CG_TestModelNextSkin_f;
pub use crate::src::cgame::cg_view::CG_TestModelPrevFrame_f;
pub use crate::src::cgame::cg_view::CG_TestModelPrevSkin_f;
pub use crate::src::cgame::cg_view::CG_TestModel_f;
pub use crate::src::cgame::cg_view::CG_ZoomDown_f;
pub use crate::src::cgame::cg_view::CG_ZoomUp_f;
pub use crate::src::cgame::cg_weapons::CG_NextWeapon_f;
pub use crate::src::cgame::cg_weapons::CG_PrevWeapon_f;
pub use crate::src::cgame::cg_weapons::CG_Weapon_f;
pub use crate::stdlib::strtol;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct consoleCommand_t {
    pub cmd: *mut i8,
    pub function: Option<unsafe extern "C" fn() -> ()>,
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
// cg_consolecmds.c -- text commands typed in at the local console, or
// executed by a key binding
#[no_mangle]

pub unsafe extern "C" fn CG_TargetCommand_f() {
    let mut targetNum: i32 = 0;
    let mut test: [i8; 4] = [0; 4];
    targetNum = crate::src::cgame::cg_main::CG_CrosshairPlayer();
    if targetNum == -(1) {
        return;
    }
    crate::src::cgame::cg_syscalls::trap_Argv(1, test.as_mut_ptr(), 4);
    crate::src::cgame::cg_syscalls::trap_SendClientCommand(crate::src::qcommon::q_shared::va(
        b"gc %i %i\x00" as *const u8 as *mut i8,
        targetNum,
        atoi(test.as_mut_ptr()),
    ));
}
/*
=================
CG_SizeUp_f

Keybinding command
=================
*/

unsafe extern "C" fn CG_SizeUp_f() {
    crate::src::cgame::cg_syscalls::trap_Cvar_Set(
        b"cg_viewsize\x00" as *const u8 as *const i8,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *mut i8,
            crate::src::cgame::cg_main::cg_viewsize.integer + 10i32,
        ),
    );
}
/*
=================
CG_SizeDown_f

Keybinding command
=================
*/

unsafe extern "C" fn CG_SizeDown_f() {
    crate::src::cgame::cg_syscalls::trap_Cvar_Set(
        b"cg_viewsize\x00" as *const u8 as *const i8,
        crate::src::qcommon::q_shared::va(
            b"%i\x00" as *const u8 as *mut i8,
            crate::src::cgame::cg_main::cg_viewsize.integer - 10i32,
        ),
    );
}
/*
=============
CG_Viewpos_f

Debugging command to print the current position
=============
*/

unsafe extern "C" fn CG_Viewpos_f() {
    crate::src::cgame::cg_main::CG_Printf(
        b"(%i %i %i) : %i\n\x00" as *const u8 as *const i8,
        crate::src::cgame::cg_main::cg.refdef.vieworg[0usize] as i32,
        crate::src::cgame::cg_main::cg.refdef.vieworg[1usize] as i32,
        crate::src::cgame::cg_main::cg.refdef.vieworg[2usize] as i32,
        crate::src::cgame::cg_main::cg.refdefViewAngles[1usize] as i32,
    );
}

unsafe extern "C" fn CG_ScoresDown_f() {
    if (crate::src::cgame::cg_main::cg.scoresRequestTime + 2000)
        < crate::src::cgame::cg_main::cg.time
    {
        // the scores are more than two seconds out of data,
        // so request new ones
        crate::src::cgame::cg_main::cg.scoresRequestTime = crate::src::cgame::cg_main::cg.time;
        crate::src::cgame::cg_syscalls::trap_SendClientCommand(
            b"score\x00" as *const u8 as *const i8,
        );
        // leave the current scores up if they were already
        // displayed, but if this is the first hit, clear them out
        if crate::src::cgame::cg_main::cg.showScores as u64 == 0 {
            crate::src::cgame::cg_main::cg.showScores = crate::src::qcommon::q_shared::qtrue;
            crate::src::cgame::cg_main::cg.numScores = 0
        }
    } else {
        // show the cached contents even if they just pressed if it
        // is within two seconds
        crate::src::cgame::cg_main::cg.showScores = crate::src::qcommon::q_shared::qtrue
    };
}

unsafe extern "C" fn CG_ScoresUp_f() {
    if crate::src::cgame::cg_main::cg.showScores as u64 != 0 {
        crate::src::cgame::cg_main::cg.showScores = crate::src::qcommon::q_shared::qfalse;
        crate::src::cgame::cg_main::cg.scoreFadeTime = crate::src::cgame::cg_main::cg.time
    };
}

unsafe extern "C" fn CG_TellTarget_f() {
    let mut clientNum: i32 = 0;
    let mut command: [i8; 128] = [0; 128];
    let mut message: [i8; 128] = [0; 128];
    clientNum = crate::src::cgame::cg_main::CG_CrosshairPlayer();
    if clientNum == -(1) {
        return;
    }
    crate::src::cgame::cg_syscalls::trap_Args(message.as_mut_ptr(), 128);
    crate::src::qcommon::q_shared::Com_sprintf(
        command.as_mut_ptr(),
        128,
        b"tell %i %s\x00" as *const u8 as *const i8,
        clientNum,
        message.as_mut_ptr(),
    );
    crate::src::cgame::cg_syscalls::trap_SendClientCommand(command.as_mut_ptr());
}

unsafe extern "C" fn CG_TellAttacker_f() {
    let mut clientNum: i32 = 0;
    let mut command: [i8; 128] = [0; 128];
    let mut message: [i8; 128] = [0; 128];
    clientNum = crate::src::cgame::cg_main::CG_LastAttacker();
    if clientNum == -(1) {
        return;
    }
    crate::src::cgame::cg_syscalls::trap_Args(message.as_mut_ptr(), 128);
    crate::src::qcommon::q_shared::Com_sprintf(
        command.as_mut_ptr(),
        128,
        b"tell %i %s\x00" as *const u8 as *const i8,
        clientNum,
        message.as_mut_ptr(),
    );
    crate::src::cgame::cg_syscalls::trap_SendClientCommand(command.as_mut_ptr());
}
/*
==================
CG_StartOrbit_f
==================
*/

unsafe extern "C" fn CG_StartOrbit_f() {
    let mut var: [i8; 1024] = [0; 1024];
    crate::src::cgame::cg_syscalls::trap_Cvar_VariableStringBuffer(
        b"developer\x00" as *const u8 as *const i8,
        var.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if atoi(var.as_mut_ptr()) == 0 {
        return;
    }
    if crate::src::cgame::cg_main::cg_cameraOrbit.value != 0f32 {
        crate::src::cgame::cg_syscalls::trap_Cvar_Set(
            b"cg_cameraOrbit\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
        crate::src::cgame::cg_syscalls::trap_Cvar_Set(
            b"cg_thirdPerson\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
    } else {
        crate::src::cgame::cg_syscalls::trap_Cvar_Set(
            b"cg_cameraOrbit\x00" as *const u8 as *const i8,
            b"5\x00" as *const u8 as *const i8,
        );
        crate::src::cgame::cg_syscalls::trap_Cvar_Set(
            b"cg_thirdPerson\x00" as *const u8 as *const i8,
            b"1\x00" as *const u8 as *const i8,
        );
        crate::src::cgame::cg_syscalls::trap_Cvar_Set(
            b"cg_thirdPersonAngle\x00" as *const u8 as *const i8,
            b"0\x00" as *const u8 as *const i8,
        );
        crate::src::cgame::cg_syscalls::trap_Cvar_Set(
            b"cg_thirdPersonRange\x00" as *const u8 as *const i8,
            b"100\x00" as *const u8 as *const i8,
        );
    };
}

static mut commands: [consoleCommand_t; 21] = unsafe {
    [
        {
            let mut init = consoleCommand_t {
                cmd: b"testgun\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_view::CG_TestGun_f as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"testmodel\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_view::CG_TestModel_f as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"nextframe\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_view::CG_TestModelNextFrame_f
                        as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"prevframe\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_view::CG_TestModelPrevFrame_f
                        as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"nextskin\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_view::CG_TestModelNextSkin_f
                        as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"prevskin\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_view::CG_TestModelPrevSkin_f
                        as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"viewpos\x00" as *const u8 as *mut i8,
                function: Some(CG_Viewpos_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"+scores\x00" as *const u8 as *mut i8,
                function: Some(CG_ScoresDown_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"-scores\x00" as *const u8 as *mut i8,
                function: Some(CG_ScoresUp_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"+zoom\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_view::CG_ZoomDown_f as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"-zoom\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_view::CG_ZoomUp_f as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"sizeup\x00" as *const u8 as *mut i8,
                function: Some(CG_SizeUp_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"sizedown\x00" as *const u8 as *mut i8,
                function: Some(CG_SizeDown_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"weapnext\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_weapons::CG_NextWeapon_f as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"weapprev\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_weapons::CG_PrevWeapon_f as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"weapon\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_weapons::CG_Weapon_f as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"tcmd\x00" as *const u8 as *mut i8,
                function: Some(CG_TargetCommand_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"tell_target\x00" as *const u8 as *mut i8,
                function: Some(CG_TellTarget_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"tell_attacker\x00" as *const u8 as *mut i8,
                function: Some(CG_TellAttacker_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"startOrbit\x00" as *const u8 as *mut i8,
                function: Some(CG_StartOrbit_f as unsafe extern "C" fn() -> ()),
            };
            init
        },
        {
            let mut init = consoleCommand_t {
                cmd: b"loaddeferred\x00" as *const u8 as *mut i8,
                function: Some(
                    crate::src::cgame::cg_players::CG_LoadDeferredPlayers
                        as unsafe extern "C" fn() -> (),
                ),
            };
            init
        },
    ]
};
/*
=================
CG_ConsoleCommand

The string has been tokenized and can be retrieved with
Cmd_Argc() / Cmd_Argv()
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_ConsoleCommand() -> crate::src::qcommon::q_shared::qboolean {
    let mut cmd: *const i8 = 0 as *const i8;
    let mut i: i32 = 0;
    cmd = crate::src::cgame::cg_main::CG_Argv(0);
    i = 0;
    while (i as usize)
        < (::std::mem::size_of::<[consoleCommand_t; 21]>())
            .wrapping_div(::std::mem::size_of::<consoleCommand_t>())
    {
        if crate::src::qcommon::q_shared::Q_stricmp(cmd, commands[i as usize].cmd) == 0 {
            commands[i as usize]
                .function
                .expect("non-null function pointer")();
            return crate::src::qcommon::q_shared::qtrue;
        }
        i += 1
    }
    return crate::src::qcommon::q_shared::qfalse;
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
// The entire cgame module is unloaded and reloaded on each level change,
// so there is NO persistant data between levels on the client side.
// If you absolutely need something stored, it can either be kept
// by the server in the server stored userinfos, or stashed in a cvar.
// time for fragments to sink into ground before going away
// amount to scale up the icons when activating
// num frame for '-' stats digit
// very large characters
//=================================================
// player entities need to track more information
// than any other type of entity.
// note that not every player entity is a client entity,
// because corpses after respawn are outside the normal
// client numbering range
// when changing animation, set animationTime to frameTime + lerping time
// The current lerp will finish out, then it will lerp to the new animation
// time when ->oldFrame was exactly on
// time when ->frame will be exactly on
// may include ANIM_TOGGLEBIT
// time when the first frame of the animation will be exact
// flip from 0 to 1
// machinegun spinning
//=================================================
// centity_t have a direct corespondence with gentity_t in the game, but
// only the entityState_t is directly communicated to the cgame
// from cg.frame
// from cg.nextFrame, if available
// true if next is valid to interpolate to
// true if cg.frame holds this entity
// move to playerEntity?
// so missile trails can handle dropped initial packets
// last time this entity was found in a snapshot
// decay the error from this time
// false if origin / angles is an interpolation
// exact interpolated position of entity on this frame
//======================================================================
// local entities are created as a result of events or predicted actions,
// and live independently from all server transmitted entities
// fade alpha instead of rgb
// do not scale size over time
// tumble over time, used for ejecting shells
// sound 1 for kamikaze
// sound 2 for kamikaze
// fragment local entities can leave marks on walls
// fragment local entities can make sounds on impacts
// 1.0 / (endTime - startTime)
// 0.0 = no bounce, 1.0 = perfect
// mark to leave on fragment impact
//======================================================================
// each client has an associated clientInfo_t
// that contains media references necessary to present the
// client model and other color coded effects
// this is regenerated each time a client's configstring changes,
// usually as a result of a userinfo (name, model, etc) change
// 0 = not bot, 1-5 = bot
// updated by score servercmds
// location index for team mode
// you only get this info about your teammates
// in tourney mode
// task in teamplay (offence/defence)
// true when this is a team leader
// so can display quad/flag status
// when clientinfo is changed, the loading of models/skins/sounds
// can be deferred until you are dead, to prevent hitches in
// gameplay
// true if using the new mission pack animations
// true if legs yaw is always the same as torso yaw
// true if torso never changes yaw
// move head in icon views
// from model
// each WP_* weapon enum has an associated weaponInfo_t
// that contains media references necessary to present the
// weapon and its effects
// the hands don't actually draw, they just position the weapon
// so it will rotate centered instead of by tag
// fast firing weapons randomly choose
// each IT_* item has an associated itemInfo_t
// that constains media references necessary to present the
// item and its effects
//======================================================================
// all cg.stepTime, cg.duckTime, cg.landTime, etc are set to cg.time when the action
// occurs, and they will have visible effects for #define STEP_TIME or whatever msec after
// incremented each frame
// taking a level menu screenshot
// don't defer players at initial startup
// don't play voice rewards, because game will end shortly
// there are only one or two snapshot_t that are relevant at a time
// the number of snapshots the client system has received
// the time from latestSnapshotNum, so we don't need to read the snapshot yet
// cg.snap->serverTime <= cg.time
// cg.nextSnap->serverTime > cg.time, or NULL
// (float)( cg.time - cg.frame->serverTime ) / (cg.nextFrame->serverTime - cg.frame->serverTime)
// cg.time - cg.oldTime
// this is the time value that the client
// is rendering at.
// time at last frame, used for missile trails and prediction checking
// either cg.snap->time or cg.nextSnap->time
// 5 min, 1 min, overtime
// set on a map restart to set back the weapon
// during deaths, chasecams, etc
// prediction state
// true if prediction has hit a trigger_teleport
// clear until the first call to CG_PredictPlayerState
// for stair up smoothing
// for duck viewheight smoothing
// for landing hard
// input state sent to server
// auto rotating items
// view rendering
// will be converted to refdef.viewaxis
// zoom key
// information screen text during loading
// scoreboard
// list of names
// length of list
// width in device units
// next time to offset
// current paint x
// current paint x
// current offset from start
// current offset from start
// centerprinting
// low ammo warning state
// 1 = low, 2 = empty
// crosshair client ID
// powerup active flashing
// attacking player
// reward medals
// sound buffer mainly for announcer sounds
// warmup countdown
//==========================
// the pulse around the crosshair is timed separately
// blend blobs
// status bar head
// view movement
// temp working variables for player view
//qboolean cameraMode;		// if rendering from a loaded camera
// development tool
// all of the model, shader, and sound references that are
// loaded at gamestate time are stored in cgMedia_t
// Other media that can be tied to clients, weapons, or items are
// stored in the clientInfo_t, itemInfo_t, weaponInfo_t, and powerupInfo_t
// gib explosions
// wall mark shaders
// powerup shaders
// weapon effect models
// weapon effect shaders
// special effects models
// scoreboard headers
// medals shown during gameplay
// sounds
//sfxHandle_t	sfx_railg;
// teamplay sounds
// tournament sounds
// The client game static (cgs) structure hold everything
// loaded or calculated from the gamestate.  It will NOT
// be cleared when a tournement restart is done, allowing
// all clients to begin playing instantly
// gamestate from server
// rendering configuration
// derived from glconfig
// reliable command stream counter
// the number of snapshots cgame has requested
// detected on startup by checking sv_running
// parsed from serverinfo
// beep whenever changed
// beep whenever changed
// from configstrings
// flag status from configstrings
//
// locally derived information from gamestate
//
// teamchat width is *3 because of embedded color codes
// orders
// media
//==============================================================================
//extern	vmCvar_t		cg_pmove_fixed;
//
// cg_main.c
//
//
// cg_view.c
//
//
// cg_drawtools.c
//
//
// cg_draw.c, cg_newDraw.c
//
//
// cg_player.c
//
//
// cg_predict.c
//
//
// cg_events.c
//
//
// cg_ents.c
//
//
// cg_weapons.c
//
// should this be in pmove?
//
// cg_marks.c
//
//
// cg_localents.c
//
//
// cg_effects.c
//
//
// cg_snapshot.c
//
//
// cg_info.c
//
//
// cg_scoreboard.c
//
//
// cg_consolecmds.c
//
/*
=================
CG_InitConsoleCommands

Let the client system know about all of our commands
so it can perform tab completion
=================
*/
#[no_mangle]

pub unsafe extern "C" fn CG_InitConsoleCommands() {
    let mut i: i32 = 0;
    i = 0;
    while (i as usize)
        < (::std::mem::size_of::<[consoleCommand_t; 21]>())
            .wrapping_div(::std::mem::size_of::<consoleCommand_t>())
    {
        crate::src::cgame::cg_syscalls::trap_AddCommand(commands[i as usize].cmd);
        i += 1
    }
    //
    // the game server will interpret these commands, which will be automatically
    // forwarded to the server after they are not recognized locally
    //
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"kill\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"say\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"say_team\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"tell\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"give\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"god\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"notarget\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"noclip\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"where\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"team\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"follow\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"follownext\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"followprev\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"levelshot\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"addbot\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"setviewpos\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"callvote\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"vote\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"callteamvote\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"teamvote\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"stats\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"teamtask\x00" as *const u8 as *const i8);
    crate::src::cgame::cg_syscalls::trap_AddCommand(b"loaddefered\x00" as *const u8 as *const i8);
    // spelled wrong, but not changing for demo
}
