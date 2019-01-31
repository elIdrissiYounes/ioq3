use libc;
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
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
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
    // override on command line, config files etc.
    // broadcast scan this many ports after
    // PORT_SERVER so a single machine can
									// run multiple servers
    // the svc_strings[] array in cl_parse.c should mirror this
//
// server to client
//
    pub type svc_ops_e = libc::c_uint;
    //
    pub const svc_voipOpus: svc_ops_e = 10;
    // new commands, supported only by ioquake3 protocol but not legacy
    // not wrapped in USE_VOIP, so this value is reserved.
    pub const svc_voipSpeex: svc_ops_e = 9;
    pub const svc_EOF: svc_ops_e = 8;
    pub const svc_snapshot: svc_ops_e = 7;
    // [short] size [size bytes]
    pub const svc_download: svc_ops_e = 6;
    // [string] to be executed by client game module
    pub const svc_serverCommand: svc_ops_e = 5;
    // only in gamestate messages
    pub const svc_baseline: svc_ops_e = 4;
    // [short] [string] only in gamestate messages
    pub const svc_configstring: svc_ops_e = 3;
    pub const svc_gamestate: svc_ops_e = 2;
    pub const svc_nop: svc_ops_e = 1;
    pub const svc_bad: svc_ops_e = 0;
    use super::q_shared_h::{qboolean, byte};
    use super::{libc};
    extern "C" {
        // TTimo
// copy a msg_t in case we need to store it as is for a bit
// (as I needed this to keep an msg_t from a static var for later use)
// sets data buffer as MSG_Init does prior to do the copy
        #[no_mangle]
        pub fn MSG_Copy(buf: *mut msg_t, data: *mut byte, length: libc::c_int,
                        src: *mut msg_t);
        #[no_mangle]
        pub fn MSG_WriteByte(sb: *mut msg_t, c: libc::c_int);
        #[no_mangle]
        pub fn MSG_ReadLong(sb: *mut msg_t) -> libc::c_int;
        #[no_mangle]
        pub fn Netchan_Transmit(chan: *mut netchan_t, length: libc::c_int,
                                data: *const byte);
        #[no_mangle]
        pub fn Netchan_TransmitNextFragment(chan: *mut netchan_t);
        #[no_mangle]
        pub fn Netchan_Process(chan: *mut netchan_t, msg: *mut msg_t)
         -> qboolean;
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
    }
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
    // the server looks at a sharedEntity, which is the start of the game's gentity_t structure
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sharedEntity_t {
        pub s: entityState_t,
        pub r: entityShared_t,
    }
    use super::q_shared_h::{entityState_t, qboolean, vec3_t};
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/server/server.h"]
pub mod server_h {
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
    pub type voipServerPacket_t = voipServerPacket_s;
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
    pub struct netchan_buffer_s {
        pub msg: msg_t,
        pub msgBuffer: [byte; 16384],
        pub clientCommandString: [libc::c_char; 1024],
        pub next: *mut netchan_buffer_s,
    }
    pub type netchan_buffer_t = netchan_buffer_s;
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
    pub type client_t = client_s;
    use super::{libc};
    use super::q_shared_h::{byte, playerState_t, usercmd_t, fileHandle_t,
                            qboolean};
    use super::qcommon_h::{msg_t, netchan_t};
    use super::g_public_h::{sharedEntity_t};
    extern "C" {
        #[no_mangle]
        pub fn SV_RateMsec(client: *mut client_t) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/server/sv_net_chan.c"]
pub mod sv_net_chan_c {
    use super::server_h::{client_t};
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, fileHandle_t, vec_t,
                       vec3_t, playerState_s, playerState_t, usercmd_s,
                       usercmd_t, trType_t, TR_GRAVITY, TR_SINE,
                       TR_LINEAR_STOP, TR_LINEAR, TR_INTERPOLATE,
                       TR_STATIONARY, trajectory_t, entityState_s,
                       entityState_t, Q_strncpyz};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      svc_ops_e, svc_voipOpus, svc_voipSpeex, svc_EOF,
                      svc_snapshot, svc_download, svc_serverCommand,
                      svc_baseline, svc_configstring, svc_gamestate, svc_nop,
                      svc_bad, MSG_Copy, MSG_WriteByte, MSG_ReadLong,
                      Netchan_Transmit, Netchan_TransmitNextFragment,
                      Netchan_Process, Com_DPrintf, Z_MallocDebug, Z_Free};
use self::g_public_h::{entityShared_t, sharedEntity_t};
use self::server_h::{voipServerPacket_s, voipServerPacket_t, clientSnapshot_t,
                     clientState_t, CS_ACTIVE, CS_PRIMED, CS_CONNECTED,
                     CS_ZOMBIE, CS_FREE, netchan_buffer_s, netchan_buffer_t,
                     client_s, client_t, SV_RateMsec};
// clip to a specific entity
//
// sv_net_chan.c
//
#[no_mangle]
pub unsafe extern "C" fn SV_Netchan_Transmit(mut client: *mut client_t,
                                             mut msg: *mut msg_t) {
    MSG_WriteByte(msg, svc_EOF as libc::c_int);
    if 0 != (*client).netchan.unsentFragments as libc::c_uint ||
           !(*client).netchan_start_queue.is_null() {
        let mut netbuf: *mut netchan_buffer_t = 0 as *mut netchan_buffer_t;
        Com_DPrintf(b"#462 SV_Netchan_Transmit: unsent fragments, stacked\n\x00"
                        as *const u8 as *const libc::c_char);
        netbuf =
            Z_MallocDebug(::std::mem::size_of::<netchan_buffer_t>() as
                              libc::c_ulong as libc::c_int,
                          b"sizeof(netchan_buffer_t)\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          b"code/server/sv_net_chan.c\x00" as *const u8 as
                              *const libc::c_char as *mut libc::c_char,
                          232i32) as *mut netchan_buffer_t;
        MSG_Copy(&mut (*netbuf).msg, (*netbuf).msgBuffer.as_mut_ptr(),
                 ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong as
                     libc::c_int, msg);
        if 0 != (*client).compat as u64 {
            Q_strncpyz((*netbuf).clientCommandString.as_mut_ptr(),
                       (*client).lastClientCommandString.as_mut_ptr(),
                       ::std::mem::size_of::<[libc::c_char; 1024]>() as
                           libc::c_ulong as libc::c_int);
        }
        (*netbuf).next = 0 as *mut netchan_buffer_s;
        *(*client).netchan_end_queue = netbuf;
        (*client).netchan_end_queue =
            &mut (**(*client).netchan_end_queue).next
    } else {
        if 0 != (*client).compat as u64 {
            SV_Netchan_Encode(client, msg,
                              (*client).lastClientCommandString.as_mut_ptr());
        }
        Netchan_Transmit(&mut (*client).netchan, (*msg).cursize, (*msg).data);
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
==============
SV_Netchan_Encode

	// first four bytes of the data are always:
	long reliableAcknowledge;

==============
*/
unsafe extern "C" fn SV_Netchan_Encode(mut client: *mut client_t,
                                       mut msg: *mut msg_t,
                                       mut clientCommandString:
                                           *const libc::c_char) {
    let mut i: libc::c_long = 0;
    let mut index: libc::c_long = 0;
    let mut key: byte = 0;
    let mut string: *mut byte = 0 as *mut byte;
    let mut srdc: libc::c_int = 0;
    let mut sbit: libc::c_int = 0;
    let mut soob: qboolean = qfalse;
    if (*msg).cursize < 4i32 { return }
    srdc = (*msg).readcount;
    sbit = (*msg).bit;
    soob = (*msg).oob;
    (*msg).bit = 0i32;
    (*msg).readcount = 0i32;
    (*msg).oob = qfalse;
    MSG_ReadLong(msg);
    (*msg).oob = soob;
    (*msg).bit = sbit;
    (*msg).readcount = srdc;
    string = clientCommandString as *mut byte;
    index = 0i32 as libc::c_long;
    key = ((*client).challenge ^ (*client).netchan.outgoingSequence) as byte;
    i = 4i32 as libc::c_long;
    while i < (*msg).cursize as libc::c_long {
        if 0 == *string.offset(index as isize) {
            index = 0i32 as libc::c_long
        }
        if *string.offset(index as isize) as libc::c_int > 127i32 ||
               *string.offset(index as isize) as libc::c_int == '%' as i32 {
            key =
                (key as libc::c_int ^
                     ('.' as i32) << (i & 1i32 as libc::c_long)) as byte
        } else {
            key =
                (key as libc::c_int ^
                     (*string.offset(index as isize) as libc::c_int) <<
                         (i & 1i32 as libc::c_long)) as byte
        }
        index += 1;
        *(*msg).data.offset(i as isize) =
            (*(*msg).data.offset(i as isize) as libc::c_int ^
                 key as libc::c_int) as byte;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_Netchan_TransmitNextFragment(mut client:
                                                             *mut client_t)
 -> libc::c_int {
    if 0 != (*client).netchan.unsentFragments as u64 {
        Netchan_TransmitNextFragment(&mut (*client).netchan);
        return SV_RateMsec(client)
    } else {
        if !(*client).netchan_start_queue.is_null() {
            SV_Netchan_TransmitNextInQueue(client);
            return SV_RateMsec(client)
        }
    }
    return -1i32;
}
/*
=================
SV_Netchan_TransmitNextInQueue
=================
*/
#[no_mangle]
pub unsafe extern "C" fn SV_Netchan_TransmitNextInQueue(mut client:
                                                            *mut client_t) {
    let mut netbuf: *mut netchan_buffer_t = 0 as *mut netchan_buffer_t;
    Com_DPrintf(b"#462 Netchan_TransmitNextFragment: popping a queued message for transmit\n\x00"
                    as *const u8 as *const libc::c_char);
    netbuf = (*client).netchan_start_queue;
    if 0 != (*client).compat as u64 {
        SV_Netchan_Encode(client, &mut (*netbuf).msg,
                          (*netbuf).clientCommandString.as_mut_ptr());
    }
    Netchan_Transmit(&mut (*client).netchan, (*netbuf).msg.cursize,
                     (*netbuf).msg.data);
    (*client).netchan_start_queue = (*netbuf).next;
    if (*client).netchan_start_queue.is_null() {
        Com_DPrintf(b"#462 Netchan_TransmitNextFragment: emptied queue\n\x00"
                        as *const u8 as *const libc::c_char);
        (*client).netchan_end_queue = &mut (*client).netchan_start_queue
    } else {
        Com_DPrintf(b"#462 Netchan_TransmitNextFragment: remaining queued message\n\x00"
                        as *const u8 as *const libc::c_char);
    }
    Z_Free(netbuf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn SV_Netchan_Process(mut client: *mut client_t,
                                            mut msg: *mut msg_t) -> qboolean {
    let mut ret: libc::c_int = 0;
    ret = Netchan_Process(&mut (*client).netchan, msg) as libc::c_int;
    if 0 == ret { return qfalse }
    if 0 != (*client).compat as u64 { SV_Netchan_Decode(client, msg); }
    return qtrue;
}
/*
==============
SV_Netchan_Decode

	// first 12 bytes of the data are always:
	long serverId;
	long messageAcknowledge;
	long reliableAcknowledge;

==============
*/
unsafe extern "C" fn SV_Netchan_Decode(mut client: *mut client_t,
                                       mut msg: *mut msg_t) {
    let mut serverId: libc::c_int = 0;
    let mut messageAcknowledge: libc::c_int = 0;
    let mut reliableAcknowledge: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut srdc: libc::c_int = 0;
    let mut sbit: libc::c_int = 0;
    let mut soob: qboolean = qfalse;
    let mut key: byte = 0;
    let mut string: *mut byte = 0 as *mut byte;
    srdc = (*msg).readcount;
    sbit = (*msg).bit;
    soob = (*msg).oob;
    (*msg).oob = qfalse;
    serverId = MSG_ReadLong(msg);
    messageAcknowledge = MSG_ReadLong(msg);
    reliableAcknowledge = MSG_ReadLong(msg);
    (*msg).oob = soob;
    (*msg).bit = sbit;
    (*msg).readcount = srdc;
    string =
        (*client).reliableCommands[(reliableAcknowledge & 64i32 - 1i32) as
                                       usize].as_mut_ptr() as *mut byte;
    index = 0i32;
    key = ((*client).challenge ^ serverId ^ messageAcknowledge) as byte;
    i = (*msg).readcount + 12i32;
    while i < (*msg).cursize {
        if 0 == *string.offset(index as isize) { index = 0i32 }
        if *string.offset(index as isize) as libc::c_int > 127i32 ||
               *string.offset(index as isize) as libc::c_int == '%' as i32 {
            key = (key as libc::c_int ^ ('.' as i32) << (i & 1i32)) as byte
        } else {
            key =
                (key as libc::c_int ^
                     (*string.offset(index as isize) as libc::c_int) <<
                         (i & 1i32)) as byte
        }
        index += 1;
        *(*msg).data.offset(i as isize) =
            (*(*msg).data.offset(i as isize) as libc::c_int ^
                 key as libc::c_int) as byte;
        i += 1
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_Netchan_FreeQueue(mut client: *mut client_t) {
    let mut netbuf: *mut netchan_buffer_t = 0 as *mut netchan_buffer_t;
    let mut next: *mut netchan_buffer_t = 0 as *mut netchan_buffer_t;
    netbuf = (*client).netchan_start_queue;
    while !netbuf.is_null() {
        next = (*netbuf).next;
        Z_Free(netbuf as *mut libc::c_void);
        netbuf = next
    }
    (*client).netchan_start_queue = 0 as *mut netchan_buffer_t;
    (*client).netchan_end_queue = &mut (*client).netchan_start_queue;
}