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
    pub type fileHandle_t = libc::c_int;
    // parameters to the main Error routine
    pub type unnamed = libc::c_uint;
    // pop up the need-cd dialog
    pub const ERR_NEED_CD: unnamed = 4;
    // client disconnected from the server
    pub const ERR_DISCONNECT: unnamed = 3;
    // don't kill server
    pub const ERR_SERVERDISCONNECT: unnamed = 2;
    // print to console and disconnect from game
    pub const ERR_DROP: unnamed = 1;
    // exit the entire game with a popup window
    pub const ERR_FATAL: unnamed = 0;
    // font rendering values used by ui and cgame
    // default
    // default
    pub type ha_pref = libc::c_uint;
    pub const h_dontcare: ha_pref = 2;
    pub const h_low: ha_pref = 1;
    pub const h_high: ha_pref = 0;
    /*
==============================================================

MATHLIB

==============================================================
*/
    pub type vec_t = libc::c_float;
    pub type vec3_t = [vec_t; 3];
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct cvar_s {
        pub name: *mut libc::c_char,
        pub string: *mut libc::c_char,
        pub resetString: *mut libc::c_char,
        pub latchedString: *mut libc::c_char,
        pub flags: libc::c_int,
        pub modified: qboolean,
        pub modificationCount: libc::c_int,
        pub value: libc::c_float,
        pub integer: libc::c_int,
        pub validate: qboolean,
        pub integral: qboolean,
        pub min: libc::c_float,
        pub max: libc::c_float,
        pub description: *mut libc::c_char,
        pub next: *mut cvar_t,
        pub prev: *mut cvar_t,
        pub hashNext: *mut cvar_t,
        pub hashPrev: *mut cvar_t,
        pub hashIndex: libc::c_int,
    }
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
    pub type cvar_t = cvar_s;
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
        pub fn Hunk_AllocDebug(size: libc::c_int, preference: ha_pref,
                               label: *mut libc::c_char,
                               file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
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
        //=============================================
        //
// key / value info strings
//
        #[no_mangle]
        pub fn Info_ValueForKey(s: *const libc::c_char,
                                key: *const libc::c_char)
         -> *mut libc::c_char;
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
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
// qcommon.h -- definitions common between client and server, but not game.or ref modules
    //Ignore __attribute__ on non-gcc platforms
    //#define	PRE_RELEASE_DEMO
    //============================================================================
    //
// msg.c
//
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct msg_t {
        pub allowoverflow: qboolean,
        pub overflowed: qboolean,
        pub oob: qboolean,
        pub data: *mut byte,
        pub maxsize: libc::c_int,
        pub cursize: libc::c_int,
        pub readcount: libc::c_int,
        pub bit: libc::c_int,
    }
    //============================================================================
    /*
==============================================================

NET

==============================================================
*/
    // if this flag is set, always attempt ipv6 connections instead of ipv4 if a v6 address is found.
    // disables ipv6 multicast support if set.
    // number of old messages that must be kept on client and
    // server for delta comrpession and ping estimation
    // max number of usercmd_t in a packet
    // max string commands buffered for restransmit
    pub type netadrtype_t = libc::c_uint;
    pub const NA_UNSPEC: netadrtype_t = 7;
    pub const NA_MULTICAST6: netadrtype_t = 6;
    pub const NA_IP6: netadrtype_t = 5;
    pub const NA_IP: netadrtype_t = 4;
    pub const NA_BROADCAST: netadrtype_t = 3;
    pub const NA_LOOPBACK: netadrtype_t = 2;
    pub const NA_BOT: netadrtype_t = 1;
    // an address lookup failed
    pub const NA_BAD: netadrtype_t = 0;
    pub type netsrc_t = libc::c_uint;
    pub const NS_SERVER: netsrc_t = 1;
    pub const NS_CLIENT: netsrc_t = 0;
    // maximum length of an IPv6 address string including trailing '\0'
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netadr_t {
        pub type_0: netadrtype_t,
        pub ip: [byte; 4],
        pub ip6: [byte; 16],
        pub port: libc::c_ushort,
        pub scope_id: libc::c_ulong,
    }
    // max length of a message, which may
    // be fragmented into multiple packets
    // ACK window of 48 download chunks. Cannot set this higher, or clients
    // will overflow the reliable commands buffer
    // 896 byte block chunks
    /*
Netchan handles packet fragmentation and out of order / duplicate suppression
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netchan_t {
        pub sock: netsrc_t,
        pub dropped: libc::c_int,
        pub remoteAddress: netadr_t,
        pub qport: libc::c_int,
        pub incomingSequence: libc::c_int,
        pub outgoingSequence: libc::c_int,
        pub fragmentSequence: libc::c_int,
        pub fragmentLength: libc::c_int,
        pub fragmentBuffer: [byte; 16384],
        pub unsentFragments: qboolean,
        pub unsentFragmentStart: libc::c_int,
        pub unsentLength: libc::c_int,
        pub unsentBuffer: [byte; 16384],
        pub challenge: libc::c_int,
        pub lastSentTime: libc::c_int,
        pub lastSentSize: libc::c_int,
        pub compat: qboolean,
    }
    pub type vm_t = vm_s;
    use super::q_shared_h::{qboolean, byte, cvar_t, fileHandle_t};
    use super::{libc};
    use super::stdint_h::{intptr_t};
    extern "C" {
        /*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
        pub type vm_s;
        #[no_mangle]
        pub fn NET_JoinMulticast6();
        #[no_mangle]
        pub fn NET_LeaveMulticast6();
        #[no_mangle]
        pub fn VM_Call(vm: *mut vm_t, callNum: libc::c_int, ...) -> intptr_t;
        #[no_mangle]
        pub fn VM_ExplicitArgPtr(vm: *mut vm_t, intValue: intptr_t)
         -> *mut libc::c_void;
        // allocates an initial text buffer that will grow as needed
        #[no_mangle]
        pub fn Cbuf_AddText(text: *const libc::c_char);
        // Parses a single line of text into arguments and tries to execute it
// as if it was typed at the console
        /*
==============================================================

CVAR

==============================================================
*/
        /*

cvar_t variables are used to hold scalar or string variables that can be changed
or displayed at the console or prog code as well as accessed directly
in C code.

The user can access cvars from the console in three ways:
r_draworder			prints the current value
r_draworder 0		sets the current value to 0
set r_draworder 0	as above, but creates the cvar if not present

Cvars are restricted from having the same names as commands to keep this
interface from being ambiguous.

The are also occasionally used to communicated information between different
modules of the program.

*/
        #[no_mangle]
        pub fn Cvar_Get(var_name: *const libc::c_char,
                        value: *const libc::c_char, flags: libc::c_int)
         -> *mut cvar_t;
        // updates an interpreted modules' version of a cvar
        #[no_mangle]
        pub fn Cvar_Set(var_name: *const libc::c_char,
                        value: *const libc::c_char);
        // expands value to a string and calls Cvar_Set/Cvar_SetSafe
        #[no_mangle]
        pub fn Cvar_VariableValue(var_name: *const libc::c_char)
         -> libc::c_float;
        #[no_mangle]
        pub fn Cvar_InfoString(bit: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cvar_InfoString_Big(bit: libc::c_int) -> *mut libc::c_char;
        #[no_mangle]
        pub fn Cvar_CheckRange(cv: *mut cvar_t, minVal: libc::c_float,
                               maxVal: libc::c_float,
                               shouldBeIntegral: qboolean);
        #[no_mangle]
        pub static mut cvar_modifiedFlags: libc::c_int;
        #[no_mangle]
        pub fn FS_Restart(checksumFeed: libc::c_int);
        #[no_mangle]
        pub fn FS_FOpenFileRead(qpath: *const libc::c_char,
                                file: *mut fileHandle_t, uniqueFILE: qboolean)
         -> libc::c_long;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        #[no_mangle]
        pub fn FS_LoadedPakNames() -> *const libc::c_char;
        #[no_mangle]
        pub fn FS_LoadedPakChecksums() -> *const libc::c_char;
        // Returns a space separated string containing the checksums of all loaded pk3 files.
// Servers with sv_pure set will get this string and pass it to clients.
        #[no_mangle]
        pub fn FS_ReferencedPakNames() -> *const libc::c_char;
        #[no_mangle]
        pub fn FS_ReferencedPakChecksums() -> *const libc::c_char;
        // Returns a space separated string containing the checksums of all loaded 
// AND referenced pk3 files. Servers with sv_pure set will get this string 
// back from clients for pure validation 
        #[no_mangle]
        pub fn FS_ClearPakReferences(flags: libc::c_int);
        #[no_mangle]
        pub fn CopyString(in_0: *const libc::c_char) -> *mut libc::c_char;
        // will be journaled properly
        #[no_mangle]
        pub fn Com_Milliseconds() -> libc::c_int;
        #[no_mangle]
        pub static mut com_dedicated: *mut cvar_t;
        #[no_mangle]
        pub static mut com_sv_running: *mut cvar_t;
        #[no_mangle]
        pub static mut com_frameTime: libc::c_int;
        #[no_mangle]
        pub static mut com_errorEntered: qboolean;
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn Hunk_Clear();
        #[no_mangle]
        pub fn Hunk_SetMark();
        #[no_mangle]
        pub fn Hunk_AllocateTempMemory(size: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Hunk_FreeTempMemory(buf: *mut libc::c_void);
        #[no_mangle]
        pub fn CL_Disconnect(showMainMenu: qboolean);
        #[no_mangle]
        pub fn CL_MapLoading();
        // dump all memory on an error
        #[no_mangle]
        pub fn CL_ShutdownAll(shutdownRef: qboolean);
        // initialize renderer interface
        #[no_mangle]
        pub fn CL_StartHunkUsers(rendererOnly: qboolean);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/server/server.h"]
pub mod server_h {
    // this structure will be cleared only when the game dll changes
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct serverStatic_t {
        pub initialized: qboolean,
        pub time: libc::c_int,
        pub snapFlagServerBit: libc::c_int,
        pub clients: *mut client_t,
        pub numSnapshotEntities: libc::c_int,
        pub nextSnapshotEntities: libc::c_int,
        pub snapshotEntities: *mut entityState_t,
        pub nextHeartbeatTime: libc::c_int,
        pub challenges: [challenge_t; 2048],
        pub redirectAddress: netadr_t,
        pub authorizeAddress: netadr_t,
        pub masterResolveTime: [libc::c_int; 5],
    }
    //=============================================================================
    // MAX_CHALLENGES is made large to prevent a denial
// of service attack that could cycle all of them
// out before legitimate users connected
    // Allow a certain amount of challenges to have the same IP address
// to make it a bit harder to DOS one single IP address from connecting
// while not allowing a single ip to grab all challenge resources
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct challenge_t {
        pub adr: netadr_t,
        pub challenge: libc::c_int,
        pub clientChallenge: libc::c_int,
        pub time: libc::c_int,
        pub pingTime: libc::c_int,
        pub firstTime: libc::c_int,
        pub wasrefused: qboolean,
        pub connected: qboolean,
    }
    pub type client_t = client_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct client_s {
        pub state: clientState_t,
        pub userinfo: [libc::c_char; 1024],
        pub reliableCommands: [[libc::c_char; 1024]; 64],
        pub reliableSequence: libc::c_int,
        pub reliableAcknowledge: libc::c_int,
        pub reliableSent: libc::c_int,
        pub messageAcknowledge: libc::c_int,
        pub gamestateMessageNum: libc::c_int,
        pub challenge: libc::c_int,
        pub lastUsercmd: usercmd_t,
        pub lastMessageNum: libc::c_int,
        pub lastClientCommand: libc::c_int,
        pub lastClientCommandString: [libc::c_char; 1024],
        pub gentity: *mut sharedEntity_t,
        pub name: [libc::c_char; 32],
        pub downloadName: [libc::c_char; 64],
        pub download: fileHandle_t,
        pub downloadSize: libc::c_int,
        pub downloadCount: libc::c_int,
        pub downloadClientBlock: libc::c_int,
        pub downloadCurrentBlock: libc::c_int,
        pub downloadXmitBlock: libc::c_int,
        pub downloadBlocks: [*mut libc::c_uchar; 48],
        pub downloadBlockSize: [libc::c_int; 48],
        pub downloadEOF: qboolean,
        pub downloadSendTime: libc::c_int,
        pub deltaMessage: libc::c_int,
        pub nextReliableTime: libc::c_int,
        pub lastPacketTime: libc::c_int,
        pub lastConnectTime: libc::c_int,
        pub lastSnapshotTime: libc::c_int,
        pub rateDelayed: qboolean,
        pub timeoutCount: libc::c_int,
        pub frames: [clientSnapshot_t; 32],
        pub ping: libc::c_int,
        pub rate: libc::c_int,
        pub snapshotMsec: libc::c_int,
        pub pureAuthentic: libc::c_int,
        pub gotCP: qboolean,
        pub netchan: netchan_t,
        pub netchan_start_queue: *mut netchan_buffer_t,
        pub netchan_end_queue: *mut *mut netchan_buffer_t,
        pub hasVoip: qboolean,
        pub muteAllVoip: qboolean,
        pub ignoreVoipFromClient: [qboolean; 64],
        pub voipPacket: [*mut voipServerPacket_t; 64],
        pub queuedVoipPackets: libc::c_int,
        pub queuedVoipIndex: libc::c_int,
        pub oldServerTime: libc::c_int,
        pub csUpdated: [qboolean; 1024],
        pub compat: qboolean,
    }
    pub type voipServerPacket_t = voipServerPacket_s;
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
// server.h
    //=============================================================================
    // !!! MUST NOT CHANGE, SERVER AND
    // GAME BOTH REFERENCE !!!
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct voipServerPacket_s {
        pub generation: libc::c_int,
        pub sequence: libc::c_int,
        pub frames: libc::c_int,
        pub len: libc::c_int,
        pub sender: libc::c_int,
        pub flags: libc::c_int,
        pub data: [byte; 4000],
    }
    pub type netchan_buffer_t = netchan_buffer_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct netchan_buffer_s {
        pub msg: msg_t,
        pub msgBuffer: [byte; 16384],
        pub clientCommandString: [libc::c_char; 1024],
        pub next: *mut netchan_buffer_s,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientSnapshot_t {
        pub areabytes: libc::c_int,
        pub areabits: [byte; 32],
        pub ps: playerState_t,
        pub num_entities: libc::c_int,
        pub first_entity: libc::c_int,
        pub messageSent: libc::c_int,
        pub messageAcked: libc::c_int,
        pub messageSize: libc::c_int,
    }
    pub type clientState_t = libc::c_uint;
    // client is fully in game
    pub const CS_ACTIVE: clientState_t = 4;
    // gamestate has been sent, but client hasn't sent a usercmd
    pub const CS_PRIMED: clientState_t = 3;
    // connection for a couple seconds
    // has been assigned to a client_t, but no gamestate yet
    pub const CS_CONNECTED: clientState_t = 2;
    // client has been disconnected, but don't reuse
    pub const CS_ZOMBIE: clientState_t = 1;
    // can be reused for a new connection
    pub const CS_FREE: clientState_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct server_t {
        pub state: serverState_t,
        pub restarting: qboolean,
        pub serverId: libc::c_int,
        pub restartedServerId: libc::c_int,
        pub checksumFeed: libc::c_int,
        pub checksumFeedServerId: libc::c_int,
        pub snapshotCounter: libc::c_int,
        pub timeResidual: libc::c_int,
        pub nextFrameTime: libc::c_int,
        pub configstrings: [*mut libc::c_char; 1024],
        pub svEntities: [svEntity_t; 1024],
        pub entityParsePoint: *mut libc::c_char,
        pub gentities: *mut sharedEntity_t,
        pub gentitySize: libc::c_int,
        pub num_entities: libc::c_int,
        pub gameClients: *mut playerState_t,
        pub gameClientSize: libc::c_int,
        pub restartTime: libc::c_int,
        pub time: libc::c_int,
    }
    pub type svEntity_t = svEntity_s;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct svEntity_s {
        pub worldSector: *mut worldSector_s,
        pub nextEntityInWorldSector: *mut svEntity_s,
        pub baseline: entityState_t,
        pub numClusters: libc::c_int,
        pub clusternums: [libc::c_int; 16],
        pub lastCluster: libc::c_int,
        pub areanum: libc::c_int,
        pub areanum2: libc::c_int,
        pub snapshotCounter: libc::c_int,
    }
    pub type serverState_t = libc::c_uint;
    // actively running
    pub const SS_GAME: serverState_t = 2;
    // spawning level entities
    pub const SS_LOADING: serverState_t = 1;
    // no map loaded
    pub const SS_DEAD: serverState_t = 0;
    use super::q_shared_h::{qboolean, entityState_t, usercmd_t, fileHandle_t,
                            byte, playerState_t, cvar_t};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t, msg_t, vm_t};
    use super::g_public_h::{sharedEntity_t};
    extern "C" {
        pub type worldSector_s;
        #[no_mangle]
        pub fn SV_BotInitBotLib();
        #[no_mangle]
        pub fn SV_BotInitCvars();
        #[no_mangle]
        pub static mut sv_banFile: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_strictAuth: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_lanForceRate: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_mapChecksum: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_killserver: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_padPackets: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_showloss: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_reconnectlimit: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_master: [*mut cvar_t; 5];
        #[no_mangle]
        pub static mut sv_allowDownload: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_zombietime: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_timeout: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_fps: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_privatePassword: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_rconPassword: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_voip: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_voipProtocol: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_pure: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_serverid: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_floodProtect: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_maxPing: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_minPing: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_dlRate: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_maxRate: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_minRate: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_maxclients: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_hostname: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_privateClients: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_mapname: *mut cvar_t;
        #[no_mangle]
        pub static mut sv_gametype: *mut cvar_t;
        #[no_mangle]
        pub fn SV_AddOperatorCommands();
        //=============================================================================
        // persistant server info across maps
        #[no_mangle]
        pub static mut svs: serverStatic_t;
        #[no_mangle]
        pub fn SV_FreeClient(client: *mut client_t);
        // cleared each map
        #[no_mangle]
        pub static mut sv: server_t;
        #[no_mangle]
        pub fn SV_ShutdownGameProgs();
        #[no_mangle]
        pub fn SV_MasterShutdown();
        #[no_mangle]
        pub fn SV_RemoveOperatorCommands();
        #[no_mangle]
        pub fn SV_SendClientSnapshot(client: *mut client_t);
        // game virtual machine
        #[no_mangle]
        pub static mut gvm: *mut vm_t;
        //
// sv_ccmds.c
//
        #[no_mangle]
        pub fn SV_Heartbeat_f();
        //
// sv_bot.c
//
        #[no_mangle]
        pub fn SV_BotFrame(time: libc::c_int);
        #[no_mangle]
        pub fn SV_GentityNum(num: libc::c_int) -> *mut sharedEntity_t;
        #[no_mangle]
        pub fn SV_DropClient(drop_0: *mut client_t,
                             reason: *const libc::c_char);
        #[no_mangle]
        pub fn SV_InitGameProgs();
        //============================================================
//
// high level object sorting to reduce interaction tests
//
        #[no_mangle]
        pub fn SV_ClearWorld();
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/game/g_public.h"]
pub mod g_public_h {
    // the server looks at a sharedEntity, which is the start of the game's gentity_t structure
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sharedEntity_t {
        pub s: entityState_t,
        pub r: entityShared_t,
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
    //
// functions exported by the game subsystem
//
    pub type unnamed_0 = libc::c_uint;
    // ConsoleCommand will be called when a command has been issued
	// that is not recognized as a builtin function.
	// The game can issue trap_argc() / trap_argv() commands to get the command
	// and parameters.  Return qfalse if the game doesn't recognize it as a command.
    // ( int time );
    pub const BOTAI_START_FRAME: unnamed_0 = 10;
    // ( void );
    pub const GAME_CONSOLE_COMMAND: unnamed_0 = 9;
    // ( int levelTime );
    pub const GAME_RUN_FRAME: unnamed_0 = 8;
    // ( int clientNum );
    pub const GAME_CLIENT_THINK: unnamed_0 = 7;
    // ( int clientNum );
    pub const GAME_CLIENT_COMMAND: unnamed_0 = 6;
    // ( int clientNum );
    pub const GAME_CLIENT_DISCONNECT: unnamed_0 = 5;
    // ( int clientNum );
    pub const GAME_CLIENT_USERINFO_CHANGED: unnamed_0 = 4;
    // return NULL if the client is allowed to connect, otherwise return
	// a text string with the reason for denial
    // ( int clientNum );
    pub const GAME_CLIENT_BEGIN: unnamed_0 = 3;
    // ( int clientNum, qboolean firstTime, qboolean isBot );
    pub const GAME_CLIENT_CONNECT: unnamed_0 = 2;
    // init and shutdown will be called every single level
	// The game should call G_GET_ENTITY_TOKEN to parse through all the
	// entity configuration text and spawn gentities.
    // (void);
    pub const GAME_SHUTDOWN: unnamed_0 = 1;
    // ( int levelTime, int randomSeed, int restart );
    pub const GAME_INIT: unnamed_0 = 0;
    use super::q_shared_h::{entityState_t, qboolean, vec3_t};
    use super::{libc};
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn rand() -> libc::c_int;
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/cm_public.h"]
pub mod cm_public_h {
    use super::{libc};
    use super::q_shared_h::{qboolean};
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
        #[no_mangle]
        pub fn CM_LoadMap(name: *const libc::c_char, clientload: qboolean,
                          checksum: *mut libc::c_int);
        #[no_mangle]
        pub fn CM_ClearMap();
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/server/sv_init.c"]
pub mod sv_init_c { }
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/server/sv_variadic.h"]
pub mod sv_variadic_h {
    use super::server_h::{client_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn SV_SendServerCommand(cl: *mut client_t,
                                    fmt: *const libc::c_char, ...);
    }
}
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, unnamed,
                       ERR_NEED_CD, ERR_DISCONNECT, ERR_SERVERDISCONNECT,
                       ERR_DROP, ERR_FATAL, ha_pref, h_dontcare, h_low,
                       h_high, vec_t, vec3_t, cvar_s, cvar_t, playerState_s,
                       playerState_t, usercmd_s, usercmd_t, trType_t,
                       TR_GRAVITY, TR_SINE, TR_LINEAR_STOP, TR_LINEAR,
                       TR_INTERPOLATE, TR_STATIONARY, trajectory_t,
                       entityState_s, entityState_t, Hunk_AllocDebug,
                       Q_strncpyz, va, Info_ValueForKey, Com_Error,
                       Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      vm_t, vm_s, NET_JoinMulticast6, NET_LeaveMulticast6,
                      VM_Call, VM_ExplicitArgPtr, Cbuf_AddText, Cvar_Get,
                      Cvar_Set, Cvar_VariableValue, Cvar_InfoString,
                      Cvar_InfoString_Big, Cvar_CheckRange,
                      cvar_modifiedFlags, FS_Restart, FS_FOpenFileRead,
                      FS_FCloseFile, FS_LoadedPakNames, FS_LoadedPakChecksums,
                      FS_ReferencedPakNames, FS_ReferencedPakChecksums,
                      FS_ClearPakReferences, CopyString, Com_Milliseconds,
                      com_dedicated, com_sv_running, com_frameTime,
                      com_errorEntered, Z_MallocDebug, Z_Free, Hunk_Clear,
                      Hunk_SetMark, Hunk_AllocateTempMemory,
                      Hunk_FreeTempMemory, CL_Disconnect, CL_MapLoading,
                      CL_ShutdownAll, CL_StartHunkUsers};
use self::server_h::{serverStatic_t, challenge_t, client_t, client_s,
                     voipServerPacket_t, voipServerPacket_s, netchan_buffer_t,
                     netchan_buffer_s, clientSnapshot_t, clientState_t,
                     CS_ACTIVE, CS_PRIMED, CS_CONNECTED, CS_ZOMBIE, CS_FREE,
                     server_t, svEntity_t, svEntity_s, serverState_t, SS_GAME,
                     SS_LOADING, SS_DEAD, worldSector_s, SV_BotInitBotLib,
                     SV_BotInitCvars, sv_banFile, sv_strictAuth,
                     sv_lanForceRate, sv_mapChecksum, sv_killserver,
                     sv_padPackets, sv_showloss, sv_reconnectlimit, sv_master,
                     sv_allowDownload, sv_zombietime, sv_timeout, sv_fps,
                     sv_privatePassword, sv_rconPassword, sv_voip,
                     sv_voipProtocol, sv_pure, sv_serverid, sv_floodProtect,
                     sv_maxPing, sv_minPing, sv_dlRate, sv_maxRate,
                     sv_minRate, sv_maxclients, sv_hostname,
                     sv_privateClients, sv_mapname, sv_gametype,
                     SV_AddOperatorCommands, svs, SV_FreeClient, sv,
                     SV_ShutdownGameProgs, SV_MasterShutdown,
                     SV_RemoveOperatorCommands, SV_SendClientSnapshot, gvm,
                     SV_Heartbeat_f, SV_BotFrame, SV_GentityNum,
                     SV_DropClient, SV_InitGameProgs, SV_ClearWorld};
use self::g_public_h::{sharedEntity_t, entityShared_t, unnamed_0,
                       BOTAI_START_FRAME, GAME_CONSOLE_COMMAND,
                       GAME_RUN_FRAME, GAME_CLIENT_THINK, GAME_CLIENT_COMMAND,
                       GAME_CLIENT_DISCONNECT, GAME_CLIENT_USERINFO_CHANGED,
                       GAME_CLIENT_BEGIN, GAME_CLIENT_CONNECT, GAME_SHUTDOWN,
                       GAME_INIT};
use self::string_h::{memset, strcmp, strlen};
use self::stdlib_h::{rand};
use self::cm_public_h::{CM_LoadMap, CM_ClearMap};
use self::sv_variadic_h::{SV_SendServerCommand};
// AVI files have the start of pixel lines 4 byte-aligned
//
// server interface
//
#[no_mangle]
pub unsafe extern "C" fn SV_Init() {
    let mut index: libc::c_int = 0;
    SV_AddOperatorCommands();
    Cvar_Get(b"dmflags\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x4i32);
    Cvar_Get(b"fraglimit\x00" as *const u8 as *const libc::c_char,
             b"20\x00" as *const u8 as *const libc::c_char, 0x4i32);
    Cvar_Get(b"timelimit\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char, 0x4i32);
    sv_gametype =
        Cvar_Get(b"g_gametype\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0x4i32 | 0x20i32);
    Cvar_Get(b"sv_keywords\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x4i32);
    sv_mapname =
        Cvar_Get(b"mapname\x00" as *const u8 as *const libc::c_char,
                 b"nomap\x00" as *const u8 as *const libc::c_char,
                 0x4i32 | 0x40i32);
    sv_privateClients =
        Cvar_Get(b"sv_privateClients\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x4i32);
    sv_hostname =
        Cvar_Get(b"sv_hostname\x00" as *const u8 as *const libc::c_char,
                 b"noname\x00" as *const u8 as *const libc::c_char,
                 0x4i32 | 0x1i32);
    sv_maxclients =
        Cvar_Get(b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
                 b"8\x00" as *const u8 as *const libc::c_char,
                 0x4i32 | 0x20i32);
    sv_minRate =
        Cvar_Get(b"sv_minRate\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x4i32);
    sv_maxRate =
        Cvar_Get(b"sv_maxRate\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x4i32);
    sv_dlRate =
        Cvar_Get(b"sv_dlRate\x00" as *const u8 as *const libc::c_char,
                 b"100\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x4i32);
    sv_minPing =
        Cvar_Get(b"sv_minPing\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x4i32);
    sv_maxPing =
        Cvar_Get(b"sv_maxPing\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x4i32);
    sv_floodProtect =
        Cvar_Get(b"sv_floodProtect\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 0x1i32 | 0x4i32);
    Cvar_Get(b"sv_cheats\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char, 0x8i32 | 0x40i32);
    sv_serverid =
        Cvar_Get(b"sv_serverid\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0x8i32 | 0x40i32);
    sv_pure =
        Cvar_Get(b"sv_pure\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x8i32);
    sv_voip =
        Cvar_Get(b"sv_voip\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x20i32);
    Cvar_CheckRange(sv_voip, 0i32 as libc::c_float, 1i32 as libc::c_float,
                    qtrue);
    sv_voipProtocol =
        Cvar_Get(b"sv_voipProtocol\x00" as *const u8 as *const libc::c_char,
                 if 0 != (*sv_voip).integer {
                     b"opus\x00" as *const u8 as *const libc::c_char
                 } else { b"\x00" as *const u8 as *const libc::c_char },
                 0x8i32 | 0x40i32);
    Cvar_Get(b"sv_paks\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x8i32 | 0x40i32);
    Cvar_Get(b"sv_pakNames\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x8i32 | 0x40i32);
    Cvar_Get(b"sv_referencedPaks\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x8i32 | 0x40i32);
    Cvar_Get(b"sv_referencedPakNames\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x8i32 | 0x40i32);
    sv_rconPassword =
        Cvar_Get(b"rconPassword\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0x100i32);
    sv_privatePassword =
        Cvar_Get(b"sv_privatePassword\x00" as *const u8 as
                     *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0x100i32);
    sv_fps =
        Cvar_Get(b"sv_fps\x00" as *const u8 as *const libc::c_char,
                 b"20\x00" as *const u8 as *const libc::c_char, 0x100i32);
    sv_timeout =
        Cvar_Get(b"sv_timeout\x00" as *const u8 as *const libc::c_char,
                 b"200\x00" as *const u8 as *const libc::c_char, 0x100i32);
    sv_zombietime =
        Cvar_Get(b"sv_zombietime\x00" as *const u8 as *const libc::c_char,
                 b"2\x00" as *const u8 as *const libc::c_char, 0x100i32);
    Cvar_Get(b"nextmap\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x100i32);
    sv_allowDownload =
        Cvar_Get(b"sv_allowDownload\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x4i32);
    Cvar_Get(b"sv_dlURL\x00" as *const u8 as *const libc::c_char,
             b"\x00" as *const u8 as *const libc::c_char, 0x4i32 | 0x1i32);
    sv_master[0usize] =
        Cvar_Get(b"sv_master1\x00" as *const u8 as *const libc::c_char,
                 b"master.quake3arena.com\x00" as *const u8 as
                     *const libc::c_char, 0i32);
    sv_master[1usize] =
        Cvar_Get(b"sv_master2\x00" as *const u8 as *const libc::c_char,
                 b"master.ioquake3.org\x00" as *const u8 as
                     *const libc::c_char, 0i32);
    index = 2i32;
    while index < 5i32 {
        sv_master[index as usize] =
            Cvar_Get(va(b"sv_master%d\x00" as *const u8 as *const libc::c_char
                            as *mut libc::c_char, index + 1i32),
                     b"\x00" as *const u8 as *const libc::c_char, 0x1i32);
        index += 1
    }
    sv_reconnectlimit =
        Cvar_Get(b"sv_reconnectlimit\x00" as *const u8 as *const libc::c_char,
                 b"3\x00" as *const u8 as *const libc::c_char, 0i32);
    sv_showloss =
        Cvar_Get(b"sv_showloss\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    sv_padPackets =
        Cvar_Get(b"sv_padPackets\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    sv_killserver =
        Cvar_Get(b"sv_killserver\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0i32);
    sv_mapChecksum =
        Cvar_Get(b"sv_mapChecksum\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0x40i32);
    sv_lanForceRate =
        Cvar_Get(b"sv_lanForceRate\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    sv_strictAuth =
        Cvar_Get(b"sv_strictAuth\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char, 0x1i32);
    sv_banFile =
        Cvar_Get(b"sv_banFile\x00" as *const u8 as *const libc::c_char,
                 b"serverbans.dat\x00" as *const u8 as *const libc::c_char,
                 0x1i32);
    SV_BotInitCvars();
    SV_BotInitBotLib();
    Cbuf_AddText(b"rehashbans\n\x00" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn SV_Shutdown(mut finalmsg: *mut libc::c_char) {
    if com_sv_running.is_null() || 0 == (*com_sv_running).integer { return }
    Com_Printf(b"----- Server Shutdown (%s) -----\n\x00" as *const u8 as
                   *const libc::c_char, finalmsg);
    NET_LeaveMulticast6();
    if !svs.clients.is_null() && 0 == com_errorEntered as u64 {
        SV_FinalMessage(finalmsg);
    }
    SV_RemoveOperatorCommands();
    SV_MasterShutdown();
    SV_ShutdownGameProgs();
    SV_ClearServer();
    if !svs.clients.is_null() {
        let mut index: libc::c_int = 0;
        index = 0i32;
        while index < (*sv_maxclients).integer {
            SV_FreeClient(&mut *svs.clients.offset(index as isize));
            index += 1
        }
        Z_Free(svs.clients as *mut libc::c_void);
    }
    memset(&mut svs as *mut serverStatic_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<serverStatic_t>() as libc::c_ulong);
    Cvar_Set(b"sv_running\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    Cvar_Set(b"ui_singlePlayerActive\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    Com_Printf(b"---------------------------\n\x00" as *const u8 as
                   *const libc::c_char);
    if (*sv_killserver).integer != 2i32 { CL_Disconnect(qfalse); };
}
/*
================
SV_ClearServer
================
*/
unsafe extern "C" fn SV_ClearServer() {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 1024i32 {
        if !sv.configstrings[i as usize].is_null() {
            Z_Free(sv.configstrings[i as usize] as *mut libc::c_void);
        }
        i += 1
    }
    memset(&mut sv as *mut server_t as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<server_t>() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn SV_FinalMessage(mut message: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut cl: *mut client_t = 0 as *mut client_t;
    j = 0i32;
    while j < 2i32 {
        i = 0i32;
        cl = svs.clients;
        while i < (*sv_maxclients).integer {
            if (*cl).state as libc::c_uint >=
                   CS_CONNECTED as libc::c_int as libc::c_uint {
                if (*cl).netchan.remoteAddress.type_0 as libc::c_uint !=
                       NA_LOOPBACK as libc::c_int as libc::c_uint {
                    SV_SendServerCommand(cl,
                                         b"print \"%s\n\"\n\x00" as *const u8
                                             as *const libc::c_char, message);
                    SV_SendServerCommand(cl,
                                         b"disconnect \"%s\"\x00" as *const u8
                                             as *const libc::c_char, message);
                }
                (*cl).lastSnapshotTime = 0i32;
                SV_SendClientSnapshot(cl);
            }
            i += 1;
            cl = cl.offset(1isize)
        }
        j += 1
    };
}
//
// sv_init.c
//
#[no_mangle]
pub unsafe extern "C" fn SV_SetConfigstring(mut index: libc::c_int,
                                            mut val: *const libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut client: *mut client_t = 0 as *mut client_t;
    if index < 0i32 || index >= 1024i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_SetConfigstring: bad index %i\x00" as *const u8 as
                      *const libc::c_char, index);
    }
    if val.is_null() { val = b"\x00" as *const u8 as *const libc::c_char }
    if 0 == strcmp(val, sv.configstrings[index as usize]) { return }
    Z_Free(sv.configstrings[index as usize] as *mut libc::c_void);
    sv.configstrings[index as usize] = CopyString(val);
    if sv.state as libc::c_uint == SS_GAME as libc::c_int as libc::c_uint ||
           0 != sv.restarting as libc::c_uint {
        i = 0i32;
        client = svs.clients;
        while i < (*sv_maxclients).integer {
            if ((*client).state as libc::c_uint) <
                   CS_ACTIVE as libc::c_int as libc::c_uint {
                if (*client).state as libc::c_uint ==
                       CS_PRIMED as libc::c_int as libc::c_uint {
                    (*client).csUpdated[index as usize] = qtrue
                }
            } else if !(index == 0i32 && !(*client).gentity.is_null() &&
                            0 != (*(*client).gentity).r.svFlags & 0x200i32) {
                SV_SendConfigstring(client, index);
            }
            i += 1;
            client = client.offset(1isize)
        }
    };
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
/*
===============
SV_SendConfigstring

Creates and sends the server command necessary to update the CS index for the
given client
===============
*/
unsafe extern "C" fn SV_SendConfigstring(mut client: *mut client_t,
                                         mut index: libc::c_int) {
    let mut maxChunkSize: libc::c_int = 1024i32 - 24i32;
    let mut len: libc::c_int = 0;
    len = strlen(sv.configstrings[index as usize]) as libc::c_int;
    if len >= maxChunkSize {
        let mut sent: libc::c_int = 0i32;
        let mut remaining: libc::c_int = len;
        let mut cmd: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut buf: [libc::c_char; 1024] = [0; 1024];
        while remaining > 0i32 {
            if sent == 0i32 {
                cmd =
                    b"bcs0\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            } else if remaining < maxChunkSize {
                cmd =
                    b"bcs2\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            } else {
                cmd =
                    b"bcs1\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char
            }
            Q_strncpyz(buf.as_mut_ptr(),
                       &mut *sv.configstrings[index as
                                                  usize].offset(sent as
                                                                    isize),
                       maxChunkSize);
            SV_SendServerCommand(client,
                                 b"%s %i \"%s\"\n\x00" as *const u8 as
                                     *const libc::c_char, cmd, index,
                                 buf.as_mut_ptr());
            sent += maxChunkSize - 1i32;
            remaining -= maxChunkSize - 1i32
        }
    } else {
        SV_SendServerCommand(client,
                             b"cs %i \"%s\"\n\x00" as *const u8 as
                                 *const libc::c_char, index,
                             sv.configstrings[index as usize]);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_GetConfigstring(mut index: libc::c_int,
                                            mut buffer: *mut libc::c_char,
                                            mut bufferSize: libc::c_int) {
    if bufferSize < 1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_GetConfigstring: bufferSize == %i\x00" as *const u8 as
                      *const libc::c_char, bufferSize);
    }
    if index < 0i32 || index >= 1024i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_GetConfigstring: bad index %i\x00" as *const u8 as
                      *const libc::c_char, index);
    }
    if sv.configstrings[index as usize].is_null() {
        *buffer.offset(0isize) = 0i32 as libc::c_char;
        return
    }
    Q_strncpyz(buffer, sv.configstrings[index as usize], bufferSize);
}
#[no_mangle]
pub unsafe extern "C" fn SV_UpdateConfigstrings(mut client: *mut client_t) {
    let mut index: libc::c_int = 0;
    index = 0i32;
    while index < 1024i32 {
        // if the CS hasn't changed since we went to CS_PRIMED, ignore
        if !(0 == (*client).csUpdated[index as usize] as u64) {
            // do not always send server info to all clients
            if !(index == 0i32 && !(*client).gentity.is_null() &&
                     0 != (*(*client).gentity).r.svFlags & 0x200i32) {
                SV_SendConfigstring(client, index);
                (*client).csUpdated[index as usize] = qfalse
            }
        }
        index += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_SetUserinfo(mut index: libc::c_int,
                                        mut val: *const libc::c_char) {
    if index < 0i32 || index >= (*sv_maxclients).integer {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_SetUserinfo: bad index %i\x00" as *const u8 as
                      *const libc::c_char, index);
    }
    if val.is_null() { val = b"\x00" as *const u8 as *const libc::c_char }
    Q_strncpyz((*svs.clients.offset(index as isize)).userinfo.as_mut_ptr(),
               val,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz((*svs.clients.offset(index as isize)).name.as_mut_ptr(),
               Info_ValueForKey(val,
                                b"name\x00" as *const u8 as
                                    *const libc::c_char),
               ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as
                   libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn SV_GetUserinfo(mut index: libc::c_int,
                                        mut buffer: *mut libc::c_char,
                                        mut bufferSize: libc::c_int) {
    if bufferSize < 1i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_GetUserinfo: bufferSize == %i\x00" as *const u8 as
                      *const libc::c_char, bufferSize);
    }
    if index < 0i32 || index >= (*sv_maxclients).integer {
        Com_Error(ERR_DROP as libc::c_int,
                  b"SV_GetUserinfo: bad index %i\x00" as *const u8 as
                      *const libc::c_char, index);
    }
    Q_strncpyz(buffer,
               (*svs.clients.offset(index as isize)).userinfo.as_mut_ptr(),
               bufferSize);
}
#[no_mangle]
pub unsafe extern "C" fn SV_ChangeMaxClients() {
    let mut oldMaxClients: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut oldClients: *mut client_t = 0 as *mut client_t;
    let mut count: libc::c_int = 0;
    count = 0i32;
    i = 0i32;
    while i < (*sv_maxclients).integer {
        if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
               CS_CONNECTED as libc::c_int as libc::c_uint {
            if i > count { count = i }
        }
        i += 1
    }
    count += 1;
    oldMaxClients = (*sv_maxclients).integer;
    SV_BoundMaxClients(count);
    if (*sv_maxclients).integer == oldMaxClients { return }
    oldClients =
        Hunk_AllocateTempMemory((count as
                                     libc::c_ulong).wrapping_mul(::std::mem::size_of::<client_t>()
                                                                     as
                                                                     libc::c_ulong)
                                    as libc::c_int) as *mut client_t;
    i = 0i32;
    while i < count {
        if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
               CS_CONNECTED as libc::c_int as libc::c_uint {
            *oldClients.offset(i as isize) = *svs.clients.offset(i as isize)
        } else {
            memset(&mut *oldClients.offset(i as isize) as *mut client_t as
                       *mut libc::c_void, 0i32,
                   ::std::mem::size_of::<client_t>() as libc::c_ulong);
        }
        i += 1
    }
    Z_Free(svs.clients as *mut libc::c_void);
    svs.clients =
        Z_MallocDebug(((*sv_maxclients).integer as
                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<client_t>()
                                                           as libc::c_ulong)
                          as libc::c_int,
                      b"sv_maxclients->integer * sizeof(client_t)\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/server/sv_init.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 338i32) as
            *mut client_t;
    memset(svs.clients as *mut libc::c_void, 0i32,
           ((*sv_maxclients).integer as
                libc::c_ulong).wrapping_mul(::std::mem::size_of::<client_t>()
                                                as libc::c_ulong));
    i = 0i32;
    while i < count {
        if (*oldClients.offset(i as isize)).state as libc::c_uint >=
               CS_CONNECTED as libc::c_int as libc::c_uint {
            *svs.clients.offset(i as isize) = *oldClients.offset(i as isize)
        }
        i += 1
    }
    Hunk_FreeTempMemory(oldClients as *mut libc::c_void);
    if 0 != (*com_dedicated).integer {
        svs.numSnapshotEntities = (*sv_maxclients).integer * 32i32 * 256i32
    } else {
        svs.numSnapshotEntities = (*sv_maxclients).integer * 4i32 * 256i32
    };
}
/*
===============
SV_BoundMaxClients

===============
*/
unsafe extern "C" fn SV_BoundMaxClients(mut minimum: libc::c_int) {
    Cvar_Get(b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
             b"8\x00" as *const u8 as *const libc::c_char, 0i32);
    (*sv_maxclients).modified = qfalse;
    if (*sv_maxclients).integer < minimum {
        Cvar_Set(b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, minimum));
    } else if (*sv_maxclients).integer > 64i32 {
        Cvar_Set(b"sv_maxclients\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, 64i32));
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_SpawnServer(mut server: *mut libc::c_char,
                                        mut killBots: qboolean) {
    let mut i: libc::c_int = 0;
    let mut checksum: libc::c_int = 0;
    let mut isBot: qboolean = qfalse;
    let mut systemInfo: [libc::c_char; 16384] = [0; 16384];
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    SV_ShutdownGameProgs();
    Com_Printf(b"------ Server Initialization ------\n\x00" as *const u8 as
                   *const libc::c_char);
    Com_Printf(b"Server: %s\n\x00" as *const u8 as *const libc::c_char,
               server);
    CL_MapLoading();
    CL_ShutdownAll(qfalse);
    Hunk_Clear();
    CM_ClearMap();
    if 0. ==
           Cvar_VariableValue(b"sv_running\x00" as *const u8 as
                                  *const libc::c_char) {
        SV_Startup();
    } else if 0 != (*sv_maxclients).modified as u64 { SV_ChangeMaxClients(); }
    FS_ClearPakReferences(0i32);
    svs.snapshotEntities =
        Hunk_AllocDebug((::std::mem::size_of::<entityState_t>() as
                             libc::c_ulong).wrapping_mul(svs.numSnapshotEntities
                                                             as libc::c_ulong)
                            as libc::c_int, h_high,
                        b"sizeof(entityState_t)*svs.numSnapshotEntities\x00"
                            as *const u8 as *const libc::c_char as
                            *mut libc::c_char,
                        b"code/server/sv_init.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 439i32)
            as *mut entityState_t;
    svs.nextSnapshotEntities = 0i32;
    svs.snapFlagServerBit ^= 4i32;
    Cvar_Set(b"nextmap\x00" as *const u8 as *const libc::c_char,
             b"map_restart 0\x00" as *const u8 as *const libc::c_char);
    i = 0i32;
    while i < (*sv_maxclients).integer {
        if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
               CS_CONNECTED as libc::c_int as libc::c_uint {
            (*svs.clients.offset(i as isize)).oldServerTime = sv.time
        }
        i += 1
    }
    SV_ClearServer();
    i = 0i32;
    while i < 1024i32 {
        sv.configstrings[i as usize] =
            CopyString(b"\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
    Cvar_Set(b"cl_paused\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    sv.checksumFeed =
        ((rand() as libc::c_uint) << 16i32 ^ rand() as libc::c_uint ^
             Com_Milliseconds() as libc::c_uint) as libc::c_int;
    FS_Restart(sv.checksumFeed);
    CM_LoadMap(va(b"maps/%s.bsp\x00" as *const u8 as *const libc::c_char as
                      *mut libc::c_char, server), qfalse, &mut checksum);
    Cvar_Set(b"mapname\x00" as *const u8 as *const libc::c_char, server);
    Cvar_Set(b"sv_mapChecksum\x00" as *const u8 as *const libc::c_char,
             va(b"%i\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char, checksum));
    sv.serverId = com_frameTime;
    sv.restartedServerId = sv.serverId;
    sv.checksumFeedServerId = sv.serverId;
    Cvar_Set(b"sv_serverid\x00" as *const u8 as *const libc::c_char,
             va(b"%i\x00" as *const u8 as *const libc::c_char as
                    *mut libc::c_char, sv.serverId));
    SV_ClearWorld();
    sv.state = SS_LOADING;
    SV_InitGameProgs();
    (*sv_gametype).modified = qfalse;
    i = 0i32;
    while i < 3i32 {
        VM_Call(gvm, GAME_RUN_FRAME as libc::c_int, sv.time);
        SV_BotFrame(sv.time);
        sv.time += 100i32;
        svs.time += 100i32;
        i += 1
    }
    SV_CreateBaseline();
    let mut current_block_73: u64;
    i = 0i32;
    while i < (*sv_maxclients).integer {
        // send the new gamestate to all connected clients
        if (*svs.clients.offset(i as isize)).state as libc::c_uint >=
               CS_CONNECTED as libc::c_int as libc::c_uint {
            let mut denied: *mut libc::c_char = 0 as *mut libc::c_char;
            if (*svs.clients.offset(i as isize)).netchan.remoteAddress.type_0
                   as libc::c_uint == NA_BOT as libc::c_int as libc::c_uint {
                if 0 != killBots as u64 {
                    SV_DropClient(&mut *svs.clients.offset(i as isize),
                                  b"\x00" as *const u8 as
                                      *const libc::c_char);
                    current_block_73 = 13460095289871124136;
                } else {
                    isBot = qtrue;
                    current_block_73 = 6560072651652764009;
                }
            } else { isBot = qfalse; current_block_73 = 6560072651652764009; }
            match current_block_73 {
                13460095289871124136 => { }
                _ => {
                    denied =
                        VM_ExplicitArgPtr(gvm,
                                          VM_Call(gvm,
                                                  GAME_CLIENT_CONNECT as
                                                      libc::c_int, i,
                                                  qfalse as libc::c_int,
                                                  isBot as libc::c_uint)) as
                            *mut libc::c_char;
                    if !denied.is_null() {
                        SV_DropClient(&mut *svs.clients.offset(i as isize),
                                      denied);
                    } else if 0 == isBot as u64 {
                        (*svs.clients.offset(i as isize)).state = CS_CONNECTED
                    } else {
                        let mut client: *mut client_t = 0 as *mut client_t;
                        let mut ent: *mut sharedEntity_t =
                            0 as *mut sharedEntity_t;
                        client =
                            &mut *svs.clients.offset(i as isize) as
                                *mut client_t;
                        (*client).state = CS_ACTIVE;
                        ent = SV_GentityNum(i);
                        (*ent).s.number = i;
                        (*client).gentity = ent;
                        (*client).deltaMessage = -1i32;
                        (*client).lastSnapshotTime = 0i32;
                        VM_Call(gvm, GAME_CLIENT_BEGIN as libc::c_int, i);
                    }
                }
            }
        }
        i += 1
    }
    VM_Call(gvm, GAME_RUN_FRAME as libc::c_int, sv.time);
    SV_BotFrame(sv.time);
    sv.time += 100i32;
    svs.time += 100i32;
    if 0 != (*sv_pure).integer {
        p = FS_LoadedPakChecksums();
        Cvar_Set(b"sv_paks\x00" as *const u8 as *const libc::c_char, p);
        if strlen(p) == 0i32 as libc::c_ulong {
            Com_Printf(b"WARNING: sv_pure set but no PK3 files loaded\n\x00"
                           as *const u8 as *const libc::c_char);
        }
        p = FS_LoadedPakNames();
        Cvar_Set(b"sv_pakNames\x00" as *const u8 as *const libc::c_char, p);
        SV_TouchFile(b"vm/cgame.qvm\x00" as *const u8 as *const libc::c_char);
        SV_TouchFile(b"vm/ui.qvm\x00" as *const u8 as *const libc::c_char);
    } else {
        Cvar_Set(b"sv_paks\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char);
        Cvar_Set(b"sv_pakNames\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char);
    }
    p = FS_ReferencedPakChecksums();
    Cvar_Set(b"sv_referencedPaks\x00" as *const u8 as *const libc::c_char, p);
    p = FS_ReferencedPakNames();
    Cvar_Set(b"sv_referencedPakNames\x00" as *const u8 as *const libc::c_char,
             p);
    Q_strncpyz(systemInfo.as_mut_ptr(), Cvar_InfoString_Big(0x8i32),
               ::std::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong
                   as libc::c_int);
    cvar_modifiedFlags &= !0x8i32;
    SV_SetConfigstring(1i32, systemInfo.as_mut_ptr());
    SV_SetConfigstring(0i32, Cvar_InfoString(0x4i32));
    cvar_modifiedFlags &= !0x4i32;
    sv.state = SS_GAME;
    SV_Heartbeat_f();
    Hunk_SetMark();
    if 0 != (*com_dedicated).integer { CL_StartHunkUsers(qtrue); }
    Com_Printf(b"-----------------------------------\n\x00" as *const u8 as
                   *const libc::c_char);
}
/*
================
SV_TouchFile
================
*/
unsafe extern "C" fn SV_TouchFile(mut filename: *const libc::c_char) {
    let mut f: fileHandle_t = 0;
    FS_FOpenFileRead(filename, &mut f, qfalse);
    if 0 != f { FS_FCloseFile(f); };
}
/*
================
SV_CreateBaseline

Entity baselines are used to compress non-delta messages
to the clients -- only the fields that differ from the
baseline will be transmitted
================
*/
unsafe extern "C" fn SV_CreateBaseline() {
    let mut svent: *mut sharedEntity_t = 0 as *mut sharedEntity_t;
    let mut entnum: libc::c_int = 0;
    entnum = 1i32;
    while entnum < sv.num_entities {
        svent = SV_GentityNum(entnum);
        if !(0 == (*svent).r.linked as u64) {
            (*svent).s.number = entnum;
            sv.svEntities[entnum as usize].baseline = (*svent).s
        }
        entnum += 1
    };
}
/*
===============
SV_Startup

Called when a host starts a map when it wasn't running
one before.  Successive map or map_restart commands will
NOT cause this to be called, unless the game is exited to
the menu system first.
===============
*/
unsafe extern "C" fn SV_Startup() {
    if 0 != svs.initialized as u64 {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"SV_Startup: svs.initialized\x00" as *const u8 as
                      *const libc::c_char);
    }
    SV_BoundMaxClients(1i32);
    svs.clients =
        Z_MallocDebug((::std::mem::size_of::<client_t>() as
                           libc::c_ulong).wrapping_mul((*sv_maxclients).integer
                                                           as libc::c_ulong)
                          as libc::c_int,
                      b"sizeof(client_t) * sv_maxclients->integer\x00" as
                          *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/server/sv_init.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 273i32) as
            *mut client_t;
    if 0 != (*com_dedicated).integer {
        svs.numSnapshotEntities = (*sv_maxclients).integer * 32i32 * 256i32
    } else {
        svs.numSnapshotEntities = (*sv_maxclients).integer * 4i32 * 256i32
    }
    svs.initialized = qtrue;
    if 0 != (*sv_killserver).integer {
        Cvar_Set(b"sv_killserver\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char);
    }
    Cvar_Set(b"sv_running\x00" as *const u8 as *const libc::c_char,
             b"1\x00" as *const u8 as *const libc::c_char);
    NET_JoinMulticast6();
}