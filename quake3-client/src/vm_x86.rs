use libc;
#[header_src = "/usr/include/x86_64-linux-gnu/bits/types.h"]
pub mod types_h {
    pub type __uint8_t = libc::c_uchar;
    pub type __int32_t = libc::c_int;
    pub type __off_t = libc::c_long;
    use super::{libc};
}
#[header_src = "/usr/lib/llvm-6.0/lib/clang/6.0.0/include/stddef.h"]
pub mod stddef_h {
    pub type size_t = libc::c_ulong;
    use super::{libc};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-intn.h"]
pub mod stdint_intn_h {
    pub type int32_t = __int32_t;
    use super::types_h::{__int32_t};
}
#[header_src = "/usr/include/x86_64-linux-gnu/bits/stdint-uintn.h"]
pub mod stdint_uintn_h {
    pub type uint8_t = __uint8_t;
    use super::types_h::{__uint8_t};
}
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
    use super::{libc};
    extern "C" {
        #[no_mangle]
        pub static mut Q_VMftol:
                   Option<unsafe extern "C" fn() -> libc::c_int>;
        // this is only here so the functions in q_shared.c and bg_*.c can link
        #[no_mangle]
        pub fn Com_Error(level: libc::c_int, error: *const libc::c_char, ...)
         -> !;
        #[no_mangle]
        pub fn Com_Printf(msg: *const libc::c_char, ...);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qfiles.h"]
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
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/vm_local.h"]
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
        pub symName: [libc::c_char; 1],
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
        pub static mut currentVM: *mut vm_t;
        #[no_mangle]
        pub fn VM_BlockCopy(dest: libc::c_uint, src: libc::c_uint, n: size_t);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/qcommon.h"]
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
        pub fn Z_MallocDebug(size: libc::c_int, label: *mut libc::c_char,
                             file: *mut libc::c_char, line: libc::c_int)
         -> *mut libc::c_void;
        #[no_mangle]
        pub fn Z_Free(ptr: *mut libc::c_void);
    }
}
#[header_src = "/home/miguelsaldivar/workspace/ioq3/code/qcommon/vm_x86.c"]
pub mod vm_x86_c {
    pub type ELastCommand = libc::c_uint;
    pub const LAST_COMMAND_SUB_BL_2: ELastCommand = 3;
    pub const LAST_COMMAND_SUB_BL_1: ELastCommand = 2;
    pub const LAST_COMMAND_MOV_STACK_EAX: ELastCommand = 1;
    pub const LAST_COMMAND_NONE: ELastCommand = 0;
    pub const VM_JMP_VIOLATION: unnamed_1 = 0;
    pub const VM_BLOCK_COPY: unnamed_1 = 1;
    pub type unnamed_1 = libc::c_uint;
    use super::{libc};
    use super::qcommon_h::{vm_t};
    use super::q_shared_h::{qboolean};
    use super::stdint_h::{intptr_t};
    use super::stdint_uintn_h::{uint8_t};
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
    }
}
#[header_src = "/usr/include/x86_64-linux-gnu/sys/mman.h"]
pub mod mman_h {
    use super::{libc};
    use super::stddef_h::{size_t};
    use super::types_h::{__off_t};
    extern "C" {
        #[no_mangle]
        pub fn munmap(__addr: *mut libc::c_void, __len: size_t)
         -> libc::c_int;
        #[no_mangle]
        pub fn mprotect(__addr: *mut libc::c_void, __len: size_t,
                        __prot: libc::c_int) -> libc::c_int;
        #[no_mangle]
        pub fn mmap(__addr: *mut libc::c_void, __len: size_t,
                    __prot: libc::c_int, __flags: libc::c_int,
                    __fd: libc::c_int, __offset: __off_t)
         -> *mut libc::c_void;
    }
}
use self::types_h::{__uint8_t, __int32_t, __off_t};
use self::stddef_h::{size_t};
use self::stdint_intn_h::{int32_t};
use self::stdint_uintn_h::{uint8_t};
use self::stdint_h::{intptr_t};
use self::q_shared_h::{byte, qboolean, qtrue, qfalse, unnamed, ERR_NEED_CD,
                       ERR_DISCONNECT, ERR_SERVERDISCONNECT, ERR_DROP,
                       ERR_FATAL, Q_VMftol, Com_Error, Com_Printf};
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
                       OP_ENTER, OP_BREAK, OP_IGNORE, OP_UNDEF, currentVM,
                       VM_BlockCopy};
use self::qcommon_h::{vm_t, Z_MallocDebug, Z_Free};
use self::vm_x86_c::{ELastCommand, LAST_COMMAND_SUB_BL_2,
                     LAST_COMMAND_SUB_BL_1, LAST_COMMAND_MOV_STACK_EAX,
                     LAST_COMMAND_NONE, VM_JMP_VIOLATION, VM_BLOCK_COPY,
                     unnamed_1};
use self::string_h::{memcpy, memset};
use self::mman_h::{munmap, mprotect, mmap};
#[no_mangle]
pub unsafe extern "C" fn VM_Compile(mut vm: *mut vm_t,
                                    mut header: *mut vmHeader_t) {
    let mut op: libc::c_int = 0;
    let mut maxLength: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut callProcOfsSyscall: libc::c_int = 0;
    let mut callProcOfs: libc::c_int = 0;
    let mut callDoSyscallOfs: libc::c_int = 0;
    jusedSize = (*header).instructionCount + 2i32;
    maxLength = (*header).codeLength * 8i32 + 64i32;
    buf =
        Z_MallocDebug(maxLength,
                      b"maxLength\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/vm_x86.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 1083i32)
            as *mut byte;
    jused =
        Z_MallocDebug(jusedSize,
                      b"jusedSize\x00" as *const u8 as *const libc::c_char as
                          *mut libc::c_char,
                      b"code/qcommon/vm_x86.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 1084i32)
            as *mut byte;
    code =
        Z_MallocDebug((*header).codeLength + 32i32,
                      b"header->codeLength+32\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char,
                      b"code/qcommon/vm_x86.c\x00" as *const u8 as
                          *const libc::c_char as *mut libc::c_char, 1085i32)
            as *mut byte;
    memset(jused as *mut libc::c_void, 0i32, jusedSize as libc::c_ulong);
    memset(buf as *mut libc::c_void, 0i32, maxLength as libc::c_ulong);
    memset(code as *mut libc::c_void, 0i32,
           ((*header).codeLength + 32i32) as libc::c_ulong);
    memcpy(code as *mut libc::c_void,
           (header as *mut byte).offset((*header).codeOffset as isize) as
               *const libc::c_void, (*header).codeLength as libc::c_ulong);
    pc = -1i32;
    i = 0i32;
    while i < (*vm).numJumpTableTargets {
        if (*((*vm).jumpTableTargets.offset((i as
                                                 libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                                 as
                                                                                 libc::c_ulong)
                                                as isize) as
                  *mut libc::c_int)) < 0i32 ||
               *((*vm).jumpTableTargets.offset((i as
                                                    libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                                    as
                                                                                    libc::c_ulong)
                                                   as isize) as
                     *mut libc::c_int) >= (*vm).instructionCount {
            Z_Free(buf as *mut libc::c_void);
            Z_Free(jused as *mut libc::c_void);
            Com_Error(ERR_DROP as libc::c_int,
                      b"VM_CompileX86: jump target out of range at offset %d\x00"
                          as *const u8 as *const libc::c_char, pc);
        }
        *jused.offset(*((*vm).jumpTableTargets.offset((i as
                                                           libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>()
                                                                                           as
                                                                                           libc::c_ulong)
                                                          as isize) as
                            *mut libc::c_int) as isize) = 1i32 as byte;
        i += 1
    }
    compiledOfs = 0i32;
    callDoSyscallOfs = compiledOfs;
    callProcOfs = EmitCallDoSyscall(vm);
    callProcOfsSyscall = EmitCallProcedure(vm, callDoSyscallOfs);
    (*vm).entryOfs = compiledOfs;
    pass = 0i32;
    while pass < 3i32 {
        oc0 = -23423i32;
        oc1 = -234354i32;
        pop0 = -43435i32;
        pop1 = -545455i32;
        pc = 0i32;
        instruction = 0i32;
        compiledOfs = (*vm).entryOfs;
        LastCommand = LAST_COMMAND_NONE;
        while instruction < (*header).instructionCount {
            if compiledOfs > maxLength - 16i32 {
                Z_Free(buf as *mut libc::c_void);
                Z_Free(jused as *mut libc::c_void);
                Com_Error(ERR_DROP as libc::c_int,
                          b"VM_CompileX86: maxLength exceeded\x00" as
                              *const u8 as *const libc::c_char);
            }
            *(*vm).instructionPointers.offset(instruction as isize) =
                compiledOfs as intptr_t;
            if (*vm).jumpTableTargets.is_null() {
                jlabel = 1i32
            } else {
                jlabel = *jused.offset(instruction as isize) as libc::c_int
            }
            instruction += 1;
            if pc > (*header).codeLength {
                Z_Free(buf as *mut libc::c_void);
                Z_Free(jused as *mut libc::c_void);
                Com_Error(ERR_DROP as libc::c_int,
                          b"VM_CompileX86: pc > header->codeLength\x00" as
                              *const u8 as *const libc::c_char);
            }
            op = *code.offset(pc as isize) as libc::c_int;
            pc += 1;
            match op {
                0 => { }
                2 => {
                    EmitString(b"CC\x00" as *const u8 as *const libc::c_char);
                }
                3 => {
                    EmitString(b"81 EE\x00" as *const u8 as
                                   *const libc::c_char);
                    Emit4(Constant4());
                }
                8 => {
                    if !(0 != ConstOptimize(vm, callProcOfsSyscall) as u64) {
                        EmitPushStack(vm);
                        EmitString(b"C7 04 9F\x00" as *const u8 as
                                       *const libc::c_char);
                        lastConst = Constant4();
                        Emit4(lastConst);
                        if *code.offset(pc as isize) as libc::c_int ==
                               OP_JUMP as libc::c_int {
                            if lastConst < 0i32 ||
                                   lastConst >= (*vm).instructionCount {
                                Z_Free(buf as *mut libc::c_void);
                                Z_Free(jused as *mut libc::c_void);
                                Com_Error(ERR_DROP as libc::c_int,
                                          b"VM_CompileX86: jump target out of range at offset %d\x00"
                                              as *const u8 as
                                              *const libc::c_char, pc);
                            }
                            *jused.offset(lastConst as isize) = 1i32 as byte
                        }
                    }
                }
                9 => {
                    EmitPushStack(vm);
                    EmitString(b"8D 86\x00" as *const u8 as
                                   *const libc::c_char);
                    oc0 = oc1;
                    oc1 = Constant4();
                    Emit4(oc1);
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                }
                33 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitString(b"8B D6\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"81 C2\x00" as *const u8 as
                                   *const libc::c_char);
                    Emit4(Constant1() & 0xffi32);
                    EmitString(b"81\x00" as *const u8 as *const libc::c_char);
                    EmitString(b"E2\x00" as *const u8 as *const libc::c_char);
                    Emit4((*vm).dataMask);
                    EmitRexString(0x41i32 as byte,
                                  b"89 04 11\x00" as *const u8 as
                                      *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                5 => { EmitCallRel(vm, callProcOfs); }
                6 => { EmitPushStack(vm); }
                7 => { EmitCommand(LAST_COMMAND_SUB_BL_1); }
                4 => {
                    v = Constant4();
                    EmitString(b"81 C6\x00" as *const u8 as
                                   *const libc::c_char);
                    Emit4(v);
                    EmitString(b"C3\x00" as *const u8 as *const libc::c_char);
                }
                29 => {
                    if *code.offset(pc as isize) as libc::c_int ==
                           OP_CONST as libc::c_int &&
                           *code.offset((pc + 5i32) as isize) as libc::c_int
                               == OP_ADD as libc::c_int &&
                           *code.offset((pc + 6i32) as isize) as libc::c_int
                               == OP_STORE4 as libc::c_int {
                        if oc0 == oc1 && pop0 == OP_LOCAL as libc::c_int &&
                               pop1 == OP_LOCAL as libc::c_int {
                            compiledOfs -= 12i32;
                            *(*vm).instructionPointers.offset((instruction -
                                                                   1i32) as
                                                                  isize) =
                                compiledOfs as intptr_t
                        }
                        pc += 1;
                        v = Constant4();
                        EmitMovEDXStack(vm, (*vm).dataMask);
                        if v == 1i32 && oc0 == oc1 &&
                               pop0 == OP_LOCAL as libc::c_int &&
                               pop1 == OP_LOCAL as libc::c_int {
                            EmitRexString(0x41i32 as byte,
                                          b"FF 04 11\x00" as *const u8 as
                                              *const libc::c_char);
                        } else {
                            EmitRexString(0x41i32 as byte,
                                          b"8B 04 11\x00" as *const u8 as
                                              *const libc::c_char);
                            EmitString(b"05\x00" as *const u8 as
                                           *const libc::c_char);
                            Emit4(v);
                            if oc0 == oc1 && pop0 == OP_LOCAL as libc::c_int
                                   && pop1 == OP_LOCAL as libc::c_int {
                                EmitRexString(0x41i32 as byte,
                                              b"89 04 11\x00" as *const u8 as
                                                  *const libc::c_char);
                            } else {
                                EmitCommand(LAST_COMMAND_SUB_BL_1);
                                EmitString(b"8B 14 9F\x00" as *const u8 as
                                               *const libc::c_char);
                                EmitString(b"81\x00" as *const u8 as
                                               *const libc::c_char);
                                EmitString(b"E2\x00" as *const u8 as
                                               *const libc::c_char);
                                Emit4((*vm).dataMask);
                                EmitRexString(0x41i32 as byte,
                                              b"89 04 11\x00" as *const u8 as
                                                  *const libc::c_char);
                            }
                        }
                        EmitCommand(LAST_COMMAND_SUB_BL_1);
                        pc += 1;
                        pc += 1;
                        instruction += 3i32
                    } else if *code.offset(pc as isize) as libc::c_int ==
                                  OP_CONST as libc::c_int &&
                                  *code.offset((pc + 5i32) as isize) as
                                      libc::c_int == OP_SUB as libc::c_int &&
                                  *code.offset((pc + 6i32) as isize) as
                                      libc::c_int == OP_STORE4 as libc::c_int
                     {
                        if oc0 == oc1 && pop0 == OP_LOCAL as libc::c_int &&
                               pop1 == OP_LOCAL as libc::c_int {
                            compiledOfs -= 12i32;
                            *(*vm).instructionPointers.offset((instruction -
                                                                   1i32) as
                                                                  isize) =
                                compiledOfs as intptr_t
                        }
                        pc += 1;
                        v = Constant4();
                        EmitMovEDXStack(vm, (*vm).dataMask);
                        if v == 1i32 && oc0 == oc1 &&
                               pop0 == OP_LOCAL as libc::c_int &&
                               pop1 == OP_LOCAL as libc::c_int {
                            EmitRexString(0x41i32 as byte,
                                          b"FF 0C 11\x00" as *const u8 as
                                              *const libc::c_char);
                        } else {
                            EmitRexString(0x41i32 as byte,
                                          b"8B 04 11\x00" as *const u8 as
                                              *const libc::c_char);
                            EmitString(b"2D\x00" as *const u8 as
                                           *const libc::c_char);
                            Emit4(v);
                            if oc0 == oc1 && pop0 == OP_LOCAL as libc::c_int
                                   && pop1 == OP_LOCAL as libc::c_int {
                                EmitRexString(0x41i32 as byte,
                                              b"89 04 11\x00" as *const u8 as
                                                  *const libc::c_char);
                            } else {
                                EmitCommand(LAST_COMMAND_SUB_BL_1);
                                EmitString(b"8B 14 9F\x00" as *const u8 as
                                               *const libc::c_char);
                                EmitString(b"81\x00" as *const u8 as
                                               *const libc::c_char);
                                EmitString(b"E2\x00" as *const u8 as
                                               *const libc::c_char);
                                Emit4((*vm).dataMask);
                                EmitRexString(0x41i32 as byte,
                                              b"89 04 11\x00" as *const u8 as
                                                  *const libc::c_char);
                            }
                        }
                        EmitCommand(LAST_COMMAND_SUB_BL_1);
                        pc += 1;
                        pc += 1;
                        instruction += 3i32
                    } else if *buf.offset((compiledOfs - 3i32) as isize) as
                                  libc::c_int == 0x89i32 &&
                                  *buf.offset((compiledOfs - 2i32) as isize)
                                      as libc::c_int == 0x4i32 &&
                                  *buf.offset((compiledOfs - 1i32) as isize)
                                      as libc::c_int == 0x9fi32 {
                        compiledOfs -= 3i32;
                        *(*vm).instructionPointers.offset((instruction - 1i32)
                                                              as isize) =
                            compiledOfs as intptr_t;
                        EmitString(b"81\x00" as *const u8 as
                                       *const libc::c_char);
                        EmitString(b"E0\x00" as *const u8 as
                                       *const libc::c_char);
                        Emit4((*vm).dataMask);
                        EmitRexString(0x41i32 as byte,
                                      b"8B 04 01\x00" as *const u8 as
                                          *const libc::c_char);
                        EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                    } else {
                        EmitMovEAXStack(vm, (*vm).dataMask);
                        EmitRexString(0x41i32 as byte,
                                      b"8B 04 01\x00" as *const u8 as
                                          *const libc::c_char);
                        EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                    }
                }
                28 => {
                    EmitMovEAXStack(vm, (*vm).dataMask);
                    EmitRexString(0x41i32 as byte,
                                  b"0F B7 04 01\x00" as *const u8 as
                                      *const libc::c_char);
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                }
                27 => {
                    EmitMovEAXStack(vm, (*vm).dataMask);
                    EmitRexString(0x41i32 as byte,
                                  b"0F B6 04 01\x00" as *const u8 as
                                      *const libc::c_char);
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                }
                32 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitString(b"8B 54 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"81\x00" as *const u8 as *const libc::c_char);
                    EmitString(b"E2\x00" as *const u8 as *const libc::c_char);
                    Emit4((*vm).dataMask);
                    EmitRexString(0x41i32 as byte,
                                  b"89 04 11\x00" as *const u8 as
                                      *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_2);
                }
                31 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitString(b"8B 54 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"81\x00" as *const u8 as *const libc::c_char);
                    EmitString(b"E2\x00" as *const u8 as *const libc::c_char);
                    Emit4((*vm).dataMask);
                    Emit1(0x66i32);
                    EmitRexString(0x41i32 as byte,
                                  b"89 04 11\x00" as *const u8 as
                                      *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_2);
                }
                30 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitString(b"8B 54 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"81\x00" as *const u8 as *const libc::c_char);
                    EmitString(b"E2\x00" as *const u8 as *const libc::c_char);
                    Emit4((*vm).dataMask);
                    EmitRexString(0x41i32 as byte,
                                  b"88 04 11\x00" as *const u8 as
                                      *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_2);
                }
                11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitCommand(LAST_COMMAND_SUB_BL_2);
                    EmitString(b"39 44 9F 04\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitBranchConditions(vm, op);
                }
                21 | 22 | 23 | 24 | 25 | 26 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_2);
                    EmitString(b"D9 44 9F 04\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D8 5C 9F 08\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"DF E0\x00" as *const u8 as
                                   *const libc::c_char);
                    match op {
                        21 => {
                            EmitString(b"F6 C4 40\x00" as *const u8 as
                                           *const libc::c_char);
                            EmitJumpIns(vm,
                                        b"0F 85\x00" as *const u8 as
                                            *const libc::c_char, Constant4());
                        }
                        22 => {
                            EmitString(b"F6 C4 40\x00" as *const u8 as
                                           *const libc::c_char);
                            EmitJumpIns(vm,
                                        b"0F 84\x00" as *const u8 as
                                            *const libc::c_char, Constant4());
                        }
                        23 => {
                            EmitString(b"F6 C4 01\x00" as *const u8 as
                                           *const libc::c_char);
                            EmitJumpIns(vm,
                                        b"0F 85\x00" as *const u8 as
                                            *const libc::c_char, Constant4());
                        }
                        24 => {
                            EmitString(b"F6 C4 41\x00" as *const u8 as
                                           *const libc::c_char);
                            EmitJumpIns(vm,
                                        b"0F 85\x00" as *const u8 as
                                            *const libc::c_char, Constant4());
                        }
                        25 => {
                            EmitString(b"F6 C4 41\x00" as *const u8 as
                                           *const libc::c_char);
                            EmitJumpIns(vm,
                                        b"0F 84\x00" as *const u8 as
                                            *const libc::c_char, Constant4());
                        }
                        26 => {
                            EmitString(b"F6 C4 01\x00" as *const u8 as
                                           *const libc::c_char);
                            EmitJumpIns(vm,
                                        b"0F 84\x00" as *const u8 as
                                            *const libc::c_char, Constant4());
                        }
                        _ => { }
                    }
                }
                37 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitString(b"F7 D8\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                }
                38 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitString(b"01 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                39 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitString(b"29 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                40 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"99\x00" as *const u8 as *const libc::c_char);
                    EmitString(b"F7 3C 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"89 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                41 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"33 D2\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"F7 34 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"89 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                42 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"99\x00" as *const u8 as *const libc::c_char);
                    EmitString(b"F7 3C 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"89 54 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                43 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"33 D2\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"F7 34 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"89 54 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                44 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"F7 2C 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"89 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                45 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"F7 24 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"89 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                46 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitString(b"21 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                47 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitString(b"09 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                48 => {
                    EmitMovEAXStack(vm, 0i32);
                    EmitString(b"31 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                49 => {
                    EmitString(b"F7 14 9F\x00" as *const u8 as
                                   *const libc::c_char);
                }
                50 => {
                    EmitMovECXStack(vm);
                    EmitString(b"D3 64 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                51 => {
                    EmitMovECXStack(vm);
                    EmitString(b"D3 7C 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                52 => {
                    EmitMovECXStack(vm);
                    EmitString(b"D3 6C 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                53 => {
                    EmitString(b"D9 04 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D9 E0\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D9 1C 9F\x00" as *const u8 as
                                   *const libc::c_char);
                }
                54 => {
                    EmitString(b"D9 44 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D8 04 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D9 5C 9F FC\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                55 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                    EmitString(b"D9 04 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D8 64 9F 04\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D9 1C 9F\x00" as *const u8 as
                                   *const libc::c_char);
                }
                56 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                    EmitString(b"D9 04 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D8 74 9F 04\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D9 1C 9F\x00" as *const u8 as
                                   *const libc::c_char);
                }
                57 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                    EmitString(b"D9 04 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D8 4C 9F 04\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D9 1C 9F\x00" as *const u8 as
                                   *const libc::c_char);
                }
                58 => {
                    EmitString(b"DB 04 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"D9 1C 9F\x00" as *const u8 as
                                   *const libc::c_char);
                }
                59 => {
                    EmitRexString(0x48i32 as byte,
                                  b"BA\x00" as *const u8 as
                                      *const libc::c_char);
                    EmitPtr(::std::mem::transmute::<Option<unsafe extern "C" fn()
                                                               ->
                                                                   libc::c_int>,
                                                    *mut libc::c_void>(Q_VMftol));
                    EmitRexString(0x48i32 as byte,
                                  b"FF D2\x00" as *const u8 as
                                      *const libc::c_char);
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                }
                35 => {
                    EmitString(b"0F BE 04 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                }
                36 => {
                    EmitString(b"0F BF 04 9F\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                }
                34 => {
                    EmitString(b"B8\x00" as *const u8 as *const libc::c_char);
                    Emit4(VM_BLOCK_COPY as libc::c_int);
                    EmitString(b"B9\x00" as *const u8 as *const libc::c_char);
                    Emit4(Constant4());
                    EmitCallRel(vm, callDoSyscallOfs);
                    EmitCommand(LAST_COMMAND_SUB_BL_2);
                }
                10 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                    EmitString(b"8B 44 9F 04\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitString(b"81 F8\x00" as *const u8 as
                                   *const libc::c_char);
                    Emit4((*vm).instructionCount);
                    EmitString(b"73 04\x00" as *const u8 as
                                   *const libc::c_char);
                    EmitRexString(0x49i32 as byte,
                                  b"FF 24 C0\x00" as *const u8 as
                                      *const libc::c_char);
                    EmitCallErrJump(vm, callDoSyscallOfs);
                }
                _ => {
                    Z_Free(buf as *mut libc::c_void);
                    Z_Free(jused as *mut libc::c_void);
                    Com_Error(ERR_DROP as libc::c_int,
                              b"VM_CompileX86: bad opcode %i at offset %i\x00"
                                  as *const u8 as *const libc::c_char, op,
                              pc);
                }
            }
            pop0 = pop1;
            pop1 = op
        }
        pass += 1
    }
    (*vm).codeLength = compiledOfs;
    (*vm).codeBase =
        mmap(0 as *mut libc::c_void, compiledOfs as size_t, 0x2i32,
             0x1i32 | 0x20i32, -1i32, 0i32 as __off_t) as *mut byte;
    if (*vm).codeBase == -1i32 as *mut libc::c_void as *mut byte {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"VM_CompileX86: can\'t mmap memory\x00" as *const u8 as
                      *const libc::c_char);
    }
    memcpy((*vm).codeBase as *mut libc::c_void, buf as *const libc::c_void,
           compiledOfs as libc::c_ulong);
    if 0 !=
           mprotect((*vm).codeBase as *mut libc::c_void,
                    compiledOfs as size_t, 0x1i32 | 0x4i32) {
        Com_Error(ERR_FATAL as libc::c_int,
                  b"VM_CompileX86: mprotect failed\x00" as *const u8 as
                      *const libc::c_char);
    }
    Z_Free(code as *mut libc::c_void);
    Z_Free(buf as *mut libc::c_void);
    Z_Free(jused as *mut libc::c_void);
    Com_Printf(b"VM file %s compiled to %i bytes of code\n\x00" as *const u8
                   as *const libc::c_char, (*vm).name.as_mut_ptr(),
               compiledOfs);
    (*vm).destroy = Some(VM_Destroy_Compiled);
    i = 0i32;
    while i < (*header).instructionCount {
        let ref mut fresh0 = *(*vm).instructionPointers.offset(i as isize);
        *fresh0 += (*vm).codeBase as intptr_t;
        i += 1
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
// vm_x86.c -- load time compiler and execution environment for x86
// for PROT_ stuff
/* need this on NX enabled systems (i386 with PAE kernel or
   * noexec32=on x86_64) */
// workaround for systems that use the old MAP_ANON macro
unsafe extern "C" fn VM_Destroy_Compiled(mut self_0: *mut vm_t) {
    munmap((*self_0).codeBase as *mut libc::c_void,
           (*self_0).codeLength as size_t);
}
static mut compiledOfs: libc::c_int = 0i32;
static mut jused: *mut byte = 0 as *const byte as *mut byte;
/*

  eax		scratch
  ebx/bl	opStack offset
  ecx		scratch (required for shifts)
  edx		scratch (required for divisions)
  esi		program stack
  edi   	opStack base
x86_64:
  r8		vm->instructionPointers
  r9		vm->dataBase

*/
static mut buf: *mut byte = 0 as *const byte as *mut byte;
static mut code: *mut byte = 0 as *const byte as *mut byte;
static mut pop1: libc::c_int = 0;
static mut pop0: libc::c_int = 0;
static mut pc: libc::c_int = 0i32;
/*
=================
EmitCallErrJump
Emit the code that triggers execution of the jump violation handler
=================
*/
unsafe extern "C" fn EmitCallErrJump(mut vm: *mut vm_t,
                                     mut sysCallOfs: libc::c_int) {
    EmitString(b"B8\x00" as *const u8 as *const libc::c_char);
    Emit4(VM_JMP_VIOLATION as libc::c_int);
    EmitCallRel(vm, sysCallOfs);
}
/*
=================
EmitCallRel
Relative call to vm->codeBase + callOfs
=================
*/
#[no_mangle]
pub unsafe extern "C" fn EmitCallRel(mut vm: *mut vm_t,
                                     mut callOfs: libc::c_int) {
    EmitString(b"E8\x00" as *const u8 as *const libc::c_char);
    Emit4(callOfs - compiledOfs - 4i32);
}
unsafe extern "C" fn Emit4(mut v: libc::c_int) {
    Emit1(v & 0xffi32);
    Emit1(v >> 8i32 & 0xffi32);
    Emit1(v >> 16i32 & 0xffi32);
    Emit1(v >> 24i32 & 0xffi32);
}
unsafe extern "C" fn Emit1(mut v: libc::c_int) {
    *buf.offset(compiledOfs as isize) = v as byte;
    compiledOfs += 1;
    LastCommand = LAST_COMMAND_NONE;
}
static mut LastCommand: ELastCommand = LAST_COMMAND_NONE;
unsafe extern "C" fn EmitString(mut string: *const libc::c_char) {
    let mut c1: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    loop  {
        c1 = *string.offset(0isize) as libc::c_int;
        c2 = *string.offset(1isize) as libc::c_int;
        v = Hex(c1) << 4i32 | Hex(c2);
        Emit1(v);
        if 0 == *string.offset(2isize) { break ; }
        string = string.offset(3isize)
    };
}
unsafe extern "C" fn Hex(mut c: libc::c_int) -> libc::c_int {
    if c >= 'a' as i32 && c <= 'f' as i32 { return 10i32 + c - 'a' as i32 }
    if c >= 'A' as i32 && c <= 'F' as i32 { return 10i32 + c - 'A' as i32 }
    if c >= '0' as i32 && c <= '9' as i32 { return c - '0' as i32 }
    Z_Free(buf as *mut libc::c_void);
    Z_Free(jused as *mut libc::c_void);
    Com_Error(ERR_DROP as libc::c_int,
              b"Hex: bad char \'%c\'\x00" as *const u8 as *const libc::c_char,
              c);
}
unsafe extern "C" fn EmitRexString(mut rex: byte,
                                   mut string: *const libc::c_char) {
    if 0 != rex { Emit1(rex as libc::c_int); }
    EmitString(string);
}
// add bl, bytes
// sub bl, bytes
unsafe extern "C" fn EmitCommand(mut command: ELastCommand) {
    match command as libc::c_uint {
        1 => {
            EmitString(b"89 04 9F\x00" as *const u8 as *const libc::c_char);
        }
        2 => {
            EmitString(b"80 EB\x00" as *const u8 as *const libc::c_char);
            Emit1(1i32);
        }
        3 => {
            EmitString(b"80 EB\x00" as *const u8 as *const libc::c_char);
            Emit1(2i32);
        }
        _ => { }
    }
    LastCommand = command;
}
unsafe extern "C" fn Constant4() -> libc::c_int {
    let mut v: libc::c_int = 0;
    v = NextConstant4();
    pc += 4i32;
    return v;
}
unsafe extern "C" fn NextConstant4() -> libc::c_int {
    return (*code.offset(pc as isize) as libc::c_uint |
                (*code.offset((pc + 1i32) as isize) as libc::c_uint) << 8i32 |
                (*code.offset((pc + 2i32) as isize) as libc::c_uint) << 16i32
                |
                (*code.offset((pc + 3i32) as isize) as libc::c_uint) << 24i32)
               as libc::c_int;
}
unsafe extern "C" fn EmitPtr(mut ptr: *mut libc::c_void) {
    let mut v: intptr_t = ptr as intptr_t;
    Emit4(v as libc::c_int);
    Emit1((v >> 32i32 & 0xffi32 as libc::c_long) as libc::c_int);
    Emit1((v >> 40i32 & 0xffi32 as libc::c_long) as libc::c_int);
    Emit1((v >> 48i32 & 0xffi32 as libc::c_long) as libc::c_int);
    Emit1((v >> 56i32 & 0xffi32 as libc::c_long) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn EmitMovECXStack(mut vm: *mut vm_t) {
    if 0 == jlabel {
        if LastCommand as libc::c_uint ==
               LAST_COMMAND_MOV_STACK_EAX as libc::c_int as libc::c_uint {
            compiledOfs -= 3i32;
            *(*vm).instructionPointers.offset((instruction - 1i32) as isize) =
                compiledOfs as intptr_t;
            EmitString(b"89 C1\x00" as *const u8 as *const libc::c_char);
            return
        }
        if pop1 == OP_DIVI as libc::c_int || pop1 == OP_DIVU as libc::c_int ||
               pop1 == OP_MULI as libc::c_int ||
               pop1 == OP_MULU as libc::c_int ||
               pop1 == OP_STORE4 as libc::c_int ||
               pop1 == OP_STORE2 as libc::c_int ||
               pop1 == OP_STORE1 as libc::c_int {
            EmitString(b"89 C1\x00" as *const u8 as *const libc::c_char);
            return
        }
    }
    EmitString(b"8B 0C 9F\x00" as *const u8 as *const libc::c_char);
}
static mut instruction: libc::c_int = 0;
static mut jlabel: libc::c_int = 0;
unsafe extern "C" fn EmitMovEAXStack(mut vm: *mut vm_t,
                                     mut andit: libc::c_int) {
    if 0 == jlabel {
        if LastCommand as libc::c_uint ==
               LAST_COMMAND_MOV_STACK_EAX as libc::c_int as libc::c_uint {
            compiledOfs -= 3i32;
            *(*vm).instructionPointers.offset((instruction - 1i32) as isize) =
                compiledOfs as intptr_t
        } else if pop1 == OP_CONST as libc::c_int &&
                      *buf.offset((compiledOfs - 7i32) as isize) as
                          libc::c_int == 0xc7i32 &&
                      *buf.offset((compiledOfs - 6i32) as isize) as
                          libc::c_int == 0x4i32 &&
                      *buf.offset((compiledOfs - 5i32) as isize) as
                          libc::c_int == 0x9fi32 {
            compiledOfs -= 7i32;
            *(*vm).instructionPointers.offset((instruction - 1i32) as isize) =
                compiledOfs as intptr_t;
            EmitString(b"B8\x00" as *const u8 as *const libc::c_char);
            if 0 != andit {
                Emit4(lastConst & andit);
            } else { Emit4(lastConst); }
            return
        } else {
            if pop1 != OP_DIVI as libc::c_int &&
                   pop1 != OP_DIVU as libc::c_int &&
                   pop1 != OP_MULI as libc::c_int &&
                   pop1 != OP_MULU as libc::c_int &&
                   pop1 != OP_STORE4 as libc::c_int &&
                   pop1 != OP_STORE2 as libc::c_int &&
                   pop1 != OP_STORE1 as libc::c_int {
                EmitString(b"8B 04 9F\x00" as *const u8 as
                               *const libc::c_char);
            }
        }
    } else {
        EmitString(b"8B 04 9F\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != andit {
        EmitString(b"25\x00" as *const u8 as *const libc::c_char);
        Emit4(andit);
    };
}
static mut lastConst: libc::c_int = 0i32;
/*
=================
EmitJumpIns
Jump to constant instruction number
=================
*/
#[no_mangle]
pub unsafe extern "C" fn EmitJumpIns(mut vm: *mut vm_t,
                                     mut jmpop: *const libc::c_char,
                                     mut cdest: libc::c_int) {
    if cdest < 0i32 || cdest >= (*vm).instructionCount {
        Z_Free(buf as *mut libc::c_void);
        Z_Free(jused as *mut libc::c_void);
        Com_Error(ERR_DROP as libc::c_int,
                  b"VM_CompileX86: jump target out of range at offset %d\x00"
                      as *const u8 as *const libc::c_char, pc);
    }
    *jused.offset(cdest as isize) = 1i32 as byte;
    EmitString(jmpop);
    if pass == 2i32 {
        Emit4((*(*vm).instructionPointers.offset(cdest as isize) -
                   compiledOfs as libc::c_long - 4i32 as libc::c_long) as
                  libc::c_int);
    } else { compiledOfs += 4i32 };
}
static mut pass: libc::c_int = 0;
/*
=================
EmitBranchConditions
Emits x86 branch condition as given in op
=================
*/
#[no_mangle]
pub unsafe extern "C" fn EmitBranchConditions(mut vm: *mut vm_t,
                                              mut op: libc::c_int) {
    match op {
        11 => {
            EmitJumpIns(vm, b"0F 84\x00" as *const u8 as *const libc::c_char,
                        Constant4());
        }
        12 => {
            EmitJumpIns(vm, b"0F 85\x00" as *const u8 as *const libc::c_char,
                        Constant4());
        }
        13 => {
            EmitJumpIns(vm, b"0F 8C\x00" as *const u8 as *const libc::c_char,
                        Constant4());
        }
        14 => {
            EmitJumpIns(vm, b"0F 8E\x00" as *const u8 as *const libc::c_char,
                        Constant4());
        }
        15 => {
            EmitJumpIns(vm, b"0F 8F\x00" as *const u8 as *const libc::c_char,
                        Constant4());
        }
        16 => {
            EmitJumpIns(vm, b"0F 8D\x00" as *const u8 as *const libc::c_char,
                        Constant4());
        }
        17 => {
            EmitJumpIns(vm, b"0F 82\x00" as *const u8 as *const libc::c_char,
                        Constant4());
        }
        18 => {
            EmitJumpIns(vm, b"0F 86\x00" as *const u8 as *const libc::c_char,
                        Constant4());
        }
        19 => {
            EmitJumpIns(vm, b"0F 87\x00" as *const u8 as *const libc::c_char,
                        Constant4());
        }
        20 => {
            EmitJumpIns(vm, b"0F 83\x00" as *const u8 as *const libc::c_char,
                        Constant4());
        }
        _ => { }
    };
}
static mut oc1: libc::c_int = 0;
static mut oc0: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn EmitMovEDXStack(mut vm: *mut vm_t,
                                         mut andit: libc::c_int) {
    if 0 == jlabel {
        if LastCommand as libc::c_uint ==
               LAST_COMMAND_MOV_STACK_EAX as libc::c_int as libc::c_uint {
            compiledOfs -= 3i32;
            *(*vm).instructionPointers.offset((instruction - 1i32) as isize) =
                compiledOfs as intptr_t;
            EmitString(b"8B D0\x00" as *const u8 as *const libc::c_char);
        } else if pop1 == OP_DIVI as libc::c_int ||
                      pop1 == OP_DIVU as libc::c_int ||
                      pop1 == OP_MULI as libc::c_int ||
                      pop1 == OP_MULU as libc::c_int ||
                      pop1 == OP_STORE4 as libc::c_int ||
                      pop1 == OP_STORE2 as libc::c_int ||
                      pop1 == OP_STORE1 as libc::c_int {
            EmitString(b"8B D0\x00" as *const u8 as *const libc::c_char);
        } else if pop1 == OP_CONST as libc::c_int &&
                      *buf.offset((compiledOfs - 7i32) as isize) as
                          libc::c_int == 0xc7i32 &&
                      *buf.offset((compiledOfs - 6i32) as isize) as
                          libc::c_int == 0x7i32 &&
                      *buf.offset((compiledOfs - 5i32) as isize) as
                          libc::c_int == 0x9fi32 {
            compiledOfs -= 7i32;
            *(*vm).instructionPointers.offset((instruction - 1i32) as isize) =
                compiledOfs as intptr_t;
            EmitString(b"BA\x00" as *const u8 as *const libc::c_char);
            if 0 != andit {
                Emit4(lastConst & andit);
            } else { Emit4(lastConst); }
            return
        } else {
            EmitString(b"8B 14 9F\x00" as *const u8 as *const libc::c_char);
        }
    } else {
        EmitString(b"8B 14 9F\x00" as *const u8 as *const libc::c_char);
    }
    if 0 != andit {
        EmitString(b"81\x00" as *const u8 as *const libc::c_char);
        EmitString(b"E2\x00" as *const u8 as *const libc::c_char);
        Emit4(andit);
    };
}
unsafe extern "C" fn EmitPushStack(mut vm: *mut vm_t) {
    if 0 == jlabel {
        if LastCommand as libc::c_uint ==
               LAST_COMMAND_SUB_BL_1 as libc::c_int as libc::c_uint {
            compiledOfs -= 3i32;
            *(*vm).instructionPointers.offset((instruction - 1i32) as isize) =
                compiledOfs as intptr_t;
            return
        }
        if LastCommand as libc::c_uint ==
               LAST_COMMAND_SUB_BL_2 as libc::c_int as libc::c_uint {
            compiledOfs -= 3i32;
            *(*vm).instructionPointers.offset((instruction - 1i32) as isize) =
                compiledOfs as intptr_t;
            EmitString(b"80 EB\x00" as *const u8 as *const libc::c_char);
            Emit1(1i32);
            return
        }
    }
    EmitString(b"80 C3\x00" as *const u8 as *const libc::c_char);
    Emit1(1i32);
}
unsafe extern "C" fn Constant1() -> libc::c_int {
    let mut v: libc::c_int = 0;
    v = *code.offset(pc as isize) as libc::c_int;
    pc += 1i32;
    return v;
}
/*
=================
ConstOptimize
Constant values for immediately following instructions may be translated to immediate values
instead of opStack operations, which will save expensive operations on memory
=================
*/
#[no_mangle]
pub unsafe extern "C" fn ConstOptimize(mut vm: *mut vm_t,
                                       mut callProcOfsSyscall: libc::c_int)
 -> qboolean {
    let mut v: libc::c_int = 0;
    let mut op1: libc::c_int = 0;
    if !(*vm).jumpTableTargets.is_null() &&
           0 == *jused.offset(instruction as isize) {
        op1 = *code.offset((pc + 4i32) as isize) as libc::c_int
    } else { return qfalse }
    match op1 {
        29 => {
            EmitPushStack(vm);
            EmitRexString(0x41i32 as byte,
                          b"8B 81\x00" as *const u8 as *const libc::c_char);
            Emit4(Constant4() & (*vm).dataMask);
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1i32;
            return qtrue
        }
        28 => {
            EmitPushStack(vm);
            EmitRexString(0x41i32 as byte,
                          b"0F B7 81\x00" as *const u8 as
                              *const libc::c_char);
            Emit4(Constant4() & (*vm).dataMask);
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1i32;
            return qtrue
        }
        27 => {
            EmitPushStack(vm);
            EmitRexString(0x41i32 as byte,
                          b"0F B6 81\x00" as *const u8 as
                              *const libc::c_char);
            Emit4(Constant4() & (*vm).dataMask);
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1i32;
            return qtrue
        }
        32 => {
            EmitMovEAXStack(vm, (*vm).dataMask);
            EmitRexString(0x41i32 as byte,
                          b"C7 04 01\x00" as *const u8 as
                              *const libc::c_char);
            Emit4(Constant4());
            EmitCommand(LAST_COMMAND_SUB_BL_1);
            pc += 1;
            instruction += 1i32;
            return qtrue
        }
        31 => {
            EmitMovEAXStack(vm, (*vm).dataMask);
            Emit1(0x66i32);
            EmitRexString(0x41i32 as byte,
                          b"C7 04 01\x00" as *const u8 as
                              *const libc::c_char);
            Emit2(Constant4());
            EmitCommand(LAST_COMMAND_SUB_BL_1);
            pc += 1;
            instruction += 1i32;
            return qtrue
        }
        30 => {
            EmitMovEAXStack(vm, (*vm).dataMask);
            EmitRexString(0x41i32 as byte,
                          b"C6 04 01\x00" as *const u8 as
                              *const libc::c_char);
            Emit1(Constant4());
            EmitCommand(LAST_COMMAND_SUB_BL_1);
            pc += 1;
            instruction += 1i32;
            return qtrue
        }
        38 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0i32);
            if 0 != iss8(v) {
                EmitString(b"83 C0\x00" as *const u8 as *const libc::c_char);
                Emit1(v);
            } else {
                EmitString(b"05\x00" as *const u8 as *const libc::c_char);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1i32;
            return qtrue
        }
        39 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0i32);
            if 0 != iss8(v) {
                EmitString(b"83 E8\x00" as *const u8 as *const libc::c_char);
                Emit1(v);
            } else {
                EmitString(b"2D\x00" as *const u8 as *const libc::c_char);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1i32;
            return qtrue
        }
        44 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0i32);
            if 0 != iss8(v) {
                EmitString(b"6B C0\x00" as *const u8 as *const libc::c_char);
                Emit1(v);
            } else {
                EmitString(b"69 C0\x00" as *const u8 as *const libc::c_char);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1i32;
            return qtrue
        }
        50 => {
            v = NextConstant4();
            if !(v < 0i32 || v > 31i32) {
                EmitMovEAXStack(vm, 0i32);
                EmitString(b"C1 E0\x00" as *const u8 as *const libc::c_char);
                Emit1(v);
                EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                pc += 5i32;
                instruction += 1i32;
                return qtrue
            }
        }
        51 => {
            v = NextConstant4();
            if !(v < 0i32 || v > 31i32) {
                EmitMovEAXStack(vm, 0i32);
                EmitString(b"C1 F8\x00" as *const u8 as *const libc::c_char);
                Emit1(v);
                EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                pc += 5i32;
                instruction += 1i32;
                return qtrue
            }
        }
        52 => {
            v = NextConstant4();
            if !(v < 0i32 || v > 31i32) {
                EmitMovEAXStack(vm, 0i32);
                EmitString(b"C1 E8\x00" as *const u8 as *const libc::c_char);
                Emit1(v);
                EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                pc += 5i32;
                instruction += 1i32;
                return qtrue
            }
        }
        46 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0i32);
            if 0 != iss8(v) {
                EmitString(b"83 E0\x00" as *const u8 as *const libc::c_char);
                Emit1(v);
            } else {
                EmitString(b"25\x00" as *const u8 as *const libc::c_char);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1i32;
            instruction += 1i32;
            return qtrue
        }
        47 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0i32);
            if 0 != iss8(v) {
                EmitString(b"83 C8\x00" as *const u8 as *const libc::c_char);
                Emit1(v);
            } else {
                EmitString(b"0D\x00" as *const u8 as *const libc::c_char);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1i32;
            instruction += 1i32;
            return qtrue
        }
        48 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0i32);
            if 0 != iss8(v) {
                EmitString(b"83 F0\x00" as *const u8 as *const libc::c_char);
                Emit1(v);
            } else {
                EmitString(b"35\x00" as *const u8 as *const libc::c_char);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1i32;
            instruction += 1i32;
            return qtrue
        }
        11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => {
            EmitMovEAXStack(vm, 0i32);
            EmitCommand(LAST_COMMAND_SUB_BL_1);
            EmitString(b"3D\x00" as *const u8 as *const libc::c_char);
            Emit4(Constant4());
            pc += 1;
            EmitBranchConditions(vm, op1);
            instruction += 1;
            return qtrue
        }
        21 | 22 => {
            if !(0 != NextConstant4()) {
                pc += 5i32;
                EmitMovEAXStack(vm, 0i32);
                EmitCommand(LAST_COMMAND_SUB_BL_1);
                EmitString(b"25\x00" as *const u8 as *const libc::c_char);
                Emit4(0x7fffffffi32);
                if op1 == OP_EQF as libc::c_int {
                    EmitJumpIns(vm,
                                b"0F 84\x00" as *const u8 as
                                    *const libc::c_char, Constant4());
                } else {
                    EmitJumpIns(vm,
                                b"0F 85\x00" as *const u8 as
                                    *const libc::c_char, Constant4());
                }
                instruction += 1i32;
                return qtrue
            }
        }
        10 => {
            EmitJumpIns(vm, b"E9\x00" as *const u8 as *const libc::c_char,
                        Constant4());
            pc += 1i32;
            instruction += 1i32;
            return qtrue
        }
        5 => {
            v = Constant4();
            EmitCallConst(vm, v, callProcOfsSyscall);
            pc += 1i32;
            instruction += 1i32;
            return qtrue
        }
        _ => { }
    }
    return qfalse;
}
/*
=================
EmitCallConst
Call to constant instruction number or syscall
=================
*/
#[no_mangle]
pub unsafe extern "C" fn EmitCallConst(mut vm: *mut vm_t,
                                       mut cdest: libc::c_int,
                                       mut callProcOfsSyscall: libc::c_int) {
    if cdest < 0i32 {
        EmitString(b"B8\x00" as *const u8 as *const libc::c_char);
        Emit4(cdest);
        EmitCallRel(vm, callProcOfsSyscall);
    } else { EmitCallIns(vm, cdest); };
}
/*
=================
EmitCallIns
Call to constant instruction number
=================
*/
#[no_mangle]
pub unsafe extern "C" fn EmitCallIns(mut vm: *mut vm_t,
                                     mut cdest: libc::c_int) {
    if cdest < 0i32 || cdest >= (*vm).instructionCount {
        Z_Free(buf as *mut libc::c_void);
        Z_Free(jused as *mut libc::c_void);
        Com_Error(ERR_DROP as libc::c_int,
                  b"VM_CompileX86: jump target out of range at offset %d\x00"
                      as *const u8 as *const libc::c_char, pc);
    }
    *jused.offset(cdest as isize) = 1i32 as byte;
    EmitString(b"E8\x00" as *const u8 as *const libc::c_char);
    if pass == 2i32 {
        Emit4((*(*vm).instructionPointers.offset(cdest as isize) -
                   compiledOfs as libc::c_long - 4i32 as libc::c_long) as
                  libc::c_int);
    } else { compiledOfs += 4i32 };
}
unsafe extern "C" fn iss8(mut v: int32_t) -> libc::c_int {
    return (-127i32 - 1i32 <= v && v <= 127i32) as libc::c_int;
}
unsafe extern "C" fn Emit2(mut v: libc::c_int) {
    Emit1(v & 255i32);
    Emit1(v >> 8i32 & 255i32);
}
/*
=================
EmitCallProcedure
VM OP_CALL procedure for call destinations obtained at runtime
=================
*/
#[no_mangle]
pub unsafe extern "C" fn EmitCallProcedure(mut vm: *mut vm_t,
                                           mut sysCallOfs: libc::c_int)
 -> libc::c_int {
    let mut jmpSystemCall: libc::c_int = 0;
    let mut jmpBadAddr: libc::c_int = 0;
    let mut retval: libc::c_int = 0;
    EmitString(b"8B 04 9F\x00" as *const u8 as *const libc::c_char);
    EmitString(b"80 EB\x00" as *const u8 as *const libc::c_char);
    Emit1(1i32);
    EmitString(b"85 C0\x00" as *const u8 as *const libc::c_char);
    EmitString(b"7C\x00" as *const u8 as *const libc::c_char);
    let fresh1 = compiledOfs;
    compiledOfs = compiledOfs + 1;
    jmpSystemCall = fresh1;
    EmitString(b"81 F8\x00" as *const u8 as *const libc::c_char);
    Emit4((*vm).instructionCount);
    EmitString(b"73\x00" as *const u8 as *const libc::c_char);
    let fresh2 = compiledOfs;
    compiledOfs = compiledOfs + 1;
    jmpBadAddr = fresh2;
    EmitRexString(0x49i32 as byte,
                  b"FF 14 C0\x00" as *const u8 as *const libc::c_char);
    EmitString(b"8B 04 9F\x00" as *const u8 as *const libc::c_char);
    EmitString(b"C3\x00" as *const u8 as *const libc::c_char);
    *buf.offset(jmpBadAddr as isize) =
        (compiledOfs - (jmpBadAddr + 1i32)) as byte;
    EmitCallErrJump(vm, sysCallOfs);
    *buf.offset(jmpSystemCall as isize) =
        (compiledOfs - (jmpSystemCall + 1i32)) as byte;
    retval = compiledOfs;
    EmitCallRel(vm, sysCallOfs);
    EmitString(b"80 C3\x00" as *const u8 as *const libc::c_char);
    Emit1(1i32);
    EmitString(b"C3\x00" as *const u8 as *const libc::c_char);
    return retval;
}
/*
=================
EmitCallDoSyscall
Call to DoSyscall()
=================
*/
#[no_mangle]
pub unsafe extern "C" fn EmitCallDoSyscall(mut vm: *mut vm_t) -> libc::c_int {
    EmitRexString(0x48i32 as byte,
                  b"BA\x00" as *const u8 as *const libc::c_char);
    EmitPtr(::std::mem::transmute::<Option<unsafe extern "C" fn() -> ()>,
                                    *mut libc::c_void>(Some(DoSyscall)));
    EmitString(b"51\x00" as *const u8 as *const libc::c_char);
    EmitString(b"56\x00" as *const u8 as *const libc::c_char);
    EmitString(b"57\x00" as *const u8 as *const libc::c_char);
    EmitRexString(0x41i32 as byte,
                  b"50\x00" as *const u8 as *const libc::c_char);
    EmitRexString(0x41i32 as byte,
                  b"51\x00" as *const u8 as *const libc::c_char);
    EmitString(b"A3\x00" as *const u8 as *const libc::c_char);
    EmitPtr(&mut vm_syscallNum as *mut libc::c_int as *mut libc::c_void);
    EmitString(b"89 F0\x00" as *const u8 as *const libc::c_char);
    EmitString(b"A3\x00" as *const u8 as *const libc::c_char);
    EmitPtr(&mut vm_programStack as *mut libc::c_int as *mut libc::c_void);
    EmitString(b"88 D8\x00" as *const u8 as *const libc::c_char);
    EmitString(b"A2\x00" as *const u8 as *const libc::c_char);
    EmitPtr(&mut vm_opStackOfs as *mut uint8_t as *mut libc::c_void);
    EmitRexString(0x48i32 as byte,
                  b"89 F8\x00" as *const u8 as *const libc::c_char);
    EmitRexString(0x48i32 as byte,
                  b"A3\x00" as *const u8 as *const libc::c_char);
    EmitPtr(&mut vm_opStackBase as *mut *mut libc::c_int as
                *mut libc::c_void);
    EmitString(b"89 C8\x00" as *const u8 as *const libc::c_char);
    EmitString(b"A3\x00" as *const u8 as *const libc::c_char);
    EmitPtr(&mut vm_arg as *mut intptr_t as *mut libc::c_void);
    EmitString(b"55\x00" as *const u8 as *const libc::c_char);
    EmitRexString(0x48i32 as byte,
                  b"89 E5\x00" as *const u8 as *const libc::c_char);
    EmitRexString(0x48i32 as byte,
                  b"83 E4 F0\x00" as *const u8 as *const libc::c_char);
    EmitString(b"FF D2\x00" as *const u8 as *const libc::c_char);
    EmitRexString(0x48i32 as byte,
                  b"89 EC\x00" as *const u8 as *const libc::c_char);
    EmitString(b"5D\x00" as *const u8 as *const libc::c_char);
    EmitRexString(0x41i32 as byte,
                  b"59\x00" as *const u8 as *const libc::c_char);
    EmitRexString(0x41i32 as byte,
                  b"58\x00" as *const u8 as *const libc::c_char);
    EmitString(b"5F\x00" as *const u8 as *const libc::c_char);
    EmitString(b"5E\x00" as *const u8 as *const libc::c_char);
    EmitString(b"59\x00" as *const u8 as *const libc::c_char);
    EmitString(b"C3\x00" as *const u8 as *const libc::c_char);
    return compiledOfs;
}
#[no_mangle]
pub static mut vm_arg: intptr_t = 0;
#[no_mangle]
pub static mut vm_opStackBase: *mut libc::c_int =
    0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut vm_opStackOfs: uint8_t = 0;
#[no_mangle]
pub static mut vm_programStack: libc::c_int = 0;
/*
=================
DoSyscall

Assembler helper routines will write its arguments directly to global variables so as to
work around different calling conventions
=================
*/
#[no_mangle]
pub static mut vm_syscallNum: libc::c_int = 0;
unsafe extern "C" fn DoSyscall() {
    let mut savedVM: *mut vm_t = 0 as *mut vm_t;
    savedVM = currentVM;
    (*currentVM).programStack = vm_programStack - 4i32;
    if vm_syscallNum < 0i32 {
        let mut data: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut ret: *mut libc::c_int = 0 as *mut libc::c_int;
        let mut index: libc::c_int = 0;
        let mut args: [intptr_t; 16] = [0; 16];
        data =
            (*savedVM).dataBase.offset(vm_programStack as
                                           isize).offset(4isize) as
                *mut libc::c_int;
        ret =
            &mut *vm_opStackBase.offset((vm_opStackOfs as libc::c_int + 1i32)
                                            as isize) as *mut libc::c_int;
        args[0usize] = !vm_syscallNum as intptr_t;
        index = 1i32;
        while (index as libc::c_ulong) <
                  (::std::mem::size_of::<[intptr_t; 16]>() as
                       libc::c_ulong).wrapping_div(::std::mem::size_of::<intptr_t>()
                                                       as libc::c_ulong) {
            args[index as usize] = *data.offset(index as isize) as intptr_t;
            index += 1
        }
        *ret =
            (*savedVM).systemCall.expect("non-null function pointer")(args.as_mut_ptr())
                as libc::c_int
    } else {
        match vm_syscallNum {
            0 => { ErrJump(); }
            1 => {
                if (vm_opStackOfs as libc::c_int) < 1i32 {
                    Com_Error(ERR_DROP as libc::c_int,
                              b"VM_BLOCK_COPY failed due to corrupted opStack\x00"
                                  as *const u8 as *const libc::c_char);
                }
                VM_BlockCopy(*vm_opStackBase.offset((vm_opStackOfs as
                                                         libc::c_int - 1i32)
                                                        as isize) as
                                 libc::c_uint,
                             *vm_opStackBase.offset(vm_opStackOfs as isize) as
                                 libc::c_uint, vm_arg as size_t);
            }
            _ => {
                Com_Error(ERR_DROP as libc::c_int,
                          b"Unknown VM operation %d\x00" as *const u8 as
                              *const libc::c_char, vm_syscallNum);
            }
        }
    }
    currentVM = savedVM;
}
/*
=================
ErrJump
Error handler for jump/call to invalid instruction number
=================
*/
unsafe extern "C" fn ErrJump() -> ! {
    Com_Error(ERR_DROP as libc::c_int,
              b"program tried to execute code outside VM\x00" as *const u8 as
                  *const libc::c_char);
}
static mut jusedSize: libc::c_int = 0i32;