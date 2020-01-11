use ::libc;

pub mod stdlib_h {
    #[inline]

    pub unsafe extern "C" fn atoi(mut __nptr: *const i8) -> i32 {
        return crate::stdlib::strtol(__nptr, 0 as *mut *mut i8, 10) as i32;
    }
}

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qcommon_h::msg_t;
pub use crate::qcommon_h::netadr_t;
pub use crate::qcommon_h::netadrtype_t;
pub use crate::qcommon_h::netchan_t;
pub use crate::qcommon_h::netsrc_t;
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
pub use crate::src::qcommon::common::cl_packetdelay;
pub use crate::src::qcommon::common::com_timescale;
pub use crate::src::qcommon::common::sv_packetdelay;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::S_Malloc;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::cvar::Cvar_Get;
pub use crate::src::qcommon::huffman::Huff_Compress;
pub use crate::src::qcommon::msg::MSG_BeginReadingOOB;
pub use crate::src::qcommon::msg::MSG_InitOOB;
pub use crate::src::qcommon::msg::MSG_ReadLong;
pub use crate::src::qcommon::msg::MSG_ReadShort;
pub use crate::src::qcommon::msg::MSG_WriteData;
pub use crate::src::qcommon::msg::MSG_WriteLong;
pub use crate::src::qcommon::msg::MSG_WriteShort;
pub use crate::src::qcommon::net_chan::stdlib_h::atoi;
pub use crate::src::qcommon::net_ip::NET_AdrToString;
pub use crate::src::qcommon::net_ip::Sys_SendPacket;
pub use crate::src::qcommon::net_ip::Sys_StringToAdr;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cvar_s;
pub use crate::src::qcommon::q_shared::cvar_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::va;
pub use crate::src::qcommon::q_shared::Q_CountChar;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
use crate::src::qcommon::q_shared::ShortSwap;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::sys::sys_unix::Sys_Milliseconds;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::strchr;
use crate::stdlib::strcmp;
use crate::stdlib::strlen;
pub use crate::stdlib::strtol;
use crate::stdlib::vsnprintf;
//=============================================================================

pub type packetQueue_t = packetQueue_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packetQueue_s {
    pub next: *mut packetQueue_s,
    pub length: i32,
    pub data: *mut crate::src::qcommon::q_shared::byte,
    pub to: crate::qcommon_h::netadr_t,
    pub release: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loopmsg_t {
    pub data: [crate::src::qcommon::q_shared::byte; 1400],
    pub datalen: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loopback_t {
    pub msgs: [loopmsg_t; 16],
    pub get: i32,
    pub send: i32,
}
#[no_mangle]

pub static mut showpackets: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut showdrop: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *mut crate::src::qcommon::q_shared::cvar_t;
#[no_mangle]

pub static mut qport: *mut crate::src::qcommon::q_shared::cvar_t =
    0 as *mut crate::src::qcommon::q_shared::cvar_t;

static mut netsrcString: [*mut i8; 2] = [
    b"client\x00" as *const u8 as *mut i8,
    b"server\x00" as *const u8 as *mut i8,
];
/*
===============
Netchan_Init

===============
*/
#[no_mangle]

pub unsafe extern "C" fn Netchan_Init(mut port: i32) {
    port &= 0xffff;
    showpackets = crate::src::qcommon::cvar::Cvar_Get(
        b"showpackets\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x100,
    );
    showdrop = crate::src::qcommon::cvar::Cvar_Get(
        b"showdrop\x00" as *const u8 as *const i8,
        b"0\x00" as *const u8 as *const i8,
        0x100,
    );
    qport = crate::src::qcommon::cvar::Cvar_Get(
        b"net_qport\x00" as *const u8 as *const i8,
        crate::src::qcommon::q_shared::va(b"%i\x00" as *const u8 as *mut i8, port),
        0x10,
    );
}
/*
==============
Netchan_Setup

called to open a channel to a remote system
==============
*/
#[no_mangle]

pub unsafe extern "C" fn Netchan_Setup(
    mut sock: crate::qcommon_h::netsrc_t,
    mut chan: *mut crate::qcommon_h::netchan_t,
    mut adr: crate::qcommon_h::netadr_t,
    mut qport_0: i32,
    mut challenge: i32,
    mut compat: crate::src::qcommon::q_shared::qboolean,
) {
    crate::stdlib::memset(
        chan as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::qcommon_h::netchan_t>(),
    );
    (*chan).sock = sock;
    (*chan).remoteAddress = adr;
    (*chan).qport = qport_0;
    (*chan).incomingSequence = 0;
    (*chan).outgoingSequence = 1;
    (*chan).challenge = challenge;
    (*chan).compat = compat;
}
/*
=================
Netchan_TransmitNextFragment

Send one fragment of the current message
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Netchan_TransmitNextFragment(mut chan: *mut crate::qcommon_h::netchan_t) {
    let mut send: crate::qcommon_h::msg_t = crate::qcommon_h::msg_t {
        allowoverflow: crate::src::qcommon::q_shared::qfalse,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        oob: crate::src::qcommon::q_shared::qfalse,
        data: 0 as *mut crate::src::qcommon::q_shared::byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    let mut send_buf: [crate::src::qcommon::q_shared::byte; 1400] = [0; 1400];
    let mut fragmentLength: i32 = 0;
    let mut outgoingSequence: i32 = 0;
    // write the packet header
    crate::src::qcommon::msg::MSG_InitOOB(
        &mut send,
        send_buf.as_mut_ptr(),
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 1400]>() as i32,
    ); // <-- only do the oob here
    outgoingSequence = ((*chan).outgoingSequence as u32 | (1) << 31) as i32;
    crate::src::qcommon::msg::MSG_WriteLong(&mut send, outgoingSequence);
    // send the qport if we are a client
    if (*chan).sock == crate::qcommon_h::NS_CLIENT {
        crate::src::qcommon::msg::MSG_WriteShort(&mut send, (*qport).integer);
    }
    if (*chan).compat as u64 == 0 {
        crate::src::qcommon::msg::MSG_WriteLong(
            &mut send,
            (*chan).challenge ^ (*chan).outgoingSequence * (*chan).challenge,
        );
    }
    // copy the reliable message to the packet first
    fragmentLength = 1400 - 100;
    if (*chan).unsentFragmentStart + fragmentLength > (*chan).unsentLength {
        fragmentLength = (*chan).unsentLength - (*chan).unsentFragmentStart
    }
    crate::src::qcommon::msg::MSG_WriteShort(&mut send, (*chan).unsentFragmentStart);
    crate::src::qcommon::msg::MSG_WriteShort(&mut send, fragmentLength);
    crate::src::qcommon::msg::MSG_WriteData(
        &mut send,
        (*chan)
            .unsentBuffer
            .as_mut_ptr()
            .offset((*chan).unsentFragmentStart as isize) as *const libc::c_void,
        fragmentLength,
    );
    // send the datagram
    NET_SendPacket(
        (*chan).sock,
        send.cursize,
        send.data as *const libc::c_void,
        (*chan).remoteAddress,
    );
    // Store send time and size of this packet for rate control
    (*chan).lastSentTime = crate::src::sys::sys_unix::Sys_Milliseconds();
    (*chan).lastSentSize = send.cursize;
    if (*showpackets).integer != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"%s send %4i : s=%i fragment=%i,%i\n\x00" as *const u8 as *const i8,
            netsrcString[(*chan).sock as usize],
            send.cursize,
            (*chan).outgoingSequence,
            (*chan).unsentFragmentStart,
            fragmentLength,
        );
    }
    (*chan).unsentFragmentStart += fragmentLength;
    // this exit condition is a little tricky, because a packet
    // that is exactly the fragment length still needs to send
    // a second packet of zero length so that the other side
    // can tell there aren't more to follow
    if (*chan).unsentFragmentStart == (*chan).unsentLength && fragmentLength != 1400 - 100 {
        (*chan).outgoingSequence += 1;
        (*chan).unsentFragments = crate::src::qcommon::q_shared::qfalse
    };
}
/*
===============
Netchan_Transmit

Sends a message to a connection, fragmenting if necessary
A 0 length will still generate a packet.
================
*/
#[no_mangle]

pub unsafe extern "C" fn Netchan_Transmit(
    mut chan: *mut crate::qcommon_h::netchan_t,
    mut length: i32,
    mut data: *const crate::src::qcommon::q_shared::byte,
) {
    let mut send: crate::qcommon_h::msg_t = crate::qcommon_h::msg_t {
        allowoverflow: crate::src::qcommon::q_shared::qfalse,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        oob: crate::src::qcommon::q_shared::qfalse,
        data: 0 as *mut crate::src::qcommon::q_shared::byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    let mut send_buf: [crate::src::qcommon::q_shared::byte; 1400] = [0; 1400];
    if length > 16384 {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"Netchan_Transmit: length = %i\x00" as *const u8 as *const i8,
            length,
        );
    }
    (*chan).unsentFragmentStart = 0;
    // fragment large reliable messages
    if length >= 1400 - 100 {
        (*chan).unsentFragments = crate::src::qcommon::q_shared::qtrue;
        (*chan).unsentLength = length;
        crate::stdlib::memcpy(
            (*chan).unsentBuffer.as_mut_ptr() as *mut libc::c_void,
            data as *const libc::c_void,
            length as usize,
        );
        // only send the first fragment now
        Netchan_TransmitNextFragment(chan);
        return;
    }
    // write the packet header
    crate::src::qcommon::msg::MSG_InitOOB(
        &mut send,
        send_buf.as_mut_ptr(),
        ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 1400]>() as i32,
    );
    crate::src::qcommon::msg::MSG_WriteLong(&mut send, (*chan).outgoingSequence);
    // send the qport if we are a client
    if (*chan).sock == crate::qcommon_h::NS_CLIENT {
        crate::src::qcommon::msg::MSG_WriteShort(&mut send, (*qport).integer);
    }
    if (*chan).compat as u64 == 0 {
        crate::src::qcommon::msg::MSG_WriteLong(
            &mut send,
            (*chan).challenge ^ (*chan).outgoingSequence * (*chan).challenge,
        );
    }
    (*chan).outgoingSequence += 1;
    crate::src::qcommon::msg::MSG_WriteData(&mut send, data as *const libc::c_void, length);
    // send the datagram
    NET_SendPacket(
        (*chan).sock,
        send.cursize,
        send.data as *const libc::c_void,
        (*chan).remoteAddress,
    );
    // Store send time and size of this packet for rate control
    (*chan).lastSentTime = crate::src::sys::sys_unix::Sys_Milliseconds();
    (*chan).lastSentSize = send.cursize;
    if (*showpackets).integer != 0 {
        crate::src::qcommon::common::Com_Printf(
            b"%s send %4i : s=%i ack=%i\n\x00" as *const u8 as *const i8,
            netsrcString[(*chan).sock as usize],
            send.cursize,
            (*chan).outgoingSequence - 1i32,
            (*chan).incomingSequence,
        );
    };
}
/*
=================
Netchan_Process

Returns qfalse if the message should not be processed due to being
out of order or a fragment.

Msg must be large enough to hold MAX_MSGLEN, because if this is the
final fragment of a multi-part message, the entire thing will be
copied out.
=================
*/
#[no_mangle]

pub unsafe extern "C" fn Netchan_Process(
    mut chan: *mut crate::qcommon_h::netchan_t,
    mut msg: *mut crate::qcommon_h::msg_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut sequence: i32 = 0;
    let mut fragmentStart: i32 = 0;
    let mut fragmentLength: i32 = 0;
    let mut fragmented: crate::src::qcommon::q_shared::qboolean =
        crate::src::qcommon::q_shared::qfalse;
    // XOR unscramble all data in the packet after the header
    //	Netchan_UnScramblePacket( msg );
    // get sequence numbers
    crate::src::qcommon::msg::MSG_BeginReadingOOB(msg);
    sequence = crate::src::qcommon::msg::MSG_ReadLong(msg);
    // check for fragment information
    if sequence as u32 & (1) << 31 != 0 {
        sequence = (sequence as u32 & !((1) << 31)) as i32;
        fragmented = crate::src::qcommon::q_shared::qtrue
    } else {
        fragmented = crate::src::qcommon::q_shared::qfalse
    }
    // read the qport if we are a server
    if (*chan).sock == crate::qcommon_h::NS_SERVER {
        crate::src::qcommon::msg::MSG_ReadShort(msg);
    }
    if (*chan).compat as u64 == 0 {
        let mut checksum: i32 = crate::src::qcommon::msg::MSG_ReadLong(msg);
        // UDP spoofing protection
        if (*chan).challenge ^ sequence * (*chan).challenge != checksum {
            return crate::src::qcommon::q_shared::qfalse;
        }
    }
    // read the fragment information
    if fragmented as u64 != 0 {
        fragmentStart = crate::src::qcommon::msg::MSG_ReadShort(msg); // stop warning message
        fragmentLength = crate::src::qcommon::msg::MSG_ReadShort(msg)
    } else {
        fragmentStart = 0;
        fragmentLength = 0
    }
    if (*showpackets).integer != 0 {
        if fragmented as u64 != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"%s recv %4i : s=%i fragment=%i,%i\n\x00" as *const u8 as *const i8,
                netsrcString[(*chan).sock as usize],
                (*msg).cursize,
                sequence,
                fragmentStart,
                fragmentLength,
            );
        } else {
            crate::src::qcommon::common::Com_Printf(
                b"%s recv %4i : s=%i\n\x00" as *const u8 as *const i8,
                netsrcString[(*chan).sock as usize],
                (*msg).cursize,
                sequence,
            );
        }
    }
    //
    // discard out of order or duplicated packets
    //
    if sequence <= (*chan).incomingSequence {
        if (*showdrop).integer != 0 || (*showpackets).integer != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"%s:Out of order packet %i at %i\n\x00" as *const u8 as *const i8,
                crate::src::qcommon::net_ip::NET_AdrToString((*chan).remoteAddress),
                sequence,
                (*chan).incomingSequence,
            );
        }
        return crate::src::qcommon::q_shared::qfalse;
    }
    //
    // dropped packets don't keep the message from being used
    //
    (*chan).dropped = sequence - ((*chan).incomingSequence + 1);
    if (*chan).dropped > 0 {
        if (*showdrop).integer != 0 || (*showpackets).integer != 0 {
            crate::src::qcommon::common::Com_Printf(
                b"%s:Dropped %i packets at %i\n\x00" as *const u8 as *const i8,
                crate::src::qcommon::net_ip::NET_AdrToString((*chan).remoteAddress),
                (*chan).dropped,
                sequence,
            );
        }
    }
    //
    // if this is the final framgent of a reliable message,
    // bump incoming_reliable_sequence
    //
    if fragmented as u64 != 0 {
        // TTimo
        // make sure we add the fragments in correct order
        // either a packet was dropped, or we received this one too soon
        // we don't reconstruct the fragments. we will wait till this fragment gets to us again
        // (NOTE: we could probably try to rebuild by out of order chunks if needed)
        if sequence != (*chan).fragmentSequence {
            (*chan).fragmentSequence = sequence;
            (*chan).fragmentLength = 0
        }
        // if we missed a fragment, dump the message
        if fragmentStart != (*chan).fragmentLength {
            if (*showdrop).integer != 0 || (*showpackets).integer != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"%s:Dropped a message fragment\n\x00" as *const u8 as *const i8,
                    crate::src::qcommon::net_ip::NET_AdrToString((*chan).remoteAddress),
                );
            }
            // we can still keep the part that we have so far,
            // so we don't need to clear chan->fragmentLength
            return crate::src::qcommon::q_shared::qfalse;
        }
        // copy the fragment to the fragment buffer
        if fragmentLength < 0
            || (*msg).readcount + fragmentLength > (*msg).cursize
            || ((*chan).fragmentLength + fragmentLength) as usize
                > ::std::mem::size_of::<[crate::src::qcommon::q_shared::byte; 16384]>()
        {
            if (*showdrop).integer != 0 || (*showpackets).integer != 0 {
                crate::src::qcommon::common::Com_Printf(
                    b"%s:illegal fragment length\n\x00" as *const u8 as *const i8,
                    crate::src::qcommon::net_ip::NET_AdrToString((*chan).remoteAddress),
                );
            }
            return crate::src::qcommon::q_shared::qfalse;
        }
        crate::stdlib::memcpy(
            (*chan)
                .fragmentBuffer
                .as_mut_ptr()
                .offset((*chan).fragmentLength as isize) as *mut libc::c_void,
            (*msg).data.offset((*msg).readcount as isize) as *const libc::c_void,
            fragmentLength as usize,
        );
        (*chan).fragmentLength += fragmentLength;
        // if this wasn't the last fragment, don't process anything
        if fragmentLength == 1400 - 100 {
            return crate::src::qcommon::q_shared::qfalse;
        }
        if (*chan).fragmentLength > (*msg).maxsize {
            crate::src::qcommon::common::Com_Printf(
                b"%s:fragmentLength %i > msg->maxsize\n\x00" as *const u8 as *const i8,
                crate::src::qcommon::net_ip::NET_AdrToString((*chan).remoteAddress),
                (*chan).fragmentLength,
            );
            return crate::src::qcommon::q_shared::qfalse;
        }
        // copy the full message over the partial fragment
        // make sure the sequence number is still there
        *((*msg).data as *mut i32) = sequence; // past the sequence number
        crate::stdlib::memcpy(
            (*msg).data.offset(4) as *mut libc::c_void,
            (*chan).fragmentBuffer.as_mut_ptr() as *const libc::c_void,
            (*chan).fragmentLength as usize,
        ); // past the sequence number
        (*msg).cursize = (*chan).fragmentLength + 4;
        (*chan).fragmentLength = 0;
        (*msg).readcount = 4;
        (*msg).bit = 32;
        // TTimo
        // clients were not acking fragmented messages
        (*chan).incomingSequence = sequence;
        return crate::src::qcommon::q_shared::qtrue;
    }
    //
    // the message can now be read from the current message pointer
    //
    (*chan).incomingSequence = sequence;
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub static mut loopbacks: [loopback_t; 2] = [loopback_t {
    msgs: [loopmsg_t {
        data: [0; 1400],
        datalen: 0,
    }; 16],
    get: 0,
    send: 0,
}; 2];
#[no_mangle]

pub unsafe extern "C" fn NET_GetLoopPacket(
    mut sock: crate::qcommon_h::netsrc_t,
    mut net_from: *mut crate::qcommon_h::netadr_t,
    mut net_message: *mut crate::qcommon_h::msg_t,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut i: i32 = 0;
    let mut loop_0: *mut loopback_t = 0 as *mut loopback_t;
    loop_0 = &mut *loopbacks.as_mut_ptr().offset(sock as isize) as *mut loopback_t;
    if (*loop_0).send - (*loop_0).get > 16 {
        (*loop_0).get = (*loop_0).send - 16
    }
    if (*loop_0).get >= (*loop_0).send {
        return crate::src::qcommon::q_shared::qfalse;
    }
    i = (*loop_0).get & 16 - 1;
    (*loop_0).get += 1;
    crate::stdlib::memcpy(
        (*net_message).data as *mut libc::c_void,
        (*loop_0).msgs[i as usize].data.as_mut_ptr() as *const libc::c_void,
        (*loop_0).msgs[i as usize].datalen as usize,
    );
    (*net_message).cursize = (*loop_0).msgs[i as usize].datalen;
    crate::stdlib::memset(
        net_from as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::qcommon_h::netadr_t>(),
    );
    (*net_from).type_0 = crate::qcommon_h::NA_LOOPBACK;
    return crate::src::qcommon::q_shared::qtrue;
}
#[no_mangle]

pub unsafe extern "C" fn NET_SendLoopPacket(
    mut sock: crate::qcommon_h::netsrc_t,
    mut length: i32,
    mut data: *const libc::c_void,
    mut to: crate::qcommon_h::netadr_t,
) {
    let mut i: i32 = 0;
    let mut loop_0: *mut loopback_t = 0 as *mut loopback_t;
    loop_0 = &mut *loopbacks.as_mut_ptr().offset((sock ^ 1u32) as isize) as *mut loopback_t;
    i = (*loop_0).send & 16 - 1;
    (*loop_0).send += 1;
    crate::stdlib::memcpy(
        (*loop_0).msgs[i as usize].data.as_mut_ptr() as *mut libc::c_void,
        data,
        length as usize,
    );
    (*loop_0).msgs[i as usize].datalen = length;
}
#[no_mangle]

pub static mut packetQueue: *mut packetQueue_t = 0 as *mut packetQueue_t;

unsafe extern "C" fn NET_QueuePacket(
    mut length: i32,
    mut data: *const libc::c_void,
    mut to: crate::qcommon_h::netadr_t,
    mut offset: i32,
) {
    let mut new: *mut packetQueue_t = 0 as *mut packetQueue_t;
    let mut next: *mut packetQueue_t = packetQueue;
    if offset > 999 {
        offset = 999
    }
    new = crate::src::qcommon::common::S_Malloc(::std::mem::size_of::<packetQueue_t>() as i32)
        as *mut packetQueue_t;
    (*new).data =
        crate::src::qcommon::common::S_Malloc(length) as *mut crate::src::qcommon::q_shared::byte;
    crate::stdlib::memcpy((*new).data as *mut libc::c_void, data, length as usize);
    (*new).length = length;
    (*new).to = to;
    (*new).release = crate::src::sys::sys_unix::Sys_Milliseconds()
        + (offset as f32 / (*crate::src::qcommon::common::com_timescale).value) as i32;
    (*new).next = 0 as *mut packetQueue_s;
    if packetQueue.is_null() {
        packetQueue = new;
        return;
    }
    while !next.is_null() {
        if (*next).next.is_null() {
            (*next).next = new;
            return;
        }
        next = (*next).next
    }
}
#[no_mangle]

pub unsafe extern "C" fn NET_FlushPacketQueue() {
    let mut last: *mut packetQueue_t = 0 as *mut packetQueue_t;
    let mut now: i32 = 0;
    while !packetQueue.is_null() {
        now = crate::src::sys::sys_unix::Sys_Milliseconds();
        if (*packetQueue).release >= now {
            break;
        }
        crate::src::qcommon::net_ip::Sys_SendPacket(
            (*packetQueue).length,
            (*packetQueue).data as *const libc::c_void,
            (*packetQueue).to,
        );
        last = packetQueue;
        packetQueue = (*packetQueue).next;
        crate::src::qcommon::common::Z_Free((*last).data as *mut libc::c_void);
        crate::src::qcommon::common::Z_Free(last as *mut libc::c_void);
    }
}
#[no_mangle]

pub unsafe extern "C" fn NET_SendPacket(
    mut sock: crate::qcommon_h::netsrc_t,
    mut length: i32,
    mut data: *const libc::c_void,
    mut to: crate::qcommon_h::netadr_t,
) {
    // sequenced packets are shown in netchan, so just show oob
    if (*showpackets).integer != 0 && *(data as *mut i32) == -(1) {
        crate::src::qcommon::common::Com_Printf(
            b"send packet %4i\n\x00" as *const u8 as *const i8,
            length,
        );
    }
    if to.type_0 == crate::qcommon_h::NA_LOOPBACK {
        NET_SendLoopPacket(sock, length, data, to);
        return;
    }
    if to.type_0 == crate::qcommon_h::NA_BOT {
        return;
    }
    if to.type_0 == crate::qcommon_h::NA_BAD {
        return;
    }
    if sock == crate::qcommon_h::NS_CLIENT
        && (*crate::src::qcommon::common::cl_packetdelay).integer > 0
    {
        NET_QueuePacket(
            length,
            data,
            to,
            (*crate::src::qcommon::common::cl_packetdelay).integer,
        );
    } else if sock == crate::qcommon_h::NS_SERVER
        && (*crate::src::qcommon::common::sv_packetdelay).integer > 0
    {
        NET_QueuePacket(
            length,
            data,
            to,
            (*crate::src::qcommon::common::sv_packetdelay).integer,
        );
    } else {
        crate::src::qcommon::net_ip::Sys_SendPacket(length, data, to);
    };
}
/*
===============
NET_OutOfBandPrint

Sends a text message in an out-of-band datagram
================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_OutOfBandPrint(
    mut sock: crate::qcommon_h::netsrc_t,
    mut adr: crate::qcommon_h::netadr_t,
    mut format: *const i8,
    mut args: ...
) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut string: [i8; 16384] = [0; 16384];
    // set the header
    string[0] = -1;
    string[1] = -1;
    string[2] = -1;
    string[3] = -1;
    argptr = args.clone();
    crate::stdlib::vsnprintf(
        string.as_mut_ptr().offset(4),
        (::std::mem::size_of::<[i8; 16384]>()).wrapping_sub(4usize),
        format,
        argptr.as_va_list(),
    );
    // send the datagram
    NET_SendPacket(
        sock,
        crate::stdlib::strlen(string.as_mut_ptr()) as i32,
        string.as_mut_ptr() as *const libc::c_void,
        adr,
    );
}
/*
===============
NET_OutOfBandPrint

Sends a data message in an out-of-band datagram (only used for "connect")
================
*/
#[no_mangle]

pub unsafe extern "C" fn NET_OutOfBandData(
    mut sock: crate::qcommon_h::netsrc_t,
    mut adr: crate::qcommon_h::netadr_t,
    mut format: *mut crate::src::qcommon::q_shared::byte,
    mut len: i32,
) {
    let mut string: [crate::src::qcommon::q_shared::byte; 32768] = [0; 32768];
    let mut i: i32 = 0;
    let mut mbuf: crate::qcommon_h::msg_t = crate::qcommon_h::msg_t {
        allowoverflow: crate::src::qcommon::q_shared::qfalse,
        overflowed: crate::src::qcommon::q_shared::qfalse,
        oob: crate::src::qcommon::q_shared::qfalse,
        data: 0 as *mut crate::src::qcommon::q_shared::byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
        bit: 0,
    };
    // set the header
    string[0] = 0xff;
    string[1] = 0xff;
    string[2] = 0xff;
    string[3] = 0xff;

    for i in 0..len {
        string[(i + 4) as usize] = *format.offset(i as isize);
    }
    mbuf.data = string.as_mut_ptr();
    mbuf.cursize = len + 4;
    crate::src::qcommon::huffman::Huff_Compress(&mut mbuf, 12);
    // send the datagram
    NET_SendPacket(sock, mbuf.cursize, mbuf.data as *const libc::c_void, adr);
}
/*
=============
NET_StringToAdr

Traps "localhost" for loopback, passes everything else to system
return 0 on address not found, 1 on address found with port, 2 on address found without port.
=============
*/
#[no_mangle]

pub unsafe extern "C" fn NET_StringToAdr(
    mut s: *const i8,
    mut a: *mut crate::qcommon_h::netadr_t,
    mut family: crate::qcommon_h::netadrtype_t,
) -> i32 {
    let mut base: [i8; 1024] = [0; 1024];
    let mut search: *mut i8 = 0 as *mut i8;
    let mut port: *mut i8 = 0 as *mut i8;
    if crate::stdlib::strcmp(s, b"localhost\x00" as *const u8 as *const i8) == 0 {
        crate::stdlib::memset(
            a as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::qcommon_h::netadr_t>(),
        );
        (*a).type_0 = crate::qcommon_h::NA_LOOPBACK;
        // as NA_LOOPBACK doesn't require ports report port was given.
        return 1i32;
    }
    crate::src::qcommon::q_shared::Q_strncpyz(
        base.as_mut_ptr(),
        s,
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    if *base.as_mut_ptr() as i32 == '[' as i32
        || crate::src::qcommon::q_shared::Q_CountChar(base.as_mut_ptr(), ':' as i8) > 1
    {
        // This is an ipv6 address, handle it specially.
        search = crate::stdlib::strchr(base.as_mut_ptr(), ']' as i32);
        if !search.is_null() {
            *search = '\u{0}' as i8;
            search = search.offset(1);
            if *search as i32 == ':' as i32 {
                port = search.offset(1)
            }
        }
        if *base.as_mut_ptr() as i32 == '[' as i32 {
            search = base.as_mut_ptr().offset(1)
        } else {
            search = base.as_mut_ptr()
        }
    } else {
        // look for a port number
        port = crate::stdlib::strchr(base.as_mut_ptr(), ':' as i32);
        if !port.is_null() {
            *port = '\u{0}' as i8;
            port = port.offset(1)
        }
        search = base.as_mut_ptr()
    }
    if crate::src::qcommon::net_ip::Sys_StringToAdr(search, a, family) as u64 == 0 {
        (*a).type_0 = crate::qcommon_h::NA_BAD;
        return 0i32;
    }
    if !port.is_null() {
        (*a).port = crate::src::qcommon::q_shared::ShortSwap(atoi(port) as i16) as u16;
        return 1i32;
    } else {
        (*a).port = crate::src::qcommon::q_shared::ShortSwap(27960) as u16;
        return 2i32;
    };
}
