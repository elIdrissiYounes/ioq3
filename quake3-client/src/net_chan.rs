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
    use super::{libc};
    extern "C" {
        // buffer size safe library replacements
        #[no_mangle]
        pub fn Q_strncpyz(dest: *mut libc::c_char, src: *const libc::c_char,
                          destsize: libc::c_int);
        // Count the number of char tocount encountered in string
        #[no_mangle]
        pub fn Q_CountChar(string: *const libc::c_char, tocount: libc::c_char)
         -> libc::c_int;
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
    use super::q_shared_h::{qboolean, byte, cvar_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn MSG_InitOOB(buf: *mut msg_t, data: *mut byte,
                           length: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteData(buf: *mut msg_t, data: *const libc::c_void,
                             length: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteShort(sb: *mut msg_t, c: libc::c_int);
        #[no_mangle]
        pub fn MSG_WriteLong(sb: *mut msg_t, c: libc::c_int);
        #[no_mangle]
        pub fn MSG_BeginReadingOOB(sb: *mut msg_t);
        #[no_mangle]
        pub fn MSG_ReadShort(sb: *mut msg_t) -> libc::c_int;
        #[no_mangle]
        pub fn MSG_ReadLong(sb: *mut msg_t) -> libc::c_int;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
        #[no_mangle]
        pub fn Sys_SendPacket(length: libc::c_int, data: *const libc::c_void,
                              to: netadr_t);
        // Sys_Milliseconds should only be used for profiling purposes,
// any game related timing information should come from event timestamps
        #[no_mangle]
        pub fn Sys_Milliseconds() -> libc::c_int;
        #[no_mangle]
        pub static mut sv_packetdelay: *mut cvar_t;
        #[no_mangle]
        pub static mut com_timescale: *mut cvar_t;
        #[no_mangle]
        pub fn S_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub static mut cl_packetdelay: *mut cvar_t;
        #[no_mangle]
        pub fn Huff_Compress(buf: *mut msg_t, offset: libc::c_int);
        #[no_mangle]
        pub fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
        #[no_mangle]
        pub fn Sys_StringToAdr(s: *const libc::c_char, a: *mut netadr_t,
                               family: netadrtype_t) -> qboolean;
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
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/net_chan.c"]
pub mod net_chan_c {
    pub type packetQueue_t = packetQueue_s;
    //=============================================================================
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct packetQueue_s {
        pub next: *mut packetQueue_s,
        pub length: libc::c_int,
        pub data: *mut byte,
        pub to: netadr_t,
        pub release: libc::c_int,
    }
    //==============================================================================
    /*
=============================================================================

LOOPBACK BUFFERS FOR LOCAL PLAYER

=============================================================================
*/
    // there needs to be enough loopback messages to hold a complete
// gamestate of maximum size
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct loopmsg_t {
        pub data: [byte; 1400],
        pub datalen: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct loopback_t {
        pub msgs: [loopmsg_t; 16],
        pub get: libc::c_int,
        pub send: libc::c_int,
    }
    use super::{libc};
    use super::q_shared_h::{byte, cvar_t};
    use super::qcommon_h::{netadr_t, netsrc_t};
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
        #[no_mangle]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn strcmp(_: *const libc::c_char, _: *const libc::c_char)
         -> libc::c_int;
        #[no_mangle]
        pub fn strchr(_: *const libc::c_char, _: libc::c_int)
         -> *mut libc::c_char;
    }
}
#[header_src = "/usr/include/stdlib.h"]
pub mod stdlib_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    }
}
#[header_src =
      "/home/miguelsaldivar/workspace/ioq3/code/qcommon/q_platform.h"]
pub mod q_platform_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn ShortSwap(l: libc::c_short) -> libc::c_short;
    }
}
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, unnamed, ERR_NEED_CD,
                       ERR_DISCONNECT, ERR_SERVERDISCONNECT, ERR_DROP,
                       ERR_FATAL, cvar_s, cvar_t, Q_strncpyz, Q_CountChar, va,
                       Com_Error, Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netsrc_t, NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      MSG_InitOOB, MSG_WriteData, MSG_WriteShort,
                      MSG_WriteLong, MSG_BeginReadingOOB, MSG_ReadShort,
                      MSG_ReadLong, Z_Free, Sys_SendPacket, Sys_Milliseconds,
                      sv_packetdelay, com_timescale, S_MallocDebug,
                      cl_packetdelay, Huff_Compress, NET_AdrToString,
                      Sys_StringToAdr, Cvar_Get};
use self::net_chan_c::{packetQueue_t, packetQueue_s, loopmsg_t, loopback_t};
use self::string_h::{memcpy, memset, strcmp, strchr};
use self::stdlib_h::{atoi};
use self::q_platform_h::{ShortSwap};
#[no_mangle]
pub unsafe extern "C" fn NET_FlushPacketQueue() {
    let mut last: *mut packetQueue_t = 0 as *mut packetQueue_t;
    let mut now: libc::c_int = 0;
    while !packetQueue.is_null() {
        now = Sys_Milliseconds();
        if (*packetQueue).release >= now { break ; }
        Sys_SendPacket((*packetQueue).length,
                       (*packetQueue).data as *const libc::c_void,
                       (*packetQueue).to);
        last = packetQueue;
        packetQueue = (*packetQueue).next;
        Z_Free((*last).data as *mut libc::c_void);
        Z_Free(last as *mut libc::c_void);
    };
}
#[no_mangle]
pub static mut packetQueue: *mut packetQueue_t =
    0 as *const packetQueue_t as *mut packetQueue_t;
#[no_mangle]
pub unsafe extern "C" fn NET_SendPacket(mut sock: netsrc_t,
                                        mut length: libc::c_int,
                                        mut data: *const libc::c_void,
                                        mut to: netadr_t) {
    if 0 != (*showpackets).integer && *(data as *mut libc::c_int) == -1i32 {
        Com_Printf(b"send packet %4i\n\x00" as *const u8 as
                       *const libc::c_char, length);
    }
    if to.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint
       {
        NET_SendLoopPacket(sock, length, data, to);
        return
    }
    if to.type_0 as libc::c_uint == NA_BOT as libc::c_int as libc::c_uint {
        return
    }
    if to.type_0 as libc::c_uint == NA_BAD as libc::c_int as libc::c_uint {
        return
    }
    if sock as libc::c_uint == NS_CLIENT as libc::c_int as libc::c_uint &&
           (*cl_packetdelay).integer > 0i32 {
        NET_QueuePacket(length, data, to, (*cl_packetdelay).integer);
    } else if sock as libc::c_uint == NS_SERVER as libc::c_int as libc::c_uint
                  && (*sv_packetdelay).integer > 0i32 {
        NET_QueuePacket(length, data, to, (*sv_packetdelay).integer);
    } else { Sys_SendPacket(length, data, to); };
}
unsafe extern "C" fn NET_QueuePacket(mut length: libc::c_int,
                                     mut data: *const libc::c_void,
                                     mut to: netadr_t,
                                     mut offset: libc::c_int) {
    let mut new: *mut packetQueue_t = 0 as *mut packetQueue_t;
    let mut next: *mut packetQueue_t = packetQueue;
    if offset > 999i32 { offset = 999i32 }
    new =
        S_MallocDebug(::std::mem::size_of::<packetQueue_t>() as libc::c_ulong
                          as libc::c_int,
                      b"sizeof(packetQueue_t)\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      b"code/qcommon/net_chan.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 498i32) as
            *mut packetQueue_t;
    (*new).data =
        S_MallocDebug(length,
                      b"length\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/net_chan.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 499i32) as
            *mut byte;
    memcpy((*new).data as *mut libc::c_void, data, length as libc::c_ulong);
    (*new).length = length;
    (*new).to = to;
    (*new).release =
        Sys_Milliseconds() +
            (offset as libc::c_float / (*com_timescale).value) as libc::c_int;
    (*new).next = 0 as *mut packetQueue_s;
    if packetQueue.is_null() { packetQueue = new; return }
    while !next.is_null() {
        if (*next).next.is_null() { (*next).next = new; return }
        next = (*next).next
    };
}
#[no_mangle]
pub unsafe extern "C" fn NET_SendLoopPacket(mut sock: netsrc_t,
                                            mut length: libc::c_int,
                                            mut data: *const libc::c_void,
                                            mut to: netadr_t) {
    let mut i: libc::c_int = 0;
    let mut loop_0: *mut loopback_t = 0 as *mut loopback_t;
    loop_0 =
        &mut loopbacks[(sock as libc::c_uint ^ 1i32 as libc::c_uint) as usize]
            as *mut loopback_t;
    i = (*loop_0).send & 16i32 - 1i32;
    (*loop_0).send += 1;
    memcpy((*loop_0).msgs[i as usize].data.as_mut_ptr() as *mut libc::c_void,
           data, length as libc::c_ulong);
    (*loop_0).msgs[i as usize].datalen = length;
}
#[no_mangle]
pub static mut loopbacks: [loopback_t; 2] =
    [loopback_t{msgs: [loopmsg_t{data: [0; 1400], datalen: 0,}; 16],
                get: 0,
                send: 0,}; 2];
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

packet header
-------------
4	outgoing sequence.  high bit will be set if this is a fragmented message
[2	qport (only for client to server)]
[2	fragment start byte]
[2	fragment length. if < FRAGMENT_SIZE, this is the last fragment]

if the sequence number is -1, the packet should be handled as an out-of-band
message instead of as part of a netcon.

All fragments will have the same sequence numbers.

The qport field is a workaround for bad address translating routers that
sometimes remap the client's source port on a packet during gameplay.

If the base part of the net address matches and the qport matches, then the
channel matches even if the IP port differs.  The IP port should be updated
to the new value before sending out any replies.

*/
// max size of a network packet
// two ints and a short
#[no_mangle]
pub static mut showpackets: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn NET_OutOfBandData(mut sock: netsrc_t,
                                           mut adr: netadr_t,
                                           mut format: *mut byte,
                                           mut len: libc::c_int) {
    let mut string: [byte; 32768] = [0; 32768];
    let mut i: libc::c_int = 0;
    let mut mbuf: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    string[0usize] = 0xffi32 as byte;
    string[1usize] = 0xffi32 as byte;
    string[2usize] = 0xffi32 as byte;
    string[3usize] = 0xffi32 as byte;
    i = 0i32;
    while i < len {
        string[(i + 4i32) as usize] = *format.offset(i as isize);
        i += 1
    }
    mbuf.data = string.as_mut_ptr();
    mbuf.cursize = len + 4i32;
    Huff_Compress(&mut mbuf, 12i32);
    NET_SendPacket(sock, mbuf.cursize, mbuf.data as *const libc::c_void, adr);
}
#[no_mangle]
pub unsafe extern "C" fn NET_StringToAdr(mut s: *const libc::c_char,
                                         mut a: *mut netadr_t,
                                         mut family: netadrtype_t)
 -> libc::c_int {
    let mut base: [libc::c_char; 1024] = [0; 1024];
    let mut search: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut port: *mut libc::c_char = 0 as *mut libc::c_char;
    if 0 == strcmp(s, b"localhost\x00" as *const u8 as *const libc::c_char) {
        memset(a as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
        (*a).type_0 = NA_LOOPBACK;
        return 1i32
    }
    Q_strncpyz(base.as_mut_ptr(), s,
               ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                   as libc::c_int);
    if *base.as_mut_ptr() as libc::c_int == '[' as i32 ||
           Q_CountChar(base.as_mut_ptr(), ':' as i32 as libc::c_char) > 1i32 {
        search = strchr(base.as_mut_ptr(), ']' as i32);
        if !search.is_null() {
            *search = '\u{0}' as i32 as libc::c_char;
            search = search.offset(1isize);
            if *search as libc::c_int == ':' as i32 {
                port = search.offset(1isize)
            }
        }
        if *base.as_mut_ptr() as libc::c_int == '[' as i32 {
            search = base.as_mut_ptr().offset(1isize)
        } else { search = base.as_mut_ptr() }
    } else {
        port = strchr(base.as_mut_ptr(), ':' as i32);
        if !port.is_null() {
            *port = '\u{0}' as i32 as libc::c_char;
            port = port.offset(1isize)
        }
        search = base.as_mut_ptr()
    }
    if 0 == Sys_StringToAdr(search, a, family) as u64 {
        (*a).type_0 = NA_BAD;
        return 0i32
    }
    if !port.is_null() {
        (*a).port = ShortSwap(atoi(port) as libc::c_short) as libc::c_ushort;
        return 1i32
    } else {
        (*a).port = ShortSwap(27960i32 as libc::c_short) as libc::c_ushort;
        return 2i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn NET_GetLoopPacket(mut sock: netsrc_t,
                                           mut net_from: *mut netadr_t,
                                           mut net_message: *mut msg_t)
 -> qboolean {
    let mut i: libc::c_int = 0;
    let mut loop_0: *mut loopback_t = 0 as *mut loopback_t;
    loop_0 = &mut loopbacks[sock as usize] as *mut loopback_t;
    if (*loop_0).send - (*loop_0).get > 16i32 {
        (*loop_0).get = (*loop_0).send - 16i32
    }
    if (*loop_0).get >= (*loop_0).send { return qfalse }
    i = (*loop_0).get & 16i32 - 1i32;
    (*loop_0).get += 1;
    memcpy((*net_message).data as *mut libc::c_void,
           (*loop_0).msgs[i as usize].data.as_mut_ptr() as
               *const libc::c_void,
           (*loop_0).msgs[i as usize].datalen as libc::c_ulong);
    (*net_message).cursize = (*loop_0).msgs[i as usize].datalen;
    memset(net_from as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<netadr_t>() as libc::c_ulong);
    (*net_from).type_0 = NA_LOOPBACK;
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_Init(mut port: libc::c_int) {
    port &= 0xffffi32;
    showpackets =
        Cvar_Get(b"showpackets\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x100i32);
    showdrop =
        Cvar_Get(b"showdrop\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char, 0x100i32);
    qport =
        Cvar_Get(b"net_qport\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, port), 0x10i32);
}
#[no_mangle]
pub static mut qport: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut showdrop: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn Netchan_Setup(mut sock: netsrc_t,
                                       mut chan: *mut netchan_t,
                                       mut adr: netadr_t,
                                       mut qport_0: libc::c_int,
                                       mut challenge: libc::c_int,
                                       mut compat: qboolean) {
    memset(chan as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<netchan_t>() as libc::c_ulong);
    (*chan).sock = sock;
    (*chan).remoteAddress = adr;
    (*chan).qport = qport_0;
    (*chan).incomingSequence = 0i32;
    (*chan).outgoingSequence = 1i32;
    (*chan).challenge = challenge;
    (*chan).compat = compat;
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_Transmit(mut chan: *mut netchan_t,
                                          mut length: libc::c_int,
                                          mut data: *const byte) {
    let mut send: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    let mut send_buf: [byte; 1400] = [0; 1400];
    if length > 16384i32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Netchan_Transmit: length = %i\x00" as *const u8 as
                      *const libc::c_char, length);
    }
    (*chan).unsentFragmentStart = 0i32;
    if length >= 1400i32 - 100i32 {
        (*chan).unsentFragments = qtrue;
        (*chan).unsentLength = length;
        memcpy((*chan).unsentBuffer.as_mut_ptr() as *mut libc::c_void,
               data as *const libc::c_void, length as libc::c_ulong);
        Netchan_TransmitNextFragment(chan);
        return
    }
    MSG_InitOOB(&mut send, send_buf.as_mut_ptr(),
                ::std::mem::size_of::<[byte; 1400]>() as libc::c_ulong as
                    libc::c_int);
    MSG_WriteLong(&mut send, (*chan).outgoingSequence);
    if (*chan).sock as libc::c_uint ==
           NS_CLIENT as libc::c_int as libc::c_uint {
        MSG_WriteShort(&mut send, (*qport).integer);
    }
    if 0 == (*chan).compat as u64 {
        MSG_WriteLong(&mut send,
                      (*chan).challenge ^
                          (*chan).outgoingSequence * (*chan).challenge);
    }
    (*chan).outgoingSequence += 1;
    MSG_WriteData(&mut send, data as *const libc::c_void, length);
    NET_SendPacket((*chan).sock, send.cursize,
                   send.data as *const libc::c_void, (*chan).remoteAddress);
    (*chan).lastSentTime = Sys_Milliseconds();
    (*chan).lastSentSize = send.cursize;
    if 0 != (*showpackets).integer {
        Com_Printf(b"%s send %4i : s=%i ack=%i\n\x00" as *const u8 as
                       *const libc::c_char,
                   netsrcString[(*chan).sock as usize], send.cursize,
                   (*chan).outgoingSequence - 1i32, (*chan).incomingSequence);
    };
}
static mut netsrcString: [*mut libc::c_char; 2] =
    [b"client\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
     b"server\x00" as *const u8 as *const libc::c_char as *mut libc::c_char];
#[no_mangle]
pub unsafe extern "C" fn Netchan_TransmitNextFragment(mut chan:
                                                          *mut netchan_t) {
    let mut send: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    let mut send_buf: [byte; 1400] = [0; 1400];
    let mut fragmentLength: libc::c_int = 0;
    let mut outgoingSequence: libc::c_int = 0;
    MSG_InitOOB(&mut send, send_buf.as_mut_ptr(),
                ::std::mem::size_of::<[byte; 1400]>() as libc::c_ulong as
                    libc::c_int);
    outgoingSequence =
        ((*chan).outgoingSequence as libc::c_uint | 1u32 << 31i32) as
            libc::c_int;
    MSG_WriteLong(&mut send, outgoingSequence);
    if (*chan).sock as libc::c_uint ==
           NS_CLIENT as libc::c_int as libc::c_uint {
        MSG_WriteShort(&mut send, (*qport).integer);
    }
    if 0 == (*chan).compat as u64 {
        MSG_WriteLong(&mut send,
                      (*chan).challenge ^
                          (*chan).outgoingSequence * (*chan).challenge);
    }
    fragmentLength = 1400i32 - 100i32;
    if (*chan).unsentFragmentStart + fragmentLength > (*chan).unsentLength {
        fragmentLength = (*chan).unsentLength - (*chan).unsentFragmentStart
    }
    MSG_WriteShort(&mut send, (*chan).unsentFragmentStart);
    MSG_WriteShort(&mut send, fragmentLength);
    MSG_WriteData(&mut send,
                  (*chan).unsentBuffer.as_mut_ptr().offset((*chan).unsentFragmentStart
                                                               as isize) as
                      *const libc::c_void, fragmentLength);
    NET_SendPacket((*chan).sock, send.cursize,
                   send.data as *const libc::c_void, (*chan).remoteAddress);
    (*chan).lastSentTime = Sys_Milliseconds();
    (*chan).lastSentSize = send.cursize;
    if 0 != (*showpackets).integer {
        Com_Printf(b"%s send %4i : s=%i fragment=%i,%i\n\x00" as *const u8 as
                       *const libc::c_char,
                   netsrcString[(*chan).sock as usize], send.cursize,
                   (*chan).outgoingSequence, (*chan).unsentFragmentStart,
                   fragmentLength);
    }
    (*chan).unsentFragmentStart += fragmentLength;
    if (*chan).unsentFragmentStart == (*chan).unsentLength &&
           fragmentLength != 1400i32 - 100i32 {
        (*chan).outgoingSequence += 1;
        (*chan).unsentFragments = qfalse
    };
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_Process(mut chan: *mut netchan_t,
                                         mut msg: *mut msg_t) -> qboolean {
    let mut sequence: libc::c_int = 0;
    let mut fragmentStart: libc::c_int = 0;
    let mut fragmentLength: libc::c_int = 0;
    let mut fragmented: qboolean = qfalse;
    MSG_BeginReadingOOB(msg);
    sequence = MSG_ReadLong(msg);
    if 0 != sequence as libc::c_uint & 1u32 << 31i32 {
        sequence =
            (sequence as libc::c_uint & !(1u32 << 31i32)) as libc::c_int;
        fragmented = qtrue
    } else { fragmented = qfalse }
    if (*chan).sock as libc::c_uint ==
           NS_SERVER as libc::c_int as libc::c_uint {
        MSG_ReadShort(msg);
    }
    if 0 == (*chan).compat as u64 {
        let mut checksum: libc::c_int = MSG_ReadLong(msg);
        if (*chan).challenge ^ sequence * (*chan).challenge != checksum {
            return qfalse
        }
    }
    if 0 != fragmented as u64 {
        fragmentStart = MSG_ReadShort(msg);
        fragmentLength = MSG_ReadShort(msg)
    } else { fragmentStart = 0i32; fragmentLength = 0i32 }
    if 0 != (*showpackets).integer {
        if 0 != fragmented as u64 {
            Com_Printf(b"%s recv %4i : s=%i fragment=%i,%i\n\x00" as *const u8
                           as *const libc::c_char,
                       netsrcString[(*chan).sock as usize], (*msg).cursize,
                       sequence, fragmentStart, fragmentLength);
        } else {
            Com_Printf(b"%s recv %4i : s=%i\n\x00" as *const u8 as
                           *const libc::c_char,
                       netsrcString[(*chan).sock as usize], (*msg).cursize,
                       sequence);
        }
    }
    if sequence <= (*chan).incomingSequence {
        if 0 != (*showdrop).integer || 0 != (*showpackets).integer {
            Com_Printf(b"%s:Out of order packet %i at %i\n\x00" as *const u8
                           as *const libc::c_char,
                       NET_AdrToString((*chan).remoteAddress), sequence,
                       (*chan).incomingSequence);
        }
        return qfalse
    }
    (*chan).dropped = sequence - ((*chan).incomingSequence + 1i32);
    if (*chan).dropped > 0i32 {
        if 0 != (*showdrop).integer || 0 != (*showpackets).integer {
            Com_Printf(b"%s:Dropped %i packets at %i\n\x00" as *const u8 as
                           *const libc::c_char,
                       NET_AdrToString((*chan).remoteAddress),
                       (*chan).dropped, sequence);
        }
    }
    if 0 != fragmented as u64 {
        if sequence != (*chan).fragmentSequence {
            (*chan).fragmentSequence = sequence;
            (*chan).fragmentLength = 0i32
        }
        if fragmentStart != (*chan).fragmentLength {
            if 0 != (*showdrop).integer || 0 != (*showpackets).integer {
                Com_Printf(b"%s:Dropped a message fragment\n\x00" as *const u8
                               as *const libc::c_char,
                           NET_AdrToString((*chan).remoteAddress));
            }
            return qfalse
        }
        if fragmentLength < 0i32 ||
               (*msg).readcount + fragmentLength > (*msg).cursize ||
               ((*chan).fragmentLength + fragmentLength) as libc::c_ulong >
                   ::std::mem::size_of::<[byte; 16384]>() as libc::c_ulong {
            if 0 != (*showdrop).integer || 0 != (*showpackets).integer {
                Com_Printf(b"%s:illegal fragment length\n\x00" as *const u8 as
                               *const libc::c_char,
                           NET_AdrToString((*chan).remoteAddress));
            }
            return qfalse
        }
        memcpy((*chan).fragmentBuffer.as_mut_ptr().offset((*chan).fragmentLength
                                                              as isize) as
                   *mut libc::c_void,
               (*msg).data.offset((*msg).readcount as isize) as
                   *const libc::c_void, fragmentLength as libc::c_ulong);
        (*chan).fragmentLength += fragmentLength;
        if fragmentLength == 1400i32 - 100i32 { return qfalse }
        if (*chan).fragmentLength > (*msg).maxsize {
            Com_Printf(b"%s:fragmentLength %i > msg->maxsize\n\x00" as
                           *const u8 as *const libc::c_char,
                       NET_AdrToString((*chan).remoteAddress),
                       (*chan).fragmentLength);
            return qfalse
        }
        *((*msg).data as *mut libc::c_int) = sequence;
        memcpy((*msg).data.offset(4isize) as *mut libc::c_void,
               (*chan).fragmentBuffer.as_mut_ptr() as *const libc::c_void,
               (*chan).fragmentLength as libc::c_ulong);
        (*msg).cursize = (*chan).fragmentLength + 4i32;
        (*chan).fragmentLength = 0i32;
        (*msg).readcount = 4i32;
        (*msg).bit = 32i32;
        (*chan).incomingSequence = sequence;
        return qtrue
    }
    (*chan).incomingSequence = sequence;
    return qtrue;
}