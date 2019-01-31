#include "../qcommon/q_shared.h"
#include "botlib.h"
#include "be_interface.h"
#include "l_memory.h"
#include "l_script.h"
#include "l_precomp.h"
#include "l_log.h"
#include "l_variadic.h"

//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
void QDECL SourceError(source_t *source, char *str, ...)
{
	char text[1024];
	va_list ap;

	va_start(ap, str);
	Q_vsnprintf(text, sizeof(text), str, ap);
	va_end(ap);
#ifdef BOTLIB
	botimport.Print(PRT_ERROR, "file %s, line %d: %s\n", source->scriptstack->filename, source->scriptstack->line, text);
#endif	//BOTLIB
#ifdef MEQCC
	printf("error: file %s, line %d: %s\n", source->scriptstack->filename, source->scriptstack->line, text);
#endif //MEQCC
#ifdef BSPC
	Log_Print("error: file %s, line %d: %s\n", source->scriptstack->filename, source->scriptstack->line, text);
#endif //BSPC
} //end of the function SourceError

//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
void QDECL SourceWarning(source_t *source, char *str, ...)
{
	char text[1024];
	va_list ap;

	va_start(ap, str);
	Q_vsnprintf(text, sizeof(text), str, ap);
	va_end(ap);
#ifdef BOTLIB
	botimport.Print(PRT_WARNING, "file %s, line %d: %s\n", source->scriptstack->filename, source->scriptstack->line, text);
#endif //BOTLIB
#ifdef MEQCC
	printf("warning: file %s, line %d: %s\n", source->scriptstack->filename, source->scriptstack->line, text);
#endif //MEQCC
#ifdef BSPC
	Log_Print("warning: file %s, line %d: %s\n", source->scriptstack->filename, source->scriptstack->line, text);
#endif //BSPC
} //end of the function ScriptWarning

//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
void QDECL ScriptError(script_t *script, char *str, ...)
{
	char text[1024];
	va_list ap;

	if (script->flags & SCFL_NOERRORS) return;

	va_start(ap, str);
	Q_vsnprintf(text, sizeof(text), str, ap);
	va_end(ap);
#ifdef BOTLIB
	botimport.Print(PRT_ERROR, "file %s, line %d: %s\n", script->filename, script->line, text);
#endif //BOTLIB
#ifdef MEQCC
	printf("error: file %s, line %d: %s\n", script->filename, script->line, text);
#endif //MEQCC
#ifdef BSPC
	Log_Print("error: file %s, line %d: %s\n", script->filename, script->line, text);
#endif //BSPC
} //end of the function ScriptError
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
void QDECL ScriptWarning(script_t *script, char *str, ...)
{
	char text[1024];
	va_list ap;

	if (script->flags & SCFL_NOWARNINGS) return;

	va_start(ap, str);
	Q_vsnprintf(text, sizeof(text), str, ap);
	va_end(ap);
#ifdef BOTLIB
	botimport.Print(PRT_WARNING, "file %s, line %d: %s\n", script->filename, script->line, text);
#endif //BOTLIB
#ifdef MEQCC
	printf("warning: file %s, line %d: %s\n", script->filename, script->line, text);
#endif //MEQCC
#ifdef BSPC
	Log_Print("warning: file %s, line %d: %s\n", script->filename, script->line, text);
#endif //BSPC
} //end of the function ScriptWarning

