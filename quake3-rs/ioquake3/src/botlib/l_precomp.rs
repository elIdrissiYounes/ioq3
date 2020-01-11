// =============== BEGIN l_precomp_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct define_s {
    pub name: *mut i8,
    pub flags: i32,
    pub builtin: i32,
    pub numparms: i32,
    pub parms: *mut crate::src::botlib::l_script::token_t,
    pub tokens: *mut crate::src::botlib::l_script::token_t,
    pub next: *mut crate::src::botlib::l_precomp::define_s,
    pub hashnext: *mut crate::src::botlib::l_precomp::define_s,
}

pub type define_t = crate::src::botlib::l_precomp::define_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct indent_s {
    pub type_0: i32,
    pub skip: i32,
    pub script: *mut crate::src::botlib::l_script::script_t,
    pub next: *mut crate::src::botlib::l_precomp::indent_s,
}

pub type indent_t = crate::src::botlib::l_precomp::indent_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct source_s {
    pub filename: [i8; 1024],
    pub includepath: [i8; 1024],
    pub punctuations: *mut crate::src::botlib::l_script::punctuation_t,
    pub scriptstack: *mut crate::src::botlib::l_script::script_t,
    pub tokens: *mut crate::src::botlib::l_script::token_t,
    pub defines: *mut crate::src::botlib::l_precomp::define_t,
    pub definehash: *mut *mut crate::src::botlib::l_precomp::define_t,
    pub indentstack: *mut crate::src::botlib::l_precomp::indent_t,
    pub skip: i32,
    pub token: crate::src::botlib::l_script::token_t,
}

pub type source_t = crate::src::botlib::l_precomp::source_s;
use ::libc;

pub use crate::internal::__builtin_va_list;
pub use crate::internal::__va_list_tag;
pub use crate::stdarg_h::va_list;
pub use crate::stddef_h::size_t;
pub use crate::stdlib::__time_t;
pub use crate::stdlib::time_t;

pub use crate::be_aas_h::C2RustUnnamed_0;
pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
pub use crate::src::botlib::l_script::punctuation_s;
pub use crate::src::botlib::l_script::punctuation_t;
pub use crate::src::botlib::l_script::script_s;
pub use crate::src::botlib::l_script::script_t;
pub use crate::src::botlib::l_script::token_s;
pub use crate::src::botlib::l_script::token_t;
pub use crate::src::botlib::l_script::EndOfScript;
pub use crate::src::botlib::l_script::FreeScript;
pub use crate::src::botlib::l_script::LoadScriptFile;
pub use crate::src::botlib::l_script::LoadScriptMemory;
pub use crate::src::botlib::l_script::PS_ReadToken;
pub use crate::src::botlib::l_script::PS_SetBaseFolder;
pub use crate::src::botlib::l_script::StripDoubleQuotes;
pub use crate::src::qcommon::common::Com_Error;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::pc_token_s;
pub use crate::src::qcommon::q_shared::pc_token_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::Q_strcat;
pub use crate::src::qcommon::q_shared::Q_stricmp;
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::ERR_DISCONNECT;
pub use crate::src::qcommon::q_shared::ERR_DROP;
pub use crate::src::qcommon::q_shared::ERR_FATAL;
pub use crate::src::qcommon::q_shared::ERR_NEED_CD;
pub use crate::src::qcommon::q_shared::ERR_SERVERDISCONNECT;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
use crate::stdlib::fabs;
use crate::stdlib::sprintf;
use crate::stdlib::vsnprintf;

use crate::src::botlib::be_interface::botimport;
use crate::src::botlib::l_log::Log_Write;
use crate::src::botlib::l_memory::FreeMemory;
use crate::src::botlib::l_memory::GetClearedMemory;
use crate::src::botlib::l_memory::GetMemory;
use crate::stdlib::ctime;
use crate::stdlib::free;
use crate::stdlib::labs;
use crate::stdlib::memcpy;
use crate::stdlib::memmove;
use crate::stdlib::memset;
use crate::stdlib::strcat;
use crate::stdlib::strcmp;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
use crate::stdlib::strncat;
use crate::stdlib::time;
//end of the function PC_Directive_endif
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================

pub type operator_t = operator_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct operator_s {
    pub operator: i32,
    pub priority: i32,
    pub parentheses: i32,
    pub prev: *mut operator_s,
    pub next: *mut operator_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct value_s {
    pub intvalue: isize,
    pub floatvalue: f32,
    pub parentheses: i32,
    pub prev: *mut value_s,
    pub next: *mut value_s,
}

pub type value_t = value_s;
//directive name with parse function

pub type directive_t = directive_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct directive_s {
    pub name: *mut i8,
    pub func: Option<unsafe extern "C" fn(_: *mut crate::src::botlib::l_precomp::source_t) -> i32>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct builtin {
    pub string: *mut i8,
    pub builtin: i32,
}
#[no_mangle]

pub static mut numtokens: i32 = 0;
/*
int tokenheapinitialized;				//true when the token heap is initialized
token_t token_heap[TOKEN_HEAP_SIZE];	//heap with tokens
token_t *freetokens;					//free tokens from the heap
*/
//list with global defines added to every source loaded
#[no_mangle]

pub static mut globaldefines: *mut crate::src::botlib::l_precomp::define_t =
    0 as *mut crate::src::botlib::l_precomp::define_t;
//print a source error
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn SourceError(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut str: *mut i8,
    mut args: ...
) {
    let mut text: [i8; 1024] = [0; 1024];
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>(),
        str,
        ap.as_va_list(),
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        3,
        b"file %s, line %d: %s\n\x00" as *const u8 as *mut i8,
        (*(*source).scriptstack).filename.as_mut_ptr(),
        (*(*source).scriptstack).line,
        text.as_mut_ptr(),
    );
    //BOTLIB
    //MEQCC
    //BSPC
}
//print a source warning
//end of the function SourceError
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn SourceWarning(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut str: *mut i8,
    mut args: ...
) {
    let mut text: [i8; 1024] = [0; 1024];
    let mut ap: ::std::ffi::VaListImpl;
    ap = args.clone();
    crate::stdlib::vsnprintf(
        text.as_mut_ptr(),
        ::std::mem::size_of::<[i8; 1024]>(),
        str,
        ap.as_va_list(),
    );
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        2,
        b"file %s, line %d: %s\n\x00" as *const u8 as *mut i8,
        (*(*source).scriptstack).filename.as_mut_ptr(),
        (*(*source).scriptstack).line,
        text.as_mut_ptr(),
    );
    //BOTLIB
    //MEQCC
    //BSPC
}
//end of the function ScriptWarning
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_PushIndent(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut type_0: i32,
    mut skip: i32,
) {
    let mut indent: *mut crate::src::botlib::l_precomp::indent_t =
        0 as *mut crate::src::botlib::l_precomp::indent_t;
    indent = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_precomp::indent_t,
    >()) as *mut crate::src::botlib::l_precomp::indent_t;
    (*indent).type_0 = type_0;
    (*indent).script = (*source).scriptstack;
    (*indent).skip = (skip != 0) as i32;
    (*source).skip += (*indent).skip;
    (*indent).next = (*source).indentstack;
    (*source).indentstack = indent;
}
//end of the function PC_PushIndent
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_PopIndent(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut type_0: *mut i32,
    mut skip: *mut i32,
) {
    let mut indent: *mut crate::src::botlib::l_precomp::indent_t =
        0 as *mut crate::src::botlib::l_precomp::indent_t;
    *type_0 = 0;
    *skip = 0;
    indent = (*source).indentstack;
    if indent.is_null() {
        return;
    }
    //must be an indent from the current script
    if (*(*source).indentstack).script != (*source).scriptstack {
        return;
    }
    *type_0 = (*indent).type_0;
    *skip = (*indent).skip;
    (*source).indentstack = (*(*source).indentstack).next;
    (*source).skip -= (*indent).skip;
    crate::src::botlib::l_memory::FreeMemory(indent as *mut libc::c_void);
}
//end of the function PC_PopIndent
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_PushScript(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut script: *mut crate::src::botlib::l_script::script_t,
) {
    let mut s: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t; //end for
    s = (*source).scriptstack;
    while !s.is_null() {
        if crate::src::qcommon::q_shared::Q_stricmp(
            (*s).filename.as_mut_ptr(),
            (*script).filename.as_mut_ptr(),
        ) == 0
        {
            SourceError(
                source,
                b"%s recursively included\x00" as *const u8 as *mut i8,
                (*script).filename.as_mut_ptr(),
            );
            return;
        }
        s = (*s).next
        //end if
    }
    //push the script on the script stack
    (*script).next = (*source).scriptstack;
    (*source).scriptstack = script;
}
//end of the function PC_PushScript
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_InitTokenHeap() {
    /*
    int i;

    if (tokenheapinitialized) return;
    freetokens = NULL;
    for (i = 0; i < TOKEN_HEAP_SIZE; i++)
    {
        token_heap[i].next = freetokens;
        freetokens = &token_heap[i];
    } //end for
    tokenheapinitialized = qtrue;
    */
}
//end of the function PC_InitTokenHeap
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_CopyToken(
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> *mut crate::src::botlib::l_script::token_t {
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    //	t = (token_t *) malloc(sizeof(token_t));
    t = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_script::token_t,
    >()) as *mut crate::src::botlib::l_script::token_t;
    //	t = freetokens;
    if t.is_null() {
        crate::src::qcommon::common::Com_Error(
            crate::src::qcommon::q_shared::ERR_FATAL as i32,
            b"out of token space\x00" as *const u8 as *const i8,
        ); //end if
    }
    //	freetokens = freetokens->next;
    crate::stdlib::memcpy(
        t as *mut libc::c_void,
        token as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
    );
    (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
    numtokens += 1;
    return t;
}
//end of the function PC_CopyToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FreeToken(mut token: *mut crate::src::botlib::l_script::token_t) {
    //free(token);
    crate::src::botlib::l_memory::FreeMemory(token as *mut libc::c_void);
    //	token->next = freetokens;
    //	freetokens = token;
    numtokens -= 1;
}
//end of the function PC_FreeToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ReadSourceToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    let mut type_0: i32 = 0;
    let mut skip: i32 = 0;
    //if there's no token already available
    while (*source).tokens.is_null() {
        //end while
        //if there's a token to read from the script
        if crate::src::botlib::l_script::PS_ReadToken((*source).scriptstack, token) != 0 {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
        //if at the end of the script
        if crate::src::botlib::l_script::EndOfScript((*source).scriptstack) != 0 {
            //end if
            //remove all indents of the script
            while !(*source).indentstack.is_null()
                && (*(*source).indentstack).script == (*source).scriptstack
            {
                SourceWarning(source, b"missing #endif\x00" as *const u8 as *mut i8);
                PC_PopIndent(source, &mut type_0, &mut skip);
            }
            //end if
        }
        //if this was the initial script
        if (*(*source).scriptstack).next.is_null() {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //remove the script and return to the last one
        script = (*source).scriptstack;
        (*source).scriptstack = (*(*source).scriptstack).next;
        crate::src::botlib::l_script::FreeScript(script);
    }
    //copy the already available token
    crate::stdlib::memcpy(
        token as *mut libc::c_void,
        (*source).tokens as *const libc::c_void,
        ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
    );
    //free the read token
    t = (*source).tokens;
    (*source).tokens = (*(*source).tokens).next;
    PC_FreeToken(t);
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_ReadSourceToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_UnreadSourceToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    t = PC_CopyToken(token);
    (*t).next = (*source).tokens;
    (*source).tokens = t;
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_UnreadSourceToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ReadDefineParms(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut define: *mut crate::src::botlib::l_precomp::define_t,
    mut parms: *mut *mut crate::src::botlib::l_script::token_t,
    mut maxparms: i32,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end if
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut last: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut i: i32 = 0;
    let mut done: i32 = 0;
    let mut lastcomma: i32 = 0;
    let mut numparms: i32 = 0;
    let mut indent: i32 = 0;
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"define %s missing parms\x00" as *const u8 as *mut i8,
            (*define).name,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    if (*define).numparms > maxparms {
        SourceError(
            source,
            b"define with more than %d parameters\x00" as *const u8 as *mut i8,
            maxparms,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //

    for i in 0..(*define).numparms {
        let ref mut fresh0 = *parms.offset(i as isize);

        *fresh0 = 0 as *mut crate::src::botlib::l_script::token_t;
    }
    //if no leading "("
    if crate::stdlib::strcmp(
        token.string.as_mut_ptr(),
        b"(\x00" as *const u8 as *const i8,
    ) != 0
    {
        PC_UnreadSourceToken(source, &mut token); //end if
        SourceError(
            source,
            b"define %s missing parms\x00" as *const u8 as *mut i8,
            (*define).name,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //read the define parameters
    done = 0; //end for
    numparms = 0; //end if
    indent = 0; //end if
    while done == 0 {
        if numparms >= maxparms {
            SourceError(
                source,
                b"define %s with too many parms\x00" as *const u8 as *mut i8,
                (*define).name,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if numparms >= (*define).numparms {
            SourceWarning(
                source,
                b"define %s has too many parms\x00" as *const u8 as *mut i8,
                (*define).name,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        let ref mut fresh1 = *parms.offset(numparms as isize);
        *fresh1 = 0 as *mut crate::src::botlib::l_script::token_t;
        lastcomma = 1;
        last = 0 as *mut crate::src::botlib::l_script::token_t;
        //end if
        while done == 0 {
            if PC_ReadSourceToken(source, &mut token) == 0 {
                SourceError(
                    source,
                    b"define %s incomplete\x00" as *const u8 as *mut i8,
                    (*define).name,
                ); //end while
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
            //
            //end if
            //
            if crate::stdlib::strcmp(
                token.string.as_mut_ptr(),
                b",\x00" as *const u8 as *const i8,
            ) == 0
            {
                if indent <= 0 {
                    if lastcomma != 0 {
                        SourceWarning(source, b"too many comma\'s\x00" as *const u8 as *mut i8);
                        //end if
                    }
                    break;
                }
                //end if
            }
            lastcomma = 0;
            if crate::stdlib::strcmp(
                token.string.as_mut_ptr(),
                b"(\x00" as *const u8 as *const i8,
            ) == 0
            {
                //
                //end if
                indent += 1
            } else {
                if crate::stdlib::strcmp(
                    token.string.as_mut_ptr(),
                    b")\x00" as *const u8 as *const i8,
                ) == 0
                {
                    indent -= 1;
                    if indent <= 0 {
                        //end if
                        //end if
                        if (*parms.offset(((*define).numparms - 1i32) as isize)).is_null() {
                            SourceWarning(
                                source,
                                b"too few define parms\x00" as *const u8 as *mut i8,
                            ); //end if
                        }
                        done = 1;
                        break;
                    }
                }
                //
                if numparms < (*define).numparms {
                    //
                    t = PC_CopyToken(&mut token);
                    (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                    if !last.is_null() {
                        (*last).next = t
                    } else {
                        let ref mut fresh2 = *parms.offset(numparms as isize);
                        *fresh2 = t
                    }
                    last = t
                }
            }
        }
        numparms += 1
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_ReadDefineParms
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_StringizeTokens(
    mut tokens: *mut crate::src::botlib::l_script::token_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t; //end for
    (*token).type_0 = 1;
    (*token).whitespace_p = 0 as *mut i8;
    (*token).endwhitespace_p = 0 as *mut i8;
    (*token).string[0] = '\u{0}' as i8;
    crate::stdlib::strcat(
        (*token).string.as_mut_ptr(),
        b"\"\x00" as *const u8 as *const i8,
    );
    t = tokens;
    while !t.is_null() {
        crate::stdlib::strncat(
            (*token).string.as_mut_ptr(),
            (*t).string.as_mut_ptr(),
            (1024usize)
                .wrapping_sub(crate::stdlib::strlen((*token).string.as_mut_ptr()))
                .wrapping_sub(1usize),
        );
        t = (*t).next
    }
    crate::stdlib::strncat(
        (*token).string.as_mut_ptr(),
        b"\"\x00" as *const u8 as *const i8,
        (1024usize)
            .wrapping_sub(crate::stdlib::strlen((*token).string.as_mut_ptr()))
            .wrapping_sub(1usize),
    );
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_StringizeTokens
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_MergeTokens(
    mut t1: *mut crate::src::botlib::l_script::token_t,
    mut t2: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    //merging of a name with a name or number
    if (*t1).type_0 == 4 && ((*t2).type_0 == 4 || (*t2).type_0 == 3) {
        crate::stdlib::strcat((*t1).string.as_mut_ptr(), (*t2).string.as_mut_ptr()); //end if
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //merging of two strings
    if (*t1).type_0 == 1 && (*t2).type_0 == 1 {
        //end if
        //remove trailing double quote
        (*t1).string[crate::stdlib::strlen((*t1).string.as_mut_ptr()).wrapping_sub(1usize)] =
            '\u{0}' as i8;
        //concat without leading double quote
        crate::stdlib::strcat(
            (*t1).string.as_mut_ptr(),
            &mut *(*t2).string.as_mut_ptr().offset(1),
        );
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //FIXME: merging of two number of the same sub type
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function PC_MergeTokens
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
/*
void PC_PrintDefine(define_t *define)
{
    printf("define->name = %s\n", define->name);
    printf("define->flags = %d\n", define->flags);
    printf("define->builtin = %d\n", define->builtin);
    printf("define->numparms = %d\n", define->numparms);
//	token_t *parms;					//define parameters
//	token_t *tokens;					//macro tokens (possibly containing parm tokens)
//	struct define_s *next;			//next defined macro in a list
} //end of the function PC_PrintDefine*/
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_PrintDefineHashTable(
    mut definehash: *mut *mut crate::src::botlib::l_precomp::define_t,
) {
    let mut i: i32 = 0; //end for
    let mut d: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    i = 0;
    while i < 1024 {
        crate::src::botlib::l_log::Log_Write(b"%4d:\x00" as *const u8 as *mut i8, i);
        d = *definehash.offset(i as isize);
        while !d.is_null() {
            crate::src::botlib::l_log::Log_Write(b" %s\x00" as *const u8 as *mut i8, (*d).name);
            d = (*d).hashnext
        }
        crate::src::botlib::l_log::Log_Write(b"\n\x00" as *const u8 as *mut i8);
        i += 1
    }
    //end for
}
//end of the function PC_PrintDefineHashTable
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
//char primes[16] = {1, 3, 5, 7, 11, 13, 17, 19, 23, 27, 29, 31, 37, 41, 43, 47};
#[no_mangle]

pub unsafe extern "C" fn PC_NameHash(mut name: *mut i8) -> i32 {
    let mut hash: i32 = 0; //end while
    let mut i: i32 = 0;
    hash = 0;
    i = 0;
    while *name.offset(i as isize) as i32 != '\u{0}' as i32 {
        hash += *name.offset(i as isize) as i32 * (119 + i);
        i += 1
        //hash += (name[i] << 7) + i;
        //hash += (name[i] << (i&15));
    }
    hash = (hash ^ hash >> 10 ^ hash >> 20) & 1024 - 1;
    return hash;
}
//end of the function PC_NameHash
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_AddDefineToHash(
    mut define: *mut crate::src::botlib::l_precomp::define_t,
    mut definehash: *mut *mut crate::src::botlib::l_precomp::define_t,
) {
    let mut hash: i32 = 0;
    hash = PC_NameHash((*define).name);
    (*define).hashnext = *definehash.offset(hash as isize);
    let ref mut fresh3 = *definehash.offset(hash as isize);
    *fresh3 = define;
}
//end of the function PC_AddDefineToHash
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FindHashedDefine(
    mut definehash: *mut *mut crate::src::botlib::l_precomp::define_t,
    mut name: *mut i8,
) -> *mut crate::src::botlib::l_precomp::define_t {
    let mut d: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t; //end for
    let mut hash: i32 = 0;
    hash = PC_NameHash(name);
    d = *definehash.offset(hash as isize);
    while !d.is_null() {
        if crate::stdlib::strcmp((*d).name, name) == 0 {
            return d;
        }
        d = (*d).hashnext
    }
    return 0 as *mut crate::src::botlib::l_precomp::define_t;
}
//end of the function PC_FindHashedDefine
//DEFINEHASHING
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FindDefine(
    mut defines: *mut crate::src::botlib::l_precomp::define_t,
    mut name: *mut i8,
) -> *mut crate::src::botlib::l_precomp::define_t {
    let mut d: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t; //end for
    d = defines;
    while !d.is_null() {
        if crate::stdlib::strcmp((*d).name, name) == 0 {
            return d;
        }
        d = (*d).next
    }
    return 0 as *mut crate::src::botlib::l_precomp::define_t;
}
//end of the function PC_FindDefine
//============================================================================
//
// Parameter:				-
// Returns:					number of the parm
//								if no parm found with the given name -1 is returned
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FindDefineParm(
    mut define: *mut crate::src::botlib::l_precomp::define_t,
    mut name: *mut i8,
) -> i32 {
    let mut p: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t; //end for
    let mut i: i32 = 0;
    i = 0;
    p = (*define).parms;
    while !p.is_null() {
        if crate::stdlib::strcmp((*p).string.as_mut_ptr(), name) == 0 {
            return i;
        }
        i += 1;
        p = (*p).next
    }
    return -(1);
}
//end of the function PC_FindDefineParm
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FreeDefine(mut define: *mut crate::src::botlib::l_precomp::define_t) {
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut next: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    //free the define parameters
    t = (*define).parms; //end for
    while !t.is_null() {
        next = (*t).next;
        PC_FreeToken(t);
        t = next
    }
    //free the define tokens
    t = (*define).tokens; //end for
    while !t.is_null() {
        next = (*t).next;
        PC_FreeToken(t);
        t = next
    }
    //free the define
    crate::src::botlib::l_memory::FreeMemory((*define).name as *mut libc::c_void);
    crate::src::botlib::l_memory::FreeMemory(define as *mut libc::c_void);
}
//add builtin defines
//end of the function PC_FreeDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_AddBuiltinDefines(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) {
    let mut i: i32 = 0;
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut builtin_0: [builtin; 5] = [
        {
            let mut init = builtin {
                string: b"__LINE__\x00" as *const u8 as *mut i8,
                builtin: 1,
            };
            init
        },
        {
            let mut init = builtin {
                string: b"__FILE__\x00" as *const u8 as *mut i8,
                builtin: 2,
            };
            init
        },
        {
            let mut init = builtin {
                string: b"__DATE__\x00" as *const u8 as *mut i8,
                builtin: 3,
            };
            init
        },
        {
            let mut init = builtin {
                string: b"__TIME__\x00" as *const u8 as *mut i8,
                builtin: 4,
            };
            init
        },
        {
            let mut init = builtin {
                string: 0 as *mut i8,
                builtin: 0,
            };
            init
        },
    ];
    i = 0;
    while !builtin_0[i as usize].string.is_null() {
        define = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
            crate::src::botlib::l_precomp::define_t,
        >()) as *mut crate::src::botlib::l_precomp::define_t;
        crate::stdlib::memset(
            define as *mut libc::c_void,
            0,
            ::std::mem::size_of::<crate::src::botlib::l_precomp::define_t>(),
        );
        (*define).name = crate::src::botlib::l_memory::GetMemory(
            crate::stdlib::strlen(builtin_0[i as usize].string).wrapping_add(1usize),
        ) as *mut i8;
        crate::stdlib::strcpy((*define).name, builtin_0[i as usize].string);
        (*define).flags |= 0x1;
        (*define).builtin = builtin_0[i as usize].builtin;
        //DEFINEHASHING
        PC_AddDefineToHash(define, (*source).definehash);
        i += 1
    }
    //add the define to the source
    //end for
}
//end of the function PC_AddBuiltinDefines
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpandBuiltinDefine(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut deftoken: *mut crate::src::botlib::l_script::token_t,
    mut define: *mut crate::src::botlib::l_precomp::define_t,
    mut firsttoken: *mut *mut crate::src::botlib::l_script::token_t,
    mut lasttoken: *mut *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut token: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t; //end switch
    let mut t: crate::stdlib::time_t = 0;
    let mut curtime: *mut i8 = 0 as *mut i8;
    token = PC_CopyToken(deftoken);
    match (*define).builtin {
        1 => {
            crate::stdlib::sprintf(
                (*token).string.as_mut_ptr(),
                b"%d\x00" as *const u8 as *const i8,
                (*deftoken).line,
            );
            (*token).intvalue = (*deftoken).line as usize;
            (*token).floatvalue = (*deftoken).line as f32;
            //end case
            //NUMBERVALUE
            (*token).type_0 = 3; //end case
            (*token).subtype = 0x8 | 0x1000; //end case
            *firsttoken = token; //end case
            *lasttoken = token
        }
        2 => {
            crate::stdlib::strcpy(
                (*token).string.as_mut_ptr(),
                (*(*source).scriptstack).filename.as_mut_ptr(),
            );
            (*token).type_0 = 4;
            (*token).subtype = crate::stdlib::strlen((*token).string.as_mut_ptr()) as i32;
            *firsttoken = token;
            *lasttoken = token
        }
        3 => {
            t = crate::stdlib::time(0 as *mut crate::stdlib::time_t);
            curtime = crate::stdlib::ctime(&mut t);
            crate::stdlib::strcpy(
                (*token).string.as_mut_ptr(),
                b"\"\x00" as *const u8 as *const i8,
            );
            crate::stdlib::strncat((*token).string.as_mut_ptr(), curtime.offset(4), 7);
            crate::stdlib::strncat(
                (*token).string.as_mut_ptr().offset(7),
                curtime.offset(20),
                4,
            );
            crate::stdlib::strcat(
                (*token).string.as_mut_ptr(),
                b"\"\x00" as *const u8 as *const i8,
            );
            crate::stdlib::free(curtime as *mut libc::c_void);
            (*token).type_0 = 4;
            (*token).subtype = crate::stdlib::strlen((*token).string.as_mut_ptr()) as i32;
            *firsttoken = token;
            *lasttoken = token
        }
        4 => {
            t = crate::stdlib::time(0 as *mut crate::stdlib::time_t);
            curtime = crate::stdlib::ctime(&mut t);
            crate::stdlib::strcpy(
                (*token).string.as_mut_ptr(),
                b"\"\x00" as *const u8 as *const i8,
            );
            crate::stdlib::strncat((*token).string.as_mut_ptr(), curtime.offset(11), 8);
            crate::stdlib::strcat(
                (*token).string.as_mut_ptr(),
                b"\"\x00" as *const u8 as *const i8,
            );
            crate::stdlib::free(curtime as *mut libc::c_void);
            (*token).type_0 = 4;
            (*token).subtype = crate::stdlib::strlen((*token).string.as_mut_ptr()) as i32;
            *firsttoken = token;
            *lasttoken = token
        }
        5 | _ => {
            *firsttoken = 0 as *mut crate::src::botlib::l_script::token_t;
            *lasttoken = 0 as *mut crate::src::botlib::l_script::token_t
        }
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_ExpandBuiltinDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpandDefine(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut deftoken: *mut crate::src::botlib::l_script::token_t,
    mut define: *mut crate::src::botlib::l_precomp::define_t,
    mut firsttoken: *mut *mut crate::src::botlib::l_script::token_t,
    mut lasttoken: *mut *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut parms: [*mut crate::src::botlib::l_script::token_t; 128] = [
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
        0 as *mut crate::src::botlib::l_script::token_t,
    ];
    let mut dt: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut pt: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut t1: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut t2: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut first: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut last: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut nextpt: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut parmnum: i32 = 0;
    let mut i: i32 = 0;
    //if it is a builtin define
    if (*define).builtin != 0 {
        return PC_ExpandBuiltinDefine(source, deftoken, define, firsttoken, lasttoken);
    } //end if
      //if the define has parameters
    if (*define).numparms != 0 {
        if PC_ReadDefineParms(source, define, parms.as_mut_ptr(), 128) == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        } //end if
          //DEBUG_EVAL
    }
    //empty list at first
    first = 0 as *mut crate::src::botlib::l_script::token_t;
    last = 0 as *mut crate::src::botlib::l_script::token_t;
    let mut current_block_41: u64;
    //create a list with tokens of the expanded define
    dt = (*define).tokens; //end for
    while !dt.is_null() {
        parmnum = -(1);
        //end else
        //if the token is a name, it could be a define parameter
        if (*dt).type_0 == 4 {
            parmnum = PC_FindDefineParm(define, (*dt).string.as_mut_ptr())
        } //end if
          //if it is a define parameter
        if parmnum >= 0 {
            pt = parms[parmnum as usize];
            while !pt.is_null() {
                t = PC_CopyToken(pt);
                //add the token to the list
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !last.is_null() {
                    (*last).next = t
                } else {
                    first = t
                }
                last = t;
                pt = (*pt).next
            }
        //end for
        } else {
            //if stringizing operator
            if (*dt).string[0] as i32 == '#' as i32 && (*dt).string[1] as i32 == '\u{0}' as i32 {
                //end else
                //the stringizing operator must be followed by a define parameter
                if !(*dt).next.is_null() {
                    parmnum = PC_FindDefineParm(define, (*(*dt).next).string.as_mut_ptr())
                } else {
                    parmnum = -(1)
                }
                //end if
                if parmnum >= 0 {
                    //
                    dt = (*dt).next; //end if
                                     //step over the stringizing operator
                                     //stringize the define parameter tokens
                    if PC_StringizeTokens(parms[parmnum as usize], &mut token) == 0 {
                        SourceError(
                            source,
                            b"can\'t stringize tokens\x00" as *const u8 as *mut i8,
                        ); //end if
                        return crate::src::qcommon::q_shared::qfalse as i32;
                    }
                    t = PC_CopyToken(&mut token);
                    current_block_41 = 13131896068329595644;
                } else {
                    SourceWarning(
                        source,
                        b"stringizing operator without define parameter\x00" as *const u8
                            as *mut i8,
                    );
                    current_block_41 = 1917311967535052937;
                }
            } else {
                t = PC_CopyToken(dt);
                current_block_41 = 13131896068329595644;
            }
            match current_block_41 {
                1917311967535052937 => {}
                _ => {
                    //add the token to the list
                    (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                    if !last.is_null() {
                        (*last).next = t
                    } else {
                        first = t
                    }
                    last = t
                }
            }
        }
        dt = (*dt).next
    }
    //check for the merging operator
    t = first; //end for
    while !t.is_null() {
        if !(*t).next.is_null() {
            //end if
            //if the merging operator
            if (*(*t).next).string[0] as i32 == '#' as i32
                && (*(*t).next).string[1] as i32 == '#' as i32
            {
                t1 = t;
                t2 = (*(*t).next).next;
                if !t2.is_null() {
                    //end if
                    if PC_MergeTokens(t1, t2) == 0 {
                        SourceError(
                            source,
                            b"can\'t merge %s with %s\x00" as *const u8 as *mut i8,
                            (*t1).string.as_mut_ptr(),
                            (*t2).string.as_mut_ptr(),
                        ); //end if
                        return crate::src::qcommon::q_shared::qfalse as i32;
                    }
                    PC_FreeToken((*t1).next);
                    (*t1).next = (*t2).next;
                    if t2 == last {
                        last = t1
                    }
                    PC_FreeToken(t2);
                    continue;
                }
            }
            //end if
        }
        t = (*t).next
    }
    //store the first and last token of the list
    *firsttoken = first;
    *lasttoken = last;
    //free all the parameter tokens
    //end for
    for i in 0..(*define).numparms {
        pt = parms[i as usize];

        while !pt.is_null() {
            nextpt = (*pt).next;
            PC_FreeToken(pt);
            pt = nextpt
        }
    }
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_ExpandDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpandDefineIntoSource(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut deftoken: *mut crate::src::botlib::l_script::token_t,
    mut define: *mut crate::src::botlib::l_precomp::define_t,
) -> i32 {
    let mut firsttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t; //end if
    let mut lasttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    if PC_ExpandDefine(source, deftoken, define, &mut firsttoken, &mut lasttoken) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if !firsttoken.is_null() && !lasttoken.is_null() {
        (*lasttoken).next = (*source).tokens;
        (*source).tokens = firsttoken;
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function PC_ExpandDefineIntoSource
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ConvertPath(mut path: *mut i8) {
    let mut ptr: *mut i8 = 0 as *mut i8;
    //remove double path seperators
    ptr = path; //end while
    while *ptr != 0 {
        if (*ptr as i32 == '\\' as i32 || *ptr as i32 == '/' as i32)
            && (*ptr.offset(1) as i32 == '\\' as i32 || *ptr.offset(1) as i32 == '/' as i32)
        {
            crate::stdlib::memmove(
                ptr as *mut libc::c_void,
                ptr.offset(1isize) as *const libc::c_void,
                crate::stdlib::strlen(ptr),
            ); //end if
        } else {
            ptr = ptr.offset(1)
        }
        //end else
    }
    //set OS dependent path seperators
    ptr = path;
    while *ptr != 0 {
        if *ptr as i32 == '/' as i32 || *ptr as i32 == '\\' as i32 {
            *ptr = '/' as i8
        }
        ptr = ptr.offset(1)
    }
    //end while
}
//end of the function PC_ConvertPath
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_include(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut path: [i8; 64] = [0; 64];
    //QUAKE
    if (*source).skip > 0 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"#include without file name\x00" as *const u8 as *mut i8,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    } //end if
    if token.linescrossed > 0 {
        SourceError(
            source,
            b"#include without file name\x00" as *const u8 as *mut i8,
        ); //end else
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if token.type_0 == 1 {
        crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
        PC_ConvertPath(token.string.as_mut_ptr());
        script = crate::src::botlib::l_script::LoadScriptFile(token.string.as_mut_ptr());
        if script.is_null() {
            crate::src::qcommon::q_shared::Q_strncpyz(
                path.as_mut_ptr(),
                (*source).includepath.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 64]>() as i32,
            );
            crate::src::qcommon::q_shared::Q_strcat(
                path.as_mut_ptr(),
                ::std::mem::size_of::<[i8; 64]>() as i32,
                token.string.as_mut_ptr(),
            );
            script = crate::src::botlib::l_script::LoadScriptFile(path.as_mut_ptr())
        }
    //end if
    } else if token.type_0 == 5 && *token.string.as_mut_ptr() as i32 == '<' as i32 {
        crate::src::qcommon::q_shared::Q_strncpyz(
            path.as_mut_ptr(),
            (*source).includepath.as_mut_ptr(),
            ::std::mem::size_of::<[i8; 64]>() as i32,
        ); //end if
        while PC_ReadSourceToken(source, &mut token) != 0 {
            //end while
            if token.linescrossed > 0 {
                PC_UnreadSourceToken(source, &mut token); //end if
                break; //end if
            } else {
                if token.type_0 == 5 && *token.string.as_mut_ptr() as i32 == '>' as i32 {
                    break; //end if
                }
                crate::src::qcommon::q_shared::Q_strcat(
                    path.as_mut_ptr(),
                    ::std::mem::size_of::<[i8; 64]>() as i32,
                    token.string.as_mut_ptr(),
                );
            }
        }
        if *token.string.as_mut_ptr() as i32 != '>' as i32 {
            SourceWarning(
                source,
                b"#include missing trailing >\x00" as *const u8 as *mut i8,
            );
        }
        if crate::stdlib::strlen(path.as_mut_ptr()) == 0 {
            SourceError(
                source,
                b"#include without file name between < >\x00" as *const u8 as *mut i8,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        PC_ConvertPath(path.as_mut_ptr());
        script = crate::src::botlib::l_script::LoadScriptFile(path.as_mut_ptr())
    } else {
        SourceError(
            source,
            b"#include without file name\x00" as *const u8 as *mut i8,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //QUAKE
    if script.is_null() {
        SourceError(
            source,
            b"file %s not found\x00" as *const u8 as *mut i8,
            path.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as i32; //end if
                                                             //SCREWUP
    }
    PC_PushScript(source, script);
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//read a token only if on the same line, lines are concatenated with a slash
//end of the function PC_Directive_include
//============================================================================
// reads a token from the current line, continues reading on the next
// line only if a backslash '\' is encountered.
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ReadLine(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut crossline: i32 = 0; //end if
    crossline = 0;
    loop {
        if PC_ReadSourceToken(source, token) == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if (*token).linescrossed > crossline {
            PC_UnreadSourceToken(source, token);
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        crossline = 1;
        if !(crate::stdlib::strcmp(
            (*token).string.as_mut_ptr(),
            b"\\\x00" as *const u8 as *const i8,
        ) == 0)
        {
            break;
        }
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//returns true if there was a white space in front of the token
//end of the function PC_ReadLine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_WhiteSpaceBeforeToken(
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    return ((*token)
        .endwhitespace_p
        .wrapping_offset_from((*token).whitespace_p)
        > 0) as i32;
}
//end of the function PC_WhiteSpaceBeforeToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ClearTokenWhiteSpace(
    mut token: *mut crate::src::botlib::l_script::token_t,
) {
    (*token).whitespace_p = 0 as *mut i8;
    (*token).endwhitespace_p = 0 as *mut i8;
    (*token).linescrossed = 0;
}
//end of the function PC_ClearTokenWhiteSpace
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_undef(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut lastdefine: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut hash: i32 = 0;
    if (*source).skip > 0 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //
    if PC_ReadLine(source, &mut token) == 0 {
        SourceError(source, b"undef without name\x00" as *const u8 as *mut i8); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    } //end if
    if token.type_0 != 4 {
        PC_UnreadSourceToken(source, &mut token); //end for
        SourceError(
            source,
            b"expected name, found %s\x00" as *const u8 as *mut i8,
            token.string.as_mut_ptr(),
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    } //end else
    hash = PC_NameHash(token.string.as_mut_ptr()); //end if
    lastdefine = 0 as *mut crate::src::botlib::l_precomp::define_t;
    define = *(*source).definehash.offset(hash as isize);
    while !define.is_null() {
        if crate::stdlib::strcmp((*define).name, token.string.as_mut_ptr()) == 0 {
            if (*define).flags & 0x1 != 0 {
                SourceWarning(
                    source,
                    b"can\'t undef %s\x00" as *const u8 as *mut i8,
                    token.string.as_mut_ptr(),
                );
            } else {
                if !lastdefine.is_null() {
                    (*lastdefine).hashnext = (*define).hashnext
                } else {
                    let ref mut fresh4 = *(*source).definehash.offset(hash as isize);
                    *fresh4 = (*define).hashnext
                }
                PC_FreeDefine(define);
            }
            break;
        } else {
            lastdefine = define;
            define = (*define).hashnext
        }
    }
    //DEFINEHASHING
    //DEFINEHASHING
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_Directive_undef
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_define(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut last: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    if (*source).skip > 0 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //
    if PC_ReadLine(source, &mut token) == 0 {
        SourceError(source, b"#define without name\x00" as *const u8 as *mut i8); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    } //end if
    if token.type_0 != 4 {
        PC_UnreadSourceToken(source, &mut token);
        SourceError(
            source,
            b"expected name after #define, found %s\x00" as *const u8 as *mut i8,
            token.string.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //check if the define already exists
    define = PC_FindHashedDefine((*source).definehash, token.string.as_mut_ptr());
    //DEFINEHASHING
    if !define.is_null() {
        //end if
        if (*define).flags & 0x1 != 0 {
            SourceError(
                source,
                b"can\'t redefine %s\x00" as *const u8 as *mut i8,
                token.string.as_mut_ptr(),
            ); //end if
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        SourceWarning(
            source,
            b"redefinition of %s\x00" as *const u8 as *mut i8,
            token.string.as_mut_ptr(),
        );
        //unread the define name before executing the #undef directive
        PC_UnreadSourceToken(source, &mut token);
        if PC_Directive_undef(source) == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    //allocate define
    define = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_precomp::define_t,
    >()) as *mut crate::src::botlib::l_precomp::define_t;
    crate::stdlib::memset(
        define as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::botlib::l_precomp::define_t>(),
    );
    (*define).name = crate::src::botlib::l_memory::GetMemory(
        crate::stdlib::strlen(token.string.as_mut_ptr()).wrapping_add(1usize),
    ) as *mut i8;
    crate::stdlib::strcpy((*define).name, token.string.as_mut_ptr());
    //add the define to the source
    PC_AddDefineToHash(define, (*source).definehash);
    //DEFINEHASHING
    //DEFINEHASHING
    //if nothing is defined, just return
    if PC_ReadLine(source, &mut token) == 0 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //if it is a define with parameters
    if PC_WhiteSpaceBeforeToken(&mut token) == 0
        && crate::stdlib::strcmp(
            token.string.as_mut_ptr(),
            b"(\x00" as *const u8 as *const i8,
        ) == 0
    {
        //end if
        //read the define parameters
        last = 0 as *mut crate::src::botlib::l_script::token_t; //end if
        if PC_CheckTokenString(source, b")\x00" as *const u8 as *mut i8) == 0 {
            loop {
                if PC_ReadLine(source, &mut token) == 0 {
                    SourceError(
                        source,
                        b"expected define parameter\x00" as *const u8 as *mut i8,
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
                //end if
                //if it isn't a name
                if token.type_0 != 4 {
                    SourceError(
                        source,
                        b"invalid define parameter\x00" as *const u8 as *mut i8,
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
                //
                if PC_FindDefineParm(define, token.string.as_mut_ptr()) >= 0 {
                    SourceError(
                        source,
                        b"two the same define parameters\x00" as *const u8 as *mut i8,
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
                //add the define parm
                t = PC_CopyToken(&mut token);
                PC_ClearTokenWhiteSpace(t);
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !last.is_null() {
                    (*last).next = t
                } else {
                    (*define).parms = t
                }
                last = t;
                (*define).numparms += 1;
                //read next token
                if PC_ReadLine(source, &mut token) == 0 {
                    SourceError(
                        source,
                        b"define parameters not terminated\x00" as *const u8 as *mut i8,
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
                //
                if crate::stdlib::strcmp(
                    token.string.as_mut_ptr(),
                    b")\x00" as *const u8 as *const i8,
                ) == 0
                {
                    break;
                }
                if crate::stdlib::strcmp(
                    token.string.as_mut_ptr(),
                    b",\x00" as *const u8 as *const i8,
                ) != 0
                {
                    SourceError(source, b"define not terminated\x00" as *const u8 as *mut i8);
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
            }
            //then it must be a comma
            //end while
        }
        if PC_ReadLine(source, &mut token) == 0 {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    //read the defined stuff
    last = 0 as *mut crate::src::botlib::l_script::token_t; //end if
    loop {
        t = PC_CopyToken(&mut token);
        if (*t).type_0 == 4 && crate::stdlib::strcmp((*t).string.as_mut_ptr(), (*define).name) == 0
        {
            SourceError(
                source,
                b"recursive define (removed recursion)\x00" as *const u8 as *mut i8,
            );
        } else {
            PC_ClearTokenWhiteSpace(t);
            (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
            if !last.is_null() {
                (*last).next = t
            } else {
                (*define).tokens = t
            }
            last = t
        }
        if !(PC_ReadLine(source, &mut token) != 0) {
            break;
        }
    }
    //
    if !last.is_null() {
        //end if
        //check for merge operators at the beginning or end
        if crate::stdlib::strcmp(
            (*(*define).tokens).string.as_mut_ptr(),
            b"##\x00" as *const u8 as *const i8,
        ) == 0
            || crate::stdlib::strcmp(
                (*last).string.as_mut_ptr(),
                b"##\x00" as *const u8 as *const i8,
            ) == 0
        {
            SourceError(
                source,
                b"define with misplaced ##\x00" as *const u8 as *mut i8,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //end if
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_Directive_define
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_DefineFromString(
    mut string: *mut i8,
) -> *mut crate::src::botlib::l_precomp::define_t {
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    let mut src: crate::src::botlib::l_precomp::source_t =
        crate::src::botlib::l_precomp::source_t {
            filename: [0; 1024],
            includepath: [0; 1024],
            punctuations: 0 as *mut crate::src::botlib::l_script::punctuation_t,
            scriptstack: 0 as *mut crate::src::botlib::l_script::script_t,
            tokens: 0 as *mut crate::src::botlib::l_script::token_t,
            defines: 0 as *mut crate::src::botlib::l_precomp::define_t,
            definehash: 0 as *mut *mut crate::src::botlib::l_precomp::define_t,
            indentstack: 0 as *mut crate::src::botlib::l_precomp::indent_t,
            skip: 0,
            token: crate::src::botlib::l_script::token_t {
                string: [0; 1024],
                type_0: 0,
                subtype: 0,
                intvalue: 0,
                floatvalue: 0.,
                whitespace_p: 0 as *mut i8,
                endwhitespace_p: 0 as *mut i8,
                line: 0,
                linescrossed: 0,
                next: 0 as *mut crate::src::botlib::l_script::token_s,
            },
        };
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut res: i32 = 0;
    let mut i: i32 = 0;
    let mut def: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    PC_InitTokenHeap();
    script = crate::src::botlib::l_script::LoadScriptMemory(
        string,
        crate::stdlib::strlen(string) as i32,
        b"*extern\x00" as *const u8 as *mut i8,
    );
    //create a new source
    crate::stdlib::memset(
        &mut src as *mut crate::src::botlib::l_precomp::source_t as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::botlib::l_precomp::source_t>(),
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        src.filename.as_mut_ptr(),
        b"*extern\x00" as *const u8 as *const i8,
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    src.scriptstack = script;
    src.definehash = crate::src::botlib::l_memory::GetClearedMemory((1024usize).wrapping_mul(
        ::std::mem::size_of::<*mut crate::src::botlib::l_precomp::define_t>(),
    )) as *mut *mut crate::src::botlib::l_precomp::define_t;
    //DEFINEHASHING
    //create a define from the source
    res = PC_Directive_define(&mut src);
    //free any tokens if left
    t = src.tokens; //end for
    while !t.is_null() {
        src.tokens = (*src.tokens).next; //end for
        PC_FreeToken(t);
        t = src.tokens
    }
    def = 0 as *mut crate::src::botlib::l_precomp::define_t;
    i = 0;
    while i < 1024 {
        if !(*src.definehash.offset(i as isize)).is_null() {
            def = *src.definehash.offset(i as isize);
            break;
        } else {
            i += 1
        }
        //end if
    }
    //DEFINEHASHING
    //
    crate::src::botlib::l_memory::FreeMemory(src.definehash as *mut libc::c_void);
    //DEFINEHASHING
    //
    crate::src::botlib::l_script::FreeScript(script);
    //if the define was created successfully
    if res > 0 {
        return def;
    }
    //free the define is created
    if !src.defines.is_null() {
        PC_FreeDefine(def);
    }
    //
    return 0 as *mut crate::src::botlib::l_precomp::define_t;
}
//add a define to the source
//end of the function PC_DefineFromString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_AddDefine(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut string: *mut i8,
) -> i32 {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    define = PC_DefineFromString(string);
    if define.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    PC_AddDefineToHash(define, (*source).definehash);
    //DEFINEHASHING
    //DEFINEHASHING
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//add a globals define that will be added to all opened sources
//end of the function PC_AddDefine
//============================================================================
// add a globals define that will be added to all opened sources
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_AddGlobalDefine(mut string: *mut i8) -> i32 {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    define = PC_DefineFromString(string);
    if define.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    (*define).next = globaldefines;
    globaldefines = define;
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//remove the given global define
//end of the function PC_AddGlobalDefine
//============================================================================
// remove the given global define
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_RemoveGlobalDefine(mut name: *mut i8) -> i32 {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t; //end if
    define = PC_FindDefine(globaldefines, name);
    if !define.is_null() {
        PC_FreeDefine(define);
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//remove all globals defines
//end of the function PC_RemoveGlobalDefine
//============================================================================
// remove all globals defines
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_RemoveAllGlobalDefines() {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    define = globaldefines;
    while !define.is_null() {
        globaldefines = (*globaldefines).next;
        PC_FreeDefine(define);
        define = globaldefines
    }
    //end for
}
//end of the function PC_RemoveAllGlobalDefines
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_CopyDefine(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut define: *mut crate::src::botlib::l_precomp::define_t,
) -> *mut crate::src::botlib::l_precomp::define_t {
    let mut newdefine: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut token: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut newtoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut lasttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    newdefine = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_precomp::define_t,
    >()) as *mut crate::src::botlib::l_precomp::define_t;
    //copy the define name
    (*newdefine).name = crate::src::botlib::l_memory::GetMemory(
        crate::stdlib::strlen((*define).name).wrapping_add(1usize),
    ) as *mut i8;
    crate::stdlib::strcpy((*newdefine).name, (*define).name);
    (*newdefine).flags = (*define).flags;
    (*newdefine).builtin = (*define).builtin;
    (*newdefine).numparms = (*define).numparms;
    //the define is not linked
    (*newdefine).next = 0 as *mut crate::src::botlib::l_precomp::define_s;
    (*newdefine).hashnext = 0 as *mut crate::src::botlib::l_precomp::define_s;
    //copy the define tokens
    (*newdefine).tokens = 0 as *mut crate::src::botlib::l_script::token_t; //end for
    lasttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    token = (*define).tokens;
    while !token.is_null() {
        newtoken = PC_CopyToken(token);
        (*newtoken).next = 0 as *mut crate::src::botlib::l_script::token_s;
        if !lasttoken.is_null() {
            (*lasttoken).next = newtoken
        } else {
            (*newdefine).tokens = newtoken
        }
        lasttoken = newtoken;
        token = (*token).next
    }
    //copy the define parameters
    (*newdefine).parms = 0 as *mut crate::src::botlib::l_script::token_t; //end for
    lasttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    token = (*define).parms;
    while !token.is_null() {
        newtoken = PC_CopyToken(token);
        (*newtoken).next = 0 as *mut crate::src::botlib::l_script::token_s;
        if !lasttoken.is_null() {
            (*lasttoken).next = newtoken
        } else {
            (*newdefine).parms = newtoken
        }
        lasttoken = newtoken;
        token = (*token).next
    }
    return newdefine;
}
//end of the function PC_CopyDefine
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_AddGlobalDefinesToSource(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut newdefine: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    define = globaldefines;
    while !define.is_null() {
        newdefine = PC_CopyDefine(source, define);
        PC_AddDefineToHash(newdefine, (*source).definehash);
        define = (*define).next
        //DEFINEHASHING
        //DEFINEHASHING
    }
    //end for
}
//end of the function PC_AddGlobalDefinesToSource
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_if_def(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut type_0: i32,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end if
    let mut d: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t; //end if
    let mut skip: i32 = 0;
    if PC_ReadLine(source, &mut token) == 0 {
        SourceError(source, b"#ifdef without name\x00" as *const u8 as *mut i8);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if token.type_0 != 4 {
        PC_UnreadSourceToken(source, &mut token);
        SourceError(
            source,
            b"expected name after #ifdef, found %s\x00" as *const u8 as *mut i8,
            token.string.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    d = PC_FindHashedDefine((*source).definehash, token.string.as_mut_ptr());
    //DEFINEHASHING
    skip = ((type_0 == 0x8) as i32
        == (d == 0 as *mut crate::src::botlib::l_precomp::define_t) as i32) as i32;
    PC_PushIndent(source, type_0, skip);
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_Directiveif_def
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_ifdef(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    return PC_Directive_if_def(source, 0x8);
}
//end of the function PC_Directive_ifdef
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_ifndef(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    return PC_Directive_if_def(source, 0x10);
}
//end of the function PC_Directive_ifndef
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_else(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut type_0: i32 = 0; //end if
    let mut skip: i32 = 0; //end if
    PC_PopIndent(source, &mut type_0, &mut skip);
    if type_0 == 0 {
        SourceError(source, b"misplaced #else\x00" as *const u8 as *mut i8);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if type_0 == 0x2 {
        SourceError(source, b"#else after #else\x00" as *const u8 as *mut i8);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    PC_PushIndent(source, 0x2, (skip == 0) as i32);
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_Directive_else
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_endif(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut type_0: i32 = 0; //end if
    let mut skip: i32 = 0; //end switch
    PC_PopIndent(source, &mut type_0, &mut skip);
    if type_0 == 0 {
        SourceError(source, b"misplaced #endif\x00" as *const u8 as *mut i8);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
#[no_mangle]

pub unsafe extern "C" fn PC_OperatorPriority(mut op: i32) -> i32 {
    match op {
        26 => return 15,
        27 => return 15,
        28 => return 15,
        29 => return 14,
        30 => return 14,
        5 => return 7,
        6 => return 6,
        7 => return 12,
        8 => return 12,
        9 => return 11,
        10 => return 11,
        36 => return 16,
        37 => return 12,
        38 => return 12,
        21 => return 13,
        22 => return 13,
        32 => return 10,
        33 => return 8,
        34 => return 9,
        35 => return 16,
        42 => return 5,
        43 => return 5,
        _ => {}
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//
#[no_mangle]

pub unsafe extern "C" fn PC_EvaluateTokens(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut tokens: *mut crate::src::botlib::l_script::token_t,
    mut intvalue: *mut isize,
    mut floatvalue: *mut f32,
    mut integer: i32,
) -> i32 {
    let mut o: *mut operator_t = 0 as *mut operator_t;
    let mut firstoperator: *mut operator_t = 0 as *mut operator_t;
    let mut lastoperator: *mut operator_t = 0 as *mut operator_t;
    let mut v: *mut value_t = 0 as *mut value_t;
    let mut firstvalue: *mut value_t = 0 as *mut value_t;
    let mut lastvalue: *mut value_t = 0 as *mut value_t;
    let mut v1: *mut value_t = 0 as *mut value_t;
    let mut v2: *mut value_t = 0 as *mut value_t;
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut brace: i32 = 0;
    let mut parentheses: i32 = 0;
    let mut error: i32 = 0;
    let mut lastwasvalue: i32 = 0;
    let mut negativevalue: i32 = 0;
    let mut questmarkintvalue: i32 = 0;
    let mut questmarkfloatvalue: f32 = 0f32;
    let mut gotquestmarkvalue: i32 = crate::src::qcommon::q_shared::qfalse as i32;
    //
    let mut operator_heap: [operator_t; 64] = [operator_t {
        operator: 0,
        priority: 0,
        parentheses: 0,
        prev: 0 as *mut operator_s,
        next: 0 as *mut operator_s,
    }; 64]; //end for
    let mut numoperators: i32 = 0; //end switch
    let mut value_heap: [value_t; 64] = [value_t {
        intvalue: 0,
        floatvalue: 0.,
        parentheses: 0,
        prev: 0 as *mut value_s,
        next: 0 as *mut value_s,
    }; 64]; //end if
    let mut numvalues: i32 = 0; //end if
    lastoperator = 0 as *mut operator_t; //end if
    firstoperator = lastoperator; //end if
    lastvalue = 0 as *mut value_t;
    firstvalue = lastvalue;
    if !intvalue.is_null() {
        *intvalue = 0isize
    }
    if !floatvalue.is_null() {
        *floatvalue = 0f32
    }
    t = tokens;
    while !t.is_null() {
        let mut current_block_97: u64;
        match (*t).type_0 {
            4 => {
                if lastwasvalue != 0 || negativevalue != 0 {
                    SourceError(
                        source,
                        b"syntax error in #if/#elif\x00" as *const u8 as *mut i8,
                    );
                    error = 1
                } else if crate::stdlib::strcmp(
                    (*t).string.as_mut_ptr(),
                    b"defined\x00" as *const u8 as *const i8,
                ) != 0
                {
                    SourceError(
                        source,
                        b"undefined name %s in #if/#elif\x00" as *const u8 as *mut i8,
                        (*t).string.as_mut_ptr(),
                    );
                    error = 1
                } else {
                    t = (*t).next;
                    if crate::stdlib::strcmp(
                        (*t).string.as_mut_ptr(),
                        b"(\x00" as *const u8 as *const i8,
                    ) == 0
                    {
                        brace = crate::src::qcommon::q_shared::qtrue as i32;
                        t = (*t).next
                    }
                    if t.is_null() || (*t).type_0 != 4 {
                        SourceError(
                            source,
                            b"defined without name in #if/#elif\x00" as *const u8 as *mut i8,
                        );
                        error = 1
                    } else if numvalues >= 64 {
                        SourceError(source, b"out of value space\x00" as *const u8 as *mut i8);
                        error = 1
                    } else {
                        let fresh5 = numvalues;
                        numvalues = numvalues + 1;
                        v = &mut *value_heap.as_mut_ptr().offset(fresh5 as isize) as *mut value_t;
                        //v = (value_t *) GetClearedMemory(sizeof(value_t));
                        if !PC_FindHashedDefine((*source).definehash, (*t).string.as_mut_ptr())
                            .is_null()
                        {
                            //end else
                            //DEFINEHASHING
                            (*v).intvalue = 1; //end if
                            (*v).floatvalue = 1f32
                        } else {
                            (*v).intvalue = 0; //end if
                            (*v).floatvalue = 0f32
                        }
                        (*v).parentheses = parentheses;
                        (*v).next = 0 as *mut value_s;
                        (*v).prev = lastvalue;
                        if !lastvalue.is_null() {
                            (*lastvalue).next = v
                        } else {
                            firstvalue = v
                        }
                        lastvalue = v;
                        if brace != 0 {
                            t = (*t).next;
                            if t.is_null()
                                || crate::stdlib::strcmp(
                                    (*t).string.as_mut_ptr(),
                                    b")\x00" as *const u8 as *const i8,
                                ) != 0
                            {
                                SourceError(
                                    source,
                                    b"defined without ) in #if/#elif\x00" as *const u8 as *mut i8,
                                );
                                error = 1;
                                current_block_97 = 2473505634946569239;
                            } else {
                                current_block_97 = 1854459640724737493;
                            }
                        //end if
                        } else {
                            current_block_97 = 1854459640724737493;
                        }
                        match current_block_97 {
                            2473505634946569239 => {}
                            _ => {
                                brace = crate::src::qcommon::q_shared::qfalse as i32;
                                // defined() creates a value
                                lastwasvalue = 1
                            }
                        }
                    }
                }
            }
            3 => {
                if lastwasvalue != 0 {
                    SourceError(
                        source,
                        b"syntax error in #if/#elif\x00" as *const u8 as *mut i8,
                    ); //end if
                    error = 1
                } else if numvalues >= 64 {
                    SourceError(source, b"out of value space\x00" as *const u8 as *mut i8);
                    error = 1
                } else {
                    let fresh6 = numvalues;
                    numvalues = numvalues + 1;
                    v = &mut *value_heap.as_mut_ptr().offset(fresh6 as isize) as *mut value_t;
                    //v = (value_t *) GetClearedMemory(sizeof(value_t));
                    if negativevalue != 0 {
                        //end else
                        (*v).intvalue = -((*t).intvalue as i32) as isize; //end if
                        (*v).floatvalue = -(*t).floatvalue
                    } else {
                        (*v).intvalue = (*t).intvalue as isize;
                        (*v).floatvalue = (*t).floatvalue
                    }
                    (*v).parentheses = parentheses;
                    (*v).next = 0 as *mut value_s;
                    (*v).prev = lastvalue;
                    if !lastvalue.is_null() {
                        (*lastvalue).next = v
                    } else {
                        firstvalue = v
                    }
                    lastvalue = v;
                    //last token was a value
                    lastwasvalue = 1;
                    //
                    negativevalue = 0
                }
            }
            5 => {
                if negativevalue != 0 {
                    SourceError(
                        source,
                        b"misplaced minus sign in #if/#elif\x00" as *const u8 as *mut i8,
                    ); //end if
                    error = 1
                } else if (*t).subtype == 44 {
                    //end else if
                    parentheses += 1
                } else if (*t).subtype == 45 {
                    parentheses -= 1; //end if
                    if parentheses < 0 {
                        SourceError(
                            source,
                            b"too many ) in #if/#elsif\x00" as *const u8 as *mut i8,
                        ); //end if
                        error = 1
                    }
                } else {
                    //check for invalid operators on floating point values
                    if integer == 0 {
                        if (*t).subtype == 35
                            || (*t).subtype == 28
                            || (*t).subtype == 21
                            || (*t).subtype == 22
                            || (*t).subtype == 32
                            || (*t).subtype == 33
                            || (*t).subtype == 34
                        {
                            SourceError(
                                source,
                                b"illigal operator %s on floating point operands\x00" as *const u8
                                    as *mut i8,
                                (*t).string.as_mut_ptr(),
                            ); //end if
                            error = 1;
                            current_block_97 = 2473505634946569239;
                        } else {
                            current_block_97 = 6712462580143783635;
                        }
                    //end if
                    } else {
                        current_block_97 = 6712462580143783635; //end switch
                    } //end if
                    match current_block_97 {
                        2473505634946569239 => {}
                        _ => {
                            let mut current_block_80: u64; //end case
                            match (*t).subtype {
                                36 | 35 => {
                                    if lastwasvalue != 0 {
                                        SourceError(
                                            source,
                                            b"! or ~ after value in #if/#elif\x00" as *const u8
                                                as *mut i8,
                                        ); //end case
                                        error = 1;
                                        current_block_80 = 10248984122780841972;
                                    } else {
                                        current_block_80 = 10248984122780841972;
                                    }
                                }
                                16 | 17 => {
                                    SourceError(
                                        source,
                                        b"++ or -- used in #if/#elif\x00" as *const u8 as *mut i8,
                                    );
                                    current_block_80 = 10248984122780841972;
                                }
                                30 => {
                                    if lastwasvalue == 0 {
                                        negativevalue = 1;
                                        current_block_80 = 10248984122780841972;
                                    } else {
                                        current_block_80 = 5706227035632243100;
                                    }
                                    //end if
                                }
                                26 | 27 | 28 | 29 | 5 | 6 | 7 | 8 | 9 | 10 | 37 | 38 | 21 | 22
                                | 32 | 33 | 34 | 42 | 43 => {
                                    current_block_80 = 5706227035632243100; //end if
                                }
                                _ => {
                                    SourceError(
                                        source,
                                        b"invalid operator %s in #if/#elif\x00" as *const u8
                                            as *mut i8,
                                        (*t).string.as_mut_ptr(),
                                    ); //end if
                                    error = 1;
                                    current_block_80 = 10248984122780841972;
                                }
                            }
                            match current_block_80 {
                                5706227035632243100 => {
                                    if lastwasvalue == 0 {
                                        SourceError(
                                            source,
                                            b"operator %s after operator in #if/#elif\x00"
                                                as *const u8
                                                as *mut i8,
                                            (*t).string.as_mut_ptr(),
                                        );
                                        error = 1
                                    }
                                }
                                _ => {}
                            }
                            if error == 0 && negativevalue == 0 {
                                //o = (operator_t *) GetClearedMemory(sizeof(operator_t));
                                if numoperators >= 64 {
                                    SourceError(
                                        source,
                                        b"out of operator space\x00" as *const u8 as *mut i8,
                                    ); //end if
                                    error = 1
                                } else {
                                    let fresh7 = numoperators; //end if
                                    numoperators = numoperators + 1;
                                    o = &mut *operator_heap.as_mut_ptr().offset(fresh7 as isize)
                                        as *mut operator_t;
                                    (*o).operator = (*t).subtype;
                                    (*o).priority = PC_OperatorPriority((*t).subtype);
                                    (*o).parentheses = parentheses;
                                    (*o).next = 0 as *mut operator_s;
                                    (*o).prev = lastoperator;
                                    if !lastoperator.is_null() {
                                        (*lastoperator).next = o
                                    } else {
                                        firstoperator = o
                                    }
                                    lastoperator = o;
                                    lastwasvalue = 0
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                SourceError(
                    source,
                    b"unknown %s in #if/#elif\x00" as *const u8 as *mut i8,
                    (*t).string.as_mut_ptr(),
                );
                error = 1
            }
        }
        if error != 0 {
            break;
        }
        t = (*t).next
    }
    if error == 0 {
        if lastwasvalue == 0 {
            SourceError(
                source,
                b"trailing operator in #if/#elif\x00" as *const u8 as *mut i8,
            );
            error = 1
        } else if parentheses != 0 {
            SourceError(
                source,
                b"too many ( in #if/#elif\x00" as *const u8 as *mut i8,
            );
            error = 1
        }
        //end else if
    }
    //
    gotquestmarkvalue = crate::src::qcommon::q_shared::qfalse as i32;
    questmarkintvalue = 0;
    questmarkfloatvalue = 0f32;
    //while there are operators
    while error == 0 && !firstoperator.is_null() {
        v = firstvalue; //end while
        o = firstoperator; //end for
        while !(*o).next.is_null() {
            //if the current operator is nested deeper in parentheses
            //than the next operator
            if (*o).parentheses > (*(*o).next).parentheses {
                break;
            }
            //if the current and next operator are nested equally deep in parentheses
            if (*o).parentheses == (*(*o).next).parentheses {
                //end if
                //if the priority of the current operator is equal or higher
                //than the priority of the next operator
                if (*o).priority >= (*(*o).next).priority {
                    break;
                }
            }
            //if the arity of the operator isn't equal to 1
            if (*o).operator != 36 && (*o).operator != 35 {
                v = (*v).next
            }
            //if there's no value or no next value
            if v.is_null() {
                SourceError(
                    source,
                    b"mising values in #if/#elif\x00" as *const u8 as *mut i8,
                );
                error = 1;
                break;
            } else {
                o = (*o).next
            }
            //end if
        }
        if error != 0 {
            break;
        }
        v1 = v;
        v2 = (*v).next;
        //DEBUG_EVAL
        match (*o).operator {
            36 => {
                (*v1).intvalue = ((*v1).intvalue == 0) as i32 as isize;
                (*v1).floatvalue = ((*v1).floatvalue == 0.) as i32 as f32
                //end switch
                //end if
            }
            35 => (*v1).intvalue = !(*v1).intvalue,
            26 => {
                (*v1).intvalue *= (*v2).intvalue; //end if
                (*v1).floatvalue *= (*v2).floatvalue
            }
            27 => {
                if (*v2).intvalue == 0 || (*v2).floatvalue == 0. {
                    SourceError(
                        source,
                        b"divide by zero in #if/#elif\x00" as *const u8 as *mut i8,
                    ); //end else
                    error = 1
                } else {
                    (*v1).intvalue /= (*v2).intvalue; //end if
                    (*v1).floatvalue /= (*v2).floatvalue
                }
            }
            28 => {
                if (*v2).intvalue == 0 {
                    SourceError(
                        source,
                        b"divide by zero in #if/#elif\x00" as *const u8 as *mut i8,
                    ); //end if
                    error = 1
                } else {
                    (*v1).intvalue %= (*v2).intvalue
                }
            }
            29 => {
                (*v1).intvalue += (*v2).intvalue;
                (*v1).floatvalue += (*v2).floatvalue
            }
            30 => {
                (*v1).intvalue -= (*v2).intvalue;
                (*v1).floatvalue -= (*v2).floatvalue
            }
            5 => {
                (*v1).intvalue = ((*v1).intvalue != 0 && (*v2).intvalue != 0) as i32 as isize;
                (*v1).floatvalue = ((*v1).floatvalue != 0. && (*v2).floatvalue != 0.) as i32 as f32
            }
            6 => {
                (*v1).intvalue = ((*v1).intvalue != 0 || (*v2).intvalue != 0) as i32 as isize;
                (*v1).floatvalue = ((*v1).floatvalue != 0. || (*v2).floatvalue != 0.) as i32 as f32
            }
            7 => {
                (*v1).intvalue = ((*v1).intvalue >= (*v2).intvalue) as i32 as isize;
                (*v1).floatvalue = ((*v1).floatvalue >= (*v2).floatvalue) as i32 as f32
            }
            8 => {
                (*v1).intvalue = ((*v1).intvalue <= (*v2).intvalue) as i32 as isize;
                (*v1).floatvalue = ((*v1).floatvalue <= (*v2).floatvalue) as i32 as f32
            }
            9 => {
                (*v1).intvalue = ((*v1).intvalue == (*v2).intvalue) as i32 as isize;
                (*v1).floatvalue = ((*v1).floatvalue == (*v2).floatvalue) as i32 as f32
            }
            10 => {
                (*v1).intvalue = ((*v1).intvalue != (*v2).intvalue) as i32 as isize;
                (*v1).floatvalue = ((*v1).floatvalue != (*v2).floatvalue) as i32 as f32
            }
            37 => {
                (*v1).intvalue = ((*v1).intvalue > (*v2).intvalue) as i32 as isize;
                (*v1).floatvalue = ((*v1).floatvalue > (*v2).floatvalue) as i32 as f32
            }
            38 => {
                (*v1).intvalue = ((*v1).intvalue < (*v2).intvalue) as i32 as isize;
                (*v1).floatvalue = ((*v1).floatvalue < (*v2).floatvalue) as i32 as f32
            }
            21 => (*v1).intvalue >>= (*v2).intvalue,
            22 => (*v1).intvalue <<= (*v2).intvalue,
            32 => (*v1).intvalue &= (*v2).intvalue,
            33 => (*v1).intvalue |= (*v2).intvalue,
            34 => (*v1).intvalue ^= (*v2).intvalue,
            42 => {
                if gotquestmarkvalue == 0 {
                    SourceError(
                        source,
                        b": without ? in #if/#elif\x00" as *const u8 as *mut i8,
                    );
                    error = 1
                } else {
                    if integer != 0 {
                        if questmarkintvalue == 0 {
                            (*v1).intvalue = (*v2).intvalue
                        }
                    } else if questmarkfloatvalue == 0. {
                        (*v1).floatvalue = (*v2).floatvalue
                    }
                    gotquestmarkvalue = crate::src::qcommon::q_shared::qfalse as i32
                }
            }
            43 => {
                if gotquestmarkvalue != 0 {
                    SourceError(
                        source,
                        b"? after ? in #if/#elif\x00" as *const u8 as *mut i8,
                    );
                    error = 1
                } else {
                    questmarkintvalue = (*v1).intvalue as i32;
                    questmarkfloatvalue = (*v1).floatvalue;
                    gotquestmarkvalue = crate::src::qcommon::q_shared::qtrue as i32
                }
            }
            _ => {}
        }
        //DEBUG_EVAL
        if error != 0 {
            break;
        }
        //if not an operator with arity 1
        if (*o).operator != 36 && (*o).operator != 35 {
            //end if
            //remove the second value if not question mark operator
            if (*o).operator != 43 {
                v = (*v).next
            }
            //
            if !v.is_null() {
                if !(*v).prev.is_null() {
                    (*(*v).prev).next = (*v).next
                } else {
                    firstvalue = (*v).next
                }
                if !(*v).next.is_null() {
                    (*(*v).next).prev = (*v).prev
                }
            }
        }
        //remove the operator
        if !(*o).prev.is_null() {
            (*(*o).prev).next = (*o).next
        } else {
            firstoperator = (*o).next
        } //end if
        if !(*o).next.is_null() {
            (*(*o).next).prev = (*o).prev
        }
    } //end for
    if !firstvalue.is_null() {
        if !intvalue.is_null() {
            *intvalue = (*firstvalue).intvalue
        } //end for
        if !floatvalue.is_null() {
            *floatvalue = (*firstvalue).floatvalue
        }
    }
    o = firstoperator;
    while !o.is_null() {
        lastoperator = (*o).next;
        o = lastoperator
    }
    v = firstvalue;
    while !v.is_null() {
        lastvalue = (*v).next;
        v = lastvalue
    }
    if error == 0 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    if !intvalue.is_null() {
        *intvalue = 0isize
    }
    if !floatvalue.is_null() {
        *floatvalue = 0f32
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function PC_EvaluateTokens
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Evaluate(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut intvalue: *mut isize,
    mut floatvalue: *mut f32,
    mut integer: i32,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut firsttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut lasttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut nexttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut defined: i32 = crate::src::qcommon::q_shared::qfalse as i32;
    if !intvalue.is_null() {
        *intvalue = 0isize
    }
    if !floatvalue.is_null() {
        *floatvalue = 0f32
    }
    //
    if PC_ReadLine(source, &mut token) == 0 {
        SourceError(
            source,
            b"no value after #if/#elif\x00" as *const u8 as *mut i8,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    firsttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    lasttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    loop {
        //if the token is a name
        if token.type_0 == 4 {
            if defined != 0 {
                defined = crate::src::qcommon::q_shared::qfalse as i32; //end if
                t = PC_CopyToken(&mut token); //end if
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else {
                    firsttoken = t
                }
                lasttoken = t
            } else if crate::stdlib::strcmp(
                token.string.as_mut_ptr(),
                b"defined\x00" as *const u8 as *const i8,
            ) == 0
            {
                defined = crate::src::qcommon::q_shared::qtrue as i32;
                t = PC_CopyToken(&mut token);
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else {
                    firsttoken = t
                }
                lasttoken = t
            } else {
                //then it must be a define
                define = PC_FindHashedDefine((*source).definehash, token.string.as_mut_ptr());
                //DEFINEHASHING
                if define.is_null() {
                    SourceError(
                        source,
                        b"can\'t evaluate %s, not defined\x00" as *const u8 as *mut i8,
                        token.string.as_mut_ptr(),
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
                if PC_ExpandDefineIntoSource(source, &mut token, define) == 0 {
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
            }
        //end else
        } else if token.type_0 == 3 || token.type_0 == 5 {
            //if the token is a number or a punctuation
            t = PC_CopyToken(&mut token); //end else
            (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
            if !lasttoken.is_null() {
                (*lasttoken).next = t
            } else {
                firsttoken = t
            }
            lasttoken = t
        } else {
            //can't evaluate the token
            SourceError(
                source,
                b"can\'t evaluate %s\x00" as *const u8 as *mut i8,
                token.string.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if !(PC_ReadLine(source, &mut token) != 0) {
            break;
        }
        //end else
    }
    //
    if PC_EvaluateTokens(source, firsttoken, intvalue, floatvalue, integer) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    //DEBUG_EVAL
    t = firsttoken; //end for
    while !t.is_null() {
        //DEBUG_EVAL
        nexttoken = (*t).next;
        PC_FreeToken(t);
        t = nexttoken
    }
    //DEBUG_EVAL
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_Evaluate
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_DollarEvaluate(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut intvalue: *mut isize,
    mut floatvalue: *mut f32,
    mut integer: i32,
) -> i32 {
    let mut indent: i32 = 0;
    let mut defined: i32 = crate::src::qcommon::q_shared::qfalse as i32;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut firsttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut lasttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut t: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut nexttoken: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    if !intvalue.is_null() {
        *intvalue = 0isize
    }
    if !floatvalue.is_null() {
        *floatvalue = 0f32
    }
    //
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"no leading ( after $evalint/$evalfloat\x00" as *const u8 as *mut i8,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    } //end if
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(source, b"nothing to evaluate\x00" as *const u8 as *mut i8);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    indent = 1;
    firsttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    lasttoken = 0 as *mut crate::src::botlib::l_script::token_t;
    loop
    //if the token is a name
    {
        if token.type_0 == 4 {
            if defined != 0 {
                defined = crate::src::qcommon::q_shared::qfalse as i32; //end if
                t = PC_CopyToken(&mut token); //end if
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else {
                    firsttoken = t
                }
                lasttoken = t
            } else if crate::stdlib::strcmp(
                token.string.as_mut_ptr(),
                b"defined\x00" as *const u8 as *const i8,
            ) == 0
            {
                defined = crate::src::qcommon::q_shared::qtrue as i32;
                t = PC_CopyToken(&mut token);
                (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
                if !lasttoken.is_null() {
                    (*lasttoken).next = t
                } else {
                    firsttoken = t
                }
                lasttoken = t
            } else {
                //then it must be a define
                define = PC_FindHashedDefine((*source).definehash, token.string.as_mut_ptr());
                //DEFINEHASHING
                if define.is_null() {
                    SourceError(
                        source,
                        b"can\'t evaluate %s, not defined\x00" as *const u8 as *mut i8,
                        token.string.as_mut_ptr(),
                    ); //end if
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
                if PC_ExpandDefineIntoSource(source, &mut token, define) == 0 {
                    return crate::src::qcommon::q_shared::qfalse as i32;
                }
            }
        //end else
        } else if token.type_0 == 3 || token.type_0 == 5 {
            //if the token is a number or a punctuation
            if *token.string.as_mut_ptr() as i32 == '(' as i32 {
                indent += 1
            } else if *token.string.as_mut_ptr() as i32 == ')' as i32 {
                indent -= 1
            } //end else
            if indent <= 0 {
                break;
            }
            t = PC_CopyToken(&mut token);
            (*t).next = 0 as *mut crate::src::botlib::l_script::token_s;
            if !lasttoken.is_null() {
                (*lasttoken).next = t
            } else {
                firsttoken = t
            }
            lasttoken = t
        } else {
            //can't evaluate the token
            SourceError(
                source,
                b"can\'t evaluate %s\x00" as *const u8 as *mut i8,
                token.string.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if !(PC_ReadSourceToken(source, &mut token) != 0) {
            break;
        }
        //end else
    }
    //
    if PC_EvaluateTokens(source, firsttoken, intvalue, floatvalue, integer) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    //DEBUG_EVAL
    t = firsttoken; //end for
    while !t.is_null() {
        //DEBUG_EVAL
        nexttoken = (*t).next;
        PC_FreeToken(t);
        t = nexttoken
    }
    //DEBUG_EVAL
    //
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_DollarEvaluate
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_elif(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut value: isize = 0; //end if
    let mut type_0: i32 = 0;
    let mut skip: i32 = 0;
    PC_PopIndent(source, &mut type_0, &mut skip);
    if type_0 == 0 || type_0 == 0x2 {
        SourceError(source, b"misplaced #elif\x00" as *const u8 as *mut i8);
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if PC_Evaluate(
        source,
        &mut value,
        0 as *mut f32,
        crate::src::qcommon::q_shared::qtrue as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    skip = (value == 0) as i32;
    PC_PushIndent(source, 0x4, skip);
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_Directive_elif
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_if(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut value: isize = 0;
    let mut skip: i32 = 0;
    if PC_Evaluate(
        source,
        &mut value,
        0 as *mut f32,
        crate::src::qcommon::q_shared::qtrue as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    skip = (value == 0) as i32;
    PC_PushIndent(source, 0x1, skip);
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_Directive
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_line(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    SourceError(
        source,
        b"#line directive not supported\x00" as *const u8 as *mut i8,
    );
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function PC_Directive_line
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_error(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    crate::stdlib::strcpy(token.string.as_mut_ptr(), b"\x00" as *const u8 as *const i8);
    PC_ReadSourceToken(source, &mut token);
    SourceError(
        source,
        b"#error directive: %s\x00" as *const u8 as *mut i8,
        token.string.as_mut_ptr(),
    );
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function PC_Directive_error
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_pragma(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    SourceWarning(
        source,
        b"#pragma directive not supported\x00" as *const u8 as *mut i8,
    );
    while PC_ReadLine(source, &mut token) != 0 {}
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_Directive_pragma
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn UnreadSignToken(mut source: *mut crate::src::botlib::l_precomp::source_t) {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0;
    crate::stdlib::strcpy(
        token.string.as_mut_ptr(),
        b"-\x00" as *const u8 as *const i8,
    );
    token.type_0 = 5;
    token.subtype = 30;
    PC_UnreadSourceToken(source, &mut token);
}
//end of the function UnreadSignToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_eval(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut value: isize = 0;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_Evaluate(
        source,
        &mut value,
        0 as *mut f32,
        crate::src::qcommon::q_shared::qtrue as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0;
    crate::stdlib::sprintf(
        token.string.as_mut_ptr(),
        b"%ld\x00" as *const u8 as *const i8,
        crate::stdlib::labs(value),
    );
    token.type_0 = 3;
    token.subtype = 0x1000 | 0x2000 | 0x8;
    PC_UnreadSourceToken(source, &mut token);
    if value < 0 {
        UnreadSignToken(source);
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_Directive_eval
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_Directive_evalfloat(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut value: f32 = 0.;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_Evaluate(
        source,
        0 as *mut isize,
        &mut value,
        crate::src::qcommon::q_shared::qfalse as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0;
    crate::stdlib::sprintf(
        token.string.as_mut_ptr(),
        b"%1.2f\x00" as *const u8 as *const i8,
        crate::stdlib::fabs(value as f64),
    );
    token.type_0 = 3;
    token.subtype = 0x800 | 0x2000 | 0x8;
    PC_UnreadSourceToken(source, &mut token);
    if value < 0f32 {
        UnreadSignToken(source);
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_Directive_evalfloat
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub static mut directives: [directive_t; 20] = unsafe {
    [
        {
            let mut init = directive_s {
                name: b"if\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_if
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"ifdef\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_ifdef
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"ifndef\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_ifndef
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"elif\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_elif
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"else\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_else
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"endif\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_endif
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"include\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_include
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"define\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_define
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"undef\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_undef
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"line\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_line
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"error\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_error
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"pragma\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_pragma
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"eval\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_eval
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"evalfloat\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_Directive_evalfloat
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: 0 as *mut i8,
                func: None,
            };
            init
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
    ]
};
#[no_mangle]

pub unsafe extern "C" fn PC_ReadDirective(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut i: i32 = 0;
    //read the directive name
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(source, b"found # without name\x00" as *const u8 as *mut i8); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //directive name must be on the same line
    if token.linescrossed > 0 {
        PC_UnreadSourceToken(source, &mut token); //end if
        SourceError(
            source,
            b"found # at end of line\x00" as *const u8 as *mut i8,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if if is a name
    if token.type_0 == 4 {
        //end if
        //find the precompiler directive
        i = 0;
        while !directives[i as usize].name.is_null() {
            if crate::stdlib::strcmp(directives[i as usize].name, token.string.as_mut_ptr()) == 0 {
                return directives[i as usize]
                    .func
                    .expect("non-null function pointer")(source);
            }
            i += 1
            //end if
        }
        //end for
    }
    SourceError(
        source,
        b"unknown precompiler directive %s\x00" as *const u8 as *mut i8,
        token.string.as_mut_ptr(),
    );
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//end of the function PC_ReadDirective
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_DollarDirective_evalint(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut value: isize = 0;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_DollarEvaluate(
        source,
        &mut value,
        0 as *mut f32,
        crate::src::qcommon::q_shared::qtrue as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0;
    crate::stdlib::sprintf(
        token.string.as_mut_ptr(),
        b"%ld\x00" as *const u8 as *const i8,
        crate::stdlib::labs(value),
    );
    token.type_0 = 3;
    token.subtype = 0x1000 | 0x2000 | 0x8;
    token.intvalue = crate::stdlib::labs(value) as usize;
    token.floatvalue = token.intvalue as f32;
    //NUMBERVALUE
    PC_UnreadSourceToken(source, &mut token);
    if value < 0 {
        UnreadSignToken(source);
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_DollarDirective_evalint
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_DollarDirective_evalfloat(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut value: f32 = 0.;
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_DollarEvaluate(
        source,
        0 as *mut isize,
        &mut value,
        crate::src::qcommon::q_shared::qfalse as i32,
    ) == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    token.line = (*(*source).scriptstack).line;
    token.whitespace_p = (*(*source).scriptstack).script_p;
    token.endwhitespace_p = (*(*source).scriptstack).script_p;
    token.linescrossed = 0;
    crate::stdlib::sprintf(
        token.string.as_mut_ptr(),
        b"%1.2f\x00" as *const u8 as *const i8,
        crate::stdlib::fabs(value as f64),
    );
    token.type_0 = 3;
    token.subtype = 0x800 | 0x2000 | 0x8;
    token.floatvalue = crate::stdlib::fabs(value as f64) as f32;
    token.intvalue = token.floatvalue as usize;
    //NUMBERVALUE
    PC_UnreadSourceToken(source, &mut token);
    if value < 0f32 {
        UnreadSignToken(source);
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_DollarDirective_evalfloat
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub static mut dollardirectives: [directive_t; 20] = unsafe {
    [
        {
            let mut init = directive_s {
                name: b"evalint\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_DollarDirective_evalint
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: b"evalfloat\x00" as *const u8 as *mut i8,
                func: Some(
                    PC_DollarDirective_evalfloat
                        as unsafe extern "C" fn(
                            _: *mut crate::src::botlib::l_precomp::source_t,
                        ) -> i32,
                ),
            };
            init
        },
        {
            let mut init = directive_s {
                name: 0 as *mut i8,
                func: None,
            };
            init
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
        directive_t {
            name: 0 as *mut i8,
            func: None,
        },
    ]
};
#[no_mangle]

pub unsafe extern "C" fn PC_ReadDollarDirective(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut i: i32 = 0;
    //read the directive name
    if PC_ReadSourceToken(source, &mut token) == 0 {
        SourceError(source, b"found $ without name\x00" as *const u8 as *mut i8); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //directive name must be on the same line
    if token.linescrossed > 0 {
        PC_UnreadSourceToken(source, &mut token); //end if
        SourceError(
            source,
            b"found $ at end of line\x00" as *const u8 as *mut i8,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if if is a name
    if token.type_0 == 4 {
        //end if
        //find the precompiler directive
        i = 0;
        while !dollardirectives[i as usize].name.is_null() {
            if crate::stdlib::strcmp(dollardirectives[i as usize].name, token.string.as_mut_ptr())
                == 0
            {
                return dollardirectives[i as usize]
                    .func
                    .expect("non-null function pointer")(source);
            }
            i += 1
            //end if
        }
        //end for
    }
    PC_UnreadSourceToken(source, &mut token);
    SourceError(
        source,
        b"unknown precompiler directive %s\x00" as *const u8 as *mut i8,
        token.string.as_mut_ptr(),
    );
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//read a token from the source
//end of the function PC_ReadDirective
//QUAKEC
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ReadToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    loop {
        if PC_ReadSourceToken(source, token) == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //check for precompiler directives
        if (*token).type_0 == 5 && *(*token).string.as_mut_ptr() as i32 == '#' as i32 {
            //end if
            //QUAKC
            //read the precompiler directive
            if PC_ReadDirective(source) == 0 {
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
        //end if
        } else if (*token).type_0 == 5 && *(*token).string.as_mut_ptr() as i32 == '$' as i32 {
            //end if
            //QUAKEC
            //read the precompiler directive
            if PC_ReadDollarDirective(source) == 0 {
                return crate::src::qcommon::q_shared::qfalse as i32;
            }
        //end if
        } else {
            // recursively concatenate strings that are behind each other still resolving defines
            if (*token).type_0 == 1 {
                let mut newtoken: crate::src::botlib::l_script::token_t =
                    crate::src::botlib::l_script::token_t {
                        string: [0; 1024],
                        type_0: 0,
                        subtype: 0,
                        intvalue: 0,
                        floatvalue: 0.,
                        whitespace_p: 0 as *mut i8,
                        endwhitespace_p: 0 as *mut i8,
                        line: 0,
                        linescrossed: 0,
                        next: 0 as *mut crate::src::botlib::l_script::token_s,
                    }; //end if
                if PC_ReadToken(source, &mut newtoken) != 0 {
                    if newtoken.type_0 == 1 {
                        (*token).string[crate::stdlib::strlen((*token).string.as_mut_ptr())
                            .wrapping_sub(1usize)] = '\u{0}' as i8;
                        if crate::stdlib::strlen((*token).string.as_mut_ptr())
                            .wrapping_add(crate::stdlib::strlen(
                                newtoken.string.as_mut_ptr().offset(1),
                            ))
                            .wrapping_add(1usize)
                            >= 1024usize
                        {
                            SourceError(
                                source,
                                b"string longer than MAX_TOKEN %d\x00" as *const u8 as *mut i8,
                                1024i32,
                            );
                            return crate::src::qcommon::q_shared::qfalse as i32;
                        }
                        crate::stdlib::strcat(
                            (*token).string.as_mut_ptr(),
                            newtoken.string.as_mut_ptr().offset(1isize),
                        );
                    } else {
                        PC_UnreadToken(source, &mut newtoken);
                    }
                }
            }
            //if skipping source because of conditional compilation
            if (*source).skip != 0 {
                continue;
            }
            //if the token is a name
            if (*token).type_0 == 4 {
                //end if
                //check if the name is a define macro
                define = PC_FindHashedDefine((*source).definehash, (*token).string.as_mut_ptr());
                //end if
                if !define.is_null() {
                    //DEFINEHASHING
                    //if it is a define macro
                    //expand the defined macro
                    if PC_ExpandDefineIntoSource(source, token, define) == 0 {
                        return crate::src::qcommon::q_shared::qfalse as i32;
                    }
                    continue;
                }
            }
            //copy token for unreading
            crate::stdlib::memcpy(
                &mut (*source).token as *mut crate::src::botlib::l_script::token_t
                    as *mut libc::c_void,
                token as *const libc::c_void,
                ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
            );
            //found a token
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    //end while
}
//expect a certain token
//end of the function PC_ReadToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpectTokenString(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut string: *mut i8,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end if
    if PC_ReadToken(source, &mut token) == 0 {
        SourceError(
            source,
            b"couldn\'t find expected %s\x00" as *const u8 as *mut i8,
            string,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if crate::stdlib::strcmp(token.string.as_mut_ptr(), string) != 0 {
        SourceError(
            source,
            b"expected %s, found %s\x00" as *const u8 as *mut i8,
            string,
            token.string.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//expect a certain token type
//end of the function PC_ExpectTokenString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpectTokenType(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut type_0: i32,
    mut subtype: i32,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut str: [i8; 1024] = [0; 1024]; //end if
    if PC_ReadToken(source, token) == 0 {
        SourceError(
            source,
            b"couldn\'t read expected token\x00" as *const u8 as *mut i8,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    } //end else if
    if (*token).type_0 != type_0 {
        crate::stdlib::strcpy(str.as_mut_ptr(), b"\x00" as *const u8 as *const i8);
        if type_0 == 1 {
            crate::stdlib::strcpy(str.as_mut_ptr(), b"string\x00" as *const u8 as *const i8);
        }
        if type_0 == 2 {
            crate::stdlib::strcpy(str.as_mut_ptr(), b"literal\x00" as *const u8 as *const i8);
        }
        if type_0 == 3 {
            crate::stdlib::strcpy(str.as_mut_ptr(), b"number\x00" as *const u8 as *const i8);
        }
        if type_0 == 4 {
            crate::stdlib::strcpy(str.as_mut_ptr(), b"name\x00" as *const u8 as *const i8);
        }
        if type_0 == 5 {
            crate::stdlib::strcpy(
                str.as_mut_ptr(),
                b"punctuation\x00" as *const u8 as *const i8,
            );
        }
        SourceError(
            source,
            b"expected a %s, found %s\x00" as *const u8 as *mut i8,
            str.as_mut_ptr(),
            (*token).string.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if (*token).type_0 == 3 {
        if (*token).subtype & subtype != subtype {
            crate::stdlib::strcpy(str.as_mut_ptr(), b"\x00" as *const u8 as *const i8);
            if subtype & 0x8 != 0 {
                crate::stdlib::strcpy(str.as_mut_ptr(), b"decimal\x00" as *const u8 as *const i8);
            }
            if subtype & 0x100 != 0 {
                crate::stdlib::strcpy(str.as_mut_ptr(), b"hex\x00" as *const u8 as *const i8);
            }
            if subtype & 0x200 != 0 {
                crate::stdlib::strcpy(str.as_mut_ptr(), b"octal\x00" as *const u8 as *const i8);
            }
            if subtype & 0x400 != 0 {
                crate::stdlib::strcpy(str.as_mut_ptr(), b"binary\x00" as *const u8 as *const i8);
            }
            if subtype & 0x2000 != 0 {
                crate::stdlib::strcat(str.as_mut_ptr(), b" long\x00" as *const u8 as *const i8);
            }
            if subtype & 0x4000 != 0 {
                crate::stdlib::strcat(str.as_mut_ptr(), b" unsigned\x00" as *const u8 as *const i8);
            }
            if subtype & 0x800 != 0 {
                crate::stdlib::strcat(str.as_mut_ptr(), b" float\x00" as *const u8 as *const i8);
            }
            if subtype & 0x1000 != 0 {
                crate::stdlib::strcat(str.as_mut_ptr(), b" integer\x00" as *const u8 as *const i8);
            }
            SourceError(
                source,
                b"expected %s, found %s\x00" as *const u8 as *mut i8,
                str.as_mut_ptr(),
                (*token).string.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    //end if
    } else if (*token).type_0 == 5 {
        if (*token).subtype != subtype {
            SourceError(
                source,
                b"found %s\x00" as *const u8 as *mut i8,
                (*token).string.as_mut_ptr(),
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //end if
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//expect a token
//end of the function PC_ExpectTokenType
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ExpectAnyToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    if PC_ReadToken(source, token) == 0 {
        SourceError(
            source,
            b"couldn\'t read expected token\x00" as *const u8 as *mut i8,
        ); //end if
        return crate::src::qcommon::q_shared::qfalse as i32;
    } else {
        return crate::src::qcommon::q_shared::qtrue as i32;
    };
    //end else
}
//returns true when the token is available
//end of the function PC_ExpectAnyToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_CheckTokenString(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut string: *mut i8,
) -> i32 {
    let mut tok: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_ReadToken(source, &mut tok) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the token is available
    if crate::stdlib::strcmp(tok.string.as_mut_ptr(), string) == 0 {
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //
    PC_UnreadSourceToken(source, &mut tok);
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//returns true and reads the token when a token with the given type is available
//end of the function PC_CheckTokenString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_CheckTokenType(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut type_0: i32,
    mut subtype: i32,
    mut token: *mut crate::src::botlib::l_script::token_t,
) -> i32 {
    let mut tok: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    if PC_ReadToken(source, &mut tok) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    //if the type matches
    if tok.type_0 == type_0 && tok.subtype & subtype == subtype {
        crate::stdlib::memcpy(
            token as *mut libc::c_void,
            &mut tok as *mut crate::src::botlib::l_script::token_t as *const libc::c_void,
            ::std::mem::size_of::<crate::src::botlib::l_script::token_t>(),
        ); //end if
        return crate::src::qcommon::q_shared::qtrue as i32;
    }
    //
    PC_UnreadSourceToken(source, &mut tok);
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//skip tokens until the given token string is read
//end of the function PC_CheckTokenType
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_SkipUntilString(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut string: *mut i8,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    }; //end while
    while PC_ReadToken(source, &mut token) != 0 {
        if crate::stdlib::strcmp(token.string.as_mut_ptr(), string) == 0 {
            return crate::src::qcommon::q_shared::qtrue as i32;
        }
    }
    return crate::src::qcommon::q_shared::qfalse as i32;
}
//unread the last token read from the script
//end of the function PC_SkipUntilString
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_UnreadLastToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) {
    PC_UnreadSourceToken(source, &mut (*source).token);
}
//unread the given token
//end of the function PC_UnreadLastToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_UnreadToken(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut token: *mut crate::src::botlib::l_script::token_t,
) {
    PC_UnreadSourceToken(source, token);
}
//set the source include path
//end of the function PC_UnreadToken
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_SetIncludePath(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut path: *mut i8,
) {
    let mut len: crate::stddef_h::size_t = 0;
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*source).includepath.as_mut_ptr(),
        path,
        (::std::mem::size_of::<[i8; 1024]>()).wrapping_sub(1usize) as i32,
    );
    len = crate::stdlib::strlen((*source).includepath.as_mut_ptr());
    //add trailing path seperator
    if len > 0
        && (*source).includepath[len.wrapping_sub(1usize)] as i32 != '\\' as i32
        && (*source).includepath[len.wrapping_sub(1usize)] as i32 != '/' as i32
    {
        crate::stdlib::strcat(
            (*source).includepath.as_mut_ptr(),
            b"/\x00" as *const u8 as *const i8,
        );
    };
    //end if
}
//set the punction set
//end of the function PC_SetIncludePath
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_SetPunctuations(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut p: *mut crate::src::botlib::l_script::punctuation_t,
) {
    (*source).punctuations = p;
}
//load a source file
//end of the function PC_SetPunctuations
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn LoadSourceFile(
    mut filename: *const i8,
) -> *mut crate::src::botlib::l_precomp::source_t {
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t;
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    PC_InitTokenHeap();
    script = crate::src::botlib::l_script::LoadScriptFile(filename);
    if script.is_null() {
        return 0 as *mut crate::src::botlib::l_precomp::source_t;
    }
    (*script).next = 0 as *mut crate::src::botlib::l_script::script_s;
    source = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_precomp::source_t,
    >()) as *mut crate::src::botlib::l_precomp::source_t;
    crate::stdlib::memset(
        source as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::botlib::l_precomp::source_t>(),
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*source).filename.as_mut_ptr(),
        filename,
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    (*source).scriptstack = script;
    (*source).tokens = 0 as *mut crate::src::botlib::l_script::token_t;
    (*source).defines = 0 as *mut crate::src::botlib::l_precomp::define_t;
    (*source).indentstack = 0 as *mut crate::src::botlib::l_precomp::indent_t;
    (*source).skip = 0;
    (*source).definehash = crate::src::botlib::l_memory::GetClearedMemory((1024usize).wrapping_mul(
        ::std::mem::size_of::<*mut crate::src::botlib::l_precomp::define_t>(),
    )) as *mut *mut crate::src::botlib::l_precomp::define_t;
    //DEFINEHASHING
    PC_AddGlobalDefinesToSource(source);
    return source;
}
//load a source from memory
//end of the function LoadSourceFile
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn LoadSourceMemory(
    mut ptr: *mut i8,
    mut length: i32,
    mut name: *mut i8,
) -> *mut crate::src::botlib::l_precomp::source_t {
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t;
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    PC_InitTokenHeap();
    script = crate::src::botlib::l_script::LoadScriptMemory(ptr, length, name);
    if script.is_null() {
        return 0 as *mut crate::src::botlib::l_precomp::source_t;
    }
    (*script).next = 0 as *mut crate::src::botlib::l_script::script_s;
    source = crate::src::botlib::l_memory::GetMemory(::std::mem::size_of::<
        crate::src::botlib::l_precomp::source_t,
    >()) as *mut crate::src::botlib::l_precomp::source_t;
    crate::stdlib::memset(
        source as *mut libc::c_void,
        0,
        ::std::mem::size_of::<crate::src::botlib::l_precomp::source_t>(),
    );
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*source).filename.as_mut_ptr(),
        name,
        ::std::mem::size_of::<[i8; 1024]>() as i32,
    );
    (*source).scriptstack = script;
    (*source).tokens = 0 as *mut crate::src::botlib::l_script::token_t;
    (*source).defines = 0 as *mut crate::src::botlib::l_precomp::define_t;
    (*source).indentstack = 0 as *mut crate::src::botlib::l_precomp::indent_t;
    (*source).skip = 0;
    (*source).definehash = crate::src::botlib::l_memory::GetClearedMemory((1024usize).wrapping_mul(
        ::std::mem::size_of::<*mut crate::src::botlib::l_precomp::define_t>(),
    )) as *mut *mut crate::src::botlib::l_precomp::define_t;
    //DEFINEHASHING
    PC_AddGlobalDefinesToSource(source);
    return source;
}
//free the given source
//end of the function LoadSourceMemory
//============================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn FreeSource(mut source: *mut crate::src::botlib::l_precomp::source_t) {
    let mut script: *mut crate::src::botlib::l_script::script_t =
        0 as *mut crate::src::botlib::l_script::script_t;
    let mut token: *mut crate::src::botlib::l_script::token_t =
        0 as *mut crate::src::botlib::l_script::token_t;
    let mut define: *mut crate::src::botlib::l_precomp::define_t =
        0 as *mut crate::src::botlib::l_precomp::define_t;
    let mut indent: *mut crate::src::botlib::l_precomp::indent_t =
        0 as *mut crate::src::botlib::l_precomp::indent_t;
    let mut i: i32 = 0;
    //PC_PrintDefineHashTable(source->definehash);
    //free all the scripts
    while !(*source).scriptstack.is_null() {
        script = (*source).scriptstack; //end for
        (*source).scriptstack = (*(*source).scriptstack).next;
        crate::src::botlib::l_script::FreeScript(script);
    }
    //free all the tokens
    while !(*source).tokens.is_null() {
        token = (*source).tokens; //end for
        (*source).tokens = (*(*source).tokens).next; //end for
        PC_FreeToken(token);
    }

    for i in 0..1024 {
        while !(*(*source).definehash.offset(i as isize)).is_null() {
            define = *(*source).definehash.offset(i as isize);
            let ref mut fresh8 = *(*source).definehash.offset(i as isize);
            *fresh8 = (**(*source).definehash.offset(i as isize)).hashnext;
            PC_FreeDefine(define);
        }
    }
    //DEFINEHASHING
    //DEFINEHASHING
    //free all indents
    while !(*source).indentstack.is_null() {
        indent = (*source).indentstack; //end for
        (*source).indentstack = (*(*source).indentstack).next;
        crate::src::botlib::l_memory::FreeMemory(indent as *mut libc::c_void);
    }
    //
    if !(*source).definehash.is_null() {
        crate::src::botlib::l_memory::FreeMemory((*source).definehash as *mut libc::c_void);
    }
    //DEFINEHASHING
    //free the source itself
    crate::src::botlib::l_memory::FreeMemory(source as *mut libc::c_void);
}
#[no_mangle]

pub static mut sourceFiles: [*mut crate::src::botlib::l_precomp::source_t; 64] =
    [0 as *mut crate::src::botlib::l_precomp::source_t; 64];
//BSPC
//
#[no_mangle]

pub unsafe extern "C" fn PC_LoadSourceHandle(mut filename: *const i8) -> i32 {
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t; //end for
    let mut i: i32 = 0;
    i = 1;
    while i < 64 {
        if sourceFiles[i as usize].is_null() {
            break;
        }
        i += 1
    }
    if i >= 64 {
        return 0i32;
    }
    crate::src::botlib::l_script::PS_SetBaseFolder(b"\x00" as *const u8 as *mut i8);
    source = LoadSourceFile(filename);
    if source.is_null() {
        return 0i32;
    }
    sourceFiles[i as usize] = source;
    return i;
}
//end of the function PC_LoadSourceHandle
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_FreeSourceHandle(mut handle: i32) -> i32 {
    if handle < 1 || handle >= 64 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if sourceFiles[handle as usize].is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    FreeSource(sourceFiles[handle as usize]);
    sourceFiles[handle as usize] = 0 as *mut crate::src::botlib::l_precomp::source_t;
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function PC_FreeSourceHandle
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_ReadTokenHandle(
    mut handle: i32,
    mut pc_token: *mut crate::src::qcommon::q_shared::pc_token_t,
) -> i32 {
    let mut token: crate::src::botlib::l_script::token_t = crate::src::botlib::l_script::token_t {
        string: [0; 1024],
        type_0: 0,
        subtype: 0,
        intvalue: 0,
        floatvalue: 0.,
        whitespace_p: 0 as *mut i8,
        endwhitespace_p: 0 as *mut i8,
        line: 0,
        linescrossed: 0,
        next: 0 as *mut crate::src::botlib::l_script::token_s,
    };
    let mut ret: i32 = 0;
    if handle < 1 || handle >= 64 {
        return 0i32;
    }
    if sourceFiles[handle as usize].is_null() {
        return 0i32;
    }
    ret = PC_ReadToken(sourceFiles[handle as usize], &mut token);
    crate::stdlib::strcpy((*pc_token).string.as_mut_ptr(), token.string.as_mut_ptr());
    (*pc_token).type_0 = token.type_0;
    (*pc_token).subtype = token.subtype;
    (*pc_token).intvalue = token.intvalue as i32;
    (*pc_token).floatvalue = token.floatvalue;
    if (*pc_token).type_0 == 1 {
        crate::src::botlib::l_script::StripDoubleQuotes((*pc_token).string.as_mut_ptr());
    }
    return ret;
}
//end of the function PC_ReadTokenHandle
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_SourceFileAndLine(
    mut handle: i32,
    mut filename: *mut i8,
    mut line: *mut i32,
) -> i32 {
    if handle < 1 || handle >= 64 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if sourceFiles[handle as usize].is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    crate::stdlib::strcpy(
        filename,
        (*sourceFiles[handle as usize]).filename.as_mut_ptr(),
    );
    if !(*sourceFiles[handle as usize]).scriptstack.is_null() {
        *line = (*(*sourceFiles[handle as usize]).scriptstack).line
    } else {
        *line = 0
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//set the base folder to load files from
//end of the function PC_SourceFileAndLine
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_SetBaseFolder(mut path: *mut i8) {
    crate::src::botlib::l_script::PS_SetBaseFolder(path);
}
//end of the function PC_SetBaseFolder
//============================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//============================================================================
#[no_mangle]

pub unsafe extern "C" fn PC_CheckOpenSourceHandles() {
    let mut i: i32 = 0;
    i = 1;
    while i < 64 {
        if !sourceFiles[i as usize].is_null() {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3i32,
                b"file %s still open in precompiler\n\x00" as *const u8 as *mut i8,
                (*(*sourceFiles[i as usize]).scriptstack)
                    .filename
                    .as_mut_ptr(),
            );
            //BOTLIB
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function PC_CheckOpenSourceHandles
