use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __uint16_t = libc::c_ushort;
    pub type __uint32_t = libc::c_uint;
    pub type __time_t = libc::c_long;
    pub type __suseconds_t = libc::c_long;
    pub type __ssize_t = libc::c_long;
    pub type __socklen_t = libc::c_uint;
    use super::{libc};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/stdio.h"]
pub mod stdio_h {
    pub type ssize_t = __ssize_t;
    use super::types_h::{__ssize_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types/struct_timeval.h"]
pub mod struct_timeval_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct timeval {
        pub tv_sec: __time_t,
        pub tv_usec: __suseconds_t,
    }
    use super::types_h::{__time_t, __suseconds_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/sys/select.h"]
pub mod select_h {
    pub type __fd_mask = libc::c_long;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct fd_set {
        pub __fds_bits: [__fd_mask; 16],
    }
    use super::{libc};
    use super::struct_timeval_h::{timeval};
    extern "C" {
        #[no_mangle]
        pub fn select(__nfds: libc::c_int, __readfds: *mut fd_set,
                      __writefds: *mut fd_set, __exceptfds: *mut fd_set,
                      __timeout: *mut timeval) -> libc::c_int;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    pub type uint16_t = __uint16_t;
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t};
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
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
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
    // Pulls off \n terminated lines of text from the command buffer and sends
// them through Cmd_ExecuteString.  Stops when the buffer is empty.
// Normally called once per frame, but may be explicitly invoked.
// Do not call inside a command function, or current args will be destroyed.
    //===========================================================================
    /*

Command execution takes a null terminated string, breaks it into tokens,
then searches for a command or variable that matches the first token.

*/
    pub type xcommand_t = Option<unsafe extern "C" fn() -> ()>;
    use super::q_shared_h::{qboolean, byte, cvar_t};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn MSG_Init(buf: *mut msg_t, data: *mut byte,
                        length: libc::c_int);
        // don't set the cvar immediately
        #[no_mangle]
        pub fn Cvar_SetValue(var_name: *const libc::c_char,
                             value: libc::c_float);
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
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
        #[no_mangle]
        pub fn Cmd_AddCommand(cmd_name: *const libc::c_char,
                              function: xcommand_t);
        #[no_mangle]
        pub fn CL_PacketEvent(from: netadr_t, msg: *mut msg_t);
        #[no_mangle]
        pub fn Com_RunAndTimeServerPacket(evFrom: *mut netadr_t,
                                          buf: *mut msg_t);
        #[no_mangle]
        pub static mut com_sv_running: *mut cvar_t;
    }
}
#[header_src = "/usr/include/netinet/in.h"]
pub mod in_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: in6_addr,
        pub ipv6mr_interface: libc::c_uint,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct in6_addr {
        pub __in6_u: unnamed_0,
    }
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed_0 {
        pub __u6_addr8: [uint8_t; 16],
        pub __u6_addr16: [uint16_t; 8],
        pub __u6_addr32: [uint32_t; 4],
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: in_port_t,
        pub sin6_flowinfo: uint32_t,
        pub sin6_addr: in6_addr,
        pub sin6_scope_id: uint32_t,
    }
    pub type in_port_t = uint16_t;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: in_port_t,
        pub sin_addr: in_addr,
        pub sin_zero: [libc::c_uchar; 8],
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct in_addr {
        pub s_addr: in_addr_t,
    }
    pub type in_addr_t = uint32_t;
    pub const IPPROTO_TCP: unnamed_2 = 6;
    pub const IPPROTO_UDP: unnamed_2 = 17;
    pub const IPPROTO_IPV6: unnamed_2 = 41;
    pub type unnamed_2 = libc::c_uint;
    pub const IPPROTO_MAX: unnamed_2 = 256;
    pub const IPPROTO_RAW: unnamed_2 = 255;
    pub const IPPROTO_MPLS: unnamed_2 = 137;
    pub const IPPROTO_UDPLITE: unnamed_2 = 136;
    pub const IPPROTO_SCTP: unnamed_2 = 132;
    pub const IPPROTO_COMP: unnamed_2 = 108;
    pub const IPPROTO_PIM: unnamed_2 = 103;
    pub const IPPROTO_ENCAP: unnamed_2 = 98;
    pub const IPPROTO_BEETPH: unnamed_2 = 94;
    pub const IPPROTO_MTP: unnamed_2 = 92;
    pub const IPPROTO_AH: unnamed_2 = 51;
    pub const IPPROTO_ESP: unnamed_2 = 50;
    pub const IPPROTO_GRE: unnamed_2 = 47;
    pub const IPPROTO_RSVP: unnamed_2 = 46;
    pub const IPPROTO_DCCP: unnamed_2 = 33;
    pub const IPPROTO_TP: unnamed_2 = 29;
    pub const IPPROTO_IDP: unnamed_2 = 22;
    pub const IPPROTO_PUP: unnamed_2 = 12;
    pub const IPPROTO_EGP: unnamed_2 = 8;
    pub const IPPROTO_IPIP: unnamed_2 = 4;
    pub const IPPROTO_IGMP: unnamed_2 = 2;
    pub const IPPROTO_ICMP: unnamed_2 = 1;
    pub const IPPROTO_IP: unnamed_2 = 0;
    use super::{libc};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
    use super::sockaddr_h::{sa_family_t};
    extern "C" {
        #[no_mangle]
        pub fn htons(__hostshort: uint16_t) -> uint16_t;
        #[no_mangle]
        pub static in6addr_any: in6_addr;
        #[no_mangle]
        pub fn ntohs(__netshort: uint16_t) -> uint16_t;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/sockaddr.h"]
pub mod sockaddr_h {
    pub type sa_family_t = libc::c_ushort;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/socket.h"]
pub mod socket_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [libc::c_char; 14],
    }
    pub type socklen_t = __socklen_t;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        pub __ss_padding: [libc::c_char; 118],
        pub __ss_align: libc::c_ulong,
    }
    use super::sockaddr_h::{sa_family_t};
    use super::{libc};
    use super::types_h::{__socklen_t};
}
#[header_src = "/usr/include/netdb.h"]
pub mod netdb_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct addrinfo {
        pub ai_flags: libc::c_int,
        pub ai_family: libc::c_int,
        pub ai_socktype: libc::c_int,
        pub ai_protocol: libc::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_addr: *mut sockaddr,
        pub ai_canonname: *mut libc::c_char,
        pub ai_next: *mut addrinfo,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct hostent {
        pub h_name: *mut libc::c_char,
        pub h_aliases: *mut *mut libc::c_char,
        pub h_addrtype: libc::c_int,
        pub h_length: libc::c_int,
        pub h_addr_list: *mut *mut libc::c_char,
    }
    use super::{libc};
    use super::socket_h::{socklen_t, sockaddr};
    extern "C" {
        #[no_mangle]
        pub fn freeaddrinfo(__ai: *mut addrinfo);
        #[no_mangle]
        pub fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
        #[no_mangle]
        pub fn getaddrinfo(__name: *const libc::c_char,
                           __service: *const libc::c_char,
                           __req: *const addrinfo, __pai: *mut *mut addrinfo)
         -> libc::c_int;
        #[no_mangle]
        pub fn gethostbyname(__name: *const libc::c_char) -> *mut hostent;
        #[no_mangle]
        pub fn getnameinfo(__sa: *const sockaddr, __salen: socklen_t,
                           __host: *mut libc::c_char, __hostlen: socklen_t,
                           __serv: *mut libc::c_char, __servlen: socklen_t,
                           __flags: libc::c_int) -> libc::c_int;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/socket_type.h"]
pub mod socket_type_h {
    pub const SOCK_DGRAM: __socket_type = 2;
    pub const SOCK_STREAM: __socket_type = 1;
    pub type __socket_type = libc::c_uint;
    pub const SOCK_NONBLOCK: __socket_type = 2048;
    pub const SOCK_CLOEXEC: __socket_type = 524288;
    pub const SOCK_PACKET: __socket_type = 10;
    pub const SOCK_DCCP: __socket_type = 6;
    pub const SOCK_SEQPACKET: __socket_type = 5;
    pub const SOCK_RDM: __socket_type = 4;
    pub const SOCK_RAW: __socket_type = 3;
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/net_ip.c"]
pub mod net_ip_c {
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
    pub type SOCKET = libc::c_int;
    pub type ioctlarg_t = libc::c_int;
    // use an admin local address per default so that network admins can decide on how to handle quake3 traffic.
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct nip_localaddr_t {
        pub ifname: [libc::c_char; 16],
        pub type_0: netadrtype_t,
        pub family: sa_family_t,
        pub addr: sockaddr_storage,
        pub netmask: sockaddr_storage,
    }
    use super::{libc};
    use super::qcommon_h::{netadrtype_t, netadr_t, msg_t};
    use super::sockaddr_h::{sa_family_t};
    use super::socket_h::{sockaddr_storage};
    use super::in_h::{sockaddr_in6};
    use super::select_h::{fd_set};
    use super::q_shared_h::{qboolean};
}
#[header_src = "/usr/include/ifaddrs.h"]
pub mod ifaddrs_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct ifaddrs {
        pub ifa_next: *mut ifaddrs,
        pub ifa_name: *mut libc::c_char,
        pub ifa_flags: libc::c_uint,
        pub ifa_addr: *mut sockaddr,
        pub ifa_netmask: *mut sockaddr,
        pub ifa_ifu: unnamed_1,
        pub ifa_data: *mut libc::c_void,
    }
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed_1 {
        pub ifu_broadaddr: *mut sockaddr,
        pub ifu_dstaddr: *mut sockaddr,
    }
    use super::{libc};
    use super::socket_h::{sockaddr};
    extern "C" {
        #[no_mangle]
        pub fn freeifaddrs(__ifa: *mut ifaddrs);
        #[no_mangle]
        pub fn getifaddrs(__ifap: *mut *mut ifaddrs) -> libc::c_int;
    }
}
#[header_src = "/usr/include/net/if.h"]
pub mod if_h {
    pub const IFF_UP: unnamed_3 = 1;
    pub type unnamed_3 = libc::c_uint;
    pub const IFF_DYNAMIC: unnamed_3 = 32768;
    pub const IFF_AUTOMEDIA: unnamed_3 = 16384;
    pub const IFF_PORTSEL: unnamed_3 = 8192;
    pub const IFF_MULTICAST: unnamed_3 = 4096;
    pub const IFF_SLAVE: unnamed_3 = 2048;
    pub const IFF_MASTER: unnamed_3 = 1024;
    pub const IFF_ALLMULTI: unnamed_3 = 512;
    pub const IFF_PROMISC: unnamed_3 = 256;
    pub const IFF_NOARP: unnamed_3 = 128;
    pub const IFF_RUNNING: unnamed_3 = 64;
    pub const IFF_NOTRAILERS: unnamed_3 = 32;
    pub const IFF_POINTOPOINT: unnamed_3 = 16;
    pub const IFF_LOOPBACK: unnamed_3 = 8;
    pub const IFF_DEBUG: unnamed_3 = 4;
    pub const IFF_BROADCAST: unnamed_3 = 2;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn if_nametoindex(__ifname: *const libc::c_char) -> libc::c_uint;
    }
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
        pub fn memcmp(_: *const libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> libc::c_int;
        #[no_mangle]
        pub fn strlen(_: *const libc::c_char) -> libc::c_ulong;
        #[no_mangle]
        pub fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
#[header_src = "/usr/include/errno.h"]
pub mod errno_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn __errno_location() -> *mut libc::c_int;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/sys/socket.h"]
pub mod sys_socket_h {
    use super::stdio_h::{ssize_t};
    use super::{libc};
    use super::stddef_h::{size_t};
    use super::socket_h::{sockaddr, socklen_t};
    extern "C" {
        #[no_mangle]
        pub fn recv(__fd: libc::c_int, __buf: *mut libc::c_void, __n: size_t,
                    __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        pub fn send(__fd: libc::c_int, __buf: *const libc::c_void,
                    __n: size_t, __flags: libc::c_int) -> ssize_t;
        #[no_mangle]
        pub fn connect(__fd: libc::c_int, __addr: *const sockaddr,
                       __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        pub fn socket(__domain: libc::c_int, __type: libc::c_int,
                      __protocol: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn bind(__fd: libc::c_int, __addr: *const sockaddr,
                    __len: socklen_t) -> libc::c_int;
        #[no_mangle]
        pub fn setsockopt(__fd: libc::c_int, __level: libc::c_int,
                          __optname: libc::c_int,
                          __optval: *const libc::c_void, __optlen: socklen_t)
         -> libc::c_int;
        #[no_mangle]
        pub fn recvfrom(__fd: libc::c_int, __buf: *mut libc::c_void,
                        __n: size_t, __flags: libc::c_int,
                        __addr: *mut sockaddr, __addr_len: *mut socklen_t)
         -> ssize_t;
        #[no_mangle]
        pub fn sendto(__fd: libc::c_int, __buf: *const libc::c_void,
                      __n: size_t, __flags: libc::c_int,
                      __addr: *const sockaddr, __addr_len: socklen_t)
         -> ssize_t;
    }
}
#[header_src = "/usr/include/unistd.h"]
pub mod unistd_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn close(__fd: libc::c_int) -> libc::c_int;
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/sys/ioctl.h"]
pub mod ioctl_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, ...)
         -> libc::c_int;
    }
}
use self::types_h::{__uint8_t, __uint16_t, __uint32_t, __time_t,
                    __suseconds_t, __ssize_t, __socklen_t};
use self::stddef_h::{size_t};
use self::stdio_h::{ssize_t};
use self::struct_timeval_h::{timeval};
use self::select_h::{__fd_mask, fd_set, select};
use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, unnamed, ERR_NEED_CD,
                       ERR_DISCONNECT, ERR_SERVERDISCONNECT, ERR_DROP,
                       ERR_FATAL, cvar_s, cvar_t, Com_sprintf, Q_strncpyz,
                       Q_CountChar, va, Com_Error, Com_Printf};
use self::qcommon_h::{msg_t, netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6,
                      NA_IP, NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD,
                      netadr_t, xcommand_t, MSG_Init, Cvar_SetValue,
                      Com_DPrintf, Cvar_Get, Cmd_AddCommand, CL_PacketEvent,
                      Com_RunAndTimeServerPacket, com_sv_running};
use self::in_h::{ipv6_mreq, in6_addr, unnamed_0, sockaddr_in6, in_port_t,
                 sockaddr_in, in_addr, in_addr_t, IPPROTO_TCP, IPPROTO_UDP,
                 IPPROTO_IPV6, unnamed_2, IPPROTO_MAX, IPPROTO_RAW,
                 IPPROTO_MPLS, IPPROTO_UDPLITE, IPPROTO_SCTP, IPPROTO_COMP,
                 IPPROTO_PIM, IPPROTO_ENCAP, IPPROTO_BEETPH, IPPROTO_MTP,
                 IPPROTO_AH, IPPROTO_ESP, IPPROTO_GRE, IPPROTO_RSVP,
                 IPPROTO_DCCP, IPPROTO_TP, IPPROTO_IDP, IPPROTO_PUP,
                 IPPROTO_EGP, IPPROTO_IPIP, IPPROTO_IGMP, IPPROTO_ICMP,
                 IPPROTO_IP, htons, in6addr_any, ntohs};
use self::sockaddr_h::{sa_family_t};
use self::socket_h::{sockaddr, socklen_t, sockaddr_storage};
use self::netdb_h::{addrinfo, hostent, freeaddrinfo, gai_strerror,
                    getaddrinfo, gethostbyname, getnameinfo};
use self::socket_type_h::{SOCK_DGRAM, SOCK_STREAM, __socket_type,
                          SOCK_NONBLOCK, SOCK_CLOEXEC, SOCK_PACKET, SOCK_DCCP,
                          SOCK_SEQPACKET, SOCK_RDM, SOCK_RAW};
use self::net_ip_c::{SOCKET, ioctlarg_t, nip_localaddr_t};
use self::ifaddrs_h::{ifaddrs, unnamed_1, freeifaddrs, getifaddrs};
use self::if_h::{IFF_UP, unnamed_3, IFF_DYNAMIC, IFF_AUTOMEDIA, IFF_PORTSEL,
                 IFF_MULTICAST, IFF_SLAVE, IFF_MASTER, IFF_ALLMULTI,
                 IFF_PROMISC, IFF_NOARP, IFF_RUNNING, IFF_NOTRAILERS,
                 IFF_POINTOPOINT, IFF_LOOPBACK, IFF_DEBUG, IFF_BROADCAST,
                 if_nametoindex};
use self::string_h::{memcpy, memset, memcmp, strlen, strerror};
use self::stdlib_h::{rand};
use self::errno_h::{__errno_location};
use self::sys_socket_h::{recv, send, connect, socket, bind, setsockopt,
                         recvfrom, sendto};
use self::unistd_h::{close};
use self::ioctl_h::{ioctl};
#[no_mangle]
pub unsafe extern "C" fn NET_Init() {
    NET_Config(qtrue);
    Cmd_AddCommand(b"net_restart\x00" as *const u8 as *const libc::c_char,
                   Some(NET_Restart_f));
}
#[no_mangle]
pub unsafe extern "C" fn NET_Restart_f() { NET_Config(qtrue); }
#[no_mangle]
pub unsafe extern "C" fn NET_Config(mut enableNetworking: qboolean) {
    let mut modified: qboolean = qfalse;
    let mut stop: qboolean = qfalse;
    let mut start: qboolean = qfalse;
    modified = NET_GetCvars();
    if 0 == (*net_enabled).integer { enableNetworking = qfalse }
    if enableNetworking as libc::c_uint == networkingEnabled as libc::c_uint
           && 0 == modified as u64 {
        return
    }
    if enableNetworking as libc::c_uint == networkingEnabled as libc::c_uint {
        if 0 != enableNetworking as u64 {
            stop = qtrue;
            start = qtrue
        } else { stop = qfalse; start = qfalse }
    } else {
        if 0 != enableNetworking as u64 {
            stop = qfalse;
            start = qtrue
        } else { stop = qtrue; start = qfalse }
        networkingEnabled = enableNetworking as libc::c_int
    }
    if 0 != stop as u64 {
        if ip_socket != -1i32 { close(ip_socket); ip_socket = -1i32 }
        if multicast6_socket != -1i32 {
            if multicast6_socket != ip6_socket { close(multicast6_socket); }
            multicast6_socket = -1i32
        }
        if ip6_socket != -1i32 { close(ip6_socket); ip6_socket = -1i32 }
        if socks_socket != -1i32 { close(socks_socket); socks_socket = -1i32 }
    }
    if 0 != start as u64 {
        if 0 != (*net_enabled).integer { NET_OpenIP(); NET_SetMulticast6(); }
    };
}
/*
====================
NET_SetMulticast
Set the current multicast group
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_SetMulticast6() {
    let mut addr: sockaddr_in6 =
        sockaddr_in6{sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         in6_addr{__in6_u: unnamed_0{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    if 0 == *(*net_mcast6addr).string ||
           0 ==
               Sys_StringToSockaddr((*net_mcast6addr).string,
                                    &mut addr as *mut sockaddr_in6 as
                                        *mut sockaddr,
                                    ::std::mem::size_of::<sockaddr_in6>() as
                                        libc::c_ulong as libc::c_int,
                                    10i32 as sa_family_t) as u64 {
        Com_Printf(b"WARNING: NET_JoinMulticast6: Incorrect multicast address given, please set cvar %s to a sane value.\n\x00"
                       as *const u8 as *const libc::c_char,
                   (*net_mcast6addr).name);
        Cvar_SetValue((*net_enabled).name,
                      ((*net_enabled).integer | 0x8i32) as libc::c_float);
        return
    }
    memcpy(&mut curgroup.ipv6mr_multiaddr as *mut in6_addr as
               *mut libc::c_void,
           &mut addr.sin6_addr as *mut in6_addr as *const libc::c_void,
           ::std::mem::size_of::<in6_addr>() as libc::c_ulong);
    if 0 != *(*net_mcast6iface).string {
        curgroup.ipv6mr_interface = if_nametoindex((*net_mcast6iface).string)
    } else { curgroup.ipv6mr_interface = 0i32 as libc::c_uint };
}
// Keep track of currently joined multicast group.
static mut curgroup: ipv6_mreq =
    ipv6_mreq{ipv6mr_multiaddr:
                  in6_addr{__in6_u: unnamed_0{__u6_addr8: [0; 16],},},
              ipv6mr_interface: 0,};
static mut net_mcast6iface: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut net_enabled: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut net_mcast6addr: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
=============
Sys_StringToSockaddr
=============
*/
unsafe extern "C" fn Sys_StringToSockaddr(mut s: *const libc::c_char,
                                          mut sadr: *mut sockaddr,
                                          mut sadr_len: libc::c_int,
                                          mut family: sa_family_t)
 -> qboolean {
    let mut hints: addrinfo =
        addrinfo{ai_flags: 0,
                 ai_family: 0,
                 ai_socktype: 0,
                 ai_protocol: 0,
                 ai_addrlen: 0,
                 ai_addr: 0 as *mut sockaddr,
                 ai_canonname: 0 as *mut libc::c_char,
                 ai_next: 0 as *mut addrinfo,};
    let mut res: *mut addrinfo = 0 as *mut addrinfo;
    let mut search: *mut addrinfo = 0 as *mut addrinfo;
    let mut hintsp: *mut addrinfo = 0 as *mut addrinfo;
    let mut retval: libc::c_int = 0;
    memset(sadr as *mut libc::c_void, '\u{0}' as i32,
           ::std::mem::size_of::<sockaddr>() as libc::c_ulong);
    memset(&mut hints as *mut addrinfo as *mut libc::c_void, '\u{0}' as i32,
           ::std::mem::size_of::<addrinfo>() as libc::c_ulong);
    hintsp = &mut hints;
    (*hintsp).ai_family = family as libc::c_int;
    (*hintsp).ai_socktype = SOCK_DGRAM as libc::c_int;
    retval = getaddrinfo(s, 0 as *const libc::c_char, hintsp, &mut res);
    if 0 == retval {
        if family as libc::c_int == 0i32 {
            if 0 != (*net_enabled).integer & 0x4i32 {
                if 0 != (*net_enabled).integer & 0x2i32 {
                    search = SearchAddrInfo(res, 10i32 as sa_family_t)
                }
                if search.is_null() && 0 != (*net_enabled).integer & 0x1i32 {
                    search = SearchAddrInfo(res, 2i32 as sa_family_t)
                }
            } else {
                if 0 != (*net_enabled).integer & 0x1i32 {
                    search = SearchAddrInfo(res, 2i32 as sa_family_t)
                }
                if search.is_null() && 0 != (*net_enabled).integer & 0x2i32 {
                    search = SearchAddrInfo(res, 10i32 as sa_family_t)
                }
            }
        } else { search = SearchAddrInfo(res, family) }
        if !search.is_null() {
            if (*search).ai_addrlen > sadr_len as libc::c_uint {
                (*search).ai_addrlen = sadr_len as socklen_t
            }
            memcpy(sadr as *mut libc::c_void,
                   (*search).ai_addr as *const libc::c_void,
                   (*search).ai_addrlen as libc::c_ulong);
            freeaddrinfo(res);
            return qtrue
        } else {
            Com_Printf(b"Sys_StringToSockaddr: Error resolving %s: No address of required type found.\n\x00"
                           as *const u8 as *const libc::c_char, s);
        }
    } else {
        Com_Printf(b"Sys_StringToSockaddr: Error resolving %s: %s\n\x00" as
                       *const u8 as *const libc::c_char, s,
                   gai_strerror(retval));
    }
    if !res.is_null() { freeaddrinfo(res); }
    return qfalse;
}
unsafe extern "C" fn SearchAddrInfo(mut hints: *mut addrinfo,
                                    mut family: sa_family_t)
 -> *mut addrinfo {
    while !hints.is_null() {
        if (*hints).ai_family == family as libc::c_int { return hints }
        hints = (*hints).ai_next
    }
    return 0 as *mut addrinfo;
}
/*
====================
NET_OpenIP
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_OpenIP() {
    let mut i: libc::c_int = 0;
    let mut err: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut port6: libc::c_int = 0;
    port = (*net_port).integer;
    port6 = (*net_port6).integer;
    NET_GetLocalAddress();
    if 0 != (*net_enabled).integer & 0x2i32 {
        i = 0i32;
        while i < 10i32 {
            ip6_socket =
                NET_IP6Socket((*net_ip6).string, port6 + i, &mut boundto,
                              &mut err);
            if ip6_socket != -1i32 {
                Cvar_SetValue(b"net_port6\x00" as *const u8 as
                                  *const libc::c_char,
                              (port6 + i) as libc::c_float);
                break ;
            } else { if err == 97i32 { break ; } i += 1 }
        }
        if ip6_socket == -1i32 {
            Com_Printf(b"WARNING: Couldn\'t bind to a v6 ip address.\n\x00" as
                           *const u8 as *const libc::c_char);
        }
    }
    if 0 != (*net_enabled).integer & 0x1i32 {
        i = 0i32;
        while i < 10i32 {
            ip_socket = NET_IPSocket((*net_ip).string, port + i, &mut err);
            if ip_socket != -1i32 {
                Cvar_SetValue(b"net_port\x00" as *const u8 as
                                  *const libc::c_char,
                              (port + i) as libc::c_float);
                if 0 != (*net_socksEnabled).integer {
                    NET_OpenSocks(port + i);
                }
                break ;
            } else { if err == 97i32 { break ; } i += 1 }
        }
        if ip_socket == -1i32 {
            Com_Printf(b"WARNING: Couldn\'t bind to a v4 ip address.\n\x00" as
                           *const u8 as *const libc::c_char);
        }
    };
}
static mut ip_socket: SOCKET = -1i32;
/*
====================
NET_OpenSocks
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_OpenSocks(mut port: libc::c_int) {
    let mut address: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut h: *mut hostent = 0 as *mut hostent;
    let mut len: libc::c_int = 0;
    let mut rfc1929: qboolean = qfalse;
    let mut buf: [libc::c_uchar; 64] = [0; 64];
    usingSocks = qfalse;
    Com_Printf(b"Opening connection to SOCKS server.\n\x00" as *const u8 as
                   *const libc::c_char);
    socks_socket =
        socket(2i32, SOCK_STREAM as libc::c_int, IPPROTO_TCP as libc::c_int);
    if socks_socket == -1i32 {
        Com_Printf(b"WARNING: NET_OpenSocks: socket: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
        return
    }
    h = gethostbyname((*net_socksServer).string);
    if h.is_null() {
        Com_Printf(b"WARNING: NET_OpenSocks: gethostbyname: %s\n\x00" as
                       *const u8 as *const libc::c_char, NET_ErrorString());
        return
    }
    if (*h).h_addrtype != 2i32 {
        Com_Printf(b"WARNING: NET_OpenSocks: gethostbyname: address type was not AF_INET\n\x00"
                       as *const u8 as *const libc::c_char);
        return
    }
    address.sin_family = 2i32 as sa_family_t;
    address.sin_addr.s_addr =
        *(*(*h).h_addr_list.offset(0isize) as *mut libc::c_int) as in_addr_t;
    address.sin_port =
        htons((*net_socksPort).integer as libc::c_short as uint16_t);
    if connect(socks_socket,
               &mut address as *mut sockaddr_in as *mut sockaddr,
               ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                   socklen_t) == -1i32 {
        Com_Printf(b"NET_OpenSocks: connect: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
        return
    }
    if 0 != *(*net_socksUsername).string as libc::c_int ||
           0 != *(*net_socksPassword).string as libc::c_int {
        rfc1929 = qtrue
    } else { rfc1929 = qfalse }
    buf[0usize] = 5i32 as libc::c_uchar;
    if 0 != rfc1929 as u64 {
        buf[1usize] = 2i32 as libc::c_uchar;
        len = 4i32
    } else { buf[1usize] = 1i32 as libc::c_uchar; len = 3i32 }
    buf[2usize] = 0i32 as libc::c_uchar;
    if 0 != rfc1929 as u64 { buf[2usize] = 2i32 as libc::c_uchar }
    if send(socks_socket, buf.as_mut_ptr() as *mut libc::c_void,
            len as size_t, 0i32) == -1i32 as libc::c_long {
        Com_Printf(b"NET_OpenSocks: send: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
        return
    }
    len =
        recv(socks_socket, buf.as_mut_ptr() as *mut libc::c_void,
             64i32 as size_t, 0i32) as libc::c_int;
    if len == -1i32 {
        Com_Printf(b"NET_OpenSocks: recv: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
        return
    }
    if len != 2i32 || buf[0usize] as libc::c_int != 5i32 {
        Com_Printf(b"NET_OpenSocks: bad response\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    match buf[1usize] as libc::c_int {
        0 | 2 => { }
        _ => {
            Com_Printf(b"NET_OpenSocks: request denied\n\x00" as *const u8 as
                           *const libc::c_char);
            return
        }
    }
    if buf[1usize] as libc::c_int == 2i32 {
        let mut ulen: libc::c_int = 0;
        let mut plen: libc::c_int = 0;
        ulen = strlen((*net_socksUsername).string) as libc::c_int;
        plen = strlen((*net_socksPassword).string) as libc::c_int;
        buf[0usize] = 1i32 as libc::c_uchar;
        buf[1usize] = ulen as libc::c_uchar;
        if 0 != ulen {
            memcpy(&mut buf[2usize] as *mut libc::c_uchar as
                       *mut libc::c_void,
                   (*net_socksUsername).string as *const libc::c_void,
                   ulen as libc::c_ulong);
        }
        buf[(2i32 + ulen) as usize] = plen as libc::c_uchar;
        if 0 != plen {
            memcpy(&mut buf[(3i32 + ulen) as usize] as *mut libc::c_uchar as
                       *mut libc::c_void,
                   (*net_socksPassword).string as *const libc::c_void,
                   plen as libc::c_ulong);
        }
        if send(socks_socket, buf.as_mut_ptr() as *mut libc::c_void,
                (3i32 + ulen + plen) as size_t, 0i32) == -1i32 as libc::c_long
           {
            Com_Printf(b"NET_OpenSocks: send: %s\n\x00" as *const u8 as
                           *const libc::c_char, NET_ErrorString());
            return
        }
        len =
            recv(socks_socket, buf.as_mut_ptr() as *mut libc::c_void,
                 64i32 as size_t, 0i32) as libc::c_int;
        if len == -1i32 {
            Com_Printf(b"NET_OpenSocks: recv: %s\n\x00" as *const u8 as
                           *const libc::c_char, NET_ErrorString());
            return
        }
        if len != 2i32 || buf[0usize] as libc::c_int != 1i32 {
            Com_Printf(b"NET_OpenSocks: bad response\n\x00" as *const u8 as
                           *const libc::c_char);
            return
        }
        if buf[1usize] as libc::c_int != 0i32 {
            Com_Printf(b"NET_OpenSocks: authentication failed\n\x00" as
                           *const u8 as *const libc::c_char);
            return
        }
    }
    buf[0usize] = 5i32 as libc::c_uchar;
    buf[1usize] = 3i32 as libc::c_uchar;
    buf[2usize] = 0i32 as libc::c_uchar;
    buf[3usize] = 1i32 as libc::c_uchar;
    *(&mut buf[4usize] as *mut libc::c_uchar as *mut libc::c_int) =
        0i32 as in_addr_t as libc::c_int;
    *(&mut buf[8usize] as *mut libc::c_uchar as *mut libc::c_short) =
        htons(port as libc::c_short as uint16_t) as libc::c_short;
    if send(socks_socket, buf.as_mut_ptr() as *mut libc::c_void,
            10i32 as size_t, 0i32) == -1i32 as libc::c_long {
        Com_Printf(b"NET_OpenSocks: send: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
        return
    }
    len =
        recv(socks_socket, buf.as_mut_ptr() as *mut libc::c_void,
             64i32 as size_t, 0i32) as libc::c_int;
    if len == -1i32 {
        Com_Printf(b"NET_OpenSocks: recv: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
        return
    }
    if len < 2i32 || buf[0usize] as libc::c_int != 5i32 {
        Com_Printf(b"NET_OpenSocks: bad response\n\x00" as *const u8 as
                       *const libc::c_char);
        return
    }
    if buf[1usize] as libc::c_int != 0i32 {
        Com_Printf(b"NET_OpenSocks: request denied: %i\n\x00" as *const u8 as
                       *const libc::c_char, buf[1usize] as libc::c_int);
        return
    }
    if buf[3usize] as libc::c_int != 1i32 {
        Com_Printf(b"NET_OpenSocks: relay address is not IPV4: %i\n\x00" as
                       *const u8 as *const libc::c_char,
                   buf[3usize] as libc::c_int);
        return
    }
    (*(&mut socksRelayAddr as *mut sockaddr as *mut sockaddr_in)).sin_family =
        2i32 as sa_family_t;
    (*(&mut socksRelayAddr as *mut sockaddr as
           *mut sockaddr_in)).sin_addr.s_addr =
        *(&mut buf[4usize] as *mut libc::c_uchar as *mut libc::c_int) as
            in_addr_t;
    (*(&mut socksRelayAddr as *mut sockaddr as *mut sockaddr_in)).sin_port =
        *(&mut buf[8usize] as *mut libc::c_uchar as *mut libc::c_short) as
            in_port_t;
    memset((*(&mut socksRelayAddr as *mut sockaddr as
                  *mut sockaddr_in)).sin_zero.as_mut_ptr() as
               *mut libc::c_void, 0i32, 8i32 as libc::c_ulong);
    usingSocks = qtrue;
}
static mut usingSocks: qboolean = qfalse;
static mut socksRelayAddr: sockaddr =
    sockaddr{sa_family: 0, sa_data: [0; 14],};
//=============================================================================
/*
====================
NET_ErrorString
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_ErrorString() -> *mut libc::c_char {
    return strerror(*__errno_location());
}
static mut socks_socket: SOCKET = -1i32;
static mut net_socksPassword: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut net_socksUsername: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut net_socksPort: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut net_socksServer: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut net_socksEnabled: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut net_ip: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
//=============================================================================
/*
====================
NET_IPSocket
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_IPSocket(mut net_interface: *mut libc::c_char,
                                      mut port: libc::c_int,
                                      mut err: *mut libc::c_int) -> SOCKET {
    let mut newsocket: SOCKET = 0;
    let mut address: sockaddr_in =
        sockaddr_in{sin_family: 0,
                    sin_port: 0,
                    sin_addr: in_addr{s_addr: 0,},
                    sin_zero: [0; 8],};
    let mut _true: ioctlarg_t = 1i32;
    let mut i: libc::c_int = 1i32;
    *err = 0i32;
    if !net_interface.is_null() {
        Com_Printf(b"Opening IP socket: %s:%i\n\x00" as *const u8 as
                       *const libc::c_char, net_interface, port);
    } else {
        Com_Printf(b"Opening IP socket: 0.0.0.0:%i\n\x00" as *const u8 as
                       *const libc::c_char, port);
    }
    newsocket =
        socket(2i32, SOCK_DGRAM as libc::c_int, IPPROTO_UDP as libc::c_int);
    if newsocket == -1i32 {
        *err = *__errno_location();
        Com_Printf(b"WARNING: NET_IPSocket: socket: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
        return newsocket
    }
    if ioctl(newsocket, 0x5421i32 as libc::c_ulong,
             &mut _true as *mut ioctlarg_t) == -1i32 {
        Com_Printf(b"WARNING: NET_IPSocket: ioctl FIONBIO: %s\n\x00" as
                       *const u8 as *const libc::c_char, NET_ErrorString());
        *err = *__errno_location();
        close(newsocket);
        return -1i32
    }
    if setsockopt(newsocket, 1i32, 6i32,
                  &mut i as *mut libc::c_int as *mut libc::c_char as
                      *const libc::c_void,
                  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                      socklen_t) == -1i32 {
        Com_Printf(b"WARNING: NET_IPSocket: setsockopt SO_BROADCAST: %s\n\x00"
                       as *const u8 as *const libc::c_char,
                   NET_ErrorString());
    }
    if net_interface.is_null() || 0 == *net_interface.offset(0isize) {
        address.sin_family = 2i32 as sa_family_t;
        address.sin_addr.s_addr = 0i32 as in_addr_t
    } else if 0 ==
                  Sys_StringToSockaddr(net_interface,
                                       &mut address as *mut sockaddr_in as
                                           *mut sockaddr,
                                       ::std::mem::size_of::<sockaddr_in>() as
                                           libc::c_ulong as libc::c_int,
                                       2i32 as sa_family_t) as u64 {
        close(newsocket);
        return -1i32
    }
    if port == -1i32 {
        address.sin_port = 0i32 as in_port_t
    } else { address.sin_port = htons(port as libc::c_short as uint16_t) }
    if bind(newsocket,
            &mut address as *mut sockaddr_in as *mut libc::c_void as
                *const sockaddr,
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                socklen_t) == -1i32 {
        Com_Printf(b"WARNING: NET_IPSocket: bind: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
        *err = *__errno_location();
        close(newsocket);
        return -1i32
    }
    return newsocket;
}
static mut ip6_socket: SOCKET = -1i32;
// And the currently bound address.
static mut boundto: sockaddr_in6 =
    sockaddr_in6{sin6_family: 0,
                 sin6_port: 0,
                 sin6_flowinfo: 0,
                 sin6_addr:
                     in6_addr{__in6_u: unnamed_0{__u6_addr8: [0; 16],},},
                 sin6_scope_id: 0,};
static mut net_ip6: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
/*
====================
NET_IP6Socket
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_IP6Socket(mut net_interface: *mut libc::c_char,
                                       mut port: libc::c_int,
                                       mut bindto: *mut sockaddr_in6,
                                       mut err: *mut libc::c_int) -> SOCKET {
    let mut newsocket: SOCKET = 0;
    let mut address: sockaddr_in6 =
        sockaddr_in6{sin6_family: 0,
                     sin6_port: 0,
                     sin6_flowinfo: 0,
                     sin6_addr:
                         in6_addr{__in6_u: unnamed_0{__u6_addr8: [0; 16],},},
                     sin6_scope_id: 0,};
    let mut _true: ioctlarg_t = 1i32;
    *err = 0i32;
    if !net_interface.is_null() {
        if 0 != Q_CountChar(net_interface, ':' as i32 as libc::c_char) {
            Com_Printf(b"Opening IP6 socket: [%s]:%i\n\x00" as *const u8 as
                           *const libc::c_char, net_interface, port);
        } else {
            Com_Printf(b"Opening IP6 socket: %s:%i\n\x00" as *const u8 as
                           *const libc::c_char, net_interface, port);
        }
    } else {
        Com_Printf(b"Opening IP6 socket: [::]:%i\n\x00" as *const u8 as
                       *const libc::c_char, port);
    }
    newsocket =
        socket(10i32, SOCK_DGRAM as libc::c_int, IPPROTO_UDP as libc::c_int);
    if newsocket == -1i32 {
        *err = *__errno_location();
        Com_Printf(b"WARNING: NET_IP6Socket: socket: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
        return newsocket
    }
    if ioctl(newsocket, 0x5421i32 as libc::c_ulong,
             &mut _true as *mut ioctlarg_t) == -1i32 {
        Com_Printf(b"WARNING: NET_IP6Socket: ioctl FIONBIO: %s\n\x00" as
                       *const u8 as *const libc::c_char, NET_ErrorString());
        *err = *__errno_location();
        close(newsocket);
        return -1i32
    }
    let mut i: libc::c_int = 1i32;
    if setsockopt(newsocket, IPPROTO_IPV6 as libc::c_int, 26i32,
                  &mut i as *mut libc::c_int as *mut libc::c_char as
                      *const libc::c_void,
                  ::std::mem::size_of::<libc::c_int>() as libc::c_ulong as
                      socklen_t) == -1i32 {
        Com_DPrintf(b"WARNING: NET_IP6Socket: setsockopt IPV6_V6ONLY: %s\n\x00"
                        as *const u8 as *const libc::c_char,
                    NET_ErrorString());
    }
    if net_interface.is_null() || 0 == *net_interface.offset(0isize) {
        address.sin6_family = 10i32 as sa_family_t;
        address.sin6_addr = in6addr_any
    } else if 0 ==
                  Sys_StringToSockaddr(net_interface,
                                       &mut address as *mut sockaddr_in6 as
                                           *mut sockaddr,
                                       ::std::mem::size_of::<sockaddr_in6>()
                                           as libc::c_ulong as libc::c_int,
                                       10i32 as sa_family_t) as u64 {
        close(newsocket);
        return -1i32
    }
    if port == -1i32 {
        address.sin6_port = 0i32 as in_port_t
    } else { address.sin6_port = htons(port as libc::c_short as uint16_t) }
    if bind(newsocket,
            &mut address as *mut sockaddr_in6 as *mut libc::c_void as
                *const sockaddr,
            ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                socklen_t) == -1i32 {
        Com_Printf(b"WARNING: NET_IP6Socket: bind: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
        *err = *__errno_location();
        close(newsocket);
        return -1i32
    }
    if !bindto.is_null() { *bindto = address }
    return newsocket;
}
unsafe extern "C" fn NET_GetLocalAddress() {
    let mut ifap: *mut ifaddrs = 0 as *mut ifaddrs;
    let mut search: *mut ifaddrs = 0 as *mut ifaddrs;
    numIP = 0i32;
    if 0 != getifaddrs(&mut ifap) {
        Com_Printf(b"NET_GetLocalAddress: Unable to get list of network interfaces: %s\n\x00"
                       as *const u8 as *const libc::c_char,
                   NET_ErrorString());
    } else {
        search = ifap;
        while !search.is_null() {
            if 0 != (*ifap).ifa_flags & IFF_UP as libc::c_int as libc::c_uint
               {
                NET_AddLocalAddress((*search).ifa_name, (*search).ifa_addr,
                                    (*search).ifa_netmask);
            }
            search = (*search).ifa_next
        }
        freeifaddrs(ifap);
        Sys_ShowIP();
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sys_ShowIP() {
    let mut i: libc::c_int = 0;
    let mut addrbuf: [libc::c_char; 48] = [0; 48];
    i = 0i32;
    while i < numIP {
        Sys_SockaddrToString(addrbuf.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 48]>() as
                                 libc::c_ulong as libc::c_int,
                             &mut localIP[i as usize].addr as
                                 *mut sockaddr_storage as *mut sockaddr);
        if localIP[i as usize].type_0 as libc::c_uint ==
               NA_IP as libc::c_int as libc::c_uint {
            Com_Printf(b"IP: %s\n\x00" as *const u8 as *const libc::c_char,
                       addrbuf.as_mut_ptr());
        } else if localIP[i as usize].type_0 as libc::c_uint ==
                      NA_IP6 as libc::c_int as libc::c_uint {
            Com_Printf(b"IP6: %s\n\x00" as *const u8 as *const libc::c_char,
                       addrbuf.as_mut_ptr());
        }
        i += 1
    };
}
static mut localIP: [nip_localaddr_t; 32] =
    [nip_localaddr_t{ifname: [0; 16],
                     type_0: NA_BAD,
                     family: 0,
                     addr:
                         sockaddr_storage{ss_family: 0,
                                          __ss_padding: [0; 118],
                                          __ss_align: 0,},
                     netmask:
                         sockaddr_storage{ss_family: 0,
                                          __ss_padding: [0; 118],
                                          __ss_align: 0,},}; 32];
/*
=============
Sys_SockaddrToString
=============
*/
unsafe extern "C" fn Sys_SockaddrToString(mut dest: *mut libc::c_char,
                                          mut destlen: libc::c_int,
                                          mut input: *mut sockaddr) {
    let mut inputlen: socklen_t = 0;
    if (*input).sa_family as libc::c_int == 10i32 {
        inputlen =
            ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                socklen_t
    } else {
        inputlen =
            ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as socklen_t
    }
    if 0 !=
           getnameinfo(input, inputlen, dest, destlen as socklen_t,
                       0 as *mut libc::c_char, 0i32 as socklen_t, 1i32) &&
           destlen > 0i32 {
        *dest = '\u{0}' as i32 as libc::c_char
    };
}
static mut numIP: libc::c_int = 0;
/*
=====================
NET_AddLocalAddress
=====================
*/
unsafe extern "C" fn NET_AddLocalAddress(mut ifname: *mut libc::c_char,
                                         mut addr: *mut sockaddr,
                                         mut netmask: *mut sockaddr) {
    let mut addrlen: libc::c_int = 0;
    let mut family: sa_family_t = 0;
    if addr.is_null() || netmask.is_null() || ifname.is_null() { return }
    family = (*addr).sa_family;
    if numIP < 32i32 {
        if family as libc::c_int == 2i32 {
            addrlen =
                ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                    libc::c_int;
            localIP[numIP as usize].type_0 = NA_IP
        } else if family as libc::c_int == 10i32 {
            addrlen =
                ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                    libc::c_int;
            localIP[numIP as usize].type_0 = NA_IP6
        } else { return }
        Q_strncpyz(localIP[numIP as usize].ifname.as_mut_ptr(), ifname,
                   ::std::mem::size_of::<[libc::c_char; 16]>() as
                       libc::c_ulong as libc::c_int);
        localIP[numIP as usize].family = family;
        memcpy(&mut localIP[numIP as usize].addr as *mut sockaddr_storage as
                   *mut libc::c_void, addr as *const libc::c_void,
               addrlen as libc::c_ulong);
        memcpy(&mut localIP[numIP as usize].netmask as *mut sockaddr_storage
                   as *mut libc::c_void, netmask as *const libc::c_void,
               addrlen as libc::c_ulong);
        numIP += 1
    };
}
static mut net_port6: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut net_port: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut multicast6_socket: SOCKET = -1i32;
static mut networkingEnabled: libc::c_int = 0i32;
//===================================================================
/*
====================
NET_GetCvars
====================
*/
unsafe extern "C" fn NET_GetCvars() -> qboolean {
    let mut modified: libc::c_int = 0;
    net_enabled =
        Cvar_Get(b"net_enabled\x00" as *const u8 as *const libc::c_char,
                 b"1\x00" as *const u8 as *const libc::c_char,
                 0x20i32 | 0x1i32);
    modified = (*net_enabled).modified as libc::c_int;
    (*net_enabled).modified = qfalse;
    net_ip =
        Cvar_Get(b"net_ip\x00" as *const u8 as *const libc::c_char,
                 b"0.0.0.0\x00" as *const u8 as *const libc::c_char, 0x20i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_ip).modified as libc::c_uint) as
            libc::c_int as libc::c_int;
    (*net_ip).modified = qfalse;
    net_ip6 =
        Cvar_Get(b"net_ip6\x00" as *const u8 as *const libc::c_char,
                 b"::\x00" as *const u8 as *const libc::c_char, 0x20i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_ip6).modified as libc::c_uint)
            as libc::c_int as libc::c_int;
    (*net_ip6).modified = qfalse;
    net_port =
        Cvar_Get(b"net_port\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, 27960i32), 0x20i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_port).modified as libc::c_uint)
            as libc::c_int as libc::c_int;
    (*net_port).modified = qfalse;
    net_port6 =
        Cvar_Get(b"net_port6\x00" as *const u8 as *const libc::c_char,
                 va(b"%i\x00" as *const u8 as *const libc::c_char as
                        *mut libc::c_char, 27960i32), 0x20i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_port6).modified as libc::c_uint)
            as libc::c_int as libc::c_int;
    (*net_port6).modified = qfalse;
    net_mcast6addr =
        Cvar_Get(b"net_mcast6addr\x00" as *const u8 as *const libc::c_char,
                 b"ff04::696f:7175:616b:6533\x00" as *const u8 as
                     *const libc::c_char, 0x20i32 | 0x1i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_mcast6addr).modified as
                                            libc::c_uint) as libc::c_int as
            libc::c_int;
    (*net_mcast6addr).modified = qfalse;
    net_mcast6iface =
        Cvar_Get(b"net_mcast6iface\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 0x20i32 | 0x1i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_mcast6iface).modified as
                                            libc::c_uint) as libc::c_int as
            libc::c_int;
    (*net_mcast6iface).modified = qfalse;
    net_socksEnabled =
        Cvar_Get(b"net_socksEnabled\x00" as *const u8 as *const libc::c_char,
                 b"0\x00" as *const u8 as *const libc::c_char,
                 0x20i32 | 0x1i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_socksEnabled).modified as
                                            libc::c_uint) as libc::c_int as
            libc::c_int;
    (*net_socksEnabled).modified = qfalse;
    net_socksServer =
        Cvar_Get(b"net_socksServer\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 0x20i32 | 0x1i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_socksServer).modified as
                                            libc::c_uint) as libc::c_int as
            libc::c_int;
    (*net_socksServer).modified = qfalse;
    net_socksPort =
        Cvar_Get(b"net_socksPort\x00" as *const u8 as *const libc::c_char,
                 b"1080\x00" as *const u8 as *const libc::c_char,
                 0x20i32 | 0x1i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_socksPort).modified as
                                            libc::c_uint) as libc::c_int as
            libc::c_int;
    (*net_socksPort).modified = qfalse;
    net_socksUsername =
        Cvar_Get(b"net_socksUsername\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 0x20i32 | 0x1i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_socksUsername).modified as
                                            libc::c_uint) as libc::c_int as
            libc::c_int;
    (*net_socksUsername).modified = qfalse;
    net_socksPassword =
        Cvar_Get(b"net_socksPassword\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char,
                 0x20i32 | 0x1i32);
    modified =
        (modified as
             libc::c_uint).wrapping_add((*net_socksPassword).modified as
                                            libc::c_uint) as libc::c_int as
            libc::c_int;
    (*net_socksPassword).modified = qfalse;
    net_dropsim =
        Cvar_Get(b"net_dropsim\x00" as *const u8 as *const libc::c_char,
                 b"\x00" as *const u8 as *const libc::c_char, 0x100i32);
    return (if 0 != modified {
                qtrue as libc::c_int
            } else { qfalse as libc::c_int }) as qboolean;
}
static mut net_dropsim: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn NET_Shutdown() {
    if 0 == networkingEnabled { return }
    NET_Config(qfalse);
}
#[no_mangle]
pub unsafe extern "C" fn NET_CompareAdr(mut a: netadr_t, mut b: netadr_t)
 -> qboolean {
    if 0 == NET_CompareBaseAdr(a, b) as u64 { return qfalse }
    if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint ||
           a.type_0 as libc::c_uint == NA_IP6 as libc::c_int as libc::c_uint {
        if a.port as libc::c_int == b.port as libc::c_int { return qtrue }
    } else { return qtrue }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn NET_CompareBaseAdr(mut a: netadr_t, mut b: netadr_t)
 -> qboolean {
    return NET_CompareBaseAdrMask(a, b, -1i32);
}
#[no_mangle]
pub unsafe extern "C" fn NET_CompareBaseAdrMask(mut a: netadr_t,
                                                mut b: netadr_t,
                                                mut netmask: libc::c_int)
 -> qboolean {
    let mut cmpmask: byte = 0;
    let mut addra: *mut byte = 0 as *mut byte;
    let mut addrb: *mut byte = 0 as *mut byte;
    let mut curbyte: libc::c_int = 0;
    if a.type_0 as libc::c_uint != b.type_0 as libc::c_uint { return qfalse }
    if a.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint
       {
        return qtrue
    }
    if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint {
        addra = &mut a.ip as *mut [byte; 4] as *mut byte;
        addrb = &mut b.ip as *mut [byte; 4] as *mut byte;
        if netmask < 0i32 || netmask > 32i32 { netmask = 32i32 }
    } else if a.type_0 as libc::c_uint ==
                  NA_IP6 as libc::c_int as libc::c_uint {
        addra = &mut a.ip6 as *mut [byte; 16] as *mut byte;
        addrb = &mut b.ip6 as *mut [byte; 16] as *mut byte;
        if netmask < 0i32 || netmask > 128i32 { netmask = 128i32 }
    } else {
        Com_Printf(b"NET_CompareBaseAdr: bad address type\n\x00" as *const u8
                       as *const libc::c_char);
        return qfalse
    }
    curbyte = netmask >> 3i32;
    if 0 != curbyte &&
           0 !=
               memcmp(addra as *const libc::c_void,
                      addrb as *const libc::c_void, curbyte as libc::c_ulong)
       {
        return qfalse
    }
    netmask &= 0x7i32;
    if 0 != netmask {
        cmpmask = ((1i32 << netmask) - 1i32) as byte;
        cmpmask = ((cmpmask as libc::c_int) << 8i32 - netmask) as byte;
        if *addra.offset(curbyte as isize) as libc::c_int &
               cmpmask as libc::c_int ==
               *addrb.offset(curbyte as isize) as libc::c_int &
                   cmpmask as libc::c_int {
            return qtrue
        }
    } else { return qtrue }
    return qfalse;
}
#[no_mangle]
pub unsafe extern "C" fn NET_IsLocalAddress(mut adr: netadr_t) -> qboolean {
    return (adr.type_0 as libc::c_uint ==
                NA_LOOPBACK as libc::c_int as libc::c_uint) as libc::c_int as
               qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn NET_AdrToString(mut a: netadr_t)
 -> *const libc::c_char {
    static mut s: [libc::c_char; 48] = [0; 48];
    if a.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint
       {
        Com_sprintf(s.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 48]>() as
                        libc::c_ulong as libc::c_int,
                    b"loopback\x00" as *const u8 as *const libc::c_char);
    } else if a.type_0 as libc::c_uint ==
                  NA_BOT as libc::c_int as libc::c_uint {
        Com_sprintf(s.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 48]>() as
                        libc::c_ulong as libc::c_int,
                    b"bot\x00" as *const u8 as *const libc::c_char);
    } else if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint
                  ||
                  a.type_0 as libc::c_uint ==
                      NA_IP6 as libc::c_int as libc::c_uint {
        let mut sadr: sockaddr_storage =
            sockaddr_storage{ss_family: 0,
                             __ss_padding: [0; 118],
                             __ss_align: 0,};
        memset(&mut sadr as *mut sockaddr_storage as *mut libc::c_void, 0i32,
               ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong);
        NetadrToSockadr(&mut a,
                        &mut sadr as *mut sockaddr_storage as *mut sockaddr);
        Sys_SockaddrToString(s.as_mut_ptr(),
                             ::std::mem::size_of::<[libc::c_char; 48]>() as
                                 libc::c_ulong as libc::c_int,
                             &mut sadr as *mut sockaddr_storage as
                                 *mut sockaddr);
    }
    return s.as_mut_ptr();
}
unsafe extern "C" fn NetadrToSockadr(mut a: *mut netadr_t,
                                     mut s: *mut sockaddr) {
    if (*a).type_0 as libc::c_uint ==
           NA_BROADCAST as libc::c_int as libc::c_uint {
        (*(s as *mut sockaddr_in)).sin_family = 2i32 as sa_family_t;
        (*(s as *mut sockaddr_in)).sin_port = (*a).port;
        (*(s as *mut sockaddr_in)).sin_addr.s_addr = 0xffffffffu32
    } else if (*a).type_0 as libc::c_uint ==
                  NA_IP as libc::c_int as libc::c_uint {
        (*(s as *mut sockaddr_in)).sin_family = 2i32 as sa_family_t;
        (*(s as *mut sockaddr_in)).sin_addr.s_addr =
            *(&mut (*a).ip as *mut [byte; 4] as *mut libc::c_int) as
                in_addr_t;
        (*(s as *mut sockaddr_in)).sin_port = (*a).port
    } else if (*a).type_0 as libc::c_uint ==
                  NA_IP6 as libc::c_int as libc::c_uint {
        (*(s as *mut sockaddr_in6)).sin6_family = 10i32 as sa_family_t;
        (*(s as *mut sockaddr_in6)).sin6_addr =
            *(&mut (*a).ip6 as *mut [byte; 16] as *mut in6_addr);
        (*(s as *mut sockaddr_in6)).sin6_port = (*a).port;
        (*(s as *mut sockaddr_in6)).sin6_scope_id = (*a).scope_id as uint32_t
    } else if (*a).type_0 as libc::c_uint ==
                  NA_MULTICAST6 as libc::c_int as libc::c_uint {
        (*(s as *mut sockaddr_in6)).sin6_family = 10i32 as sa_family_t;
        (*(s as *mut sockaddr_in6)).sin6_addr = curgroup.ipv6mr_multiaddr;
        (*(s as *mut sockaddr_in6)).sin6_port = (*a).port
    };
}
#[no_mangle]
pub unsafe extern "C" fn NET_AdrToStringwPort(mut a: netadr_t)
 -> *const libc::c_char {
    static mut s: [libc::c_char; 48] = [0; 48];
    if a.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint
       {
        Com_sprintf(s.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 48]>() as
                        libc::c_ulong as libc::c_int,
                    b"loopback\x00" as *const u8 as *const libc::c_char);
    } else if a.type_0 as libc::c_uint ==
                  NA_BOT as libc::c_int as libc::c_uint {
        Com_sprintf(s.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 48]>() as
                        libc::c_ulong as libc::c_int,
                    b"bot\x00" as *const u8 as *const libc::c_char);
    } else if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint
     {
        Com_sprintf(s.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 48]>() as
                        libc::c_ulong as libc::c_int,
                    b"%s:%hu\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString(a), ntohs(a.port) as libc::c_int);
    } else if a.type_0 as libc::c_uint ==
                  NA_IP6 as libc::c_int as libc::c_uint {
        Com_sprintf(s.as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 48]>() as
                        libc::c_ulong as libc::c_int,
                    b"[%s]:%hu\x00" as *const u8 as *const libc::c_char,
                    NET_AdrToString(a), ntohs(a.port) as libc::c_int);
    }
    return s.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn NET_JoinMulticast6() {
    let mut err: libc::c_int = 0;
    if ip6_socket == -1i32 || multicast6_socket != -1i32 ||
           0 != (*net_enabled).integer & 0x8i32 {
        return
    }
    if *(&mut boundto.sin6_addr as *mut in6_addr as
             *const uint8_t).offset(0isize) as libc::c_int == 0xffi32 ||
           0 !=
               {
                   let mut __a: *const in6_addr =
                       &mut boundto.sin6_addr as *mut in6_addr as
                           *const in6_addr;
                   ((*__a).__in6_u.__u6_addr32[0usize] == 0i32 as libc::c_uint
                        &&
                        (*__a).__in6_u.__u6_addr32[1usize] ==
                            0i32 as libc::c_uint &&
                        (*__a).__in6_u.__u6_addr32[2usize] ==
                            0i32 as libc::c_uint &&
                        (*__a).__in6_u.__u6_addr32[3usize] ==
                            0i32 as libc::c_uint) as libc::c_int
               } {
        multicast6_socket = ip6_socket
    } else {
        multicast6_socket =
            NET_IP6Socket((*net_mcast6addr).string,
                          ntohs(boundto.sin6_port) as libc::c_int,
                          0 as *mut sockaddr_in6, &mut err);
        if multicast6_socket == -1i32 { multicast6_socket = ip6_socket }
    }
    if 0 != curgroup.ipv6mr_interface {
        if setsockopt(multicast6_socket, IPPROTO_IPV6 as libc::c_int, 17i32,
                      &mut curgroup.ipv6mr_interface as *mut libc::c_uint as
                          *mut libc::c_char as *const libc::c_void,
                      ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                          as socklen_t) < 0i32 {
            Com_Printf(b"NET_JoinMulticast6: Couldn\'t set scope on multicast socket: %s\n\x00"
                           as *const u8 as *const libc::c_char,
                       NET_ErrorString());
            if multicast6_socket != ip6_socket {
                close(multicast6_socket);
                multicast6_socket = -1i32;
                return
            }
        }
    }
    if 0 !=
           setsockopt(multicast6_socket, IPPROTO_IPV6 as libc::c_int, 20i32,
                      &mut curgroup as *mut ipv6_mreq as *mut libc::c_char as
                          *const libc::c_void,
                      ::std::mem::size_of::<ipv6_mreq>() as libc::c_ulong as
                          socklen_t) {
        Com_Printf(b"NET_JoinMulticast6: Couldn\'t join multicast group: %s\n\x00"
                       as *const u8 as *const libc::c_char,
                   NET_ErrorString());
        if multicast6_socket != ip6_socket {
            close(multicast6_socket);
            multicast6_socket = -1i32;
            return
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn NET_LeaveMulticast6() {
    if multicast6_socket != -1i32 {
        if multicast6_socket != ip6_socket {
            close(multicast6_socket);
        } else {
            setsockopt(multicast6_socket, IPPROTO_IPV6 as libc::c_int, 21i32,
                       &mut curgroup as *mut ipv6_mreq as *mut libc::c_char as
                           *const libc::c_void,
                       ::std::mem::size_of::<ipv6_mreq>() as libc::c_ulong as
                           socklen_t);
        }
        multicast6_socket = -1i32
    };
}
/*
====================
NET_Event

Called from NET_Sleep which uses select() to determine which sockets have seen action.
====================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_Event(mut fdr: *mut fd_set) {
    let mut bufData: [byte; 16385] = [0; 16385];
    let mut from: netadr_t =
        netadr_t{type_0: NA_BAD,
                 ip: [0; 4],
                 ip6: [0; 16],
                 port: 0,
                 scope_id: 0,};
    let mut netmsg: msg_t =
        msg_t{allowoverflow: qfalse,
              overflowed: qfalse,
              oob: qfalse,
              data: 0 as *mut byte,
              maxsize: 0,
              cursize: 0,
              readcount: 0,
              bit: 0,};
    loop  {
        MSG_Init(&mut netmsg, bufData.as_mut_ptr(),
                 ::std::mem::size_of::<[byte; 16385]>() as libc::c_ulong as
                     libc::c_int);
        if !(0 != NET_GetPacket(&mut from, &mut netmsg, fdr) as u64) {
            break ;
        }
        if (*net_dropsim).value > 0.0f32 && (*net_dropsim).value <= 100.0f32 {
            // com_dropsim->value percent of incoming packets get dropped.
            if rand() <
                   (2147483647i32 as libc::c_double / 100.0f64 *
                        (*net_dropsim).value as libc::c_double) as libc::c_int
               {
                // drop this packet
                continue ;
            }
        }
        if 0 != (*com_sv_running).integer {
            Com_RunAndTimeServerPacket(&mut from, &mut netmsg);
        } else { CL_PacketEvent(from, &mut netmsg); }
    };
}
//=============================================================================
/*
==================
NET_GetPacket

Receive one packet
==================
*/
#[no_mangle]
pub unsafe extern "C" fn NET_GetPacket(mut net_from: *mut netadr_t,
                                       mut net_message: *mut msg_t,
                                       mut fdr: *mut fd_set) -> qboolean {
    let mut ret: libc::c_int = 0;
    let mut from: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut fromlen: socklen_t = 0;
    let mut err: libc::c_int = 0;
    if ip_socket != -1i32 &&
           (*fdr).__fds_bits[(ip_socket /
                                  (8i32 *
                                       ::std::mem::size_of::<__fd_mask>() as
                                           libc::c_ulong as libc::c_int)) as
                                 usize] &
               (1u64 <<
                    ip_socket %
                        (8i32 *
                             ::std::mem::size_of::<__fd_mask>() as
                                 libc::c_ulong as libc::c_int)) as __fd_mask
               != 0i32 as libc::c_long {
        fromlen =
            ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
                socklen_t;
        ret =
            recvfrom(ip_socket, (*net_message).data as *mut libc::c_void,
                     (*net_message).maxsize as size_t, 0i32,
                     &mut from as *mut sockaddr_storage as *mut sockaddr,
                     &mut fromlen) as libc::c_int;
        if ret == -1i32 {
            err = *__errno_location();
            if err != 11i32 && err != 104i32 {
                Com_Printf(b"NET_GetPacket: %s\n\x00" as *const u8 as
                               *const libc::c_char, NET_ErrorString());
            }
        } else {
            memset((*(&mut from as *mut sockaddr_storage as
                          *mut sockaddr_in)).sin_zero.as_mut_ptr() as
                       *mut libc::c_void, 0i32, 8i32 as libc::c_ulong);
            if 0 != usingSocks as libc::c_uint &&
                   memcmp(&mut from as *mut sockaddr_storage as
                              *const libc::c_void,
                          &mut socksRelayAddr as *mut sockaddr as
                              *const libc::c_void, fromlen as libc::c_ulong)
                       == 0i32 {
                if ret < 10i32 ||
                       *(*net_message).data.offset(0isize) as libc::c_int !=
                           0i32 ||
                       *(*net_message).data.offset(1isize) as libc::c_int !=
                           0i32 ||
                       *(*net_message).data.offset(2isize) as libc::c_int !=
                           0i32 ||
                       *(*net_message).data.offset(3isize) as libc::c_int !=
                           1i32 {
                    return qfalse
                }
                (*net_from).type_0 = NA_IP;
                (*net_from).ip[0usize] = *(*net_message).data.offset(4isize);
                (*net_from).ip[1usize] = *(*net_message).data.offset(5isize);
                (*net_from).ip[2usize] = *(*net_message).data.offset(6isize);
                (*net_from).ip[3usize] = *(*net_message).data.offset(7isize);
                (*net_from).port =
                    *(&mut *(*net_message).data.offset(8isize) as *mut byte as
                          *mut libc::c_short) as libc::c_ushort;
                (*net_message).readcount = 10i32
            } else {
                SockadrToNetadr(&mut from as *mut sockaddr_storage as
                                    *mut sockaddr, net_from);
                (*net_message).readcount = 0i32
            }
            if ret >= (*net_message).maxsize {
                Com_Printf(b"Oversize packet from %s\n\x00" as *const u8 as
                               *const libc::c_char,
                           NET_AdrToString(*net_from));
                return qfalse
            }
            (*net_message).cursize = ret;
            return qtrue
        }
    }
    if ip6_socket != -1i32 &&
           (*fdr).__fds_bits[(ip6_socket /
                                  (8i32 *
                                       ::std::mem::size_of::<__fd_mask>() as
                                           libc::c_ulong as libc::c_int)) as
                                 usize] &
               (1u64 <<
                    ip6_socket %
                        (8i32 *
                             ::std::mem::size_of::<__fd_mask>() as
                                 libc::c_ulong as libc::c_int)) as __fd_mask
               != 0i32 as libc::c_long {
        fromlen =
            ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
                socklen_t;
        ret =
            recvfrom(ip6_socket, (*net_message).data as *mut libc::c_void,
                     (*net_message).maxsize as size_t, 0i32,
                     &mut from as *mut sockaddr_storage as *mut sockaddr,
                     &mut fromlen) as libc::c_int;
        if ret == -1i32 {
            err = *__errno_location();
            if err != 11i32 && err != 104i32 {
                Com_Printf(b"NET_GetPacket: %s\n\x00" as *const u8 as
                               *const libc::c_char, NET_ErrorString());
            }
        } else {
            SockadrToNetadr(&mut from as *mut sockaddr_storage as
                                *mut sockaddr, net_from);
            (*net_message).readcount = 0i32;
            if ret >= (*net_message).maxsize {
                Com_Printf(b"Oversize packet from %s\n\x00" as *const u8 as
                               *const libc::c_char,
                           NET_AdrToString(*net_from));
                return qfalse
            }
            (*net_message).cursize = ret;
            return qtrue
        }
    }
    if multicast6_socket != -1i32 && multicast6_socket != ip6_socket &&
           (*fdr).__fds_bits[(multicast6_socket /
                                  (8i32 *
                                       ::std::mem::size_of::<__fd_mask>() as
                                           libc::c_ulong as libc::c_int)) as
                                 usize] &
               (1u64 <<
                    multicast6_socket %
                        (8i32 *
                             ::std::mem::size_of::<__fd_mask>() as
                                 libc::c_ulong as libc::c_int)) as __fd_mask
               != 0i32 as libc::c_long {
        fromlen =
            ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong as
                socklen_t;
        ret =
            recvfrom(multicast6_socket,
                     (*net_message).data as *mut libc::c_void,
                     (*net_message).maxsize as size_t, 0i32,
                     &mut from as *mut sockaddr_storage as *mut sockaddr,
                     &mut fromlen) as libc::c_int;
        if ret == -1i32 {
            err = *__errno_location();
            if err != 11i32 && err != 104i32 {
                Com_Printf(b"NET_GetPacket: %s\n\x00" as *const u8 as
                               *const libc::c_char, NET_ErrorString());
            }
        } else {
            SockadrToNetadr(&mut from as *mut sockaddr_storage as
                                *mut sockaddr, net_from);
            (*net_message).readcount = 0i32;
            if ret >= (*net_message).maxsize {
                Com_Printf(b"Oversize packet from %s\n\x00" as *const u8 as
                               *const libc::c_char,
                           NET_AdrToString(*net_from));
                return qfalse
            }
            (*net_message).cursize = ret;
            return qtrue
        }
    }
    return qfalse;
}
unsafe extern "C" fn SockadrToNetadr(mut s: *mut sockaddr,
                                     mut a: *mut netadr_t) {
    if (*s).sa_family as libc::c_int == 2i32 {
        (*a).type_0 = NA_IP;
        *(&mut (*a).ip as *mut [byte; 4] as *mut libc::c_int) =
            (*(s as *mut sockaddr_in)).sin_addr.s_addr as libc::c_int;
        (*a).port = (*(s as *mut sockaddr_in)).sin_port
    } else if (*s).sa_family as libc::c_int == 10i32 {
        (*a).type_0 = NA_IP6;
        memcpy((*a).ip6.as_mut_ptr() as *mut libc::c_void,
               &mut (*(s as *mut sockaddr_in6)).sin6_addr as *mut in6_addr as
                   *const libc::c_void,
               ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong);
        (*a).port = (*(s as *mut sockaddr_in6)).sin6_port;
        (*a).scope_id =
            (*(s as *mut sockaddr_in6)).sin6_scope_id as libc::c_ulong
    };
}
#[no_mangle]
pub unsafe extern "C" fn Sys_SendPacket(mut length: libc::c_int,
                                        mut data: *const libc::c_void,
                                        mut to: netadr_t) {
    let mut ret: libc::c_int = -1i32;
    let mut addr: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    if to.type_0 as libc::c_uint !=
           NA_BROADCAST as libc::c_int as libc::c_uint &&
           to.type_0 as libc::c_uint != NA_IP as libc::c_int as libc::c_uint
           &&
           to.type_0 as libc::c_uint != NA_IP6 as libc::c_int as libc::c_uint
           &&
           to.type_0 as libc::c_uint !=
               NA_MULTICAST6 as libc::c_int as libc::c_uint {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"Sys_SendPacket: bad address type\x00" as *const u8 as
                      *const libc::c_char);
    }
    if ip_socket == -1i32 &&
           to.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint
           ||
           ip_socket == -1i32 &&
               to.type_0 as libc::c_uint ==
                   NA_BROADCAST as libc::c_int as libc::c_uint ||
           ip6_socket == -1i32 &&
               to.type_0 as libc::c_uint ==
                   NA_IP6 as libc::c_int as libc::c_uint ||
           ip6_socket == -1i32 &&
               to.type_0 as libc::c_uint ==
                   NA_MULTICAST6 as libc::c_int as libc::c_uint {
        return
    }
    if to.type_0 as libc::c_uint ==
           NA_MULTICAST6 as libc::c_int as libc::c_uint &&
           0 != (*net_enabled).integer & 0x8i32 {
        return
    }
    memset(&mut addr as *mut sockaddr_storage as *mut libc::c_void, 0i32,
           ::std::mem::size_of::<sockaddr_storage>() as libc::c_ulong);
    NetadrToSockadr(&mut to,
                    &mut addr as *mut sockaddr_storage as *mut sockaddr);
    if 0 != usingSocks as libc::c_uint &&
           to.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint {
        socksBuf[0usize] = 0i32 as libc::c_char;
        socksBuf[1usize] = 0i32 as libc::c_char;
        socksBuf[2usize] = 0i32 as libc::c_char;
        socksBuf[3usize] = 1i32 as libc::c_char;
        *(&mut socksBuf[4usize] as *mut libc::c_char as *mut libc::c_int) =
            (*(&mut addr as *mut sockaddr_storage as
                   *mut sockaddr_in)).sin_addr.s_addr as libc::c_int;
        *(&mut socksBuf[8usize] as *mut libc::c_char as *mut libc::c_short) =
            (*(&mut addr as *mut sockaddr_storage as
                   *mut sockaddr_in)).sin_port as libc::c_short;
        memcpy(&mut socksBuf[10usize] as *mut libc::c_char as
                   *mut libc::c_void, data, length as libc::c_ulong);
        ret =
            sendto(ip_socket, socksBuf.as_mut_ptr() as *const libc::c_void,
                   (length + 10i32) as size_t, 0i32, &mut socksRelayAddr,
                   ::std::mem::size_of::<sockaddr>() as libc::c_ulong as
                       socklen_t) as libc::c_int
    } else if addr.ss_family as libc::c_int == 2i32 {
        ret =
            sendto(ip_socket, data, length as size_t, 0i32,
                   &mut addr as *mut sockaddr_storage as *mut sockaddr,
                   ::std::mem::size_of::<sockaddr_in>() as libc::c_ulong as
                       socklen_t) as libc::c_int
    } else if addr.ss_family as libc::c_int == 10i32 {
        ret =
            sendto(ip6_socket, data, length as size_t, 0i32,
                   &mut addr as *mut sockaddr_storage as *mut sockaddr,
                   ::std::mem::size_of::<sockaddr_in6>() as libc::c_ulong as
                       socklen_t) as libc::c_int
    }
    if ret == -1i32 {
        let mut err: libc::c_int = *__errno_location();
        if err == 11i32 { return }
        if err == 99i32 &&
               to.type_0 as libc::c_uint ==
                   NA_BROADCAST as libc::c_int as libc::c_uint {
            return
        }
        Com_Printf(b"Sys_SendPacket: %s\n\x00" as *const u8 as
                       *const libc::c_char, NET_ErrorString());
    };
}
//=============================================================================
static mut socksBuf: [libc::c_char; 4096] = [0; 4096];
#[no_mangle]
pub unsafe extern "C" fn Sys_StringToAdr(mut s: *const libc::c_char,
                                         mut a: *mut netadr_t,
                                         mut family: netadrtype_t)
 -> qboolean {
    let mut sadr: sockaddr_storage =
        sockaddr_storage{ss_family: 0,
                         __ss_padding: [0; 118],
                         __ss_align: 0,};
    let mut fam: sa_family_t = 0;
    match family as libc::c_uint {
        4 => { fam = 2i32 as sa_family_t }
        5 => { fam = 10i32 as sa_family_t }
        _ => { fam = 0i32 as sa_family_t }
    }
    if 0 ==
           Sys_StringToSockaddr(s,
                                &mut sadr as *mut sockaddr_storage as
                                    *mut sockaddr,
                                ::std::mem::size_of::<sockaddr_storage>() as
                                    libc::c_ulong as libc::c_int, fam) as u64
       {
        return qfalse
    }
    SockadrToNetadr(&mut sadr as *mut sockaddr_storage as *mut sockaddr, a);
    return qtrue;
}
//Does NOT parse port numbers, only base addresses.
#[no_mangle]
pub unsafe extern "C" fn Sys_IsLANAddress(mut adr: netadr_t) -> qboolean {
    let mut index: libc::c_int = 0;
    let mut run: libc::c_int = 0;
    let mut addrsize: libc::c_int = 0;
    let mut differed: qboolean = qfalse;
    let mut compareadr: *mut byte = 0 as *mut byte;
    let mut comparemask: *mut byte = 0 as *mut byte;
    let mut compareip: *mut byte = 0 as *mut byte;
    if adr.type_0 as libc::c_uint ==
           NA_LOOPBACK as libc::c_int as libc::c_uint {
        return qtrue
    }
    if adr.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint {
        if adr.ip[0usize] as libc::c_int == 10i32 { return qtrue }
        if adr.ip[0usize] as libc::c_int == 172i32 &&
               adr.ip[1usize] as libc::c_int & 0xf0i32 == 16i32 {
            return qtrue
        }
        if adr.ip[0usize] as libc::c_int == 192i32 &&
               adr.ip[1usize] as libc::c_int == 168i32 {
            return qtrue
        }
        if adr.ip[0usize] as libc::c_int == 127i32 { return qtrue }
    } else if adr.type_0 as libc::c_uint ==
                  NA_IP6 as libc::c_int as libc::c_uint {
        if adr.ip6[0usize] as libc::c_int == 0xfei32 &&
               adr.ip6[1usize] as libc::c_int & 0xc0i32 == 0x80i32 {
            return qtrue
        }
        if adr.ip6[0usize] as libc::c_int & 0xfei32 == 0xfci32 {
            return qtrue
        }
    }
    index = 0i32;
    while index < numIP {
        if localIP[index as usize].type_0 as libc::c_uint ==
               adr.type_0 as libc::c_uint {
            if adr.type_0 as libc::c_uint ==
                   NA_IP as libc::c_int as libc::c_uint {
                compareip =
                    &mut (*(&mut localIP[index as usize].addr as
                                *mut sockaddr_storage as
                                *mut sockaddr_in)).sin_addr.s_addr as
                        *mut in_addr_t as *mut byte;
                comparemask =
                    &mut (*(&mut localIP[index as usize].netmask as
                                *mut sockaddr_storage as
                                *mut sockaddr_in)).sin_addr.s_addr as
                        *mut in_addr_t as *mut byte;
                compareadr = adr.ip.as_mut_ptr();
                addrsize =
                    ::std::mem::size_of::<[byte; 4]>() as libc::c_ulong as
                        libc::c_int
            } else {
                compareip =
                    &mut (*(&mut localIP[index as usize].addr as
                                *mut sockaddr_storage as
                                *mut sockaddr_in6)).sin6_addr as *mut in6_addr
                        as *mut byte;
                comparemask =
                    &mut (*(&mut localIP[index as usize].netmask as
                                *mut sockaddr_storage as
                                *mut sockaddr_in6)).sin6_addr as *mut in6_addr
                        as *mut byte;
                compareadr = adr.ip6.as_mut_ptr();
                addrsize =
                    ::std::mem::size_of::<[byte; 16]>() as libc::c_ulong as
                        libc::c_int
            }
            differed = qfalse;
            run = 0i32;
            while run < addrsize {
                if *compareip.offset(run as isize) as libc::c_int &
                       *comparemask.offset(run as isize) as libc::c_int !=
                       *compareadr.offset(run as isize) as libc::c_int &
                           *comparemask.offset(run as isize) as libc::c_int {
                    differed = qtrue;
                    break ;
                } else { run += 1 }
            }
            if 0 == differed as u64 { return qtrue }
        }
        index += 1
    }
    return qfalse;
}