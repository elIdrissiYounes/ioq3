//print a source error
void QDECL SourceError(source_t *source, char *str, ...) __attribute__ ((format (printf, 2, 3)));
//print a source warning
void QDECL SourceWarning(source_t *source, char *str, ...)  __attribute__ ((format (printf, 2, 3)));
//print a script error with filename and line number
void QDECL ScriptError(script_t *script, char *str, ...) __attribute__ ((format (printf, 2, 3)));
//print a script warning with filename and line number
void QDECL ScriptWarning(script_t *script, char *str, ...) __attribute__ ((format (printf, 2, 3)));
//write to the current opened log file
void QDECL Log_Write(char *fmt, ...) __attribute__ ((format (printf, 1, 2)));
//write to the current opened log file with a time stamp
void QDECL Log_WriteTimeStamped(char *fmt, ...) __attribute__ ((format (printf, 1, 2)));
