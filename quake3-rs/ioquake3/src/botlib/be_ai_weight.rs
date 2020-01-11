// =============== BEGIN be_ai_weight_h ================
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fuzzyseperator_s {
    pub index: i32,
    pub value: i32,
    pub type_0: i32,
    pub weight: f32,
    pub minweight: f32,
    pub maxweight: f32,
    pub child: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_s,
    pub next: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_s,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct weight_s {
    pub name: *mut i8,
    pub firstseperator: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_s,
}

pub type weight_t = crate::src::botlib::be_ai_weight::weight_s;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct weightconfig_s {
    pub numweights: i32,
    pub weights: [crate::src::botlib::be_ai_weight::weight_t; 128],
    pub filename: [i8; 64],
}

pub type weightconfig_t = crate::src::botlib::be_ai_weight::weightconfig_s;

pub type fuzzyseperator_t = crate::src::botlib::be_ai_weight::fuzzyseperator_s;
use ::libc;

pub use crate::botlib_h::botlib_import_s;
pub use crate::botlib_h::botlib_import_t;
pub use crate::botlib_h::bsp_surface_s;
pub use crate::botlib_h::bsp_surface_t;
pub use crate::botlib_h::bsp_trace_s;
pub use crate::botlib_h::bsp_trace_t;
use crate::src::botlib::be_interface::botimport;
use crate::src::botlib::l_libvar::LibVarGetValue;
use crate::src::botlib::l_memory::FreeMemory;
use crate::src::botlib::l_memory::GetClearedMemory;
pub use crate::src::botlib::l_precomp::define_s;
pub use crate::src::botlib::l_precomp::define_t;
pub use crate::src::botlib::l_precomp::indent_s;
pub use crate::src::botlib::l_precomp::indent_t;
pub use crate::src::botlib::l_precomp::source_s;
pub use crate::src::botlib::l_precomp::source_t;
pub use crate::src::botlib::l_precomp::FreeSource;
pub use crate::src::botlib::l_precomp::LoadSourceFile;
pub use crate::src::botlib::l_precomp::PC_CheckTokenString;
pub use crate::src::botlib::l_precomp::PC_ExpectAnyToken;
pub use crate::src::botlib::l_precomp::PC_ExpectTokenString;
pub use crate::src::botlib::l_precomp::PC_ExpectTokenType;
pub use crate::src::botlib::l_precomp::PC_ReadToken;
pub use crate::src::botlib::l_precomp::PC_SetBaseFolder;
pub use crate::src::botlib::l_precomp::SourceError;
pub use crate::src::botlib::l_precomp::SourceWarning;
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
pub use crate::src::qcommon::q_shared::Q_strncpyz;
pub use crate::src::qcommon::q_shared::FS_APPEND;
pub use crate::src::qcommon::q_shared::FS_APPEND_SYNC;
pub use crate::src::qcommon::q_shared::FS_READ;
pub use crate::src::qcommon::q_shared::FS_WRITE;
use crate::stdlib::rand;
use crate::stdlib::strcmp;
use crate::stdlib::strcpy;
use crate::stdlib::strlen;
#[no_mangle]

pub static mut weightFileList: [*mut crate::src::botlib::be_ai_weight::weightconfig_t; 128] =
    [0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t; 128];
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ReadValue(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut value: *mut f32,
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
    if crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) == 0 {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    if crate::stdlib::strcmp(
        token.string.as_mut_ptr(),
        b"-\x00" as *const u8 as *const i8,
    ) == 0
    {
        crate::src::botlib::l_precomp::SourceWarning(
            source,
            b"negative value set to zero\x00" as *const u8 as *mut i8,
        );
        if crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) == 0 {
            crate::src::botlib::l_precomp::SourceError(
                source,
                b"Missing return value\x00" as *const u8 as *mut i8,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    }
    if token.type_0 != 3 {
        crate::src::botlib::l_precomp::SourceError(
            source,
            b"invalid return value %s\x00" as *const u8 as *mut i8,
            token.string.as_mut_ptr(),
        );
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    *value = token.floatvalue;
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function ReadValue
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ReadFuzzyWeight(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
    mut fs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t,
) -> i32 {
    if crate::src::botlib::l_precomp::PC_CheckTokenString(
        source,
        b"balance\x00" as *const u8 as *mut i8,
    ) != 0
    {
        //end if
        (*fs).type_0 = 1; //end if
        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
            source,
            b"(\x00" as *const u8 as *mut i8,
        ) == 0
        {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if ReadValue(source, &mut (*fs).weight) == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
            source,
            b",\x00" as *const u8 as *mut i8,
        ) == 0
        {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if ReadValue(source, &mut (*fs).minweight) == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
            source,
            b",\x00" as *const u8 as *mut i8,
        ) == 0
        {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if ReadValue(source, &mut (*fs).maxweight) == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if crate::src::botlib::l_precomp::PC_ExpectTokenString(
            source,
            b")\x00" as *const u8 as *mut i8,
        ) == 0
        {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    } else {
        (*fs).type_0 = 0;
        if ReadValue(source, &mut (*fs).weight) == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        (*fs).minweight = (*fs).weight;
        (*fs).maxweight = (*fs).weight
    }
    if crate::src::botlib::l_precomp::PC_ExpectTokenString(source, b";\x00" as *const u8 as *mut i8)
        == 0
    {
        return crate::src::qcommon::q_shared::qfalse as i32;
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//end of the function ReadFuzzyWeight
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FreeFuzzySeperators_r(
    mut fs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t,
) {
    if fs.is_null() {
        return;
    }
    if !(*fs).child.is_null() {
        FreeFuzzySeperators_r((*fs).child);
    }
    if !(*fs).next.is_null() {
        FreeFuzzySeperators_r((*fs).next);
    }
    crate::src::botlib::l_memory::FreeMemory(fs as *mut libc::c_void);
}
//end of the function FreeFuzzySeperators
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FreeWeightConfig2(
    mut config: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
) {
    let mut i: i32 = 0; //end for

    for i in 0..(*config).numweights {
        FreeFuzzySeperators_r((*config).weights[i as usize].firstseperator);

        if !(*config).weights[i as usize].name.is_null() {
            crate::src::botlib::l_memory::FreeMemory(
                (*config).weights[i as usize].name as *mut libc::c_void,
            );
        }
    }
    crate::src::botlib::l_memory::FreeMemory(config as *mut libc::c_void);
}
//free a weight configuration
//end of the function FreeWeightConfig2
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FreeWeightConfig(
    mut config: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
) {
    if crate::src::botlib::l_libvar::LibVarGetValue(
        b"bot_reloadcharacters\x00" as *const u8 as *const i8,
    ) == 0.
    {
        return;
    }
    FreeWeightConfig2(config);
}
//end of the function FreeWeightConfig
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ReadFuzzySeperators_r(
    mut source: *mut crate::src::botlib::l_precomp::source_t,
) -> *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t {
    let mut newindent: i32 = 0;
    let mut index: i32 = 0;
    let mut def: i32 = 0;
    let mut founddefault: i32 = 0;
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
    let mut fs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t =
        0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    let mut lastfs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t =
        0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    let mut firstfs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t =
        0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    founddefault = crate::src::qcommon::q_shared::qfalse as i32;
    firstfs = 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    lastfs = 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    if crate::src::botlib::l_precomp::PC_ExpectTokenString(source, b"(\x00" as *const u8 as *mut i8)
        == 0
    {
        return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    }
    if crate::src::botlib::l_precomp::PC_ExpectTokenType(source, 3, 0x1000, &mut token) == 0 {
        return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    }
    index = token.intvalue as i32;
    if crate::src::botlib::l_precomp::PC_ExpectTokenString(source, b")\x00" as *const u8 as *mut i8)
        == 0
    {
        return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    }
    if crate::src::botlib::l_precomp::PC_ExpectTokenString(source, b"{\x00" as *const u8 as *mut i8)
        == 0
    {
        return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    }
    if crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) == 0 {
        return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    }
    loop {
        def = (crate::stdlib::strcmp(
            token.string.as_mut_ptr(),
            b"default\x00" as *const u8 as *const i8,
        ) == 0) as i32;
        //end if
        if def != 0
            || crate::stdlib::strcmp(
                token.string.as_mut_ptr(),
                b"case\x00" as *const u8 as *const i8,
            ) == 0
        {
            fs = crate::src::botlib::l_memory::GetClearedMemory(::std::mem::size_of::<
                crate::src::botlib::be_ai_weight::fuzzyseperator_t,
            >()) as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
            (*fs).index = index;
            if !lastfs.is_null() {
                (*lastfs).next = fs
            } else {
                firstfs = fs
            }
            lastfs = fs; //end else
                         //end if
            if def != 0 {
                //end else
                if founddefault != 0 {
                    crate::src::botlib::l_precomp::SourceError(
                        source,
                        b"switch already has a default\x00" as *const u8 as *mut i8,
                    ); //end if
                    FreeFuzzySeperators_r(firstfs); //end if
                    return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
                } //end if
                (*fs).value = 999999; //end if
                founddefault = crate::src::qcommon::q_shared::qtrue as i32
            } else {
                if crate::src::botlib::l_precomp::PC_ExpectTokenType(source, 3, 0x1000, &mut token)
                    == 0
                {
                    FreeFuzzySeperators_r(firstfs); //end if
                    return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
                }
                (*fs).value = token.intvalue as i32
            }
            if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                source,
                b":\x00" as *const u8 as *mut i8,
            ) == 0
                || crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) == 0
            {
                FreeFuzzySeperators_r(firstfs);
                return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
            }
            newindent = crate::src::qcommon::q_shared::qfalse as i32;
            if crate::stdlib::strcmp(
                token.string.as_mut_ptr(),
                b"{\x00" as *const u8 as *const i8,
            ) == 0
            {
                newindent = crate::src::qcommon::q_shared::qtrue as i32;
                if crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) == 0 {
                    FreeFuzzySeperators_r(firstfs);
                    return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
                }
                //end if
            } //end else
            if crate::stdlib::strcmp(
                token.string.as_mut_ptr(),
                b"return\x00" as *const u8 as *const i8,
            ) == 0
            {
                if ReadFuzzyWeight(source, fs) == 0 {
                    FreeFuzzySeperators_r(firstfs);
                    return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
                }
            //end if
            } else if crate::stdlib::strcmp(
                token.string.as_mut_ptr(),
                b"switch\x00" as *const u8 as *const i8,
            ) == 0
            {
                (*fs).child = ReadFuzzySeperators_r(source);
                if (*fs).child.is_null() {
                    FreeFuzzySeperators_r(firstfs);
                    return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
                }
            //end if
            } else {
                crate::src::botlib::l_precomp::SourceError(
                    source,
                    b"invalid name %s\x00" as *const u8 as *mut i8,
                    token.string.as_mut_ptr(),
                );
                return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
            }
            if newindent != 0 {
                if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                    source,
                    b"}\x00" as *const u8 as *mut i8,
                ) == 0
                {
                    FreeFuzzySeperators_r(firstfs);
                    return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
                }
                //end if
            }
        } else {
            FreeFuzzySeperators_r(firstfs);
            crate::src::botlib::l_precomp::SourceError(
                source,
                b"invalid name %s\x00" as *const u8 as *mut i8,
                token.string.as_mut_ptr(),
            );
            return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
        }
        if crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) == 0 {
            FreeFuzzySeperators_r(firstfs);
            return 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
        }
        if !(crate::stdlib::strcmp(
            token.string.as_mut_ptr(),
            b"}\x00" as *const u8 as *const i8,
        ) != 0)
        {
            break;
        }
    }
    //
    if founddefault == 0 {
        crate::src::botlib::l_precomp::SourceWarning(
            source,
            b"switch without default\x00" as *const u8 as *mut i8,
        ); //end if
        fs = crate::src::botlib::l_memory::GetClearedMemory(::std::mem::size_of::<
            crate::src::botlib::be_ai_weight::fuzzyseperator_t,
        >()) as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
        (*fs).index = index;
        (*fs).value = 999999;
        (*fs).weight = 0f32;
        (*fs).next = 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_s;
        (*fs).child = 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_s;
        if !lastfs.is_null() {
            (*lastfs).next = fs
        } else {
            firstfs = fs
        }
    }
    //
    return firstfs;
}
//reads a weight configuration
//end of the function ReadFuzzySeperators_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ReadWeightConfig(
    mut filename: *mut i8,
) -> *mut crate::src::botlib::be_ai_weight::weightconfig_t {
    let mut newindent: i32 = 0;
    let mut avail: i32 = 0;
    let mut n: i32 = 0;
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
    let mut source: *mut crate::src::botlib::l_precomp::source_t =
        0 as *mut crate::src::botlib::l_precomp::source_t;
    let mut fs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t =
        0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t;
    let mut config: *mut crate::src::botlib::be_ai_weight::weightconfig_t =
        0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
    //DEBUG
    if crate::src::botlib::l_libvar::LibVarGetValue(
        b"bot_reloadcharacters\x00" as *const u8 as *const i8,
    ) == 0.
    {
        avail = -(1); //end if
                      //end if
                      //end for
        for n in 0..128 {
            config = weightFileList[n as usize];

            if config.is_null() {
                //end if
                if avail == -(1) {
                    avail = n
                }
            } else if crate::stdlib::strcmp(filename, (*config).filename.as_mut_ptr()) == 0 {
                //end if
                //botimport.Print( PRT_MESSAGE, "retained %s\n", filename );
                return config;
            }
        }
        if avail == -(1) {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3,
                b"weightFileList was full trying to load %s\n\x00" as *const u8 as *mut i8,
                filename,
            );
            return 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
        }
    }
    crate::src::botlib::l_precomp::PC_SetBaseFolder(b"botfiles\x00" as *const u8 as *mut i8);
    source = crate::src::botlib::l_precomp::LoadSourceFile(filename);
    if source.is_null() {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"counldn\'t load %s\n\x00" as *const u8 as *mut i8,
            filename,
        );
        return 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
    }
    //
    config = crate::src::botlib::l_memory::GetClearedMemory(::std::mem::size_of::<
        crate::src::botlib::be_ai_weight::weightconfig_t,
    >()) as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
    (*config).numweights = 0;
    crate::src::qcommon::q_shared::Q_strncpyz(
        (*config).filename.as_mut_ptr(),
        filename,
        ::std::mem::size_of::<[i8; 64]>() as i32,
    );
    //parse the item config file
    while crate::src::botlib::l_precomp::PC_ReadToken(source, &mut token) != 0 {
        if crate::stdlib::strcmp(
            token.string.as_mut_ptr(),
            b"weight\x00" as *const u8 as *const i8,
        ) == 0
        {
            if (*config).numweights >= 128 {
                crate::src::botlib::l_precomp::SourceWarning(
                    source,
                    b"too many fuzzy weights\x00" as *const u8 as *mut i8,
                ); //end while
                break; //end if
            } else {
                if crate::src::botlib::l_precomp::PC_ExpectTokenType(source, 1, 0, &mut token) == 0
                {
                    FreeWeightConfig(config); //end if
                    crate::src::botlib::l_precomp::FreeSource(source); //end if
                    return 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
                }
                crate::src::botlib::l_script::StripDoubleQuotes(token.string.as_mut_ptr());
                (*config).weights[(*config).numweights as usize].name =
                    crate::src::botlib::l_memory::GetClearedMemory(
                        crate::stdlib::strlen(token.string.as_mut_ptr()).wrapping_add(1usize),
                    ) as *mut i8;
                crate::stdlib::strcpy(
                    (*config).weights[(*config).numweights as usize].name,
                    token.string.as_mut_ptr(),
                );
                if crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) == 0 {
                    FreeWeightConfig(config);
                    crate::src::botlib::l_precomp::FreeSource(source);
                    return 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
                }
                newindent = crate::src::qcommon::q_shared::qfalse as i32;
                if crate::stdlib::strcmp(
                    token.string.as_mut_ptr(),
                    b"{\x00" as *const u8 as *const i8,
                ) == 0
                {
                    newindent = crate::src::qcommon::q_shared::qtrue as i32;
                    if crate::src::botlib::l_precomp::PC_ExpectAnyToken(source, &mut token) == 0 {
                        FreeWeightConfig(config);
                        crate::src::botlib::l_precomp::FreeSource(source);
                        return 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
                    }
                    //end if
                } //end else
                if crate::stdlib::strcmp(
                    token.string.as_mut_ptr(),
                    b"switch\x00" as *const u8 as *const i8,
                ) == 0
                {
                    fs = ReadFuzzySeperators_r(source); //end if
                    if fs.is_null() {
                        FreeWeightConfig(config); //end if
                        crate::src::botlib::l_precomp::FreeSource(source); //end else if
                        return 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
                    } //end if
                    (*config).weights[(*config).numweights as usize].firstseperator = fs
                } else if crate::stdlib::strcmp(
                    token.string.as_mut_ptr(),
                    b"return\x00" as *const u8 as *const i8,
                ) == 0
                {
                    fs = crate::src::botlib::l_memory::GetClearedMemory(::std::mem::size_of::<
                        crate::src::botlib::be_ai_weight::fuzzyseperator_t,
                    >())
                        as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t; //end if
                    (*fs).index = 0;
                    (*fs).value = 999999;
                    (*fs).next = 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_s;
                    (*fs).child = 0 as *mut crate::src::botlib::be_ai_weight::fuzzyseperator_s;
                    if ReadFuzzyWeight(source, fs) == 0 {
                        crate::src::botlib::l_memory::FreeMemory(fs as *mut libc::c_void);
                        FreeWeightConfig(config);
                        crate::src::botlib::l_precomp::FreeSource(source);
                        return 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
                    }
                    (*config).weights[(*config).numweights as usize].firstseperator = fs
                } else {
                    crate::src::botlib::l_precomp::SourceError(
                        source,
                        b"invalid name %s\x00" as *const u8 as *mut i8,
                        token.string.as_mut_ptr(),
                    );
                    FreeWeightConfig(config);
                    crate::src::botlib::l_precomp::FreeSource(source);
                    return 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
                }
                if newindent != 0 {
                    if crate::src::botlib::l_precomp::PC_ExpectTokenString(
                        source,
                        b"}\x00" as *const u8 as *mut i8,
                    ) == 0
                    {
                        FreeWeightConfig(config);
                        crate::src::botlib::l_precomp::FreeSource(source);
                        return 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
                    }
                    //end if
                } //end if
                (*config).numweights += 1
            }
        } else {
            crate::src::botlib::l_precomp::SourceError(
                source,
                b"invalid name %s\x00" as *const u8 as *mut i8,
                token.string.as_mut_ptr(),
            ); //end if
            FreeWeightConfig(config);
            crate::src::botlib::l_precomp::FreeSource(source);
            return 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t;
        }
        //end else
    }
    //free the source at the end of a pass
    crate::src::botlib::l_precomp::FreeSource(source);
    //if the file was located in a pak file
    crate::src::botlib::be_interface::botimport
        .Print
        .expect("non-null function pointer")(
        1,
        b"loaded %s\n\x00" as *const u8 as *mut i8,
        filename,
    );
    //DEBUG
    //
    if crate::src::botlib::l_libvar::LibVarGetValue(
        b"bot_reloadcharacters\x00" as *const u8 as *const i8,
    ) == 0.
    {
        weightFileList[avail as usize] = config
    } //end if
      //
    return config;
}
//find the fuzzy weight with the given name
//end of the function ReadWeightConfig
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FindFuzzyWeight(
    mut wc: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
    mut name: *mut i8,
) -> i32 {
    let mut i: i32 = 0; //end if

    for i in 0..(*wc).numweights {
        if crate::stdlib::strcmp((*wc).weights[i as usize].name, name) == 0 {
            return i;
        }
    }
    return -(1);
}
//end of the function FindFuzzyWeight
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FuzzyWeight_r(
    mut inventory: *mut i32,
    mut fs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t,
) -> f32 {
    let mut scale: f32 = 0.; //end else if
    let mut w1: f32 = 0.; //end if
    let mut w2: f32 = 0.; //end if
    if *inventory.offset((*fs).index as isize) < (*fs).value {
        if !(*fs).child.is_null() {
            return FuzzyWeight_r(inventory, (*fs).child);
        } else {
            return (*fs).weight;
        }
    } else {
        if !(*fs).next.is_null() {
            if *inventory.offset((*fs).index as isize) < (*(*fs).next).value {
                //first weight
                if !(*fs).child.is_null() {
                    w1 = FuzzyWeight_r(inventory, (*fs).child)
                } else {
                    w1 = (*fs).weight
                }
                //second weight
                if !(*(*fs).next).child.is_null() {
                    w2 = FuzzyWeight_r(inventory, (*(*fs).next).child)
                } else {
                    w2 = (*(*fs).next).weight
                }
                //the scale factor
                if (*(*fs).next).value == 999999 {
                    // is fs->next the default case?
                    return w2;
                } else {
                    scale = (*inventory.offset((*fs).index as isize) - (*fs).value) as f32
                        / ((*(*fs).next).value - (*fs).value) as f32
                } // can't interpolate, return default weight
                  //scale between the two weights
                return (1f32 - scale) * w1 + scale * w2;
            }
            return FuzzyWeight_r(inventory, (*fs).next);
        }
    }
    return (*fs).weight;
}
//end of the function FuzzyWeight_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FuzzyWeightUndecided_r(
    mut inventory: *mut i32,
    mut fs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t,
) -> f32 {
    let mut scale: f32 = 0.; //end else if
    let mut w1: f32 = 0.; //end if
    let mut w2: f32 = 0.; //end if
    if *inventory.offset((*fs).index as isize) < (*fs).value {
        if !(*fs).child.is_null() {
            return FuzzyWeightUndecided_r(inventory, (*fs).child);
        } else {
            return (*fs).minweight
                + (crate::stdlib::rand() & 0x7fffi32) as f32 / 32767f32
                    * ((*fs).maxweight - (*fs).minweight);
        }
    } else {
        if !(*fs).next.is_null() {
            if *inventory.offset((*fs).index as isize) < (*(*fs).next).value {
                //first weight
                if !(*fs).child.is_null() {
                    w1 = FuzzyWeightUndecided_r(inventory, (*fs).child)
                } else {
                    w1 = (*fs).minweight
                        + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32
                            * ((*fs).maxweight - (*fs).minweight)
                }
                //second weight
                if !(*(*fs).next).child.is_null() {
                    w2 = FuzzyWeight_r(inventory, (*(*fs).next).child)
                } else {
                    w2 = (*(*fs).next).minweight
                        + (crate::stdlib::rand() & 0x7fff) as f32 / 32767f32
                            * ((*(*fs).next).maxweight - (*(*fs).next).minweight)
                }
                //the scale factor
                if (*(*fs).next).value == 999999 {
                    // is fs->next the default case?
                    return w2;
                } else {
                    scale = (*inventory.offset((*fs).index as isize) - (*fs).value) as f32
                        / ((*(*fs).next).value - (*fs).value) as f32
                } // can't interpolate, return default weight
                  //scale between the two weights
                return (1f32 - scale) * w1 + scale * w2;
            }
            return FuzzyWeightUndecided_r(inventory, (*fs).next);
        }
    }
    return (*fs).weight;
}
//returns the fuzzy weight for the given inventory and weight
//end of the function FuzzyWeightUndecided_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FuzzyWeight(
    mut inventory: *mut i32,
    mut wc: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
    mut weightnum: i32,
) -> f32 {
    return FuzzyWeight_r(inventory, (*wc).weights[weightnum as usize].firstseperator);
}
//end of the function FuzzyWeight
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn FuzzyWeightUndecided(
    mut inventory: *mut i32,
    mut wc: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
    mut weightnum: i32,
) -> f32 {
    return FuzzyWeightUndecided_r(inventory, (*wc).weights[weightnum as usize].firstseperator);
}
//end of the function FuzzyWeightUndecided
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EvolveFuzzySeperator_r(
    mut fs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t,
) {
    if !(*fs).child.is_null() {
        //end else if
        EvolveFuzzySeperator_r((*fs).child); //end if
    } else if (*fs).type_0 == 1 {
        //every once in a while an evolution leap occurs, mutation
        if (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64) < 0.01 {
            (*fs).weight = ((*fs).weight as f64
                + 2.0
                    * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                    * ((*fs).maxweight - (*fs).minweight) as f64) as f32
        } else {
            (*fs).weight = ((*fs).weight as f64
                + 2.0
                    * (((crate::stdlib::rand() & 0x7fff) as f32 / 32767f32) as f64 - 0.5)
                    * ((*fs).maxweight - (*fs).minweight) as f64
                    * 0.5) as f32
        }
        //modify bounds if necessary because of mutation
        if (*fs).weight < (*fs).minweight {
            (*fs).minweight = (*fs).weight
        } else if (*fs).weight > (*fs).maxweight {
            (*fs).maxweight = (*fs).weight
        }
    }
    if !(*fs).next.is_null() {
        EvolveFuzzySeperator_r((*fs).next);
    };
}
//evolves the weight configuration
//end of the function EvolveFuzzySeperator_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn EvolveWeightConfig(
    mut config: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
) {
    let mut i: i32 = 0;
    i = 0;
    while i < (*config).numweights {
        EvolveFuzzySeperator_r((*config).weights[i as usize].firstseperator);
        i += 1
    }
    //end for
}
//end of the function EvolveWeightConfig
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ScaleFuzzySeperator_r(
    mut fs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t,
    mut scale: f32,
) {
    if !(*fs).child.is_null() {
        //end else if
        ScaleFuzzySeperator_r((*fs).child, scale); //end if
    } else if (*fs).type_0 == 1 {
        //
        (*fs).weight = ((*fs).maxweight + (*fs).minweight) * scale;
        //get the weight between bounds
        if (*fs).weight < (*fs).minweight {
            (*fs).weight = (*fs).minweight
        } else if (*fs).weight > (*fs).maxweight {
            (*fs).weight = (*fs).maxweight
        }
    }
    if !(*fs).next.is_null() {
        ScaleFuzzySeperator_r((*fs).next, scale);
    };
}
//scales the weight with the given name
//end of the function ScaleFuzzySeperator_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ScaleWeight(
    mut config: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
    mut name: *mut i8,
    mut scale: f32,
) {
    let mut i: i32 = 0;
    if scale < 0f32 {
        scale = 0f32
    } else if scale > 1f32 {
        scale = 1f32
    }
    i = 0;
    while i < (*config).numweights {
        if crate::stdlib::strcmp(name, (*config).weights[i as usize].name) == 0 {
            ScaleFuzzySeperator_r((*config).weights[i as usize].firstseperator, scale);
            break;
        } else {
            i += 1
        }
        //end if
    }
    //end for
}
//end of the function ScaleWeight
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ScaleFuzzySeperatorBalanceRange_r(
    mut fs: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t,
    mut scale: f32,
) {
    if !(*fs).child.is_null() {
        //end else if
        ScaleFuzzySeperatorBalanceRange_r((*fs).child, scale); //end if
    } else if (*fs).type_0 == 1 {
        let mut mid: f32 = (((*fs).minweight + (*fs).maxweight) as f64 * 0.5) as f32;
        //end if
        (*fs).maxweight = mid + ((*fs).maxweight - mid) * scale;
        (*fs).minweight = mid + ((*fs).minweight - mid) * scale;
        if (*fs).maxweight < (*fs).minweight {
            (*fs).maxweight = (*fs).minweight
        }
    }
    if !(*fs).next.is_null() {
        ScaleFuzzySeperatorBalanceRange_r((*fs).next, scale);
    };
}
//get the weight between bounds
//end of the function ScaleFuzzySeperatorBalanceRange_r
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn ScaleFuzzyBalanceRange(
    mut config: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
    mut scale: f32,
) {
    let mut i: i32 = 0;
    if scale < 0f32 {
        scale = 0f32
    } else if scale > 100f32 {
        scale = 100f32
    }
    i = 0;
    while i < (*config).numweights {
        ScaleFuzzySeperatorBalanceRange_r((*config).weights[i as usize].firstseperator, scale);
        i += 1
    }
    //end for
}
//end of the function ScaleFuzzyBalanceRange
//===========================================================================
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn InterbreedFuzzySeperator_r(
    mut fs1: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t,
    mut fs2: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t,
    mut fsout: *mut crate::src::botlib::be_ai_weight::fuzzyseperator_t,
) -> i32 {
    if !(*fs1).child.is_null() {
        //end else if
        if (*fs2).child.is_null() || (*fsout).child.is_null() {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3,
                b"cannot interbreed weight configs, unequal child\n\x00" as *const u8 as *mut i8,
            ); //end if
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if InterbreedFuzzySeperator_r((*fs2).child, (*fs2).child, (*fsout).child) == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
    //end if
    } else if (*fs1).type_0 == 1 {
        if (*fs2).type_0 != 1 || (*fsout).type_0 != 1 {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3,
                b"cannot interbreed weight configs, unequal balance\n\x00" as *const u8 as *mut i8,
            ); //end if
            return crate::src::qcommon::q_shared::qfalse as i32;
        } //end if
        (*fsout).weight = ((*fs1).weight + (*fs2).weight) / 2f32; //end if
        if (*fsout).weight > (*fsout).maxweight {
            (*fsout).maxweight = (*fsout).weight
        }
        if (*fsout).weight > (*fsout).minweight {
            (*fsout).minweight = (*fsout).weight
        }
    }
    if !(*fs1).next.is_null() {
        if (*fs2).next.is_null() || (*fsout).next.is_null() {
            crate::src::botlib::be_interface::botimport
                .Print
                .expect("non-null function pointer")(
                3,
                b"cannot interbreed weight configs, unequal next\n\x00" as *const u8 as *mut i8,
            );
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        if InterbreedFuzzySeperator_r((*fs1).next, (*fs2).next, (*fsout).next) == 0 {
            return crate::src::qcommon::q_shared::qfalse as i32;
        }
        //end if
    }
    return crate::src::qcommon::q_shared::qtrue as i32;
}
//interbreed the weight configurations and stores the interbreeded one in configout
//end of the function InterbreedFuzzySeperator_r
//===========================================================================
// config1 and config2 are interbreeded and stored in configout
//
// Parameter:				-
// Returns:					-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn InterbreedWeightConfigs(
    mut config1: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
    mut config2: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
    mut configout: *mut crate::src::botlib::be_ai_weight::weightconfig_t,
) {
    let mut i: i32 = 0; //end if
    if (*config1).numweights != (*config2).numweights
        || (*config1).numweights != (*configout).numweights
    {
        crate::src::botlib::be_interface::botimport
            .Print
            .expect("non-null function pointer")(
            3,
            b"cannot interbreed weight configs, unequal numweights\n\x00" as *const u8 as *mut i8,
        );
        return;
    }
    i = 0;
    while i < (*config1).numweights {
        InterbreedFuzzySeperator_r(
            (*config1).weights[i as usize].firstseperator,
            (*config2).weights[i as usize].firstseperator,
            (*configout).weights[i as usize].firstseperator,
        );
        i += 1
    }
    //end for
}
//frees cached weight configurations
//end of the function InterbreedWeightConfigs
//===========================================================================
//
// Parameter:			-
// Returns:				-
// Changes Globals:		-
//===========================================================================
#[no_mangle]

pub unsafe extern "C" fn BotShutdownWeights() {
    let mut i: i32 = 0;
    i = 0;
    while i < 128 {
        if !weightFileList[i as usize].is_null() {
            FreeWeightConfig2(weightFileList[i as usize]);
            weightFileList[i as usize] = 0 as *mut crate::src::botlib::be_ai_weight::weightconfig_t
        }
        i += 1
        //end if
    }
    //end for
}
//end of the function BotShutdownWeights
