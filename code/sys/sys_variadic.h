void Sys_Error( const char *error, ... );
void Sys_Warn( char *warning, ... ) __attribute__ ((format (printf, 1, 2))); 
void *Sys_LoadGameDll(const char *name, intptr_t (QDECL **entryPoint)(int, ...), intptr_t (*systemcalls)(intptr_t, ...));
char *CON_Input( void );
