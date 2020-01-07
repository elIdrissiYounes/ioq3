use ::libc;

pub use crate::opus_private_h::ChannelLayout;
/* Copyright (c) 2011 Xiph.Org Foundation
Written by Jean-Marc Valin */
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
#[no_mangle]

pub unsafe extern "C" fn validate_layout(
    mut layout: *const crate::opus_private_h::ChannelLayout,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut max_channel: libc::c_int = 0;
    max_channel = (*layout).nb_streams + (*layout).nb_coupled_streams;
    if max_channel > 255 as libc::c_int {
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as libc::c_int >= max_channel
            && (*layout).mapping[i as usize] as libc::c_int != 255 as libc::c_int
        {
            return 0 as libc::c_int;
        }
        i += 1
    }
    return 1 as libc::c_int;
}
#[no_mangle]

pub unsafe extern "C" fn get_left_channel(
    mut layout: *const crate::opus_private_h::ChannelLayout,
    mut stream_id: libc::c_int,
    mut prev: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = if prev < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (prev) + 1 as libc::c_int
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as libc::c_int == stream_id * 2 as libc::c_int {
            return i;
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn get_right_channel(
    mut layout: *const crate::opus_private_h::ChannelLayout,
    mut stream_id: libc::c_int,
    mut prev: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = if prev < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (prev) + 1 as libc::c_int
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as libc::c_int
            == stream_id * 2 as libc::c_int + 1 as libc::c_int
        {
            return i;
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
#[no_mangle]

pub unsafe extern "C" fn get_mono_channel(
    mut layout: *const crate::opus_private_h::ChannelLayout,
    mut stream_id: libc::c_int,
    mut prev: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = if prev < 0 as libc::c_int {
        0 as libc::c_int
    } else {
        (prev) + 1 as libc::c_int
    };
    while i < (*layout).nb_channels {
        if (*layout).mapping[i as usize] as libc::c_int == stream_id + (*layout).nb_coupled_streams
        {
            return i;
        }
        i += 1
    }
    return -(1 as libc::c_int);
}
