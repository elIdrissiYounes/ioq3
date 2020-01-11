use ::libc;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::qfiles_h::vmHeader_t;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::common::Com_Printf;
pub use crate::src::qcommon::common::Q_VMftol;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__off_t;
pub use crate::stdlib::__uint8_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::intptr_t;
pub use crate::stdlib::uint8_t;

pub use crate::qcommon_h::vm_t;
pub use crate::src::qcommon::common::Z_Free;
pub use crate::src::qcommon::common::Z_Malloc;
pub use crate::src::qcommon::vm::currentVM;
pub use crate::src::qcommon::vm::VM_BlockCopy;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::mmap;
use crate::stdlib::mprotect;
use crate::stdlib::munmap;
pub use crate::vm_local_h::vmSymbol_s;
pub use crate::vm_local_h::vm_s;
pub use crate::vm_local_h::OP_ADD;
pub use crate::vm_local_h::OP_ADDF;
pub use crate::vm_local_h::OP_ARG;
pub use crate::vm_local_h::OP_BAND;
pub use crate::vm_local_h::OP_BCOM;
pub use crate::vm_local_h::OP_BLOCK_COPY;
pub use crate::vm_local_h::OP_BOR;
pub use crate::vm_local_h::OP_BREAK;
pub use crate::vm_local_h::OP_BXOR;
pub use crate::vm_local_h::OP_CALL;
pub use crate::vm_local_h::OP_CONST;
pub use crate::vm_local_h::OP_CVFI;
pub use crate::vm_local_h::OP_CVIF;
pub use crate::vm_local_h::OP_DIVF;
pub use crate::vm_local_h::OP_DIVI;
pub use crate::vm_local_h::OP_DIVU;
pub use crate::vm_local_h::OP_ENTER;
pub use crate::vm_local_h::OP_EQ;
pub use crate::vm_local_h::OP_EQF;
pub use crate::vm_local_h::OP_GEF;
pub use crate::vm_local_h::OP_GEI;
pub use crate::vm_local_h::OP_GEU;
pub use crate::vm_local_h::OP_GTF;
pub use crate::vm_local_h::OP_GTI;
pub use crate::vm_local_h::OP_GTU;
pub use crate::vm_local_h::OP_IGNORE;
pub use crate::vm_local_h::OP_JUMP;
pub use crate::vm_local_h::OP_LEAVE;
pub use crate::vm_local_h::OP_LEF;
pub use crate::vm_local_h::OP_LEI;
pub use crate::vm_local_h::OP_LEU;
pub use crate::vm_local_h::OP_LOAD1;
pub use crate::vm_local_h::OP_LOAD2;
pub use crate::vm_local_h::OP_LOAD4;
pub use crate::vm_local_h::OP_LOCAL;
pub use crate::vm_local_h::OP_LSH;
pub use crate::vm_local_h::OP_LTF;
pub use crate::vm_local_h::OP_LTI;
pub use crate::vm_local_h::OP_LTU;
pub use crate::vm_local_h::OP_MODI;
pub use crate::vm_local_h::OP_MODU;
pub use crate::vm_local_h::OP_MULF;
pub use crate::vm_local_h::OP_MULI;
pub use crate::vm_local_h::OP_MULU;
pub use crate::vm_local_h::OP_NE;
pub use crate::vm_local_h::OP_NEF;
pub use crate::vm_local_h::OP_NEGF;
pub use crate::vm_local_h::OP_NEGI;
pub use crate::vm_local_h::OP_POP;
pub use crate::vm_local_h::OP_PUSH;
pub use crate::vm_local_h::OP_RSHI;
pub use crate::vm_local_h::OP_RSHU;
pub use crate::vm_local_h::OP_SEX16;
pub use crate::vm_local_h::OP_SEX8;
pub use crate::vm_local_h::OP_STORE1;
pub use crate::vm_local_h::OP_STORE2;
pub use crate::vm_local_h::OP_STORE4;
pub use crate::vm_local_h::OP_SUB;
pub use crate::vm_local_h::OP_SUBF;
pub use crate::vm_local_h::OP_UNDEF;

pub type ELastCommand = u32;

pub const LAST_COMMAND_SUB_BL_2: ELastCommand = 3;

pub const LAST_COMMAND_SUB_BL_1: ELastCommand = 2;

pub const LAST_COMMAND_MOV_STACK_EAX: ELastCommand = 1;

pub const LAST_COMMAND_NONE: ELastCommand = 0;

pub const VM_JMP_VIOLATION: C2RustUnnamed_143 = 0;

pub const VM_BLOCK_COPY: C2RustUnnamed_143 = 1;

pub type C2RustUnnamed_143 = u32;
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

static mut buf: *mut crate::src::qcommon::q_shared::byte =
    0 as *mut crate::src::qcommon::q_shared::byte;

static mut jused: *mut crate::src::qcommon::q_shared::byte =
    0 as *mut crate::src::qcommon::q_shared::byte;

static mut jusedSize: i32 = 0;

static mut compiledOfs: i32 = 0;

static mut code: *mut crate::src::qcommon::q_shared::byte =
    0 as *mut crate::src::qcommon::q_shared::byte;

static mut pc: i32 = 0;

static mut instruction: i32 = 0;

static mut pass: i32 = 0;

static mut lastConst: i32 = 0;

static mut oc0: i32 = 0;

static mut oc1: i32 = 0;

static mut pop0: i32 = 0;

static mut pop1: i32 = 0;

static mut jlabel: i32 = 0;

static mut LastCommand: ELastCommand = LAST_COMMAND_NONE;

unsafe extern "C" fn iss8(mut v: crate::stdlib::int32_t) -> i32 {
    return (-(127) - 1 <= v && v <= 127) as i32;
}

unsafe extern "C" fn NextConstant4() -> i32 {
    return (*code.offset(pc as isize) as u32
        | (*code.offset((pc + 1) as isize) as u32) << 8
        | (*code.offset((pc + 2) as isize) as u32) << 16
        | (*code.offset((pc + 3) as isize) as u32) << 24) as i32;
}

unsafe extern "C" fn Constant4() -> i32 {
    let mut v: i32 = 0;
    v = NextConstant4();
    pc += 4;
    return v;
}

unsafe extern "C" fn Constant1() -> i32 {
    let mut v: i32 = 0;
    v = *code.offset(pc as isize) as i32;
    pc += 1;
    return v;
}

unsafe extern "C" fn Emit1(mut v: i32) {
    *buf.offset(compiledOfs as isize) = v as crate::src::qcommon::q_shared::byte;
    compiledOfs += 1;
    LastCommand = LAST_COMMAND_NONE;
}

unsafe extern "C" fn Emit2(mut v: i32) {
    Emit1(v & 255);
    Emit1(v >> 8 & 255);
}

unsafe extern "C" fn Emit4(mut v: i32) {
    Emit1(v & 0xff);
    Emit1(v >> 8 & 0xff);
    Emit1(v >> 16 & 0xff);
    Emit1(v >> 24 & 0xff);
}

unsafe extern "C" fn EmitPtr(mut ptr: *mut libc::c_void) {
    let mut v: crate::stdlib::intptr_t = ptr as crate::stdlib::intptr_t;
    Emit4(v as i32);
    Emit1((v >> 32 & 0xff) as i32);
    Emit1((v >> 40 & 0xff) as i32);
    Emit1((v >> 48 & 0xff) as i32);
    Emit1((v >> 56 & 0xff) as i32);
}

unsafe extern "C" fn Hex(mut c: i32) -> i32 {
    if c >= 'a' as i32 && c <= 'f' as i32 {
        return 10i32 + c - 'a' as i32;
    }
    if c >= 'A' as i32 && c <= 'F' as i32 {
        return 10i32 + c - 'A' as i32;
    }
    if c >= '0' as i32 && c <= '9' as i32 {
        return c - '0' as i32;
    }
    crate::src::qcommon::common::Z_Free(buf as *mut libc::c_void);
    crate::src::qcommon::common::Z_Free(jused as *mut libc::c_void);
    crate::src::qcommon::common::Com_Error(
        crate::src::qcommon::q_shared::ERR_DROP as i32,
        b"Hex: bad char \'%c\'\x00" as *const u8 as *const i8,
        c,
    );
}

unsafe extern "C" fn EmitString(mut string: *const i8) {
    let mut c1: i32 = 0;
    let mut c2: i32 = 0;
    let mut v: i32 = 0;
    loop {
        c1 = *string.offset(0) as i32;
        c2 = *string.offset(1) as i32;
        v = Hex(c1) << 4 | Hex(c2);
        Emit1(v);
        if *string.offset(2) == 0 {
            break;
        }
        string = string.offset(3)
    }
}

unsafe extern "C" fn EmitRexString(
    mut rex: crate::src::qcommon::q_shared::byte,
    mut string: *const i8,
) {
    if rex != 0 {
        Emit1(rex as i32);
    }
    EmitString(string);
}
// add bl, bytes
// sub bl, bytes

unsafe extern "C" fn EmitCommand(mut command: ELastCommand) {
    match command {
        1 => {
            EmitString(b"89 04 9F\x00" as *const u8 as *const i8); // mov dword ptr [edi + ebx * 4], eax
        }
        2 => {
            EmitString(b"80 EB\x00" as *const u8 as *const i8); // sub bl, 1
            Emit1(1i32); // sub bl, 2
        }
        3 => {
            EmitString(b"80 EB\x00" as *const u8 as *const i8);
            Emit1(2i32);
        }
        _ => {}
    }
    LastCommand = command;
}

unsafe extern "C" fn EmitPushStack(mut vm: *mut crate::qcommon_h::vm_t) {
    if jlabel == 0 {
        if LastCommand == LAST_COMMAND_SUB_BL_1 {
            // sub bl, 1
            compiledOfs -= 3;
            *(*vm).instructionPointers.offset((instruction - 1) as isize) =
                compiledOfs as crate::stdlib::intptr_t;
            return;
        }
        if LastCommand == LAST_COMMAND_SUB_BL_2 {
            // sub bl, 2
            compiledOfs -= 3; //	sub bl, 1
            *(*vm).instructionPointers.offset((instruction - 1) as isize) =
                compiledOfs as crate::stdlib::intptr_t;
            EmitString(b"80 EB\x00" as *const u8 as *const i8);
            Emit1(1);
            return;
        }
    }
    EmitString(b"80 C3\x00" as *const u8 as *const i8);
    Emit1(1);
    // add bl, 1
}

unsafe extern "C" fn EmitMovEAXStack(mut vm: *mut crate::qcommon_h::vm_t, mut andit: i32) {
    if jlabel == 0 {
        if LastCommand == LAST_COMMAND_MOV_STACK_EAX {
            // mov eax, dword ptr [edi + ebx * 4]
            // mov [edi + ebx * 4], eax
            compiledOfs -= 3;
            *(*vm).instructionPointers.offset((instruction - 1) as isize) =
                compiledOfs as crate::stdlib::intptr_t
        } else if pop1 == crate::vm_local_h::OP_CONST as i32
            && *buf.offset((compiledOfs - 7) as isize) as i32 == 0xc7
            && *buf.offset((compiledOfs - 6) as isize) as i32 == 0x4
            && *buf.offset((compiledOfs - 5) as isize) as i32 == 0x9f
        {
            // mov [edi + ebx * 4], 0x12345678
            compiledOfs -= 7; // mov	eax, 0x12345678
            *(*vm).instructionPointers.offset((instruction - 1) as isize) =
                compiledOfs as crate::stdlib::intptr_t;
            EmitString(b"B8\x00" as *const u8 as *const i8);
            if andit != 0 {
                Emit4(lastConst & andit);
            } else {
                Emit4(lastConst);
            }
            return;
        } else {
            if pop1 != crate::vm_local_h::OP_DIVI as i32
                && pop1 != crate::vm_local_h::OP_DIVU as i32
                && pop1 != crate::vm_local_h::OP_MULI as i32
                && pop1 != crate::vm_local_h::OP_MULU as i32
                && pop1 != crate::vm_local_h::OP_STORE4 as i32
                && pop1 != crate::vm_local_h::OP_STORE2 as i32
                && pop1 != crate::vm_local_h::OP_STORE1 as i32
            {
                EmitString(b"8B 04 9F\x00" as *const u8 as *const i8);
                // mov eax, dword ptr [edi + ebx * 4]
            }
        }
    } else {
        EmitString(b"8B 04 9F\x00" as *const u8 as *const i8); // and eax, 0x12345678
    }
    if andit != 0 {
        EmitString(b"25\x00" as *const u8 as *const i8);
        Emit4(andit);
    };
}
#[no_mangle]

pub unsafe extern "C" fn EmitMovECXStack(mut vm: *mut crate::qcommon_h::vm_t) {
    if jlabel == 0 {
        if LastCommand == LAST_COMMAND_MOV_STACK_EAX {
            // mov [edi + ebx * 4], eax
            compiledOfs -= 3; // mov ecx, eax
            *(*vm).instructionPointers.offset((instruction - 1) as isize) =
                compiledOfs as crate::stdlib::intptr_t; // mov ecx, eax
            EmitString(b"89 C1\x00" as *const u8 as *const i8);
            return;
        }
        if pop1 == crate::vm_local_h::OP_DIVI as i32
            || pop1 == crate::vm_local_h::OP_DIVU as i32
            || pop1 == crate::vm_local_h::OP_MULI as i32
            || pop1 == crate::vm_local_h::OP_MULU as i32
            || pop1 == crate::vm_local_h::OP_STORE4 as i32
            || pop1 == crate::vm_local_h::OP_STORE2 as i32
            || pop1 == crate::vm_local_h::OP_STORE1 as i32
        {
            EmitString(b"89 C1\x00" as *const u8 as *const i8);
            return;
        }
    }
    EmitString(b"8B 0C 9F\x00" as *const u8 as *const i8);
    // mov ecx, dword ptr [edi + ebx * 4]
}
#[no_mangle]

pub unsafe extern "C" fn EmitMovEDXStack(mut vm: *mut crate::qcommon_h::vm_t, mut andit: i32) {
    if jlabel == 0 {
        if LastCommand == LAST_COMMAND_MOV_STACK_EAX {
            // mov edx, dword ptr [edi + ebx * 4]
            // mov dword ptr [edi + ebx * 4], eax
            compiledOfs -= 3;
            *(*vm).instructionPointers.offset((instruction - 1) as isize) =
                compiledOfs as crate::stdlib::intptr_t;
            EmitString(b"8B D0\x00" as *const u8 as *const i8);
        // mov edx, eax
        } else if pop1 == crate::vm_local_h::OP_DIVI as i32
            || pop1 == crate::vm_local_h::OP_DIVU as i32
            || pop1 == crate::vm_local_h::OP_MULI as i32
            || pop1 == crate::vm_local_h::OP_MULU as i32
            || pop1 == crate::vm_local_h::OP_STORE4 as i32
            || pop1 == crate::vm_local_h::OP_STORE2 as i32
            || pop1 == crate::vm_local_h::OP_STORE1 as i32
        {
            EmitString(b"8B D0\x00" as *const u8 as *const i8);
        // mov edx, eax
        } else if pop1 == crate::vm_local_h::OP_CONST as i32
            && *buf.offset((compiledOfs - 7) as isize) as i32 == 0xc7
            && *buf.offset((compiledOfs - 6) as isize) as i32 == 0x7
            && *buf.offset((compiledOfs - 5) as isize) as i32 == 0x9f
        {
            // mov dword ptr [edi + ebx * 4], 0x12345678
            compiledOfs -= 7; // mov edx, 0x12345678
            *(*vm).instructionPointers.offset((instruction - 1) as isize) =
                compiledOfs as crate::stdlib::intptr_t;
            EmitString(b"BA\x00" as *const u8 as *const i8);
            if andit != 0 {
                Emit4(lastConst & andit);
            } else {
                Emit4(lastConst);
            }
            return;
        } else {
            EmitString(b"8B 14 9F\x00" as *const u8 as *const i8);
        }
    // mov edx, dword ptr [edi + ebx * 4]
    } else {
        EmitString(b"8B 14 9F\x00" as *const u8 as *const i8);
    }
    if andit != 0 {
        EmitString(b"81\x00" as *const u8 as *const i8);
        EmitString(b"E2\x00" as *const u8 as *const i8);
        Emit4(andit);
    };
    // and edx, 0x12345678
}
/*
=================
ErrJump
Error handler for jump/call to invalid instruction number
=================
*/

unsafe extern "C" fn ErrJump() -> ! {
    crate::src::qcommon::common::Com_Error(
        crate::src::qcommon::q_shared::ERR_DROP as i32,
        b"program tried to execute code outside VM\x00" as *const u8 as *const i8,
    );
}
/*
=================
DoSyscall

Assembler helper routines will write its arguments directly to global variables so as to
work around different calling conventions
=================
*/
#[no_mangle]

pub static mut vm_syscallNum: i32 = 0;
#[no_mangle]

pub static mut vm_programStack: i32 = 0;
#[no_mangle]

pub static mut vm_opStackBase: *mut i32 = 0 as *mut i32;
#[no_mangle]

pub static mut vm_opStackOfs: crate::stdlib::uint8_t = 0;
#[no_mangle]

pub static mut vm_arg: crate::stdlib::intptr_t = 0;

unsafe extern "C" fn DoSyscall() {
    let mut savedVM: *mut crate::qcommon_h::vm_t = 0 as *mut crate::qcommon_h::vm_t;
    // save currentVM so as to allow for recursive VM entry
    savedVM = crate::src::qcommon::vm::currentVM;
    // modify VM stack pointer for recursive VM entry
    (*crate::src::qcommon::vm::currentVM).programStack = vm_programStack - 4;
    if vm_syscallNum < 0 {
        let mut data: *mut i32 = 0 as *mut i32;
        let mut ret: *mut i32 = 0 as *mut i32;
        let mut index: i32 = 0;
        let mut args: [crate::stdlib::intptr_t; 16] = [0; 16];
        data = (*savedVM)
            .dataBase
            .offset(vm_programStack as isize)
            .offset(4) as *mut i32;
        ret = &mut *vm_opStackBase.offset((vm_opStackOfs as i32 + 1) as isize) as *mut i32;
        args[0] = !vm_syscallNum as crate::stdlib::intptr_t;
        index = 1;
        while (index as usize)
            < (::std::mem::size_of::<[crate::stdlib::intptr_t; 16]>())
                .wrapping_div(::std::mem::size_of::<crate::stdlib::intptr_t>())
        {
            args[index as usize] = *data.offset(index as isize) as crate::stdlib::intptr_t;
            index += 1
        }
        *ret = (*savedVM).systemCall.expect("non-null function pointer")(args.as_mut_ptr()) as i32
    } else {
        match vm_syscallNum {
            0 => {
                ErrJump();
            }
            1 => {
                if (vm_opStackOfs as i32) < 1 {
                    crate::src::qcommon::common::Com_Error(
                        crate::src::qcommon::q_shared::ERR_DROP as i32,
                        b"VM_BLOCK_COPY failed due to corrupted opStack\x00" as *const u8
                            as *const i8,
                    );
                }
                crate::src::qcommon::vm::VM_BlockCopy(
                    *vm_opStackBase.offset((vm_opStackOfs as i32 - 1i32) as isize) as u32,
                    *vm_opStackBase.offset(vm_opStackOfs as isize) as u32,
                    vm_arg as crate::stddef_h::size_t,
                );
            }
            _ => {
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as i32,
                    b"Unknown VM operation %d\x00" as *const u8 as *const i8,
                    vm_syscallNum,
                );
            }
        }
    }
    crate::src::qcommon::vm::currentVM = savedVM;
}
/*
=================
EmitCallRel
Relative call to vm->codeBase + callOfs
=================
*/
#[no_mangle]

pub unsafe extern "C" fn EmitCallRel(mut vm: *mut crate::qcommon_h::vm_t, mut callOfs: i32) {
    EmitString(b"E8\x00" as *const u8 as *const i8); // call 0x12345678
    Emit4(callOfs - compiledOfs - 4);
}
/*
=================
EmitCallDoSyscall
Call to DoSyscall()
=================
*/
#[no_mangle]

pub unsafe extern "C" fn EmitCallDoSyscall(mut vm: *mut crate::qcommon_h::vm_t) -> i32 {
    // use edx register to store DoSyscall address
    EmitRexString(0x48, b"BA\x00" as *const u8 as *const i8); // mov edx, DoSyscall
    EmitPtr(::std::mem::transmute::<
        Option<unsafe extern "C" fn() -> ()>,
        *mut libc::c_void,
    >(Some(DoSyscall as unsafe extern "C" fn() -> ())));
    // Push important registers to stack as we can't really make
    // any assumptions about calling conventions.
    EmitString(b"51\x00" as *const u8 as *const i8); // push ebx
    EmitString(b"56\x00" as *const u8 as *const i8); // push esi
    EmitString(b"57\x00" as *const u8 as *const i8); // push edi
    EmitRexString(0x41, b"50\x00" as *const u8 as *const i8); // push r8
    EmitRexString(0x41, b"51\x00" as *const u8 as *const i8); // push r9
                                                              // write arguments to global vars
                                                              // syscall number
    EmitString(b"A3\x00" as *const u8 as *const i8); // mov [0x12345678], eax
    EmitPtr(&mut vm_syscallNum as *mut i32 as *mut libc::c_void);
    // vm_programStack value
    EmitString(b"89 F0\x00" as *const u8 as *const i8); // mov eax, esi
    EmitString(b"A3\x00" as *const u8 as *const i8); // mov [0x12345678], eax
    EmitPtr(&mut vm_programStack as *mut i32 as *mut libc::c_void);
    // vm_opStackOfs
    EmitString(b"88 D8\x00" as *const u8 as *const i8); // mov al, bl
    EmitString(b"A2\x00" as *const u8 as *const i8); // mov [0x12345678], al
    EmitPtr(&mut vm_opStackOfs as *mut crate::stdlib::uint8_t as *mut libc::c_void);
    // vm_opStackBase
    EmitRexString(0x48, b"89 F8\x00" as *const u8 as *const i8); // mov eax, edi
    EmitRexString(0x48, b"A3\x00" as *const u8 as *const i8); // mov [0x12345678], eax
    EmitPtr(&mut vm_opStackBase as *mut *mut i32 as *mut libc::c_void);
    // vm_arg
    EmitString(b"89 C8\x00" as *const u8 as *const i8); // mov eax, ecx
    EmitString(b"A3\x00" as *const u8 as *const i8); // mov [0x12345678], eax
    EmitPtr(&mut vm_arg as *mut crate::stdlib::intptr_t as *mut libc::c_void);
    // align the stack pointer to a 16-byte-boundary
    EmitString(b"55\x00" as *const u8 as *const i8); // push ebp
    EmitRexString(0x48, b"89 E5\x00" as *const u8 as *const i8); // mov ebp, esp
    EmitRexString(0x48, b"83 E4 F0\x00" as *const u8 as *const i8); // and esp, 0xFFFFFFF0
                                                                    // call the syscall wrapper function DoSyscall()
    EmitString(b"FF D2\x00" as *const u8 as *const i8); // call edx
                                                        // reset the stack pointer to its previous value
    EmitRexString(0x48, b"89 EC\x00" as *const u8 as *const i8); // mov esp, ebp
    EmitString(b"5D\x00" as *const u8 as *const i8); // pop ebp
    EmitRexString(0x41, b"59\x00" as *const u8 as *const i8); // pop r9
    EmitRexString(0x41, b"58\x00" as *const u8 as *const i8); // pop r8
    EmitString(b"5F\x00" as *const u8 as *const i8); // pop edi
    EmitString(b"5E\x00" as *const u8 as *const i8); // pop esi
    EmitString(b"59\x00" as *const u8 as *const i8); // pop ebx
    EmitString(b"C3\x00" as *const u8 as *const i8); // ret
    return compiledOfs;
}
/*
=================
EmitCallErrJump
Emit the code that triggers execution of the jump violation handler
=================
*/

unsafe extern "C" fn EmitCallErrJump(mut vm: *mut crate::qcommon_h::vm_t, mut sysCallOfs: i32) {
    EmitString(b"B8\x00" as *const u8 as *const i8); // mov eax, 0x12345678
    Emit4(VM_JMP_VIOLATION as i32);
    EmitCallRel(vm, sysCallOfs);
}
/*
=================
EmitCallProcedure
VM OP_CALL procedure for call destinations obtained at runtime
=================
*/
#[no_mangle]

pub unsafe extern "C" fn EmitCallProcedure(
    mut vm: *mut crate::qcommon_h::vm_t,
    mut sysCallOfs: i32,
) -> i32 {
    let mut jmpSystemCall: i32 = 0; // mov eax, dword ptr [edi + ebx * 4]
    let mut jmpBadAddr: i32 = 0; // sub bl, 1
    let mut retval: i32 = 0; // test eax, eax
    EmitString(b"8B 04 9F\x00" as *const u8 as *const i8);
    EmitString(b"80 EB\x00" as *const u8 as *const i8);
    Emit1(1);
    EmitString(b"85 C0\x00" as *const u8 as *const i8);
    // Jump to syscall code, 1 byte offset should suffice
    EmitString(b"7C\x00" as *const u8 as *const i8); // jl systemCall
    let fresh0 = compiledOfs;
    compiledOfs = compiledOfs + 1;
    jmpSystemCall = fresh0;
    /* *********** Call inside VM ************/
    EmitString(b"81 F8\x00" as *const u8 as *const i8); // cmp eax, vm->instructionCount
    Emit4((*vm).instructionCount);
    // Error jump if invalid jump target
    EmitString(b"73\x00" as *const u8 as *const i8); // jae badAddr
    let fresh1 = compiledOfs; // call qword ptr [r8 + eax * 8]
    compiledOfs = compiledOfs + 1; // mov eax, dword ptr [edi + ebx * 4]
    jmpBadAddr = fresh1; // ret
    EmitRexString(0x49, b"FF 14 C0\x00" as *const u8 as *const i8);
    EmitString(b"8B 04 9F\x00" as *const u8 as *const i8);
    EmitString(b"C3\x00" as *const u8 as *const i8);
    // badAddr:
    *buf.offset(jmpBadAddr as isize) =
        (compiledOfs - (jmpBadAddr + 1)) as crate::src::qcommon::q_shared::byte;
    EmitCallErrJump(vm, sysCallOfs);
    /* *********** System Call ************/
    // systemCall:
    *buf.offset(jmpSystemCall as isize) =
        (compiledOfs - (jmpSystemCall + 1)) as crate::src::qcommon::q_shared::byte;
    retval = compiledOfs;
    EmitCallRel(vm, sysCallOfs);
    // have opStack reg point at return value
    EmitString(b"80 C3\x00" as *const u8 as *const i8); // add bl, 1
    Emit1(1); // ret
    EmitString(b"C3\x00" as *const u8 as *const i8);
    return retval;
}
/*
=================
EmitJumpIns
Jump to constant instruction number
=================
*/
#[no_mangle]

pub unsafe extern "C" fn EmitJumpIns(
    mut vm: *mut crate::qcommon_h::vm_t,
    mut jmpop: *const i8,
    mut cdest: i32,
) {
    if cdest < 0 || cdest >= (*vm).instructionCount {
        crate::src::qcommon::common::Z_Free(buf as *mut libc::c_void); // j??? 0x12345678
        crate::src::qcommon::common::Z_Free(jused as *mut libc::c_void);
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"VM_CompileX86: jump target out of range at offset %d\x00" as *const u8 as *const i8,
            pc,
        );
    }
    *jused.offset(cdest as isize) = 1u8;
    EmitString(jmpop);
    // we only know all the jump addresses in the third pass
    if pass == 2 {
        Emit4(
            (*(*vm).instructionPointers.offset(cdest as isize) - compiledOfs as isize - 4isize)
                as i32,
        );
    } else {
        compiledOfs += 4
    };
}
/*
=================
EmitCallIns
Call to constant instruction number
=================
*/
#[no_mangle]

pub unsafe extern "C" fn EmitCallIns(mut vm: *mut crate::qcommon_h::vm_t, mut cdest: i32) {
    if cdest < 0 || cdest >= (*vm).instructionCount {
        crate::src::qcommon::common::Z_Free(buf as *mut libc::c_void); // call 0x12345678
        crate::src::qcommon::common::Z_Free(jused as *mut libc::c_void);
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"VM_CompileX86: jump target out of range at offset %d\x00" as *const u8 as *const i8,
            pc,
        );
    }
    *jused.offset(cdest as isize) = 1u8;
    EmitString(b"E8\x00" as *const u8 as *const i8);
    // we only know all the jump addresses in the third pass
    if pass == 2 {
        Emit4(
            (*(*vm).instructionPointers.offset(cdest as isize) - compiledOfs as isize - 4isize)
                as i32,
        );
    } else {
        compiledOfs += 4
    };
}
/*
=================
EmitCallConst
Call to constant instruction number or syscall
=================
*/
#[no_mangle]

pub unsafe extern "C" fn EmitCallConst(
    mut vm: *mut crate::qcommon_h::vm_t,
    mut cdest: i32,
    mut callProcOfsSyscall: i32,
) {
    if cdest < 0 {
        EmitString(b"B8\x00" as *const u8 as *const i8); // mov eax, cdest
        Emit4(cdest);
        EmitCallRel(vm, callProcOfsSyscall);
    } else {
        EmitCallIns(vm, cdest);
    };
}
/*
=================
EmitBranchConditions
Emits x86 branch condition as given in op
=================
*/
#[no_mangle]

pub unsafe extern "C" fn EmitBranchConditions(mut vm: *mut crate::qcommon_h::vm_t, mut op: i32) {
    match op {
        11 => {
            EmitJumpIns(vm, b"0F 84\x00" as *const u8 as *const i8, Constant4());
            // je 0x12345678
        }
        12 => {
            EmitJumpIns(vm, b"0F 85\x00" as *const u8 as *const i8, Constant4());
            // jne 0x12345678
        }
        13 => {
            EmitJumpIns(vm, b"0F 8C\x00" as *const u8 as *const i8, Constant4());
            // jl 0x12345678
        }
        14 => {
            EmitJumpIns(vm, b"0F 8E\x00" as *const u8 as *const i8, Constant4());
            // jle 0x12345678
        }
        15 => {
            EmitJumpIns(vm, b"0F 8F\x00" as *const u8 as *const i8, Constant4());
            // jg 0x12345678
        }
        16 => {
            EmitJumpIns(vm, b"0F 8D\x00" as *const u8 as *const i8, Constant4());
            // jge 0x12345678
        }
        17 => {
            EmitJumpIns(vm, b"0F 82\x00" as *const u8 as *const i8, Constant4());
            // jb 0x12345678
        }
        18 => {
            EmitJumpIns(vm, b"0F 86\x00" as *const u8 as *const i8, Constant4());
            // jbe 0x12345678
        }
        19 => {
            EmitJumpIns(vm, b"0F 87\x00" as *const u8 as *const i8, Constant4());
            // ja 0x12345678
        }
        20 => {
            EmitJumpIns(vm, b"0F 83\x00" as *const u8 as *const i8, Constant4());
            // jae 0x12345678
        }
        _ => {}
    };
}
/*
=================
ConstOptimize
Constant values for immediately following instructions may be translated to immediate values
instead of opStack operations, which will save expensive operations on memory
=================
*/
#[no_mangle]

pub unsafe extern "C" fn ConstOptimize(
    mut vm: *mut crate::qcommon_h::vm_t,
    mut callProcOfsSyscall: i32,
) -> crate::src::qcommon::q_shared::qboolean {
    let mut v: i32 = 0;
    let mut op1: i32 = 0;
    // we can safely perform optimizations only in case if
    // we are 100% sure that next instruction is not a jump label
    if !(*vm).jumpTableTargets.is_null() && *jused.offset(instruction as isize) == 0 {
        op1 = *code.offset((pc + 4) as isize) as i32
    } else {
        return crate::src::qcommon::q_shared::qfalse;
    } // mov eax, dword ptr [r9 + 0x12345678]
    match op1 {
        29 => {
            EmitPushStack(vm); // mov dword ptr [edi + ebx * 4], eax
            EmitRexString(0x41, b"8B 81\x00" as *const u8 as *const i8); // OP_LOAD4
            Emit4(Constant4() & (*vm).dataMask); // movzx eax, word ptr [r9 + 0x12345678]
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // mov dword ptr [edi + ebx * 4], eax
            pc += 1; // OP_LOAD2
            instruction += 1; // movzx eax, byte ptr [r9 + 0x12345678]
            return crate::src::qcommon::q_shared::qtrue;
        }
        28 => {
            EmitPushStack(vm); // mov dword ptr [edi + ebx * 4], eax
            EmitRexString(0x41, b"0F B7 81\x00" as *const u8 as *const i8); // OP_LOAD1
            Emit4(Constant4() & (*vm).dataMask); // mov dword ptr [r9 + eax], 0x12345678
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // sub bl, 1
            pc += 1; // OP_STORE4
            instruction += 1; // mov word ptr [r9 + eax], 0x1234
            return crate::src::qcommon::q_shared::qtrue;
        }
        27 => {
            EmitPushStack(vm); // sub bl, 1
            EmitRexString(0x41, b"0F B6 81\x00" as *const u8 as *const i8); // OP_STORE2
            Emit4(Constant4() & (*vm).dataMask); // mov byte [r9 + eax], 0x12
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // sub bl, 1
            pc += 1; // OP_STORE1
            instruction += 1; // add eax, 0x7F
            return crate::src::qcommon::q_shared::qtrue;
        }
        32 => {
            EmitMovEAXStack(vm, (*vm).dataMask); // add eax, 0x12345678
            EmitRexString(0x41, b"C7 04 01\x00" as *const u8 as *const i8); // OP_ADD
            Emit4(Constant4()); // sub eax, 0x7F
            EmitCommand(LAST_COMMAND_SUB_BL_1); // sub eax, 0x12345678
            pc += 1; // OP_SUB
            instruction += 1; // imul eax, 0x7F
            return crate::src::qcommon::q_shared::qtrue;
        }
        31 => {
            EmitMovEAXStack(vm, (*vm).dataMask); // imul eax, 0x12345678
            Emit1(0x66); // OP_MULI
            EmitRexString(0x41, b"C7 04 01\x00" as *const u8 as *const i8); // shl eax, 0x12
            Emit2(Constant4()); // CONST + OP_LSH
            EmitCommand(LAST_COMMAND_SUB_BL_1); // sar eax, 0x12
            pc += 1; // CONST + OP_RSHI
            instruction += 1; // shr eax, 0x12
            return crate::src::qcommon::q_shared::qtrue;
        }
        30 => {
            EmitMovEAXStack(vm, (*vm).dataMask); // CONST + OP_RSHU
            EmitRexString(0x41, b"C6 04 01\x00" as *const u8 as *const i8); // and eax, 0x7F
            Emit1(Constant4()); // and eax, 0x12345678
            EmitCommand(LAST_COMMAND_SUB_BL_1); // OP_BAND
            pc += 1; // or eax, 0x7F
            instruction += 1; // or eax, 0x12345678
            return crate::src::qcommon::q_shared::qtrue;
        }
        38 => {
            v = Constant4(); // OP_BOR
            EmitMovEAXStack(vm, 0); // xor eax, 0x7F
            if iss8(v) != 0 {
                EmitString(b"83 C0\x00" as *const u8 as *const i8); // xor eax, 0x12345678
                Emit1(v); // OP_BXOR
            } else {
                EmitString(b"05\x00" as *const u8 as *const i8); // cmp eax, 0x12345678
                Emit4(v); // OP_*
            } // CONST + OP_EQF|OP_NEF
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1;
            return crate::src::qcommon::q_shared::qtrue;
        }
        39 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0);
            if iss8(v) != 0 {
                EmitString(b"83 E8\x00" as *const u8 as *const i8);
                Emit1(v);
            } else {
                EmitString(b"2D\x00" as *const u8 as *const i8);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1;
            return crate::src::qcommon::q_shared::qtrue;
        }
        44 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0);
            if iss8(v) != 0 {
                EmitString(b"6B C0\x00" as *const u8 as *const i8);
                Emit1(v);
            } else {
                EmitString(b"69 C0\x00" as *const u8 as *const i8);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1;
            return crate::src::qcommon::q_shared::qtrue;
        }
        50 => {
            v = NextConstant4();
            if !(v < 0 || v > 31) {
                EmitMovEAXStack(vm, 0);
                EmitString(b"C1 E0\x00" as *const u8 as *const i8);
                Emit1(v);
                EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                pc += 5;
                instruction += 1;
                return crate::src::qcommon::q_shared::qtrue;
            }
        }
        51 => {
            v = NextConstant4();
            if !(v < 0 || v > 31) {
                EmitMovEAXStack(vm, 0);
                EmitString(b"C1 F8\x00" as *const u8 as *const i8);
                Emit1(v);
                EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                pc += 5;
                instruction += 1;
                return crate::src::qcommon::q_shared::qtrue;
            }
        }
        52 => {
            v = NextConstant4();
            if !(v < 0 || v > 31) {
                EmitMovEAXStack(vm, 0);
                EmitString(b"C1 E8\x00" as *const u8 as *const i8);
                Emit1(v);
                EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                pc += 5;
                instruction += 1;
                return crate::src::qcommon::q_shared::qtrue;
            }
        }
        46 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0);
            if iss8(v) != 0 {
                EmitString(b"83 E0\x00" as *const u8 as *const i8);
                Emit1(v);
            } else {
                EmitString(b"25\x00" as *const u8 as *const i8);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1;
            return crate::src::qcommon::q_shared::qtrue;
        }
        47 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0);
            if iss8(v) != 0 {
                EmitString(b"83 C8\x00" as *const u8 as *const i8);
                Emit1(v);
            } else {
                EmitString(b"0D\x00" as *const u8 as *const i8);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1;
            return crate::src::qcommon::q_shared::qtrue;
        }
        48 => {
            v = Constant4();
            EmitMovEAXStack(vm, 0);
            if iss8(v) != 0 {
                EmitString(b"83 F0\x00" as *const u8 as *const i8);
                Emit1(v);
            } else {
                EmitString(b"35\x00" as *const u8 as *const i8);
                Emit4(v);
            }
            EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
            pc += 1;
            instruction += 1;
            return crate::src::qcommon::q_shared::qtrue;
        }
        11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => {
            EmitMovEAXStack(vm, 0);
            EmitCommand(LAST_COMMAND_SUB_BL_1);
            EmitString(b"3D\x00" as *const u8 as *const i8);
            Emit4(Constant4());
            pc += 1;
            EmitBranchConditions(vm, op1);
            instruction += 1;
            return crate::src::qcommon::q_shared::qtrue;
        }
        21 | 22 => {
            if !(NextConstant4() != 0) {
                pc += 5;
                EmitMovEAXStack(vm, 0);
                EmitCommand(LAST_COMMAND_SUB_BL_1);
                // floating point hack :)
                EmitString(b"25\x00" as *const u8 as *const i8); // and eax, 0x7FFFFFFF
                Emit4(0x7fffffff); // jnz 0x12345678
                if op1 == crate::vm_local_h::OP_EQF as i32 {
                    EmitJumpIns(vm, b"0F 84\x00" as *const u8 as *const i8, Constant4());
                // jz 0x12345678
                } else {
                    EmitJumpIns(vm, b"0F 85\x00" as *const u8 as *const i8, Constant4());
                    // jmp 0x12345678
                } // OP_JUMP
                instruction += 1; // OP_CALL
                return crate::src::qcommon::q_shared::qtrue;
            }
        }
        10 => {
            EmitJumpIns(vm, b"E9\x00" as *const u8 as *const i8, Constant4());
            pc += 1;
            instruction += 1;
            return crate::src::qcommon::q_shared::qtrue;
        }
        5 => {
            v = Constant4();
            EmitCallConst(vm, v, callProcOfsSyscall);
            pc += 1;
            instruction += 1;
            return crate::src::qcommon::q_shared::qtrue;
        }
        _ => {}
    }
    return crate::src::qcommon::q_shared::qfalse;
}
/*
=================
VM_Compile
=================
*/
#[no_mangle]

pub unsafe extern "C" fn VM_Compile(
    mut vm: *mut crate::qcommon_h::vm_t,
    mut header: *mut crate::qfiles_h::vmHeader_t,
) {
    let mut op: i32 = 0;
    let mut maxLength: i32 = 0;
    let mut v: i32 = 0;
    let mut i: i32 = 0;
    let mut callProcOfsSyscall: i32 = 0;
    let mut callProcOfs: i32 = 0;
    let mut callDoSyscallOfs: i32 = 0;
    jusedSize = (*header).instructionCount + 2;
    // allocate a very large temp buffer, we will shrink it later
    maxLength = (*header).codeLength * 8 + 64;
    buf = crate::src::qcommon::common::Z_Malloc(maxLength)
        as *mut crate::src::qcommon::q_shared::byte;
    jused = crate::src::qcommon::common::Z_Malloc(jusedSize)
        as *mut crate::src::qcommon::q_shared::byte;
    code = crate::src::qcommon::common::Z_Malloc((*header).codeLength + 32)
        as *mut crate::src::qcommon::q_shared::byte;
    crate::stdlib::memset(jused as *mut libc::c_void, 0, jusedSize as usize);
    crate::stdlib::memset(buf as *mut libc::c_void, 0, maxLength as usize);
    // copy code in larger buffer and put some zeros at the end
    // so we can safely look ahead for a few instructions in it
    // without a chance to get false-positive because of some garbage bytes
    crate::stdlib::memset(
        code as *mut libc::c_void,
        0,
        ((*header).codeLength + 32i32) as usize,
    );
    crate::stdlib::memcpy(
        code as *mut libc::c_void,
        (header as *mut crate::src::qcommon::q_shared::byte).offset((*header).codeOffset as isize)
            as *const libc::c_void,
        (*header).codeLength as usize,
    );
    // ensure that the optimisation pass knows about all the jump
    // table targets
    pc = -(1); // a bogus value to be printed in out-of-bounds error messages
    i = 0;
    while i < (*vm).numJumpTableTargets {
        if (*((*vm)
            .jumpTableTargets
            .offset((i as usize).wrapping_mul(::std::mem::size_of::<i32>()) as isize)
            as *mut i32))
            < 0
            || *((*vm)
                .jumpTableTargets
                .offset((i as usize).wrapping_mul(::std::mem::size_of::<i32>()) as isize)
                as *mut i32)
                >= (*vm).instructionCount
        {
            crate::src::qcommon::common::Z_Free(buf as *mut libc::c_void);
            crate::src::qcommon::common::Z_Free(jused as *mut libc::c_void);
            crate::src::qcommon::common::Com_Error(
                crate::src::qcommon::q_shared::ERR_DROP as i32,
                b"VM_CompileX86: jump target out of range at offset %d\x00" as *const u8
                    as *const i8,
                pc,
            );
        }
        *jused.offset(
            *((*vm)
                .jumpTableTargets
                .offset((i as usize).wrapping_mul(::std::mem::size_of::<i32>()) as isize)
                as *mut i32) as isize,
        ) = 1u8;
        i += 1
    }
    // Start buffer with x86-VM specific procedures
    compiledOfs = 0;
    callDoSyscallOfs = compiledOfs;
    callProcOfs = EmitCallDoSyscall(vm);
    callProcOfsSyscall = EmitCallProcedure(vm, callDoSyscallOfs);
    (*vm).entryOfs = compiledOfs;
    pass = 0;
    while pass < 3 {
        oc0 = -(23423);
        oc1 = -(234354);
        pop0 = -(43435);
        pop1 = -(545455);
        // translate all instructions
        pc = 0;
        instruction = 0;
        //code = (byte *)header + header->codeOffset;
        compiledOfs = (*vm).entryOfs; // int 3
        LastCommand = LAST_COMMAND_NONE; // sub esi, 0x12345678
        while instruction < (*header).instructionCount {
            if compiledOfs > maxLength - 16 {
                crate::src::qcommon::common::Z_Free(buf as *mut libc::c_void); // mov dword ptr [edi + ebx * 4], 0x12345678
                crate::src::qcommon::common::Z_Free(jused as *mut libc::c_void); // lea eax, [0x12345678 + esi]
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as i32,
                    b"VM_CompileX86: maxLength exceeded\x00" as *const u8 as *const i8,
                ); // mov dword ptr [edi + ebx * 4], eax
            } // mov eax, dword ptr [edi + ebx * 4]
            *(*vm).instructionPointers.offset(instruction as isize) =
                compiledOfs as crate::stdlib::intptr_t; // mov edx, esi
            if (*vm).jumpTableTargets.is_null() {
                jlabel = 1
            } else {
                jlabel = *jused.offset(instruction as isize) as i32
            } // add edx, 0x12345678
            instruction += 1; // and edx, 0x12345678
            if pc > (*header).codeLength {
                crate::src::qcommon::common::Z_Free(buf as *mut libc::c_void); // mov dword ptr [r9 + edx], eax
                crate::src::qcommon::common::Z_Free(jused as *mut libc::c_void); // sub bl, 1
                crate::src::qcommon::common::Com_Error(
                    crate::src::qcommon::q_shared::ERR_DROP as i32,
                    b"VM_CompileX86: pc > header->codeLength\x00" as *const u8 as *const i8,
                ); // sub bl, 1
            } // add	esi, 0x12345678
            op = *code.offset(pc as isize) as i32; // ret
            pc += 1; // OP_CONST
            match op {
                0 => {}
                2 => {
                    EmitString(b"CC\x00" as *const u8 as *const i8);
                }
                3 => {
                    EmitString(b"81 EE\x00" as *const u8 as *const i8);
                    Emit4(Constant4());
                }
                8 => {
                    if !(ConstOptimize(vm, callProcOfsSyscall) as u64 != 0) {
                        EmitPushStack(vm);
                        EmitString(b"C7 04 9F\x00" as *const u8 as *const i8);
                        lastConst = Constant4();
                        Emit4(lastConst);
                        if *code.offset(pc as isize) as i32 == crate::vm_local_h::OP_JUMP as i32 {
                            if lastConst < 0 || lastConst >= (*vm).instructionCount {
                                crate::src::qcommon::common::Z_Free(buf as *mut libc::c_void);
                                crate::src::qcommon::common::Z_Free(jused as *mut libc::c_void);
                                crate::src::qcommon::common::Com_Error(
                                    crate::src::qcommon::q_shared::ERR_DROP as i32,
                                    b"VM_CompileX86: jump target out of range at offset %d\x00"
                                        as *const u8
                                        as *const i8,
                                    pc,
                                );
                            }
                            *jused.offset(lastConst as isize) = 1u8
                        }
                    }
                }
                9 => {
                    EmitPushStack(vm);
                    EmitString(b"8D 86\x00" as *const u8 as *const i8);
                    oc0 = oc1;
                    oc1 = Constant4();
                    Emit4(oc1);
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX);
                }
                33 => {
                    EmitMovEAXStack(vm, 0);
                    EmitString(b"8B D6\x00" as *const u8 as *const i8);
                    EmitString(b"81 C2\x00" as *const u8 as *const i8);
                    Emit4(Constant1() & 0xff);
                    EmitString(b"81\x00" as *const u8 as *const i8);
                    EmitString(b"E2\x00" as *const u8 as *const i8);
                    Emit4((*vm).dataMask);
                    EmitRexString(0x41, b"89 04 11\x00" as *const u8 as *const i8);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                5 => {
                    EmitCallRel(vm, callProcOfs);
                }
                6 => {
                    EmitPushStack(vm);
                }
                7 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                4 => {
                    v = Constant4();
                    EmitString(b"81 C6\x00" as *const u8 as *const i8);
                    Emit4(v);
                    EmitString(b"C3\x00" as *const u8 as *const i8);
                }
                29 => {
                    if *code.offset(pc as isize) as i32 == crate::vm_local_h::OP_CONST as i32
                        && *code.offset((pc + 5) as isize) as i32
                            == crate::vm_local_h::OP_ADD as i32
                        && *code.offset((pc + 6) as isize) as i32
                            == crate::vm_local_h::OP_STORE4 as i32
                    {
                        if oc0 == oc1
                            && pop0 == crate::vm_local_h::OP_LOCAL as i32
                            && pop1 == crate::vm_local_h::OP_LOCAL as i32
                        {
                            compiledOfs -= 12;
                            *(*vm).instructionPointers.offset((instruction - 1) as isize) =
                                compiledOfs as crate::stdlib::intptr_t
                        }
                        pc += 1;
                        v = Constant4();
                        EmitMovEDXStack(vm, (*vm).dataMask);
                        if v == 1
                            && oc0 == oc1
                            && pop0 == crate::vm_local_h::OP_LOCAL as i32
                            && pop1 == crate::vm_local_h::OP_LOCAL as i32
                        {
                            EmitRexString(0x41u8, b"FF 04 11\x00" as *const u8 as *const i8);
                        // inc dword ptr [r9 + edx]
                        } else {
                            EmitRexString(0x41, b"8B 04 11\x00" as *const u8 as *const i8); // mov eax, dword ptr [r9 + edx]
                            EmitString(b"05\x00" as *const u8 as *const i8); // add eax, v
                            Emit4(v);
                            if oc0 == oc1
                                && pop0 == crate::vm_local_h::OP_LOCAL as i32
                                && pop1 == crate::vm_local_h::OP_LOCAL as i32
                            {
                                EmitRexString(0x41u8, b"89 04 11\x00" as *const u8 as *const i8);
                            // mov dword ptr [r9 + edx], eax
                            } else {
                                EmitCommand(LAST_COMMAND_SUB_BL_1); // sub bl, 1
                                                                    // mov dword ptr [r9 + edx], eax
                                EmitString(b"8B 14 9F\x00" as *const u8 as *const i8); // mov edx, dword ptr [edi + ebx * 4]
                                EmitString(b"81\x00" as *const u8 as *const i8); // and edx, 0x12345678
                                EmitString(b"E2\x00" as *const u8 as *const i8); // sub bl, 1
                                Emit4((*vm).dataMask); // OP_ADD
                                EmitRexString(0x41u8, b"89 04 11\x00" as *const u8 as *const i8);
                                // OP_STORE
                            }
                        } // OP_CONST
                        EmitCommand(LAST_COMMAND_SUB_BL_1);
                        pc += 1;
                        pc += 1;
                        instruction += 3
                    } else if *code.offset(pc as isize) as i32 == crate::vm_local_h::OP_CONST as i32
                        && *code.offset((pc + 5) as isize) as i32
                            == crate::vm_local_h::OP_SUB as i32
                        && *code.offset((pc + 6) as isize) as i32
                            == crate::vm_local_h::OP_STORE4 as i32
                    {
                        if oc0 == oc1
                            && pop0 == crate::vm_local_h::OP_LOCAL as i32
                            && pop1 == crate::vm_local_h::OP_LOCAL as i32
                        {
                            compiledOfs -= 12;
                            *(*vm).instructionPointers.offset((instruction - 1) as isize) =
                                compiledOfs as crate::stdlib::intptr_t
                        }
                        pc += 1;
                        v = Constant4();
                        EmitMovEDXStack(vm, (*vm).dataMask);
                        if v == 1
                            && oc0 == oc1
                            && pop0 == crate::vm_local_h::OP_LOCAL as i32
                            && pop1 == crate::vm_local_h::OP_LOCAL as i32
                        {
                            EmitRexString(0x41u8, b"FF 0C 11\x00" as *const u8 as *const i8);
                        // dec dword ptr [r9 + edx]
                        } else {
                            EmitRexString(0x41, b"8B 04 11\x00" as *const u8 as *const i8); // mov eax, dword ptr [r9 + edx]
                            EmitString(b"2D\x00" as *const u8 as *const i8); // sub eax, v
                            Emit4(v);
                            if oc0 == oc1
                                && pop0 == crate::vm_local_h::OP_LOCAL as i32
                                && pop1 == crate::vm_local_h::OP_LOCAL as i32
                            {
                                EmitRexString(0x41u8, b"89 04 11\x00" as *const u8 as *const i8);
                            // mov dword ptr [r9 + edx], eax
                            } else {
                                EmitCommand(LAST_COMMAND_SUB_BL_1); // sub bl, 1
                                                                    // mov dword ptr [r9 + edx], eax
                                EmitString(b"8B 14 9F\x00" as *const u8 as *const i8); // mov edx, dword ptr [edi + ebx * 4]
                                EmitString(b"81\x00" as *const u8 as *const i8); // and edx, 0x12345678
                                EmitString(b"E2\x00" as *const u8 as *const i8); // sub bl, 1
                                Emit4((*vm).dataMask); // OP_SUB
                                EmitRexString(0x41u8, b"89 04 11\x00" as *const u8 as *const i8);
                                // OP_STORE
                            }
                        } // and eax, 0x12345678
                        EmitCommand(LAST_COMMAND_SUB_BL_1); // mov eax, dword ptr [r9 + eax]
                        pc += 1; // mov dword ptr [edi + ebx * 4], eax
                        pc += 1; // mov eax, dword ptr [r9 + eax]
                        instruction += 3
                    } else if *buf.offset((compiledOfs - 3) as isize) as i32 == 0x89
                        && *buf.offset((compiledOfs - 2) as isize) as i32 == 0x4
                        && *buf.offset((compiledOfs - 1) as isize) as i32 == 0x9f
                    {
                        compiledOfs -= 3; // mov dword ptr [edi + ebx * 4], eax
                        *(*vm).instructionPointers.offset((instruction - 1) as isize) =
                            compiledOfs as crate::stdlib::intptr_t; // movzx eax, word ptr [r9 + eax]
                        EmitString(b"81\x00" as *const u8 as *const i8); // mov dword ptr [edi + ebx * 4], eax
                        EmitString(b"E0\x00" as *const u8 as *const i8); // movzx eax, byte ptr [r9 + eax]
                        Emit4((*vm).dataMask); // mov dword ptr [edi + ebx * 4], eax
                        EmitRexString(0x41, b"8B 04 01\x00" as *const u8 as *const i8); // mov edx, dword ptr -4[edi + ebx * 4]
                        EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // and edx, 0x12345678
                    } else {
                        EmitMovEAXStack(vm, (*vm).dataMask); // mov dword ptr [r9 + edx], eax
                        EmitRexString(0x41, b"8B 04 01\x00" as *const u8 as *const i8); // sub bl, 2
                        EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // mov edx, dword ptr -4[edi + ebx * 4]
                    }
                }
                28 => {
                    EmitMovEAXStack(vm, (*vm).dataMask); // and edx, 0x12345678
                    EmitRexString(0x41, b"0F B7 04 01\x00" as *const u8 as *const i8); // mov word ptr [r9 + edx], eax
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // sub bl, 2
                }
                27 => {
                    EmitMovEAXStack(vm, (*vm).dataMask); // mov edx, dword ptr -4[edi + ebx * 4]
                    EmitRexString(0x41, b"0F B6 04 01\x00" as *const u8 as *const i8); // and edx, 0x12345678
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // mov byte ptr [r9 + edx], eax
                }
                32 => {
                    EmitMovEAXStack(vm, 0); // sub bl, 2
                    EmitString(b"8B 54 9F FC\x00" as *const u8 as *const i8); // sub bl, 2
                    EmitString(b"81\x00" as *const u8 as *const i8); // cmp	eax, dword ptr 4[edi + ebx * 4]
                    EmitString(b"E2\x00" as *const u8 as *const i8); // sub bl, 2
                    Emit4((*vm).dataMask); // fld dword ptr 4[edi + ebx * 4]
                    EmitRexString(0x41, b"89 04 11\x00" as *const u8 as *const i8); // fcomp dword ptr 8[edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_SUB_BL_2); // fnstsw ax
                }
                31 => {
                    EmitMovEAXStack(vm, 0); // test	ah,0x40
                    EmitString(b"8B 54 9F FC\x00" as *const u8 as *const i8); // jne 0x12345678
                    EmitString(b"81\x00" as *const u8 as *const i8); // test	ah,0x40
                    EmitString(b"E2\x00" as *const u8 as *const i8); // je 0x12345678
                    Emit4((*vm).dataMask); // test	ah,0x01
                    Emit1(0x66); // jne 0x12345678
                    EmitRexString(0x41, b"89 04 11\x00" as *const u8 as *const i8); // test	ah,0x41
                    EmitCommand(LAST_COMMAND_SUB_BL_2); // jne 0x12345678
                }
                30 => {
                    EmitMovEAXStack(vm, 0); // test	ah,0x41
                    EmitString(b"8B 54 9F FC\x00" as *const u8 as *const i8); // je 0x12345678
                    EmitString(b"81\x00" as *const u8 as *const i8); // test	ah,0x01
                    EmitString(b"E2\x00" as *const u8 as *const i8); // je 0x12345678
                    Emit4((*vm).dataMask); // neg eax
                    EmitRexString(0x41, b"88 04 11\x00" as *const u8 as *const i8); // mov eax, dword ptr [edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_SUB_BL_2); // add dword ptr -4[edi + ebx * 4], eax
                }
                11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 => {
                    EmitMovEAXStack(vm, 0); // sub bl, 1
                    EmitCommand(LAST_COMMAND_SUB_BL_2); // mov eax, dword ptr [edi + ebx * 4]
                    EmitString(b"39 44 9F 04\x00" as *const u8 as *const i8); // sub dword ptr -4[edi + ebx * 4], eax
                    EmitBranchConditions(vm, op); // sub bl, 1
                }
                21 | 22 | 23 | 24 | 25 | 26 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_2); // mov eax,dword ptr -4[edi + ebx * 4]
                    EmitString(b"D9 44 9F 04\x00" as *const u8 as *const i8); // cdq
                    EmitString(b"D8 5C 9F 08\x00" as *const u8 as *const i8); // idiv dword ptr [edi + ebx * 4]
                    EmitString(b"DF E0\x00" as *const u8 as *const i8); // mov dword ptr -4[edi + ebx * 4],eax
                    match op {
                        21 => {
                            EmitString(b"F6 C4 40\x00" as *const u8 as *const i8); // sub bl, 1
                            EmitJumpIns(vm, b"0F 85\x00" as *const u8 as *const i8, Constant4());
                            // mov eax,dword ptr -4[edi + ebx * 4]
                        }
                        22 => {
                            EmitString(b"F6 C4 40\x00" as *const u8 as *const i8); // xor edx, edx
                            EmitJumpIns(vm, b"0F 84\x00" as *const u8 as *const i8, Constant4());
                            // div dword ptr [edi + ebx * 4]
                        }
                        23 => {
                            EmitString(b"F6 C4 01\x00" as *const u8 as *const i8); // mov dword ptr -4[edi + ebx * 4],eax
                            EmitJumpIns(vm, b"0F 85\x00" as *const u8 as *const i8, Constant4());
                            // sub bl, 1
                        }
                        24 => {
                            EmitString(b"F6 C4 41\x00" as *const u8 as *const i8); // mov eax,dword ptr -4[edi + ebx * 4]
                            EmitJumpIns(vm, b"0F 85\x00" as *const u8 as *const i8, Constant4());
                            // cdq
                        }
                        25 => {
                            EmitString(b"F6 C4 41\x00" as *const u8 as *const i8); // idiv dword ptr [edi + ebx * 4]
                            EmitJumpIns(vm, b"0F 84\x00" as *const u8 as *const i8, Constant4());
                            // mov dword ptr -4[edi + ebx * 4],edx
                        }
                        26 => {
                            EmitString(b"F6 C4 01\x00" as *const u8 as *const i8); // sub bl, 1
                            EmitJumpIns(vm, b"0F 84\x00" as *const u8 as *const i8, Constant4());
                            // mov eax,dword ptr -4[edi + ebx * 4]
                        }
                        _ => {}
                    }
                }
                37 => {
                    EmitMovEAXStack(vm, 0); // xor edx, edx
                    EmitString(b"F7 D8\x00" as *const u8 as *const i8); // div dword ptr [edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // mov dword ptr -4[edi + ebx * 4],edx
                }
                38 => {
                    EmitMovEAXStack(vm, 0); // sub bl, 1
                    EmitString(b"01 44 9F FC\x00" as *const u8 as *const i8); // mov eax,dword ptr -4[edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // imul dword ptr [edi + ebx * 4]
                }
                39 => {
                    EmitMovEAXStack(vm, 0); // mov dword ptr -4[edi + ebx * 4],eax
                    EmitString(b"29 44 9F FC\x00" as *const u8 as *const i8); // sub bl, 1
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // mov eax,dword ptr -4[edi + ebx * 4]
                }
                40 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as *const i8); // mul dword ptr [edi + ebx * 4]
                    EmitString(b"99\x00" as *const u8 as *const i8); // mov dword ptr -4[edi + ebx * 4],eax
                    EmitString(b"F7 3C 9F\x00" as *const u8 as *const i8); // sub bl, 1
                    EmitString(b"89 44 9F FC\x00" as *const u8 as *const i8); // mov eax, dword ptr [edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // and dword ptr -4[edi + ebx * 4],eax
                }
                41 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as *const i8); // sub bl, 1
                    EmitString(b"33 D2\x00" as *const u8 as *const i8); // mov eax, dword ptr [edi + ebx * 4]
                    EmitString(b"F7 34 9F\x00" as *const u8 as *const i8); // or dword ptr -4[edi + ebx * 4],eax
                    EmitString(b"89 44 9F FC\x00" as *const u8 as *const i8); // sub bl, 1
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // mov eax, dword ptr [edi + ebx * 4]
                }
                42 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as *const i8); // xor dword ptr -4[edi + ebx * 4],eax
                    EmitString(b"99\x00" as *const u8 as *const i8); // sub bl, 1
                    EmitString(b"F7 3C 9F\x00" as *const u8 as *const i8); // not dword ptr [edi + ebx * 4]
                    EmitString(b"89 54 9F FC\x00" as *const u8 as *const i8); // shl dword ptr -4[edi + ebx * 4], cl
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // sub bl, 1
                }
                43 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as *const i8); // sar dword ptr -4[edi + ebx * 4], cl
                    EmitString(b"33 D2\x00" as *const u8 as *const i8); // sub bl, 1
                    EmitString(b"F7 34 9F\x00" as *const u8 as *const i8); // shr dword ptr -4[edi + ebx * 4], cl
                    EmitString(b"89 54 9F FC\x00" as *const u8 as *const i8); // sub bl, 1
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // fld dword ptr [edi + ebx * 4]
                }
                44 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as *const i8); // fchs
                    EmitString(b"F7 2C 9F\x00" as *const u8 as *const i8); // fstp dword ptr [edi + ebx * 4]
                    EmitString(b"89 44 9F FC\x00" as *const u8 as *const i8); // fld dword ptr -4[edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // fadd dword ptr [edi + ebx * 4]
                }
                45 => {
                    EmitString(b"8B 44 9F FC\x00" as *const u8 as *const i8); // fstp dword ptr -4[edi + ebx * 4]
                    EmitString(b"F7 24 9F\x00" as *const u8 as *const i8); // sub bl, 1
                    EmitString(b"89 44 9F FC\x00" as *const u8 as *const i8); // sub bl, 1
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // fld dword ptr [edi + ebx * 4]
                }
                46 => {
                    EmitMovEAXStack(vm, 0); // fsub dword ptr 4[edi + ebx * 4]
                    EmitString(b"21 44 9F FC\x00" as *const u8 as *const i8); // fstp dword ptr [edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // sub bl, 1
                }
                47 => {
                    EmitMovEAXStack(vm, 0); // fld dword ptr [edi + ebx * 4]
                    EmitString(b"09 44 9F FC\x00" as *const u8 as *const i8); // fdiv dword ptr 4[edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // fstp dword ptr [edi + ebx * 4]
                }
                48 => {
                    EmitMovEAXStack(vm, 0); // sub bl, 1
                    EmitString(b"31 44 9F FC\x00" as *const u8 as *const i8); // fld dword ptr [edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // fmul dword ptr 4[edi + ebx * 4]
                }
                49 => {
                    EmitString(b"F7 14 9F\x00" as *const u8 as *const i8);
                    // fstp dword ptr [edi + ebx * 4]
                }
                50 => {
                    EmitMovECXStack(vm); // fild dword ptr [edi + ebx * 4]
                    EmitString(b"D3 64 9F FC\x00" as *const u8 as *const i8); // fstp dword ptr [edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_SUB_BL_1); // mov edx, Q_VMftol
                }
                51 => {
                    EmitMovECXStack(vm);
                    EmitString(b"D3 7C 9F FC\x00" as *const u8 as *const i8);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                52 => {
                    EmitMovECXStack(vm);
                    EmitString(b"D3 6C 9F FC\x00" as *const u8 as *const i8);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                53 => {
                    EmitString(b"D9 04 9F\x00" as *const u8 as *const i8);
                    EmitString(b"D9 E0\x00" as *const u8 as *const i8);
                    EmitString(b"D9 1C 9F\x00" as *const u8 as *const i8);
                }
                54 => {
                    EmitString(b"D9 44 9F FC\x00" as *const u8 as *const i8);
                    EmitString(b"D8 04 9F\x00" as *const u8 as *const i8);
                    EmitString(b"D9 5C 9F FC\x00" as *const u8 as *const i8);
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                }
                55 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                    EmitString(b"D9 04 9F\x00" as *const u8 as *const i8);
                    EmitString(b"D8 64 9F 04\x00" as *const u8 as *const i8);
                    EmitString(b"D9 1C 9F\x00" as *const u8 as *const i8);
                }
                56 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                    EmitString(b"D9 04 9F\x00" as *const u8 as *const i8);
                    EmitString(b"D8 74 9F 04\x00" as *const u8 as *const i8);
                    EmitString(b"D9 1C 9F\x00" as *const u8 as *const i8);
                }
                57 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                    EmitString(b"D9 04 9F\x00" as *const u8 as *const i8);
                    EmitString(b"D8 4C 9F 04\x00" as *const u8 as *const i8);
                    EmitString(b"D9 1C 9F\x00" as *const u8 as *const i8);
                }
                58 => {
                    EmitString(b"DB 04 9F\x00" as *const u8 as *const i8);
                    EmitString(b"D9 1C 9F\x00" as *const u8 as *const i8);
                }
                59 => {
                    // WHENHELLISFROZENOVER
                    // FTOL_PTR
                    // call the library conversion function
                    EmitRexString(0x48, b"BA\x00" as *const u8 as *const i8); // call edx
                    EmitPtr(::std::mem::transmute::<
                        Option<unsafe extern "C" fn() -> i32>,
                        *mut libc::c_void,
                    >(crate::src::qcommon::common::Q_VMftol)); // mov dword ptr [edi + ebx * 4], eax
                    EmitRexString(0x48, b"FF D2\x00" as *const u8 as *const i8); // movsx eax, byte ptr [edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // mov dword ptr [edi + ebx * 4], eax
                }
                35 => {
                    EmitString(b"0F BE 04 9F\x00" as *const u8 as *const i8); // movsx eax, word ptr [edi + ebx * 4]
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // mov dword ptr [edi + ebx * 4], eax
                }
                36 => {
                    EmitString(b"0F BF 04 9F\x00" as *const u8 as *const i8); // mov eax, 0x12345678
                    EmitCommand(LAST_COMMAND_MOV_STACK_EAX); // mov ecx, 0x12345678
                }
                34 => {
                    EmitString(b"B8\x00" as *const u8 as *const i8); // sub bl, 2
                    Emit4(VM_BLOCK_COPY as i32); // sub bl, 1
                    EmitString(b"B9\x00" as *const u8 as *const i8); // mov eax, dword ptr 4[edi + ebx * 4]
                    Emit4(Constant4()); // cmp eax, vm->instructionCount
                    EmitCallRel(vm, callDoSyscallOfs); // jae +4
                    EmitCommand(LAST_COMMAND_SUB_BL_2); // jmp qword ptr [r8 + eax * 8]
                }
                10 => {
                    EmitCommand(LAST_COMMAND_SUB_BL_1);
                    EmitString(b"8B 44 9F 04\x00" as *const u8 as *const i8);
                    EmitString(b"81 F8\x00" as *const u8 as *const i8);
                    Emit4((*vm).instructionCount);
                    EmitString(b"73 04\x00" as *const u8 as *const i8);
                    EmitRexString(0x49, b"FF 24 C0\x00" as *const u8 as *const i8);
                    EmitCallErrJump(vm, callDoSyscallOfs);
                }
                _ => {
                    crate::src::qcommon::common::Z_Free(buf as *mut libc::c_void);
                    crate::src::qcommon::common::Z_Free(jused as *mut libc::c_void);
                    crate::src::qcommon::common::Com_Error(
                        crate::src::qcommon::q_shared::ERR_DROP as i32,
                        b"VM_CompileX86: bad opcode %i at offset %i\x00" as *const u8 as *const i8,
                        op,
                        pc,
                    );
                }
            }
            pop0 = pop1;
            pop1 = op
        }
        pass += 1
    }
    // copy to an exact sized buffer with the appropriate permission bits
    (*vm).codeLength = compiledOfs;
    (*vm).codeBase = crate::stdlib::mmap(
        0 as *mut libc::c_void,
        compiledOfs as crate::stddef_h::size_t,
        0x2,
        0x1 | 0x20,
        -(1),
        0,
    ) as *mut crate::src::qcommon::q_shared::byte;
    if (*vm).codeBase == -(1i32) as *mut crate::src::qcommon::q_shared::byte {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"VM_CompileX86: can\'t mmap memory\x00" as *const u8 as *const i8,
        );
    }
    crate::stdlib::memcpy(
        (*vm).codeBase as *mut libc::c_void,
        buf as *const libc::c_void,
        compiledOfs as usize,
    );
    if crate::stdlib::mprotect(
        (*vm).codeBase as *mut libc::c_void,
        compiledOfs as crate::stddef_h::size_t,
        0x1 | 0x4,
    ) != 0
    {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"VM_CompileX86: mprotect failed\x00" as *const u8 as *const i8,
        );
    }
    crate::src::qcommon::common::Z_Free(code as *mut libc::c_void);
    crate::src::qcommon::common::Z_Free(buf as *mut libc::c_void);
    crate::src::qcommon::common::Z_Free(jused as *mut libc::c_void);
    crate::src::qcommon::common::Com_Printf(
        b"VM file %s compiled to %i bytes of code\n\x00" as *const u8 as *const i8,
        (*vm).name.as_mut_ptr(),
        compiledOfs,
    );
    (*vm).destroy =
        Some(VM_Destroy_Compiled as unsafe extern "C" fn(_: *mut crate::qcommon_h::vm_t) -> ());
    // offset all the instruction pointers for the new location
    i = 0;
    while i < (*header).instructionCount {
        let ref mut fresh2 = *(*vm).instructionPointers.offset(i as isize);
        *fresh2 += (*vm).codeBase as crate::stdlib::intptr_t;
        i += 1
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
// vm_x86.c -- load time compiler and execution environment for x86
// for PROT_ stuff
/* need this on NX enabled systems (i386 with PAE kernel or
 * noexec32=on x86_64) */
// workaround for systems that use the old MAP_ANON macro

unsafe extern "C" fn VM_Destroy_Compiled(mut self_0: *mut crate::qcommon_h::vm_t) {
    crate::stdlib::munmap(
        (*self_0).codeBase as *mut libc::c_void,
        (*self_0).codeLength as crate::stddef_h::size_t,
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
// Max number of arguments to pass from engine to vm's vmMain function.
// command number + 12 arguments
// Max number of arguments to pass from a vm to engine's syscall handler function for the vm.
// syscall number + 15 arguments
// don't change, this is hardcoded into x86 VMs, opStack protection relies
// on this
// don't change
// Hardcoded in q3asm a reserved at end of bss
//-------------------
//-------------------
// *(stack[top-1]) = stack[top]
//-------------------
// variable sized
// DO NOT MOVE OR CHANGE THESE WITHOUT CHANGING THE VM_OFFSET_* DEFINES
// USED BY THE ASM CODE
// the vm may be recursively entered
//------------------------------------
// hint for FS_ReadFileDir()
// for dynamic linked modules
// for interpreted modules
// actually allocated
// if programStack < stackBottom, error
// counts recursive VM_Call
// increment breakCount on function entry to this
/*
==============
VM_CallCompiled

This function is called directly by the generated code
==============
*/
#[no_mangle]

pub unsafe extern "C" fn VM_CallCompiled(
    mut vm: *mut crate::qcommon_h::vm_t,
    mut args: *mut i32,
) -> i32 {
    let mut stack: [crate::src::qcommon::q_shared::byte; 1039] = [0; 1039];
    let mut entryPoint: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut programStack: i32 = 0;
    let mut stackOnEntry: i32 = 0;
    let mut image: *mut crate::src::qcommon::q_shared::byte =
        0 as *mut crate::src::qcommon::q_shared::byte;
    let mut opStack: *mut i32 = 0 as *mut i32;
    let mut opStackOfs: i32 = 0;
    let mut arg: i32 = 0;
    crate::src::qcommon::vm::currentVM = vm;
    // interpret the code
    (*vm).currentlyInterpreting = crate::src::qcommon::q_shared::qtrue;
    // we might be called recursively, so this might not be the very top
    stackOnEntry = (*vm).programStack;
    programStack = stackOnEntry;
    // set up the stack frame
    image = (*vm).dataBase; // return stack
    programStack -= 8 + 4 * 13; // will terminate the loop on return

    for arg in 0..13 {
        *(&mut *image.offset((programStack + 8 + arg * 4) as isize)
            as *mut crate::src::qcommon::q_shared::byte as *mut i32) = *args.offset(arg as isize);
    }
    *(&mut *image.offset((programStack + 4) as isize) as *mut crate::src::qcommon::q_shared::byte
        as *mut i32) = 0;
    *(&mut *image.offset(programStack as isize) as *mut crate::src::qcommon::q_shared::byte
        as *mut i32) = -(1);
    // off we go into generated code...
    entryPoint = (*vm).codeBase.offset((*vm).entryOfs as isize) as *mut libc::c_void;
    opStack = (stack.as_mut_ptr() as crate::stdlib::intptr_t + 16 - 1 & !(16i32 - 1) as isize)
        as *mut i32;
    *opStack = 0xdeadbeefu32 as i32;
    opStackOfs = 0;
    asm!("movq $5, %rax\nmovq $3, %r8\nmovq $4, %r9\npush %r15\npush %r14\npush %r13\npush %r12\ncallq *%rax\npop %r12\npop %r13\npop %r14\npop %r15\n"
     : "+{si}" (programStack), "+{di}" (opStack), "+{bx}" (opStackOfs) : "imr"
     ((*vm).instructionPointers), "imr" ((*vm).dataBase), "imr" (entryPoint) :
     "cc", "memory", "rax", "rcx", "rdx", "r8", "r9", "r10", "r11" :
     "volatile");
    if opStackOfs != 1 || *opStack as u32 != 0xdeadbeef {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"opStack corrupted in compiled code\x00" as *const u8 as *const i8,
        );
    }
    if programStack != stackOnEntry - (8 + 4 * 13) {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_DROP as i32,
            b"programStack corrupted in compiled code\x00" as *const u8 as *const i8,
        );
    }
    (*vm).programStack = stackOnEntry;
    return *opStack.offset(opStackOfs as isize);
}
