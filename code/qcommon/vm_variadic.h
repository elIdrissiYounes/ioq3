intptr_t QDECL VM_Call( vm_t *vm, int callnum, ... );
intptr_t QDECL VM_DllSyscall( intptr_t arg, ... ) ;

int VM_CallCompiled(vm_t *vm, int *args);
