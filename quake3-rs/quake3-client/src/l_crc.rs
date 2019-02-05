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
    use super::{libc};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_crc.h"]
pub mod l_crc_h {
    use super::{libc};
    use super::q_shared_h::{byte};
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/botlib/l_crc.c"]
pub mod l_crc_c {
    use super::{libc};
}
use self::q_shared_h::{byte};
#[no_mangle]
pub unsafe extern "C" fn CRC_Init(mut crcvalue: *mut libc::c_ushort) {
    *crcvalue = 0xffffi32 as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn CRC_ProcessByte(mut crcvalue: *mut libc::c_ushort,
                                         mut data: byte) {
    *crcvalue =
        ((*crcvalue as libc::c_int) << 8i32 ^
             crctable[(*crcvalue as libc::c_int >> 8i32 ^ data as libc::c_int)
                          as usize] as libc::c_int) as libc::c_ushort;
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
 * name:		l_crc.c
 *
 * desc:		CRC calculation
 *
 * $Archive: /MissionPack/CODE/botlib/l_crc.c $
 *
 *****************************************************************************/
// FIXME: byte swap?
// this is a 16 bit, non-reflected CRC using the polynomial 0x1021
// and the initial and final xor values shown below...  in other words, the
// CCITT standard CRC used by XMODEM
#[no_mangle]
pub static mut crctable: [libc::c_ushort; 257] =
    [0i32 as libc::c_ushort, 0x1021i32 as libc::c_ushort,
     0x2042i32 as libc::c_ushort, 0x3063i32 as libc::c_ushort,
     0x4084i32 as libc::c_ushort, 0x50a5i32 as libc::c_ushort,
     0x60c6i32 as libc::c_ushort, 0x70e7i32 as libc::c_ushort,
     0x8108i32 as libc::c_ushort, 0x9129i32 as libc::c_ushort,
     0xa14ai32 as libc::c_ushort, 0xb16bi32 as libc::c_ushort,
     0xc18ci32 as libc::c_ushort, 0xd1adi32 as libc::c_ushort,
     0xe1cei32 as libc::c_ushort, 0xf1efi32 as libc::c_ushort,
     0x1231i32 as libc::c_ushort, 0x210i32 as libc::c_ushort,
     0x3273i32 as libc::c_ushort, 0x2252i32 as libc::c_ushort,
     0x52b5i32 as libc::c_ushort, 0x4294i32 as libc::c_ushort,
     0x72f7i32 as libc::c_ushort, 0x62d6i32 as libc::c_ushort,
     0x9339i32 as libc::c_ushort, 0x8318i32 as libc::c_ushort,
     0xb37bi32 as libc::c_ushort, 0xa35ai32 as libc::c_ushort,
     0xd3bdi32 as libc::c_ushort, 0xc39ci32 as libc::c_ushort,
     0xf3ffi32 as libc::c_ushort, 0xe3dei32 as libc::c_ushort,
     0x2462i32 as libc::c_ushort, 0x3443i32 as libc::c_ushort,
     0x420i32 as libc::c_ushort, 0x1401i32 as libc::c_ushort,
     0x64e6i32 as libc::c_ushort, 0x74c7i32 as libc::c_ushort,
     0x44a4i32 as libc::c_ushort, 0x5485i32 as libc::c_ushort,
     0xa56ai32 as libc::c_ushort, 0xb54bi32 as libc::c_ushort,
     0x8528i32 as libc::c_ushort, 0x9509i32 as libc::c_ushort,
     0xe5eei32 as libc::c_ushort, 0xf5cfi32 as libc::c_ushort,
     0xc5aci32 as libc::c_ushort, 0xd58di32 as libc::c_ushort,
     0x3653i32 as libc::c_ushort, 0x2672i32 as libc::c_ushort,
     0x1611i32 as libc::c_ushort, 0x630i32 as libc::c_ushort,
     0x76d7i32 as libc::c_ushort, 0x66f6i32 as libc::c_ushort,
     0x5695i32 as libc::c_ushort, 0x46b4i32 as libc::c_ushort,
     0xb75bi32 as libc::c_ushort, 0xa77ai32 as libc::c_ushort,
     0x9719i32 as libc::c_ushort, 0x8738i32 as libc::c_ushort,
     0xf7dfi32 as libc::c_ushort, 0xe7fei32 as libc::c_ushort,
     0xd79di32 as libc::c_ushort, 0xc7bci32 as libc::c_ushort,
     0x48c4i32 as libc::c_ushort, 0x58e5i32 as libc::c_ushort,
     0x6886i32 as libc::c_ushort, 0x78a7i32 as libc::c_ushort,
     0x840i32 as libc::c_ushort, 0x1861i32 as libc::c_ushort,
     0x2802i32 as libc::c_ushort, 0x3823i32 as libc::c_ushort,
     0xc9cci32 as libc::c_ushort, 0xd9edi32 as libc::c_ushort,
     0xe98ei32 as libc::c_ushort, 0xf9afi32 as libc::c_ushort,
     0x8948i32 as libc::c_ushort, 0x9969i32 as libc::c_ushort,
     0xa90ai32 as libc::c_ushort, 0xb92bi32 as libc::c_ushort,
     0x5af5i32 as libc::c_ushort, 0x4ad4i32 as libc::c_ushort,
     0x7ab7i32 as libc::c_ushort, 0x6a96i32 as libc::c_ushort,
     0x1a71i32 as libc::c_ushort, 0xa50i32 as libc::c_ushort,
     0x3a33i32 as libc::c_ushort, 0x2a12i32 as libc::c_ushort,
     0xdbfdi32 as libc::c_ushort, 0xcbdci32 as libc::c_ushort,
     0xfbbfi32 as libc::c_ushort, 0xeb9ei32 as libc::c_ushort,
     0x9b79i32 as libc::c_ushort, 0x8b58i32 as libc::c_ushort,
     0xbb3bi32 as libc::c_ushort, 0xab1ai32 as libc::c_ushort,
     0x6ca6i32 as libc::c_ushort, 0x7c87i32 as libc::c_ushort,
     0x4ce4i32 as libc::c_ushort, 0x5cc5i32 as libc::c_ushort,
     0x2c22i32 as libc::c_ushort, 0x3c03i32 as libc::c_ushort,
     0xc60i32 as libc::c_ushort, 0x1c41i32 as libc::c_ushort,
     0xedaei32 as libc::c_ushort, 0xfd8fi32 as libc::c_ushort,
     0xcdeci32 as libc::c_ushort, 0xddcdi32 as libc::c_ushort,
     0xad2ai32 as libc::c_ushort, 0xbd0bi32 as libc::c_ushort,
     0x8d68i32 as libc::c_ushort, 0x9d49i32 as libc::c_ushort,
     0x7e97i32 as libc::c_ushort, 0x6eb6i32 as libc::c_ushort,
     0x5ed5i32 as libc::c_ushort, 0x4ef4i32 as libc::c_ushort,
     0x3e13i32 as libc::c_ushort, 0x2e32i32 as libc::c_ushort,
     0x1e51i32 as libc::c_ushort, 0xe70i32 as libc::c_ushort,
     0xff9fi32 as libc::c_ushort, 0xefbei32 as libc::c_ushort,
     0xdfddi32 as libc::c_ushort, 0xcffci32 as libc::c_ushort,
     0xbf1bi32 as libc::c_ushort, 0xaf3ai32 as libc::c_ushort,
     0x9f59i32 as libc::c_ushort, 0x8f78i32 as libc::c_ushort,
     0x9188i32 as libc::c_ushort, 0x81a9i32 as libc::c_ushort,
     0xb1cai32 as libc::c_ushort, 0xa1ebi32 as libc::c_ushort,
     0xd10ci32 as libc::c_ushort, 0xc12di32 as libc::c_ushort,
     0xf14ei32 as libc::c_ushort, 0xe16fi32 as libc::c_ushort,
     0x1080i32 as libc::c_ushort, 0xa1i32 as libc::c_ushort,
     0x30c2i32 as libc::c_ushort, 0x20e3i32 as libc::c_ushort,
     0x5004i32 as libc::c_ushort, 0x4025i32 as libc::c_ushort,
     0x7046i32 as libc::c_ushort, 0x6067i32 as libc::c_ushort,
     0x83b9i32 as libc::c_ushort, 0x9398i32 as libc::c_ushort,
     0xa3fbi32 as libc::c_ushort, 0xb3dai32 as libc::c_ushort,
     0xc33di32 as libc::c_ushort, 0xd31ci32 as libc::c_ushort,
     0xe37fi32 as libc::c_ushort, 0xf35ei32 as libc::c_ushort,
     0x2b1i32 as libc::c_ushort, 0x1290i32 as libc::c_ushort,
     0x22f3i32 as libc::c_ushort, 0x32d2i32 as libc::c_ushort,
     0x4235i32 as libc::c_ushort, 0x5214i32 as libc::c_ushort,
     0x6277i32 as libc::c_ushort, 0x7256i32 as libc::c_ushort,
     0xb5eai32 as libc::c_ushort, 0xa5cbi32 as libc::c_ushort,
     0x95a8i32 as libc::c_ushort, 0x8589i32 as libc::c_ushort,
     0xf56ei32 as libc::c_ushort, 0xe54fi32 as libc::c_ushort,
     0xd52ci32 as libc::c_ushort, 0xc50di32 as libc::c_ushort,
     0x34e2i32 as libc::c_ushort, 0x24c3i32 as libc::c_ushort,
     0x14a0i32 as libc::c_ushort, 0x481i32 as libc::c_ushort,
     0x7466i32 as libc::c_ushort, 0x6447i32 as libc::c_ushort,
     0x5424i32 as libc::c_ushort, 0x4405i32 as libc::c_ushort,
     0xa7dbi32 as libc::c_ushort, 0xb7fai32 as libc::c_ushort,
     0x8799i32 as libc::c_ushort, 0x97b8i32 as libc::c_ushort,
     0xe75fi32 as libc::c_ushort, 0xf77ei32 as libc::c_ushort,
     0xc71di32 as libc::c_ushort, 0xd73ci32 as libc::c_ushort,
     0x26d3i32 as libc::c_ushort, 0x36f2i32 as libc::c_ushort,
     0x691i32 as libc::c_ushort, 0x16b0i32 as libc::c_ushort,
     0x6657i32 as libc::c_ushort, 0x7676i32 as libc::c_ushort,
     0x4615i32 as libc::c_ushort, 0x5634i32 as libc::c_ushort,
     0xd94ci32 as libc::c_ushort, 0xc96di32 as libc::c_ushort,
     0xf90ei32 as libc::c_ushort, 0xe92fi32 as libc::c_ushort,
     0x99c8i32 as libc::c_ushort, 0x89e9i32 as libc::c_ushort,
     0xb98ai32 as libc::c_ushort, 0xa9abi32 as libc::c_ushort,
     0x5844i32 as libc::c_ushort, 0x4865i32 as libc::c_ushort,
     0x7806i32 as libc::c_ushort, 0x6827i32 as libc::c_ushort,
     0x18c0i32 as libc::c_ushort, 0x8e1i32 as libc::c_ushort,
     0x3882i32 as libc::c_ushort, 0x28a3i32 as libc::c_ushort,
     0xcb7di32 as libc::c_ushort, 0xdb5ci32 as libc::c_ushort,
     0xeb3fi32 as libc::c_ushort, 0xfb1ei32 as libc::c_ushort,
     0x8bf9i32 as libc::c_ushort, 0x9bd8i32 as libc::c_ushort,
     0xabbbi32 as libc::c_ushort, 0xbb9ai32 as libc::c_ushort,
     0x4a75i32 as libc::c_ushort, 0x5a54i32 as libc::c_ushort,
     0x6a37i32 as libc::c_ushort, 0x7a16i32 as libc::c_ushort,
     0xaf1i32 as libc::c_ushort, 0x1ad0i32 as libc::c_ushort,
     0x2ab3i32 as libc::c_ushort, 0x3a92i32 as libc::c_ushort,
     0xfd2ei32 as libc::c_ushort, 0xed0fi32 as libc::c_ushort,
     0xdd6ci32 as libc::c_ushort, 0xcd4di32 as libc::c_ushort,
     0xbdaai32 as libc::c_ushort, 0xad8bi32 as libc::c_ushort,
     0x9de8i32 as libc::c_ushort, 0x8dc9i32 as libc::c_ushort,
     0x7c26i32 as libc::c_ushort, 0x6c07i32 as libc::c_ushort,
     0x5c64i32 as libc::c_ushort, 0x4c45i32 as libc::c_ushort,
     0x3ca2i32 as libc::c_ushort, 0x2c83i32 as libc::c_ushort,
     0x1ce0i32 as libc::c_ushort, 0xcc1i32 as libc::c_ushort,
     0xef1fi32 as libc::c_ushort, 0xff3ei32 as libc::c_ushort,
     0xcf5di32 as libc::c_ushort, 0xdf7ci32 as libc::c_ushort,
     0xaf9bi32 as libc::c_ushort, 0xbfbai32 as libc::c_ushort,
     0x8fd9i32 as libc::c_ushort, 0x9ff8i32 as libc::c_ushort,
     0x6e17i32 as libc::c_ushort, 0x7e36i32 as libc::c_ushort,
     0x4e55i32 as libc::c_ushort, 0x5e74i32 as libc::c_ushort,
     0x2e93i32 as libc::c_ushort, 0x3eb2i32 as libc::c_ushort,
     0xed1i32 as libc::c_ushort, 0x1ef0i32 as libc::c_ushort, 0];
#[no_mangle]
pub unsafe extern "C" fn CRC_Value(mut crcvalue: libc::c_ushort)
 -> libc::c_ushort {
    return (crcvalue as libc::c_int ^ 0i32) as libc::c_ushort;
}
#[no_mangle]
pub unsafe extern "C" fn CRC_ProcessString(mut data: *mut libc::c_uchar,
                                           mut length: libc::c_int)
 -> libc::c_ushort {
    let mut crcvalue: libc::c_ushort = 0;
    let mut i: libc::c_int = 0;
    let mut ind: libc::c_int = 0;
    CRC_Init(&mut crcvalue);
    i = 0i32;
    while i < length {
        ind =
            crcvalue as libc::c_int >> 8i32 ^
                *data.offset(i as isize) as libc::c_int;
        if ind < 0i32 || ind > 256i32 { ind = 0i32 }
        crcvalue =
            ((crcvalue as libc::c_int) << 8i32 ^
                 crctable[ind as usize] as libc::c_int) as libc::c_ushort;
        i += 1
    }
    return CRC_Value(crcvalue);
}
#[no_mangle]
pub unsafe extern "C" fn CRC_ContinueProcessString(mut crc:
                                                       *mut libc::c_ushort,
                                                   mut data:
                                                       *mut libc::c_char,
                                                   mut length: libc::c_int) {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < length {
        *crc =
            ((*crc as libc::c_int) << 8i32 ^
                 crctable[(*crc as libc::c_int >> 8i32 ^
                               *data.offset(i as isize) as libc::c_int) as
                              usize] as libc::c_int) as libc::c_ushort;
        i += 1
    };
}