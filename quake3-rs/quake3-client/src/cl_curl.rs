#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(const_raw_ptr_to_usize_cast, custom_attribute, extern_types, libc)]
extern crate libc;
#[header_src = "/usr/include/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    use super::{libc};
}
#[header_src = "/usr/lib/clang/7.0.1/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/sys/select.h"]
pub mod select_h {
    pub type __fd_mask = libc::c_long;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct fd_set {
        pub __fds_bits: [__fd_mask; 16],
    }
    use super::{libc};
}
#[header_src = "/usr/include/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint8_t};
}
#[header_src =
      "ioq3/code/qcommon/q_shared.h"]
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
    pub type qhandle_t = libc::c_int;
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
    pub type connstate_t = libc::c_uint;
    // playing a cinematic or a static pic, not connected to a server
    pub const CA_CINEMATIC: connstate_t = 9;
    // game views should be displayed
    pub const CA_ACTIVE: connstate_t = 8;
    // got gamestate, waiting for first frame
    pub const CA_PRIMED: connstate_t = 7;
    // only during cgame initialization, never during main loop
    pub const CA_LOADING: connstate_t = 6;
    // netchan_t established, getting gamestate
    pub const CA_CONNECTED: connstate_t = 5;
    // sending challenge packets to the server
    pub const CA_CHALLENGING: connstate_t = 4;
    // sending request packets to the server
    pub const CA_CONNECTING: connstate_t = 3;
    // not used any more, was checking cd key 
    pub const CA_AUTHORIZING: connstate_t = 2;
    // not talking to a server
    pub const CA_DISCONNECTED: connstate_t = 1;
    pub const CA_UNINITIALIZED: connstate_t = 0;
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Com_sprintf(dest: *mut libc::c_char, size: libc::c_int,
                           fmt: *const libc::c_char, ...) -> libc::c_int;
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
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
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
    use super::{libc};
    use super::q_shared_h::{byte, qboolean, fileHandle_t, cvar_t};
    extern "C" {
        #[no_mangle]
        pub fn NET_AdrToString(a: netadr_t) -> *const libc::c_char;
        // updates an interpreted modules' version of a cvar
        #[no_mangle]
        pub fn Cvar_Set(var_name: *const libc::c_char,
                        value: *const libc::c_char);
        // don't set the cvar immediately
        #[no_mangle]
        pub fn Cvar_SetValue(var_name: *const libc::c_char,
                             value: libc::c_float);
        // will properly create any needed paths and deal with seperater character issues
        #[no_mangle]
        pub fn FS_SV_FOpenFileWrite(filename: *const libc::c_char)
         -> fileHandle_t;
        #[no_mangle]
        pub fn FS_SV_Rename(from: *const libc::c_char,
                            to: *const libc::c_char, safe: qboolean);
        // returns 1 if a file is in the PAK file, otherwise -1
        #[no_mangle]
        pub fn FS_Write(buffer: *const libc::c_void, len: libc::c_int,
                        f: fileHandle_t) -> libc::c_int;
        // properly handles partial reads and reads from other dlls
        #[no_mangle]
        pub fn FS_FCloseFile(f: fileHandle_t);
        #[no_mangle]
        pub fn Com_DPrintf(fmt: *const libc::c_char, ...);
        #[no_mangle]
        pub static mut com_developer: *mut cvar_t;
    }
}
#[header_src =
      "ioq3/code/renderercommon/tr_types.h"]
pub mod tr_types_h {
    /*
** glconfig_t
**
** Contains variables specific to the OpenGL configuration
** being run right now.  These are constant once the OpenGL
** subsystem is initialized.
*/
    pub type textureCompression_t = libc::c_uint;
    // this is for the GL_EXT_texture_compression_s3tc extension.
    pub const TC_S3TC_ARB: textureCompression_t = 2;
    // this is for the GL_S3_s3tc extension.
    pub const TC_S3TC: textureCompression_t = 1;
    pub const TC_NONE: textureCompression_t = 0;
    pub type glDriverType_t = libc::c_uint;
    // driver is a 3Dfx standalone driver
    pub const GLDRV_VOODOO: glDriverType_t = 2;
    // WARNING: there are tests that check for
								// > GLDRV_ICD for minidriverness, so this
								// should always be the lowest value in this
								// enum set
    // driver is a non-3Dfx standalone driver
    pub const GLDRV_STANDALONE: glDriverType_t = 1;
    // driver is integrated with window system
    pub const GLDRV_ICD: glDriverType_t = 0;
    pub type glHardwareType_t = libc::c_uint;
    // where you don't have src*dst
    pub const GLHW_PERMEDIA2: glHardwareType_t = 4;
    // where you can't modulate alpha on alpha textures
    pub const GLHW_RAGEPRO: glHardwareType_t = 3;
    // the hardware type then there can NOT exist a secondary
							// display adapter
    // where you can't interpolate alpha
    pub const GLHW_RIVA128: glHardwareType_t = 2;
    // Voodoo Banshee or Voodoo3, relevant since if this is
    pub const GLHW_3DFX_2D3D: glHardwareType_t = 1;
    // where everything works the way it should
    pub const GLHW_GENERIC: glHardwareType_t = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct glconfig_t {
        pub renderer_string: [libc::c_char; 1024],
        pub vendor_string: [libc::c_char; 1024],
        pub version_string: [libc::c_char; 1024],
        pub extensions_string: [libc::c_char; 8192],
        pub maxTextureSize: libc::c_int,
        pub numTextureUnits: libc::c_int,
        pub colorBits: libc::c_int,
        pub depthBits: libc::c_int,
        pub stencilBits: libc::c_int,
        pub driverType: glDriverType_t,
        pub hardwareType: glHardwareType_t,
        pub deviceSupportsGamma: qboolean,
        pub textureCompression: textureCompression_t,
        pub textureEnvAddAvailable: qboolean,
        pub vidWidth: libc::c_int,
        pub vidHeight: libc::c_int,
        pub windowAspect: libc::c_float,
        pub displayFrequency: libc::c_int,
        pub isFullscreen: qboolean,
        pub stereoEnabled: qboolean,
        pub smpActive: qboolean,
    }
    use super::{libc};
    use super::q_shared_h::{qboolean};
}
#[header_src = "/usr/include/curl/curl.h"]
pub mod curl_h {
    pub type CURL = ();
    pub type CURLcode = libc::c_uint;
    pub const CURL_LAST: CURLcode = 94;
    pub const CURLE_RECURSIVE_API_CALL: CURLcode = 93;
    pub const CURLE_HTTP2_STREAM: CURLcode = 92;
    pub const CURLE_SSL_INVALIDCERTSTATUS: CURLcode = 91;
    pub const CURLE_SSL_PINNEDPUBKEYNOTMATCH: CURLcode = 90;
    pub const CURLE_NO_CONNECTION_AVAILABLE: CURLcode = 89;
    pub const CURLE_CHUNK_FAILED: CURLcode = 88;
    pub const CURLE_FTP_BAD_FILE_LIST: CURLcode = 87;
    pub const CURLE_RTSP_SESSION_ERROR: CURLcode = 86;
    pub const CURLE_RTSP_CSEQ_ERROR: CURLcode = 85;
    pub const CURLE_FTP_PRET_FAILED: CURLcode = 84;
    pub const CURLE_SSL_ISSUER_ERROR: CURLcode = 83;
    pub const CURLE_SSL_CRL_BADFILE: CURLcode = 82;
    pub const CURLE_AGAIN: CURLcode = 81;
    pub const CURLE_SSL_SHUTDOWN_FAILED: CURLcode = 80;
    pub const CURLE_SSH: CURLcode = 79;
    pub const CURLE_REMOTE_FILE_NOT_FOUND: CURLcode = 78;
    pub const CURLE_SSL_CACERT_BADFILE: CURLcode = 77;
    pub const CURLE_CONV_REQD: CURLcode = 76;
    pub const CURLE_CONV_FAILED: CURLcode = 75;
    pub const CURLE_TFTP_NOSUCHUSER: CURLcode = 74;
    pub const CURLE_REMOTE_FILE_EXISTS: CURLcode = 73;
    pub const CURLE_TFTP_UNKNOWNID: CURLcode = 72;
    pub const CURLE_TFTP_ILLEGAL: CURLcode = 71;
    pub const CURLE_REMOTE_DISK_FULL: CURLcode = 70;
    pub const CURLE_TFTP_PERM: CURLcode = 69;
    pub const CURLE_TFTP_NOTFOUND: CURLcode = 68;
    pub const CURLE_LOGIN_DENIED: CURLcode = 67;
    pub const CURLE_SSL_ENGINE_INITFAILED: CURLcode = 66;
    pub const CURLE_SEND_FAIL_REWIND: CURLcode = 65;
    pub const CURLE_USE_SSL_FAILED: CURLcode = 64;
    pub const CURLE_FILESIZE_EXCEEDED: CURLcode = 63;
    pub const CURLE_LDAP_INVALID_URL: CURLcode = 62;
    pub const CURLE_BAD_CONTENT_ENCODING: CURLcode = 61;
    pub const CURLE_PEER_FAILED_VERIFICATION: CURLcode = 60;
    pub const CURLE_SSL_CIPHER: CURLcode = 59;
    pub const CURLE_SSL_CERTPROBLEM: CURLcode = 58;
    pub const CURLE_OBSOLETE57: CURLcode = 57;
    pub const CURLE_RECV_ERROR: CURLcode = 56;
    pub const CURLE_SEND_ERROR: CURLcode = 55;
    pub const CURLE_SSL_ENGINE_SETFAILED: CURLcode = 54;
    pub const CURLE_SSL_ENGINE_NOTFOUND: CURLcode = 53;
    pub const CURLE_GOT_NOTHING: CURLcode = 52;
    pub const CURLE_OBSOLETE51: CURLcode = 51;
    pub const CURLE_OBSOLETE50: CURLcode = 50;
    pub const CURLE_TELNET_OPTION_SYNTAX: CURLcode = 49;
    pub const CURLE_UNKNOWN_OPTION: CURLcode = 48;
    pub const CURLE_TOO_MANY_REDIRECTS: CURLcode = 47;
    pub const CURLE_OBSOLETE46: CURLcode = 46;
    pub const CURLE_INTERFACE_FAILED: CURLcode = 45;
    pub const CURLE_OBSOLETE44: CURLcode = 44;
    pub const CURLE_BAD_FUNCTION_ARGUMENT: CURLcode = 43;
    pub const CURLE_ABORTED_BY_CALLBACK: CURLcode = 42;
    pub const CURLE_FUNCTION_NOT_FOUND: CURLcode = 41;
    pub const CURLE_OBSOLETE40: CURLcode = 40;
    pub const CURLE_LDAP_SEARCH_FAILED: CURLcode = 39;
    pub const CURLE_LDAP_CANNOT_BIND: CURLcode = 38;
    pub const CURLE_FILE_COULDNT_READ_FILE: CURLcode = 37;
    pub const CURLE_BAD_DOWNLOAD_RESUME: CURLcode = 36;
    pub const CURLE_SSL_CONNECT_ERROR: CURLcode = 35;
    pub const CURLE_HTTP_POST_ERROR: CURLcode = 34;
    pub const CURLE_RANGE_ERROR: CURLcode = 33;
    pub const CURLE_OBSOLETE32: CURLcode = 32;
    pub const CURLE_FTP_COULDNT_USE_REST: CURLcode = 31;
    pub const CURLE_FTP_PORT_FAILED: CURLcode = 30;
    pub const CURLE_OBSOLETE29: CURLcode = 29;
    pub const CURLE_OPERATION_TIMEDOUT: CURLcode = 28;
    pub const CURLE_OUT_OF_MEMORY: CURLcode = 27;
    pub const CURLE_READ_ERROR: CURLcode = 26;
    pub const CURLE_UPLOAD_FAILED: CURLcode = 25;
    pub const CURLE_OBSOLETE24: CURLcode = 24;
    pub const CURLE_WRITE_ERROR: CURLcode = 23;
    pub const CURLE_HTTP_RETURNED_ERROR: CURLcode = 22;
    pub const CURLE_QUOTE_ERROR: CURLcode = 21;
    pub const CURLE_OBSOLETE20: CURLcode = 20;
    pub const CURLE_FTP_COULDNT_RETR_FILE: CURLcode = 19;
    pub const CURLE_PARTIAL_FILE: CURLcode = 18;
    pub const CURLE_FTP_COULDNT_SET_TYPE: CURLcode = 17;
    pub const CURLE_HTTP2: CURLcode = 16;
    pub const CURLE_FTP_CANT_GET_HOST: CURLcode = 15;
    pub const CURLE_FTP_WEIRD_227_FORMAT: CURLcode = 14;
    pub const CURLE_FTP_WEIRD_PASV_REPLY: CURLcode = 13;
    pub const CURLE_FTP_ACCEPT_TIMEOUT: CURLcode = 12;
    pub const CURLE_FTP_WEIRD_PASS_REPLY: CURLcode = 11;
    pub const CURLE_FTP_ACCEPT_FAILED: CURLcode = 10;
    pub const CURLE_REMOTE_ACCESS_DENIED: CURLcode = 9;
    pub const CURLE_WEIRD_SERVER_REPLY: CURLcode = 8;
    pub const CURLE_COULDNT_CONNECT: CURLcode = 7;
    pub const CURLE_COULDNT_RESOLVE_HOST: CURLcode = 6;
    pub const CURLE_COULDNT_RESOLVE_PROXY: CURLcode = 5;
    pub const CURLE_NOT_BUILT_IN: CURLcode = 4;
    pub const CURLE_URL_MALFORMAT: CURLcode = 3;
    pub const CURLE_FAILED_INIT: CURLcode = 2;
    pub const CURLE_UNSUPPORTED_PROTOCOL: CURLcode = 1;
    pub const CURLE_OK: CURLcode = 0;
    pub type CURLoption = libc::c_uint;
    pub const CURLOPT_LASTENTRY: CURLoption = 10283;
    pub const CURLOPT_CURLU: CURLoption = 10282;
    pub const CURLOPT_UPKEEP_INTERVAL_MS: CURLoption = 281;
    pub const CURLOPT_UPLOAD_BUFFERSIZE: CURLoption = 280;
    pub const CURLOPT_DOH_URL: CURLoption = 10279;
    pub const CURLOPT_DISALLOW_USERNAME_IN_URL: CURLoption = 278;
    pub const CURLOPT_PROXY_TLS13_CIPHERS: CURLoption = 10277;
    pub const CURLOPT_TLS13_CIPHERS: CURLoption = 10276;
    pub const CURLOPT_DNS_SHUFFLE_ADDRESSES: CURLoption = 275;
    pub const CURLOPT_HAPROXYPROTOCOL: CURLoption = 274;
    pub const CURLOPT_RESOLVER_START_DATA: CURLoption = 10273;
    pub const CURLOPT_RESOLVER_START_FUNCTION: CURLoption = 20272;
    pub const CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS: CURLoption = 271;
    pub const CURLOPT_TIMEVALUE_LARGE: CURLoption = 30270;
    pub const CURLOPT_MIMEPOST: CURLoption = 10269;
    pub const CURLOPT_SSH_COMPRESSION: CURLoption = 268;
    pub const CURLOPT_SOCKS5_AUTH: CURLoption = 267;
    pub const CURLOPT_REQUEST_TARGET: CURLoption = 10266;
    pub const CURLOPT_SUPPRESS_CONNECT_HEADERS: CURLoption = 265;
    pub const CURLOPT_ABSTRACT_UNIX_SOCKET: CURLoption = 10264;
    pub const CURLOPT_PROXY_PINNEDPUBLICKEY: CURLoption = 10263;
    pub const CURLOPT_PRE_PROXY: CURLoption = 10262;
    pub const CURLOPT_PROXY_SSL_OPTIONS: CURLoption = 261;
    pub const CURLOPT_PROXY_CRLFILE: CURLoption = 10260;
    pub const CURLOPT_PROXY_SSL_CIPHER_LIST: CURLoption = 10259;
    pub const CURLOPT_PROXY_KEYPASSWD: CURLoption = 10258;
    pub const CURLOPT_PROXY_SSLKEYTYPE: CURLoption = 10257;
    pub const CURLOPT_PROXY_SSLKEY: CURLoption = 10256;
    pub const CURLOPT_PROXY_SSLCERTTYPE: CURLoption = 10255;
    pub const CURLOPT_PROXY_SSLCERT: CURLoption = 10254;
    pub const CURLOPT_PROXY_TLSAUTH_TYPE: CURLoption = 10253;
    pub const CURLOPT_PROXY_TLSAUTH_PASSWORD: CURLoption = 10252;
    pub const CURLOPT_PROXY_TLSAUTH_USERNAME: CURLoption = 10251;
    pub const CURLOPT_PROXY_SSLVERSION: CURLoption = 250;
    pub const CURLOPT_PROXY_SSL_VERIFYHOST: CURLoption = 249;
    pub const CURLOPT_PROXY_SSL_VERIFYPEER: CURLoption = 248;
    pub const CURLOPT_PROXY_CAPATH: CURLoption = 10247;
    pub const CURLOPT_PROXY_CAINFO: CURLoption = 10246;
    pub const CURLOPT_KEEP_SENDING_ON_ERROR: CURLoption = 245;
    pub const CURLOPT_TCP_FASTOPEN: CURLoption = 244;
    pub const CURLOPT_CONNECT_TO: CURLoption = 10243;
    pub const CURLOPT_TFTP_NO_OPTIONS: CURLoption = 242;
    pub const CURLOPT_STREAM_DEPENDS_E: CURLoption = 10241;
    pub const CURLOPT_STREAM_DEPENDS: CURLoption = 10240;
    pub const CURLOPT_STREAM_WEIGHT: CURLoption = 239;
    pub const CURLOPT_DEFAULT_PROTOCOL: CURLoption = 10238;
    pub const CURLOPT_PIPEWAIT: CURLoption = 237;
    pub const CURLOPT_SERVICE_NAME: CURLoption = 10236;
    pub const CURLOPT_PROXY_SERVICE_NAME: CURLoption = 10235;
    pub const CURLOPT_PATH_AS_IS: CURLoption = 234;
    pub const CURLOPT_SSL_FALSESTART: CURLoption = 233;
    pub const CURLOPT_SSL_VERIFYSTATUS: CURLoption = 232;
    pub const CURLOPT_UNIX_SOCKET_PATH: CURLoption = 10231;
    pub const CURLOPT_PINNEDPUBLICKEY: CURLoption = 10230;
    pub const CURLOPT_HEADEROPT: CURLoption = 229;
    pub const CURLOPT_PROXYHEADER: CURLoption = 10228;
    pub const CURLOPT_EXPECT_100_TIMEOUT_MS: CURLoption = 227;
    pub const CURLOPT_SSL_ENABLE_ALPN: CURLoption = 226;
    pub const CURLOPT_SSL_ENABLE_NPN: CURLoption = 225;
    pub const CURLOPT_LOGIN_OPTIONS: CURLoption = 10224;
    pub const CURLOPT_DNS_LOCAL_IP6: CURLoption = 10223;
    pub const CURLOPT_DNS_LOCAL_IP4: CURLoption = 10222;
    pub const CURLOPT_DNS_INTERFACE: CURLoption = 10221;
    pub const CURLOPT_XOAUTH2_BEARER: CURLoption = 10220;
    pub const CURLOPT_XFERINFOFUNCTION: CURLoption = 20219;
    pub const CURLOPT_SASL_IR: CURLoption = 218;
    pub const CURLOPT_MAIL_AUTH: CURLoption = 10217;
    pub const CURLOPT_SSL_OPTIONS: CURLoption = 216;
    pub const CURLOPT_TCP_KEEPINTVL: CURLoption = 215;
    pub const CURLOPT_TCP_KEEPIDLE: CURLoption = 214;
    pub const CURLOPT_TCP_KEEPALIVE: CURLoption = 213;
    pub const CURLOPT_ACCEPTTIMEOUT_MS: CURLoption = 212;
    pub const CURLOPT_DNS_SERVERS: CURLoption = 10211;
    pub const CURLOPT_GSSAPI_DELEGATION: CURLoption = 210;
    pub const CURLOPT_CLOSESOCKETDATA: CURLoption = 10209;
    pub const CURLOPT_CLOSESOCKETFUNCTION: CURLoption = 20208;
    pub const CURLOPT_TRANSFER_ENCODING: CURLoption = 207;
    pub const CURLOPT_TLSAUTH_TYPE: CURLoption = 10206;
    pub const CURLOPT_TLSAUTH_PASSWORD: CURLoption = 10205;
    pub const CURLOPT_TLSAUTH_USERNAME: CURLoption = 10204;
    pub const CURLOPT_RESOLVE: CURLoption = 10203;
    pub const CURLOPT_FNMATCH_DATA: CURLoption = 10202;
    pub const CURLOPT_CHUNK_DATA: CURLoption = 10201;
    pub const CURLOPT_FNMATCH_FUNCTION: CURLoption = 20200;
    pub const CURLOPT_CHUNK_END_FUNCTION: CURLoption = 20199;
    pub const CURLOPT_CHUNK_BGN_FUNCTION: CURLoption = 20198;
    pub const CURLOPT_WILDCARDMATCH: CURLoption = 197;
    pub const CURLOPT_INTERLEAVEFUNCTION: CURLoption = 20196;
    pub const CURLOPT_INTERLEAVEDATA: CURLoption = 10195;
    pub const CURLOPT_RTSP_SERVER_CSEQ: CURLoption = 194;
    pub const CURLOPT_RTSP_CLIENT_CSEQ: CURLoption = 193;
    pub const CURLOPT_RTSP_TRANSPORT: CURLoption = 10192;
    pub const CURLOPT_RTSP_STREAM_URI: CURLoption = 10191;
    pub const CURLOPT_RTSP_SESSION_ID: CURLoption = 10190;
    pub const CURLOPT_RTSP_REQUEST: CURLoption = 189;
    pub const CURLOPT_FTP_USE_PRET: CURLoption = 188;
    pub const CURLOPT_MAIL_RCPT: CURLoption = 10187;
    pub const CURLOPT_MAIL_FROM: CURLoption = 10186;
    pub const CURLOPT_SSH_KEYDATA: CURLoption = 10185;
    pub const CURLOPT_SSH_KEYFUNCTION: CURLoption = 20184;
    pub const CURLOPT_SSH_KNOWNHOSTS: CURLoption = 10183;
    pub const CURLOPT_REDIR_PROTOCOLS: CURLoption = 182;
    pub const CURLOPT_PROTOCOLS: CURLoption = 181;
    pub const CURLOPT_SOCKS5_GSSAPI_NEC: CURLoption = 180;
    pub const CURLOPT_SOCKS5_GSSAPI_SERVICE: CURLoption = 10179;
    pub const CURLOPT_TFTP_BLKSIZE: CURLoption = 178;
    pub const CURLOPT_NOPROXY: CURLoption = 10177;
    pub const CURLOPT_PROXYPASSWORD: CURLoption = 10176;
    pub const CURLOPT_PROXYUSERNAME: CURLoption = 10175;
    pub const CURLOPT_PASSWORD: CURLoption = 10174;
    pub const CURLOPT_USERNAME: CURLoption = 10173;
    pub const CURLOPT_CERTINFO: CURLoption = 172;
    pub const CURLOPT_ADDRESS_SCOPE: CURLoption = 171;
    pub const CURLOPT_ISSUERCERT: CURLoption = 10170;
    pub const CURLOPT_CRLFILE: CURLoption = 10169;
    pub const CURLOPT_SEEKDATA: CURLoption = 10168;
    pub const CURLOPT_SEEKFUNCTION: CURLoption = 20167;
    pub const CURLOPT_PROXY_TRANSFER_MODE: CURLoption = 166;
    pub const CURLOPT_COPYPOSTFIELDS: CURLoption = 10165;
    pub const CURLOPT_OPENSOCKETDATA: CURLoption = 10164;
    pub const CURLOPT_OPENSOCKETFUNCTION: CURLoption = 20163;
    pub const CURLOPT_SSH_HOST_PUBLIC_KEY_MD5: CURLoption = 10162;
    pub const CURLOPT_POSTREDIR: CURLoption = 161;
    pub const CURLOPT_NEW_DIRECTORY_PERMS: CURLoption = 160;
    pub const CURLOPT_NEW_FILE_PERMS: CURLoption = 159;
    pub const CURLOPT_HTTP_CONTENT_DECODING: CURLoption = 158;
    pub const CURLOPT_HTTP_TRANSFER_DECODING: CURLoption = 157;
    pub const CURLOPT_CONNECTTIMEOUT_MS: CURLoption = 156;
    pub const CURLOPT_TIMEOUT_MS: CURLoption = 155;
    pub const CURLOPT_FTP_SSL_CCC: CURLoption = 154;
    pub const CURLOPT_SSH_PRIVATE_KEYFILE: CURLoption = 10153;
    pub const CURLOPT_SSH_PUBLIC_KEYFILE: CURLoption = 10152;
    pub const CURLOPT_SSH_AUTH_TYPES: CURLoption = 151;
    pub const CURLOPT_SSL_SESSIONID_CACHE: CURLoption = 150;
    pub const CURLOPT_SOCKOPTDATA: CURLoption = 10149;
    pub const CURLOPT_SOCKOPTFUNCTION: CURLoption = 20148;
    pub const CURLOPT_FTP_ALTERNATIVE_TO_USER: CURLoption = 10147;
    pub const CURLOPT_MAX_RECV_SPEED_LARGE: CURLoption = 30146;
    pub const CURLOPT_MAX_SEND_SPEED_LARGE: CURLoption = 30145;
    pub const CURLOPT_CONV_FROM_UTF8_FUNCTION: CURLoption = 20144;
    pub const CURLOPT_CONV_TO_NETWORK_FUNCTION: CURLoption = 20143;
    pub const CURLOPT_CONV_FROM_NETWORK_FUNCTION: CURLoption = 20142;
    pub const CURLOPT_CONNECT_ONLY: CURLoption = 141;
    pub const CURLOPT_LOCALPORTRANGE: CURLoption = 140;
    pub const CURLOPT_LOCALPORT: CURLoption = 139;
    pub const CURLOPT_FTP_FILEMETHOD: CURLoption = 138;
    pub const CURLOPT_FTP_SKIP_PASV_IP: CURLoption = 137;
    pub const CURLOPT_IGNORE_CONTENT_LENGTH: CURLoption = 136;
    pub const CURLOPT_COOKIELIST: CURLoption = 10135;
    pub const CURLOPT_FTP_ACCOUNT: CURLoption = 10134;
    pub const CURLOPT_IOCTLDATA: CURLoption = 10131;
    pub const CURLOPT_IOCTLFUNCTION: CURLoption = 20130;
    pub const CURLOPT_FTPSSLAUTH: CURLoption = 129;
    pub const CURLOPT_TCP_NODELAY: CURLoption = 121;
    pub const CURLOPT_POSTFIELDSIZE_LARGE: CURLoption = 30120;
    pub const CURLOPT_USE_SSL: CURLoption = 119;
    pub const CURLOPT_NETRC_FILE: CURLoption = 10118;
    pub const CURLOPT_MAXFILESIZE_LARGE: CURLoption = 30117;
    pub const CURLOPT_RESUME_FROM_LARGE: CURLoption = 30116;
    pub const CURLOPT_INFILESIZE_LARGE: CURLoption = 30115;
    pub const CURLOPT_MAXFILESIZE: CURLoption = 114;
    pub const CURLOPT_IPRESOLVE: CURLoption = 113;
    pub const CURLOPT_FTP_RESPONSE_TIMEOUT: CURLoption = 112;
    pub const CURLOPT_PROXYAUTH: CURLoption = 111;
    pub const CURLOPT_FTP_CREATE_MISSING_DIRS: CURLoption = 110;
    pub const CURLOPT_SSL_CTX_DATA: CURLoption = 10109;
    pub const CURLOPT_SSL_CTX_FUNCTION: CURLoption = 20108;
    pub const CURLOPT_HTTPAUTH: CURLoption = 107;
    pub const CURLOPT_FTP_USE_EPRT: CURLoption = 106;
    pub const CURLOPT_UNRESTRICTED_AUTH: CURLoption = 105;
    pub const CURLOPT_HTTP200ALIASES: CURLoption = 10104;
    pub const CURLOPT_PRIVATE: CURLoption = 10103;
    pub const CURLOPT_ACCEPT_ENCODING: CURLoption = 10102;
    pub const CURLOPT_PROXYTYPE: CURLoption = 101;
    pub const CURLOPT_SHARE: CURLoption = 10100;
    pub const CURLOPT_NOSIGNAL: CURLoption = 99;
    pub const CURLOPT_BUFFERSIZE: CURLoption = 98;
    pub const CURLOPT_CAPATH: CURLoption = 10097;
    pub const CURLOPT_COOKIESESSION: CURLoption = 96;
    pub const CURLOPT_DEBUGDATA: CURLoption = 10095;
    pub const CURLOPT_DEBUGFUNCTION: CURLoption = 20094;
    pub const CURLOPT_PREQUOTE: CURLoption = 10093;
    pub const CURLOPT_DNS_CACHE_TIMEOUT: CURLoption = 92;
    pub const CURLOPT_DNS_USE_GLOBAL_CACHE: CURLoption = 91;
    pub const CURLOPT_SSLENGINE_DEFAULT: CURLoption = 90;
    pub const CURLOPT_SSLENGINE: CURLoption = 10089;
    pub const CURLOPT_SSLKEYTYPE: CURLoption = 10088;
    pub const CURLOPT_SSLKEY: CURLoption = 10087;
    pub const CURLOPT_SSLCERTTYPE: CURLoption = 10086;
    pub const CURLOPT_FTP_USE_EPSV: CURLoption = 85;
    pub const CURLOPT_HTTP_VERSION: CURLoption = 84;
    pub const CURLOPT_SSL_CIPHER_LIST: CURLoption = 10083;
    pub const CURLOPT_COOKIEJAR: CURLoption = 10082;
    pub const CURLOPT_SSL_VERIFYHOST: CURLoption = 81;
    pub const CURLOPT_HTTPGET: CURLoption = 80;
    pub const CURLOPT_HEADERFUNCTION: CURLoption = 20079;
    pub const CURLOPT_CONNECTTIMEOUT: CURLoption = 78;
    pub const CURLOPT_EGDSOCKET: CURLoption = 10077;
    pub const CURLOPT_RANDOM_FILE: CURLoption = 10076;
    pub const CURLOPT_FORBID_REUSE: CURLoption = 75;
    pub const CURLOPT_FRESH_CONNECT: CURLoption = 74;
    pub const CURLOPT_OBSOLETE72: CURLoption = 72;
    pub const CURLOPT_MAXCONNECTS: CURLoption = 71;
    pub const CURLOPT_TELNETOPTIONS: CURLoption = 10070;
    pub const CURLOPT_FILETIME: CURLoption = 69;
    pub const CURLOPT_MAXREDIRS: CURLoption = 68;
    pub const CURLOPT_CAINFO: CURLoption = 10065;
    pub const CURLOPT_SSL_VERIFYPEER: CURLoption = 64;
    pub const CURLOPT_KRBLEVEL: CURLoption = 10063;
    pub const CURLOPT_INTERFACE: CURLoption = 10062;
    pub const CURLOPT_HTTPPROXYTUNNEL: CURLoption = 61;
    pub const CURLOPT_POSTFIELDSIZE: CURLoption = 60;
    pub const CURLOPT_PROXYPORT: CURLoption = 59;
    pub const CURLOPT_AUTOREFERER: CURLoption = 58;
    pub const CURLOPT_PROGRESSDATA: CURLoption = 10057;
    pub const CURLOPT_PROGRESSFUNCTION: CURLoption = 20056;
    pub const CURLOPT_PUT: CURLoption = 54;
    pub const CURLOPT_TRANSFERTEXT: CURLoption = 53;
    pub const CURLOPT_FOLLOWLOCATION: CURLoption = 52;
    pub const CURLOPT_NETRC: CURLoption = 51;
    pub const CURLOPT_APPEND: CURLoption = 50;
    pub const CURLOPT_DIRLISTONLY: CURLoption = 48;
    pub const CURLOPT_POST: CURLoption = 47;
    pub const CURLOPT_UPLOAD: CURLoption = 46;
    pub const CURLOPT_FAILONERROR: CURLoption = 45;
    pub const CURLOPT_NOBODY: CURLoption = 44;
    pub const CURLOPT_NOPROGRESS: CURLoption = 43;
    pub const CURLOPT_HEADER: CURLoption = 42;
    pub const CURLOPT_VERBOSE: CURLoption = 41;
    pub const CURLOPT_OBSOLETE40: CURLoption = 10040;
    pub const CURLOPT_POSTQUOTE: CURLoption = 10039;
    pub const CURLOPT_STDERR: CURLoption = 10037;
    pub const CURLOPT_CUSTOMREQUEST: CURLoption = 10036;
    pub const CURLOPT_TIMEVALUE: CURLoption = 34;
    pub const CURLOPT_TIMECONDITION: CURLoption = 33;
    pub const CURLOPT_SSLVERSION: CURLoption = 32;
    pub const CURLOPT_COOKIEFILE: CURLoption = 10031;
    pub const CURLOPT_HEADERDATA: CURLoption = 10029;
    pub const CURLOPT_QUOTE: CURLoption = 10028;
    pub const CURLOPT_CRLF: CURLoption = 27;
    pub const CURLOPT_KEYPASSWD: CURLoption = 10026;
    pub const CURLOPT_SSLCERT: CURLoption = 10025;
    pub const CURLOPT_HTTPPOST: CURLoption = 10024;
    pub const CURLOPT_HTTPHEADER: CURLoption = 10023;
    pub const CURLOPT_COOKIE: CURLoption = 10022;
    pub const CURLOPT_RESUME_FROM: CURLoption = 21;
    pub const CURLOPT_LOW_SPEED_TIME: CURLoption = 20;
    pub const CURLOPT_LOW_SPEED_LIMIT: CURLoption = 19;
    pub const CURLOPT_USERAGENT: CURLoption = 10018;
    pub const CURLOPT_FTPPORT: CURLoption = 10017;
    pub const CURLOPT_REFERER: CURLoption = 10016;
    pub const CURLOPT_POSTFIELDS: CURLoption = 10015;
    pub const CURLOPT_INFILESIZE: CURLoption = 14;
    pub const CURLOPT_TIMEOUT: CURLoption = 13;
    pub const CURLOPT_READFUNCTION: CURLoption = 20012;
    pub const CURLOPT_WRITEFUNCTION: CURLoption = 20011;
    pub const CURLOPT_ERRORBUFFER: CURLoption = 10010;
    pub const CURLOPT_READDATA: CURLoption = 10009;
    pub const CURLOPT_RANGE: CURLoption = 10007;
    pub const CURLOPT_PROXYUSERPWD: CURLoption = 10006;
    pub const CURLOPT_USERPWD: CURLoption = 10005;
    pub const CURLOPT_PROXY: CURLoption = 10004;
    pub const CURLOPT_PORT: CURLoption = 3;
    pub const CURLOPT_URL: CURLoption = 10002;
    pub const CURLOPT_WRITEDATA: CURLoption = 10001;
    pub type CURLINFO = libc::c_uint;
    pub const CURLINFO_LASTONE: CURLINFO = 56;
    pub const CURLINFO_APPCONNECT_TIME_T: CURLINFO = 6291512;
    pub const CURLINFO_REDIRECT_TIME_T: CURLINFO = 6291511;
    pub const CURLINFO_STARTTRANSFER_TIME_T: CURLINFO = 6291510;
    pub const CURLINFO_PRETRANSFER_TIME_T: CURLINFO = 6291509;
    pub const CURLINFO_CONNECT_TIME_T: CURLINFO = 6291508;
    pub const CURLINFO_NAMELOOKUP_TIME_T: CURLINFO = 6291507;
    pub const CURLINFO_TOTAL_TIME_T: CURLINFO = 6291506;
    pub const CURLINFO_SCHEME: CURLINFO = 1048625;
    pub const CURLINFO_PROTOCOL: CURLINFO = 2097200;
    pub const CURLINFO_PROXY_SSL_VERIFYRESULT: CURLINFO = 2097199;
    pub const CURLINFO_HTTP_VERSION: CURLINFO = 2097198;
    pub const CURLINFO_TLS_SSL_PTR: CURLINFO = 4194349;
    pub const CURLINFO_ACTIVESOCKET: CURLINFO = 5242924;
    pub const CURLINFO_TLS_SESSION: CURLINFO = 4194347;
    pub const CURLINFO_LOCAL_PORT: CURLINFO = 2097194;
    pub const CURLINFO_LOCAL_IP: CURLINFO = 1048617;
    pub const CURLINFO_PRIMARY_PORT: CURLINFO = 2097192;
    pub const CURLINFO_RTSP_CSEQ_RECV: CURLINFO = 2097191;
    pub const CURLINFO_RTSP_SERVER_CSEQ: CURLINFO = 2097190;
    pub const CURLINFO_RTSP_CLIENT_CSEQ: CURLINFO = 2097189;
    pub const CURLINFO_RTSP_SESSION_ID: CURLINFO = 1048612;
    pub const CURLINFO_CONDITION_UNMET: CURLINFO = 2097187;
    pub const CURLINFO_CERTINFO: CURLINFO = 4194338;
    pub const CURLINFO_APPCONNECT_TIME: CURLINFO = 3145761;
    pub const CURLINFO_PRIMARY_IP: CURLINFO = 1048608;
    pub const CURLINFO_REDIRECT_URL: CURLINFO = 1048607;
    pub const CURLINFO_FTP_ENTRY_PATH: CURLINFO = 1048606;
    pub const CURLINFO_LASTSOCKET: CURLINFO = 2097181;
    pub const CURLINFO_COOKIELIST: CURLINFO = 4194332;
    pub const CURLINFO_SSL_ENGINES: CURLINFO = 4194331;
    pub const CURLINFO_NUM_CONNECTS: CURLINFO = 2097178;
    pub const CURLINFO_OS_ERRNO: CURLINFO = 2097177;
    pub const CURLINFO_PROXYAUTH_AVAIL: CURLINFO = 2097176;
    pub const CURLINFO_HTTPAUTH_AVAIL: CURLINFO = 2097175;
    pub const CURLINFO_HTTP_CONNECTCODE: CURLINFO = 2097174;
    pub const CURLINFO_PRIVATE: CURLINFO = 1048597;
    pub const CURLINFO_REDIRECT_COUNT: CURLINFO = 2097172;
    pub const CURLINFO_REDIRECT_TIME: CURLINFO = 3145747;
    pub const CURLINFO_CONTENT_TYPE: CURLINFO = 1048594;
    pub const CURLINFO_STARTTRANSFER_TIME: CURLINFO = 3145745;
    pub const CURLINFO_CONTENT_LENGTH_UPLOAD_T: CURLINFO = 6291472;
    pub const CURLINFO_CONTENT_LENGTH_UPLOAD: CURLINFO = 3145744;
    pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD_T: CURLINFO = 6291471;
    pub const CURLINFO_CONTENT_LENGTH_DOWNLOAD: CURLINFO = 3145743;
    pub const CURLINFO_FILETIME_T: CURLINFO = 6291470;
    pub const CURLINFO_FILETIME: CURLINFO = 2097166;
    pub const CURLINFO_SSL_VERIFYRESULT: CURLINFO = 2097165;
    pub const CURLINFO_REQUEST_SIZE: CURLINFO = 2097164;
    pub const CURLINFO_HEADER_SIZE: CURLINFO = 2097163;
    pub const CURLINFO_SPEED_UPLOAD_T: CURLINFO = 6291466;
    pub const CURLINFO_SPEED_UPLOAD: CURLINFO = 3145738;
    pub const CURLINFO_SPEED_DOWNLOAD_T: CURLINFO = 6291465;
    pub const CURLINFO_SPEED_DOWNLOAD: CURLINFO = 3145737;
    pub const CURLINFO_SIZE_DOWNLOAD_T: CURLINFO = 6291464;
    pub const CURLINFO_SIZE_DOWNLOAD: CURLINFO = 3145736;
    pub const CURLINFO_SIZE_UPLOAD_T: CURLINFO = 6291463;
    pub const CURLINFO_SIZE_UPLOAD: CURLINFO = 3145735;
    pub const CURLINFO_PRETRANSFER_TIME: CURLINFO = 3145734;
    pub const CURLINFO_CONNECT_TIME: CURLINFO = 3145733;
    pub const CURLINFO_NAMELOOKUP_TIME: CURLINFO = 3145732;
    pub const CURLINFO_TOTAL_TIME: CURLINFO = 3145731;
    pub const CURLINFO_RESPONSE_CODE: CURLINFO = 2097154;
    pub const CURLINFO_EFFECTIVE_URL: CURLINFO = 1048577;
    pub const CURLINFO_NONE: CURLINFO = 0;
    use super::{libc};
}
#[header_src = "/usr/include/curl/multi.h"]
pub mod multi_h {
    pub type CURLM = ();
    pub type CURLMcode = libc::c_int;
    pub const CURLM_LAST: CURLMcode = 9;
    pub const CURLM_RECURSIVE_API_CALL: CURLMcode = 8;
    pub const CURLM_ADDED_ALREADY: CURLMcode = 7;
    pub const CURLM_UNKNOWN_OPTION: CURLMcode = 6;
    pub const CURLM_BAD_SOCKET: CURLMcode = 5;
    pub const CURLM_INTERNAL_ERROR: CURLMcode = 4;
    pub const CURLM_OUT_OF_MEMORY: CURLMcode = 3;
    pub const CURLM_BAD_EASY_HANDLE: CURLMcode = 2;
    pub const CURLM_BAD_HANDLE: CURLMcode = 1;
    pub const CURLM_OK: CURLMcode = 0;
    pub const CURLM_CALL_MULTI_PERFORM: CURLMcode = -1;
    pub type CURLMSG = libc::c_uint;
    pub const CURLMSG_LAST: CURLMSG = 2;
    pub const CURLMSG_DONE: CURLMSG = 1;
    pub const CURLMSG_NONE: CURLMSG = 0;
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct CURLMsg {
        pub msg: CURLMSG,
        pub easy_handle: *mut libc::c_void,
        pub data: unnamed_0,
    }
    #[derive
    ( Copy , Clone )]
    #[repr
    ( C )]
    pub union unnamed_0 {
        pub whatever: *mut libc::c_void,
        pub result: CURLcode,
    }
    use super::{libc};
    use super::curl_h::{CURL, CURLcode};
}
#[header_src =
      "ioq3/code/client/client.h"]
pub mod client_h {
    /*
=============================================================================

the clientConnection_t structure is wiped when disconnecting from a server,
either to go to a full screen console, play a demo, or connect to a different server

A connection can be to either a server through the network layer or a
demo through a file.

=============================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientConnection_t {
        pub state: connstate_t,
        pub clientNum: libc::c_int,
        pub lastPacketSentTime: libc::c_int,
        pub lastPacketTime: libc::c_int,
        pub servername: [libc::c_char; 4096],
        pub serverAddress: netadr_t,
        pub connectTime: libc::c_int,
        pub connectPacketCount: libc::c_int,
        pub serverMessage: [libc::c_char; 1024],
        pub challenge: libc::c_int,
        pub checksumFeed: libc::c_int,
        pub reliableSequence: libc::c_int,
        pub reliableAcknowledge: libc::c_int,
        pub reliableCommands: [[libc::c_char; 1024]; 64],
        pub serverMessageSequence: libc::c_int,
        pub serverCommandSequence: libc::c_int,
        pub lastExecutedServerCommand: libc::c_int,
        pub serverCommands: [[libc::c_char; 1024]; 64],
        pub download: fileHandle_t,
        pub downloadTempName: [libc::c_char; 4096],
        pub downloadName: [libc::c_char; 4096],
        pub cURLEnabled: qboolean,
        pub cURLUsed: qboolean,
        pub cURLDisconnected: qboolean,
        pub downloadURL: [libc::c_char; 4096],
        pub downloadCURL: *mut libc::c_void,
        pub downloadCURLM: *mut libc::c_void,
        pub sv_allowDownload: libc::c_int,
        pub sv_dlURL: [libc::c_char; 256],
        pub downloadNumber: libc::c_int,
        pub downloadBlock: libc::c_int,
        pub downloadCount: libc::c_int,
        pub downloadSize: libc::c_int,
        pub downloadList: [libc::c_char; 1024],
        pub downloadRestart: qboolean,
        pub demoName: [libc::c_char; 64],
        pub spDemoRecording: qboolean,
        pub demorecording: qboolean,
        pub demoplaying: qboolean,
        pub demowaiting: qboolean,
        pub firstDemoFrameSkipped: qboolean,
        pub demofile: fileHandle_t,
        pub timeDemoFrames: libc::c_int,
        pub timeDemoStart: libc::c_int,
        pub timeDemoBaseTime: libc::c_int,
        pub timeDemoLastFrame: libc::c_int,
        pub timeDemoMinDuration: libc::c_int,
        pub timeDemoMaxDuration: libc::c_int,
        pub timeDemoDurations: [libc::c_uchar; 4096],
        pub aviVideoFrameRemainder: libc::c_float,
        pub aviSoundFrameRemainder: libc::c_float,
        pub voipEnabled: qboolean,
        pub voipCodecInitialized: qboolean,
        pub opusDecoder: [*mut OpusDecoder; 64],
        pub voipIncomingGeneration: [byte; 64],
        pub voipIncomingSequence: [libc::c_int; 64],
        pub voipGain: [libc::c_float; 64],
        pub voipIgnore: [qboolean; 64],
        pub voipMuteAll: qboolean,
        pub voipTargets: [uint8_t; 8],
        pub voipFlags: uint8_t,
        pub opusEncoder: *mut OpusEncoder,
        pub voipOutgoingDataSize: libc::c_int,
        pub voipOutgoingDataFrames: libc::c_int,
        pub voipOutgoingSequence: libc::c_int,
        pub voipOutgoingGeneration: byte,
        pub voipOutgoingData: [byte; 1024],
        pub voipPower: libc::c_float,
        pub compat: qboolean,
        pub netchan: netchan_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct clientStatic_t {
        pub cddialog: qboolean,
        pub rendererStarted: qboolean,
        pub soundStarted: qboolean,
        pub soundRegistered: qboolean,
        pub uiStarted: qboolean,
        pub cgameStarted: qboolean,
        pub framecount: libc::c_int,
        pub frametime: libc::c_int,
        pub realtime: libc::c_int,
        pub realFrametime: libc::c_int,
        pub numlocalservers: libc::c_int,
        pub localServers: [serverInfo_t; 128],
        pub numglobalservers: libc::c_int,
        pub globalServers: [serverInfo_t; 4096],
        pub numGlobalServerAddresses: libc::c_int,
        pub globalServerAddresses: [netadr_t; 4096],
        pub numfavoriteservers: libc::c_int,
        pub favoriteServers: [serverInfo_t; 128],
        pub pingUpdateSource: libc::c_int,
        pub updateServer: netadr_t,
        pub updateChallenge: [libc::c_char; 1024],
        pub updateInfoString: [libc::c_char; 1024],
        pub authorizeServer: netadr_t,
        pub rconAddress: netadr_t,
        pub glconfig: glconfig_t,
        pub charSetShader: qhandle_t,
        pub whiteShader: qhandle_t,
        pub consoleShader: qhandle_t,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct serverInfo_t {
        pub adr: netadr_t,
        pub hostName: [libc::c_char; 32],
        pub mapName: [libc::c_char; 32],
        pub game: [libc::c_char; 32],
        pub netType: libc::c_int,
        pub gameType: libc::c_int,
        pub clients: libc::c_int,
        pub maxClients: libc::c_int,
        pub minPing: libc::c_int,
        pub maxPing: libc::c_int,
        pub ping: libc::c_int,
        pub visible: qboolean,
        pub punkbuster: libc::c_int,
        pub g_humanplayers: libc::c_int,
        pub g_needpass: libc::c_int,
    }
    use super::q_shared_h::{connstate_t, fileHandle_t, qboolean, byte,
                            qhandle_t};
    use super::{libc};
    use super::qcommon_h::{netadr_t, netchan_t};
    use super::curl_h::{CURL};
    use super::multi_h::{CURLM};
    use super::opus_h::{OpusDecoder, OpusEncoder};
    use super::stdint_uintn_h::{uint8_t};
    use super::tr_types_h::{glconfig_t};
    extern "C" {
        #[no_mangle]
        pub static mut clc: clientConnection_t;
        #[no_mangle]
        pub fn CL_WritePacket();
        // 20ms at 48k
        // 3 frame is 60ms of audio, the max opus will encode at once
        //=================================================
        //
// cl_main
//
        #[no_mangle]
        pub fn CL_AddReliableCommand(cmd: *const libc::c_char,
                                     isDisconnectCmd: qboolean);
        #[no_mangle]
        pub static mut cls: clientStatic_t;
        #[no_mangle]
        pub fn CL_NextDownload();
    }
}
#[header_src = "/usr/include/opus/opus.h"]
pub mod opus_h {
    extern "C" {
        /* Copyright (c) 2010-2011 Xiph.Org Foundation, Skype Limited
   Written by Jean-Marc Valin and Koen Vos */
/*
   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:

   - Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.

   THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
   ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
   LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
   A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER
   OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
   PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
   PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
   LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
   NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
   SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/
        /* *
 * @file opus.h
 * @brief Opus reference implementation API
 */
        /* *
 * @mainpage Opus
 *
 * The Opus codec is designed for interactive speech and audio transmission over the Internet.
 * It is designed by the IETF Codec Working Group and incorporates technology from
 * Skype's SILK codec and Xiph.Org's CELT codec.
 *
 * The Opus codec is designed to handle a wide range of interactive audio applications,
 * including Voice over IP, videoconferencing, in-game chat, and even remote live music
 * performances. It can scale from low bit-rate narrowband speech to very high quality
 * stereo music. Its main features are:

 * @li Sampling rates from 8 to 48 kHz
 * @li Bit-rates from 6 kb/s to 510 kb/s
 * @li Support for both constant bit-rate (CBR) and variable bit-rate (VBR)
 * @li Audio bandwidth from narrowband to full-band
 * @li Support for speech and music
 * @li Support for mono and stereo
 * @li Support for multichannel (up to 255 channels)
 * @li Frame sizes from 2.5 ms to 60 ms
 * @li Good loss robustness and packet loss concealment (PLC)
 * @li Floating point and fixed-point implementation
 *
 * Documentation sections:
 * @li @ref opus_encoder
 * @li @ref opus_decoder
 * @li @ref opus_repacketizer
 * @li @ref opus_multistream
 * @li @ref opus_libinfo
 * @li @ref opus_custom
 */
        /* * @defgroup opus_encoder Opus Encoder
  * @{
  *
  * @brief This page describes the process and functions used to encode Opus.
  *
  * Since Opus is a stateful codec, the encoding process starts with creating an encoder
  * state. This can be done with:
  *
  * @code
  * int          error;
  * OpusEncoder *enc;
  * enc = opus_encoder_create(Fs, channels, application, &error);
  * @endcode
  *
  * From this point, @c enc can be used for encoding an audio stream. An encoder state
  * @b must @b not be used for more than one stream at the same time. Similarly, the encoder
  * state @b must @b not be re-initialized for each frame.
  *
  * While opus_encoder_create() allocates memory for the state, it's also possible
  * to initialize pre-allocated memory:
  *
  * @code
  * int          size;
  * int          error;
  * OpusEncoder *enc;
  * size = opus_encoder_get_size(channels);
  * enc = malloc(size);
  * error = opus_encoder_init(enc, Fs, channels, application);
  * @endcode
  *
  * where opus_encoder_get_size() returns the required size for the encoder state. Note that
  * future versions of this code may change the size, so no assuptions should be made about it.
  *
  * The encoder state is always continuous in memory and only a shallow copy is sufficient
  * to copy it (e.g. memcpy())
  *
  * It is possible to change some of the encoder's settings using the opus_encoder_ctl()
  * interface. All these settings already default to the recommended value, so they should
  * only be changed when necessary. The most common settings one may want to change are:
  *
  * @code
  * opus_encoder_ctl(enc, OPUS_SET_BITRATE(bitrate));
  * opus_encoder_ctl(enc, OPUS_SET_COMPLEXITY(complexity));
  * opus_encoder_ctl(enc, OPUS_SET_SIGNAL(signal_type));
  * @endcode
  *
  * where
  *
  * @arg bitrate is in bits per second (b/s)
  * @arg complexity is a value from 1 to 10, where 1 is the lowest complexity and 10 is the highest
  * @arg signal_type is either OPUS_AUTO (default), OPUS_SIGNAL_VOICE, or OPUS_SIGNAL_MUSIC
  *
  * See @ref opus_encoderctls and @ref opus_genericctls for a complete list of parameters that can be set or queried. Most parameters can be set or changed at any time during a stream.
  *
  * To encode a frame, opus_encode() or opus_encode_float() must be called with exactly one frame (2.5, 5, 10, 20, 40 or 60 ms) of audio data:
  * @code
  * len = opus_encode(enc, audio_frame, frame_size, packet, max_packet);
  * @endcode
  *
  * where
  * <ul>
  * <li>audio_frame is the audio data in opus_int16 (or float for opus_encode_float())</li>
  * <li>frame_size is the duration of the frame in samples (per channel)</li>
  * <li>packet is the byte array to which the compressed data is written</li>
  * <li>max_packet is the maximum number of bytes that can be written in the packet (4000 bytes is recommended).
  *     Do not use max_packet to control VBR target bitrate, instead use the #OPUS_SET_BITRATE CTL.</li>
  * </ul>
  *
  * opus_encode() and opus_encode_float() return the number of bytes actually written to the packet.
  * The return value <b>can be negative</b>, which indicates that an error has occurred. If the return value
  * is 2 bytes or less, then the packet does not need to be transmitted (DTX).
  *
  * Once the encoder state if no longer needed, it can be destroyed with
  *
  * @code
  * opus_encoder_destroy(enc);
  * @endcode
  *
  * If the encoder was created with opus_encoder_init() rather than opus_encoder_create(),
  * then no action is required aside from potentially freeing the memory that was manually
  * allocated for it (calling free(enc) for the example above)
  *
  */
        /* * Opus encoder state.
  * This contains the complete state of an Opus encoder.
  * It is position independent and can be freely copied.
  * @see opus_encoder_create,opus_encoder_init
  */
        pub type OpusEncoder;
        /* *@}*/
        /* * @defgroup opus_decoder Opus Decoder
  * @{
  *
  * @brief This page describes the process and functions used to decode Opus.
  *
  * The decoding process also starts with creating a decoder
  * state. This can be done with:
  * @code
  * int          error;
  * OpusDecoder *dec;
  * dec = opus_decoder_create(Fs, channels, &error);
  * @endcode
  * where
  * @li Fs is the sampling rate and must be 8000, 12000, 16000, 24000, or 48000
  * @li channels is the number of channels (1 or 2)
  * @li error will hold the error code in case of failure (or #OPUS_OK on success)
  * @li the return value is a newly created decoder state to be used for decoding
  *
  * While opus_decoder_create() allocates memory for the state, it's also possible
  * to initialize pre-allocated memory:
  * @code
  * int          size;
  * int          error;
  * OpusDecoder *dec;
  * size = opus_decoder_get_size(channels);
  * dec = malloc(size);
  * error = opus_decoder_init(dec, Fs, channels);
  * @endcode
  * where opus_decoder_get_size() returns the required size for the decoder state. Note that
  * future versions of this code may change the size, so no assuptions should be made about it.
  *
  * The decoder state is always continuous in memory and only a shallow copy is sufficient
  * to copy it (e.g. memcpy())
  *
  * To decode a frame, opus_decode() or opus_decode_float() must be called with a packet of compressed audio data:
  * @code
  * frame_size = opus_decode(dec, packet, len, decoded, max_size, 0);
  * @endcode
  * where
  *
  * @li packet is the byte array containing the compressed data
  * @li len is the exact number of bytes contained in the packet
  * @li decoded is the decoded audio data in opus_int16 (or float for opus_decode_float())
  * @li max_size is the max duration of the frame in samples (per channel) that can fit into the decoded_frame array
  *
  * opus_decode() and opus_decode_float() return the number of samples (per channel) decoded from the packet.
  * If that value is negative, then an error has occurred. This can occur if the packet is corrupted or if the audio
  * buffer is too small to hold the decoded audio.
  *
  * Opus is a stateful codec with overlapping blocks and as a result Opus
  * packets are not coded independently of each other. Packets must be
  * passed into the decoder serially and in the correct order for a correct
  * decode. Lost packets can be replaced with loss concealment by calling
  * the decoder with a null pointer and zero length for the missing packet.
  *
  * A single codec state may only be accessed from a single thread at
  * a time and any required locking must be performed by the caller. Separate
  * streams must be decoded with separate decoder states and can be decoded
  * in parallel unless the library was compiled with NONTHREADSAFE_PSEUDOSTACK
  * defined.
  *
  */
        /* * Opus decoder state.
  * This contains the complete state of an Opus decoder.
  * It is position independent and can be freely copied.
  * @see opus_decoder_create,opus_decoder_init
  */
        pub type OpusDecoder;
    }
}
#[header_src =
      "ioq3/code/client/cl_curl.h"]
pub mod cl_curl_h {
    use super::q_shared_h::{cvar_t, qboolean};
    use super::{libc};
    use super::curl_h::{CURL, CURLcode, CURLoption, CURLINFO};
    use super::multi_h::{CURLM, CURLMcode, CURLMsg};
    use super::select_h::{fd_set};
}
#[header_src =
      "ioq3/code/client/cl_curl.c"]
pub mod cl_curl_c {
    use super::curl_h::{CURL};
}
#[header_src = "/usr/include/SDL2/SDL_loadso.h"]
pub mod SDL_loadso_h {
    use super::{libc};
    extern "C" {
        /* *
 *  Unload a shared object from memory.
 */
        #[no_mangle]
        pub fn SDL_UnloadObject(handle: *mut libc::c_void);
        /* *
 *  Given an object handle, this function looks up the address of the
 *  named function in the shared object and returns it.  This address
 *  is no longer valid after calling SDL_UnloadObject().
 */
        #[no_mangle]
        pub fn SDL_LoadFunction(handle: *mut libc::c_void,
                                name: *const libc::c_char)
         -> *mut libc::c_void;
    }
}
#[header_src =
      "ioq3/code/sys/sys_loadlib.h"]
pub mod sys_loadlib_h {
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
        pub fn Sys_LoadDll(name: *const libc::c_char, useSystemLib: qboolean)
         -> *mut libc::c_void;
    }
}
#[header_src =
      "ioq3/code/client/cl_variadic.h"]
pub mod cl_variadic_h {
    use super::curl_h::{CURLcode, CURL, CURLoption};
    extern "C" {
        #[no_mangle]
        pub fn qcurl_easy_setopt_warn(curl: *mut libc::c_void,
                                      option: CURLoption, ...) -> CURLcode;
    }
}
use self::types_h::{__uint8_t};
use self::stddef_h::{size_t};
use self::select_h::{__fd_mask, fd_set};
use self::stdint_uintn_h::{uint8_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, qhandle_t, fileHandle_t,
                       unnamed, ERR_NEED_CD, ERR_DISCONNECT,
                       ERR_SERVERDISCONNECT, ERR_DROP, ERR_FATAL, cvar_s,
                       cvar_t, connstate_t, CA_CINEMATIC, CA_ACTIVE,
                       CA_PRIMED, CA_LOADING, CA_CONNECTED, CA_CHALLENGING,
                       CA_CONNECTING, CA_AUTHORIZING, CA_DISCONNECTED,
                       CA_UNINITIALIZED, Com_sprintf, Q_strncpyz, va,
                       Com_Error, Com_Printf};
use self::qcommon_h::{netadrtype_t, NA_UNSPEC, NA_MULTICAST6, NA_IP6, NA_IP,
                      NA_BROADCAST, NA_LOOPBACK, NA_BOT, NA_BAD, netsrc_t,
                      NS_SERVER, NS_CLIENT, netadr_t, netchan_t,
                      NET_AdrToString, Cvar_Set, Cvar_SetValue,
                      FS_SV_FOpenFileWrite, FS_SV_Rename, FS_Write,
                      FS_FCloseFile, Com_DPrintf, com_developer};
use self::tr_types_h::{textureCompression_t, TC_S3TC_ARB, TC_S3TC, TC_NONE,
                       glDriverType_t, GLDRV_VOODOO, GLDRV_STANDALONE,
                       GLDRV_ICD, glHardwareType_t, GLHW_PERMEDIA2,
                       GLHW_RAGEPRO, GLHW_RIVA128, GLHW_3DFX_2D3D,
                       GLHW_GENERIC, glconfig_t};
use self::curl_h::{CURL, CURLcode, CURL_LAST, CURLE_RECURSIVE_API_CALL,
                   CURLE_HTTP2_STREAM, CURLE_SSL_INVALIDCERTSTATUS,
                   CURLE_SSL_PINNEDPUBKEYNOTMATCH,
                   CURLE_NO_CONNECTION_AVAILABLE, CURLE_CHUNK_FAILED,
                   CURLE_FTP_BAD_FILE_LIST, CURLE_RTSP_SESSION_ERROR,
                   CURLE_RTSP_CSEQ_ERROR, CURLE_FTP_PRET_FAILED,
                   CURLE_SSL_ISSUER_ERROR, CURLE_SSL_CRL_BADFILE, CURLE_AGAIN,
                   CURLE_SSL_SHUTDOWN_FAILED, CURLE_SSH,
                   CURLE_REMOTE_FILE_NOT_FOUND, CURLE_SSL_CACERT_BADFILE,
                   CURLE_CONV_REQD, CURLE_CONV_FAILED, CURLE_TFTP_NOSUCHUSER,
                   CURLE_REMOTE_FILE_EXISTS, CURLE_TFTP_UNKNOWNID,
                   CURLE_TFTP_ILLEGAL, CURLE_REMOTE_DISK_FULL,
                   CURLE_TFTP_PERM, CURLE_TFTP_NOTFOUND, CURLE_LOGIN_DENIED,
                   CURLE_SSL_ENGINE_INITFAILED, CURLE_SEND_FAIL_REWIND,
                   CURLE_USE_SSL_FAILED, CURLE_FILESIZE_EXCEEDED,
                   CURLE_LDAP_INVALID_URL, CURLE_BAD_CONTENT_ENCODING,
                   CURLE_PEER_FAILED_VERIFICATION, CURLE_SSL_CIPHER,
                   CURLE_SSL_CERTPROBLEM, CURLE_OBSOLETE57, CURLE_RECV_ERROR,
                   CURLE_SEND_ERROR, CURLE_SSL_ENGINE_SETFAILED,
                   CURLE_SSL_ENGINE_NOTFOUND, CURLE_GOT_NOTHING,
                   CURLE_OBSOLETE51, CURLE_OBSOLETE50,
                   CURLE_TELNET_OPTION_SYNTAX, CURLE_UNKNOWN_OPTION,
                   CURLE_TOO_MANY_REDIRECTS, CURLE_OBSOLETE46,
                   CURLE_INTERFACE_FAILED, CURLE_OBSOLETE44,
                   CURLE_BAD_FUNCTION_ARGUMENT, CURLE_ABORTED_BY_CALLBACK,
                   CURLE_FUNCTION_NOT_FOUND, CURLE_OBSOLETE40,
                   CURLE_LDAP_SEARCH_FAILED, CURLE_LDAP_CANNOT_BIND,
                   CURLE_FILE_COULDNT_READ_FILE, CURLE_BAD_DOWNLOAD_RESUME,
                   CURLE_SSL_CONNECT_ERROR, CURLE_HTTP_POST_ERROR,
                   CURLE_RANGE_ERROR, CURLE_OBSOLETE32,
                   CURLE_FTP_COULDNT_USE_REST, CURLE_FTP_PORT_FAILED,
                   CURLE_OBSOLETE29, CURLE_OPERATION_TIMEDOUT,
                   CURLE_OUT_OF_MEMORY, CURLE_READ_ERROR, CURLE_UPLOAD_FAILED,
                   CURLE_OBSOLETE24, CURLE_WRITE_ERROR,
                   CURLE_HTTP_RETURNED_ERROR, CURLE_QUOTE_ERROR,
                   CURLE_OBSOLETE20, CURLE_FTP_COULDNT_RETR_FILE,
                   CURLE_PARTIAL_FILE, CURLE_FTP_COULDNT_SET_TYPE,
                   CURLE_HTTP2, CURLE_FTP_CANT_GET_HOST,
                   CURLE_FTP_WEIRD_227_FORMAT, CURLE_FTP_WEIRD_PASV_REPLY,
                   CURLE_FTP_ACCEPT_TIMEOUT, CURLE_FTP_WEIRD_PASS_REPLY,
                   CURLE_FTP_ACCEPT_FAILED, CURLE_REMOTE_ACCESS_DENIED,
                   CURLE_WEIRD_SERVER_REPLY, CURLE_COULDNT_CONNECT,
                   CURLE_COULDNT_RESOLVE_HOST, CURLE_COULDNT_RESOLVE_PROXY,
                   CURLE_NOT_BUILT_IN, CURLE_URL_MALFORMAT, CURLE_FAILED_INIT,
                   CURLE_UNSUPPORTED_PROTOCOL, CURLE_OK, CURLoption,
                   CURLOPT_LASTENTRY, CURLOPT_CURLU,
                   CURLOPT_UPKEEP_INTERVAL_MS, CURLOPT_UPLOAD_BUFFERSIZE,
                   CURLOPT_DOH_URL, CURLOPT_DISALLOW_USERNAME_IN_URL,
                   CURLOPT_PROXY_TLS13_CIPHERS, CURLOPT_TLS13_CIPHERS,
                   CURLOPT_DNS_SHUFFLE_ADDRESSES, CURLOPT_HAPROXYPROTOCOL,
                   CURLOPT_RESOLVER_START_DATA,
                   CURLOPT_RESOLVER_START_FUNCTION,
                   CURLOPT_HAPPY_EYEBALLS_TIMEOUT_MS, CURLOPT_TIMEVALUE_LARGE,
                   CURLOPT_MIMEPOST, CURLOPT_SSH_COMPRESSION,
                   CURLOPT_SOCKS5_AUTH, CURLOPT_REQUEST_TARGET,
                   CURLOPT_SUPPRESS_CONNECT_HEADERS,
                   CURLOPT_ABSTRACT_UNIX_SOCKET,
                   CURLOPT_PROXY_PINNEDPUBLICKEY, CURLOPT_PRE_PROXY,
                   CURLOPT_PROXY_SSL_OPTIONS, CURLOPT_PROXY_CRLFILE,
                   CURLOPT_PROXY_SSL_CIPHER_LIST, CURLOPT_PROXY_KEYPASSWD,
                   CURLOPT_PROXY_SSLKEYTYPE, CURLOPT_PROXY_SSLKEY,
                   CURLOPT_PROXY_SSLCERTTYPE, CURLOPT_PROXY_SSLCERT,
                   CURLOPT_PROXY_TLSAUTH_TYPE, CURLOPT_PROXY_TLSAUTH_PASSWORD,
                   CURLOPT_PROXY_TLSAUTH_USERNAME, CURLOPT_PROXY_SSLVERSION,
                   CURLOPT_PROXY_SSL_VERIFYHOST, CURLOPT_PROXY_SSL_VERIFYPEER,
                   CURLOPT_PROXY_CAPATH, CURLOPT_PROXY_CAINFO,
                   CURLOPT_KEEP_SENDING_ON_ERROR, CURLOPT_TCP_FASTOPEN,
                   CURLOPT_CONNECT_TO, CURLOPT_TFTP_NO_OPTIONS,
                   CURLOPT_STREAM_DEPENDS_E, CURLOPT_STREAM_DEPENDS,
                   CURLOPT_STREAM_WEIGHT, CURLOPT_DEFAULT_PROTOCOL,
                   CURLOPT_PIPEWAIT, CURLOPT_SERVICE_NAME,
                   CURLOPT_PROXY_SERVICE_NAME, CURLOPT_PATH_AS_IS,
                   CURLOPT_SSL_FALSESTART, CURLOPT_SSL_VERIFYSTATUS,
                   CURLOPT_UNIX_SOCKET_PATH, CURLOPT_PINNEDPUBLICKEY,
                   CURLOPT_HEADEROPT, CURLOPT_PROXYHEADER,
                   CURLOPT_EXPECT_100_TIMEOUT_MS, CURLOPT_SSL_ENABLE_ALPN,
                   CURLOPT_SSL_ENABLE_NPN, CURLOPT_LOGIN_OPTIONS,
                   CURLOPT_DNS_LOCAL_IP6, CURLOPT_DNS_LOCAL_IP4,
                   CURLOPT_DNS_INTERFACE, CURLOPT_XOAUTH2_BEARER,
                   CURLOPT_XFERINFOFUNCTION, CURLOPT_SASL_IR,
                   CURLOPT_MAIL_AUTH, CURLOPT_SSL_OPTIONS,
                   CURLOPT_TCP_KEEPINTVL, CURLOPT_TCP_KEEPIDLE,
                   CURLOPT_TCP_KEEPALIVE, CURLOPT_ACCEPTTIMEOUT_MS,
                   CURLOPT_DNS_SERVERS, CURLOPT_GSSAPI_DELEGATION,
                   CURLOPT_CLOSESOCKETDATA, CURLOPT_CLOSESOCKETFUNCTION,
                   CURLOPT_TRANSFER_ENCODING, CURLOPT_TLSAUTH_TYPE,
                   CURLOPT_TLSAUTH_PASSWORD, CURLOPT_TLSAUTH_USERNAME,
                   CURLOPT_RESOLVE, CURLOPT_FNMATCH_DATA, CURLOPT_CHUNK_DATA,
                   CURLOPT_FNMATCH_FUNCTION, CURLOPT_CHUNK_END_FUNCTION,
                   CURLOPT_CHUNK_BGN_FUNCTION, CURLOPT_WILDCARDMATCH,
                   CURLOPT_INTERLEAVEFUNCTION, CURLOPT_INTERLEAVEDATA,
                   CURLOPT_RTSP_SERVER_CSEQ, CURLOPT_RTSP_CLIENT_CSEQ,
                   CURLOPT_RTSP_TRANSPORT, CURLOPT_RTSP_STREAM_URI,
                   CURLOPT_RTSP_SESSION_ID, CURLOPT_RTSP_REQUEST,
                   CURLOPT_FTP_USE_PRET, CURLOPT_MAIL_RCPT, CURLOPT_MAIL_FROM,
                   CURLOPT_SSH_KEYDATA, CURLOPT_SSH_KEYFUNCTION,
                   CURLOPT_SSH_KNOWNHOSTS, CURLOPT_REDIR_PROTOCOLS,
                   CURLOPT_PROTOCOLS, CURLOPT_SOCKS5_GSSAPI_NEC,
                   CURLOPT_SOCKS5_GSSAPI_SERVICE, CURLOPT_TFTP_BLKSIZE,
                   CURLOPT_NOPROXY, CURLOPT_PROXYPASSWORD,
                   CURLOPT_PROXYUSERNAME, CURLOPT_PASSWORD, CURLOPT_USERNAME,
                   CURLOPT_CERTINFO, CURLOPT_ADDRESS_SCOPE,
                   CURLOPT_ISSUERCERT, CURLOPT_CRLFILE, CURLOPT_SEEKDATA,
                   CURLOPT_SEEKFUNCTION, CURLOPT_PROXY_TRANSFER_MODE,
                   CURLOPT_COPYPOSTFIELDS, CURLOPT_OPENSOCKETDATA,
                   CURLOPT_OPENSOCKETFUNCTION,
                   CURLOPT_SSH_HOST_PUBLIC_KEY_MD5, CURLOPT_POSTREDIR,
                   CURLOPT_NEW_DIRECTORY_PERMS, CURLOPT_NEW_FILE_PERMS,
                   CURLOPT_HTTP_CONTENT_DECODING,
                   CURLOPT_HTTP_TRANSFER_DECODING, CURLOPT_CONNECTTIMEOUT_MS,
                   CURLOPT_TIMEOUT_MS, CURLOPT_FTP_SSL_CCC,
                   CURLOPT_SSH_PRIVATE_KEYFILE, CURLOPT_SSH_PUBLIC_KEYFILE,
                   CURLOPT_SSH_AUTH_TYPES, CURLOPT_SSL_SESSIONID_CACHE,
                   CURLOPT_SOCKOPTDATA, CURLOPT_SOCKOPTFUNCTION,
                   CURLOPT_FTP_ALTERNATIVE_TO_USER,
                   CURLOPT_MAX_RECV_SPEED_LARGE, CURLOPT_MAX_SEND_SPEED_LARGE,
                   CURLOPT_CONV_FROM_UTF8_FUNCTION,
                   CURLOPT_CONV_TO_NETWORK_FUNCTION,
                   CURLOPT_CONV_FROM_NETWORK_FUNCTION, CURLOPT_CONNECT_ONLY,
                   CURLOPT_LOCALPORTRANGE, CURLOPT_LOCALPORT,
                   CURLOPT_FTP_FILEMETHOD, CURLOPT_FTP_SKIP_PASV_IP,
                   CURLOPT_IGNORE_CONTENT_LENGTH, CURLOPT_COOKIELIST,
                   CURLOPT_FTP_ACCOUNT, CURLOPT_IOCTLDATA,
                   CURLOPT_IOCTLFUNCTION, CURLOPT_FTPSSLAUTH,
                   CURLOPT_TCP_NODELAY, CURLOPT_POSTFIELDSIZE_LARGE,
                   CURLOPT_USE_SSL, CURLOPT_NETRC_FILE,
                   CURLOPT_MAXFILESIZE_LARGE, CURLOPT_RESUME_FROM_LARGE,
                   CURLOPT_INFILESIZE_LARGE, CURLOPT_MAXFILESIZE,
                   CURLOPT_IPRESOLVE, CURLOPT_FTP_RESPONSE_TIMEOUT,
                   CURLOPT_PROXYAUTH, CURLOPT_FTP_CREATE_MISSING_DIRS,
                   CURLOPT_SSL_CTX_DATA, CURLOPT_SSL_CTX_FUNCTION,
                   CURLOPT_HTTPAUTH, CURLOPT_FTP_USE_EPRT,
                   CURLOPT_UNRESTRICTED_AUTH, CURLOPT_HTTP200ALIASES,
                   CURLOPT_PRIVATE, CURLOPT_ACCEPT_ENCODING,
                   CURLOPT_PROXYTYPE, CURLOPT_SHARE, CURLOPT_NOSIGNAL,
                   CURLOPT_BUFFERSIZE, CURLOPT_CAPATH, CURLOPT_COOKIESESSION,
                   CURLOPT_DEBUGDATA, CURLOPT_DEBUGFUNCTION, CURLOPT_PREQUOTE,
                   CURLOPT_DNS_CACHE_TIMEOUT, CURLOPT_DNS_USE_GLOBAL_CACHE,
                   CURLOPT_SSLENGINE_DEFAULT, CURLOPT_SSLENGINE,
                   CURLOPT_SSLKEYTYPE, CURLOPT_SSLKEY, CURLOPT_SSLCERTTYPE,
                   CURLOPT_FTP_USE_EPSV, CURLOPT_HTTP_VERSION,
                   CURLOPT_SSL_CIPHER_LIST, CURLOPT_COOKIEJAR,
                   CURLOPT_SSL_VERIFYHOST, CURLOPT_HTTPGET,
                   CURLOPT_HEADERFUNCTION, CURLOPT_CONNECTTIMEOUT,
                   CURLOPT_EGDSOCKET, CURLOPT_RANDOM_FILE,
                   CURLOPT_FORBID_REUSE, CURLOPT_FRESH_CONNECT,
                   CURLOPT_OBSOLETE72, CURLOPT_MAXCONNECTS,
                   CURLOPT_TELNETOPTIONS, CURLOPT_FILETIME, CURLOPT_MAXREDIRS,
                   CURLOPT_CAINFO, CURLOPT_SSL_VERIFYPEER, CURLOPT_KRBLEVEL,
                   CURLOPT_INTERFACE, CURLOPT_HTTPPROXYTUNNEL,
                   CURLOPT_POSTFIELDSIZE, CURLOPT_PROXYPORT,
                   CURLOPT_AUTOREFERER, CURLOPT_PROGRESSDATA,
                   CURLOPT_PROGRESSFUNCTION, CURLOPT_PUT,
                   CURLOPT_TRANSFERTEXT, CURLOPT_FOLLOWLOCATION,
                   CURLOPT_NETRC, CURLOPT_APPEND, CURLOPT_DIRLISTONLY,
                   CURLOPT_POST, CURLOPT_UPLOAD, CURLOPT_FAILONERROR,
                   CURLOPT_NOBODY, CURLOPT_NOPROGRESS, CURLOPT_HEADER,
                   CURLOPT_VERBOSE, CURLOPT_OBSOLETE40, CURLOPT_POSTQUOTE,
                   CURLOPT_STDERR, CURLOPT_CUSTOMREQUEST, CURLOPT_TIMEVALUE,
                   CURLOPT_TIMECONDITION, CURLOPT_SSLVERSION,
                   CURLOPT_COOKIEFILE, CURLOPT_HEADERDATA, CURLOPT_QUOTE,
                   CURLOPT_CRLF, CURLOPT_KEYPASSWD, CURLOPT_SSLCERT,
                   CURLOPT_HTTPPOST, CURLOPT_HTTPHEADER, CURLOPT_COOKIE,
                   CURLOPT_RESUME_FROM, CURLOPT_LOW_SPEED_TIME,
                   CURLOPT_LOW_SPEED_LIMIT, CURLOPT_USERAGENT,
                   CURLOPT_FTPPORT, CURLOPT_REFERER, CURLOPT_POSTFIELDS,
                   CURLOPT_INFILESIZE, CURLOPT_TIMEOUT, CURLOPT_READFUNCTION,
                   CURLOPT_WRITEFUNCTION, CURLOPT_ERRORBUFFER,
                   CURLOPT_READDATA, CURLOPT_RANGE, CURLOPT_PROXYUSERPWD,
                   CURLOPT_USERPWD, CURLOPT_PROXY, CURLOPT_PORT, CURLOPT_URL,
                   CURLOPT_WRITEDATA, CURLINFO, CURLINFO_LASTONE,
                   CURLINFO_APPCONNECT_TIME_T, CURLINFO_REDIRECT_TIME_T,
                   CURLINFO_STARTTRANSFER_TIME_T, CURLINFO_PRETRANSFER_TIME_T,
                   CURLINFO_CONNECT_TIME_T, CURLINFO_NAMELOOKUP_TIME_T,
                   CURLINFO_TOTAL_TIME_T, CURLINFO_SCHEME, CURLINFO_PROTOCOL,
                   CURLINFO_PROXY_SSL_VERIFYRESULT, CURLINFO_HTTP_VERSION,
                   CURLINFO_TLS_SSL_PTR, CURLINFO_ACTIVESOCKET,
                   CURLINFO_TLS_SESSION, CURLINFO_LOCAL_PORT,
                   CURLINFO_LOCAL_IP, CURLINFO_PRIMARY_PORT,
                   CURLINFO_RTSP_CSEQ_RECV, CURLINFO_RTSP_SERVER_CSEQ,
                   CURLINFO_RTSP_CLIENT_CSEQ, CURLINFO_RTSP_SESSION_ID,
                   CURLINFO_CONDITION_UNMET, CURLINFO_CERTINFO,
                   CURLINFO_APPCONNECT_TIME, CURLINFO_PRIMARY_IP,
                   CURLINFO_REDIRECT_URL, CURLINFO_FTP_ENTRY_PATH,
                   CURLINFO_LASTSOCKET, CURLINFO_COOKIELIST,
                   CURLINFO_SSL_ENGINES, CURLINFO_NUM_CONNECTS,
                   CURLINFO_OS_ERRNO, CURLINFO_PROXYAUTH_AVAIL,
                   CURLINFO_HTTPAUTH_AVAIL, CURLINFO_HTTP_CONNECTCODE,
                   CURLINFO_PRIVATE, CURLINFO_REDIRECT_COUNT,
                   CURLINFO_REDIRECT_TIME, CURLINFO_CONTENT_TYPE,
                   CURLINFO_STARTTRANSFER_TIME,
                   CURLINFO_CONTENT_LENGTH_UPLOAD_T,
                   CURLINFO_CONTENT_LENGTH_UPLOAD,
                   CURLINFO_CONTENT_LENGTH_DOWNLOAD_T,
                   CURLINFO_CONTENT_LENGTH_DOWNLOAD, CURLINFO_FILETIME_T,
                   CURLINFO_FILETIME, CURLINFO_SSL_VERIFYRESULT,
                   CURLINFO_REQUEST_SIZE, CURLINFO_HEADER_SIZE,
                   CURLINFO_SPEED_UPLOAD_T, CURLINFO_SPEED_UPLOAD,
                   CURLINFO_SPEED_DOWNLOAD_T, CURLINFO_SPEED_DOWNLOAD,
                   CURLINFO_SIZE_DOWNLOAD_T, CURLINFO_SIZE_DOWNLOAD,
                   CURLINFO_SIZE_UPLOAD_T, CURLINFO_SIZE_UPLOAD,
                   CURLINFO_PRETRANSFER_TIME, CURLINFO_CONNECT_TIME,
                   CURLINFO_NAMELOOKUP_TIME, CURLINFO_TOTAL_TIME,
                   CURLINFO_RESPONSE_CODE, CURLINFO_EFFECTIVE_URL,
                   CURLINFO_NONE};
use self::multi_h::{CURLM, CURLMcode, CURLM_LAST, CURLM_RECURSIVE_API_CALL,
                    CURLM_ADDED_ALREADY, CURLM_UNKNOWN_OPTION,
                    CURLM_BAD_SOCKET, CURLM_INTERNAL_ERROR,
                    CURLM_OUT_OF_MEMORY, CURLM_BAD_EASY_HANDLE,
                    CURLM_BAD_HANDLE, CURLM_OK, CURLM_CALL_MULTI_PERFORM,
                    CURLMSG, CURLMSG_LAST, CURLMSG_DONE, CURLMSG_NONE,
                    CURLMsg, unnamed_0};
use self::client_h::{clientConnection_t, clientStatic_t, serverInfo_t, clc,
                     CL_WritePacket, CL_AddReliableCommand, cls,
                     CL_NextDownload};
use self::opus_h::{OpusEncoder, OpusDecoder};
use self::SDL_loadso_h::{SDL_UnloadObject, SDL_LoadFunction};
use self::sys_loadlib_h::{Sys_LoadDll};
use self::cl_variadic_h::{qcurl_easy_setopt_warn};
/*
===========================================================================
Copyright (C) 2006 Tony J. White (tjw@tjw.org)

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
pub static mut cl_cURLLib: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut qcurl_version:
           Option<unsafe extern "C" fn() -> *mut libc::c_char> =
    None;
#[no_mangle]
pub static mut qcurl_easy_init:
           Option<unsafe extern "C" fn() -> *mut libc::c_void> =
    None;
#[no_mangle]
pub static mut qcurl_easy_setopt:
           Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                       _: CURLoption, ...) -> CURLcode> =
    None;
#[no_mangle]
pub static mut qcurl_easy_perform:
           Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CURLcode> =
    None;
#[no_mangle]
pub static mut qcurl_easy_cleanup:
           Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()> =
    None;
#[no_mangle]
pub static mut qcurl_easy_getinfo:
           Option<unsafe extern "C" fn(_: *mut libc::c_void, _: CURLINFO, ...)
                      -> CURLcode> =
    None;
#[no_mangle]
pub static mut qcurl_easy_reset:
           Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()> =
    None;
#[no_mangle]
pub static mut qcurl_easy_strerror:
           Option<unsafe extern "C" fn(_: CURLcode) -> *const libc::c_char> =
    None;
#[no_mangle]
pub static mut qcurl_multi_init:
           Option<unsafe extern "C" fn() -> *mut libc::c_void> =
    None;
#[no_mangle]
pub static mut qcurl_multi_add_handle:
           Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                       _: *mut libc::c_void) -> CURLMcode> =
    None;
#[no_mangle]
pub static mut qcurl_multi_remove_handle:
           Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                       _: *mut libc::c_void) -> CURLMcode> =
    None;
#[no_mangle]
pub static mut qcurl_multi_fdset:
           Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut fd_set,
                                       _: *mut fd_set, _: *mut fd_set,
                                       _: *mut libc::c_int) -> CURLMcode> =
    None;
#[no_mangle]
pub static mut qcurl_multi_perform:
           Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                       _: *mut libc::c_int) -> CURLMcode> =
    None;
#[no_mangle]
pub static mut qcurl_multi_cleanup:
           Option<unsafe extern "C" fn(_: *mut libc::c_void) -> CURLMcode> =
    None;
#[no_mangle]
pub static mut qcurl_multi_info_read:
           Option<unsafe extern "C" fn(_: *mut libc::c_void,
                                       _: *mut libc::c_int) -> *mut CURLMsg> =
    None;
#[no_mangle]
pub static mut qcurl_multi_strerror:
           Option<unsafe extern "C" fn(_: CURLMcode) -> *const libc::c_char> =
    None;
#[no_mangle]
pub unsafe extern "C" fn CL_cURL_Init() -> qboolean {
    if !cURLLib.is_null() { return qtrue }
    Com_Printf(b"Loading \"%s\"...\x00" as *const u8 as *const libc::c_char,
               (*cl_cURLLib).string);
    cURLLib = Sys_LoadDll((*cl_cURLLib).string, qtrue);
    if cURLLib.is_null() {
        cURLLib =
            Sys_LoadDll(b"libcurl.so.3\x00" as *const u8 as
                            *const libc::c_char, qtrue);
        if cURLLib.is_null() { return qfalse }
    }
    clc.cURLEnabled = qtrue;
    qcurl_version =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn()
                                           ->
                                               *mut libc::c_char>>(GPA(b"curl_version\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char
                                                                           as
                                                                           *mut libc::c_char));
    qcurl_easy_init =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn()
                                           ->
                                               *mut libc::c_void>>(GPA(b"curl_easy_init\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char
                                                                           as
                                                                           *mut libc::c_char));
    qcurl_easy_setopt =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _:
                                                                CURLoption, ...)
                                           ->
                                               CURLcode>>(GPA(b"curl_easy_setopt\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char));
    qcurl_easy_perform =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           ->
                                               CURLcode>>(GPA(b"curl_easy_perform\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char));
    qcurl_easy_cleanup =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           ->
                                               ()>>(GPA(b"curl_easy_cleanup\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char));
    qcurl_easy_getinfo =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: CURLINFO, ...)
                                           ->
                                               CURLcode>>(GPA(b"curl_easy_getinfo\x00"
                                                                  as *const u8
                                                                  as
                                                                  *const libc::c_char
                                                                  as
                                                                  *mut libc::c_char));
    qcurl_easy_duphandle =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           ->
                                               *mut libc::c_void>>(GPA(b"curl_easy_duphandle\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char
                                                                           as
                                                                           *mut libc::c_char));
    qcurl_easy_reset =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           ->
                                               ()>>(GPA(b"curl_easy_reset\x00"
                                                            as *const u8 as
                                                            *const libc::c_char
                                                            as
                                                            *mut libc::c_char));
    qcurl_easy_strerror =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: CURLcode)
                                           ->
                                               *const libc::c_char>>(GPA(b"curl_easy_strerror\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char
                                                                             as
                                                                             *mut libc::c_char));
    qcurl_multi_init =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn()
                                           ->
                                               *mut libc::c_void>>(GPA(b"curl_multi_init\x00"
                                                                           as
                                                                           *const u8
                                                                           as
                                                                           *const libc::c_char
                                                                           as
                                                                           *mut libc::c_char));
    qcurl_multi_add_handle =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _:
                                                                *mut libc::c_void)
                                           ->
                                               CURLMcode>>(GPA(b"curl_multi_add_handle\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char));
    qcurl_multi_remove_handle =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _:
                                                                *mut libc::c_void)
                                           ->
                                               CURLMcode>>(GPA(b"curl_multi_remove_handle\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char));
    qcurl_multi_fdset =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _: *mut fd_set,
                                                            _: *mut fd_set,
                                                            _: *mut fd_set,
                                                            _:
                                                                *mut libc::c_int)
                                           ->
                                               CURLMcode>>(GPA(b"curl_multi_fdset\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char));
    qcurl_multi_perform =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _:
                                                                *mut libc::c_int)
                                           ->
                                               CURLMcode>>(GPA(b"curl_multi_perform\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char));
    qcurl_multi_cleanup =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void)
                                           ->
                                               CURLMcode>>(GPA(b"curl_multi_cleanup\x00"
                                                                   as
                                                                   *const u8
                                                                   as
                                                                   *const libc::c_char
                                                                   as
                                                                   *mut libc::c_char));
    qcurl_multi_info_read =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_:
                                                                *mut libc::c_void,
                                                            _:
                                                                *mut libc::c_int)
                                           ->
                                               *mut CURLMsg>>(GPA(b"curl_multi_info_read\x00"
                                                                      as
                                                                      *const u8
                                                                      as
                                                                      *const libc::c_char
                                                                      as
                                                                      *mut libc::c_char));
    qcurl_multi_strerror =
        ::std::mem::transmute::<*mut libc::c_void,
                                Option<unsafe extern "C" fn(_: CURLMcode)
                                           ->
                                               *const libc::c_char>>(GPA(b"curl_multi_strerror\x00"
                                                                             as
                                                                             *const u8
                                                                             as
                                                                             *const libc::c_char
                                                                             as
                                                                             *mut libc::c_char));
    if 0 == clc.cURLEnabled as u64 {
        CL_cURL_Shutdown();
        Com_Printf(b"FAIL One or more symbols not found\n\x00" as *const u8 as
                       *const libc::c_char);
        return qfalse
    }
    Com_Printf(b"OK\n\x00" as *const u8 as *const libc::c_char);
    return qtrue;
}
#[no_mangle]
pub unsafe extern "C" fn CL_cURL_Shutdown() {
    CL_cURL_Cleanup();
    if !cURLLib.is_null() {
        SDL_UnloadObject(cURLLib);
        cURLLib = 0 as *mut libc::c_void
    }
    qcurl_easy_init = None;
    qcurl_easy_setopt = None;
    qcurl_easy_perform = None;
    qcurl_easy_cleanup = None;
    qcurl_easy_getinfo = None;
    qcurl_easy_duphandle = None;
    qcurl_easy_reset = None;
    qcurl_multi_init = None;
    qcurl_multi_add_handle = None;
    qcurl_multi_remove_handle = None;
    qcurl_multi_fdset = None;
    qcurl_multi_perform = None;
    qcurl_multi_cleanup = None;
    qcurl_multi_info_read = None;
    qcurl_multi_strerror = None;
}
/*
===========================================================================
Copyright (C) 2006 Tony J. White (tjw@tjw.org)

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
pub static mut qcurl_easy_duphandle:
           Option<unsafe extern "C" fn(_: *mut libc::c_void)
                      -> *mut libc::c_void> =
    None;
static mut cURLLib: *mut libc::c_void =
    0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn CL_cURL_Cleanup() {
    if !clc.downloadCURLM.is_null() {
        let mut result: CURLMcode = CURLM_OK;
        if !clc.downloadCURL.is_null() {
            result =
                qcurl_multi_remove_handle.expect("non-null function pointer")(clc.downloadCURLM,
                                                                              clc.downloadCURL);
            if result as libc::c_int != CURLM_OK as libc::c_int {
                Com_DPrintf(b"qcurl_multi_remove_handle failed: %s\n\x00" as
                                *const u8 as *const libc::c_char,
                            qcurl_multi_strerror.expect("non-null function pointer")(result));
            }
            qcurl_easy_cleanup.expect("non-null function pointer")(clc.downloadCURL);
        }
        result =
            qcurl_multi_cleanup.expect("non-null function pointer")(clc.downloadCURLM);
        if result as libc::c_int != CURLM_OK as libc::c_int {
            Com_DPrintf(b"CL_cURL_Cleanup: qcurl_multi_cleanup failed: %s\n\x00"
                            as *const u8 as *const libc::c_char,
                        qcurl_multi_strerror.expect("non-null function pointer")(result));
        }
        clc.downloadCURLM = 0 as *mut libc::c_void;
        clc.downloadCURL = 0 as *mut libc::c_void
    } else if !clc.downloadCURL.is_null() {
        qcurl_easy_cleanup.expect("non-null function pointer")(clc.downloadCURL);
        clc.downloadCURL = 0 as *mut libc::c_void
    };
}
/*
=================
GPA
=================
*/
unsafe extern "C" fn GPA(mut str: *mut libc::c_char) -> *mut libc::c_void {
    let mut rv: *mut libc::c_void = 0 as *mut libc::c_void;
    rv = SDL_LoadFunction(cURLLib, str);
    if rv.is_null() {
        Com_Printf(b"Can\'t load symbol %s\n\x00" as *const u8 as
                       *const libc::c_char, str);
        clc.cURLEnabled = qfalse;
        return 0 as *mut libc::c_void
    } else {
        Com_DPrintf(b"Loaded symbol %s (0x%p)\n\x00" as *const u8 as
                        *const libc::c_char, str, rv);
        return rv
    };
}
#[no_mangle]
pub unsafe extern "C" fn CL_cURL_BeginDownload(mut localName:
                                                   *const libc::c_char,
                                               mut remoteURL:
                                                   *const libc::c_char) {
    let mut result: CURLMcode = CURLM_OK;
    clc.cURLUsed = qtrue;
    Com_Printf(b"URL: %s\n\x00" as *const u8 as *const libc::c_char,
               remoteURL);
    Com_DPrintf(b"***** CL_cURL_BeginDownload *****\nLocalname: %s\nRemoteURL: %s\n****************************\n\x00"
                    as *const u8 as *const libc::c_char, localName,
                remoteURL);
    CL_cURL_Cleanup();
    Q_strncpyz(clc.downloadURL.as_mut_ptr(), remoteURL,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Q_strncpyz(clc.downloadName.as_mut_ptr(), localName,
               ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                   as libc::c_int);
    Com_sprintf(clc.downloadTempName.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
                    as libc::c_int,
                b"%s.tmp\x00" as *const u8 as *const libc::c_char, localName);
    Cvar_Set(b"cl_downloadName\x00" as *const u8 as *const libc::c_char,
             localName);
    Cvar_Set(b"cl_downloadSize\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    Cvar_Set(b"cl_downloadCount\x00" as *const u8 as *const libc::c_char,
             b"0\x00" as *const u8 as *const libc::c_char);
    Cvar_SetValue(b"cl_downloadTime\x00" as *const u8 as *const libc::c_char,
                  cls.realtime as libc::c_float);
    clc.downloadBlock = 0i32;
    clc.downloadCount = 0i32;
    clc.downloadCURL = qcurl_easy_init.expect("non-null function pointer")();
    if clc.downloadCURL.is_null() {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_cURL_BeginDownload: qcurl_easy_init() failed\x00" as
                      *const u8 as *const libc::c_char);
    }
    clc.download = FS_SV_FOpenFileWrite(clc.downloadTempName.as_mut_ptr());
    if 0 == clc.download {
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_cURL_BeginDownload: failed to open %s for writing\x00"
                      as *const u8 as *const libc::c_char,
                  clc.downloadTempName.as_mut_ptr());
    }
    if 0 != (*com_developer).integer {
        qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_VERBOSE, 1i32);
    }
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_URL,
                           clc.downloadURL.as_mut_ptr());
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_TRANSFERTEXT, 0i32);
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_REFERER,
                           va(b"ioQ3://%s\x00" as *const u8 as
                                  *const libc::c_char as *mut libc::c_char,
                              NET_AdrToString(clc.serverAddress)));
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_USERAGENT,
                           va(b"%s %s\x00" as *const u8 as *const libc::c_char
                                  as *mut libc::c_char,
                              b"ioq3 1.36_GIT_363a9303-2019-02-25\x00" as
                                  *const u8 as *const libc::c_char,
                              qcurl_version.expect("non-null function pointer")()));
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_WRITEFUNCTION,
                           Some(CL_cURL_CallbackWrite));
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_WRITEDATA,
                           &mut clc.download as *mut fileHandle_t);
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_NOPROGRESS, 0i32);
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_PROGRESSFUNCTION,
                           Some(CL_cURL_CallbackProgress));
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_PROGRESSDATA,
                           0 as *mut libc::c_void);
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_FAILONERROR, 1i32);
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_FOLLOWLOCATION, 1i32);
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_MAXREDIRS, 5i32);
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_PROTOCOLS,
                           1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32 |
                               1i32 << 3i32);
    qcurl_easy_setopt_warn(clc.downloadCURL, CURLOPT_BUFFERSIZE, 524288i32);
    clc.downloadCURLM =
        qcurl_multi_init.expect("non-null function pointer")();
    if clc.downloadCURLM.is_null() {
        qcurl_easy_cleanup.expect("non-null function pointer")(clc.downloadCURL);
        clc.downloadCURL = 0 as *mut libc::c_void;
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_cURL_BeginDownload: qcurl_multi_init() failed\x00" as
                      *const u8 as *const libc::c_char);
    }
    result =
        qcurl_multi_add_handle.expect("non-null function pointer")(clc.downloadCURLM,
                                                                   clc.downloadCURL);
    if result as libc::c_int != CURLM_OK as libc::c_int {
        qcurl_easy_cleanup.expect("non-null function pointer")(clc.downloadCURL);
        clc.downloadCURL = 0 as *mut libc::c_void;
        Com_Error(ERR_DROP as libc::c_int,
                  b"CL_cURL_BeginDownload: qcurl_multi_add_handle() failed: %s\x00"
                      as *const u8 as *const libc::c_char,
                  qcurl_multi_strerror.expect("non-null function pointer")(result));
    }
    if 0 == clc.sv_allowDownload & 8i32 && 0 == clc.cURLDisconnected as u64 {
        CL_AddReliableCommand(b"disconnect\x00" as *const u8 as
                                  *const libc::c_char, qtrue);
        CL_WritePacket();
        CL_WritePacket();
        CL_WritePacket();
        clc.cURLDisconnected = qtrue
    };
}
unsafe extern "C" fn CL_cURL_CallbackProgress(mut dummy: *mut libc::c_void,
                                              mut dltotal: libc::c_double,
                                              mut dlnow: libc::c_double,
                                              mut ultotal: libc::c_double,
                                              mut ulnow: libc::c_double)
 -> libc::c_int {
    clc.downloadSize = dltotal as libc::c_int;
    Cvar_SetValue(b"cl_downloadSize\x00" as *const u8 as *const libc::c_char,
                  clc.downloadSize as libc::c_float);
    clc.downloadCount = dlnow as libc::c_int;
    Cvar_SetValue(b"cl_downloadCount\x00" as *const u8 as *const libc::c_char,
                  clc.downloadCount as libc::c_float);
    return 0i32;
}
unsafe extern "C" fn CL_cURL_CallbackWrite(mut buffer: *mut libc::c_void,
                                           mut size: size_t,
                                           mut nmemb: size_t,
                                           mut stream: *mut libc::c_void)
 -> size_t {
    FS_Write(buffer, size.wrapping_mul(nmemb) as libc::c_int,
             *(stream as *mut fileHandle_t).offset(0isize));
    return size.wrapping_mul(nmemb);
}
#[no_mangle]
pub unsafe extern "C" fn CL_cURL_PerformDownload() {
    let mut res: CURLMcode = CURLM_OK;
    let mut msg: *mut CURLMsg = 0 as *mut CURLMsg;
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0i32;
    res =
        qcurl_multi_perform.expect("non-null function pointer")(clc.downloadCURLM,
                                                                &mut c);
    while res as libc::c_int == CURLM_CALL_MULTI_PERFORM as libc::c_int &&
              i < 100i32 {
        res =
            qcurl_multi_perform.expect("non-null function pointer")(clc.downloadCURLM,
                                                                    &mut c);
        i += 1
    }
    if res as libc::c_int == CURLM_CALL_MULTI_PERFORM as libc::c_int {
        return
    }
    msg =
        qcurl_multi_info_read.expect("non-null function pointer")(clc.downloadCURLM,
                                                                  &mut c);
    if msg.is_null() { return }
    FS_FCloseFile(clc.download);
    if (*msg).msg as libc::c_uint ==
           CURLMSG_DONE as libc::c_int as libc::c_uint &&
           (*msg).data.result as libc::c_uint ==
               CURLE_OK as libc::c_int as libc::c_uint {
        FS_SV_Rename(clc.downloadTempName.as_mut_ptr(),
                     clc.downloadName.as_mut_ptr(), qfalse);
        clc.downloadRestart = qtrue
    } else {
        let mut code: libc::c_long = 0;
        qcurl_easy_getinfo.expect("non-null function pointer")((*msg).easy_handle,
                                                               CURLINFO_RESPONSE_CODE,
                                                               &mut code);
        Com_Error(ERR_DROP as libc::c_int,
                  b"Download Error: %s Code: %ld URL: %s\x00" as *const u8 as
                      *const libc::c_char,
                  qcurl_easy_strerror.expect("non-null function pointer")((*msg).data.result),
                  code, clc.downloadURL.as_mut_ptr());
    }
    CL_NextDownload();
}