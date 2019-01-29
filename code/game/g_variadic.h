#include "../qcommon/q_shared.h"

void QDECL G_LogPrintf( const char *fmt, ... ) __attribute__ ((format (printf, 1, 2)));
void QDECL G_Printf( const char *fmt, ... ) __attribute__ ((format (printf, 1, 2)));
void QDECL G_Error( const char *fmt, ... ) __attribute__ ((noreturn, format (printf, 1, 2)));
void QDECL PrintMsg( gentity_t *ent, const char *fmt, ... ) __attribute__ ((format (printf, 2, 3)));
