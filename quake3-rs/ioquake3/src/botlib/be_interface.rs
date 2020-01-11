// =============== BEGIN be_interface_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct botlib_globals_s {
    pub botlibsetup: i32,
    pub maxentities: i32,
    pub maxclients: i32,
    pub time: f32,
}

pub type botlib_globals_t = crate::src::botlib::be_interface::botlib_globals_s;
use ::libc;

pub use crate::stdlib::__clock_t;
pub use crate::stdlib::clock_t;

pub use crate::be_aas_h::aas_altroutegoal_s;
pub use crate::be_aas_h::aas_altroutegoal_t;
pub use crate::be_aas_h::aas_areainfo_s;
pub use crate::be_aas_h::aas_areainfo_t;
pub use crate::be_aas_h::aas_clientmove_s;
pub use crate::be_aas_h::aas_entityinfo_s;
pub use crate::be_aas_h::aas_entityinfo_t;
pub use crate::be_aas_h::aas_predictroute_s;
pub use crate::be_aas_h::aas_trace_s;
pub use crate::be_aas_h::aas_trace_t;
pub use crate::botlib_h::aas_export_s;
pub use crate::botlib_h::aas_export_t;
pub use crate::botlib_h::ai_export_s;
pub use crate::botlib_h::ai_export_t;
pub use crate::botlib_h::bot_entitystate_s;
pub use crate::botlib_h::bot_entitystate_t;
pub use crate::botlib_h::bot_input_s;
pub use crate::botlib_h::bot_input_t;
pub use crate::botlib_h::botlib_export_s;
pub use crate::botlib_h::botlib_export_t;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::botlib_h::ea_export_s;
pub use crate::botlib_h::ea_export_t;
pub use crate::src::botlib::be_ai_chat::bot_consolemessage_s;
pub use crate::src::botlib::be_ai_chat::bot_consolemessage_t;
pub use crate::src::botlib::be_ai_chat::bot_match_s;
pub use crate::src::botlib::be_ai_chat::bot_match_t;
pub use crate::src::botlib::be_ai_chat::bot_matchvariable_s;
pub use crate::src::botlib::be_ai_chat::bot_matchvariable_t;
pub use crate::src::botlib::be_ai_chat::BotAllocChatState;
pub use crate::src::botlib::be_ai_chat::BotChatLength;
pub use crate::src::botlib::be_ai_chat::BotEnterChat;
pub use crate::src::botlib::be_ai_chat::BotFindMatch;
pub use crate::src::botlib::be_ai_chat::BotFreeChatState;
pub use crate::src::botlib::be_ai_chat::BotGetChatMessage;
pub use crate::src::botlib::be_ai_chat::BotInitialChat;
pub use crate::src::botlib::be_ai_chat::BotLoadChatFile;
pub use crate::src::botlib::be_ai_chat::BotMatchVariable;
pub use crate::src::botlib::be_ai_chat::BotNextConsoleMessage;
pub use crate::src::botlib::be_ai_chat::BotNumConsoleMessages;
pub use crate::src::botlib::be_ai_chat::BotNumInitialChats;
pub use crate::src::botlib::be_ai_chat::BotQueueConsoleMessage;
pub use crate::src::botlib::be_ai_chat::BotRemoveConsoleMessage;
pub use crate::src::botlib::be_ai_chat::BotReplaceSynonyms;
pub use crate::src::botlib::be_ai_chat::BotReplyChat;
pub use crate::src::botlib::be_ai_chat::BotSetChatGender;
pub use crate::src::botlib::be_ai_chat::BotSetChatName;
pub use crate::src::botlib::be_ai_chat::BotSetupChatAI;
pub use crate::src::botlib::be_ai_chat::BotShutdownChatAI;
pub use crate::src::botlib::be_ai_chat::StringContains;
pub use crate::src::botlib::be_ai_chat::UnifyWhiteSpaces;
pub use crate::src::botlib::be_ai_goal::bot_goal_s;
pub use crate::src::botlib::be_ai_goal::bot_goal_t;
pub use crate::src::botlib::be_ai_goal::BotAllocGoalState;
pub use crate::src::botlib::be_ai_goal::BotAvoidGoalTime;
pub use crate::src::botlib::be_ai_goal::BotChooseLTGItem;
pub use crate::src::botlib::be_ai_goal::BotChooseNBGItem;
pub use crate::src::botlib::be_ai_goal::BotDumpAvoidGoals;
pub use crate::src::botlib::be_ai_goal::BotDumpGoalStack;
pub use crate::src::botlib::be_ai_goal::BotEmptyGoalStack;
pub use crate::src::botlib::be_ai_goal::BotFreeGoalState;
pub use crate::src::botlib::be_ai_goal::BotFreeItemWeights;
pub use crate::src::botlib::be_ai_goal::BotGetLevelItemGoal;
pub use crate::src::botlib::be_ai_goal::BotGetMapLocationGoal;
pub use crate::src::botlib::be_ai_goal::BotGetNextCampSpotGoal;
pub use crate::src::botlib::be_ai_goal::BotGetSecondGoal;
pub use crate::src::botlib::be_ai_goal::BotGetTopGoal;
pub use crate::src::botlib::be_ai_goal::BotGoalName;
pub use crate::src::botlib::be_ai_goal::BotInitLevelItems;
pub use crate::src::botlib::be_ai_goal::BotInterbreedGoalFuzzyLogic;
pub use crate::src::botlib::be_ai_goal::BotItemGoalInVisButNotVisible;
pub use crate::src::botlib::be_ai_goal::BotLoadItemWeights;
pub use crate::src::botlib::be_ai_goal::BotMutateGoalFuzzyLogic;
pub use crate::src::botlib::be_ai_goal::BotPopGoal;
pub use crate::src::botlib::be_ai_goal::BotPushGoal;
pub use crate::src::botlib::be_ai_goal::BotRemoveFromAvoidGoals;
pub use crate::src::botlib::be_ai_goal::BotResetAvoidGoals;
pub use crate::src::botlib::be_ai_goal::BotResetGoalState;
pub use crate::src::botlib::be_ai_goal::BotSaveGoalFuzzyLogic;
pub use crate::src::botlib::be_ai_goal::BotSetAvoidGoalTime;
pub use crate::src::botlib::be_ai_goal::BotSetupGoalAI;
pub use crate::src::botlib::be_ai_goal::BotShutdownGoalAI;
pub use crate::src::botlib::be_ai_goal::BotTouchingGoal;
pub use crate::src::botlib::be_ai_goal::BotUpdateEntityItems;
pub use crate::src::botlib::be_ai_move::bot_initmove_s;
pub use crate::src::botlib::be_ai_move::bot_initmove_t;
pub use crate::src::botlib::be_ai_move::bot_moveresult_s;
pub use crate::src::botlib::be_ai_move::bot_moveresult_t;
pub use crate::src::botlib::be_ai_move::BotAddAvoidSpot;
pub use crate::src::botlib::be_ai_move::BotAllocMoveState;
pub use crate::src::botlib::be_ai_move::BotFreeMoveState;
pub use crate::src::botlib::be_ai_move::BotInitMoveState;
pub use crate::src::botlib::be_ai_move::BotMoveInDirection;
pub use crate::src::botlib::be_ai_move::BotMoveToGoal;
pub use crate::src::botlib::be_ai_move::BotMovementViewTarget;
pub use crate::src::botlib::be_ai_move::BotPredictVisiblePosition;
pub use crate::src::botlib::be_ai_move::BotReachabilityArea;
pub use crate::src::botlib::be_ai_move::BotResetAvoidReach;
pub use crate::src::botlib::be_ai_move::BotResetLastAvoidReach;
pub use crate::src::botlib::be_ai_move::BotResetMoveState;
pub use crate::src::botlib::be_ai_move::BotSetBrushModelTypes;
pub use crate::src::botlib::be_ai_move::BotSetupMoveAI;
pub use crate::src::botlib::be_ai_move::BotShutdownMoveAI;
pub use crate::src::botlib::be_ai_weap::projectileinfo_s;
pub use crate::src::botlib::be_ai_weap::projectileinfo_t;
pub use crate::src::botlib::be_ai_weap::weaponinfo_s;
pub use crate::src::botlib::be_ai_weap::weaponinfo_t;
pub use crate::src::botlib::be_ai_weap::BotAllocWeaponState;
pub use crate::src::botlib::be_ai_weap::BotChooseBestFightWeapon;
pub use crate::src::botlib::be_ai_weap::BotFreeWeaponState;
pub use crate::src::botlib::be_ai_weap::BotGetWeaponInfo;
pub use crate::src::botlib::be_ai_weap::BotLoadWeaponWeights;
pub use crate::src::botlib::be_ai_weap::BotResetWeaponState;
pub use crate::src::botlib::be_ai_weap::BotSetupWeaponAI;
pub use crate::src::botlib::be_ai_weap::BotShutdownWeaponAI;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::pc_token_s;
pub use crate::src::qcommon::q_shared::pc_token_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

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

pub static mut botlibglobals: crate::src::botlib::be_interface::botlib_globals_t =
    crate::src::botlib::be_interface::botlib_globals_t {
        botlibsetup: 0,
        maxentities: 0,
        maxclients: 0,
        time: 0.,
    };
#[no_mangle]

pub static mut be_botlib_export: crate::botlib_h::botlib_export_t =
    crate::botlib_h::botlib_export_t {
        aas: crate::botlib_h::aas_export_t {
            AAS_EntityInfo: None,
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
            AAS_PredictClientMovement: None,
        },
        ea: crate::botlib_h::ea_export_t {
            EA_Command: None,
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
            EA_ResetInput: None,
        },
        ai: crate::botlib_h::ai_export_t {
            BotLoadCharacter: None,
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
            GeneticParentsAndChildSelection: None,
        },
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
        Test: None,
    };
#[no_mangle]

pub static mut botimport: crate::botlib_h::botlib_import_t = crate::botlib_h::botlib_import_t {
    Print: None,
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
    DebugPolygonDelete: None,
};
//
#[no_mangle]

pub static mut botDeveloper: i32 = 0;
//qtrue if the library is setup
#[no_mangle]

pub static mut botlibsetup: i32 = crate::src::qcommon::q_shared::qfalse as i32;
//true if developer is on
//
//===========================================================================
//
// several functions used by the exported functions
//
//===========================================================================
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Sys_MilliSeconds() -> i32 {
    return (crate::stdlib::clock() * 1000 / 1000000i32 as crate::stdlib::__clock_t) as i32;
}
//end of the function Sys_MilliSeconds
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ValidClientNumber(
    mut num: i32,
    mut str: *mut i8,
) -> crate::src::qcommon::q_shared::qboolean {
    if num < 0 || num > botlibglobals.maxclients {
        //end if
        //weird: the disabled stuff results in a crash
        botimport.Print.expect("non-null function pointer")(
            3,
            b"%s: invalid client number %d, [0, %d]\n\x00" as *const u8 as *mut i8,
            str,
            num,
            botlibglobals.maxclients,
        );
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
//end of the function BotValidateClientNumber
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ValidEntityNumber(
    mut num: i32,
    mut str: *mut i8,
) -> crate::src::qcommon::q_shared::qboolean {
    if num < 0 || num > botlibglobals.maxentities {
        botimport.Print.expect("non-null function pointer")(
            3,
            b"%s: invalid entity number %d, [0, %d]\n\x00" as *const u8 as *mut i8,
            str,
            num,
            botlibglobals.maxentities,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
//end of the function BotValidateClientNumber
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLibSetup(mut str: *mut i8) -> crate::src::qcommon::q_shared::qboolean {
    if botlibglobals.botlibsetup == 0 {
        botimport.Print.expect("non-null function pointer")(
            3,
            b"%s: bot library used before being setup\n\x00" as *const u8 as *mut i8,
            str,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse;
    }
    return crate::src::qcommon::q_shared::qtrue;
}
//end of the function BotLibSetup
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Export_BotLibSetup() -> i32 {
    let mut errnum: i32 = 0;
    botDeveloper = crate::src::botlib::l_libvar::LibVarGetValue(
        b"bot_developer\x00" as *const u8 as *const i8,
    ) as i32;
    crate::stdlib::memset(
        &mut botlibglobals as *mut crate::src::botlib::be_interface::botlib_globals_t
            as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::botlib::be_interface::botlib_globals_t>(),
    );
    //initialize byte swapping (litte endian etc.)
    //	Swap_Init();
    if botDeveloper != 0 {
        crate::src::botlib::l_log::Log_Open(b"botlib.log\x00" as *const u8 as *mut i8);
        //be_aas_main.c
    } //be_ea.c
    botimport.Print.expect("non-null function pointer")(
        1,
        b"------- BotLib Initialization -------\n\x00" as *const u8 as *mut i8,
    ); //be_ai_weap.c
    botlibglobals.maxclients = crate::src::botlib::l_libvar::LibVarValue(
        b"maxclients\x00" as *const u8 as *const i8,
        b"128\x00" as *const u8 as *const i8,
    ) as i32; //be_ai_goal.c
    botlibglobals.maxentities = crate::src::botlib::l_libvar::LibVarValue(
        b"maxentities\x00" as *const u8 as *const i8,
        b"1024\x00" as *const u8 as *const i8,
    ) as i32; //be_ai_chat.c
    errnum = crate::src::botlib::be_aas_main::AAS_Setup(); //be_ai_move.c
    if errnum != 0 {
        return errnum;
    }
    errnum = crate::src::botlib::be_ea::EA_Setup();
    if errnum != 0 {
        return errnum;
    }
    errnum = crate::src::botlib::be_ai_weap::BotSetupWeaponAI();
    if errnum != 0 {
        return errnum;
    }
    errnum = crate::src::botlib::be_ai_goal::BotSetupGoalAI();
    if errnum != 0 {
        return errnum;
    }
    errnum = crate::src::botlib::be_ai_chat::BotSetupChatAI();
    if errnum != 0 {
        return errnum;
    }
    errnum = crate::src::botlib::be_ai_move::BotSetupMoveAI();
    if errnum != 0 {
        return errnum;
    }
    botlibsetup = crate::src::qcommon::q_shared::qtrue as i32;
    botlibglobals.botlibsetup = crate::src::qcommon::q_shared::qtrue as i32;
    return 0;
}
//end of the function Export_BotLibSetup
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Export_BotLibShutdown() -> i32 {
    if BotLibSetup(b"BotLibShutdown\x00" as *const u8 as *mut i8) as u64 == 0 {
        return 1i32;
    }
    //DumpFileCRCs();
    //DEMO
    //
    crate::src::botlib::be_ai_chat::BotShutdownChatAI(); //be_ai_chat.c
    crate::src::botlib::be_ai_move::BotShutdownMoveAI(); //be_ai_move.c
    crate::src::botlib::be_ai_goal::BotShutdownGoalAI(); //be_ai_goal.c
    crate::src::botlib::be_ai_weap::BotShutdownWeaponAI(); //be_ai_weap.c
    crate::src::botlib::be_ai_weight::BotShutdownWeights(); //be_ai_weight.c
    crate::src::botlib::be_ai_char::BotShutdownCharacters(); //be_ai_char.c
                                                             //shud down aas
    crate::src::botlib::be_aas_main::AAS_Shutdown();
    //shut down bot elemantary actions
    crate::src::botlib::be_ea::EA_Shutdown();
    //free all libvars
    crate::src::botlib::l_libvar::LibVarDeAllocAll();
    //remove all global defines from the pre compiler
    crate::src::botlib::l_precomp::PC_RemoveAllGlobalDefines();
    //dump all allocated memory
    //	DumpMemory();
    //shut down library log file
    crate::src::botlib::l_log::Log_Shutdown();
    //
    botlibsetup = crate::src::qcommon::q_shared::qfalse as i32;
    botlibglobals.botlibsetup = crate::src::qcommon::q_shared::qfalse as i32;
    // print any files still open
    crate::src::botlib::l_precomp::PC_CheckOpenSourceHandles();
    //
    return 0;
}
//end of the function Export_BotLibShutdown
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Export_BotLibVarSet(mut var_name: *const i8, mut value: *const i8) -> i32 {
    crate::src::botlib::l_libvar::LibVarSet(var_name, value);
    return 0;
}
//end of the function Export_BotLibVarSet
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Export_BotLibVarGet(
    mut var_name: *const i8,
    mut value: *mut i8,
    mut size: i32,
) -> i32 {
    let mut varvalue: *mut i8 = 0 as *mut i8;
    varvalue = crate::src::botlib::l_libvar::LibVarGetString(var_name);
    crate::stdlib::strncpy(value, varvalue, (size - 1i32) as usize);
    *value.offset((size - 1i32) as isize) = '\u{0}' as i8;
    return 0;
}
//end of the function Export_BotLibVarGet
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Export_BotLibStartFrame(mut time: f32) -> i32 {
    if BotLibSetup(b"BotStartFrame\x00" as *const u8 as *mut i8) as u64 == 0 {
        return 1i32;
    }
    return crate::src::botlib::be_aas_main::AAS_StartFrame(time);
}
//end of the function Export_BotLibStartFrame
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Export_BotLibLoadMap(mut mapname: *const i8) -> i32 {
    let mut errnum: i32 = 0;
    if BotLibSetup(b"BotLoadMap\x00" as *const u8 as *mut i8) as u64 == 0 {
        return 1i32;
    }
    //
    botimport.Print.expect("non-null function pointer")(
        1,
        b"------------ Map Loading ------------\n\x00" as *const u8 as *mut i8,
    );
    //startup AAS for the current map, model and sound index
    errnum = crate::src::botlib::be_aas_main::AAS_LoadMap(mapname);
    if errnum != 0 {
        return errnum;
    }
    //initialize the items in the level
    crate::src::botlib::be_ai_goal::BotInitLevelItems(); //be_ai_goal.h
    crate::src::botlib::be_ai_move::BotSetBrushModelTypes(); //be_ai_move.h
                                                             //
    botimport.Print.expect("non-null function pointer")(
        1,
        b"-------------------------------------\n\x00" as *const u8 as *mut i8,
    );
    //
    return 0;
}
//end of the function Export_BotLibLoadMap
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Export_BotLibUpdateEntity(
    mut ent: i32,
    mut state: *mut crate::botlib_h::bot_entitystate_t,
) -> i32 {
    if BotLibSetup(b"BotUpdateEntity\x00" as *const u8 as *mut i8) as u64 == 0 {
        return 1i32;
    }
    if ValidEntityNumber(ent, b"BotUpdateEntity\x00" as *const u8 as *mut i8) as u64 == 0 {
        return 2i32;
    }
    return crate::src::botlib::be_aas_entity::AAS_UpdateEntity(ent, state);
}
#[no_mangle]

pub unsafe extern "C" fn BotExportTest(
    mut _parm0: i32,
    mut _parm1: *mut i8,
    mut _parm2: *mut crate::src::qcommon::q_shared::vec_t,
    mut _parm3: *mut crate::src::qcommon::q_shared::vec_t,
) -> i32 {
    //	return AAS_PointLight(parm2, NULL, NULL, NULL);
    return 0;
}
//end of the function BotExportTest
/*
============
Init_AAS_Export
============
*/

unsafe extern "C" fn Init_AAS_Export(mut aas: *mut crate::botlib_h::aas_export_t) {
    //--------------------------------------------
    // be_aas_entity.c
    //--------------------------------------------
    (*aas).AAS_EntityInfo = Some(
        crate::src::botlib::be_aas_entity::AAS_EntityInfo
            as unsafe extern "C" fn(_: i32, _: *mut crate::be_aas_h::aas_entityinfo_t) -> (),
    );
    //--------------------------------------------
    // be_aas_main.c
    //--------------------------------------------
    (*aas).AAS_Initialized =
        Some(crate::src::botlib::be_aas_main::AAS_Initialized as unsafe extern "C" fn() -> i32);
    (*aas).AAS_PresenceTypeBoundingBox = Some(
        crate::src::botlib::be_aas_sample::AAS_PresenceTypeBoundingBox
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> (),
    );
    (*aas).AAS_Time =
        Some(crate::src::botlib::be_aas_main::AAS_Time as unsafe extern "C" fn() -> f32);
    //--------------------------------------------
    // be_aas_sample.c
    //--------------------------------------------
    (*aas).AAS_PointAreaNum = Some(
        crate::src::botlib::be_aas_sample::AAS_PointAreaNum
            as unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> i32,
    );
    (*aas).AAS_PointReachabilityAreaIndex = Some(
        crate::src::botlib::be_aas_sample::AAS_PointReachabilityAreaIndex
            as unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> i32,
    );
    (*aas).AAS_TraceAreas = Some(
        crate::src::botlib::be_aas_sample::AAS_TraceAreas
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut i32,
                _: *mut crate::src::qcommon::q_shared::vec3_t,
                _: i32,
            ) -> i32,
    );
    (*aas).AAS_BBoxAreas = Some(
        crate::src::botlib::be_aas_sample::AAS_BBoxAreas
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut i32,
                _: i32,
            ) -> i32,
    );
    (*aas).AAS_AreaInfo = Some(
        crate::src::botlib::be_aas_sample::AAS_AreaInfo
            as unsafe extern "C" fn(_: i32, _: *mut crate::be_aas_h::aas_areainfo_t) -> i32,
    );
    //--------------------------------------------
    // be_aas_bspq3.c
    //--------------------------------------------
    (*aas).AAS_PointContents = Some(
        crate::src::botlib::be_aas_bspq3::AAS_PointContents
            as unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> i32,
    );
    (*aas).AAS_NextBSPEntity = Some(
        crate::src::botlib::be_aas_bspq3::AAS_NextBSPEntity as unsafe extern "C" fn(_: i32) -> i32,
    );
    (*aas).AAS_ValueForBSPEpairKey = Some(
        crate::src::botlib::be_aas_bspq3::AAS_ValueForBSPEpairKey
            as unsafe extern "C" fn(_: i32, _: *mut i8, _: *mut i8, _: i32) -> i32,
    );
    (*aas).AAS_VectorForBSPEpairKey = Some(
        crate::src::botlib::be_aas_bspq3::AAS_VectorForBSPEpairKey
            as unsafe extern "C" fn(
                _: i32,
                _: *mut i8,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
    );
    (*aas).AAS_FloatForBSPEpairKey = Some(
        crate::src::botlib::be_aas_bspq3::AAS_FloatForBSPEpairKey
            as unsafe extern "C" fn(_: i32, _: *mut i8, _: *mut f32) -> i32,
    );
    (*aas).AAS_IntForBSPEpairKey = Some(
        crate::src::botlib::be_aas_bspq3::AAS_IntForBSPEpairKey
            as unsafe extern "C" fn(_: i32, _: *mut i8, _: *mut i32) -> i32,
    );
    //--------------------------------------------
    // be_aas_reach.c
    //--------------------------------------------
    (*aas).AAS_AreaReachability = Some(
        crate::src::botlib::be_aas_reach::AAS_AreaReachability
            as unsafe extern "C" fn(_: i32) -> i32,
    );
    //--------------------------------------------
    // be_aas_route.c
    //--------------------------------------------
    (*aas).AAS_AreaTravelTimeToGoalArea = Some(
        crate::src::botlib::be_aas_route::AAS_AreaTravelTimeToGoalArea
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
            ) -> i32,
    );
    (*aas).AAS_EnableRoutingArea = Some(
        crate::src::botlib::be_aas_route::AAS_EnableRoutingArea
            as unsafe extern "C" fn(_: i32, _: i32) -> i32,
    );
    (*aas).AAS_PredictRoute = Some(
        crate::src::botlib::be_aas_route::AAS_PredictRoute
            as unsafe extern "C" fn(
                _: *mut crate::be_aas_h::aas_predictroute_s,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
                _: i32,
            ) -> i32,
    );
    //--------------------------------------------
    // be_aas_altroute.c
    //--------------------------------------------
    (*aas).AAS_AlternativeRouteGoals = Some(
        crate::src::botlib::be_aas_routealt::AAS_AlternativeRouteGoals
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
                _: *mut crate::be_aas_h::aas_altroutegoal_t,
                _: i32,
                _: i32,
            ) -> i32,
    );
    //--------------------------------------------
    // be_aas_move.c
    //--------------------------------------------
    (*aas).AAS_Swimming = Some(
        crate::src::botlib::be_aas_move::AAS_Swimming
            as unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t) -> i32,
    );
    (*aas).AAS_PredictClientMovement = Some(
        crate::src::botlib::be_aas_move::AAS_PredictClientMovement
            as unsafe extern "C" fn(
                _: *mut crate::be_aas_h::aas_clientmove_s,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: i32,
                _: f32,
                _: i32,
                _: i32,
                _: i32,
            ) -> i32,
    );
}
/*
============
Init_EA_Export
============
*/

unsafe extern "C" fn Init_EA_Export(mut ea: *mut crate::botlib_h::ea_export_t) {
    //ClientCommand elementary actions
    (*ea).EA_Command = Some(
        crate::src::botlib::be_ea::EA_Command as unsafe extern "C" fn(_: i32, _: *mut i8) -> (),
    );
    (*ea).EA_Say =
        Some(crate::src::botlib::be_ea::EA_Say as unsafe extern "C" fn(_: i32, _: *mut i8) -> ());
    (*ea).EA_SayTeam = Some(
        crate::src::botlib::be_ea::EA_SayTeam as unsafe extern "C" fn(_: i32, _: *mut i8) -> (),
    );
    (*ea).EA_Action =
        Some(crate::src::botlib::be_ea::EA_Action as unsafe extern "C" fn(_: i32, _: i32) -> ());
    (*ea).EA_Gesture =
        Some(crate::src::botlib::be_ea::EA_Gesture as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_Talk = Some(crate::src::botlib::be_ea::EA_Talk as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_Attack =
        Some(crate::src::botlib::be_ea::EA_Attack as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_Use = Some(crate::src::botlib::be_ea::EA_Use as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_Respawn =
        Some(crate::src::botlib::be_ea::EA_Respawn as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_Crouch =
        Some(crate::src::botlib::be_ea::EA_Crouch as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_MoveUp =
        Some(crate::src::botlib::be_ea::EA_MoveUp as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_MoveDown =
        Some(crate::src::botlib::be_ea::EA_MoveDown as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_MoveForward =
        Some(crate::src::botlib::be_ea::EA_MoveForward as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_MoveBack =
        Some(crate::src::botlib::be_ea::EA_MoveBack as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_MoveLeft =
        Some(crate::src::botlib::be_ea::EA_MoveLeft as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_MoveRight =
        Some(crate::src::botlib::be_ea::EA_MoveRight as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_SelectWeapon = Some(
        crate::src::botlib::be_ea::EA_SelectWeapon as unsafe extern "C" fn(_: i32, _: i32) -> (),
    );
    (*ea).EA_Jump = Some(crate::src::botlib::be_ea::EA_Jump as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_DelayedJump =
        Some(crate::src::botlib::be_ea::EA_DelayedJump as unsafe extern "C" fn(_: i32) -> ());
    (*ea).EA_Move = Some(
        crate::src::botlib::be_ea::EA_Move
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: f32,
            ) -> (),
    );
    (*ea).EA_View = Some(
        crate::src::botlib::be_ea::EA_View
            as unsafe extern "C" fn(_: i32, _: *mut crate::src::qcommon::q_shared::vec_t) -> (),
    );
    (*ea).EA_GetInput = Some(
        crate::src::botlib::be_ea::EA_GetInput
            as unsafe extern "C" fn(_: i32, _: f32, _: *mut crate::botlib_h::bot_input_t) -> (),
    );
    (*ea).EA_EndRegular = Some(
        crate::src::botlib::be_ea::EA_EndRegular as unsafe extern "C" fn(_: i32, _: f32) -> (),
    );
    (*ea).EA_ResetInput =
        Some(crate::src::botlib::be_ea::EA_ResetInput as unsafe extern "C" fn(_: i32) -> ());
}
/*
============
Init_AI_Export
============
*/

unsafe extern "C" fn Init_AI_Export(mut ai: *mut crate::botlib_h::ai_export_t) {
    //-----------------------------------
    // be_ai_char.h
    //-----------------------------------
    (*ai).BotLoadCharacter = Some(
        crate::src::botlib::be_ai_char::BotLoadCharacter
            as unsafe extern "C" fn(_: *mut i8, _: f32) -> i32,
    );
    (*ai).BotFreeCharacter = Some(
        crate::src::botlib::be_ai_char::BotFreeCharacter as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).Characteristic_Float = Some(
        crate::src::botlib::be_ai_char::Characteristic_Float
            as unsafe extern "C" fn(_: i32, _: i32) -> f32,
    );
    (*ai).Characteristic_BFloat = Some(
        crate::src::botlib::be_ai_char::Characteristic_BFloat
            as unsafe extern "C" fn(_: i32, _: i32, _: f32, _: f32) -> f32,
    );
    (*ai).Characteristic_Integer = Some(
        crate::src::botlib::be_ai_char::Characteristic_Integer
            as unsafe extern "C" fn(_: i32, _: i32) -> i32,
    );
    (*ai).Characteristic_BInteger = Some(
        crate::src::botlib::be_ai_char::Characteristic_BInteger
            as unsafe extern "C" fn(_: i32, _: i32, _: i32, _: i32) -> i32,
    );
    (*ai).Characteristic_String = Some(
        crate::src::botlib::be_ai_char::Characteristic_String
            as unsafe extern "C" fn(_: i32, _: i32, _: *mut i8, _: i32) -> (),
    );
    //-----------------------------------
    // be_ai_chat.h
    //-----------------------------------
    (*ai).BotAllocChatState =
        Some(crate::src::botlib::be_ai_chat::BotAllocChatState as unsafe extern "C" fn() -> i32);
    (*ai).BotFreeChatState = Some(
        crate::src::botlib::be_ai_chat::BotFreeChatState as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotQueueConsoleMessage = Some(
        crate::src::botlib::be_ai_chat::BotQueueConsoleMessage
            as unsafe extern "C" fn(_: i32, _: i32, _: *mut i8) -> (),
    );
    (*ai).BotRemoveConsoleMessage = Some(
        crate::src::botlib::be_ai_chat::BotRemoveConsoleMessage
            as unsafe extern "C" fn(_: i32, _: i32) -> (),
    );
    (*ai).BotNextConsoleMessage = Some(
        crate::src::botlib::be_ai_chat::BotNextConsoleMessage
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::botlib::be_ai_chat::bot_consolemessage_t,
            ) -> i32,
    );
    (*ai).BotNumConsoleMessages = Some(
        crate::src::botlib::be_ai_chat::BotNumConsoleMessages
            as unsafe extern "C" fn(_: i32) -> i32,
    );
    (*ai).BotInitialChat = Some(
        crate::src::botlib::be_ai_chat::BotInitialChat
            as unsafe extern "C" fn(
                _: i32,
                _: *mut i8,
                _: i32,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
            ) -> (),
    );
    (*ai).BotNumInitialChats = Some(
        crate::src::botlib::be_ai_chat::BotNumInitialChats
            as unsafe extern "C" fn(_: i32, _: *mut i8) -> i32,
    );
    (*ai).BotReplyChat = Some(
        crate::src::botlib::be_ai_chat::BotReplyChat
            as unsafe extern "C" fn(
                _: i32,
                _: *mut i8,
                _: i32,
                _: i32,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
                _: *mut i8,
            ) -> i32,
    );
    (*ai).BotChatLength =
        Some(crate::src::botlib::be_ai_chat::BotChatLength as unsafe extern "C" fn(_: i32) -> i32);
    (*ai).BotEnterChat = Some(
        crate::src::botlib::be_ai_chat::BotEnterChat
            as unsafe extern "C" fn(_: i32, _: i32, _: i32) -> (),
    );
    (*ai).BotGetChatMessage = Some(
        crate::src::botlib::be_ai_chat::BotGetChatMessage
            as unsafe extern "C" fn(_: i32, _: *mut i8, _: i32) -> (),
    );
    (*ai).StringContains = Some(
        crate::src::botlib::be_ai_chat::StringContains
            as unsafe extern "C" fn(_: *mut i8, _: *mut i8, _: i32) -> i32,
    );
    (*ai).BotFindMatch = Some(
        crate::src::botlib::be_ai_chat::BotFindMatch
            as unsafe extern "C" fn(
                _: *mut i8,
                _: *mut crate::src::botlib::be_ai_chat::bot_match_t,
                _: usize,
            ) -> i32,
    );
    (*ai).BotMatchVariable = Some(
        crate::src::botlib::be_ai_chat::BotMatchVariable
            as unsafe extern "C" fn(
                _: *mut crate::src::botlib::be_ai_chat::bot_match_t,
                _: i32,
                _: *mut i8,
                _: i32,
            ) -> (),
    );
    (*ai).UnifyWhiteSpaces = Some(
        crate::src::botlib::be_ai_chat::UnifyWhiteSpaces as unsafe extern "C" fn(_: *mut i8) -> (),
    );
    (*ai).BotReplaceSynonyms = Some(
        crate::src::botlib::be_ai_chat::BotReplaceSynonyms
            as unsafe extern "C" fn(_: *mut i8, _: usize) -> (),
    );
    (*ai).BotLoadChatFile = Some(
        crate::src::botlib::be_ai_chat::BotLoadChatFile
            as unsafe extern "C" fn(_: i32, _: *mut i8, _: *mut i8) -> i32,
    );
    (*ai).BotSetChatGender = Some(
        crate::src::botlib::be_ai_chat::BotSetChatGender
            as unsafe extern "C" fn(_: i32, _: i32) -> (),
    );
    (*ai).BotSetChatName = Some(
        crate::src::botlib::be_ai_chat::BotSetChatName
            as unsafe extern "C" fn(_: i32, _: *mut i8, _: i32) -> (),
    );
    //-----------------------------------
    // be_ai_goal.h
    //-----------------------------------
    (*ai).BotResetGoalState = Some(
        crate::src::botlib::be_ai_goal::BotResetGoalState as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotResetAvoidGoals = Some(
        crate::src::botlib::be_ai_goal::BotResetAvoidGoals as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotRemoveFromAvoidGoals = Some(
        crate::src::botlib::be_ai_goal::BotRemoveFromAvoidGoals
            as unsafe extern "C" fn(_: i32, _: i32) -> (),
    );
    (*ai).BotPushGoal = Some(
        crate::src::botlib::be_ai_goal::BotPushGoal
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
            ) -> (),
    );
    (*ai).BotPopGoal =
        Some(crate::src::botlib::be_ai_goal::BotPopGoal as unsafe extern "C" fn(_: i32) -> ());
    (*ai).BotEmptyGoalStack = Some(
        crate::src::botlib::be_ai_goal::BotEmptyGoalStack as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotDumpAvoidGoals = Some(
        crate::src::botlib::be_ai_goal::BotDumpAvoidGoals as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotDumpGoalStack = Some(
        crate::src::botlib::be_ai_goal::BotDumpGoalStack as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotGoalName = Some(
        crate::src::botlib::be_ai_goal::BotGoalName
            as unsafe extern "C" fn(_: i32, _: *mut i8, _: i32) -> (),
    );
    (*ai).BotGetTopGoal = Some(
        crate::src::botlib::be_ai_goal::BotGetTopGoal
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
            ) -> i32,
    );
    (*ai).BotGetSecondGoal = Some(
        crate::src::botlib::be_ai_goal::BotGetSecondGoal
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
            ) -> i32,
    );
    (*ai).BotChooseLTGItem = Some(
        crate::src::botlib::be_ai_goal::BotChooseLTGItem
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut i32,
                _: i32,
            ) -> i32,
    );
    (*ai).BotChooseNBGItem = Some(
        crate::src::botlib::be_ai_goal::BotChooseNBGItem
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut i32,
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
                _: f32,
            ) -> i32,
    );
    (*ai).BotTouchingGoal = Some(
        crate::src::botlib::be_ai_goal::BotTouchingGoal
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
            ) -> i32,
    );
    (*ai).BotItemGoalInVisButNotVisible = Some(
        crate::src::botlib::be_ai_goal::BotItemGoalInVisButNotVisible
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
            ) -> i32,
    );
    (*ai).BotGetLevelItemGoal = Some(
        crate::src::botlib::be_ai_goal::BotGetLevelItemGoal
            as unsafe extern "C" fn(
                _: i32,
                _: *mut i8,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
            ) -> i32,
    );
    (*ai).BotGetNextCampSpotGoal = Some(
        crate::src::botlib::be_ai_goal::BotGetNextCampSpotGoal
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
            ) -> i32,
    );
    (*ai).BotGetMapLocationGoal = Some(
        crate::src::botlib::be_ai_goal::BotGetMapLocationGoal
            as unsafe extern "C" fn(
                _: *mut i8,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
            ) -> i32,
    );
    (*ai).BotAvoidGoalTime = Some(
        crate::src::botlib::be_ai_goal::BotAvoidGoalTime
            as unsafe extern "C" fn(_: i32, _: i32) -> f32,
    );
    (*ai).BotSetAvoidGoalTime = Some(
        crate::src::botlib::be_ai_goal::BotSetAvoidGoalTime
            as unsafe extern "C" fn(_: i32, _: i32, _: f32) -> (),
    );
    (*ai).BotInitLevelItems =
        Some(crate::src::botlib::be_ai_goal::BotInitLevelItems as unsafe extern "C" fn() -> ());
    (*ai).BotUpdateEntityItems =
        Some(crate::src::botlib::be_ai_goal::BotUpdateEntityItems as unsafe extern "C" fn() -> ());
    (*ai).BotLoadItemWeights = Some(
        crate::src::botlib::be_ai_goal::BotLoadItemWeights
            as unsafe extern "C" fn(_: i32, _: *mut i8) -> i32,
    );
    (*ai).BotFreeItemWeights = Some(
        crate::src::botlib::be_ai_goal::BotFreeItemWeights as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotInterbreedGoalFuzzyLogic = Some(
        crate::src::botlib::be_ai_goal::BotInterbreedGoalFuzzyLogic
            as unsafe extern "C" fn(_: i32, _: i32, _: i32) -> (),
    );
    (*ai).BotSaveGoalFuzzyLogic = Some(
        crate::src::botlib::be_ai_goal::BotSaveGoalFuzzyLogic
            as unsafe extern "C" fn(_: i32, _: *mut i8) -> (),
    );
    (*ai).BotMutateGoalFuzzyLogic = Some(
        crate::src::botlib::be_ai_goal::BotMutateGoalFuzzyLogic
            as unsafe extern "C" fn(_: i32, _: f32) -> (),
    );
    (*ai).BotAllocGoalState = Some(
        crate::src::botlib::be_ai_goal::BotAllocGoalState as unsafe extern "C" fn(_: i32) -> i32,
    );
    (*ai).BotFreeGoalState = Some(
        crate::src::botlib::be_ai_goal::BotFreeGoalState as unsafe extern "C" fn(_: i32) -> (),
    );
    //-----------------------------------
    // be_ai_move.h
    //-----------------------------------
    (*ai).BotResetMoveState = Some(
        crate::src::botlib::be_ai_move::BotResetMoveState as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotMoveToGoal = Some(
        crate::src::botlib::be_ai_move::BotMoveToGoal
            as unsafe extern "C" fn(
                _: *mut crate::src::botlib::be_ai_move::bot_moveresult_t,
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
                _: i32,
            ) -> (),
    );
    (*ai).BotMoveInDirection = Some(
        crate::src::botlib::be_ai_move::BotMoveInDirection
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: f32,
                _: i32,
            ) -> i32,
    );
    (*ai).BotResetAvoidReach = Some(
        crate::src::botlib::be_ai_move::BotResetAvoidReach as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotResetLastAvoidReach = Some(
        crate::src::botlib::be_ai_move::BotResetLastAvoidReach
            as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotReachabilityArea = Some(
        crate::src::botlib::be_ai_move::BotReachabilityArea
            as unsafe extern "C" fn(_: *mut crate::src::qcommon::q_shared::vec_t, _: i32) -> i32,
    );
    (*ai).BotMovementViewTarget = Some(
        crate::src::botlib::be_ai_move::BotMovementViewTarget
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
                _: i32,
                _: f32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
    );
    (*ai).BotPredictVisiblePosition = Some(
        crate::src::botlib::be_ai_move::BotPredictVisiblePosition
            as unsafe extern "C" fn(
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: i32,
                _: *mut crate::src::botlib::be_ai_goal::bot_goal_t,
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
    );
    (*ai).BotAllocMoveState =
        Some(crate::src::botlib::be_ai_move::BotAllocMoveState as unsafe extern "C" fn() -> i32);
    (*ai).BotFreeMoveState = Some(
        crate::src::botlib::be_ai_move::BotFreeMoveState as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotInitMoveState = Some(
        crate::src::botlib::be_ai_move::BotInitMoveState
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::botlib::be_ai_move::bot_initmove_t,
            ) -> (),
    );
    (*ai).BotAddAvoidSpot = Some(
        crate::src::botlib::be_ai_move::BotAddAvoidSpot
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: f32,
                _: i32,
            ) -> (),
    );
    //-----------------------------------
    // be_ai_weap.h
    //-----------------------------------
    (*ai).BotChooseBestFightWeapon = Some(
        crate::src::botlib::be_ai_weap::BotChooseBestFightWeapon
            as unsafe extern "C" fn(_: i32, _: *mut i32) -> i32,
    );
    (*ai).BotGetWeaponInfo = Some(
        crate::src::botlib::be_ai_weap::BotGetWeaponInfo
            as unsafe extern "C" fn(
                _: i32,
                _: i32,
                _: *mut crate::src::botlib::be_ai_weap::weaponinfo_t,
            ) -> (),
    );
    (*ai).BotLoadWeaponWeights = Some(
        crate::src::botlib::be_ai_weap::BotLoadWeaponWeights
            as unsafe extern "C" fn(_: i32, _: *mut i8) -> i32,
    );
    (*ai).BotAllocWeaponState =
        Some(crate::src::botlib::be_ai_weap::BotAllocWeaponState as unsafe extern "C" fn() -> i32);
    (*ai).BotFreeWeaponState = Some(
        crate::src::botlib::be_ai_weap::BotFreeWeaponState as unsafe extern "C" fn(_: i32) -> (),
    );
    (*ai).BotResetWeaponState = Some(
        crate::src::botlib::be_ai_weap::BotResetWeaponState as unsafe extern "C" fn(_: i32) -> (),
    );
    //-----------------------------------
    // be_ai_gen.h
    //-----------------------------------
    (*ai).GeneticParentsAndChildSelection = Some(
        crate::src::botlib::be_ai_gen::GeneticParentsAndChildSelection
            as unsafe extern "C" fn(
                _: i32,
                _: *mut f32,
                _: *mut i32,
                _: *mut i32,
                _: *mut i32,
            ) -> i32,
    );
}
//linking of bot library
/*
============
GetBotLibAPI
============
*/
#[no_mangle]

pub unsafe extern "C" fn GetBotLibAPI(
    mut apiVersion: i32,
    mut import: *mut crate::botlib_h::botlib_import_t,
) -> *mut crate::botlib_h::botlib_export_t {
    botimport = *import;
    crate::stdlib::memset(
        &mut be_botlib_export as *mut crate::botlib_h::botlib_export_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::botlib_h::botlib_export_t>(),
    );
    if apiVersion != 2 {
        botimport.Print.expect("non-null function pointer")(
            3,
            b"Mismatched BOTLIB_API_VERSION: expected %i, got %i\n\x00" as *const u8 as *mut i8,
            2i32,
            apiVersion,
        );
        return 0 as *mut crate::botlib_h::botlib_export_t;
    }
    Init_AAS_Export(&mut be_botlib_export.aas);
    Init_EA_Export(&mut be_botlib_export.ea);
    Init_AI_Export(&mut be_botlib_export.ai);
    be_botlib_export.BotLibSetup = Some(Export_BotLibSetup as unsafe extern "C" fn() -> i32);
    be_botlib_export.BotLibShutdown = Some(Export_BotLibShutdown as unsafe extern "C" fn() -> i32);
    be_botlib_export.BotLibVarSet =
        Some(Export_BotLibVarSet as unsafe extern "C" fn(_: *const i8, _: *const i8) -> i32);
    be_botlib_export.BotLibVarGet =
        Some(Export_BotLibVarGet as unsafe extern "C" fn(_: *const i8, _: *mut i8, _: i32) -> i32);
    be_botlib_export.PC_AddGlobalDefine = Some(
        crate::src::botlib::l_precomp::PC_AddGlobalDefine
            as unsafe extern "C" fn(_: *mut i8) -> i32,
    );
    be_botlib_export.PC_LoadSourceHandle = Some(
        crate::src::botlib::l_precomp::PC_LoadSourceHandle
            as unsafe extern "C" fn(_: *const i8) -> i32,
    );
    be_botlib_export.PC_FreeSourceHandle = Some(
        crate::src::botlib::l_precomp::PC_FreeSourceHandle as unsafe extern "C" fn(_: i32) -> i32,
    );
    be_botlib_export.PC_ReadTokenHandle = Some(
        crate::src::botlib::l_precomp::PC_ReadTokenHandle
            as unsafe extern "C" fn(
                _: i32,
                _: *mut crate::src::qcommon::q_shared::pc_token_t,
            ) -> i32,
    );
    be_botlib_export.PC_SourceFileAndLine = Some(
        crate::src::botlib::l_precomp::PC_SourceFileAndLine
            as unsafe extern "C" fn(_: i32, _: *mut i8, _: *mut i32) -> i32,
    );
    be_botlib_export.BotLibStartFrame =
        Some(Export_BotLibStartFrame as unsafe extern "C" fn(_: f32) -> i32);
    be_botlib_export.BotLibLoadMap =
        Some(Export_BotLibLoadMap as unsafe extern "C" fn(_: *const i8) -> i32);
    be_botlib_export.BotLibUpdateEntity = Some(
        Export_BotLibUpdateEntity
            as unsafe extern "C" fn(_: i32, _: *mut crate::botlib_h::bot_entitystate_t) -> i32,
    );
    be_botlib_export.Test = Some(
        BotExportTest
            as unsafe extern "C" fn(
                _: i32,
                _: *mut i8,
                _: *mut crate::src::qcommon::q_shared::vec_t,
                _: *mut crate::src::qcommon::q_shared::vec_t,
            ) -> i32,
    );
    return &mut be_botlib_export;
}
