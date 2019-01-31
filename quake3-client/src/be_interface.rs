use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __clock_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/clock_t.h"]
pub mod clock_t_h {
    pub type clock_t = __clock_t;
    use super::types_h::{__clock_t};
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
    pub type cplane_t = cplane_s;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas.h"]
pub mod be_aas_h {
    // client movement prediction stop events, stop as soon as:
    // the ground is hit
    // there's no ground
    // water is entered
    // slime is entered
    // lava is entered
    // the ground is hit with damage
    // there's a gap
    // touching a jump pad area
    // touching teleporter
    // the given stoparea is entered
    // a ground face in the area is hit
    // hit the specified bounding box
    // touching a cluster portal
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_clientmove_s {
        pub endpos: vec3_t,
        pub endarea: libc::c_int,
        pub velocity: vec3_t,
        pub trace: aas_trace_t,
        pub presencetype: libc::c_int,
        pub stopevent: libc::c_int,
        pub endcontents: libc::c_int,
        pub time: libc::c_float,
        pub frames: libc::c_int,
    }
    pub type aas_trace_t = aas_trace_s;
    //a trace is returned when a box is swept through the AAS world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_trace_s {
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub ent: libc::c_int,
        pub lastarea: libc::c_int,
        pub area: libc::c_int,
        pub planenum: libc::c_int,
    }
    /* Defined in botlib.h

//bsp_trace_t hit surface
typedef struct bsp_surface_s
{
	char name[16];
	int flags;
	int value;
} bsp_surface_t;

//a trace is returned when a box is swept through the BSP world
typedef struct bsp_trace_s
{
	qboolean		allsolid;	// if true, plane is not valid
	qboolean		startsolid;	// if true, the initial point was in a solid area
	float			fraction;	// time completed, 1.0 = didn't hit anything
	vec3_t			endpos;		// final position
	cplane_t		plane;		// surface normal at impact
	float			exp_dist;	// expanded plane distance
	int				sidenum;	// number of the brush side hit
	bsp_surface_t	surface;	// hit surface
	int				contents;	// contents on other side of surface hit
	int				ent;		// number of entity hit
} bsp_trace_t;
//
*/
    //entity info
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_entityinfo_s {
        pub valid: libc::c_int,
        pub type_0: libc::c_int,
        pub flags: libc::c_int,
        pub ltime: libc::c_float,
        pub update_time: libc::c_float,
        pub number: libc::c_int,
        pub origin: vec3_t,
        pub angles: vec3_t,
        pub old_origin: vec3_t,
        pub lastvisorigin: vec3_t,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub groundent: libc::c_int,
        pub solid: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub frame: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
    }
    // area info
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_areainfo_s {
        pub contents: libc::c_int,
        pub flags: libc::c_int,
        pub presencetype: libc::c_int,
        pub cluster: libc::c_int,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub center: vec3_t,
    }
    // alternate route goals
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_altroutegoal_s {
        pub origin: vec3_t,
        pub areanum: libc::c_int,
        pub starttraveltime: libc::c_ushort,
        pub goaltraveltime: libc::c_ushort,
        pub extratraveltime: libc::c_ushort,
    }
    // route prediction stop events
    //no route to goal
    //stop as soon as on of the given travel types is used
    //stop when entering the given contents
    //stop when entering the given area
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_predictroute_s {
        pub endpos: vec3_t,
        pub endarea: libc::c_int,
        pub stopevent: libc::c_int,
        pub endcontents: libc::c_int,
        pub endtravelflags: libc::c_int,
        pub numareas: libc::c_int,
        pub time: libc::c_int,
    }
    pub type aas_altroutegoal_t = aas_altroutegoal_s;
    pub type aas_areainfo_t = aas_areainfo_s;
    pub type aas_entityinfo_t = aas_entityinfo_s;
    use super::q_shared_h::{vec3_t, qboolean};
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_chat.h"]
pub mod be_ai_chat_h {
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
 * name:		be_ai_chat.h
 *
 * desc:		char AI
 *
 * $Archive: /source/code/botlib/be_ai_chat.h $
 *
 *****************************************************************************/
    //a console message
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_consolemessage_s {
        pub handle: libc::c_int,
        pub time: libc::c_float,
        pub type_0: libc::c_int,
        pub message: [libc::c_char; 256],
        pub prev: *mut bot_consolemessage_s,
        pub next: *mut bot_consolemessage_s,
    }
    //returned to AI when a match is found
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_match_s {
        pub string: [libc::c_char; 256],
        pub type_0: libc::c_int,
        pub subtype: libc::c_int,
        pub variables: [bot_matchvariable_t; 8],
    }
    pub type bot_matchvariable_t = bot_matchvariable_s;
    //match variable
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_matchvariable_s {
        pub offset: libc::c_char,
        pub length: libc::c_int,
    }
    pub type bot_match_t = bot_match_s;
    pub type bot_consolemessage_t = bot_consolemessage_s;
    use super::{libc};
    extern "C" {
        //shutdown the chat AI
        #[no_mangle]
        pub fn BotShutdownChatAI();
        //setup the chat AI
        #[no_mangle]
        pub fn BotSetupChatAI() -> libc::c_int;
        //store the bot name in the chat state
        #[no_mangle]
        pub fn BotSetChatName(chatstate: libc::c_int, name: *mut libc::c_char,
                              client: libc::c_int);
        //store the gender of the bot in the chat state
        #[no_mangle]
        pub fn BotSetChatGender(chatstate: libc::c_int, gender: libc::c_int);
        //loads a chat file for the chat state
        #[no_mangle]
        pub fn BotLoadChatFile(chatstate: libc::c_int,
                               chatfile: *mut libc::c_char,
                               chatname: *mut libc::c_char) -> libc::c_int;
        //replace all the context related synonyms in the string
        #[no_mangle]
        pub fn BotReplaceSynonyms(string: *mut libc::c_char,
                                  context: libc::c_ulong);
        //unify all the white spaces in the string
        #[no_mangle]
        pub fn UnifyWhiteSpaces(string: *mut libc::c_char);
        //returns a variable from a match
        #[no_mangle]
        pub fn BotMatchVariable(match_0: *mut bot_match_t,
                                variable: libc::c_int, buf: *mut libc::c_char,
                                size: libc::c_int);
        //finds a match for the given string using the match templates
        #[no_mangle]
        pub fn BotFindMatch(str: *mut libc::c_char, match_0: *mut bot_match_t,
                            context: libc::c_ulong) -> libc::c_int;
        //checks if the first string contains the second one, returns index into first string or -1 if not found
        #[no_mangle]
        pub fn StringContains(str1: *mut libc::c_char,
                              str2: *mut libc::c_char,
                              casesensitive: libc::c_int) -> libc::c_int;
        //get the chat message ready to be output
        #[no_mangle]
        pub fn BotGetChatMessage(chatstate: libc::c_int,
                                 buf: *mut libc::c_char, size: libc::c_int);
        //enters the selected chat message
        #[no_mangle]
        pub fn BotEnterChat(chatstate: libc::c_int, clientto: libc::c_int,
                            sendto: libc::c_int);
        //returns the length of the currently selected chat message
        #[no_mangle]
        pub fn BotChatLength(chatstate: libc::c_int) -> libc::c_int;
        //find and select a reply for the given message
        #[no_mangle]
        pub fn BotReplyChat(chatstate: libc::c_int,
                            message: *mut libc::c_char, mcontext: libc::c_int,
                            vcontext: libc::c_int, var0: *mut libc::c_char,
                            var1: *mut libc::c_char, var2: *mut libc::c_char,
                            var3: *mut libc::c_char, var4: *mut libc::c_char,
                            var5: *mut libc::c_char, var6: *mut libc::c_char,
                            var7: *mut libc::c_char) -> libc::c_int;
        //returns the number of initial chat messages of the given type
        #[no_mangle]
        pub fn BotNumInitialChats(chatstate: libc::c_int,
                                  type_0: *mut libc::c_char) -> libc::c_int;
        //selects a chat message of the given type
        #[no_mangle]
        pub fn BotInitialChat(chatstate: libc::c_int,
                              type_0: *mut libc::c_char,
                              mcontext: libc::c_int, var0: *mut libc::c_char,
                              var1: *mut libc::c_char,
                              var2: *mut libc::c_char,
                              var3: *mut libc::c_char,
                              var4: *mut libc::c_char,
                              var5: *mut libc::c_char,
                              var6: *mut libc::c_char,
                              var7: *mut libc::c_char);
        //returns the number of console messages currently stored in the state
        #[no_mangle]
        pub fn BotNumConsoleMessages(chatstate: libc::c_int) -> libc::c_int;
        //returns the next console message from the state
        #[no_mangle]
        pub fn BotNextConsoleMessage(chatstate: libc::c_int,
                                     cm: *mut bot_consolemessage_t)
         -> libc::c_int;
        //removes the console message from the chat state
        #[no_mangle]
        pub fn BotRemoveConsoleMessage(chatstate: libc::c_int,
                                       handle: libc::c_int);
        //adds a console message to the chat state
        #[no_mangle]
        pub fn BotQueueConsoleMessage(chatstate: libc::c_int,
                                      type_0: libc::c_int,
                                      message: *mut libc::c_char);
        //frees the chatstate
        #[no_mangle]
        pub fn BotFreeChatState(handle: libc::c_int);
        //returns the handle to a newly allocated chat state
        #[no_mangle]
        pub fn BotAllocChatState() -> libc::c_int;
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
    use super::q_shared_h::{vec3_t, vec_t};
    use super::{libc};
    extern "C" {
        //initializes the items in the level
        #[no_mangle]
        pub fn BotInitLevelItems();
        //shut down the goal AI
        #[no_mangle]
        pub fn BotShutdownGoalAI();
        //setup the goal AI
        #[no_mangle]
        pub fn BotSetupGoalAI() -> libc::c_int;
        //free the given goal state
        #[no_mangle]
        pub fn BotFreeGoalState(handle: libc::c_int);
        //returns the handle of a newly allocated goal state
        #[no_mangle]
        pub fn BotAllocGoalState(client: libc::c_int) -> libc::c_int;
        //mutate the goal fuzzy logic
        #[no_mangle]
        pub fn BotMutateGoalFuzzyLogic(goalstate: libc::c_int,
                                       range: libc::c_float);
        //save the goal fuzzy logic to disk
        #[no_mangle]
        pub fn BotSaveGoalFuzzyLogic(goalstate: libc::c_int,
                                     filename: *mut libc::c_char);
        //interbreed the goal fuzzy logic
        #[no_mangle]
        pub fn BotInterbreedGoalFuzzyLogic(parent1: libc::c_int,
                                           parent2: libc::c_int,
                                           child: libc::c_int);
        //frees the item weights of the bot
        #[no_mangle]
        pub fn BotFreeItemWeights(goalstate: libc::c_int);
        //loads item weights for the bot
        #[no_mangle]
        pub fn BotLoadItemWeights(goalstate: libc::c_int,
                                  filename: *mut libc::c_char) -> libc::c_int;
        //regularly update dynamic entity items (dropped weapons, flags etc.)
        #[no_mangle]
        pub fn BotUpdateEntityItems();
        //set the avoid goal time
        #[no_mangle]
        pub fn BotSetAvoidGoalTime(goalstate: libc::c_int,
                                   number: libc::c_int,
                                   avoidtime: libc::c_float);
        //returns the avoid goal time
        #[no_mangle]
        pub fn BotAvoidGoalTime(goalstate: libc::c_int, number: libc::c_int)
         -> libc::c_float;
        //get the map location with the given name
        #[no_mangle]
        pub fn BotGetMapLocationGoal(name: *mut libc::c_char,
                                     goal: *mut bot_goal_t) -> libc::c_int;
        //get the next camp spot in the map
        #[no_mangle]
        pub fn BotGetNextCampSpotGoal(num: libc::c_int, goal: *mut bot_goal_t)
         -> libc::c_int;
        //search for a goal for the given classname, the index can be used
//as a start point for the search when multiple goals are available with that same classname
        #[no_mangle]
        pub fn BotGetLevelItemGoal(index: libc::c_int,
                                   classname: *mut libc::c_char,
                                   goal: *mut bot_goal_t) -> libc::c_int;
        //returns true if the goal should be visible but isn't
        #[no_mangle]
        pub fn BotItemGoalInVisButNotVisible(viewer: libc::c_int,
                                             eye: *mut vec_t,
                                             viewangles: *mut vec_t,
                                             goal: *mut bot_goal_t)
         -> libc::c_int;
        //returns true if the bot touches the goal
        #[no_mangle]
        pub fn BotTouchingGoal(origin: *mut vec_t, goal: *mut bot_goal_t)
         -> libc::c_int;
        //choose the best nearby goal item for the bot
//the item may not be further away from the current bot position than maxtime
//also the travel time from the nearby goal towards the long term goal may not
//be larger than the travel time towards the long term goal from the current bot position
        #[no_mangle]
        pub fn BotChooseNBGItem(goalstate: libc::c_int, origin: *mut vec_t,
                                inventory: *mut libc::c_int,
                                travelflags: libc::c_int,
                                ltg: *mut bot_goal_t, maxtime: libc::c_float)
         -> libc::c_int;
        //choose the best long term goal item for the bot
        #[no_mangle]
        pub fn BotChooseLTGItem(goalstate: libc::c_int, origin: *mut vec_t,
                                inventory: *mut libc::c_int,
                                travelflags: libc::c_int) -> libc::c_int;
        //get the second goal on the stack
        #[no_mangle]
        pub fn BotGetSecondGoal(goalstate: libc::c_int, goal: *mut bot_goal_t)
         -> libc::c_int;
        //get the top goal from the stack
        #[no_mangle]
        pub fn BotGetTopGoal(goalstate: libc::c_int, goal: *mut bot_goal_t)
         -> libc::c_int;
        //get the name name of the goal with the given number
        #[no_mangle]
        pub fn BotGoalName(number: libc::c_int, name: *mut libc::c_char,
                           size: libc::c_int);
        //dump the goal stack
        #[no_mangle]
        pub fn BotDumpGoalStack(goalstate: libc::c_int);
        //dump the avoid goals
        #[no_mangle]
        pub fn BotDumpAvoidGoals(goalstate: libc::c_int);
        //empty the bot's goal stack
        #[no_mangle]
        pub fn BotEmptyGoalStack(goalstate: libc::c_int);
        //pop a goal from the goal stack
        #[no_mangle]
        pub fn BotPopGoal(goalstate: libc::c_int);
        //push a goal onto the goal stack
        #[no_mangle]
        pub fn BotPushGoal(goalstate: libc::c_int, goal: *mut bot_goal_t);
        //remove the goal with the given number from the avoid goals
        #[no_mangle]
        pub fn BotRemoveFromAvoidGoals(goalstate: libc::c_int,
                                       number: libc::c_int);
        //reset avoid goals
        #[no_mangle]
        pub fn BotResetAvoidGoals(goalstate: libc::c_int);
        //reset the whole goal state, but keep the item weights
        #[no_mangle]
        pub fn BotResetGoalState(goalstate: libc::c_int);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_move.h"]
pub mod be_ai_move_h {
    //NOTE: the ideal_viewangles are only valid if MFL_MOVEMENTVIEW is set
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_moveresult_s {
        pub failure: libc::c_int,
        pub type_0: libc::c_int,
        pub blocked: libc::c_int,
        pub blockentity: libc::c_int,
        pub traveltype: libc::c_int,
        pub flags: libc::c_int,
        pub weapon: libc::c_int,
        pub movedir: vec3_t,
        pub ideal_viewangles: vec3_t,
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
 * name:		be_ai_move.h
 *
 * desc:		movement AI
 *
 * $Archive: /source/code/botlib/be_ai_move.h $
 *
 *****************************************************************************/
    //movement types
    //move flags
    //bot is performing a barrier jump
    //bot is in the ground
    //bot is swimming
    //bot is against a ladder
    //bot is waterjumping
    //bot is being teleported
    //bot is being pulled by the grapple
    //bot is using the grapple hook
    //bot has reset the grapple
    //bot should walk slowly
    // move result flags
    //bot uses view for movement
    //bot uses view for swimming
    //bot is waiting for something
    //bot has set the view in movement code
    //bot uses weapon for movement
    //bot is ontop of obstacle
    //bot is ontop of a func_bobbing
    //bot is ontop of an elevator (func_plat)
    //bot is blocked by an avoid spot
    //
    // avoid spot types
    //clear all avoid spots
    //avoid always
    //never totally block
    // restult types
    //elevator is up
    //waiting for func bobbing to arrive
    //grapple path is obstructed
    //stuck in solid area, this is bad
    //structure used to initialize the movement state
//the or_moveflags MFL_ONGROUND, MFL_TELEPORTED and MFL_WATERJUMP come from the playerstate
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_initmove_s {
        pub origin: vec3_t,
        pub velocity: vec3_t,
        pub viewoffset: vec3_t,
        pub entitynum: libc::c_int,
        pub client: libc::c_int,
        pub thinktime: libc::c_float,
        pub presencetype: libc::c_int,
        pub viewangles: vec3_t,
        pub or_moveflags: libc::c_int,
    }
    pub type bot_initmove_t = bot_initmove_s;
    pub type bot_moveresult_t = bot_moveresult_s;
    use super::{libc};
    use super::q_shared_h::{vec3_t, vec_t};
    use super::be_ai_goal_h::{bot_goal_t};
    extern "C" {
        //must be called every map change
        #[no_mangle]
        pub fn BotSetBrushModelTypes();
        //shutdown movement AI
        #[no_mangle]
        pub fn BotShutdownMoveAI();
        //setup movement AI
        #[no_mangle]
        pub fn BotSetupMoveAI() -> libc::c_int;
        //add a spot to avoid (if type == AVOID_CLEAR all spots are removed)
        #[no_mangle]
        pub fn BotAddAvoidSpot(movestate: libc::c_int, origin: *mut vec_t,
                               radius: libc::c_float, type_0: libc::c_int);
        //initialize movement state before performing any movement
        #[no_mangle]
        pub fn BotInitMoveState(handle: libc::c_int,
                                initmove: *mut bot_initmove_t);
        //frees the movestate with the given handle
        #[no_mangle]
        pub fn BotFreeMoveState(handle: libc::c_int);
        //returns the handle of a newly allocated movestate
        #[no_mangle]
        pub fn BotAllocMoveState() -> libc::c_int;
        //predict the position of a player based on movement towards a goal
        #[no_mangle]
        pub fn BotPredictVisiblePosition(origin: *mut vec_t,
                                         areanum: libc::c_int,
                                         goal: *mut bot_goal_t,
                                         travelflags: libc::c_int,
                                         target: *mut vec_t) -> libc::c_int;
        //view target based on movement
        #[no_mangle]
        pub fn BotMovementViewTarget(movestate: libc::c_int,
                                     goal: *mut bot_goal_t,
                                     travelflags: libc::c_int,
                                     lookahead: libc::c_float,
                                     target: *mut vec_t) -> libc::c_int;
        //returns a reachability area if the origin is in one
        #[no_mangle]
        pub fn BotReachabilityArea(origin: *mut vec_t, client: libc::c_int)
         -> libc::c_int;
        //resets the last avoid reachability
        #[no_mangle]
        pub fn BotResetLastAvoidReach(movestate: libc::c_int);
        //reset avoid reachability
        #[no_mangle]
        pub fn BotResetAvoidReach(movestate: libc::c_int);
        //moves the bot in the specified direction using the specified type of movement
        #[no_mangle]
        pub fn BotMoveInDirection(movestate: libc::c_int, dir: *mut vec_t,
                                  speed: libc::c_float, type_0: libc::c_int)
         -> libc::c_int;
        //moves the bot to the given goal
        #[no_mangle]
        pub fn BotMoveToGoal(result: *mut bot_moveresult_t,
                             movestate: libc::c_int, goal: *mut bot_goal_t,
                             travelflags: libc::c_int);
        //resets the whole move state
        #[no_mangle]
        pub fn BotResetMoveState(movestate: libc::c_int);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_weap.h"]
pub mod be_ai_weap_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct weaponinfo_s {
        pub valid: libc::c_int,
        pub number: libc::c_int,
        pub name: [libc::c_char; 80],
        pub model: [libc::c_char; 80],
        pub level: libc::c_int,
        pub weaponindex: libc::c_int,
        pub flags: libc::c_int,
        pub projectile: [libc::c_char; 80],
        pub numprojectiles: libc::c_int,
        pub hspread: libc::c_float,
        pub vspread: libc::c_float,
        pub speed: libc::c_float,
        pub acceleration: libc::c_float,
        pub recoil: vec3_t,
        pub offset: vec3_t,
        pub angleoffset: vec3_t,
        pub extrazvelocity: libc::c_float,
        pub ammoamount: libc::c_int,
        pub ammoindex: libc::c_int,
        pub activate: libc::c_float,
        pub reload: libc::c_float,
        pub spinup: libc::c_float,
        pub spindown: libc::c_float,
        pub proj: projectileinfo_t,
    }
    pub type projectileinfo_t = projectileinfo_s;
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
 * name:		be_ai_weap.h
 *
 * desc:		weapon AI
 *
 * $Archive: /source/code/botlib/be_ai_weap.h $
 *
 *****************************************************************************/
    //projectile flags
    //projectile damages through window
    //set when projectile returns to owner
    //weapon flags
    //set when projectile is fired with key-up event
    //damage types
    //damage on impact
    //radial damage
    //damage to all entities visible to the projectile
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct projectileinfo_s {
        pub name: [libc::c_char; 80],
        pub model: [libc::c_char; 80],
        pub flags: libc::c_int,
        pub gravity: libc::c_float,
        pub damage: libc::c_int,
        pub radius: libc::c_float,
        pub visdamage: libc::c_int,
        pub damagetype: libc::c_int,
        pub healthinc: libc::c_int,
        pub push: libc::c_float,
        pub detonation: libc::c_float,
        pub bounce: libc::c_float,
        pub bouncefric: libc::c_float,
        pub bouncestop: libc::c_float,
    }
    pub type weaponinfo_t = weaponinfo_s;
    use super::{libc};
    use super::q_shared_h::{vec3_t};
    extern "C" {
        //shut down the weapon AI
        #[no_mangle]
        pub fn BotShutdownWeaponAI();
        //setup the weapon AI
        #[no_mangle]
        pub fn BotSetupWeaponAI() -> libc::c_int;
        //resets the whole weapon state
        #[no_mangle]
        pub fn BotResetWeaponState(weaponstate: libc::c_int);
        //frees the weapon state
        #[no_mangle]
        pub fn BotFreeWeaponState(weaponstate: libc::c_int);
        //returns a handle to a newly allocated weapon state
        #[no_mangle]
        pub fn BotAllocWeaponState() -> libc::c_int;
        //loads the weapon weights
        #[no_mangle]
        pub fn BotLoadWeaponWeights(weaponstate: libc::c_int,
                                    filename: *mut libc::c_char)
         -> libc::c_int;
        //returns the information of the current weapon
        #[no_mangle]
        pub fn BotGetWeaponInfo(weaponstate: libc::c_int, weapon: libc::c_int,
                                weaponinfo: *mut weaponinfo_t);
        //returns the best weapon to fight with
        #[no_mangle]
        pub fn BotChooseBestFightWeapon(weaponstate: libc::c_int,
                                        inventory: *mut libc::c_int)
         -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/botlib.h"]
pub mod botlib_h {
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
 * name:		botlib.h
 *
 * desc:		bot AI library
 *
 * $Archive: /source/code/game/botai.h $
 *
 *****************************************************************************/
    //debug line colors
    //0xf2f2f0f0L
    //0xd0d1d2d3L
    //0xf3f3f1f1L
    //0xdcdddedfL
    //0xe0e1e2e3L
    //Print types
    //console message types
    //botlib error codes
    //no error
    //library not setup
    //invalid entity number
    //no AAS file available
    //cannot open AAS file
    //incorrect AAS file id
    //incorrect AAS file version
    //cannot read AAS file lump
    //cannot load initial chats
    //cannot load item weights
    //cannot load item config
    //cannot load weapon weights
    //cannot load weapon config
    //action flags
    //the bot input, will be converted to a usercmd_t
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_input_s {
        pub thinktime: libc::c_float,
        pub dir: vec3_t,
        pub speed: libc::c_float,
        pub viewangles: vec3_t,
        pub actionflags: libc::c_int,
        pub weapon: libc::c_int,
    }
    pub type bot_input_t = bot_input_s;
    //bsp_trace_t hit surface
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_surface_s {
        pub name: [libc::c_char; 16],
        pub flags: libc::c_int,
        pub value: libc::c_int,
    }
    pub type bsp_surface_t = bsp_surface_s;
    //remove the bsp_trace_s structure definition l8r on
//a trace is returned when a box is swept through the world
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bsp_trace_s {
        pub allsolid: qboolean,
        pub startsolid: qboolean,
        pub fraction: libc::c_float,
        pub endpos: vec3_t,
        pub plane: cplane_t,
        pub exp_dist: libc::c_float,
        pub sidenum: libc::c_int,
        pub surface: bsp_surface_t,
        pub contents: libc::c_int,
        pub ent: libc::c_int,
    }
    pub type bsp_trace_t = bsp_trace_s;
    // BSPTRACE
    //entity state
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct bot_entitystate_s {
        pub type_0: libc::c_int,
        pub flags: libc::c_int,
        pub origin: vec3_t,
        pub angles: vec3_t,
        pub old_origin: vec3_t,
        pub mins: vec3_t,
        pub maxs: vec3_t,
        pub groundent: libc::c_int,
        pub solid: libc::c_int,
        pub modelindex: libc::c_int,
        pub modelindex2: libc::c_int,
        pub frame: libc::c_int,
        pub event: libc::c_int,
        pub eventParm: libc::c_int,
        pub powerups: libc::c_int,
        pub weapon: libc::c_int,
        pub legsAnim: libc::c_int,
        pub torsoAnim: libc::c_int,
    }
    pub type bot_entitystate_t = bot_entitystate_s;
    //bot AI library exported functions
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct botlib_import_s {
        pub Print: Option<unsafe extern "C" fn(_: libc::c_int,
                                               _: *mut libc::c_char, ...)
                              -> ()>,
        pub Trace: Option<unsafe extern "C" fn(_: *mut bsp_trace_t,
                                               _: *mut vec_t, _: *mut vec_t,
                                               _: *mut vec_t, _: *mut vec_t,
                                               _: libc::c_int, _: libc::c_int)
                              -> ()>,
        pub EntityTrace: Option<unsafe extern "C" fn(_: *mut bsp_trace_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: *mut vec_t,
                                                     _: libc::c_int,
                                                     _: libc::c_int) -> ()>,
        pub PointContents: Option<unsafe extern "C" fn(_: *mut vec_t)
                                      -> libc::c_int>,
        pub inPVS: Option<unsafe extern "C" fn(_: *mut vec_t, _: *mut vec_t)
                              -> libc::c_int>,
        pub BSPEntityData: Option<unsafe extern "C" fn()
                                      -> *mut libc::c_char>,
        pub BSPModelMinsMaxsOrigin: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t,
                                                                _: *mut vec_t)
                                               -> ()>,
        pub BotClientCommand: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _:
                                                              *mut libc::c_char)
                                         -> ()>,
        pub GetMemory: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut libc::c_void>,
        pub FreeMemory: Option<unsafe extern "C" fn(_: *mut libc::c_void)
                                   -> ()>,
        pub AvailableMemory: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub HunkAlloc: Option<unsafe extern "C" fn(_: libc::c_int)
                                  -> *mut libc::c_void>,
        pub FS_FOpenFile: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *mut fileHandle_t,
                                                      _: fsMode_t)
                                     -> libc::c_int>,
        pub FS_Read: Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                                 _: libc::c_int,
                                                 _: fileHandle_t)
                                -> libc::c_int>,
        pub FS_Write: Option<unsafe extern "C" fn(_: *const libc::c_void,
                                                  _: libc::c_int,
                                                  _: fileHandle_t)
                                 -> libc::c_int>,
        pub FS_FCloseFile: Option<unsafe extern "C" fn(_: fileHandle_t)
                                      -> ()>,
        pub FS_Seek: Option<unsafe extern "C" fn(_: fileHandle_t,
                                                 _: libc::c_long,
                                                 _: libc::c_int)
                                -> libc::c_int>,
        pub DebugLineCreate: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub DebugLineDelete: Option<unsafe extern "C" fn(_: libc::c_int)
                                        -> ()>,
        pub DebugLineShow: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: libc::c_int) -> ()>,
        pub DebugPolygonCreate: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _: libc::c_int,
                                                            _: *mut vec3_t)
                                           -> libc::c_int>,
        pub DebugPolygonDelete: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
    }
    pub type botlib_import_t = botlib_import_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct aas_export_s {
        pub AAS_EntityInfo: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _:
                                                            *mut aas_entityinfo_s)
                                       -> ()>,
        pub AAS_Initialized: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub AAS_PresenceTypeBoundingBox: Option<unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         *mut vec_t,
                                                                     _:
                                                                         *mut vec_t)
                                                    -> ()>,
        pub AAS_Time: Option<unsafe extern "C" fn() -> libc::c_float>,
        pub AAS_PointAreaNum: Option<unsafe extern "C" fn(_: *mut vec_t)
                                         -> libc::c_int>,
        pub AAS_PointReachabilityAreaIndex: Option<unsafe extern "C" fn(_:
                                                                            *mut vec_t)
                                                       -> libc::c_int>,
        pub AAS_TraceAreas: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                        _: *mut vec_t,
                                                        _: *mut libc::c_int,
                                                        _: *mut vec3_t,
                                                        _: libc::c_int)
                                       -> libc::c_int>,
        pub AAS_BBoxAreas: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                       _: *mut vec_t,
                                                       _: *mut libc::c_int,
                                                       _: libc::c_int)
                                      -> libc::c_int>,
        pub AAS_AreaInfo: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut aas_areainfo_s)
                                     -> libc::c_int>,
        pub AAS_PointContents: Option<unsafe extern "C" fn(_: *mut vec_t)
                                          -> libc::c_int>,
        pub AAS_NextBSPEntity: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> libc::c_int>,
        pub AAS_ValueForBSPEpairKey: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     *mut libc::c_char,
                                                                 _:
                                                                     *mut libc::c_char,
                                                                 _:
                                                                     libc::c_int)
                                                -> libc::c_int>,
        pub AAS_VectorForBSPEpairKey: Option<unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut libc::c_char,
                                                                  _:
                                                                      *mut vec_t)
                                                 -> libc::c_int>,
        pub AAS_FloatForBSPEpairKey: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     *mut libc::c_char,
                                                                 _:
                                                                     *mut libc::c_float)
                                                -> libc::c_int>,
        pub AAS_IntForBSPEpairKey: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   *mut libc::c_int)
                                              -> libc::c_int>,
        pub AAS_AreaReachability: Option<unsafe extern "C" fn(_: libc::c_int)
                                             -> libc::c_int>,
        pub AAS_AreaTravelTimeToGoalArea: Option<unsafe extern "C" fn(_:
                                                                          libc::c_int,
                                                                      _:
                                                                          *mut vec_t,
                                                                      _:
                                                                          libc::c_int,
                                                                      _:
                                                                          libc::c_int)
                                                     -> libc::c_int>,
        pub AAS_EnableRoutingArea: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _: libc::c_int)
                                              -> libc::c_int>,
        pub AAS_PredictRoute: Option<unsafe extern "C" fn(_:
                                                              *mut aas_predictroute_s,
                                                          _: libc::c_int,
                                                          _: *mut vec_t,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int,
                                                          _: libc::c_int)
                                         -> libc::c_int>,
        pub AAS_AlternativeRouteGoals: Option<unsafe extern "C" fn(_:
                                                                       *mut vec_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut vec_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut aas_altroutegoal_s,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int>,
        pub AAS_Swimming: Option<unsafe extern "C" fn(_: *mut vec_t)
                                     -> libc::c_int>,
        pub AAS_PredictClientMovement: Option<unsafe extern "C" fn(_:
                                                                       *mut aas_clientmove_s,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut vec_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut vec_t,
                                                                   _:
                                                                       *mut vec_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_float,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       libc::c_int)
                                                  -> libc::c_int>,
    }
    pub type aas_export_t = aas_export_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ea_export_s {
        pub EA_Command: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: *mut libc::c_char)
                                   -> ()>,
        pub EA_Say: Option<unsafe extern "C" fn(_: libc::c_int,
                                                _: *mut libc::c_char) -> ()>,
        pub EA_SayTeam: Option<unsafe extern "C" fn(_: libc::c_int,
                                                    _: *mut libc::c_char)
                                   -> ()>,
        pub EA_Action: Option<unsafe extern "C" fn(_: libc::c_int,
                                                   _: libc::c_int) -> ()>,
        pub EA_Gesture: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Talk: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Attack: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Use: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Respawn: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveUp: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveDown: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveForward: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
        pub EA_MoveBack: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveLeft: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_MoveRight: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_Crouch: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_SelectWeapon: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: libc::c_int)
                                        -> ()>,
        pub EA_Jump: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub EA_DelayedJump: Option<unsafe extern "C" fn(_: libc::c_int)
                                       -> ()>,
        pub EA_Move: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *mut vec_t,
                                                 _: libc::c_float) -> ()>,
        pub EA_View: Option<unsafe extern "C" fn(_: libc::c_int,
                                                 _: *mut vec_t) -> ()>,
        pub EA_EndRegular: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: libc::c_float)
                                      -> ()>,
        pub EA_GetInput: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: libc::c_float,
                                                     _: *mut bot_input_t)
                                    -> ()>,
        pub EA_ResetInput: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
    }
    pub type ea_export_t = ea_export_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ai_export_s {
        pub BotLoadCharacter: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_char,
                                                          _: libc::c_float)
                                         -> libc::c_int>,
        pub BotFreeCharacter: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub Characteristic_Float: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _: libc::c_int)
                                             -> libc::c_float>,
        pub Characteristic_BFloat: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _: libc::c_int,
                                                               _:
                                                                   libc::c_float,
                                                               _:
                                                                   libc::c_float)
                                              -> libc::c_float>,
        pub Characteristic_Integer: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int)
                                               -> libc::c_int>,
        pub Characteristic_BInteger: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int)
                                                -> libc::c_int>,
        pub Characteristic_String: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _: libc::c_int,
                                                               _:
                                                                   *mut libc::c_char,
                                                               _: libc::c_int)
                                              -> ()>,
        pub BotAllocChatState: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotFreeChatState: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub BotQueueConsoleMessage: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut libc::c_char)
                                               -> ()>,
        pub BotRemoveConsoleMessage: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int)
                                                -> ()>,
        pub BotNextConsoleMessage: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _:
                                                                   *mut bot_consolemessage_s)
                                              -> libc::c_int>,
        pub BotNumConsoleMessages: Option<unsafe extern "C" fn(_: libc::c_int)
                                              -> libc::c_int>,
        pub BotInitialChat: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut libc::c_char,
                                                        _: libc::c_int,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: *mut libc::c_char)
                                       -> ()>,
        pub BotNumInitialChats: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut libc::c_char)
                                           -> libc::c_int>,
        pub BotReplyChat: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: *mut libc::c_char,
                                                      _: libc::c_int,
                                                      _: libc::c_int,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: *mut libc::c_char)
                                     -> libc::c_int>,
        pub BotChatLength: Option<unsafe extern "C" fn(_: libc::c_int)
                                      -> libc::c_int>,
        pub BotEnterChat: Option<unsafe extern "C" fn(_: libc::c_int,
                                                      _: libc::c_int,
                                                      _: libc::c_int) -> ()>,
        pub BotGetChatMessage: Option<unsafe extern "C" fn(_: libc::c_int,
                                                           _:
                                                               *mut libc::c_char,
                                                           _: libc::c_int)
                                          -> ()>,
        pub StringContains: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                        _: *mut libc::c_char,
                                                        _: libc::c_int)
                                       -> libc::c_int>,
        pub BotFindMatch: Option<unsafe extern "C" fn(_: *mut libc::c_char,
                                                      _: *mut bot_match_s,
                                                      _: libc::c_ulong)
                                     -> libc::c_int>,
        pub BotMatchVariable: Option<unsafe extern "C" fn(_: *mut bot_match_s,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut libc::c_char,
                                                          _: libc::c_int)
                                         -> ()>,
        pub UnifyWhiteSpaces: Option<unsafe extern "C" fn(_:
                                                              *mut libc::c_char)
                                         -> ()>,
        pub BotReplaceSynonyms: Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_char,
                                                            _: libc::c_ulong)
                                           -> ()>,
        pub BotLoadChatFile: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: *mut libc::c_char,
                                                         _: *mut libc::c_char)
                                        -> libc::c_int>,
        pub BotSetChatGender: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int)
                                         -> ()>,
        pub BotSetChatName: Option<unsafe extern "C" fn(_: libc::c_int,
                                                        _: *mut libc::c_char,
                                                        _: libc::c_int)
                                       -> ()>,
        pub BotResetGoalState: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> ()>,
        pub BotResetAvoidGoals: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
        pub BotRemoveFromAvoidGoals: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_int)
                                                -> ()>,
        pub BotPushGoal: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut bot_goal_s)
                                    -> ()>,
        pub BotPopGoal: Option<unsafe extern "C" fn(_: libc::c_int) -> ()>,
        pub BotEmptyGoalStack: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> ()>,
        pub BotDumpAvoidGoals: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> ()>,
        pub BotDumpGoalStack: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub BotGoalName: Option<unsafe extern "C" fn(_: libc::c_int,
                                                     _: *mut libc::c_char,
                                                     _: libc::c_int) -> ()>,
        pub BotGetTopGoal: Option<unsafe extern "C" fn(_: libc::c_int,
                                                       _: *mut bot_goal_s)
                                      -> libc::c_int>,
        pub BotGetSecondGoal: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: *mut bot_goal_s)
                                         -> libc::c_int>,
        pub BotChooseLTGItem: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: *mut vec_t,
                                                          _: *mut libc::c_int,
                                                          _: libc::c_int)
                                         -> libc::c_int>,
        pub BotChooseNBGItem: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: *mut vec_t,
                                                          _: *mut libc::c_int,
                                                          _: libc::c_int,
                                                          _: *mut bot_goal_s,
                                                          _: libc::c_float)
                                         -> libc::c_int>,
        pub BotTouchingGoal: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                         _: *mut bot_goal_s)
                                        -> libc::c_int>,
        pub BotItemGoalInVisButNotVisible: Option<unsafe extern "C" fn(_:
                                                                           libc::c_int,
                                                                       _:
                                                                           *mut vec_t,
                                                                       _:
                                                                           *mut vec_t,
                                                                       _:
                                                                           *mut bot_goal_s)
                                                      -> libc::c_int>,
        pub BotGetLevelItemGoal: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _:
                                                                 *mut libc::c_char,
                                                             _:
                                                                 *mut bot_goal_s)
                                            -> libc::c_int>,
        pub BotGetNextCampSpotGoal: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int,
                                                                _:
                                                                    *mut bot_goal_s)
                                               -> libc::c_int>,
        pub BotGetMapLocationGoal: Option<unsafe extern "C" fn(_:
                                                                   *mut libc::c_char,
                                                               _:
                                                                   *mut bot_goal_s)
                                              -> libc::c_int>,
        pub BotAvoidGoalTime: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int)
                                         -> libc::c_float>,
        pub BotSetAvoidGoalTime: Option<unsafe extern "C" fn(_: libc::c_int,
                                                             _: libc::c_int,
                                                             _: libc::c_float)
                                            -> ()>,
        pub BotInitLevelItems: Option<unsafe extern "C" fn() -> ()>,
        pub BotUpdateEntityItems: Option<unsafe extern "C" fn() -> ()>,
        pub BotLoadItemWeights: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut libc::c_char)
                                           -> libc::c_int>,
        pub BotFreeItemWeights: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
        pub BotInterbreedGoalFuzzyLogic: Option<unsafe extern "C" fn(_:
                                                                         libc::c_int,
                                                                     _:
                                                                         libc::c_int,
                                                                     _:
                                                                         libc::c_int)
                                                    -> ()>,
        pub BotSaveGoalFuzzyLogic: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _:
                                                                   *mut libc::c_char)
                                              -> ()>,
        pub BotMutateGoalFuzzyLogic: Option<unsafe extern "C" fn(_:
                                                                     libc::c_int,
                                                                 _:
                                                                     libc::c_float)
                                                -> ()>,
        pub BotAllocGoalState: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> libc::c_int>,
        pub BotFreeGoalState: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub BotResetMoveState: Option<unsafe extern "C" fn(_: libc::c_int)
                                          -> ()>,
        pub BotMoveToGoal: Option<unsafe extern "C" fn(_:
                                                           *mut bot_moveresult_s,
                                                       _: libc::c_int,
                                                       _: *mut bot_goal_s,
                                                       _: libc::c_int) -> ()>,
        pub BotMoveInDirection: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _: *mut vec_t,
                                                            _: libc::c_float,
                                                            _: libc::c_int)
                                           -> libc::c_int>,
        pub BotResetAvoidReach: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
        pub BotResetLastAvoidReach: Option<unsafe extern "C" fn(_:
                                                                    libc::c_int)
                                               -> ()>,
        pub BotReachabilityArea: Option<unsafe extern "C" fn(_: *mut vec_t,
                                                             _: libc::c_int)
                                            -> libc::c_int>,
        pub BotMovementViewTarget: Option<unsafe extern "C" fn(_: libc::c_int,
                                                               _:
                                                                   *mut bot_goal_s,
                                                               _: libc::c_int,
                                                               _:
                                                                   libc::c_float,
                                                               _: *mut vec_t)
                                              -> libc::c_int>,
        pub BotPredictVisiblePosition: Option<unsafe extern "C" fn(_:
                                                                       *mut vec_t,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut bot_goal_s,
                                                                   _:
                                                                       libc::c_int,
                                                                   _:
                                                                       *mut vec_t)
                                                  -> libc::c_int>,
        pub BotAllocMoveState: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotFreeMoveState: Option<unsafe extern "C" fn(_: libc::c_int)
                                         -> ()>,
        pub BotInitMoveState: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _:
                                                              *mut bot_initmove_s)
                                         -> ()>,
        pub BotAddAvoidSpot: Option<unsafe extern "C" fn(_: libc::c_int,
                                                         _: *mut vec_t,
                                                         _: libc::c_float,
                                                         _: libc::c_int)
                                        -> ()>,
        pub BotChooseBestFightWeapon: Option<unsafe extern "C" fn(_:
                                                                      libc::c_int,
                                                                  _:
                                                                      *mut libc::c_int)
                                                 -> libc::c_int>,
        pub BotGetWeaponInfo: Option<unsafe extern "C" fn(_: libc::c_int,
                                                          _: libc::c_int,
                                                          _:
                                                              *mut weaponinfo_s)
                                         -> ()>,
        pub BotLoadWeaponWeights: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_char)
                                             -> libc::c_int>,
        pub BotAllocWeaponState: Option<unsafe extern "C" fn()
                                            -> libc::c_int>,
        pub BotFreeWeaponState: Option<unsafe extern "C" fn(_: libc::c_int)
                                           -> ()>,
        pub BotResetWeaponState: Option<unsafe extern "C" fn(_: libc::c_int)
                                            -> ()>,
        pub GeneticParentsAndChildSelection: Option<unsafe extern "C" fn(_:
                                                                             libc::c_int,
                                                                         _:
                                                                             *mut libc::c_float,
                                                                         _:
                                                                             *mut libc::c_int,
                                                                         _:
                                                                             *mut libc::c_int,
                                                                         _:
                                                                             *mut libc::c_int)
                                                        -> libc::c_int>,
    }
    pub type ai_export_t = ai_export_s;
    //bot AI library imported functions
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct botlib_export_s {
        pub aas: aas_export_t,
        pub ea: ea_export_t,
        pub ai: ai_export_t,
        pub BotLibSetup: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotLibShutdown: Option<unsafe extern "C" fn() -> libc::c_int>,
        pub BotLibVarSet: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *const libc::c_char)
                                     -> libc::c_int>,
        pub BotLibVarGet: Option<unsafe extern "C" fn(_: *const libc::c_char,
                                                      _: *mut libc::c_char,
                                                      _: libc::c_int)
                                     -> libc::c_int>,
        pub PC_AddGlobalDefine: Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_char)
                                           -> libc::c_int>,
        pub PC_LoadSourceHandle: Option<unsafe extern "C" fn(_:
                                                                 *const libc::c_char)
                                            -> libc::c_int>,
        pub PC_FreeSourceHandle: Option<unsafe extern "C" fn(_: libc::c_int)
                                            -> libc::c_int>,
        pub PC_ReadTokenHandle: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut pc_token_t)
                                           -> libc::c_int>,
        pub PC_SourceFileAndLine: Option<unsafe extern "C" fn(_: libc::c_int,
                                                              _:
                                                                  *mut libc::c_char,
                                                              _:
                                                                  *mut libc::c_int)
                                             -> libc::c_int>,
        pub BotLibStartFrame: Option<unsafe extern "C" fn(_: libc::c_float)
                                         -> libc::c_int>,
        pub BotLibLoadMap: Option<unsafe extern "C" fn(_: *const libc::c_char)
                                      -> libc::c_int>,
        pub BotLibUpdateEntity: Option<unsafe extern "C" fn(_: libc::c_int,
                                                            _:
                                                                *mut bot_entitystate_t)
                                           -> libc::c_int>,
        pub Test: Option<unsafe extern "C" fn(_: libc::c_int,
                                              _: *mut libc::c_char,
                                              _: *mut vec_t, _: *mut vec_t)
                             -> libc::c_int>,
    }
    pub type botlib_export_t = botlib_export_s;
    use super::{libc};
    use super::q_shared_h::{vec3_t, qboolean, cplane_t, vec_t, fileHandle_t,
                            fsMode_t, pc_token_t};
    use super::be_aas_h::{aas_entityinfo_s, aas_areainfo_s,
                          aas_predictroute_s, aas_altroutegoal_s,
                          aas_clientmove_s};
    use super::be_ai_chat_h::{bot_consolemessage_s, bot_match_s};
    use super::be_ai_goal_h::{bot_goal_s};
    use super::be_ai_move_h::{bot_moveresult_s, bot_initmove_s};
    use super::be_ai_weap_h::{weaponinfo_s};
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_interface.h"]
pub mod be_interface_h {
    pub type botlib_globals_t = botlib_globals_s;
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
    /* ****************************************************************************
 * name:		be_interface.h
 *
 * desc:		botlib interface
 *
 * $Archive: /source/code/botlib/be_interface.h $
 *
 *****************************************************************************/
    //#define DEBUG			//debug code
    //randomize bot behaviour
    //FIXME: get rid of this global structure
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct botlib_globals_s {
        pub botlibsetup: libc::c_int,
        pub maxentities: libc::c_int,
        pub maxclients: libc::c_int,
        pub time: libc::c_float,
    }
    use super::{libc};
    use super::botlib_h::{botlib_import_t};
}
#[header_src = "/usr/include/assert.h"]
pub mod assert_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __assert_fail(__assertion: *const libc::c_char,
                             __file: *const libc::c_char,
                             __line: libc::c_uint,
                             __function: *const libc::c_char) -> !;
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strncpy(_: *mut libc::c_char, _: *const libc::c_char,
                       _: libc::c_ulong) -> *mut libc::c_char;
    }
}
#[header_src = "/usr/include/time.h"]
pub mod time_h {
    use super::clock_t_h::{clock_t};
    extern "C" {
        #[no_mangle]
        pub fn clock() -> clock_t;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_log.h"]
pub mod l_log_h {
    use super::{libc};
    extern "C" {
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
        /* ****************************************************************************
 * name:		l_log.h
 *
 * desc:		log file
 *
 * $Archive: /source/code/botlib/l_log.h $
 *
 *****************************************************************************/
        //open a log file
        #[no_mangle]
        pub fn Log_Open(filename: *mut libc::c_char);
        //close log file if present
        #[no_mangle]
        pub fn Log_Shutdown();
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_libvar.h"]
pub mod l_libvar_h {
    use super::{libc};
    extern "C" {
        //removes all library variables
        #[no_mangle]
        pub fn LibVarDeAllocAll();
        //gets the string of the library variable with the given name
        #[no_mangle]
        pub fn LibVarGetString(var_name: *const libc::c_char)
         -> *mut libc::c_char;
        //gets the value of the library variable with the given name
        #[no_mangle]
        pub fn LibVarGetValue(var_name: *const libc::c_char) -> libc::c_float;
        //creates the library variable if not existing already and returns the value
        #[no_mangle]
        pub fn LibVarValue(var_name: *const libc::c_char,
                           value: *const libc::c_char) -> libc::c_float;
        //sets the library variable
        #[no_mangle]
        pub fn LibVarSet(var_name: *const libc::c_char,
                         value: *const libc::c_char);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_precomp.h"]
pub mod l_precomp_h {
    use super::{libc};
    use super::q_shared_h::{pc_token_t};
    extern "C" {
        //add a globals define that will be added to all opened sources
        #[no_mangle]
        pub fn PC_AddGlobalDefine(string: *mut libc::c_char) -> libc::c_int;
        //remove all globals defines
        #[no_mangle]
        pub fn PC_RemoveAllGlobalDefines();
        //BSPC
        //
        #[no_mangle]
        pub fn PC_LoadSourceHandle(filename: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn PC_FreeSourceHandle(handle: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn PC_ReadTokenHandle(handle: libc::c_int,
                                  pc_token: *mut pc_token_t) -> libc::c_int;
        #[no_mangle]
        pub fn PC_SourceFileAndLine(handle: libc::c_int,
                                    filename: *mut libc::c_char,
                                    line: *mut libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn PC_CheckOpenSourceHandles();
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_interface.c"]
pub mod be_interface_c {
    use super::botlib_h::{botlib_export_t, bot_entitystate_t};
    use super::{libc};
    use super::q_shared_h::{vec_t, qboolean};
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_entity.h"]
pub mod be_aas_entity_h {
    use super::{libc};
    use super::botlib_h::{bot_entitystate_t};
    use super::be_aas_h::{aas_entityinfo_t};
    extern "C" {
        //updates an entity
        #[no_mangle]
        pub fn AAS_UpdateEntity(ent: libc::c_int,
                                state: *mut bot_entitystate_t) -> libc::c_int;
        //returns the info of the given entity
        #[no_mangle]
        pub fn AAS_EntityInfo(entnum: libc::c_int,
                              info: *mut aas_entityinfo_t);
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_main.h"]
pub mod be_aas_main_h {
    use super::{libc};
    extern "C" {
        //start a new map
        #[no_mangle]
        pub fn AAS_LoadMap(mapname: *const libc::c_char) -> libc::c_int;
        //start a new time frame
        #[no_mangle]
        pub fn AAS_StartFrame(time: libc::c_float) -> libc::c_int;
        //shutdown AAS
        #[no_mangle]
        pub fn AAS_Shutdown();
        //setup AAS with the given number of entities and clients
        #[no_mangle]
        pub fn AAS_Setup() -> libc::c_int;
        //returns the current time
        #[no_mangle]
        pub fn AAS_Time() -> libc::c_float;
        //AASINTERN
        //returns true if AAS is initialized
        #[no_mangle]
        pub fn AAS_Initialized() -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ea.h"]
pub mod be_ea_h {
    use super::{libc};
    use super::botlib_h::{bot_input_t};
    use super::q_shared_h::{vec_t};
    extern "C" {
        #[no_mangle]
        pub fn EA_Shutdown();
        //setup and shutdown routines
        #[no_mangle]
        pub fn EA_Setup() -> libc::c_int;
        #[no_mangle]
        pub fn EA_ResetInput(client: libc::c_int);
        //send regular input to the server
        #[no_mangle]
        pub fn EA_EndRegular(client: libc::c_int, thinktime: libc::c_float);
        #[no_mangle]
        pub fn EA_GetInput(client: libc::c_int, thinktime: libc::c_float,
                           input: *mut bot_input_t);
        #[no_mangle]
        pub fn EA_View(client: libc::c_int, viewangles: *mut vec_t);
        #[no_mangle]
        pub fn EA_Move(client: libc::c_int, dir: *mut vec_t,
                       speed: libc::c_float);
        #[no_mangle]
        pub fn EA_DelayedJump(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Jump(client: libc::c_int);
        //regular elementary actions
        #[no_mangle]
        pub fn EA_SelectWeapon(client: libc::c_int, weapon: libc::c_int);
        #[no_mangle]
        pub fn EA_MoveRight(client: libc::c_int);
        #[no_mangle]
        pub fn EA_MoveLeft(client: libc::c_int);
        #[no_mangle]
        pub fn EA_MoveBack(client: libc::c_int);
        #[no_mangle]
        pub fn EA_MoveForward(client: libc::c_int);
        #[no_mangle]
        pub fn EA_MoveDown(client: libc::c_int);
        #[no_mangle]
        pub fn EA_MoveUp(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Crouch(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Respawn(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Use(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Attack(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Talk(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Gesture(client: libc::c_int);
        #[no_mangle]
        pub fn EA_Action(client: libc::c_int, action: libc::c_int);
        #[no_mangle]
        pub fn EA_SayTeam(client: libc::c_int, str: *mut libc::c_char);
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
 * name:		be_ea.h
 *
 * desc:		elementary actions
 *
 * $Archive: /source/code/botlib/be_ea.h $
 *
 *****************************************************************************/
        //ClientCommand elementary actions
        #[no_mangle]
        pub fn EA_Say(client: libc::c_int, str: *mut libc::c_char);
        #[no_mangle]
        pub fn EA_Command(client: libc::c_int, command: *mut libc::c_char);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_char.h"]
pub mod be_ai_char_h {
    use super::{libc};
    extern "C" {
        //free cached bot characters
        #[no_mangle]
        pub fn BotShutdownCharacters();
        //returns a string characteristic
        #[no_mangle]
        pub fn Characteristic_String(character: libc::c_int,
                                     index: libc::c_int,
                                     buf: *mut libc::c_char,
                                     size: libc::c_int);
        //returns a bounded integer characteristic
        #[no_mangle]
        pub fn Characteristic_BInteger(character: libc::c_int,
                                       index: libc::c_int, min: libc::c_int,
                                       max: libc::c_int) -> libc::c_int;
        //returns an integer characteristic
        #[no_mangle]
        pub fn Characteristic_Integer(character: libc::c_int,
                                      index: libc::c_int) -> libc::c_int;
        //returns a bounded float characteristic
        #[no_mangle]
        pub fn Characteristic_BFloat(character: libc::c_int,
                                     index: libc::c_int, min: libc::c_float,
                                     max: libc::c_float) -> libc::c_float;
        //returns a float characteristic
        #[no_mangle]
        pub fn Characteristic_Float(character: libc::c_int,
                                    index: libc::c_int) -> libc::c_float;
        //frees a bot character
        #[no_mangle]
        pub fn BotFreeCharacter(character: libc::c_int);
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
 * name:		be_ai_char.h
 *
 * desc:		bot characters
 *
 * $Archive: /source/code/botlib/be_ai_char.h $
 *
 *****************************************************************************/
        //loads a bot character from a file
        #[no_mangle]
        pub fn BotLoadCharacter(charfile: *mut libc::c_char,
                                skill: libc::c_float) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_weight.h"]
pub mod be_ai_weight_h {
    extern "C" {
        //frees cached weight configurations
        #[no_mangle]
        pub fn BotShutdownWeights();
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_ai_gen.h"]
pub mod be_ai_gen_h {
    use super::{libc};
    extern "C" {
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
 * name:		be_ai_gen.h
 *
 * desc:		genetic selection
 *
 * $Archive: /source/code/botlib/be_ai_gen.h $
 *
 *****************************************************************************/
        #[no_mangle]
        pub fn GeneticParentsAndChildSelection(numranks: libc::c_int,
                                               ranks: *mut libc::c_float,
                                               parent1: *mut libc::c_int,
                                               parent2: *mut libc::c_int,
                                               child: *mut libc::c_int)
         -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_move.h"]
pub mod be_aas_move_h {
    use super::{libc};
    use super::be_aas_h::{aas_clientmove_s};
    use super::q_shared_h::{vec_t};
    extern "C" {
        //AASINTERN
        //movement prediction
        #[no_mangle]
        pub fn AAS_PredictClientMovement(move_0: *mut aas_clientmove_s,
                                         entnum: libc::c_int,
                                         origin: *mut vec_t,
                                         presencetype: libc::c_int,
                                         onground: libc::c_int,
                                         velocity: *mut vec_t,
                                         cmdmove: *mut vec_t,
                                         cmdframes: libc::c_int,
                                         maxframes: libc::c_int,
                                         frametime: libc::c_float,
                                         stopevent: libc::c_int,
                                         stopareanum: libc::c_int,
                                         visualize: libc::c_int)
         -> libc::c_int;
        //returns true if swimming at the given origin
        #[no_mangle]
        pub fn AAS_Swimming(origin: *mut vec_t) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_routealt.h"]
pub mod be_aas_routealt_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    use super::be_aas_h::{aas_altroutegoal_t};
    extern "C" {
        //AASINTERN
        #[no_mangle]
        pub fn AAS_AlternativeRouteGoals(start: *mut vec_t,
                                         startareanum: libc::c_int,
                                         goal: *mut vec_t,
                                         goalareanum: libc::c_int,
                                         travelflags: libc::c_int,
                                         altroutegoals:
                                             *mut aas_altroutegoal_t,
                                         maxaltroutegoals: libc::c_int,
                                         type_0: libc::c_int) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_route.h"]
pub mod be_aas_route_h {
    use super::{libc};
    use super::be_aas_h::{aas_predictroute_s};
    use super::q_shared_h::{vec_t};
    extern "C" {
        //predict a route up to a stop event
        #[no_mangle]
        pub fn AAS_PredictRoute(route: *mut aas_predictroute_s,
                                areanum: libc::c_int, origin: *mut vec_t,
                                goalareanum: libc::c_int,
                                travelflags: libc::c_int,
                                maxareas: libc::c_int, maxtime: libc::c_int,
                                stopevent: libc::c_int,
                                stopcontents: libc::c_int,
                                stoptfl: libc::c_int,
                                stopareanum: libc::c_int) -> libc::c_int;
        //enable or disable an area for routing
        #[no_mangle]
        pub fn AAS_EnableRoutingArea(areanum: libc::c_int,
                                     enable: libc::c_int) -> libc::c_int;
        //returns the travel time from the area to the goal area using the given travel flags
        #[no_mangle]
        pub fn AAS_AreaTravelTimeToGoalArea(areanum: libc::c_int,
                                            origin: *mut vec_t,
                                            goalareanum: libc::c_int,
                                            travelflags: libc::c_int)
         -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_reach.h"]
pub mod be_aas_reach_h {
    use super::{libc};
    extern "C" {
        //AASINTERN
        //returns true if the are has reachabilities to other areas
        #[no_mangle]
        pub fn AAS_AreaReachability(areanum: libc::c_int) -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_bsp.h"]
pub mod be_aas_bsp_h {
    use super::{libc};
    use super::q_shared_h::{vec_t};
    extern "C" {
        //get an integer for the BSP epair key
        #[no_mangle]
        pub fn AAS_IntForBSPEpairKey(ent: libc::c_int, key: *mut libc::c_char,
                                     value: *mut libc::c_int) -> libc::c_int;
        //get a float for the BSP epair key
        #[no_mangle]
        pub fn AAS_FloatForBSPEpairKey(ent: libc::c_int,
                                       key: *mut libc::c_char,
                                       value: *mut libc::c_float)
         -> libc::c_int;
        //get a vector for the BSP epair key
        #[no_mangle]
        pub fn AAS_VectorForBSPEpairKey(ent: libc::c_int,
                                        key: *mut libc::c_char, v: *mut vec_t)
         -> libc::c_int;
        //return the value of the BSP epair key
        #[no_mangle]
        pub fn AAS_ValueForBSPEpairKey(ent: libc::c_int,
                                       key: *mut libc::c_char,
                                       value: *mut libc::c_char,
                                       size: libc::c_int) -> libc::c_int;
        //handle to the next bsp entity
        #[no_mangle]
        pub fn AAS_NextBSPEntity(ent: libc::c_int) -> libc::c_int;
        //returns the contents at the given point
        #[no_mangle]
        pub fn AAS_PointContents(point: *mut vec_t) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/botlib/be_aas_sample.h"]
pub mod be_aas_sample_h {
    use super::{libc};
    use super::be_aas_h::{aas_areainfo_t};
    use super::q_shared_h::{vec_t, vec3_t};
    extern "C" {
        //return area information
        #[no_mangle]
        pub fn AAS_AreaInfo(areanum: libc::c_int, info: *mut aas_areainfo_t)
         -> libc::c_int;
        //returns the areas the bounding box is in
        #[no_mangle]
        pub fn AAS_BBoxAreas(absmins: *mut vec_t, absmaxs: *mut vec_t,
                             areas: *mut libc::c_int, maxareas: libc::c_int)
         -> libc::c_int;
        //stores the areas the trace went through and returns the number of passed areas
        #[no_mangle]
        pub fn AAS_TraceAreas(start: *mut vec_t, end: *mut vec_t,
                              areas: *mut libc::c_int, points: *mut vec3_t,
                              maxareas: libc::c_int) -> libc::c_int;
        //
        #[no_mangle]
        pub fn AAS_PointReachabilityAreaIndex(point: *mut vec_t)
         -> libc::c_int;
        //returns the area the point is in
        #[no_mangle]
        pub fn AAS_PointAreaNum(point: *mut vec_t) -> libc::c_int;
        //AASINTERN
        //returns the mins and maxs of the bounding box for the given presence type
        #[no_mangle]
        pub fn AAS_PresenceTypeBoundingBox(presencetype: libc::c_int,
                                           mins: *mut vec_t,
                                           maxs: *mut vec_t);
    }
}
use self::types_h::{__clock_t};
use self::clock_t_h::{clock_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, cplane_s, pc_token_s, pc_token_t, fsMode_t,
                       FS_APPEND_SYNC, FS_APPEND, FS_WRITE, FS_READ,
                       cplane_t};
use self::be_aas_h::{aas_clientmove_s, aas_trace_t, aas_trace_s,
                     aas_entityinfo_s, aas_areainfo_s, aas_altroutegoal_s,
                     aas_predictroute_s, aas_altroutegoal_t, aas_areainfo_t,
                     aas_entityinfo_t};
use self::be_ai_chat_h::{bot_consolemessage_s, bot_match_s,
                         bot_matchvariable_t, bot_matchvariable_s,
                         bot_match_t, bot_consolemessage_t, BotShutdownChatAI,
                         BotSetupChatAI, BotSetChatName, BotSetChatGender,
                         BotLoadChatFile, BotReplaceSynonyms,
                         UnifyWhiteSpaces, BotMatchVariable, BotFindMatch,
                         StringContains, BotGetChatMessage, BotEnterChat,
                         BotChatLength, BotReplyChat, BotNumInitialChats,
                         BotInitialChat, BotNumConsoleMessages,
                         BotNextConsoleMessage, BotRemoveConsoleMessage,
                         BotQueueConsoleMessage, BotFreeChatState,
                         BotAllocChatState};
use self::be_ai_goal_h::{bot_goal_s, bot_goal_t, BotInitLevelItems,
                         BotShutdownGoalAI, BotSetupGoalAI, BotFreeGoalState,
                         BotAllocGoalState, BotMutateGoalFuzzyLogic,
                         BotSaveGoalFuzzyLogic, BotInterbreedGoalFuzzyLogic,
                         BotFreeItemWeights, BotLoadItemWeights,
                         BotUpdateEntityItems, BotSetAvoidGoalTime,
                         BotAvoidGoalTime, BotGetMapLocationGoal,
                         BotGetNextCampSpotGoal, BotGetLevelItemGoal,
                         BotItemGoalInVisButNotVisible, BotTouchingGoal,
                         BotChooseNBGItem, BotChooseLTGItem, BotGetSecondGoal,
                         BotGetTopGoal, BotGoalName, BotDumpGoalStack,
                         BotDumpAvoidGoals, BotEmptyGoalStack, BotPopGoal,
                         BotPushGoal, BotRemoveFromAvoidGoals,
                         BotResetAvoidGoals, BotResetGoalState};
use self::be_ai_move_h::{bot_moveresult_s, bot_initmove_s, bot_initmove_t,
                         bot_moveresult_t, BotSetBrushModelTypes,
                         BotShutdownMoveAI, BotSetupMoveAI, BotAddAvoidSpot,
                         BotInitMoveState, BotFreeMoveState,
                         BotAllocMoveState, BotPredictVisiblePosition,
                         BotMovementViewTarget, BotReachabilityArea,
                         BotResetLastAvoidReach, BotResetAvoidReach,
                         BotMoveInDirection, BotMoveToGoal,
                         BotResetMoveState};
use self::be_ai_weap_h::{weaponinfo_s, projectileinfo_t, projectileinfo_s,
                         weaponinfo_t, BotShutdownWeaponAI, BotSetupWeaponAI,
                         BotResetWeaponState, BotFreeWeaponState,
                         BotAllocWeaponState, BotLoadWeaponWeights,
                         BotGetWeaponInfo, BotChooseBestFightWeapon};
use self::botlib_h::{bot_input_s, bot_input_t, bsp_surface_s, bsp_surface_t,
                     bsp_trace_s, bsp_trace_t, bot_entitystate_s,
                     bot_entitystate_t, botlib_import_s, botlib_import_t,
                     aas_export_s, aas_export_t, ea_export_s, ea_export_t,
                     ai_export_s, ai_export_t, botlib_export_s,
                     botlib_export_t};
use self::be_interface_h::{botlib_globals_t, botlib_globals_s};
use self::assert_h::{__assert_fail};
use self::string_h::{memset, strncpy};
use self::time_h::{clock};
use self::l_log_h::{Log_Open, Log_Shutdown};
use self::l_libvar_h::{LibVarDeAllocAll, LibVarGetString, LibVarGetValue,
                       LibVarValue, LibVarSet};
use self::l_precomp_h::{PC_AddGlobalDefine, PC_RemoveAllGlobalDefines,
                        PC_LoadSourceHandle, PC_FreeSourceHandle,
                        PC_ReadTokenHandle, PC_SourceFileAndLine,
                        PC_CheckOpenSourceHandles};
use self::be_aas_entity_h::{AAS_UpdateEntity, AAS_EntityInfo};
use self::be_aas_main_h::{AAS_LoadMap, AAS_StartFrame, AAS_Shutdown,
                          AAS_Setup, AAS_Time, AAS_Initialized};
use self::be_ea_h::{EA_Shutdown, EA_Setup, EA_ResetInput, EA_EndRegular,
                    EA_GetInput, EA_View, EA_Move, EA_DelayedJump, EA_Jump,
                    EA_SelectWeapon, EA_MoveRight, EA_MoveLeft, EA_MoveBack,
                    EA_MoveForward, EA_MoveDown, EA_MoveUp, EA_Crouch,
                    EA_Respawn, EA_Use, EA_Attack, EA_Talk, EA_Gesture,
                    EA_Action, EA_SayTeam, EA_Say, EA_Command};
use self::be_ai_char_h::{BotShutdownCharacters, Characteristic_String,
                         Characteristic_BInteger, Characteristic_Integer,
                         Characteristic_BFloat, Characteristic_Float,
                         BotFreeCharacter, BotLoadCharacter};
use self::be_ai_weight_h::{BotShutdownWeights};
use self::be_ai_gen_h::{GeneticParentsAndChildSelection};
use self::be_aas_move_h::{AAS_PredictClientMovement, AAS_Swimming};
use self::be_aas_routealt_h::{AAS_AlternativeRouteGoals};
use self::be_aas_route_h::{AAS_PredictRoute, AAS_EnableRoutingArea,
                           AAS_AreaTravelTimeToGoalArea};
use self::be_aas_reach_h::{AAS_AreaReachability};
use self::be_aas_bsp_h::{AAS_IntForBSPEpairKey, AAS_FloatForBSPEpairKey,
                         AAS_VectorForBSPEpairKey, AAS_ValueForBSPEpairKey,
                         AAS_NextBSPEntity, AAS_PointContents};
use self::be_aas_sample_h::{AAS_AreaInfo, AAS_BBoxAreas, AAS_TraceAreas,
                            AAS_PointReachabilityAreaIndex, AAS_PointAreaNum,
                            AAS_PresenceTypeBoundingBox};
//linking of bot library
#[no_mangle]
pub unsafe extern "C" fn GetBotLibAPI(mut apiVersion: libc::c_int,
                                      mut import: *mut botlib_import_t)
 -> *mut botlib_export_t {
    if !import.is_null() {
    } else {
        __assert_fail(b"import\x00" as *const u8 as *const libc::c_char,
                      b"code/botlib/be_interface.c\x00" as *const u8 as
                          *const libc::c_char, 853i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"botlib_export_t *GetBotLibAPI(int, botlib_import_t *)\x00")).as_ptr());
    }
    botimport = *import;
    if botimport.Print.is_some() {
    } else {
        __assert_fail(b"botimport.Print\x00" as *const u8 as
                          *const libc::c_char,
                      b"code/botlib/be_interface.c\x00" as *const u8 as
                          *const libc::c_char, 855i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 54],
                                                &[libc::c_char; 54]>(b"botlib_export_t *GetBotLibAPI(int, botlib_import_t *)\x00")).as_ptr());
    }
    memset(&mut be_botlib_export as *mut botlib_export_t as *mut libc::c_void,
           0i32, ::std::mem::size_of::<botlib_export_t>() as libc::c_ulong);
    if apiVersion != 2i32 {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"Mismatched BOTLIB_API_VERSION: expected %i, got %i\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            2i32, apiVersion);
        return 0 as *mut botlib_export_t
    }
    Init_AAS_Export(&mut be_botlib_export.aas);
    Init_EA_Export(&mut be_botlib_export.ea);
    Init_AI_Export(&mut be_botlib_export.ai);
    be_botlib_export.BotLibSetup = Some(Export_BotLibSetup);
    be_botlib_export.BotLibShutdown = Some(Export_BotLibShutdown);
    be_botlib_export.BotLibVarSet = Some(Export_BotLibVarSet);
    be_botlib_export.BotLibVarGet = Some(Export_BotLibVarGet);
    be_botlib_export.PC_AddGlobalDefine = Some(PC_AddGlobalDefine);
    be_botlib_export.PC_LoadSourceHandle = Some(PC_LoadSourceHandle);
    be_botlib_export.PC_FreeSourceHandle = Some(PC_FreeSourceHandle);
    be_botlib_export.PC_ReadTokenHandle = Some(PC_ReadTokenHandle);
    be_botlib_export.PC_SourceFileAndLine = Some(PC_SourceFileAndLine);
    be_botlib_export.BotLibStartFrame = Some(Export_BotLibStartFrame);
    be_botlib_export.BotLibLoadMap = Some(Export_BotLibLoadMap);
    be_botlib_export.BotLibUpdateEntity = Some(Export_BotLibUpdateEntity);
    be_botlib_export.Test = Some(BotExportTest);
    return &mut be_botlib_export;
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
/* ****************************************************************************
 * name:		be_interface.c
 *
 * desc:		bot library interface
 *
 * $Archive: /MissionPack/code/botlib/be_interface.c $
 *
 *****************************************************************************/
//library globals in a structure
#[no_mangle]
pub static mut be_botlib_export: botlib_export_t =
    botlib_export_s{aas:
                        aas_export_s{AAS_EntityInfo: None,
                                     AAS_Initialized: None,
                                     AAS_PresenceTypeBoundingBox: None,
                                     AAS_Time: None,
                                     AAS_PointAreaNum: None,
                                     AAS_PointReachabilityAreaIndex: None,
                                     AAS_TraceAreas: None,
                                     AAS_BBoxAreas: None,
                                     AAS_AreaInfo: None,
                                     AAS_PointContents: None,
                                     AAS_NextBSPEntity: None,
                                     AAS_ValueForBSPEpairKey: None,
                                     AAS_VectorForBSPEpairKey: None,
                                     AAS_FloatForBSPEpairKey: None,
                                     AAS_IntForBSPEpairKey: None,
                                     AAS_AreaReachability: None,
                                     AAS_AreaTravelTimeToGoalArea: None,
                                     AAS_EnableRoutingArea: None,
                                     AAS_PredictRoute: None,
                                     AAS_AlternativeRouteGoals: None,
                                     AAS_Swimming: None,
                                     AAS_PredictClientMovement: None,},
                    ea:
                        ea_export_s{EA_Command: None,
                                    EA_Say: None,
                                    EA_SayTeam: None,
                                    EA_Action: None,
                                    EA_Gesture: None,
                                    EA_Talk: None,
                                    EA_Attack: None,
                                    EA_Use: None,
                                    EA_Respawn: None,
                                    EA_MoveUp: None,
                                    EA_MoveDown: None,
                                    EA_MoveForward: None,
                                    EA_MoveBack: None,
                                    EA_MoveLeft: None,
                                    EA_MoveRight: None,
                                    EA_Crouch: None,
                                    EA_SelectWeapon: None,
                                    EA_Jump: None,
                                    EA_DelayedJump: None,
                                    EA_Move: None,
                                    EA_View: None,
                                    EA_EndRegular: None,
                                    EA_GetInput: None,
                                    EA_ResetInput: None,},
                    ai:
                        ai_export_s{BotLoadCharacter: None,
                                    BotFreeCharacter: None,
                                    Characteristic_Float: None,
                                    Characteristic_BFloat: None,
                                    Characteristic_Integer: None,
                                    Characteristic_BInteger: None,
                                    Characteristic_String: None,
                                    BotAllocChatState: None,
                                    BotFreeChatState: None,
                                    BotQueueConsoleMessage: None,
                                    BotRemoveConsoleMessage: None,
                                    BotNextConsoleMessage: None,
                                    BotNumConsoleMessages: None,
                                    BotInitialChat: None,
                                    BotNumInitialChats: None,
                                    BotReplyChat: None,
                                    BotChatLength: None,
                                    BotEnterChat: None,
                                    BotGetChatMessage: None,
                                    StringContains: None,
                                    BotFindMatch: None,
                                    BotMatchVariable: None,
                                    UnifyWhiteSpaces: None,
                                    BotReplaceSynonyms: None,
                                    BotLoadChatFile: None,
                                    BotSetChatGender: None,
                                    BotSetChatName: None,
                                    BotResetGoalState: None,
                                    BotResetAvoidGoals: None,
                                    BotRemoveFromAvoidGoals: None,
                                    BotPushGoal: None,
                                    BotPopGoal: None,
                                    BotEmptyGoalStack: None,
                                    BotDumpAvoidGoals: None,
                                    BotDumpGoalStack: None,
                                    BotGoalName: None,
                                    BotGetTopGoal: None,
                                    BotGetSecondGoal: None,
                                    BotChooseLTGItem: None,
                                    BotChooseNBGItem: None,
                                    BotTouchingGoal: None,
                                    BotItemGoalInVisButNotVisible: None,
                                    BotGetLevelItemGoal: None,
                                    BotGetNextCampSpotGoal: None,
                                    BotGetMapLocationGoal: None,
                                    BotAvoidGoalTime: None,
                                    BotSetAvoidGoalTime: None,
                                    BotInitLevelItems: None,
                                    BotUpdateEntityItems: None,
                                    BotLoadItemWeights: None,
                                    BotFreeItemWeights: None,
                                    BotInterbreedGoalFuzzyLogic: None,
                                    BotSaveGoalFuzzyLogic: None,
                                    BotMutateGoalFuzzyLogic: None,
                                    BotAllocGoalState: None,
                                    BotFreeGoalState: None,
                                    BotResetMoveState: None,
                                    BotMoveToGoal: None,
                                    BotMoveInDirection: None,
                                    BotResetAvoidReach: None,
                                    BotResetLastAvoidReach: None,
                                    BotReachabilityArea: None,
                                    BotMovementViewTarget: None,
                                    BotPredictVisiblePosition: None,
                                    BotAllocMoveState: None,
                                    BotFreeMoveState: None,
                                    BotInitMoveState: None,
                                    BotAddAvoidSpot: None,
                                    BotChooseBestFightWeapon: None,
                                    BotGetWeaponInfo: None,
                                    BotLoadWeaponWeights: None,
                                    BotAllocWeaponState: None,
                                    BotFreeWeaponState: None,
                                    BotResetWeaponState: None,
                                    GeneticParentsAndChildSelection: None,},
                    BotLibSetup: None,
                    BotLibShutdown: None,
                    BotLibVarSet: None,
                    BotLibVarGet: None,
                    PC_AddGlobalDefine: None,
                    PC_LoadSourceHandle: None,
                    PC_FreeSourceHandle: None,
                    PC_ReadTokenHandle: None,
                    PC_SourceFileAndLine: None,
                    BotLibStartFrame: None,
                    BotLibLoadMap: None,
                    BotLibUpdateEntity: None,
                    Test: None,};
#[no_mangle]
pub unsafe extern "C" fn BotExportTest(mut parm0: libc::c_int,
                                       mut parm1: *mut libc::c_char,
                                       mut parm2: *mut vec_t,
                                       mut parm3: *mut vec_t) -> libc::c_int {
    return 0i32;
}
//end of the function Export_BotLibLoadMap
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn Export_BotLibUpdateEntity(mut ent: libc::c_int,
                                                   mut state:
                                                       *mut bot_entitystate_t)
 -> libc::c_int {
    if 0 ==
           BotLibSetup(b"BotUpdateEntity\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char) as u64 {
        return 1i32
    }
    if 0 ==
           ValidEntityNumber(ent,
                             b"BotUpdateEntity\x00" as *const u8 as
                                 *const libc::c_char as *mut libc::c_char) as
               u64 {
        return 2i32
    }
    return AAS_UpdateEntity(ent, state);
}
//end of the function BotValidateClientNumber
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ValidEntityNumber(mut num: libc::c_int,
                                           mut str: *mut libc::c_char)
 -> qboolean {
    if num < 0i32 || num > botlibglobals.maxentities {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"%s: invalid entity number %d, [0, %d]\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            str, num,
                                                            botlibglobals.maxentities);
        return qfalse
    }
    return qtrue;
}
#[no_mangle]
pub static mut botlibglobals: botlib_globals_t =
    botlib_globals_s{botlibsetup: 0,
                     maxentities: 0,
                     maxclients: 0,
                     time: 0.,};
#[no_mangle]
pub static mut botimport: botlib_import_t =
    botlib_import_s{Print: None,
                    Trace: None,
                    EntityTrace: None,
                    PointContents: None,
                    inPVS: None,
                    BSPEntityData: None,
                    BSPModelMinsMaxsOrigin: None,
                    BotClientCommand: None,
                    GetMemory: None,
                    FreeMemory: None,
                    AvailableMemory: None,
                    HunkAlloc: None,
                    FS_FOpenFile: None,
                    FS_Read: None,
                    FS_Write: None,
                    FS_FCloseFile: None,
                    FS_Seek: None,
                    DebugLineCreate: None,
                    DebugLineDelete: None,
                    DebugLineShow: None,
                    DebugPolygonCreate: None,
                    DebugPolygonDelete: None,};
//end of the function BotValidateClientNumber
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn BotLibSetup(mut str: *mut libc::c_char) -> qboolean {
    if 0 == botlibglobals.botlibsetup {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"%s: bot library used before being setup\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            str);
        return qfalse
    }
    return qtrue;
}
//end of the function Export_BotLibStartFrame
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn Export_BotLibLoadMap(mut mapname:
                                                  *const libc::c_char)
 -> libc::c_int {
    let mut errnum: libc::c_int = 0;
    if 0 ==
           BotLibSetup(b"BotLoadMap\x00" as *const u8 as *const libc::c_char
                           as *mut libc::c_char) as u64 {
        return 1i32
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"------------ Map Loading ------------\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
    errnum = AAS_LoadMap(mapname);
    if errnum != 0i32 { return errnum }
    BotInitLevelItems();
    BotSetBrushModelTypes();
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"-------------------------------------\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
    return 0i32;
}
//end of the function Export_BotLibVarGet
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn Export_BotLibStartFrame(mut time: libc::c_float)
 -> libc::c_int {
    if 0 ==
           BotLibSetup(b"BotStartFrame\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char) as u64 {
        return 1i32
    }
    return AAS_StartFrame(time);
}
//end of the function Export_BotLibVarSet
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn Export_BotLibVarGet(mut var_name:
                                                 *const libc::c_char,
                                             mut value: *mut libc::c_char,
                                             mut size: libc::c_int)
 -> libc::c_int {
    let mut varvalue: *mut libc::c_char = 0 as *mut libc::c_char;
    varvalue = LibVarGetString(var_name);
    strncpy(value, varvalue, (size - 1i32) as libc::c_ulong);
    *value.offset((size - 1i32) as isize) = '\u{0}' as i32 as libc::c_char;
    return 0i32;
}
//end of the function Export_BotLibShutdown
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn Export_BotLibVarSet(mut var_name:
                                                 *const libc::c_char,
                                             mut value: *const libc::c_char)
 -> libc::c_int {
    LibVarSet(var_name, value);
    return 0i32;
}
//end of the function Export_BotLibSetup
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn Export_BotLibShutdown() -> libc::c_int {
    if 0 ==
           BotLibSetup(b"BotLibShutdown\x00" as *const u8 as
                           *const libc::c_char as *mut libc::c_char) as u64 {
        return 1i32
    }
    BotShutdownChatAI();
    BotShutdownMoveAI();
    BotShutdownGoalAI();
    BotShutdownWeaponAI();
    BotShutdownWeights();
    BotShutdownCharacters();
    AAS_Shutdown();
    EA_Shutdown();
    LibVarDeAllocAll();
    PC_RemoveAllGlobalDefines();
    Log_Shutdown();
    botlibsetup = qfalse as libc::c_int;
    botlibglobals.botlibsetup = qfalse as libc::c_int;
    PC_CheckOpenSourceHandles();
    return 0i32;
}
//
//qtrue if the library is setup
#[no_mangle]
pub static mut botlibsetup: libc::c_int = qfalse as libc::c_int;
//end of the function BotLibSetup
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn Export_BotLibSetup() -> libc::c_int {
    let mut errnum: libc::c_int = 0;
    botDeveloper =
        LibVarGetValue(b"bot_developer\x00" as *const u8 as
                           *const libc::c_char) as libc::c_int;
    memset(&mut botlibglobals as *mut botlib_globals_t as *mut libc::c_void,
           0i32, ::std::mem::size_of::<botlib_globals_t>() as libc::c_ulong);
    if 0 != botDeveloper {
        Log_Open(b"botlib.log\x00" as *const u8 as *const libc::c_char as
                     *mut libc::c_char);
    }
    botimport.Print.expect("non-null function pointer")(1i32,
                                                        b"------- BotLib Initialization -------\n\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char);
    botlibglobals.maxclients =
        LibVarValue(b"maxclients\x00" as *const u8 as *const libc::c_char,
                    b"128\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    botlibglobals.maxentities =
        LibVarValue(b"maxentities\x00" as *const u8 as *const libc::c_char,
                    b"1024\x00" as *const u8 as *const libc::c_char) as
            libc::c_int;
    errnum = AAS_Setup();
    if errnum != 0i32 { return errnum }
    errnum = EA_Setup();
    if errnum != 0i32 { return errnum }
    errnum = BotSetupWeaponAI();
    if errnum != 0i32 { return errnum }
    errnum = BotSetupGoalAI();
    if errnum != 0i32 { return errnum }
    errnum = BotSetupChatAI();
    if errnum != 0i32 { return errnum }
    errnum = BotSetupMoveAI();
    if errnum != 0i32 { return errnum }
    botlibsetup = qtrue as libc::c_int;
    botlibglobals.botlibsetup = qtrue as libc::c_int;
    return 0i32;
}
//true if developer is on
#[no_mangle]
pub static mut botDeveloper: libc::c_int = 0;
/*
============
Init_AI_Export
============
*/
unsafe extern "C" fn Init_AI_Export(mut ai: *mut ai_export_t) {
    (*ai).BotLoadCharacter = Some(BotLoadCharacter);
    (*ai).BotFreeCharacter = Some(BotFreeCharacter);
    (*ai).Characteristic_Float = Some(Characteristic_Float);
    (*ai).Characteristic_BFloat = Some(Characteristic_BFloat);
    (*ai).Characteristic_Integer = Some(Characteristic_Integer);
    (*ai).Characteristic_BInteger = Some(Characteristic_BInteger);
    (*ai).Characteristic_String = Some(Characteristic_String);
    (*ai).BotAllocChatState = Some(BotAllocChatState);
    (*ai).BotFreeChatState = Some(BotFreeChatState);
    (*ai).BotQueueConsoleMessage = Some(BotQueueConsoleMessage);
    (*ai).BotRemoveConsoleMessage = Some(BotRemoveConsoleMessage);
    (*ai).BotNextConsoleMessage = Some(BotNextConsoleMessage);
    (*ai).BotNumConsoleMessages = Some(BotNumConsoleMessages);
    (*ai).BotInitialChat = Some(BotInitialChat);
    (*ai).BotNumInitialChats = Some(BotNumInitialChats);
    (*ai).BotReplyChat = Some(BotReplyChat);
    (*ai).BotChatLength = Some(BotChatLength);
    (*ai).BotEnterChat = Some(BotEnterChat);
    (*ai).BotGetChatMessage = Some(BotGetChatMessage);
    (*ai).StringContains = Some(StringContains);
    (*ai).BotFindMatch = Some(BotFindMatch);
    (*ai).BotMatchVariable = Some(BotMatchVariable);
    (*ai).UnifyWhiteSpaces = Some(UnifyWhiteSpaces);
    (*ai).BotReplaceSynonyms = Some(BotReplaceSynonyms);
    (*ai).BotLoadChatFile = Some(BotLoadChatFile);
    (*ai).BotSetChatGender = Some(BotSetChatGender);
    (*ai).BotSetChatName = Some(BotSetChatName);
    (*ai).BotResetGoalState = Some(BotResetGoalState);
    (*ai).BotResetAvoidGoals = Some(BotResetAvoidGoals);
    (*ai).BotRemoveFromAvoidGoals = Some(BotRemoveFromAvoidGoals);
    (*ai).BotPushGoal = Some(BotPushGoal);
    (*ai).BotPopGoal = Some(BotPopGoal);
    (*ai).BotEmptyGoalStack = Some(BotEmptyGoalStack);
    (*ai).BotDumpAvoidGoals = Some(BotDumpAvoidGoals);
    (*ai).BotDumpGoalStack = Some(BotDumpGoalStack);
    (*ai).BotGoalName = Some(BotGoalName);
    (*ai).BotGetTopGoal = Some(BotGetTopGoal);
    (*ai).BotGetSecondGoal = Some(BotGetSecondGoal);
    (*ai).BotChooseLTGItem = Some(BotChooseLTGItem);
    (*ai).BotChooseNBGItem = Some(BotChooseNBGItem);
    (*ai).BotTouchingGoal = Some(BotTouchingGoal);
    (*ai).BotItemGoalInVisButNotVisible = Some(BotItemGoalInVisButNotVisible);
    (*ai).BotGetLevelItemGoal = Some(BotGetLevelItemGoal);
    (*ai).BotGetNextCampSpotGoal = Some(BotGetNextCampSpotGoal);
    (*ai).BotGetMapLocationGoal = Some(BotGetMapLocationGoal);
    (*ai).BotAvoidGoalTime = Some(BotAvoidGoalTime);
    (*ai).BotSetAvoidGoalTime = Some(BotSetAvoidGoalTime);
    (*ai).BotInitLevelItems = Some(BotInitLevelItems);
    (*ai).BotUpdateEntityItems = Some(BotUpdateEntityItems);
    (*ai).BotLoadItemWeights = Some(BotLoadItemWeights);
    (*ai).BotFreeItemWeights = Some(BotFreeItemWeights);
    (*ai).BotInterbreedGoalFuzzyLogic = Some(BotInterbreedGoalFuzzyLogic);
    (*ai).BotSaveGoalFuzzyLogic = Some(BotSaveGoalFuzzyLogic);
    (*ai).BotMutateGoalFuzzyLogic = Some(BotMutateGoalFuzzyLogic);
    (*ai).BotAllocGoalState = Some(BotAllocGoalState);
    (*ai).BotFreeGoalState = Some(BotFreeGoalState);
    (*ai).BotResetMoveState = Some(BotResetMoveState);
    (*ai).BotMoveToGoal = Some(BotMoveToGoal);
    (*ai).BotMoveInDirection = Some(BotMoveInDirection);
    (*ai).BotResetAvoidReach = Some(BotResetAvoidReach);
    (*ai).BotResetLastAvoidReach = Some(BotResetLastAvoidReach);
    (*ai).BotReachabilityArea = Some(BotReachabilityArea);
    (*ai).BotMovementViewTarget = Some(BotMovementViewTarget);
    (*ai).BotPredictVisiblePosition = Some(BotPredictVisiblePosition);
    (*ai).BotAllocMoveState = Some(BotAllocMoveState);
    (*ai).BotFreeMoveState = Some(BotFreeMoveState);
    (*ai).BotInitMoveState = Some(BotInitMoveState);
    (*ai).BotAddAvoidSpot = Some(BotAddAvoidSpot);
    (*ai).BotChooseBestFightWeapon = Some(BotChooseBestFightWeapon);
    (*ai).BotGetWeaponInfo = Some(BotGetWeaponInfo);
    (*ai).BotLoadWeaponWeights = Some(BotLoadWeaponWeights);
    (*ai).BotAllocWeaponState = Some(BotAllocWeaponState);
    (*ai).BotFreeWeaponState = Some(BotFreeWeaponState);
    (*ai).BotResetWeaponState = Some(BotResetWeaponState);
    (*ai).GeneticParentsAndChildSelection =
        Some(GeneticParentsAndChildSelection);
}
/*
============
Init_EA_Export
============
*/
unsafe extern "C" fn Init_EA_Export(mut ea: *mut ea_export_t) {
    (*ea).EA_Command = Some(EA_Command);
    (*ea).EA_Say = Some(EA_Say);
    (*ea).EA_SayTeam = Some(EA_SayTeam);
    (*ea).EA_Action = Some(EA_Action);
    (*ea).EA_Gesture = Some(EA_Gesture);
    (*ea).EA_Talk = Some(EA_Talk);
    (*ea).EA_Attack = Some(EA_Attack);
    (*ea).EA_Use = Some(EA_Use);
    (*ea).EA_Respawn = Some(EA_Respawn);
    (*ea).EA_Crouch = Some(EA_Crouch);
    (*ea).EA_MoveUp = Some(EA_MoveUp);
    (*ea).EA_MoveDown = Some(EA_MoveDown);
    (*ea).EA_MoveForward = Some(EA_MoveForward);
    (*ea).EA_MoveBack = Some(EA_MoveBack);
    (*ea).EA_MoveLeft = Some(EA_MoveLeft);
    (*ea).EA_MoveRight = Some(EA_MoveRight);
    (*ea).EA_SelectWeapon = Some(EA_SelectWeapon);
    (*ea).EA_Jump = Some(EA_Jump);
    (*ea).EA_DelayedJump = Some(EA_DelayedJump);
    (*ea).EA_Move = Some(EA_Move);
    (*ea).EA_View = Some(EA_View);
    (*ea).EA_GetInput = Some(EA_GetInput);
    (*ea).EA_EndRegular = Some(EA_EndRegular);
    (*ea).EA_ResetInput = Some(EA_ResetInput);
}
//end of the function BotExportTest
/*
============
Init_AAS_Export
============
*/
unsafe extern "C" fn Init_AAS_Export(mut aas: *mut aas_export_t) {
    (*aas).AAS_EntityInfo = Some(AAS_EntityInfo);
    (*aas).AAS_Initialized = Some(AAS_Initialized);
    (*aas).AAS_PresenceTypeBoundingBox = Some(AAS_PresenceTypeBoundingBox);
    (*aas).AAS_Time = Some(AAS_Time);
    (*aas).AAS_PointAreaNum = Some(AAS_PointAreaNum);
    (*aas).AAS_PointReachabilityAreaIndex =
        Some(AAS_PointReachabilityAreaIndex);
    (*aas).AAS_TraceAreas = Some(AAS_TraceAreas);
    (*aas).AAS_BBoxAreas = Some(AAS_BBoxAreas);
    (*aas).AAS_AreaInfo = Some(AAS_AreaInfo);
    (*aas).AAS_PointContents = Some(AAS_PointContents);
    (*aas).AAS_NextBSPEntity = Some(AAS_NextBSPEntity);
    (*aas).AAS_ValueForBSPEpairKey = Some(AAS_ValueForBSPEpairKey);
    (*aas).AAS_VectorForBSPEpairKey = Some(AAS_VectorForBSPEpairKey);
    (*aas).AAS_FloatForBSPEpairKey = Some(AAS_FloatForBSPEpairKey);
    (*aas).AAS_IntForBSPEpairKey = Some(AAS_IntForBSPEpairKey);
    (*aas).AAS_AreaReachability = Some(AAS_AreaReachability);
    (*aas).AAS_AreaTravelTimeToGoalArea = Some(AAS_AreaTravelTimeToGoalArea);
    (*aas).AAS_EnableRoutingArea = Some(AAS_EnableRoutingArea);
    (*aas).AAS_PredictRoute = Some(AAS_PredictRoute);
    (*aas).AAS_AlternativeRouteGoals = Some(AAS_AlternativeRouteGoals);
    (*aas).AAS_Swimming = Some(AAS_Swimming);
    (*aas).AAS_PredictClientMovement = Some(AAS_PredictClientMovement);
}
//
#[no_mangle]
pub unsafe extern "C" fn Sys_MilliSeconds() -> libc::c_int {
    return (clock() * 1000i32 as libc::c_long / 1000000i32 as __clock_t) as
               libc::c_int;
}
//end of the function Sys_MilliSeconds
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]
pub unsafe extern "C" fn ValidClientNumber(mut num: libc::c_int,
                                           mut str: *mut libc::c_char)
 -> qboolean {
    if num < 0i32 || num > botlibglobals.maxclients {
        botimport.Print.expect("non-null function pointer")(3i32,
                                                            b"%s: invalid client number %d, [0, %d]\n\x00"
                                                                as *const u8
                                                                as
                                                                *const libc::c_char
                                                                as
                                                                *mut libc::c_char,
                                                            str, num,
                                                            botlibglobals.maxclients);
        return qfalse
    }
    return qtrue;
}