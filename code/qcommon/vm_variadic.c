#include "vm_local.h"
#include "vm_variadic.h"
/*
============
VM_DllSyscall

Dlls will call this directly

 rcg010206 The horror; the horror.

  The syscall mechanism relies on stack manipulation to get its args.
   This is likely due to C's inability to pass "..." parameters to
   a function in one clean chunk. On PowerPC Linux, these parameters
   are not necessarily passed on the stack, so while (&arg[0] == arg)
   is true, (&arg[1] == 2nd function parameter) is not necessarily
   accurate, as arg's value might have been stored to the stack or
   other piece of scratch memory to give it a valid address, but the
   next parameter might still be sitting in a register.

  Quake's syscall system also assumes that the stack grows downward,
   and that any needed types can be squeezed, safely, into a signed int.

  This hack below copies all needed values for an argument to a
   array in memory, so that Quake can get the correct values. This can
   also be used on systems where the stack grows upwards, as the
   presumably standard and safe stdargs.h macros are used.

  As for having enough space in a signed int for your datatypes, well,
   it might be better to wait for DOOM 3 before you start porting.  :)

  The original code, while probably still inherently dangerous, seems
   to work well enough for the platforms it already works on. Rather
   than add the performance hit for those platforms, the original code
   is still in use there.

  For speed, we just grab 15 arguments, and don't worry about exactly
   how many the syscall actually needs; the extra is thrown away.
 
============
*/
intptr_t QDECL VM_DllSyscall( intptr_t arg, ... ) 
{
#if !id386 || defined __clang__
  // rcg010206 - see commentary above
  intptr_t args[MAX_VMSYSCALL_ARGS];
  int i;
  va_list ap;
  
  args[0] = arg;
  
  va_start(ap, arg);
  for (i = 1; i < ARRAY_LEN (args); i++)
    args[i] = va_arg(ap, intptr_t);
  va_end(ap);
  
  return currentVM->systemCall( args );
#else // original id code
	return currentVM->systemCall( &arg );
#endif
}


/*
==============
VM_Call


Upon a system call, the stack will look like:

sp+32	parm1
sp+28	parm0
sp+24	return value
sp+20	return address
sp+16	local1
sp+14	local0
sp+12	arg1
sp+8	arg0
sp+4	return stack
sp		return address

An interpreted function will immediately execute
an OP_ENTER instruction, which will subtract space for
locals from sp
==============
*/

intptr_t QDECL VM_Call( vm_t *vm, int callnum, ... )
{
	vm_t	*oldVM;
	intptr_t r;
	int i;

	if(!vm || !vm->name[0])
		Com_Error(ERR_FATAL, "VM_Call with NULL vm");

	oldVM = currentVM;
	currentVM = vm;
	lastVM = vm;

	if ( vm_debugLevel ) {
	  Com_Printf( "VM_Call( %d )\n", callnum );
	}

	++vm->callLevel;
	// if we have a dll loaded, call it directly
	if ( vm->entryPoint ) {
		//rcg010207 -  see dissertation at top of VM_DllSyscall() in this file.
		int args[MAX_VMMAIN_ARGS-1];
		va_list ap;
		va_start(ap, callnum);
		for (i = 0; i < ARRAY_LEN(args); i++) {
			args[i] = va_arg(ap, int);
		}
		va_end(ap);

		r = vm->entryPoint( callnum,  args[0],  args[1],  args[2], args[3],
                            args[4],  args[5],  args[6], args[7],
                            args[8],  args[9], args[10], args[11]);
	} else {
#if ( id386 || idsparc ) && !defined __clang__ // calling convention doesn't need conversion in some cases
#ifndef NO_VM_COMPILED
		if ( vm->compiled )
			r = VM_CallCompiled( vm, (int*)&callnum );
		else
#endif
			r = VM_CallInterpreted( vm, (int*)&callnum );
#else
		struct {
			int callnum;
			int args[MAX_VMMAIN_ARGS-1];
		} a;
		va_list ap;

		a.callnum = callnum;
		va_start(ap, callnum);
		for (i = 0; i < ARRAY_LEN(a.args); i++) {
			a.args[i] = va_arg(ap, int);
		}
		va_end(ap);
#ifndef NO_VM_COMPILED
		if ( vm->compiled )
			r = VM_CallCompiled( vm, &a.callnum );
		else
#endif
			r = VM_CallInterpreted( vm, &a.callnum );
#endif
	}
	--vm->callLevel;

	if ( oldVM != NULL )
	  currentVM = oldVM;
	return r;
}

/*
==============
VM_CallCompiled

This function is called directly by the generated code
==============
*/

#if defined(_MSC_VER) && defined(idx64)
extern uint8_t qvmcall64(int *programStack, int *opStack, intptr_t *instructionPointers, byte *dataBase);
#endif

int VM_CallCompiled(vm_t *vm, int *args)
{
	byte	stack[OPSTACK_SIZE + 15];
	void	*entryPoint;
	int		programStack, stackOnEntry;
	byte	*image;
	int	*opStack;
	int		opStackOfs;
	int		arg;

	currentVM = vm;

	// interpret the code
	vm->currentlyInterpreting = qtrue;

	// we might be called recursively, so this might not be the very top
	programStack = stackOnEntry = vm->programStack;

	// set up the stack frame 
	image = vm->dataBase;

	programStack -= ( 8 + 4 * MAX_VMMAIN_ARGS );

	for ( arg = 0; arg < MAX_VMMAIN_ARGS; arg++ )
		*(int *)&image[ programStack + 8 + arg * 4 ] = args[ arg ];

	*(int *)&image[ programStack + 4 ] = 0;	// return stack
	*(int *)&image[ programStack ] = -1;	// will terminate the loop on return

	// off we go into generated code...
	entryPoint = vm->codeBase + vm->entryOfs;
	opStack = PADP(stack, 16);
	*opStack = 0xDEADBEEF;
	opStackOfs = 0;

#ifdef _MSC_VER
  #if idx64
	opStackOfs = qvmcall64(&programStack, opStack, vm->instructionPointers, vm->dataBase);
  #else
	__asm
	{
		pushad

		mov	esi, dword ptr programStack
		mov	edi, dword ptr opStack
		mov	ebx, dword ptr opStackOfs

		call	entryPoint

		mov	dword ptr opStackOfs, ebx
		mov	dword ptr opStack, edi
		mov	dword ptr programStack, esi
		
		popad
	}
  #endif		
#elif idx64
	__asm__ volatile(
		"movq %5, %%rax\n"
		"movq %3, %%r8\n"
		"movq %4, %%r9\n"
		"push %%r15\n"
		"push %%r14\n"
		"push %%r13\n"
		"push %%r12\n"
		"callq *%%rax\n"
		"pop %%r12\n"
		"pop %%r13\n"
		"pop %%r14\n"
		"pop %%r15\n"
		: "+S" (programStack), "+D" (opStack), "+b" (opStackOfs)
		: "g" (vm->instructionPointers), "g" (vm->dataBase), "g" (entryPoint)
		: "cc", "memory", "%rax", "%rcx", "%rdx", "%r8", "%r9", "%r10", "%r11"
	);
#else
	__asm__ volatile(
		"calll *%3\n"
		: "+S" (programStack), "+D" (opStack), "+b" (opStackOfs)
		: "g" (entryPoint)
		: "cc", "memory", "%eax", "%ecx", "%edx"
	);
#endif

	if(opStackOfs != 1 || *opStack != 0xDEADBEEF)
	{
		Com_Error(ERR_DROP, "opStack corrupted in compiled code");
	}
	if(programStack != stackOnEntry - (8 + 4 * MAX_VMMAIN_ARGS))
		Com_Error(ERR_DROP, "programStack corrupted in compiled code");

	vm->programStack = stackOnEntry;

	return opStack[opStackOfs];
}
