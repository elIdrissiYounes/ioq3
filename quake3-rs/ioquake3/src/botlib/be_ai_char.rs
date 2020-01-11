use ::libc;

pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;

pub use crate::src::botlib::l_precomp::define_s;
pub use crate::src::botlib::l_precomp::define_t;
pub use crate::src::botlib::l_precomp::indent_s;
pub use crate::src::botlib::l_precomp::indent_t;
pub use crate::src::botlib::l_precomp::source_s;
pub use crate::src::botlib::l_precomp::source_t;
pub use crate::src::botlib::l_precomp::FreeSource;
pub use crate::src::botlib::l_precomp::LoadSourceFile;
pub use crate::src::botlib::l_precomp::PC_ExpectAnyToken;
pub use crate::src::botlib::l_precomp::PC_ExpectTokenString;
pub use crate::src::botlib::l_precomp::PC_ExpectTokenType;
pub use crate::src::botlib::l_precomp::PC_ReadToken;
pub use crate::src::botlib::l_precomp::PC_SetBaseFolder;
pub use crate::src::botlib::l_precomp::SourceError;
pub use crate::src::botlib::l_script::punctuation_s;
pub use crate::src::botlib::l_script::punctuation_t;
pub use crate::src::botlib::l_script::script_s;
pub use crate::src::botlib::l_script::script_t;
pub use crate::src::botlib::l_script::token_s;
pub use crate::src::botlib::l_script::token_t;
pub use crate::src::botlib::l_script::StripDoubleQuotes;
pub use crate::src::qcommon::q_shared::byte;
pub use crate::src::qcommon::q_shared::cplane_s;
pub use crate::src::qcommon::q_shared::cplane_t;
pub use crate::src::qcommon::q_shared::fileHandle_t;
pub use crate::src::qcommon::q_shared::fsMode_t;
pub use crate::src::qcommon::q_shared::qboolean;
pub use crate::src::qcommon::q_shared::qfalse;
pub use crate::src::qcommon::q_shared::qtrue;
pub use crate::src::qcommon::q_shared::vec3_t;
pub use crate::src::qcommon::q_shared::vec_t;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;

//a bot character

pub type bot_character_t = bot_character_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_character_s {
    pub filename: [i8; 64],
    pub skill: f32,
    pub c: [bot_characteristic_t; 1],
}
//variable sized
//a characteristic

pub type bot_characteristic_t = bot_characteristic_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bot_characteristic_s {
    pub type_0: i8,
    pub value: cvalue,
}
//characteristic type
//characteristic value
//characteristic value

#[repr(C)]
#[derive(Copy, Clone)]
pub union cvalue {
    pub integer: i32,
    pub _float: f32,
    pub string: *mut i8,
}
#[no_mangle]

pub static mut botcharacters: [*mut bot_character_t; 65] = [0 as *mut bot_character_t; 65];
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotCharacterFromHandle(mut handle: i32) -> *mut bot_character_t {
    if handle <= 0 || handle > 64 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4,
            b"character handle %d out of range\n\x00" as *const u8 as *mut i8,
            handle,
        ); //end if
        return 0 as *mut bot_character_t;
    } //end if
    if botcharacters[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4,
            b"invalid character %d\n\x00" as *const u8 as *mut i8,
            handle,
        );
        return 0 as *mut bot_character_t;
    }
    return botcharacters[handle as usize];
}
//end of the function BotCharacterFromHandle
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDumpCharacter(mut ch: *mut bot_character_t) {
    let mut _i: i32 = 0; //end for
    crate::src::botlib::l_log::Log_Write(
        b"%s\n\x00" as *const u8 as *mut i8,
        (*ch).filename.as_mut_ptr(),
    );
    crate::src::botlib::l_log::Log_Write(
        b"skill %.1f\n\x00" as *const u8 as *mut i8,
        (*ch).skill as f64,
    );
    crate::src::botlib::l_log::Log_Write(b"{\n\x00" as *const u8 as *mut i8);

    for i in 0..80 {
        match (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 as i32 {
            1 => {
                crate::src::botlib::l_log::Log_Write(
                    b" %4d %d\n\x00" as *const u8 as *mut i8,
                    i,
                    (*(*ch).c.as_mut_ptr().offset(i as isize)).value.integer,
                );
            }
            2 => {
                crate::src::botlib::l_log::Log_Write(
                    b" %4d %f\n\x00" as *const u8 as *mut i8,
                    i,
                    (*(*ch).c.as_mut_ptr().offset(i as isize)).value._float as f64,
                );
            }
            3 => {
                crate::src::botlib::l_log::Log_Write(
                    b" %4d %s\n\x00" as *const u8 as *mut i8,
                    i,
                    (*(*ch).c.as_mut_ptr().offset(i as isize)).value.string,
                );
            }
            _ => {}
        }
    }
    crate::src::botlib::l_log::Log_Write(b"}\n\x00" as *const u8 as *mut i8);
}
//end of the function BotDumpCharacter
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeCharacterStrings(mut ch: *mut bot_character_t) {
    let mut i: i32 = 0;
    i = 0;
    while i < 80 {
        if (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 3 {
            crate::src::botlib::l_memory::FreeMemory(
                (*(*ch).c.as_mut_ptr().offset(i as isize)).value.string as *mut libc::c_void,
            );
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function BotFreeCharacterStrings
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeCharacter2(mut handle: i32) {
    if handle <= 0 || handle > 64 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4,
            b"character handle %d out of range\n\x00" as *const u8 as *mut i8,
            handle,
        ); //end if
        return;
    } //end if
    if botcharacters[handle as usize].is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            4,
            b"invalid character %d\n\x00" as *const u8 as *mut i8,
            handle,
        );
        return;
    }
    BotFreeCharacterStrings(botcharacters[handle as usize]);
    crate::src::botlib::l_memory::FreeMemory(botcharacters[handle as usize] as *mut libc::c_void);
    botcharacters[handle as usize] = 0 as *mut bot_character_t;
}
//frees a bot character
//end of the function BotFreeCharacter2
//========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFreeCharacter(mut handle: i32) {
    if crate::src::botlib::l_libvar::LibVarGetValue(
        b"bot_reloadcharacters\x00" as *const u8 as *const i8,
    ) == 0.
    {
        return;
    }
    BotFreeCharacter2(handle);
}
//end of the function BotFreeCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotDefaultCharacteristics(
    mut ch: *mut bot_character_t,
    mut defaultch: *mut bot_character_t,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < 80 {
        if !((*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 != 0) {
            //
            if (*(*defaultch).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 2 {
                (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 = 2i8; //end if
                (*(*ch).c.as_mut_ptr().offset(i as isize)).value._float =
                    (*(*defaultch).c.as_mut_ptr().offset(i as isize))
                        .value
                        ._float
            } else if (*(*defaultch).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 1 {
                (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 = 1i8; //end else if
                (*(*ch).c.as_mut_ptr().offset(i as isize)).value.integer =
                    (*(*defaultch).c.as_mut_ptr().offset(i as isize))
                        .value
                        .integer
            } else if (*(*defaultch).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 3 {
                (*(*ch).c.as_mut_ptr().offset(i as isize)).type_0 = 3i8;
                let ref mut fresh0 = (*(*ch).c.as_mut_ptr().offset(i as isize)).value.string;
                *fresh0 = crate::src::botlib::l_memory::GetMemory(
                    crate::stdlib::strlen(
                        (*(*defaultch).c.as_mut_ptr().offset(i as isize))
                            .value
                            .string,
                    )
                    .wrapping_add(1usize),
                ) as *mut i8;
                crate::stdlib::strcpy(
                    (*(*ch).c.as_mut_ptr().offset(i as isize)).value.string,
                    (*(*defaultch).c.as_mut_ptr().offset(i as isize))
                        .value
                        .string,
                );
            }
        }
        i += 1
        //end else if
    }
    //end for
}
//end of the function BotDefaultCharacteristics
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadCharacterFromFile(
    mut charfile: *mut i8,
    mut skill: i32,
) -> *mut bot_character_t {
    let mut indent: i32 = 0;
    let mut index: i32 = 0;
    let mut foundcharacter: i32 = 0;
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t;
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
    foundcharacter = crate::src::qcommon::q_shared::qfalse as i32;
    //a bot character is parsed in two phases
    crate::src::botlib::l_precomp::PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *mut i8); //end if
    source = crate::src::botlib::l_precomp::LoadSourceFile(charfile);
    if source.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"counldn\'t load %s\n\x00" as *const u8 as *mut i8,
            charfile,
        );
        return 0 as *mut bot_character_t;
    }
    ch = crate::src::botlib::l_memory::GetClearedMemory(
        (::std::mem::size_of::<bot_character_t>())
            .wrapping_add((80usize).wrapping_mul(::std::mem::size_of::<bot_characteristic_t>())),
    ) as *mut bot_character_t;
    crate::stdlib::strcpy((*ch).filename.as_mut_ptr(), charfile);
    //end else
    while crate::src::botlib::l_precomp::PC_ReadToken(source, &mut token) != 0 {
        if crate::stdlib::strcmp(
            token.string.as_mut_ptr(),
            b"skill\x00" as *const u8 as *const i8,
        ) == 0
        {
            //end while
            if crate::src::botlib::l_precomp::PC_ExpectTokenType(source, 3, 0, &mut token) == 0 {
                crate::src::botlib::l_precomp::FreeSource(source); //end if
                BotFreeCharacterStrings(ch);
                crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                return 0 as *mut bot_character_t;
            }
            //end else
            if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                source,
                b"{\x00" as *const u8 as *mut i8,
            ) == 0
            {
                crate::src::botlib::l_precomp::FreeSource(source); //end if
                BotFreeCharacterStrings(ch);
                crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                return 0 as *mut bot_character_t;
            }
            if skill < 0 || token.intvalue == skill as usize {
                //if it's the correct skill
                foundcharacter = crate::src::qcommon::q_shared::qtrue as i32; //end if
                (*ch).skill = token.intvalue as f32;
                while crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) != 0 {
                    if crate::stdlib::strcmp(
                        token.string.as_mut_ptr(),
                        b"}\x00" as *const u8 as *const i8,
                    ) == 0
                    {
                        break;
                        //end else
                    } //end if
                    if token.type_0 != 3 || token.subtype & 0x1000 == 0 {
                        crate::src::botlib::l_precomp::SourceError(
                            source,
                            b"expected integer index, found %s\x00" as *const u8 as *mut i8,
                            token.string.as_mut_ptr(),
                        ); //end if
                        crate::src::botlib::l_precomp::FreeSource(source); //end if
                        BotFreeCharacterStrings(ch); //end if
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void); //end if
                        return 0 as *mut bot_character_t;
                    } //end if
                    index = token.intvalue as i32;
                    if index < 0 || index > 80 {
                        crate::src::botlib::l_precomp::SourceError(
                            source,
                            b"characteristic index out of range [0, %d]\x00" as *const u8
                                as *mut i8,
                            80i32,
                        );
                        crate::src::botlib::l_precomp::FreeSource(source);
                        BotFreeCharacterStrings(ch);
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t;
                    }
                    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 != 0 {
                        crate::src::botlib::l_precomp::SourceError(
                            source,
                            b"characteristic %d already initialized\x00" as *const u8 as *mut i8,
                            index,
                        );
                        crate::src::botlib::l_precomp::FreeSource(source);
                        BotFreeCharacterStrings(ch);
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t;
                    }
                    if crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) == 0 {
                        crate::src::botlib::l_precomp::FreeSource(source);
                        BotFreeCharacterStrings(ch);
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t;
                    }
                    if token.type_0 == 3 {
                        if token.subtype & 0x800 != 0 {
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).value._float =
                                token.floatvalue;
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 = 2i8
                        } else {
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).value.integer =
                                token.intvalue as i32;
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 = 1i8
                        }
                    //end else
                    } else if token.type_0 == 1 {
                        crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr()); //end else if
                        let ref mut fresh1 =
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).value.string;
                        *fresh1 = crate::src::botlib::l_memory::GetMemory(
                            crate::stdlib::strlen(token.string.as_mut_ptr()).wrapping_add(1usize),
                        ) as *mut i8;
                        crate::stdlib::strcpy(
                            (*(*ch).c.as_mut_ptr().offset(index as isize)).value.string,
                            token.string.as_mut_ptr(),
                        );
                        (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 = 3i8
                    } else {
                        crate::src::botlib::l_precomp::SourceError(
                            source,
                            b"expected integer, float or string, found %s\x00" as *const u8
                                as *mut i8,
                            token.string.as_mut_ptr(),
                        );
                        crate::src::botlib::l_precomp::FreeSource(source);
                        BotFreeCharacterStrings(ch);
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t;
                    }
                }
                break;
            } else {
                indent = 1;
                while indent != 0 {
                    //end while
                    if crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) == 0 {
                        crate::src::botlib::l_precomp::FreeSource(source); //end if
                        BotFreeCharacterStrings(ch);
                        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
                        return 0 as *mut bot_character_t;
                    }
                    if crate::stdlib::strcmp(
                        token.string.as_mut_ptr(),
                        b"{\x00" as *const u8 as *const i8,
                    ) == 0
                    {
                        indent += 1
                    } else if crate::stdlib::strcmp(
                        token.string.as_mut_ptr(),
                        b"}\x00" as *const u8 as *const i8,
                    ) == 0
                    {
                        indent -= 1
                    }
                }
            }
        } else {
            crate::src::botlib::l_precomp::SourceError(
                source,
                b"unknown definition %s\x00" as *const u8 as *mut i8,
                token.string.as_mut_ptr(),
            );
            crate::src::botlib::l_precomp::FreeSource(source);
            BotFreeCharacterStrings(ch);
            crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
            return 0 as *mut bot_character_t;
        }
    }
    crate::src::botlib::l_precomp::FreeSource(source);
    //
    if foundcharacter == 0 {
        BotFreeCharacterStrings(ch); //end if
        crate::src::botlib::l_memory::FreeMemory(ch as *mut libc::c_void);
        return 0 as *mut bot_character_t;
    }
    return ch;
}
//end of the function BotLoadCharacterFromFile
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotFindCachedCharacter(mut charfile: *mut i8, mut skill: f32) -> i32 {
    let mut _handle: i32 = 0; //end for

    for handle in 1..=64 {
        if !botcharacters[handle as usize].is_null() {
            if crate::stdlib::strcmp(
                (*botcharacters[handle as usize]).filename.as_mut_ptr(),
                charfile,
            ) == 0
                && (skill < 0f32
                    || crate::stdlib::fabs(
                        ((*botcharacters[handle as usize]).skill - skill) as f64,
                    ) < 0.01)
            {
                return handle;
            }
        }
    }
    return 0;
}
//end of the function BotFindCachedCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadCachedCharacter(
    mut charfile: *mut i8,
    mut skill: f32,
    mut reload: i32,
) -> i32 {
    let mut handle: i32 = 0;
    let mut cachedhandle: i32 = 0;
    let mut intskill: i32 = 0;
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    //DEBUG
    //find a free spot for a character
    handle = 1; //end for
    while handle <= 64 {
        if botcharacters[handle as usize].is_null() {
            break;
        }
        handle += 1
    }
    if handle > 64 {
        return 0i32;
    }
    //try to load a cached character with the given skill
    if reload == 0 {
        cachedhandle = BotFindCachedCharacter(charfile, skill);
        if cachedhandle != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1,
                b"loaded cached skill %f from %s\n\x00" as *const u8 as *mut i8,
                skill as f64,
                charfile,
            ); //end else
            return cachedhandle;
        }
        //end if
    }
    //
    intskill = (skill as f64 + 0.5) as i32;
    //try to load the character with the given skill
    ch = BotLoadCharacterFromFile(charfile, intskill); //end if
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        //
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1,
            b"loaded skill %d from %s\n\x00" as *const u8 as *mut i8,
            intskill,
            charfile,
        );
        //DEBUG
        return handle;
    }
    //
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        2,
        b"couldn\'t find skill %d in %s\n\x00" as *const u8 as *mut i8,
        intskill,
        charfile,
    );
    //
    if reload == 0 {
        //end if
        //try to load a cached default character with the given skill
        cachedhandle =
            BotFindCachedCharacter(b"bots/default_c.c\x00" as *const u8 as *mut i8, skill);
        if cachedhandle != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1,
                b"loaded cached default skill %d from %s\n\x00" as *const u8 as *mut i8,
                intskill,
                charfile,
            );
            return cachedhandle;
        }
        //end if
    }
    //try to load the default character with the given skill
    ch = BotLoadCharacterFromFile(b"bots/default_c.c\x00" as *const u8 as *mut i8, intskill); //end if
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1,
            b"loaded default skill %d from %s\n\x00" as *const u8 as *mut i8,
            intskill,
            charfile,
        );
        return handle;
    }
    //
    if reload == 0 {
        //end if
        //try to load a cached character with any skill
        cachedhandle = BotFindCachedCharacter(charfile, -1f32);
        if cachedhandle != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1,
                b"loaded cached skill %f from %s\n\x00" as *const u8 as *mut i8,
                (*botcharacters[cachedhandle as usize]).skill as f64,
                charfile,
            );
            return cachedhandle;
        }
        //end if
    }
    //try to load a character with any skill
    ch = BotLoadCharacterFromFile(charfile, -(1)); //end if
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1,
            b"loaded skill %f from %s\n\x00" as *const u8 as *mut i8,
            (*ch).skill as f64,
            charfile,
        );
        return handle;
    }
    //
    if reload == 0 {
        //end if
        //try to load a cached character with any skill
        cachedhandle =
            BotFindCachedCharacter(b"bots/default_c.c\x00" as *const u8 as *mut i8, -1f32);
        if cachedhandle != 0 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                1,
                b"loaded cached default skill %f from %s\n\x00" as *const u8 as *mut i8,
                (*botcharacters[cachedhandle as usize]).skill as f64,
                charfile,
            );
            return cachedhandle;
        }
        //end if
    }
    //try to load a character with any skill
    ch = BotLoadCharacterFromFile(b"bots/default_c.c\x00" as *const u8 as *mut i8, -(1)); //end if
    if !ch.is_null() {
        botcharacters[handle as usize] = ch;
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1,
            b"loaded default skill %f from %s\n\x00" as *const u8 as *mut i8,
            (*ch).skill as f64,
            charfile,
        );
        return handle;
    }
    //
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        2,
        b"couldn\'t load any skill from %s\n\x00" as *const u8 as *mut i8,
        charfile,
    );
    //couldn't load any character
    return 0;
}
//end of the function BotLoadCachedCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadCharacterSkill(mut charfile: *mut i8, mut skill: f32) -> i32 {
    let mut ch: i32 = 0; //end if
    let mut defaultch: i32 = 0;
    defaultch = BotLoadCachedCharacter(
        b"bots/default_c.c\x00" as *const u8 as *mut i8,
        skill,
        crate::src::qcommon::q_shared::qfalse as i32,
    );
    ch = BotLoadCachedCharacter(
        charfile,
        skill,
        crate::src::botlib::l_libvar::LibVarGetValue(
            b"bot_reloadcharacters\x00" as *const u8 as *const i8,
        ) as i32,
    );
    if defaultch != 0 && ch != 0 {
        BotDefaultCharacteristics(
            botcharacters[ch as usize],
            botcharacters[defaultch as usize],
        );
    }
    return ch;
}
//end of the function BotLoadCharacterSkill
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotInterpolateCharacters(
    mut handle1: i32,
    mut handle2: i32,
    mut desiredskill: f32,
) -> i32 {
    let mut ch1: *mut bot_character_t = 0 as *mut bot_character_t;
    let mut ch2: *mut bot_character_t = 0 as *mut bot_character_t;
    let mut out: *mut bot_character_t = 0 as *mut bot_character_t;
    let mut _i: i32 = 0;
    let mut handle: i32 = 0;
    let mut scale: f32 = 0.;
    ch1 = BotCharacterFromHandle(handle1);
    ch2 = BotCharacterFromHandle(handle2);
    if ch1.is_null() || ch2.is_null() {
        return 0i32;
    }
    //find a free spot for a character
    handle = 1; //end for
    while handle <= 64 {
        if botcharacters[handle as usize].is_null() {
            break; //end for
        }
        handle += 1
    }
    if handle > 64 {
        return 0i32;
    }
    out = crate::src::botlib::l_memory::GetClearedMemory(
        (::std::mem::size_of::<bot_character_t>())
            .wrapping_add((80usize).wrapping_mul(::std::mem::size_of::<bot_characteristic_t>())),
    ) as *mut bot_character_t;
    (*out).skill = desiredskill;
    crate::stdlib::strcpy((*out).filename.as_mut_ptr(), (*ch1).filename.as_mut_ptr());
    botcharacters[handle as usize] = out;
    scale = (desiredskill - (*ch1).skill) / ((*ch2).skill - (*ch1).skill);

    for i in 0..80 {
        if (*(*ch1).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 2
            && (*(*ch2).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 2
        {
            (*(*out).c.as_mut_ptr().offset(i as isize)).type_0 = 2i8; //end if
            (*(*out).c.as_mut_ptr().offset(i as isize)).value._float =
                (*(*ch1).c.as_mut_ptr().offset(i as isize)).value._float
                    + ((*(*ch2).c.as_mut_ptr().offset(i as isize)).value._float
                        - (*(*ch1).c.as_mut_ptr().offset(i as isize)).value._float)
                        * scale
        } else if (*(*ch1).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 1 {
            (*(*out).c.as_mut_ptr().offset(i as isize)).type_0 = 1i8; //end else if
            (*(*out).c.as_mut_ptr().offset(i as isize)).value.integer =
                (*(*ch1).c.as_mut_ptr().offset(i as isize)).value.integer
        } else if (*(*ch1).c.as_mut_ptr().offset(i as isize)).type_0 as i32 == 3 {
            (*(*out).c.as_mut_ptr().offset(i as isize)).type_0 = 3i8;
            let ref mut fresh2 = (*(*out).c.as_mut_ptr().offset(i as isize)).value.string;
            *fresh2 = crate::src::botlib::l_memory::GetMemory(
                crate::stdlib::strlen((*(*ch1).c.as_mut_ptr().offset(i as isize)).value.string)
                    .wrapping_add(1usize),
            ) as *mut i8;
            crate::stdlib::strcpy(
                (*(*out).c.as_mut_ptr().offset(i as isize)).value.string,
                (*(*ch1).c.as_mut_ptr().offset(i as isize)).value.string,
            );
        }
    }
    return handle;
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
//
/* ****************************************************************************
 * name:		be_ai_char.h
 *
 * desc:		bot characters
 *
 * $Archive: /source/code/botlib/be_ai_char.h $
 *
 *****************************************************************************/
//loads a bot character from a file
//end of the function BotInterpolateCharacters
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotLoadCharacter(mut charfile: *mut i8, mut skill: f32) -> i32 {
    let mut firstskill: i32 = 0;
    let mut secondskill: i32 = 0;
    let mut handle: i32 = 0;
    //make sure the skill is in the valid range
    if (skill as f64) < 1.0 {
        skill = 1f32
    } else if skill as f64 > 5.0 {
        skill = 5f32
    }
    //skill 1, 4 and 5 should be available in the character files
    if skill as f64 == 1.0 || skill as f64 == 4.0 || skill as f64 == 5.0 {
        return BotLoadCharacterSkill(charfile, skill);
    } //end if
      //check if there's a cached skill
    handle = BotFindCachedCharacter(charfile, skill); //end if
    if handle != 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            1,
            b"loaded cached skill %f from %s\n\x00" as *const u8 as *mut i8,
            skill as f64,
            charfile,
        ); //end else
        return handle;
    } //end if
    if (skill as f64) < 4.0 {
        //load skill 1 and 4
        firstskill = BotLoadCharacterSkill(charfile, 1f32);
        if firstskill == 0 {
            return 0i32;
        }
        secondskill = BotLoadCharacterSkill(charfile, 4f32);
        if secondskill == 0 {
            return firstskill;
        }
    } else {
        //load skill 4 and 5
        firstskill = BotLoadCharacterSkill(charfile, 4f32);
        if firstskill == 0 {
            return 0i32;
        }
        secondskill = BotLoadCharacterSkill(charfile, 5f32);
        if secondskill == 0 {
            return firstskill;
        }
    }
    //interpolate between the two skills
    handle = BotInterpolateCharacters(firstskill, secondskill, skill);
    if handle == 0 {
        return 0i32;
    }
    //write the character to the log file
    BotDumpCharacter(botcharacters[handle as usize]);
    //
    return handle;
}
//end of the function BotLoadCharacter
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn CheckCharacteristicIndex(mut character: i32, mut index: i32) -> i32 {
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t; //end if
    ch = BotCharacterFromHandle(character); //end if
    if ch.is_null() {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if index < 0 || index >= 80 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"characteristic %d does not exist\n\x00" as *const u8 as *mut i8,
            index,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 == 0 {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"characteristic %d is not initialized\n\x00" as *const u8 as *mut i8,
            index,
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//returns a float characteristic
//end of the function CheckCharacteristicIndex
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Characteristic_Float(mut character: i32, mut index: i32) -> f32 {
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() {
        return 0f32;
    }
    //check if the index is in range
    if CheckCharacteristicIndex(character, index) == 0 {
        return 0f32;
    }
    //an integer will be converted to a float
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as i32 == 1 {
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value.integer as f32;
    } else if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as i32 == 2 {
        //end if
        //floats are just returned
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value._float;
    } else {
        //end else if
        //cannot convert a string pointer to a float
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"characteristic %d is not a float\n\x00" as *const u8 as *mut i8,
            index,
        );
        return 0f32;
    };
    //end else if
    //	return 0;
}
//returns a bounded float characteristic
//end of the function Characteristic_Float
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Characteristic_BFloat(
    mut character: i32,
    mut index: i32,
    mut min: f32,
    mut max: f32,
) -> f32 {
    let mut value: f32 = 0.; //end if
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() {
        return 0f32;
    }
    if min > max {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"cannot bound characteristic %d between %f and %f\n\x00" as *const u8 as *mut i8,
            index,
            min as f64,
            max as f64,
        );
        return 0f32;
    }
    value = Characteristic_Float(character, index);
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
//returns an integer characteristic
//end of the function Characteristic_BFloat
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Characteristic_Integer(mut character: i32, mut index: i32) -> i32 {
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() {
        return 0i32;
    }
    //check if the index is in range
    if CheckCharacteristicIndex(character, index) == 0 {
        return 0i32;
    }
    //an integer will just be returned
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as i32 == 1 {
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value.integer;
    } else if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as i32 == 2 {
        //end if
        //floats are casted to integers
        return (*(*ch).c.as_mut_ptr().offset(index as isize)).value._float as i32;
    } else {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"characteristic %d is not an integer\n\x00" as *const u8 as *mut i8,
            index,
        ); //end else if
        return 0i32;
    };
    //end else if
    //	return 0;
}
//returns a bounded integer characteristic
//end of the function Characteristic_Integer
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Characteristic_BInteger(
    mut character: i32,
    mut index: i32,
    mut min: i32,
    mut max: i32,
) -> i32 {
    let mut value: i32 = 0; //end if
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() {
        return 0i32;
    }
    if min > max {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"cannot bound characteristic %d between %d and %d\n\x00" as *const u8 as *mut i8,
            index,
            min,
            max,
        );
        return 0i32;
    }
    value = Characteristic_Integer(character, index);
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
//returns a string characteristic
//end of the function Characteristic_BInteger
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn Characteristic_String(
    mut character: i32,
    mut index: i32,
    mut buf: *mut i8,
    mut size: i32,
) {
    let mut ch: *mut bot_character_t = 0 as *mut bot_character_t;
    ch = BotCharacterFromHandle(character);
    if ch.is_null() {
        return;
    }
    //check if the index is in range
    if CheckCharacteristicIndex(character, index) == 0 {
        return;
    }
    //an integer will be converted to a float
    if (*(*ch).c.as_mut_ptr().offset(index as isize)).type_0 as i32 == 3 {
        crate::stdlib::strncpy(
            buf,
            (*(*ch).c.as_mut_ptr().offset(index as isize)).value.string,
            (size - 1i32) as usize,
        ); //end if
        *buf.offset((size - 1i32) as isize) = '\u{0}' as i8
    } else {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3i32,
            b"characteristic %d is not a string\n\x00" as *const u8 as *mut i8,
            index,
        );
    };
    //end else if
}
//free cached bot characters
//end of the function Characteristic_String
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotShutdownCharacters() {
    let mut handle: i32 = 0;
    handle = 1;
    while handle <= 64 {
        if !botcharacters[handle as usize].is_null() {
            BotFreeCharacter2(handle);
        }
        handle += 1
        //end if
    }
    //end for
}
//end of the function BotShutdownCharacters
