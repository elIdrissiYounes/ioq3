use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::uint8_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::bg_public_h::GT_1FCTF;
pub use crate::bg_public_h::GT_CTF;
pub use crate::bg_public_h::GT_FFA;
pub use crate::bg_public_h::GT_HARVESTER;
pub use crate::bg_public_h::GT_MAX_GAME_TYPE;
pub use crate::bg_public_h::GT_OBELISK;
pub use crate::bg_public_h::GT_SINGLE_PLAYER;
pub use crate::bg_public_h::GT_TEAM;
pub use crate::bg_public_h::GT_TOURNAMENT;
pub use crate::g_public_h::entityShared_t;
pub use crate::g_public_h::sharedEntity_t;
pub use crate::g_public_h::BOTAI_START_FRAME;
pub use crate::g_public_h::GAME_CLIENT_BEGIN;
pub use crate::g_public_h::GAME_CLIENT_COMMAND;
pub use crate::g_public_h::GAME_CLIENT_CONNECT;
pub use crate::g_public_h::GAME_CLIENT_DISCONNECT;
pub use crate::g_public_h::GAME_CLIENT_THINK;
pub use crate::g_public_h::GAME_CLIENT_USERINFO_CHANGED;
pub use crate::g_public_h::GAME_CONSOLE_COMMAND;
pub use crate::g_public_h::GAME_INIT;
pub use crate::g_public_h::GAME_RUN_FRAME;
pub use crate::g_public_h::GAME_SHUTDOWN;
pub use crate::qcommon_h::clc_EOF;
pub use crate::qcommon_h::clc_bad;
pub use crate::qcommon_h::clc_clientCommand;
pub use crate::qcommon_h::clc_move;
pub use crate::qcommon_h::clc_moveNoDelta;
pub use crate::qcommon_h::clc_nop;
pub use crate::qcommon_h::clc_ops_e;
pub use crate::qcommon_h::clc_voipOpus;
pub use crate::qcommon_h::clc_voipSpeex;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
pub use crate::qcommon_h::svc_EOF;
pub use crate::qcommon_h::svc_bad;
pub use crate::qcommon_h::svc_baseline;
pub use crate::qcommon_h::svc_configstring;
pub use crate::qcommon_h::svc_download;
pub use crate::qcommon_h::svc_gamestate;
pub use crate::qcommon_h::svc_nop;
pub use crate::qcommon_h::svc_ops_e;
pub use crate::qcommon_h::svc_serverCommand;
pub use crate::qcommon_h::svc_snapshot;
pub use crate::qcommon_h::svc_voipOpus;
pub use crate::qcommon_h::svc_voipSpeex;
pub use crate::qcommon_h::vm_t;
pub use crate::qcommon_h::NA_BAD;
pub use crate::qcommon_h::NA_BOT;
pub use crate::qcommon_h::NA_BROADCAST;
pub use crate::qcommon_h::NA_IP;
pub use crate::qcommon_h::NA_IP6;
pub use crate::qcommon_h::NA_LOOPBACK;
pub use crate::qcommon_h::NA_MULTICAST6;
pub use crate::qcommon_h::NA_UNSPEC;
pub use crate::qcommon_h::NS_CLIENT;
pub use crate::qcommon_h::NS_SERVER;
pub use crate::server_h::challenge_t;
pub use crate::server_h::clientSnapshot_t;
pub use crate::server_h::clientState_t;
pub use crate::server_h::client_s;
pub use crate::server_h::client_t;
pub use crate::server_h::leakyBucket_s;
pub use crate::server_h::leakyBucket_t;
pub use crate::server_h::netchan_buffer_s;
pub use crate::server_h::netchan_buffer_t;
pub use crate::server_h::serverBan_t;
pub use crate::server_h::serverState_t;
pub use crate::server_h::serverStatic_t;
pub use crate::server_h::server_t;
pub use crate::server_h::svEntity_s;
pub use crate::server_h::svEntity_t;
pub use crate::server_h::voipServerPacket_s;
pub use crate::server_h::voipServerPacket_t;
pub use crate::server_h::worldSector_s;
pub use crate::server_h::C2RustUnnamed_164;
pub use crate::server_h::CS_ACTIVE;
pub use crate::server_h::CS_CONNECTED;
pub use crate::server_h::CS_FREE;
pub use crate::server_h::CS_PRIMED;
pub use crate::server_h::CS_ZOMBIE;
pub use crate::server_h::SS_DEAD;
pub use crate::server_h::SS_GAME;
pub use crate::server_h::SS_LOADING;
pub use crate::src::qcommon::cmd::Cmd_Argc;
pub use crate::src::qcommon::cmd::Cmd_Args_Sanitize;
pub use crate::src::qcommon::cmd::Cmd_Argv;
pub use crate::src::qcommon::cmd::Cmd_TokenizeString;
pub use crate::src::qcommon::cmd::Cmd_TokenizeStringIgnoreQuotes;
pub use crate::src::qcommon::common::com_cl_running;
pub use crate::src::qcommon::common::com_dedicated;
pub use crate::src::qcommon::common::com_gamename;
pub use crate::src::qcommon::common::com_legacyprotocol;
pub use crate::src::qcommon::common::com_protocol;
pub use crate::src::qcommon::common::com_standalone;
pub use crate::src::qcommon::common::Com_DPrintf;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_IsVoipTarget;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::common::Z_Malloc;
pub use crate::src::qcommon::cvar::Cvar_VariableString;
pub use crate::src::qcommon::cvar::Cvar_VariableValue;
pub use crate::src::qcommon::files::FS_FCloseFile;
pub use crate::src::qcommon::files::FS_FileIsInPAK;
pub use crate::src::qcommon::files::FS_FilenameCompare;
pub use crate::src::qcommon::files::FS_LoadedPakPureChecksums;
pub use crate::src::qcommon::files::FS_Read;
pub use crate::src::qcommon::files::FS_ReferencedPakNames;
pub use crate::src::qcommon::files::FS_SV_FOpenFileRead;
pub use crate::src::qcommon::files::FS_idPak;
pub use crate::src::qcommon::msg::MSG_Bitstream;
pub use crate::src::qcommon::msg::MSG_HashKey;
pub use crate::src::qcommon::msg::MSG_Init;
pub use crate::src::qcommon::msg::MSG_ReadByte;
pub use crate::src::qcommon::msg::MSG_ReadData;
pub use crate::src::qcommon::msg::MSG_ReadDeltaUsercmdKey;
pub use crate::src::qcommon::msg::MSG_ReadLong;
pub use crate::src::qcommon::msg::MSG_ReadShort;
pub use crate::src::qcommon::msg::MSG_ReadString;
pub use crate::src::qcommon::msg::MSG_WriteBigString;
pub use crate::src::qcommon::msg::MSG_WriteByte;
pub use crate::src::qcommon::msg::MSG_WriteData;
pub use crate::src::qcommon::msg::MSG_WriteDeltaEntity;
pub use crate::src::qcommon::msg::MSG_WriteLong;
pub use crate::src::qcommon::msg::MSG_WriteShort;
pub use crate::src::qcommon::msg::MSG_WriteString;
pub use crate::src::qcommon::net_chan::NET_OutOfBandPrint;
pub use crate::src::qcommon::net_chan::NET_StringToAdr;
pub use crate::src::qcommon::net_chan::Netchan_Setup;
pub use crate::src::qcommon::net_ip::NET_AdrToString;
pub use crate::src::qcommon::net_ip::NET_CompareAdr;
pub use crate::src::qcommon::net_ip::NET_CompareBaseAdr;
pub use crate::src::qcommon::net_ip::NET_CompareBaseAdrMask;
pub use crate::src::qcommon::net_ip::NET_IsLocalAddress;
pub use crate::src::qcommon::net_ip::Sys_IsLANAddress;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::entityState_s;
pub use crate::src::qcommon::q_shared::entityState_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::playerState_s;
pub use crate::src::qcommon::q_shared::playerState_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::trType_t;
pub use crate::src::qcommon::q_shared::trajectory_t;
pub use crate::src::qcommon::q_shared::usercmd_s;
pub use crate::src::qcommon::q_shared::usercmd_t;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Com_sprintf;
pub use crate::src::qcommon::q_shared::Info_SetValueForKey;
pub use crate::src::qcommon::q_shared::Info_ValueForKey;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;

pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::TR_GRAVITY;
pub use crate::src::qcommon::q_shared::TR_INTERPOLATE;
pub use crate::src::qcommon::q_shared::TR_LINEAR;
pub use crate::src::qcommon::q_shared::TR_LINEAR_STOP;
pub use crate::src::qcommon::q_shared::TR_SINE;
pub use crate::src::qcommon::q_shared::TR_STATIONARY;
pub use crate::src::qcommon::vm::VM_Call;
pub use crate::src::qcommon::vm::VM_ExplicitArgPtr;
pub use crate::src::server::sv_bot::SV_BotFreeClient;
pub use crate::src::server::sv_ccmds::SV_Heartbeat_f;
pub use crate::src::server::sv_client::stdlib_h::atoi;
pub use crate::src::server::sv_game::SV_GentityNum;
pub use crate::src::server::sv_init::SV_SetUserinfo;
pub use crate::src::server::sv_init::SV_UpdateConfigstrings;
pub use crate::src::server::sv_main::gvm;
pub use crate::src::server::sv_main::outboundLeakyBucket;
pub use crate::src::server::sv_main::serverBans;
pub use crate::src::server::sv_main::serverBansCount;
pub use crate::src::server::sv_main::sv;
pub use crate::src::server::sv_main::sv_allowDownload;
pub use crate::src::server::sv_main::sv_floodProtect;
pub use crate::src::server::sv_main::sv_fps;
pub use crate::src::server::sv_main::sv_lanForceRate;
pub use crate::src::server::sv_main::sv_maxPing;
pub use crate::src::server::sv_main::sv_maxclients;
pub use crate::src::server::sv_main::sv_minPing;
pub use crate::src::server::sv_main::sv_privateClients;
pub use crate::src::server::sv_main::sv_privatePassword;
pub use crate::src::server::sv_main::sv_pure;
pub use crate::src::server::sv_main::sv_reconnectlimit;
pub use crate::src::server::sv_main::sv_strictAuth;
pub use crate::src::server::sv_main::sv_voip;
pub use crate::src::server::sv_main::svs;
pub use crate::src::server::sv_main::SVC_RateLimit;
pub use crate::src::server::sv_main::SVC_RateLimitAddress;
pub use crate::src::server::sv_main::SV_RateMsec;
pub use crate::src::server::sv_main::SV_SendServerCommand;
pub use crate::src::server::sv_net_chan::SV_Netchan_FreeQueue;
pub use crate::src::server::sv_net_chan::SV_Netchan_Transmit;
pub use crate::src::server::sv_net_chan::SV_Netchan_TransmitNextFragment;
pub use crate::src::server::sv_snapshot::SV_SendClientSnapshot;
pub use crate::src::server::sv_snapshot::SV_SendMessageToClient;
pub use crate::src::server::sv_snapshot::SV_UpdateServerCommandsToClient;

pub use crate::stdlib::rand;

pub use crate::stdlib::strtol;
pub use crate::vm_local_h::vm_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucmd_t {
    pub name: *mut i8,
    pub func: Option<unsafe extern "C" fn(_: *mut crate::server_h::client_t) -> ()>,
}
/*
=================
SV_GetChallenge

A "getchallenge" OOB command has been received
Returns a challenge number that can be used
in a subsequent connectResponse command.
We do this to prevent denial of service attacks that
flood the server with invalid connection IPs.  With a
challenge, they must give a valid IP address.

If we are authorizing, a challenge request will cause a packet
to be sent to the authorize server.

When an authorizeip is returned, a challenge response will be
sent to that ip.

ioquake3: we added a possibility for clients to add a challenge
to their packets, to make it more difficult for malicious servers
to hi-jack client connections.
Also, the auth stuff is completely disabled for com_standalone games
as well as IPv6 connections, since there is no way to use the
v4-only auth server for these new types of connections.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_GetChallenge(mut from: crate::qcommon_h::netadr_t) {
    let mut i: i32 = 0;
    let mut oldest: i32 = 0;
    let mut oldestTime: i32 = 0;
    let mut oldestClientTime: i32 = 0;
    let mut clientChallenge: i32 = 0;
    let mut challenge: *mut crate::server_h::challenge_t = 0 as *mut crate::server_h::challenge_t;
    let mut wasfound: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    let mut gameName: *mut i8 = 0 as *mut i8;
    let mut gameMismatch: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    // ignore if we are in single player
    if crate::src::qcommon::cvar::Cvar_VariableValue(b"g_gametype\x00" as *const u8 as *const i8)
        == crate::bg_public_h::GT_SINGLE_PLAYER as i32 as f32
        || crate::src::qcommon::cvar::Cvar_VariableValue(
            b"ui_singlePlayerActive\x00" as *const u8 as *const i8,
        ) != 0.
    {
        return;
    }
    // Prevent using getchallenge as an amplifier
    if crate::src::server::sv_main::SVC_RateLimitAddress(from, 10, 1000) as u64 != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"SV_GetChallenge: rate limit from %s exceeded, dropping request\n\x00" as *const u8
                as *const i8,
            crate::src::qcommon::net_ip::NET_AdrToString(from),
        );
        return;
    }
    // Allow getchallenge to be DoSed relatively easily, but prevent
    // excess outbound bandwidth usage when being flooded inbound
    if crate::src::server::sv_main::SVC_RateLimit(
        &mut crate::src::server::sv_main::outboundLeakyBucket,
        10,
        100,
    ) as u64
        != 0
    {
        crate::src::qcommon::common::Com_DPrintf(
            b"SV_GetChallenge: rate limit exceeded, dropping request\n\x00" as *const u8
                as *const i8,
        );
        return;
    }
    gameName = crate::src::qcommon::cmd::Cmd_Argv(2);
    // gamename is optional for legacy protocol
    if (*crate::src::qcommon::common::com_legacyprotocol).integer != 0 && *gameName == 0 {
        gameMismatch = crate::src::qcommon::q_shared::qfalse
    } else {
        gameMismatch = (*gameName == 0
            || crate::stdlib::strcmp(
                gameName,
                (*crate::src::qcommon::common::com_gamename).string,
            ) != 0) as crate::src::qcommon::q_shared::qboolean
    }
    // reject client if the gamename string sent by the client doesn't match ours
    if gameMismatch as u64 != 0 {
        crate::src::qcommon::net_chan::NET_OutOfBandPrint(
            crate::qcommon_h::NS_SERVER,
            from,
            b"print\nGame mismatch: This is a %s server\n\x00" as *const u8 as *const i8,
            (*crate::src::qcommon::common::com_gamename).string,
        );
        return;
    }
    oldest = 0;
    oldestTime = 0x7fffffff;
    oldestClientTime = oldestTime;
    // see if we already have a challenge for this ip
    challenge = &mut *crate::src::server::sv_main::svs
        .challenges
        .as_mut_ptr()
        .offset(0) as *mut crate::server_h::challenge_t;
    clientChallenge = atoi(crate::src::qcommon::cmd::Cmd_Argv(1));
    i = 0;
    while i < 2048 {
        if (*challenge).connected as u64 == 0
            && crate::src::qcommon::net_ip::NET_CompareAdr(from, (*challenge).adr) != 0
        {
            wasfound = crate::src::qcommon::q_shared::qtrue;
            if (*challenge).time < oldestClientTime {
                oldestClientTime = (*challenge).time
            }
        }
        if wasfound != 0 && i >= 2048 / 2 {
            i = 2048;
            break;
        } else {
            if (*challenge).time < oldestTime {
                oldestTime = (*challenge).time;
                oldest = i
            }
            i += 1;
            challenge = challenge.offset(1)
        }
    }
    if i == 2048 {
        // this is the first time this client has asked for a challenge
        challenge = &mut *crate::src::server::sv_main::svs
            .challenges
            .as_mut_ptr()
            .offset(oldest as isize) as *mut crate::server_h::challenge_t;
        (*challenge).clientChallenge = clientChallenge;
        (*challenge).adr = from;
        (*challenge).firstTime = crate::src::server::sv_main::svs.time;
        (*challenge).connected = crate::src::qcommon::q_shared::qfalse
    }
    // always generate a new challenge number, so the client cannot circumvent sv_maxping
    (*challenge).challenge = ((crate::stdlib::rand() as u32) << 16
        ^ crate::stdlib::rand() as u32
        ^ crate::src::server::sv_main::svs.time as u32) as i32;
    (*challenge).wasrefused = crate::src::qcommon::q_shared::qfalse;
    (*challenge).time = crate::src::server::sv_main::svs.time;
    // Drop the authorize stuff if this client is coming in via v6 as the auth server does not support ipv6.
    // Drop also for addresses coming in on local LAN and for stand-alone games independent from id's assets.
    if (*challenge).adr.type_0 == crate::qcommon_h::NA_IP
        && (*crate::src::qcommon::common::com_standalone).integer == 0
        && crate::src::qcommon::net_ip::Sys_IsLANAddress(from) as u64 == 0
    {
        // look up the authorize server's IP
        if crate::src::server::sv_main::svs.authorizeAddress.type_0 == crate::qcommon_h::NA_BAD {
            crate::src::qcommon::common::Com_Printf(
                b"Resolving %s\n\x00" as *const u8 as *const i8,
                b"authorize.quake3arena.com\x00" as *const u8 as *const i8,
            );
            if crate::src::qcommon::net_chan::NET_StringToAdr(
                b"authorize.quake3arena.com\x00" as *const u8 as *const i8,
                &mut crate::src::server::sv_main::svs.authorizeAddress,
                crate::qcommon_h::NA_IP,
            ) != 0
            {
                crate::src::server::sv_main::svs.authorizeAddress.port =
                    crate::src::qcommon::q_shared::ShortSwap(27952) as u16;
                crate::src::qcommon::common::Com_Printf(
                    b"%s resolved to %i.%i.%i.%i:%i\n\x00" as *const u8 as *const i8,
                    b"authorize.quake3arena.com\x00" as *const u8 as *const i8,
                    crate::src::server::sv_main::svs.authorizeAddress.ip[0usize] as i32,
                    crate::src::server::sv_main::svs.authorizeAddress.ip[1usize] as i32,
                    crate::src::server::sv_main::svs.authorizeAddress.ip[2usize] as i32,
                    crate::src::server::sv_main::svs.authorizeAddress.ip[3usize] as i32,
                    crate::src::qcommon::q_shared::ShortSwap(
                        crate::src::server::sv_main::svs.authorizeAddress.port as i16,
                    ) as i32,
                );
            }
        }
        // we couldn't contact the auth server, let them in.
        if crate::src::server::sv_main::svs.authorizeAddress.type_0 == crate::qcommon_h::NA_BAD {
            crate::src::qcommon::common::Com_Printf(
                b"Couldn\'t resolve auth server address\n\x00" as *const u8 as *const i8,
            );
        } else if crate::src::server::sv_main::svs.time - oldestClientTime > 5000 {
            crate::src::qcommon::common::Com_DPrintf(
                b"authorize server timed out\n\x00" as *const u8 as *const i8,
            );
        } else {
            // if they have been challenging for a long time and we
            // haven't heard anything from the authorize server, go ahead and
            // let them in, assuming the id server is down
            // otherwise send their ip to the authorize server
            let mut game: *const i8 = 0 as *const i8;
            crate::src::qcommon::common::Com_DPrintf(
                b"sending getIpAuthorize for %s\n\x00" as *const u8 as *const i8,
                crate::src::qcommon::net_ip::NET_AdrToString(from),
            );
            game = crate::src::qcommon::cvar::Cvar_VariableString(
                b"fs_game\x00" as *const u8 as *const i8,
            );
            if *game.offset(0) as i32 == 0 {
                game = b"baseq3\x00" as *const u8 as *const i8
            }
            // the 0 is for backwards compatibility with obsolete sv_allowanonymous flags
            // getIpAuthorize <challenge> <IP> <game> 0 <auth-flag>
            crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                crate::qcommon_h::NS_SERVER,
                crate::src::server::sv_main::svs.authorizeAddress,
                b"getIpAuthorize %i %i.%i.%i.%i %s 0 %s\x00" as *const u8 as *const i8,
                (*challenge).challenge,
                from.ip[0usize] as i32,
                from.ip[1usize] as i32,
                from.ip[2usize] as i32,
                from.ip[3usize] as i32,
                game,
                (*crate::src::server::sv_main::sv_strictAuth).string,
            );
            return;
        }
    }
    (*challenge).pingTime = crate::src::server::sv_main::svs.time;
    crate::src::qcommon::net_chan::NET_OutOfBandPrint(
        crate::qcommon_h::NS_SERVER,
        (*challenge).adr,
        b"challengeResponse %d %d %d\x00" as *const u8 as *const i8,
        (*challenge).challenge,
        clientChallenge,
        (*crate::src::qcommon::common::com_protocol).integer,
    );
}
/*
====================
SV_AuthorizeIpPacket

A packet has been returned from the authorize server.
If we have a challenge adr for that ip, send the
challengeResponse to it
====================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_AuthorizeIpPacket(mut from: crate::qcommon_h::netadr_t) {
    let mut challenge: i32 = 0;
    let mut i: i32 = 0;
    let mut s: *mut i8 = 0 as *mut i8;
    let mut r: *mut i8 = 0 as *mut i8;
    let mut challengeptr: *mut crate::server_h::challenge_t =
        0 as *mut crate::server_h::challenge_t;
    if crate::src::qcommon::net_ip::NET_CompareBaseAdr(
        from,
        crate::src::server::sv_main::svs.authorizeAddress,
    ) as u64
        == 0
    {
        crate::src::qcommon::common::Com_Printf(
            b"SV_AuthorizeIpPacket: not from authorize server\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    challenge = atoi(crate::src::qcommon::cmd::Cmd_Argv(1));
    i = 0;
    while i < 2048 {
        if crate::src::server::sv_main::svs.challenges[i as usize].challenge == challenge {
            break;
        }
        i += 1
    }
    if i == 2048 {
        crate::src::qcommon::common::Com_Printf(
            b"SV_AuthorizeIpPacket: challenge not found\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    challengeptr = &mut *crate::src::server::sv_main::svs
        .challenges
        .as_mut_ptr()
        .offset(i as isize) as *mut crate::server_h::challenge_t;
    // send a packet back to the original client
    (*challengeptr).pingTime = crate::src::server::sv_main::svs.time; // reason
    s = crate::src::qcommon::cmd::Cmd_Argv(2);
    r = crate::src::qcommon::cmd::Cmd_Argv(3);
    if crate::src::qcommon::q_shared::Q_stricmp(s, b"demo\x00" as *const u8 as *const i8) == 0 {
        // they are a demo client trying to connect to a real server
        crate::src::qcommon::net_chan::NET_OutOfBandPrint(
            crate::qcommon_h::NS_SERVER,
            (*challengeptr).adr,
            b"print\nServer is not a demo server\n\x00" as *const u8 as *const i8,
        );
        // clear the challenge record so it won't timeout and let them through
        crate::stdlib::memset(
            challengeptr as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::server_h::challenge_t>(),
        );
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(s, b"accept\x00" as *const u8 as *const i8) == 0 {
        crate::src::qcommon::net_chan::NET_OutOfBandPrint(
            crate::qcommon_h::NS_SERVER,
            (*challengeptr).adr,
            b"challengeResponse %d %d %d\x00" as *const u8 as *const i8,
            (*challengeptr).challenge,
            (*challengeptr).clientChallenge,
            (*crate::src::qcommon::common::com_protocol).integer,
        );
        return;
    }
    if crate::src::qcommon::q_shared::Q_stricmp(s, b"unknown\x00" as *const u8 as *const i8) == 0 {
        if r.is_null() {
            crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                crate::qcommon_h::NS_SERVER,
                (*challengeptr).adr,
                b"print\nAwaiting CD key authorization\n\x00" as *const u8 as *const i8,
            );
        } else {
            crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                crate::qcommon_h::NS_SERVER,
                (*challengeptr).adr,
                b"print\n%s\n\x00" as *const u8 as *const i8,
                r,
            );
        }
        // clear the challenge record so it won't timeout and let them through
        crate::stdlib::memset(
            challengeptr as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::server_h::challenge_t>(),
        );
        return;
    }
    // authorization failed
    if r.is_null() {
        crate::src::qcommon::net_chan::NET_OutOfBandPrint(
            crate::qcommon_h::NS_SERVER,
            (*challengeptr).adr,
            b"print\nSomeone is using this CD Key\n\x00" as *const u8 as *const i8,
        );
    } else {
        crate::src::qcommon::net_chan::NET_OutOfBandPrint(
            crate::qcommon_h::NS_SERVER,
            (*challengeptr).adr,
            b"print\n%s\n\x00" as *const u8 as *const i8,
            r,
        );
    }
    // clear the challenge record so it won't timeout and let them through
    crate::stdlib::memset(
        challengeptr as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::server_h::challenge_t>(),
    );
}
/*
==================
SV_IsBanned

Check whether a certain address is banned
==================
*/

unsafe extern "C" fn SV_IsBanned(
    mut from: *mut crate::qcommon_h::netadr_t,
    mut isexception: crate::src::qcommon::q_shared::qboolean,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut _index: i32 = 0;
    let mut curban: *mut crate::server_h::serverBan_t = 0 as *mut crate::server_h::serverBan_t;
    if isexception as u64 == 0 {
        // If this is a query for a ban, first check whether the client is excepted
        if SV_IsBanned(from, crate::src::qcommon::q_shared::qtrue) as u64 != 0 {
            return crate::src::qcommon::q_shared::qfalse;
        }
    }

    for index in 0..crate::src::server::sv_main::serverBansCount {
        curban = &mut *crate::src::server::sv_main::serverBans
            .as_mut_ptr()
            .offset(index as isize) as *mut crate::server_h::serverBan_t;

        if (*curban).isexception == isexception {
            if crate::src::qcommon::net_ip::NET_CompareBaseAdrMask(
                (*curban).ip,
                *from,
                (*curban).subnet,
            ) as u64
                != 0
            {
                return crate::src::qcommon::q_shared::qtrue;
            }
        }
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
==================
SV_DirectConnect

A "connect" OOB command has been received
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_DirectConnect(mut from: crate::qcommon_h::netadr_t) {
    let mut current_block: u64;
    let mut userinfo: [i8; 1024] = [0; 1024];
    let mut i: i32 = 0;
    let mut cl: *mut crate::server_h::client_t = 0 as *mut crate::server_h::client_t;
    let mut newcl: *mut crate::server_h::client_t = 0 as *mut crate::server_h::client_t;
    let mut temp: crate::server_h::client_t = crate::server_h::client_t {
        state: crate::server_h::CS_FREE,
        userinfo: [0; 1024],
        reliableCommands: [[0; 1024]; 64],
        reliableSequence: 0,
        reliableAcknowledge: 0,
        reliableSent: 0,
        messageAcknowledge: 0,
        gamestateMessageNum: 0,
        challenge: 0,
        lastUsercmd: crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        },
        lastMessageNum: 0,
        lastClientCommand: 0,
        lastClientCommandString: [0; 1024],
        gentity: 0 as *mut crate::g_public_h::sharedEntity_t,
        name: [0; 32],
        downloadName: [0; 64],
        download: 0,
        downloadSize: 0,
        downloadCount: 0,
        downloadClientBlock: 0,
        downloadCurrentBlock: 0,
        downloadXmitBlock: 0,
        downloadBlocks: [0 as *mut u8; 48],
        downloadBlockSize: [0; 48],
        downloadEOF: crate::src::qcommon::q_shared::qfalse,
        downloadSendTime: 0,
        deltaMessage: 0,
        nextReliableTime: 0,
        lastPacketTime: 0,
        lastConnectTime: 0,
        lastSnapshotTime: 0,
        rateDelayed: crate::src::qcommon::q_shared::qfalse,
        timeoutCount: 0,
        frames: [crate::server_h::clientSnapshot_t {
            areabytes: 0,
            areabits: [0; 32],
            ps: crate::src::qcommon::q_shared::playerState_t {
                commandTime: 0,
                pm_type: 0,
                bobCycle: 0,
                pm_flags: 0,
                pm_time: 0,
                origin: [0.; 3],
                velocity: [0.; 3],
                weaponTime: 0,
                gravity: 0,
                speed: 0,
                delta_angles: [0; 3],
                groundEntityNum: 0,
                legsTimer: 0,
                legsAnim: 0,
                torsoTimer: 0,
                torsoAnim: 0,
                movementDir: 0,
                grapplePoint: [0.; 3],
                eFlags: 0,
                eventSequence: 0,
                events: [0; 2],
                eventParms: [0; 2],
                externalEvent: 0,
                externalEventParm: 0,
                externalEventTime: 0,
                clientNum: 0,
                weapon: 0,
                weaponstate: 0,
                viewangles: [0.; 3],
                viewheight: 0,
                damageEvent: 0,
                damageYaw: 0,
                damagePitch: 0,
                damageCount: 0,
                stats: [0; 16],
                persistant: [0; 16],
                powerups: [0; 16],
                ammo: [0; 16],
                generic1: 0,
                loopSound: 0,
                jumppad_ent: 0,
                ping: 0,
                pmove_framecount: 0,
                jumppad_frame: 0,
                entityEventSequence: 0,
            },
            num_entities: 0,
            first_entity: 0,
            messageSent: 0,
            messageAcked: 0,
            messageSize: 0,
        }; 32],
        ping: 0,
        rate: 0,
        snapshotMsec: 0,
        pureAuthentic: 0,
        gotCP: crate::src::qcommon::q_shared::qfalse,
        netchan: crate::qcommon_h::netchan_t {
            sock: crate::qcommon_h::NS_CLIENT,
            dropped: 0,
            remoteAddress: crate::qcommon_h::netadr_t {
                type_0: crate::qcommon_h::NA_BAD,
                ip: [0; 4],
                ip6: [0; 16],
                port: 0,
                scope_id: 0,
            },
            qport: 0,
            incomingSequence: 0,
            outgoingSequence: 0,
            fragmentSequence: 0,
            fragmentLength: 0,
            fragmentBuffer: [0; 16384],
            unsentFragments: crate::src::qcommon::q_shared::qfalse,
            unsentFragmentStart: 0,
            unsentLength: 0,
            unsentBuffer: [0; 16384],
            challenge: 0,
            lastSentTime: 0,
            lastSentSize: 0,
            compat: crate::src::qcommon::q_shared::qfalse,
        },
        netchan_start_queue: 0 as *mut crate::server_h::netchan_buffer_t,
        netchan_end_queue: 0 as *mut *mut crate::server_h::netchan_buffer_t,
        hasVoip: crate::src::qcommon::q_shared::qfalse,
        muteAllVoip: crate::src::qcommon::q_shared::qfalse,
        ignoreVoipFromClient: [crate::src::qcommon::q_shared::qfalse; 64],
        voipPacket: [0 as *mut crate::server_h::voipServerPacket_t; 64],
        queuedVoipPackets: 0,
        queuedVoipIndex: 0,
        oldServerTime: 0,
        csUpdated: [crate::src::qcommon::q_shared::qfalse; 1024],
        compat: crate::src::qcommon::q_shared::qfalse,
    };
    let mut ent: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    let mut clientNum: i32 = 0;
    let mut version: i32 = 0;
    let mut qport: i32 = 0;
    let mut challenge: i32 = 0;
    let mut password: *mut i8 = 0 as *mut i8;
    let mut startIndex: i32 = 0;
    let mut denied: crate::stdlib::intptr_t = 0;
    let mut count: i32 = 0;
    let mut ip: *mut i8 = 0 as *mut i8;
    let mut compat: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::common::Com_DPrintf(
        b"SVC_DirectConnect ()\n\x00" as *const u8 as *const i8,
    );
    // Check whether this client is banned.
    if SV_IsBanned(&mut from, crate::src::qcommon::q_shared::qfalse) as u64 != 0 {
        crate::src::qcommon::net_chan::NET_OutOfBandPrint(
            crate::qcommon_h::NS_SERVER,
            from,
            b"print\nYou are banned from this server.\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        userinfo.as_mut_ptr(),
        crate::src::qcommon::cmd::Cmd_Argv(1),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    version = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"protocol\x00" as *const u8 as *const i8,
    ));
    if version > 0 && (*crate::src::qcommon::common::com_legacyprotocol).integer == version {
        compat = crate::src::qcommon::q_shared::qtrue
    } else if version != (*crate::src::qcommon::common::com_protocol).integer {
        crate::src::qcommon::net_chan::NET_OutOfBandPrint(
            crate::qcommon_h::NS_SERVER,
            from,
            b"print\nServer uses protocol version %i (yours is %i).\n\x00" as *const u8
                as *const i8,
            (*crate::src::qcommon::common::com_protocol).integer,
            version,
        );
        crate::src::qcommon::common::Com_DPrintf(
            b"    rejected connect from version %i\n\x00" as *const u8 as *const i8,
            version,
        );
        return;
    }
    challenge = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"challenge\x00" as *const u8 as *const i8,
    ));
    qport = atoi(crate::src::qcommon::q_shared::Info_ValueForKey(
        userinfo.as_mut_ptr(),
        b"qport\x00" as *const u8 as *const i8,
    ));
    // quick reject
    i = 0;
    cl = crate::src::server::sv_main::svs.clients;
    while i < (*crate::src::server::sv_main::sv_maxclients).integer {
        if !((*cl).state == crate::server_h::CS_FREE) {
            if crate::src::qcommon::net_ip::NET_CompareBaseAdr(from, (*cl).netchan.remoteAddress)
                != 0
                && ((*cl).netchan.qport == qport
                    || from.port as i32 == (*cl).netchan.remoteAddress.port as i32)
            {
                if crate::src::server::sv_main::svs.time - (*cl).lastConnectTime
                    < (*crate::src::server::sv_main::sv_reconnectlimit).integer * 1000
                {
                    crate::src::qcommon::common::Com_DPrintf(
                        b"%s:reconnect rejected : too soon\n\x00" as *const u8 as *const i8,
                        crate::src::qcommon::net_ip::NET_AdrToString(from),
                    );
                    return;
                }
                break;
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
    // don't let "ip" overflow userinfo string
    if crate::src::qcommon::net_ip::NET_IsLocalAddress(from) as u64 != 0 {
        ip = b"localhost\x00" as *const u8 as *mut i8
    } else {
        ip = crate::src::qcommon::net_ip::NET_AdrToString(from) as *mut i8
    }
    if crate::stdlib::strlen(ip)
        .wrapping_add(crate::stdlib::strlen(userinfo.as_mut_ptr()))
        .wrapping_add(4usize)
        >= 1024usize
    {
        crate::src::qcommon::net_chan::NET_OutOfBandPrint(crate::qcommon_h::NS_SERVER, from,
                           b"print\nUserinfo string length exceeded.  Try removing setu cvars from your config.\n\x00"
                               as *const u8 as *const i8);
        return;
    }
    crate::src::qcommon::q_shared::Info_SetValueForKey(
        userinfo.as_mut_ptr(),
        b"ip\x00" as *const u8 as *const i8,
        ip,
    );
    // see if the challenge is valid (LAN clients don't need to challenge)
    if crate::src::qcommon::net_ip::NET_IsLocalAddress(from) as u64 == 0 {
        let mut ping: i32 = 0;
        let mut challengeptr: *mut crate::server_h::challenge_t =
            0 as *mut crate::server_h::challenge_t;
        i = 0;
        while i < 2048 {
            if crate::src::qcommon::net_ip::NET_CompareAdr(
                from,
                crate::src::server::sv_main::svs.challenges[i as usize].adr,
            ) as u64
                != 0
            {
                if challenge == crate::src::server::sv_main::svs.challenges[i as usize].challenge {
                    break;
                }
            }
            i += 1
        }
        if i == 2048 {
            crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                crate::qcommon_h::NS_SERVER,
                from,
                b"print\nNo or bad challenge for your address.\n\x00" as *const u8 as *const i8,
            );
            return;
        }
        challengeptr = &mut *crate::src::server::sv_main::svs
            .challenges
            .as_mut_ptr()
            .offset(i as isize) as *mut crate::server_h::challenge_t;
        if (*challengeptr).wasrefused as u64 != 0 {
            // Return silently, so that error messages written by the server keep being displayed.
            return;
        }
        ping = crate::src::server::sv_main::svs.time - (*challengeptr).pingTime;
        // never reject a LAN client based on ping
        if crate::src::qcommon::net_ip::Sys_IsLANAddress(from) as u64 == 0 {
            if (*crate::src::server::sv_main::sv_minPing).value != 0.
                && (ping as f32) < (*crate::src::server::sv_main::sv_minPing).value
            {
                crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                    crate::qcommon_h::NS_SERVER,
                    from,
                    b"print\nServer is for high pings only\n\x00" as *const u8 as *const i8,
                );
                crate::src::qcommon::common::Com_DPrintf(
                    b"Client %i rejected on a too low ping\n\x00" as *const u8 as *const i8,
                    i,
                );
                (*challengeptr).wasrefused = crate::src::qcommon::q_shared::qtrue;
                return;
            }
            if (*crate::src::server::sv_main::sv_maxPing).value != 0.
                && ping as f32 > (*crate::src::server::sv_main::sv_maxPing).value
            {
                crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                    crate::qcommon_h::NS_SERVER,
                    from,
                    b"print\nServer is for low pings only\n\x00" as *const u8 as *const i8,
                );
                crate::src::qcommon::common::Com_DPrintf(
                    b"Client %i rejected on a too high ping\n\x00" as *const u8 as *const i8,
                    i,
                );
                (*challengeptr).wasrefused = crate::src::qcommon::q_shared::qtrue;
                return;
            }
        }
        crate::src::qcommon::common::Com_Printf(
            b"Client %i connecting with %i challenge ping\n\x00" as *const u8 as *const i8,
            i,
            ping,
        );
        (*challengeptr).connected = crate::src::qcommon::q_shared::qtrue
    }
    newcl = &mut temp;
    crate::stdlib::memset(
        newcl as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::server_h::client_t>(),
    );
    // if there is already a slot for this ip, reuse it
    i = 0;
    cl = crate::src::server::sv_main::svs.clients;
    loop {
        if !(i < (*crate::src::server::sv_main::sv_maxclients).integer) {
            current_block = 479107131381816815;
            break;
        }
        if !((*cl).state == crate::server_h::CS_FREE) {
            if crate::src::qcommon::net_ip::NET_CompareBaseAdr(from, (*cl).netchan.remoteAddress)
                != 0
                && ((*cl).netchan.qport == qport
                    || from.port as i32 == (*cl).netchan.remoteAddress.port as i32)
            {
                crate::src::qcommon::common::Com_Printf(
                    b"%s:reconnect\n\x00" as *const u8 as *const i8,
                    crate::src::qcommon::net_ip::NET_AdrToString(from),
                );
                newcl = cl;
                current_block = 4128612776030147116;
                break;
            }
        }
        i += 1;
        cl = cl.offset(1)
    }
    match current_block {
        479107131381816815 => {
            // find a client slot
            // if "sv_privateClients" is set > 0, then that number
            // of client slots will be reserved for connections that
            // have "password" set to the value of "sv_privatePassword"
            // Info requests will report the maxclients as if the private
            // slots didn't exist, to prevent people from trying to connect
            // to a full server.
            // This is to allow us to reserve a couple slots here on our
            // servers so we can play without having to kick people.
            // check for privateClient password
            password = crate::src::qcommon::q_shared::Info_ValueForKey(
                userinfo.as_mut_ptr(),
                b"password\x00" as *const u8 as *const i8,
            );
            if *password as i32 != 0
                && crate::stdlib::strcmp(
                    password,
                    (*crate::src::server::sv_main::sv_privatePassword).string,
                ) == 0
            {
                startIndex = 0
            } else {
                // skip past the reserved slots
                startIndex = (*crate::src::server::sv_main::sv_privateClients).integer
            }
            newcl = 0 as *mut crate::server_h::client_t;
            i = startIndex;
            while i < (*crate::src::server::sv_main::sv_maxclients).integer {
                cl = &mut *crate::src::server::sv_main::svs.clients.offset(i as isize)
                    as *mut crate::server_h::client_t;
                if (*cl).state == crate::server_h::CS_FREE {
                    newcl = cl;
                    break;
                } else {
                    i += 1
                }
            }
            if newcl.is_null() {
                if crate::src::qcommon::net_ip::NET_IsLocalAddress(from) as u64 != 0 {
                    count = 0;
                    i = startIndex;
                    while i < (*crate::src::server::sv_main::sv_maxclients).integer {
                        cl = &mut *crate::src::server::sv_main::svs.clients.offset(i as isize)
                            as *mut crate::server_h::client_t;
                        if (*cl).netchan.remoteAddress.type_0 == crate::qcommon_h::NA_BOT {
                            count += 1
                        }
                        i += 1
                    }
                    // if they're all bots
                    if count >= (*crate::src::server::sv_main::sv_maxclients).integer - startIndex {
                        SV_DropClient(
                            &mut *crate::src::server::sv_main::svs.clients.offset(
                                ((*crate::src::server::sv_main::sv_maxclients).integer - 1)
                                    as isize,
                            ),
                            b"only bots on server\x00" as *const u8 as *const i8,
                        );
                        newcl = &mut *crate::src::server::sv_main::svs.clients.offset(
                            ((*crate::src::server::sv_main::sv_maxclients).integer - 1) as isize,
                        ) as *mut crate::server_h::client_t
                    } else {
                        crate::src::qcommon::common::Com_Error(
                            crate::src::qcommon::q_shared::ERR_FATAL as i32,
                            b"server is full on local connect\x00" as *const u8 as *const i8,
                        );
                    }
                } else {
                    crate::src::qcommon::net_chan::NET_OutOfBandPrint(
                        crate::qcommon_h::NS_SERVER,
                        from,
                        b"print\nServer is full.\n\x00" as *const u8 as *const i8,
                    );
                    crate::src::qcommon::common::Com_DPrintf(
                        b"Rejected a connection.\n\x00" as *const u8 as *const i8,
                    );
                    return;
                }
            }
            // we got a newcl, so reset the reliableSequence and reliableAcknowledge
            (*cl).reliableAcknowledge = 0;
            (*cl).reliableSequence = 0
        }
        _ => {}
    }
    // this doesn't work because it nukes the players userinfo
    //			// disconnect the client from the game first so any flags the
    //			// player might have are dropped
    //			VM_Call( gvm, GAME_CLIENT_DISCONNECT, newcl - svs.clients );
    //
    // build a new connection
    // accept the new client
    // this is the only place a client_t is ever initialized
    *newcl = temp;
    clientNum = newcl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32;
    ent = crate::src::server::sv_game::SV_GentityNum(clientNum);
    (*newcl).gentity = ent;
    // save the challenge
    (*newcl).challenge = challenge;
    // save the address
    (*newcl).compat = compat;
    crate::src::qcommon::net_chan::Netchan_Setup(
        crate::qcommon_h::NS_SERVER,
        &mut (*newcl).netchan,
        from,
        qport,
        challenge,
        compat,
    );
    // init the netchan queue
    (*newcl).netchan_end_queue = &mut (*newcl).netchan_start_queue;
    // save the userinfo
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*newcl).userinfo.as_mut_ptr(),
        userinfo.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    // get the game a chance to reject this connection or modify the userinfo
    denied = crate::src::qcommon::vm::VM_Call(
        crate::src::server::sv_main::gvm,
        crate::g_public_h::GAME_CLIENT_CONNECT as i32,
        clientNum,
        crate::src::qcommon::q_shared::qtrue as i32,
        crate::src::qcommon::q_shared::qfalse as i32,
    ); // firstTime = qtrue
    if denied != 0 {
        // we can't just use VM_ArgPtr, because that is only valid inside a VM_Call
        let mut str: *mut i8 =
            crate::src::qcommon::vm::VM_ExplicitArgPtr(crate::src::server::sv_main::gvm, denied)
                as *mut i8;
        crate::src::qcommon::net_chan::NET_OutOfBandPrint(
            crate::qcommon_h::NS_SERVER,
            from,
            b"print\n%s\n\x00" as *const u8 as *const i8,
            str,
        );
        crate::src::qcommon::common::Com_DPrintf(
            b"Game rejected a connection: %s.\n\x00" as *const u8 as *const i8,
            str,
        );
        return;
    }
    SV_UserinfoChanged(newcl);
    // send the connect packet to the client
    crate::src::qcommon::net_chan::NET_OutOfBandPrint(
        crate::qcommon_h::NS_SERVER,
        from,
        b"connectResponse %d\x00" as *const u8 as *const i8,
        challenge,
    );
    crate::src::qcommon::common::Com_DPrintf(
        b"Going from CS_FREE to CS_CONNECTED for %s\n\x00" as *const u8 as *const i8,
        (*newcl).name.as_mut_ptr(),
    );
    (*newcl).state = crate::server_h::CS_CONNECTED;
    (*newcl).lastSnapshotTime = 0;
    (*newcl).lastPacketTime = crate::src::server::sv_main::svs.time;
    (*newcl).lastConnectTime = crate::src::server::sv_main::svs.time;
    // when we receive the first packet from the client, we will
    // notice that it is from a different serverid and that the
    // gamestate message was not just sent, forcing a retransmit
    (*newcl).gamestateMessageNum = -(1);
    // if this was the first client on the server, or the last client
    // the server can hold, send a heartbeat to the master.
    count = 0;
    i = 0;
    cl = crate::src::server::sv_main::svs.clients;
    while i < (*crate::src::server::sv_main::sv_maxclients).integer {
        if (*crate::src::server::sv_main::svs.clients.offset(i as isize)).state
            >= crate::server_h::CS_CONNECTED
        {
            count += 1
        }
        i += 1;
        cl = cl.offset(1)
    }
    if count == 1 || count == (*crate::src::server::sv_main::sv_maxclients).integer {
        crate::src::server::sv_ccmds::SV_Heartbeat_f();
    };
}
/*
=====================
SV_FreeClient

Destructor for data allocated in a client structure
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_FreeClient(mut client: *mut crate::server_h::client_t) {
    let mut index: i32 = 0;
    index = (*client).queuedVoipIndex;
    while index < (*client).queuedVoipPackets {
        index = (index as usize).wrapping_rem(
            (::std::mem::size_of::<[*mut crate::server_h::voipServerPacket_t; 64]>()).wrapping_div(
                ::std::mem::size_of::<*mut crate::server_h::voipServerPacket_t>(),
            ),
        ) as i32;
        crate::src::qcommon::common::Z_Free(
            (*client).voipPacket[index as usize] as *mut libc::c_void,
        );
        index += 1
    }
    (*client).queuedVoipPackets = 0;
    crate::src::server::sv_net_chan::SV_Netchan_FreeQueue(client);
    SV_CloseDownload(client);
}
/*
=====================
SV_DropClient

Called when the player is totally leaving the server, either willingly
or unwillingly.  This is NOT called if the entire server is quiting
or crashing -- SV_FinalMessage() will handle that
=====================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_DropClient(
    mut drop_0: *mut crate::server_h::client_t,
    mut reason: *const i8,
) {
    let mut i: i32 = 0;
    let mut challenge: *mut crate::server_h::challenge_t = 0 as *mut crate::server_h::challenge_t;
    let isBot: crate::src::qcommon::q_shared::qboolean = ((*drop_0).netchan.remoteAddress.type_0
        == crate::qcommon_h::NA_BOT)
        as crate::src::qcommon::q_shared::qboolean;
    if (*drop_0).state == crate::server_h::CS_ZOMBIE {
        return;
        // already dropped
    }
    if isBot as u64 == 0 {
        // see if we already have a challenge for this ip
        challenge = &mut *crate::src::server::sv_main::svs
            .challenges
            .as_mut_ptr()
            .offset(0) as *mut crate::server_h::challenge_t;
        i = 0;
        while i < 2048 {
            if crate::src::qcommon::net_ip::NET_CompareAdr(
                (*drop_0).netchan.remoteAddress,
                (*challenge).adr,
            ) as u64
                != 0
            {
                crate::stdlib::memset(
                    challenge as *mut libc::c_void,
                    0,
                    ::std::mem::size_of::<crate::server_h::challenge_t>(),
                );
                break;
            } else {
                i += 1;
                challenge = challenge.offset(1)
            }
        }
    }
    // Free all allocated data on the client structure
    SV_FreeClient(drop_0);
    // tell everyone why they got dropped
    crate::src::server::sv_main::SV_SendServerCommand(
        0 as *mut crate::server_h::client_t,
        b"print \"%s^7 %s\n\"\x00" as *const u8 as *const i8,
        (*drop_0).name.as_mut_ptr(),
        reason,
    );
    // call the prog function for removing a client
    // this will remove the body, among other things
    crate::src::qcommon::vm::VM_Call(
        crate::src::server::sv_main::gvm,
        crate::g_public_h::GAME_CLIENT_DISCONNECT as i32,
        drop_0.wrapping_offset_from(crate::src::server::sv_main::svs.clients),
    );
    // add the disconnect command
    crate::src::server::sv_main::SV_SendServerCommand(
        drop_0,
        b"disconnect \"%s\"\x00" as *const u8 as *const i8,
        reason,
    );
    if isBot as u64 != 0 {
        crate::src::server::sv_bot::SV_BotFreeClient(
            drop_0.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
        );
        // bots shouldn't go zombie, as there's no real net connection.
        (*drop_0).state = crate::server_h::CS_FREE
    } else {
        crate::src::qcommon::common::Com_DPrintf(
            b"Going to CS_ZOMBIE for %s\n\x00" as *const u8 as *const i8,
            (*drop_0).name.as_mut_ptr(),
        );
        (*drop_0).state = crate::server_h::CS_ZOMBIE
        // become free in a few seconds
    }
    // nuke user info
    crate::src::server::sv_init::SV_SetUserinfo(
        drop_0.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
        b"\x00" as *const u8 as *const i8,
    );
    // if this was the last client on the server, send a heartbeat
    // to the master so it is known the server is empty
    // send a heartbeat now so the master will get up to date info
    // if there is already a slot for this ip, reuse it
    i = 0;
    while i < (*crate::src::server::sv_main::sv_maxclients).integer {
        if (*crate::src::server::sv_main::svs.clients.offset(i as isize)).state
            >= crate::server_h::CS_CONNECTED
        {
            break;
        }
        i += 1
    }
    if i == (*crate::src::server::sv_main::sv_maxclients).integer {
        crate::src::server::sv_ccmds::SV_Heartbeat_f();
    };
}
/*
================
SV_SendClientGameState

Sends the first message from the server to a connected client.
This will be sent on the initial connection and upon each new map load.

It will be resent if the client acknowledges a later message but has
the wrong gamestate.
================
*/

unsafe extern "C" fn SV_SendClientGameState(mut client: *mut crate::server_h::client_t) {
    let mut start: i32 = 0;
    let mut base: *mut crate::src::qcommon::q_shared::entityState_t =
        0 as *mut crate::src::qcommon::q_shared::entityState_t;
    let mut nullstate: crate::src::qcommon::q_shared::entityState_t =
        crate::src::qcommon::q_shared::entityState_t {
            number: 0,
            eType: 0,
            eFlags: 0,
            pos: crate::src::qcommon::q_shared::trajectory_t {
                trType: crate::src::qcommon::q_shared::TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            apos: crate::src::qcommon::q_shared::trajectory_t {
                trType: crate::src::qcommon::q_shared::TR_STATIONARY,
                trTime: 0,
                trDuration: 0,
                trBase: [0.; 3],
                trDelta: [0.; 3],
            },
            time: 0,
            time2: 0,
            origin: [0.; 3],
            origin2: [0.; 3],
            angles: [0.; 3],
            angles2: [0.; 3],
            otherEntityNum: 0,
            otherEntityNum2: 0,
            groundEntityNum: 0,
            constantLight: 0,
            loopSound: 0,
            modelindex: 0,
            modelindex2: 0,
            clientNum: 0,
            frame: 0,
            solid: 0,
            event: 0,
            eventParm: 0,
            powerups: 0,
            weapon: 0,
            legsAnim: 0,
            torsoAnim: 0,
            generic1: 0,
        };
    let mut msg: crate::qcommon_h::msg_t = crate::qcommon_h::msg_t {
        allowoverflow: crate::src::qcommon::q_shared::qfalse,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        oob: crate::src::qcommon::q_shared::qfalse,
        data: 0 as *mut crate::src::qcommon::q_shared::byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    let mut msgBuffer: [crate::src::qcommon::q_shared::byte; 16384] = [0; 16384];
    crate::src::qcommon::common::Com_DPrintf(
        b"SV_SendClientGameState() for %s\n\x00" as *const u8 as *const i8,
        (*client).name.as_mut_ptr(),
    );
    crate::src::qcommon::common::Com_DPrintf(
        b"Going from CS_CONNECTED to CS_PRIMED for %s\n\x00" as *const u8 as *const i8,
        (*client).name.as_mut_ptr(),
    );
    (*client).state = crate::server_h::CS_PRIMED;
    (*client).pureAuthentic = 0;
    (*client).gotCP = crate::src::qcommon::q_shared::qfalse;
    // when we receive the first packet from the client, we will
    // notice that it is from a different serverid and that the
    // gamestate message was not just sent, forcing a retransmit
    (*client).gamestateMessageNum = (*client).netchan.outgoingSequence;
    crate::src::qcommon::msg::MSG_Init(
        &mut msg,
        msgBuffer.as_mut_ptr(),
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16384]>() as i32,
    );
    // NOTE, MRE: all server->client messages now acknowledge
    // let the client know which reliable clientCommands we have received
    crate::src::qcommon::msg::MSG_WriteLong(&mut msg, (*client).lastClientCommand);
    // send any server commands waiting to be sent first.
    // we have to do this cause we send the client->reliableSequence
    // with a gamestate and it sets the clc.serverCommandSequence at
    // the client side
    crate::src::server::sv_snapshot::SV_UpdateServerCommandsToClient(client, &mut msg);
    // send the gamestate
    crate::src::qcommon::msg::MSG_WriteByte(&mut msg, crate::qcommon_h::svc_gamestate as i32);
    crate::src::qcommon::msg::MSG_WriteLong(&mut msg, (*client).reliableSequence);
    // write the configstrings
    start = 0;
    while start < 1024 {
        if *crate::src::server::sv_main::sv.configstrings[start as usize].offset(0) != 0 {
            crate::src::qcommon::msg::MSG_WriteByte(
                &mut msg,
                crate::qcommon_h::svc_configstring as i32,
            );
            crate::src::qcommon::msg::MSG_WriteShort(&mut msg, start);
            crate::src::qcommon::msg::MSG_WriteBigString(
                &mut msg,
                crate::src::server::sv_main::sv.configstrings[start as usize],
            );
        }
        start += 1
    }
    // write the baselines
    crate::stdlib::memset(
        &mut nullstate as *mut crate::src::qcommon::q_shared::entityState_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::entityState_t>(),
    );
    start = 0;
    while start < (1) << 10 {
        base = &mut (*crate::src::server::sv_main::sv
            .svEntities
            .as_mut_ptr()
            .offset(start as isize))
        .baseline;
        if !((*base).number == 0) {
            crate::src::qcommon::msg::MSG_WriteByte(
                &mut msg,
                crate::qcommon_h::svc_baseline as i32,
            );
            crate::src::qcommon::msg::MSG_WriteDeltaEntity(
                &mut msg,
                &mut nullstate,
                base,
                crate::src::qcommon::q_shared::qtrue,
            );
        }
        start += 1
    }
    crate::src::qcommon::msg::MSG_WriteByte(&mut msg, crate::qcommon_h::svc_EOF as i32);
    crate::src::qcommon::msg::MSG_WriteLong(
        &mut msg,
        client.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
    );
    // write the checksum feed
    crate::src::qcommon::msg::MSG_WriteLong(&mut msg, crate::src::server::sv_main::sv.checksumFeed);
    // deliver this to the client
    crate::src::server::sv_snapshot::SV_SendMessageToClient(&mut msg, client);
}
/*
==================
SV_ClientEnterWorld
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_ClientEnterWorld(
    mut client: *mut crate::server_h::client_t,
    mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    let mut clientNum: i32 = 0;
    let mut ent: *mut crate::g_public_h::sharedEntity_t =
        0 as *mut crate::g_public_h::sharedEntity_t;
    crate::src::qcommon::common::Com_DPrintf(
        b"Going from CS_PRIMED to CS_ACTIVE for %s\n\x00" as *const u8 as *const i8,
        (*client).name.as_mut_ptr(),
    );
    (*client).state = crate::server_h::CS_ACTIVE;
    // resend all configstrings using the cs commands since these are
    // no longer sent when the client is CS_PRIMED
    crate::src::server::sv_init::SV_UpdateConfigstrings(client);
    // set up the entity for the client
    clientNum = client.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32; // generate a snapshot immediately
    ent = crate::src::server::sv_game::SV_GentityNum(clientNum);
    (*ent).s.number = clientNum;
    (*client).gentity = ent;
    (*client).deltaMessage = -(1);
    (*client).lastSnapshotTime = 0;
    if !cmd.is_null() {
        crate::stdlib::memcpy(
            &mut (*client).lastUsercmd as *mut crate::src::qcommon::q_shared::usercmd_t
                as *mut libc::c_void,
            cmd as *const libc::c_void,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::usercmd_t>(),
        );
    } else {
        crate::stdlib::memset(
            &mut (*client).lastUsercmd as *mut crate::src::qcommon::q_shared::usercmd_t
                as *mut libc::c_void,
            '\u{0}' as i32,
            ::std::mem::size_of::<crate::src::qcommon::q_shared::usercmd_t>(),
        );
    }
    // call the game begin function
    crate::src::qcommon::vm::VM_Call(
        crate::src::server::sv_main::gvm,
        crate::g_public_h::GAME_CLIENT_BEGIN as i32,
        client.wrapping_offset_from(crate::src::server::sv_main::svs.clients),
    );
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
// sv_client.c -- server code for dealing with clients
/*
============================================================

CLIENT COMMAND EXECUTION

============================================================
*/
/*
==================
SV_CloseDownload

clear/free any download vars
==================
*/

unsafe extern "C" fn SV_CloseDownload(mut cl: *mut crate::server_h::client_t) {
    let mut i: i32 = 0;
    // EOF
    if (*cl).download != 0 {
        crate::src::qcommon::files::FS_FCloseFile((*cl).download);
    }
    (*cl).download = 0;
    *(*cl).downloadName.as_mut_ptr() = 0i8;
    // Free the temporary buffer space
    i = 0;
    while i < 48 {
        if !(*cl).downloadBlocks[i as usize].is_null() {
            crate::src::qcommon::common::Z_Free(
                (*cl).downloadBlocks[i as usize] as *mut libc::c_void,
            );
            (*cl).downloadBlocks[i as usize] = 0 as *mut u8
        }
        i += 1
    }
}
/*
==================
SV_StopDownload_f

Abort a download if in progress
==================
*/

unsafe extern "C" fn SV_StopDownload_f(mut cl: *mut crate::server_h::client_t) {
    if *(*cl).downloadName.as_mut_ptr() != 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"clientDownload: %d : file \"%s\" aborted\n\x00" as *const u8 as *const i8,
            cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
            (*cl).downloadName.as_mut_ptr(),
        );
    }
    SV_CloseDownload(cl);
}
/*
==================
SV_DoneDownload_f

Downloads are finished
==================
*/

unsafe extern "C" fn SV_DoneDownload_f(mut cl: *mut crate::server_h::client_t) {
    if (*cl).state == crate::server_h::CS_ACTIVE {
        return;
    }
    crate::src::qcommon::common::Com_DPrintf(
        b"clientDownload: %s Done\n\x00" as *const u8 as *const i8,
        (*cl).name.as_mut_ptr(),
    );
    // resend the game state to update any clients that entered during the download
    SV_SendClientGameState(cl);
}
/*
==================
SV_NextDownload_f

The argument will be the last acknowledged block from the client, it should be
the same as cl->downloadClientBlock
==================
*/

unsafe extern "C" fn SV_NextDownload_f(mut cl: *mut crate::server_h::client_t) {
    let mut block: i32 = atoi(crate::src::qcommon::cmd::Cmd_Argv(1));
    if block == (*cl).downloadClientBlock {
        crate::src::qcommon::common::Com_DPrintf(
            b"clientDownload: %d : client acknowledge of block %d\n\x00" as *const u8 as *const i8,
            cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
            block,
        );
        // Find out if we are done.  A zero-length block indicates EOF
        if (*cl).downloadBlockSize[((*cl).downloadClientBlock % 48i32) as usize] == 0 {
            crate::src::qcommon::common::Com_Printf(
                b"clientDownload: %d : file \"%s\" completed\n\x00" as *const u8 as *const i8,
                cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
                (*cl).downloadName.as_mut_ptr(),
            );
            SV_CloseDownload(cl);
            return;
        }
        (*cl).downloadSendTime = crate::src::server::sv_main::svs.time;
        (*cl).downloadClientBlock += 1;
        return;
    }
    // We aren't getting an acknowledge for the correct block, drop the client
    // FIXME: this is bad... the client will never parse the disconnect message
    //			because the cgame isn't loaded yet
    SV_DropClient(cl, b"broken download\x00" as *const u8 as *const i8);
}
/*
==================
SV_BeginDownload_f
==================
*/

unsafe extern "C" fn SV_BeginDownload_f(mut cl: *mut crate::server_h::client_t) {
    // Kill any existing download
    SV_CloseDownload(cl);
    // cl->downloadName is non-zero now, SV_WriteDownloadToClient will see this and open
    // the file itself
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*cl).downloadName.as_mut_ptr(),
        crate::src::qcommon::cmd::Cmd_Argv(1),
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
}
/*
==================
SV_WriteDownloadToClient

Check to see if the client wants a file, open it if needed and start pumping the client
Fill up msg with data, return number of download blocks added
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_WriteDownloadToClient(
    mut cl: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) -> i32 {
    let mut curindex: i32 = 0; // Nothing being downloaded
    let mut unreferenced: i32 = 1;
    let mut errorMessage: [i8; 1024] = [0; 1024];
    let mut pakbuf: [i8; 64] = [0; 64];
    let mut pakptr: *mut i8 = 0 as *mut i8;
    let mut numRefPaks: i32 = 0;
    if *(*cl).downloadName.as_mut_ptr() == 0 {
        return 0i32;
    }
    if (*cl).download == 0 {
        let mut idPack: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        let mut missionPack: crate::src::qcommon::q_shared::qboolean =
            crate::src::qcommon::q_shared::qfalse;
        // Chop off filename extension.
        crate::src::qcommon::q_shared::Com_sprintf(
            pakbuf.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
            b"%s\x00" as *const u8 as *const i8,
            (*cl).downloadName.as_mut_ptr(),
        );
        pakptr = crate::stdlib::strrchr(pakbuf.as_mut_ptr(), '.' as i32);
        if !pakptr.is_null() {
            *pakptr = '\u{0}' as i8;
            // Check for pk3 filename extension
            if crate::src::qcommon::q_shared::Q_stricmp(
                pakptr.offset(1),
                b"pk3\x00" as *const u8 as *const i8,
            ) == 0
            {
                let mut referencedPaks: *const i8 =
                    crate::src::qcommon::files::FS_ReferencedPakNames();
                // Check whether the file appears in the list of referenced
                // paks to prevent downloading of arbitrary files.
                crate::src::qcommon::cmd::Cmd_TokenizeStringIgnoreQuotes(referencedPaks);
                numRefPaks = crate::src::qcommon::cmd::Cmd_Argc();
                curindex = 0;
                while curindex < numRefPaks {
                    if crate::src::qcommon::files::FS_FilenameCompare(
                        crate::src::qcommon::cmd::Cmd_Argv(curindex),
                        pakbuf.as_mut_ptr(),
                    ) as u64
                        == 0
                    {
                        unreferenced = 0;
                        // now that we know the file is referenced,
                        // check whether it's legal to download it.
                        missionPack = crate::src::qcommon::files::FS_idPak(
                            pakbuf.as_mut_ptr(),
                            b"missionpack\x00" as *const u8 as *mut i8,
                            4,
                        );
                        idPack = missionPack;
                        idPack = (idPack != 0
                            || crate::src::qcommon::files::FS_idPak(
                                pakbuf.as_mut_ptr(),
                                b"baseq3\x00" as *const u8 as *mut i8,
                                9,
                            ) != 0)
                            as crate::src::qcommon::q_shared::qboolean;
                        break;
                    } else {
                        curindex += 1
                    }
                }
            }
        }
        (*cl).download = 0;
        // We open the file here
        if (*crate::src::server::sv_main::sv_allowDownload).integer & 1 == 0
            || (*crate::src::server::sv_main::sv_allowDownload).integer & 4 != 0
            || idPack != 0
            || unreferenced != 0
            || {
                (*cl).downloadSize = crate::src::qcommon::files::FS_SV_FOpenFileRead(
                    (*cl).downloadName.as_mut_ptr(),
                    &mut (*cl).download,
                ) as i32;
                ((*cl).downloadSize) < 0
            }
        {
            // cannot auto-download file
            if unreferenced != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"clientDownload: %d : \"%s\" is not referenced and cannot be downloaded.\n\x00"
                        as *const u8 as *const i8,
                    cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
                    (*cl).downloadName.as_mut_ptr(),
                );
                crate::src::qcommon::q_shared::Com_sprintf(
                    errorMessage.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 1024]>() as i32,
                    b"File \"%s\" is not referenced and cannot be downloaded.\x00" as *const u8
                        as *const i8,
                    (*cl).downloadName.as_mut_ptr(),
                );
            } else if idPack as u64 != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"clientDownload: %d : \"%s\" cannot download id pk3 files\n\x00" as *const u8
                        as *const i8,
                    cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
                    (*cl).downloadName.as_mut_ptr(),
                );
                if missionPack as u64 != 0 {
                    crate::src::qcommon::q_shared::Com_sprintf(errorMessage.as_mut_ptr(),
                                
                                ::std::mem::size_of::<[i8; 1024]>() as i32,
                                b"Cannot autodownload Team Arena file \"%s\"\nThe Team Arena mission pack can be found in your local game store.\x00"
                                    as *const u8 as *const i8,
                                (*cl).downloadName.as_mut_ptr());
                } else {
                    crate::src::qcommon::q_shared::Com_sprintf(
                        errorMessage.as_mut_ptr(),
                        ::std::mem::size_of::<[i8; 1024]>() as i32,
                        b"Cannot autodownload id pk3 file \"%s\"\x00" as *const u8 as *const i8,
                        (*cl).downloadName.as_mut_ptr(),
                    );
                }
            } else if (*crate::src::server::sv_main::sv_allowDownload).integer & 1 == 0
                || (*crate::src::server::sv_main::sv_allowDownload).integer & 4 != 0
            {
                crate::src::qcommon::common::Com_Printf(
                    b"clientDownload: %d : \"%s\" download disabled\n\x00" as *const u8
                        as *const i8,
                    cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
                    (*cl).downloadName.as_mut_ptr(),
                );
                if (*crate::src::server::sv_main::sv_pure).integer != 0 {
                    crate::src::qcommon::q_shared::Com_sprintf(errorMessage.as_mut_ptr(),
                                
                                ::std::mem::size_of::<[i8; 1024]>() as i32,
                                b"Could not download \"%s\" because autodownloading is disabled on the server.\n\nYou will need to get this file elsewhere before you can connect to this pure server.\n\x00"
                                    as *const u8 as *const i8,
                                (*cl).downloadName.as_mut_ptr());
                } else {
                    crate::src::qcommon::q_shared::Com_sprintf(errorMessage.as_mut_ptr(),
                                
                                ::std::mem::size_of::<[i8; 1024]>() as i32,
                                b"Could not download \"%s\" because autodownloading is disabled on the server.\n\nThe server you are connecting to is not a pure server, set autodownload to No in your settings and you might be able to join the game anyway.\n\x00"
                                    as *const u8 as *const i8,
                                (*cl).downloadName.as_mut_ptr());
                }
            } else {
                // NOTE TTimo this is NOT supposed to happen unless bug in our filesystem scheme?
                //   if the pk3 is referenced, it must have been found somewhere in the filesystem
                crate::src::qcommon::common::Com_Printf(
                    b"clientDownload: %d : \"%s\" file not found on server\n\x00" as *const u8
                        as *const i8,
                    cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
                    (*cl).downloadName.as_mut_ptr(),
                ); // client is expecting block zero
                crate::src::qcommon::q_shared::Com_sprintf(
                    errorMessage.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 1024]>() as i32,
                    b"File \"%s\" not found on server for autodownloading.\n\x00" as *const u8
                        as *const i8,
                    (*cl).downloadName.as_mut_ptr(),
                ); // illegal file size
            }
            crate::src::qcommon::msg::MSG_WriteByte(msg, crate::qcommon_h::svc_download as i32);
            crate::src::qcommon::msg::MSG_WriteShort(msg, 0);
            crate::src::qcommon::msg::MSG_WriteLong(msg, -(1));
            crate::src::qcommon::msg::MSG_WriteString(msg, errorMessage.as_mut_ptr());
            *(*cl).downloadName.as_mut_ptr() = 0i8;
            if (*cl).download != 0 {
                crate::src::qcommon::files::FS_FCloseFile((*cl).download);
            }
            return 1i32;
        }
        crate::src::qcommon::common::Com_Printf(
            b"clientDownload: %d : beginning \"%s\"\n\x00" as *const u8 as *const i8,
            cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
            (*cl).downloadName.as_mut_ptr(),
        );
        // Init
        (*cl).downloadXmitBlock = 0;
        (*cl).downloadClientBlock = (*cl).downloadXmitBlock;
        (*cl).downloadCurrentBlock = (*cl).downloadClientBlock;
        (*cl).downloadCount = 0;
        (*cl).downloadEOF = crate::src::qcommon::q_shared::qfalse
    }
    // Perform any reads that we need to
    while (*cl).downloadCurrentBlock - (*cl).downloadClientBlock < 48
        && (*cl).downloadSize != (*cl).downloadCount
    {
        curindex = (*cl).downloadCurrentBlock % 48;
        if (*cl).downloadBlocks[curindex as usize].is_null() {
            (*cl).downloadBlocks[curindex as usize] =
                crate::src::qcommon::common::Z_Malloc(1024) as *mut u8
        }
        (*cl).downloadBlockSize[curindex as usize] = crate::src::qcommon::files::FS_Read(
            (*cl).downloadBlocks[curindex as usize] as *mut libc::c_void,
            1024,
            (*cl).download,
        );
        if (*cl).downloadBlockSize[curindex as usize] < 0 {
            // EOF right now
            (*cl).downloadCount = (*cl).downloadSize;
            break;
        } else {
            (*cl).downloadCount += (*cl).downloadBlockSize[curindex as usize];
            // Load in next block
            (*cl).downloadCurrentBlock += 1
        }
    }
    // Check to see if we have eof condition and add the EOF block
    if (*cl).downloadCount == (*cl).downloadSize
        && (*cl).downloadEOF as u64 == 0
        && (*cl).downloadCurrentBlock - (*cl).downloadClientBlock < 48
    {
        (*cl).downloadBlockSize[((*cl).downloadCurrentBlock % 48i32) as usize] = 0;
        (*cl).downloadCurrentBlock += 1;
        (*cl).downloadEOF = crate::src::qcommon::q_shared::qtrue
        // We have added the EOF block
    } // Nothing to transmit
    if (*cl).downloadClientBlock == (*cl).downloadCurrentBlock {
        return 0i32;
    }
    // Write out the next section of the file, if we have already reached our window,
    // automatically start retransmitting
    if (*cl).downloadXmitBlock == (*cl).downloadCurrentBlock {
        // We have transmitted the complete window, should we start resending?
        if crate::src::server::sv_main::svs.time - (*cl).downloadSendTime > 1000 {
            (*cl).downloadXmitBlock = (*cl).downloadClientBlock
        } else {
            return 0i32;
        }
    }
    // Send current block
    curindex = (*cl).downloadXmitBlock % 48;
    crate::src::qcommon::msg::MSG_WriteByte(msg, crate::qcommon_h::svc_download as i32);
    crate::src::qcommon::msg::MSG_WriteShort(msg, (*cl).downloadXmitBlock);
    // block zero is special, contains file size
    if (*cl).downloadXmitBlock == 0 {
        crate::src::qcommon::msg::MSG_WriteLong(msg, (*cl).downloadSize);
    }
    crate::src::qcommon::msg::MSG_WriteShort(msg, (*cl).downloadBlockSize[curindex as usize]);
    // Write the block
    if (*cl).downloadBlockSize[curindex as usize] != 0 {
        crate::src::qcommon::msg::MSG_WriteData(
            msg,
            (*cl).downloadBlocks[curindex as usize] as *const libc::c_void,
            (*cl).downloadBlockSize[curindex as usize],
        );
    }
    crate::src::qcommon::common::Com_DPrintf(
        b"clientDownload: %d : writing block %d\n\x00" as *const u8 as *const i8,
        cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
        (*cl).downloadXmitBlock,
    );
    // Move on to the next block
    // It will get sent with next snap shot.  The rate will keep us in line.
    (*cl).downloadXmitBlock += 1;
    (*cl).downloadSendTime = crate::src::server::sv_main::svs.time;
    return 1;
}
/*
==================
SV_SendQueuedMessages

Send one round of fragments, or queued messages to all clients that have data pending.
Return the shortest time interval for sending next packet to client
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SendQueuedMessages() -> i32 {
    let mut _i: i32 = 0;
    let mut retval: i32 = -(1);
    let mut nextFragT: i32 = 0;
    let mut cl: *mut crate::server_h::client_t = 0 as *mut crate::server_h::client_t;

    for i in 0..(*crate::src::server::sv_main::sv_maxclients).integer {
        cl = &mut *crate::src::server::sv_main::svs.clients.offset(i as isize)
            as *mut crate::server_h::client_t;

        if (*cl).state as u64 != 0 {
            nextFragT = crate::src::server::sv_main::SV_RateMsec(cl);
            if nextFragT == 0 {
                nextFragT = crate::src::server::sv_net_chan::SV_Netchan_TransmitNextFragment(cl)
            }
            if nextFragT >= 0 && (retval == -(1) || retval > nextFragT) {
                retval = nextFragT
            }
        }
    }
    return retval;
}
/*
==================
SV_SendDownloadMessages

Send one round of download messages to all clients
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_SendDownloadMessages() -> i32 {
    let mut _i: i32 = 0;
    let mut numDLs: i32 = 0;
    let mut retval: i32 = 0;
    let mut cl: *mut crate::server_h::client_t = 0 as *mut crate::server_h::client_t;
    let mut msg: crate::qcommon_h::msg_t = crate::qcommon_h::msg_t {
        allowoverflow: crate::src::qcommon::q_shared::qfalse,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        oob: crate::src::qcommon::q_shared::qfalse,
        data: 0 as *mut crate::src::qcommon::q_shared::byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    let mut msgBuffer: [crate::src::qcommon::q_shared::byte; 16384] = [0; 16384];

    for i in 0..(*crate::src::server::sv_main::sv_maxclients).integer {
        cl = &mut *crate::src::server::sv_main::svs.clients.offset(i as isize)
            as *mut crate::server_h::client_t;

        if (*cl).state != 0 && *(*cl).downloadName.as_mut_ptr() as i32 != 0 {
            crate::src::qcommon::msg::MSG_Init(
                &mut msg,
                msgBuffer.as_mut_ptr(),
                ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16384]>() as i32,
            );
            crate::src::qcommon::msg::MSG_WriteLong(&mut msg, (*cl).lastClientCommand);
            retval = SV_WriteDownloadToClient(cl, &mut msg);
            if retval != 0 {
                crate::src::qcommon::msg::MSG_WriteByte(&mut msg, crate::qcommon_h::svc_EOF as i32);
                crate::src::server::sv_net_chan::SV_Netchan_Transmit(cl, &mut msg);
                numDLs += retval
            }
        }
    }
    return numDLs;
}
/*
=================
SV_Disconnect_f

The client is going to disconnect, so remove the connection immediately  FIXME: move to game?
=================
*/

unsafe extern "C" fn SV_Disconnect_f(mut cl: *mut crate::server_h::client_t) {
    SV_DropClient(cl, b"disconnected\x00" as *const u8 as *const i8);
}
/*
=================
SV_VerifyPaks_f

If we are pure, disconnect the client if they do no meet the following conditions:

1. the first two checksums match our view of cgame and ui
2. there are no any additional checksums that we do not have

This routine would be a bit simpler with a goto but i abstained

=================
*/

unsafe extern "C" fn SV_VerifyPaks_f(mut cl: *mut crate::server_h::client_t) {
    let mut nChkSum1: i32 = 0;
    let mut nChkSum2: i32 = 0;
    let mut nClientPaks: i32 = 0;
    let mut nServerPaks: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut nCurArg: i32 = 0;
    let mut nClientChkSum: [i32; 1024] = [0; 1024];
    let mut nServerChkSum: [i32; 1024] = [0; 1024];
    let mut pPaks: *const i8 = 0 as *const i8;
    let mut pArg: *const i8 = 0 as *const i8;
    let mut bGood: crate::src::qcommon::q_shared::qboolean = crate::src::qcommon::q_shared::qtrue;
    // if we are pure, we "expect" the client to load certain things from
    // certain pk3 files, namely we want the client to have loaded the
    // ui and cgame that we think should be loaded based on the pure setting
    //
    if (*crate::src::server::sv_main::sv_pure).integer != 0 {
        nChkSum2 = 0;
        nChkSum1 = nChkSum2;
        // we run the game, so determine which cgame and ui the client "should" be running
        bGood = (crate::src::qcommon::files::FS_FileIsInPAK(
            b"vm/cgame.qvm\x00" as *const u8 as *const i8,
            &mut nChkSum1,
        ) == 1) as crate::src::qcommon::q_shared::qboolean;
        if bGood as u64 != 0 {
            bGood = (crate::src::qcommon::files::FS_FileIsInPAK(
                b"vm/ui.qvm\x00" as *const u8 as *const i8,
                &mut nChkSum2,
            ) == 1) as crate::src::qcommon::q_shared::qboolean
        }
        nClientPaks = crate::src::qcommon::cmd::Cmd_Argc();
        // start at arg 2 ( skip serverId cl_paks )
        nCurArg = 1;
        let fresh0 = nCurArg;
        nCurArg = nCurArg + 1;
        pArg = crate::src::qcommon::cmd::Cmd_Argv(fresh0);
        if pArg.is_null() {
            bGood = crate::src::qcommon::q_shared::qfalse
        } else if atoi(pArg) < crate::src::server::sv_main::sv.checksumFeedServerId {
            crate::src::qcommon::common::Com_DPrintf(
                b"ignoring outdated cp command from client %s\n\x00" as *const u8 as *const i8,
                (*cl).name.as_mut_ptr(),
            );
            return;
        }
        // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=475
        // we may get incoming cp sequences from a previous checksumFeed, which we need to ignore
        // since serverId is a frame count, it always goes up
        // we basically use this while loop to avoid using 'goto' :)
        if bGood as u64 != 0 {
            // must be at least 6: "cl_paks cgame ui @ firstref ... numChecksums"
            // numChecksums is encoded
            if nClientPaks < 6 {
                bGood = crate::src::qcommon::q_shared::qfalse
            } else {
                // verify first to be the cgame checksum
                let fresh1 = nCurArg;
                nCurArg = nCurArg + 1;
                pArg = crate::src::qcommon::cmd::Cmd_Argv(fresh1);
                if pArg.is_null() || *pArg as i32 == '@' as i32 || atoi(pArg) != nChkSum1 {
                    bGood = crate::src::qcommon::q_shared::qfalse
                } else {
                    // verify the second to be the ui checksum
                    let fresh2 = nCurArg;
                    nCurArg = nCurArg + 1;
                    pArg = crate::src::qcommon::cmd::Cmd_Argv(fresh2);
                    if pArg.is_null() || *pArg as i32 == '@' as i32 || atoi(pArg) != nChkSum2 {
                        bGood = crate::src::qcommon::q_shared::qfalse
                    } else {
                        // should be sitting at the delimeter now
                        let fresh3 = nCurArg;
                        nCurArg = nCurArg + 1;
                        pArg = crate::src::qcommon::cmd::Cmd_Argv(fresh3);
                        if *pArg as i32 != '@' as i32 {
                            bGood = crate::src::qcommon::q_shared::qfalse
                        } else {
                            // store checksums since tokenization is not re-entrant
                            i = 0;
                            while nCurArg < nClientPaks {
                                let fresh4 = nCurArg;
                                nCurArg = nCurArg + 1;
                                nClientChkSum[i as usize] =
                                    atoi(crate::src::qcommon::cmd::Cmd_Argv(fresh4));
                                i += 1
                            }
                            // store number to compare against (minus one cause the last is the number of checksums)
                            nClientPaks = i - 1;
                            // make sure none of the client check sums are the same
                            // so the client can't send 5 the same checksums
                            i = 0;
                            while i < nClientPaks {
                                j = 0;
                                while j < nClientPaks {
                                    if !(i == j) {
                                        if nClientChkSum[i as usize] == nClientChkSum[j as usize] {
                                            bGood = crate::src::qcommon::q_shared::qfalse;
                                            break;
                                        }
                                    }
                                    j += 1
                                }
                                if bGood == crate::src::qcommon::q_shared::qfalse {
                                    break;
                                }
                                i += 1
                            }
                            if !(bGood == crate::src::qcommon::q_shared::qfalse) {
                                // get the pure checksums of the pk3 files loaded by the server
                                pPaks = crate::src::qcommon::files::FS_LoadedPakPureChecksums();
                                crate::src::qcommon::cmd::Cmd_TokenizeString(pPaks);
                                nServerPaks = crate::src::qcommon::cmd::Cmd_Argc();
                                if nServerPaks > 1024 {
                                    nServerPaks = 1024
                                }
                                i = 0;
                                while i < nServerPaks {
                                    nServerChkSum[i as usize] =
                                        atoi(crate::src::qcommon::cmd::Cmd_Argv(i));
                                    i += 1
                                }
                                // check if the client has provided any pure checksums of pk3 files not loaded by the server
                                i = 0;
                                while i < nClientPaks {
                                    j = 0;
                                    while j < nServerPaks {
                                        if nClientChkSum[i as usize] == nServerChkSum[j as usize] {
                                            break;
                                        }
                                        j += 1
                                    }
                                    if j >= nServerPaks {
                                        bGood = crate::src::qcommon::q_shared::qfalse;
                                        break;
                                    } else {
                                        i += 1
                                    }
                                }
                                if !(bGood == crate::src::qcommon::q_shared::qfalse) {
                                    // check if the number of checksums was correct
                                    nChkSum1 = crate::src::server::sv_main::sv.checksumFeed;
                                    i = 0;
                                    while i < nClientPaks {
                                        nChkSum1 ^= nClientChkSum[i as usize];
                                        i += 1
                                    }
                                    nChkSum1 ^= nClientPaks;
                                    if nChkSum1 != nClientChkSum[nClientPaks as usize] {
                                        bGood = crate::src::qcommon::q_shared::qfalse
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        (*cl).gotCP = crate::src::qcommon::q_shared::qtrue;
        if bGood as u64 != 0 {
            (*cl).pureAuthentic = 1
        } else {
            (*cl).pureAuthentic = 0;
            (*cl).lastSnapshotTime = 0;
            (*cl).state = crate::server_h::CS_ACTIVE;
            crate::src::server::sv_snapshot::SV_SendClientSnapshot(cl);
            SV_DropClient(
                cl,
                b"Unpure client detected. Invalid .PK3 files referenced!\x00" as *const u8
                    as *const i8,
            );
        }
    };
}
/*
=================
SV_ResetPureClient_f
=================
*/

unsafe extern "C" fn SV_ResetPureClient_f(mut cl: *mut crate::server_h::client_t) {
    (*cl).pureAuthentic = 0;
    (*cl).gotCP = crate::src::qcommon::q_shared::qfalse;
}
/*
=================
SV_UserinfoChanged

Pull specific info from a newly changed userinfo string
into a more C friendly form.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_UserinfoChanged(mut cl: *mut crate::server_h::client_t) {
    let mut val: *mut i8 = 0 as *mut i8;
    let mut ip: *mut i8 = 0 as *mut i8;
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    // name for C code
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*cl).name.as_mut_ptr(),
        crate::src::qcommon::q_shared::Info_ValueForKey(
            (*cl).userinfo.as_mut_ptr(),
            b"name\x00" as *const u8 as *const i8,
        ),
        ::std::mem::size_of::<[i8; 32]>() as i32,
    );
    // rate command
    // if the client is on the same subnet as the server and we aren't running an
    // internet public server, assume they don't need a rate choke
    if crate::src::qcommon::net_ip::Sys_IsLANAddress((*cl).netchan.remoteAddress) != 0
        && (*crate::src::qcommon::common::com_dedicated).integer != 2
        && (*crate::src::server::sv_main::sv_lanForceRate).integer == 1
    {
        (*cl).rate = 99999
    // lans should not rate limit
    } else {
        val = crate::src::qcommon::q_shared::Info_ValueForKey(
            (*cl).userinfo.as_mut_ptr(),
            b"rate\x00" as *const u8 as *const i8,
        );
        if crate::stdlib::strlen(val) != 0 {
            i = atoi(val);
            (*cl).rate = i;
            if (*cl).rate < 1000 {
                (*cl).rate = 1000
            } else if (*cl).rate > 90000 {
                (*cl).rate = 90000
            }
        } else {
            (*cl).rate = 3000
        }
    }
    val = crate::src::qcommon::q_shared::Info_ValueForKey(
        (*cl).userinfo.as_mut_ptr(),
        b"handicap\x00" as *const u8 as *const i8,
    );
    if crate::stdlib::strlen(val) != 0 {
        i = atoi(val);
        if i <= 0 || i > 100 || crate::stdlib::strlen(val) > 4 {
            crate::src::qcommon::q_shared::Info_SetValueForKey(
                (*cl).userinfo.as_mut_ptr(),
                b"handicap\x00" as *const u8 as *const i8,
                b"100\x00" as *const u8 as *const i8,
            );
        }
    }
    // snaps command
    val = crate::src::qcommon::q_shared::Info_ValueForKey(
        (*cl).userinfo.as_mut_ptr(),
        b"snaps\x00" as *const u8 as *const i8,
    );
    if crate::stdlib::strlen(val) != 0 {
        i = atoi(val);
        if i < 1 {
            i = 1
        } else if i > (*crate::src::server::sv_main::sv_fps).integer {
            i = (*crate::src::server::sv_main::sv_fps).integer
        }
        i = 1000 / i
    } else {
        i = 50
    }
    if i != (*cl).snapshotMsec {
        // Reset last sent snapshot so we avoid desync between server frame time and snapshot send time
        (*cl).lastSnapshotTime = 0;
        (*cl).snapshotMsec = i
    }
    if (*cl).compat as u64 != 0 {
        (*cl).hasVoip = crate::src::qcommon::q_shared::qfalse
    } else {
        val = crate::src::qcommon::q_shared::Info_ValueForKey(
            (*cl).userinfo.as_mut_ptr(),
            b"cl_voipProtocol\x00" as *const u8 as *const i8,
        );
        (*cl).hasVoip =
            (crate::src::qcommon::q_shared::Q_stricmp(val, b"opus\x00" as *const u8 as *const i8)
                == 0) as crate::src::qcommon::q_shared::qboolean
    }
    // TTimo
    // maintain the IP information
    // the banning code relies on this being consistently present
    if crate::src::qcommon::net_ip::NET_IsLocalAddress((*cl).netchan.remoteAddress) as u64 != 0 {
        ip = b"localhost\x00" as *const u8 as *mut i8
    } else {
        ip = crate::src::qcommon::net_ip::NET_AdrToString((*cl).netchan.remoteAddress) as *mut i8
    }
    val = crate::src::qcommon::q_shared::Info_ValueForKey(
        (*cl).userinfo.as_mut_ptr(),
        b"ip\x00" as *const u8 as *const i8,
    );
    if *val.offset(0) != 0 {
        len = crate::stdlib::strlen(ip)
            .wrapping_sub(crate::stdlib::strlen(val))
            .wrapping_add(crate::stdlib::strlen((*cl).userinfo.as_mut_ptr())) as i32
    } else {
        len = crate::stdlib::strlen(ip)
            .wrapping_add(4usize)
            .wrapping_add(crate::stdlib::strlen((*cl).userinfo.as_mut_ptr())) as i32
    }
    if len >= 1024 {
        SV_DropClient(
            cl,
            b"userinfo string length exceeded\x00" as *const u8 as *const i8,
        );
    } else {
        crate::src::qcommon::q_shared::Info_SetValueForKey(
            (*cl).userinfo.as_mut_ptr(),
            b"ip\x00" as *const u8 as *const i8,
            ip,
        );
    };
}
/*
==================
SV_UpdateUserinfo_f
==================
*/

unsafe extern "C" fn SV_UpdateUserinfo_f(mut cl: *mut crate::server_h::client_t) {
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*cl).userinfo.as_mut_ptr(),
        crate::src::qcommon::cmd::Cmd_Argv(1),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    SV_UserinfoChanged(cl);
    // call prog code to allow overrides
    crate::src::qcommon::vm::VM_Call(
        crate::src::server::sv_main::gvm,
        crate::g_public_h::GAME_CLIENT_USERINFO_CHANGED as i32,
        cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients),
    );
}

unsafe extern "C" fn SV_UpdateVoipIgnore(
    mut cl: *mut crate::server_h::client_t,
    mut idstr: *const i8,
    mut ignore: crate::src::qcommon::q_shared::qboolean,
) {
    if *idstr as i32 >= '0' as i32 && *idstr as i32 <= '9' as i32 {
        let id: i32 = atoi(idstr);
        if id >= 0 && id < 64 {
            (*cl).ignoreVoipFromClient[id as usize] = ignore
        }
    };
}
/*
==================
SV_Voip_f
==================
*/

unsafe extern "C" fn SV_Voip_f(mut cl: *mut crate::server_h::client_t) {
    let mut cmd: *const i8 = crate::src::qcommon::cmd::Cmd_Argv(1);
    if crate::stdlib::strcmp(cmd, b"ignore\x00" as *const u8 as *const i8) == 0 {
        SV_UpdateVoipIgnore(
            cl,
            crate::src::qcommon::cmd::Cmd_Argv(2i32),
            crate::src::qcommon::q_shared::qtrue,
        );
    } else if crate::stdlib::strcmp(cmd, b"unignore\x00" as *const u8 as *const i8) == 0 {
        SV_UpdateVoipIgnore(
            cl,
            crate::src::qcommon::cmd::Cmd_Argv(2i32),
            crate::src::qcommon::q_shared::qfalse,
        );
    } else if crate::stdlib::strcmp(cmd, b"muteall\x00" as *const u8 as *const i8) == 0 {
        (*cl).muteAllVoip = crate::src::qcommon::q_shared::qtrue
    } else if crate::stdlib::strcmp(cmd, b"unmuteall\x00" as *const u8 as *const i8) == 0 {
        (*cl).muteAllVoip = crate::src::qcommon::q_shared::qfalse
    };
}

static mut ucmds: [ucmd_t; 10] = unsafe {
    [
        {
            let mut init = ucmd_t {
                name: b"userinfo\x00" as *const u8 as *mut i8,
                func: Some(
                    SV_UpdateUserinfo_f
                        as unsafe extern "C" fn(_: *mut crate::server_h::client_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"disconnect\x00" as *const u8 as *mut i8,
                func: Some(
                    SV_Disconnect_f
                        as unsafe extern "C" fn(_: *mut crate::server_h::client_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"cp\x00" as *const u8 as *mut i8,
                func: Some(
                    SV_VerifyPaks_f
                        as unsafe extern "C" fn(_: *mut crate::server_h::client_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"vdr\x00" as *const u8 as *mut i8,
                func: Some(
                    SV_ResetPureClient_f
                        as unsafe extern "C" fn(_: *mut crate::server_h::client_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"download\x00" as *const u8 as *mut i8,
                func: Some(
                    SV_BeginDownload_f
                        as unsafe extern "C" fn(_: *mut crate::server_h::client_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"nextdl\x00" as *const u8 as *mut i8,
                func: Some(
                    SV_NextDownload_f
                        as unsafe extern "C" fn(_: *mut crate::server_h::client_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"stopdl\x00" as *const u8 as *mut i8,
                func: Some(
                    SV_StopDownload_f
                        as unsafe extern "C" fn(_: *mut crate::server_h::client_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"donedl\x00" as *const u8 as *mut i8,
                func: Some(
                    SV_DoneDownload_f
                        as unsafe extern "C" fn(_: *mut crate::server_h::client_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: b"voip\x00" as *const u8 as *mut i8,
                func: Some(
                    SV_Voip_f as unsafe extern "C" fn(_: *mut crate::server_h::client_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = ucmd_t {
                name: 0 as *mut i8,
                func: None,
            };
            init
        },
    ]
};
/*
==================
SV_ExecuteClientCommand

Also called by bot code
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_ExecuteClientCommand(
    mut cl: *mut crate::server_h::client_t,
    mut s: *const i8,
    mut clientOK: crate::src::qcommon::q_shared::qboolean,
) {
    let mut u: *mut ucmd_t = 0 as *mut ucmd_t;
    let mut bProcessed: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    crate::src::qcommon::cmd::Cmd_TokenizeString(s);
    // see if it is a server level command
    u = ucmds.as_mut_ptr();
    while !(*u).name.is_null() {
        if crate::stdlib::strcmp(crate::src::qcommon::cmd::Cmd_Argv(0), (*u).name) == 0 {
            (*u).func.expect("non-null function pointer")(cl);
            bProcessed = crate::src::qcommon::q_shared::qtrue;
            break;
        } else {
            u = u.offset(1)
        }
    }
    if clientOK as u64 != 0 {
        // pass unknown strings to the game
        if (*u).name.is_null()
            && crate::src::server::sv_main::sv.state == crate::server_h::SS_GAME
            && ((*cl).state == crate::server_h::CS_ACTIVE
                || (*cl).state == crate::server_h::CS_PRIMED)
        {
            crate::src::qcommon::cmd::Cmd_Args_Sanitize();
            crate::src::qcommon::vm::VM_Call(
                crate::src::server::sv_main::gvm,
                crate::g_public_h::GAME_CLIENT_COMMAND as i32,
                cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients),
            );
        }
    } else if bProcessed as u64 == 0 {
        crate::src::qcommon::common::Com_DPrintf(
            b"client text ignored for %s: %s\n\x00" as *const u8 as *const i8,
            (*cl).name.as_mut_ptr(),
            crate::src::qcommon::cmd::Cmd_Argv(0i32),
        );
    };
}
/*
===============
SV_ClientCommand
===============
*/

unsafe extern "C" fn SV_ClientCommand(
    mut cl: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut seq: i32 = 0;
    let mut s: *const i8 = 0 as *const i8;
    let mut clientOk: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qtrue;
    seq = crate::src::qcommon::msg::MSG_ReadLong(msg);
    s = crate::src::qcommon::msg::MSG_ReadString(msg);
    // see if we have already executed it
    if (*cl).lastClientCommand >= seq {
        return crate::src::qcommon::q_shared::qtrue;
    }
    crate::src::qcommon::common::Com_DPrintf(
        b"clientCommand: %s : %i : %s\n\x00" as *const u8 as *const i8,
        (*cl).name.as_mut_ptr(),
        seq,
        s,
    );
    // drop the connection if we have somehow lost commands
    if seq > (*cl).lastClientCommand + 1 {
        crate::src::qcommon::common::Com_Printf(
            b"Client %s lost %i clientCommands\n\x00" as *const u8 as *const i8,
            (*cl).name.as_mut_ptr(),
            seq - (*cl).lastClientCommand + 1i32,
        );
        SV_DropClient(cl, b"Lost reliable commands\x00" as *const u8 as *const i8);
        return crate::src::qcommon::q_shared::qfalse;
    }
    // malicious users may try using too many string commands
    // to lag other players.  If we decide that we want to stall
    // the command, we will stop processing the rest of the packet,
    // including the usercmd.  This causes flooders to lag themselves
    // but not other people
    // We don't do this when the client hasn't been active yet since it's
    // normal to spam a lot of commands when downloading
    if (*crate::src::qcommon::common::com_cl_running).integer == 0
        && (*cl).state >= crate::server_h::CS_ACTIVE
        && (*crate::src::server::sv_main::sv_floodProtect).integer != 0
        && crate::src::server::sv_main::svs.time < (*cl).nextReliableTime
    {
        // ignore any other text messages from this client but let them keep playing
        // TTimo - moved the ignored verbose to the actual processing in SV_ExecuteClientCommand, only printing if the core doesn't intercept
        clientOk = crate::src::qcommon::q_shared::qfalse
    }
    // don't allow another command for one second
    (*cl).nextReliableTime = crate::src::server::sv_main::svs.time + 1000;
    SV_ExecuteClientCommand(cl, s, clientOk);
    (*cl).lastClientCommand = seq;
    crate::src::qcommon::q_shared::Com_sprintf(
        (*cl).lastClientCommandString.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>() as i32,
        b"%s\x00" as *const u8 as *const i8,
        s,
    );
    return crate::src::qcommon::q_shared::qtrue;
    // continue procesing
}
//==================================================================================
/*
==================
SV_ClientThink

Also called by bot code
==================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_ClientThink(
    mut cl: *mut crate::server_h::client_t,
    mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t,
) {
    (*cl).lastUsercmd = *cmd;
    if (*cl).state != crate::server_h::CS_ACTIVE {
        return;
        // may have been kicked during the last usercmd
    }
    crate::src::qcommon::vm::VM_Call(
        crate::src::server::sv_main::gvm,
        crate::g_public_h::GAME_CLIENT_THINK as i32,
        cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients),
    );
}
/*
==================
SV_UserMove

The message usually contains all the movement commands
that were in the last three packets, so that the information
in dropped packets can be recovered.

On very fast clients, there may be multiple usercmd packed into
each of the backup packets.
==================
*/

unsafe extern "C" fn SV_UserMove(
    mut cl: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
    mut delta: crate::src::qcommon::q_shared::qboolean,
) {
    let mut i: i32 = 0;
    let mut key: i32 = 0;
    let mut cmdCount: i32 = 0;
    let mut nullcmd: crate::src::qcommon::q_shared::usercmd_t =
        crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        };
    let mut cmds: [crate::src::qcommon::q_shared::usercmd_t; 32] =
        [crate::src::qcommon::q_shared::usercmd_t {
            serverTime: 0,
            angles: [0; 3],
            buttons: 0,
            weapon: 0,
            forwardmove: 0,
            rightmove: 0,
            upmove: 0,
        }; 32];
    let mut cmd: *mut crate::src::qcommon::q_shared::usercmd_t =
        0 as *mut crate::src::qcommon::q_shared::usercmd_t;
    let mut oldcmd: *mut crate::src::qcommon::q_shared::usercmd_t =
        0 as *mut crate::src::qcommon::q_shared::usercmd_t;
    if delta as u64 != 0 {
        (*cl).deltaMessage = (*cl).messageAcknowledge
    } else {
        (*cl).deltaMessage = -(1)
    }
    cmdCount = crate::src::qcommon::msg::MSG_ReadByte(msg);
    if cmdCount < 1 {
        crate::src::qcommon::common::Com_Printf(b"cmdCount < 1\n\x00" as *const u8 as *const i8);
        return;
    }
    if cmdCount > 32 {
        crate::src::qcommon::common::Com_Printf(
            b"cmdCount > MAX_PACKET_USERCMDS\n\x00" as *const u8 as *const i8,
        );
        return;
    }
    // use the checksum feed in the key
    key = crate::src::server::sv_main::sv.checksumFeed;
    // also use the message acknowledge
    key ^= (*cl).messageAcknowledge;
    // also use the last acknowledged server command in the key
    key ^= crate::src::qcommon::msg::MSG_HashKey(
        (*cl).reliableCommands[((*cl).reliableAcknowledge & 64i32 - 1) as usize].as_mut_ptr(),
        32,
    );
    crate::stdlib::memset(
        &mut nullcmd as *mut crate::src::qcommon::q_shared::usercmd_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::qcommon::q_shared::usercmd_t>(),
    );
    oldcmd = &mut nullcmd;
    i = 0;
    while i < cmdCount {
        cmd = &mut *cmds.as_mut_ptr().offset(i as isize)
            as *mut crate::src::qcommon::q_shared::usercmd_t;
        crate::src::qcommon::msg::MSG_ReadDeltaUsercmdKey(msg, key, oldcmd, cmd);
        oldcmd = cmd;
        i += 1
    }
    // save time for ping calculation
    (*cl).frames[((*cl).messageAcknowledge & 32i32 - 1) as usize].messageAcked =
        crate::src::server::sv_main::svs.time;
    // TTimo
    // catch the no-cp-yet situation before SV_ClientEnterWorld
    // if CS_ACTIVE, then it's time to trigger a new gamestate emission
    // if not, then we are getting remaining parasite usermove commands, which we should ignore
    if (*crate::src::server::sv_main::sv_pure).integer != 0
        && (*cl).pureAuthentic == 0
        && (*cl).gotCP as u64 == 0
    {
        if (*cl).state == crate::server_h::CS_ACTIVE {
            // we didn't get a cp yet, don't assume anything and just send the gamestate all over again
            crate::src::qcommon::common::Com_DPrintf(
                b"%s: didn\'t get cp command, resending gamestate\n\x00" as *const u8 as *const i8,
                (*cl).name.as_mut_ptr(),
            );
            SV_SendClientGameState(cl);
        }
        return;
    }
    // if this is the first usercmd we have received
    // this gamestate, put the client into the world
    if (*cl).state == crate::server_h::CS_PRIMED {
        SV_ClientEnterWorld(cl, &mut *cmds.as_mut_ptr().offset(0isize));
        // the moves can be processed normaly
    }
    // a bad cp command was sent, drop the client
    if (*crate::src::server::sv_main::sv_pure).integer != 0 && (*cl).pureAuthentic == 0 {
        SV_DropClient(
            cl,
            b"Cannot validate pure client!\x00" as *const u8 as *const i8,
        );
        return;
    }
    if (*cl).state != crate::server_h::CS_ACTIVE {
        (*cl).deltaMessage = -(1);
        return;
    }
    // usually, the first couple commands will be duplicates
    // of ones we have previously received, but the servertimes
    // in the commands will cause them to be immediately discarded
    i = 0;
    while i < cmdCount {
        // if this is a cmd from before a map_restart ignore it
        if !(cmds[i as usize].serverTime > cmds[(cmdCount - 1) as usize].serverTime) {
            // extremely lagged or cmd from before a map_restart
            //if ( cmds[i].serverTime > svs.time + 3000 ) {
            //	continue;
            //}
            // don't execute if this is an old cmd which is already executed
            // these old cmds are included when cl_packetdup > 0
            if !(cmds[i as usize].serverTime <= (*cl).lastUsercmd.serverTime) {
                SV_ClientThink(cl, &mut *cmds.as_mut_ptr().offset(i as isize));
            }
        }
        i += 1
    }
}
/*
==================
SV_ShouldIgnoreVoipSender

Blocking of voip packets based on source client
==================
*/

unsafe extern "C" fn SV_ShouldIgnoreVoipSender(
    mut cl: *const crate::server_h::client_t,
) -> crate::src::qcommon::q_shared::qboolean {
    if (*crate::src::server::sv_main::sv_voip).integer == 0 {
        return crate::src::qcommon::q_shared::qtrue;
    } else {
        if (*cl).hasVoip as u64 == 0 {
            // VoIP disabled on this server.
            // client doesn't have VoIP support?!
            return crate::src::qcommon::q_shared::qtrue;
        }
    }
    // !!! FIXME: implement player blacklist.
    return crate::src::qcommon::q_shared::qfalse;
    // don't ignore.
}

unsafe extern "C" fn SV_UserVoip(
    mut cl: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
    mut ignoreData: crate::src::qcommon::q_shared::qboolean,
) {
    let mut sender: i32 = 0; // short/invalid packet, bail.
    let mut generation: i32 = 0;
    let mut sequence: i32 = 0;
    let mut frames: i32 = 0;
    let mut packetsize: i32 = 0;
    let mut recips: [crate::stdlib::uint8_t; 8] = [0; 8];
    let mut flags: i32 = 0;
    let mut encoded: [crate::src::qcommon::q_shared::byte; 4000] = [0; 4000];
    let mut client: *mut crate::server_h::client_t = 0 as *mut crate::server_h::client_t;
    let mut packet: *mut crate::server_h::voipServerPacket_t =
        0 as *mut crate::server_h::voipServerPacket_t;
    let mut i: i32 = 0;
    sender = cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32;
    generation = crate::src::qcommon::msg::MSG_ReadByte(msg);
    sequence = crate::src::qcommon::msg::MSG_ReadLong(msg);
    frames = crate::src::qcommon::msg::MSG_ReadByte(msg);
    crate::src::qcommon::msg::MSG_ReadData(
        msg,
        recips.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[crate::stdlib::uint8_t; 8]>() as i32,
    );
    flags = crate::src::qcommon::msg::MSG_ReadByte(msg);
    packetsize = crate::src::qcommon::msg::MSG_ReadShort(msg);
    if (*msg).readcount > (*msg).cursize {
        return;
    }
    if packetsize as usize > ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 4000]>() {
        // overlarge packet?
        let mut bytesleft: i32 = packetsize;
        while bytesleft != 0 {
            let mut br: i32 = bytesleft;
            if br as usize > ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 4000]>() {
                br = ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 4000]>() as i32
            }
            crate::src::qcommon::msg::MSG_ReadData(
                msg,
                encoded.as_mut_ptr() as *mut libc::c_void,
                br,
            );
            bytesleft -= br
        }
        return;
        // overlarge packet, bail.
    } // Blacklisted, disabled, etc.
    crate::src::qcommon::msg::MSG_ReadData(
        msg,
        encoded.as_mut_ptr() as *mut libc::c_void,
        packetsize,
    );
    if ignoreData != 0 || SV_ShouldIgnoreVoipSender(cl) != 0 {
        return;
    }
    // !!! FIXME: see if we read past end of msg...
    // !!! FIXME: reject if not opus data.
    // !!! FIXME: decide if this is bogus data?
    // decide who needs this VoIP packet sent to them...
    i = 0; // no VoIP allowed if downloading, to save bandwidth.
    client = crate::src::server::sv_main::svs.clients; // not addressed to this player.
    while i < (*crate::src::server::sv_main::sv_maxclients).integer {
        if !((*client).state != crate::server_h::CS_ACTIVE) {
            if !(i == sender) {
                if !((*client).hasVoip as u64 == 0) {
                    if !((*client).muteAllVoip as u64 != 0) {
                        if !((*client).ignoreVoipFromClient[sender as usize] as u64 != 0) {
                            if !(*(*cl).downloadName.as_mut_ptr() != 0) {
                                if crate::src::qcommon::common::Com_IsVoipTarget(
                                    recips.as_mut_ptr(),
                                    ::std::mem::size_of::<[crate::stdlib::uint8_t; 8]>() as i32,
                                    i,
                                ) as u64
                                    != 0
                                {
                                    flags |= 0x2
                                } else {
                                    flags &= !(0x2)
                                }
                                if !(flags & (0x1 | 0x2) == 0) {
                                    // Transmit this packet to the client.
                                    if (*client).queuedVoipPackets as usize
                                        >= (::std::mem::size_of::<
                                            [*mut crate::server_h::voipServerPacket_t; 64],
                                        >())
                                        .wrapping_div(::std::mem::size_of::<
                                            *mut crate::server_h::voipServerPacket_t,
                                        >())
                                    {
                                        crate::src::qcommon::common::Com_Printf(
                                            b"Too many VoIP packets queued for client #%d\n\x00"
                                                as *const u8
                                                as *const i8,
                                            i,
                                        );
                                    // no room for another packet right now.
                                    } else {
                                        packet = crate::src::qcommon::common::Z_Malloc(
                                            ::std::mem::size_of::<crate::server_h::voipServerPacket_t>(
                                            ) as i32,
                                        )
                                            as *mut crate::server_h::voipServerPacket_t;
                                        (*packet).sender = sender;
                                        (*packet).frames = frames;
                                        (*packet).len = packetsize;
                                        (*packet).generation = generation;
                                        (*packet).sequence = sequence;
                                        (*packet).flags = flags;
                                        crate::stdlib::memcpy(
                                            (*packet).data.as_mut_ptr() as *mut libc::c_void,
                                            encoded.as_mut_ptr() as *const libc::c_void,
                                            packetsize as usize,
                                        );
                                        (*client).voipPacket[(((*client).queuedVoipIndex
                                            + (*client).queuedVoipPackets)
                                            as usize)
                                            .wrapping_rem(
                                                (::std::mem::size_of::<
                                                    [*mut crate::server_h::voipServerPacket_t; 64],
                                                >(
                                                ))
                                                .wrapping_div(::std::mem::size_of::<
                                                    *mut crate::server_h::voipServerPacket_t,
                                                >(
                                                )),
                                            )] = packet;
                                        (*client).queuedVoipPackets += 1
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // !!! FIXME: possible to DoS?
        i += 1;
        client = client.offset(1)
    }
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
// server.h
//=============================================================================
// !!! MUST NOT CHANGE, SERVER AND
// GAME BOTH REFERENCE !!!
// for delta compression of initial sighting
// if -1, use headnode instead
// if all the clusters don't fit in clusternums
// used to prevent double adding from portal views
// no map loaded
// spawning level entities
// actively running
// if true, send configstring changes during SS_LOADING
// changes each server start
// serverId before a map_restart
// the feed key that we use to compute the pure checksum strings
// https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=475
// the serverId associated with the current checksumFeed (always <= serverId)
// incremented for each snapshot built
// <= 1000 / sv_frame->value
// when time > nextFrameTime, process world
// used during game VM init
// the game virtual machine will update these on init and changes
// current number, <= MAX_GENTITIES
// will be > sizeof(playerState_t) due to game private data
// portalarea visibility bits
// into the circular sv_packet_entities[]
// the entities MUST be in increasing state number
// order, otherwise the delta compression will fail
// time the message was transmitted
// time the message was acked
// used to rate drop packets
// can be reused for a new connection
// client has been disconnected, but don't reuse
// connection for a couple seconds
// has been assigned to a client_t, but no gamestate yet
// gamestate has been sent, but client hasn't sent a usercmd
// client is fully in game
// valid command string for SV_Netchan_Encode
// name, etc
// last added reliable message, not necessarily sent or acknowledged yet
// last acknowledged reliable message
// last sent reliable message, not necessarily acknowledged yet
// netchan->outgoingSequence of gamestate
// for delta compression
// reliable client message sequence
// SV_GentityNum(clientnum)
// extracted from userinfo, high bits masked
// downloading
// if not empty string, we are downloading
// file being downloaded
// total bytes (can't use EOF because of paks)
// bytes sent
// last block we sent to the client, awaiting ack
// current block number
// last block we xmited
// the buffers for the download blocks
// We have sent the EOF block
// time we last got an ack from the client
// frame last client usercmd message
// svs.time when another reliable command will be allowed
// svs.time when packet was last received
// svs.time when connection started
// svs.time of last sent snapshot
// true if nextSnapshotTime was set based on rate instead of snapshotMsec
// must timeout a few frames in a row so debugging doesn't break
// updates can be delta'd from here
// bytes / second
// requests a snapshot every snapshotMsec unless rate choked
// TTimo - additional flag to distinguish between a bad pure checksum, and no cp command at all
// TTimo
// queuing outgoing fragmented messages to send them properly, without udp packet bursts
// in case large fragmented messages are stacking up
// buffer them into this queue, and hand them out to netchan as needed
//=============================================================================
// MAX_CHALLENGES is made large to prevent a denial
// of service attack that could cycle all of them
// out before legitimate users connected
// Allow a certain amount of challenges to have the same IP address
// to make it a bit harder to DOS one single IP address from connecting
// while not allowing a single ip to grab all challenge resources
// challenge number coming from the client
// time the last packet was sent to the autherize server
// time the challenge response was sent to client
// time the adr was first used, for authorize timeout checks
// this structure will be cleared only when the game dll changes
// sv_init has completed
// will be strictly increasing across level changes
// ^= SNAPFLAG_SERVERCOUNT every SV_SpawnServer()
// [sv_maxclients->integer];
// sv_maxclients->integer*PACKET_BACKUP*MAX_SNAPSHOT_ENTITIES
// next snapshotEntities to use
// [numSnapshotEntities]
// to prevent invalid IPs from connecting
// for rcon return messages
// authorize server address
// next svs.time that server should do dns lookup for master server
// Structure for managing bans
// For a CIDR-Notation type suffix
//=============================================================================
// persistant server info across maps
// cleared each map
// game virtual machine
//===========================================================
//
// sv_main.c
//
//
// sv_init.c
//
//
// sv_client.c
//
/*
===========================================================================

USER CMD EXECUTION

===========================================================================
*/
/*
===================
SV_ExecuteClientMessage

Parse a client packet
===================
*/
#[no_mangle]

pub unsafe extern "C" fn SV_ExecuteClientMessage(
    mut cl: *mut crate::server_h::client_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) {
    let mut c: i32 = 0;
    let mut serverId: i32 = 0;
    crate::src::qcommon::msg::MSG_Bitstream(msg);
    serverId = crate::src::qcommon::msg::MSG_ReadLong(msg);
    (*cl).messageAcknowledge = crate::src::qcommon::msg::MSG_ReadLong(msg);
    if (*cl).messageAcknowledge < 0 {
        // usually only hackers create messages like this
        // it is more annoying for them to let them hanging
        return;
    }
    (*cl).reliableAcknowledge = crate::src::qcommon::msg::MSG_ReadLong(msg);
    // NOTE: when the client message is fux0red the acknowledgement numbers
    // can be out of range, this could cause the server to send thousands of server
    // commands which the server thinks are not yet acknowledged in SV_UpdateServerCommandsToClient
    if (*cl).reliableAcknowledge < (*cl).reliableSequence - 64 {
        // usually only hackers create messages like this
        // it is more annoying for them to let them hanging
        (*cl).reliableAcknowledge = (*cl).reliableSequence;
        return;
    }
    // if this is a usercmd from a previous gamestate,
    // ignore it or retransmit the current gamestate
    //
    // if the client was downloading, let it stay at whatever serverId and
    // gamestate it was at.  This allows it to keep downloading even when
    // the gamestate changes.  After the download is finished, we'll
    // notice and send it a new game state
    //
    // https://zerowing.idsoftware.com/bugzilla/show_bug.cgi?id=536
    // don't drop as long as previous command was a nextdl, after a dl is done, downloadName is set back to ""
    // but we still need to read the next message to move to next download or send gamestate
    // I don't like this hack though, it must have been working fine at some point, suspecting the fix is somewhere else
    if serverId != crate::src::server::sv_main::sv.serverId
        && *(*cl).downloadName.as_mut_ptr() == 0
        && crate::stdlib::strstr(
            (*cl).lastClientCommandString.as_mut_ptr(),
            b"nextdl\x00" as *const u8 as *const i8,
        )
        .is_null()
    {
        if serverId >= crate::src::server::sv_main::sv.restartedServerId
            && serverId < crate::src::server::sv_main::sv.serverId
        {
            // TTimo - use a comparison here to catch multiple map_restart
            // they just haven't caught the map_restart yet
            crate::src::qcommon::common::Com_DPrintf(
                b"%s : ignoring pre map_restart / outdated client message\n\x00" as *const u8
                    as *const i8,
                (*cl).name.as_mut_ptr(),
            );
            return;
        }
        // if we can tell that the client has dropped the last
        // gamestate we sent them, resend it
        if (*cl).state != crate::server_h::CS_ACTIVE
            && (*cl).messageAcknowledge > (*cl).gamestateMessageNum
        {
            crate::src::qcommon::common::Com_DPrintf(
                b"%s : dropped gamestate, resending\n\x00" as *const u8 as *const i8,
                (*cl).name.as_mut_ptr(),
            );
            SV_SendClientGameState(cl);
        }
        return;
    }
    // this client has acknowledged the new gamestate so it's
    // safe to start sending it the real time again
    if (*cl).oldServerTime != 0 && serverId == crate::src::server::sv_main::sv.serverId {
        crate::src::qcommon::common::Com_DPrintf(
            b"%s acknowledged gamestate\n\x00" as *const u8 as *const i8,
            (*cl).name.as_mut_ptr(),
        );
        (*cl).oldServerTime = 0
    }
    loop
    // read optional clientCommand strings
    {
        c = crate::src::qcommon::msg::MSG_ReadByte(msg);
        if c == crate::qcommon_h::clc_EOF as i32 {
            break;
        }
        if c != crate::qcommon_h::clc_clientCommand as i32 {
            break;
        }
        if SV_ClientCommand(cl, msg) as u64 == 0 {
            return;
            // we couldn't execute it because of the flood protection
        }
        if (*cl).state == crate::server_h::CS_ZOMBIE {
            return;
            // disconnect command
        }
    }
    // skip legacy speex voip data
    if c == crate::qcommon_h::clc_voipSpeex as i32 {
        SV_UserVoip(cl, msg, crate::src::qcommon::q_shared::qtrue);
        c = crate::src::qcommon::msg::MSG_ReadByte(msg)
    }
    // read optional voip data
    if c == crate::qcommon_h::clc_voipOpus as i32 {
        SV_UserVoip(cl, msg, crate::src::qcommon::q_shared::qfalse);
        c = crate::src::qcommon::msg::MSG_ReadByte(msg)
    }
    // read the usercmd_t
    if c == crate::qcommon_h::clc_move as i32 {
        SV_UserMove(cl, msg, crate::src::qcommon::q_shared::qtrue);
    } else if c == crate::qcommon_h::clc_moveNoDelta as i32 {
        SV_UserMove(cl, msg, crate::src::qcommon::q_shared::qfalse);
    } else if c != crate::qcommon_h::clc_EOF as i32 {
        crate::src::qcommon::common::Com_Printf(
            b"WARNING: bad command byte for client %i\n\x00" as *const u8 as *const i8,
            cl.wrapping_offset_from(crate::src::server::sv_main::svs.clients) as i32,
        );
    };
    //	if ( msg->readcount != msg->cursize ) {
    //		Com_Printf( "WARNING: Junk at end of packet for client %i\n", cl - svs.clients );
    //	}
}
