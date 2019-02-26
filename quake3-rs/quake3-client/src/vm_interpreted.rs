#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_mut)]
#![feature(custom_attribute, libc)]
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
#[header_src = "/usr/include/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint8_t};
}
#[header_src = "/usr/include/stdint.h"]
pub mod stdint_h {
    pub type intptr_t = libc::c_long;
    use super::{libc};
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
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn Hunk_AllocDebug(size: libc::c_int, preference: ha_pref,
                               label: *mut libc::c_char,
                               file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn qftolsse(f: libc::c_float) -> libc::c_long;
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src =
      "ioq3/code/qcommon/qfiles.h"]
pub mod qfiles_h {
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
// qfiles.h: quake file formats
// This file must be identical in the quake and utils directories
//
    //Ignore __attribute__ on non-gcc platforms
    // surface geometry should not exceed these limits
    // the maximum size of game relative pathnames
    /*
========================================================================

QVM files

========================================================================
*/
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vmHeader_t {
        pub vmMagic: libc::c_int,
        pub instructionCount: libc::c_int,
        pub codeOffset: libc::c_int,
        pub codeLength: libc::c_int,
        pub dataOffset: libc::c_int,
        pub dataLength: libc::c_int,
        pub litLength: libc::c_int,
        pub bssLength: libc::c_int,
        pub jtrgLength: libc::c_int,
    }
    use super::{libc};
}
#[header_src =
      "ioq3/code/qcommon/vm_local.h"]
pub mod vm_local_h {
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vm_s {
        pub programStack: libc::c_int,
        pub systemCall: Option<unsafe extern "C" fn(_: *mut intptr_t)
                                   -> intptr_t>,
        pub name: [libc::c_char; 64],
        pub searchPath: *mut libc::c_void,
        pub dllHandle: *mut libc::c_void,
        pub entryPoint: Option<unsafe extern "C" fn(_: libc::c_int, ...)
                                   -> intptr_t>,
        pub destroy: Option<unsafe extern "C" fn(_: *mut vm_t) -> ()>,
        pub currentlyInterpreting: qboolean,
        pub compiled: qboolean,
        pub codeBase: *mut byte,
        pub entryOfs: libc::c_int,
        pub codeLength: libc::c_int,
        pub instructionPointers: *mut intptr_t,
        pub instructionCount: libc::c_int,
        pub dataBase: *mut byte,
        pub dataMask: libc::c_int,
        pub dataAlloc: libc::c_int,
        pub stackBottom: libc::c_int,
        pub numSymbols: libc::c_int,
        pub symbols: *mut vmSymbol_s,
        pub callLevel: libc::c_int,
        pub breakFunction: libc::c_int,
        pub breakCount: libc::c_int,
        pub jumpTableTargets: *mut byte,
        pub numJumpTableTargets: libc::c_int,
    }
    #[derive
    ( Copy , Clone )]
    #[repr(C)]
    pub struct vmSymbol_s {
        pub next: *mut vmSymbol_s,
        pub symValue: libc::c_int,
        pub profileCount: libc::c_int,
        pub symName: [libc::c_char; 0],
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
    // Max number of arguments to pass from engine to vm's vmMain function.
// command number + 12 arguments
    // Max number of arguments to pass from a vm to engine's syscall handler function for the vm.
// syscall number + 15 arguments
    // don't change, this is hardcoded into x86 VMs, opStack protection relies
// on this
    // don't change
// Hardcoded in q3asm a reserved at end of bss
    pub type unnamed_0 = libc::c_uint;
    pub const OP_CVFI: unnamed_0 = 59;
    pub const OP_CVIF: unnamed_0 = 58;
    pub const OP_MULF: unnamed_0 = 57;
    pub const OP_DIVF: unnamed_0 = 56;
    pub const OP_SUBF: unnamed_0 = 55;
    pub const OP_ADDF: unnamed_0 = 54;
    pub const OP_NEGF: unnamed_0 = 53;
    pub const OP_RSHU: unnamed_0 = 52;
    pub const OP_RSHI: unnamed_0 = 51;
    pub const OP_LSH: unnamed_0 = 50;
    pub const OP_BCOM: unnamed_0 = 49;
    pub const OP_BXOR: unnamed_0 = 48;
    pub const OP_BOR: unnamed_0 = 47;
    pub const OP_BAND: unnamed_0 = 46;
    pub const OP_MULU: unnamed_0 = 45;
    pub const OP_MULI: unnamed_0 = 44;
    pub const OP_MODU: unnamed_0 = 43;
    pub const OP_MODI: unnamed_0 = 42;
    pub const OP_DIVU: unnamed_0 = 41;
    pub const OP_DIVI: unnamed_0 = 40;
    pub const OP_SUB: unnamed_0 = 39;
    pub const OP_ADD: unnamed_0 = 38;
    pub const OP_NEGI: unnamed_0 = 37;
    pub const OP_SEX16: unnamed_0 = 36;
    //-------------------
    pub const OP_SEX8: unnamed_0 = 35;
    pub const OP_BLOCK_COPY: unnamed_0 = 34;
    pub const OP_ARG: unnamed_0 = 33;
    // *(stack[top-1]) = stack[top]
    pub const OP_STORE4: unnamed_0 = 32;
    pub const OP_STORE2: unnamed_0 = 31;
    pub const OP_STORE1: unnamed_0 = 30;
    pub const OP_LOAD4: unnamed_0 = 29;
    pub const OP_LOAD2: unnamed_0 = 28;
    //-------------------
    pub const OP_LOAD1: unnamed_0 = 27;
    pub const OP_GEF: unnamed_0 = 26;
    pub const OP_GTF: unnamed_0 = 25;
    pub const OP_LEF: unnamed_0 = 24;
    pub const OP_LTF: unnamed_0 = 23;
    pub const OP_NEF: unnamed_0 = 22;
    pub const OP_EQF: unnamed_0 = 21;
    pub const OP_GEU: unnamed_0 = 20;
    pub const OP_GTU: unnamed_0 = 19;
    pub const OP_LEU: unnamed_0 = 18;
    pub const OP_LTU: unnamed_0 = 17;
    pub const OP_GEI: unnamed_0 = 16;
    pub const OP_GTI: unnamed_0 = 15;
    pub const OP_LEI: unnamed_0 = 14;
    pub const OP_LTI: unnamed_0 = 13;
    pub const OP_NE: unnamed_0 = 12;
    //-------------------
    pub const OP_EQ: unnamed_0 = 11;
    pub const OP_JUMP: unnamed_0 = 10;
    pub const OP_LOCAL: unnamed_0 = 9;
    pub const OP_CONST: unnamed_0 = 8;
    pub const OP_POP: unnamed_0 = 7;
    pub const OP_PUSH: unnamed_0 = 6;
    pub const OP_CALL: unnamed_0 = 5;
    pub const OP_LEAVE: unnamed_0 = 4;
    pub const OP_ENTER: unnamed_0 = 3;
    pub const OP_BREAK: unnamed_0 = 2;
    pub const OP_IGNORE: unnamed_0 = 1;
    pub const OP_UNDEF: unnamed_0 = 0;
    use super::{libc};
    use super::stdint_h::{intptr_t};
    use super::qcommon_h::{vm_t};
    use super::q_shared_h::{qboolean, byte};
    use super::qfiles_h::{vmHeader_t};
    use super::stddef_h::{size_t};
    extern "C" {
        #[no_mangle]
        pub fn VM_BlockCopy(dest: libc::c_uint, src: libc::c_uint, n: size_t);
        #[no_mangle]
        pub fn VM_ValueToSymbol(vm: *mut vm_t, value: libc::c_int)
         -> *const libc::c_char;
    }
}
#[header_src =
      "ioq3/code/qcommon/qcommon.h"]
pub mod qcommon_h {
    /*
==============================================================

VIRTUAL MACHINE

==============================================================
*/
    pub type vm_t = vm_s;
    use super::vm_local_h::{vm_s};
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn VM_Debug(level: libc::c_int);
    }
}
#[header_src = "/usr/include/string.h"]
pub mod string_h {
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub fn memcpy(_: *mut libc::c_void, _: *const libc::c_void,
                      _: libc::c_ulong) -> *mut libc::c_void;
    }
}
#[header_src =
      "ioq3/code/qcommon/vm_interpreted.c"]
pub mod vm_interpreted_c {
    use super::{libc};
    use super::qcommon_h::{vm_t};
}
use self::types_h::{__uint8_t};
use self::stddef_h::{size_t};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, unnamed, ERR_NEED_CD,
                       ERR_DISCONNECT, ERR_SERVERDISCONNECT, ERR_DROP,
                       ERR_FATAL, ha_pref, h_dontcare, h_low, h_high,
                       Hunk_AllocDebug, qftolsse, Com_Error, Com_Printf};
use self::qfiles_h::{vmHeader_t};
use self::vm_local_h::{vm_s, vmSymbol_s, unnamed_0, OP_CVFI, OP_CVIF, OP_MULF,
                       OP_DIVF, OP_SUBF, OP_ADDF, OP_NEGF, OP_RSHU, OP_RSHI,
                       OP_LSH, OP_BCOM, OP_BXOR, OP_BOR, OP_BAND, OP_MULU,
                       OP_MULI, OP_MODU, OP_MODI, OP_DIVU, OP_DIVI, OP_SUB,
                       OP_ADD, OP_NEGI, OP_SEX16, OP_SEX8, OP_BLOCK_COPY,
                       OP_ARG, OP_STORE4, OP_STORE2, OP_STORE1, OP_LOAD4,
                       OP_LOAD2, OP_LOAD1, OP_GEF, OP_GTF, OP_LEF, OP_LTF,
                       OP_NEF, OP_EQF, OP_GEU, OP_GTU, OP_LEU, OP_LTU, OP_GEI,
                       OP_GTI, OP_LEI, OP_LTI, OP_NE, OP_EQ, OP_JUMP,
                       OP_LOCAL, OP_CONST, OP_POP, OP_PUSH, OP_CALL, OP_LEAVE,
                       OP_ENTER, OP_BREAK, OP_IGNORE, OP_UNDEF, VM_BlockCopy,
                       VM_ValueToSymbol};
use self::qcommon_h::{vm_t, VM_Debug};
use self::string_h::{memcpy};
#[no_mangle]
pub unsafe extern "C" fn VM_PrepareInterpreter(mut vm: *mut vm_t,
                                               mut header: *mut vmHeader_t) {
    let mut op: libc::c_int = 0;
    let mut byte_pc: libc::c_int = 0;
    let mut int_pc: libc::c_int = 0;
    let mut code: *mut byte = 0 as *mut byte;
    let mut instruction: libc::c_int = 0;
    let mut codeBase: *mut libc::c_int = 0 as *mut libc::c_int;
    (*vm).codeBase =
        Hunk_AllocDebug((*vm).codeLength * 4i32, h_high,
                        b"vm->codeLength*4\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char,
                        b"code/qcommon/vm_interpreted.c\x00" as *const u8 as
                            *const libc::c_char as *mut libc::c_char, 178i32)
            as *mut byte;
    byte_pc = 0i32;
    int_pc = byte_pc;
    instruction = 0i32;
    code = (header as *mut byte).offset((*header).codeOffset as isize);
    codeBase = (*vm).codeBase as *mut libc::c_int;
    while instruction < (*header).instructionCount {
        *(*vm).instructionPointers.offset(instruction as isize) =
            int_pc as intptr_t;
        instruction += 1;
        op = *code.offset(byte_pc as isize) as libc::c_int;
        *codeBase.offset(int_pc as isize) = op;
        if byte_pc > (*header).codeLength {
            Com_Error(ERR_DROP as libc::c_int,
                      b"VM_PrepareInterpreter: pc > header->codeLength\x00" as
                          *const u8 as *const libc::c_char);
        }
        byte_pc += 1;
        int_pc += 1;
        match op {
            3 | 8 | 9 | 4 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 |
            21 | 22 | 23 | 24 | 25 | 26 | 34 => {
                *codeBase.offset(int_pc as isize) =
                    loadWord(&mut *code.offset(byte_pc as isize) as *mut byte
                                 as *mut libc::c_void);
                byte_pc += 4i32;
                int_pc += 1
            }
            33 => {
                *codeBase.offset(int_pc as isize) =
                    *code.offset(byte_pc as isize) as libc::c_int;
                byte_pc += 1;
                int_pc += 1
            }
            _ => { }
        }
    }
    int_pc = 0i32;
    instruction = 0i32;
    while instruction < (*header).instructionCount {
        op = *codeBase.offset(int_pc as isize);
        instruction += 1;
        int_pc += 1;
        match op {
            11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 |
            24 | 25 | 26 => {
                if *codeBase.offset(int_pc as isize) < 0i32 ||
                       *codeBase.offset(int_pc as isize) >
                           (*vm).instructionCount {
                    Com_Error(ERR_DROP as libc::c_int,
                              b"VM_PrepareInterpreter: Jump to invalid instruction number\x00"
                                  as *const u8 as *const libc::c_char);
                }
                *codeBase.offset(int_pc as isize) =
                    *(*vm).instructionPointers.offset(*codeBase.offset(int_pc
                                                                           as
                                                                           isize)
                                                          as isize) as
                        libc::c_int;
                int_pc += 1
            }
            3 | 8 | 9 | 4 | 34 | 33 => { int_pc += 1 }
            _ => { }
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
//#define	DEBUG_VM
unsafe extern "C" fn loadWord(mut addr: *mut libc::c_void) -> libc::c_int {
    let mut word: libc::c_int = 0;
    memcpy(&mut word as *mut libc::c_int as *mut libc::c_void, addr,
           4i32 as libc::c_ulong);
    return word;
}
#[no_mangle]
pub unsafe extern "C" fn VM_CallInterpreted(mut vm: *mut vm_t,
                                            mut args: *mut libc::c_int)
 -> libc::c_int {
    let mut current_block: u64;
    let mut stack: [byte; 1039] = [0; 1039];
    let mut opStack: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut opStackOfs: uint8_t = 0;
    let mut programCounter: libc::c_int = 0;
    let mut programStack: libc::c_int = 0;
    let mut stackOnEntry: libc::c_int = 0;
    let mut image: *mut byte = 0 as *mut byte;
    let mut codeImage: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut v1: libc::c_int = 0;
    let mut dataMask: libc::c_int = 0;
    let mut arg: libc::c_int = 0;
    (*vm).currentlyInterpreting = qtrue;
    stackOnEntry = (*vm).programStack;
    programStack = stackOnEntry;
    image = (*vm).dataBase;
    codeImage = (*vm).codeBase as *mut libc::c_int;
    dataMask = (*vm).dataMask;
    programCounter = 0i32;
    programStack -= 8i32 + 4i32 * 13i32;
    arg = 0i32;
    while arg < 13i32 {
        *(&mut *image.offset((programStack + 8i32 + arg * 4i32) as isize) as
              *mut byte as *mut libc::c_int) = *args.offset(arg as isize);
        arg += 1
    }
    *(&mut *image.offset((programStack + 4i32) as isize) as *mut byte as
          *mut libc::c_int) = 0i32;
    *(&mut *image.offset(programStack as isize) as *mut byte as
          *mut libc::c_int) = -1i32;
    VM_Debug(0i32);
    opStack =
        (stack.as_mut_ptr() as intptr_t + 16i32 as libc::c_long -
             1i32 as libc::c_long & !(16i32 - 1i32) as libc::c_long) as
            *mut libc::c_void as *mut libc::c_int;
    *opStack = 0xdeadbeefu32 as libc::c_int;
    opStackOfs = 0i32 as uint8_t;
    //	vm_debugLevel=2;
	// main interpreter loop, will exit when a LEAVE instruction
	// grabs the -1 program counter
    's_105:
        loop  {
            let mut opcode: libc::c_int = 0;
            let mut r0: libc::c_int = 0;
            let mut r1: libc::c_int = 0;
            //		unsigned int	r2;
            'c_6530:
                loop  {
                    r0 = *opStack.offset(opStackOfs as isize);
                    r1 =
                        *opStack.offset((opStackOfs as libc::c_int - 1i32) as
                                            uint8_t as isize);
                    loop  {
                        let fresh0 = programCounter;
                        programCounter = programCounter + 1;
                        opcode = *codeImage.offset(fresh0 as isize);
                        match opcode {
                            2 => { (*vm).breakCount += 1 }
                            8 => {
                                opStackOfs = opStackOfs.wrapping_add(1);
                                r1 = r0;
                                let ref mut fresh1 =
                                    *opStack.offset(opStackOfs as isize);
                                *fresh1 =
                                    *codeImage.offset(programCounter as
                                                          isize);
                                r0 = *fresh1;
                                programCounter += 1i32
                            }
                            9 => {
                                opStackOfs = opStackOfs.wrapping_add(1);
                                r1 = r0;
                                let ref mut fresh2 =
                                    *opStack.offset(opStackOfs as isize);
                                *fresh2 =
                                    *codeImage.offset(programCounter as isize)
                                        + programStack;
                                r0 = *fresh2;
                                programCounter += 1i32
                            }
                            29 => {
                                let ref mut fresh3 =
                                    *opStack.offset(opStackOfs as isize);
                                *fresh3 =
                                    *(&mut *image.offset((r0 & dataMask) as
                                                             isize) as
                                          *mut byte as *mut libc::c_int);
                                r0 = *fresh3
                            }
                            28 => {
                                let ref mut fresh4 =
                                    *opStack.offset(opStackOfs as isize);
                                *fresh4 =
                                    *(&mut *image.offset((r0 & dataMask) as
                                                             isize) as
                                          *mut byte as *mut libc::c_ushort) as
                                        libc::c_int;
                                r0 = *fresh4
                            }
                            27 => {
                                let ref mut fresh5 =
                                    *opStack.offset(opStackOfs as isize);
                                *fresh5 =
                                    *image.offset((r0 & dataMask) as isize) as
                                        libc::c_int;
                                r0 = *fresh5
                            }
                            32 => {
                                *(&mut *image.offset((r1 & dataMask) as isize)
                                      as *mut byte as *mut libc::c_int) = r0;
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                continue 'c_6530 ;
                            }
                            31 => {
                                *(&mut *image.offset((r1 & dataMask) as isize)
                                      as *mut byte as *mut libc::c_short) =
                                    r0 as libc::c_short;
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                continue 'c_6530 ;
                            }
                            30 => {
                                *image.offset((r1 & dataMask) as isize) =
                                    r0 as byte;
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                continue 'c_6530 ;
                            }
                            33 => {
                                *(&mut *image.offset((*codeImage.offset(programCounter
                                                                            as
                                                                            isize)
                                                          + programStack &
                                                          dataMask) as isize)
                                      as *mut byte as *mut libc::c_int) = r0;
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                programCounter += 1i32;
                                continue 'c_6530 ;
                            }
                            34 => {
                                VM_BlockCopy(r1 as libc::c_uint,
                                             r0 as libc::c_uint,
                                             *codeImage.offset(programCounter
                                                                   as isize)
                                                 as size_t);
                                programCounter += 1i32;
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                continue 'c_6530 ;
                            }
                            5 => {
                                *(&mut *image.offset(programStack as isize) as
                                      *mut byte as *mut libc::c_int) =
                                    programCounter;
                                programCounter = r0;
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                if programCounter < 0i32 {
                                    let mut r: libc::c_int = 0;
                                    (*vm).programStack = programStack - 4i32;
                                    *(&mut *image.offset((programStack + 4i32)
                                                             as isize) as
                                          *mut byte as *mut libc::c_int) =
                                        -1i32 - programCounter;
                                    if ::std::mem::size_of::<intptr_t>() as
                                           libc::c_ulong !=
                                           ::std::mem::size_of::<libc::c_int>()
                                               as libc::c_ulong {
                                        let mut argarr: [intptr_t; 16] =
                                            [0; 16];
                                        let mut imagePtr: *mut libc::c_int =
                                            &mut *image.offset(programStack as
                                                                   isize) as
                                                *mut byte as *mut libc::c_int;
                                        let mut i: libc::c_int = 0;
                                        i = 0i32;
                                        while (i as libc::c_ulong) <
                                                  (::std::mem::size_of::<[intptr_t; 16]>()
                                                       as
                                                       libc::c_ulong).wrapping_div(::std::mem::size_of::<intptr_t>()
                                                                                       as
                                                                                       libc::c_ulong)
                                              {
                                            imagePtr =
                                                imagePtr.offset(1isize);
                                            argarr[i as usize] =
                                                *imagePtr as intptr_t;
                                            i += 1
                                        }
                                        r =
                                            (*vm).systemCall.expect("non-null function pointer")(argarr.as_mut_ptr())
                                                as libc::c_int
                                    } else {
                                        let mut argptr: *mut intptr_t =
                                            &mut *image.offset((programStack +
                                                                    4i32) as
                                                                   isize) as
                                                *mut byte as *mut intptr_t;
                                        r =
                                            (*vm).systemCall.expect("non-null function pointer")(argptr)
                                                as libc::c_int
                                    }
                                    opStackOfs = opStackOfs.wrapping_add(1);
                                    *opStack.offset(opStackOfs as isize) = r;
                                    programCounter =
                                        *(&mut *image.offset(programStack as
                                                                 isize) as
                                              *mut byte as *mut libc::c_int)
                                } else if programCounter as libc::c_uint >=
                                              (*vm).instructionCount as
                                                  libc::c_uint {
                                    Com_Error(ERR_DROP as libc::c_int,
                                              b"VM program counter out of range in OP_CALL\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                } else {
                                    programCounter =
                                        *(*vm).instructionPointers.offset(programCounter
                                                                              as
                                                                              isize)
                                            as libc::c_int
                                }
                                continue 'c_6530 ;
                            }
                            6 => {
                                opStackOfs = opStackOfs.wrapping_add(1);
                                continue 'c_6530 ;
                            }
                            7 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                continue 'c_6530 ;
                            }
                            3 => {
                                v1 =
                                    *codeImage.offset(programCounter as
                                                          isize);
                                programCounter += 1i32;
                                programStack -= v1;
                                continue 'c_6530 ;
                            }
                            4 => {
                                v1 =
                                    *codeImage.offset(programCounter as
                                                          isize);
                                programStack += v1;
                                programCounter =
                                    *(&mut *image.offset(programStack as
                                                             isize) as
                                          *mut byte as *mut libc::c_int);
                                // check for leaving the VM
                                if programCounter == -1i32 { break 's_105 ; }
                                if programCounter as libc::c_uint >=
                                       (*vm).codeLength as libc::c_uint {
                                    Com_Error(ERR_DROP as libc::c_int,
                                              b"VM program counter out of range in OP_LEAVE\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                }
                                continue 'c_6530 ;
                            }
                            10 => {
                                if r0 as libc::c_uint >=
                                       (*vm).instructionCount as libc::c_uint
                                   {
                                    Com_Error(ERR_DROP as libc::c_int,
                                              b"VM program counter out of range in OP_JUMP\x00"
                                                  as *const u8 as
                                                  *const libc::c_char);
                                }
                                programCounter =
                                    *(*vm).instructionPointers.offset(r0 as
                                                                          isize)
                                        as libc::c_int;
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                continue 'c_6530 ;
                            }
                            11 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if r1 == r0 {
                                    current_block = 4691324637564808323;
                                    break ;
                                } else {
                                    current_block = 562309032768341766;
                                    break ;
                                }
                            }
                            12 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if r1 != r0 {
                                    current_block = 3812947724376655173;
                                    break ;
                                } else {
                                    current_block = 3575278370434307847;
                                    break ;
                                }
                            }
                            13 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if r1 < r0 {
                                    current_block = 2522825242109451841;
                                    break ;
                                } else {
                                    current_block = 8304106758420804164;
                                    break ;
                                }
                            }
                            14 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if r1 <= r0 {
                                    current_block = 8533724845731836612;
                                    break ;
                                } else {
                                    current_block = 16667286137552459707;
                                    break ;
                                }
                            }
                            15 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if r1 > r0 {
                                    current_block = 7728257318064351663;
                                    break ;
                                } else {
                                    current_block = 18039443766442739006;
                                    break ;
                                }
                            }
                            16 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if r1 >= r0 {
                                    current_block = 5590933039760577279;
                                    break ;
                                } else {
                                    current_block = 10357520176418200368;
                                    break ;
                                }
                            }
                            17 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if (r1 as libc::c_uint) < r0 as libc::c_uint {
                                    current_block = 12608488225262500095;
                                    break ;
                                } else {
                                    current_block = 14114759727632161892;
                                    break ;
                                }
                            }
                            18 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if r1 as libc::c_uint <= r0 as libc::c_uint {
                                    current_block = 5089124893069931607;
                                    break ;
                                } else {
                                    current_block = 7034501744547627146;
                                    break ;
                                }
                            }
                            19 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if r1 as libc::c_uint > r0 as libc::c_uint {
                                    current_block = 4871270227279186910;
                                    break ;
                                } else {
                                    current_block = 5908482871227205451;
                                    break ;
                                }
                            }
                            20 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if r1 as libc::c_uint >= r0 as libc::c_uint {
                                    current_block = 15993708482136914563;
                                    break ;
                                } else {
                                    current_block = 6938158527927677584;
                                    break ;
                                }
                            }
                            21 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if *(opStack as
                                         *mut libc::c_float).offset((opStackOfs
                                                                         as
                                                                         libc::c_int
                                                                         +
                                                                         1i32)
                                                                        as
                                                                        uint8_t
                                                                        as
                                                                        isize)
                                       ==
                                       *(opStack as
                                             *mut libc::c_float).offset((opStackOfs
                                                                             as
                                                                             libc::c_int
                                                                             +
                                                                             2i32)
                                                                            as
                                                                            uint8_t
                                                                            as
                                                                            isize)
                                   {
                                    current_block = 6186816898867308296;
                                    break ;
                                } else {
                                    current_block = 7173345243791314703;
                                    break ;
                                }
                            }
                            22 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if *(opStack as
                                         *mut libc::c_float).offset((opStackOfs
                                                                         as
                                                                         libc::c_int
                                                                         +
                                                                         1i32)
                                                                        as
                                                                        uint8_t
                                                                        as
                                                                        isize)
                                       !=
                                       *(opStack as
                                             *mut libc::c_float).offset((opStackOfs
                                                                             as
                                                                             libc::c_int
                                                                             +
                                                                             2i32)
                                                                            as
                                                                            uint8_t
                                                                            as
                                                                            isize)
                                   {
                                    current_block = 11226769033371074123;
                                    break ;
                                } else {
                                    current_block = 4183419379601546972;
                                    break ;
                                }
                            }
                            23 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if *(opStack as
                                         *mut libc::c_float).offset((opStackOfs
                                                                         as
                                                                         libc::c_int
                                                                         +
                                                                         1i32)
                                                                        as
                                                                        uint8_t
                                                                        as
                                                                        isize)
                                       <
                                       *(opStack as
                                             *mut libc::c_float).offset((opStackOfs
                                                                             as
                                                                             libc::c_int
                                                                             +
                                                                             2i32)
                                                                            as
                                                                            uint8_t
                                                                            as
                                                                            isize)
                                   {
                                    current_block = 4122836492991094814;
                                    break ;
                                } else {
                                    current_block = 2413388577390654262;
                                    break ;
                                }
                            }
                            24 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if *(opStack as
                                         *mut libc::c_float).offset((opStackOfs
                                                                         as
                                                                         libc::c_int
                                                                         +
                                                                         1i32)
                                                                        as
                                                                        uint8_t
                                                                        as
                                                                        isize)
                                       <=
                                       *(opStack as
                                             *mut libc::c_float).offset((opStackOfs
                                                                             as
                                                                             libc::c_int
                                                                             +
                                                                             2i32)
                                                                            as
                                                                            uint8_t
                                                                            as
                                                                            isize)
                                   {
                                    current_block = 654039154479240366;
                                    break ;
                                } else {
                                    current_block = 6957654774345280688;
                                    break ;
                                }
                            }
                            25 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if *(opStack as
                                         *mut libc::c_float).offset((opStackOfs
                                                                         as
                                                                         libc::c_int
                                                                         +
                                                                         1i32)
                                                                        as
                                                                        uint8_t
                                                                        as
                                                                        isize)
                                       >
                                       *(opStack as
                                             *mut libc::c_float).offset((opStackOfs
                                                                             as
                                                                             libc::c_int
                                                                             +
                                                                             2i32)
                                                                            as
                                                                            uint8_t
                                                                            as
                                                                            isize)
                                   {
                                    current_block = 2346768750020253347;
                                    break ;
                                } else {
                                    current_block = 11814324130289762492;
                                    break ;
                                }
                            }
                            26 => {
                                opStackOfs =
                                    (opStackOfs as libc::c_int - 2i32) as
                                        uint8_t;
                                if *(opStack as
                                         *mut libc::c_float).offset((opStackOfs
                                                                         as
                                                                         libc::c_int
                                                                         +
                                                                         1i32)
                                                                        as
                                                                        uint8_t
                                                                        as
                                                                        isize)
                                       >=
                                       *(opStack as
                                             *mut libc::c_float).offset((opStackOfs
                                                                             as
                                                                             libc::c_int
                                                                             +
                                                                             2i32)
                                                                            as
                                                                            uint8_t
                                                                            as
                                                                            isize)
                                   {
                                    current_block = 14187386403465544025;
                                    break ;
                                } else {
                                    current_block = 8551376836414271792;
                                    break ;
                                }
                            }
                            37 => {
                                *opStack.offset(opStackOfs as isize) = -r0;
                                continue 'c_6530 ;
                            }
                            38 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    r1 + r0;
                                continue 'c_6530 ;
                            }
                            39 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    r1 - r0;
                                continue 'c_6530 ;
                            }
                            40 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    r1 / r0;
                                continue 'c_6530 ;
                            }
                            41 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    (r1 as
                                         libc::c_uint).wrapping_div(r0 as
                                                                        libc::c_uint)
                                        as libc::c_int;
                                continue 'c_6530 ;
                            }
                            42 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    r1 % r0;
                                continue 'c_6530 ;
                            }
                            43 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    (r1 as
                                         libc::c_uint).wrapping_rem(r0 as
                                                                        libc::c_uint)
                                        as libc::c_int;
                                continue 'c_6530 ;
                            }
                            44 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    r1 * r0;
                                continue 'c_6530 ;
                            }
                            45 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    (r1 as
                                         libc::c_uint).wrapping_mul(r0 as
                                                                        libc::c_uint)
                                        as libc::c_int;
                                continue 'c_6530 ;
                            }
                            46 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    (r1 as libc::c_uint & r0 as libc::c_uint)
                                        as libc::c_int;
                                continue 'c_6530 ;
                            }
                            47 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    (r1 as libc::c_uint | r0 as libc::c_uint)
                                        as libc::c_int;
                                continue 'c_6530 ;
                            }
                            48 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    (r1 as libc::c_uint ^ r0 as libc::c_uint)
                                        as libc::c_int;
                                continue 'c_6530 ;
                            }
                            49 => {
                                *opStack.offset(opStackOfs as isize) =
                                    !(r0 as libc::c_uint) as libc::c_int;
                                continue 'c_6530 ;
                            }
                            50 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    r1 << r0;
                                continue 'c_6530 ;
                            }
                            51 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    r1 >> r0;
                                continue 'c_6530 ;
                            }
                            52 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *opStack.offset(opStackOfs as isize) =
                                    (r1 as libc::c_uint >> r0) as libc::c_int;
                                continue 'c_6530 ;
                            }
                            53 => {
                                *(opStack as
                                      *mut libc::c_float).offset(opStackOfs as
                                                                     isize) =
                                    -*(opStack as
                                           *mut libc::c_float).offset(opStackOfs
                                                                          as
                                                                          isize);
                                continue 'c_6530 ;
                            }
                            54 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *(opStack as
                                      *mut libc::c_float).offset(opStackOfs as
                                                                     isize) =
                                    *(opStack as
                                          *mut libc::c_float).offset(opStackOfs
                                                                         as
                                                                         isize)
                                        +
                                        *(opStack as
                                              *mut libc::c_float).offset((opStackOfs
                                                                              as
                                                                              libc::c_int
                                                                              +
                                                                              1i32)
                                                                             as
                                                                             uint8_t
                                                                             as
                                                                             isize);
                                continue 'c_6530 ;
                            }
                            55 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *(opStack as
                                      *mut libc::c_float).offset(opStackOfs as
                                                                     isize) =
                                    *(opStack as
                                          *mut libc::c_float).offset(opStackOfs
                                                                         as
                                                                         isize)
                                        -
                                        *(opStack as
                                              *mut libc::c_float).offset((opStackOfs
                                                                              as
                                                                              libc::c_int
                                                                              +
                                                                              1i32)
                                                                             as
                                                                             uint8_t
                                                                             as
                                                                             isize);
                                continue 'c_6530 ;
                            }
                            56 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *(opStack as
                                      *mut libc::c_float).offset(opStackOfs as
                                                                     isize) =
                                    *(opStack as
                                          *mut libc::c_float).offset(opStackOfs
                                                                         as
                                                                         isize)
                                        /
                                        *(opStack as
                                              *mut libc::c_float).offset((opStackOfs
                                                                              as
                                                                              libc::c_int
                                                                              +
                                                                              1i32)
                                                                             as
                                                                             uint8_t
                                                                             as
                                                                             isize);
                                continue 'c_6530 ;
                            }
                            57 => {
                                opStackOfs = opStackOfs.wrapping_sub(1);
                                *(opStack as
                                      *mut libc::c_float).offset(opStackOfs as
                                                                     isize) =
                                    *(opStack as
                                          *mut libc::c_float).offset(opStackOfs
                                                                         as
                                                                         isize)
                                        *
                                        *(opStack as
                                              *mut libc::c_float).offset((opStackOfs
                                                                              as
                                                                              libc::c_int
                                                                              +
                                                                              1i32)
                                                                             as
                                                                             uint8_t
                                                                             as
                                                                             isize);
                                continue 'c_6530 ;
                            }
                            58 => {
                                *(opStack as
                                      *mut libc::c_float).offset(opStackOfs as
                                                                     isize) =
                                    *opStack.offset(opStackOfs as isize) as
                                        libc::c_float;
                                continue 'c_6530 ;
                            }
                            59 => {
                                *opStack.offset(opStackOfs as isize) =
                                    qftolsse(*(opStack as
                                                   *mut libc::c_float).offset(opStackOfs
                                                                                  as
                                                                                  isize))
                                        as libc::c_int;
                                continue 'c_6530 ;
                            }
                            35 => {
                                *opStack.offset(opStackOfs as isize) =
                                    *opStack.offset(opStackOfs as isize) as
                                        libc::c_schar as libc::c_int;
                                continue 'c_6530 ;
                            }
                            36 => {
                                *opStack.offset(opStackOfs as isize) =
                                    *opStack.offset(opStackOfs as isize) as
                                        libc::c_short as libc::c_int;
                                continue 'c_6530 ;
                            }
                            _ => { break 'c_6530 ; }
                        }
                    }
                    match current_block {
                        4122836492991094814 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        6186816898867308296 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        15993708482136914563 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        4871270227279186910 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        5089124893069931607 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        12608488225262500095 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        5590933039760577279 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        7728257318064351663 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        4691324637564808323 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        3812947724376655173 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        2522825242109451841 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        8533724845731836612 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        562309032768341766 => { programCounter += 1i32 }
                        3575278370434307847 => { programCounter += 1i32 }
                        8304106758420804164 => { programCounter += 1i32 }
                        16667286137552459707 => { programCounter += 1i32 }
                        18039443766442739006 => { programCounter += 1i32 }
                        10357520176418200368 => { programCounter += 1i32 }
                        14114759727632161892 => { programCounter += 1i32 }
                        7034501744547627146 => { programCounter += 1i32 }
                        5908482871227205451 => { programCounter += 1i32 }
                        6938158527927677584 => { programCounter += 1i32 }
                        7173345243791314703 => { programCounter += 1i32 }
                        4183419379601546972 => { programCounter += 1i32 }
                        2413388577390654262 => { programCounter += 1i32 }
                        6957654774345280688 => { programCounter += 1i32 }
                        11814324130289762492 => { programCounter += 1i32 }
                        8551376836414271792 => { programCounter += 1i32 }
                        14187386403465544025 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        2346768750020253347 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        654039154479240366 => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                        _ => {
                            programCounter =
                                *codeImage.offset(programCounter as isize)
                        }
                    }
                }
        }
    (*vm).currentlyInterpreting = qfalse;
    if opStackOfs as libc::c_int != 1i32 ||
           *opStack as libc::c_uint != 0xdeadbeefu32 {
        Com_Error(ERR_DROP as libc::c_int,
                  b"Interpreter error: opStack[0] = %X, opStackOfs = %d\x00"
                      as *const u8 as *const libc::c_char,
                  *opStack.offset(0isize), opStackOfs as libc::c_int);
    }
    (*vm).programStack = stackOnEntry;
    return *opStack.offset(opStackOfs as isize);
}
#[no_mangle]
pub unsafe extern "C" fn VM_Indent(mut vm: *mut vm_t) -> *mut libc::c_char {
    static mut string: *mut libc::c_char =
        b"                                        \x00" as *const u8 as
            *const libc::c_char as *mut libc::c_char;
    if (*vm).callLevel > 20i32 { return string }
    return string.offset((2i32 * (20i32 - (*vm).callLevel)) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn VM_StackTrace(mut vm: *mut vm_t,
                                       mut programCounter: libc::c_int,
                                       mut programStack: libc::c_int) {
    let mut count: libc::c_int = 0;
    count = 0i32;
    loop  {
        Com_Printf(b"%s\n\x00" as *const u8 as *const libc::c_char,
                   VM_ValueToSymbol(vm, programCounter));
        programStack =
            *(&mut *(*vm).dataBase.offset((programStack + 4i32) as isize) as
                  *mut byte as *mut libc::c_int);
        programCounter =
            *(&mut *(*vm).dataBase.offset(programStack as isize) as *mut byte
                  as *mut libc::c_int);
        if !(programCounter != -1i32 && { count += 1; count < 32i32 }) {
            break ;
        }
    };
}