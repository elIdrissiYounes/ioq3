#include "sys_local.h"
#include "sys_loadlib.h"
#include "sys_variadic.h"
#include "../qcommon/q_shared.h"

#ifndef DEDICATED
#include "../client/client.h"
#endif

#include <unistd.h>
#include <signal.h>
#include <termios.h>
#include <fcntl.h>
#include <sys/time.h>
/*
=================
Sys_Error
=================
*/
void Sys_Error( const char *error, ... )
{
	va_list argptr;
	char    string[1024];

	va_start (argptr,error);
	Q_vsnprintf (string, sizeof(string), error, argptr);
	va_end (argptr);

	Sys_ErrorDialog( string );

	Sys_Exit( 3 );
}

#if 0
/*
=================
Sys_Warn
=================
*/
static __attribute__ ((format (printf, 1, 2))) void Sys_Warn( char *warning, ... )
{
	va_list argptr;
	char    string[1024];

	va_start (argptr,warning);
	Q_vsnprintf (string, sizeof(string), warning, argptr);
	va_end (argptr);

	CON_Print( va( "Warning: %s", string ) );
}
#endif

/*
=================
Sys_LoadGameDll

Used to load a development dll instead of a virtual machine
=================
*/
void *Sys_LoadGameDll(const char *name,
	intptr_t (QDECL **entryPoint)(int, ...),
	intptr_t (*systemcalls)(intptr_t, ...))
{
	void *libHandle;
	void (*dllEntry)(intptr_t (*syscallptr)(intptr_t, ...));

	assert(name);

	if(!Sys_DllExtension(name))
	{
		Com_Printf("Refusing to attempt to load library \"%s\": Extension not allowed.\n", name);
		return NULL;
	}

	Com_Printf( "Loading DLL file: %s\n", name);
	libHandle = Sys_LoadLibrary(name);

	if(!libHandle)
	{
		Com_Printf("Sys_LoadGameDll(%s) failed:\n\"%s\"\n", name, Sys_LibraryError());
		return NULL;
	}

	dllEntry = Sys_LoadFunction( libHandle, "dllEntry" );
	*entryPoint = Sys_LoadFunction( libHandle, "vmMain" );

	if ( !*entryPoint || !dllEntry )
	{
		Com_Printf ( "Sys_LoadGameDll(%s) failed to find vmMain function:\n\"%s\" !\n", name, Sys_LibraryError( ) );
		Sys_UnloadLibrary(libHandle);

		return NULL;
	}

	Com_Printf ( "Sys_LoadGameDll(%s) found vmMain function at %p\n", name, *entryPoint );
	dllEntry( systemcalls );

	return libHandle;
}

/*
==================
CON_Input
==================
*/
extern qboolean ttycon_on;
extern qboolean stdin_active;
extern int TTY_erase;
extern field_t TTY_con;
#ifndef DEDICATED
// Don't use "]" as it would be the same as in-game console,
//   this makes it clear where input came from.
#define TTY_CONSOLE_PROMPT "tty]"
#else
#define TTY_CONSOLE_PROMPT "]"
#endif
char *CON_Input( void )
{
	// we use this when sending back commands
	static char text[MAX_EDIT_LINE];
	int avail;
	char key;
	field_t *history;
	size_t UNUSED_VAR size;

	if(ttycon_on)
	{
		avail = read(STDIN_FILENO, &key, 1);
		if (avail != -1)
		{
			// we have something
			// backspace?
			// NOTE TTimo testing a lot of values .. seems it's the only way to get it to work everywhere
			if ((key == TTY_erase) || (key == 127) || (key == 8))
			{
				if (TTY_con.cursor > 0)
				{
					TTY_con.cursor--;
					TTY_con.buffer[TTY_con.cursor] = '\0';
					CON_Back();
				}
				return NULL;
			}
			// check if this is a control char
			if ((key) && (key) < ' ')
			{
				if (key == '\n')
				{
#ifndef DEDICATED
					// if not in the game explicitly prepend a slash if needed
					if (clc.state != CA_ACTIVE && con_autochat->integer && TTY_con.cursor &&
						TTY_con.buffer[0] != '/' && TTY_con.buffer[0] != '\\')
					{
						memmove(TTY_con.buffer + 1, TTY_con.buffer, sizeof(TTY_con.buffer) - 1);
						TTY_con.buffer[0] = '\\';
						TTY_con.cursor++;
					}

					if (TTY_con.buffer[0] == '/' || TTY_con.buffer[0] == '\\') {
						Q_strncpyz(text, TTY_con.buffer + 1, sizeof(text));
					} else if (TTY_con.cursor) {
						if (con_autochat->integer) {
							Com_sprintf(text, sizeof(text), "cmd say %s", TTY_con.buffer);
						} else {
							Q_strncpyz(text, TTY_con.buffer, sizeof(text));
						}
					} else {
						text[0] = '\0';
					}

					// push it in history
					Hist_Add(&TTY_con);
					CON_Hide();
					Com_Printf("%s%s\n", TTY_CONSOLE_PROMPT, TTY_con.buffer);
					Field_Clear(&TTY_con);
					CON_Show();
#else
					// push it in history
					Hist_Add(&TTY_con);
					Q_strncpyz(text, TTY_con.buffer, sizeof(text));
					Field_Clear(&TTY_con);
					key = '\n';
					size = write(STDOUT_FILENO, &key, 1);
					size = write(STDOUT_FILENO, TTY_CONSOLE_PROMPT, strlen(TTY_CONSOLE_PROMPT));
#endif
					return text;
				}
				if (key == '\t')
				{
					CON_Hide();
					Field_AutoComplete( &TTY_con );
					CON_Show();
					return NULL;
				}
				avail = read(STDIN_FILENO, &key, 1);
				if (avail != -1)
				{
					// VT 100 keys
					if (key == '[' || key == 'O')
					{
						avail = read(STDIN_FILENO, &key, 1);
						if (avail != -1)
						{
							switch (key)
							{
								case 'A':
									history = Hist_Prev();
									if (history)
									{
										CON_Hide();
										TTY_con = *history;
										CON_Show();
									}
									tcflush(STDIN_FILENO, TCIFLUSH);
									return NULL;
									break;
								case 'B':
									history = Hist_Next();
									CON_Hide();
									if (history)
									{
										TTY_con = *history;
									} else
									{
										Field_Clear(&TTY_con);
									}
									CON_Show();
									tcflush(STDIN_FILENO, TCIFLUSH);
									return NULL;
									break;
								case 'C':
									return NULL;
								case 'D':
									return NULL;
							}
						}
					}
				}
				Com_DPrintf("droping ISCTL sequence: %d, TTY_erase: %d\n", key, TTY_erase);
				tcflush(STDIN_FILENO, TCIFLUSH);
				return NULL;
			}
			if (TTY_con.cursor >= sizeof(text) - 1)
				return NULL;
			// push regular character
			TTY_con.buffer[TTY_con.cursor] = key;
			TTY_con.cursor++; // next char will always be '\0'
			// print the current line (this is differential)
			size = write(STDOUT_FILENO, &key, 1);
		}

		return NULL;
	}
	else if (stdin_active)
	{
		int     len;
		fd_set  fdset;
		struct timeval timeout;

		FD_ZERO(&fdset);
		FD_SET(STDIN_FILENO, &fdset); // stdin
		timeout.tv_sec = 0;
		timeout.tv_usec = 0;
		if(select (STDIN_FILENO + 1, &fdset, NULL, NULL, &timeout) == -1 || !FD_ISSET(STDIN_FILENO, &fdset))
			return NULL;

		len = read(STDIN_FILENO, text, sizeof(text));
		if (len == 0)
		{ // eof!
			stdin_active = qfalse;
			return NULL;
		}

		if (len < 1)
			return NULL;
		text[len-1] = 0;    // rip off the /n and terminate

		return text;
	}
	return NULL;
}
